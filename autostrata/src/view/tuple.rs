use crate::{Attr, AttrSet, Children, Cursor, Diff, Platform, View};

macro_rules! tuples {
    ($(($t:ident, $i:tt)),+) => {
        impl<P: Platform, $($t: Diff<P>),+> Diff<P> for ($($t),+,) {
            type State = ($($t::State),+,);

            fn init(self, cursor: &mut P::Cursor) -> Self::State {
                cursor.enter_children();
                // let ret = ($(self.$i.init(cursor)),+,);
                let ret = (
                    $({
                        let item = self.$i.init(cursor);
                        cursor.next_sibling();
                        item
                    }),+,
                );
                cursor.exit_children();
                ret
            }

            fn diff(self, state: &mut Self::State, cursor: &mut P::Cursor) {
                cursor.enter_children();
                $(
                    self.$i.diff(&mut state.$i, cursor);
                    cursor.next_sibling();
                )+
                cursor.exit_children();
            }
        }

        impl<P: Platform, $($t: View<P>),+> Children<P> for ($($t),+,) {
        }

        impl<P: Platform, $($t: Attr<P>),+> AttrSet<P> for ($($t),+,) {
        }
    }
}

tuples!((T0, 0));
tuples!((T0, 0), (T1, 1));
tuples!((T0, 0), (T1, 1), (T2, 2));
tuples!((T0, 0), (T1, 1), (T2, 2), (T3, 3));
tuples!((T0, 0), (T1, 1), (T2, 2), (T3, 3), (T4, 4));
tuples!((T0, 0), (T1, 1), (T2, 2), (T3, 3), (T4, 4), (T5, 5));
tuples!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6)
);
tuples!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7)
);
