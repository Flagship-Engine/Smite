use crate::traits::float::Trig;
use crate::traits::Numeric;

#[derive(Copy, Clone, Debug)]
pub struct Radians<N>(N);

impl<Number: Trig> Radians<Number> {
    pub fn new(n: Number) -> Self {
        Self(n)
    }

    #[inline(always)]
    pub fn acos(n: Number) -> Self {
        Self(n.acos())
    }
    #[inline(always)]
    pub fn asin(n: Number) -> Self {
        Self(n.asin())
    }
    #[inline(always)]
    pub fn atan(n: Number) -> Self {
        Self(n.atan())
    }
    #[inline(always)]
    pub fn cos(&self) -> Number {
        self.0.cos()
    }
    #[inline(always)]
    pub fn sin(&self) -> Number {
        self.0.sin()
    }
    #[inline(always)]
    pub fn sin_cos(&self) -> (Number, Number) {
        (self.sin(), self.cos())
    }
    #[inline(always)]
    pub fn tan(&self) -> Number {
        self.0.tan()
    }
    #[inline(always)]
    pub fn value(self) -> Number {
        self.0
    }
}

macro_rules! make_math_operations {
    (@infix rad_rad ($Left:ty, $Right:ty), ($op:tt, $trait:ident, $fn:ident)) => {
        impl<Value: Numeric> core::ops::$trait<$Right> for $Left {
            type Output = Radians<Value>;

            #[inline(always)]
            fn $fn(self, other: $Right) -> Radians<Value> {
                Radians(self.0 $op other.0)
            }
        }
    };

    (@infix rad_val ($Left:ty, $Right:ty), ($op:tt, $trait:ident, $fn:ident)) => {
        impl<Value: Numeric> core::ops::$trait<$Right> for $Left {
            type Output = Radians<Value>;

            #[inline(always)]
            fn $fn(self, other: $Right) -> Radians<Value> {
                Radians(self.0 $op other)
            }
        }
    };

    (@infix rad_val deref ($Left:ty, $Right:ty), ($op:tt, $trait:ident, $fn:ident)) => {
        impl<Value: Numeric> core::ops::$trait<$Right> for $Left {
            type Output = Radians<Value>;

            #[inline(always)]
            fn $fn(self, other: $Right) -> Radians<Value> {
                Radians(self.0 $op *other)
            }
        }
    };

    (@assign ($Left:ty, $Right:ty), ($op:tt, $trait:ident, $fn:ident)) => {
        impl<Value: Copy + core::ops::$trait<Value>> core::ops::$trait<$Right> for $Left {
            #[inline(always)]
            fn $fn(&mut self, other: $Right) {
                self.0 $op other.0;
            }
        }
    };

    [@infix prim-vec ($Left:ty, $Right:ty), ($op:tt, $trait:ident, $fn:ident)] => {
        impl core::ops::$trait<$Right> for $Left {
            type Output = Radians<$Left>;
            #[inline(always)]
            fn $fn(self, other: $Right) -> Radians<$Left> {
                Radians(self $op other.0)
            }
        }
    };
    [@prim $data:tt $($type:ty),*] => {
        $(
            make_math_operations!(@infix prim-vec ($type, Radians<$type>), $data );
            make_math_operations!(@infix prim-vec ($type, &Radians<$type>), $data );
            make_math_operations!(@infix prim-vec ($type, &mut Radians<$type>), $data );
        )*
    };

    [@unary neg ($Type:ty)] => {
        impl<Value: Copy + core::ops::Neg<Output = Value>> core::ops::Neg for $Type {
            type Output = Radians<Value>;
            fn neg(self) -> Radians<Value> {
                Radians(-self.0)
            }
        }
    };


    (@ rad_val $op1:tt, $tr1:ident, $f1:ident, $op2:tt, $tr2:ident, $f2:ident) => {
        make_math_operations!(@infix rad_val (Radians<Value>, Value), ($op1, $tr1, $f1)  );
        make_math_operations!(@infix rad_val (&Radians<Value>, Value), ($op1, $tr1, $f1)  );
        make_math_operations!(@infix rad_val (&mut Radians<Value>, Value), ($op1, $tr1, $f1)  );

        make_math_operations!(@infix rad_val deref (Radians<Value>, &Value), ($op1, $tr1, $f1)  );
        make_math_operations!(@infix rad_val deref (&Radians<Value>, &Value), ($op1, $tr1, $f1)  );
        make_math_operations!(@infix rad_val deref (&mut Radians<Value>, &Value), ($op1, $tr1, $f1)  );

        make_math_operations!(@infix rad_val deref (Radians<Value>, &mut Value), ($op1, $tr1, $f1)  );
        make_math_operations!(@infix rad_val deref (&Radians<Value>, &mut Value), ($op1, $tr1, $f1)  );
        make_math_operations!(@infix rad_val deref (&mut Radians<Value>, &mut Value), ($op1, $tr1, $f1)  );
    };
    (@ rad_rad $op1:tt, $tr1:ident, $f1:ident, $op2:tt, $tr2:ident, $f2:ident) => {
        make_math_operations!(@infix rad_rad (Radians<Value>, Radians<Value>), ($op1, $tr1, $f1)  );
        make_math_operations!(@infix rad_rad (&Radians<Value>, Radians<Value>), ($op1, $tr1, $f1)  );
        make_math_operations!(@infix rad_rad (&mut Radians<Value>, Radians<Value>), ($op1, $tr1, $f1)  );
        make_math_operations!(@infix rad_rad (Radians<Value>, &Radians<Value>), ($op1, $tr1, $f1)  );
        make_math_operations!(@infix rad_rad (&Radians<Value>, &Radians<Value>), ($op1, $tr1, $f1)  );
        make_math_operations!(@infix rad_rad (&mut Radians<Value>, &Radians<Value>), ($op1, $tr1, $f1)  );
        make_math_operations!(@infix rad_rad (Radians<Value>, &mut Radians<Value>), ($op1, $tr1, $f1)  );
        make_math_operations!(@infix rad_rad (&Radians<Value>, &mut Radians<Value>), ($op1, $tr1, $f1)  );
        make_math_operations!(@infix rad_rad (&mut Radians<Value>, &mut Radians<Value>), ($op1, $tr1, $f1)  );
    };

    ($($op1:tt, $tr1:ident, $f1:ident, $op2:tt, $tr2:ident, $f2:ident [ $($how:ident),* ];)*) => {
        make_math_operations!(@unary neg (Radians<Value>));
        make_math_operations!(@unary neg (&Radians<Value>));
        make_math_operations!(@unary neg (&mut Radians<Value>));
        $(
            $(make_math_operations!(@ $how $op1, $tr1, $f1, $op2, $tr2, $f2 );)*

            make_math_operations!(@assign (Radians<Value>, Radians<Value>), ($op2, $tr2, $f2)  );
            make_math_operations!(@assign (Radians<Value>, &Radians<Value>), ($op2, $tr2, $f2)  );
            make_math_operations!(@assign (Radians<Value>, &mut Radians<Value>), ($op2, $tr2, $f2)  );

            make_math_operations![@prim ($op1, $tr1, $f1) f32, f64, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128];
        )*
    };
}

make_math_operations! {
    +, Add, add, +=, AddAssign, add_assign [rad_rad, rad_val];
    -, Sub, sub, -=, SubAssign, sub_assign [rad_rad, rad_val];
    *, Mul, mul, *=, MulAssign, mul_assign [rad_val];
    /, Div, div, /=, DivAssign, div_assign [rad_val];
}

pub trait AsRadians<T> {
    fn as_radians(&self) -> Radians<T>;
}

impl<T: Trig + Copy> AsRadians<T> for T {
    fn as_radians(&self) -> Radians<T> {
        Radians::new(*self)
    }
}

impl<T: core::fmt::Display> core::fmt::Display for Radians<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{} rad", self.0)
    }
}
