use crate::layout::{Anchor, Background, ItemType, Template};
use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use std::collections::HashMap;

pub struct WatermarkRenderer {
    // Simple renderer without external font dependencies
}

impl WatermarkRenderer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(WatermarkRenderer {})
    }

    pub fn render_watermark(
        &self,
        image: &mut DynamicImage,
        template: &Template,
        variables: &HashMap<String, String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let substituted_template = template.substitute_variables(variables);

        let (width, height) = image.dimensions();

        // Calculate position based on anchor
        let (x, y) = self.calculate_position(&substituted_template, width, height);

        // Convert to RgbaImage for manipulation
        let mut rgba_image = image.to_rgba8();

        // Render background if present
        if let Some(ref background) = substituted_template.background {
            self.render_background(&mut rgba_image, background, x, y, width, height)?;
        }

        // Render text items
        let mut current_y = y;
        for item in &substituted_template.items {
            match item.item_type {
                ItemType::Text => {
                    self.render_text(
                        &mut rgba_image,
                        &item.value,
                        x,
                        current_y,
                        item.font_size,
                        &item.color,
                    )?;
                    current_y += item.font_size as i32 + 5; // Add some spacing
                }
                ItemType::Logo => {
                    // TODO: Implement logo rendering
                    println!("Logo rendering not implemented yet");
                }
            }
        }

        // Convert back to DynamicImage
        *image = DynamicImage::ImageRgba8(rgba_image);

        Ok(())
    }

    fn calculate_position(&self, template: &Template, width: u32, height: u32) -> (i32, i32) {
        let padding = template.padding as i32;

        match template.anchor {
            Anchor::TopLeft => (padding, padding),
            Anchor::TopRight => (width as i32 - padding, padding),
            Anchor::BottomLeft => (padding, height as i32 - padding),
            Anchor::BottomRight => (width as i32 - padding, height as i32 - padding),
            Anchor::Center => (width as i32 / 2, height as i32 / 2),
        }
    }

    fn render_background(
        &self,
        image: &mut RgbaImage,
        background: &Background,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let bg_color = background
            .color
            .as_ref()
            .and_then(|c| parse_color(c))
            .unwrap_or(Rgba([0, 0, 0, (255.0 * background.opacity) as u8]));

        // Calculate background size based on content - make it larger and more visible
        let bg_width = 400; // Increased width
        let bg_height = 120; // Increased height

        let (bg_x, bg_y) = match (x, y) {
            (x, y) if x < width as i32 / 2 && y < height as i32 / 2 => (x, y), // Top-left
            (x, y) if x >= width as i32 / 2 && y < height as i32 / 2 => (x - bg_width, y), // Top-right
            (x, y) if x < width as i32 / 2 && y >= height as i32 / 2 => (x, y - bg_height), // Bottom-left
            (x, y) => (x - bg_width, y - bg_height), // Bottom-right
        };

        // Draw background rectangle with padding and border
        let padding = 10;
        for dy in -padding..bg_height + padding {
            for dx in -padding..bg_width + padding {
                let px = (bg_x + dx).max(0).min(width as i32 - 1) as u32;
                let py = (bg_y + dy).max(0).min(height as i32 - 1) as u32;

                if px < width && py < height {
                    // Add some border effect
                    let is_border = dx < 0 || dx >= bg_width || dy < 0 || dy >= bg_height;
                    let border_color = if is_border {
                        Rgba([255, 255, 255, (255.0 * background.opacity * 0.8) as u8])
                    // White border
                    } else {
                        bg_color
                    };
                    image.put_pixel(px, py, border_color);
                }
            }
        }

        Ok(())
    }

    fn render_text(
        &self,
        image: &mut RgbaImage,
        text: &str,
        x: i32,
        y: i32,
        font_size: u32,
        color: &Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let text_color = color
            .as_ref()
            .and_then(|c| parse_color(c))
            .unwrap_or(Rgba([255, 255, 255, 255])); // Bright white for better visibility

        // Simple text rendering using basic pixel drawing
        self.draw_simple_text(image, text, x, y, font_size, text_color);

        Ok(())
    }

    fn draw_simple_text(
        &self,
        image: &mut RgbaImage,
        text: &str,
        x: i32,
        y: i32,
        font_size: u32,
        color: Rgba<u8>,
    ) {
        // Make characters larger and more visible
        let char_width = (font_size as f32 * 1.2) as i32; // Increased width
        let char_height = (font_size as f32 * 1.8) as i32; // Increased height

        for (i, ch) in text.chars().enumerate() {
            let char_x = x + (i as i32 * char_width);
            let char_y = y;

            // Draw a simple representation of each character
            self.draw_character(image, ch, char_x, char_y, char_width, char_height, color);
        }
    }

    fn draw_character(
        &self,
        image: &mut RgbaImage,
        ch: char,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: Rgba<u8>,
    ) {
        let (img_width, img_height) = image.dimensions();

        // Simple character drawing - just draw a pattern based on the character
        for dy in 0..height {
            for dx in 0..width {
                let px = x + dx;
                let py = y + dy;

                if px >= 0 && py >= 0 && px < img_width as i32 && py < img_height as i32 {
                    // Create a simple pattern based on the character
                    let pattern = self.get_character_pattern(ch, dx, dy, width, height);
                    if pattern {
                        image.put_pixel(px as u32, py as u32, color);
                    }
                }
            }
        }
    }

    fn get_character_pattern(&self, ch: char, x: i32, y: i32, width: i32, height: i32) -> bool {
        // Very simple and clear character patterns
        match ch {
            'A' | 'a' => {
                // Simple A shape
                let center_x = width / 2;
                let is_left = y == 2 * x && x <= center_x;
                let is_right = y == 2 * (width - x) && x >= center_x;
                let is_middle = y == height / 2 && x >= width / 4 && x < 3 * width / 4;
                is_left || is_right || is_middle
            }
            'B' | 'b' => {
                // Simple B shape
                let is_left = x == 0;
                let is_top = y == 0 && x < width;
                let is_middle = y == height / 2 && x < width;
                let is_bottom = y == height - 1 && x < width;
                let is_right = x == width - 1 && y > 0 && y < height - 1;
                is_left || is_top || is_middle || is_bottom || is_right
            }
            'C' | 'c' => {
                // Simple C shape
                let is_left = x == 0;
                let is_top = y == 0 && x < width;
                let is_bottom = y == height - 1 && x < width;
                is_left || is_top || is_bottom
            }
            'D' | 'd' => {
                // Simple D shape
                let is_left = x == 0;
                let is_top = y == 0 && x < width;
                let is_bottom = y == height - 1 && x < width;
                let is_right = x == width - 1 && y > 0 && y < height - 1;
                is_left || is_top || is_bottom || is_right
            }
            'E' | 'e' => {
                // Simple E shape
                let is_left = x == 0;
                let is_top = y == 0 && x < width;
                let is_middle = y == height / 2 && x < width * 3 / 4;
                let is_bottom = y == height - 1 && x < width;
                is_left || is_top || is_middle || is_bottom
            }
            'F' | 'f' => {
                // Simple F shape
                let is_left = x == 0;
                let is_top = y == 0 && x < width;
                let is_middle = y == height / 2 && x < width * 3 / 4;
                is_left || is_top || is_middle
            }
            'G' | 'g' => {
                // Simple G shape
                let is_left = x == 0;
                let is_top = y == 0 && x < width;
                let is_bottom = y == height - 1 && x < width;
                let is_right = x == width - 1 && y >= height / 2;
                let is_middle = x == width / 2 && y >= height / 2;
                is_left || is_top || is_bottom || is_right || is_middle
            }
            'H' | 'h' => {
                // Simple H shape
                let is_left = x == 0;
                let is_right = x == width - 1;
                let is_middle = y == height / 2 && x < width;
                is_left || is_right || is_middle
            }
            'I' | 'i' => {
                // Simple I shape
                let is_top = y == 0 && x < width;
                let is_bottom = y == height - 1 && x < width;
                let is_middle = x == width / 2 && y < height;
                is_top || is_bottom || is_middle
            }
            'J' | 'j' => {
                // Simple J shape
                let is_top = y == 0 && x < width;
                let is_right = x == width - 1 && y < height * 3 / 4;
                let is_bottom = y == height - 1 && x < width * 3 / 4;
                let is_left = x == 0 && y >= height * 3 / 4;
                is_top || is_right || is_bottom || is_left
            }
            'K' | 'k' => {
                // Simple K shape
                let is_left = x == 0;
                let is_diagonal = (y == x + height / 2) || (y == height - x + height / 2);
                is_left || is_diagonal
            }
            'L' | 'l' => {
                // Simple L shape
                let is_left = x == 0;
                let is_bottom = y == height - 1 && x < width;
                is_left || is_bottom
            }
            'M' | 'm' => {
                // Simple M shape
                let is_left = x == 0;
                let is_right = x == width - 1;
                let is_middle_left = x == width / 4 && y < height / 2;
                let is_middle_right = x == 3 * width / 4 && y < height / 2;
                is_left || is_right || is_middle_left || is_middle_right
            }
            'N' | 'n' => {
                // Simple N shape
                let is_left = x == 0;
                let is_right = x == width - 1;
                let is_diagonal = x == y;
                is_left || is_right || is_diagonal
            }
            'O' | 'o' => {
                // Simple O shape
                let is_left = x == 0 && y > 0 && y < height - 1;
                let is_right = x == width - 1 && y > 0 && y < height - 1;
                let is_top = y == 0 && x > 0 && x < width - 1;
                let is_bottom = y == height - 1 && x > 0 && x < width - 1;
                is_left || is_right || is_top || is_bottom
            }
            'P' | 'p' => {
                // Simple P shape
                let is_left = x == 0;
                let is_top = y == 0 && x < width;
                let is_middle = y == height / 2 && x < width;
                let is_right = x == width - 1 && y < height / 2;
                is_left || is_top || is_middle || is_right
            }
            'Q' | 'q' => {
                // Simple Q shape
                let is_left = x == 0 && y > 0 && y < height - 1;
                let is_right = x == width - 1 && y > 0 && y < height - 1;
                let is_top = y == 0 && x > 0 && x < width - 1;
                let is_bottom = y == height - 1 && x > 0 && x < width - 1;
                let is_diagonal = x == y && x > width / 2;
                is_left || is_right || is_top || is_bottom || is_diagonal
            }
            'R' | 'r' => {
                // Simple R shape
                let is_left = x == 0;
                let is_top = y == 0 && x < width;
                let is_middle = y == height / 2 && x < width;
                let is_right = x == width - 1 && y < height / 2;
                let is_diagonal = y == x + height / 2 && x > width / 2;
                is_left || is_top || is_middle || is_right || is_diagonal
            }
            'S' | 's' => {
                // Simple S shape
                let is_top = y == 0 && x > 0;
                let is_middle = y == height / 2 && x > 0 && x < width - 1;
                let is_bottom = y == height - 1 && x < width - 1;
                let is_left_top = x == 0 && y < height / 2;
                let is_right_bottom = x == width - 1 && y > height / 2;
                is_top || is_middle || is_bottom || is_left_top || is_right_bottom
            }
            'T' | 't' => {
                // Simple T shape
                let is_top = y == 0 && x < width;
                let is_middle = x == width / 2 && y < height;
                is_top || is_middle
            }
            'U' | 'u' => {
                // Simple U shape
                let is_left = x == 0 && y < height - 1;
                let is_right = x == width - 1 && y < height - 1;
                let is_bottom = y == height - 1 && x > 0 && x < width - 1;
                is_left || is_right || is_bottom
            }
            'V' | 'v' => {
                // Simple V shape
                let is_left_diagonal = y == 2 * x && x <= width / 2;
                let is_right_diagonal = y == 2 * (width - x) && x >= width / 2;
                is_left_diagonal || is_right_diagonal
            }
            'W' | 'w' => {
                // Simple W shape
                let is_left = x == 0 && y < height;
                let is_right = x == width - 1 && y < height;
                let is_middle_left = x == width / 3 && y >= height / 2;
                let is_middle_right = x == 2 * width / 3 && y >= height / 2;
                is_left || is_right || is_middle_left || is_middle_right
            }
            'X' | 'x' => {
                // Simple X shape
                let is_diagonal1 = x == y;
                let is_diagonal2 = x == width - 1 - y;
                is_diagonal1 || is_diagonal2
            }
            'Y' | 'y' => {
                // Simple Y shape
                let is_left_diagonal = y == 2 * x && x <= width / 2;
                let is_right_diagonal = y == 2 * (width - x) && x >= width / 2;
                let is_vertical = x == width / 2 && y >= height / 2;
                is_left_diagonal || is_right_diagonal || is_vertical
            }
            'Z' | 'z' => {
                // Simple Z shape
                let is_top = y == 0 && x < width;
                let is_bottom = y == height - 1 && x < width;
                let is_diagonal = x == width - 1 - y;
                is_top || is_bottom || is_diagonal
            }
            '0' => {
                // Simple 0 shape
                let is_left = x == 0 && y > 0 && y < height - 1;
                let is_right = x == width - 1 && y > 0 && y < height - 1;
                let is_top = y == 0 && x > 0 && x < width - 1;
                let is_bottom = y == height - 1 && x > 0 && x < width - 1;
                is_left || is_right || is_top || is_bottom
            }
            '1' => {
                // Simple 1 shape
                let is_middle = x == width / 2 && y < height;
                let is_top = y == 0 && x >= width / 2 - 1 && x <= width / 2 + 1;
                is_middle || is_top
            }
            '2' => {
                // Simple 2 shape
                let is_top = y == 0 && x > 0 && x < width - 1;
                let is_middle = y == height / 2 && x > 0 && x < width - 1;
                let is_bottom = y == height - 1 && x > 0 && x < width - 1;
                let is_right_top = x == width - 1 && y < height / 2;
                let is_left_bottom = x == 0 && y > height / 2;
                is_top || is_middle || is_bottom || is_right_top || is_left_bottom
            }
            '3' => {
                // Simple 3 shape
                let is_top = y == 0 && x > 0;
                let is_middle = y == height / 2 && x > 0 && x < width - 1;
                let is_bottom = y == height - 1 && x > 0;
                let is_right = x == width - 1 && y > 0 && y < height - 1;
                is_top || is_middle || is_bottom || is_right
            }
            '4' => {
                // Simple 4 shape
                let is_left = x == 0 && y < height / 2;
                let is_middle = y == height / 2 && x < width;
                let is_right = x == width - 1 && y < height;
                is_left || is_middle || is_right
            }
            '5' => {
                // Simple 5 shape
                let is_top = y == 0 && x < width;
                let is_middle = y == height / 2 && x < width;
                let is_bottom = y == height - 1 && x < width - 1;
                let is_left_top = x == 0 && y < height / 2;
                let is_right_bottom = x == width - 1 && y > height / 2;
                is_top || is_middle || is_bottom || is_left_top || is_right_bottom
            }
            '6' => {
                // Simple 6 shape
                let is_left = x == 0 && y > 0 && y < height - 1;
                let is_top = y == 0 && x > 0 && x < width - 1;
                let is_middle = y == height / 2 && x > 0 && x < width - 1;
                let is_bottom = y == height - 1 && x > 0 && x < width - 1;
                let is_right_bottom = x == width - 1 && y > height / 2;
                is_left || is_top || is_middle || is_bottom || is_right_bottom
            }
            '7' => {
                // Simple 7 shape
                let is_top = y == 0 && x < width;
                let is_diagonal = x == width - 1 - y;
                is_top || is_diagonal
            }
            '8' => {
                // Simple 8 shape
                let is_left = x == 0 && y > 0 && y < height - 1;
                let is_right = x == width - 1 && y > 0 && y < height - 1;
                let is_top = y == 0 && x > 0 && x < width - 1;
                let is_middle = y == height / 2 && x > 0 && x < width - 1;
                let is_bottom = y == height - 1 && x > 0 && x < width - 1;
                is_left || is_right || is_top || is_middle || is_bottom
            }
            '9' => {
                // Simple 9 shape
                let is_left_top = x == 0 && y < height / 2;
                let is_right = x == width - 1 && y > 0 && y < height - 1;
                let is_top = y == 0 && x > 0 && x < width - 1;
                let is_middle = y == height / 2 && x > 0 && x < width - 1;
                let is_bottom = y == height - 1 && x > 0 && x < width - 1;
                is_left_top || is_right || is_top || is_middle || is_bottom
            }
            ' ' => false,
            '|' => {
                // Vertical line for separators
                x >= width / 2 - 1 && x < width / 2 + 1
            }
            '/' => {
                // Diagonal line
                let diagonal = x + y;
                diagonal >= width / 2 + height / 2 - 2 && diagonal <= width / 2 + height / 2 + 2
            }
            _ => {
                // Default pattern for other characters
                let is_border = x < 1 || x >= width - 1 || y < 1 || y >= height - 1;
                let is_fill = (x + y) % 3 == 0;
                is_border || is_fill
            }
        }
    }
}

fn parse_color(color_str: &str) -> Option<Rgba<u8>> {
    if color_str.starts_with('#') && color_str.len() == 7 {
        let hex = &color_str[1..];
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[0..2], 16),
            u8::from_str_radix(&hex[2..4], 16),
            u8::from_str_radix(&hex[4..6], 16),
        ) {
            return Some(Rgba([r, g, b, 255]));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watermark_renderer_creation() {
        let renderer = WatermarkRenderer::new();
        assert!(renderer.is_ok());
    }

    #[test]
    fn test_parse_color() {
        assert_eq!(parse_color("#FF0000"), Some(Rgba([255, 0, 0, 255])));
        assert_eq!(parse_color("#00FF00"), Some(Rgba([0, 255, 0, 255])));
        assert_eq!(parse_color("#0000FF"), Some(Rgba([0, 0, 255, 255])));
        assert_eq!(parse_color("invalid"), None);
    }
}
