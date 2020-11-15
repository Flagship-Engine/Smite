#[cfg(test)]
mod tests;

use crate::angle::Radians;
use crate::traits::float::Float;
use crate::traits::{Identity, Numeric, Zero};
use crate::types::quat::Quat;
use crate::types::{Mat3, Vec3, Vec4};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Represents a 4x4 Matrix (using column-major order)
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Mat4<Value>([Value; 4 * 4]);

impl<Value: Identity + Zero> Identity for Mat4<Value> {
    fn identity() -> Self {
        Self([
            Value::identity(),
            Value::zero(),
            Value::zero(),
            Value::zero(),
            //
            Value::zero(),
            Value::identity(),
            Value::zero(),
            Value::zero(),
            //
            Value::zero(),
            Value::zero(),
            Value::identity(),
            Value::zero(),
            //
            Value::zero(),
            Value::zero(),
            Value::zero(),
            Value::identity(),
        ])
    }
}

impl<Value: Zero> Zero for Mat4<Value> {
    fn zero() -> Self {
        Self([
            Value::zero(),
            Value::zero(),
            Value::zero(),
            Value::zero(),
            //
            Value::zero(),
            Value::zero(),
            Value::zero(),
            Value::zero(),
            //
            Value::zero(),
            Value::zero(),
            Value::zero(),
            Value::zero(),
            //
            Value::zero(),
            Value::zero(),
            Value::zero(),
            Value::zero(),
        ])
    }
}

impl<Value: Identity + Zero> Default for Mat4<Value> {
    fn default() -> Self {
        Self::identity()
    }
}

pub mod consts {
    pub const R0C0: usize = 0;
    pub const R1C0: usize = 1;
    pub const R2C0: usize = 2;
    pub const R3C0: usize = 3;
    pub const R0C1: usize = 4;
    pub const R1C1: usize = 5;
    pub const R2C1: usize = 6;
    pub const R3C1: usize = 7;
    pub const R0C2: usize = 8;
    pub const R1C2: usize = 9;
    pub const R2C2: usize = 10;
    pub const R3C2: usize = 11;
    pub const R0C3: usize = 12;
    pub const R1C3: usize = 13;
    pub const R2C3: usize = 14;
    pub const R3C3: usize = 15;

    pub const TX: usize = R0C3;
    pub const TY: usize = R1C3;
    pub const TZ: usize = R2C3;

    pub const SX: usize = R0C0;
    pub const SY: usize = R1C1;
    pub const SZ: usize = R2C2;

    pub const RX00: usize = R1C1;
    pub const RX01: usize = R1C2;
    pub const RX10: usize = R2C1;
    pub const RX11: usize = R2C2;

    pub const RY00: usize = R0C0;
    pub const RY01: usize = R0C2;
    pub const RY10: usize = R2C0;
    pub const RY11: usize = R2C2;

    pub const RZ00: usize = R0C0;
    pub const RZ01: usize = R0C1;
    pub const RZ10: usize = R1C0;
    pub const RZ11: usize = R1C1;
}

type T4<T> = (T, T, T, T);

impl<Value> Mat4<Value> {
    #[must_use]
    #[inline(always)]
    pub fn from_columns(
        (r0c0, r1c0, r2c0, r3c0): T4<Value>, //
        (r0c1, r1c1, r2c1, r3c1): T4<Value>, //
        (r0c2, r1c2, r2c2, r3c2): T4<Value>, //
        (r0c3, r1c3, r2c3, r3c3): T4<Value>, //
    ) -> Self {
        Self([
            r0c0, r1c0, r2c0, r3c0, //
            r0c1, r1c1, r2c1, r3c1, //
            r0c2, r1c2, r2c2, r3c2, //
            r0c3, r1c3, r2c3, r3c3, //
        ])
    }

