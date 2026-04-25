use image::ImageFormat;
use litemark_core::exif::ExifData;
use litemark_core::layout;
use litemark_core::renderer::WatermarkRenderer;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn get_standard_variables() -> HashMap<String, String> {
    let mut exif = ExifData::new();
    exif.author = Some("Peter Parker".to_string());
    exif.camera_model = Some("Sony A7M4".to_string());
    exif.lens_model = Some("FE 85mm F1.8".to_string());
    exif.iso = Some(400);
    exif.aperture = Some(2.8);
    exif.shutter_speed = Some("1/200".to_string());
    exif.focal_length = Some(85.0);
    exif.date_time = Some("2024-06-15 14:30:00".to_string());
    exif.to_variables()
}

fn get_minimal_variables() -> HashMap<String, String> {
    let mut exif = ExifData::new();
    exif.author = Some("Peter Parker".to_string());
    exif.to_variables()
}

fn render_and_save(
    input_path: &str,
    output_dir: &str,
    template_name: &str,
    variables: &HashMap<String, String>,
    suffix: &str,
) {
    let input_data = fs::read(input_path).expect("读取输入图片失败");
    let mut image = litemark_core::image_io::decode_image(&input_data).expect("解码图片失败");
    let original_width = image.width();
    let original_height = image.height();

    let templates = layout::create_builtin_templates();
    let template = templates
        .iter()
        .find(|t| t.name == template_name)
        .unwrap_or_else(|| panic!("找不到模板: {}", template_name));

    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");
    renderer
        .render_watermark_with_logo_bytes(&mut image, template, variables, None)
        .expect("渲染失败");

    let filename = format!(
        "{}_{}x{}_{}{}.jpg",
        Path::new(input_path).file_stem().unwrap().to_str().unwrap(),
        original_width,
        original_height,
        template_name.to_lowercase(),
        suffix
    );
    let output_path = Path::new(output_dir).join(&filename);

    let encoded =
        litemark_core::image_io::encode_image(&image, ImageFormat::Jpeg).expect("编码 JPEG 失败");
    fs::write(&output_path, encoded).expect("写入文件失败");

    println!(
        "✅ {} ({}x{} -> {}x{})",
        output_path.display(),
        original_width,
        original_height,
        image.width(),
        image.height()
    );
}

fn main() {
    let output_dir = "output/visual_check";
    fs::create_dir_all(output_dir).expect("创建目录失败");

    // 真实照片测试
    let real_photo = "test_images/demos/DSC09787.JPG";

    println!("\n📷 真实照片渲染测试: {}\n", real_photo);

    if Path::new(real_photo).exists() {
        let standard_vars = get_standard_variables();
        render_and_save(real_photo, output_dir, "Classic", &standard_vars, "_std");
        render_and_save(real_photo, output_dir, "Overlay", &standard_vars, "_std");

        let minimal_vars = get_minimal_variables();
        render_and_save(real_photo, output_dir, "Classic", &minimal_vars, "_minimal");
    } else {
        println!("⚠️  真实照片不存在: {}，跳过", real_photo);
    }

    // 合成图测试（用于对比参考）
    println!("\n🎨 合成图渲染测试\n");

    let synthetic = image::DynamicImage::ImageRgb8(image::RgbImage::from_fn(1920, 1080, |x, y| {
        let r = ((x * 7 + y * 13) % 256) as u8;
        let g = ((x * 11 + y * 17) % 256) as u8;
        let b = ((x * 13 + y * 7 + 128) % 256) as u8;
        image::Rgb([r, g, b])
    }));

    let synthetic_path = Path::new(output_dir).join("synthetic_1920x1080.png");
    let mut buf = Vec::new();
    synthetic
        .write_to(&mut std::io::Cursor::new(&mut buf), ImageFormat::Png)
        .unwrap();
    fs::write(&synthetic_path, buf).unwrap();

    let standard_vars = get_standard_variables();
    render_and_save(
        synthetic_path.to_str().unwrap(),
        output_dir,
        "Classic",
        &standard_vars,
        "_std",
    );
    render_and_save(
        synthetic_path.to_str().unwrap(),
        output_dir,
        "Overlay",
        &standard_vars,
        "_std",
    );

    // Portrait 方向测试（模拟竖拍）
    println!("\n📱 Portrait 方向渲染测试\n");

    let portrait = image::DynamicImage::ImageRgb8(image::RgbImage::from_fn(1080, 1920, |x, y| {
        let v = (((x * 3 + y * 5) % 180 + 40) as u8);
        image::Rgb([v, v, v])
    }));

    let portrait_path = Path::new(output_dir).join("portrait_1080x1920.png");
    let mut buf = Vec::new();
    portrait
        .write_to(&mut std::io::Cursor::new(&mut buf), ImageFormat::Png)
        .unwrap();
    fs::write(&portrait_path, buf).unwrap();

    let standard_vars = get_standard_variables();
    render_and_save(
        portrait_path.to_str().unwrap(),
        output_dir,
        "Classic",
        &standard_vars,
        "_std",
    );

    println!("\n📁 输出目录: {}", output_dir);
}
