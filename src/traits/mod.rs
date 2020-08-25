use core::iter::*;
use core::ops::*;

pub mod float;
pub mod integer;

pub trait Signed: Sized + Neg<Output = Self> {
    fn abs(&self) -> Self;
    fn signum(&self) -> Self;
}

pub trait Identity {
    fn identity() -> Self;
}

pub trait Zero {
    fn zero() -> Self;
}

impl<N: Numeric + Default> Zero for N {
    #[inline(always)]
    fn zero() -> Self { Self::default() }
}

pub trait Numeric:
    Sized + Clone + PartialEq + PartialOrd
    + Sum<Self> + Product<Self>
    + Add<Self, Output = Self> + AddAssign<Self>
    + Div<Self, Output = Self> + DivAssign<Self>
    + Mul<Self, Output = Self> + MulAssign<Self>
    + Sub<Self, Output = Self> + SubAssign<Self>
    + Zero + Identity {}
