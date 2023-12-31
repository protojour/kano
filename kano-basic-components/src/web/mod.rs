use kano::{attr::To, prelude::platform::*, Children, Empty, View};
use kano_html::{attr::*, html, Html5};
use kano_web::Web;

use crate::KBCAttr;

pub fn layout(_: impl Props<Empty>, children: impl Children<Web, Html5>) -> impl View<Web, Html5> {
    view! {
        <html:div>..children</html:div>
    }
}

pub fn paragraph(
    _: impl Props<Empty>,
    children: impl Children<Web, Html5>,
) -> impl View<Web, Html5> {
    view! {
        <html:p>..children</html:p>
    }
}

pub fn strong(_: impl Props<Empty>, children: impl Children<Web, Html5>) -> impl View<Web, Html5> {
    view! {
        <html:strong>..children</html:strong>
    }
}

pub fn button(
    mut props: impl Props<KBCAttr>,
    children: impl Children<Web, Html5>,
) -> impl View<Web, Html5> {
    let_props!({ KBCAttr::OnClick(on_click), KBCAttr::To(to) } = props);

    #[cfg(feature = "web-routing")]
    if let Some(To(location)) = to {
        on_click = Some(on::click(move || {
            kano::history::push(location.clone().into_owned());
        }));
    }

    #[allow(unused_mut)]
    let mut href: Option<kano_html::properties::Property> = None;

    #[cfg(not(feature = "web-routing"))]
    if let Some(To(location)) = to {
        href = Some(kano_html::attr::href(location));
    }

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
        <html:button class={["kbc_button"]} style={button_style} ..href ..on_click>
            ..children
        </html:button>
    }
}

pub fn unordered_list(
    _: impl Props<Empty>,
    children: impl Children<Web, Html5>,
) -> impl View<Web, Html5> {
    view! {
        <html:ul>..children</html:ul>
    }
}

pub fn list_item(
    _: impl Props<Empty>,
    children: impl Children<Web, Html5>,
) -> impl View<Web, Html5> {
    view! {
        <html:li>..children</html:li>
    }
}
