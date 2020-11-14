#[cfg(test)]
mod tests;

use crate::angle::Radians;
use crate::traits::float::Float;
use crate::traits::{Identity, Numeric, Zero};
use crate::types::quat::Quat;
use crate::types::{Vec2, Vec3};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Represents a 3x3 Matrix (using column-major order)
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Mat3<Value>([Value; 3 * 3]);

impl<Value: Identity + Zero> Identity for Mat3<Value> {
    fn identity() -> Self {
        Self([
            Value::identity(),
            Value::zero(),
            Value::zero(),
            //
            Value::zero(),
            Value::identity(),
            Value::zero(),
            //
            Value::zero(),
            Value::zero(),
            Value::identity(),
        ])
    }
}

impl<Value: Zero> Zero for Mat3<Value> {
    fn zero() -> Self {
        Self([
            Value::zero(),
            Value::zero(),
            Value::zero(),
            //
            Value::zero(),
            Value::zero(),
            Value::zero(),
            //
            Value::zero(),
            Value::zero(),
            Value::zero(),
        ])
    }
}

impl<Value: Identity + Zero> Default for Mat3<Value> {
    fn default() -> Self {
        Self::identity()
    }
}

pub mod consts {
    pub const R0C0: usize = 0;
    pub const R1C0: usize = 1;
    pub const R2C0: usize = 2;
    pub const R0C1: usize = 3;
    pub const R1C1: usize = 4;
    pub const R2C1: usize = 5;
    pub const R0C2: usize = 6;
    pub const R1C2: usize = 7;
    pub const R2C2: usize = 8;

    pub const TX: usize = R0C2;
    pub const TY: usize = R1C2;

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

type T3<T> = (T, T, T);

impl<Value> Mat3<Value> {
    #[must_use]
    #[inline(always)]
    pub fn from_columns(
        (r0c0, r1c0, r2c0): T3<Value>, //
        (r0c1, r1c1, r2c1): T3<Value>, //
        (r0c2, r1c2, r2c2): T3<Value>, //
    ) -> Self {
        Self([
            r0c0, r1c0, r2c0, //
            r0c1, r1c1, r2c1, //
            r0c2, r1c2, r2c2, //
        ])
    }

    #[must_use]
    #[inline(always)]
    pub fn from_rows(
        (r0c0, r0c1, r0c2): T3<Value>, //
        (r1c0, r1c1, r1c2): T3<Value>, //
        (r2c0, r2c1, r2c2): T3<Value>, //
    ) -> Self {
        Self([
            r0c0, r1c0, r2c0, //
            r0c1, r1c1, r2c1, //
            r0c2, r1c2, r2c2, //
        ])
    }

    #[must_use]
    #[inline(always)]
    /// Note: The data is assumed to be in column-major order
    pub fn from_array(data: [Value; 3 * 3]) -> Self {
        Self(data)
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[Value; 3 * 3] {
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
        self.0.swap(consts::R1C2, consts::R2C1);
        self
    }
}

impl<Value> core::ops::Index<usize> for Mat3<Value> {
    type Output = Value;

