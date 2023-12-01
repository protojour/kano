use std::{cell::RefCell, rc::Rc};

use js_sys::Function;
use kano::{Diff, View};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::{Web, WebCursor};

pub struct ComponentConfig {
    pub tag_name: &'static str,
    pub shadow: Shadow,
    pub superclass: Superclass,
}

pub struct Shadow(pub bool);

impl Default for Shadow {
    fn default() -> Self {
        Self(true)
    }
}

pub struct Superclass {
    pub super_constructor: &'static Function,
    pub super_tag: Option<&'static str>,
}

impl Default for Superclass {
    fn default() -> Self {
        Self {
            super_constructor: &js::HtmlElementConstructor,
            super_tag: None,
        }
    }
}

pub struct ComponentHandle {
    _root_state: Box<dyn std::any::Any>,
}

type NoProps<A> = [Option<A>; 0];

type SlotChildren = (kano_html::Element<NoProps<kano_html::Attributes>, ()>,);

pub fn register_web_component<A, V, F>(func: F, config: ComponentConfig)
where
    V: View<Web>,
    <V as Diff<Web>>::State: std::any::Any,
    F: (Fn(NoProps<A>, SlotChildren) -> V) + Copy + 'static,
{
    let constructor = js_constructor(move |_this, anchor| {
        let mut cursor = WebCursor::Detached;
        let state = func([], (kano_html::slot([], ()),)).init(&mut cursor);

        let WebCursor::Node(node, _) = cursor else {
            panic!("No node rendered");
        };

        anchor.append_child(&node).unwrap();

        ComponentHandle {
            _root_state: Box::new(state),
        }
    });

    let attributes = &["foo"];

    let observed_attributes = JsValue::from(
        attributes
            .iter()
            .map(|attr| JsValue::from_str(attr))
            .collect::<js_sys::Array>(),
    );

    js::register_web_component(
        config.superclass.super_constructor,
        config.tag_name,
        config.shadow.0,
        constructor.into_js_value(),
        observed_attributes,
        config.superclass.super_tag,
    )
}

pub trait UpdateAttribute {
    fn update(&mut self, name: &str, value: &str);
}

impl UpdateAttribute for () {
    fn update(&mut self, _name: &str, _value: &str) {}
}

fn js_constructor<F: (Fn(&HtmlElement, &HtmlElement) -> ComponentHandle) + Copy + 'static>(
    hydrate_func: F,
) -> Closure<dyn FnMut(HtmlElement)> {
    Closure::wrap(Box::new(move |this: HtmlElement| {
        let handle: Rc<RefCell<Option<ComponentHandle>>> = Rc::new(RefCell::new(None));

        // hydrate
        let h = handle.clone();
        let constructor = Closure::wrap(Box::new({
            move |this, anchor| {
                *h.borrow_mut() = Some(hydrate_func(&this, &anchor));
            }
        }) as Box<dyn FnMut(HtmlElement, HtmlElement)>);
        js_sys::Reflect::set(
            &this,
            &JsValue::from_str("_hydrate"),
            &constructor.into_js_value(),
        )
        .unwrap_throw();

        // connectedCallback
        /*
            let cmp = component.clone();
            let connected = Closure::wrap(Box::new({
                move |el| {
                    let mut lock = cmp.lock().unwrap_throw();
                    lock.connected_callback(&el);
                }
            }) as Box<dyn FnMut(HtmlElement)>);
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_connectedCallback"),
                &connected.into_js_value(),
            )
            .unwrap_throw();
        */

        // disconnectedCallback
        /*
            let cmp = component.clone();
            let disconnected = Closure::wrap(Box::new(move |el| {
                let mut lock = cmp.lock().unwrap_throw();
                lock.disconnected_callback(&el);
            }) as Box<dyn FnMut(HtmlElement)>);
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_disconnectedCallback"),
                &disconnected.into_js_value(),
            )
            .unwrap_throw();
        */

        // adoptedCallback
        /*
            let cmp = component.clone();
            let adopted = Closure::wrap(Box::new(move |el| {
                let mut lock = cmp.lock().unwrap_throw();
                lock.adopted_callback(&el);
            }) as Box<dyn FnMut(HtmlElement)>);
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_adoptedCallback"),
                &adopted.into_js_value(),
            )
            .unwrap_throw();
        */

        // attributeChangedCallback
        // TODO: Reactive attributes
        /*
            let cmp = component;
            let attribute_changed = Closure::wrap(Box::new(move |el, name, old_value, new_value| {
                let mut lock = cmp.lock().unwrap_throw();
                lock.attribute_changed_callback(&el, name, old_value, new_value);
            })
                as Box<dyn FnMut(HtmlElement, String, Option<String>, Option<String>)>);
            js_sys::Reflect::set(
                &this,
                &JsValue::from_str("_attributeChangedCallback"),
                &attribute_changed.into_js_value(),
            )
            .unwrap_throw();
        */
    }) as Box<dyn FnMut(HtmlElement)>)
}

mod js {
    use super::*;

    // JavaScript shim
    #[wasm_bindgen(module = "/src/web_component/register_web_component.js")]
    extern "C" {
        pub fn register_web_component(
            superclass: &js_sys::Function,
            tag_name: &str,
            shadow: bool,
            constructor: JsValue,
            observed_attributes: JsValue,
            superclass_tag: Option<&str>,
        );
    }

    #[allow(non_upper_case_globals)]
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = HTMLElement, js_namespace = window)]
        pub static HtmlElementConstructor: js_sys::Function;
    }
}
