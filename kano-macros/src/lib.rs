//! Kano is a work-in-progress GUI application framework written for and in Rust.

mod from_property;
mod svg_view;
mod view;

#[proc_macro]
pub fn view(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output = view::view(syn::parse_macro_input!(input as view::ast::View));

    proc_macro::TokenStream::from(output)
}

#[proc_macro]
pub fn svg_view(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::LitStr);
    let output = svg_view::svg_view(input.value().into());
    proc_macro::TokenStream::from(output)
}

#[proc_macro_derive(FromProperty)]
pub fn from_property(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let result = from_property::from_property(syn::parse_macro_input!(input as syn::ItemEnum));

    proc_macro::TokenStream::from(match result {
        Ok(tokens) => tokens,
        Err(error) => error.into_compile_error(),
    })
}
