use autostrata::{reactive::*, view::*, *};
use strata_uxr::*;

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

use platform::*;

fn poc() -> impl AppView {
    let (clicks, clicks_mut) = use_state(0);
    let (show, show_mut) = use_state(true);
    let (yes, yes_mut) = use_state(false);

    layout(
        (),
        (
            view! { <paragraph>"Hello!"</paragraph> },
            Func(list),
            view! {
                <paragraph>
                    <button
                        on:click={move || {
                            clicks_mut.update(|clicks| clicks + 1);
                            show_mut.update(|show| !show);
                        }}
                    >
                        "hide/show"
                    </button>
                    <button
                        on:click={move || {
                            clicks_mut.update(|clicks| clicks + 1);
                            yes_mut.update(|yes| !yes);
                        }}
                    >
                        "yes/no"
                    </button>
                </paragraph>
            },
            view! {
                <paragraph>"clicked " {Format(clicks)} " times"</paragraph>
            },
            view! {
                <paragraph>
                    if show.get() {
                        <strong>"PRESENT"</strong>
                    }
                </paragraph>
            },
            view! {
                <paragraph>
                    if yes.get() {
                        <strong>"yes"</strong>
                    } else {
                        "no"
                    }
                </paragraph>
            },
        ),
    )
}

// TODO: should have arguments
fn list() -> impl AppView {
    view! {
        <unordered_list>
            <list_item>"One"</list_item>
            <list_item>"Two"</list_item>
            <list_item>"Three"</list_item>
        </unordered_list>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    autostrata_web::Web::hydrate(poc);
}
