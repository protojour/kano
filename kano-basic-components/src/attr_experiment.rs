use kano::{prelude::platform::*, OnEvent};
use kano_web::{html, Web};

mod k {
    pub trait Attrs {
        type Prop;
    }

    pub trait Props<A: Attrs> {
        fn cond_take<T>(&mut self, func: impl Fn(A::Prop) -> Result<T, A::Prop>) -> Option<T>;
    }

    pub trait Attr<A: Attrs> {
        fn into_prop(self) -> Option<A::Prop>;
    }

    impl<A: Attrs, const N: usize> Props<A> for [Option<A::Prop>; N] {
        fn cond_take<T>(&mut self, func: impl Fn(A::Prop) -> Result<T, A::Prop>) -> Option<T> {
            for element in self.iter_mut().rev() {
                if let Some(prop) = element.take() {
                    match func(prop) {
                        Ok(taken) => return Some(taken),
                        Err(failed) => {
                            // put back
                            *element = Some(failed);
                        }
                    }
                }
            }

            None
        }
    }

    impl<A: Attrs, T: Attr<A>> Attr<A> for Option<T> {
        fn into_prop(self) -> Option<<A as Attrs>::Prop> {
            self.and_then(T::into_prop)
        }
    }
}

macro_rules! take_prop {
    ($props:ident, $path:path) => {
        $props.cond_take(|prop| {
            if let $path(val) = prop {
                Ok(val)
            } else {
                Err(prop)
            }
        })
    };
    ($props:ident, $path:path, $default:expr) => {
        $props
            .cond_take(|prop| {
                if let $path(val) = prop {
                    Ok(val)
                } else {
                    Err(prop)
                }
            })
            .unwrap_or($default)
    };
}

pub struct KBC;

pub enum KBCProp {
    OnEvent(OnEvent),
    Big(Big),
}

impl k::Attrs for KBC {
    type Prop = KBCProp;
}

pub struct Big(bool);

impl k::Attr<KBC> for OnEvent {
    fn into_prop(self) -> Option<KBCProp> {
        Some(KBCProp::OnEvent(self))
    }
}

impl k::Attr<KBC> for Big {
    fn into_prop(self) -> Option<KBCProp> {
        Some(KBCProp::Big(self))
    }
}

fn _component(mut props: impl k::Props<KBC>, children: impl Children<Web>) -> impl View<Web> {
    let _big = take_prop!(props, KBCProp::Big, Big(false));
    let on_event = take_prop!(props, KBCProp::OnEvent).unwrap_or_else(|| kano::on::click(|| {}));

    html::div((on_event,), children)
}

#[test]
fn test() {
    let big_perhaps: Option<Big> = None;

    _component([], ());
    _component(
        [
            k::Attr::into_prop(kano::on::click(|| {})),
            k::Attr::into_prop(Big(true)),
            k::Attr::into_prop(big_perhaps),
            None,
        ],
        (),
    );
}
