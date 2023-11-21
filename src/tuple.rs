use crate::{Diff, List, Renderer};

macro_rules! tuples {
    ($(($t:ident, $i:tt)),+) => {
        impl<$($t: Diff),+> Diff for ($($t),+,) {
            type State = ($($t::State),+,);

            fn init<R: Renderer>(self, cursor: &mut R::Cursor) -> Self::State {
                R::enter_child(cursor);
                let ret = ($(self.$i.init::<R>(cursor)),+,);
                R::exit_child(cursor);
                ret
            }

            fn diff<R: Renderer>(self, state: &mut Self::State, cursor: &mut R::Cursor) {
                R::enter_child(cursor);
                $(self.$i.diff::<R>(&mut state.$i, cursor));+;
                R::exit_child(cursor);
            }
        }

        impl<$($t: Diff),+> List for ($($t),+,) {
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
