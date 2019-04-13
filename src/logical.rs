use std::{
    collections::{VecDeque, LinkedList},
    mem,
    ops::*,
    ptr,
};

new_trait!(BitOr.bitor => ElementwiseBitOr.elementwise_bitor);
new_trait!(BitAnd.bitand => ElementwiseBitAnd.elementwise_bitand);
new_trait!(BitXor.bitxor => ElementwiseBitXor.elementwise_bitxor);
new_trait_unary!(Not.not => ElementwiseNot.elementwise_not);
