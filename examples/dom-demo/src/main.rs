use autostrata::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn rand_bool() -> bool {
    let ms = js_sys::Date::now() as u128;
    (ms & 0x1) > 0
}

fn poc() -> impl View {
    let alt: Either<&'static str, ()> = Either::Left("Two");
    let opt: Option<i32> = Some(32);

    let (clicks, clicks_mut) = use_state(0);

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
            Element::new(
                "div",
                (),
                (Reactive(move || {
                    Element::new("span", (), (format!("Clicked {} times", clicks.get()),))
                }),),
            ),
            Element::new(
                "button",
                (
                    On::click(move || {
                        log("clicked!");
                        clicks_mut.update(|clicks| clicks + 1);
                    }),
                    On::mouseover(|| {
                        log("mouseover!");
                    }),
                ),
                ("click me",),
            ),
            Element::new(
                "div",
                (),
                (Reactive(move || {
                    Element::new(
                        "span",
                        (),
                        (if rand_bool() {
                            Either::Left(Element::new("strong", (), ("yes",)))
                        } else {
                            Either::Right("no")
                        },),
                    )
                }),),
            ),
            Element::new(
                "div",
                (),
                (Reactive(move || {
                    Element::new(
                        "span",
                        (),
                        (if rand_bool() {
                            Either::Left(Element::new("strong", (), ("PRESENT",)))
                        } else {
                            Either::Right(())
                        },),
                    )
                }),),
            ),
        ),
    )
}

fn main() {
    console_error_panic_hook::set_once();
    autostrata_dom::Dom::hydrate(poc());
}
