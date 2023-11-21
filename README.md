# AutoStrata

Yet another experimental GUI framework. Work in progress.

It's a fine-grained reactivity architecture inspired by https://github.com/gbj/tachys, which is probably going to be the next version of https://github.com/leptos-rs/leptos. It draws inspiration from https://github.com/linebender/xilem, but tweaked for fine-grained reactivity.

It has a `Platform` abstraction, and is designed to work on lots of platforms.

The platform API has been designed to not leak types into business logic.

## Trying out the DOM platform

```sh
cd autostrata-dom-demo
cargo install trunk
trunk serve --watch ..
```
