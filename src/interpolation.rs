pub fn interpolate(xs: &[f64], ys: &[f64], x_target: f64) -> f64 {
    if x_target <= xs[0] {
        return ys[0];
    }

    if x_target >= xs[xs.len() - 1] {
        return ys[ys.len() - 1];
    }

    for i in 1..xs.len() {
        if xs[i - 1] <= x_target && x_target <= xs[i] {
            let x0 = xs[i - 1];
            let x1 = xs[i];
            let y0 = ys[i - 1];
            let y1 = ys[i];
            let y = y0 + (y1 - y0) * (x_target - x0) / (x1 - x0);
            return y;
        }
    }
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_values() {
        let xs = [1.0, 2.0, 3.0];
        let ys = [10.0, 20.0, 30.0];
        assert_eq!(interpolate(&xs, &ys, 1.0), 10.0);
        assert_eq!(interpolate(&xs, &ys, 2.0), 20.0);
        assert_eq!(interpolate(&xs, &ys, 3.0), 30.0);
    }

    #[test]
    fn test_basic_interpolation() {
        let xs = [1.0, 2.0, 3.0];
        let ys = [10.0, 20.0, 30.0];
        assert_eq!(interpolate(&xs, &ys, 1.5), 15.0);
        assert_eq!(interpolate(&xs, &ys, 2.5), 25.0);
    }

    #[test]
    fn test_out_of_bounds() {
        let xs = [1.0, 2.0, 3.0];
        let ys = [10.0, 20.0, 30.0];
        assert_eq!(interpolate(&xs, &ys, 0.5), 10.0);
        assert_eq!(interpolate(&xs, &ys, 3.5), 30.0);
    }
}
