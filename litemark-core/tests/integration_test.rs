use image::{DynamicImage, ImageFormat, Rgb, RgbImage};
use litemark_core::{exif, image_io, layout, renderer::WatermarkRenderer};
use litemark_core::layout::RenderMode;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct CustomerRegressionSuite {
    cases: Vec<CustomerRegressionCase>,
}

#[derive(Debug, Deserialize)]
struct CustomerRegressionCase {
    id: String,
    template: String,
    variables: HashMap<String, String>,
    expected: String,
}

fn load_customer_regressions() -> CustomerRegressionSuite {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/customer_regressions.json");
    let content = fs::read_to_string(path).expect("Failed to read customer regressions file");
    serde_json::from_str(&content).expect("Failed to parse customer regressions file")
}

#[test]
fn test_image_encode_decode_roundtrip() {
    // 创建测试图像
    let test_image = DynamicImage::ImageRgb8(RgbImage::from_fn(200, 150, |x, y| {
        if (x + y) % 2 == 0 {
            Rgb([255, 0, 0])
        } else {
            Rgb([0, 0, 255])
        }
    }));

    // 编码为 JPEG
    let jpeg_data =
        image_io::encode_image(&test_image, ImageFormat::Jpeg).expect("Failed to encode image");
    assert!(!jpeg_data.is_empty());
    assert!(jpeg_data.len() > 100);

    // 解码
    let decoded = image_io::decode_image(&jpeg_data).expect("Failed to decode image");
    assert_eq!(decoded.width(), 200);
    assert_eq!(decoded.height(), 150);

    // 编码为 PNG
    let png_data =
        image_io::encode_image(&test_image, ImageFormat::Png).expect("Failed to encode as PNG");
    assert!(!png_data.is_empty());

    // 验证 PNG 数据也是有效的
    let png_decoded = image_io::decode_image(&png_data).expect("Failed to decode PNG");
    assert_eq!(png_decoded.width(), 200);
    assert_eq!(png_decoded.height(), 150);
}

#[test]
fn test_exif_data_conversion() {
    let mut exif_data = exif::ExifData::new();
    exif_data.iso = Some(400);
    exif_data.aperture = Some(2.8);
    exif_data.shutter_speed = Some("1/200".to_string());
    exif_data.focal_length = Some(85.0);
    exif_data.camera_model = Some("Nikon Z9".to_string());
    exif_data.lens_model = Some("NIKKOR Z 85mm f/1.8 S".to_string());
    exif_data.author = Some("Test Photographer".to_string());

    let variables = exif_data.to_variables();

    assert_eq!(variables.get("ISO"), Some(&"400".to_string()));
    assert_eq!(variables.get("Aperture"), Some(&"f/2.8".to_string()));
    assert_eq!(variables.get("Shutter"), Some(&"1/200".to_string()));
    assert_eq!(variables.get("Focal"), Some(&"85mm".to_string()));
    assert_eq!(variables.get("Camera"), Some(&"Nikon Z9".to_string()));
    assert_eq!(
        variables.get("Lens"),
        Some(&"NIKKOR Z 85mm f/1.8 S".to_string())
    );
    assert_eq!(
        variables.get("Author"),
        Some(&"Test Photographer".to_string())
    );
}

#[test]
fn test_exif_missing_fields() {
    let mut exif_data = exif::ExifData::new();
    exif_data.iso = Some(200);
    // 其他字段保持 None

    let missing = exif_data.get_missing_fields();
    assert_eq!(missing.len(), 7); // 7个字段缺失

    let variables = exif_data.to_variables();
    assert_eq!(variables.len(), 1); // 只有 ISO
    assert!(variables.contains_key("ISO"));
    assert!(!variables.contains_key("Aperture"));
}

#[test]
fn test_template_builtin() {
    let templates = layout::create_builtin_templates();

    assert!(!templates.is_empty());
    assert!(templates.iter().any(|t| t.name == "Classic"));
    assert!(templates.iter().any(|t| t.name == "Compact"));
    assert!(templates.iter().any(|t| t.name == "Professional"));
    assert!(templates.iter().any(|t| t.name == "Overlay"));

    let classic = templates.iter().find(|t| t.name == "Classic").unwrap();
    assert_eq!(classic.items.len(), 5); // Logo + Author + DateTime + Camera·Lens + Parameters
}

#[test]
fn test_template_variable_substitution() {
    let template = layout::Template {
        name: "Test".to_string(),
        anchor: layout::Anchor::BottomLeft,
        padding: 20,
        items: vec![
            layout::TemplateItem {
                item_type: layout::ItemType::Text,
                value: "{Camera} - {Lens}".to_string(),
                font_size: 16,
                font_size_ratio: 0.2,
                weight: Some(layout::FontWeight::Bold),
                color: Some("#000000".to_string()),
            },
            layout::TemplateItem {
                item_type: layout::ItemType::Text,
                value: "ISO {ISO}, {Aperture}, {Shutter}".to_string(),
                font_size: 14,
                font_size_ratio: 0.15,
                weight: Some(layout::FontWeight::Normal),
                color: Some("#666666".to_string()),
            },
        ],
        background: None,
        frame_height_ratio: 0.10,
        logo_size_ratio: 0.35,
        primary_font_ratio: 0.20,
        secondary_font_ratio: 0.14,
        padding_ratio: 0.1,
        render_mode: RenderMode::BottomFrame,
    };

    let mut variables = HashMap::new();
    variables.insert("Camera".to_string(), "Canon EOS R5".to_string());
    variables.insert("Lens".to_string(), "RF 24-70mm F2.8".to_string());
    variables.insert("ISO".to_string(), "800".to_string());
    variables.insert("Aperture".to_string(), "f/2.8".to_string());
    variables.insert("Shutter".to_string(), "1/125".to_string());

    let substituted = template.substitute_variables(&variables);

    assert_eq!(substituted.items[0].value, "Canon EOS R5 - RF 24-70mm F2.8");
    assert_eq!(substituted.items[1].value, "ISO 800, f/2.8, 1/125");
}

