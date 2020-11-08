use super::{Identity, Numeric, Signed};

pub trait Round {
    fn round(&self) -> Self;
    fn floor(&self) -> Self;
    fn trunc(&self) -> Self;
    fn ceil(&self) -> Self;
}

pub trait Sqrt {
    fn sqrt(&self) -> Self;
}

pub trait Trig {
    // Trigonometric functions
    fn acos(&self) -> Self;
    fn asin(&self) -> Self;
    fn atan(&self) -> Self;
    fn cos(&self) -> Self;
    fn sin(&self) -> Self;
    fn tan(&self) -> Self;
    
    // Trigonometric Constants
    fn pi() -> Self;
}

pub trait Float: Signed + Numeric + Round + Sqrt + Trig {}

macro_rules! wrapper {
    ($func: ident) => {
        #[inline(always)]
        fn $func(&self) -> Self {
            <Self>::$func(*self)
        }
    };
}

macro_rules! impl_float {
    ($float: ident) => {
        impl Round for $float {
            wrapper!(round);
            wrapper!(floor);
            wrapper!(trunc);
            wrapper!(ceil);
        }
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
            
            #[inline(always)]
            fn pi() -> Self { core::$float::consts::PI }
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
