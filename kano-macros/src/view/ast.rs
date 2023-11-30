//!
//! Abstract Syntax Tree data types,
//! plus parsing of the _template_ into AST.
//!
//! 'template' is the markup-like syntax used to express the GUI.
//!

use std::fmt::Display;

use syn::parse::{Parse, ParseStream};

pub struct Parser;

impl Parser {}

#[derive(Debug, Eq, PartialEq)]
pub enum Node {
    None,
    Element(Element),
    Fragment(Vec<Node>),
    Spread(syn::Ident),
    Text(Text),
    TextExpr(syn::Expr),
    Component(Component),
    Match(Match),
    For(For),
}

#[derive(Eq, PartialEq, Debug)]
pub enum TagName {
    Element(syn::TypePath),
    Component(syn::TypePath),
}

impl TagName {
    fn type_path(&self) -> &syn::TypePath {
        match self {
            Self::Element(type_path) => type_path,
            Self::Component(type_path) => type_path,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attr {
    pub key: AttrKey,
    pub value: AttrValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttrKey {
    Text(syn::Ident),
    On(syn::Ident),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttrValue {
    ImplicitTrue,
    Literal(syn::Lit),
    Block(syn::ExprBlock),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Text(pub syn::LitStr);

impl Parse for Text {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lit_str = input.parse::<syn::LitStr>()?;
        Ok(Self(lit_str))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Element {
    pub type_path: syn::TypePath,
    pub attrs: Vec<Attr>,
    pub children: Vec<Node>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Component {
    pub type_path: syn::TypePath,
    pub attrs: ComponentAttrs,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ComponentAttrs {
    KeyValue(Vec<Attr>),
    Positional(Vec<syn::Expr>),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Match {
    pub expr: syn::Expr,
    pub arms: Vec<MatchArm>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct MatchArm {
    pub pat: syn::Pat,
    pub node: Node,
}

#[derive(Debug, Eq, PartialEq)]
pub struct For {
    pub for_token: syn::token::For,
    pub pat: syn::Pat,
    pub in_token: syn::token::In,
    pub expression: syn::Expr,
    pub repeating_node: Box<Node>,
}

struct DisplayTypePath<'a>(&'a syn::TypePath);

impl<'a> Display for DisplayTypePath<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = &self.0;
        let tokens = quote::quote! {
            #path
        };

        tokens.fmt(f)
    }
}

pub fn parse_tag_name(input: ParseStream) -> Result<TagName, syn::Error> {
    let type_path: syn::TypePath = input.parse()?;

    let last_segment = type_path.path.segments.last().unwrap();

    let ident_string = last_segment.ident.to_string();

    if ident_string.as_str() < "a" {
        // Component names start with an uppercase letter
        Ok(TagName::Component(type_path))
    } else {
        Ok(TagName::Element(type_path))
    }
}

enum TagWithAttrs {
    Element(syn::TypePath, Vec<Attr>),
    Component(syn::TypePath, ComponentAttrs),
}

impl TagWithAttrs {
    fn type_path(&self) -> &syn::TypePath {
        match self {
            Self::Element(type_path, _) => type_path,
            Self::Component(type_path, _) => type_path,
        }
    }
}

impl Parse for Node {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Parser.parse_node(input)
    }
}

impl Parser {
    #[allow(unused)]
    pub fn parse_at_least_one(&self, input: ParseStream) -> syn::Result<Node> {
        let mut nodes = vec![self.parse_node(input)?];

        while !input.is_empty() {
            nodes.push(self.parse_node(input)?);
        }

        if nodes.len() == 1 {
            Ok(nodes.into_iter().next().unwrap())
        } else {
            Ok(Node::Fragment(nodes))
        }
    }

    fn parse_node(&self, input: ParseStream) -> syn::Result<Node> {
        if input.peek(syn::token::Lt) {
            return self.parse_element_or_fragment(input);
        }

        if let Ok(text) = input.parse::<Text>() {
            return Ok(Node::Text(text));
        }

        if input.peek(syn::token::DotDot) {
            let _dot_dot: syn::token::DotDot = input.parse()?;
            let ident = input.parse()?;
            return Ok(Node::Spread(ident));
        }

        if input.peek(syn::token::If) {
            return Ok(Node::Match(self.parse_if(input)?));
        }

        if input.peek(syn::token::Match) {
            return Ok(Node::Match(self.parse_match(input)?));
        }

        if input.peek(syn::token::For) {
            return Ok(Node::For(self.parse_for(input)?));
        }

        // Fallback: evaluate expression in {}
        // BUG: produce custom error message
        let content;
        let _brace_token = syn::braced!(content in input);

        let expr: syn::Expr = content.parse()?;

        Ok(Node::TextExpr(expr))
    }

    fn parse_element_or_fragment(&self, input: ParseStream) -> syn::Result<Node> {
        // Opening:
        input.parse::<syn::token::Lt>()?;

        if input.peek(syn::token::Gt) {
            input.parse::<syn::token::Gt>()?;
            let nodes = self.parse_children(input)?;
            input.parse::<syn::token::Lt>()?;
            input.parse::<syn::token::Slash>()?;
            input.parse::<syn::token::Gt>()?;

            return Ok(Node::Fragment(nodes));
        }

        let tag_with_attrs = self.parse_tag_then_attrs(input)?;

        if input.peek(syn::token::Slash) {
            input.parse::<syn::token::Slash>()?;
            input.parse::<syn::token::Gt>()?;

            return self.element_or_component(tag_with_attrs, vec![]);
        }

        input.parse::<syn::token::Gt>()?;

        let children = self.parse_children(input)?;

        // Closing:
        input.parse::<syn::token::Lt>()?;
        input.parse::<syn::token::Slash>()?;

        let end_name = parse_tag_name(input)?;
        if end_name.type_path() != tag_with_attrs.type_path() {
            return Err(syn::Error::new(
                input.span(),
                format!(
                    "Unexpected closing name `{}`. Expected `{}`.",
                    DisplayTypePath(end_name.type_path()),
                    DisplayTypePath(tag_with_attrs.type_path()),
                ),
            ));
        }
        input.parse::<syn::token::Gt>()?;

        self.element_or_component(tag_with_attrs, children)
    }

    fn element_or_component(
        &self,
        tag_with_attrs: TagWithAttrs,
        children: Vec<Node>,
    ) -> syn::Result<Node> {
        match tag_with_attrs {
            TagWithAttrs::Element(type_path, attrs) => {
                let attrs = attrs
                    .into_iter()
                    .map(|attr| {
                        Ok(Attr {
                            key: attr.key,
                            value: attr.value,
                        })
                    })
                    .collect::<syn::Result<Vec<_>>>()?;

                Ok(Node::Element(Element {
                    type_path,
                    attrs,
                    children,
                }))
            }
            TagWithAttrs::Component(type_path, attrs) => {
                Ok(Node::Component(Component { type_path, attrs }))
            }
        }
    }

    fn parse_tag_then_attrs(&self, input: ParseStream) -> Result<TagWithAttrs, syn::Error> {
        let tag_name = parse_tag_name(input)?;
        match tag_name {
            TagName::Element(ident) => {
                let attrs = self.parse_key_value_attrs(input)?;

                Ok(TagWithAttrs::Element(ident, attrs))
            }
            TagName::Component(type_path) => {
                let component_attrs = if input.peek(syn::token::Brace) {
                    let mut expressions = vec![];
                    while input.peek(syn::token::Brace) {
                        let content;
                        let _brace_token = syn::braced!(content in input);

                        expressions.push(content.parse()?);
                    }

                    ComponentAttrs::Positional(expressions)
                } else {
                    ComponentAttrs::KeyValue(self.parse_key_value_attrs(input)?)
                };

                Ok(TagWithAttrs::Component(type_path, component_attrs))
            }
        }
    }

    /// Parse the attributes to an element or component
    fn parse_key_value_attrs(&self, input: ParseStream) -> syn::Result<Vec<Attr>> {
        let mut attrs = vec![];

        loop {
            if input.peek(syn::token::Slash) || input.peek(syn::token::Gt) {
                break;
            }

            let name = input.parse()?;
            let key = if input.peek(syn::token::Colon) {
                if name == "on" {
                    let _ = input.parse::<syn::token::Colon>()?;
                    let event = input.parse()?;

                    AttrKey::On(event)
                } else {
                    return Err(syn::Error::new(
                        input.span(),
                        "Invalid attribute prefix".to_string(),
                    ));
                }
            } else {
                AttrKey::Text(name)
            };

            let value = if input.peek(syn::token::Eq) {
                input.parse::<syn::token::Eq>()?;
                self.parse_attr_value(input)?
            } else {
                AttrValue::ImplicitTrue
            };

            attrs.push(Attr { key, value });
        }

        Ok(attrs)
    }

    fn parse_attr_value(&self, input: ParseStream) -> syn::Result<AttrValue> {
        if input.peek(syn::Lit) {
            Ok(AttrValue::Literal(input.parse()?))
        } else {
            // let content;
            // let _brace_token = syn::braced!(content in input);

            let expr: syn::ExprBlock = input.parse()?;
            Ok(AttrValue::Block(expr))
        }
    }

    /// Parse children until we see the start of a closing tag
    fn parse_children(&self, input: ParseStream) -> syn::Result<Vec<Node>> {
        let mut children = vec![];
        while !input.is_empty() {
            if input.peek(syn::token::Lt) && input.peek2(syn::token::Slash) {
                break;
            }

            children.push(self.parse_node(input)?);
        }

        Ok(children)
    }

    /// Parse something like `{ a b }`
    fn parse_braced_fragment(&self, input: ParseStream) -> syn::Result<Node> {
        let content;
        let _brace_token = syn::braced!(content in input);

        let mut nodes = vec![];
        while !content.is_empty() {
            nodes.push(self.parse_node(&content)?);
        }

        if nodes.len() == 1 {
            Ok(nodes.into_iter().next().unwrap())
        } else {
            Ok(Node::Fragment(nodes))
        }
    }

    fn parse_if(&self, input: ParseStream) -> syn::Result<Match> {
        input.parse::<syn::token::If>()?;
        let expr = syn::Expr::parse_without_eager_brace(input)?;

        let then_branch = self.parse_braced_fragment(input)?;

        let else_branch = if input.peek(syn::token::Else) {
            self.parse_else(input)?
        } else {
            Node::None
        };

        match expr {
            syn::Expr::Let(the_let) => {
                // transform into proper match
                Ok(Match {
                    expr: *the_let.expr,
                    arms: vec![
                        MatchArm {
                            pat: *the_let.pat,
                            node: then_branch,
                        },
                        MatchArm {
                            pat: syn::parse_quote! { _ },
                            node: else_branch,
                        },
                    ],
                })
            }
            expr => Ok(Match {
                expr,
                arms: vec![
                    MatchArm {
                        pat: syn::parse_quote! { true },
                        node: then_branch,
                    },
                    MatchArm {
                        pat: syn::parse_quote! { false },
                        node: else_branch,
                    },
                ],
            }),
        }
    }

    fn parse_else(&self, input: ParseStream) -> syn::Result<Node> {
        input.parse::<syn::token::Else>()?;

        let lookahead = input.lookahead1();

        if input.peek(syn::token::If) {
            Ok(Node::Match(self.parse_if(input)?))
        } else if input.peek(syn::token::Brace) {
            self.parse_braced_fragment(input)
        } else {
            Err(lookahead.error())
        }
    }

    fn parse_match(&self, input: ParseStream) -> syn::Result<Match> {
        input.parse::<syn::token::Match>()?;

        let expr = syn::Expr::parse_without_eager_brace(input)?;
        let mut arms = vec![];

        let content;
        let _brace_token = syn::braced!(content in input);

        while !content.is_empty() {
            arms.push(self.parse_match_arm(&content)?);
        }

        Ok(Match { expr, arms })
    }

    fn parse_match_arm(&self, input: ParseStream) -> syn::Result<MatchArm> {
        // BUG: This does not support OR-patterns
        let pat: syn::Pat = syn::Pat::parse_single(input)?;

        // Guard
        if input.peek(syn::token::If) {
            panic!("Match if-guard not yet supported");
        }

        input.parse::<syn::token::FatArrow>()?;

        let node = self.parse_node(input)?;

        Ok(MatchArm { pat, node })
    }

    fn parse_for(&self, input: ParseStream) -> syn::Result<For> {
        let for_token = input.parse()?;
        let pat = syn::Pat::parse_single(input)?;
        let in_token = input.parse()?;
        let expression = syn::Expr::parse_without_eager_brace(input)?;
        let repeating_node = Box::new(self.parse_braced_fragment(input)?);

        Ok(For {
            for_token,
            pat,
            in_token,
            expression,
            repeating_node,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    fn html_parse(stream: proc_macro2::TokenStream) -> syn::Result<Node> {
        fn parse_html(input: ParseStream) -> syn::Result<Node> {
            Parser.parse_node(input)
        }

        syn::parse::Parser::parse2(parse_html, stream)
    }

    fn html_element<A>(tag_name: &str, attrs_fn: A, children: Vec<Node>) -> Node
    where
        A: Fn(&str) -> Vec<Attr>,
    {
        let attrs = attrs_fn(&tag_name);
        let ident = quote::format_ident!("{tag_name}");
        Node::Element(Element {
            type_path: syn::parse_quote! { #ident },
            attrs,
            children,
        })
    }

    fn fragment(nodes: Vec<Node>) -> Node {
        Node::Fragment(nodes)
    }

    fn text(text: &str) -> Node {
        Node::Text(Text(syn::LitStr::new(
            text,
            proc_macro2::Span::mixed_site(),
        )))
    }

    fn text_var(name: &str) -> Node {
        let ident = quote::format_ident!("{}", name);
        Node::TextExpr(syn::parse_quote! { #ident })
    }

    fn component(type_path: syn::TypePath, attrs: ComponentAttrs) -> Node {
        Node::Component(Component { type_path, attrs })
    }

    #[allow(unused)]
    fn attr(name: &str, value: AttrValue) -> Attr {
        Attr {
            key: AttrKey::Text(quote::format_ident!("{}", name)),
            value,
        }
    }

    #[allow(unused)]
    fn html_attr(tag: &str, name: &str, value: AttrValue) -> Attr {
        Attr {
            key: AttrKey::Text(quote::format_ident!("{name}")),
            value,
        }
    }

    #[test]
    fn parse_empty_element_no_children_not_self_closing() {
        let node: Node = html_parse(quote! {
            <p></p>
        })
        .unwrap();
        assert_eq!(node, html_element("p", |_| vec![], vec![]));
    }

    #[test]
    fn parse_unmatched_closing_tag_fails() {
        let result: syn::Result<Node> = html_parse(quote! {
            <p></q>
        });
        assert!(result.is_err());
    }

    #[test]
    fn parse_empty_element_self_closing() {
        let node: Node = html_parse(quote! {
            <p/>
        })
        .unwrap();
        assert_eq!(node, html_element("p", |_| vec![], vec![]));
    }

    #[test]
    fn parse_empty_component_self_closing() {
        let node: Node = html_parse(quote! {
            <P/>
        })
        .unwrap();

        assert_eq!(
            node,
            component(
                syn::parse_quote! {
                    P
                },
                ComponentAttrs::KeyValue(vec![]),
            )
        );
    }

    #[test]
    fn parse_empty_component_with_path_self_closing() {
        let node: Node = html_parse(quote! {
            <module::P/>
        })
        .unwrap();
        assert_eq!(
            node,
            component(
                syn::parse_quote! {
                    module::P
                },
                ComponentAttrs::KeyValue(vec![])
            )
        );
    }

    #[test]
    fn parse_element_with_children() {
        let node: Node = html_parse(quote! {
            <p>
                <strong>"Strong"</strong>
                "not strong"
            </p>
        })
        .unwrap();
        assert_eq!(
            node,
            html_element(
                "p",
                |_| vec![],
                vec![
                    html_element("strong", |_| vec![], vec![text("Strong")]),
                    text("not strong")
                ]
            )
        );
    }

    #[test]
    fn parse_fragment() {
        let node: Node = html_parse(quote! {
            <>
                <p/>
                <div/>
            </>
        })
        .unwrap();
        assert_eq!(
            node,
            fragment(vec![
                html_element("p", |_| vec![], vec![]),
                html_element("div", |_| vec![], vec![])
            ])
        );
    }

    #[test]
    fn parse_element_with_variable() {
        let node: Node = html_parse(quote! {
            <p>
                {variable}
            </p>
        })
        .unwrap();
        assert_eq!(
            node,
            html_element("p", |_| vec![], vec![text_var("variable")])
        );
    }

    #[test]
    fn parse_element_with_attrs() {
        let node: Node = html_parse(quote! {
            <p controls class="b" dir=42 id={foo} />
        })
        .unwrap();
        assert_eq!(
            node,
            html_element(
                "p",
                |tag| vec![
                    html_attr(tag, "controls", AttrValue::ImplicitTrue),
                    html_attr(tag, "class", AttrValue::Literal(syn::parse_quote! { "b" })),
                    html_attr(tag, "dir", AttrValue::Literal(syn::parse_quote! { 42 })),
                    html_attr(tag, "id", AttrValue::Block(syn::parse_quote! { foo })),
                ],
                vec![]
            )
        );
    }

    #[test]
    fn parse_if() {
        let node: Node = html_parse(quote! {
            <div>
                if something {
                    <p />
                    <span />
                }
            </div>
        })
        .unwrap();
        assert_eq!(
            node,
            html_element(
                "div",
                |_| vec![],
                vec![Node::Match(Match {
                    expr: syn::parse_quote! { something },
                    arms: vec![
                        MatchArm {
                            pat: syn::parse_quote! { true },
                            node: fragment(vec![
                                html_element("p", |_| vec![], vec![]),
                                html_element("span", |_| vec![], vec![])
                            ])
                        },
                        MatchArm {
                            pat: syn::parse_quote! { false },
                            node: fragment(vec![]),
                        }
                    ],
                })]
            )
        );
    }

    #[test]
    fn parse_if_let() {
        let node: Node = html_parse(quote! {
            <div>
                if let Some(for_sure) = maybe {
                    <p>{for_sure}</p>
                }
            </div>
        })
        .unwrap();
        assert_eq!(
            node,
            html_element(
                "div",
                |_| vec![],
                vec![Node::Match(Match {
                    expr: syn::parse_quote! { maybe },
                    arms: vec![
                        MatchArm {
                            pat: syn::parse_quote! { Some(for_sure) },
                            node: html_element("p", |_| vec![], vec![text_var("for_sure")])
                        },
                        MatchArm {
                            pat: syn::parse_quote! { _ },
                            node: fragment(vec![]),
                        }
                    ],
                })]
            )
        );
    }

    #[test]
    fn parse_for() {
        let node: Node = html_parse(quote! {
            <ul>
                for item in items {
                    <li>{item}</li>
                }
            </ul>
        })
        .unwrap();
        assert_eq!(
            node,
            html_element(
                "ul",
                |_| vec![],
                vec![Node::For(For {
                    for_token: syn::parse_quote! { for },
                    pat: syn::parse_quote! { item },
                    in_token: syn::parse_quote! { in },
                    expression: syn::parse_quote! { items },
                    repeating_node: Box::new(html_element(
                        "li",
                        |_| vec![],
                        vec![text_var("item")]
                    )),
                })]
            )
        );
    }
}
