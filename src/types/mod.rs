export! {
    vec2::Vec2;
    vec3::Vec3;
    vec4::Vec4;
    mat4::Mat4;
    quat::Quat;
}

impl<Value: Copy> From<Vec3<Value>> for Vec2<Value> {
    fn from(v: Vec3<Value>) -> Self {
        Self::new(v.x(), v.y())
    }
}

impl<Value: Copy> From<Vec4<Value>> for Vec2<Value> {
    fn from(v: Vec4<Value>) -> Self {
        Self::new(v.x(), v.y())
    }
}

impl<Value: Copy> From<Vec4<Value>> for Vec3<Value> {
    fn from(v: Vec4<Value>) -> Self {
        Self::new(v.x(), v.y(), v.z())
    }
}
