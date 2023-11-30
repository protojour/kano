use kano::{prelude::platform::*, Empty};
use kano_web::{
    html::{self, div, li, p, ul},
    Web,
};

use crate::KBCProperty;

pub fn layout(_: impl Props<Empty>, children: impl Children<Web>) -> impl View<Web> {
    view! {
        <div>..children</div>
    }
}

pub fn paragraph(_: impl Props<Empty>, children: impl Children<Web>) -> impl View<Web> {
    view! {
        <p>..children</p>
    }
}

pub fn strong(_: impl Props<Empty>, children: impl Children<Web>) -> impl View<Web> {
    view! {
        <html::strong>..children</html::strong>
    }
}

pub fn button(mut props: impl Props<KBCProperty>, children: impl Children<Web>) -> impl View<Web> {
    let_props!({ KBCProperty::OnEvent(on_event) } = props);

    view! {
        <html::button ..on_event>
            ..children
        </html::button>
    }
}

pub fn unordered_list(_: impl Props<Empty>, children: impl Children<Web>) -> impl View<Web> {
    view! {
        <ul>..children</ul>
    }
}

pub fn list_item(_: impl Props<Empty>, children: impl Children<Web>) -> impl View<Web> {
    view! {
        <li>..children</li>
    }
}
