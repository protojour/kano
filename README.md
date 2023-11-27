# Kano

Yet another experimental, work-in-progress GUI application framework written for and in Rust.

It's a fine-grained reactivity architecture that is explicitly platform agnostic and _component-library-first_.

Kano elevates component libraries to act as the main platform abstraction.

## Hello world!
```rust
use kano::prelude::*;

/// Two important definitions are generated here:
/// `AppPlatform` is a type alias for the current platform.
/// `View` is a trait that all views of this application must implement.
kano::define_platform!(AppPlatform, View);

fn main() {
    AppPlatform::run_app(HelloWorld);
}

fn HelloWorld() -> impl View {
    "Hello world!"
}
```

## Trying out the Web platform

```sh
cargo install trunk

(cd examples/demo/; trunk serve --watch ../.. --features web)
```

## kano-basic-components
This is a very basic platform-agnostic UI component library.

## Reactivity
Kano will use a reactivity architecture inspired by https://docs.rs/leptos_reactive/latest/leptos_reactive/.

It works by always evaluating subcribers from within Rust closures while a context is stored in a thread local.
When the subscriber is read the first time, a reactive relationship is automatically registered between the reactive view and the subscriber (a "subscription").

## Templating language
Let's use [audunhalland/hypp](https://github.com/audunhalland/hypp/blob/main/tests/compile_basic.rs) as inspiration.

This is a markup templating DSL that merges web markup and Rust.
We can just reuse part of its implementation.
Kano will use a _much_ simpler desugaring than hypp, since the intermediate language (Rust) is already a declarative expression that is view-tree-structured.

In this DSL, string literals are always `"quoted"`, so that language keywords are available without any escaping (.e.g. `if`, `for`, `match`).

## Acknowledgements
This project builds on a lot of ideas from great people.

* [leptos](https://github.com/leptos-rs/leptos)
* [xilem](https://github.com/linebender/xilem)
* [svelte](https://svelte.dev)
* [hypp](https://github.com/audunhalland/hypp)
