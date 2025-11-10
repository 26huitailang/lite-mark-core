use clap::{Parser, Subcommand};
use litemark::{Template, WatermarkRenderer};
use std::path::Path;

#[derive(Parser)]
#[command(name = "litemark")]
#[command(about = "A lightweight photo parameter watermark tool")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add watermark to a single image
    Add {
        /// Input image path
        #[arg(short, long)]
        input: String,

        /// Template name or path to template JSON
        #[arg(short, long, default_value = "classic")]
        template: String,

        /// Output image path
        #[arg(short, long)]
        output: String,

        /// Author name (overrides EXIF data)
        #[arg(long)]
        author: Option<String>,

        /// Custom font file path (uses default if not specified)
        #[arg(long)]
        font: Option<String>,

        /// Logo file path (overrides template and environment variable)
        #[arg(long)]
        logo: Option<String>,
    },

    /// Batch process images in a directory
    Batch {
        /// Input directory path
        #[arg(short, long)]
        input_dir: String,

        /// Template name or path to template JSON
        #[arg(short, long, default_value = "classic")]
        template: String,

        /// Output directory path
        #[arg(short, long)]
        output_dir: String,

        /// Author name (overrides EXIF data)
        #[arg(long)]
        author: Option<String>,

        /// Custom font file path (uses default if not specified)
        #[arg(long)]
        font: Option<String>,

        /// Logo file path (overrides template and environment variable)
        #[arg(long)]
        logo: Option<String>,
    },

    /// List available templates
    Templates,

    /// Show template details
    ShowTemplate {
        /// Template name
        template: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {
            input,
            template,
            output,
            author,
            font,
            logo,
        } => {
            process_single_image(
                &input,
                &template,
                &output,
                author.as_deref(),
                font.as_deref(),
                logo.as_deref(),
            )?;
        }
        Commands::Batch {
            input_dir,
            template,
            output_dir,
            author,
            font,
            logo,
        } => {
            process_batch(
                &input_dir,
                &template,
                &output_dir,
                author.as_deref(),
                font.as_deref(),
                logo.as_deref(),
            )?;
        }
        Commands::Templates => {
            list_templates();
        }
        Commands::ShowTemplate { template } => {
            show_template(&template)?;
        }
    }

    Ok(())
}

fn process_single_image(
    input_path: &str,
    template_name: &str,
    output_path: &str,
    author: Option<&str>,
    font: Option<&str>,
    logo: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing image: {}", input_path);

    // Load image
    let mut image = litemark::io::load_image(input_path)?;
    println!("Loaded image: {}x{}", image.width(), image.height());

    // Extract EXIF data
    let exif_data = litemark::exif_reader::extract_exif_data(input_path)?;
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
    let template = load_template(template_name)?;
    println!("Using template: {}", template.name);

    // Prepare variables
    let mut variables = exif_data.to_variables();
    if let Some(author_name) = author {
        println!("Using custom author: {}", author_name);
        variables.insert("Author".to_string(), author_name.to_string());
    }
    println!("Final variables: {:?}", variables);

    // Determine Logo path with priority: CLI > ENV > Template
    let env_logo = std::env::var("LITEMARK_LOGO").ok();
    let final_logo: Option<String> = match (logo, env_logo) {
        (Some(cli), _) => {
            println!("Using custom logo: {}", cli);
            Some(cli.to_string())
        }
        (None, Some(env)) => {
            println!("Using logo from environment: {}", env);
            Some(env)
        }
        (None, None) => None,
    };

    // Render watermark
    // Check for custom font from CLI or environment variable (own the String then borrow)
    let env_font = std::env::var("LITEMARK_FONT").ok();
    let font_path_owned: Option<String> = match (font, env_font) {
        (Some(f), _) => Some(f.to_string()),
        (None, e) => e,
    };
    if let Some(ref path) = font_path_owned {
        println!("Using custom font: {}", path);
    } else {
        println!("Using default embedded font");
    }
    let renderer = WatermarkRenderer::with_font(font_path_owned.as_deref())?;
    renderer.render_watermark(&mut image, &template, &variables, final_logo.as_deref())?;

    // Save output
    litemark::io::save_image(&image, output_path)?;
    println!("Saved watermarked image: {}", output_path);

    Ok(())
}