    #[must_use]
    #[inline(always)]
    pub fn from_rows(
        (r0c0, r0c1, r0c2, r0c3): T4<Value>, //
        (r1c0, r1c1, r1c2, r1c3): T4<Value>, //
        (r2c0, r2c1, r2c2, r2c3): T4<Value>, //
        (r3c0, r3c1, r3c2, r3c3): T4<Value>, //
    ) -> Self {
        Self([
            r0c0, r1c0, r2c0, r3c0, //
            r0c1, r1c1, r2c1, r3c1, //
            r0c2, r1c2, r2c2, r3c2, //
            r0c3, r1c3, r2c3, r3c3, //
        ])
    }

    #[must_use]
    #[inline(always)]
    /// Note: The data is assumed to be in column-major order
    pub fn from_array(data: [Value; 4 * 4]) -> Self {
        Self(data)
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[Value; 4 * 4] {
        &self.0
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *const Value {
        self.0.as_ptr()
    }

    pub fn iter(&self) -> core::slice::Iter<'_, Value> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Value> {
        self.0.iter_mut()
    }

    pub fn map<F>(mut self, f: F) -> Self
    where
        F: Fn(&Value) -> Value,
    {
        self.iter_mut().for_each(|v| *v = f(v));
        self
    }

    pub fn transposed(mut self) -> Self {
        self.0.swap(consts::R0C1, consts::R1C0);
        self.0.swap(consts::R0C2, consts::R2C0);
        self.0.swap(consts::R0C3, consts::R3C0);
        self.0.swap(consts::R1C2, consts::R2C1);
        self.0.swap(consts::R1C3, consts::R3C1);
        self.0.swap(consts::R2C3, consts::R3C2);
        self
    }
}

impl<Value> core::ops::Index<usize> for Mat4<Value> {
    type Output = Value;

