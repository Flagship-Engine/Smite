use super::*;
use crate::angle::AsRadians;

#[test]
fn quat_constructor_and_accessors_should_match() {
    let q = Quat::new(5, Vec3::new(2, 3, 4));
    assert_eq!(q.s(), 5);
    assert_eq!(q.v(), Vec3::new(2, 3, 4));
    assert_eq!(q.x(), 2);
    assert_eq!(q.y(), 3);
    assert_eq!(q.z(), 4);
}

#[test]
fn quat_should_have_identity_as_s1_v0() {
    let q: Quat<i32> = Quat::identity();
    assert_eq!(q.s(), 1);
    assert_eq!(q.v(), Vec3::zero());
}

#[test]
fn quat_default_should_be_identity() {
    let q: Quat<i32> = Quat::default();
    assert_eq!(q, Quat::identity());
}

#[test]
fn quat_from_vec4_should_work() {
    let v = Vec4::new(2, 3, 4, 5);
    let q = Quat::from_vec4(v);
    assert_eq!(q.x(), v.x());
    assert_eq!(q.y(), v.y());
    assert_eq!(q.z(), v.z());
    assert_eq!(q.s(), v.w());
}

#[test]
fn quat_into_vec4_should_work() {
    let v = Vec4::new(2, 3, 4, 5);
    let q = Quat::from_vec4(v);
    assert_eq!(q.into_vec4(), v);
}

#[test]
fn quat_setter_should_work() {
    let mut q = Quat::new(3, Vec3::new(1, 2, 4));
    q.set(9, Vec3::new(8, 7, 6));
    assert_eq!(q.s(), 9);
    assert_eq!(q.x(), 8);
    assert_eq!(q.y(), 7);
    assert_eq!(q.z(), 6);
}

#[test]
fn quat_addition_should_work() {
    let q1 = Quat::new(3, Vec3::new(1, 2, 4));
    let q2 = Quat::new(7, Vec3::new(4, 3, 1));
    let q3 = Quat::new(10, Vec3::new(5, 5, 5));
    assert_eq!(q1 + q2, q3);
    assert_eq!(q2 + q1, q3);

    let mut q = q1;
    q += q2;
    assert_eq!(q, q3);
}

#[test]
fn quat_subtraction_should_work() {
    let q1 = Quat::new(3, Vec3::new(1, 2, 4));
    let q2 = Quat::new(7, Vec3::new(4, 3, 1));
    let q3 = Quat::new(-4, Vec3::new(-3, -1, 3));
    let q4 = Quat::new(4, Vec3::new(3, 1, -3));
    assert_eq!(q1 - q2, q3);
    assert_eq!(q2 - q1, q4);

    let mut q = q1;
    q -= q2;
    assert_eq!(q, q3);
}

#[test]
fn quat_multiplication_should_work() {
    let q1 = Quat::new(3, Vec3::new(1, 2, 4));
    let q2 = Quat::new(6, Vec3::new(4, 3, 1));
    let q3 = Quat::new(4, Vec3::new(8, 36, 22));
    let q4 = Quat::new(4, Vec3::new(28, 6, 32));
    assert_eq!(q1 * q2, q3);
    assert_eq!(q2 * q1, q4);

    let mut q = q1;
    q *= q2;
    assert_eq!(q, q3);
}

#[test]
fn quat_scalar_multiplication_should_work() {
    let q1 = Quat::new(3, Vec3::new(1, 2, 4));

    for i in -10..10 {
        let q2 = q1 * i;
        assert_eq!(q2.s(), q1.s() * i);
        assert_eq!(q2.v(), q1.v() * i);
    }

    let mut q = q1;
    q *= 5;
    assert_eq!(q, q1 * 5);
}

#[test]
#[allow(clippy::float_cmp)]
fn quat_scalar_division_should_work() {
    let q1 = Quat::new(3.0, Vec3::new(1.0, 2.0, 4.0));

    for i in (-10..-1).chain(1..10) {
        let n = i as f32;
        let q2 = q1 / n;
        assert_eq!(q2.s(), q1.s() / n);
        assert_eq!(q2.v(), q1.v() / n);
    }

    let mut q = q1;
    q /= 5.0;
    assert_eq!(q, q1 / 5.0);
}

#[test]
fn quat_square_should_be_own_product() {
    let q1 = Quat::new(3.0, Vec3::new(1.0, 2.0, 4.0));
    assert_eq!(q1.square(), q1 * q1);
}

#[test]
fn quat_norm_should_work() {
    let q1 = Quat::new(3.0, Vec3::new(1.0, 2.0, 4.0));
    assert_approx_eq!(q1.magnitude(), q1.into_vec4().magnitude());
}

#[test]
fn quat_square_norm_should_work() {
    let q1 = Quat::new(3.0, Vec3::new(1.0, 2.0, 4.0));
    assert_approx_eq!(q1.square_magnitude(), q1.into_vec4().square_magnitude());
}

