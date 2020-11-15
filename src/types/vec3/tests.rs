use super::*;
use crate::angle::AsRadians;

#[test]
fn vec3_should_have_identity_as_1_1_1() {
    assert_eq!(Vec3::identity(), Vec3::new(1, 1, 1));
}

#[test]
fn vec3_should_have_zero_as_0_0_0() {
    assert_eq!(Vec3::zero(), Vec3::new(0, 0, 0));
}

#[test]
fn sum_on_vec3_is_sum_of_components() {
    assert_eq!(Vec3::new(1, 2, 3).sum(), 1 + 2 + 3);
    assert_eq!(Vec3::new(4, 3, 1).sum(), 4 + 3 + 1);
}

#[test]
fn dot_of_two_vec3_is_sum_of_elementwise_products() {
    let a = Vec3::new(5, 6, 7);
    let b = Vec3::new(3, 4, 5);
    assert_eq!(a.dot(&b), 5 * 3 + 6 * 4 + 7 * 5);

    let a = Vec3::new(-2, -6, 3);
    let b = Vec3::new(3, -2, 4);
    assert_eq!(a.dot(&b), (-2 * 3) + (-6 * -2) + (3 * 4));
}

#[test]
fn cross_product_of_5_6_7_and_3_4_5_is_2_neg4_2() {
    let a = Vec3::new(5, 6, 7);
    let b = Vec3::new(3, 4, 5);
    assert_eq!(a.cross(&b), Vec3::new(2, -4, 2));
}

#[test]
fn cross_product_of_neg2_neg6_3_and_3_neg2_4_is_neg18_17_22() {
    let a = Vec3::new(-2, -6, 3);
    let b = Vec3::new(3, -2, 4);
    assert_eq!(a.cross(&b), Vec3::new(-18, 17, 22));
}

#[test]
fn square_distance_of_two_vec3_is_square_magnitude_of_difference() {
    let a = Vec3::new(-2, -6, 3);
    let b = Vec3::new(3, -2, 4);
    assert_eq!(a.square_distance(&b), (a - b).square_magnitude());
}

#[test]
fn scaling_a_vec3_by_scalar_is_componentwise_multiplication_by_scalar() {
    let mut a = Vec3::new(-2, -6, 3);
    assert_eq!(a.scale(2), &mut Vec3::new(-2 * 2, -6 * 2, 3 * 2));
}

#[test]
fn scaling_a_vec3_by_vec3_is_componentwise_multiplication() {
    let mut a = Vec3::new(-2, -6, 3);
    let b = Vec3::new(3, -2, 4);
    assert_eq!(a.scale_by(&b), &mut Vec3::new(-2 * 3, -6 * -2, 3 * 4));
}

#[test]
fn magnitude_of_vec3_is_square_root_of_square_magnitude() {
    let a = Vec3::new(-2.0, -6.0, 3.0);
    assert_approx_eq!(a.magnitude(), a.square_magnitude().sqrt());
}

#[test]
fn distance_between_two_vec3_is_square_root_of_square_distance() {
    let a = Vec3::new(-2.0, -6.0, 3.0);
    let b = Vec3::new(3.0, -2.0, 4.0);
    assert_approx_eq!(a.distance_to(&b), a.square_distance(&b).sqrt());
}

#[test]
fn normalized_float_vec3_has_magnitude_of_one() {
    let mut a = Vec3::new(-2.0, -6.0, 3.0);
    assert_approx_eq!(a.normalize().magnitude(), 1.0);
}

#[test]
fn vec3_should_have_unit_x_as_1_0_0() {
    assert_eq!(Vec3::unit_x(), Vec3::new(1, 0, 0));
}

#[test]
fn vec3_should_have_unit_y_as_0_1_0() {
    assert_eq!(Vec3::unit_y(), Vec3::new(0, 1, 0));
}

#[test]
fn vec3_should_have_unit_z_as_0_0_1() {
    assert_eq!(Vec3::unit_z(), Vec3::new(0, 0, 1));
}

#[test]
fn rotate_x_on_vec3_rotates_counterclockwise_in_yz_plane() {
    const PI: f32 = core::f32::consts::PI;
    assert_approx_eq!(
        *Vec3::new(-2.0, -6.0, 3.0).rotate_x((PI / 2.0).as_radians()),
        Vec3::new(-2.0, -3.0, -6.0)
    );
    assert_approx_eq!(
        *Vec3::new(-2.0, -6.0, 3.0).rotate_x((PI).as_radians()),
        Vec3::new(-2.0, 6.0, -3.0)
    );
    assert_approx_eq!(
        *Vec3::new(-2.0, -6.0, 3.0).rotate_x((3.0 * PI / 2.0).as_radians()),
        Vec3::new(-2.0, 3.0, 6.0)
    );
    assert_approx_eq!(
        *Vec3::new(-2.0, -6.0, 3.0).rotate_x((2.0 * PI).as_radians()),
        Vec3::new(-2.0, -6.0, 3.0)
    );
}

#[test]
fn rotate_y_on_vec3_rotates_counterclockwise_in_xz_plane() {
    const PI: f32 = core::f32::consts::PI;
    assert_approx_eq!(
        *Vec3::new(-2.0, -6.0, 3.0).rotate_y((PI / 2.0).as_radians()),
        Vec3::new(-3.0, -6.0, -2.0)
    );
    assert_approx_eq!(
        *Vec3::new(-2.0, -6.0, 3.0).rotate_y((PI).as_radians()),
        Vec3::new(2.0, -6.0, -3.0)
    );
    assert_approx_eq!(
        *Vec3::new(-2.0, -6.0, 3.0).rotate_y((3.0 * PI / 2.0).as_radians()),
        Vec3::new(3.0, -6.0, 2.0)
    );
    assert_approx_eq!(
        *Vec3::new(-2.0, -6.0, 3.0).rotate_y((2.0 * PI).as_radians()),
        Vec3::new(-2.0, -6.0, 3.0)
    );
}

#[test]
fn rotate_z_on_vec3_rotates_counterclockwise_in_xy_plane() {
    const PI: f32 = core::f32::consts::PI;
    assert_approx_eq!(
        *Vec3::new(-2.0, -6.0, 3.0).rotate_z((PI / 2.0).as_radians()),
        Vec3::new(6.0, -2.0, 3.0)
    );
    assert_approx_eq!(
        *Vec3::new(-2.0, -6.0, 3.0).rotate_z((PI).as_radians()),
        Vec3::new(2.0, 6.0, 3.0)
    );
    assert_approx_eq!(
        *Vec3::new(-2.0, -6.0, 3.0).rotate_z((3.0 * PI / 2.0).as_radians()),
        Vec3::new(-6.0, 2.0, 3.0)
    );
    assert_approx_eq!(
        *Vec3::new(-2.0, -6.0, 3.0).rotate_z((2.0 * PI).as_radians()),
        Vec3::new(-2.0, -6.0, 3.0)
    );
}
