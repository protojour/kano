use kano::{view, Children, Props, View};
use kano_html::*;
use kano_web::{
    web_component::{register_web_component, ComponentConfig, Shadow, WebComponent},
    Web,
};

fn simple_test_comp() -> impl View<Web> {
    let hello = true;

    view! {
        <section>
            if hello {
                <h1>"This is Kano Web Component!"</h1>
            }
            <button>
                <slot />
            </button>
        </section>
    }
}

/// this is closer to the real signature of a component.
/// Some kind of prop forwarder needs to be constructed.
/// A <slot> needs to be injected for children,
/// Work in progress.
fn test_comp2(_props: impl Props<Attributes>, children: impl Children<Web>) -> impl View<Web> {
    view! {
        <div>..children</div>
    }
}

fn main() {
    simple_test_comp.register(ComponentConfig {
        tag_name: "test-comp",
        shadow: Shadow(true),
        superclass: Default::default(),
    });

    register_web_component(
        test_comp2,
        ComponentConfig {
            tag_name: "test-comp2",
            shadow: Shadow(true),
            superclass: Default::default(),
        },
    );
}
