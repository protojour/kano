use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use self::ast::{Element, Node};

pub mod ast;

pub fn view(node: Node) -> TokenStream {
    match node {
        Node::Element(Element {
            tag_name,
            attrs,
            children,
        }) => {
            let tag_name = syn::LitStr::new(&tag_name.to_string(), tag_name.span());
            let children = children.into_iter().map(view);

            quote! {
                autostrata::Element::new(#tag_name, (), (
                    #(#children,)*
                ))
            }
        }
        Node::Fragment(frag) => {
            quote!()
        }
        Node::Text(text) => text.0.into_token_stream(),
        Node::TextExpr(expr) => {
            quote!()
        }
        Node::Component(component) => {
            quote!()
        }
        Node::Match(match_) => {
            quote!()
        }
        Node::For(for_) => {
            quote!()
        }
    }
}
