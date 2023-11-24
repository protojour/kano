use autostrata::{reactive::*, view::*, *};

mod platform {
    use autostrata::View;
    use autostrata_web::Web;

    /// These are typically under conditional compilation:
    ///
    /// `#[cfg(feature = "web")]`
    ///
    /// Which specifies that the whole app is compiled for the web.
    pub trait AppView: View<Web> {}

    impl<V: View<Web>> AppView for V {}
}

use platform::AppView;

fn poc() -> impl AppView {
    let (clicks, clicks_mut) = use_state(0);
    let (show, show_mut) = use_state(true);
    let (yes, yes_mut) = use_state(false);

    // let todo = view! {
    //     <div>
    //         "Hello!"
    //         <div>
    //             <button>
    //                 "hide/show"
    //             </button>
    //         </div>
    //         <div>
    //             <button>
    //                 "yes/no"
    //             </button>
    //         </div>
    //         <div>
    //             <span>"clicked " {clicks.get()} " times"</span>
    //         </div>
    //         <div>
    //             <span>
    //                 if show.get() {
    //                     <strong>"Present"</strong>
    //                 }
    //             </span>
    //         </div>
    //         <div>
    //             <span>
    //                 if yes.get() {
    //                     <strong>"Yes"</strong>
    //                 } else {
    //                     "No"
    //                 }
    //             </span>
    //         </div>
    //     </div>
    // };

    Element::new(
        "div",
        (),
        (
            "Hello!",
            Func(list),
            Element::new(
                "div",
                (),
                (
                    Element::new(
                        "button",
                        (
                            On::click(move || {
                                clicks_mut.update(|clicks| clicks + 1);
                                show_mut.update(|show| !show);
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
                            On::click(move || {
                                clicks_mut.update(|clicks| clicks + 1);
                                yes_mut.update(|yes| !yes);
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
                        (if show.get() {
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
                        (if yes.get() {
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

fn list() -> impl AppView {
    view! {
        <ul>
            <li>"One"</li>
            <li>"Two"</li>
            <li>"Three"</li>
        </ul>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    autostrata_web::Web::hydrate(poc);
}
