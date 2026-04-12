//! 渲染器单元测试
//!
//! 测试水印渲染引擎的功能

use image::{DynamicImage, ImageFormat, Rgb, RgbImage};
use litemark_core::layout::{Anchor, FontWeight, ItemType, Template, TemplateItem};
use litemark_core::renderer::WatermarkRenderer;
use std::collections::HashMap;

/// 测试渲染器使用默认字体创建
#[test]
fn test_renderer_new_with_default_font() {
    let result = WatermarkRenderer::new();
    assert!(result.is_ok(), "应能用默认字体创建渲染器");
}

/// 测试渲染器使用 None 字体数据
#[test]
fn test_renderer_from_font_bytes_none() {
    let result = WatermarkRenderer::from_font_bytes(None);
    assert!(result.is_ok(), "None 字体数据应使用默认字体");
}

/// 测试渲染器使用空字体数据（应失败）
#[test]
fn test_renderer_from_font_bytes_empty() {
    let empty_data: &[u8] = &[];
    let result = WatermarkRenderer::from_font_bytes(Some(empty_data));
    assert!(result.is_err(), "空字体数据应返回错误");
}

/// 测试渲染器使用无效字体数据
#[test]
fn test_renderer_from_font_bytes_invalid() {
    let invalid_data: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03];
    let result = WatermarkRenderer::from_font_bytes(Some(&invalid_data));
    assert!(result.is_err(), "无效字体数据应返回错误");
}

/// 测试完整水印渲染流程
#[test]
fn test_render_watermark_full_pipeline() {
    // 创建测试图像
    let mut test_image = create_test_image(800, 600);
    let original_height = test_image.height();

    // 准备 EXIF 数据
    let mut variables = HashMap::new();
    variables.insert("Author".to_string(), "Test Photographer".to_string());
    variables.insert("Camera".to_string(), "Canon R5".to_string());
    variables.insert("Aperture".to_string(), "f/2.8".to_string());
    variables.insert("ISO".to_string(), "400".to_string());
    variables.insert("Shutter".to_string(), "1/125".to_string());

    // 创建模板
    let template = create_test_template();

    // 创建渲染器并渲染
    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");
    let result = renderer.render_watermark_with_logo_bytes(
        &mut test_image,
        &template,
        &variables,
        None, // 无 Logo
    );

    assert!(result.is_ok(), "渲染应成功");
    
    // 验证图像高度增加了（添加了水印边框）
    assert!(
        test_image.height() > original_height,
        "水印应增加图像高度"
    );
}

/// 测试渲染到极小图像
#[test]
fn test_render_watermark_tiny_image() {
    let mut test_image = create_test_image(100, 100);
    let original_height = test_image.height();

    let mut variables = HashMap::new();
    variables.insert("Author".to_string(), "Test".to_string());

    let template = create_test_template();
    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");

    let result = renderer.render_watermark_with_logo_bytes(
        &mut test_image,
        &template,
        &variables,
        None,
    );

    assert!(result.is_ok());
    assert!(test_image.height() > original_height);
}

/// 测试渲染到正方形图像
#[test]
fn test_render_watermark_square_image() {
    let mut test_image = create_test_image(1024, 1024);
    let original_height = test_image.height();

    let mut variables = HashMap::new();
    variables.insert("Author".to_string(), "Test".to_string());

    let template = create_test_template();
    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");

    let result = renderer.render_watermark_with_logo_bytes(
        &mut test_image,
        &template,
        &variables,
        None,
    );

    assert!(result.is_ok());
    assert!(test_image.height() > original_height);
}

/// 测试空变量渲染
#[test]
fn test_render_watermark_empty_variables() {
    let mut test_image = create_test_image(800, 600);

    let variables: HashMap<String, String> = HashMap::new();
    let template = create_test_template();
    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");

    // 空变量应该也能渲染（只是不显示任何内容或保留占位符）
    let result = renderer.render_watermark_with_logo_bytes(
        &mut test_image,
        &template,
        &variables,
        None,
    );

    assert!(result.is_ok());
}

