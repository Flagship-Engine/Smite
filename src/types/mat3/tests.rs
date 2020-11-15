use super::*;
use crate::angle::AsRadians;

// #region identity, zero, default
#[test]
fn should_have_correct_identity() {
    assert_eq!(
        Mat3::identity(),
        Mat3::from_rows(
            (1.0, 0.0, 0.0), //
            (0.0, 1.0, 0.0), //
            (0.0, 0.0, 1.0), //
        )
    );
}

#[test]
fn should_have_correct_zero() {
    Mat3::<i32>::zero().iter().for_each(|&v| assert_eq!(v, 0));
}

#[test]
fn default_mat3_should_be_identity() {
    assert_eq!(Mat3::<i32>::default(), Mat3::identity());
}
// #endregion

// #region from_affine_2d
#[test]
fn from_affine_should_affect_the_first_2_rows() {
    assert_eq!(
        Mat3::<i32>::from_affine_2d(
            (1, 2, 3), //
            (4, 5, 6), //
        ),
        Mat3::from_rows(
            (1, 2, 3), //
            (4, 5, 6), //
            (0, 0, 1), //
        )
    );
}
// #endregion

// #region transpose
#[test]
fn transpose_of_from_columns_should_match_from_rows() {
    assert_eq!(
        Mat3::from_columns(
            (1, 2, 3), //
            (4, 5, 6), //
            (7, 8, 9), //
        )
        .transposed(),
        Mat3::from_rows(
            (1, 2, 3), //
            (4, 5, 6), //
            (7, 8, 9), //
        )
    );
}

#[test]
fn transpose_of_from_rows_should_match_from_columns() {
    assert_eq!(
        Mat3::from_rows(
            (7, 8, 9), //
            (4, 5, 6), //
            (1, 2, 3), //
        )
        .transposed(),
        Mat3::from_columns(
            (7, 8, 9), //
            (4, 5, 6), //
            (1, 2, 3), //
        )
    );
}
// #endregion

// #region from_array
#[test]
fn layout_should_be_column_major() {
    assert_eq!(
        Mat3::from_columns(
            (1, 2, 3), //
            (4, 5, 6), //
            (7, 8, 9), //
        ),
        Mat3::from_array([
            1, 2, 3, //
            4, 5, 6, //
            7, 8, 9, //
        ])
    );
}
// #endregion

// #region map, fill
#[test]
fn map_should_apply_elementwise() {
    assert_eq!(
        Mat3::from_array([1, 2, 3, 4, 5, 6, 7, 8, 9]).map(|v| v * 2),
        Mat3::from_array([2, 4, 6, 8, 10, 12, 14, 16, 18])
    );
}

#[test]
fn fill_should_override_content() {
    let mut m = Mat3::identity();
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    m.fill(&data);
    assert_eq!(m, Mat3::from_array(data));
}
// #endregion

