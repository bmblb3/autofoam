#[derive(Debug, PartialEq)]
pub struct HistogramResult {
    pub bin_edges: Vec<f64>,
    pub heights: Vec<f64>,
}

pub mod weighted_histogram;
pub use weighted_histogram::weighted_histogram;
