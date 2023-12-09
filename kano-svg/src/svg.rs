use kano::Props;

use crate::{SvgAttribute, SvgElement};

macro_rules! define_element {
    (($fn_name:ident, $tag_name:literal)) => {
        /// The
        #[doc = concat!("`", stringify!($tag_name), "`")]
        /// SVG element.
        #[allow(non_snake_case)]
        pub const fn $fn_name<T: Props<SvgAttribute>, C>(
            props: T,
            children: C,
        ) -> SvgElement<T, C> {
            SvgElement::new($tag_name, props, children)
        }
    };
}

macro_rules! define_elements {
    ($($element_def:tt,)*) => {
        $(define_element!($element_def);)*
    };
}

define_elements!(
    (a, "a"),
    (altGlyph, "altGlyph"),
    (altGlyphDef, "altGlyphDef"),
    (altGlyphItem, "altGlyphItem"),
    (animate, "animate"),
    (animateColor, "animateColor"),
    (animateMotion, "animateMotion"),
    (animateTransform, "animateTransform"),
    (animation, "animation"),
    (audio, "audio"),
    (canvas, "canvas"),
    (circle, "circle"),
    (clipPath, "clipPath"),
    (color_profile, "color-profile"),
    (cursor, "cursor"),
    (defs, "defs"),
    (desc, "desc"),
    (discard, "discard"),
    (ellipse, "ellipse"),
    (feBlend, "feBlend"),
    (feColorMatrix, "feColorMatrix"),
    (feComponentTransfer, "feComponentTransfer"),
    (feComposite, "feComposite"),
    (feConvolveMatrix, "feConvolveMatrix"),
    (feDiffuseLighting, "feDiffuseLighting"),
    (feDisplacementMap, "feDisplacementMap"),
    (feDistantLight, "feDistantLight"),
    (feDropShadow, "feDropShadow"),
    (feFlood, "feFlood"),
    (feFuncA, "feFuncA"),
    (feFuncB, "feFuncB"),
    (feFuncG, "feFuncG"),
    (feFuncR, "feFuncR"),
    (feGaussianBlur, "feGaussianBlur"),
    (feImage, "feImage"),
    (feMerge, "feMerge"),
    (feMergeNode, "feMergeNode"),
    (feMorphology, "feMorphology"),
    (feOffset, "feOffset"),
    (fePointLight, "fePointLight"),
    (feSpecularLighting, "feSpecularLighting"),
    (feSpotLight, "feSpotLight"),
    (feTile, "feTile"),
    (feTurbulence, "feTurbulence"),
    (filter, "filter"),
    (font, "font"),
    (font_face, "font-face"),
    (font_face_format, "font-face-format"),
    (font_face_name, "font-face-name"),
    (font_face_src, "font-face-src"),
    (font_face_uri, "font-face-uri"),
    (foreignObject, "foreignObject"),
    (g, "g"),
    (glyph, "glyph"),
    (glyphRef, "glyphRef"),
    (handler, "handler"),
    (hkern, "hkern"),
    (iframe, "iframe"),
    (image, "image"),
    (line, "line"),
    (linearGradient, "linearGradient"),
    (listener, "listener"),
    (marker, "marker"),
    (mask, "mask"),
    (metadata, "metadata"),
    (missing_glyph, "missing-glyph"),
    (mpath, "mpath"),
    (path, "path"),
    (pattern, "pattern"),
    (polygon, "polygon"),
    (polyline, "polyline"),
    (prefetch, "prefetch"),
    (radialGradient, "radialGradient"),
    (rect, "rect"),
    (script, "script"),
    (set, "set"),
    (solidColor, "solidColor"),
    (stop, "stop"),
    (style, "style"),
    (svg, "svg"),
    (switch, "switch"),
    (symbol, "symbol"),
    (tbreak, "tbreak"),
    (text, "text"),
    (textArea, "textArea"),
    (textPath, "textPath"),
    (title, "title"),
    (tref, "tref"),
    (tspan, "tspan"),
    (unknown, "unknown"),
    (r#use, "use"),
    (video, "video"),
    (view, "view"),
    (vkern, "vkern"),
);
