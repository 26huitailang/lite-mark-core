use image::{DynamicImage, ImageFormat, ImageOutputFormat, RgbaImage};
#[cfg(not(target_arch = "wasm32"))]
use libheif_rs::{ColorSpace, HeifContext, RgbChroma};
use std::io::Cursor;

/// 从字节数据解码图像（Core 接口，用于 Web/WASM）
///
/// # Arguments
/// * `image_data` - 图像文件的字节数据
///
/// # Returns
/// * `Ok(DynamicImage)` - 解码后的图像
/// * `Err` - 解码错误
///
/// # Examples
/// ```
/// let image_bytes = std::fs::read("photo.jpg")?;
/// let image = decode_image(&image_bytes)?;
/// println!("Image size: {}x{}", image.width(), image.height());
/// ```
pub fn decode_image(image_data: &[u8]) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    // 尝试检测是否为 HEIC/HEIF 格式
    if is_heic_format(image_data) {
        return decode_heic_from_bytes(image_data);
    }

    // 使用 image 库解码其他格式
    let img = image::load_from_memory(image_data)?;
    Ok(img)
}

/// 将图像编码为字节数据（Core 接口，用于 Web/WASM）
///
/// # Arguments
/// * `image` - 要编码的图像
/// * `format` - 输出格式
///
/// # Returns
/// * `Ok(Vec<u8>)` - 编码后的字节数据
/// * `Err` - 编码错误
///
/// # Examples
/// ```
/// let image = image::DynamicImage::new_rgb8(100, 100);
/// let jpeg_bytes = encode_image(&image, ImageFormat::Jpeg)?;
/// std::fs::write("output.jpg", jpeg_bytes)?;
/// ```
pub fn encode_image(
    image: &DynamicImage,
    format: ImageFormat,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Cursor::new(Vec::new());

    let output_format = match format {
        ImageFormat::Jpeg => ImageOutputFormat::Jpeg(90), // 90% quality
        ImageFormat::Png => ImageOutputFormat::Png,
        ImageFormat::WebP => ImageOutputFormat::WebP,
        _ => ImageOutputFormat::Jpeg(90), // 默认 JPEG
    };

    image.write_to(&mut buffer, output_format)?;
    Ok(buffer.into_inner())
}

/// 检测字节数据的图像格式
///
/// # Arguments
/// * `image_data` - 图像文件的字节数据
///
/// # Returns
/// * `ImageFormat` - 检测到的格式，无法识别时返回 JPEG
pub fn detect_format(image_data: &[u8]) -> ImageFormat {
    if is_heic_format(image_data) {
        // libheif 不在 ImageFormat 枚举中，返回 JPEG 表示需要特殊处理
        return ImageFormat::Jpeg;
    }

    image::guess_format(image_data).unwrap_or(ImageFormat::Jpeg)
}

/// 检查是否为 HEIC/HEIF 格式
#[cfg(not(target_arch = "wasm32"))]
fn is_heic_format(data: &[u8]) -> bool {
    if data.len() < 12 {
        return false;
    }
    // HEIC/HEIF 文件的魔数检测
    // ftyp box 从第 4 字节开始
    if &data[4..8] == b"ftyp" {
        // 检查品牌标识
        let brand = &data[8..12];
        matches!(brand, b"heic" | b"heix" | b"hevc" | b"hevx" | b"mif1")
    } else {
        false
    }
}

#[cfg(target_arch = "wasm32")]
fn is_heic_format(_data: &[u8]) -> bool {
    false
}

/// 从字节数据解码 HEIC/HEIF 图像
#[cfg(not(target_arch = "wasm32"))]
fn decode_heic_from_bytes(data: &[u8]) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    // 从字节数据读取 HEIC
    let ctx = HeifContext::read_from_bytes(data)?;
    let handle = ctx.primary_image_handle()?;

    // Decode to RGB
    let width = handle.width();
    let height = handle.height();

    // Decode the image
    let image =
        libheif_rs::LibHeif::new().decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;

    let planes = image.planes();
    let interleaved_plane = planes.interleaved.ok_or("No interleaved plane")?;

    // Convert to RGBA format for consistency
    let mut rgba_data = Vec::with_capacity((width * height * 4) as usize);
    let rgb_data = interleaved_plane.data;
    let stride = interleaved_plane.stride as usize;

    for y in 0..height {
        let row_start = y as usize * stride;
        for x in 0..width {
            let pixel_start = row_start + (x as usize * 3);
            if pixel_start + 2 < rgb_data.len() {
                rgba_data.push(rgb_data[pixel_start]); // R
                rgba_data.push(rgb_data[pixel_start + 1]); // G
                rgba_data.push(rgb_data[pixel_start + 2]); // B
                rgba_data.push(255); // A (fully opaque)
            }
        }
    }

    let rgba_image = RgbaImage::from_raw(width, height, rgba_data)
        .ok_or("Failed to create RGBA image from HEIC data")?;

    Ok(DynamicImage::ImageRgba8(rgba_image))
}

#[cfg(target_arch = "wasm32")]
fn decode_heic_from_bytes(_data: &[u8]) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    Err("HEIC/HEIF decoding is not supported on WebAssembly".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgb, RgbImage};

    #[test]
    fn test_encode_decode_roundtrip() {
        // 创建一个简单的测试图像
        let img = DynamicImage::ImageRgb8(RgbImage::from_fn(100, 100, |x, y| {
            if (x + y) % 2 == 0 {
                Rgb([255, 0, 0])
            } else {
                Rgb([0, 0, 255])
            }
        }));

        // 编码为 PNG
        let png_bytes = encode_image(&img, ImageFormat::Png).unwrap();
        assert!(!png_bytes.is_empty());

        // 解码
        let decoded = decode_image(&png_bytes).unwrap();
        assert_eq!(decoded.width(), 100);
        assert_eq!(decoded.height(), 100);
    }

    #[test]
    fn test_detect_format() {
        // PNG 魔数
        let png_header = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        let format = detect_format(&png_header);
        assert!(matches!(format, ImageFormat::Png));

        // JPEG 魔数
        let jpeg_header = vec![0xFF, 0xD8, 0xFF];
        let format = detect_format(&jpeg_header);
        assert!(matches!(format, ImageFormat::Jpeg));
    }

    #[test]
    fn test_is_heic_format() {
        // 模拟 HEIC 文件头
        let mut heic_data = vec![0; 12];
        heic_data[4..8].copy_from_slice(b"ftyp");
        heic_data[8..12].copy_from_slice(b"heic");

        assert!(is_heic_format(&heic_data));

        // 非 HEIC 数据
        let not_heic = vec![0xFF, 0xD8, 0xFF];
        assert!(!is_heic_format(&not_heic));
    }

    #[test]
    fn test_encode_different_formats() {
        let img = DynamicImage::ImageRgb8(RgbImage::new(10, 10));

        // 测试 JPEG 编码
        let jpeg = encode_image(&img, ImageFormat::Jpeg);
        assert!(jpeg.is_ok());

        // 测试 PNG 编码
        let png = encode_image(&img, ImageFormat::Png);
        assert!(png.is_ok());

        // 测试 WebP 编码
        let webp = encode_image(&img, ImageFormat::WebP);
        assert!(webp.is_ok());
    }
}
