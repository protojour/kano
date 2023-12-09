use kano::{platform::Platform, Children, DiffProps, View};

use crate::{
    properties::{Property, PropertyValue},
    SvgAttribute, SvgCursor, SvgElement, SvgProps,
};

impl<P: Platform, A, C: Children<P>> View<P> for SvgElement<A, C>
where
    P::Cursor: SvgCursor,
    SvgProps<A>: DiffProps<P>,
{
    type State = ElementState<P, A, C>;

    fn init(self, cursor: &mut <P as Platform>::Cursor) -> Self::State {
        cursor.svg_element(self.tag_name);
        let props = self.props.init(cursor);
        let children = self.children.init(cursor);

        ElementState { props, children }
    }

    fn diff(self, state: &mut Self::State, cursor: &mut <P as Platform>::Cursor) {
        self.props.diff(&mut state.props, cursor);
        self.children.diff(&mut state.children, cursor);
    }
}

pub struct ElementState<P: Platform, A, C: Children<P>>
where
    P::Cursor: SvgCursor,
    SvgProps<A>: DiffProps<P>,
{
    props: <SvgProps<A> as DiffProps<P>>::State,
    children: C::State,
}

impl<P: Platform, const N: usize> DiffProps<P> for SvgProps<[Option<SvgAttribute>; N]>
where
    P::Cursor: SvgCursor,
{
    type State = Self;

    fn init(self, cursor: &mut P::Cursor) -> Self::State {
        for attr in self.0.iter() {
            match attr {
                Some(SvgAttribute::Svg(property)) => {
                    set_svg_attribute(cursor, property);
                }
                Some(SvgAttribute::Xml(property)) => {
                    cursor.set_xml_attribute(
                        property.namespace.url(),
                        property.name,
                        &property.value,
                    );
                }
                _ => {}
            }
        }

        self
    }

    fn diff(self, old_props: &mut Self::State, cursor: &mut P::Cursor) {
        for (new, old) in self.0.into_iter().zip(&mut old_props.0) {
            match (new, old) {
                (Some(SvgAttribute::Svg(new)), None) => {
                    set_svg_attribute(cursor, &new);
                }
                (Some(SvgAttribute::Svg(new)), Some(SvgAttribute::Svg(old))) => {
                    if new != *old {
                        set_svg_attribute(cursor, &new);
                    }
                }
                (None, Some(SvgAttribute::Svg(old))) => {
                    cursor.remove_svg_attribute(old.name);
                }
                (Some(SvgAttribute::Xml(new)), None) => {
                    cursor.set_xml_attribute(new.namespace.url(), new.name, &new.value);
                }
                (Some(SvgAttribute::Xml(new)), Some(SvgAttribute::Xml(old))) => {
                    if new != *old {
                        cursor.set_xml_attribute(new.namespace.url(), new.name, &new.value);
                    }
                }
                (None, Some(SvgAttribute::Xml(old))) => {
                    cursor.remove_xml_attribute(old.namespace.url(), old.name);
                }
                _ => {}
            }
        }
    }
}

fn set_svg_attribute(cursor: &mut impl SvgCursor, property: &Property) {
    let name = property.name;
    match &property.value {
        PropertyValue::String(string) => {
            cursor.set_svg_attribute(name, string);
        }
        PropertyValue::CommaSep(strings) => {
            let items = strings.iter().map(|s| -> &str { s }).collect::<Vec<_>>();
            cursor.set_svg_attribute(name, &items.join(", "));
        }
        PropertyValue::SpaceSep(strings) => {
            let items = strings.iter().map(|s| -> &str { s }).collect::<Vec<_>>();
            cursor.set_svg_attribute(name, &items.join(" "));
        }
        PropertyValue::Bool(bool) => {
            cursor.set_svg_attribute(name, &format!("{bool}"));
        }
        PropertyValue::Number(number) => {
            cursor.set_svg_attribute(name, &format!("{number}"));
        }
    }
}
