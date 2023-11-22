# AutoStrata

Yet another experimental GUI framework. Work in progress.

It's a fine-grained reactivity architecture inspired by https://github.com/gbj/tachys, which is probably going to be the next version of https://github.com/leptos-rs/leptos. It draws inspiration from https://github.com/linebender/xilem, but tweaked for fine-grained reactivity.

It has a `Platform` abstraction, and is designed to work on lots of platforms.

The platform API has been designed to not leak types into business logic.

## Trying out the DOM platform

```sh
cd examples/dom-demo
cargo install trunk
trunk serve --watch ..
```

## Reactivity
AutoStrata will use a reactivity architecture inspired by https://docs.rs/leptos_reactive/latest/leptos_reactive/.

It works by always evaluating subcribers from within Rust closures while a context is stored in a thread local.
When the subscriber is read the first time, a reactive relationship is automatically registered between the reactive view and the subscriber (a "subscription").

leptos-reactive also has a concept of a reactive dependency tree, where redudant updates get optimized out.
Autostrata can implement something similar, but probably not in the first version.

## Templating language
Let's use [audunhalland/hypp](https://github.com/audunhalland/hypp/blob/main/tests/compile_basic.rs) as inspiration.

This is a markup templating DSL that merges web markup and Rust.
We can just reuse part of its implementation.
AutoStrata will use a _much_ simpler desugaring than hypp, since the intermediate language (Rust) is already a declarative expression that is view-tree-structured.

In this DSL, string literals are always `"quoted"`, so that language keywords are available without any escaping (.e.g. `if`, `for`, `match`).

### Ideas
```rust
fn my_view() -> impl View {
    let (things, things_mut) = use_state(vec![]);

    view! {
        <h2>
            "My list of things"
        </h2>
        <ul>
        for thing in things {
            <li>{thing}</li>
        }
        </ul>
    }
}
```