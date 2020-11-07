#[cfg(feature = "vec4-swizzle")]
mod swizzle;
#[cfg(test)]
mod tests;
use crate::traits::Identity;
#[cfg(feature = "vec4-swizzle")]
use crate::traits::Zero;
pub use swizzle::*;

use crate::angle::Radians;
use crate::traits::{
    float::{Float, Sqrt},
    Numeric,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Vec4<Value>([Value; 4]);

impl<Value> core::ops::Index<usize> for Vec4<Value> {
    type Output = Value;

    #[inline(always)]
    fn index(&self, i: usize) -> &Value {
        &self.0[i]
    }
}

impl<Value> core::ops::IndexMut<usize> for Vec4<Value> {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut Value {
        &mut self.0[i]
    }
}

macro_rules! make_math_operations {
    (@infix vec-vec ($Left:ty, $Right:ty), ($op:tt, $trait:ident, $fn:ident)) => {
        impl<Value: Numeric> core::ops::$trait<$Right> for $Left {
            type Output = Vec4<Value>;

            #[inline(always)]
            fn $fn(self, other: $Right) -> Vec4<Value> {
                Vec4([
                    self[0] $op other[0], // x
                    self[1] $op other[1], // y
                    self[2] $op other[2], // z
                    self[3] $op other[3], // w
                ])
            }
        }
    };

    (@infix vec-val ($Left:ty, $Right:ty), ($op:tt, $trait:ident, $fn:ident)) => {
        impl<Value: Numeric> core::ops::$trait<$Right> for $Left {
            type Output = Vec4<Value>;

            #[inline(always)]
            fn $fn(self, other: $Right) -> Vec4<Value> {
                Vec4([
                    self[0] $op other, // x
                    self[1] $op other, // y
                    self[2] $op other, // z
                    self[3] $op other, // w
                ])
            }
        }
    };

    (@infix vec-val deref ($Left:ty, $Right:ty), ($op:tt, $trait:ident, $fn:ident)) => {
        impl<Value: Numeric> core::ops::$trait<$Right> for $Left {
            type Output = Vec4<Value>;

            #[inline(always)]
            fn $fn(self, other: $Right) -> Vec4<Value> {
                Vec4([
                    self[0] $op *other, // x
                    self[1] $op *other, // y
                    self[2] $op *other, // z
                    self[3] $op *other, // w
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
                self[2] $op other[2]; // z
                self[3] $op other[3]; // w
            }
        }
    };

    (@assign vec-val ($Left:ty, $Right:ty), ($op:tt, $trait:ident, $fn:ident)) => {
        impl<Value: Copy + core::ops::$trait<Value>> core::ops::$trait<$Right> for $Left {
            #[inline(always)]
            fn $fn(&mut self, other: $Right) {
                self[0] $op other; // x
                self[1] $op other; // y
                self[2] $op other; // z
                self[3] $op other; // w
            }
        }
    };

    [@infix prim-vec ($Left:ty, $Right:ty), ($op:tt, $trait:ident, $fn:ident)] => {
        impl core::ops::$trait<$Right> for $Left {
            type Output = Vec4<$Left>;
            #[inline(always)]
            fn $fn(self, other: $Right) -> Vec4<$Left> {
                Vec4([
                    self $op other[0], // x
                    self $op other[1], // y
                    self $op other[2], // z
                    self $op other[3], // w
                ])
            }
        }
    };
    [@prim $data:tt $($type:ty),*] => {
        $(
            make_math_operations!(@infix prim-vec ($type, Vec4<$type>), $data );
            make_math_operations!(@infix prim-vec ($type, &Vec4<$type>), $data );
            make_math_operations!(@infix prim-vec ($type, &mut Vec4<$type>), $data );
        )*
    };

    [@unary neg ($Type:ty)] => {
        impl<Value: Copy + core::ops::Neg<Output = Value>> core::ops::Neg for $Type {
            type Output = Vec4<Value>;
            fn neg(self) -> Vec4<Value> {
                Vec4([-self[0], -self[1], -self[2], -self[3]])
            }
        }
    };

    ($($op1:tt, $tr1:ident, $f1:ident, $op2:tt, $tr2:ident, $f2:ident;)*) => {
        make_math_operations!(@unary neg (Vec4<Value>));
        make_math_operations!(@unary neg (&Vec4<Value>));
        make_math_operations!(@unary neg (&mut Vec4<Value>));
        $(
            make_math_operations!(@infix vec-vec (Vec4<Value>, Vec4<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (&Vec4<Value>, Vec4<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (&mut Vec4<Value>, Vec4<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (Vec4<Value>, &Vec4<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (&Vec4<Value>, &Vec4<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (&mut Vec4<Value>, &Vec4<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (Vec4<Value>, &mut Vec4<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (&Vec4<Value>, &mut Vec4<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (&mut Vec4<Value>, &mut Vec4<Value>), ($op1, $tr1, $f1)  );

            make_math_operations!(@infix vec-val (Vec4<Value>, Value), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-val (&Vec4<Value>, Value), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-val (&mut Vec4<Value>, Value), ($op1, $tr1, $f1)  );

            make_math_operations!(@infix vec-val deref (Vec4<Value>, &Value), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-val deref (&Vec4<Value>, &Value), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-val deref (&mut Vec4<Value>, &Value), ($op1, $tr1, $f1)  );

            make_math_operations!(@infix vec-val deref (Vec4<Value>, &mut Value), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-val deref (&Vec4<Value>, &mut Value), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-val deref (&mut Vec4<Value>, &mut Value), ($op1, $tr1, $f1)  );

            make_math_operations!(@assign vec-val (Vec4<Value>, Value), ($op2, $tr2, $f2)  );

            make_math_operations!(@assign vec-vec (Vec4<Value>, Vec4<Value>), ($op2, $tr2, $f2)  );
            make_math_operations!(@assign vec-vec (Vec4<Value>, &Vec4<Value>), ($op2, $tr2, $f2)  );
            make_math_operations!(@assign vec-vec (Vec4<Value>, &mut Vec4<Value>), ($op2, $tr2, $f2)  );

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

impl<Value> Vec4<Value> {
    pub const fn new(x: Value, y: Value, z: Value, w: Value) -> Self {
        Self([x, y, z, w])
    }

    pub fn iter(&self) -> core::slice::Iter<'_, Value> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Value> {
        self.0.iter_mut()
    }
}

impl<Value: Copy> Vec4<Value> {
    #[inline(always)]
    pub fn x(&self) -> Value {
        self[0]
    }
    #[inline(always)]
    pub fn y(&self) -> Value {
        self[1]
    }
    #[inline(always)]
    pub fn z(&self) -> Value {
        self[2]
    }
    #[inline(always)]
    pub fn w(&self) -> Value {
        self[3]
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
    pub fn set_z(&mut self, n: Value) {
        self[2] = n;
    }
    #[inline(always)]
    pub fn set_w(&mut self, n: Value) {
        self[3] = n;
    }

    #[inline(always)]
    pub fn as_array(&self) -> [Value; 4] {
        self.0
    }
}

impl<Value: Identity> Identity for Vec4<Value> {
    fn identity() -> Self {
        Self([
            Value::identity(), // x
            Value::identity(), // y
            Value::identity(), // z
            Value::identity(), // w
        ])
    }
}

impl<Value: Zero> Zero for Vec4<Value> {
    fn zero() -> Self {
        Self([
            Value::zero(), // x
            Value::zero(), // y
            Value::zero(), // z
            Value::zero(), // w
        ])
    }
}

impl<Value: Identity + Zero> Vec4<Value> {
    #[inline(always)]
    pub fn unit_x() -> Self {
        Self([
            Value::identity(), // x
            Value::zero(),     // y
            Value::zero(),     // z
            Value::zero(),     // w
        ])
    }

    #[inline(always)]
    pub fn unit_y() -> Self {
        Self([
            Value::zero(),     // x
            Value::identity(), // y
            Value::zero(),     // z
            Value::zero(),     // w
        ])
    }

    #[inline(always)]
    pub fn unit_z() -> Self {
        Self([
            Value::zero(),     // x
            Value::zero(),     // y
            Value::identity(), // z
            Value::zero(),     // w
        ])
    }

    #[inline(always)]
    pub fn unit_w() -> Self {
        Self([
            Value::zero(),     // x
            Value::zero(),     // y
            Value::zero(),     // z
            Value::identity(), // w
        ])
    }
}

impl<Value: Numeric> Vec4<Value> {
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

impl<Value: Numeric + Sqrt> Vec4<Value> {
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

impl<Value: Float> Vec4<Value> {
    /// angle between two vectors
    pub fn angle_between(&self, other: &Self) -> Radians<Value> {
        Radians::new((self.dot(other) / (self.magnitude() * other.magnitude())).acos())
    }
}

macro_rules! make_useful_constants {
    ($zero:expr, $one:expr, $($type:ty),*) => {
        $(impl Vec4<$type> {
            pub const ZERO: Self = Self::new($zero, $zero, $zero, $zero);
            pub const ONE: Self = Self::new($one, $one, $one, $one);
            pub const X: Self = Self::new($one, $zero, $zero, $zero);
            pub const Y: Self = Self::new($zero, $one, $zero, $zero);
            pub const Z: Self = Self::new($zero, $zero, $one, $zero);
            pub const W: Self = Self::new($zero, $zero, $zero, $one);
        })*
    };
    (neg $zero:expr, $one:expr, $($type:ty),*) => {
        make_useful_constants!($zero, $one, $($type),*);
        $(impl Vec4<$type> {
            pub const NEG_ONE: Self = Self::new(-$one, -$one, -$one, -$one);
            pub const NEG_X: Self = Self::new(-$one, $zero, $zero, $zero);
            pub const NEG_Y: Self = Self::new($zero, -$one, $zero, $zero);
            pub const NEG_Z: Self = Self::new($zero, $zero, -$one, $zero);
            pub const NEG_W: Self = Self::new($zero, $zero, $zero, -$one);
        })*
    };
}

make_useful_constants![neg 0.0, 1.0, f32, f64];
make_useful_constants![neg 0, 1, i8, i16, i32, i64, i128];
make_useful_constants![0, 1, u8, u16, u32, u64, u128];

impl<T> From<(T, T, T, T)> for Vec4<T> {
    fn from((x, y, z, w): (T, T, T, T)) -> Self {
        Self([x, y, z, w])
    }
}

impl<T> From<Vec4<T>> for (T, T, T, T) {
    fn from(Vec4([x, y, z, w]): Vec4<T>) -> Self {
        (x, y, z, w)
    }
}

impl<T> From<[T; 4]> for Vec4<T> {
    fn from(data: [T; 4]) -> Self {
        Self(data)
    }
}

impl<T> From<Vec4<T>> for [T; 4] {
    fn from(Vec4(data): Vec4<T>) -> Self {
        data
    }
}

impl<T: Copy> From<&[T; 4]> for Vec4<T> {
    fn from(data: &[T; 4]) -> Self {
        Self(*data)
    }
}

impl<T: Copy> From<&Vec4<T>> for [T; 4] {
    fn from(&Vec4(data): &Vec4<T>) -> Self {
        data
    }
}

impl<T: Copy> From<&mut [T; 4]> for Vec4<T> {
    fn from(data: &mut [T; 4]) -> Self {
        Self(*data)
    }
}

impl<T: Copy> From<&mut Vec4<T>> for [T; 4] {
    fn from(&mut Vec4(data): &mut Vec4<T>) -> Self {
        data
    }
}

impl<T: core::fmt::Display> core::fmt::Display for Vec4<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "⟨{}, {}, {}, {}⟩", self[0], self[1], self[2], self[3])
    }
}
