use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};

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
            let tag_name_span = tag_name.span();
            let tag_name = syn::LitStr::new(&tag_name.to_string(), tag_name.span());
            let children = children.into_iter().map(view);

            let element_new = quote_spanned! {tag_name_span=>
                autostrata::view::Element::new
            };

            quote! {
                #element_new(#tag_name, (), (
                    #(#children,)*
                ))
            }
        }
        Node::Fragment(_frag) => {
            quote!(())
        }
        Node::Text(text) => text.0.into_token_stream(),
        Node::TextExpr(_expr) => {
            quote!(())
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
