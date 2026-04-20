//! 处理管道集成测试
//!
//! 测试完整处理流程：图像解码 → EXIF 提取 → 模板渲染 → 图像编码

use image::{DynamicImage, ImageFormat, Rgb, RgbImage};
use litemark_core::exif::ExifData;
use litemark_core::layout::{RenderMode, Anchor, FontWeight, ItemType, Template, TemplateItem};
use litemark_core::renderer::WatermarkRenderer;

/// 测试标准处理流程
#[test]
fn test_standard_pipeline() {
    // 1. 创建测试图像
    let mut image = create_gradient_image(800, 600);

    // 2. 准备 EXIF 数据（模拟）
    let mut exif_data = ExifData::new();
    exif_data.iso = Some(400);
    exif_data.aperture = Some(2.8);
    exif_data.shutter_speed = Some("1/200".to_string());
    exif_data.focal_length = Some(85.0);
    exif_data.camera_model = Some("Sony A7M4".to_string());
    exif_data.lens_model = Some("FE 85mm F1.8".to_string());
    exif_data.author = Some("Integration Test".to_string());

    let variables = exif_data.to_variables();

    // 3. 创建模板
    let template = create_classic_template();

    // 4. 渲染水印
    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");
    let render_result = renderer.render_watermark_with_logo_bytes(
        &mut image,
        &template,
        &variables,
        None,
    );
    assert!(render_result.is_ok(), "渲染应成功");

    // 5. 编码输出
    let encode_result = litemark_core::image_io::encode_image(&image, ImageFormat::Jpeg);
    assert!(encode_result.is_ok(), "编码应成功");

    let jpeg_data = encode_result.unwrap();
    assert!(!jpeg_data.is_empty());
}

/// 测试不同尺寸图像的完整处理流程
#[test]
fn test_pipeline_various_sizes() {
    let sizes = vec![
        (400, 300),
        (800, 600),
        (1920, 1080),
        (1024, 1024),
    ];

    for (width, height) in sizes {
        let mut image = create_gradient_image(width, height);

        let mut exif_data = ExifData::new();
        exif_data.author = Some("Test".to_string());
        exif_data.iso = Some(100);

        let variables = exif_data.to_variables();
        let template = create_simple_template();
        let renderer = WatermarkRenderer::new().expect("创建渲染器失败");

        let result = renderer.render_watermark_with_logo_bytes(
            &mut image,
            &template,
            &variables,
            None,
        );

        assert!(
            result.is_ok(),
            "尺寸 {}x{} 的处理应成功",
            width,
            height
        );
    }
}

/// 测试缺失 EXIF 数据的处理流程
#[test]
fn test_pipeline_missing_exif() {
    let mut image = create_gradient_image(800, 600);

    // 无 EXIF 数据
    let exif_data = ExifData::new();
    let variables = exif_data.to_variables();

    let template = create_simple_template();
    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");

    let result = renderer.render_watermark_with_logo_bytes(
        &mut image,
        &template,
        &variables,
        None,
    );

    assert!(result.is_ok(), "无 EXIF 数据也应能处理");
}

/// 测试部分 EXIF 数据的处理流程
#[test]
fn test_pipeline_partial_exif() {
    let mut image = create_gradient_image(800, 600);

    let mut exif_data = ExifData::new();
    exif_data.iso = Some(400);
    exif_data.author = Some("Partial Test".to_string());
    // 其他字段缺失

    let variables = exif_data.to_variables();
    let template = create_classic_template();
    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");

    let result = renderer.render_watermark_with_logo_bytes(
        &mut image,
        &template,
        &variables,
        None,
    );

    assert!(result.is_ok(), "部分 EXIF 数据也应能处理");
}

/// 测试不同模板的处理流程
#[test]
fn test_pipeline_different_templates() {
    let templates = vec![
        ("simple", create_simple_template()),
        ("classic", create_classic_template()),
        ("minimal", create_minimal_template()),
    ];

    for (name, template) in templates {
        let mut image = create_gradient_image(800, 600);

        let mut exif_data = ExifData::new();
        exif_data.author = Some("Template Test".to_string());
        exif_data.camera_model = Some("Test Camera".to_string());
        exif_data.iso = Some(200);

        let variables = exif_data.to_variables();
        let renderer = WatermarkRenderer::new().expect("创建渲染器失败");

        let result = renderer.render_watermark_with_logo_bytes(
            &mut image,
            &template,
            &variables,
            None,
        );

        assert!(result.is_ok(), "模板 '{}' 的处理应成功", name);
    }
}

/// 测试处理流程后图像尺寸变化
#[test]
fn test_pipeline_image_size_change() {
    let mut image = create_gradient_image(1920, 1080);
    let original_height = image.height();
    let original_width = image.width();

    let mut exif_data = ExifData::new();
    exif_data.author = Some("Size Test".to_string());

    let variables = exif_data.to_variables();
    let template = create_classic_template();
    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");

    renderer.render_watermark_with_logo_bytes(
        &mut image,
        &template,
        &variables,
        None,
    ).expect("渲染失败");

    // 宽度应保持不变
    assert_eq!(image.width(), original_width, "宽度应保持不变");
    // 高度应增加（添加了水印边框）
    assert!(
        image.height() > original_height,
        "高度应增加"
    );
}

