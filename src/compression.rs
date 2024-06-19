// extern crate flate2;

// use flate2::write::GzEncoder;
// use flate2::Compression;
// use std::fs::{File, metadata};
// use std::io::{self, BufReader, BufWriter, copy};
// use std::time::Instant;
// use log::info;

// pub fn compress_file(source_file: &str, target_file: &str) -> io::Result<String> {
//     info!("Starting compression from {} to {}", source_file, target_file);

//     // Open input file for reading
//     let input_file = File::open(source_file)?;
//     let input_metadata = metadata(source_file)?;
//     let mut input = BufReader::new(input_file);

//     // Create output file for writing compressed data
//     let output_file = File::create(target_file)?;
//     let output = BufWriter::new(output_file);
//     let mut encoder = GzEncoder::new(output, Compression::best());

//     // Perform compression
//     let start = Instant::now();
//     copy(&mut input, &mut encoder)?;
//     encoder.finish()?;
//     let elapsed = start.elapsed();

//     // Output compression results
//     let output_metadata = metadata(target_file)?;

//     let result = format!(
//         "Source file size: {} bytes\nCompressed file size: {} bytes\nCompression ratio: {:.2}%\nCompression time: {:?}",
//         input_metadata.len(),
//         output_metadata.len(),
//         (1.0 - (output_metadata.len() as f64 / input_metadata.len() as f64)) * 100.0,
//         elapsed
//     );

//     info!("Compression completed: {}", result);

//     Ok(result)
// }




extern crate flate2;
extern crate tar;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::{File, metadata};
use std::io::{self, BufReader, BufWriter, copy};
use std::time::Instant;
use log::info;

pub fn compress_file(source_file: &str, target_file: &str) -> io::Result<String> {
    info!("Starting compression from {} to {}", source_file, target_file);

    let input_file = File::open(source_file)?;
    let input_metadata = metadata(source_file)?;
    let mut input = BufReader::new(input_file);

    let output_file = File::create(target_file)?;
    let output = BufWriter::new(output_file);
    let mut encoder = GzEncoder::new(output, Compression::best());

    let start = Instant::now();
    copy(&mut input, &mut encoder)?;
    encoder.finish()?;
    let elapsed = start.elapsed();

    let output_metadata = metadata(target_file)?;

    let result = format!(
        "Source file size: {} bytes\nCompressed file size: {} bytes\nCompression ratio: {:.2}%\nCompression time: {:?}",
        input_metadata.len(),
        output_metadata.len(),
        (1.0 - (output_metadata.len() as f64 / input_metadata.len() as f64)) * 100.0,
        elapsed
    );

    info!("Compression completed: {}", result);

    Ok(result)
}

pub fn compress_folder(source_folder: &str, target_file: &str) -> io::Result<String> {
    info!("Starting folder compression from {} to {}", source_folder, target_file);

    let tar_gz = File::create(target_file)?;
    let enc = GzEncoder::new(tar_gz, Compression::best());
    let mut tar = tar::Builder::new(enc);

    let start = Instant::now();
    tar.append_dir_all("", source_folder)?;
    let enc = tar.into_inner()?;
    enc.finish()?;
    let elapsed = start.elapsed();

    let result = format!(
        "Folder compressed to {}\nCompression time: {:?}",
        target_file,
        elapsed
    );

    info!("Folder compression completed: {}", result);

    Ok(result)
}
