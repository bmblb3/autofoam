use autofoam::rosin_rammler::RosinRammler;
use statrs::assert_almost_eq;

#[rstest::rstest]
#[case(100.0, 1.0, 1, (100) as f64)] // gamma(1+1/1) = gamma(2) = 1! = 1
#[case(100.0, 2.0, 2, (100*100) as f64)] // gamma(1+2/2) = gamma(3) = 1! = 1
#[case(50.0, 1.0, 1, (50) as f64)]
#[case(100.0, 1.0, 2, (2*100*100) as f64)] // gamma(1+2/1) = gamma(3) = 2! = 2
#[case(100.0, 2.0, 1, 88.6)]
fn test_kth_moment(#[case] scale: f64, #[case] shape: f64, #[case] k: u8, #[case] expected: f64) {
    let dist = RosinRammler::new(shape, scale).unwrap();
    assert_almost_eq!(dist.kth_moment(k), expected, 0.1);
}

#[rstest::rstest]
#[case(3.0, 100.0, 0.9)]
#[case(3.0, 100.0, 0.1)]
#[case(3.0, 1.0, 0.9)]
#[case(10.0, 100.0, 0.9)]
fn test_weibull_from_cdf(#[case] shape: f64, #[case] x: f64, #[case] p_x: f64) {
    let dist = RosinRammler::from_shape_and_cdf(shape, x, p_x).unwrap();
    assert_almost_eq!(dist.cdf(x), p_x, 0.001);
}

#[rstest::rstest]
#[case(210.0, 250.0)]
fn test_weibull_from_smd_and_dv90(#[case] smd: f64, #[case] dv90: f64) {
    let dist = RosinRammler::from_smd_and_dv90(smd * 1e-6, dv90 * 1e-6).unwrap();
    assert_almost_eq!(dist.smd(), smd * 1e-6, 1e-6);
}
