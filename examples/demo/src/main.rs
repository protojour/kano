#![allow(non_snake_case)]

use autostrata::prelude::*;
use strata_uxr::*;
use todo::{add_todo, delete_todo, Todo};

mod todo;

autostrata::define_platform!(AppPlatform, View);

fn main() {
    AppPlatform::run_app(App).unwrap();
}

fn App() -> impl View {
    let clicks = use_state(|| 0);
    let show = use_state(|| true);
    let yes = use_state(|| false);

    let todos = use_state(|| {
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
                        todos.update(|todos| {
                            add_todo(todos, None);
                        });
                    }}
                >
                    "append item"
                </button>
            </paragraph>
            <TodoList {todos} />
            <paragraph>
                <button
                    on:click={move || {
                        clicks.update(|clicks| *clicks += 1);
                        show.toggle();
                    }}
                >
                    "hide/show"
                </button>
                <button
                    on:click={move || {
                        clicks.update(|clicks| *clicks += 1);
                        yes.toggle();
                    }}
                >
                    "yes/no"
                </button>
            </paragraph>
            <paragraph>"clicked " {Fmt(clicks)} " times"</paragraph>
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

fn TodoList(todos: State<Vec<Todo>>) -> impl View {
    let delete = move |id: usize| {
        todos.update(|todos| {
            delete_todo(todos, id);
        });
    };

    view! {
        <unordered_list>
        for Todo { id, text } in todos.get_ref() {
            <list_item>
                {text.clone()}

                " ("<button on:click={move || delete(id)}>"x"</button>")"
            </list_item>
        }
        </unordered_list>
    }
}