#[test]
fn quat_conjugate_should_negate_vector_part() {
    let q1 = Quat::new(3.0, Vec3::new(1.0, 2.0, 4.0));
    let q2 = Quat::new(3.0, Vec3::new(-1.0, -2.0, -4.0));
    assert_eq!(q1.conjugate(), q2);
}

#[test]
fn product_of_quat_and_its_inverse_should_be_identity_quat() {
    let q1 = Quat::new(3.0, Vec3::new(1.0, 2.0, 4.0));
    assert_eq!(q1 * q1.inverse(), Quat::identity());
    assert_eq!(q1.inverse() * q1, Quat::identity());
}

#[test]
fn normalizing_a_quat_makes_it_unit_size() {
    let q1 = Quat::new(3.0, Vec3::new(1.0, 2.0, 4.0));
    assert_approx_eq!(q1.normalized().magnitude(), 1.0);
    assert_approx_eq!(q1.normalized().square_magnitude(), 1.0);
}

#[cfg(test)]
macro_rules! assert_quat_approx_eq {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                assert_approx_eq!(left_val.s(), right_val.s());
                assert_approx_eq!(left_val.v(), right_val.v());
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                assert_approx_eq!(left_val.s(), right_val.s(), $($arg)+);
                assert_approx_eq!(left_val.v(), right_val.v(), $($arg)+);
            }
        }
    };
}

#[allow(clippy::cognitive_complexity)]
fn from_axis_angle_test_helper(axis: Vec3<f32>, axis_name: &str) {
    const PI: f32 = core::f32::consts::PI;
    const PI_HALF: f32 = PI / 2.0;
    const PI_QUARTER: f32 = PI / 4.0;

    let mut unit_axis = axis;
    unit_axis.normalize();

    let q1 = Quat::from_axis_angle(axis, (PI + PI).as_radians());
    let q2 = Quat::new(-1.0, Vec3::zero());
    assert_quat_approx_eq!(q1, q2, "{}: 1 turn", axis_name);

    let q1 = Quat::from_axis_angle(axis, (PI + PI_HALF).as_radians());
    let q2 = Quat::new(-PI_QUARTER.cos(), PI_QUARTER.cos() * unit_axis);
    assert_quat_approx_eq!(q1, q2, "{}: 3/4 turn", axis_name);

    let q1 = Quat::from_axis_angle(axis, PI.as_radians());
    let q2 = Quat::new(0.0, unit_axis);
    assert_quat_approx_eq!(q1, q2, "{}: 1/2 turn", axis_name);

    let q1 = Quat::from_axis_angle(axis, PI_HALF.as_radians());
    let q2 = Quat::new(PI_QUARTER.cos(), PI_QUARTER.cos() * unit_axis);
    assert_quat_approx_eq!(q1, q2, "{}: 1/4 turn", axis_name);

    let q1 = Quat::from_axis_angle(axis, (0.0).as_radians());
    let q2 = Quat::new(1.0, Vec3::zero());
    assert_quat_approx_eq!(q1, q2, "{}: 0 turn", axis_name);

    let q1 = Quat::from_axis_angle(axis, -PI_HALF.as_radians());
    let q2 = Quat::new(PI_QUARTER.cos(), -PI_QUARTER.cos() * unit_axis);
    assert_quat_approx_eq!(q1, q2, "{}: -1/4 turn", axis_name);

    let q1 = Quat::from_axis_angle(axis, -PI.as_radians());
    let q2 = Quat::new(0.0, -unit_axis);
    assert_quat_approx_eq!(q1, q2, "{}: -1/2 turn", axis_name);

    let q1 = Quat::from_axis_angle(axis, -(PI + PI_HALF).as_radians());
    let q2 = Quat::new(-PI_QUARTER.cos(), -PI_QUARTER.cos() * unit_axis);
    assert_quat_approx_eq!(q1, q2, "{}: -3/4 turn", axis_name);

    let q1 = Quat::from_axis_angle(axis, -(PI + PI).as_radians());
    let q2 = Quat::new(-1.0, Vec3::zero());
    assert_quat_approx_eq!(q1, q2, "{}: -1 turn", axis_name);
}

#[test]
fn quat_from_axis_angle_should_work_for_x_axis() {
    from_axis_angle_test_helper(Vec3::unit_x(), "x-axis");
}

#[test]
fn quat_from_axis_angle_should_work_for_negative_x_axis() {
    from_axis_angle_test_helper(-Vec3::unit_x(), "negative x-axis");
}

#[test]
fn quat_from_axis_angle_should_work_for_y_axis() {
    from_axis_angle_test_helper(Vec3::unit_y(), "y-axis");
}

#[test]
fn quat_from_axis_angle_should_work_for_negative_y_axis() {
    from_axis_angle_test_helper(-Vec3::unit_y(), "negative y-axis");
}

