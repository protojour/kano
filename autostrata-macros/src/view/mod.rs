use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::view::ast::{AttrKey, AttrValue};

use self::ast::{Element, Match, Node};

pub mod ast;

pub fn view(node: Node) -> TokenStream {
    match node {
        Node::None => quote!(()),
        Node::Element(Element {
            tag_name,
            attrs,
            children,
        }) => {
            let attrs: Vec<_> = attrs
                .into_iter()
                .map(|attr| {
                    let value = match attr.value {
                        AttrValue::ImplicitTrue => quote! { true },
                        AttrValue::Expr(expr) => quote! { #expr },
                        _ => todo!(),
                    };

                    match attr.key {
                        AttrKey::On(event) => {
                            quote! {
                                autostrata::On::#event(#value)
                            }
                        }
                        AttrKey::Text(_) => todo!(),
                    }
                })
                .collect();
            let attrs = if attrs.is_empty() {
                quote! { () }
            } else {
                quote! { (#(#attrs),*,) }
            };

            let children = children.into_iter().map(view);

            quote! {
                #tag_name(#attrs, (
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
        Node::Component(component) => {
            let type_path = component.type_path;

            quote! {
                autostrata::view::Func(#type_path)
            }
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
