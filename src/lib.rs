#[cfg(feature = "ndarray")]
extern crate ndarray;

#[macro_use]
pub(crate) mod macros;

#[cfg(feature = "arithmetic")]
pub mod arithmetic;

#[cfg(feature = "logical")]
pub mod logical;
