use super::vectors::Vec2;

#[test]
fn test_vec2_element_wise_multiplication() {
    let v1 = Vec2::new(2.0, 3.0);
    let v2 = Vec2::new(4.0, 5.0);
    let result = v1 * v2;
    assert_eq!(result.x, 8.0);
    assert_eq!(result.y, 15.0);
}

#[test]
fn test_vec2_scalar_multiplication() {
    let v = Vec2::new(2.0, 3.0);
    let scalar = 2.5;
    let result1 = v * scalar;

    assert_eq!(result1.x, 5.0);
    assert_eq!(result1.y, 7.5);

    let result2 = scalar * v;
    assert_eq!(result2.x, 5.0);
    assert_eq!(result2.y, 7.5);
}

#[test]
fn test_vec2_element_wise_with_0() {
    let v1 = Vec2::new(1.0, 1.0);
    let v2 = Vec2::new(0.5, 0.0);
    let result = v1 * v2;
    assert_eq!(result.x, 0.5);
    assert_eq!(result.y, 0.0);
}
