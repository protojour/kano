//! Kano is a work-in-progress GUI application framework written for and in Rust.

mod attribute;
mod view;

#[proc_macro]
pub fn view(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output = view::view(syn::parse_macro_input!(input as view::ast::Node));

    proc_macro::TokenStream::from(output)
}

#[proc_macro_derive(Attribute)]
pub fn attribute(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let result = attribute::attribute(syn::parse_macro_input!(input as syn::ItemEnum));

    proc_macro::TokenStream::from(match result {
        Ok(tokens) => tokens,
        Err(error) => error.into_compile_error(),
    })
}
