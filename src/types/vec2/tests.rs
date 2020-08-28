use super::*;
use crate::angle::AsRadians;

#[test]
fn sum_on_vec2_is_sum_of_components() {
    assert_eq!(Vec2::new(1, 2).sum(), 1 + 2);
    assert_eq!(Vec2::new(4, 3).sum(), 4 + 3);
}

#[test]
fn dot_of_two_vec2_is_sum_of_elementwise_products() {
    let a = Vec2::new(5, 6);
    let b = Vec2::new(3, 4);
    assert_eq!(a.dot(&b), 5 * 3 + 6 * 4);

    let a = Vec2::new(-2, -6);
    let b = Vec2::new(3, -2);
    assert_eq!(a.dot(&b), (-2 * 3) + (-6 * -2));
}

#[test]
fn square_distance_of_two_vec2_is_square_magnitude_of_difference() {
    let a = Vec2::new(-2, -6);
    let b = Vec2::new(3, -2);
    assert_eq!(a.square_distance(&b), (a - b).square_magnitude());
}

#[test]
fn scaling_a_vec2_by_scalar_is_componentwise_multiplication_by_scalar() {
    let mut a = Vec2::new(-2, -6);
    assert_eq!(a.scale(2), &mut Vec2::new(-2 * 2, -6 * 2));
}

#[test]
fn scaling_a_vec2_by_vec2_is_componentwise_multiplication() {
    let mut a = Vec2::new(-2, -6);
    let b = Vec2::new(3, -2);
    assert_eq!(a.scale_by(&b), &mut Vec2::new(-2 * 3, -6 * -2));
}

#[test]
fn magnitude_of_vec2_is_square_root_of_square_magnitude() {
    let a = Vec2::new(-2.0, -6.0);
    assert_approx_eq!(a.magnitude(), a.square_magnitude().sqrt());
}

#[test]
fn distance_between_two_vec2_is_square_root_of_square_distance() {
    let a = Vec2::new(-2.0, -6.0);
    let b = Vec2::new(3.0, -2.0);
    assert_approx_eq!(a.distance_to(&b), a.square_distance(&b).sqrt());
}

#[test]
fn normalized_float_vec2_has_magnitude_of_one() {
    let mut a = Vec2::new(-2.0, -6.0);
    assert_approx_eq!(a.normalize().magnitude(), 1.0);
}

#[test]
fn vec2_unit_x_const_is_1_0() {
    assert_eq!(Vec2::unit_x(), Vec2::new(1, 0));
}

#[test]
fn vec2_unit_y_const_is_0_1() {
    assert_eq!(Vec2::unit_y(), Vec2::new(0, 1));
}

#[test]
fn rotate_x_on_vec2_rotates_counterclockwise() {
    const PI: f32 = core::f32::consts::PI;
    assert_approx_eq!(
        *Vec2::new(-2.0, -6.0).rotate((PI / 2.0).as_radians()),
        Vec2::new(6.0, -2.0)
    );
    assert_approx_eq!(
        *Vec2::new(-2.0, -6.0).rotate((PI).as_radians()),
        Vec2::new(2.0, 6.0)
    );
    assert_approx_eq!(
        *Vec2::new(-2.0, -6.0).rotate((3.0 * PI / 2.0).as_radians()),
        Vec2::new(-6.0, 2.0)
    );
    assert_approx_eq!(
        *Vec2::new(-2.0, -6.0).rotate((2.0 * PI).as_radians()),
        Vec2::new(-2.0, -6.0)
    );
}
