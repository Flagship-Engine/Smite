use super::{Identity, Numeric, Signed};

pub trait Sqrt {
    fn sqrt(&self) -> Self;
}

pub trait Trig {
    fn acos(&self) -> Self;
    fn asin(&self) -> Self;
    fn atan(&self) -> Self;
    fn cos(&self) -> Self;
    fn sin(&self) -> Self;
    fn tan(&self) -> Self;
    fn cosh(&self) -> Self;
    fn sinh(&self) -> Self;
    fn tanh(&self) -> Self;
}

pub trait Float: Signed + Numeric + Sqrt + Trig {}

macro_rules! wrapper {
    ($func: ident) => {
        #[inline(always)]
        fn $func(&self) -> Self {
            <Self>::$func(*self)
        }
    };
}

macro_rules! impl_float {
    ($float: ty) => {
        impl Signed for $float {
            wrapper!(abs);
            wrapper!(signum);
        }
        impl Sqrt for $float {
            wrapper!(sqrt);
        }
        impl Trig for $float {
            wrapper!(acos);
            wrapper!(asin);
            wrapper!(atan);
            wrapper!(cos);
            wrapper!(sin);
            wrapper!(tan);
            wrapper!(cosh);
            wrapper!(sinh);
            wrapper!(tanh);
        }
        impl Identity for $float {
            #[inline(always)]
            fn identity() -> Self { 1.0 }
        }
        impl Numeric for $float {}
        impl Float for $float {}
    };
}

impl_float!(f32);
impl_float!(f64);