    #[inline(always)]
    fn index(&self, i: usize) -> &Value {
        &self.0[i]
    }
}

impl<Value> core::ops::IndexMut<usize> for Mat4<Value> {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut Value {
        &mut self.0[i]
    }
}

impl<Value: Clone> Mat4<Value> {
    pub fn fill(&mut self, data: &[Value; 4 * 4]) {
        self.0.clone_from_slice(data);
    }
}

impl<Value: Identity + Zero> Mat4<Value> {
    #[must_use]
    #[inline(always)]
    pub fn from_affine(
        (r0c0, r0c1, r0c2, r0c3): T4<Value>, //
        (r1c0, r1c1, r1c2, r1c3): T4<Value>, //
        (r2c0, r2c1, r2c2, r2c3): T4<Value>, //
    ) -> Self {
        Self([
            r0c0,
            r1c0,
            r2c0,
            Value::zero(),
            //
            r0c1,
            r1c1,
            r2c1,
            Value::zero(),
            //
            r0c2,
            r1c2,
            r2c2,
            Value::zero(),
            //
            r0c3,
            r1c3,
            r2c3,
            Value::identity(),
        ])
    }
}

impl<Value: Copy + Identity + Zero> Mat4<Value> {
    #[must_use]
    #[inline(always)]
    pub fn from_mat3(m: &Mat3<Value>) -> Self {
        use super::mat3::consts::*;
        let zero = Value::zero();
        let one = Value::identity();
        Self([
            m[R0C0], m[R1C0], m[R2C0], zero, //
            m[R0C1], m[R1C1], m[R2C1], zero, //
            m[R0C2], m[R1C2], m[R2C2], zero, //
            zero, zero, zero, one, //
        ])
    }
}

macro_rules! entrywise {
        ($self:ident $op:tt $other:ident) => {
            $self.iter_mut()
            .zip($other.iter())
            .for_each(|(a, &b)| *a $op b);
        };
    }

// -------- Matrix addition --------

impl<Value: Numeric> AddAssign<&Mat4<Value>> for Mat4<Value> {
    #[inline(always)]
    fn add_assign(&mut self, other: &Mat4<Value>) {
        entrywise!(self += other);
    }
}

impl<Value: Numeric> Add<&Mat4<Value>> for Mat4<Value> {
    type Output = Mat4<Value>;
    #[inline(always)]
    fn add(mut self, other: &Mat4<Value>) -> Mat4<Value> {
        self += other;
        self
    }
}

// -------- Matrix subtraction --------

impl<Value: Numeric> SubAssign<&Mat4<Value>> for Mat4<Value> {
    #[inline(always)]
    fn sub_assign(&mut self, other: &Mat4<Value>) {
        entrywise!(self -= other);
    }
}

impl<Value: Numeric> Sub<&Mat4<Value>> for Mat4<Value> {
    type Output = Mat4<Value>;
    #[inline(always)]
    fn sub(mut self, other: &Mat4<Value>) -> Mat4<Value> {
        self -= other;
        self
    }
}

// -------- Matrix multiplication --------

impl<Value: Numeric> MulAssign<&Mat4<Value>> for Mat4<Value> {
    fn mul_assign(&mut self, other: &Mat4<Value>) {
        let s_r0c0 = self[consts::R0C0];
        let s_r0c1 = self[consts::R0C1];
        let s_r0c2 = self[consts::R0C2];
        let s_r0c3 = self[consts::R0C3];

        let s_r1c0 = self[consts::R1C0];
        let s_r1c1 = self[consts::R1C1];
        let s_r1c2 = self[consts::R1C2];
        let s_r1c3 = self[consts::R1C3];

        let s_r2c0 = self[consts::R2C0];
        let s_r2c1 = self[consts::R2C1];
        let s_r2c2 = self[consts::R2C2];
        let s_r2c3 = self[consts::R2C3];

        let s_r3c0 = self[consts::R3C0];
        let s_r3c1 = self[consts::R3C1];
        let s_r3c2 = self[consts::R3C2];
        let s_r3c3 = self[consts::R3C3];

        let o_r0c0 = other[consts::R0C0];
        let o_r0c1 = other[consts::R0C1];
        let o_r0c2 = other[consts::R0C2];
        let o_r0c3 = other[consts::R0C3];

        let o_r1c0 = other[consts::R1C0];
        let o_r1c1 = other[consts::R1C1];
        let o_r1c2 = other[consts::R1C2];
        let o_r1c3 = other[consts::R1C3];

        let o_r2c0 = other[consts::R2C0];
        let o_r2c1 = other[consts::R2C1];
        let o_r2c2 = other[consts::R2C2];
        let o_r2c3 = other[consts::R2C3];

        let o_r3c0 = other[consts::R3C0];
        let o_r3c1 = other[consts::R3C1];
        let o_r3c2 = other[consts::R3C2];
        let o_r3c3 = other[consts::R3C3];

        self[consts::R0C0] = s_r0c0 * o_r0c0 + s_r0c1 * o_r1c0 + s_r0c2 * o_r2c0 + s_r0c3 * o_r3c0;
        self[consts::R0C1] = s_r0c0 * o_r0c1 + s_r0c1 * o_r1c1 + s_r0c2 * o_r2c1 + s_r0c3 * o_r3c1;
        self[consts::R0C2] = s_r0c0 * o_r0c2 + s_r0c1 * o_r1c2 + s_r0c2 * o_r2c2 + s_r0c3 * o_r3c2;
        self[consts::R0C3] = s_r0c0 * o_r0c3 + s_r0c1 * o_r1c3 + s_r0c2 * o_r2c3 + s_r0c3 * o_r3c3;

        self[consts::R1C0] = s_r1c0 * o_r0c0 + s_r1c1 * o_r1c0 + s_r1c2 * o_r2c0 + s_r1c3 * o_r3c0;
        self[consts::R1C1] = s_r1c0 * o_r0c1 + s_r1c1 * o_r1c1 + s_r1c2 * o_r2c1 + s_r1c3 * o_r3c1;
        self[consts::R1C2] = s_r1c0 * o_r0c2 + s_r1c1 * o_r1c2 + s_r1c2 * o_r2c2 + s_r1c3 * o_r3c2;
        self[consts::R1C3] = s_r1c0 * o_r0c3 + s_r1c1 * o_r1c3 + s_r1c2 * o_r2c3 + s_r1c3 * o_r3c3;

        self[consts::R2C0] = s_r2c0 * o_r0c0 + s_r2c1 * o_r1c0 + s_r2c2 * o_r2c0 + s_r2c3 * o_r3c0;
        self[consts::R2C1] = s_r2c0 * o_r0c1 + s_r2c1 * o_r1c1 + s_r2c2 * o_r2c1 + s_r2c3 * o_r3c1;
        self[consts::R2C2] = s_r2c0 * o_r0c2 + s_r2c1 * o_r1c2 + s_r2c2 * o_r2c2 + s_r2c3 * o_r3c2;
        self[consts::R2C3] = s_r2c0 * o_r0c3 + s_r2c1 * o_r1c3 + s_r2c2 * o_r2c3 + s_r2c3 * o_r3c3;

        self[consts::R3C0] = s_r3c0 * o_r0c0 + s_r3c1 * o_r1c0 + s_r3c2 * o_r2c0 + s_r3c3 * o_r3c0;
        self[consts::R3C1] = s_r3c0 * o_r0c1 + s_r3c1 * o_r1c1 + s_r3c2 * o_r2c1 + s_r3c3 * o_r3c1;
        self[consts::R3C2] = s_r3c0 * o_r0c2 + s_r3c1 * o_r1c2 + s_r3c2 * o_r2c2 + s_r3c3 * o_r3c2;
        self[consts::R3C3] = s_r3c0 * o_r0c3 + s_r3c1 * o_r1c3 + s_r3c2 * o_r2c3 + s_r3c3 * o_r3c3;
    }
}

impl<Value: Numeric> Mul<&Mat4<Value>> for Mat4<Value> {
    type Output = Mat4<Value>;
    #[inline(always)]
    fn mul(mut self, other: &Mat4<Value>) -> Mat4<Value> {
        self *= other;
        self
    }
}

// -------- vector multiplication --------

impl<Value: Numeric> Mul<Vec4<Value>> for &Mat4<Value> {
    type Output = Vec4<Value>;
    #[inline(always)]
    fn mul(self, other: Vec4<Value>) -> Vec4<Value> {
        use consts::*;
        let [x, y, z, w]: [Value; 4] = other.into();
        Vec4::new(
            self[R0C0] * x + self[R0C1] * y + self[R0C2] * z + self[R0C3] * w,
            self[R1C0] * x + self[R1C1] * y + self[R1C2] * z + self[R1C3] * w,
            self[R2C0] * x + self[R2C1] * y + self[R2C2] * z + self[R2C3] * w,
            self[R3C0] * x + self[R3C1] * y + self[R3C2] * z + self[R3C3] * w,
        )
    }
}

impl<Value: Numeric> Mul<&Vec4<Value>> for &Mat4<Value> {
    type Output = Vec4<Value>;
    #[inline(always)]
    fn mul(self, &other: &Vec4<Value>) -> Vec4<Value> {
        self * other
    }
}

// -------- scalar multiplication --------

impl<Value: Numeric> MulAssign<Value> for Mat4<Value> {
    #[inline(always)]
    fn mul_assign(&mut self, other: Value) {
        self.iter_mut().for_each(|a| *a *= other);
    }
}

impl<Value: Numeric> MulAssign<&Value> for Mat4<Value> {
    #[inline(always)]
    fn mul_assign(&mut self, &other: &Value) {
        *self *= other;
    }
}

impl<Value: Numeric> Mul<Value> for Mat4<Value> {
    type Output = Mat4<Value>;
    #[inline(always)]
    fn mul(mut self, other: Value) -> Mat4<Value> {
        self *= other;
        self
    }
}

impl<Value: Numeric> Mul<Value> for &Mat4<Value> {
    type Output = Mat4<Value>;
    #[inline(always)]
    fn mul(self, other: Value) -> Mat4<Value> {
        *self * other
    }
}

impl<Value: Numeric> Mul<&Value> for Mat4<Value> {
    type Output = Mat4<Value>;
    #[inline(always)]
    fn mul(self, &other: &Value) -> Mat4<Value> {
        self * other
    }
}

impl<Value: Numeric> Mul<&Value> for &Mat4<Value> {
    type Output = Mat4<Value>;
    #[inline(always)]
    fn mul(self, &other: &Value) -> Mat4<Value> {
        *self * other
    }
}

// -------- scalar division --------

impl<Value: Numeric> DivAssign<Value> for Mat4<Value> {
    #[inline(always)]
    fn div_assign(&mut self, other: Value) {
        self.iter_mut().for_each(|a| *a /= other);
    }
}

impl<Value: Numeric> DivAssign<&Value> for Mat4<Value> {
    #[inline(always)]
    fn div_assign(&mut self, &other: &Value) {
        *self /= other;
    }
}

impl<Value: Numeric> Div<Value> for Mat4<Value> {
    type Output = Mat4<Value>;
    #[inline(always)]
    fn div(mut self, other: Value) -> Mat4<Value> {
        self /= other;
        self
    }
}

impl<Value: Numeric> Div<Value> for &Mat4<Value> {
    type Output = Mat4<Value>;
    #[inline(always)]
    fn div(self, other: Value) -> Mat4<Value> {
        *self / other
    }
}

impl<Value: Numeric> Div<&Value> for Mat4<Value> {
    type Output = Mat4<Value>;
    #[inline(always)]
    fn div(self, &other: &Value) -> Mat4<Value> {
        self / other
    }
}

impl<Value: Numeric> Div<&Value> for &Mat4<Value> {
    type Output = Mat4<Value>;
    #[inline(always)]
    fn div(self, &other: &Value) -> Mat4<Value> {
        *self / other
    }
}

// -------- Additional operations --------

impl<Value: Numeric> Mat4<Value> {
    pub fn from_translation(translation: Vec3<Value>) -> Mat4<Value> {
        let mut res = Self::identity();
        res[consts::TX] = translation.x();
        res[consts::TY] = translation.y();
        res[consts::TZ] = translation.z();
        res
    }

