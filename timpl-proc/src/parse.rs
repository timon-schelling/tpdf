use proc_macro::{Delimiter, LineColumn, TokenStream, TokenTree};

pub(crate) fn parse(input: TokenStream) -> String {
    let mut previous: Option<LineColumn> = None;

    input.into_iter().fold(String::new(), |mut acc, token| {
        if previous.is_none() {
            previous = Some(LineColumn {
                line: token.span().start().line,
                column: 1,
            });
        }

        if let Some(previous) = previous {
            let current = token.span().start();

            let newlines = current.line - previous.line;

            acc.push_str("\n".repeat(newlines).as_str());

            let spaces = if newlines == 0 {
                current.column - previous.column
            } else {
                current.column - 1
            };

            acc.push_str(" ".repeat(spaces).as_str());
        }

        previous = Some(token.span().end());

        match token {
            TokenTree::Group(group) => match group.delimiter() {
                Delimiter::Brace => {
                    acc.push('{');
                    acc.push('}');
                }

                Delimiter::Parenthesis => {
                    acc.push('(');
                    acc.push_str(&parse(group.stream()));
                    acc.push(')');
                }

                Delimiter::Bracket => {
                    acc.push('[');
                    acc.push_str(&parse(group.stream()));
                    acc.push(']');
                }

                Delimiter::None => {
                    acc.push_str(&parse(group.stream()));
                }
            },

            TokenTree::Ident(ident) => {
                acc.push_str(&ident.to_string());
            }

            TokenTree::Punct(punct) => {
                acc.push_str(&punct.to_string());
            }

            TokenTree::Literal(literal) => {
                acc.push_str(&literal.to_string());
            }
        }

        acc
    })
}
