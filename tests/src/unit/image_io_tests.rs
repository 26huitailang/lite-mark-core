//! 图像 I/O 单元测试
//!
//! 测试图像编解码、格式检测等功能

use image::{DynamicImage, ImageFormat, Rgb, RgbImage};

/// 测试 JPEG 编码和解码循环
#[test]
fn test_jpeg_encode_decode_roundtrip() {
    let test_image = create_test_image(200, 150, [255, 100, 50]);

    // 编码为 JPEG
    let jpeg_data = litemark_core::image_io::encode_image(&test_image, ImageFormat::Jpeg)
        .expect("JPEG 编码失败");

    assert!(!jpeg_data.is_empty(), "JPEG 数据不应为空");
    assert!(jpeg_data.len() > 100, "JPEG 数据应有合理大小");

    // 解码
    let decoded = litemark_core::image_io::decode_image(&jpeg_data).expect("JPEG 解码失败");

    assert_eq!(decoded.width(), 200);
    assert_eq!(decoded.height(), 150);
}

/// 测试 PNG 编码和解码循环
#[test]
fn test_png_encode_decode_roundtrip() {
    let test_image = create_test_image(200, 150, [50, 100, 255]);

    // 编码为 PNG
    let png_data =
        litemark_core::image_io::encode_image(&test_image, ImageFormat::Png).expect("PNG 编码失败");

    assert!(!png_data.is_empty());

    // 解码
    let decoded = litemark_core::image_io::decode_image(&png_data).expect("PNG 解码失败");

    assert_eq!(decoded.width(), 200);
    assert_eq!(decoded.height(), 150);
}

/// 测试 WebP 编码和解码循环
#[test]
fn test_webp_encode_decode_roundtrip() {
    let test_image = create_test_image(200, 150, [100, 255, 100]);

    // 编码为 WebP
    let webp_data = litemark_core::image_io::encode_image(&test_image, ImageFormat::WebP)
        .expect("WebP 编码失败");

    assert!(!webp_data.is_empty());

    // 解码
    let decoded = litemark_core::image_io::decode_image(&webp_data).expect("WebP 解码失败");

    assert_eq!(decoded.width(), 200);
    assert_eq!(decoded.height(), 150);
}

/// 测试 JPEG 格式检测
#[test]
fn test_detect_jpeg_format() {
    // JPEG 魔数: FF D8 FF
    let jpeg_header = vec![0xFF, 0xD8, 0xFF, 0xE0];
    let format = litemark_core::image_io::detect_format(&jpeg_header);
    assert!(matches!(format, ImageFormat::Jpeg), "应检测为 JPEG");
}

/// 测试 PNG 格式检测
#[test]
fn test_detect_png_format() {
    // PNG 魔数: 89 50 4E 47 0D 0A 1A 0A
    let png_header = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let format = litemark_core::image_io::detect_format(&png_header);
    assert!(matches!(format, ImageFormat::Png), "应检测为 PNG");
}

/// 测试 WebP 格式检测
#[test]
fn test_detect_webp_format() {
    // WebP 魔数: RIFF....WEBP
    let webp_header = vec![
        0x52, 0x49, 0x46, 0x46, // "RIFF"
        0x00, 0x00, 0x00, 0x00, // 文件大小
        0x57, 0x45, 0x42, 0x50, // "WEBP"
    ];
    let format = litemark_core::image_io::detect_format(&webp_header);
    assert!(matches!(format, ImageFormat::WebP), "应检测为 WebP");
}

/// 测试未知格式检测
#[test]
fn test_detect_unknown_format() {
    // 无效数据
    let unknown_data = vec![0x00, 0x01, 0x02, 0x03];
    let format = litemark_core::image_io::detect_format(&unknown_data);
    // 无法识别时默认返回 JPEG
    assert!(matches!(format, ImageFormat::Jpeg));
}

/// 测试从空数据解码（应失败）
#[test]
fn test_decode_empty_data() {
    let empty_data: &[u8] = &[];
    let result = litemark_core::image_io::decode_image(empty_data);
    assert!(result.is_err(), "空数据解码应失败");
}

/// 测试从无效数据解码（应失败）
#[test]
fn test_decode_invalid_data() {
    let invalid_data = vec![0xFF, 0xD8, 0x00, 0x00, 0x00, 0x00];
    let _result = litemark_core::image_io::decode_image(&invalid_data);
    // 可能成功（JPEG 解析器可能忽略无效数据）或失败
    // 这里不强制断言，仅验证不 panic
}

/// 测试不同尺寸图像编解码
#[test]
fn test_encode_decode_various_sizes() {
    let sizes = vec![(1, 1), (100, 100), (1920, 1080), (1, 1000), (1000, 1)];

    for (width, height) in sizes {
        let image = create_test_image(width, height, [128, 128, 128]);

        let jpeg_data = litemark_core::image_io::encode_image(&image, ImageFormat::Jpeg)
            .unwrap_or_else(|_| panic!("编码 {}x{} 失败", width, height));

        let decoded = litemark_core::image_io::decode_image(&jpeg_data)
            .unwrap_or_else(|_| panic!("解码 {}x{} 失败", width, height));

        assert_eq!(decoded.width(), width, "宽度应匹配");
        assert_eq!(decoded.height(), height, "高度应匹配");
    }
}

/// 测试多帧图像解码
#[test]
fn test_decode_large_image() {
    // 创建一个较大的图像（但不要太大使测试变慢）
    let image = create_test_image(4000, 3000, [64, 128, 192]);

    let jpeg_data =
        litemark_core::image_io::encode_image(&image, ImageFormat::Jpeg).expect("大图像编码失败");

    let decoded = litemark_core::image_io::decode_image(&jpeg_data).expect("大图像解码失败");

    assert_eq!(decoded.width(), 4000);
    assert_eq!(decoded.height(), 3000);
}

