use kano::{view, Children, Props, View};
use kano_html::*;
use kano_web::{
    web_component::{register_web_component, ComponentConfig, Shadow},
    Web,
};

fn test_comp(_props: impl Props<Attributes>, children: impl Children<Web>) -> impl View<Web> {
    let hello = true;

    view! {
        <section>
            if hello {
                <h1>"This is Kano Web Component!"</h1>
            }
            <button>
                ..children
            </button>
        </section>
    }
}

fn main() {
    register_web_component(
        test_comp,
        ComponentConfig {
            tag_name: "test-comp",
            shadow: Shadow(true),
            superclass: Default::default(),
        },
    );
}
