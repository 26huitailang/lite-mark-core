use criterion::{Criterion, black_box, criterion_group, criterion_main};
use image::{DynamicImage, Rgb, RgbImage};
use litemark_core::exif::ExifData;
use litemark_core::layout::{Anchor, FontWeight, ItemType, RenderMode, Template, TemplateItem};
use litemark_core::renderer::WatermarkRenderer;
use std::collections::HashMap;

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
    exif.author = Some("Benchmark Test".to_string());
    exif.camera_model = Some("Sony A7M4".to_string());
    exif.lens_model = Some("FE 85mm F1.8".to_string());
    exif.iso = Some(400);
    exif.aperture = Some(2.8);
    exif.shutter_speed = Some("1/200".to_string());
    exif.focal_length = Some(85.0);
    exif.to_variables()
}

fn create_benchmark_template() -> Template {
    Template {
        name: "Benchmark".to_string(),
        anchor: Anchor::BottomLeft,
        padding: 20,
        items: vec![
            TemplateItem {
                item_type: ItemType::Text,
                value: "{Author}".to_string(),
                font_size: 20,
                font_size_ratio: 0.22,
                weight: Some(FontWeight::Bold),
                color: Some("#1A1A1A".to_string()),
            },
            TemplateItem {
                item_type: ItemType::Text,
                value: "{Camera} • {Lens}".to_string(),
                font_size: 14,
                font_size_ratio: 0.16,
                weight: Some(FontWeight::Normal),
                color: Some("#4A4A4A".to_string()),
            },
            TemplateItem {
                item_type: ItemType::Text,
                value: "{Aperture} | ISO {ISO} | {Shutter}".to_string(),
                font_size: 14,
                font_size_ratio: 0.14,
                weight: Some(FontWeight::Normal),
                color: Some("#6A6A6A".to_string()),
            },
        ],
        background: None,
        frame_height_ratio: 0.10,
        logo_size_ratio: 0.35,
        primary_font_ratio: 0.22,
        secondary_font_ratio: 0.16,
        padding_ratio: 0.10,
        render_mode: RenderMode::BottomFrame,
    }
}

fn bench_render_1920x1080(c: &mut Criterion) {
    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");
    let variables = get_test_variables();
    let template = create_benchmark_template();

    c.bench_function("render_1920x1080", |b| {
        b.iter(|| {
            let mut image = create_test_image(1920, 1080);
            renderer
                .render_watermark_with_logo_bytes(
                    black_box(&mut image),
                    black_box(&template),
                    black_box(&variables),
                    black_box(None),
                )
                .expect("渲染失败");
        })
    });
}

fn bench_render_800x600(c: &mut Criterion) {
    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");
    let variables = get_test_variables();
    let template = create_benchmark_template();

    c.bench_function("render_800x600", |b| {
        b.iter(|| {
            let mut image = create_test_image(800, 600);
            renderer
                .render_watermark_with_logo_bytes(
                    black_box(&mut image),
                    black_box(&template),
                    black_box(&variables),
                    black_box(None),
                )
                .expect("渲染失败");
        })
    });
}

fn bench_render_4000x3000(c: &mut Criterion) {
    let renderer = WatermarkRenderer::new().expect("创建渲染器失败");
    let variables = get_test_variables();
    let template = create_benchmark_template();

    c.bench_function("render_4000x3000", |b| {
        b.iter(|| {
            let mut image = create_test_image(4000, 3000);
            renderer
                .render_watermark_with_logo_bytes(
                    black_box(&mut image),
                    black_box(&template),
                    black_box(&variables),
                    black_box(None),
                )
                .expect("渲染失败");
        })
    });
}

criterion_group!(
    benches,
    bench_render_1920x1080,
    bench_render_800x600,
    bench_render_4000x3000,
);
criterion_main!(benches);
