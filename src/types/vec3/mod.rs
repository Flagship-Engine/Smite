#[cfg(feature = "vec3-swizzle")]
mod swizzle;
#[cfg(test)]
mod tests;
#[cfg(feature = "vec3-swizzle")]
pub use swizzle::*;

use crate::angle::Radians;
use crate::traits::{
    float::{Float, Sqrt},
    Identity, Numeric, Zero,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Vec3<Value>([Value; 3]);

impl<Value> core::ops::Index<usize> for Vec3<Value> {
    type Output = Value;

    #[inline(always)]
    fn index(&self, i: usize) -> &Value {
        &self.0[i]
    }
}

impl<Value> core::ops::IndexMut<usize> for Vec3<Value> {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut Value {
        &mut self.0[i]
    }
}

macro_rules! make_math_operations {
    (@infix vec-vec ($Left:ty, $Right:ty), ($op:tt, $trait:ident, $fn:ident)) => {
        impl<Value: Numeric> core::ops::$trait<$Right> for $Left {
            type Output = Vec3<Value>;

            #[inline(always)]
            fn $fn(self, other: $Right) -> Vec3<Value> {
                Vec3([
                    self[0] $op other[0], // x
                    self[1] $op other[1], // y
                    self[2] $op other[2], // z
                ])
            }
        }
    };

    (@infix vec-val ($Left:ty, $Right:ty), ($op:tt, $trait:ident, $fn:ident)) => {
        impl<Value: Numeric> core::ops::$trait<$Right> for $Left {
            type Output = Vec3<Value>;

            #[inline(always)]
            fn $fn(self, other: $Right) -> Vec3<Value> {
                Vec3([
                    self[0] $op other, // x
                    self[1] $op other, // y
                    self[2] $op other, // z
                ])
            }
        }
    };

    (@infix vec-val deref ($Left:ty, $Right:ty), ($op:tt, $trait:ident, $fn:ident)) => {
        impl<Value: Numeric> core::ops::$trait<$Right> for $Left {
            type Output = Vec3<Value>;

            #[inline(always)]
            fn $fn(self, other: $Right) -> Vec3<Value> {
                Vec3([
                    self[0] $op *other, // x
                    self[1] $op *other, // y
                    self[2] $op *other, // z
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
            }
        }
    };

    [@infix prim-vec ($Left:ty, $Right:ty), ($op:tt, $trait:ident, $fn:ident)] => {
        impl core::ops::$trait<$Right> for $Left {
            type Output = Vec3<$Left>;
            #[inline(always)]
            fn $fn(self, other: $Right) -> Vec3<$Left> {
                Vec3([
                    self $op other[0], // x
                    self $op other[1], // y
                    self $op other[2], // z
                ])
            }
        }
    };
    [@prim $data:tt $($type:ty),*] => {
        $(
            make_math_operations!(@infix prim-vec ($type, Vec3<$type>), $data );
            make_math_operations!(@infix prim-vec ($type, &Vec3<$type>), $data );
            make_math_operations!(@infix prim-vec ($type, &mut Vec3<$type>), $data );
        )*
    };

    [@unary neg ($Type:ty)] => {
        impl<Value: Copy + core::ops::Neg<Output = Value>> core::ops::Neg for $Type {
            type Output = Vec3<Value>;
            fn neg(self) -> Vec3<Value> {
                Vec3([-self[0], -self[1], -self[2]])
            }
        }
    };

    ($($op1:tt, $tr1:ident, $f1:ident, $op2:tt, $tr2:ident, $f2:ident;)*) => {
        make_math_operations!(@unary neg (Vec3<Value>));
        make_math_operations!(@unary neg (&Vec3<Value>));
        make_math_operations!(@unary neg (&mut Vec3<Value>));
        $(
            make_math_operations!(@infix vec-vec (Vec3<Value>, Vec3<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (&Vec3<Value>, Vec3<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (&mut Vec3<Value>, Vec3<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (Vec3<Value>, &Vec3<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (&Vec3<Value>, &Vec3<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (&mut Vec3<Value>, &Vec3<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (Vec3<Value>, &mut Vec3<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (&Vec3<Value>, &mut Vec3<Value>), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-vec (&mut Vec3<Value>, &mut Vec3<Value>), ($op1, $tr1, $f1)  );

            make_math_operations!(@infix vec-val (Vec3<Value>, Value), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-val (&Vec3<Value>, Value), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-val (&mut Vec3<Value>, Value), ($op1, $tr1, $f1)  );

            make_math_operations!(@infix vec-val deref (Vec3<Value>, &Value), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-val deref (&Vec3<Value>, &Value), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-val deref (&mut Vec3<Value>, &Value), ($op1, $tr1, $f1)  );

            make_math_operations!(@infix vec-val deref (Vec3<Value>, &mut Value), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-val deref (&Vec3<Value>, &mut Value), ($op1, $tr1, $f1)  );
            make_math_operations!(@infix vec-val deref (&mut Vec3<Value>, &mut Value), ($op1, $tr1, $f1)  );

            make_math_operations!(@assign vec-val (Vec3<Value>, Value), ($op2, $tr2, $f2)  );

            make_math_operations!(@assign vec-vec (Vec3<Value>, Vec3<Value>), ($op2, $tr2, $f2)  );
            make_math_operations!(@assign vec-vec (Vec3<Value>, &Vec3<Value>), ($op2, $tr2, $f2)  );
            make_math_operations!(@assign vec-vec (Vec3<Value>, &mut Vec3<Value>), ($op2, $tr2, $f2)  );

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

impl<Value> Vec3<Value> {
    pub const fn new(x: Value, y: Value, z: Value) -> Self {
        Self([x, y, z])
    }

    pub fn iter(&self) -> core::slice::Iter<'_, Value> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Value> {
        self.0.iter_mut()
    }
}

impl<Value: Copy> Vec3<Value> {
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
    pub fn as_array(&self) -> [Value; 3] {
        self.0
    }
}

impl<Value: Identity> Identity for Vec3<Value> {
    fn identity() -> Self {
        Self([
            Value::identity(), // x
            Value::identity(), // y
            Value::identity(), // z
        ])
    }
}

impl<Value: Zero> Zero for Vec3<Value> {
    fn zero() -> Self {
        Self([
            Value::zero(), // x
            Value::zero(), // y
            Value::zero(), // z
        ])
    }
}

impl<Value: Identity + Zero> Vec3<Value> {
    #[inline(always)]
    pub fn unit_x() -> Self {
        Self([
            Value::identity(), // x
            Value::zero(),     // y
            Value::zero(),     // z
        ])
    }

    #[inline(always)]
    pub fn unit_y() -> Self {
        Self([
            Value::zero(),     // x
            Value::identity(), // y
            Value::zero(),     // z
        ])
    }

    #[inline(always)]
    pub fn unit_z() -> Self {
        Self([
            Value::zero(),     // x
            Value::zero(),     // y
            Value::identity(), // z
        ])
    }
}

impl<Value: Numeric> Vec3<Value> {
    #[inline(always)]
    pub fn sum(&self) -> Value {
        self.iter().cloned().sum()
    }

    /// dot product of two vectors
    #[inline(always)]
    pub fn dot(&self, other: &Self) -> Value {
        self.iter().zip(other.iter()).map(|(&a, &b)| a * b).sum()
    }

    /// cross product of two vectors
    #[inline(always)]
    pub fn cross(&self, other: &Self) -> Self {
        let [x1, y1, z1] = self.0;
        let [x2, y2, z2] = other.0;

        Self([
            y1 * z2 - z1 * y2, // x
            z1 * x2 - x1 * z2, // y
            x1 * y2 - y1 * x2, // z
        ])
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

impl<Value: Numeric + Sqrt> Vec3<Value> {
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

impl<Value: Float> Vec3<Value> {
    #[inline(always)]
    pub fn rotate_x<Angle: Into<Radians<Value>>>(&mut self, angle: Angle) -> &mut Self {
        let (s, c) = angle.into().sin_cos();
        let [_, y, z] = self.0;
        self.set_y(c * y - s * z);
        self.set_z(s * y + c * z);
        self
    }

    #[inline(always)]
    pub fn rotate_y<Angle: Into<Radians<Value>>>(&mut self, angle: Angle) -> &mut Self {
        let (s, c) = angle.into().sin_cos();
        let [x, _, z] = self.0;
        self.set_x(c * x - s * z);
        self.set_z(-s * x + c * z);
        self
    }

    #[inline(always)]
    pub fn rotate_z<Angle: Into<Radians<Value>>>(&mut self, angle: Angle) -> &mut Self {
        let (s, c) = angle.into().sin_cos();
        let [x, y, _] = self.0;
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
        $(impl Vec3<$type> {
            pub const ZERO: Self = Self::new($zero, $zero, $zero);
            pub const ONE: Self = Self::new($one, $one, $one);
            pub const X: Self = Self::new($one, $zero, $zero);
            pub const Y: Self = Self::new($zero, $one, $zero);
            pub const Z: Self = Self::new($zero, $zero, $one);
        })*
    };
    (neg $zero:expr, $one:expr, $($type:ty),*) => {
        make_useful_constants!($zero, $one, $($type),*);
        $(impl Vec3<$type> {
            pub const NEG_ONE: Self = Self::new(-$one, -$one, -$one);
            pub const NEG_0: Self = Self::new(-$one, $zero, $zero);
            pub const NEG_1: Self = Self::new($zero, -$one, $zero);
            pub const NEG_2: Self = Self::new($zero, $zero, -$one);
        })*
    };
}

make_useful_constants![neg 0.0, 1.0, f32, f64];
make_useful_constants![neg 0, 1, i8, i16, i32, i64, i128];
make_useful_constants![0, 1, u8, u16, u32, u64, u128];

impl<T> From<(T, T, T)> for Vec3<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self([x, y, z])
    }
}

impl<T> From<Vec3<T>> for (T, T, T) {
    fn from(Vec3([x, y, z]): Vec3<T>) -> Self {
        (x, y, z)
    }
}

impl<T> From<[T; 3]> for Vec3<T> {
    fn from(data: [T; 3]) -> Self {
        Self(data)
    }
}

impl<T> From<Vec3<T>> for [T; 3] {
    fn from(Vec3(data): Vec3<T>) -> Self {
        data
    }
}

impl<T: Copy> From<&[T; 3]> for Vec3<T> {
    fn from(data: &[T; 3]) -> Self {
        Self(*data)
    }
}

impl<T: Copy> From<&Vec3<T>> for [T; 3] {
    fn from(&Vec3(data): &Vec3<T>) -> Self {
        data
    }
}

impl<T: Copy> From<&mut [T; 3]> for Vec3<T> {
    fn from(data: &mut [T; 3]) -> Self {
        Self(*data)
    }
}

impl<T: Copy> From<&mut Vec3<T>> for [T; 3] {
    fn from(&mut Vec3(data): &mut Vec3<T>) -> Self {
        data
    }
}

impl<T: core::fmt::Display> core::fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "⟨{}, {}, {}⟩", self[0], self[1], self[2])
    }
}
