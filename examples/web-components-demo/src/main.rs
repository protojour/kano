use kano::{view, View};
use kano_web::{
    html::*,
    web_component::{ComponentConfig, Shadow, WebComponent},
    Web,
};

fn test_comp() -> impl View<Web> {
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

fn main() {
    test_comp.register(ComponentConfig {
        tag_name: "test-comp",
        shadow: Shadow(true),
        superclass: Default::default(),
    });
}