    pub fn from_scale(scale: Vec3<Value>) -> Mat4<Value> {
        let mut res = Self::identity();
        res[consts::SX] = scale.x();
        res[consts::SY] = scale.y();
        res[consts::SZ] = scale.z();
        res
    }
}

impl<Value: Numeric + Neg<Output = Value>> Mat4<Value> {
    pub fn adjugate_determinant(&self) -> (Self, Value) {
        let r0c0 = self[consts::R0C0];
        let r0c1 = self[consts::R0C1];
        let r0c2 = self[consts::R0C2];
        let r0c3 = self[consts::R0C3];
        let r1c0 = self[consts::R1C0];
        let r1c1 = self[consts::R1C1];
        let r1c2 = self[consts::R1C2];
        let r1c3 = self[consts::R1C3];
        let r2c0 = self[consts::R2C0];
        let r2c1 = self[consts::R2C1];
        let r2c2 = self[consts::R2C2];
        let r2c3 = self[consts::R2C3];
        let r3c0 = self[consts::R3C0];
        let r3c1 = self[consts::R3C1];
        let r3c2 = self[consts::R3C2];
        let r3c3 = self[consts::R3C3];

        let s1 = r0c0 * r1c1 - r1c0 * r0c1;
        let s2 = r0c0 * r1c2 - r1c0 * r0c2;
        let s3 = r0c0 * r1c3 - r1c0 * r0c3;
        let s4 = r0c1 * r1c2 - r1c1 * r0c2;
        let s5 = r0c1 * r1c3 - r1c1 * r0c3;
        let s6 = r0c2 * r1c3 - r1c2 * r0c3;

        let c6 = r2c2 * r3c3 - r3c2 * r2c3;
        let c5 = r2c1 * r3c3 - r3c1 * r2c3;
        let c4 = r2c1 * r3c2 - r3c1 * r2c2;
        let c3 = r2c0 * r3c3 - r3c0 * r2c3;
        let c2 = r2c0 * r3c2 - r3c0 * r2c2;
        let c1 = r2c0 * r3c1 - r3c0 * r2c1;

        let det = (s1 * c6 - s2 * c5 + s3 * c4) + (s4 * c3 - s5 * c2 + s6 * c1);

        (
            Self::from_rows(
                (
                    r1c1 * c6 - r1c2 * c5 + r1c3 * c4,
                    -r0c1 * c6 + r0c2 * c5 - r0c3 * c4,
                    r3c1 * s6 - r3c2 * s5 + r3c3 * s4,
                    -r2c1 * s6 + r2c2 * s5 - r2c3 * s4,
                ),
                (
                    -r1c0 * c6 + r1c2 * c3 - r1c3 * c2,
                    r0c0 * c6 - r0c2 * c3 + r0c3 * c2,
                    -r3c0 * s6 + r3c2 * s3 - r3c3 * s2,
                    r2c0 * s6 - r2c2 * s3 + r2c3 * s2,
                ),
                (
                    r1c0 * c5 - r1c1 * c3 + r1c3 * c1,
                    -r0c0 * c5 + r0c1 * c3 - r0c3 * c1,
                    r3c0 * s5 - r3c1 * s3 + r3c3 * s1,
                    -r2c0 * s5 + r2c1 * s3 - r2c3 * s1,
                ),
                (
                    -r1c0 * c4 + r1c1 * c2 - r1c2 * c1,
                    r0c0 * c4 - r0c1 * c2 + r0c2 * c1,
                    -r3c0 * s4 + r3c1 * s2 - r3c2 * s1,
                    r2c0 * s4 - r2c1 * s2 + r2c2 * s1,
                ),
            ),
            det,
        )
    }

