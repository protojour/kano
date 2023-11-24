use crate::element::Element;

pub const fn div<A, C>(attrs: A, children: C) -> Element<A, C> {
    Element::new("div", attrs, children)
}

pub const fn span<A, C>(attrs: A, children: C) -> Element<A, C> {
    Element::new("span", attrs, children)
}

pub const fn strong<A, C>(attrs: A, children: C) -> Element<A, C> {
    Element::new("strong", attrs, children)
}

pub const fn button<A, C>(attrs: A, children: C) -> Element<A, C> {
    Element::new("button", attrs, children)
}

pub const fn slot<A, C>(attrs: A, children: C) -> Element<A, C> {
    Element::new("slot", attrs, children)
}

pub const fn ul<A, C>(attrs: A, children: C) -> Element<A, C> {
    Element::new("ul", attrs, children)
}

pub const fn li<A, C>(attrs: A, children: C) -> Element<A, C> {
    Element::new("li", attrs, children)
}

pub const fn section<A, C>(attrs: A, children: C) -> Element<A, C> {
    Element::new("section", attrs, children)
}

pub const fn h1<A, C>(attrs: A, children: C) -> Element<A, C> {
    Element::new("h1", attrs, children)
}
