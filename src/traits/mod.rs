use core::iter::*;
use core::ops::*;
use core::fmt::Debug;

pub mod generic;
pub mod float;
pub mod integer;

pub use generic::{Identity, Signed, Zero};

pub trait Numeric:
    Sized + Debug + Clone + Copy
    + PartialEq + PartialOrd
    + Sum<Self> + Product<Self>
    + Add<Self, Output = Self> + AddAssign<Self>
    + Div<Self, Output = Self> + DivAssign<Self>
    + Mul<Self, Output = Self> + MulAssign<Self>
    + Sub<Self, Output = Self> + SubAssign<Self>
    + Rem<Self, Output = Self> + RemAssign<Self>
    + Identity + Zero {}
