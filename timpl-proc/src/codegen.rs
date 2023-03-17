use proc_macro::{Delimiter, Group, Literal, Punct, Spacing, TokenStream, TokenTree};
use std::str::FromStr;

pub(crate) fn codegen(format: String, input: TokenStream, offset: usize) -> TokenStream {
    let args = argsgen(input, offset);

    format_call(
        format,
        args.into_iter().map(|arg| {
            if arg.offset > 0 {
                indent_call(
                    arg.offset,
                    format_call("{}".to_string(), Some(arg.stream)),
                )
            } else {
                arg.stream
            }
        }),
    )
}

struct Arg {
    stream: TokenStream,
    offset: usize,
}

fn argsgen(input: TokenStream, offset: usize) -> Vec<Arg> {
    let streams = input
        .clone()
        .into_iter()
        .filter(|token| match token {
            TokenTree::Group(group) => matches!(group.delimiter(), Delimiter::Brace),
            _ => false,
        })
        .map(|token| {
            if let TokenTree::Group(group) = token {
                group.stream()
            } else {
                unreachable!()
            }
        })
        .collect::<Vec<TokenStream>>();

    let offsets = input
        .into_iter()
        .filter_map(|token| match token {
            TokenTree::Group(group) => match group.delimiter() {
                Delimiter::Brace => {
                    if group.span().start().line == group.span().end().line {
                        return Some(0);
                    }
                    Some(group.span().start().column - 1 - offset)
                }
                _ => None,
            },
            _ => None,
        })
        .collect::<Vec<usize>>();

    streams
        .into_iter()
        .zip(offsets.into_iter())
        .map(|(stream, offset)| Arg { stream, offset })
        .collect()
}

fn format_call(format: String, args: impl IntoIterator<Item = TokenStream>) -> TokenStream {
    let args = Some(TokenStream::from_iter(Some(TokenTree::Literal(
        Literal::string(&format),
    ))))
    .into_iter()
    .chain(args);

    fn_call("::std::fmt::format", Some(fn_call("format_args!", args)))
}

fn indent_call(indent: usize, arg: TokenStream) -> TokenStream {
    let indent =
        TokenStream::from_iter(Some(TokenTree::Literal(Literal::usize_unsuffixed(indent))));

    fn_call("::timpl::__internal::indent", vec![indent, arg])
}

fn fn_call(name: &str, args: impl IntoIterator<Item = TokenStream>) -> TokenStream {
    tokenstream(|s| {
        s.extend(TokenStream::from_str(name));
        s.extend(group(
            Delimiter::Parenthesis,
            tokenstream(|s| {
                let mut iter = args.into_iter().peekable();
                while let Some(arg) = iter.next() {
                    s.extend(arg);
                    if iter.peek().is_some() {
                        s.extend(Some(TokenTree::Punct(Punct::new(',', Spacing::Alone))));
                    }
                }
            }),
        ));
    })
}

fn tokenstream(f: impl FnOnce(&mut TokenStream)) -> TokenStream {
    let mut s = TokenStream::new();
    f(&mut s);
    s
}

fn group(delimeter: Delimiter, stream: TokenStream) -> TokenStream {
    let mut s = TokenStream::new();
    s.extend(Some(TokenTree::Group(Group::new(delimeter, stream))));
    s
}
