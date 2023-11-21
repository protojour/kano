use crate::{Attr, AttrSet, Diff, List, Platform};

macro_rules! tuples {
    ($(($t:ident, $i:tt)),+) => {
        impl<$($t: Diff),+> Diff for ($($t),+,) {
            type State = ($($t::State),+,);

            fn init<P: Platform>(self, cursor: &mut P::Cursor) -> Self::State {
                P::enter_child(cursor);
                let ret = ($(self.$i.init::<P>(cursor)),+,);
                P::exit_child(cursor);
                ret
            }

            fn diff<P: Platform>(self, state: &mut Self::State, cursor: &mut P::Cursor) {
                P::enter_child(cursor);
                $(self.$i.diff::<P>(&mut state.$i, cursor));+;
                P::exit_child(cursor);
            }
        }

        impl<$($t: Diff),+> List for ($($t),+,) {
        }

        impl<$($t: Attr),+> AttrSet for ($($t),+,) {
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
