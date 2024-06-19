// extern crate flate2;

// use flate2::write::GzEncoder;
// use flate2::Compression;
// use std::env;
// use std::fs::File;
// use std::io::{self, BufReader, copy};
// use std::time::Instant;

// fn main() -> io::Result<()> {
//     let args: Vec<String> = env::args().collect();
    
//     if args.len() != 3 {
//         eprintln!("Usage: <source> <target>");
//         std::process::exit(1);
//     }
    
//     let source = &args[1];
//     let target = &args[2];
    
//     let input_file = File::open(source)?;
//     let mut input = BufReader::new(input_file);
//     let output_file = File::create(target)?;
//     let mut encoder = GzEncoder::new(output_file, Compression::best());
    
//     let start = Instant::now();
//     copy(&mut input, &mut encoder)?;
//     let output = encoder.finish()?;
    
//     println!(
//         "Source len: {}",
//         input.get_ref().metadata()?.len()
//     );
//     println!("Target len: {}", output.metadata()?.len());
//     println!("Elapsed: {:?}", start.elapsed());
    
//     Ok(())
// }





// mod compression;

// use eframe::egui;
// use rfd::FileDialog;
// use compression::{compress_file, compress_folder};
// use log::{error, info};

// #[derive(Default)]
// struct MyApp {
//     source_file: Option<String>,
//     target_file: Option<String>,
//     source_folder: Option<String>,
//     folder_target_file: Option<String>,
//     compression_result: Option<String>,
//     folder_compression_result: Option<String>,
// }

// impl eframe::App for MyApp {
//     fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("File Compressor");

//             ui.horizontal(|ui| {
//                 if ui.button("File Compression").clicked() {
//                     self.compression_result = None;
//                     self.folder_compression_result = None;
//                 }
//                 if ui.button("Folder Compression").clicked() {
//                     self.compression_result = None;
//                     self.folder_compression_result = None;
//                 }
//             });

//             egui::ScrollArea::vertical().show(ui, |ui| {
//                 egui::CollapsingHeader::new("File Compression").show(ui, |ui| {
//                     if ui.button("Select Source File").clicked() {
//                         if let Some(file) = FileDialog::new().pick_file() {
//                             self.source_file = Some(file.display().to_string());
//                             info!("Selected source file: {}", self.source_file.as_ref().unwrap());
//                         }
//                     }

//                     if let Some(ref source) = self.source_file {
//                         ui.label(format!("Source File: {}", source));
//                     }

//                     if ui.button("Select Target File").clicked() {
//                         if let Some(file) = FileDialog::new().save_file() {
//                             self.target_file = Some(file.display().to_string());
//                             info!("Selected target file: {}", self.target_file.as_ref().unwrap());
//                         }
//                     }

//                     if let Some(ref target) = self.target_file {
//                         ui.label(format!("Target File: {}", target));
//                     }

//                     if ui.button("Compress File").clicked() {
//                         if let (Some(source), Some(target)) = (&self.source_file, &self.target_file) {
//                             match compress_file(source, target) {
//                                 Ok(msg) => self.compression_result = Some(msg),
//                                 Err(e) => {
//                                     error!("Compression failed: {:?}", e);
//                                     self.compression_result = Some(format!("Error: {:?}", e))
//                                 },
//                             }
//                         } else {
//                             self.compression_result = Some("Please select both source and target files.".to_string());
//                         }
//                     }

//                     if let Some(result) = &self.compression_result {
//                         ui.label(result);
//                     }
//                 });

//                 egui::CollapsingHeader::new("Folder Compression").show(ui, |ui| {
//                     if ui.button("Select Source Folder").clicked() {
//                         if let Some(folder) = FileDialog::new().pick_folder() {
//                             self.source_folder = Some(folder.display().to_string());
//                             info!("Selected source folder: {}", self.source_folder.as_ref().unwrap());
//                         }
//                     }

//                     if let Some(ref source) = self.source_folder {
//                         ui.label(format!("Source Folder: {}", source));
//                     }

