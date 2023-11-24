use crate::element::Element;

pub const fn div<A, C>(attrs: A, children: C) -> Element<A, C> {
    Element {
        name: "div",
        attrs,
        children,
    }
}

pub const fn span<A, C>(attrs: A, children: C) -> Element<A, C> {
    Element {
        name: "span",
        attrs,
        children,
    }
}

pub const fn strong<A, C>(attrs: A, children: C) -> Element<A, C> {
    Element {
        name: "strong",
        attrs,
        children,
    }
}

pub const fn button<A, C>(attrs: A, children: C) -> Element<A, C> {
    Element {
        name: "button",
        attrs,
        children,
    }
}