#[test]
fn test_template_json_serialization() {
    let template = layout::Template {
        name: "CustomTest".to_string(),
        anchor: layout::Anchor::TopRight,
        padding: 15,
        items: vec![layout::TemplateItem {
            item_type: layout::ItemType::Text,
            value: "{Author}".to_string(),
            font_size: 18,
            font_size_ratio: 0.25,
            weight: Some(layout::FontWeight::Bold),
            color: Some("#FFFFFF".to_string()),
        }],
        background: None,
        frame_height_ratio: 0.12,
        logo_size_ratio: 0.4,
        primary_font_ratio: 0.25,
        secondary_font_ratio: 0.18,
        padding_ratio: 0.1,
        render_mode: RenderMode::BottomFrame,
    };

    // 序列化
    let json = template.to_json().expect("Failed to serialize");
    assert!(json.contains("CustomTest"));
    assert!(json.contains("Author"));

    // 反序列化
    let parsed = layout::Template::from_json(&json).expect("Failed to deserialize");
    assert_eq!(parsed.name, "CustomTest");
    assert_eq!(parsed.items.len(), 1);
    assert_eq!(parsed.frame_height_ratio, 0.12);
}

#[test]
fn test_renderer_creation() {
    // 测试默认字体
    let renderer = WatermarkRenderer::new();
    assert!(
        renderer.is_ok(),
        "Failed to create renderer with default font"
    );

    // 测试空字体数据
    let renderer = WatermarkRenderer::from_font_bytes(None);
    assert!(
        renderer.is_ok(),
        "Failed to create renderer with None font data"
    );
}

#[test]
fn test_full_watermark_pipeline() {
    // 创建测试图像
    let mut test_image = DynamicImage::ImageRgb8(RgbImage::from_fn(800, 600, |x, y| {
        let r = ((x as f32 / 800.0) * 255.0) as u8;
        let g = ((y as f32 / 600.0) * 255.0) as u8;
        Rgb([r, g, 255 - r])
    }));

    let original_height = test_image.height();

    // 准备 EXIF 数据
    let mut exif_data = exif::ExifData::new();
    exif_data.iso = Some(100);
    exif_data.aperture = Some(4.0);
    exif_data.shutter_speed = Some("1/60".to_string());
    exif_data.camera_model = Some("Test Camera".to_string());
    exif_data.author = Some("Integration Test".to_string());

    let variables = exif_data.to_variables();

    // 加载模板
    let templates = layout::create_builtin_templates();
    let template = templates
        .iter()
        .find(|t| t.name == "Classic")
        .expect("Classic template not found");

    // 创建渲染器
    let renderer = WatermarkRenderer::new().expect("Failed to create renderer");

    // 渲染水印
    let result = renderer.render_watermark_with_logo_bytes(
        &mut test_image,
        template,
        &variables,
        None, // 无 Logo
    );

    assert!(result.is_ok(), "Watermark rendering failed");

    // 验证图像尺寸增加了边框
    assert!(
        test_image.height() > original_height,
        "Image height should increase after adding watermark"
    );

    // 验证可以编码
    let encoded = image_io::encode_image(&test_image, ImageFormat::Jpeg);
    assert!(encoded.is_ok(), "Failed to encode watermarked image");
    assert!(
        !encoded.unwrap().is_empty(),
        "Encoded data should not be empty"
    );
}

#[test]
fn test_detect_image_format() {
    // JPEG 魔数
    let jpeg_header = vec![0xFF, 0xD8, 0xFF, 0xE0];
    let format = image_io::detect_format(&jpeg_header);
    assert!(matches!(format, ImageFormat::Jpeg));

    // PNG 魔数
    let png_header = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let format = image_io::detect_format(&png_header);
    assert!(matches!(format, ImageFormat::Png));
}

#[test]
fn test_exif_from_empty_bytes() {
    let empty_data: &[u8] = &[];
    let result = exif::extract_from_bytes(empty_data);

    // 应该返回空的 ExifData 而不是错误
    assert!(result.is_ok());
    let exif_data = result.unwrap();
    assert!(exif_data.iso.is_none());
    assert!(exif_data.camera_model.is_none());
}

#[test]
fn test_customer_regression_suite() {
    let suite = load_customer_regressions();

    for case in suite.cases {
        let template = layout::Template {
            name: case.id.clone(),
            anchor: layout::Anchor::BottomLeft,
            padding: 0,
            items: vec![layout::TemplateItem {
                item_type: layout::ItemType::Text,
                value: case.template.clone(),
                font_size: 16,
                font_size_ratio: 0.20,
                weight: Some(layout::FontWeight::Normal),
                color: Some("#000000".to_string()),
            }],
            background: None,
            frame_height_ratio: 0.10,
            logo_size_ratio: 0.35,
            primary_font_ratio: 0.20,
            secondary_font_ratio: 0.14,
            padding_ratio: 0.1,
            render_mode: RenderMode::BottomFrame,
        };

        let rendered = template.substitute_variables(&case.variables);
        assert_eq!(rendered.items[0].value, case.expected, "case: {}", case.id);
    }
}