// #region addition, subtraction
#[test]
fn addition_should_be_elementwise() {
    let m = Mat3::from_array([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let n = Mat3::from_array([19, 18, 17, 16, 15, 14, 13, 12, 11]);
    let expected = Mat3::zero().map(|_| 20);
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
    let m = Mat3::from_array([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let n = Mat3::from_array([11, 12, 13, 14, 15, 16, 17, 18, 19]);
    let expected1 = Mat3::zero().map(|_| -10);
    let expected2 = Mat3::zero().map(|_| 10);
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
    let m = Mat3::from_rows(
        (-1, 2, 3), //
        (4, 5, -6), //
        (-7, 8, 9), //
    );

    let n = Mat3::from_rows(
        (-33, 32, 31),  //
        (23, -22, 21),  //
        (13, -12, -11), //
    );

    let expected1 = Mat3::from_rows(
        (118, -112, -22),  //
        (-95, 90, 295),    //
        (532, -508, -148), //
    );

    let expected2 = Mat3::from_rows(
        (-56, 342, -12),  //
        (-258, 104, 390), //
        (16, -122, 12),   //
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
fn should_produce_correct_result_when_multiplied_with_a_vec3() {
    let m = Mat3::from_rows(
        (1, 2, 3), //
        (4, 5, 6), //
        (7, 8, 9), //
    );
    let v = Vec3::new(-1, 3, 2);
    let expected = Vec3::new(11, 23, 35);
    assert_eq!(&m * v, expected);
    assert_eq!(&m * &v, expected);
}

#[test]
#[allow(clippy::op_ref)]
fn scalar_multiplication_should_be_elementwise() {
    let m = Mat3::from_array([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let s = 4;
    let expected = Mat3::from_array([4, 8, 12, 16, 20, 24, 28, 32, 36]);
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
    let m = Mat3::from_array([4, 8, 12, 16, 20, 24, 28, 32, 36]);
    let s = 4;
    let expected = Mat3::from_array([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(m / s, expected);
    assert_eq!(m / &s, expected);
    assert_eq!(&m / s, expected);
    assert_eq!(&m / &s, expected);

    let mut m2 = m;
    m2 /= s;
    assert_eq!(m2, expected);
}
// #endregion

// #region from_translation_2d, from_scale, , from_scale_2d
#[test]
fn from_translation_2d_should_affect_last_column() {
    assert_eq!(
        Mat3::from_translation_2d(Vec2::new(2, 3)),
        Mat3::from_affine_2d(
            (1, 0, 2), //
            (0, 1, 3), //
        ),
    );
}

#[test]
fn from_scale_should_affect_values_in_diagonal() {
    assert_eq!(
        Mat3::from_scale(Vec3::new(5, 6, 7)),
        Mat3::from_rows(
            (5, 0, 0), //
            (0, 6, 0), //
            (0, 0, 7), //
        ),
    );
}

#[test]
fn from_scale_2d_should_affect_2_first_values_in_diagonal() {
    assert_eq!(
        Mat3::from_scale_2d(Vec2::new(3, 5)),
        Mat3::from_affine_2d(
            (3, 0, 0), //
            (0, 5, 0), //
        ),
    );
}
// #endregion

// #region adjugate, determinant, inverse
#[test]
fn adjugate_and_determinant_should_calculate_correctly() {
    let m = Mat3::from_rows(
        (1, 9, 4),  //
        (2, -8, 5), //
        (3, 7, 6),  //
    );

    let expected_adj = Mat3::from_rows(
        (-83, -26, 77), //
        (3, -6, 3),     //
        (38, 20, -26),  //
    );
    let expected_det = 96;

    let (actual_adj, actual_det) = m.adjugate_determinant();

    assert_eq!(actual_adj, expected_adj);
    assert_eq!(actual_det, expected_det);
}

#[test]
fn product_of_matrix_and_its_inverse_should_approximate_identity_matrix() {
    let m = Mat3::from_rows(
        (1.0, 9.0, 4.0),  //
        (2.0, -8.0, 5.0), //
        (3.0, 7.0, 6.0),  //
    );
    let actual = m * &m.inverse();
    let expected = Mat3::identity();

    actual
        .iter()
        .zip(expected.iter())
        .for_each(|(&a, &b)| assert_approx_eq!(a, b));
}
// #endregion

// #region from_rotation_x/y/z/2d, from_axis_angle, rotate_2d, from_quat
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

            let actual = Mat3::$fn(angle);
            let expected = Mat3::from_axis_angle(axis, angle);

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
fn from_rotation_2d_should_match_from_axis_angle_on_z_axis() {
    from_rotation_helper!(Vec3::<f32>::unit_z(), from_rotation_2d);
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

            let actual = Mat3::from_axis_angle(*axis, angle);
            let expected = Mat3::from_quat(&Quat::from_axis_angle(*axis, angle));

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
fn rotate_2d_on_identity_matrix_should_match_from_rotation_2d() {
    use std::f32::consts::PI;
    const STEPS: usize = 60;
    const FSTEPS: f32 = STEPS as f32;
    const INCREMENT: f32 = 2.0 * PI / FSTEPS;

    for i in 0..=STEPS {
        let f = i as f32;
        let angle = (INCREMENT * f).as_radians();

        let mut actual = Mat3::identity();
        actual.rotate_2d(angle);
        let expected = Mat3::from_rotation_2d(angle);

        actual.iter().zip(expected.iter()).for_each(|(&a, &b)| {
            assert_approx_eq!(
                a,
                b,
                "rotate_2d({}) == {}, expected = {}",
                angle,
                actual,
                expected
            )
        });
    }
}

#[test]
fn from_quat_on_identity_quat_should_be_identity_matrix() {
    let actual: Mat3<f32> = Mat3::from_quat(&Quat::identity());
    let expected = Mat3::identity();

    actual
        .iter()
        .zip(expected.iter())
        .for_each(|(&a, &b)| assert_approx_eq!(a, b));
}

#[test]
fn from_quat_on_right_angles_around_y_should_be_correct() {
    let s = std::f32::consts::FRAC_PI_2;
    let expects = [
        Mat3::identity(),
        Mat3::from_rows(
            (0.0, 0.0, 1.0),  //
            (0.0, 1.0, 0.0),  //
            (-1.0, 0.0, 0.0), //
        ),
        Mat3::from_rows(
            (-1.0, 0.0, 0.0), //
            (0.0, 1.0, 0.0), //
            (0.0, 0.0, -1.0), //
        ),
        Mat3::from_rows(
            (0.0, 0.0, -1.0), //
            (0.0, 1.0, 0.0), //
            (1.0, 0.0, 0.0), //
        ),
        Mat3::identity(),
    ];

    let y_axis = Vec3::unit_y();

    for (i, expected) in expects.iter().enumerate() {
        let angle = ((i as f32) * s).as_radians();
        let q = Quat::from_axis_angle(y_axis, angle);
        let actual: Mat3<f32> = Mat3::from_quat(&q);
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
