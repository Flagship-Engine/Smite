use super::*;
use crate::angle::AsRadians;

// #region identity, zero, default
#[test]
fn should_have_correct_identity() {
    assert_eq!(
        Mat4::identity(),
        Mat4::from_rows(
            (1.0, 0.0, 0.0, 0.0), //
            (0.0, 1.0, 0.0, 0.0), //
            (0.0, 0.0, 1.0, 0.0), //
            (0.0, 0.0, 0.0, 1.0), //
        )
    );
}

#[test]
fn should_have_correct_zero() {
    Mat4::<i32>::zero().iter().for_each(|&v| assert_eq!(v, 0));
}

#[test]
fn default_mat4_should_be_identity() {
    assert_eq!(Mat4::<i32>::default(), Mat4::identity());
}
// #endregion

// #region from_affine
#[test]
fn from_affine_should_affect_the_first_3_rows() {
    assert_eq!(
        Mat4::<i32>::from_affine(
            (1, 2, 3, 4),    //
            (5, 6, 7, 8),    //
            (9, 10, 11, 12), //
        ),
        Mat4::from_rows(
            (1, 2, 3, 4),    //
            (5, 6, 7, 8),    //
            (9, 10, 11, 12), //
            (0, 0, 0, 1),    //
        )
    );
}
// #endregion

// #region transpose
#[test]
fn transpose_of_from_columns_should_match_from_rows() {
    assert_eq!(
        Mat4::from_columns(
            (1, 2, 3, 4),     //
            (5, 6, 7, 8),     //
            (9, 10, 11, 12),  //
            (13, 14, 15, 16), //
        )
        .transposed(),
        Mat4::from_rows(
            (1, 2, 3, 4),     //
            (5, 6, 7, 8),     //
            (9, 10, 11, 12),  //
            (13, 14, 15, 16), //
        )
    );
}

#[test]
fn transpose_of_from_rows_should_match_from_columns() {
    assert_eq!(
        Mat4::from_rows(
            (13, 14, 15, 16), //
            (9, 10, 11, 12),  //
            (5, 6, 7, 8),     //
            (1, 2, 3, 4),     //
        )
        .transposed(),
        Mat4::from_columns(
            (13, 14, 15, 16), //
            (9, 10, 11, 12),  //
            (5, 6, 7, 8),     //
            (1, 2, 3, 4),     //
        )
    );
}
// #endregion

// #region from_array
#[test]
fn layout_should_be_column_major() {
    assert_eq!(
        Mat4::from_columns(
            (1, 2, 3, 4),     //
            (5, 6, 7, 8),     //
            (9, 10, 11, 12),  //
            (13, 14, 15, 16), //
        ),
        Mat4::from_array([
            1, 2, 3, 4, //
            5, 6, 7, 8, //
            9, 10, 11, 12, //
            13, 14, 15, 16, //
        ])
    );
}
// #endregion

// #region map, fill
#[test]
fn map_should_apply_elementwise() {
    assert_eq!(
        Mat4::from_array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]).map(|v| v * 2),
        Mat4::from_array([2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32])
    );
}

#[test]
fn fill_should_override_content() {
    let mut m = Mat4::identity();
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    m.fill(&data);
    assert_eq!(m, Mat4::from_array(data));
}
// #endregion

// #region addition, subtraction
#[test]
fn addition_should_be_elementwise() {
    let m = Mat4::from_array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    let n = Mat4::from_array([19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4]);
    let expected = Mat4::zero().map(|_| 20);
    assert_eq!(m + &n, expected);
    assert_eq!(n + &m, expected);

    let mut m2 = m;
    let mut n2 = n;
    m2 += &n;
    n2 += &m;
    assert_eq!(m2, expected);
    assert_eq!(n2, expected);
}

