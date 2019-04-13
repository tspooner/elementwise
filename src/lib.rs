use std::{
    collections::{VecDeque, LinkedList},
    mem,
    ops::*,
    ptr,
};

macro_rules! impl_iter {
    ($newtrait:ident => $newmethod:ident <- $oldtrait:ident => $oldmethod:ident) => {
        impl_iter!(Vec; $newtrait => $newmethod <- $oldtrait => $oldmethod);
        impl_iter!(VecDeque; $newtrait => $newmethod <- $oldtrait => $oldmethod);
        impl_iter!(LinkedList; $newtrait => $newmethod <- $oldtrait => $oldmethod);
    };
    ($col:ident; $newtrait:ident => $newmethod:ident <- $oldtrait:ident => $oldmethod:ident) => {
        impl<T: Copy + $oldtrait<Output = T>> $newtrait for $col<T> {
            fn $newmethod(&self, other: &$col<T>) -> $col<T> {
                self.into_iter().zip(other.into_iter()).map(|(&x, &y)| x.$oldmethod(y)).collect()
            }
        }
    };
}

macro_rules! impl_array {
    ($newtrait:ident => $newmethod:ident <- $oldtrait:ident => $oldmethod:ident) => {
        impl_array!([T;
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            11, 12, 13, 14, 15, 16, 17, 18, 19,
            20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
            30, 31, 32
        ]; $newtrait => $newmethod <- $oldtrait => $oldmethod);
    };
    ([$nt:ident; $size:expr]; $newtrait:ident => $newmethod:ident <- $oldtrait:ident => $oldmethod:ident) => {
        impl<$nt: Copy + $oldtrait<Output = $nt>> $newtrait for [$nt; $size] {
            fn $newmethod(&self, other: &[$nt; $size]) -> [$nt; $size] {
                let mut items: [$nt; $size] = unsafe { mem::uninitialized() };

                for i in 0..$size {
                    unsafe {
                        ptr::write(&mut items[i], self[i].$oldmethod(other[i]));
                    }
                }

                items
            }
        }
    };
    ([$nt:ident; $($size:expr),*]; $newtrait:ident => $newmethod:ident <- $oldtrait:ident => $oldmethod:ident) => {
        $(impl_array!([$nt; $size]; $newtrait => $newmethod <- $oldtrait => $oldmethod);)*
    };
}

macro_rules! impl_tuple {
    ($newtrait:ident => $newmethod:ident <- $oldtrait:ident => $oldmethod:ident) => {
        impl_tuple!((T1, 0), (T2, 1); $newtrait => $newmethod <- $oldtrait => $oldmethod);
        impl_tuple!((T1, 0), (T2, 1), (T3, 2); $newtrait => $newmethod <- $oldtrait => $oldmethod);
        impl_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3);
                    $newtrait => $newmethod <- $oldtrait => $oldmethod);
        impl_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4);
                    $newtrait => $newmethod <- $oldtrait => $oldmethod);
        impl_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5);
                    $newtrait => $newmethod <- $oldtrait => $oldmethod);
        impl_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6);
                    $newtrait => $newmethod <- $oldtrait => $oldmethod);
        impl_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7);
                    $newtrait => $newmethod <- $oldtrait => $oldmethod);
        impl_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7),
                    (T9, 8);
                    $newtrait => $newmethod <- $oldtrait => $oldmethod);
        impl_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7),
                    (T9, 8), (T10, 9);
                    $newtrait => $newmethod <- $oldtrait => $oldmethod);
        impl_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7),
                    (T9, 8), (T10, 9), (T11, 10);
                    $newtrait => $newmethod <- $oldtrait => $oldmethod);
        impl_tuple!((T1, 0), (T2, 1), (T3, 2), (T4, 3), (T5, 4), (T6, 5), (T7, 6), (T8, 7),
                    (T9, 8), (T10, 9), (T11, 10), (T12, 11);
                    $newtrait => $newmethod <- $oldtrait => $oldmethod);
    };
    ($(($type:ident, $i:tt)),+; $newtrait:ident => $newmethod:ident <- $oldtrait:ident => $oldmethod:ident) => {
        impl<$($type),*> $newtrait for ($($type),*)
        where
            $($type: Copy + $oldtrait<Output = $type>),*
        {
            fn $newmethod(&self, other: &($($type),*)) -> ($($type),*) {
                (
                    $(self.$i.$oldmethod(other.$i)),*
                )
            }
        }
    };
}

macro_rules! simple {
    ($newtrait:ident => $newmethod:ident <- $oldtrait:ident => $oldmethod:ident) => {
        pub trait $newtrait {
            fn $newmethod(&self, v: &Self) -> Self;
        }

        impl_iter!($newtrait => $newmethod <- $oldtrait => $oldmethod);
        impl_array!($newtrait => $newmethod <- $oldtrait => $oldmethod);
        impl_tuple!($newtrait => $newmethod <- $oldtrait => $oldmethod);
    }
}

simple!(ElementwiseAdd => elementwise_add <- Add => add);
simple!(ElementwiseDiv => elementwise_div <- Div => div);
simple!(ElementwiseMul => elementwise_mul <- Mul => mul);
simple!(ElementwiseRem => elementwise_rem <- Rem => rem);
simple!(ElementwiseShl => elementwise_shl <- Shl => shl);
simple!(ElementwiseShr => elementwise_shr <- Shr => shr);
simple!(ElementwiseSub => elementwise_sub <- Sub => sub);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_arrays() {
        let a1 = [5.0, 6.0];
        let a2 = [-5.0, 4.0];

        assert_eq!(a1.elementwise_add(&a2), [0.0, 10.0]);
    }

    #[test]
    fn test_mul_arrays() {
        let a1 = [5.0, 6.0];
        let a2 = [-5.0, 4.0];

        assert_eq!(a1.elementwise_mul(&a2), [-25.0, 24.0]);
    }

    #[test]
    fn test_add_mixed_tuples() {
        let t1 = (5.0f64, 2u32);
        let t2 = (1.0f64, 5u32);

        assert_eq!(t1.elementwise_mul(&t2), (5.0f64, 10u32));
    }
}
