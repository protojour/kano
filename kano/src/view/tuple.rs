use crate::{
    markup::{Cursor, Markup},
    Children, View,
};

macro_rules! tuples {
    ($(($t:ident, $i:tt)),+) => {
        impl<P, M: Markup<P>, $($t: View<P, M>),+> Children<P, M> for ($($t),+,) {
            type ConstState = ($($t::ConstState),+,);
            type DiffState = ($($t::DiffState),+,);

            fn init_const(self, cursor: &mut M::Cursor) -> Self::ConstState {
                cursor.enter_children();
                // let ret = ($(self.$i.init(cursor)),+,);
                let ret = (
                    $({
                        let item = self.$i.init_const(cursor);
                        cursor.next_sibling();
                        item
                    }),+,
                );
                cursor.exit_children();
                ret
            }

            fn init_diff(self, cursor: &mut M::Cursor) -> Self::DiffState {
                cursor.enter_children();
                // let ret = ($(self.$i.init(cursor)),+,);
                let ret = (
                    $({
                        let item = self.$i.init_diff(cursor);
                        cursor.next_sibling();
                        item
                    }),+,
                );
                cursor.exit_children();
                ret
            }

            fn diff(self, state: &mut Self::DiffState, cursor: &mut M::Cursor) {
                cursor.enter_children();
                $(
                    self.$i.diff(&mut state.$i, cursor);
                    cursor.next_sibling();
                )+
                cursor.exit_children();
            }
        }
    }
}

// FIXME: Have an upper limit on these, before the view macro changes into using Dyn arrays:
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
tuples!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8)
);
tuples!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9)
);
tuples!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9),
    (T10, 10)
);
tuples!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9),
    (T10, 10),
    (T11, 11)
);
tuples!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9),
    (T10, 10),
    (T11, 11),
    (T12, 12)
);
tuples!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9),
    (T10, 10),
    (T11, 11),
    (T12, 12),
    (T13, 13)
);
tuples!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9),
    (T10, 10),
    (T11, 11),
    (T12, 12),
    (T13, 13),
    (T14, 14)
);
tuples!(
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9),
    (T10, 10),
    (T11, 11),
    (T12, 12),
    (T13, 13),
    (T14, 14),
    (T15, 15)
);
