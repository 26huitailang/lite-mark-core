/// 渲染简单的 Logo 测试图片 - 加粗的 "Peter"
/// 运行方式: cargo run --example render_test
use image::{Rgba, RgbaImage};
use rusttype::{point, Font, Scale};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 生成 Logo 测试图片...");

    // 加载 DejaVuSans.ttf 字体
    let font_path = "assets/fonts/DejaVuSans.ttf";
    let font_data =
        std::fs::read(font_path).map_err(|e| format!("无法读取字体文件 {}: {}", font_path, e))?;

    let font = Font::try_from_vec(font_data).ok_or("无法解析字体文件")?;

    // 创建透明背景的小画布 (200x80)
    let width = 200u32;
    let height = 80u32;
    let mut image = RgbaImage::from_pixel(width, height, Rgba([0, 0, 0, 0])); // 透明背景

    // 渲染加粗的 "Peter" 文字 (通过多次渲染实现加粗效果)
    let text = "Peter";
    let font_size = 48.0;
    let color = Rgba([0, 0, 0, 255]); // 黑色
    let x = 20;
    let y = 15;

    // 多次渲染以实现加粗效果
    for dx in -1..=1 {
        for dy in -1..=1 {
            render_text(&mut image, &font, text, x + dx, y + dy, font_size, color);
        }
    }

    // 保存图片
    let output_path = "test_logo_peter.png";
    image.save(output_path)?;

    println!("✅ Logo 测试图片已生成: {}", output_path);
    println!("   尺寸: {}x{}", width, height);
    println!("   内容: 加粗的 \"Peter\"");
    println!("   背景: 透明");

    Ok(())
}

/// 在图片上渲染文本
fn render_text(
    image: &mut RgbaImage,
    font: &Font,
    text: &str,
    x: i32,
    y: i32,
    font_size: f32,
    color: Rgba<u8>,
) {
    let scale = Scale::uniform(font_size);
    let v_metrics = font.v_metrics(scale);
    let baseline_y = y as f32 + v_metrics.ascent;
    let offset = point(x as f32, baseline_y);

    // 布局并渲染字形
    let glyphs: Vec<_> = font.layout(text, scale, offset).collect();

    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|px, py, v| {
                let px = px as i32 + bounding_box.min.x;
                let py = py as i32 + bounding_box.min.y;

                if px >= 0 && py >= 0 && px < image.width() as i32 && py < image.height() as i32 {
                    let alpha = (v * 255.0) as u8;
                    if alpha > 10 {
                        // 使用纯色以获得更好的可见性
                        let pixel_color = Rgba([color[0], color[1], color[2], 255]);
                        image.put_pixel(px as u32, py as u32, pixel_color);
                    }
                }
            });
        }
    }
}
