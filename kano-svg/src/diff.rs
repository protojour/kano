use kano::{markup::NestMarkup, Children, DiffProps, View};

use crate::{
    properties::{Property, PropertyValue},
    Svg1_1, SvgAttribute, SvgElement, SvgMarkup, SvgProps, SvgRootElement,
};

impl<P, M, A, C> View<P, M> for SvgRootElement<A, C>
where
    M: NestMarkup<P, Svg1_1>,
    <M as NestMarkup<P, Svg1_1>>::Nested: SvgMarkup<P>,
    SvgProps<A>: DiffProps<P, M::Nested>,
    C: Children<P, M::Nested>,
{
    type ConstState = (
        <SvgProps<A> as DiffProps<P, M::Nested>>::ConstState,
        C::ConstState,
    );
    type DiffState = (
        <SvgProps<A> as DiffProps<P, M::Nested>>::DiffState,
        C::DiffState,
    );

    fn init_const(self, outer_cursor: &mut M::Cursor) -> Self::ConstState {
        let mut svg_cursor = M::nest(outer_cursor);

        M::Nested::svg_element("svg", &mut svg_cursor);
        let props = self.props.init_const(&mut svg_cursor);
        let children = self.children.init_const(&mut svg_cursor);

        M::unnest(svg_cursor, outer_cursor);

        (props, children)
    }

    fn init_diff(self, outer_cursor: &mut M::Cursor) -> Self::DiffState {
        let mut svg_cursor = M::nest(outer_cursor);

        M::Nested::svg_element("svg", &mut svg_cursor);
        let props = self.props.init_diff(&mut svg_cursor);
        let children = self.children.init_diff(&mut svg_cursor);

        M::unnest(svg_cursor, outer_cursor);

        (props, children)
    }

    fn diff(self, (props, children): &mut Self::DiffState, cursor: &mut M::Cursor) {
        let mut svg_cursor = M::nest(cursor);
        self.props.diff(props, &mut svg_cursor);
        self.children.diff(children, &mut svg_cursor);
        M::unnest(svg_cursor, cursor);
    }
}

impl<P, M, A, C> View<P, M> for SvgElement<A, C>
where
    M: SvgMarkup<P>,
    SvgProps<A>: DiffProps<P, M>,
    C: Children<P, M>,
{
    type ConstState = (<SvgProps<A> as DiffProps<P, M>>::ConstState, C::ConstState);
    type DiffState = (<SvgProps<A> as DiffProps<P, M>>::DiffState, C::DiffState);

    fn init_const(self, cursor: &mut M::Cursor) -> Self::ConstState {
        M::svg_element(self.tag_name, cursor);
        let props = self.props.init_const(cursor);
        let children = self.children.init_const(cursor);

        (props, children)
    }

    fn init_diff(self, cursor: &mut M::Cursor) -> Self::DiffState {
        M::svg_element(self.tag_name, cursor);
        let props = self.props.init_diff(cursor);
        let children = self.children.init_diff(cursor);

        (props, children)
    }

    fn diff(self, (props, children): &mut Self::DiffState, cursor: &mut M::Cursor) {
        self.props.diff(props, cursor);
        self.children.diff(children, cursor);
    }
}

impl<P, M, const N: usize> DiffProps<P, M> for SvgProps<[Option<SvgAttribute>; N]>
where
    M: SvgMarkup<P>,
{
    type ConstState = ();
    type DiffState = Self;

    fn init_const(self, cursor: &mut M::Cursor) -> Self::ConstState {
        set_attributes::<P, M>(cursor, self.0.iter());
    }

    fn init_diff(self, cursor: &mut M::Cursor) -> Self::DiffState {
        set_attributes::<P, M>(cursor, self.0.iter());
        self
    }

    fn diff(self, old_props: &mut Self::DiffState, cursor: &mut M::Cursor) {
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

fn set_attributes<'a, P, M: SvgMarkup<P>>(
    cursor: &mut M::Cursor,
    it: impl Iterator<Item = &'a Option<SvgAttribute>>,
) {
    for attr in it {
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
