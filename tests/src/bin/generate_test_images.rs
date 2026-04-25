//! 测试图片生成工具
//!
//! 生成各种尺寸、格式、EXIF 组合的测试图片

use anyhow::Result;
use image::{DynamicImage, ImageBuffer, ImageFormat, Rgb};
use std::fs;
use std::path::{Path, PathBuf};

/// 测试图片配置
struct TestImageConfig {
    name: String,
    width: u32,
    height: u32,
    format: ImageFormat,
    pattern: ImagePattern,
}

/// 图片生成模式
#[derive(Clone)]
enum ImagePattern {
    /// 纯色
    Solid([u8; 3]),
    /// 水平渐变
    HorizontalGradient([u8; 3], [u8; 3]),
    /// 垂直渐变
    VerticalGradient([u8; 3], [u8; 3]),
    /// 对角渐变
    DiagonalGradient([u8; 3], [u8; 3]),
    /// 棋盘格
    Checkerboard {
        color1: [u8; 3],
        color2: [u8; 3],
        size: u32,
    },
    /// 条纹
    Stripes {
        color1: [u8; 3],
        color2: [u8; 3],
        width: u32,
    },
}

impl ImagePattern {
    fn generate(&self, width: u32, height: u32) -> DynamicImage {
        match self {
            ImagePattern::Solid(color) => {
                let img = ImageBuffer::from_fn(width, height, |_, _| Rgb(*color));
                DynamicImage::ImageRgb8(img)
            }
            ImagePattern::HorizontalGradient(c1, c2) => {
                let img = ImageBuffer::from_fn(width, height, |x, _| {
                    let t = x as f32 / width as f32;
                    Rgb([
                        (c1[0] as f32 * (1.0 - t) + c2[0] as f32 * t) as u8,
                        (c1[1] as f32 * (1.0 - t) + c2[1] as f32 * t) as u8,
                        (c1[2] as f32 * (1.0 - t) + c2[2] as f32 * t) as u8,
                    ])
                });
                DynamicImage::ImageRgb8(img)
            }
            ImagePattern::VerticalGradient(c1, c2) => {
                let img = ImageBuffer::from_fn(width, height, |_, y| {
                    let t = y as f32 / height as f32;
                    Rgb([
                        (c1[0] as f32 * (1.0 - t) + c2[0] as f32 * t) as u8,
                        (c1[1] as f32 * (1.0 - t) + c2[1] as f32 * t) as u8,
                        (c1[2] as f32 * (1.0 - t) + c2[2] as f32 * t) as u8,
                    ])
                });
                DynamicImage::ImageRgb8(img)
            }
            ImagePattern::DiagonalGradient(c1, c2) => {
                let img = ImageBuffer::from_fn(width, height, |x, y| {
                    let t = ((x as f32 / width as f32) + (y as f32 / height as f32)) / 2.0;
                    Rgb([
                        (c1[0] as f32 * (1.0 - t) + c2[0] as f32 * t) as u8,
                        (c1[1] as f32 * (1.0 - t) + c2[1] as f32 * t) as u8,
                        (c1[2] as f32 * (1.0 - t) + c2[2] as f32 * t) as u8,
                    ])
                });
                DynamicImage::ImageRgb8(img)
            }
            ImagePattern::Checkerboard {
                color1,
                color2,
                size,
            } => {
                let img = ImageBuffer::from_fn(width, height, |x, y| {
                    let cx = x / size;
                    let cy = y / size;
                    if (cx + cy) % 2 == 0 {
                        Rgb(*color1)
                    } else {
                        Rgb(*color2)
                    }
                });
                DynamicImage::ImageRgb8(img)
            }
            ImagePattern::Stripes {
                color1,
                color2,
                width: stripe_width,
            } => {
                let img = ImageBuffer::from_fn(width, height, |x, _| {
                    if (x / stripe_width) % 2 == 0 {
                        Rgb(*color1)
                    } else {
                        Rgb(*color2)
                    }
                });
                DynamicImage::ImageRgb8(img)
            }
        }
    }
}

/// 获取测试图片尺寸矩阵
fn get_test_dimensions() -> Vec<(u32, u32, &'static str)> {
    vec![
        (100, 100, "tiny"),      // 极小
        (400, 300, "small"),     // 小图
        (600, 800, "portrait"),  // 竖屏
        (800, 600, "landscape"), // 横屏
        (1024, 1024, "square"),  // 正方形
        (1920, 1080, "fhd"),     // 全高清
        (3840, 2160, "4k"),      // 4K
        (7680, 4320, "8k"),      // 8K
        (10000, 10000, "huge"),  // 极大
    ]
}

