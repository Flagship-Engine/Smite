#[cfg(feature = "vec2-swizzle")]
mod swizzle;
#[cfg(test)]
mod tests;
use crate::traits::Identity;
#[cfg(feature = "vec2-swizzle")]
use crate::traits::Zero;
pub use swizzle::*;

use crate::angle::Radians;
use crate::traits::{
    float::{Float, Sqrt},
    Numeric,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Vec2<Value>([Value; 2]);

impl<Value> core::ops::Index<usize> for Vec2<Value> {
    type Output = Value;

    #[inline(always)]
    fn index(&self, i: usize) -> &Value {
        &self.0[i]
    }
}

impl<Value> core::ops::IndexMut<usize> for Vec2<Value> {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut Value {
        &mut self.0[i]
    }
}

macro_rules! make_math_operations {
    (@infix vec-vec ($Left:ty, $Right:ty), ($op:tt, $trait:ident, $fn:ident)) => {
        impl<Value: Numeric> core::ops::$trait<$Right> for $Left {
            type Output = Vec2<Value>;

            #[inline(always)]
            fn $fn(self, other: $Right) -> Vec2<Value> {
                Vec2([
                    self[0] $op other[0], // x
                    self[1] $op other[1], // y
                ])
            }
        }
    };

    (@infix vec-val ($Left:ty, $Right:ty), ($op:tt, $trait:ident, $fn:ident)) => {
        impl<Value: Numeric> core::ops::$trait<$Right> for $Left {
            type Output = Vec2<Value>;

            #[inline(always)]
            fn $fn(self, other: $Right) -> Vec2<Value> {
                Vec2([
                    self[0] $op other, // x
                    self[1] $op other, // y
                ])
            }
        }
    };

    (@infix vec-val deref ($Left:ty, $Right:ty), ($op:tt, $trait:ident, $fn:ident)) => {
        impl<Value: Numeric> core::ops::$trait<$Right> for $Left {
            type Output = Vec2<Value>;

            #[inline(always)]
            fn $fn(self, other: $Right) -> Vec2<Value> {
                Vec2([
                    self[0] $op *other, // x
                    self[1] $op *other, // y
                ])
            }
        }
    };

    (@assign vec-vec ($Left:ty, $Right:ty), ($op:tt, $trait:ident, $fn:ident)) => {
        impl<Value: Copy + core::ops::$trait<Value>> core::ops::$trait<$Right> for $Left {
            #[inline(always)]
            fn $fn(&mut self, other: $Right) {
                self[0] $op other[0]; // x
                self[1] $op other[1]; // y
            }
        }
    };

    (@assign vec-val ($Left:ty, $Right:ty), ($op:tt, $trait:ident, $fn:ident)) => {
        impl<Value: Copy + core::ops::$trait<Value>> core::ops::$trait<$Right> for $Left {
            #[inline(always)]
            fn $fn(&mut self, other: $Right) {
                self[0] $op other; // x
                self[1] $op other; // y
            }
        }
    };

    [@infix prim-vec ($Left:ty, $Right:ty), ($op:tt, $trait:ident, $fn:ident)] => {
        impl core::ops::$trait<$Right> for $Left {
            type Output = Vec2<$Left>;
            #[inline(always)]
            fn $fn(self, other: $Right) -> Vec2<$Left> {
                Vec2([
                    self $op other[0], // x
                    self $op other[1], // y
                ])
            }
        }
    };
    [@prim $data:tt $($type:ty),*] => {
        $(
            make_math_operations!(@infix prim-vec ($type, Vec2<$type>), $data );
            make_math_operations!(@infix prim-vec ($type, &Vec2<$type>), $data );
            make_math_operations!(@infix prim-vec ($type, &mut Vec2<$type>), $data );
        )*
    };

    [@unary neg ($Type:ty)] => {
        impl<Value: Copy + core::ops::Neg<Output = Value>> core::ops::Neg for $Type {
            type Output = Vec2<Value>;
            fn neg(self) -> Vec2<Value> {
                Vec2([-self[0], -self[1]])
            }
        }
    };

    ($($op1:tt, $tr1:ident, $f1:ident, $op2:tt, $tr2:ident, $f2:ident;)*) => {
        make_math_operations!(@unary neg (Vec2<Value>));
        make_math_operations!(@unary neg (&Vec2<Value>));
        make_math_operations!(@unary neg (&mut Vec2<Value>));
        $(
            make_math_operations!(@infix vec-vec (Vec2<Value>, Vec2<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (&Vec2<Value>, Vec2<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (&mut Vec2<Value>, Vec2<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (Vec2<Value>, &Vec2<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (&Vec2<Value>, &Vec2<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (&mut Vec2<Value>, &Vec2<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (Vec2<Value>, &mut Vec2<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (&Vec2<Value>, &mut Vec2<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (&mut Vec2<Value>, &mut Vec2<Value>), ($op1, $tr1, $f1)  );

            make_math_operations!(@infix vec-val (Vec2<Value>, Value), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-val (&Vec2<Value>, Value), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-val (&mut Vec2<Value>, Value), ($op1, $tr1, $f1)  );

            make_math_operations!(@infix vec-val deref (Vec2<Value>, &Value), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-val deref (&Vec2<Value>, &Value), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-val deref (&mut Vec2<Value>, &Value), ($op1, $tr1, $f1)  );

            make_math_operations!(@infix vec-val deref (Vec2<Value>, &mut Value), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-val deref (&Vec2<Value>, &mut Value), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-val deref (&mut Vec2<Value>, &mut Value), ($op1, $tr1, $f1)  );

            make_math_operations!(@assign vec-val (Vec2<Value>, Value), ($op2, $tr2, $f2)  );

            make_math_operations!(@assign vec-vec (Vec2<Value>, Vec2<Value>), ($op2, $tr2, $f2)  );
            make_math_operations!(@assign vec-vec (Vec2<Value>, &Vec2<Value>), ($op2, $tr2, $f2)  );
            make_math_operations!(@assign vec-vec (Vec2<Value>, &mut Vec2<Value>), ($op2, $tr2, $f2)  );

            make_math_operations![@prim ($op1, $tr1, $f1) f32, f64, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128];
        )*
    };
}

make_math_operations! {
    +, Add, add, +=, AddAssign, add_assign;
    -, Sub, sub, -=, SubAssign, sub_assign;
    *, Mul, mul, *=, MulAssign, mul_assign;
    /, Div, div, /=, DivAssign, div_assign;
}

impl<Value> Vec2<Value> {
    pub const fn new(x: Value, y: Value) -> Self {
        Self([x, y])
    }

    pub fn iter(&self) -> core::slice::Iter<'_, Value> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Value> {
        self.0.iter_mut()
    }

    pub fn map<F>(mut self, f: F) -> Self
    where F: Fn(&Value) -> Value {
        self.iter_mut().for_each(|v| *v = f(v));
        self
    }
}

impl<Value: Copy> Vec2<Value> {
    #[inline(always)]
    pub fn x(&self) -> Value {
        self[0]
    }
    #[inline(always)]
    pub fn y(&self) -> Value {
        self[1]
    }

    #[inline(always)]
    pub fn set_x(&mut self, n: Value) {
        self[0] = n;
    }
    #[inline(always)]
    pub fn set_y(&mut self, n: Value) {
        self[1] = n;
    }

    #[inline(always)]
    pub fn as_array(&self) -> [Value; 2] {
        self.0
    }
}

impl<Value: Identity> Identity for Vec2<Value> {
    #[inline(always)]
    fn identity() -> Self {
        Self([
            Value::identity(), // x
            Value::identity(), // y
        ])
    }
}

impl<Value: Zero> Zero for Vec2<Value> {
    #[inline(always)]
    fn zero() -> Self {
        Self([
            Value::zero(), // x
            Value::zero(), // y
        ])
    }
}

impl<Value: Identity + Zero> Vec2<Value> {
    #[inline(always)]
    pub fn unit_x() -> Self {
        Self([
            Value::identity(), // x
            Value::zero(),     // y
        ])
    }

    #[inline(always)]
    pub fn unit_y() -> Self {
        Self([
            Value::zero(),     // x
            Value::identity(), // y
        ])
    }
}

impl<Value: Numeric> Vec2<Value> {
    #[inline(always)]
    pub fn sum(&self) -> Value {
        self.iter().cloned().sum()
    }

    /// dot product of two vectors
    #[inline(always)]
    pub fn dot(&self, other: &Self) -> Value {
        self.iter().zip(other.iter()).map(|(&a, &b)| a * b).sum()
    }

    #[inline(always)]
    pub fn square_magnitude(&self) -> Value {
        self.dot(self)
    }

    /// square distance between two vectors
    pub fn square_distance(&self, other: &Self) -> Value {
        (self - other).square_magnitude()
    }

    #[inline(always)]
    pub fn scale(&mut self, scalar: Value) -> &mut Self {
        self.iter_mut().for_each(|v| *v *= scalar);
        self
    }

    #[inline(always)]
    pub fn scale_by(&mut self, other: &Self) -> &mut Self {
        self.iter_mut()
            .zip(other.iter())
            .for_each(|(v, &s)| *v *= s);
        self
    }
}

impl<Value: Numeric + Sqrt> Vec2<Value> {
    #[inline(always)]
    pub fn magnitude(&self) -> Value {
        self.square_magnitude().sqrt()
    }

    /// distance between two vectors
    pub fn distance_to(&self, other: &Self) -> Value {
        (other - self).magnitude()
    }

    #[inline(always)]
    pub fn normalize(&mut self) -> &mut Self {
        let len = self.magnitude();
        if len > Value::zero() {
            self.iter_mut().for_each(|v| *v /= len);
        }
        self
    }
}

impl<Value: Float> Vec2<Value> {
    #[inline(always)]
    pub fn rotate<Angle: Into<Radians<Value>>>(&mut self, angle: Angle) -> &mut Self {
        let (s, c) = angle.into().sin_cos();
        let [x, y] = self.0;
        self.set_x(c * x - s * y);
        self.set_y(s * x + c * y);
        self
    }

    /// angle between two vectors
    pub fn angle_between(&self, other: &Self) -> Radians<Value> {
        Radians::new((self.dot(other) / (self.magnitude() * other.magnitude())).acos())
    }
}

macro_rules! make_useful_constants {
    ($zero:expr, $one:expr, $($type:ty),*) => {
        $(impl Vec2<$type> {
            pub const ZERO: Self = Self::new($zero, $zero);
            pub const ONE: Self = Self::new($one, $one);
            pub const X: Self = Self::new($one, $zero);
            pub const Y: Self = Self::new($zero, $one);
        })*
    };
    (neg $zero:expr, $one:expr, $($type:ty),*) => {
        make_useful_constants!($zero, $one, $($type),*);
        $(impl Vec2<$type> {
            pub const NEG_ONE: Self = Self::new(-$one, -$one);
            pub const NEG_X: Self = Self::new(-$one, $zero);
            pub const NEG_Y: Self = Self::new($zero, -$one);
        })*
    };
}

make_useful_constants![neg 0.0, 1.0, f32, f64];
make_useful_constants![neg 0, 1, i8, i16, i32, i64, i128];
make_useful_constants![0, 1, u8, u16, u32, u64, u128];

impl<T> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self {
        Self([x, y])
    }
}

impl<T> From<Vec2<T>> for (T, T) {
    fn from(Vec2([x, y]): Vec2<T>) -> Self {
        (x, y)
    }
}

impl<T> From<[T; 2]> for Vec2<T> {
    fn from(data: [T; 2]) -> Self {
        Self(data)
    }
}

impl<T> From<Vec2<T>> for [T; 2] {
    fn from(Vec2(data): Vec2<T>) -> Self {
        data
    }
}

impl<T: Copy> From<&[T; 2]> for Vec2<T> {
    fn from(data: &[T; 2]) -> Self {
        Self(*data)
    }
}

impl<T: Copy> From<&Vec2<T>> for [T; 2] {
    fn from(&Vec2(data): &Vec2<T>) -> Self {
        data
    }
}

impl<T: Copy> From<&mut [T; 2]> for Vec2<T> {
    fn from(data: &mut [T; 2]) -> Self {
        Self(*data)
    }
}

impl<T: Copy> From<&mut Vec2<T>> for [T; 2] {
    fn from(&mut Vec2(data): &mut Vec2<T>) -> Self {
        data
    }
}

impl<T: core::fmt::Display> core::fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "⟨{}, {}⟩", self[0], self[1])
    }
}
