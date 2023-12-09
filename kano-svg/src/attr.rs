use std::borrow::Cow;

use crate::properties::{Property, PropertyValue, Strings};
use crate::SvgAttribute;

macro_rules! define_attr {
    (($ident:ident, $name:literal, $idl:literal, STRING)) => {
        /// The
        #[doc = concat!("`", stringify!($name), "`")]
        /// SVG attribute.
        #[allow(non_snake_case)]
        pub fn $ident(value: impl Into<Cow<'static, str>>) -> SvgAttribute {
            SvgAttribute::Svg(Property::new($name, PropertyValue::String(value.into())))
        }
    };
    (($ident:ident, $name:literal, $idl:literal, NUMBER)) => {
        /// The
        #[doc = concat!("`", stringify!($name), "`")]
        /// attribute.
        #[allow(non_snake_case)]
        pub fn $ident(value: i32) -> SvgAttribute {
            SvgAttribute::Svg(Property::new($name, PropertyValue::Number(value)))
        }
    };
    (($ident:ident, $name:literal, $idl:literal, BOOL)) => {
        /// The
        #[doc = concat!("`", stringify!($name), "`")]
        /// attribute.
        #[allow(non_snake_case)]
        pub fn $ident(value: bool) -> SvgAttribute {
            SvgAttribute::Svg(Property::new($name, PropertyValue::Bool(value)))
        }
    };
    (($ident:ident, $name:literal, $idl:literal, SPACE_SEP)) => {
        /// The
        #[doc = concat!("`", stringify!($name), "`")]
        /// attribute.
        #[allow(non_snake_case)]
        pub fn $ident(value: impl Into<Strings>) -> SvgAttribute {
            SvgAttribute::Svg(Property::new(
                $name,
                PropertyValue::SpaceSep(value.into().0),
            ))
        }
    };
    (($ident:ident, $name:literal, $idl:literal, COMMA_SEP)) => {
        /// The
        #[doc = concat!("`", stringify!($name), "`")]
        /// attribute.
        #[allow(non_snake_case)]
        pub fn $ident(value: impl Into<Strings>) -> SvgAttribute {
            SvgAttribute::Svg(Property::new(
                $name,
                PropertyValue::CommaSep(value.into().0),
            ))
        }
    };
    (($ident:ident, $name:literal, $idl:literal, COMMA_SEP | STRING)) => {
        define_attr!(($ident, $name, $idl, COMMA_SEP));
    };
    (($ident:ident, $name:literal, $idl:literal, COMMA_SEP | SPACE_SEP)) => {
        define_attr!(($ident, $name, $idl, COMMA_SEP));
    };
}

macro_rules! define_attrs {
    ($($attr_def:tt,)*) => {
        $(define_attr!($attr_def);)*
    };
}

