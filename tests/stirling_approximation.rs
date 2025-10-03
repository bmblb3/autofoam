use autofoam::gamma_function::stirling_approximation::gamma_function;

#[rstest::rstest]
#[case(0.001)]
#[case(0.01)]
#[case(0.1)]
#[case(1.0)]
#[case(10.0)]
#[case(100.0)]
fn test_output(#[case] expected: f64) {
    let epsilon_percent = (0.0001 / expected) + 0.05; // asymptotic series is less accurate for smaller values
    let actual = gamma_function(expected + 1.0) / gamma_function(expected);
    let diff = (actual - expected).abs();
    let percent_diff = if expected == 0.0 {
        diff
    } else {
        diff / (expected)
    };
    assert!(
        percent_diff < epsilon_percent,
        "expected: {:.2}-{:.2}, actual: {}",
        expected - (epsilon_percent * expected),
        expected + (epsilon_percent * expected),
        actual
    );
}
