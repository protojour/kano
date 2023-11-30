use kano::{prelude::platform::*, Empty};
use kano_web::{html, Web};

use crate::KBCProperty;

pub fn layout(_: impl Props<Empty>, children: impl Children<Web>) -> impl View<Web> {
    html::div([], children)
}

pub fn paragraph(_: impl Props<Empty>, children: impl Children<Web>) -> impl View<Web> {
    html::p([], children)
}

pub fn strong(_: impl Props<Empty>, children: impl Children<Web>) -> impl View<Web> {
    html::strong([], children)
}

pub fn button(mut props: impl Props<KBCProperty>, children: impl Children<Web>) -> impl View<Web> {
    let on_event = props.cond_take(|prop| {
        let KBCProperty::OnEvent(on_event) = prop;
        Ok(on_event)
    });

    html::button([kano::Attribute::into_prop(on_event)], children)
}

pub fn unordered_list(_: impl Props<Empty>, children: impl Children<Web>) -> impl View<Web> {
    html::ul([], children)
}

pub fn list_item(_: impl Props<Empty>, children: impl Children<Web>) -> impl View<Web> {
    html::li([], children)
}
