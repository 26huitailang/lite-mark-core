use image::{Rgba, RgbaImage};

/// Alpha blend a foreground color over a background pixel
pub(super) fn blend_pixel(bg: Rgba<u8>, fg: Rgba<u8>) -> Rgba<u8> {
    let alpha = fg[3] as f32 / 255.0;
    if alpha < 0.01 {
        return bg;
    }

    let r = ((fg[0] as f32 * alpha) + (bg[0] as f32 * (1.0 - alpha))) as u8;
    let g = ((fg[1] as f32 * alpha) + (bg[1] as f32 * (1.0 - alpha))) as u8;
    let b = ((fg[2] as f32 * alpha) + (bg[2] as f32 * (1.0 - alpha))) as u8;
    let a = bg[3].max(fg[3]);

    Rgba([r, g, b, a])
}

impl super::WatermarkRenderer {
    pub(super) fn render_frame_background(
        &self,
        image: &mut RgbaImage,
        frame_y: u32,
        width: u32,
        height: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Draw frame background (usually white or light color)
        let bg_color = Rgba([255, 255, 255, 255]); // White frame background

        for y in 0..height {
            for x in 0..width {
                if frame_y + y < image.height() {
                    image.put_pixel(x, frame_y + y, bg_color);
                }
            }
        }

        Ok(())
    }

    pub(super) fn render_gradient_background(
        &self,
        image: &mut RgbaImage,
        frame_y: u32,
        width: u32,
        height: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Gradient from semi-transparent white (at top) to solid white (at bottom)
        for y in 0..height {
            let progress = y as f32 / height as f32;
            // Alpha ranges from ~25% (64) at top to 100% (255) at bottom
            let alpha = (64.0 + 191.0 * progress) as u8;
            let overlay = Rgba([255, 255, 255, alpha]);

            for x in 0..width {
                if frame_y + y < image.height() {
                    let original = *image.get_pixel(x, frame_y + y);
                    let blended = blend_pixel(original, overlay);
                    image.put_pixel(x, frame_y + y, blended);
                }
            }
        }

        Ok(())
    }

    pub(super) fn render_minimal_line(
        &self,
        image: &mut RgbaImage,
        frame_y: u32,
        width: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let line_color = Rgba([224, 224, 224, 255]); // #E0E0E0
        let y = frame_y;

        for x in 0..width {
            if y < image.height() {
                image.put_pixel(x, y, line_color);
            }
        }

        Ok(())
    }

    pub(super) fn render_rounded_rect(
        &self,
        image: &mut RgbaImage,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        radius: u32,
        color: Rgba<u8>,
    ) {
        let r = radius.min(width / 2).min(height / 2);
        let r_sq = (r * r) as f32;

        for dy in 0..height {
            for dx in 0..width {
                let px = x + dx;
                let py = y + dy;
                if px >= image.width() || py >= image.height() {
                    continue;
                }

                // Determine if pixel is inside the rounded rectangle
                let mut inside = true;

                // Check corner regions
                if dx < r && dy < r {
                    // Top-left corner
                    let cx = r - dx;
                    let cy = r - dy;
                    if (cx * cx + cy * cy) as f32 > r_sq {
                        inside = false;
                    }
                } else if dx >= width - r && dy < r {
                    // Top-right corner
                    let cx = dx - (width - r);
                    let cy = r - dy;
                    if (cx * cx + cy * cy) as f32 > r_sq {
                        inside = false;
                    }
                } else if dx < r && dy >= height - r {
                    // Bottom-left corner
                    let cx = r - dx;
                    let cy = dy - (height - r);
                    if (cx * cx + cy * cy) as f32 > r_sq {
                        inside = false;
                    }
                } else if dx >= width - r && dy >= height - r {
                    // Bottom-right corner
                    let cx = dx - (width - r);
                    let cy = dy - (height - r);
                    if (cx * cx + cy * cy) as f32 > r_sq {
                        inside = false;
                    }
                }

                if inside {
                    let bg = image.get_pixel(px, py);
                    let blended = blend_pixel(*bg, color);
                    image.put_pixel(px, py, blended);
                }
            }
        }
    }

