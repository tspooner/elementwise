#[macro_use]
mod collection;
pub use self::collection::*;

#[macro_use]
mod array;
pub use self::array::*;

#[macro_use]
mod tuple;
pub use self::tuple::*;

#[macro_use]
mod identity;
pub use self::identity::*;

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

macro_rules! new_trait_unary {
    ($ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        pub trait $nt {
            fn $nm(&self) -> Self;
        }

        impl_collection_unary!($ot.$om => $nt.$nm);
        impl_array_unary!($ot.$om => $nt.$nm);
        impl_tuple_unary!($ot.$om => $nt.$nm);
    };
}
