use litemark_core::layout::{self, Template};
use std::path::Path;
use walkdir::WalkDir;

/// Load template by name or path
pub fn load_template(template_name: &str) -> Result<Template, Box<dyn std::error::Error>> {
    // Check if it's a built-in template
    let builtin_templates = layout::create_builtin_templates();

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

/// Load font bytes from path or environment variable
/// Returns None if no custom font specified (will use default embedded font)
pub fn load_font_bytes(font_path: Option<&str>) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
    // Priority: CLI argument > Environment variable
    let env_font = std::env::var("LITEMARK_FONT").ok();
    let final_font_path = font_path.or(env_font.as_deref());

    if let Some(path) = final_font_path {
        let bytes = std::fs::read(path)?;
        Ok(Some(bytes))
    } else {
        Ok(None)
    }
}

/// Load logo bytes from path or environment variable
/// Returns None if no logo specified
pub fn load_logo_bytes(logo_path: Option<&str>) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
    // Priority: CLI argument > Environment variable
    let env_logo = std::env::var("LITEMARK_LOGO").ok();
    let final_logo_path = logo_path.or(env_logo.as_deref());

    if let Some(path) = final_logo_path {
        let bytes = std::fs::read(path)?;
        Ok(Some(bytes))
    } else {
        Ok(None)
    }
}

/// Find all image files in a directory
pub fn find_images_in_directory(dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut images = Vec::new();

    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                if matches!(
                    ext.as_str(),
                    "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "heic" | "heif"
                ) {
                    images.push(path.to_string_lossy().to_string());
                }
            }
        }
    }

    Ok(images)
}

/// Create output path with suffix
pub fn create_output_path(
    input_path: &str,
    output_dir: Option<&str>,
    suffix: &str,
) -> String {
    let input = Path::new(input_path);
    let file_stem = input.file_stem().unwrap().to_string_lossy();
    let extension = input.extension().unwrap_or_default().to_string_lossy();

    let output_filename = format!("{}{}.{}", file_stem, suffix, extension);

    if let Some(dir) = output_dir {
        Path::new(dir)
            .join(output_filename)
            .to_string_lossy()
            .to_string()
    } else {
        input
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(output_filename)
            .to_string_lossy()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_output_path_with_dir() {
        let result = create_output_path("/input/photo.jpg", Some("/output"), "_watermarked");
        assert!(result.ends_with("photo_watermarked.jpg"));
        assert!(result.contains("/output"));
    }

    #[test]
    fn test_create_output_path_without_dir() {
        let result = create_output_path("photo.jpg", None, "_watermarked");
        assert_eq!(result, "./photo_watermarked.jpg");
    }

    #[test]
    fn test_load_builtin_template() {
        let result = load_template("classic");
        assert!(result.is_ok());
    }
}
