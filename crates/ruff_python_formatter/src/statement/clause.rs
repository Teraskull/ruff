use crate::comments::{
    leading_alternate_branch_comments, trailing_comments, SourceComment, SuppressionKind,
};
use crate::prelude::*;
use crate::verbatim::write_suppressed_header;
use ruff_formatter::{Argument, Arguments, FormatError};
use ruff_python_ast::node::AnyNodeRef;
use ruff_python_ast::{
    ElifElseClause, ExceptHandlerExceptHandler, MatchCase, Ranged, StmtClassDef, StmtFor,
    StmtFunctionDef, StmtIf, StmtMatch, StmtTry, StmtWhile, StmtWith,
};
use ruff_python_trivia::{SimpleToken, SimpleTokenKind, SimpleTokenizer};
use ruff_text_size::{TextRange, TextSize};

/// The header of a compound statement clause.
///
/// > A compound statement consists of one or more ‘clauses.’ A clause consists of a header and a ‘suite.’
/// > The clause headers of a particular compound statement are all at the same indentation level.
/// > Each clause header begins with a uniquely identifying keyword and ends with a colon.
/// [source](https://docs.python.org/3/reference/compound_stmts.html#compound-statements)
#[derive(Copy, Clone)]
pub(crate) enum ClauseHeader<'a> {
    Class(&'a StmtClassDef),
    Function(&'a StmtFunctionDef),
    If(&'a StmtIf),
    ElifElse(&'a ElifElseClause),
    Try(&'a StmtTry),
    ExceptHandler(&'a ExceptHandlerExceptHandler),
    TryFinally(&'a StmtTry),
    Match(&'a StmtMatch),
    MatchCase(&'a MatchCase),
    For(&'a StmtFor),
    While(&'a StmtWhile),
    With(&'a StmtWith),
    OrElse(ElseClause<'a>),
}

impl<'a> ClauseHeader<'a> {
    /// The range from the clause keyword up to and including the final colon.
    pub(crate) fn range(self, source: &str) -> FormatResult<TextRange> {
        let keyword_range = self.keyword_range(source)?;

        let mut last_child_end = None;

        self.visit(&mut |child| last_child_end = Some(child.end()));

        let end = match self {
            ClauseHeader::Class(class) => Some(last_child_end.unwrap_or(class.name.end())),
            ClauseHeader::Function(function) => Some(last_child_end.unwrap_or(function.name.end())),
            ClauseHeader::ElifElse(_)
            | ClauseHeader::Try(_)
            | ClauseHeader::If(_)
            | ClauseHeader::TryFinally(_)
            | ClauseHeader::Match(_)
            | ClauseHeader::MatchCase(_)
            | ClauseHeader::For(_)
            | ClauseHeader::While(_)
            | ClauseHeader::With(_)
            | ClauseHeader::OrElse(_) => last_child_end,

            ClauseHeader::ExceptHandler(handler) => handler
                .name
                .as_ref()
                .map(ruff_python_ast::Ranged::end)
                .or(last_child_end),
        };

        let colon = colon_range(end.unwrap_or(keyword_range.end()), source)?;

        Ok(TextRange::new(keyword_range.start(), colon.end()))
    }

    /// Visits the nodes in the case header.
    pub(crate) fn visit<F>(self, visitor: &mut F)
    where
        F: FnMut(AnyNodeRef),
    {
        fn visit<'a, N, F>(node: N, visitor: &mut F)
        where
            N: Into<AnyNodeRef<'a>>,
            F: FnMut(AnyNodeRef<'a>),
        {
            visitor(node.into());
        }

        match self {
            ClauseHeader::Class(class) => {
                if let Some(type_params) = &class.type_params {
                    visit(type_params.as_ref(), visitor);
                }

                if let Some(arguments) = &class.arguments {
                    visit(arguments.as_ref(), visitor);
                }
            }
            ClauseHeader::Function(function) => {
                visit(function.parameters.as_ref(), visitor);
                if let Some(type_params) = function.type_params.as_ref() {
                    visit(type_params, visitor);
                }
            }
            ClauseHeader::If(if_stmt) => {
                visit(if_stmt.test.as_ref(), visitor);
            }
            ClauseHeader::ElifElse(clause) => {
                if let Some(test) = clause.test.as_ref() {
                    visit(test, visitor);
                }
            }

            ClauseHeader::ExceptHandler(handler) => {
                if let Some(ty) = handler.type_.as_deref() {
                    visit(ty, visitor);
                }
            }
            ClauseHeader::Match(match_stmt) => {
                visit(match_stmt.subject.as_ref(), visitor);
            }
            ClauseHeader::MatchCase(match_case) => {
                visit(&match_case.pattern, visitor);

                if let Some(guard) = match_case.guard.as_deref() {
                    visit(guard, visitor);
                }
            }
            ClauseHeader::For(for_stmt) => {
                visit(for_stmt.target.as_ref(), visitor);
                visit(for_stmt.iter.as_ref(), visitor);
            }
            ClauseHeader::While(while_stmt) => {
                visit(while_stmt.test.as_ref(), visitor);
            }
            ClauseHeader::With(with_stmt) => {
                for item in &with_stmt.items {
                    visit(item, visitor);
                }
            }
            ClauseHeader::Try(_) | ClauseHeader::TryFinally(_) | ClauseHeader::OrElse(_) => {}
        }
    }

    fn keyword_range(self, source: &str) -> FormatResult<TextRange> {
        match self {
            ClauseHeader::Class(header) => {
                find_keyword(header.start(), SimpleTokenKind::Class, source)
            }
            ClauseHeader::Function(header) => {
                let keyword = if header.is_async {
                    SimpleTokenKind::Async
                } else {
                    SimpleTokenKind::Def
                };
                find_keyword(header.start(), keyword, source)
            }
            ClauseHeader::If(header) => find_keyword(header.start(), SimpleTokenKind::If, source),
            ClauseHeader::ElifElse(ElifElseClause {
                test: None, range, ..
            }) => find_keyword(range.start(), SimpleTokenKind::Else, source),
            ClauseHeader::ElifElse(ElifElseClause {
                test: Some(_),
                range,
                ..
            }) => find_keyword(range.start(), SimpleTokenKind::Elif, source),
            ClauseHeader::Try(header) => find_keyword(header.start(), SimpleTokenKind::Try, source),
            ClauseHeader::ExceptHandler(header) => {
                find_keyword(header.start(), SimpleTokenKind::Except, source)
            }
            ClauseHeader::TryFinally(header) => {
                let last_statement = header
                    .orelse
                    .last()
                    .map(AnyNodeRef::from)
                    .or_else(|| header.handlers.last().map(AnyNodeRef::from))
                    .or_else(|| header.body.last().map(AnyNodeRef::from))
                    .unwrap();

                find_keyword(last_statement.end(), SimpleTokenKind::Finally, source)
            }
            ClauseHeader::Match(header) => {
                find_keyword(header.start(), SimpleTokenKind::Match, source)
            }
            ClauseHeader::MatchCase(header) => {
                find_keyword(header.start(), SimpleTokenKind::Case, source)
            }
            ClauseHeader::For(header) => {
                let keyword = if header.is_async {
                    SimpleTokenKind::Async
                } else {
                    SimpleTokenKind::For
                };
                find_keyword(header.start(), keyword, source)
            }
            ClauseHeader::While(header) => {
                find_keyword(header.start(), SimpleTokenKind::While, source)
            }
            ClauseHeader::With(header) => {
                let keyword = if header.is_async {
                    SimpleTokenKind::Async
                } else {
                    SimpleTokenKind::With
                };

                find_keyword(header.start(), keyword, source)
            }
            ClauseHeader::OrElse(header) => match header {
                ElseClause::Try(try_stmt) => {
                    let last_statement = try_stmt
                        .handlers
                        .last()
                        .map(AnyNodeRef::from)
                        .or_else(|| try_stmt.body.last().map(AnyNodeRef::from))
                        .unwrap();

                    find_keyword(last_statement.end(), SimpleTokenKind::Else, source)
                }
                ElseClause::For(StmtFor { body, .. })
                | ElseClause::While(StmtWhile { body, .. }) => {
                    find_keyword(body.last().unwrap().end(), SimpleTokenKind::Else, source)
                }
            },
        }
    }
}

fn find_keyword(
    start_position: TextSize,
    keyword: SimpleTokenKind,
    source: &str,
) -> FormatResult<TextRange> {
    let mut tokenizer = SimpleTokenizer::starts_at(start_position, source).skip_trivia();

    match tokenizer.next() {
        Some(token) if token.kind() == keyword => Ok(token.range()),
        Some(other) => {
            debug_assert!(
                false,
                "Expected the keyword token {keyword:?} but found the token {other:?} instead."
            );
            Err(FormatError::syntax_error(
                "Expected the keyword token but found another token instead.",
            ))
        }
        None => {
            debug_assert!(
                false,
                "Expected the keyword token {keyword:?} but reached the end of the source instead."
            );
            Err(FormatError::syntax_error(
                "Expected the case header keyword token but reached the end of the source instead.",
            ))
        }
    }
}

fn colon_range(after_keyword_or_condition: TextSize, source: &str) -> FormatResult<TextRange> {
    let mut tokenizer = SimpleTokenizer::starts_at(after_keyword_or_condition, source)
        .skip_trivia()
        .skip_while(|token| token.kind() == SimpleTokenKind::RParen);

    match tokenizer.next() {
        Some(SimpleToken {
            kind: SimpleTokenKind::Colon,
            range,
        }) => Ok(range),
        Some(token) => {
            debug_assert!(false, "Expected the colon marking the end of the case header but found {token:?} instead.");
            Err(FormatError::syntax_error("Expected colon marking the end of the case header but found another token instead."))
        }
        None => {
            debug_assert!(false, "Expected the colon marking the end of the case header but found the end of the range.");
            Err(FormatError::syntax_error("Expected the colon marking the end of the case header but found the end of the range."))
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) enum ElseClause<'a> {
    Try(&'a StmtTry),
    For(&'a StmtFor),
    While(&'a StmtWhile),
}

pub(crate) struct FormatClauseHeader<'a, 'ast> {
    header: ClauseHeader<'a>,
    /// How to format the clause header
    formatter: Argument<'a, PyFormatContext<'ast>>,

    /// Leading comments coming before the branch, together with the previous node, if any. Only relevant
    /// for alternate branches.
    leading_comments: Option<(&'a [SourceComment], Option<AnyNodeRef<'a>>)>,

    /// The trailing comments coming after the colon.
    trailing_colon_comment: &'a [SourceComment],
}

/// Formats a clause header, handling the case where the clause header is suppressed and should not be formatted.
///
/// Calls the `formatter` to format the content of the `header`, except if the `trailing_colon_comment` is a `fmt: skip` suppression comment.
/// Takes care of formatting the `trailing_colon_comment` and adds the `:` at the end of the header.
pub(crate) fn clause_header<'a, 'ast, Content>(
    header: ClauseHeader<'a>,
    trailing_colon_comment: &'a [SourceComment],
    formatter: &'a Content,
) -> FormatClauseHeader<'a, 'ast>
where
    Content: Format<PyFormatContext<'ast>>,
{
    FormatClauseHeader {
        header,
        formatter: Argument::new(formatter),
        leading_comments: None,
        trailing_colon_comment,
    }
}

impl<'a> FormatClauseHeader<'a, '_> {
    /// Sets the leading comments that precede an alternate branch.
    #[must_use]
    pub(crate) fn with_leading_comments<N>(
        mut self,
        comments: &'a [SourceComment],
        last_node: Option<N>,
    ) -> Self
    where
        N: Into<AnyNodeRef<'a>>,
    {
        self.leading_comments = Some((comments, last_node.map(Into::into)));
        self
    }
}

impl<'ast> Format<PyFormatContext<'ast>> for FormatClauseHeader<'_, 'ast> {
    fn fmt(&self, f: &mut Formatter<PyFormatContext<'ast>>) -> FormatResult<()> {
        if let Some((leading_comments, last_node)) = self.leading_comments {
            leading_alternate_branch_comments(leading_comments, last_node).fmt(f)?;
        }

        if SuppressionKind::has_skip_comment(self.trailing_colon_comment, f.context().source()) {
            write_suppressed_header(self.header, f)?;
        } else {
            f.write_fmt(Arguments::from(&self.formatter))?;
            text(":").fmt(f)?;
        }

        trailing_comments(self.trailing_colon_comment).fmt(f)
    }
}