    pub(super) fn render_vertical_line(
        &self,
        image: &mut RgbaImage,
        x: u32,
        y_start: u32,
        height: u32,
        color: Rgba<u8>,
    ) {
        for y in y_start..(y_start + height) {
            if x < image.width() && y < image.height() {
                image.put_pixel(x, y, color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;

    #[test]
    fn test_blend_pixel_alpha_zero_returns_background() {
        let bg = Rgba([100, 100, 100, 255]);
        let fg = Rgba([200, 200, 200, 0]);
        let result = blend_pixel(bg, fg);
        assert_eq!(result, bg, "Alpha=0 时必须完全返回背景色");
    }

    #[test]
    fn test_blend_pixel_alpha_255_returns_foreground() {
        let bg = Rgba([100, 100, 100, 255]);
        let fg = Rgba([200, 200, 200, 255]);
        let result = blend_pixel(bg, fg);
        assert_eq!(result, fg, "Alpha=255 时必须完全返回前景色");
    }

    #[test]
    fn test_blend_pixel_alpha_128_half_blend() {
        let bg = Rgba([0, 0, 0, 255]);
        let fg = Rgba([255, 255, 255, 128]);
        let result = blend_pixel(bg, fg);
        // alpha = 128/255 ≈ 0.50196
        // r = 255 * 0.50196 + 0 * 0.49804 = 128.0
        assert_eq!(result.0[0], 128, "红色通道应混合为 128");
        assert_eq!(result.0[1], 128, "绿色通道应混合为 128");
        assert_eq!(result.0[2], 128, "蓝色通道应混合为 128");
        assert_eq!(result.0[3], 255, "Alpha 应取 max(bg.a, fg.a) = 255");
    }

    #[test]
    fn test_blend_pixel_preserves_bg_alpha_when_fg_opaque() {
        let bg = Rgba([100, 100, 100, 128]);
        let fg = Rgba([200, 200, 200, 255]);
        let result = blend_pixel(bg, fg);
        assert_eq!(result.0[3], 255, "fg.a=255, bg.a=128, max=255");
    }

    #[test]
    fn test_blend_pixel_alpha_below_threshold() {
        let bg = Rgba([100, 100, 100, 255]);
        let fg = Rgba([200, 200, 200, 2]); // alpha=2, 2/255=0.0078 < 0.01
        let result = blend_pixel(bg, fg);
        assert_eq!(result, bg, "Alpha < 0.01 时应直接返回背景色");
    }

    #[test]
    fn test_render_frame_background_fills_region() {
        let renderer = crate::renderer::WatermarkRenderer::new().unwrap();
        let mut img = RgbaImage::from_pixel(10, 10, Rgba([0, 0, 0, 255]));
        renderer.render_frame_background(&mut img, 5, 10, 5).unwrap();

        // y=0..4 应保持黑色
        for y in 0..5 {
            for x in 0..10 {
                assert_eq!(*img.get_pixel(x, y), Rgba([0, 0, 0, 255]),
                    "frame_y 上方的像素应保持不变");
            }
        }
        // y=5..9 应变为白色
        for y in 5..10 {
            for x in 0..10 {
                assert_eq!(*img.get_pixel(x, y), Rgba([255, 255, 255, 255]),
                    "frame_y 开始的区域应被白色填充");
            }
        }
    }

    #[test]
    fn test_render_frame_background_clamps_to_image_height() {
        let renderer = crate::renderer::WatermarkRenderer::new().unwrap();
        let mut img = RgbaImage::from_pixel(10, 10, Rgba([0, 0, 0, 255]));
        // frame_y=8, height=5 → 只会填充 y=8,9，不会越界
        renderer.render_frame_background(&mut img, 8, 10, 5).unwrap();
        assert_eq!(*img.get_pixel(0, 9), Rgba([255, 255, 255, 255]));
        // 确保没有 panic
    }

    #[test]
    fn test_render_minimal_line_draws_single_row() {
        let renderer = crate::renderer::WatermarkRenderer::new().unwrap();
        let mut img = RgbaImage::from_pixel(10, 10, Rgba([0, 0, 0, 255]));
        renderer.render_minimal_line(&mut img, 5, 10).unwrap();

        // y=5 整行应为线颜色
        for x in 0..10 {
            assert_eq!(*img.get_pixel(x, 5), Rgba([224, 224, 224, 255]),
                "目标行应被线颜色填充");
        }
        // y=4 和 y=6 应保持黑色
        for x in 0..10 {
            assert_eq!(*img.get_pixel(x, 4), Rgba([0, 0, 0, 255]),
                "目标行上方的像素应保持不变");
            assert_eq!(*img.get_pixel(x, 6), Rgba([0, 0, 0, 255]),
                "目标行下方的像素应保持不变");
        }
    }

    #[test]
    fn test_render_vertical_line_draws_column() {
        let renderer = crate::renderer::WatermarkRenderer::new().unwrap();
        let mut img = RgbaImage::from_pixel(10, 10, Rgba([0, 0, 0, 255]));
        renderer.render_vertical_line(&mut img, 5, 2, 5, Rgba([255, 0, 0, 255]));

        // x=5, y=2..6 应为红色
        for y in 2..7 {
            assert_eq!(*img.get_pixel(5, y), Rgba([255, 0, 0, 255]),
                "目标列范围内应被填充");
        }
        // x=5, y=1 和 y=7 应保持黑色
        assert_eq!(*img.get_pixel(5, 1), Rgba([0, 0, 0, 255]));
        assert_eq!(*img.get_pixel(5, 7), Rgba([0, 0, 0, 255]));
        // x=4 和 x=6 应保持黑色
        assert_eq!(*img.get_pixel(4, 4), Rgba([0, 0, 0, 255]));
        assert_eq!(*img.get_pixel(6, 4), Rgba([0, 0, 0, 255]));
    }

    #[test]
    fn test_render_rounded_rect_center_filled() {
        let renderer = crate::renderer::WatermarkRenderer::new().unwrap();
        let mut img = RgbaImage::from_pixel(20, 20, Rgba([0, 0, 0, 255]));
        renderer.render_rounded_rect(&mut img, 5, 5, 10, 10, 2, Rgba([255, 0, 0, 255]));

        // 中心区域 (远离圆角) 应被填充
        assert_eq!(*img.get_pixel(8, 8), Rgba([255, 0, 0, 255]),
            "矩形中心应被填充");
        assert_eq!(*img.get_pixel(12, 12), Rgba([255, 0, 0, 255]),
            "矩形中心应被填充");
    }

    #[test]
    fn test_render_rounded_rect_corner_not_filled() {
        let renderer = crate::renderer::WatermarkRenderer::new().unwrap();
        let mut img = RgbaImage::from_pixel(20, 20, Rgba([0, 0, 0, 255]));
        renderer.render_rounded_rect(&mut img, 5, 5, 10, 10, 2, Rgba([255, 0, 0, 255]));

        // 圆角外（相对于矩形位置 (5,5) 的 (5,5) 即绝对 (5,5) 应该是圆角外）
        // 实际上圆角半径=2，所以 (5,5) 是圆角区域，不应填充
        // 绝对坐标 (5,5) 对应矩形内 (0,0)，圆角中心在 (5+2, 5+2) = (7,7)
        // (5,5) 距离中心 (7,7) 的距离 = sqrt(4+4) = 2.8 > 半径 2，所以不应填充
        assert_eq!(*img.get_pixel(5, 5), Rgba([0, 0, 0, 255]),
            "圆角外的像素应保持背景色");
    }

    #[test]
    fn test_render_rounded_rect_radius_clamped() {
        let renderer = crate::renderer::WatermarkRenderer::new().unwrap();
        let mut img = RgbaImage::from_pixel(20, 20, Rgba([0, 0, 0, 255]));
        // 半径大于宽高一半，应被 clamp 到 min(w/2, h/2)
        renderer.render_rounded_rect(&mut img, 5, 5, 4, 4, 10, Rgba([255, 0, 0, 255]));

        // 不应 panic，中心区域应被填充
        assert_eq!(*img.get_pixel(6, 6), Rgba([255, 0, 0, 255]));
    }

    #[test]
    fn test_render_rounded_rect_outside_bounds_ignored() {
        let renderer = crate::renderer::WatermarkRenderer::new().unwrap();
        let mut img = RgbaImage::from_pixel(10, 10, Rgba([0, 0, 0, 255]));
        // 矩形部分超出图像边界，不应 panic
        renderer.render_rounded_rect(&mut img, 8, 8, 5, 5, 1, Rgba([255, 0, 0, 255]));
        // 图像内且在矩形非圆角区域的部分应被填充
        // (8,9): dx=0, dy=1, 不在圆角判断区域内 (dy=1 不小于 r=1)
        assert_eq!(*img.get_pixel(8, 9), Rgba([255, 0, 0, 255]));
    }

    #[test]
    fn test_render_gradient_background_alpha_gradient() {
        let renderer = crate::renderer::WatermarkRenderer::new().unwrap();
        let mut img = RgbaImage::from_pixel(10, 10, Rgba([0, 0, 0, 255]));
        renderer.render_gradient_background(&mut img, 0, 10, 10).unwrap();

        // blend_pixel 的 alpha = max(bg.a, fg.a)，混合后像素 alpha 永远是 255
        // 应检查 RGB 亮度是否单调递增
        // progress=0: overlay alpha=64, blend -> r ≈ 255*(64/255) = 64
        // progress=1: overlay alpha=255, blend -> r = 255
        let top_pixel = img.get_pixel(5, 0);
        assert!(top_pixel[0] >= 60 && top_pixel[0] <= 68,
            "首行 RGB 应约为 64（25% 混合），实际 r={}", top_pixel[0]);

        let bottom_pixel = img.get_pixel(5, 9);
        // height=10, y=9, progress=0.9, alpha=64+191*0.9=235.9≈235
        assert!(bottom_pixel[0] >= 230 && bottom_pixel[0] <= 240,
            "末行 RGB 应约为 235（progress=0.9 时 alpha≈235），实际 r={}", bottom_pixel[0]);

        // RGB 亮度应单调递增
        let mut last_brightness = 0;
        for y in 0..10 {
            let brightness = img.get_pixel(5, y)[0];
            assert!(brightness >= last_brightness,
                "RGB 亮度应单调递增: y={} 时 brightness={} < 上一行 brightness={}",
                y, brightness, last_brightness);
            last_brightness = brightness;
        }
    }
}
