use clap::Parser;
use std::error::Error;
mod args;
use args::Args;
use autofoam::vtp::calculate_polygon_areas;
use autofoam::vtp::VtkReader as VtpReader;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let vtp = VtpReader::from_file(&args.file)?;
    let (points, connectivity, offsets) = vtp.geometry()?;
    let scalar_vec = vtp.field(&args.field)?;

    let area_vec = calculate_polygon_areas(&points, &connectivity, &offsets);

    let (scalar_min, scalar_max) = scalar_vec
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &scalar| {
            (min.min(scalar), max.max(scalar))
        });

    let bin_start = scalar_min.floor();
    let bin_width = 0.1;
    let num_bins = ((scalar_max.ceil() - bin_start) / bin_width).round() as usize;

    let mut area_binned = vec![0.0; num_bins];
    for (&scalar, &area) in scalar_vec.iter().zip(&area_vec) {
        let bin_index = ((scalar - bin_start) / bin_width).floor() as usize;
        if bin_index < num_bins {
            area_binned[bin_index] += area;
        }
    }

    let area_cumsum: Vec<f64> = area_binned
        .iter()
        .scan(0.0, |sum, &val| {
            *sum += val;
            Some(*sum)
        })
        .collect();

    let area_target = if let Some(percentile) = args.percentile {
        (percentile * 0.01) * area_vec.iter().sum::<f64>()
    } else if let Some(area) = args.area {
        area
    } else {
        panic!("Either percentile or area must be provided");
    };

    let scalar_target = match area_cumsum
        .binary_search_by(|probe| probe.partial_cmp(&area_target).unwrap())
    {
        Ok(idx) | Err(idx) if idx == 0 => bin_start,
        Ok(idx) | Err(idx) if idx >= area_cumsum.len() => bin_start + (num_bins as f64) * bin_width,
        Ok(idx) | Err(idx) => {
            let x0 = bin_start + ((idx - 1) as f64) * bin_width;
            let x1 = bin_start + (idx as f64) * bin_width;
            let y0 = area_cumsum[idx - 1];
            let y1 = area_cumsum[idx];

            if (y1 - y0).abs() < f64::EPSILON {
                // if bin is essentially empty
                x0
            } else {
                x0 + (area_target - y0) * (x1 - x0) / (y1 - y0)
            }
        }
    };

    println!("{:.1}", scalar_target);

    Ok(())
}
