use std::{
    collections::{VecDeque, LinkedList},
    mem,
    ops::*,
    ptr,
};

macro_rules! new_trait {
    ($ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        pub trait $nt<RHS = Self> {
            fn $nm(&self, other: &RHS) -> Self;
        }

        impl_collection!($ot.$om => $nt.$nm);
        impl_array!($ot.$om => $nt.$nm);
        impl_tuple!($ot.$om => $nt.$nm);
    };
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

impl_collection_unary!(Neg.neg => ElementwiseNeg.elementwise_neg);
impl_array_unary!(Neg.neg => ElementwiseNeg.elementwise_neg);
impl_tuple_unary!(Neg.neg => ElementwiseNeg.elementwise_neg);

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
