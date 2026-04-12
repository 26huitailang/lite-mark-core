//! HTML 视觉报告生成工具
//!
//! 生成包含输入/输出对比、参数展示的 HTML 报告

use anyhow::{Context, Result};
use chrono::Local;
use image::{DynamicImage, ImageFormat, Rgb, RgbImage};
use litemark_core::exif::ExifData;
use litemark_core::layout::{self, Anchor, FontWeight, ItemType, Template, TemplateItem};
use litemark_core::renderer::WatermarkRenderer;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tera::{Context as TeraContext, Tera};

/// 测试用例结果
#[derive(Serialize)]
struct TestCaseResult {
    id: String,
    name: String,
    template_name: String,
    image_size: String,
    input_path: String,
    output_path: String,
    metadata: TestCaseMetadata,
    success: bool,
    duration_ms: u64,
}

/// 测试用例元数据
#[derive(Serialize)]
struct TestCaseMetadata {
    template_json: String,
    exif_data: HashMap<String, String>,
    variables: HashMap<String, String>,
}

/// 报告摘要
#[derive(Serialize)]
struct ReportSummary {
    generated_at: String,
    total_cases: usize,
    success_count: usize,
    failure_count: usize,
    duration_seconds: f64,
}

fn main() -> Result<()> {
    println!("📊 LiteMark 视觉报告生成工具\n");

    let start_time = std::time::Instant::now();

    // 创建输出目录
    let report_dir = create_report_directory()?;
    println!("报告目录: {}\n", report_dir.display());

    // 加载模板
    let templates = load_test_templates();
    println!("加载了 {} 个测试模板", templates.len());

    // 生成测试用例
    let mut results = Vec::new();

    for (template_idx, (template_name, template)) in templates.iter().enumerate() {
        println!("\n处理模板 {}: {}", template_idx + 1, template_name);

        // 生成不同尺寸的测试用例
        let sizes = vec![(800, 600), (1920, 1080), (1024, 1024)];

        for (idx, (width, height)) in sizes.iter().enumerate() {
            let case_id = format!("TC{:03}", template_idx * 10 + idx + 1);
            let case_name = format!("{}_{}x{}", template_name, width, height);

            print!("  生成 {}... ", case_name);

            let case_start = std::time::Instant::now();

            match generate_test_case(
                &case_id,
                &case_name,
                template_name,
                template,
                *width,
                *height,
                &report_dir,
            ) {
                Ok(result) => {
                    let duration = case_start.elapsed().as_millis() as u64;
                    let result = TestCaseResult {
                        id: case_id,
                        name: case_name,
                        template_name: template_name.clone(),
                        image_size: format!("{}x{}", width, height),
                        input_path: result.0,
                        output_path: result.1,
                        metadata: result.2,
                        success: true,
                        duration_ms: duration,
                    };
                    results.push(result);
                    println!("✅ ({}ms)", duration);
                }
                Err(e) => {
                    let duration = case_start.elapsed().as_millis() as u64;
                    println!("❌ {} ({}ms)", e, duration);
                    results.push(TestCaseResult {
                        id: case_id,
                        name: case_name,
                        template_name: template_name.clone(),
                        image_size: format!("{}x{}", width, height),
                        input_path: String::new(),
                        output_path: String::new(),
                        metadata: TestCaseMetadata {
                            template_json: String::new(),
                            exif_data: HashMap::new(),
                            variables: HashMap::new(),
                        },
                        success: false,
                        duration_ms: duration,
                    });
                }
            }
        }
    }

    // 生成 HTML 报告
    println!("\n生成 HTML 报告...");
    generate_html_report(&results, &report_dir)?;

    // 生成摘要 JSON
    let total_duration = start_time.elapsed().as_secs_f64();
    let summary = ReportSummary {
        generated_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        total_cases: results.len(),
        success_count: results.iter().filter(|r| r.success).count(),
        failure_count: results.iter().filter(|r| !r.success).count(),
        duration_seconds: total_duration,
    };

    let summary_json = serde_json::to_string_pretty(&summary)?;
    fs::write(report_dir.join("summary.json"), summary_json)?;

    println!("\n✅ 报告生成完成！");
    println!("位置: {}", report_dir.display());
    println!("打开: file://{}/index.html", report_dir.canonicalize()?.display());
    println!(
        "\n摘要: {} 成功, {} 失败, 耗时 {:.2}s",
        summary.success_count, summary.failure_count, summary.duration_seconds
    );

    Ok(())
}

/// 创建报告目录
fn create_report_directory() -> Result<PathBuf> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let report_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("target")
        .join("test-reports")
        .join(timestamp.to_string());

    fs::create_dir_all(&report_dir)?;
    fs::create_dir_all(report_dir.join("test-cases"))?;

    // 创建 latest 符号链接
    let latest_link = report_dir.parent().unwrap().join("latest");
    let _ = fs::remove_file(&latest_link);
    #[cfg(unix)]
    std::os::unix::fs::symlink(&report_dir, latest_link)?;

    Ok(report_dir)
}

/// 加载测试模板
fn load_test_templates() -> Vec<(String, Template)> {
    let mut templates = Vec::new();

    // 添加内置模板
    let builtin = layout::create_builtin_templates();
    for template in builtin {
        templates.push((template.name.clone(), template));
    }

    // 如果没有内置模板，添加默认测试模板
    if templates.is_empty() {
        templates.push(("Classic".to_string(), create_classic_template()));
        templates.push(("Compact".to_string(), create_compact_template()));
    }

    templates
}

