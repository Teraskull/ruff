use ruff_formatter::{write, Buffer, FormatResult};
use ruff_python_ast::MatchCase;

use crate::comments::SourceComment;
use crate::not_yet_implemented_custom_text;
use crate::prelude::*;
use crate::statement::clause::{clause_header, ClauseHeader};
use crate::{FormatNodeRule, PyFormatter};

#[derive(Default)]
pub struct FormatMatchCase;

impl FormatNodeRule<MatchCase> for FormatMatchCase {
    fn fmt_fields(&self, item: &MatchCase, f: &mut PyFormatter) -> FormatResult<()> {
        let MatchCase {
            range: _,
            pattern,
            guard,
            body,
        } = item;

        let comments = f.context().comments().clone();
        let dangling_item_comments = comments.dangling_comments(item);

        write!(
            f,
            [
                clause_header(
                    ClauseHeader::MatchCase(item),
                    dangling_item_comments,
                    &format_with(|f| {
                        write!(
                            f,
                            [
                                text("case"),
                                space(),
                                format_with(|f: &mut PyFormatter| {
                                    let comments = f.context().comments();

                                    for comment in comments.leading_trailing_comments(pattern) {
                                        // This is a lie, but let's go with it.
                                        comment.mark_formatted();
                                    }

                                    // Replace the whole `format_with` with `pattern.format()` once pattern formatting is implemented.
                                    not_yet_implemented_custom_text(
                                        "NOT_YET_IMPLEMENTED_Pattern",
                                        pattern,
                                    )
                                    .fmt(f)
                                }),
                            ]
                        )?;

                        if let Some(guard) = guard {
                            write!(f, [space(), text("if"), space(), guard.format()])?;
                        }

                        Ok(())
                    }),
                ),
                block_indent(&body.format())
            ]
        )
    }

    fn fmt_dangling_comments(
        &self,
        _dangling_comments: &[SourceComment],
        _f: &mut PyFormatter,
    ) -> FormatResult<()> {
        // Handled as part of `fmt_fields`
        Ok(())
    }
}
