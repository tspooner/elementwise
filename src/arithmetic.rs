use std::{
    collections::{VecDeque, LinkedList},
    mem,
    ops::*,
    ptr,
};

macro_rules! impl_collection {
    ($collection:ident<$it:ident>; Neg.neg => $nt:ident.$nm:ident) => {
        impl<$it: Copy + Neg<Output = $it>> $nt for $collection<$it> {
            fn $nm(&self) -> $collection<$it> {
                self.into_iter().map(|&x| x.neg()).collect()
            }
        }
    };
    ($collection:ident<$it:ident>; $ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        impl<$it: Copy + $ot<Output = $it>> $nt for $collection<$it> {
            fn $nm(&self, other: &$collection<$it>) -> $collection<$it> {
                self.into_iter().zip(other.into_iter()).map(|(x, &y)| x.$om(y)).collect()
            }
        }

        impl<$it: Copy + $ot<Output = $it>> $nt<$it> for $collection<$it> {
            fn $nm(&self, other: &$it) -> $collection<$it> {
                self.into_iter().map(|x| x.$om(*other)).collect()
            }
        }
    };
    ($ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        impl_collection!(Vec<T>; $ot.$om => $nt.$nm);
        impl_collection!(VecDeque<T>; $ot.$om => $nt.$nm);
        impl_collection!(LinkedList<T>; $ot.$om => $nt.$nm);
    };
}

macro_rules! impl_array {
    ([$it:ident; $length:expr]; Neg.neg => $nt:ident.$nm:ident) => {
        impl<$it: Copy + Neg<Output = $it>> $nt for [$it; $length] {
            fn $nm(&self) -> [$it; $length] {
                let mut items: [$it; $length] = unsafe { mem::uninitialized() };

                for i in 0..$length {
                    unsafe {
                        ptr::write(&mut items[i], self[i].neg());
                    }
                }

                items
            }
        }
    };
    ([$it:ident; $length:expr]; $ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        impl<$it: Copy + $ot<Output = $it>> $nt for [$it; $length] {
            fn $nm(&self, other: &[$it; $length]) -> [$it; $length] {
                let mut items: [$it; $length] = unsafe { mem::uninitialized() };

                for i in 0..$length {
                    unsafe {
                        ptr::write(&mut items[i], self[i].$om(other[i]));
                    }
                }

                items
            }
        }

        impl<$it: Copy + $ot<Output = $it>> $nt<$it> for [$it; $length] {
            fn $nm(&self, other: &$it) -> [$it; $length] {
                let mut items: [$it; $length] = unsafe { mem::uninitialized() };

                for i in 0..$length {
                    unsafe {
                        ptr::write(&mut items[i], self[i].$om(*other));
                    }
                }

                items
            }
        }
    };
    ([$it:ident; $($length:expr),*]; $ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        $(impl_array!([$it; $length]; $ot.$om => $nt.$nm);)*
    };
    ($ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        impl_array!([T;
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            11, 12, 13, 14, 15, 16, 17, 18, 19,
            20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
            30, 31, 32
        ]; $ot.$om => $nt.$nm);
    };
}

macro_rules! impl_tuple {
    ($(($it:ident, $i:tt)),+; Neg.neg => $nt:ident.$nm:ident) => {
        impl<$($it),*> $nt for ($($it),*)
        where
            $($it: Copy + Neg<Output = $it>),*
        {
            fn $nm(&self) -> ($($it),*) {
                ($(self.$i.neg()),*)
            }
        }
    };
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

macro_rules! new_trait {
    ($ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        pub trait $nt<RHS = Self> {
            fn $nm(&self, v: &RHS) -> Self;
        }

        impl_collection!($ot.$om => $nt.$nm);
        impl_array!($ot.$om => $nt.$nm);
        impl_tuple!($ot.$om => $nt.$nm);
    }
}

new_trait!(Add.add => ElementwiseAdd.elementwise_add);
new_trait!(Div.div => ElementwiseDiv.elementwise_div);
new_trait!(Mul.mul => ElementwiseMul.elementwise_mul);
new_trait!(Rem.rem => ElementwiseRem.elementwise_rem);
new_trait!(Shl.shl => ElementwiseShl.elementwise_shl);
new_trait!(Shr.shr => ElementwiseShr.elementwise_shr);
new_trait!(Sub.sub => ElementwiseSub.elementwise_sub);

pub trait ElementwiseNeg {
    fn elementwise_neg(&self) -> Self;
}

impl_collection!(Neg.neg => ElementwiseNeg.elementwise_neg);
impl_array!(Neg.neg => ElementwiseNeg.elementwise_neg);
impl_tuple!(Neg.neg => ElementwiseNeg.elementwise_neg);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_scalar() {
        let a1 = [5.0, 6.0];

        assert_eq!(a1.elementwise_add(&1.0), [6.0, 7.0]);
    }

    #[test]
    fn test_add_arrays() {
        let a1 = [5.0, 6.0];
        let a2 = [-5.0, 4.0];

        assert_eq!(a1.elementwise_add(&a2), [0.0, 10.0]);
    }
    #[test]
    fn test_neg_arrays() {
        let a = [-5.0, 4.0];

        assert_eq!(a.elementwise_neg(), [5.0, -4.0]);
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
