use autostrata::{on_click, Diff, Element, View};

fn static_text() -> impl View {
    ""
}

fn static_element() -> impl View {
    Element::new(
        "div",
        (),
        ("Yo!", Element::new("span", (on_click(),), ("text",))),
    )
}
