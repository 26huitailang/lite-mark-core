use indicatif::{ProgressBar, ProgressStyle};
use litemark_core::{exif, image_io, layout, renderer::WatermarkRenderer};
use rayon::prelude::*;
use std::time::Instant;

use crate::utils;

/// Batch processing result
pub struct BatchResult {
    pub total: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub errors: Vec<(String, String)>, // (image_path, error_message)
    pub elapsed: std::time::Duration,
}

impl BatchResult {
    fn new() -> Self {
        Self {
            total: 0,
            succeeded: 0,
            failed: 0,
            errors: Vec::new(),
            elapsed: std::time::Duration::default(),
        }
    }
}

/// Batch processing configuration
struct BatchConfig {
    concurrency: usize,
}

impl BatchConfig {
    fn from_args(concurrency: Option<usize>) -> Self {
        let detected_cpus = num_cpus::get();
        let default_concurrency = (detected_cpus * 2).clamp(2, 32);

        let concurrency = concurrency.unwrap_or(default_concurrency);
        let concurrency = concurrency.clamp(1, 32); // Clamp to [1, 32]

        Self { concurrency }
    }
}

/// Process a batch of images
pub fn process_batch(
    input_dir: &str,
    template_name: &str,
    output_dir: &str,
    author: Option<&str>,
    font: Option<&str>,
    logo: Option<&str>,
    concurrency: Option<usize>,
    show_progress: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    // Create configuration
    let config = BatchConfig::from_args(concurrency);

    println!("Batch processing directory: {}", input_dir);
    println!("ℹ️  Detected {} CPU cores", num_cpus::get());
    println!("✓ Using {} concurrent tasks", config.concurrency);

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(output_dir)?;

    // Find all images
    let images = utils::find_images_in_directory(input_dir)?;
    let total_images = images.len();

    if total_images == 0 {
        println!("⚠️  No images found in directory");
        return Ok(());
    }

    println!("Found {} images to process\n", total_images);

    // Load font bytes once and share across all tasks
    let font_bytes = utils::load_font_bytes(font)?;
    if font.is_some() {
        println!("Using custom font: {}", font.unwrap());
    } else {
        println!("Using default embedded font");
    }

    // Load logo bytes once
    let logo_bytes = utils::load_logo_bytes(logo)?;
    if let Some(logo_path) = logo {
        println!("Using custom logo: {}", logo_path);
    }

    // Load template once (shared across all tasks)
    let template = utils::load_template(template_name)?;
    println!("Using template: {}\n", template.name);

    // Create progress bar
    let progress_bar = if show_progress {
        let pb = ProgressBar::new(total_images as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("█▓▒░-"),
        );
        pb.set_message("Processing images...");
        Some(pb)
    } else {
        None
    };

    // Configure Rayon thread pool
    rayon::ThreadPoolBuilder::new()
        .num_threads(config.concurrency)
        .build()
        .unwrap()
        .install(|| {
            // Process images in parallel
            let results: Vec<_> = images
                .par_iter()
                .map(|image_path| {
                    let result = process_single_image_in_batch(
                        image_path,
                        &template,
                        font_bytes.as_deref(),
                        logo_bytes.as_deref(),
                        output_dir,
                        author,
                    );

                    // Update progress bar
                    if let Some(ref pb) = progress_bar {
                        pb.inc(1);
                    }

                    (image_path.clone(), result)
                })
                .collect();

            // Finish progress bar
            if let Some(pb) = progress_bar {
                pb.finish_with_message("Processing complete!");
            }

            // Aggregate results
            let mut batch_result = BatchResult::new();
            batch_result.total = total_images;
            batch_result.elapsed = start_time.elapsed();

            println!("\n=== Processing Results ===");

            for (image_path, result) in results {
                match result {
                    Ok(_) => {
                        batch_result.succeeded += 1;
                        if !show_progress {
                            println!("✓ {}", image_path);
                        }
                    }
                    Err(e) => {
                        batch_result.failed += 1;
                        let error_msg = e.to_string();
                        batch_result
                            .errors
                            .push((image_path.clone(), error_msg.clone()));
                        eprintln!("✗ {}: {}", image_path, error_msg);
                    }
                }
            }

            // Print summary
            println!("\n=== Summary ===");
            println!("Total images:    {}", batch_result.total);
            println!("✓ Succeeded:     {}", batch_result.succeeded);
            println!("✗ Failed:        {}", batch_result.failed);
            println!(
                "⏱  Time elapsed:  {:.2}s",
                batch_result.elapsed.as_secs_f64()
            );

            if batch_result.total > 0 {
                let throughput = batch_result.succeeded as f64 / batch_result.elapsed.as_secs_f64();
                println!("📊 Throughput:    {:.2} images/s", throughput);
            }

            if batch_result.failed > 0 {
                println!("\n⚠️  {} images failed to process", batch_result.failed);
            }
        });

    Ok(())
}

/// Process a single image in batch mode
fn process_single_image_in_batch(
    input_path: &str,
    template: &layout::Template,
    font_bytes: Option<&[u8]>,
    logo_bytes: Option<&[u8]>,
    output_dir: &str,
    author: Option<&str>,
) -> Result<(), String> {
    // Read image file to bytes
    let image_bytes = std::fs::read(input_path).map_err(|e| e.to_string())?;

    // Decode image using Core
    let mut image = image_io::decode_image(&image_bytes).map_err(|e| e.to_string())?;

    // Extract EXIF data from bytes using Core
    let exif_data = exif::extract_from_bytes(&image_bytes).map_err(|e| e.to_string())?;

    // Check for missing fields
    let missing_fields = exif_data.get_missing_fields();
    if !missing_fields.is_empty() {
        eprintln!("  ⚠️  Missing fields: {}", missing_fields.join(", "));
    }

    // Prepare variables
    let mut variables = exif_data.to_variables();
    if let Some(author_name) = author {
        variables.insert("Author".to_string(), author_name.to_string());
    }

    // Create renderer with font bytes (using pre-loaded font)
    let renderer = WatermarkRenderer::from_font_bytes(font_bytes).map_err(|e| e.to_string())?;

    // Render watermark with logo bytes
    renderer
        .render_watermark_with_logo_bytes(&mut image, template, &variables, logo_bytes)
        .map_err(|e| e.to_string())?;

    // Create output path
    let output_path = utils::create_output_path(input_path, Some(output_dir), "_watermarked");

    // Handle HEIC/HEIF format conversion to JPEG
    let final_output_path = if input_path.to_lowercase().ends_with(".heic")
        || input_path.to_lowercase().ends_with(".heif")
    {
        output_path
            .replace(".HEIC", ".jpg")
            .replace(".heic", ".jpg")
            .replace(".HEIF", ".jpg")
            .replace(".heif", ".jpg")
    } else {
        output_path
    };

    // Encode image to bytes
    let output_bytes =
        image_io::encode_image(&image, image::ImageFormat::Jpeg).map_err(|e| e.to_string())?;

    // Write to file
    std::fs::write(&final_output_path, output_bytes).map_err(|e| e.to_string())?;

    Ok(())
}