#[test]
fn subtraction_should_be_elementwise() {
    let m = Mat4::from_array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    let n = Mat4::from_array([4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
    let expected1 = Mat4::zero().map(|_| -3);
    let expected2 = Mat4::zero().map(|_| 3);
    assert_eq!(m - &n, expected1);
    assert_eq!(n - &m, expected2);

    let mut m2 = m;
    let mut n2 = n;
    m2 -= &n;
    n2 -= &m;
    assert_eq!(m2, expected1);
    assert_eq!(n2, expected2);
}
// #endregion

// #region multiplication, division
#[test]
fn should_multiply_correctly() {
    let m = Mat4::from_rows(
        (-1, 2, 3, 4),     //
        (5, -6, 7, 8),     //
        (9, -10, 11, -12), //
        (13, 14, -15, 16), //
    );

    let n = Mat4::from_rows(
        (-44, -43, 42, 41), //
        (34, -33, 32, 31),  //
        (24, 23, -22, 21),  //
        (14, 13, -12, -11), //
    );

    let expected1 = Mat4::from_rows(
        (240, 98, -92, 40),       //
        (-144, 248, -232, 78),    //
        (-640, 40, -40, 422),     //
        (-232, -1158, 1132, 476), //
    );

    let expected2 = Mat4::from_rows(
        (740, 324, -586, -368), //
        (492, 380, -242, -16),  //
        (166, 424, -324, 880),  //
        (-200, -84, 166, 128),  //
    );

    let actual1 = m * &n;
    let actual2 = n * &m;

    assert_eq!(actual1, expected1);
    assert_eq!(actual2, expected2);

    let mut m2 = m;
    let mut n2 = n;

    m2 *= &n;
    n2 *= &m;

    assert_eq!(m2, expected1);
    assert_eq!(n2, expected2);
}

#[test]
fn should_produce_correct_result_when_multiplied_with_a_vec4() {
    let m = Mat4::from_rows(
        (1, 2, 3, 4),     //
        (5, 6, 7, 8),     //
        (9, 10, 11, 12),  //
        (13, 14, 15, 16), //
    );
    let v = Vec4::new(-1, 3, 2, 1);
    let expected = Vec4::new(15, 35, 55, 75);
    assert_eq!(&m * v, expected);
    assert_eq!(&m * &v, expected);
}

#[test]
#[allow(clippy::op_ref)]
fn scalar_multiplication_should_be_elementwise() {
    let m = Mat4::from_array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    let s = 4;
    let expected = Mat4::from_array([4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64]);
    assert_eq!(m * s, expected);
    assert_eq!(m * &s, expected);
    assert_eq!(&m * s, expected);
    assert_eq!(&m * &s, expected);

    let mut m2 = m;
    m2 *= s;
    assert_eq!(m2, expected);
}

#[test]
#[allow(clippy::op_ref)]
fn scalar_division_should_be_elementwise() {
    let m = Mat4::from_array([4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64]);
    let s = 4;
    let expected = Mat4::from_array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    assert_eq!(m / s, expected);
    assert_eq!(m / &s, expected);
    assert_eq!(&m / s, expected);
    assert_eq!(&m / &s, expected);

    let mut m2 = m;
    m2 /= s;
    assert_eq!(m2, expected);
}
// #endregion

// #region from_translation, from_scale
#[test]
fn from_translation_should_affect_last_column() {
    assert_eq!(
        Mat4::from_translation(Vec3::new(2, 3, 4)),
        Mat4::from_affine(
            (1, 0, 0, 2), //
            (0, 1, 0, 3), //
            (0, 0, 1, 4), //
        ),
    );
}

#[test]
fn from_scale_should_affect_3_first_values_in_diagonal() {
    assert_eq!(
        Mat4::from_scale(Vec3::new(5, 6, 7)),
        Mat4::from_affine(
            (5, 0, 0, 0), //
            (0, 6, 0, 0), //
            (0, 0, 7, 0), //
        ),
    );
}
// #endregion

// #region adjugate, determinant, inverse
#[test]
fn adjugate_and_determinant_should_calculate_correctly() {
    let m = Mat4::from_rows(
        (-11, 12, 13, 14), //
        (21, -22, 23, 24), //
        (31, 32, -33, 34), //
        (41, 42, 43, -44), //
    );

    let expected_adj = Mat4::from_rows(
        (131_736, -75_416, -50_776, -38_456), //
        (-127_688, 73_568, -46_728, -36_608), //
        (-120_472, -64_152, 48_312, -35_992), //
        (-116_864, -62_744, -44_704, 34_584), //
    );
    let expected_det = -6_183_584;

    let (actual_adj, actual_det) = m.adjugate_determinant();

    assert_eq!(actual_adj, expected_adj);
    assert_eq!(actual_det, expected_det);
}

#[test]
fn product_of_matrix_and_its_inverse_should_approximate_identity_matrix() {
    let m = Mat4::from_rows(
        (1.0, 5.0, 9.0, -3.0),  //
        (2.0, 6.0, 0.0, 14.0),  //
        (3.0, 7.0, 11.0, 15.0), //
        (4.0, 3.0, -2.0, 1.0),  //
    );
    let actual = m * &m.inverse();
    let expected = Mat4::identity();

    actual
        .iter()
        .zip(expected.iter())
        .for_each(|(&a, &b)| assert_approx_eq!(a, b));
}
// #endregion

// #region from_rotation_x/y/z, from_axis_angle, rotate_x/y/z, from_quat
macro_rules! from_rotation_helper {
    ($axis:expr, $fn:tt) => {{
        let axis = $axis;
        use std::f32::consts::PI;
        const STEPS: usize = 60;
        const FSTEPS: f32 = STEPS as f32;
        const INCREMENT: f32 = 2.0 * PI / FSTEPS;

        for i in 0..=STEPS {
            let f = i as f32;
            let angle = (INCREMENT * f).as_radians();

            let actual = Mat4::$fn(angle);
            let expected = Mat4::from_axis_angle(axis, angle);

            actual.iter().zip(expected.iter()).for_each(|(&a, &b)| {
                assert_approx_eq!(
                    a,
                    b,
                    "{}({}, {}) == {}, expected = {}",
                    stringify!($fn),
                    $axis,
                    angle,
                    actual,
                    expected
                )
            });
        }
    }};
}

#[test]
fn from_rotation_x_should_match_from_axis_angle_on_x_axis() {
    from_rotation_helper!(Vec3::<f32>::unit_x(), from_rotation_x);
}

#[test]
fn from_rotation_y_should_match_from_axis_angle_on_y_axis() {
    from_rotation_helper!(Vec3::<f32>::unit_y(), from_rotation_y);
}

#[test]
fn from_rotation_z_should_match_from_axis_angle_on_z_axis() {
    from_rotation_helper!(Vec3::<f32>::unit_z(), from_rotation_z);
}

#[test]
fn from_axis_angle_should_match_matrix_of_quat_from_axis_angle() {
    use std::f32::consts::PI;
    const STEPS: usize = 60;
    const FSTEPS: f32 = STEPS as f32;
    const INCREMENT: f32 = 2.0 * PI / FSTEPS;

    let axes = [
        Vec3::unit_x(),
        Vec3::unit_y(),
        Vec3::unit_z(),
        Vec3::new(1.0, 2.0, -3.0),
    ];

    for axis in &axes {
        for i in 0..=STEPS {
            let f = i as f32;
            let angle = (INCREMENT * f).as_radians();

            let actual = Mat4::from_axis_angle(*axis, angle);
            let expected = Mat4::from_quat(&Quat::from_axis_angle(*axis, angle));

            actual.iter().zip(expected.iter()).for_each(|(&a, &b)| {
                assert_approx_eq!(
                    a,
                    b,
                    "from_axis_angle({}, {}) == {}, expected = {}",
                    axis,
                    angle,
                    actual,
                    expected
                )
            });
        }
    }
}

#[test]
fn rotate_x_on_identity_matrix_should_match_from_rotation_x() {
    use std::f32::consts::PI;
    const STEPS: usize = 60;
    const FSTEPS: f32 = STEPS as f32;
    const INCREMENT: f32 = 2.0 * PI / FSTEPS;

    for i in 0..=STEPS {
        let f = i as f32;
        let angle = (INCREMENT * f).as_radians();

        let mut actual = Mat4::identity();
        actual.rotate_x(angle);
        let expected = Mat4::from_rotation_x(angle);

        actual.iter().zip(expected.iter()).for_each(|(&a, &b)| {
            assert_approx_eq!(
                a,
                b,
                "rotate_x({}) == {}, expected = {}",
                angle,
                actual,
                expected
            )
        });
    }
}

#[test]
fn rotate_y_on_identity_matrix_should_match_from_rotation_y() {
    use std::f32::consts::PI;
    const STEPS: usize = 60;
    const FSTEPS: f32 = STEPS as f32;
    const INCREMENT: f32 = 2.0 * PI / FSTEPS;

    for i in 0..=STEPS {
        let f = i as f32;
        let angle = (INCREMENT * f).as_radians();

        let mut actual = Mat4::identity();
        actual.rotate_y(angle);
        let expected = Mat4::from_rotation_y(angle);

        actual.iter().zip(expected.iter()).for_each(|(&a, &b)| {
            assert_approx_eq!(
                a,
                b,
                "rotate_y({}) == {}, expected = {}",
                angle,
                actual,
                expected
            )
        });
    }
}

#[test]
fn rotate_z_on_identity_matrix_should_match_from_rotation_z() {
    use std::f32::consts::PI;
    const STEPS: usize = 60;
    const FSTEPS: f32 = STEPS as f32;
    const INCREMENT: f32 = 2.0 * PI / FSTEPS;

    for i in 0..=STEPS {
        let f = i as f32;
        let angle = (INCREMENT * f).as_radians();

        let mut actual = Mat4::identity();
        actual.rotate_z(angle);
        let expected = Mat4::from_rotation_z(angle);

        actual.iter().zip(expected.iter()).for_each(|(&a, &b)| {
            assert_approx_eq!(
                a,
                b,
                "rotate_z({}) == {}, expected = {}",
                angle,
                actual,
                expected
            )
        });
    }
}

#[test]
fn from_quat_on_identity_quat_should_be_identity_matrix() {
    let actual: Mat4<f32> = Mat4::from_quat(&Quat::identity());
    let expected = Mat4::identity();

    actual
        .iter()
        .zip(expected.iter())
        .for_each(|(&a, &b)| assert_approx_eq!(a, b));
}

#[test]
fn from_quat_on_right_angles_around_y_should_be_correct() {
    let s = std::f32::consts::FRAC_PI_2;
    let expects = [
        Mat4::identity(),
        Mat4::from_affine(
            (0.0, 0.0, 1.0, 0.0),  //
            (0.0, 1.0, 0.0, 0.0),  //
            (-1.0, 0.0, 0.0, 0.0), //
        ),
        Mat4::from_affine(
            (-1.0, 0.0, 0.0, 0.0), //
            (0.0, 1.0, 0.0, 0.0),  //
            (0.0, 0.0, -1.0, 0.0), //
        ),
        Mat4::from_affine(
            (0.0, 0.0, -1.0, 0.0), //
            (0.0, 1.0, 0.0, 0.0),  //
            (1.0, 0.0, 0.0, 0.0),  //
        ),
        Mat4::identity(),
    ];

    let y_axis = Vec3::unit_y();

    for (i, expected) in expects.iter().enumerate() {
        let angle = ((i as f32) * s).as_radians();
        let q = Quat::from_axis_angle(y_axis, angle);
        let actual: Mat4<f32> = Mat4::from_quat(&q);
        actual.iter().zip(expected.iter()).for_each(|(&a, &b)| {
            assert_approx_eq!(
                a,
                b,
                "from_quat({:?}) = {}, expected = {}",
                q,
                actual,
                expected
            )
        });
    }
}
// #endregion

// #region orthogonal/perspective projections
#[test]
#[allow(clippy::unreadable_literal, clippy::excessive_precision)]
pub fn from_orthogonal_should_match_expected_values_1() {
    let tests = [
        (
            (0.0, 100.0, 10.0, 80.0, 0.5, 100.0),
            Mat4::from_rows(
                (0.02, 0.0, 0.0, -1.0),                                 //
                (0.0, 0.02857142857142857, 0.0, -1.2857142857142858),   //
                (0.0, 0.0, -0.020100502512562814, -1.0100502512562815), //
                (0.0, 0.0, 0.0, 1.0),                                   //
            ),
        ),
        (
            (-5.0, 7.0, -9.0, 3.0, 10.0, 11.0),
            Mat4::from_rows(
                (0.16666666666666666, 0.0, 0.0, -0.16666666666666666), //
                (0.0, 0.16666666666666666, 0.0, 0.5),                  //
                (0.0, 0.0, -2.0, -21.0),                               //
                (0.0, 0.0, 0.0, 1.0),                                  //
            ),
        ),
    ];

    for ((left, right, bottom, top, near, far), expected) in &tests {
        let actual: Mat4<f32> = Mat4::from_orthogonal(*left, *right, *bottom, *top, *near, *far);

        actual.iter().zip(expected.iter()).for_each(|(&a, &b)| {
            assert_approx_eq!(
                a,
                b,
                "from_orthogonal({}, {}, {}, {}, {}, {}) = {}, expected = {}",
                left,
                right,
                bottom,
                top,
                near,
                far,
                actual,
                expected
            )
        });
    }
}

#[test]
pub fn from_orthogonal_symmetric_should_match_from_orthogonal_on_symmetric_input() {
    let tests = [
        (100.0, 80.0, 0.3, 1000.0),
        (50.0, 2000.0, 50.0, 51.0),
        (1.0, 1.0, 0.0, 1.0),
    ];

    for (width, height, near, far) in &tests {
        let right = width / 2.0;
        let top = height / 2.0;
        let left = -right;
        let bottom = -top;

        let expected = Mat4::from_orthogonal(left, right, bottom, top, *near, *far);
        let actual = Mat4::from_orthogonal_symmetric(*width, *height, *near, *far);

        actual.iter().zip(expected.iter()).for_each(|(&a, &b)| {
            assert_approx_eq!(
                a,
                b,
                "from_orthogonal_symmetric({}, {}, {}, {}) = {}, expected = {}",
                width,
                height,
                near,
                far,
                actual,
                expected
            )
        });
    }
}

#[test]
#[allow(clippy::unreadable_literal, clippy::excessive_precision)]
pub fn from_perspective_should_match_expected_values_1() {
    let tests = [
        (
            (60.0, (100.0, 80.0), 0.3, 1000.0),
            Mat4::from_rows(
                (1.3856407, 0.0, 0.0, 0.0),          //
                (0.0, 1.7320509, 0.0, 0.0),          //
                (0.0, 0.0, -1.0006001, -0.60018003), //
                (0.0, 0.0, -1.0, 0.0),
            ),
        ),
        (
            (120.0, (40.0, 40.0), 42.0, 53.0),
            Mat4::from_rows(
                (0.57735026, 0.0, 0.0, 0.0),       //
                (0.0, 0.57735026, 0.0, 0.0),       //
                (0.0, 0.0, -8.636364, -404.72726), //
                (0.0, 0.0, -1.0, 0.0),             //
            ),
        ),
        (
            (90.0, (7.9, 123.4), 100.0, 101.0),
            Mat4::from_rows(
                (15.620254, 0.0, 0.0, 0.0),   //
                (0.0, 1.0, 0.0, 0.0),         //
                (0.0, 0.0, -201.0, -20200.0), //
                (0.0, 0.0, -1.0, 0.0),        //
            ),
        ),
    ];

    for ((fov_deg, (width, height), near, far), expected) in &tests {
        let fov = std::f32::consts::PI * fov_deg / 180.0;
        let aspect = width / height;
        let actual: Mat4<f32> = Mat4::from_perspective(fov.as_radians(), aspect, *near, *far);
        actual.iter().zip(expected.iter()).for_each(|(&a, &b)| {
            assert_approx_eq!(
                a,
                b,
                "from_perspective({}, {}, {}, {}) = {}, expected = {}",
                fov,
                aspect,
                near,
                far,
                actual,
                expected
            )
        });
    }
}
// #endregions
