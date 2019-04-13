macro_rules! impl_tuple {
    ($(($it:ident, $i:tt)),+; $ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        impl<$($it),*> $nt for ($($it),*)
        where
            $($it: Copy + $ot<Output = $it>),*
        {
            fn $nm(&self, other: &($($it),*)) -> ($($it),*) {
                ($(self.$i.$om(other.$i)),*)
            }
        }
    };
    ($ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        impl_tuple!((T1, 0), (T2, 1); $ot.$om => $nt.$nm);
        impl_tuple!((T1, 0), (T2, 1), (T3, 2); $ot.$om => $nt.$nm);
        impl_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3); $ot.$om => $nt.$nm);
        impl_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4); $ot.$om => $nt.$nm);
        impl_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5); $ot.$om => $nt.$nm);
        impl_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6);
                    $ot.$om => $nt.$nm);
        impl_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7);
                    $ot.$om => $nt.$nm);
        impl_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7),
                    (T9, 8); $ot.$om => $nt.$nm);
        impl_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7),
                    (T9, 8), (T10, 9); $ot.$om => $nt.$nm);
        impl_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7),
                    (T9, 8), (T10, 9), (T11, 10); $ot.$om => $nt.$nm);
        impl_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7),
                    (T9, 8), (T10, 9), (T11, 10), (T12, 11); $ot.$om => $nt.$nm);
    };
}

macro_rules! impl_tuple_unary {
    ($(($it:ident, $i:tt)),+; $ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        impl<$($it),*> $nt for ($($it),*)
        where
            $($it: Copy + $ot<Output = $it>),*
        {
            fn $nm(&self) -> ($($it),*) {
                ($(self.$i.$om()),*)
            }
        }
    };
    ($ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        impl_tuple_unary!((T1, 0), (T2, 1); $ot.$om => $nt.$nm);
        impl_tuple_unary!((T1, 0), (T2, 1), (T3, 2); $ot.$om => $nt.$nm);
        impl_tuple_unary!((T1, 0), (T2, 1), (T3, 2), (T4, 3); $ot.$om => $nt.$nm);
        impl_tuple_unary!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4); $ot.$om => $nt.$nm);
        impl_tuple_unary!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5);
                          $ot.$om => $nt.$nm);
        impl_tuple_unary!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6);
                          $ot.$om => $nt.$nm);
        impl_tuple_unary!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7);
                          $ot.$om => $nt.$nm);
        impl_tuple_unary!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7),
                          (T9, 8); $ot.$om => $nt.$nm);
        impl_tuple_unary!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7),
                          (T9, 8), (T10, 9); $ot.$om => $nt.$nm);
        impl_tuple_unary!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7),
                          (T9, 8), (T10, 9), (T11, 10); $ot.$om => $nt.$nm);
        impl_tuple_unary!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7),
                          (T9, 8), (T10, 9), (T11, 10), (T12, 11); $ot.$om => $nt.$nm);
    };
}
