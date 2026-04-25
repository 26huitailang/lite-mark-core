use image::{DynamicImage, ImageFormat, Rgb, RgbImage};
use litemark_core::exif::ExifData;
use litemark_core::layout;
use litemark_core::renderer::WatermarkRenderer;
use std::collections::HashMap;
use std::fs;

fn create_test_image(width: u32, height: u32) -> DynamicImage {
    let img = RgbImage::from_fn(width, height, |x, y| {
        let r = ((x * 7 + y * 13) % 256) as u8;
        let g = ((x * 11 + y * 17) % 256) as u8;
        let b = ((x * 13 + y * 7 + 128) % 256) as u8;
        Rgb([r, g, b])
    });
    DynamicImage::ImageRgb8(img)
}

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

fn main() {
    let templates = layout::create_builtin_templates();
    let variables = get_test_variables();
    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");

    fs::create_dir_all("output/visual_check").expect("创建目录失败");

    for template in &templates {
        if template.name != "Classic" && template.name != "Overlay" {
            continue;
        }

        let mut image = create_test_image(1920, 1080);
        renderer
            .render_watermark_with_logo_bytes(&mut image, template, &variables, None)
            .expect("渲染失败");

        let path = format!("output/visual_check/{}_1920x1080.png", template.name.to_lowercase());
        let mut buf = Vec::new();
        image
            .write_to(&mut std::io::Cursor::new(&mut buf), ImageFormat::Png)
            .expect("编码失败");
        fs::write(&path, buf).expect("写入失败");
        println!("Generated: {}", path);
    }
}
