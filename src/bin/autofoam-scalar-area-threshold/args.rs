use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[command(
    about = "Determines the scalar field value that defines a region with a specified total area."
)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args(["percentile", "area"]),
))]
pub struct Args {
    #[arg(long, help = "Path to .vtp file")]
    pub file: String,

    #[arg(long, help = "Scalar field name")]
    pub field: String,

    #[arg(long, help = "Percentile threshold (0-100)")]
    pub percentile: Option<f64>,

    #[arg(long, help = "Absolute area threshold")]
    pub area: Option<f64>,
}
