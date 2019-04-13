macro_rules! impl_collection {
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

macro_rules! impl_collection_unary {
    ($collection:ident<$it:ident>; $ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        impl<$it: Copy + $ot<Output = $it>> $nt for $collection<$it> {
            fn $nm(&self) -> $collection<$it> {
                self.into_iter().map(|&x| x.$om()).collect()
            }
        }
    };
    ($ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        impl_collection_unary!(Vec<T>; $ot.$om => $nt.$nm);
        impl_collection_unary!(VecDeque<T>; $ot.$om => $nt.$nm);
        impl_collection_unary!(LinkedList<T>; $ot.$om => $nt.$nm);
    };
}