fn process_batch(
    input_dir: &str,
    template_name: &str,
    output_dir: &str,
    author: Option<&str>,
    font: Option<&str>,
    logo: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Batch processing directory: {}", input_dir);

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(output_dir)?;

    // Find all images
    let images = litemark::io::find_images_in_directory(input_dir)?;
    println!("Found {} images to process", images.len());

    // Load template
    let template = load_template(template_name)?;
    println!("Using template: {}", template.name);

    // Determine Logo path with priority: CLI > ENV > Template
    let env_logo = std::env::var("LITEMARK_LOGO").ok();
    let final_logo: Option<String> = match (logo, env_logo) {
        (Some(cli), _) => {
            println!("Using custom logo: {}", cli);
            Some(cli.to_string())
        }
        (None, Some(env)) => {
            println!("Using logo from environment: {}", env);
            Some(env)
        }
        (None, None) => None,
    };

    // Process each image
    let mut processed = 0;
    let mut errors = 0;

    for image_path in images {
        match process_single_image_in_batch(
            &image_path,
            &template,
            output_dir,
            author,
            font.as_deref(),
            final_logo.as_deref(),
        ) {
            Ok(_) => {
                processed += 1;
                println!("✓ Processed: {}", image_path);
            }
            Err(e) => {
                errors += 1;
                eprintln!("✗ Error processing {}: {}", image_path, e);
            }
        }
    }

    println!(
        "Batch processing complete: {} processed, {} errors",
        processed, errors
    );
    Ok(())
}

fn process_single_image_in_batch(
    input_path: &str,
    template: &Template,
    output_dir: &str,
    author: Option<&str>,
    font: Option<&str>,
    logo: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Load image
    let mut image = litemark::io::load_image(input_path)?;

    // Extract EXIF data
    let exif_data = litemark::exif_reader::extract_exif_data(input_path)?;

    // Check for missing fields and warn user
    let missing_fields = exif_data.get_missing_fields();
    if !missing_fields.is_empty() {
        eprintln!("  ⚠️  Missing fields: {}", missing_fields.join(", "));
    }

    // Prepare variables
    let mut variables = exif_data.to_variables();
    if let Some(author_name) = author {
        variables.insert("Author".to_string(), author_name.to_string());
    }

    // Render watermark
    // Check for custom font from CLI or environment variable (own the String then borrow)
    let env_font = std::env::var("LITEMARK_FONT").ok();
    let font_path_owned: Option<String> = match (font, env_font) {
        (Some(f), _) => Some(f.to_string()),
        (None, e) => e,
    };
    let renderer = WatermarkRenderer::with_font(font_path_owned.as_deref())?;
    renderer.render_watermark(&mut image, template, &variables, logo)?;

    // Create output path
    let output_path =
        litemark::io::create_output_path(input_path, Some(output_dir), "_watermarked");

    // Save output
    litemark::io::save_image(&image, &output_path)?;

    Ok(())
}

fn load_template(template_name: &str) -> Result<Template, Box<dyn std::error::Error>> {
    // Check if it's a built-in template
    let builtin_templates = litemark::layout::create_builtin_templates();

    // Try exact match first
    if let Some(template) = builtin_templates.iter().find(|t| t.name == template_name) {
        return Ok(template.clone());
    }

    // Try case-insensitive match
    if let Some(template) = builtin_templates
        .iter()
        .find(|t| t.name.to_lowercase() == template_name.to_lowercase())
    {
        return Ok(template.clone());
    }

    // Try common aliases
    let alias = match template_name.to_lowercase().as_str() {
        "classic" => "ClassicParam",
        "modern" => "Modern",
        "minimal" => "Minimal",
        _ => template_name,
    };

    if let Some(template) = builtin_templates.iter().find(|t| t.name == alias) {
        return Ok(template.clone());
    }

    // Check if it's a file path (absolute or relative)
    if Path::new(template_name).exists() {
        let content = std::fs::read_to_string(template_name)?;
        return Ok(Template::from_json(&content)?);
    }

    // Try loading from templates/ directory
    let template_file = format!("templates/{}.json", template_name);
    if Path::new(&template_file).exists() {
        let content = std::fs::read_to_string(&template_file)?;
        return Ok(Template::from_json(&content)?);
    }

    // Try with .json extension if not already present
    if !template_name.ends_with(".json") {
        let template_with_ext = format!("templates/{}", template_name);
        if Path::new(&template_with_ext).exists() {
            let content = std::fs::read_to_string(&template_with_ext)?;
            return Ok(Template::from_json(&content)?);
        }
    }

    Err(format!("Template '{}' not found", template_name).into())
}