#[test]
fn quat_from_axis_angle_should_work_for_z_axis() {
    from_axis_angle_test_helper(Vec3::unit_z(), "z-axis");
}

#[test]
fn quat_from_axis_angle_should_work_for_negative_z_axis() {
    from_axis_angle_test_helper(-Vec3::unit_z(), "negative z-axis");
}

#[test]
fn quat_from_axis_angle_should_work_for_any_axis() {
    from_axis_angle_test_helper(Vec3::new(3.0, 1.0, 2.0), "(x, y, z) = (3, 1, 2)");
    from_axis_angle_test_helper(Vec3::new(5.0, -3.0, 0.0), "(x, y, z) = (5, -3, 0)");
    from_axis_angle_test_helper(Vec3::new(1.0, 0.0, -5.0), "(x, y, z) = (1, 0, -5)");
    from_axis_angle_test_helper(
        Vec3::new(0.0, 1000.0, -1000.0),
        "(x, y, z) = (0, 1000, -1000)",
    );
}

#[test]
fn quat_should_rotate_vec3s_correctly() {
    const PI: f32 = core::f32::consts::PI;

    let qs = [
        Quat::from_axis_angle(Vec3::unit_x(), PI.as_radians()),
        Quat::from_axis_angle(Vec3::unit_y(), (PI / 2.0).as_radians()),
        Quat::from_axis_angle(Vec3::unit_z(), -(3.0 * PI / 4.0).as_radians()),
        Quat::from_axis_angle(Vec3::new(4.0, 2.0, 1.0), (-2.315).as_radians()),
    ];

    let vs = [
        Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(-5.0, 2.6, 4.9),
        Vec3::new(0.0, -4.1, -0.1),
        Vec3::new(-1.3, 1.5, -2.4),
    ];

    for q in &qs {
        for v in &vs {
            assert_approx_eq!(
                q.apply_rotation(v),
                (q * Quat::new(0.0, *v) * q.inverse()).v()
            );
        }
    }
}

#[test]
fn quats_should_slerp_correctly_between_cardinal_axes() {
    const PI: f32 = core::f32::consts::PI;

    let qs = [
        Quat::new(1.0, [0.0, 0.0, 0.0].into()),
        Quat::new(0.0, [1.0, 0.0, 0.0].into()),
        Quat::new(0.0, [0.0, 1.0, 0.0].into()),
        Quat::new(0.0, [0.0, 0.0, 1.0].into()),
    ];

    const STEPS: usize = 100;
    const FSTEPS: f32 = STEPS as f32;
    const FSTEPS_2: f32 = 2.0 * FSTEPS;
    const INCREMENT: f32 = 1.0 / FSTEPS;

    for p in &qs {
        for q in &qs {
            for i in 0..=STEPS {
                let f = i as f32;
                let t = INCREMENT * f;

                let r = if p == q {
                   *p
                } else {
                    let a = (f * PI / FSTEPS_2).cos();
                    let b = ((FSTEPS - f) * PI / FSTEPS_2).cos();
                    Quat::new(a * p.s + b * q.s, a * p.v + b * q.v)
                };

                assert_quat_approx_eq!(p.slerp(&q, t), r, "slerp({:?}, {:?}, {}) == {:?}", p, q, t, r);
            }
        }
    }
}

#[test]
#[allow(clippy::unreadable_literal)]
#[allow(clippy::excessive_precision)]
fn quats_should_slerp_correctly_between_arbitrary_orientations() {
    use core::f32::consts::FRAC_PI_4;
    let p = Quat::new(1.0/3.0, [2.0/3.0, 2.0/3.0, 0.0].into());
    let q = Quat::from_axis_angle(Vec3::new(FRAC_PI_4.cos(), 0.0, FRAC_PI_4.cos()), FRAC_PI_4.as_radians());

    let tests = [
        (0.0, p),
        (0.1, Quat::new(0.42378840777972404, [0.6562517646050643, 0.623425575504739, 0.0328261891003253].into())),
        (0.2, Quat::new(0.5094809729615506, [0.6384619437126412, 0.5731784642993116, 0.0652834794133296].into())),
        (0.3, Quat::new(0.5894480208585584, [0.6134971249529246, 0.5164900071383027, 0.0970071178146219].into())),
        (0.5, Quat::new(0.7286853469983307, [0.5432421858711844, 0.38640252938062086, 0.15683965649056364].into())),
        (0.75, Quat::new(0.8562033639126905, [0.42165530463757955, 0.2001973717233015, 0.22145793291427807].into())),
        (0.9, Quat::new(0.9043917861995282, [0.333919802286284, 0.08087318464036171, 0.2530466176459223].into())),
        (1.0, q),
    ];

    for (t, r) in &tests {
        assert_quat_approx_eq!(p.slerp(&q, *t), r);
    }
}
