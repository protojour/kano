use kano::{markup::NestMarkup, Children, DiffProps, View};

use crate::{
    properties::{Property, PropertyValue},
    Svg1_1, SvgAttribute, SvgElement, SvgMarkup, SvgProps, SvgRootElement,
};

impl<P, M, A, C> View<P, M> for SvgElement<A, C>
where
    M: SvgMarkup<P>,
    SvgProps<A>: DiffProps<P, M>,
    C: Children<P, M>,
{
    type State = ElementState<P, M, A, C>;

    fn init(self, cursor: &mut M::Cursor) -> Self::State {
        M::svg_element(self.tag_name, cursor);
        let props = self.props.init(cursor);
        let children = self.children.init(cursor);

        ElementState { props, children }
    }

    fn diff(self, state: &mut Self::State, cursor: &mut M::Cursor) {
        self.props.diff(&mut state.props, cursor);
        self.children.diff(&mut state.children, cursor);
    }
}

impl<P, M, A, C> View<P, M> for SvgRootElement<A, C>
where
    M: NestMarkup<P, Svg1_1>,
    <M as NestMarkup<P, Svg1_1>>::Nested: SvgMarkup<P>,
    SvgProps<A>: DiffProps<P, M::Nested>,
    C: Children<P, M::Nested>,
{
    type State = ElementState<P, M::Nested, A, C>;

    fn init(self, cursor: &mut M::Cursor) -> Self::State {
        let mut svg_cursor = M::nest(cursor);

        M::Nested::svg_element("svg", &mut svg_cursor);
        let props = self.props.init(&mut svg_cursor);
        let children = self.children.init(&mut svg_cursor);

        M::unnest(svg_cursor, cursor);

        ElementState { props, children }
    }

    fn diff(self, state: &mut Self::State, cursor: &mut M::Cursor) {
        let mut svg_cursor = M::nest(cursor);
        self.props.diff(&mut state.props, &mut svg_cursor);
        self.children.diff(&mut state.children, &mut svg_cursor);
        M::unnest(svg_cursor, cursor);
    }
}

pub struct ElementState<P, M, A, C>
where
    M: SvgMarkup<P>,
    SvgProps<A>: DiffProps<P, M>,
    C: Children<P, M>,
{
    props: <SvgProps<A> as DiffProps<P, M>>::State,
    children: C::State,
}

impl<P, M, const N: usize> DiffProps<P, M> for SvgProps<[Option<SvgAttribute>; N]>
where
    M: SvgMarkup<P>,
{
    type State = Self;

    fn init(self, cursor: &mut M::Cursor) -> Self::State {
        for attr in self.0.iter() {
            match attr {
                Some(SvgAttribute::Svg(property)) => {
                    set_svg_attribute::<P, M>(cursor, property);
                }
                Some(SvgAttribute::Xml(property)) => {
                    M::set_xml_attribute(
                        property.namespace.url(),
                        property.name,
                        &property.value,
                        cursor,
                    );
                }
                _ => {}
            }
        }

        self
    }

    fn diff(self, old_props: &mut Self::State, cursor: &mut M::Cursor) {
        for (new, old) in self.0.into_iter().zip(&mut old_props.0) {
            match (new, old) {
                (Some(SvgAttribute::Svg(new)), None) => {
                    set_svg_attribute::<P, M>(cursor, &new);
                }
                (Some(SvgAttribute::Svg(new)), Some(SvgAttribute::Svg(old))) => {
                    if new != *old {
                        set_svg_attribute::<P, M>(cursor, &new);
                    }
                }
                (None, Some(SvgAttribute::Svg(old))) => {
                    M::remove_svg_attribute(old.name, cursor);
                }
                (Some(SvgAttribute::Xml(new)), None) => {
                    M::set_xml_attribute(new.namespace.url(), new.name, &new.value, cursor);
                }
                (Some(SvgAttribute::Xml(new)), Some(SvgAttribute::Xml(old))) => {
                    if new != *old {
                        M::set_xml_attribute(new.namespace.url(), new.name, &new.value, cursor);
                    }
                }
                (None, Some(SvgAttribute::Xml(old))) => {
                    M::remove_xml_attribute(old.namespace.url(), old.name, cursor);
                }
                _ => {}
            }
        }
    }
}

fn set_svg_attribute<P, M: SvgMarkup<P>>(cursor: &mut M::Cursor, property: &Property) {
    let name = property.name;
    match &property.value {
        PropertyValue::String(string) => {
            M::set_svg_attribute(name, string, cursor);
        }
        PropertyValue::CommaSep(strings) => {
            let items = strings.iter().map(|s| -> &str { s }).collect::<Vec<_>>();
            M::set_svg_attribute(name, &items.join(", "), cursor);
        }
        PropertyValue::SpaceSep(strings) => {
            let items = strings.iter().map(|s| -> &str { s }).collect::<Vec<_>>();
            M::set_svg_attribute(name, &items.join(" "), cursor);
        }
        PropertyValue::Bool(bool) => {
            M::set_svg_attribute(name, &format!("{bool}"), cursor);
        }
        PropertyValue::Number(number) => {
            M::set_svg_attribute(name, &format!("{number}"), cursor);
        }
    }
}
