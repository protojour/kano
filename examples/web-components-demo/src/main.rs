use autostrata::{Element, View};
use autostrata_dom::web_component::{ComponentConfig, Shadow, WebComponent};

fn test_comp() -> impl View {
    Element::new(
        "section",
        (),
        (
            Element::new("h1", (), ("This is AutoStrata Web Component!",)),
            Element::new("button", (), (Element::new("slot", (), ()),)),
        ),
    )
}

fn main() {
    test_comp.register(ComponentConfig {
        tag_name: "test-comp",
        shadow: Shadow(true),
        superclass: Default::default(),
    });
}