//                     if ui.button("Select Target File").clicked() {
//                         if let Some(file) = FileDialog::new().save_file() {
//                             self.folder_target_file = Some(file.display().to_string());
//                             info!("Selected target file: {}", self.folder_target_file.as_ref().unwrap());
//                         }
//                     }

//                     if let Some(ref target) = self.folder_target_file {
//                         ui.label(format!("Target File: {}", target));
//                     }

//                     if ui.button("Compress Folder").clicked() {
//                         if let (Some(source), Some(target)) = (&self.source_folder, &self.folder_target_file) {
//                             match compress_folder(source, target) {
//                                 Ok(msg) => self.folder_compression_result = Some(msg),
//                                 Err(e) => {
//                                     error!("Compression failed: {:?}", e);
//                                     self.folder_compression_result = Some(format!("Error: {:?}", e))
//                                 },
//                             }
//                         } else {
//                             self.folder_compression_result = Some("Please select both source folder and target file.".to_string());
//                         }
//                     }

//                     if let Some(result) = &self.folder_compression_result {
//                         ui.label(result);
//                     }
//                 });
//             });
//         });
//     }
// }


// fn main() {
//     env_logger::init();
//     let options = eframe::NativeOptions::default();
//     let _ = eframe::run_native(
//         "Minoot",
//         options,
//         Box::new(|_cc| Box::new(MyApp::default())),
//     );
// }

mod compression;

use eframe::egui;
use rfd::FileDialog;
use compression::{compress_file, compress_folder};
use log::{error, info};
use eframe::App;
use std::path::PathBuf;
use dirs::download_dir;

#[derive(Default)]
struct MyApp {
    source_file: Option<PathBuf>,
    target_file: Option<PathBuf>,
    source_folder: Option<PathBuf>,
    folder_target_file: Option<PathBuf>,
    compression_result: Option<String>,
    folder_compression_result: Option<String>,
    loading: bool,
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("File Compressor").on_hover_text("Welcome to File Compressor!");

            ui.horizontal(|ui| {
                if ui.button("File Compression").clicked() {
                    self.reset_compression();
                }
                if ui.button("Folder Compression").clicked() {
                    self.reset_compression();
                }
            });

            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::CollapsingHeader::new("File Compression").default_open(true).show(ui, |ui| {
                    self.file_compression_ui(ui);
                });

                egui::CollapsingHeader::new("Folder Compression").default_open(true).show(ui, |ui| {
                    self.folder_compression_ui(ui);
                });
            });
        });
    }
}

impl MyApp {
    fn reset_compression(&mut self) {
        self.compression_result = None;
        self.folder_compression_result = None;
    }

    fn set_default_target_file(&mut self) {
        if let Some(source) = &self.source_file {
            if let Some(download_dir) = download_dir() {
                let file_name = source.file_name().unwrap_or_default();
                self.target_file = Some(download_dir.join(file_name));
                info!("Default target file set to: {:?}", self.target_file);
            }
        }
    }

    fn set_default_folder_target_file(&mut self) {
        if let Some(source) = &self.source_folder {
            if let Some(download_dir) = download_dir() {
                let folder_name = source.file_name().unwrap_or_default();
                self.folder_target_file = Some(download_dir.join(format!("{}.zip", folder_name.to_string_lossy())));
                info!("Default folder target file set to: {:?}", self.folder_target_file);
            }
        }
    }

    fn file_compression_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Select a source file and a destination to compress:");

        if let Some(ref source) = self.source_file {
            ui.label(format!("Source File: {:?}", source));
        }

        // if let Some(ref target) = self.target_file {
        //     ui.label(format!("Destination: {:?}", target));
        // }

        ui.horizontal(|ui| {
            if ui.button("Select Source File").clicked() {
                if let Some(file) = FileDialog::new().pick_file() {
                    self.source_file = Some(file.clone());
                    self.set_default_target_file();
                    info!("Selected source file: {:?}", file);
                }
            }

            if ui.button("Select Destination").clicked() {
                if let Some(download_dir) = download_dir() {
                    if let Some(file) = FileDialog::new()
                        .set_directory(download_dir)
                        .set_file_name(
                            self.source_file
                                .as_ref()
                                .and_then(|s| s.file_name())
                                .map(|s| s.to_string_lossy().to_string())
                                .unwrap_or_default(),
                        )
                        .save_file()
                    {
                        self.target_file = Some(file.clone());
                        info!("Selected destination: {:?}", file);
                    }
                }
            }
        });

