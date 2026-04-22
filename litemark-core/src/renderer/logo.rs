use image::RgbaImage;

impl super::WatermarkRenderer {
    /// 从字节数据渲染 Logo
    pub(super) fn render_logo_from_bytes(
        &self,
        image: &mut RgbaImage,
        logo_data: &[u8],
        center_x: i32,
        center_y: i32,
        target_height: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 从字节数据加载 Logo
        let logo_img = match image::load_from_memory(logo_data) {
            Ok(img) => img,
            Err(_) => {
                // Logo 加载失败，静默跳过
                return Ok(());
            }
        };

        let logo_rgba = logo_img.to_rgba8();
        let (logo_w, logo_h) = logo_rgba.dimensions();

        // Scale logo to fit target height while maintaining aspect ratio
        let aspect_ratio = logo_w as f32 / logo_h as f32;
        let scaled_h = target_height;
        let scaled_w = (target_height as f32 * aspect_ratio) as u32;

        let start_x = center_x - (scaled_w as i32 / 2);
        let start_y = center_y - (scaled_h as i32 / 2);

        // Draw logo with alpha blending using bilinear interpolation
        for y in 0..scaled_h {
            for x in 0..scaled_w {
                // Bilinear sampling for smoother logo scaling
                let src_xf = (x as f32 / scaled_w as f32) * (logo_w - 1) as f32;
                let src_yf = (y as f32 / scaled_h as f32) * (logo_h - 1) as f32;

                let src_x0 = src_xf as u32;
                let src_y0 = src_yf as u32;
                let src_x1 = (src_x0 + 1).min(logo_w - 1);
                let src_y1 = (src_y0 + 1).min(logo_h - 1);

                let fx = src_xf - src_x0 as f32;
                let fy = src_yf - src_y0 as f32;

                let p00 = logo_rgba.get_pixel(src_x0, src_y0);
                let p10 = logo_rgba.get_pixel(src_x1, src_y0);
                let p01 = logo_rgba.get_pixel(src_x0, src_y1);
                let p11 = logo_rgba.get_pixel(src_x1, src_y1);

                let px = start_x + x as i32;
                let py = start_y + y as i32;
                if px >= 0 && py >= 0 && px < image.width() as i32 && py < image.height() as i32 {
                    let px_u32 = px as u32;
                    let py_u32 = py as u32;

                    // Bilinear interpolation for each channel
                    let sample = |idx: usize| {
                        let v00 = p00[idx] as f32;
                        let v10 = p10[idx] as f32;
                        let v01 = p01[idx] as f32;
                        let v11 = p11[idx] as f32;

                        let v0 = v00 * (1.0 - fx) + v10 * fx;
                        let v1 = v01 * (1.0 - fx) + v11 * fx;
                        (v0 * (1.0 - fy) + v1 * fy) as u8
                    };

                    let alpha = sample(3) as f32 / 255.0;

                    if alpha > 0.01 {
                        let bg_pixel = image.get_pixel(px_u32, py_u32);

                        let r = ((sample(0) as f32 * alpha) + (bg_pixel[0] as f32 * (1.0 - alpha)))
                            as u8;
                        let g = ((sample(1) as f32 * alpha) + (bg_pixel[1] as f32 * (1.0 - alpha)))
                            as u8;
                        let b = ((sample(2) as f32 * alpha) + (bg_pixel[2] as f32 * (1.0 - alpha)))
                            as u8;

                        image.put_pixel(px_u32, py_u32, image::Rgba([r, g, b, 255]));
                    }
                }
            }
        }

        Ok(())
    }
}
