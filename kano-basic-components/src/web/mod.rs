use kano::prelude::platform::*;
use kano_web::{html, Web};

pub fn layout(_attrs: (), children: impl Children<Web>) -> impl View<Web> {
    html::div((), children)
}

pub fn paragraph(_attrs: (), children: impl Children<Web>) -> impl View<Web> {
    html::p((), children)
}

pub fn strong(_attrs: (), children: impl Children<Web>) -> impl View<Web> {
    html::strong((), children)
}

pub fn button(attrs: impl AttrSet<Web>, children: impl Children<Web>) -> impl View<Web> {
    html::button(attrs, children)
}

pub fn unordered_list(_attrs: (), children: impl Children<Web>) -> impl View<Web> {
    html::ul((), children)
}

pub fn list_item(_attrs: (), children: impl Children<Web>) -> impl View<Web> {
    html::li((), children)
}
