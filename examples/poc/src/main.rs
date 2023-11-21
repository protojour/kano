use autostrata::{dom::Dom, Element, View};

fn poc() -> impl View {
    let opt: Option<i32> = Some(32);

    Element::new(
        "div",
        (),
        (
            "Hello!",
            Element::new(
                "ul",
                (),
                (
                    Element::new("li", (), ("One",)),
                    opt.map(|_| Element::new("li", (), ("Two",))),
                    Element::new("li", (), ("Three",)),
                ),
            ),
        ),
    )
}

fn main() {
    Dom::hydrate(poc());
}