    pub fn inverse(&self) -> Self {
        let (mut adj, det) = self.adjugate_determinant();
        assert!(
            det != Value::zero(),
            "Cannot calculate inverse of matrix: det == 0.0"
        );
        let invdet = Value::identity() / det;

        adj *= invdet;
        adj
    }
}

impl<Value: Float> Mat4<Value> {
    /// Assuming right-handed coordinated
    pub fn from_rotation_x<Angle: Into<Radians<Value>>>(angle: Angle) -> Mat4<Value> {
        let mut res = Self::identity();
        let (s, c) = angle.into().sin_cos();
        res[consts::RX00] = c;
        res[consts::RX01] = -s;
        res[consts::RX10] = s;
        res[consts::RX11] = c;
        res
    }

    /// Assuming right-handed coordinated
    pub fn from_rotation_y<Angle: Into<Radians<Value>>>(angle: Angle) -> Mat4<Value> {
        let mut res = Self::identity();
        let (s, c) = angle.into().sin_cos();
        res[consts::RY00] = c;
        res[consts::RY01] = s;
        res[consts::RY10] = -s;
        res[consts::RY11] = c;
        res
    }

    /// Assuming right-handed coordinated
    pub fn from_rotation_z<Angle: Into<Radians<Value>>>(angle: Angle) -> Mat4<Value> {
        let mut res = Self::identity();
        let (s, c) = angle.into().sin_cos();
        res[consts::RZ00] = c;
        res[consts::RZ01] = -s;
        res[consts::RZ10] = s;
        res[consts::RZ11] = c;
        res
    }

