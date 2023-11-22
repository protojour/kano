use std::time::Instant;

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
    let alt: Either<&'static str, ()> = Either::Left("Two");
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
                    opt.map(|_| Element::new("li", (), (alt,))),
                    Element::new("li", (), ("Three",)),
                ),
            ),
            Reactive(move || {
                Element::new(
                    "span",
                    (),
                    (if (Instant::now().elapsed().as_millis() & 0x1) > 0 {
                        Either::Left(Element::new("strong", (), ("yes",)))
                    } else {
                        Either::Right("no")
                    },),
                )
            }),
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