/// 测试渲染结果可编码
#[test]
fn test_render_output_encodable() {
    let mut test_image = create_test_image(800, 600);

    let mut variables = HashMap::new();
    variables.insert("Author".to_string(), "Test".to_string());

    let template = create_test_template();
    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");

    renderer.render_watermark_with_logo_bytes(
        &mut test_image,
        &template,
        &variables,
        None,
    ).expect("渲染失败");

    // 验证能编码为 JPEG
    let jpeg_result = litemark_core::image_io::encode_image(&test_image, ImageFormat::Jpeg);
    assert!(jpeg_result.is_ok(), "应能编码为 JPEG");
    assert!(!jpeg_result.unwrap().is_empty());

    // 验证能编码为 PNG
    let png_result = litemark_core::image_io::encode_image(&test_image, ImageFormat::Png);
    assert!(png_result.is_ok(), "应能编码为 PNG");
}

/// 测试多次渲染（验证渲染器可重用）
#[test]
fn test_renderer_reusable() {
    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");

    for i in 0..3 {
        let mut test_image = create_test_image(800, 600);
        let mut variables = HashMap::new();
        variables.insert("Author".to_string(), format!("Test {}", i));

        let template = create_test_template();

        let result = renderer.render_watermark_with_logo_bytes(
            &mut test_image,
            &template,
            &variables,
            None,
        );

        assert!(result.is_ok(), "第 {} 次渲染应成功", i);
    }
}

/// 测试不同模板渲染
#[test]
fn test_render_different_templates() {
    let mut test_image = create_test_image(800, 600);

    let mut variables = HashMap::new();
    variables.insert("Author".to_string(), "Test".to_string());

    // 极简模板
    let minimal_template = Template {
        name: "Minimal".to_string(),
        anchor: Anchor::BottomLeft,
        padding: 10,
        items: vec![TemplateItem {
            item_type: ItemType::Text,
            value: "{Author}".to_string(),
            font_size: 14,
            font_size_ratio: 0.3,
            weight: Some(FontWeight::Normal),
            color: Some("#000000".to_string()),
        }],
        background: None,
        frame_height_ratio: 0.06,
        logo_size_ratio: 0.0,
        primary_font_ratio: 0.3,
        secondary_font_ratio: 0.2,
        padding_ratio: 0.15,
    };

    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");

    let result = renderer.render_watermark_with_logo_bytes(
        &mut test_image,
        &minimal_template,
        &variables,
        None,
    );

    assert!(result.is_ok());
}

/// 测试特殊字符渲染
#[test]
fn test_render_unicode_text() {
    let mut test_image = create_test_image(800, 600);

    let mut variables = HashMap::new();
    variables.insert("Author".to_string(), "摄影师 📷".to_string());

    let template = create_test_template();
    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");

    let result = renderer.render_watermark_with_logo_bytes(
        &mut test_image,
        &template,
        &variables,
        None,
    );

    assert!(result.is_ok());
}

/// 辅助函数：创建测试图像
fn create_test_image(width: u32, height: u32) -> DynamicImage {
    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let r = ((x as f32 / width as f32) * 255.0) as u8;
        let g = ((y as f32 / height as f32) * 255.0) as u8;
        Rgb([r, g, 128])
    });
    DynamicImage::ImageRgb8(img)
}

/// 辅助函数：创建测试模板
fn create_test_template() -> Template {
    Template {
        name: "Test".to_string(),
        anchor: Anchor::BottomLeft,
        padding: 20,
        items: vec![
            TemplateItem {
                item_type: ItemType::Text,
                value: "{Author}".to_string(),
                font_size: 20,
                font_size_ratio: 0.25,
                weight: Some(FontWeight::Bold),
                color: Some("#000000".to_string()),
            },
            TemplateItem {
                item_type: ItemType::Text,
                value: "{Camera} • {Aperture} • ISO {ISO}".to_string(),
                font_size: 14,
                font_size_ratio: 0.18,
                weight: Some(FontWeight::Normal),
                color: Some("#666666".to_string()),
            },
        ],
        background: None,
        frame_height_ratio: 0.10,
        logo_size_ratio: 0.35,
        primary_font_ratio: 0.25,
        secondary_font_ratio: 0.18,
        padding_ratio: 0.12,
    }
}

use image::ImageBuffer;