    pub fn from_quat(quat: &Quat<Value>) -> Mat4<Value> {
        let xx = quat.x() * quat.x();
        let xy = quat.x() * quat.y();
        let xz = quat.x() * quat.z();
        let xs = quat.x() * quat.s();
        let yy = quat.y() * quat.y();
        let yz = quat.y() * quat.z();
        let ys = quat.y() * quat.s();
        let zz = quat.z() * quat.z();
        let zs = quat.z() * quat.s();

        let zero = Value::zero();
        let one = Value::identity();
        let two = one + one;

        Self::from_affine(
            (
                one - (yy + zz) * two,
                (xy - zs) * two,
                (xz + ys) * two,
                zero,
            ),
            (
                (xy + zs) * two,
                one - (xx + zz) * two,
                (yz - xs) * two,
                zero,
            ),
            (
                (xz - ys) * two,
                (yz + xs) * two,
                one - (xx + yy) * two,
                zero,
            ),
        )
    }

    pub fn rotate_x<Angle: Into<Radians<Value>>>(&mut self, x_angle: Angle) {
        let r0c1 = self[consts::R0C1];
        let r0c2 = self[consts::R0C2];
        let r1c1 = self[consts::R1C1];
        let r1c2 = self[consts::R1C2];
        let r2c1 = self[consts::R2C1];
        let r2c2 = self[consts::R2C2];
        let r3c1 = self[consts::R3C1];
        let r3c2 = self[consts::R3C2];

        let (s, c) = x_angle.into().sin_cos();

        self[consts::R0C1] = c * r0c1 + s * r0c2;
        self[consts::R0C2] = c * r0c2 - s * r0c1;
        self[consts::R1C1] = c * r1c1 + s * r1c2;
        self[consts::R1C2] = c * r1c2 - s * r1c1;
        self[consts::R2C1] = c * r2c1 + s * r2c2;
        self[consts::R2C2] = c * r2c2 - s * r2c1;
        self[consts::R3C1] = c * r3c1 + s * r3c2;
        self[consts::R3C2] = c * r3c2 - s * r3c1;
    }

