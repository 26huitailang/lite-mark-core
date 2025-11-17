use image::{DynamicImage, ImageFormat, RgbaImage};
use std::path::Path;
use walkdir::WalkDir;
use libheif_rs::{HeifContext, ColorSpace, RgbChroma};

pub fn load_image(path: &str) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    let ext = Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    // Check if it's a HEIC/HEIF file
    if ext == "heic" || ext == "heif" {
        return load_heic_image(path);
    }

    let img = image::open(path)?;
    Ok(img)
}

/// Load HEIC/HEIF image and convert to DynamicImage
fn load_heic_image(path: &str) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    // Read HEIC file
    let ctx = HeifContext::read_from_file(path)?;
    let handle = ctx.primary_image_handle()?;
    
    // Decode to RGB - updated API for libheif-rs 1.0
    let width = handle.width();
    let height = handle.height();
    
    // Decode the image
    let image = libheif_rs::LibHeif::new().decode(
        &handle,
        ColorSpace::Rgb(RgbChroma::Rgb),
        None,
    )?;
    
    let planes = image.planes();
    let interleaved_plane = planes.interleaved.ok_or("No interleaved plane")?;
    
    // Convert to RGBA format for consistency
    let mut rgba_data = Vec::with_capacity((width * height * 4) as usize);
    let rgb_data = interleaved_plane.data;
    let stride = interleaved_plane.stride as usize;
    
    for y in 0..height {
        let row_start = y as usize * stride;
        for x in 0..width {
            let pixel_start = row_start + (x as usize * 3);
            if pixel_start + 2 < rgb_data.len() {
                rgba_data.push(rgb_data[pixel_start]);     // R
                rgba_data.push(rgb_data[pixel_start + 1]); // G
                rgba_data.push(rgb_data[pixel_start + 2]); // B
                rgba_data.push(255);                       // A (fully opaque)
            }
        }
    }
    
    let rgba_image = RgbaImage::from_raw(width, height, rgba_data)
        .ok_or("Failed to create RGBA image from HEIC data")?;
    
    Ok(DynamicImage::ImageRgba8(rgba_image))
}

pub fn save_image(image: &DynamicImage, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    image.save(path)?;
    Ok(())
}

pub fn get_image_format(path: &str) -> ImageFormat {
    let ext = Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "png" => ImageFormat::Png,
        "gif" => ImageFormat::Gif,
        "bmp" => ImageFormat::Bmp,
        "webp" => ImageFormat::WebP,
        _ => ImageFormat::Jpeg, // Default to JPEG
    }
}

pub fn find_images_in_directory(dir_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut images = Vec::new();
    let supported_extensions = ["jpg", "jpeg", "png", "gif", "bmp", "webp", "heic", "heif"];

    for entry in WalkDir::new(dir_path) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if supported_extensions.contains(&ext.to_lowercase().as_str()) {
                    images.push(path.to_string_lossy().to_string());
                }
            }
        }
    }

    Ok(images)
}

pub fn create_output_path(input_path: &str, output_dir: Option<&str>, suffix: &str) -> String {
    let input_path = Path::new(input_path);
    let stem = input_path.file_stem().unwrap_or_default().to_string_lossy();
    let extension = input_path.extension().unwrap_or_default().to_string_lossy();

    let filename = format!("{}{}.{}", stem, suffix, extension);

    if let Some(output_dir) = output_dir {
        let output_path = Path::new(output_dir);
        output_path.join(filename).to_string_lossy().to_string()
    } else {
        // Save in the same directory as input
        if let Some(parent) = input_path.parent() {
            parent.join(filename).to_string_lossy().to_string()
        } else {
            filename
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_output_path() {
        let input = "/path/to/image.jpg";
        let output = create_output_path(input, None, "_watermarked");
        assert_eq!(output, "/path/to/image_watermarked.jpg");

        let output_with_dir = create_output_path(input, Some("/output"), "_watermarked");
        assert_eq!(output_with_dir, "/output/image_watermarked.jpg");
    }
}
