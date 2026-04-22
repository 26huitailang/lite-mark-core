use image::{Rgba, RgbaImage};

/// Alpha blend a foreground color over a background pixel
fn blend_pixel(bg: Rgba<u8>, fg: Rgba<u8>) -> Rgba<u8> {
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
