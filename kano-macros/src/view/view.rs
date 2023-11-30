use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;

use crate::view::ast::{AttrKey, AttrValue, ComponentAttrs};

use super::ast::{Attr, Element, For, Match, Node};

pub fn view(node: Node) -> TokenStream {
    match node {
        Node::None => quote!(()),
        Node::Element(Element {
            type_path,
            attrs,
            children,
        }) => {
            let span = type_path.span();
            let attrs: Vec<_> = attrs
                .into_iter()
                .map(|attr| match attr {
                    Attr::KeyValue(attr) => {
                        let value = match attr.value {
                            AttrValue::ImplicitTrue => quote! { true },
                            AttrValue::Block(block) => {
                                let span = block.span();
                                quote_spanned! {span=>
                                    #[allow(unused_braces)]
                                    #block
                                }
                            }
                            AttrValue::Literal(lit) => {
                                quote! { #lit }
                            }
                        };

                        match attr.key {
                            AttrKey::On(ident) => {
                                let span = ident.span();
                                quote_spanned! {span=>
                                    kano::on::#ident(
                                        #value
                                    )
                                }
                            }
                            AttrKey::Path(path) => {
                                let span = path.span();
                                quote_spanned! {span=>
                                    #path(#value)
                                }
                            }
                        }
                    }
                    Attr::Implicit(ident) => {
                        quote! { #ident }
                    }
                })
                .collect();
            let attrs = quote_spanned! {span=>
                [#(kano::Attribute::into_prop(#attrs)),*]
            };

            match gen_children(children) {
                Children::Listed(children) => {
                    quote_spanned! {span=>
                        #type_path(#attrs, (
                            #(#children,)*
                        ))
                    }
                }
                Children::Spread(ident) => {
                    quote_spanned! {span=>
                        #type_path(#attrs, #ident)
                    }
                }
            }
        }
        Node::Fragment(_frag) => {
            quote!(())
        }
        Node::Spread(ident) => {
            quote! {
                #ident
            }
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

enum Children {
    Listed(Vec<TokenStream>),
    Spread(syn::Ident),
}

fn gen_children(nodes: Vec<Node>) -> Children {
    if nodes.len() == 1 {
        let node = nodes.into_iter().next().unwrap();
        match node {
            Node::Spread(ident) => Children::Spread(ident),
            _ => Children::Listed(vec![view(node)]),
        }
    } else {
        Children::Listed(nodes.into_iter().map(view).collect())
    }
}
