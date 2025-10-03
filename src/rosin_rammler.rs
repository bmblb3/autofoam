use color_eyre::eyre::Result;
use statrs::distribution::Weibull;

pub trait WeibullExt {
    fn from_shape_and_cdf(shape: f64, x: f64, p_x: f64) -> Result<Weibull>;

    fn from_smd_and_dv90(smd: f64, dv90: f64) -> Result<Weibull>;
}

impl WeibullExt for Weibull {
    fn from_shape_and_cdf(shape: f64, x: f64, p_x: f64) -> Result<Weibull> {
        if shape <= 0.0 {
            return Err(color_eyre::eyre::eyre!(
                "Shape parameter must be positive, got {}",
                shape
            ));
        }
        if x <= 0.0 {
            return Err(color_eyre::eyre::eyre!(
                "Point `x` must be positive, got {}",
                x
            ));
        }
        if (p_x <= 0.0) || (p_x >= 1.0) {
            return Err(color_eyre::eyre::eyre!(
                "Proability `p_x` must be in the range (0, 1), got {}",
                p_x
            ));
        }
        let scale = x / (-f64::ln(1.0 - p_x)).powf(1.0 / shape);
        Weibull::new(shape, scale).map_err(|e| color_eyre::eyre::eyre!("{}", e))
    }

    fn from_smd_and_dv90(smd: f64, dv90: f64) -> Result<Weibull> {
        let mut shape = 3.0;
        for _ in 0..100 {
            let lambda = Weibull::from_shape_and_cdf(shape, dv90, 0.9)?.scale();
            let calculated_smd = smd_from_distribution(shape, lambda);
            let error = (calculated_smd - smd) / smd;
            if error.abs() < 0.001 {
                break;
            }
            shape += if error > 0.0 { 0.01 } else { -0.01 };
            shape = shape.clamp(1.5, 10.0);
        }
        Weibull::from_shape_and_cdf(shape, dv90, 0.9)
    }
}

pub fn kth_moment(x_char: f64, n: f64, k: u8) -> f64 {
    // ⟨dk⟩=(x_char^k)*Γ(1+ k/b)
    x_char.powi(k as i32) * statrs::function::gamma::gamma(1.0 + ((k as f64) / n))
}

fn smd_from_distribution(shape: f64, lambda: f64) -> f64 {
    // Calculate SMD from Rosin-Rammler: SMD = <d3>/<d2>
    let d3 = kth_moment(lambda, shape, 3);
    let d2 = kth_moment(lambda, shape, 2);
    d3 / d2
}
