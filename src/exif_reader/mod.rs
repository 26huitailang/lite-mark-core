use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExifData {
    pub iso: Option<u32>,
    pub aperture: Option<f64>,
    pub shutter_speed: Option<String>,
    pub focal_length: Option<f64>,
    pub camera_model: Option<String>,
    pub lens_model: Option<String>,
    pub date_time: Option<String>,
    pub author: Option<String>,
}

impl ExifData {
    pub fn new() -> Self {
        Self {
            iso: None,
            aperture: None,
            shutter_speed: None,
            focal_length: None,
            camera_model: None,
            lens_model: None,
            date_time: None,
            author: None,
        }
    }
}

impl Default for ExifData {
    fn default() -> Self {
        Self::new()
    }
}

impl ExifData {
    pub fn to_variables(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();

        if let Some(iso) = self.iso {
            vars.insert("ISO".to_string(), iso.to_string());
        }

        if let Some(aperture) = self.aperture {
            vars.insert("Aperture".to_string(), format!("f/{:.1}", aperture));
        }

        if let Some(ref shutter) = self.shutter_speed {
            vars.insert("Shutter".to_string(), shutter.clone());
        }

        if let Some(focal) = self.focal_length {
            vars.insert("Focal".to_string(), format!("{:.0}mm", focal));
        }

        if let Some(ref camera) = self.camera_model {
            vars.insert("Camera".to_string(), camera.clone());
        }

        if let Some(ref lens) = self.lens_model {
            vars.insert("Lens".to_string(), lens.clone());
        }

        if let Some(ref datetime) = self.date_time {
            vars.insert("DateTime".to_string(), datetime.clone());
        }

        if let Some(ref author) = self.author {
            vars.insert("Author".to_string(), author.clone());
        }

        vars
    }
}

pub fn extract_exif_data(image_path: &str) -> Result<ExifData, Box<dyn std::error::Error>> {
    // For now, we'll create a simple implementation that doesn't require complex EXIF parsing
    // This is a placeholder that will be replaced with proper EXIF extraction
    let mut data = ExifData::new();

    // Simulate some EXIF data for testing
    data.iso = Some(100);
    data.aperture = Some(2.8);
    data.shutter_speed = Some("1/125".to_string());
    data.focal_length = Some(50.0);
    data.camera_model = Some("Canon EOS R5".to_string());
    data.author = Some("Photographer".to_string());

    println!("Extracting EXIF data from: {}", image_path);
    println!("Note: This is a placeholder implementation");

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exif_data_to_variables() {
        let mut data = ExifData::new();
        data.iso = Some(100);
        data.aperture = Some(2.8);
        data.shutter_speed = Some("1/125".to_string());
        data.focal_length = Some(50.0);
        data.camera_model = Some("Canon EOS R5".to_string());
        data.author = Some("John Doe".to_string());

        let vars = data.to_variables();

        assert_eq!(vars.get("ISO"), Some(&"100".to_string()));
        assert_eq!(vars.get("Aperture"), Some(&"f/2.8".to_string()));
        assert_eq!(vars.get("Shutter"), Some(&"1/125".to_string()));
        assert_eq!(vars.get("Focal"), Some(&"50mm".to_string()));
        assert_eq!(vars.get("Camera"), Some(&"Canon EOS R5".to_string()));
        assert_eq!(vars.get("Author"), Some(&"John Doe".to_string()));
    }
}