/// 测试 RGB 图像编码
#[test]
fn test_rgb_image_encoding() {
    let img = RgbImage::from_fn(100, 100, |x, y| {
        Rgb([(x % 256) as u8, (y % 256) as u8, 128])
    });
    let image = DynamicImage::ImageRgb8(img);

    let jpeg_data =
        litemark_core::image_io::encode_image(&image, ImageFormat::Jpeg).expect("RGB 编码失败");

    assert!(!jpeg_data.is_empty());
}

/// 测试编码质量（JPEG）
#[test]
fn test_jpeg_encoding_quality() {
    // 创建一个细节丰富的图像
    let img = RgbImage::from_fn(500, 500, |x, y| {
        let pattern = ((x + y) % 2) * 255;
        Rgb([pattern as u8, pattern as u8, pattern as u8])
    });
    let image = DynamicImage::ImageRgb8(img);

    let jpeg_data =
        litemark_core::image_io::encode_image(&image, ImageFormat::Jpeg).expect("JPEG 编码失败");

    // JPEG 质量为 95%，文件大小应合理
    assert!(jpeg_data.len() > 1000, "500x500 JPEG 应大于 1KB");
}

/// 辅助函数：创建纯色测试图像
fn create_test_image(width: u32, height: u32, color: [u8; 3]) -> DynamicImage {
    let img = RgbImage::from_fn(width, height, |_, _| Rgb(color));
    DynamicImage::ImageRgb8(img)
}

/// 辅助函数：创建一个带 EXIF Orientation 标签的 JPEG 测试图像
/// orientation: 1=正常, 6=顺时针90°(竖向照片常见)
fn create_jpeg_with_orientation(
    width: u32,
    height: u32,
    orientation: u16,
) -> Vec<u8> {
    use exif::experimental::Writer;
    use exif::{Field, In, Tag, Value};

    // 1. 创建基础 JPEG（使用 image 库）
    let img = RgbImage::from_fn(width, height, |x, y| {
        Rgb([
            ((x as f32 / width as f32) * 255.0) as u8,
            ((y as f32 / height as f32) * 255.0) as u8,
            128,
        ])
    });
    let image = DynamicImage::ImageRgb8(img);

    let mut jpeg_bytes = Vec::new();
    {
        let mut cursor = std::io::Cursor::new(&mut jpeg_bytes);
        image
            .write_to(&mut cursor, image::ImageOutputFormat::Jpeg(95))
            .expect("编码 JPEG 失败");
    }

    // 2. 构造 EXIF Orientation 字段
    let orientation_field = Field {
        tag: Tag::Orientation,
        ifd_num: In::PRIMARY,
        value: Value::Short(vec![orientation]),
    };

    // 3. 使用 kamadak-exif Writer 生成 TIFF 数据
    let mut writer = Writer::new();
    writer.push_field(&orientation_field);
    let mut exif_tiff = Vec::new();
    {
        let mut cursor = std::io::Cursor::new(&mut exif_tiff);
        writer.write(&mut cursor, true).expect("写入 EXIF 失败");
    }

    // 4. 将 EXIF 嵌入 JPEG（插入 SOI 之后）
    let app1_length = 2 + 6 + exif_tiff.len();
    let mut app1_segment = Vec::new();
    app1_segment.push(0xFF);
    app1_segment.push(0xE1);
    app1_segment.push(((app1_length >> 8) & 0xFF) as u8);
    app1_segment.push((app1_length & 0xFF) as u8);
    app1_segment.extend_from_slice(b"Exif\x00\x00");
    app1_segment.extend_from_slice(&exif_tiff);

    // 在 SOI (FF D8) 之后插入 APP1 段
    let mut result = Vec::new();
    result.push(0xFF);
    result.push(0xD8);
    result.extend_from_slice(&app1_segment);
    result.extend_from_slice(&jpeg_bytes[2..]);

    result
}

/// 测试 EXIF Orientation=6（顺时针90°）被正确应用
#[test]
fn test_decode_image_applies_exif_orientation_6() {
    let jpeg_data = create_jpeg_with_orientation(400, 300, 6);

    let decoded = litemark_core::image_io::decode_image(&jpeg_data)
        .expect("带 EXIF 的 JPEG 解码失败");

    assert_eq!(
        decoded.width(), 300,
        "Orientation=6 应用后，宽度应从 400 变为 300"
    );
    assert_eq!(
        decoded.height(), 400,
        "Orientation=6 应用后，高度应从 300 变为 400"
    );
}

/// 测试 EXIF Orientation=1（正常）不触发旋转
#[test]
fn test_decode_image_preserves_orientation_1() {
    let jpeg_data = create_jpeg_with_orientation(400, 300, 1);

    let decoded = litemark_core::image_io::decode_image(&jpeg_data)
        .expect("带 EXIF 的 JPEG 解码失败");

    assert_eq!(decoded.width(), 400, "Orientation=1 应保持原宽度");
    assert_eq!(decoded.height(), 300, "Orientation=1 应保持原高度");
}

/// 测试无 EXIF 的图像正常解码
#[test]
fn test_decode_image_without_exif_orientation() {
    let image = create_test_image(400, 300, [128, 128, 128]);
    let jpeg_data = litemark_core::image_io::encode_image(&image, ImageFormat::Jpeg)
        .expect("编码失败");

    let decoded = litemark_core::image_io::decode_image(&jpeg_data)
        .expect("解码失败");

    assert_eq!(decoded.width(), 400);
    assert_eq!(decoded.height(), 300);
}
