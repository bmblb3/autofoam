use std::f64::consts::PI;

pub fn gamma_function(z: f64) -> f64 {
    if z <= 0.0 {
        return f64::INFINITY;
    }

    // For small values, use lookup/approximation
    if z < 1.0 {
        return gamma_function(z + 1.0) / z;
    }

    // Stirling's approximation for z >= 1
    (2.0 * PI / z).sqrt() * (z / std::f64::consts::E).powf(z)
}
