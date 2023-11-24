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

    pub use autostrata_web::html::*;
}

use platform::*;

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

    div(
        (),
        (
            Text("Hello!"),
            Func(list),
            div(
                (),
                (
                    button(
                        (
                            On::click(move || {
                                clicks_mut.update(|clicks| clicks + 1);
                                show_mut.update(|show| !show);
                            }),
                            On::mouseover(|| {
                                log("mouseover!");
                            }),
                        ),
                        (Text("hide/show"),),
                    ),
                    button(
                        (
                            On::click(move || {
                                clicks_mut.update(|clicks| clicks + 1);
                                yes_mut.update(|yes| !yes);
                            }),
                            On::mouseover(|| {
                                log("mouseover!");
                            }),
                        ),
                        (Text("yes/no"),),
                    ),
                ),
            ),
            div(
                (),
                (span(
                    (),
                    (
                        Text("clicked "),
                        Reactive(move || Format(clicks)),
                        Text(" times"),
                    ),
                ),),
            ),
            div(
                (),
                (Reactive(move || {
                    span(
                        (),
                        (if show.get() {
                            Either::Left(strong((), (Text("PRESENT"),)))
                        } else {
                            Either::Right(())
                        },),
                    )
                }),),
            ),
            div(
                (),
                (Reactive(move || {
                    span(
                        (),
                        (if yes.get() {
                            Either::Left(strong((), (Text("yes"),)))
                        } else {
                            Either::Right(Text("no"))
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