/// 生成单个测试用例
fn generate_test_case(
    case_id: &str,
    case_name: &str,
    _template_name: &str,
    template: &Template,
    width: u32,
    height: u32,
    report_dir: &Path,
) -> Result<(String, String, TestCaseMetadata)> {
    let case_dir = report_dir.join("test-cases").join(case_id);
    fs::create_dir_all(&case_dir)?;

    // 创建测试图像
    let mut test_image = create_test_image(width, height);

    // 保存输入图像
    let input_path = case_dir.join("input.jpg");
    let input_data = litemark_core::image_io::encode_image(&test_image, ImageFormat::Jpeg)
        .map_err(|e| anyhow::anyhow!("编码输入图像失败: {}", e))?;
    fs::write(&input_path, input_data)?;

    // 准备 EXIF 数据
    let mut exif_data = ExifData::new();
    exif_data.iso = Some(400);
    exif_data.aperture = Some(2.8);
    exif_data.shutter_speed = Some("1/200".to_string());
    exif_data.focal_length = Some(85.0);
    exif_data.camera_model = Some("Sony A7M4".to_string());
    exif_data.lens_model = Some("FE 85mm F1.8".to_string());
    exif_data.date_time = Some("2024:01:15 14:30:00".to_string());
    exif_data.author = Some("Test Photographer".to_string());

    let variables = exif_data.to_variables();

    // 渲染水印
    let renderer = WatermarkRenderer::new()
        .map_err(|e| anyhow::anyhow!("创建渲染器失败: {}", e))?;
    renderer.render_watermark_with_logo_bytes(
        &mut test_image,
        template,
        &variables,
        None,
    ).map_err(|e| anyhow::anyhow!("渲染水印失败: {}", e))?;

    // 保存输出图像
    let output_path = case_dir.join("output.jpg");
    let output_data = litemark_core::image_io::encode_image(&test_image, ImageFormat::Jpeg)
        .map_err(|e| anyhow::anyhow!("编码输出图像失败: {}", e))?;
    fs::write(&output_path, output_data)?;

    // 准备元数据
    let template_json = serde_json::to_string_pretty(template).unwrap_or_default();

    let metadata = TestCaseMetadata {
        template_json,
        exif_data: [
            ("ISO".to_string(), "400".to_string()),
            ("Aperture".to_string(), "f/2.8".to_string()),
            ("Shutter".to_string(), "1/200".to_string()),
            ("Focal".to_string(), "85mm".to_string()),
            ("Camera".to_string(), "Sony A7M4".to_string()),
            ("Lens".to_string(), "FE 85mm F1.8".to_string()),
            ("DateTime".to_string(), "2024:01:15 14:30:00".to_string()),
            ("Author".to_string(), "Test Photographer".to_string()),
        ]
        .into(),
        variables: variables.clone(),
    };

    Ok((
        format!("test-cases/{}/input.jpg", case_id),
        format!("test-cases/{}/output.jpg", case_id),
        metadata,
    ))
}

/// 生成 HTML 报告
fn generate_html_report(results: &[TestCaseResult], report_dir: &Path) -> Result<()> {
    let mut tera = Tera::default();

    // 加载并注册模板
    let template_content = load_template()?;
    tera.add_raw_template("report", &template_content)
        .context("加载报告模板失败")?;

    let mut context = TeraContext::new();
    context.insert("results", results);

    let success_count = results.iter().filter(|r| r.success).count();
    let failure_count = results.iter().filter(|r| !r.success).count();

    context.insert("success_count", &success_count);
    context.insert("failure_count", &failure_count);
    context.insert("total_count", &results.len());
    context.insert("generated_at", &Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

    let html = tera.render("report", &context).context("渲染模板失败")?;

    fs::write(report_dir.join("index.html"), html)?;

    // 复制 CSS
    let styles_src = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets/styles.css");
    fs::copy(&styles_src, report_dir.join("styles.css"))
        .context("复制样式文件失败")?;

    Ok(())
}

/// 加载模板内容
fn load_template() -> Result<String> {
    let template_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets/report_template.html");
    fs::read_to_string(&template_path)
        .context("加载报告模板失败")
}

/// 创建测试图像
fn create_test_image(width: u32, height: u32) -> DynamicImage {
    let img = image::ImageBuffer::from_fn(width, height, |x, y| {
        let r = ((x as f32 / width as f32) * 255.0) as u8;
        let g = ((y as f32 / height as f32) * 255.0) as u8;
        Rgb([r, g, 128])
    });
    DynamicImage::ImageRgb8(img)
}

/// 创建经典模板
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
        logo_size_ratio: 0.35,
        primary_font_ratio: 0.22,
        secondary_font_ratio: 0.16,
        padding_ratio: 0.12,
    }
}

/// 创建紧凑模板
fn create_compact_template() -> Template {
    Template {
        name: "Compact".to_string(),
        anchor: Anchor::BottomLeft,
        padding: 10,
        items: vec![TemplateItem {
            item_type: ItemType::Text,
            value: "{Author} • {Aperture} • ISO {ISO}".to_string(),
            font_size: 14,
            font_size_ratio: 0.35,
            weight: Some(FontWeight::Normal),
            color: Some("#000000".to_string()),
        }],
        background: None,
        frame_height_ratio: 0.06,
        logo_size_ratio: 0.0,
        primary_font_ratio: 0.35,
        secondary_font_ratio: 0.25,
        padding_ratio: 0.20,
    }
}


