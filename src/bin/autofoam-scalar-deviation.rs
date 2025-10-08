use std::error::Error;

use autofoam::vtk::calculate_polygon_areas;
use autofoam::vtk::VtpProcessor;
use clap::Parser;

#[derive(Parser)]
#[command(about = "Computes and writes normalized deviation of a scalar field")]
pub struct Args {
    #[arg(long, help = "Path to .vtp file", value_hint = clap::ValueHint::FilePath)]
    pub file: String,

    #[arg(long, help = "Scalar field name to process")]
    pub field: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let vtp = VtpProcessor::from_file(&args.file)?;

    let (points, connectivity, offsets) = vtp.geometry()?;
    let field_values_vec = vtp.field(&args.field)?;

    let areas_vec = calculate_polygon_areas(&points, &connectivity, &offsets);

    let (weighted_sum, total_area): (f64, f64) = field_values_vec
        .iter()
        .zip(areas_vec.iter())
        .fold((0.0, 0.0), |(sum, area_sum), (val, area)| {
            (sum + val * area, area_sum + area)
        });
    if total_area == 0.0 {
        return Err("Total area of the reference .vtp is zero, cannot use it".into());
    }

    let area_weighted_avg = weighted_sum / total_area;

    let deviation_vec: Vec<f64> = field_values_vec
        .iter()
        .map(|f| (f - area_weighted_avg) / (area_weighted_avg.abs() + 1e-15))
        .collect();

    let deviation_field_name = format!("{}_deviation", args.field);

    let updated_vtp = if vtp.field_exists(&deviation_field_name)? {
        vtp.remove_field(&deviation_field_name)?
            .add_field(&deviation_field_name, &deviation_vec)?
    } else {
        vtp.add_field(&deviation_field_name, &deviation_vec)?
    };

    updated_vtp.write_to_file(&args.file)?;

    Ok(())
}
