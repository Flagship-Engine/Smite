use super::*;

#[test]
fn sum_on_vec4_is_sum_of_components() {
    assert_eq!(Vec4::new(1, 2, 3, 4).sum(), 1 + 2 + 3 + 4);
    assert_eq!(Vec4::new(4, 3, 1, 9).sum(), 4 + 3 + 1 + 9);
}

#[test]
fn dot_of_two_vec4_is_sum_of_elementwise_products() {
    let a = Vec4::new(5, 6, 7, 8);
    let b = Vec4::new(3, 4, 5, 2);
    assert_eq!(a.dot(&b), 5 * 3 + 6 * 4 + 7 * 5 + 8*2);

    let a = Vec4::new(-2, -6, 3, 7);
    let b = Vec4::new(3, -2, 4, -3);
    assert_eq!(a.dot(&b), (-2 * 3) + (-6 * -2) + (3 * 4) + (7 * -3));
}

#[test]
fn square_distance_of_two_vec4_is_square_magnitude_of_difference() {
    let a = Vec4::new(-2, -6, 3, 7);
    let b = Vec4::new(3, -2, 4, -3);
    assert_eq!(a.square_distance(&b), (a - b).square_magnitude());
}

#[test]
fn scaling_a_vec4_by_scalar_is_componentwise_multiplication_by_scalar() {
    let mut a = Vec4::new(-2, -6, 3, 7);
    assert_eq!(a.scale(2), &mut Vec4::new(-2 * 2, -6 * 2, 3 * 2, 7 * 2));
}

#[test]
fn scaling_a_vec4_by_vec4_is_componentwise_multiplication() {
    let mut a = Vec4::new(-2, -6, 3, 7);
    let b = Vec4::new(3, -2, 4, -3);
    assert_eq!(a.scale_by(&b), &mut Vec4::new(-2 * 3, -6 * -2, 3 * 4, 7 * -3));
}

#[test]
fn magnitude_of_vec4_is_square_root_of_square_magnitude() {
    let a = Vec4::new(-2.0, -6.0, 3.0, 1.0);
    assert_approx_eq!(a.magnitude(), a.square_magnitude().sqrt());
}

#[test]
fn distance_between_two_vec4_is_square_root_of_square_distance() {
    let a = Vec4::new(-2.0, -6.0, 3.0, 1.0);
    let b = Vec4::new(3.0, -2.0, 4.0, -1.0);
    assert_approx_eq!(a.distance_to(&b), a.square_distance(&b).sqrt());
}

#[test]
fn normalized_float_vec4_has_magnitude_of_one() {
    let mut a = Vec4::new(-2.0, -6.0, 3.0, 1.0);
    assert_approx_eq!(a.normalize().magnitude(), 1.0);
}

#[test]
fn vec4_unit_x_const_is_1_0_0_0() {
    assert_eq!(Vec4::unit_x(), Vec4::new(1, 0, 0, 0));
}

#[test]
fn vec4_unit_y_const_is_0_1_0_0() {
    assert_eq!(Vec4::unit_y(), Vec4::new(0, 1, 0, 0));
}

#[test]
fn vec4_unit_z_const_is_0_0_1_0() {
    assert_eq!(Vec4::unit_z(), Vec4::new(0, 0, 1, 0));
}

#[test]
fn vec4_unit_w_const_is_0_0_0_1() {
    assert_eq!(Vec4::unit_w(), Vec4::new(0, 0, 0, 1));
}