fn list_templates() {
    println!("Available templates:");
    let templates = litemark::layout::create_builtin_templates();

    for template in templates {
        println!("  • {} - {}", template.name, describe_template(&template));
    }
}

fn describe_template(template: &Template) -> &'static str {
    match template.name.as_str() {
        "ClassicParam" => "Bottom-left corner with photographer name and basic parameters",
        "Modern" => "Top-right corner with clean typography",
        "Minimal" => "Subtle bottom-right signature",
        _ => "Custom template",
    }
}

fn show_template(template_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let template = load_template(template_name)?;
    let json = template.to_json()?;
    println!("Template '{}':", template.name);
    println!("{}", json);
    Ok(())
}

#[cfg(test)]
mod logo_override_tests {
    use super::*;

    #[test]
    fn test_cli_logo_overrides_env() {
        // Set environment variable
        std::env::set_var("LITEMARK_LOGO", "env_logo.png");
        
        // Simulate CLI parameter
        let cli_logo = Some("cli_logo.png");
        
        // Apply priority logic
        let env_logo = std::env::var("LITEMARK_LOGO").ok();
        let final_logo: Option<String> = match (cli_logo, env_logo) {
            (Some(cli), _) => Some(cli.to_string()),
            (None, Some(env)) => Some(env),
            _ => None,
        };
        
        // Verify result - CLI should take priority
        assert_eq!(final_logo, Some("cli_logo.png".to_string()));
        
        // Cleanup
        std::env::remove_var("LITEMARK_LOGO");
    }
    
    #[test]
    fn test_env_logo_when_no_cli() {
        // Set environment variable
        std::env::set_var("LITEMARK_LOGO", "env_logo.png");
        let cli_logo: Option<&str> = None;
        
        // Apply priority logic
        let env_logo = std::env::var("LITEMARK_LOGO").ok();
        let final_logo: Option<String> = match (cli_logo, env_logo) {
            (Some(cli), _) => Some(cli.to_string()),
            (None, Some(env)) => Some(env),
            _ => None,
        };
        
        // Verify result - should use environment variable
        assert_eq!(final_logo, Some("env_logo.png".to_string()));
        
        // Cleanup
        std::env::remove_var("LITEMARK_LOGO");
    }
    
    #[test]
    fn test_no_logo_when_all_none() {
        // Ensure no environment variable
        std::env::remove_var("LITEMARK_LOGO");
        let cli_logo: Option<&str> = None;
        
        // Apply priority logic
        let env_logo = std::env::var("LITEMARK_LOGO").ok();
        let final_logo: Option<String> = match (cli_logo, env_logo) {
            (Some(cli), _) => Some(cli.to_string()),
            (None, Some(env)) => Some(env),
            _ => None,
        };
        
        // Verify result - should be None
        assert_eq!(final_logo, None);
    }
    
    #[test]
    fn test_cli_logo_priority_with_both_set() {
        // Set environment variable
        std::env::set_var("LITEMARK_LOGO", "env_logo.png");
        let cli_logo = Some("cli_logo.png");
        
        // Apply priority logic
        let env_logo = std::env::var("LITEMARK_LOGO").ok();
        let final_logo: Option<String> = match (cli_logo, env_logo) {
            (Some(cli), _) => Some(cli.to_string()),
            (None, Some(env)) => Some(env),
            _ => None,
        };
        
        // Verify CLI parameter takes priority over environment variable
        assert_eq!(final_logo, Some("cli_logo.png".to_string()));
        assert_ne!(final_logo, Some("env_logo.png".to_string()));
        
        // Cleanup
        std::env::remove_var("LITEMARK_LOGO");
    }
}
