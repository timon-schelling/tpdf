#![feature(proc_macro_span)]

use proc_macro::TokenStream;

#[proc_macro]
pub fn timpl(input: TokenStream) -> TokenStream {
    template(input, true)
}

#[proc_macro]
pub fn timpl_raw(input: TokenStream) -> TokenStream {
    template(input, false)
}

fn template(input: TokenStream, align: bool) -> TokenStream {
    let mut format = parse::parse(input.clone());

    let mut offset = 0;

    if align {
        offset = transform::calc_offset(&format);
        format = transform::apply_offset(&format, offset);
    }

    codegen::codegen(format, input, offset)
}

mod codegen;
mod parse;
mod transform;
