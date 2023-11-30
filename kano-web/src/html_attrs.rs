use std::borrow::Cow;

use crate::props::{HtmlProperty, HtmlPropertyValue, StringOrBool, Strings};

macro_rules! define_attr {
    (($ident:ident, $name:literal, $idl:literal, STRING)) => {
        /// The
        #[doc = concat!("`", stringify!($name), "`")]
        /// attribute.
        pub fn $ident(value: impl Into<Cow<'static, str>>) -> HtmlProperty {
            HtmlProperty::new($idl, HtmlPropertyValue::String(value.into()))
        }
    };
    (($ident:ident, $name:literal, $idl:literal, COMMA_SEP | STRING)) => {
        /// The
        #[doc = concat!("`", stringify!($name), "`")]
        /// attribute.
        pub fn $ident(value: impl Into<Strings>) -> HtmlProperty {
            HtmlProperty::new($idl, HtmlPropertyValue::CommaSep(value.into().0))
        }
    };
    (($ident:ident, $name:literal, $idl:literal, SPACE_SEP | STRING)) => {
        /// The
        #[doc = concat!("`", stringify!($name), "`")]
        /// attribute.
        pub fn $ident(value: impl Into<Strings>) -> HtmlProperty {
            HtmlProperty::new($idl, HtmlPropertyValue::SpaceSep(value.into().0))
        }
    };
    (($ident:ident, $name:literal, $idl:literal, BOOL)) => {
        /// The
        #[doc = concat!("`", stringify!($name), "`")]
        /// attribute.
        pub fn $ident(value: bool) -> HtmlProperty {
            HtmlProperty::new($idl, HtmlPropertyValue::Bool(value))
        }
    };
    (($ident:ident, $name:literal, $idl:literal, NUMBER)) => {
        /// The
        #[doc = concat!("`", stringify!($name), "`")]
        /// attribute.
        pub fn $ident(value: i32) -> HtmlProperty {
            HtmlProperty::new($idl, HtmlPropertyValue::Number(value))
        }
    };
    (($ident:ident, $name:literal, $idl:literal, TRUE | EMPTY_STRING | FALSE)) => {
        /// The
        #[doc = concat!("`", stringify!($name), "`")]
        /// attribute.
        pub fn $ident(value: Option<bool>) -> HtmlProperty {
            HtmlProperty::new(
                $idl,
                match value {
                    Some(bool) => HtmlPropertyValue::Bool(bool),
                    None => HtmlPropertyValue::String("".into()),
                },
            )
        }
    };
    (($ident:ident, $name:literal, $idl:literal, BOOL | STRING)) => {
        /// The
        #[doc = concat!("`", stringify!($name), "`")]
        /// attribute.
        pub fn $ident(value: impl Into<StringOrBool>) -> HtmlProperty {
            HtmlProperty::new(
                $idl,
                match value.into() {
                    StringOrBool::String(string) => HtmlPropertyValue::String(string),
                    StringOrBool::Bool(bool) => HtmlPropertyValue::Bool(bool),
                },
            )
        }
    };
    (($ident:ident, $name:literal, $idl:literal, TRUE | FALSE | STRING)) => {
        define_attr!(($ident, $name, $idl, BOOL | STRING));
    };
    (($ident:ident, $name:literal, $idl:literal, TRUE | FALSE)) => {
        define_attr!(($ident, $name, $idl, BOOL));
    };
}

macro_rules! define_attrs {
    ($($attr_def:tt,)*) => {
        $(define_attr!($attr_def);)*
    };
}

