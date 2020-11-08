use super::*;
use std::ops::Neg;

pub trait Identity {
    fn identity() -> Self;
}

pub trait Signed: Sized + Neg<Output = Self> {
    fn abs(&self) -> Self;
    fn signum(&self) -> Self;
}

pub trait Zero {
    fn zero() -> Self;
}

impl<N: Numeric + Default> Zero for N {
    #[inline(always)]
    fn zero() -> Self { Self::default() }
}
