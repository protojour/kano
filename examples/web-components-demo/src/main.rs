use kano::property::on;
use kano::{let_props, reactive::use_state, view, Children, DeserializeAttribute, Props, View};
use kano_html::{attr::style, html};
use kano_web::{
    web_component::{register_web_component, ComponentConfig, Shadow},
    Web,
};

enum Attributes {
    ShowHeading(bool),
}

impl DeserializeAttribute for Attributes {
    fn describe(names: &mut Vec<&'static str>) {
        names.push("show_heading");
    }

    fn deserialize(name: &str, _value: String) -> Option<Self> {
        match name {
            "show_heading" => Some(Self::ShowHeading(true)),
            _ => None,
        }
    }
}

fn test_comp(mut props: impl Props<Attributes>, children: impl Children<Web>) -> impl View<Web> {
    let_props!({ Attributes::ShowHeading(show_heading) } = props);

    let style_select = use_state(|| false);

    let section_style = "
        padding: 3px;
        margin: 3px;
        background-color: #a2a8d3;
    ";

    let other_section_style = "
        padding: 3px;
        margin: 3px;
        background-color: #e7eaf6;
    ";

    view! {
        <html::section style={if style_select.get() { other_section_style } else { section_style }}>
            if show_heading.unwrap_or(false) {
                <h1>"This is Kano Web Component!"</h1>
            }
            <button on:click={move || style_select.toggle()}>
                ..children
            </button>
        </html::section>
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
