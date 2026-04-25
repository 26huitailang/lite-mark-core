//! 视觉回归测试
//!
//! 验证渲染输出与参考图像的像素一致性。
//! 运行 `UPDATE_REFS=1 cargo test --test integration visual` 可刷新参考图。

use image::{DynamicImage, GenericImageView, ImageFormat, Rgb, RgbImage};
use litemark_core::exif::ExifData;
use litemark_core::layout::{self, Template};
use litemark_core::renderer::WatermarkRenderer;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// 容差阈值：单个颜色通道允许的最大差异
const PIXEL_TOLERANCE: u8 = 2;
/// 整体容差：允许差异像素占总像素的最大比例（考虑抗锯齿差异）
const DIFF_RATIO_TOLERANCE: f64 = 0.005; // 0.5%

/// 测试图像固定尺寸
const TEST_WIDTH: u32 = 1920;
const TEST_HEIGHT: u32 = 1080;

/// 创建确定性测试图像（固定种子，确保每次生成完全相同）
fn create_deterministic_test_image(width: u32, height: u32) -> DynamicImage {
    let img = RgbImage::from_fn(width, height, |x, y| {
        // 使用确定性公式，不依赖随机数
        let r = ((x * 7 + y * 13) % 256) as u8;
        let g = ((x * 11 + y * 17) % 256) as u8;
        let b = ((x * 13 + y * 7 + 128) % 256) as u8;
        Rgb([r, g, b])
    });
    DynamicImage::ImageRgb8(img)
}

/// 加载固定 EXIF 变量
fn get_test_variables() -> HashMap<String, String> {
    let mut exif = ExifData::new();
    exif.author = Some("Visual Reg Test".to_string());
    exif.camera_model = Some("Sony A7M4".to_string());
    exif.lens_model = Some("FE 85mm F1.8".to_string());
    exif.iso = Some(400);
    exif.aperture = Some(2.8);
    exif.shutter_speed = Some("1/200".to_string());
    exif.focal_length = Some(85.0);
    exif.date_time = Some("2024-06-15 14:30:00".to_string());
    exif.to_variables()
}

/// 获取参考图路径
fn get_reference_path(template_name: &str) -> std::path::PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    Path::new(manifest_dir)
        .join("fixtures")
        .join("expected")
        .join(format!("{}_1920x1080.png", template_name))
}

/// 为指定模板生成渲染图
fn render_template(template: &Template, variables: &HashMap<String, String>) -> DynamicImage {
    let mut image = create_deterministic_test_image(TEST_WIDTH, TEST_HEIGHT);
    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");

    renderer
        .render_watermark_with_logo_bytes(&mut image, template, variables, None)
        .expect("渲染失败");

    image
}

/// 比较两张图像的像素差异
/// 返回 (差异像素数, 总像素数, 最大差异值)
fn compare_images(expected: &DynamicImage, actual: &DynamicImage) -> (usize, usize, u8) {
    assert_eq!(expected.dimensions(), actual.dimensions(), "图像尺寸不一致");

    let expected_rgba = expected.to_rgba8();
    let actual_rgba = actual.to_rgba8();

    let mut diff_count = 0usize;
    let mut max_diff = 0u8;
    let total = (expected.width() * expected.height()) as usize;

    for (exp_pix, act_pix) in expected_rgba.pixels().zip(actual_rgba.pixels()) {
        let diff = exp_pix
            .0
            .iter()
            .zip(act_pix.0.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .max()
            .unwrap_or(0);

        if diff > max_diff {
            max_diff = diff;
        }

        if diff > PIXEL_TOLERANCE {
            diff_count += 1;
        }
    }

    (diff_count, total, max_diff)
}

/// 保存参考图（用于手动更新）
fn save_reference(template_name: &str, image: &DynamicImage) {
    let path = get_reference_path(template_name);
    fs::create_dir_all(path.parent().unwrap()).expect("创建目录失败");

    let mut buf = Vec::new();
    image
        .write_to(&mut std::io::Cursor::new(&mut buf), ImageFormat::Png)
        .expect("编码 PNG 失败");
    fs::write(&path, buf).expect("写入参考图失败");
}

/// 运行单个模板的视觉回归测试
fn run_visual_regression(
    template_name: &str,
    template: &Template,
    variables: &HashMap<String, String>,
) {
    let ref_path = get_reference_path(template_name);

    // 生成实际输出
    let actual_image = render_template(template, variables);

    // 检查是否需要更新参考图
    if std::env::var("UPDATE_REFS").is_ok() {
        save_reference(template_name, &actual_image);
        println!("📝 已更新参考图: {}", ref_path.display());
        return;
    }

    // 检查参考图是否存在
    if !ref_path.exists() {
        panic!(
            "参考图不存在: {}\n请运行 `UPDATE_REFS=1 cargo test --test integration visual` 生成参考图",
            ref_path.display()
        );
    }

    // 加载参考图
    let ref_data = fs::read(&ref_path).expect("读取参考图失败");
    let expected_image = image::load_from_memory(&ref_data).expect("解码参考图失败");

    // 比较
    let (diff_count, total, max_diff) = compare_images(&expected_image, &actual_image);
    let diff_ratio = diff_count as f64 / total as f64;

    if diff_ratio > DIFF_RATIO_TOLERANCE {
        panic!(
            "视觉回归测试失败: {}\n  差异像素: {} / {} ({:.2}%)\n  最大差异: {}\n  容差: {} (像素), {:.2}% (比例)\n  请检查渲染变更，或运行 UPDATE_REFS=1 更新参考图",
            template_name,
            diff_count,
            total,
            diff_ratio * 100.0,
            max_diff,
            PIXEL_TOLERANCE,
            DIFF_RATIO_TOLERANCE * 100.0,
        );
    }

    println!(
        "✅ {}: 差异 {:.4}% (最大差异 {})",
        template_name,
        diff_ratio * 100.0,
        max_diff
    );
}

// =============================================================================
// 各模板回归测试
// =============================================================================

#[test]
fn test_visual_regression_classic() {
    let templates = layout::create_builtin_templates();
    let template = templates
        .iter()
        .find(|t| t.name == "Classic")
        .expect("应存在 Classic 模板");
    let variables = get_test_variables();
    run_visual_regression("classic", template, &variables);
}

#[test]
fn test_visual_regression_compact() {
    let templates = layout::create_builtin_templates();
    let template = templates
        .iter()
        .find(|t| t.name == "Compact")
        .expect("应存在 Compact 模板");
    let variables = get_test_variables();
    run_visual_regression("compact", template, &variables);
}

#[test]
fn test_visual_regression_professional() {
    let templates = layout::create_builtin_templates();
    let template = templates
        .iter()
        .find(|t| t.name == "Professional")
        .expect("应存在 Professional 模板");
    let variables = get_test_variables();
    run_visual_regression("professional", template, &variables);
}

#[test]
fn test_visual_regression_overlay() {
    let templates = layout::create_builtin_templates();
    let template = templates
        .iter()
        .find(|t| t.name == "Overlay")
        .expect("应存在 Overlay 模板");
    let variables = get_test_variables();
    run_visual_regression("overlay", template, &variables);
}

// =============================================================================
// 参考图生成说明
// =============================================================================
//
// 首次运行或需要更新参考图时，执行：
//   UPDATE_REFS=1 cargo test -p litemark-test-suite --test integration -- visual
//
// 参考图存储在: tests/fixtures/expected/<template>_1920x1080.png
