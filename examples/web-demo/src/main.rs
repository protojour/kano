use autostrata::{reactive::*, view::*, *};

fn poc() -> impl View {
    let alt: Either<&'static str, ()> = Either::Left("Two");

    let (clicks, clicks_mut) = use_state(0);
    let (show, show_mut) = use_state(true);
    let (yes, yes_mut) = use_state(false);

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
                    Element::new("li", (), (alt,)),
                    Element::new("li", (), ("Three",)),
                ),
            ),
            Element::new(
                "div",
                (),
                (
                    Element::new(
                        "button",
                        (
                            On::click({
                                let clicks_mut = clicks_mut.clone();
                                move || {
                                    clicks_mut.update(|clicks| clicks + 1);
                                    show_mut.update(|show| !show);
                                }
                            }),
                            On::mouseover(|| {
                                log("mouseover!");
                            }),
                        ),
                        ("hide/show",),
                    ),
                    Element::new(
                        "button",
                        (
                            On::click({
                                let clicks_mut = clicks_mut.clone();
                                move || {
                                    clicks_mut.update(|clicks| clicks + 1);
                                    yes_mut.update(|yes| !yes);
                                }
                            }),
                            On::mouseover(|| {
                                log("mouseover!");
                            }),
                        ),
                        ("yes/no",),
                    ),
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
                "div",
                (),
                (Reactive(move || {
                    Element::new(
                        "span",
                        (),
                        (if *show.get() {
                            Either::Left(Element::new("strong", (), ("PRESENT",)))
                        } else {
                            Either::Right(())
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
                        (if *yes.get() {
                            Either::Left(Element::new("strong", (), ("yes",)))
                        } else {
                            Either::Right("no")
                        },),
                    )
                }),),
            ),
        ),
    )
}

fn main() {
    console_error_panic_hook::set_once();
    autostrata_web::Web::hydrate(poc());
}
