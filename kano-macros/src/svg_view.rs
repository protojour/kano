use std::{
    env,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use proc_macro2::Span;
use quote::quote;
use xml::{
    attribute::OwnedAttribute,
    name::OwnedName,
    reader::{Events, XmlEvent},
    EventReader,
};

pub fn svg_view(svg_path: PathBuf) -> proc_macro2::TokenStream {
    let mut path: PathBuf = env::var("CARGO_MANIFEST_DIR").unwrap().into();
    path.extend(&svg_path);

    let svg_node = parse_svg(&path);

    let node = generate_svg_view_node(svg_node, true);

    quote! {
        {
            use ::kano_svg::{*, attr::*};
            ::kano::view! { #node }
        }
    }
}

fn generate_svg_view_node(node: SvgNode, root: bool) -> proc_macro2::TokenStream {
    match node {
        SvgNode::Element {
            name,
            attributes,
            children,
        } => {
            let prefix = if root {
                Some(quote! { kano_svg:svg: })
            } else {
                None
            };

            let mut out = proc_macro2::TokenStream::default();

            let tag_ident = quote::format_ident!("{}", name.local_name);
            out.extend(quote! { <#prefix #tag_ident });

            for attr in attributes {
                let prefix = attr
                    .name
                    .prefix
                    .map(|prefix| quote::format_ident!("{prefix}"))
                    .map(|prefix| quote! { #prefix: });

                let attr_ident = quote::format_ident!("{}", attr.name.local_name.replace('-', "_"));
                let value = syn::LitStr::new(&attr.value, Span::call_site());

                out.extend(quote! {
                    #prefix #attr_ident = #value
                });
            }

            if children.is_empty() {
                out.extend(quote! { /> });
            } else {
                out.extend(quote! { > });

                for child in children {
                    out.extend(generate_svg_view_node(child, false));
                }

                out.extend(quote! { </#prefix #tag_ident> });
            }

            out
        }
        SvgNode::Text(text) => {
            let lit = syn::LitStr::new(&text, Span::call_site());
            quote! { #lit }
        }
    }
}

fn parse_svg(path: &Path) -> SvgNode {
    let file = File::open(path).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);

    let mut it = parser.into_iter();

    let nodes = parse_nodes(&mut it);

    if nodes.len() == 1 {
        nodes.into_iter().next().unwrap()
    } else {
        panic!("No SVG node found");
    }
}

enum SvgNode {
    Element {
        name: OwnedName,
        attributes: Vec<OwnedAttribute>,
        // namespace: Namespace,
        children: Vec<SvgNode>,
    },
    Text(String),
}

fn parse_nodes<R: std::io::Read>(it: &mut Events<R>) -> Vec<SvgNode> {
    let mut nodes = vec![];

    while let Some(event) = it.next() {
        match event {
            Ok(XmlEvent::StartElement {
                name,
                attributes,
                namespace: _,
            }) => {
                let children = parse_nodes(it);

                nodes.push(SvgNode::Element {
                    name,
                    attributes,
                    children,
                });
            }
            Ok(XmlEvent::EndElement { name: _ }) => {
                return nodes;
            }
            Ok(XmlEvent::Characters(string)) => {
                nodes.push(SvgNode::Text(string));
            }
            _ => {}
        }
    }

    nodes
}
