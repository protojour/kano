use kano::{platform::Platform, Children, Diff, View};

use crate::{
    properties::{Property, PropertyValue},
    SvgAttribute, SvgCursor, SvgElement, SvgProps,
};

impl<P: Platform, A, C: Children<P>> Diff<P> for SvgElement<A, C>
where
    P::Cursor: SvgCursor,
    SvgProps<A>: Diff<P>,
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

impl<P: Platform, A, C: Children<P>> View<P> for SvgElement<A, C>
where
    P::Cursor: SvgCursor,
    SvgProps<A>: Diff<P>,
{
}

pub struct ElementState<P: Platform, A, C: Children<P>>
where
    P::Cursor: SvgCursor,
    SvgProps<A>: Diff<P>,
{
    props: <SvgProps<A> as Diff<P>>::State,
    children: C::State,
}

impl<P: Platform, const N: usize> Diff<P> for SvgProps<[Option<SvgAttribute>; N]>
where
    P::Cursor: SvgCursor,
{
    type State = Self;

    fn init(self, cursor: &mut P::Cursor) -> Self::State {
        for attr in self.0.iter() {
            if let Some(SvgAttribute(property)) = attr {
                set_svg_attribute(cursor, property);
            }
        }

        self
    }

    fn diff(self, old_props: &mut Self::State, cursor: &mut P::Cursor) {
        for (new, old) in self.0.into_iter().zip(&mut old_props.0) {
            match (new, old) {
                (Some(SvgAttribute(new)), None) => {
                    set_svg_attribute(cursor, &new);
                }
                (Some(SvgAttribute(new)), Some(SvgAttribute(old))) => {
                    if new != *old {
                        set_svg_attribute(cursor, &new);
                    }
                }
                (None, Some(SvgAttribute(old))) => {
                    cursor.remove_svg_attribute(old.idl_name);
                }
                (None, None) => {}
            }
        }
    }
}

fn set_svg_attribute(cursor: &mut impl SvgCursor, property: &Property) {
    let name = property.idl_name;
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
