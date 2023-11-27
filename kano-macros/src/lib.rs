mod view;

#[proc_macro]
pub fn view(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output = view::view(syn::parse_macro_input!(input as view::ast::Node));

    proc_macro::TokenStream::from(output)
}
