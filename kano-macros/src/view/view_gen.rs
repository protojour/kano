use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;

use crate::view::ast::{AttrValue, ComponentAttrs};

use super::ast::{Attr, Element, For, Match, Node, NodeKind, View};

pub fn view(view: View) -> TokenStream {
    let gen = ViewGen {
        common_namespace: view.common_namespace,
    };

    gen.node(view.root_node, Const::No)
}

struct ViewGen {
    common_namespace: Option<syn::Path>,
}

#[derive(Clone, Copy)]
enum Const {
    No,
    Current,
    Parent,
}

impl ViewGen {
    fn node(&self, node: Node, mut constant: Const) -> TokenStream {
        match (constant, node.constant) {
            (Const::No, true) => {
                constant = Const::Current;
            }
            (Const::Current, _) => {
                constant = Const::Parent;
            }
            _ => {}
        }

        match node.kind {
            NodeKind::None => quote!(()),
            NodeKind::Element(Element {
                path,
                attrs,
                children,
            }) => {
                let span = path.span();
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

                            let key = attr.key;

                            quote_spanned! {span=>
                                #key(#value)
                            }
                        }
                        Attr::Implicit(ident) => {
                            quote! { #ident }
                        }
                    })
                    .collect();
                let attrs = quote_spanned! {span=>
                    [#(::kano::FromProperty::from_property(#attrs)),*]
                };

                let path = self.element_path(&path);

                let view = match self.gen_children(children, constant) {
                    Children::Listed(children) => {
                        quote_spanned! {span=>
                            #path(#attrs, (
                                #(#children,)*
                            ))
                        }
                    }
                    Children::Spread(ident) => {
                        quote_spanned! {span=>
                            #path(#attrs, #ident)
                        }
                    }
                };

                if matches!(constant, Const::Current) {
                    quote_spanned! { span=> ::kano::view::Const(#view) }
                } else {
                    view
                }
            }
            NodeKind::Fragment(_frag) => {
                quote!(())
            }
            NodeKind::Spread(ident) => {
                quote! {
                    #ident
                }
            }
            NodeKind::Text(text) => {
                let span = text.0.span();
                let literal = text.0.into_token_stream();
                quote_spanned! {span=>
                    #literal
                }
            }
            NodeKind::TextExpr(expr) => {
                let span = expr.span();
                quote_spanned! {span=>
                    ::kano::view::Reactive(move || #expr)
                }
            }
            NodeKind::Component(component) => {
                let path = component.path;
                let span = path.span();
                match component.attrs {
                    ComponentAttrs::Positional(positional) => {
                        quote_spanned! {span=>
                            ::kano::view::Reactive(move ||
                                ::kano::view::Func(#path, (#(#positional),*,))
                            )
                        }
                    }
                    ComponentAttrs::KeyValue(_) => {
                        quote_spanned! {span=>
                            ::kano::view::Func(#path, ())
                        }
                    }
                }
            }
            NodeKind::Match(Match { expr, arms }) => {
                let span = expr.span();
                let arms = arms.into_iter().enumerate().map(|(index, arm)| {
                    let pat = arm.pat;
                    let span = pat.span();
                    let view = self.node(arm.node, constant);
                    if index == 0 {
                        quote_spanned! {span=>
                            #pat => ::kano::view::Either::Left(#view),
                        }
                    } else {
                        quote_spanned! {span=>
                            #pat => ::kano::view::Either::Right(#view),
                        }
                    }
                });

                quote_spanned! {span=>
                    ::kano::view::Reactive(move || {
                        match #expr {
                            #(#arms)*
                        }
                    })
                }
            }
            NodeKind::For(For {
                for_token,
                pat,
                in_token: _,
                expression,
                repeating_node,
            }) => {
                let span = for_token.span;
                let child = self.node(*repeating_node, constant);

                quote_spanned! {span=>
                    ::kano::view::seq_map(#expression, move |#pat| {
                        #child
                    })
                }
            }
        }
    }

    fn gen_children(&self, nodes: Vec<Node>, constant: Const) -> Children {
        if nodes.len() == 1 {
            let node = nodes.into_iter().next().unwrap();
            match node.kind {
                NodeKind::Spread(ident) => Children::Spread(ident),
                _ => Children::Listed(vec![self.node(node, constant)]),
            }
        } else {
            Children::Listed(
                nodes
                    .into_iter()
                    .map(|node| self.node(node, constant))
                    .collect(),
            )
        }
    }

    fn element_path(&self, path: &syn::Path) -> TokenStream {
        let span = path.span();

        if path.leading_colon.is_none()
            && path.segments.len() == 1
            && self.common_namespace.is_some()
        {
            let mut out_path = self.common_namespace.clone().unwrap();

            out_path.segments.extend(path.segments.clone());
            quote_spanned! {span=> #out_path }
        } else {
            quote_spanned! {span=> #path }
        }
    }
}

enum Children {
    Listed(Vec<TokenStream>),
    Spread(syn::Ident),
}
