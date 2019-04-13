macro_rules! impl_array {
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

macro_rules! impl_array_unary {
    ([$it:ident; $length:expr]; $ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        impl<$it: Copy + $ot<Output = $it>> $nt for [$it; $length] {
            fn $nm(&self) -> [$it; $length] {
                let mut items: [$it; $length] = unsafe { mem::uninitialized() };

                for i in 0..$length {
                    unsafe {
                        ptr::write(&mut items[i], self[i].$om());
                    }
                }

                items
            }
        }
    };
    ([$it:ident; $($length:expr),*]; $ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        $(impl_array_unary!([$it; $length]; $ot.$om => $nt.$nm);)*
    };
    ($ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        impl_array_unary!([T;
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
            11, 12, 13, 14, 15, 16, 17, 18, 19,
            20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
            30, 31, 32
        ]; $ot.$om => $nt.$nm);
    };
}
