#![allow(non_snake_case)]

use autostrata::prelude::*;
use strata_uxr::*;
use todo::{add_todo, delete_todo, Todo};

mod todo;

autostrata::define_platform!(AppPlatform, View);

fn main() {
    AppPlatform::run_app(App);
}

fn App() -> impl View {
    let (clicks, clicks_mut) = use_state(|| 0);
    let (show, show_mut) = use_state(|| true);
    let (yes, yes_mut) = use_state(|| false);

    let (todos, todos_mut) = use_state(|| {
        let mut todos = vec![];
        add_todo(&mut todos, Some("First".to_string()));
        add_todo(&mut todos, Some("Second".to_string()));

        todos
    });

    view! {
        <layout>
            <paragraph>"Hello!"</paragraph>
            <paragraph>
                <button
                    on:click={move || {
                        todos_mut.update(|todos| {
                            add_todo(todos, None);
                        });
                    }}
                >
                    "append item"
                </button>
            </paragraph>
            <TodoList {todos.get_ref()} {todos_mut} />
            <paragraph>
                <button
                    on:click={move || {
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

fn TodoList(todos: Ref<Vec<Todo>>, todos_mut: StateMut<Vec<Todo>>) -> impl View {
    let delete = move |id: usize| {
        todos_mut.update(|todos| {
            delete_todo(todos, id);
        });
    };

    view! {
        <unordered_list>
        for Todo { id, text } in todos {
            <list_item>
                {Format::new(text.clone())}

                " ("<button on:click={move || delete(id)}>"x"</button>")"
            </list_item>
        }
        </unordered_list>
    }
}
