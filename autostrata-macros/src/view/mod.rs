use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use self::ast::{Element, Node};

pub mod ast;

pub fn view(node: Node) -> TokenStream {
    match node {
        Node::Element(Element {
            tag_name,
            attrs: _,
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
        Node::Fragment(_frag) => {
            quote!()
        }
        Node::Text(text) => text.0.into_token_stream(),
        Node::TextExpr(_expr) => {
            quote!()
        }
        Node::Component(_component) => {
            quote!()
        }
        Node::Match(_match) => {
            quote!()
        }
        Node::For(_for) => {
            quote!()
        }
    }
}
