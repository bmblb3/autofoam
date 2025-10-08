use std::error::Error;

use autofoam::histogram::weighted_histogram;
use autofoam::interpolation::interpolate;
use autofoam::vtk::calculate_polygon_areas;
use autofoam::vtk::VtpProcessor;
use clap::ArgGroup;
use clap::Parser;

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

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let vtp = VtpProcessor::from_file(&args.file)?;
    let (points, connectivity, offsets) = vtp.geometry()?;

    let scalar_vec = vtp.field(&args.field)?;

    let area_vec = calculate_polygon_areas(&points, &connectivity, &offsets);

    let bin_width = 0.1;

    let histogram = weighted_histogram(&scalar_vec, &area_vec, &bin_width);

    let area_binned = &histogram.heights;
    let bins = &histogram.bin_edges;

    let mut area_cumsum: Vec<f64> = area_binned
        .iter()
        .scan(0.0, |sum, &val| {
            *sum += val;
            Some(*sum)
        })
        .collect();
    if let Some(&last) = area_cumsum.last() {
        area_cumsum.push(last);
    }

    let area_target = if let Some(percentile) = args.percentile {
        (percentile * 0.01) * area_vec.iter().sum::<f64>()
    } else if let Some(area) = args.area {
        area
    } else {
        panic!("Either percentile or area must be provided");
    };

    let scalar_target = interpolate(&area_cumsum, bins, area_target);

    println!("{:.1}", scalar_target);

    Ok(())
}
