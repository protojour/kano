use kano::{prelude::platform::*, Empty};
use kano_html::{
    self as html,
    attr::{class, style},
    div, li, p, ul,
};
use kano_web::Web;

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
    let_props!({ KBCProperty::OnClick(on_click) } = props);

    let button_style = "
        border: 2px solid rgb(0, 70, 100);
        cursor: pointer;
        position: relative;
        background-color: transparent;
        text-decoration: none;
        z-index: 1;
        font-family: inherit;
    ";

    view! {
        <html::button class={["kbc_button"]} style={button_style} ..on_click>
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