/// 生成基础测试图片
fn generate_basic_images(output_dir: &Path) -> Result<Vec<TestImageConfig>> {
    fs::create_dir_all(output_dir)?;

    let dimensions = get_test_dimensions();
    let patterns = vec![
        (
            "gradient",
            ImagePattern::DiagonalGradient([255, 100, 50], [50, 100, 255]),
        ),
        (
            "checker",
            ImagePattern::Checkerboard {
                color1: [200, 200, 200],
                color2: [50, 50, 50],
                size: 50,
            },
        ),
        (
            "stripes",
            ImagePattern::Stripes {
                color1: [255, 0, 0],
                color2: [0, 0, 255],
                width: 40,
            },
        ),
    ];

    let mut configs = Vec::new();

    for (width, height, size_name) in &dimensions {
        for (pattern_name, pattern) in &patterns {
            // JPEG
            configs.push(TestImageConfig {
                name: format!("{}_{}x{}_{}", size_name, width, height, pattern_name),
                width: *width,
                height: *height,
                format: ImageFormat::Jpeg,
                pattern: pattern.clone(),
            });
        }
    }

    Ok(configs)
}

/// 保存测试图片
fn save_test_image(config: &TestImageConfig, output_dir: &Path) -> Result<PathBuf> {
    let img = config.pattern.generate(config.width, config.height);

    let ext = match config.format {
        ImageFormat::Jpeg => "jpg",
        ImageFormat::Png => "png",
        ImageFormat::WebP => "webp",
        _ => "bin",
    };

    let filename = format!("{}.{})", config.name, ext);
    let path = output_dir.join(&filename);

    let mut file = fs::File::create(&path)?;

    match config.format {
        ImageFormat::Jpeg => {
            img.write_to(&mut file, ImageFormat::Jpeg)?;
        }
        ImageFormat::Png => {
            img.write_to(&mut file, ImageFormat::Png)?;
        }
        ImageFormat::WebP => {
            img.write_to(&mut file, ImageFormat::WebP)?;
        }
        _ => anyhow::bail!("Unsupported format"),
    }

    println!("Generated: {}", path.display());
    Ok(path)
}

fn main() -> Result<()> {
    println!("🎨 LiteMark 测试图片生成工具\n");

    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("images");

    // 生成基础测试图片
    println!("生成基础测试图片...");
    let jpeg_dir = base_dir.join("jpeg");
    let configs = generate_basic_images(&jpeg_dir)?;

    for config in &configs {
        save_test_image(config, &jpeg_dir)?;
    }

    // 生成 PNG 版本
    println!("\n生成 PNG 测试图片...");
    let png_dir = base_dir.join("png");
    fs::create_dir_all(&png_dir)?;

    for config in &configs {
        let png_config = TestImageConfig {
            name: config.name.clone(),
            width: config.width,
            height: config.height,
            format: ImageFormat::Png,
            pattern: config.pattern.clone(),
        };
        save_test_image(&png_config, &png_dir)?;
    }

    // 生成边界情况图片
    println!("\n生成边界情况测试图片...");
    let edge_dir = base_dir.join("edge_cases");
    fs::create_dir_all(&edge_dir)?;

    // 纯黑图片
    let black = TestImageConfig {
        name: "solid_black".to_string(),
        width: 800,
        height: 600,
        format: ImageFormat::Jpeg,
        pattern: ImagePattern::Solid([0, 0, 0]),
    };
    save_test_image(&black, &edge_dir)?;

    // 纯白图片
    let white = TestImageConfig {
        name: "solid_white".to_string(),
        width: 800,
        height: 600,
        format: ImageFormat::Jpeg,
        pattern: ImagePattern::Solid([255, 255, 255]),
    };
    save_test_image(&white, &edge_dir)?;

    // 极端宽高比
    let panoramic = TestImageConfig {
        name: "panoramic_3_1".to_string(),
        width: 1920,
        height: 640,
        format: ImageFormat::Jpeg,
        pattern: ImagePattern::HorizontalGradient([255, 0, 0], [0, 0, 255]),
    };
    save_test_image(&panoramic, &edge_dir)?;

    let tall = TestImageConfig {
        name: "tall_1_3".to_string(),
        width: 400,
        height: 1200,
        format: ImageFormat::Jpeg,
        pattern: ImagePattern::VerticalGradient([0, 255, 0], [255, 0, 255]),
    };
    save_test_image(&tall, &edge_dir)?;

    println!("\n✅ 测试图片生成完成！");
    println!("位置: {}", base_dir.display());

    Ok(())
}
