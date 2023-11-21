use autostrata::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

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
            Element::new(
                "button",
                (
                    On::click(|| {
                        log("clicked!");
                    }),
                    On::mouseover(|| {
                        log("mouseover!");
                    }),
                ),
                ("click me",),
            ),
        ),
    )
}

fn main() {
    console_error_panic_hook::set_once();
    autostrata_dom::Dom::hydrate(poc());
}
