#[cfg(test)]
mod tests;

use crate::angle::Radians;
use crate::traits::float::Float;
use crate::traits::float::Sqrt;
use crate::traits::{Identity, Numeric, Zero};
use crate::types::{Vec3, Vec4};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Quat<Value> {
    v: Vec3<Value>,
    s: Value,
}

impl<Value> Quat<Value> {
    #[inline(always)]
    pub fn new(s: Value, v: Vec3<Value>) -> Self {
        Self { s, v }
    }
}

impl<Value: Copy> Quat<Value> {
    #[inline(always)]
    pub fn from_vec4(v: Vec4<Value>) -> Self {
        Self {
            s: v.w(),
            v: v.into(),
        }
    }

    #[inline(always)]
    pub fn into_vec4(self) -> Vec4<Value> {
        Vec4::new(self.v.x(), self.v.y(), self.v.z(), self.s)
    }
}

impl<Value: Copy> Quat<Value> {
    #[inline(always)]
    pub fn s(&self) -> Value {
        self.s
    }
    #[inline(always)]
    pub fn v(&self) -> Vec3<Value> {
        self.v
    }
    #[inline(always)]
    pub fn x(&self) -> Value {
        self.v.x()
    }
    #[inline(always)]
    pub fn y(&self) -> Value {
        self.v.y()
    }
    #[inline(always)]
    pub fn z(&self) -> Value {
        self.v.z()
    }

    #[inline(always)]
    pub fn set(&mut self, s: Value, v: Vec3<Value>) {
        self.s = s;
        self.v = v;
    }
}

impl<Value: Identity + Zero> Identity for Quat<Value> {
    fn identity() -> Quat<Value> {
        Self {
            s: Value::identity(),
            v: Vec3::zero(),
        }
    }
}

impl<Value: Identity + Zero> Default for Quat<Value> {
    #[inline(always)]
    fn default() -> Self {
        Self::identity()
    }
}

////// quaternion addition //////
impl<Value: Numeric> AddAssign<Quat<Value>> for Quat<Value> {
    #[inline(always)]
    fn add_assign(&mut self, other: Quat<Value>) {
        self.s += other.s;
        self.v += other.v;
    }
}

impl<Value: Numeric> AddAssign<&Quat<Value>> for Quat<Value> {
    #[inline(always)]
    fn add_assign(&mut self, other: &Quat<Value>) {
        *self += *other;
    }
}

impl<Value: Numeric> Add<Quat<Value>> for Quat<Value> {
    type Output = Quat<Value>;
    #[inline(always)]
    fn add(mut self, other: Quat<Value>) -> Quat<Value> {
        self += other;
        self
    }
}

impl<Value: Numeric> Add<&Quat<Value>> for Quat<Value> {
    type Output = Quat<Value>;
    #[inline(always)]
    fn add(mut self, other: &Quat<Value>) -> Quat<Value> {
        self += *other;
        self
    }
}

impl<Value: Numeric> Add<Quat<Value>> for &Quat<Value> {
    type Output = Quat<Value>;
    #[inline(always)]
    fn add(self, other: Quat<Value>) -> Quat<Value> {
        *self + other
    }
}

impl<Value: Numeric> Add<&Quat<Value>> for &Quat<Value> {
    type Output = Quat<Value>;
    #[inline(always)]
    fn add(self, other: &Quat<Value>) -> Quat<Value> {
        *self + *other
    }
}

////// quaternion subtraction //////
impl<Value: Numeric> SubAssign<Quat<Value>> for Quat<Value> {
    #[inline(always)]
    fn sub_assign(&mut self, other: Quat<Value>) {
        self.s -= other.s;
        self.v -= other.v;
    }
}

impl<Value: Numeric> SubAssign<&Quat<Value>> for Quat<Value> {
    #[inline(always)]
    fn sub_assign(&mut self, other: &Quat<Value>) {
        *self -= *other;
    }
}

impl<Value: Numeric> Sub<Quat<Value>> for Quat<Value> {
    type Output = Quat<Value>;
    #[inline(always)]
    fn sub(mut self, other: Quat<Value>) -> Quat<Value> {
        self -= other;
        self
    }
}

impl<Value: Numeric> Sub<&Quat<Value>> for Quat<Value> {
    type Output = Quat<Value>;
    #[inline(always)]
    fn sub(mut self, other: &Quat<Value>) -> Quat<Value> {
        self -= *other;
        self
    }
}

impl<Value: Numeric> Sub<Quat<Value>> for &Quat<Value> {
    type Output = Quat<Value>;
    #[inline(always)]
    fn sub(self, other: Quat<Value>) -> Quat<Value> {
        *self - other
    }
}

impl<Value: Numeric> Sub<&Quat<Value>> for &Quat<Value> {
    type Output = Quat<Value>;
    #[inline(always)]
    fn sub(self, other: &Quat<Value>) -> Quat<Value> {
        *self - *other
    }
}

////// quaternion multiplication //////
impl<Value: Numeric> MulAssign<Quat<Value>> for Quat<Value> {
    #[inline(always)]
    fn mul_assign(&mut self, other: Quat<Value>) {
        let s = self.s;
        self.s = s * other.s - self.v.dot(&other.v);
        self.v = other.v * s + self.v * other.s + self.v.cross(&other.v);
    }
}
impl<Value: Numeric> Mul<Quat<Value>> for Quat<Value> {
    type Output = Quat<Value>;
    #[inline(always)]
    fn mul(mut self, other: Quat<Value>) -> Quat<Value> {
        self *= other;
        self
    }
}

impl<Value: Numeric> Mul<Quat<Value>> for &Quat<Value> {
    type Output = Quat<Value>;
    #[inline(always)]
    fn mul(self, other: Quat<Value>) -> Quat<Value> {
        *self * other
    }
}

