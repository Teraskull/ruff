use ruff_python_trivia::{first_non_trivia_token, SimpleTokenKind, SimpleTokenizer};
use ruff_text_size::TextRange;

use crate::node::AnyNodeRef;
use crate::Ranged;

/// A wrapper around an expression that may be parenthesized.
#[derive(Debug)]
pub struct ParenthesizedExpression<'a> {
    /// The underlying AST node.
    expr: AnyNodeRef<'a>,
    /// The range of the expression including parentheses, if the expression is parenthesized;
    /// or `None`, if the expression is not parenthesized.
    range: Option<TextRange>,
}

impl<'a> ParenthesizedExpression<'a> {
    pub fn from_expr(expr: AnyNodeRef<'a>, contents: &str) -> Self {
        Self {
            expr,
            range: parenthesized_range(expr, contents),
        }
    }

    /// Returns `true` if the expression is parenthesized.
    pub fn is_parenthesized(&self) -> bool {
        self.range.is_some()
    }
}

impl Ranged for ParenthesizedExpression<'_> {
    fn range(&self) -> TextRange {
        self.range.unwrap_or_else(|| self.expr.range())
    }
}

/// Returns the [`TextRange`] of a given expression including parentheses, if the expression is
/// parenthesized; or `None`, if the expression is not parenthesized.
fn parenthesized_range(expr: AnyNodeRef, contents: &str) -> Option<TextRange> {
    // First, test if there's a closing parenthesis because it tends to be cheaper.
    let right = first_non_trivia_token(expr.end(), contents)?;

    if right.kind == SimpleTokenKind::RParen {
        // Next, test for the opening parenthesis.
        let mut tokenizer =
            SimpleTokenizer::up_to_without_back_comment(expr.start(), contents).skip_trivia();
        let left = tokenizer.next_back()?;
        if left.kind == SimpleTokenKind::LParen {
            return Some(TextRange::new(left.start(), right.end()));
        }
    }

    None
}