    pub fn rotate_y<Angle: Into<Radians<Value>>>(&mut self, angle_y: Angle) {
        let r0c0 = self[consts::R0C0];
        let r0c2 = self[consts::R0C2];
        let r1c0 = self[consts::R1C0];
        let r1c2 = self[consts::R1C2];
        let r2c0 = self[consts::R2C0];
        let r2c2 = self[consts::R2C2];
        let r3c0 = self[consts::R3C0];
        let r3c2 = self[consts::R3C2];

        let (s, c) = angle_y.into().sin_cos();

        self[consts::R0C0] = c * r0c0 - s * r0c2;
        self[consts::R0C2] = c * r0c2 + s * r0c0;
        self[consts::R1C0] = c * r1c0 - s * r1c2;
        self[consts::R1C2] = c * r1c2 + s * r1c0;
        self[consts::R2C0] = c * r2c0 - s * r2c2;
        self[consts::R2C2] = c * r2c2 + s * r2c0;
        self[consts::R3C0] = c * r3c0 - s * r3c2;
        self[consts::R3C2] = c * r3c2 + s * r3c0;
    }

    pub fn rotate_z<Angle: Into<Radians<Value>>>(&mut self, angle_z: Angle) {
        let r0c0 = self[consts::R0C0];
        let r0c1 = self[consts::R0C1];
        let r1c0 = self[consts::R1C0];
        let r1c1 = self[consts::R1C1];
        let r2c0 = self[consts::R2C0];
        let r2c1 = self[consts::R2C1];
        let r3c0 = self[consts::R3C0];
        let r3c1 = self[consts::R3C1];

        let (s, c) = angle_z.into().sin_cos();

        self[consts::R0C0] = c * r0c0 + s * r0c1;
        self[consts::R0C1] = c * r0c1 - s * r0c0;
        self[consts::R1C0] = c * r1c0 + s * r1c1;
        self[consts::R1C1] = c * r1c1 - s * r1c0;
        self[consts::R2C0] = c * r2c0 + s * r2c1;
        self[consts::R2C1] = c * r2c1 - s * r2c0;
        self[consts::R3C0] = c * r3c0 + s * r3c1;
        self[consts::R3C1] = c * r3c1 - s * r3c0;
    }
}

impl<Value: Float + Neg<Output = Value>> Mat4<Value> {
    pub fn from_axis_angle<Angle: Into<Radians<Value>>>(
        mut axis: Vec3<Value>,
        angle: Angle,
    ) -> Self {
        let (s, c) = angle.into().sin_cos();
        let t = Value::identity() - c;
        axis.normalize();
        let (x, y, z) = axis.into();

        let r0c0 = c + x * x * t;
        let r1c1 = c + y * y * t;
        let r2c2 = c + z * z * t;

        let x_y_t = x * y * t;
        let z_s = z * s;
        let r1c0 = x_y_t + z_s;
        let r0c1 = x_y_t - z_s;

        let x_z_t = x * z * t;
        let y_s = y * s;
        let r2c0 = x_z_t - y_s;
        let r0c2 = x_z_t + y_s;

        let y_z_t = y * z * t;
        let x_s = x * s;
        let r2c1 = y_z_t + x_s;
        let r1c2 = y_z_t - x_s;

        let zero = Value::zero();
        let one = Value::identity();

        Self::from_rows(
            (r0c0, r0c1, r0c2, zero), //
            (r1c0, r1c1, r1c2, zero), //
            (r2c0, r2c1, r2c2, zero), //
            (zero, zero, zero, one),  //
        )
    }

