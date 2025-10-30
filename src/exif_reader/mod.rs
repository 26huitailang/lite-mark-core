use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use exif::{In, Reader, Tag, Value};

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

/// 从图片文件中提取 EXIF 数据
///
/// # Arguments
/// * `image_path` - 图片文件路径
///
/// # Returns
/// * `Ok(ExifData)` - 成功提取的 EXIF 数据，缺失字段为 None
/// * `Err` - 文件读取错误
///
/// # Examples
/// ```
/// let exif_data = extract_exif_data("photo.jpg")?;
/// if let Some(iso) = exif_data.iso {
///     println!("ISO: {}", iso);
/// }
/// ```
pub fn extract_exif_data(image_path: &str) -> Result<ExifData, Box<dyn std::error::Error>> {
    println!("Extracting EXIF data from: {}", image_path);

    // 打开文件
    let file = match File::open(image_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file {}: {}", image_path, e);
            return Err(Box::new(e));
        }
    };

    let mut bufreader = BufReader::new(file);

    // 解析 EXIF 数据
    let exifreader = Reader::new();
    let exif = match exifreader.read_from_container(&mut bufreader) {
        Ok(exif) => exif,
        Err(e) => {
            eprintln!("Warning: No EXIF data found in image: {}", image_path);
            eprintln!("  Error: {}", e);
            eprintln!("  This image may not contain EXIF data or format is unsupported");
            eprintln!("  Returning empty EXIF data");
            return Ok(ExifData::new());
        }
    };

    // 提取各个字段
    let mut data = ExifData::new();
    data.iso = extract_iso(&exif);
    data.aperture = extract_aperture(&exif);
    data.shutter_speed = extract_shutter_speed(&exif);
    data.focal_length = extract_focal_length(&exif);
    data.camera_model = extract_camera_model(&exif);
    data.lens_model = extract_lens_model(&exif);
    data.date_time = extract_date_time(&exif);
    data.author = extract_author(&exif);

    println!("Successfully extracted EXIF data:");
    println!("  ISO: {:?}", data.iso);
    println!("  Aperture: {:?}", data.aperture);
    println!("  Shutter: {:?}", data.shutter_speed);
    println!("  Focal: {:?}", data.focal_length);
    println!("  Camera: {:?}", data.camera_model);
    println!("  Lens: {:?}", data.lens_model);
    println!("  DateTime: {:?}", data.date_time);
    println!("  Author: {:?}", data.author);

    Ok(data)
}

/// 提取 ISO 感光度
fn extract_iso(exif: &exif::Exif) -> Option<u32> {
    let field = exif.get_field(Tag::PhotographicSensitivity, In::PRIMARY)?;
    field.value.get_uint(0)
}

/// 提取光圈值
fn extract_aperture(exif: &exif::Exif) -> Option<f64> {
    let field = exif.get_field(Tag::FNumber, In::PRIMARY)?;
    if let Value::Rational(rationals) = &field.value {
        if let Some(rational) = rationals.first() {
            return Some(rational.num as f64 / rational.denom as f64);
        }
    }
    None
}

/// 提取快门速度并格式化
fn extract_shutter_speed(exif: &exif::Exif) -> Option<String> {
    let field = exif.get_field(Tag::ExposureTime, In::PRIMARY)?;
    if let Value::Rational(rationals) = &field.value {
        if let Some(rational) = rationals.first() {
            let exposure_time = rational.num as f64 / rational.denom as f64;
            return Some(format_shutter_speed(exposure_time));
        }
    }
    None
}

/// 格式化快门速度
fn format_shutter_speed(exposure_time: f64) -> String {
    if exposure_time >= 1.0 {
        format!("{}s", exposure_time as u32)
    } else {
        let denominator = (1.0 / exposure_time).round() as u32;
        format!("1/{}", denominator)
    }
}

/// 提取焦距
fn extract_focal_length(exif: &exif::Exif) -> Option<f64> {
    let field = exif.get_field(Tag::FocalLength, In::PRIMARY)?;
    if let Value::Rational(rationals) = &field.value {
        if let Some(rational) = rationals.first() {
            return Some(rational.num as f64 / rational.denom as f64);
        }
    }
    None
}

/// 提取相机型号
fn extract_camera_model(exif: &exif::Exif) -> Option<String> {
    let field = exif.get_field(Tag::Model, In::PRIMARY)?;
    Some(field.display_value().to_string())
}

/// 提取镜头型号
fn extract_lens_model(exif: &exif::Exif) -> Option<String> {
    let field = exif.get_field(Tag::LensModel, In::PRIMARY)?;
    Some(field.display_value().to_string())
}

/// 提取拍摄时间
fn extract_date_time(exif: &exif::Exif) -> Option<String> {
    let field = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY)?;
    Some(field.display_value().to_string())
}

/// 提取作者/摄影师
fn extract_author(exif: &exif::Exif) -> Option<String> {
    let field = exif.get_field(Tag::Artist, In::PRIMARY)?;
    Some(field.display_value().to_string())
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

    #[test]
    fn test_format_shutter_speed_fast() {
        assert_eq!(format_shutter_speed(0.008), "1/125");
        assert_eq!(format_shutter_speed(0.002), "1/500");
        assert_eq!(format_shutter_speed(0.001), "1/1000");
        assert_eq!(format_shutter_speed(1.0 / 60.0), "1/60");
    }

    #[test]
    fn test_format_shutter_speed_slow() {
        assert_eq!(format_shutter_speed(1.0), "1s");
        assert_eq!(format_shutter_speed(2.0), "2s");
        assert_eq!(format_shutter_speed(5.0), "5s");
    }

    #[test]
    fn test_exif_data_new() {
        let data = ExifData::new();
        assert!(data.iso.is_none());
        assert!(data.aperture.is_none());
        assert!(data.shutter_speed.is_none());
        assert!(data.focal_length.is_none());
        assert!(data.camera_model.is_none());
        assert!(data.lens_model.is_none());
        assert!(data.date_time.is_none());
        assert!(data.author.is_none());
    }

    #[test]
    fn test_exif_data_default() {
        let data = ExifData::default();
        assert!(data.iso.is_none());
        assert!(data.aperture.is_none());
    }
}
