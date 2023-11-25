#![allow(non_snake_case)]

use autostrata::{platform::Platform, reactive::*, view::*, *};
use strata_uxr::*;

mod platform {
    use autostrata::View;
    use autostrata_web::Web;

    pub type Platform = autostrata_web::Web;

    /// These are typically under conditional compilation:
    ///
    /// `#[cfg(feature = "web")]`
    ///
    /// Which specifies that the whole app is compiled for the web.
    pub trait AppView: View<Web> {}

    impl<V: View<Web>> AppView for V {}
}

use platform::*;

fn main() {
    platform::Platform::run_app(Poc);
}

fn Poc() -> impl AppView {
    let (clicks, clicks_mut) = use_state(0);
    let (show, show_mut) = use_state(true);
    let (yes, yes_mut) = use_state(false);

    view! {
        <layout>
            <paragraph>"Hello!"</paragraph>
            <MyList />
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
            <paragraph>"clicked " {Format(clicks)} " times"</paragraph>
            <paragraph>
                if show.get() {
                    <strong>"PRESENT"</strong>
                }
            </paragraph>
            <paragraph>
                if yes.get() {
                    <strong>"yes"</strong>
                } else {
                    "no"
                }
            </paragraph>
        </layout>
    }
}

fn MyList() -> impl AppView {
    view! {
        <unordered_list>
            <list_item>"One"</list_item>
            <list_item>"Two"</list_item>
            <list_item>"Three"</list_item>
        </unordered_list>
    }
}