        if ui.button("Compress File").clicked() {
            if let (Some(source), Some(target)) = (
                self.source_file.as_ref().and_then(|p| p.to_str()),
                self.target_file.as_ref().and_then(|p| p.to_str()),
            ) {
                self.loading = true;
                match compress_file(source, target) {
                    Ok(msg) => self.compression_result = Some(msg),
                    Err(e) => {
                        error!("Compression failed: {:?}", e);
                        self.compression_result = Some(format!("Error: {:?}", e));
                    }
                }
                self.loading = false;
            } else {
                self.compression_result = Some("Please select both source file and target destination.".to_string());
            }
        }

        if let Some(result) = &self.compression_result {
            ui.label(result);
        }

        if self.loading {
            ui.label("Compressing file...");
        }
    }

    fn folder_compression_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Select a source folder and a destination to compress:");

        if let Some(ref source) = self.source_folder {
            ui.label(format!("Source Folder: {:?}", source));
        }

        // if let Some(ref target) = self.folder_target_file {
        //     ui.label(format!("Destination: {:?}", target));
        // }

        ui.horizontal(|ui| {
            if ui.button("Select Source Folder").clicked() {
                if let Some(folder) = FileDialog::new().pick_folder() {
                    self.source_folder = Some(folder.clone());
                    self.set_default_folder_target_file();
                    info!("Selected source folder: {:?}", folder);
                }
            }

            if ui.button("Select Destination").clicked() {
                if let Some(download_dir) = download_dir() {
                    if let Some(file) = FileDialog::new()
                        .set_directory(download_dir)
                        .set_file_name(
                            self.source_folder
                                .as_ref()
                                .and_then(|s| s.file_name())
                                .map(|s| format!("{}.zip", s.to_string_lossy()))
                                .unwrap_or_default(),
                        )
                        .save_file()
                    {
                        self.folder_target_file = Some(file.clone());
                        info!("Selected destination: {:?}", file);
                    }
                }
            }
        });

        if ui.button("Compress Folder").clicked() {
            if let (Some(source), Some(target)) = (
                self.source_folder.as_ref().and_then(|p| p.to_str()),
                self.folder_target_file.as_ref().and_then(|p| p.to_str()),
            ) {
                self.loading = true;
                match compress_folder(source, target) {
                    Ok(msg) => self.folder_compression_result = Some(msg),
                    Err(e) => {
                        error!("Compression failed: {:?}", e);
                        self.folder_compression_result = Some(format!("Error: {:?}", e));
                    }
                }
                self.loading = false;
            } else {
                self.folder_compression_result = Some("Please select both source folder and destination.".to_string());
            }
        }

        if let Some(result) = &self.folder_compression_result {
            ui.label(result);
        }

        if self.loading {
            ui.label("Compressing folder...");
        }
    }
}

fn main() {
    // Initialize logger
    env_logger::init();

    // Run the application
    let _ = eframe::run_native(
        "Minoot",
        eframe::NativeOptions {
            ..Default::default()
        },
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}


// extern crate flate2;

// use flate2::write::GzEncoder;
// use flate2::Compression;
// use std::env;
// use std::fs::{File, metadata};
// use std::io::{self, BufReader, BufWriter, copy};
// // use std::path::Path;
// use std::time::Instant;

// fn main() -> io::Result<()> {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 3 {
//         eprintln!("Usage: <source_file> <target_file>");
//         std::process::exit(1);
//     }

//     let source_file = &args[1];
//     let target_file = &args[2];

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

//     println!("Source file size: {} bytes", input_metadata.len());
//     println!("Compressed file size: {} bytes", output_metadata.len());
//     println!("Compression ratio: {:.2}%", 
//              (1.0 - (output_metadata.len() as f64 / input_metadata.len() as f64)) * 100.0);
//     println!("Compression time: {:?}", elapsed);

//     Ok(())
// }
