use crate::element::Element;
use crate::props::HtmlProp;
use crate::Web;
use kano::{Diff, Props};

macro_rules! define_element {
    (($ns:expr, $ty_name:ident, $name:ident, $dom_interface:ident)) => {
        /// The
        #[doc = concat!("`", stringify!($name), "`")]
        /// element.
        pub const fn $name<T: Props<HtmlProp> + Diff<Web>, C>(
            props: T,
            children: C,
        ) -> Element<T, C> {
            Element::new(stringify!($name), props, children)
        }
    };
}

macro_rules! define_elements {
    ($($element_def:tt,)*) => {
        $(define_element!($element_def);)*
    };
}

define_elements!(
    // the order is copied from
    // https://developer.mozilla.org/en-US/docs/Web/HTML/Element
    // DOM interfaces copied from https://html.spec.whatwg.org/multipage/grouping-content.html and friends

    // content sectioning
    (HTML_NS, Address, address, HtmlElement),
    (HTML_NS, Article, article, HtmlElement),
    (HTML_NS, Aside, aside, HtmlElement),
    (HTML_NS, Footer, footer, HtmlElement),
    (HTML_NS, Header, header, HtmlElement),
    (HTML_NS, H1, h1, HtmlHeadingElement),
    (HTML_NS, H2, h2, HtmlHeadingElement),
    (HTML_NS, H3, h3, HtmlHeadingElement),
    (HTML_NS, H4, h4, HtmlHeadingElement),
    (HTML_NS, H5, h5, HtmlHeadingElement),
    (HTML_NS, H6, h6, HtmlHeadingElement),
    (HTML_NS, Hgroup, hgroup, HtmlElement),
    (HTML_NS, Main, main, HtmlElement),
    (HTML_NS, Nav, nav, HtmlElement),
    (HTML_NS, Section, section, HtmlElement),
    // text content
    (HTML_NS, Blockquote, blockquote, HtmlQuoteElement),
    (HTML_NS, Dd, dd, HtmlElement),
    (HTML_NS, Div, div, HtmlDivElement),
    (HTML_NS, Dl, dl, HtmlDListElement),
    (HTML_NS, Dt, dt, HtmlElement),
    (HTML_NS, Figcaption, figcaption, HtmlElement),
    (HTML_NS, Figure, figure, HtmlElement),
    (HTML_NS, Hr, hr, HtmlHrElement),
    (HTML_NS, Li, li, HtmlLiElement),
    (HTML_NS, Link, link, HtmlLinkElement),
    (HTML_NS, Menu, menu, HtmlMenuElement),
    (HTML_NS, Ol, ol, HtmlOListElement),
    (HTML_NS, P, p, HtmlParagraphElement),
    (HTML_NS, Pre, pre, HtmlPreElement),
    (HTML_NS, Ul, ul, HtmlUListElement),
    // inline text
    (HTML_NS, A, a, HtmlAnchorElement),
    (HTML_NS, Abbr, abbr, HtmlElement),
    (HTML_NS, B, b, HtmlElement),
    (HTML_NS, Bdi, bdi, HtmlElement),
    (HTML_NS, Bdo, bdo, HtmlElement),
    (HTML_NS, Br, br, HtmlBrElement),
    (HTML_NS, Cite, cite, HtmlElement),
    (HTML_NS, Code, code, HtmlElement),
    (HTML_NS, Data, data, HtmlDataElement),
    (HTML_NS, Dfn, dfn, HtmlElement),
    (HTML_NS, Em, em, HtmlElement),
    (HTML_NS, I, i, HtmlElement),
    (HTML_NS, Kbd, kbd, HtmlElement),
    (HTML_NS, Mark, mark, HtmlElement),
    (HTML_NS, Q, q, HtmlQuoteElement),
    (HTML_NS, Rp, rp, HtmlElement),
    (HTML_NS, Rt, rt, HtmlElement),
    (HTML_NS, Ruby, ruby, HtmlElement),
    (HTML_NS, S, s, HtmlElement),
    (HTML_NS, Samp, samp, HtmlElement),
    (HTML_NS, Small, small, HtmlElement),
    (HTML_NS, Span, span, HtmlSpanElement),
    (HTML_NS, Strong, strong, HtmlElement),
    (HTML_NS, Sub, sub, HtmlElement),
    (HTML_NS, Sup, sup, HtmlElement),
    (HTML_NS, Time, time, HtmlTimeElement),
    (HTML_NS, U, u, HtmlElement),
    (HTML_NS, Var, var, HtmlElement),
    (HTML_NS, Wbr, wbr, HtmlElement),
    // image and multimedia
    (HTML_NS, Area, area, HtmlAreaElement),
    (HTML_NS, Audio, audio, HtmlAudioElement),
    (HTML_NS, Canvas, canvas, HtmlCanvasElement),
    (HTML_NS, Img, img, HtmlImageElement),
    (HTML_NS, Map, map, HtmlMapElement),
    (HTML_NS, Track, track, HtmlTrackElement),
    (HTML_NS, Video, video, HtmlVideoElement),
    // embedded content
    (HTML_NS, Embed, embed, HtmlEmbedElement),
    (HTML_NS, Iframe, iframe, HtmlIFrameElement),
    (HTML_NS, Object, object, HtmlObjectElement),
    (HTML_NS, Picture, picture, HtmlPictureElement),
    (HTML_NS, Portal, portal, HtmlElement),
    (HTML_NS, Source, source, HtmlSourceElement),
    // scripting
    (HTML_NS, Noscript, noscript, HtmlElement),
    (HTML_NS, Script, script, HtmlScriptElement),
    // demarcating edits
    (HTML_NS, Del, del, HtmlModElement),
    (HTML_NS, Ins, ins, HtmlModElement),
    // tables
    (HTML_NS, Caption, caption, HtmlTableCaptionElement),
    (HTML_NS, Col, col, HtmlTableColElement),
    (HTML_NS, Colgroup, colgroup, HtmlTableColElement),
    (HTML_NS, Table, table, HtmlTableElement),
    (HTML_NS, Tbody, tbody, HtmlTableSectionElement),
    (HTML_NS, Td, td, HtmlTableCellElement),
    (HTML_NS, Tfoot, tfoot, HtmlTableSectionElement),
    (HTML_NS, Th, th, HtmlTableCellElement),
    (HTML_NS, Thead, thead, HtmlTableSectionElement),
    (HTML_NS, Tr, tr, HtmlTableRowElement),
    // forms
    (HTML_NS, Button, button, HtmlButtonElement),
    (HTML_NS, Datalist, datalist, HtmlDataListElement),
    (HTML_NS, Fieldset, fieldset, HtmlFieldSetElement),
    (HTML_NS, Form, form, HtmlFormElement),
    (HTML_NS, Input, input, HtmlInputElement),
    (HTML_NS, Label, label, HtmlLabelElement),
    (HTML_NS, Legend, legend, HtmlLegendElement),
    (HTML_NS, Meter, meter, HtmlMeterElement),
    (HTML_NS, Optgroup, optgroup, HtmlOptGroupElement),
    (HTML_NS, OptionElement, option, HtmlOptionElement), // Avoid cluttering the namespace with `Option`
    (HTML_NS, Output, output, HtmlOutputElement),
    (HTML_NS, Progress, progress, HtmlProgressElement),
    (HTML_NS, Select, select, HtmlSelectElement),
    (HTML_NS, Textarea, textarea, HtmlTextAreaElement),
    // interactive elements,
    (HTML_NS, Details, details, HtmlDetailsElement),
    (HTML_NS, Dialog, dialog, HtmlDialogElement),
    (HTML_NS, Summary, summary, HtmlElement),
    // web components,
    (HTML_NS, Slot, slot, HtmlSlotElement),
    (HTML_NS, Template, template, HtmlTemplateElement),
    // SVG and MathML (TODO, svg and mathml elements)
    (SVG_NS, Svg, svg, SvgElement),
    (MATHML_NS, Math, math, Element),
);
