use autofoam::coordinates::update_coordinate_bounds;
use autofoam::stl::is_ascii;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom},
};

use clap::Parser;
mod args;

fn main() {
    let args = args::Args::parse();

    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    let mut vertex_count = 0;

    for path in &args.files {
        let mut file = File::open(path).unwrap_or_else(|e| {
            eprintln!("Failed to open {}: {}", path, e);
            std::process::exit(1);
        });

        let count = if is_ascii(&mut file) {
            process_ascii(file, &mut min, &mut max)
        } else {
            process_binary(file, &mut min, &mut max)
        }
        .unwrap_or_else(|e| {
            eprintln!("Failed to process {}: {}", path, e);
            std::process::exit(1);
        });

        vertex_count += count;
    }

    if vertex_count == 0 {
        eprintln!("No vertices found");
        std::process::exit(1);
    }

    println!(
        "{:.6} {:.6} {:.6} {:.6} {:.6} {:.6}",
        min[0], min[1], min[2], max[0], max[1], max[2]
    );
}

fn process_ascii(
    file: File,
    min: &mut [f32; 3],
    max: &mut [f32; 3],
) -> Result<usize, std::io::Error> {
    let mut count = 0;
    for line in BufReader::new(file).lines() {
        let line = line?;
        if let Some(coords) = line.trim().strip_prefix("vertex ") {
            let parts: Vec<&str> = coords.split_whitespace().collect();
            if parts.len() >= 3 {
                let x = parts[0].parse::<f32>().expect("Invalid vertex coordinate");
                let y = parts[1].parse::<f32>().expect("Invalid vertex coordinate");
                let z = parts[2].parse::<f32>().expect("Invalid vertex coordinate");
                update_coordinate_bounds([x, y, z], min, max);
                count += 1;
            }
        }
    }
    Ok(count)
}

fn process_binary(
    mut file: File,
    min: &mut [f32; 3],
    max: &mut [f32; 3],
) -> Result<usize, std::io::Error> {
    file.seek(SeekFrom::Start(80))?;

    let mut buf = [0u8; 4];
    file.read_exact(&mut buf)?;
    let triangle_count = u32::from_le_bytes(buf) as usize;

    for _ in 0..triangle_count {
        file.seek(SeekFrom::Current(12))?; // Skip normal

        for _ in 0..3 {
            file.read_exact(&mut buf)?;
            let x = f32::from_le_bytes(buf);
            file.read_exact(&mut buf)?;
            let y = f32::from_le_bytes(buf);
            file.read_exact(&mut buf)?;
            let z = f32::from_le_bytes(buf);

            update_coordinate_bounds([x, y, z], min, max);
        }

        file.seek(SeekFrom::Current(2))?; // Skip attributes
    }

    Ok(triangle_count * 3)
}