define_attrs!(
    (about, "about", "about", COMMA_SEP | SPACE_SEP),
    (accent, "accent-height", "accentHeight", NUMBER),
    (accumulate, "accumulate", "accumulate", STRING),
    (additive, "additive", "additive", STRING),
    (alignment, "alignment-baseline", "alignmentBaseline", STRING),
    (alphabetic, "alphabetic", "alphabetic", NUMBER),
    (amplitude, "amplitude", "amplitude", NUMBER),
    (arabic, "arabic-form", "arabicForm", STRING),
    (ascent, "ascent", "ascent", NUMBER),
    (attributeName, "attributeName", "attributeName", STRING),
    (attributeType, "attributeType", "attributeType", STRING),
    (azimuth, "azimuth", "azimuth", NUMBER),
    (bandwidth, "bandwidth", "bandwidth", STRING),
    (baseline, "baseline-shift", "baselineShift", STRING),
    (baseFrequency, "baseFrequency", "baseFrequency", STRING),
    (baseProfile, "baseProfile", "baseProfile", STRING),
    (bbox, "bbox", "bbox", STRING),
    (begin, "begin", "begin", STRING),
    (bias, "bias", "bias", NUMBER),
    (by, "by", "by", STRING),
    (calcMode, "calcMode", "calcMode", STRING),
    (cap_height, "cap-height", "capHeight", NUMBER),
    (class, "class", "className", SPACE_SEP),
    (clip, "clip", "clip", STRING),
    (clip_path, "clip-path", "clipPath", STRING),
    (clipPathUnits, "clipPathUnits", "clipPathUnits", STRING),
    (clip_rule, "clip-rule", "clipRule", STRING),
    (color, "color", "color", STRING),
    (
        color_interpolation,
        "color-interpolation",
        "colorInterpolation",
        STRING
    ),
    (
        color_interpolation_filters,
        "color-interpolation-filters",
        "colorInterpolationFilters",
        STRING
    ),
    (color_profile, "color-profile", "colorProfile", STRING),
    (color_redering, "color-rendering", "colorRendering", STRING),
    (content, "content", "content", STRING),
    (
        contentScriptType,
        "contentScriptType",
        "contentScriptType",
        STRING
    ),
    (
        contentStyleType,
        "contentStyleType",
        "contentStyleType",
        STRING
    ),
    (crossorigin, "crossorigin", "crossOrigin", STRING),
    (cursor, "cursor", "cursor", STRING),
    (cx, "cx", "cx", STRING),
    (cy, "cy", "cy", STRING),
    (d, "d", "d", STRING),
    (datatype, "datatype", "dataType", STRING),
    (defaultAction, "defaultAction", "defaultAction", STRING),
    (descent, "descent", "descent", NUMBER),
    (
        diffuseConstant,
        "diffuseConstant",
        "diffuseConstant",
        NUMBER
    ),
    (direction, "direction", "direction", STRING),
    (display, "display", "display", STRING),
    (dur, "dur", "dur", STRING),
    (divisor, "divisor", "divisor", NUMBER),
    (dominant, "dominant-baseline", "dominantBaseline", STRING),
    (download, "download", "download", BOOL),
    (dx, "dx", "dx", STRING),
    (dy, "dy", "dy", STRING),
    (edgeMode, "edgeMode", "edgeMode", STRING),
    (editable, "editable", "editable", STRING),
    (elevation, "elevation", "elevation", NUMBER),
    (enable, "enable-background", "enableBackground", STRING),
    (end, "end", "end", STRING),
    (event, "event", "event", STRING),
    (exponent, "exponent", "exponent", NUMBER),
    (
        externalResourcesRequired,
        "externalResourcesRequired",
        "externalResourcesRequired",
        STRING
    ),
    (fill, "fill", "fill", STRING),
    (fill_opacity, "fill-opacity", "fillOpacity", NUMBER),
    (fill_rule, "fill-rule", "fillRule", STRING),
    (filter, "filter", "filter", STRING),
    (filterRes, "filterRes", "filterRes", STRING),
    (filterUnits, "filterUnits", "filterUnits", STRING),
    (flood_color, "flood-color", "floodColor", STRING),
    (flood_opacity, "flood-opacity", "floodOpacity", STRING),
    (focusable, "focusable", "focusable", STRING),
    (focusHighlight, "focusHighlight", "focusHighlight", STRING),
    (font_family, "font-family", "fontFamily", STRING),
    (font_size, "font-size", "fontSize", STRING),
    (
        font_size_adjust,
        "font-size-adjust",
        "fontSizeAdjust",
        STRING
    ),
    (font_stretch, "font-stretch", "fontStretch", STRING),
    (font_style, "font-style", "fontStyle", STRING),
    (font_variant, "font-variant", "fontVariant", STRING),
    (font_weight, "font-weight", "fontWeight", STRING),
    (format, "format", "format", STRING),
    (fr, "fr", "fr", STRING),
    (from, "from", "from", STRING),
    (fx, "fx", "fx", STRING),
    (fy, "fy", "fy", STRING),
    (g1, "g1", "g1", COMMA_SEP),
    (g2, "g2", "g2", COMMA_SEP),
    (glyph_name, "glyph-name", "glyphName", COMMA_SEP),
    (
        glyph_orientation_horizontal,
        "glyph-orientation-horizontal",
        "glyphOrientationHorizontal",
        STRING
    ),
    (
        glyph_orientation_vertical,
        "glyph-orientation-vertical",
        "glyphOrientationVertical",
        STRING
    ),
    (glyphRef, "glyphRef", "glyphRef", STRING),
    (
        gradientTransform,
        "gradientTransform",
        "gradientTransform",
        STRING
    ),
    (gradientUnits, "gradientUnits", "gradientUnits", STRING),
    (handler, "handler", "handler", STRING),
    (hanging, "hanging", "hanging", NUMBER),
    (
        hatchContentUnits,
        "hatchContentUnits",
        "hatchContentUnits",
        STRING
    ),
    (hatchUnits, "hatchUnits", "hatchUnits", STRING),
    (height, "height", "height", STRING),
    (href, "href", "href", STRING),
    (hreflang, "hreflang", "hrefLang", STRING),
    (horiz_adv_x, "horiz-adv-x", "horizAdvX", NUMBER),
    (horiz_origin_x, "horiz-origin-x", "horizOriginX", NUMBER),
    (horiz_origin_y, "horiz-origin-y", "horizOriginY", NUMBER),
    (id, "id", "id", STRING),
    (ideographic, "ideographic", "ideographic", NUMBER),
    (image_rendering, "image-rendering", "imageRendering", STRING),
    (
        initialVisibility,
        "initialVisibility",
        "initialVisibility",
        STRING
    ),
    (r#in, "in", "in", STRING),
    (in2, "in2", "in2", STRING),
    (intercept, "intercept", "intercept", NUMBER),
    (k, "k", "k", NUMBER),
    (k1, "k1", "k1", NUMBER),
    (k2, "k2", "k2", NUMBER),
    (k3, "k3", "k3", NUMBER),
    (k4, "k4", "k4", NUMBER),
    (
        kernelMatrix,
        "kernelMatrix",
        "kernelMatrix",
        COMMA_SEP | SPACE_SEP
    ),
    (
        kernelUnitLength,
        "kernelUnitLength",
        "kernelUnitLength",
        STRING
    ),
    (keyPoints, "keyPoints", "keyPoints", STRING),
    (keySplines, "keySplines", "keySplines", STRING),
    (keyTimes, "keyTimes", "keyTimes", STRING),
    (kerning, "kerning", "kerning", STRING),
    (lang, "lang", "lang", STRING),
    (lengthAdjust, "lengthAdjust", "lengthAdjust", STRING),
    (letter_spacing, "letter-spacing", "letterSpacing", STRING),
    (lighting_color, "lighting-color", "lightingColor", STRING),
    (
        limitingConeAngle,
        "limitingConeAngle",
        "limitingConeAngle",
        NUMBER
    ),
    (local, "local", "local", STRING),
    (marker_end, "marker-end", "markerEnd", STRING),
    (marker_mid, "marker-mid", "markerMid", STRING),
    (marker_start, "marker-start", "markerStart", STRING),
    (markerHeight, "markerHeight", "markerHeight", STRING),
    (markerUnits, "markerUnits", "markerUnits", STRING),
    (markerWidth, "markerWidth", "markerWidth", STRING),
    (mask, "mask", "mask", STRING),
    (
        maskContentUnits,
        "maskContentUnits",
        "maskContentUnits",
        STRING
    ),
    (maskUnits, "maskUnits", "maskUnits", STRING),
    (mathematical, "mathematical", "mathematical", STRING),
    (max, "max", "max", STRING),
    (media, "media", "media", STRING),
    (
        mediaCharacterEncoding,
        "mediaCharacterEncoding",
        "mediaCharacterEncoding",
        STRING
    ),
    (
        mediaContentEncodings,
        "mediaContentEncodings",
        "mediaContentEncodings",
        STRING
    ),
    (mediaSize, "mediaSize", "mediaSize", NUMBER),
    (mediaTime, "mediaTime", "mediaTime", STRING),
    (method, "method", "method", STRING),
    (min, "min", "min", STRING),
    (mode, "mode", "mode", STRING),
    (name, "name", "name", STRING),
    (nav_down, "nav-down", "navDown", STRING),
    (nav_down_left, "nav-down-left", "navDownLeft", STRING),
    (nav_down_right, "nav-down-right", "navDownRight", STRING),
    (nav_left, "nav-left", "navLeft", STRING),
    (nav_next, "nav-next", "navNext", STRING),
    (nav_prev, "nav-prev", "navPrev", STRING),
    (nav_right, "nav-right", "navRight", STRING),
    (nav_up, "nav-up", "navUp", STRING),
    (nav_up_left, "nav-up-left", "navUpLeft", STRING),
    (nav_up_right, "nav-up-right", "navUpRight", STRING),
    (numOctaves, "numOctaves", "numOctaves", STRING),
    (observer, "observer", "observer", STRING),
    (offset, "offset", "offset", STRING),
    (onabort, "onabort", "onAbort", STRING),
    (onactivate, "onactivate", "onActivate", STRING),
    (onafterprint, "onafterprint", "onAfterPrint", STRING),
    (onbeforeprint, "onbeforeprint", "onBeforePrint", STRING),
    (onbegin, "onbegin", "onBegin", STRING),
    (oncancel, "oncancel", "onCancel", STRING),
    (oncanplay, "oncanplay", "onCanPlay", STRING),
    (
        oncanplaythrough,
        "oncanplaythrough",
        "onCanPlayThrough",
        STRING
    ),
    (onchange, "onchange", "onChange", STRING),
    (onclick, "onclick", "onClick", STRING),
    (onclose, "onclose", "onClose", STRING),
    (oncopy, "oncopy", "onCopy", STRING),
    (oncuechange, "oncuechange", "onCueChange", STRING),
    (oncut, "oncut", "onCut", STRING),
    (ondblclick, "ondblclick", "onDblClick", STRING),
    (ondrag, "ondrag", "onDrag", STRING),
    (ondragend, "ondragend", "onDragEnd", STRING),
    (ondragenter, "ondragenter", "onDragEnter", STRING),
    (ondragexit, "ondragexit", "onDragExit", STRING),
    (ondragleave, "ondragleave", "onDragLeave", STRING),
    (ondragover, "ondragover", "onDragOver", STRING),
    (ondragstart, "ondragstart", "onDragStart", STRING),
    (ondrop, "ondrop", "onDrop", STRING),
    (
        ondurationchange,
        "ondurationchange",
        "onDurationChange",
        STRING
    ),
    (onemptied, "onemptied", "onEmptied", STRING),
    (onend, "onend", "onEnd", STRING),
    (onended, "onended", "onEnded", STRING),
    (onerror, "onerror", "onError", STRING),
    (onfocus, "onfocus", "onFocus", STRING),
    (onfocusin, "onfocusin", "onFocusIn", STRING),
    (onfocusout, "onfocusout", "onFocusOut", STRING),
    (onhashchange, "onhashchange", "onHashChange", STRING),
    (oninput, "oninput", "onInput", STRING),
    (oninvalid, "oninvalid", "onInvalid", STRING),
    (onkeydown, "onkeydown", "onKeyDown", STRING),
    (onkeypress, "onkeypress", "onKeyPress", STRING),
    (onkeyup, "onkeyup", "onKeyUp", STRING),
    (onload, "onload", "onLoad", STRING),
    (onloadeddata, "onloadeddata", "onLoadedData", STRING),
    (
        onloadedmetadata,
        "onloadedmetadata",
        "onLoadedMetadata",
        STRING
    ),
    (onloadstart, "onloadstart", "onLoadStart", STRING),
    (onmessage, "onmessage", "onMessage", STRING),
    (onmousedown, "onmousedown", "onMouseDown", STRING),
    (onmouseenter, "onmouseenter", "onMouseEnter", STRING),
    (onmouseleave, "onmouseleave", "onMouseLeave", STRING),
    (onmousemove, "onmousemove", "onMouseMove", STRING),
    (onmouseout, "onmouseout", "onMouseOut", STRING),
    (onmouseover, "onmouseover", "onMouseOver", STRING),
    (onmouseup, "onmouseup", "onMouseUp", STRING),
    (onmousewheel, "onmousewheel", "onMouseWheel", STRING),
    (onoffline, "onoffline", "onOffline", STRING),
    (ononline, "ononline", "onOnline", STRING),
    (onpagehide, "onpagehide", "onPageHide", STRING),
    (onpageshow, "onpageshow", "onPageShow", STRING),
    (onpaste, "onpaste", "onPaste", STRING),
    (onpause, "onpause", "onPause", STRING),
    (onplay, "onplay", "onPlay", STRING),
    (onplaying, "onplaying", "onPlaying", STRING),
    (onpopstate, "onpopstate", "onPopState", STRING),
    (onprogress, "onprogress", "onProgress", STRING),
    (onratechange, "onratechange", "onRateChange", STRING),
    (onrepeat, "onrepeat", "onRepeat", STRING),
    (onreset, "onreset", "onReset", STRING),
    (onresize, "onresize", "onResize", STRING),
    (onscroll, "onscroll", "onScroll", STRING),
    (onseeked, "onseeked", "onSeeked", STRING),
    (onseeking, "onseeking", "onSeeking", STRING),
    (onselect, "onselect", "onSelect", STRING),
    (onshow, "onshow", "onShow", STRING),
    (onstalled, "onstalled", "onStalled", STRING),
    (onstorage, "onstorage", "onStorage", STRING),
    (onsubmit, "onsubmit", "onSubmit", STRING),
    (onsuspend, "onsuspend", "onSuspend", STRING),
    (ontimeupdate, "ontimeupdate", "onTimeUpdate", STRING),
    (ontoggle, "ontoggle", "onToggle", STRING),
    (onunload, "onunload", "onUnload", STRING),
    (onvolumechange, "onvolumechange", "onVolumeChange", STRING),
    (onwaiting, "onwaiting", "onWaiting", STRING),
    (onzoom, "onzoom", "onZoom", STRING),
    (opacity, "opacity", "opacity", STRING),
    (operator, "operator", "operator", STRING),
    (order, "order", "order", STRING),
    (orient, "orient", "orient", STRING),
    (orientation, "orientation", "orientation", STRING),
    (origin, "origin", "origin", STRING),
    (overflow, "overflow", "overflow", STRING),
    (overlay, "overlay", "overlay", STRING),
    (
        overline_position,
        "overline-position",
        "overlinePosition",
        NUMBER
    ),
    (
        overline_thickness,
        "overline-thickness",
        "overlineThickness",
        NUMBER
    ),
    (paint_order, "paint-order", "paintOrder", STRING),
    (panose_1, "panose-1", "panose1", STRING),
    (path, "path", "path", STRING),
    (pathLength, "pathLength", "pathLength", NUMBER),
    (
        patternContentUnits,
        "patternContentUnits",
        "patternContentUnits",
        STRING
    ),
    (
        patternTransform,
        "patternTransform",
        "patternTransform",
        STRING
    ),
    (patternUnits, "patternUnits", "patternUnits", STRING),
    (phase, "phase", "phase", STRING),
    (ping, "ping", "ping", SPACE_SEP),
    (pitch, "pitch", "pitch", STRING),
    (playbackorder, "playbackorder", "playbackOrder", STRING),
    (pointer, "pointer-events", "pointerEvents", STRING),
    (points, "points", "points", STRING),
    (pointsAtX, "pointsAtX", "pointsAtX", NUMBER),
    (pointsAtY, "pointsAtY", "pointsAtY", NUMBER),
    (pointsAtZ, "pointsAtZ", "pointsAtZ", NUMBER),
    (preserveAlpha, "preserveAlpha", "preserveAlpha", STRING),
    (
        preserveAspectRatio,
        "preserveAspectRatio",
        "preserveAspectRatio",
        STRING
    ),
    (primitiveUnits, "primitiveUnits", "primitiveUnits", STRING),
    (propagate, "propagate", "propagate", STRING),
    (property, "property", "property", COMMA_SEP | SPACE_SEP),
    (r, "r", "r", STRING),
    (radius, "radius", "radius", STRING),
    (referrerpolicy, "referrerpolicy", "referrerPolicy", STRING),
    (refX, "refX", "refX", STRING),
    (refY, "refY", "refY", STRING),
    (rel, "rel", "rel", COMMA_SEP | SPACE_SEP),
    (rev, "rev", "rev", COMMA_SEP | SPACE_SEP),
    (rendering, "rendering-intent", "renderingIntent", STRING),
    (repeatCount, "repeatCount", "repeatCount", STRING),
    (repeatDur, "repeatDur", "repeatDur", STRING),
    (
        requiredExtensions,
        "requiredExtensions",
        "requiredExtensions",
        COMMA_SEP | SPACE_SEP
    ),
    (
        requiredFeatures,
        "requiredFeatures",
        "requiredFeatures",
        COMMA_SEP | SPACE_SEP
    ),
    (
        requiredFonts,
        "requiredFonts",
        "requiredFonts",
        COMMA_SEP | SPACE_SEP
    ),
    (
        requiredFormats,
        "requiredFormats",
        "requiredFormats",
        COMMA_SEP | SPACE_SEP
    ),
    (resource, "resource", "resource", STRING),
    (restart, "restart", "restart", STRING),
    (result, "result", "result", STRING),
    (rotate, "rotate", "rotate", STRING),
    (rx, "rx", "rx", STRING),
    (ry, "ry", "ry", STRING),
    (scale, "scale", "scale", STRING),
    (seed, "seed", "seed", STRING),
    (shape, "shape-rendering", "shapeRendering", STRING),
    (side, "side", "side", STRING),
    (slope, "slope", "slope", STRING),
    (snapshotTime, "snapshotTime", "snapshotTime", STRING),
    (
        specularConstant,
        "specularConstant",
        "specularConstant",
        NUMBER
    ),
    (
        specularExponent,
        "specularExponent",
        "specularExponent",
        NUMBER
    ),
    (spreadMethod, "spreadMethod", "spreadMethod", STRING),
    (spacing, "spacing", "spacing", STRING),
    (startOffset, "startOffset", "startOffset", STRING),
    (stdDeviation, "stdDeviation", "stdDeviation", STRING),
    (stemh, "stemh", "stemh", STRING),
    (stemv, "stemv", "stemv", STRING),
    (stitchTiles, "stitchTiles", "stitchTiles", STRING),
    (stop_color, "stop-color", "stopColor", STRING),
    (stop_opacity, "stop-opacity", "stopOpacity", STRING),
    (
        strikethrough_position,
        "strikethrough-position",
        "strikethroughPosition",
        NUMBER
    ),
    (
        strikethrough_thickness,
        "strikethrough-thickness",
        "strikethroughThickness",
        NUMBER
    ),
    (string, "string", "string", STRING),
    (stroke, "stroke", "stroke", STRING),
    (
        stroke_dasharray,
        "stroke-dasharray",
        "strokeDashArray",
        COMMA_SEP | SPACE_SEP
    ),
    (
        stroke_dashoffset,
        "stroke-dashoffset",
        "strokeDashOffset",
        STRING
    ),
    (stroke_linecap, "stroke-linecap", "strokeLineCap", STRING),
    (stroke_linejoin, "stroke-linejoin", "strokeLineJoin", STRING),
    (
        stroke_miterlimit,
        "stroke-miterlimit",
        "strokeMiterLimit",
        NUMBER
    ),
    (stroke_opacity, "stroke-opacity", "strokeOpacity", NUMBER),
    (stroke_width, "stroke-width", "strokeWidth", STRING),
    (style, "style", "style", STRING),
    (surfaceScale, "surfaceScale", "surfaceScale", NUMBER),
    (syncBehavior, "syncBehavior", "syncBehavior", STRING),
    (
        syncBehaviorDefault,
        "syncBehaviorDefault",
        "syncBehaviorDefault",
        STRING
    ),
    (syncMaster, "syncMaster", "syncMaster", STRING),
    (syncTolerance, "syncTolerance", "syncTolerance", STRING),
    (
        syncToleranceDefault,
        "syncToleranceDefault",
        "syncToleranceDefault",
        STRING
    ),
    (
        systemLanguage,
        "systemLanguage",
        "systemLanguage",
        COMMA_SEP | SPACE_SEP
    ),
    (tabindex, "tabindex", "tabIndex", NUMBER),
    (tableValues, "tableValues", "tableValues", STRING),
    (target, "target", "target", STRING),
    (targetX, "targetX", "targetX", NUMBER),
    (targetY, "targetY", "targetY", NUMBER),
    (text_anchor, "text-anchor", "textAnchor", STRING),
    (text_decoration, "text-decoration", "textDecoration", STRING),
    (text_rendering, "text-rendering", "textRendering", STRING),
    (textLength, "textLength", "textLength", STRING),
    (timelinebegin, "timelinebegin", "timelineBegin", STRING),
    (title, "title", "title", STRING),
    (
        transformBehavior,
        "transformBehavior",
        "transformBehavior",
        STRING
    ),
    (r#type, "type", "type", STRING),
    (r#typeof, "typeof", "typeOf", COMMA_SEP | SPACE_SEP),
    (to, "to", "to", STRING),
    (transform, "transform", "transform", STRING),
    (u1, "u1", "u1", STRING),
    (u2, "u2", "u2", STRING),
    (
        underline_position,
        "underline-position",
        "underlinePosition",
        NUMBER
    ),
    (
        underline_thickness,
        "underline-thickness",
        "underlineThickness",
        NUMBER
    ),
    (unicode, "unicode", "unicode", STRING),
    (unicode_bidi, "unicode-bidi", "unicodeBidi", STRING),
    (unicode_range, "unicode-range", "unicodeRange", STRING),
    (units_per_em, "units-per-em", "unitsPerEm", NUMBER),
    (values, "values", "values", STRING),
    (v_alphabetic, "v-alphabetic", "vAlphabetic", NUMBER),
    (v_mathematical, "v-mathematical", "vMathematical", NUMBER),
    (vector_effect, "vector-effect", "vectorEffect", STRING),
    (v_hanging, "v-hanging", "vHanging", NUMBER),
    (v_ideographic, "v-ideographic", "vIdeographic", NUMBER),
    (version, "version", "version", STRING),
    (vert_adv_y, "vert-adv-y", "vertAdvY", NUMBER),
    (vert_origin_x, "vert-origin-x", "vertOriginX", NUMBER),
    (vert_origin_y, "vert-origin-y", "vertOriginY", NUMBER),
    (viewBox, "viewBox", "viewBox", STRING),
    (viewTarget, "viewTarget", "viewTarget", STRING),
    (visibility, "visibility", "visibility", STRING),
    (width, "width", "width", STRING),
    (widths, "widths", "widths", STRING),
    (word_spacing, "word-spacing", "wordSpacing", STRING),
    (writing_mode, "writing-mode", "writingMode", STRING),
    (x, "x", "x", STRING),
    (x1, "x1", "x1", STRING),
    (x2, "x2", "x2", STRING),
    (
        xChannelSelector,
        "xChannelSelector",
        "xChannelSelector",
        STRING
    ),
    (x_height, "x-height", "xHeight", NUMBER),
    (y, "y", "y", STRING),
    (y1, "y1", "y1", STRING),
    (y2, "y2", "y2", STRING),
    (
        yChannelSelector,
        "yChannelSelector",
        "yChannelSelector",
        STRING
    ),
    (z, "z", "z", STRING),
    (zoomAndPan, "zoomAndPan", "zoomAndPan", STRING),
);
