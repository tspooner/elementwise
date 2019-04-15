macro_rules! impl_ndarray {
    ($ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        impl<T, D> $nt for Array<T, D>
        where
            T: Copy + $ot<Output = T>,
            D: Dimension,
        {
            fn $nm(&self, other: &Array<T, D>) -> Array<T, D> {
                let values = self.into_iter().zip(other.into_iter()).map(|(x, &y)| x.$om(y));

                unsafe { Array::<T, D>::from_shape_vec_unchecked(self.dim(), values.collect()) }
            }
        }

        impl<T, D> $nt<T> for Array<T, D>
        where
            T: Copy + $ot<Output = T>,
            D: Dimension,
        {
            fn $nm(&self, other: &T) -> Array<T, D> {
                self.mapv(|v| v.$om(*other))
            }
        }
    };
}

macro_rules! impl_ndarray_unary {
    ($ot:ident.$om:ident => $nt:ident.$nm:ident) => {
        impl<T, D> $nt for Array<T, D>
        where
            T: Copy + $ot<Output = T>,
            D: Dimension,
        {
            fn $nm(&self) -> Array<T, D> {
                self.mapv(|v| v.$om())
            }
        }
    };
}
