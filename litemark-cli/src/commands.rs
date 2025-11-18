use litemark_core::{image_io, exif, layout, renderer::WatermarkRenderer};

use crate::utils;

/// Process a single image with watermark
pub fn process_single_image(
    input_path: &str,
    template_name: &str,
    output_path: &str,
    author: Option<&str>,
    font: Option<&str>,
    logo: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing image: {}", input_path);

    // Read image file to bytes
    let image_bytes = std::fs::read(input_path)?;
    
    // Decode image using Core
    let mut image = image_io::decode_image(&image_bytes)?;
    println!("Loaded image: {}x{}", image.width(), image.height());

    // Extract EXIF data from bytes using Core
    let exif_data = exif::extract_from_bytes(&image_bytes)?;
    println!("Extracted EXIF data: {:?}", exif_data);

    // Check for missing fields and warn user
    let missing_fields = exif_data.get_missing_fields();
    if !missing_fields.is_empty() {
        println!(
            "⚠️  Warning: Missing EXIF fields (will be skipped in watermark): {}",
            missing_fields.join(", ")
        );
    }

    // Load template
    let template = utils::load_template(template_name)?;
    println!("Using template: {}", template.name);

    // Prepare variables
    let mut variables = exif_data.to_variables();
    if let Some(author_name) = author {
        println!("Using custom author: {}", author_name);
        variables.insert("Author".to_string(), author_name.to_string());
    }
    println!("Final variables: {:?}", variables);

    // Load font bytes
    let font_bytes = utils::load_font_bytes(font)?;
    if font.is_some() {
        println!("Using custom font: {}", font.unwrap());
    } else {
        println!("Using default embedded font");
    }

    // Load logo bytes
    let logo_bytes = utils::load_logo_bytes(logo)?;
    if let Some(logo_path) = logo {
        println!("Using custom logo: {}", logo_path);
    }

    // Create renderer with font bytes
    let renderer = WatermarkRenderer::from_font_bytes(font_bytes.as_deref())?;
    
    // Render watermark with logo bytes
    renderer.render_watermark_with_logo_bytes(
        &mut image,
        &template,
        &variables,
        logo_bytes.as_deref(),
    )?;

    // Encode image to bytes
    let output_bytes = image_io::encode_image(&image, image::ImageFormat::Jpeg)?;

    // Write to file
    std::fs::write(output_path, output_bytes)?;
    println!("Saved watermarked image: {}", output_path);

    Ok(())
}

/// List available built-in templates
pub fn list_templates() {
    println!("Available templates:");
    let templates = layout::create_builtin_templates();

    for template in templates {
        println!("  • {} - {}", template.name, describe_template(&template));
    }
}

/// Show template details as JSON
pub fn show_template(template_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let template = utils::load_template(template_name)?;
    let json = template.to_json()?;
    println!("Template '{}':", template.name);
    println!("{}", json);
    Ok(())
}

fn describe_template(template: &layout::Template) -> &'static str {
    match template.name.as_str() {
        "ClassicParam" => "Bottom-left corner with photographer name and basic parameters",
        "Modern" => "Top-right corner with clean typography",
        "Minimal" => "Subtle bottom-right signature",
        _ => "Custom template",
    }
}
