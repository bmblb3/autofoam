use autofoam::coordinates::update_coordinate_bounds;
use autofoam::stl::{is_ascii, process_ascii_iter, process_binary_iter};
use std::fs::File;
use std::io::Seek;

use clap::Parser;
mod args;

fn main() {
    let args = args::Args::parse();

    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];

    for path in &args.files {
        let mut file = File::open(path).unwrap_or_else(|e| {
            eprintln!("Failed to open {}: {}", path, e);
            std::process::exit(1);
        });

        let mut count = 0;
        let is_ascii_file = is_ascii(&mut file);
        file.seek(std::io::SeekFrom::Start(0)).unwrap();

        let iter: Box<dyn Iterator<Item = Result<[f32; 3], _>>> = if is_ascii_file {
            Box::new(process_ascii_iter(file))
        } else {
            Box::new(process_binary_iter(file))
        };

        for vertex_result in iter {
            match vertex_result {
                Ok(coords) => {
                    update_coordinate_bounds(coords, &mut min, &mut max);
                    count += 1;
                }
                Err(e) => {
                    eprintln!("Error processing file {}: {}", path, e);
                }
            }
        }

        if count == 0 {
            eprintln!("No vertices found in file {}", path);
            continue;
        }
    }

    println!(
        "{:.6} {:.6} {:.6} {:.6} {:.6} {:.6}",
        min[0], min[1], min[2], max[0], max[1], max[2]
    );
}
