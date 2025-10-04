use color_eyre::eyre::Result;

pub struct RosinRammler {
    pub shape: f64,
    pub scale: f64,
}

impl RosinRammler {
    pub fn new(shape: f64, scale: f64) -> Result<Self> {
        if shape <= 0.0 {
            return Err(color_eyre::eyre::eyre!(
                "Shape parameter must be positive, got {}",
                shape
            ));
        }
        if scale <= 0.0 {
            return Err(color_eyre::eyre::eyre!(
                "Scale parameter must be positive, got {}",
                scale
            ));
        }
        Ok(RosinRammler { shape, scale })
    }

    pub fn from_shape_and_cdf(shape: f64, x: f64, p_x: f64) -> Result<RosinRammler> {
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
        if p_x <= 0.0 || p_x >= 1.0 {
            return Err(color_eyre::eyre::eyre!(
                "Probability `p_x` must be in the range (0, 1), got {}",
                p_x
            ));
        }
        let scale = x / (-((1.0 - p_x).ln())).powf(1.0 / shape);
        Ok(RosinRammler { shape, scale })
    }

    pub fn from_smd_and_dv90(smd: f64, dv90: f64) -> Result<RosinRammler> {
        let mut lower = 1.5;
        let mut upper = 10.0;
        let mut shape = 3.0;
        for _ in 0..2000 {
            let dist = RosinRammler::from_shape_and_cdf(shape, dv90, 0.9)?;
            let calculated_smd = dist.smd();
            let error = calculated_smd - smd;

            if error.abs() / smd < 0.001 {
                return Ok(dist);
            }
            if error > 0.0 {
                upper = shape;
            } else {
                lower = shape;
            }
            shape = 0.5 * (lower + upper);
        }
        RosinRammler::from_shape_and_cdf(shape, dv90, 0.9)
    }

    pub fn smd(&self) -> f64 {
        let d3 = self.kth_moment(3);
        let d2 = self.kth_moment(2);
        d3 / d2
    }

    pub fn kth_moment(&self, k: u8) -> f64 {
        let k_f64 = k as f64;
        self.scale.powf(k_f64) * statrs::function::gamma::gamma(1.0 + k_f64 / self.shape)
    }

    pub fn cdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            0.0
        } else {
            1.0 - (-((x / self.scale).powf(self.shape))).exp()
        }
    }
}
