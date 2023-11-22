use autostrata::{view, View};
use autostrata_dom::web_component::{ComponentConfig, Shadow, WebComponent};

fn test_comp() -> impl View {
    view!(
        <section>
            <h1>"This is AutoStrata Web Component!"</h1>
            <button>
                <slot />
            </button>
        </section>
    )
}

fn main() {
    test_comp.register(ComponentConfig {
        tag_name: "test-comp",
        shadow: Shadow(true),
        superclass: Default::default(),
    });
}
