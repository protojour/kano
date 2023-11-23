use std::sync::{Arc, Mutex};

use autostrata::{Diff, View};
use js_sys::Function;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::{Cursor, Web};

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
            super_constructor: &HtmlElementConstructor,
            super_tag: None,
        }
    }
}

pub struct ComponentHandle {
    _root_state: Box<dyn std::any::Any>,
}

pub trait WebComponent {
    fn register(&'static self, config: ComponentConfig);
    fn hydrate(&self, this: &HtmlElement, anchor: &HtmlElement) -> ComponentHandle;
}

impl<V, F> WebComponent for F
where
    V: View,
    <V as Diff>::State: std::any::Any,
    F: (Fn() -> V) + 'static,
{
    fn register(&'static self, config: ComponentConfig) {
        let constructor = js_constructor(self);

        // The function takes no arguments, so there are no attributes
        make_web_component_helper(constructor.into_js_value(), config, &[]);
    }

    fn hydrate(&self, _this: &HtmlElement, anchor: &HtmlElement) -> ComponentHandle {
        let mut cursor = Cursor::Detached;
        let state = self().init::<Web>(&mut cursor);

        let Cursor::Node(node) = cursor else {
            panic!("No node rendered");
        };

        anchor.append_child(&node).unwrap();

        ComponentHandle {
            _root_state: Box::new(state),
        }
    }
}

pub trait UpdateAttribute {
    fn update(&mut self, name: &str, value: &str);
}

impl UpdateAttribute for () {
    fn update(&mut self, _name: &str, _value: &str) {}
}

fn make_web_component_helper(
    constructor: JsValue,
    config: ComponentConfig,
    observed_attributes: &[&str],
) {
    let observed_attributes = JsValue::from(
        observed_attributes
            .iter()
            .map(|attr| JsValue::from_str(attr))
            .collect::<js_sys::Array>(),
    );

    register_web_component(
        config.superclass.super_constructor,
        config.tag_name,
        config.shadow.0,
        constructor,
        observed_attributes,
        config.superclass.super_tag,
    );
}

fn js_constructor<D: WebComponent + 'static>(spec: &'static D) -> Closure<dyn FnMut(HtmlElement)> {
    Closure::wrap(Box::new(move |this: HtmlElement| {
        let handle: Arc<Mutex<Option<ComponentHandle>>> = Arc::new(Mutex::new(None));

        // hydrate
        let h = handle.clone();
        let constructor = Closure::wrap(Box::new({
            move |this, anchor| {
                let mut lock = h.lock().unwrap_throw();
                *lock = Some(spec.hydrate(&this, &anchor));
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

// JavaScript shim
#[wasm_bindgen(module = "/src/web_component/register_web_component.js")]
extern "C" {
    fn register_web_component(
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