define_attrs!(
    (abbr, "abbr", "abbr", STRING),
    (accept, "accept", "accept", COMMA_SEP | STRING),
    (
        accept_charset,
        "accept-charset",
        "acceptCharset",
        SPACE_SEP | STRING
    ),
    (accesskey, "accesskey", "accessKey", SPACE_SEP | STRING),
    (action, "action", "action", STRING),
    (allow, "allow", "allow", STRING),
    (allowfullscreen, "allowfullscreen", "allowFullScreen", BOOL),
    (
        allowpaymentrequest,
        "allowpaymentrequest",
        "allowPaymentRequest",
        BOOL
    ),
    (allowusermedia, "allowusermedia", "allowUserMedia", BOOL),
    (alt, "alt", "alt", STRING),
    (as_, "as", "as", STRING),
    (async_, "async", "async", BOOL),
    (autocapitalize, "autocapitalize", "autoCapitalize", STRING),
    (
        autocomplete,
        "autocomplete",
        "autoComplete",
        SPACE_SEP | STRING
    ),
    (autofocus, "autofocus", "autoFocus", BOOL),
    (autoplay, "autoplay", "autoPlay", BOOL),
    (capture, "capture", "capture", BOOL),
    (charset, "charset", "charSet", STRING),
    (checked, "checked", "checked", BOOL),
    (cite, "cite", "cite", STRING),
    (class, "class", "className", SPACE_SEP | STRING),
    (cols, "cols", "cols", NUMBER),
    (colspan, "colspan", "colSpan", STRING),
    (content, "content", "content", STRING),
    (
        contenteditable,
        "contenteditable",
        "contentEditable",
        TRUE | EMPTY_STRING | FALSE
    ),
    (controls, "controls", "controls", BOOL),
    (
        controlslist,
        "controlslist",
        "controlsList",
        SPACE_SEP | STRING
    ),
    (coords, "coords", "coords", COMMA_SEP | STRING),
    (crossorigin, "crossorigin", "crossOrigin", STRING),
    (data, "data", "data", STRING),
    (datetime, "datetime", "dateTime", STRING),
    (decoding, "decoding", "decoding", STRING),
    (default, "default", "default", BOOL),
    (defer, "defer", "defer", BOOL),
    (dir, "dir", "dir", STRING),
    (dirname, "dirname", "dirName", STRING),
    (disabled, "disabled", "disabled", BOOL),
    (download, "download", "download", BOOL | STRING),
    (draggable, "draggable", "draggable", TRUE | FALSE),
    (enctype, "enctype", "encType", STRING),
    (enterkeyhint, "enterkeyhint", "enterKeyHint", STRING),
    (form, "form", "form", STRING),
    (formaction, "formaction", "formAction", STRING),
    (formenctype, "formenctype", "formEncType", STRING),
    (formmethod, "formmethod", "formMethod", STRING),
    (formnovalidate, "formnovalidate", "formNoValidate", BOOL),
    (formtarget, "formtarget", "formTarget", STRING),
    (headers, "headers", "headers", SPACE_SEP | STRING),
    (height, "height", "height", NUMBER),
    (hidden, "hidden", "hidden", BOOL),
    (high, "high", "high", NUMBER),
    (href, "href", "href", STRING),
    (hreflang, "hreflang", "hrefLang", STRING),
    (for_, "for", "htmlFor", SPACE_SEP | STRING),
    (http, "http-equiv", "httpEquiv", SPACE_SEP | STRING),
    (id, "id", "id", STRING),
    (imagesizes, "imagesizes", "imageSizes", STRING),
    (
        imagesrcset,
        "imagesrcset",
        "imageSrcSet",
        COMMA_SEP | STRING
    ),
    (inputmode, "inputmode", "inputMode", STRING),
    (integrity, "integrity", "integrity", STRING),
    (is, "is", "is", STRING),
    (ismap, "ismap", "isMap", BOOL),
    (itemid, "itemid", "itemId", STRING),
    (itemprop, "itemprop", "itemProp", SPACE_SEP | STRING),
    (itemref, "itemref", "itemRef", SPACE_SEP | STRING),
    (itemscope, "itemscope", "itemScope", BOOL),
    (itemtype, "itemtype", "itemType", SPACE_SEP | STRING),
    (kind, "kind", "kind", STRING),
    (label, "label", "label", STRING),
    (lang, "lang", "lang", STRING),
    (language, "language", "language", STRING),
    (list, "list", "list", STRING),
    (loading, "loading", "loading", STRING),
    (loop_, "loop", "loop", BOOL),
    (low, "low", "low", NUMBER),
    (manifest, "manifest", "manifest", STRING),
    (max, "max", "max", STRING),
    (maxlength, "maxlength", "maxLength", NUMBER),
    (media, "media", "media", STRING),
    (method, "method", "method", STRING),
    (min, "min", "min", STRING),
    (minlength, "minlength", "minLength", NUMBER),
    (multiple, "multiple", "multiple", BOOL),
    (muted, "muted", "muted", BOOL),
    (name, "name", "name", STRING),
    (nonce, "nonce", "nonce", STRING),
    (nomodule, "nomodule", "noModule", BOOL),
    (novalidate, "novalidate", "noValidate", BOOL),
    (onabort, "onabort", "onAbort", STRING),
    (onafterprint, "onafterprint", "onAfterPrint", STRING),
    (onauxclick, "onauxclick", "onAuxClick", STRING),
    (onbeforeprint, "onbeforeprint", "onBeforePrint", STRING),
    (onbeforeunload, "onbeforeunload", "onBeforeUnload", STRING),
    (onblur, "onblur", "onBlur", STRING),
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
    (oncontextmenu, "oncontextmenu", "onContextMenu", STRING),
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
    (onended, "onended", "onEnded", STRING),
    (onerror, "onerror", "onError", STRING),
    (onfocus, "onfocus", "onFocus", STRING),
    (onformdata, "onformdata", "onFormData", STRING),
    (onhashchange, "onhashchange", "onHashChange", STRING),
    (oninput, "oninput", "onInput", STRING),
    (oninvalid, "oninvalid", "onInvalid", STRING),
    (onkeydown, "onkeydown", "onKeyDown", STRING),
    (onkeypress, "onkeypress", "onKeyPress", STRING),
    (onkeyup, "onkeyup", "onKeyUp", STRING),
    (
        onlanguagechange,
        "onlanguagechange",
        "onLanguageChange",
        STRING
    ),
    (onload, "onload", "onLoad", STRING),
    (onloadeddata, "onloadeddata", "onLoadedData", STRING),
    (
        onloadedmetadata,
        "onloadedmetadata",
        "onLoadedMetadata",
        STRING
    ),
    (onloadend, "onloadend", "onLoadEnd", STRING),
    (onloadstart, "onloadstart", "onLoadStart", STRING),
    (onmessage, "onmessage", "onMessage", STRING),
    (onmessageerror, "onmessageerror", "onMessageError", STRING),
    (onmousedown, "onmousedown", "onMouseDown", STRING),
    (onmouseenter, "onmouseenter", "onMouseEnter", STRING),
    (onmouseleave, "onmouseleave", "onMouseLeave", STRING),
    (onmousemove, "onmousemove", "onMouseMove", STRING),
    (onmouseout, "onmouseout", "onMouseOut", STRING),
    (onmouseover, "onmouseover", "onMouseOver", STRING),
    (onmouseup, "onmouseup", "onMouseUp", STRING),
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
    (
        onrejectionhandled,
        "onrejectionhandled",
        "onRejectionHandled",
        STRING
    ),
    (onreset, "onreset", "onReset", STRING),
    (onresize, "onresize", "onResize", STRING),
    (onscroll, "onscroll", "onScroll", STRING),
    (
        onsecuritypolicyviolation,
        "onsecuritypolicyviolation",
        "onSecurityPolicyViolation",
        STRING
    ),
    (onseeked, "onseeked", "onSeeked", STRING),
    (onseeking, "onseeking", "onSeeking", STRING),
    (onselect, "onselect", "onSelect", STRING),
    (onslotchange, "onslotchange", "onSlotChange", STRING),
    (onstalled, "onstalled", "onStalled", STRING),
    (onstorage, "onstorage", "onStorage", STRING),
    (onsubmit, "onsubmit", "onSubmit", STRING),
    (onsuspend, "onsuspend", "onSuspend", STRING),
    (ontimeupdate, "ontimeupdate", "onTimeUpdate", STRING),
    (ontoggle, "ontoggle", "onToggle", STRING),
    (
        onunhandledrejection,
        "onunhandledrejection",
        "onUnhandledRejection",
        STRING
    ),
    (onunload, "onunload", "onUnload", STRING),
    (onvolumechange, "onvolumechange", "onVolumeChange", STRING),
    (onwaiting, "onwaiting", "onWaiting", STRING),
    (onwheel, "onwheel", "onWheel", STRING),
    (open, "open", "open", BOOL),
    (optimum, "optimum", "optimum", NUMBER),
    (pattern, "pattern", "pattern", STRING),
    (ping, "ping", "ping", SPACE_SEP | STRING),
    (placeholder, "placeholder", "placeholder", STRING),
    (playsinline, "playsinline", "playsInline", BOOL),
    (poster, "poster", "poster", STRING),
    (preload, "preload", "preload", STRING),
    (readonly, "readonly", "readOnly", BOOL),
    (referrerpolicy, "referrerpolicy", "referrerPolicy", STRING),
    (rel, "rel", "rel", SPACE_SEP | STRING),
    (required, "required", "required", BOOL),
    (reversed, "reversed", "reversed", BOOL),
    (rows, "rows", "rows", NUMBER),
    (rowspan, "rowspan", "rowSpan", NUMBER),
    (sandbox, "sandbox", "sandbox", SPACE_SEP | STRING),
    (scope, "scope", "scope", STRING),
    (scoped, "scoped", "scoped", BOOL),
    (seamless, "seamless", "seamless", BOOL),
    (selected, "selected", "selected", BOOL),
    (shape, "shape", "shape", STRING),
    (size, "size", "size", NUMBER),
    (sizes, "sizes", "sizes", STRING),
    (slot, "slot", "slot", STRING),
    (span, "span", "span", NUMBER),
    (spellcheck, "spellcheck", "spellCheck", TRUE | FALSE),
    (src, "src", "src", STRING),
    (srcdoc, "srcdoc", "srcDoc", STRING),
    (srclang, "srclang", "srcLang", STRING),
    (srcset, "srcset", "srcSet", COMMA_SEP | STRING),
    (start, "start", "start", NUMBER),
    (step, "step", "step", STRING),
    (style, "style", "style", STRING),
    (tabindex, "tabindex", "tabIndex", NUMBER),
    (target, "target", "target", STRING),
    (title, "title", "title", STRING),
    (translate, "translate", "translate", STRING),
    (type_, "type", "type", STRING),
    (typemustmatch, "typemustmatch", "typeMustMatch", BOOL),
    (usemap, "usemap", "useMap", STRING),
    (value, "value", "value", TRUE | FALSE | STRING),
    (width, "width", "width", NUMBER),
    (wrap, "wrap", "wrap", STRING),
    // Legacy.
    // See: https://html.spec.whatwg.org/#other-elements,-attributes-and-apis
    (align, "align", "align", STRING), // Several. Use CSS `text-align` instead,
    (alink, "alink", "aLink", STRING), // `<body>`. Use CSS `a:active {color}` instead
    (archive, "archive", "archive", SPACE_SEP | STRING), // `<object>`. List of URIs to archives
    (axis, "axis", "axis", STRING),    // `<td>` and `<th>`. Use `scope` on `<th>`
    (background, "background", "background", STRING), // `<body>`. Use CSS `background-image` instead
    (bgcolor, "bgcolor", "bgColor", STRING), // `<body>` and table elements. Use CSS `background-color` instead
    (border, "border", "border", NUMBER),    // `<table>`. Use CSS `border-width` instead,
    (bordercolor, "bordercolor", "borderColor", STRING), // `<table>`. Use CSS `border-color` instead,
    (bottommargin, "bottommargin", "bottomMargin", NUMBER), // `<body>`
    (cellpadding, "cellpadding", "cellPadding", STRING), // `<table>`
    (cellspacing, "cellspacing", "cellSpacing", STRING), // `<table>`
    (char, "char", "char", STRING), // Several table elements. When `align=char`, sets the character to align on
    (charoff, "charoff", "charOff", STRING), // Several table elements. When `char`, offsets the alignment
    (classid, "classid", "classId", STRING), // `<object>`
    (clear, "clear", "clear", STRING),       // `<br>`. Use CSS `clear` instead
    (code, "code", "code", STRING),          // `<object>`
    (codebase, "codebase", "codeBase", STRING), // `<object>`
    (codetype, "codetype", "codeType", STRING), // `<object>`
    (color, "color", "color", STRING),       // `<font>` and `<hr>`. Use CSS instead
    (compact, "compact", "compact", BOOL),   // Lists. Use CSS to reduce space between items instead
    (declare, "declare", "declare", BOOL),   // `<object>`
    (event, "event", "event", STRING),       // `<script>`
    (face, "face", "face", STRING),          // `<font>`. Use CSS instead
    (frame, "frame", "frame", STRING),       // `<table>`
    (frameborder, "frameborder", "frameBorder", STRING), // `<iframe>`. Use CSS `border` instead
    (hspace, "hspace", "hSpace", NUMBER),    // `<img>` and `<object>`
    (leftmargin, "leftmargin", "leftMargin", NUMBER), // `<body>`
    (link, "link", "link", STRING),          // `<body>`. Use CSS `a:link {color: *}` instead
    (longdesc, "longdesc", "longDesc", STRING), // `<frame>`, `<iframe>`, and `<img>`. Use an `<a>`
    (lowsrc, "lowsrc", "lowSrc", STRING),    // `<img>`. Use a `<picture>`
    (marginheight, "marginheight", "marginHeight", NUMBER), // `<body>`
    (marginwidth, "marginwidth", "marginWidth", NUMBER), // `<body>`
    (noresize, "noresize", "noResize", BOOL), // `<frame>`
    (nohref, "nohref", "noHref", BOOL), // `<area>`. Use no href instead of an explicit `nohref`
    (noshade, "noshade", "noShade", BOOL), // `<hr>`. Use background-color and height instead of borders
    (nowrap, "nowrap", "noWrap", BOOL),    // `<td>` and `<th>`
    (object, "object", "object", STRING),  // `<applet>`
    (profile, "profile", "profile", STRING), // `<head>`
    (prompt, "prompt", "prompt", STRING),  // `<isindex>`
    (rev, "rev", "rev", STRING),           // `<link>`
    (rightmargin, "rightmargin", "rightMargin", NUMBER), // `<body>`
    (rules, "rules", "rules", STRING),     // `<table>`
    (scheme, "scheme", "scheme", STRING),  // `<meta>`
    (scrolling, "scrolling", "scrolling", TRUE | FALSE | STRING), // `<frame>`. Use overflow in the child context
    (standby, "standby", "standby", STRING),                      // `<object>`
    (summary, "summary", "summary", STRING),                      // `<table>`
    (text, "text", "text", STRING), // `<body>`. Use CSS `color` instead
    (topmargin, "topmargin", "topMargin", NUMBER), // `<body>`
    (valuetype, "valuetype", "valueType", STRING), // `<param>`
    (version, "version", "version", STRING), // `<html>`. Use a doctype.
    (valign, "valign", "vAlign", STRING), // Several. Use CSS `vertical-align` instead
    (vlink, "vlink", "vLink", STRING), // `<body>`. Use CSS `a:visited {color}` instead
    (vspace, "vspace", "vSpace", NUMBER), // `<img>` and `<object>`
    // Non-standard Properties.
    (
        allowtransparency,
        "allowtransparency",
        "allowTransparency",
        STRING
    ),
    (autocorrect, "autocorrect", "autoCorrect", STRING),
    (autosave, "autosave", "autoSave", STRING),
    (
        disablepictureinpicture,
        "disablepictureinpicture",
        "disablePictureInPicture",
        BOOL
    ),
    (
        disableremoteplayback,
        "disableremoteplayback",
        "disableRemotePlayback",
        BOOL
    ),
    (prefix, "prefix", "prefix", STRING),
    (property, "property", "property", STRING),
    (results, "results", "results", NUMBER),
    (security, "security", "security", STRING),
    (unselectable, "unselectable", "unselectable", STRING),
);