    pub fn from_orthogonal(
        left: Value,
        right: Value,
        bottom: Value,
        top: Value,
        near: Value,
        far: Value,
    ) -> Mat4<Value> {
        let zero = Value::zero();
        let one = Value::identity();
        let two = one + one;

        let dx = right - left;
        let dy = top - bottom;
        let dz = far - near;

        let tx = -(right + left) / dx;
        let ty = -(top + bottom) / dy;
        let tz = -(far + near) / dz;

        Mat4::from_affine(
            (two / dx, zero, zero, tx),  //
            (zero, two / dy, zero, ty),  //
            (zero, zero, -two / dz, tz), //
        )
    }

    pub fn from_orthogonal_symmetric(
        width: Value,
        height: Value,
        near: Value,
        far: Value,
    ) -> Mat4<Value> {
        let zero = Value::zero();
        let one = Value::identity();
        let two = one + one;

        let inv_depth = near - far;
        let tz = (far + near) / inv_depth;

        Mat4::from_affine(
            (two / width, zero, zero, zero),  //
            (zero, two / height, zero, zero),  //
            (zero, zero, two / inv_depth, tz), //
        )
    }

    pub fn from_perspective<Angle: Into<Radians<Value>>>(
        fov: Angle,
        aspect: Value,
        near: Value,
        far: Value,
    ) -> Mat4<Value> {
        let zero = Value::zero();
        let one = Value::identity();
        let two = one + one;

        let half_angle = fov.into().value() / two;
        let cot = one / half_angle.tan();
        let dz = far - near;

        Mat4::from_rows(
            (cot / aspect, zero, zero, zero), //
            (zero, cot, zero, zero), //
            (zero, zero, -(far + near) / dz, -two * near * far / dz), //
            (zero, zero, -one, zero), //
        )
    }
}

impl<T: core::fmt::Display> core::fmt::Display for Mat4<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "⟨ {} {} {} {} | {} {} {} {} | {} {} {} {} | {} {} {} {} ⟩",
            self[consts::R0C0],
            self[consts::R0C1],
            self[consts::R0C2],
            self[consts::R0C3],
            self[consts::R1C0],
            self[consts::R1C1],
            self[consts::R1C2],
            self[consts::R1C3],
            self[consts::R2C0],
            self[consts::R2C1],
            self[consts::R2C2],
            self[consts::R2C3],
            self[consts::R3C1],
            self[consts::R3C1],
            self[consts::R3C2],
            self[consts::R3C3],
        )
    }
}
