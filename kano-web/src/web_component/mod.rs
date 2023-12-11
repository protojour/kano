use std::any::Any;
use std::{cell::RefCell, rc::Rc};

use js_sys::Function;
use kano::view::Reactive;
use kano::{DeserializeAttribute, View};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::web_cursor::Position;
use crate::{Html5, Web, WebCursor};

use self::properties::{read_props, ComponentProperties};
use self::slot::Slot;

mod properties;
mod slot;

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

type Props<A> = Vec<Option<A>>;
type HydrateFn = Rc<dyn Fn(Rc<RefCell<ComponentHandle>>, HtmlElement, HtmlElement)>;

pub fn register_web_component<A, V, F>(func: F, config: ComponentConfig)
where
    A: DeserializeAttribute,
    V: View<Web, Html5> + 'static,
    <V as View<Web, Html5>>::State: std::any::Any,
    F: (Fn(Props<A>, Slot) -> V) + Copy + 'static,
{
    let shadow = config.shadow.0;
    register_inner(
        ComponentClass {
            hydrate_fn: Rc::new(move |handle, _this, anchor| {
                kano::log(&format!("rust constructor anchor = {anchor:?}"));

                let mut cursor = WebCursor::new_detached();

                let state = Reactive({
                    let handle = handle.clone();
                    move || func(read_props::<A>(&handle.borrow().properties), Slot)
                })
                .init(&mut cursor);

                let Position::Node(node) = cursor.position else {
                    panic!("No node rendered");
                };

                anchor.append_child(&node).unwrap();

                let mut handle = handle.borrow_mut();
                handle.lifecycle_state = if shadow {
                    LifecycleState::ShadowHydrated
                } else {
                    LifecycleState::Hydrated
                };
                handle.update_fn = Some(Rc::new(state.update_fn()));
                handle.root_state = Some(Box::new(state));
                handle.root_node = Some(node);
            }),
            observed_attributes: observed_attributes::<A>(),
        },
        config,
    );
}

pub struct ComponentHandle {
    lifecycle_state: LifecycleState,
    properties: ComponentProperties,
    update_fn: Option<Rc<dyn Fn() -> bool>>,
    root_state: Option<Box<dyn Any>>,
    root_node: Option<web_sys::Node>,
}

#[derive(Clone, Copy)]
enum LifecycleState {
    Allocated,
    Hydrated,
    ShadowHydrated,
    ShadowAttributesDirty,
    Connected,
}

fn on_connected(handle: Rc<RefCell<ComponentHandle>>) {
    kano::log("on connected");
    let lifecycle_state = handle.borrow().lifecycle_state;

    if let LifecycleState::ShadowAttributesDirty = lifecycle_state {
        let update_fn = handle.borrow().update_fn.clone().unwrap();
        update_fn();
    }

    handle.borrow_mut().lifecycle_state = LifecycleState::Connected;
}

fn on_adopted(_handle: Rc<RefCell<ComponentHandle>>) {}

fn on_attribute_changed(handle: Rc<RefCell<ComponentHandle>>, name: String, value: Option<String>) {
    kano::log(&format!("Attribute changed `{name}` to {value:?}"));

    let mut should_update = false;

    {
        let mut handle = handle.borrow_mut();
        if let Some(value) = value {
            handle.properties.insert(name, value);
        } else {
            handle.properties.remove(&name);
        }

        let next_state = match handle.lifecycle_state {
            LifecycleState::ShadowHydrated => LifecycleState::ShadowAttributesDirty,
            LifecycleState::Connected => {
                should_update = true;
                handle.lifecycle_state
            }
            _ => handle.lifecycle_state,
        };

        handle.lifecycle_state = next_state;
    };

    if should_update {
        let update_fn = handle.borrow().update_fn.clone().unwrap();
        update_fn();
    }
}

struct ComponentClass {
    hydrate_fn: HydrateFn,
    observed_attributes: Vec<&'static str>,
}

fn observed_attributes<A: DeserializeAttribute>() -> Vec<&'static str> {
    let mut attribute_names = vec![];
    A::describe(&mut attribute_names);
    attribute_names
}

fn register_inner(
    ComponentClass {
        hydrate_fn,
        observed_attributes,
    }: ComponentClass,
    config: ComponentConfig,
) {
    let js_constructor = Closure::wrap(Box::new(move |this: HtmlElement| {
        let handle = Rc::new(RefCell::new(ComponentHandle {
            lifecycle_state: LifecycleState::Allocated,
            properties: ComponentProperties::default(),
            update_fn: None,
            root_state: None,
            root_node: None,
        }));

        let hydrate_fn = hydrate_fn.clone();

        register_js_method(&this, "_hydrate", {
            {
                let handle = handle.clone();
                Closure::wrap(Box::new(move |this, anchor| {
                    hydrate_fn(handle.clone(), this, anchor);
                })
                    as Box<dyn FnMut(HtmlElement, HtmlElement)>)
            }
            .into_js_value()
        });

        register_js_method(&this, "_connectedCallback", {
            let handle = handle.clone();
            Closure::wrap(Box::new(move |_el| {
                on_connected(handle.clone());
            }) as Box<dyn FnMut(HtmlElement)>)
            .into_js_value()
        });

        register_js_method(&this, "_adoptedCallback", {
            let handle = handle.clone();
            Closure::wrap(Box::new(move |_el| {
                on_adopted(handle.clone());
            }) as Box<dyn FnMut(HtmlElement)>)
            .into_js_value()
        });

        register_js_method(&this, "_attributeChangedCallback", {
            let handle = handle.clone();
            Closure::wrap(Box::new(move |_el, name, _old_value, new_value| {
                on_attribute_changed(handle.clone(), name, new_value);
            })
                as Box<dyn FnMut(HtmlElement, String, Option<String>, Option<String>)>)
            .into_js_value()
        });

        register_js_method(&this, "_disconnectedCallback", {
            let handle = handle.clone();
            Closure::wrap(Box::new(move |_el| {
                let mut borrow_mut = handle.borrow_mut();
                drop(borrow_mut.root_state.take());
            }) as Box<dyn FnMut(HtmlElement)>)
            .into_js_value()
        });
    }) as Box<dyn FnMut(HtmlElement)>);

    let observed_attributes = JsValue::from(
        observed_attributes
            .iter()
            .map(|attr| JsValue::from_str(attr))
            .collect::<js_sys::Array>(),
    );

    js::register_web_component(
        config.superclass.super_constructor,
        config.tag_name,
        config.shadow.0,
        js_constructor.into_js_value(),
        observed_attributes,
        config.superclass.super_tag,
    );
}

fn register_js_method(this: &HtmlElement, method_name: &str, method: JsValue) {
    js_sys::Reflect::set(this, &JsValue::from_str(method_name), &method).unwrap_throw();
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
