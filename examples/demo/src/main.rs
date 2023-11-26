#![allow(non_snake_case)]

use autostrata::{prelude::*, view::Map};
use strata_uxr::*;

autostrata::define_platform!(AppPlatform, View);

fn main() {
    AppPlatform::run_app(App);
}

fn App() -> impl View {
    let (clicks, clicks_mut) = use_state(|| 0);
    let (show, show_mut) = use_state(|| true);
    let (yes, yes_mut) = use_state(|| false);

    let (items, items_mut) =
        use_state(|| vec!["One".to_string(), "Two".to_string(), "Three".to_string()]);

    view! {
        <layout>
            <paragraph>"Hello!"</paragraph>
            <StringList {items.get_ref()} />
            <paragraph>
                <button
                    on:click={move || {
                        items_mut.update(|items| items.push(
                            "New item".to_string()
                        ));

                        clicks_mut.update(|clicks| *clicks += 1);
                        show_mut.update(|show| *show = !*show);
                    }}
                >
                    "hide/show"
                </button>
                <button
                    on:click={move || {
                        clicks_mut.update(|clicks| *clicks += 1);
                        yes_mut.update(|yes| *yes = !*yes);
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

fn StringList(items: Ref<Vec<String>>) -> impl View {
    unordered_list(
        (),
        (autostrata::view::Seq(items).seq_map(|item| {
            view! {
                <list_item>
                    {Format(item.clone())}
                </list_item>
            }
        }),),
    )
}
