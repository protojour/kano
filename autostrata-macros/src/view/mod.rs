use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use self::ast::{Element, Match, Node};

pub mod ast;

pub fn view(node: Node) -> TokenStream {
    match node {
        Node::None => quote!(()),
        Node::Element(Element {
            tag_name,
            attrs: _,
            children,
        }) => {
            let children = children.into_iter().map(view);

            quote! {
                #tag_name((), (
                    #(#children,)*
                ))
            }
        }
        Node::Fragment(_frag) => {
            quote!(())
        }
        Node::Text(text) => {
            let literal = text.0.into_token_stream();
            quote! {
                autostrata::view::Text(#literal)
            }
        }
        Node::TextExpr(expr) => {
            quote!(
                autostrata::view::Reactive(move || #expr)
            )
        }
        Node::Component(_component) => {
            quote!(())
        }
        Node::Match(Match { expr, arms }) => {
            let arms = arms.into_iter().enumerate().map(|(index, arm)| {
                let pat = arm.pat;
                let view = view(arm.node);
                if index == 0 {
                    quote!(
                        #pat => autostrata::view::Either::Left(#view),
                    )
                } else {
                    quote!(
                        #pat => autostrata::view::Either::Right(#view),
                    )
                }
            });

            quote!(
                autostrata::view::Reactive(move || {
                    match #expr {
                        #(#arms)*
                    }
                })
            )
        }
        Node::For(_for) => {
            quote!(())
        }
    }
}
