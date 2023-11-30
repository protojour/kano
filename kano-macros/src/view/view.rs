use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;

use crate::view::ast::{AttrKey, AttrValue, ComponentAttrs};

use super::ast::{Element, For, Match, Node};

pub fn view(node: Node) -> TokenStream {
    match node {
        Node::None => quote!(()),
        Node::Element(Element {
            tag_name,
            attrs,
            children,
        }) => {
            let span = tag_name.span();
            let attrs: Vec<_> = attrs
                .into_iter()
                .map(|attr| {
                    let value = match attr.value {
                        AttrValue::ImplicitTrue => quote! { true },
                        AttrValue::Block(block) => {
                            let span = block.span();
                            quote_spanned! {span=>
                                #[allow(unused_braces)]
                                #block
                            }
                        }
                        _ => todo!(),
                    };

                    match attr.key {
                        AttrKey::On(event) => {
                            quote! {
                                kano::on::#event({#value})
                            }
                        }
                        AttrKey::Text(_) => todo!(),
                    }
                })
                .collect();
            let attrs = quote_spanned! {span=>
                [#(kano::Attribute::into_prop(#attrs)),*]
            };

            let children = children.into_iter().map(view);

            quote_spanned! {span=>
                #tag_name(#attrs, (
                    #(#children,)*
                ))
            }
        }
        Node::Fragment(_frag) => {
            quote!(())
        }
        Node::Text(text) => {
            let span = text.0.span();
            let literal = text.0.into_token_stream();
            quote_spanned! {span=>
                #literal
            }
        }
        Node::TextExpr(expr) => {
            let span = expr.span();
            quote_spanned! {span=>
                kano::view::Reactive(move || #expr)
            }
        }
        Node::Component(component) => {
            let type_path = component.type_path;
            let span = type_path.span();
            match component.attrs {
                ComponentAttrs::Positional(positional) => {
                    quote_spanned! {span=>
                        kano::view::Reactive(move ||
                            kano::view::Func(#type_path, (#(#positional),*,))
                        )
                    }
                }
                ComponentAttrs::KeyValue(_) => {
                    quote_spanned! {span=>
                        kano::view::Func(#type_path, ())
                    }
                }
            }
        }
        Node::Match(Match { expr, arms }) => {
            let span = expr.span();
            let arms = arms.into_iter().enumerate().map(|(index, arm)| {
                let pat = arm.pat;
                let span = pat.span();
                let view = view(arm.node);
                if index == 0 {
                    quote_spanned! {span=>
                        #pat => kano::view::Either::Left(#view),
                    }
                } else {
                    quote_spanned! {span=>
                        #pat => kano::view::Either::Right(#view),
                    }
                }
            });

            quote_spanned! {span=>
                kano::view::Reactive(move || {
                    match #expr {
                        #(#arms)*
                    }
                })
            }
        }
        Node::For(For {
            for_token,
            pat,
            in_token: _,
            expression,
            repeating_node,
        }) => {
            let span = for_token.span;
            let child = view(*repeating_node);

            quote_spanned! {span=>
                kano::view::seq_map(#expression, move |#pat| {
                    #child
                })
            }
        }
    }
}
