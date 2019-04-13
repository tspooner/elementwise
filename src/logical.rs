use std::{
    collections::{VecDeque, LinkedList},
    mem,
    ops::*,
    ptr,
};

new_trait!(BitOr.bitor => ElementwiseBitOr.elementwise_bitor);
impl_identity!(bool, usize, u8, u16, u32, u64, isize, i8, i16, i32, i64;
               BitOr.bitor => ElementwiseBitOr.elementwise_bitor);

new_trait!(BitAnd.bitand => ElementwiseBitAnd.elementwise_bitand);
impl_identity!(bool, usize, u8, u16, u32, u64, isize, i8, i16, i32, i64;
               BitAnd.bitand => ElementwiseBitAnd.elementwise_bitand);

new_trait!(BitXor.bitxor => ElementwiseBitXor.elementwise_bitxor);
impl_identity!(bool, usize, u8, u16, u32, u64, isize, i8, i16, i32, i64;
               BitXor.bitxor => ElementwiseBitXor.elementwise_bitxor);

new_trait_unary!(Not.not => ElementwiseNot.elementwise_not);
impl_identity_unary!(bool, usize, u8, u16, u32, u64, isize, i8, i16, i32, i64;
                     Not.not => ElementwiseNot.elementwise_not);
