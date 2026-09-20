use ample::math;

#[test]
fn basic_f32_math_matches_expected_values() {
    let half_pi = core::f32::consts::FRAC_PI_2;

    assert!((math::sin(half_pi) - 1.0).abs() < 1.0e-6);
    assert_eq!(math::floor(3.75), 3.0);
    assert_eq!(math::round(3.5), 4.0);
    assert!((math::pow(2.0, 3.0) - 8.0).abs() < 1.0e-6);
}
