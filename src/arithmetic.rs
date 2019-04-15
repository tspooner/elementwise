use std::{
    collections::{VecDeque, LinkedList},
    mem,
    ops::*,
    ptr,
};

#[cfg(feature = "ndarray")]
use ndarray::{Array, Dimension};

new_trait!(Add.add => ElementwiseAdd.elementwise_add);
impl_identity!(f32, f64, usize, u8, u16, u32, u64, isize, i8, i16, i32, i64;
               Add.add => ElementwiseAdd.elementwise_add);
#[cfg(feature = "ndarray")] impl_ndarray!(Add.add => ElementwiseAdd.elementwise_add);

new_trait!(Div.div => ElementwiseDiv.elementwise_div);
impl_identity!(f32, f64, usize, u8, u16, u32, u64, isize, i8, i16, i32, i64;
               Div.div => ElementwiseDiv.elementwise_div);
#[cfg(feature = "ndarray")] impl_ndarray!(Div.div => ElementwiseDiv.elementwise_div);

new_trait!(Mul.mul => ElementwiseMul.elementwise_mul);
impl_identity!(f32, f64, usize, u8, u16, u32, u64, isize, i8, i16, i32, i64;
               Mul.mul => ElementwiseMul.elementwise_mul);
#[cfg(feature = "ndarray")] impl_ndarray!(Mul.mul => ElementwiseMul.elementwise_mul);

new_trait!(Rem.rem => ElementwiseRem.elementwise_rem);
impl_identity!(f32, f64, usize, u8, u16, u32, u64, isize, i8, i16, i32, i64;
               Rem.rem => ElementwiseRem.elementwise_rem);
#[cfg(feature = "ndarray")] impl_ndarray!(Rem.rem => ElementwiseRem.elementwise_rem);

new_trait!(Shl.shl => ElementwiseShl.elementwise_shl);
impl_identity!(usize, u8, u16, u32, u64, isize, i8, i16, i32, i64;
               Shl.shl => ElementwiseShl.elementwise_shl);
#[cfg(feature = "ndarray")] impl_ndarray!(Shl.shl => ElementwiseShl.elementwise_shl);

new_trait!(Shr.shr => ElementwiseShr.elementwise_shr);
impl_identity!(usize, u8, u16, u32, u64, isize, i8, i16, i32, i64;
               Shr.shr => ElementwiseShr.elementwise_shr);
#[cfg(feature = "ndarray")] impl_ndarray!(Shr.shr => ElementwiseShr.elementwise_shr);

new_trait!(Sub.sub => ElementwiseSub.elementwise_sub);
impl_identity!(f32, f64, usize, u8, u16, u32, u64, isize, i8, i16, i32, i64;
               Sub.sub => ElementwiseSub.elementwise_sub);
#[cfg(feature = "ndarray")] impl_ndarray!(Sub.sub => ElementwiseSub.elementwise_sub);

new_trait_unary!(Neg.neg => ElementwiseNeg.elementwise_neg);
impl_identity_unary!(f32, f64, isize, i8, i16, i32, i64;
                     Neg.neg => ElementwiseNeg.elementwise_neg);
#[cfg(feature = "ndarray")] impl_ndarray_unary!(Neg.neg => ElementwiseNeg.elementwise_neg);

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
