use crate::types::{Vec2, Vec3, Vec4};

#[rustfmt::skip]
impl<Number: Copy> Vec2<Number> {
    #[inline(always)] pub fn xx  (&self) -> Vec2<Number> { Vec2::new(self[0], self[0]) }
    #[inline(always)] pub fn xxx (&self) -> Vec3<Number> { Vec3::new(self[0], self[0], self[0]) }
    #[inline(always)] pub fn xxxx(&self) -> Vec4<Number> { Vec4::new(self[0], self[0], self[0], self[0]) }
    #[inline(always)] pub fn xxxy(&self) -> Vec4<Number> { Vec4::new(self[0], self[0], self[0], self[1]) }
    #[inline(always)] pub fn xxy (&self) -> Vec3<Number> { Vec3::new(self[0], self[0], self[1]) }
    #[inline(always)] pub fn xxyx(&self) -> Vec4<Number> { Vec4::new(self[0], self[0], self[1], self[0]) }
    #[inline(always)] pub fn xxyy(&self) -> Vec4<Number> { Vec4::new(self[0], self[0], self[1], self[1]) }
    #[inline(always)] pub fn xy  (&self) -> Vec2<Number> { Vec2::new(self[0], self[1]) }
    #[inline(always)] pub fn xyx (&self) -> Vec3<Number> { Vec3::new(self[0], self[1], self[0]) }
    #[inline(always)] pub fn xyxx(&self) -> Vec4<Number> { Vec4::new(self[0], self[1], self[0], self[0]) }
    #[inline(always)] pub fn xyxy(&self) -> Vec4<Number> { Vec4::new(self[0], self[1], self[0], self[1]) }
    #[inline(always)] pub fn xyy (&self) -> Vec3<Number> { Vec3::new(self[0], self[1], self[1]) }
    #[inline(always)] pub fn xyyx(&self) -> Vec4<Number> { Vec4::new(self[0], self[1], self[1], self[0]) }
    #[inline(always)] pub fn xyyy(&self) -> Vec4<Number> { Vec4::new(self[0], self[1], self[1], self[1]) }
    #[inline(always)] pub fn yx  (&self) -> Vec2<Number> { Vec2::new(self[1], self[0]) }
    #[inline(always)] pub fn yxx (&self) -> Vec3<Number> { Vec3::new(self[1], self[0], self[0]) }
    #[inline(always)] pub fn yxxx(&self) -> Vec4<Number> { Vec4::new(self[1], self[0], self[0], self[0]) }
    #[inline(always)] pub fn yxxy(&self) -> Vec4<Number> { Vec4::new(self[1], self[0], self[0], self[1]) }
    #[inline(always)] pub fn yxy (&self) -> Vec3<Number> { Vec3::new(self[1], self[0], self[1]) }
    #[inline(always)] pub fn yxyx(&self) -> Vec4<Number> { Vec4::new(self[1], self[0], self[1], self[0]) }
    #[inline(always)] pub fn yxyy(&self) -> Vec4<Number> { Vec4::new(self[1], self[0], self[1], self[1]) }
    #[inline(always)] pub fn yy  (&self) -> Vec2<Number> { Vec2::new(self[1], self[1]) }
    #[inline(always)] pub fn yyx (&self) -> Vec3<Number> { Vec3::new(self[1], self[1], self[0]) }
    #[inline(always)] pub fn yyxx(&self) -> Vec4<Number> { Vec4::new(self[1], self[1], self[0], self[0]) }
    #[inline(always)] pub fn yyxy(&self) -> Vec4<Number> { Vec4::new(self[1], self[1], self[0], self[1]) }
    #[inline(always)] pub fn yyy (&self) -> Vec3<Number> { Vec3::new(self[1], self[1], self[1]) }
    #[inline(always)] pub fn yyyx(&self) -> Vec4<Number> { Vec4::new(self[1], self[1], self[1], self[0]) }
    #[inline(always)] pub fn yyyy(&self) -> Vec4<Number> { Vec4::new(self[1], self[1], self[1], self[1]) }
}