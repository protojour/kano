use autostrata::*;

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
            Element::new("button", (On::click(),), ("click me",)),
        ),
    )
}

fn main() {
    console_error_panic_hook::set_once();
    autostrata_dom::Dom::hydrate(poc());
}
