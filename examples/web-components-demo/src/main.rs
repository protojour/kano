use kano::{let_props, view, Children, DeserializeAttribute, Props, View};
use kano_html::{attr::style, *};
use kano_web::{
    web_component::{register_web_component, ComponentConfig, Shadow},
    Web,
};

enum Attributes {
    ShowHeading(()),
}

impl DeserializeAttribute for Attributes {
    fn describe(names: &mut Vec<&'static str>) {
        names.push("show_heading");
    }

    fn deserialize(name: &str, _value: String) -> Option<Self> {
        match name {
            "show_heading" => Some(Self::ShowHeading(())),
            _ => None,
        }
    }
}

fn test_comp(mut props: impl Props<Attributes>, children: impl Children<Web>) -> impl View<Web> {
    let_props!({ Attributes::ShowHeading(show_heading) } = props);

    let section_style = "
        cursor: pointer;
        padding: 3px;
        margin: 3px;
        position: relative;
        background-color: #a2a8d3;
        text-decoration: none;
        z-index: 1;
        font-family: inherit;
    ";

    view! {
        <section style={section_style}>
            if show_heading.is_some() {
                <h1>"This is Kano Web Component!"</h1>
            }
            <button>
                ..children
            </button>
        </section>
    }
}

fn main() {
    kano::init::<Web>();

    register_web_component(
        test_comp,
        ComponentConfig {
            tag_name: "test-comp",
            shadow: Shadow(true),
            superclass: Default::default(),
        },
    );
}