/// 测试 Unicode 文本的处理流程
#[test]
fn test_pipeline_unicode_text() {
    let mut image = create_gradient_image(800, 600);

    let mut exif_data = ExifData::new();
    exif_data.author = Some("摄影师 📷 测试".to_string());
    exif_data.camera_model = Some("佳能 EOS R5".to_string());
    exif_data.lens_model = Some("RF 镜头".to_string());

    let variables = exif_data.to_variables();
    let template = create_classic_template();
    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");

    let result = renderer.render_watermark_with_logo_bytes(
        &mut image,
        &template,
        &variables,
        None,
    );

    assert!(result.is_ok(), "Unicode 文本处理应成功");

    // 验证可编码
    let encoded = litemark_core::image_io::encode_image(&image, ImageFormat::Jpeg);
    assert!(encoded.is_ok());
}

/// 测试特殊字符的处理流程
#[test]
fn test_pipeline_special_characters() {
    let mut image = create_gradient_image(800, 600);

    let mut exif_data = ExifData::new();
    exif_data.author = Some("Test <>&\"'".to_string());
    exif_data.camera_model = Some("Model™".to_string());

    let variables = exif_data.to_variables();
    let template = create_simple_template();
    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");

    let result = renderer.render_watermark_with_logo_bytes(
        &mut image,
        &template,
        &variables,
        None,
    );

    assert!(result.is_ok(), "特殊字符处理应成功");
}

/// 测试长文本的处理流程
#[test]
fn test_pipeline_long_text() {
    let mut image = create_gradient_image(800, 600);

    let mut exif_data = ExifData::new();
    exif_data.author = Some("Very Long Author Name That Might Cause Layout Issues If Not Handled Properly".to_string());

    let variables = exif_data.to_variables();
    let template = create_simple_template();
    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");

    let result = renderer.render_watermark_with_logo_bytes(
        &mut image,
        &template,
        &variables,
        None,
    );

    assert!(result.is_ok(), "长文本处理应成功");
}

/// 辅助函数：创建渐变测试图像
fn create_gradient_image(width: u32, height: u32) -> DynamicImage {
    let img = RgbImage::from_fn(width, height, |x, y| {
        let r = ((x as f32 / width as f32) * 255.0) as u8;
        let g = ((y as f32 / height as f32) * 255.0) as u8;
        Rgb([r, g, 128])
    });
    DynamicImage::ImageRgb8(img)
}

/// 辅助函数：创建简单模板
fn create_simple_template() -> Template {
    Template {
        name: "Simple".to_string(),
        anchor: Anchor::BottomLeft,
        padding: 20,
        items: vec![TemplateItem {
            item_type: ItemType::Text,
            value: "{Author}".to_string(),
            font_size: 16,
            font_size_ratio: 0.25,
            weight: Some(FontWeight::Normal),
            color: Some("#000000".to_string()),
        }],
        background: None,
        frame_height_ratio: 0.08,
        logo_size_ratio: 0.0,
        primary_font_ratio: 0.25,
        secondary_font_ratio: 0.18,
        padding_ratio: 0.1,
        render_mode: RenderMode::BottomFrame,
    }
}

/// 辅助函数：创建经典模板
fn create_classic_template() -> Template {
    Template {
        name: "Classic".to_string(),
        anchor: Anchor::BottomLeft,
        padding: 20,
        items: vec![
            TemplateItem {
                item_type: ItemType::Text,
                value: "{Author}".to_string(),
                font_size: 20,
                font_size_ratio: 0.22,
                weight: Some(FontWeight::Bold),
                color: Some("#000000".to_string()),
            },
            TemplateItem {
                item_type: ItemType::Text,
                value: "{Camera} • {Lens}".to_string(),
                font_size: 14,
                font_size_ratio: 0.16,
                weight: Some(FontWeight::Normal),
                color: Some("#333333".to_string()),
            },
            TemplateItem {
                item_type: ItemType::Text,
                value: "{Aperture} | ISO {ISO} | {Shutter}".to_string(),
                font_size: 14,
                font_size_ratio: 0.16,
                weight: Some(FontWeight::Normal),
                color: Some("#666666".to_string()),
            },
        ],
        background: None,
        frame_height_ratio: 0.12,
        logo_size_ratio: 0.0,
        primary_font_ratio: 0.22,
        secondary_font_ratio: 0.16,
        padding_ratio: 0.1,
        render_mode: RenderMode::BottomFrame,
    }
}

/// 辅助函数：创建极简模板
fn create_minimal_template() -> Template {
    Template {
        name: "Minimal".to_string(),
        anchor: Anchor::BottomLeft,
        padding: 10,
        items: vec![TemplateItem {
            item_type: ItemType::Text,
            value: "{Author} • {ISO}".to_string(),
            font_size: 12,
            font_size_ratio: 0.35,
            weight: Some(FontWeight::Normal),
            color: Some("#000000".to_string()),
        }],
        background: None,
        frame_height_ratio: 0.05,
        logo_size_ratio: 0.0,
        primary_font_ratio: 0.35,
        secondary_font_ratio: 0.25,
        padding_ratio: 0.2,
        render_mode: RenderMode::BottomFrame,
    }
}
