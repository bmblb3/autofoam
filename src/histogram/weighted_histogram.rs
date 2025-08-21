use super::HistogramResult;

pub fn weighted_histogram(values: &[f64], weights: &[f64], bin_width: &f64) -> HistogramResult {
    if values.is_empty() {
        panic!("Cannot create histogram from empty input");
    }

    let min_value = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_value = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let bin_edges: Vec<f64> = (0..)
        .map(|i| min_value + i as f64 * bin_width)
        .take_while(|&edge| edge <= max_value + bin_width)
        .collect();
    let mut heights: Vec<f64> = vec![0.0; bin_edges.len() - 1];

    for i in 0..bin_edges.len() - 1 {
        let (start, end) = (bin_edges[i], bin_edges[i + 1]);
        for (j, &value) in values.iter().enumerate() {
            if value >= start && value < end {
                heights[i] += weights[j];
            }
        }
    }

    HistogramResult { bin_edges, heights }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_weights() {
        let values = vec![1.0, 2.0];
        let weights = vec![1.0, 1.0];
        let bin_width = 1.0;
        let result = weighted_histogram(&values, &weights, &bin_width);
        // [1.0, 2.0) = 1.0
        // [2.0, 3.0) = 1.0
        assert_eq!(result.bin_edges, vec![1.0, 2.0, 3.0]);
        assert_eq!(result.heights, vec![1.0, 1.0]);
    }

    #[test]
    fn test_unequal_weights() {
        let values = vec![1.0, 2.0];
        let weights = vec![1.0, 2.0];
        let bin_width = 1.0;
        let result = weighted_histogram(&values, &weights, &bin_width);
        // [1.0, 2.0) = 1.0
        // [2.0, 3.0) = 2.0
        assert_eq!(result.bin_edges, vec![1.0, 2.0, 3.0]);
        assert_eq!(result.heights, vec![1.0, 2.0]);
    }

    #[test]
    fn test_spaced_out_values() {
        let values = vec![1.0, 3.0];
        let weights = vec![1.0, 1.0];
        let bin_width = 1.0;
        let result = weighted_histogram(&values, &weights, &bin_width);
        // [1.0, 2.0) = 1.0
        // [2.0, 3.0) = 0.0
        // [3.0, 4.0) = 1.0
        assert_eq!(result.bin_edges, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(result.heights, vec![1.0, 0.0, 1.0]);
    }

    #[test]
    fn test_reversed_values() {
        let values = vec![2.0, 1.0];
        let weights = vec![2.0, 1.0];
        let bin_width = 1.0;
        let result = weighted_histogram(&values, &weights, &bin_width);
        // [1.0, 2.0) = 1.0
        // [2.0, 3.0) = 2.0
        assert_eq!(result.bin_edges, vec![1.0, 2.0, 3.0]);
        assert_eq!(result.heights, vec![1.0, 2.0]);
    }

    #[test]
    fn test_tightly_spaced() {
        let values = vec![1.0, 1.2, 1.4, 1.6, 1.8, 2.0];
        let weights = vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        let bin_width = 1.0;
        let result = weighted_histogram(&values, &weights, &bin_width);
        // [1.0, 2.0) = 5.0
        // [2.0, 3.0) = 1.0
        assert_eq!(result.bin_edges, vec![1.0, 2.0, 3.0]);
        assert_eq!(result.heights, vec![5.0, 1.0]);
    }

    #[test]
    fn test_single_value() {
        let values = vec![5.0];
        let weights = vec![2.0];
        let bin_width = 1.0;
        let result = weighted_histogram(&values, &weights, &bin_width);
        // [5.0, 6.0) = 2.0
        assert_eq!(result.bin_edges, vec![5.0, 6.0]);
        assert_eq!(result.heights, vec![2.0]);
    }

    #[test]
    fn test_negative_values() {
        let values = vec![-2.0, -1.0, 0.0, 1.0];
        let weights = vec![1.0, 2.0, 3.0, 4.0];
        let bin_width = 1.0;
        let result = weighted_histogram(&values, &weights, &bin_width);
        // [-2.0, -1.0) = 1.0
        // [-1.0, 0.0) = 2.0
        // [0.0, 1.0) = 3.0
        // [1.0, 2.0) = 4.0
        assert_eq!(result.bin_edges, vec![-2.0, -1.0, 0.0, 1.0, 2.0]);
        assert_eq!(result.heights, vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_fractional_bin_width() {
        let values = vec![0.0, 0.25, 0.5, 0.75];
        let weights = vec![1.0, 1.0, 1.0, 1.0];
        let bin_width = 0.5;
        let result = weighted_histogram(&values, &weights, &bin_width);
        // [0.0, 0.5) = 2.0
        // [0.5, 1.0) = 2.0
        assert_eq!(result.bin_edges, vec![0.0, 0.5, 1.0]);
        assert_eq!(result.heights, vec![2.0, 2.0]);
    }

    #[test]
    fn test_large_bin_width() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let weights = vec![1.0, 1.0, 1.0, 1.0, 1.0];
        let bin_width = 10.0;
        let result = weighted_histogram(&values, &weights, &bin_width);
        // One bin: [1.0, 11.0) = 5.0
        assert_eq!(result.bin_edges, vec![1.0, 11.0]);
        assert_eq!(result.heights, vec![5.0]);
    }

    #[test]
    fn test_different_weights() {
        let values = vec![1.0, 1.5, 2.0, 2.5];
        let weights = vec![2.0, 3.0, 4.0, 5.0];
        let bin_width = 1.0;
        let result = weighted_histogram(&values, &weights, &bin_width);
        // [1.0, 2.0) = 5.0
        // [2.0, 3.0) = 9.0
        assert_eq!(result.bin_edges, vec![1.0, 2.0, 3.0]);
        assert_eq!(result.heights, vec![5.0, 9.0]);
    }

    #[test]
    fn test_identical_values() {
        let values = vec![5.0, 5.0, 5.0, 5.0];
        let weights = vec![1.0, 2.0, 3.0, 4.0];
        let bin_width = 1.0;
        let result = weighted_histogram(&values, &weights, &bin_width);
        // [5.0, 6.0) = 10.0
        assert_eq!(result.bin_edges, vec![5.0, 6.0]);
        assert_eq!(result.heights, vec![10.0]);
    }

    #[test]
    fn test_zero_weights() {
        let values = vec![1.0, 2.0, 3.0];
        let weights = vec![0.0, 5.0, 0.0];
        let bin_width = 1.0;
        let result = weighted_histogram(&values, &weights, &bin_width);
        // [1.0, 2.0) = 0.0
        // [2.0, 3.0) = 5.0
        // [3.0, 4.0) = 0.0
        assert_eq!(result.bin_edges, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(result.heights, vec![0.0, 5.0, 0.0]);
    }

    #[test]
    #[should_panic(expected = "Cannot create histogram from empty input")]
    fn test_empty_input() {
        let values = vec![];
        let weights = vec![];
        let bin_width = 1.0;
        let _result = weighted_histogram(&values, &weights, &bin_width);
    }

    #[test]
    fn test_zeros_input() {
        let values = vec![0.0, 0.0];
        let weights = vec![1.0, 1.0];
        let bin_width = 1.0;
        let result = weighted_histogram(&values, &weights, &bin_width);
        // [0.0, 1.0) = 2.0
        assert_eq!(result.bin_edges, vec![0.0, 1.0]);
        assert_eq!(result.heights, vec![2.0]);
    }
}