impl<Value: Numeric> MulAssign<&Quat<Value>> for Quat<Value> {
    #[inline(always)]
    fn mul_assign(&mut self, other: &Quat<Value>) {
        let s = self.s;
        self.s = s * other.s - self.v.dot(&other.v);
        self.v = other.v * s + self.v * other.s + self.v.cross(&other.v);
    }
}
impl<Value: Numeric> Mul<&Quat<Value>> for Quat<Value> {
    type Output = Quat<Value>;
    #[inline(always)]
    fn mul(mut self, other: &Quat<Value>) -> Quat<Value> {
        self *= other;
        self
    }
}

////// scalar multiplication //////
impl<Value: Numeric> MulAssign<Value> for Quat<Value> {
    #[inline(always)]
    fn mul_assign(&mut self, other: Value) {
        self.s *= other;
        self.v *= other;
    }
}
impl<Value: Numeric> Mul<Value> for Quat<Value> {
    type Output = Quat<Value>;
    #[inline(always)]
    fn mul(mut self, other: Value) -> Quat<Value> {
        self *= other;
        self
    }
}

impl<Value: Numeric> Mul<Value> for &Quat<Value> {
    type Output = Quat<Value>;
    #[inline(always)]
    fn mul(self, other: Value) -> Quat<Value> {
        *self * other
    }
}

////// scalar division //////
impl<Value: Numeric> DivAssign<Value> for Quat<Value> {
    #[inline(always)]
    fn div_assign(&mut self, other: Value) {
        self.s /= other;
        self.v /= other;
    }
}
impl<Value: Numeric> Div<Value> for Quat<Value> {
    type Output = Quat<Value>;
    #[inline(always)]
    fn div(mut self, other: Value) -> Quat<Value> {
        self /= other;
        self
    }
}

impl<Value: Numeric> Div<Value> for &Quat<Value> {
    type Output = Quat<Value>;
    #[inline(always)]
    fn div(self, other: Value) -> Quat<Value> {
        *self / other
    }
}

////// quaternion negation //////
impl<Value: Numeric + Neg<Output = Value>> Neg for Quat<Value> {
    type Output = Quat<Value>;
    #[inline(always)]
    fn neg(mut self) -> Quat<Value> {
        self.s = -self.s;
        self.v = -self.v;
        self
    }
}

impl<Value: Numeric + Neg<Output = Value>> Neg for &Quat<Value> {
    type Output = Quat<Value>;
    #[inline(always)]
    fn neg(self) -> Quat<Value> {
        -*self
    }
}

////// additional operations //////
impl<Value: Numeric> Quat<Value> {
    #[inline(always)]
    pub fn square(mut self) -> Quat<Value> {
        let s = self.s;
        let (x, y, z) = self.v.into();
        self.s = s * s - x * x - y * y - z * z;
        self.v *= s + s;
        self
    }

    #[inline(always)]
    pub fn square_magnitude(&self) -> Value {
        self.s * self.s + self.v.square_magnitude()
    }
}

impl<Value: Numeric + Neg<Output = Value>> Quat<Value> {
    #[inline(always)]
    pub fn conjugate(mut self) -> Quat<Value> {
        self.v = -self.v;
        self
    }

    #[inline(always)]
    pub fn inverse(&self) -> Quat<Value> {
        self.conjugate() / self.square_magnitude()
    }
}

impl<Value: Numeric + Sqrt> Quat<Value> {
    #[inline(always)]
    pub fn magnitude(&self) -> Value {
        self.square_magnitude().sqrt()
    }

    #[inline(always)]
    pub fn normalize(&mut self) {
        *self /= self.magnitude();
    }

    #[inline(always)]
    pub fn normalized(mut self) -> Quat<Value> {
        self.normalize();
        self
    }
}

impl<Value: Float> Quat<Value> {
    #[inline(always)]
    pub fn from_axis_angle<Angle: Into<Radians<Value>>>(
        mut axis: Vec3<Value>,
        angle: Angle,
    ) -> Quat<Value> {
        let one = Value::identity();
        let half_angle = angle.into() / (one + one);

        axis.normalize();
        Quat::new(half_angle.cos(), axis * half_angle.sin())
    }

    /// Note: Assumes the quaternion is unit size
    #[inline(always)]
    pub fn apply_rotation(&self, v: &Vec3<Value>) -> Vec3<Value> {
        let two = Value::identity() + Value::identity();
        let s = self.s;
        let u = self.v;
        v * (s * s - u.dot(&u)) + u.cross(&v) * two * s + (u * u.dot(&v) * two)
    }

    #[inline(always)]
    pub fn lerp(&self, other: &Quat<Value>, progress: Value) -> Quat<Value> {
        self + (other - self) * progress
    }

    #[inline(always)]
    pub fn nlerp(&self, other: &Quat<Value>, progress: Value) -> Quat<Value> {
        self.lerp(other, progress).normalized()
    }

    /// Note: Assumes the quaternions are both unit size
    #[inline(always)]
    pub fn slerp(&self, other: &Quat<Value>, progress: Value) -> Quat<Value> {
        let one = Value::identity();

        let dot = self.v.dot(&other.v) + self.s * other.s;
        let dot_abs = dot.abs();
        let dot_sign = dot.signum();

        if dot_abs >= one {
            return *self;
        }

        let half_angle = dot_abs.acos();
        let sin_half_angle = (one - dot_abs * dot_abs).sqrt();

        let a = dot_sign * ((one - progress) * half_angle).sin() / sin_half_angle;
        let b = (progress * half_angle).sin() / sin_half_angle;

        self * a + other * b
    }
}
