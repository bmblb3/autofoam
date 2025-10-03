use autofoam::rosin_rammler::{kth_moment, WeibullExt};
use statrs::{
    assert_almost_eq,
    distribution::{ContinuousCDF, Weibull},
};

#[rstest::rstest]
#[case(100.0, 1.0, 1, (100) as f64)] // gamma(1+1/1) = gamma(2) = 1! = 1
#[case(100.0, 2.0, 2, (100*100) as f64)] // gamma(1+2/2) = gamma(3) = 1! = 1
#[case(50.0, 1.0, 1, (50) as f64)]
#[case(100.0, 1.0, 2, (2*100*100) as f64)] // gamma(1+2/1) = gamma(3) = 2! = 2
#[case(100.0, 2.0, 1, 88.6)]
fn test_kth_moment(#[case] x_char: f64, #[case] n: f64, #[case] k: u8, #[case] expected: f64) {
    let epsilon = 0.5;
    let actual = kth_moment(x_char, n, k);
    let diff = (actual - expected).abs();
    assert!(
        diff < epsilon,
        "expected: {:.2}-{:.2}, actual: {}",
        expected - epsilon,
        expected + epsilon,
        actual
    );
}

#[rstest::rstest]
#[case(3.0, 100.0, 0.9)]
#[case(3.0, 100.0, 0.1)]
#[case(3.0, 1.0, 0.9)]
#[case(10.0, 100.0, 0.9)]
fn test_weibull_ext(#[case] shape: f64, #[case] x: f64, #[case] p_x: f64) {
    let dist = Weibull::from_shape_and_cdf(shape, x, p_x).unwrap();
    assert_almost_eq!(dist.cdf(x), p_x, 0.001);
    assert_almost_eq!(dist.inverse_cdf(p_x), x, 0.001);
}