    #[inline(always)]
    fn index(&self, i: usize) -> &Value {
        &self.0[i]
    }
}

impl<Value> core::ops::IndexMut<usize> for Mat3<Value> {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut Value {
        &mut self.0[i]
    }
}

impl<Value: Clone> Mat3<Value> {
    pub fn fill(&mut self, data: &[Value; 3 * 3]) {
        self.0.clone_from_slice(data);
    }
}

impl<Value: Identity + Zero> Mat3<Value> {
    #[must_use]
    #[inline(always)]
    pub fn from_affine_2d(
        (r0c0, r0c1, r0c2): T3<Value>, //
        (r1c0, r1c1, r1c2): T3<Value>, //
    ) -> Self {
        Self([
            r0c0,
            r1c0,
            Value::zero(),
            //
            r0c1,
            r1c1,
            Value::zero(),
            //
            r0c2,
            r1c2,
            Value::identity(),
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

impl<Value: Numeric> AddAssign<&Mat3<Value>> for Mat3<Value> {
    #[inline(always)]
    fn add_assign(&mut self, other: &Mat3<Value>) {
        entrywise!(self += other);
    }
}

impl<Value: Numeric> Add<&Mat3<Value>> for Mat3<Value> {
    type Output = Mat3<Value>;
    #[inline(always)]
    fn add(mut self, other: &Mat3<Value>) -> Mat3<Value> {
        self += other;
        self
    }
}

// -------- Matrix subtraction --------

impl<Value: Numeric> SubAssign<&Mat3<Value>> for Mat3<Value> {
    #[inline(always)]
    fn sub_assign(&mut self, other: &Mat3<Value>) {
        entrywise!(self -= other);
    }
}

impl<Value: Numeric> Sub<&Mat3<Value>> for Mat3<Value> {
    type Output = Mat3<Value>;
    #[inline(always)]
    fn sub(mut self, other: &Mat3<Value>) -> Mat3<Value> {
        self -= other;
        self
    }
}

// -------- Matrix multiplication --------

impl<Value: Numeric> MulAssign<&Mat3<Value>> for Mat3<Value> {
    fn mul_assign(&mut self, other: &Mat3<Value>) {
        let s_r0c0 = self[consts::R0C0];
        let s_r0c1 = self[consts::R0C1];
        let s_r0c2 = self[consts::R0C2];

        let s_r1c0 = self[consts::R1C0];
        let s_r1c1 = self[consts::R1C1];
        let s_r1c2 = self[consts::R1C2];

        let s_r2c0 = self[consts::R2C0];
        let s_r2c1 = self[consts::R2C1];
        let s_r2c2 = self[consts::R2C2];

        let o_r0c0 = other[consts::R0C0];
        let o_r0c1 = other[consts::R0C1];
        let o_r0c2 = other[consts::R0C2];

        let o_r1c0 = other[consts::R1C0];
        let o_r1c1 = other[consts::R1C1];
        let o_r1c2 = other[consts::R1C2];

        let o_r2c0 = other[consts::R2C0];
        let o_r2c1 = other[consts::R2C1];
        let o_r2c2 = other[consts::R2C2];

        self[consts::R0C0] = s_r0c0 * o_r0c0 + s_r0c1 * o_r1c0 + s_r0c2 * o_r2c0;
        self[consts::R0C1] = s_r0c0 * o_r0c1 + s_r0c1 * o_r1c1 + s_r0c2 * o_r2c1;
        self[consts::R0C2] = s_r0c0 * o_r0c2 + s_r0c1 * o_r1c2 + s_r0c2 * o_r2c2;

        self[consts::R1C0] = s_r1c0 * o_r0c0 + s_r1c1 * o_r1c0 + s_r1c2 * o_r2c0;
        self[consts::R1C1] = s_r1c0 * o_r0c1 + s_r1c1 * o_r1c1 + s_r1c2 * o_r2c1;
        self[consts::R1C2] = s_r1c0 * o_r0c2 + s_r1c1 * o_r1c2 + s_r1c2 * o_r2c2;

        self[consts::R2C0] = s_r2c0 * o_r0c0 + s_r2c1 * o_r1c0 + s_r2c2 * o_r2c0;
        self[consts::R2C1] = s_r2c0 * o_r0c1 + s_r2c1 * o_r1c1 + s_r2c2 * o_r2c1;
        self[consts::R2C2] = s_r2c0 * o_r0c2 + s_r2c1 * o_r1c2 + s_r2c2 * o_r2c2;
    }
}

impl<Value: Numeric> Mul<&Mat3<Value>> for Mat3<Value> {
    type Output = Mat3<Value>;
    #[inline(always)]
    fn mul(mut self, other: &Mat3<Value>) -> Mat3<Value> {
        self *= other;
        self
    }
}

// -------- vector multiplication --------

impl<Value: Numeric> Mul<Vec3<Value>> for &Mat3<Value> {
    type Output = Vec3<Value>;
    #[inline(always)]
    fn mul(self, other: Vec3<Value>) -> Vec3<Value> {
        use consts::*;
        let [x, y, z]: [Value; 3] = other.into();
        Vec3::new(
            self[R0C0] * x + self[R0C1] * y + self[R0C2] * z,
            self[R1C0] * x + self[R1C1] * y + self[R1C2] * z,
            self[R2C0] * x + self[R2C1] * y + self[R2C2] * z,
        )
    }
}

impl<Value: Numeric> Mul<&Vec3<Value>> for &Mat3<Value> {
    type Output = Vec3<Value>;
    #[inline(always)]
    fn mul(self, &other: &Vec3<Value>) -> Vec3<Value> {
        self * other
    }
}

// -------- scalar multiplication --------

impl<Value: Numeric> MulAssign<Value> for Mat3<Value> {
    #[inline(always)]
    fn mul_assign(&mut self, other: Value) {
        self.iter_mut().for_each(|a| *a *= other);
    }
}

impl<Value: Numeric> MulAssign<&Value> for Mat3<Value> {
    #[inline(always)]
    fn mul_assign(&mut self, &other: &Value) {
        *self *= other;
    }
}

impl<Value: Numeric> Mul<Value> for Mat3<Value> {
    type Output = Mat3<Value>;
    #[inline(always)]
    fn mul(mut self, other: Value) -> Mat3<Value> {
        self *= other;
        self
    }
}

impl<Value: Numeric> Mul<Value> for &Mat3<Value> {
    type Output = Mat3<Value>;
    #[inline(always)]
    fn mul(self, other: Value) -> Mat3<Value> {
        *self * other
    }
}

impl<Value: Numeric> Mul<&Value> for Mat3<Value> {
    type Output = Mat3<Value>;
    #[inline(always)]
    fn mul(self, &other: &Value) -> Mat3<Value> {
        self * other
    }
}

impl<Value: Numeric> Mul<&Value> for &Mat3<Value> {
    type Output = Mat3<Value>;
    #[inline(always)]
    fn mul(self, &other: &Value) -> Mat3<Value> {
        *self * other
    }
}

// -------- scalar division --------

impl<Value: Numeric> DivAssign<Value> for Mat3<Value> {
    #[inline(always)]
    fn div_assign(&mut self, other: Value) {
        self.iter_mut().for_each(|a| *a /= other);
    }
}

impl<Value: Numeric> DivAssign<&Value> for Mat3<Value> {
    #[inline(always)]
    fn div_assign(&mut self, &other: &Value) {
        *self /= other;
    }
}

impl<Value: Numeric> Div<Value> for Mat3<Value> {
    type Output = Mat3<Value>;
    #[inline(always)]
    fn div(mut self, other: Value) -> Mat3<Value> {
        self /= other;
        self
    }
}

impl<Value: Numeric> Div<Value> for &Mat3<Value> {
    type Output = Mat3<Value>;
    #[inline(always)]
    fn div(self, other: Value) -> Mat3<Value> {
        *self / other
    }
}

impl<Value: Numeric> Div<&Value> for Mat3<Value> {
    type Output = Mat3<Value>;
    #[inline(always)]
    fn div(self, &other: &Value) -> Mat3<Value> {
        self / other
    }
}

impl<Value: Numeric> Div<&Value> for &Mat3<Value> {
    type Output = Mat3<Value>;
    #[inline(always)]
    fn div(self, &other: &Value) -> Mat3<Value> {
        *self / other
    }
}

// -------- Additional operations --------

impl<Value: Numeric> Mat3<Value> {
    pub fn from_translation_2d(translation: Vec2<Value>) -> Mat3<Value> {
        let mut res = Self::identity();
        res[consts::TX] = translation.x();
        res[consts::TY] = translation.y();
        res
    }

    pub fn from_scale_2d(scale: Vec2<Value>) -> Mat3<Value> {
        let mut res = Self::identity();
        res[consts::SX] = scale.x();
        res[consts::SY] = scale.y();
        res
    }

    pub fn from_scale(scale: Vec3<Value>) -> Mat3<Value> {
        let mut res = Self::identity();
        res[consts::SX] = scale.x();
        res[consts::SY] = scale.y();
        res[consts::SZ] = scale.z();
        res
    }
}

impl<Value: Numeric + Neg<Output = Value>> Mat3<Value> {
    pub fn adjugate_determinant(&self) -> (Self, Value) {
        let r0c0 = self[consts::R0C0];
        let r0c1 = self[consts::R0C1];
        let r0c2 = self[consts::R0C2];

        let r1c0 = self[consts::R1C0];
        let r1c1 = self[consts::R1C1];
        let r1c2 = self[consts::R1C2];
        let r2c0 = self[consts::R2C0];
        let r2c1 = self[consts::R2C1];
        let r2c2 = self[consts::R2C2];

        let det = r0c0 * (r1c1 * r2c2 - r2c1 * r1c2) //.
                - r0c1 * (r1c0 * r2c2 - r1c2 * r2c0) //.
                + r0c2 * (r1c0 * r2c1 - r1c1 * r2c0);

        (
            Self::from_rows(
                (
                    r1c1 * r2c2 - r2c1 * r1c2,
                    r0c2 * r2c1 - r0c1 * r2c2,
                    r0c1 * r1c2 - r0c2 * r1c1,
                ),
                (
                    r1c2 * r2c0 - r1c0 * r2c2,
                    r0c0 * r2c2 - r0c2 * r2c0,
                    r1c0 * r0c2 - r0c0 * r1c2,
                ),
                (
                    r1c0 * r2c1 - r2c0 * r1c1,
                    r2c0 * r0c1 - r0c0 * r2c1,
                    r0c0 * r1c1 - r1c0 * r0c1,
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

impl<Value: Float> Mat3<Value> {
    /// Assuming right-handed coordinated
    pub fn from_rotation_x<Angle: Into<Radians<Value>>>(angle: Angle) -> Mat3<Value> {
        let mut res = Self::identity();
        let (s, c) = angle.into().sin_cos();
        res[consts::RX00] = c;
        res[consts::RX01] = -s;
        res[consts::RX10] = s;
        res[consts::RX11] = c;
        res
    }

    /// Assuming right-handed coordinated
    pub fn from_rotation_y<Angle: Into<Radians<Value>>>(angle: Angle) -> Mat3<Value> {
        let mut res = Self::identity();
        let (s, c) = angle.into().sin_cos();
        res[consts::RY00] = c;
        res[consts::RY01] = s;
        res[consts::RY10] = -s;
        res[consts::RY11] = c;
        res
    }

    /// Assuming right-handed coordinated
    pub fn from_rotation_z<Angle: Into<Radians<Value>>>(angle: Angle) -> Mat3<Value> {
        let mut res = Self::identity();
        let (s, c) = angle.into().sin_cos();
        res[consts::RZ00] = c;
        res[consts::RZ01] = -s;
        res[consts::RZ10] = s;
        res[consts::RZ11] = c;
        res
    }

    /// Assuming right-handed coordinated
    pub fn from_rotation_2d<Angle: Into<Radians<Value>>>(angle: Angle) -> Mat3<Value> {
        Self::from_rotation_z(angle)
    }

    pub fn from_quat(quat: &Quat<Value>) -> Mat3<Value> {
        let xx = quat.x() * quat.x();
        let xy = quat.x() * quat.y();
        let xz = quat.x() * quat.z();
        let xs = quat.x() * quat.s();
        let yy = quat.y() * quat.y();
        let yz = quat.y() * quat.z();
        let ys = quat.y() * quat.s();
        let zz = quat.z() * quat.z();
        let zs = quat.z() * quat.s();

        let one = Value::identity();
        let two = one + one;

        Self::from_rows(
            (one - (yy + zz) * two, (xy - zs) * two, (xz + ys) * two),
            ((xy + zs) * two, one - (xx + zz) * two, (yz - xs) * two),
            ((xz - ys) * two, (yz + xs) * two, one - (xx + yy) * two),
        )
    }

    pub fn rotate_2d<Angle: Into<Radians<Value>>>(&mut self, angle: Angle) {
        let r0c0 = self[consts::R0C0];
        let r0c1 = self[consts::R0C1];
        let r1c0 = self[consts::R1C0];
        let r1c1 = self[consts::R1C1];
        let r2c0 = self[consts::R2C0];
        let r2c1 = self[consts::R2C1];

        let (s, c) = angle.into().sin_cos();

        self[consts::R0C0] = c * r0c0 + s * r0c1;
        self[consts::R0C1] = c * r0c1 - s * r0c0;
        self[consts::R1C0] = c * r1c0 + s * r1c1;
        self[consts::R1C1] = c * r1c1 - s * r1c0;
        self[consts::R2C0] = c * r2c0 + s * r2c1;
        self[consts::R2C1] = c * r2c1 - s * r2c0;
    }
}

impl<Value: Float + Neg<Output = Value>> Mat3<Value> {
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

        Self::from_rows(
            (r0c0, r0c1, r0c2), //
            (r1c0, r1c1, r1c2), //
            (r2c0, r2c1, r2c2), //
        )
    }
}

impl<T: core::fmt::Display> core::fmt::Display for Mat3<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "⟨ {} {} {} | {} {} {} | {} {} {} ⟩",
            self[consts::R0C0],
            self[consts::R0C1],
            self[consts::R0C2],
            self[consts::R1C0],
            self[consts::R1C1],
            self[consts::R1C2],
            self[consts::R2C0],
            self[consts::R2C1],
            self[consts::R2C2],
        )
    }
}
