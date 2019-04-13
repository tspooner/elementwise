macro_rules! impl_identity {
    ($($it:ty),*; $ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        $(
            impl $nt for $it {
                fn $nm(&self, other: &$it) -> $it {
                    self.$om(other)
                }
            }
        )*
    };
}

macro_rules! impl_identity_unary {
    ($($it:ty),*; $ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        $(
            impl $nt for $it {
                fn $nm(&self) -> $it {
                    self.$om()
                }
            }
        )*
    };
}
