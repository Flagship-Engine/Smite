use super::{Identity, Numeric, Signed};

pub trait Integer: Numeric {}

macro_rules! wrapper {
    ($func: ident) => {
        #[inline(always)]
        fn $func(&self) -> Self {
            Self::$func(*self)
        }
    };
}

macro_rules! impl_uint {
    ($int: ty) => {
        impl Identity for $int {
            #[inline(always)]
            fn identity() -> Self { 1 }
        }
        impl Numeric for $int {}
        impl Integer for $int {}
    };
}

macro_rules! impl_int {
    ($int: ty) => {
        impl Signed for $int {
            wrapper!(abs);
            wrapper!(signum);
        }
        impl_uint!($int);
    };
}

impl_uint!(u8);
impl_uint!(u16);
impl_uint!(u32);
impl_uint!(u64);
impl_uint!(u128);
impl_uint!(usize);

impl_int!(i8);
impl_int!(i16);
impl_int!(i32);
impl_int!(i64);
impl_int!(i128);
impl_int!(isize);
