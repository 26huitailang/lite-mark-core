use crate::layout::{Background, ItemType, Template};
use image::{DynamicImage, Rgba, RgbaImage};
use rusttype::{point, Font, Scale};
use std::collections::HashMap;

pub struct WatermarkRenderer {
    font: Font<'static>,
}

impl WatermarkRenderer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Try to load embedded font
        let font_data = include_bytes!("../../assets/fonts/DejaVuSans.ttf");

        // Validate font data
        if font_data.len() < 100 {
            return Err("Font file appears to be invalid or empty".into());
        }

        let font = Font::try_from_bytes(font_data).ok_or_else(|| {
            format!(
                "Failed to parse font data (size: {} bytes)",
                font_data.len()
            )
        })?;

        Ok(WatermarkRenderer { font })
    }

    pub fn render_watermark(
        &self,
        image: &mut DynamicImage,
        template: &Template,
        variables: &HashMap<String, String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let substituted_template = template.substitute_variables(variables);

        let original_width = image.width();
        let original_height = image.height();

        // Create frame: add bottom border for parameters and logo
        let bottom_frame_height = 100; // Height of the bottom frame area
        let side_padding = 0; // No side padding for now

        // Create new canvas with frame
        let new_width = original_width;
        let new_height = original_height + bottom_frame_height;

        // Create new image with frame
        let mut frame_image = RgbaImage::new(new_width, new_height);

        // Copy original image to the top (centered)
        let original_rgba = image.to_rgba8();
        for y in 0..original_height {
            for x in 0..original_width {
                if x < new_width && y < original_height {
                    frame_image.put_pixel(x, y, *original_rgba.get_pixel(x, y));
                }
            }
        }

        // Render frame background (bottom area)
        let frame_y = original_height;
        self.render_frame_background(&mut frame_image, frame_y, new_width, bottom_frame_height)?;

        // Render logo and parameters in bottom frame
        self.render_frame_content(
            &mut frame_image,
            &substituted_template,
            variables,
            frame_y,
            new_width,
            bottom_frame_height,
        )?;

        // Convert back to DynamicImage
        *image = DynamicImage::ImageRgba8(frame_image);

        Ok(())
    }

    fn render_frame_background(
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

    fn render_frame_content(
        &self,
        image: &mut RgbaImage,
        template: &Template,
        variables: &HashMap<String, String>,
        frame_y: u32,
        width: u32,
        height: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let center_x = width / 2;
        let mut logo_path: Option<String> = None;
        let mut text_items: Vec<&str> = Vec::new();

        // Separate logo and text items
        for item in &template.items {
            match item.item_type {
                ItemType::Logo => {
                    logo_path = Some(item.value.clone());
                }
                ItemType::Text => {
                    text_items.push(&item.value);
                }
            }
        }

        // Render logo in center of bottom frame
        if let Some(ref logo_path) = logo_path {
            let logo_y = frame_y + height / 2 - 15; // Center vertically in frame
            self.render_logo(image, logo_path, center_x as i32, logo_y as i32, 30, 30)?;
        }

        // Render text parameters below logo
        let text_start_y = if logo_path.is_some() {
            frame_y + height / 2 + 20 // Below logo
        } else {
            frame_y + height / 2 - 10 // Center if no logo
        };

        let mut current_y = text_start_y;
        for (i, text) in text_items.iter().enumerate() {
            let substituted_text = self.substitute_text(text, variables);
            let font_size = if i == 0 { 20 } else { 16 }; // Larger font sizes
            let color = Rgba([0, 0, 0, 255]); // Black text

            // Better text centering: estimate text width based on font metrics
            let scale = Scale::uniform(font_size as f32);
            let char_count = substituted_text.len();
            let estimated_width = (char_count as f32 * font_size as f32 * 0.6) as i32; // Rough estimate
            let text_x = center_x as i32 - (estimated_width / 2);

            self.render_text_simple(
                image,
                &substituted_text,
                text_x,
                current_y as i32,
                font_size,
                color,
            );
            current_y += font_size as u32 + 10;
        }

        Ok(())
    }

    fn render_logo(
        &self,
        image: &mut RgbaImage,
        logo_path: &str,
        center_x: i32,
        center_y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Try to load logo image
        if let Ok(logo_img) = image::open(logo_path) {
            let logo_rgba = logo_img.to_rgba8();
            let (logo_w, logo_h) = logo_rgba.dimensions();

            // Scale logo to fit
            let scale = (width as f32 / logo_w as f32).min(height as f32 / logo_h as f32);
            let scaled_w = (logo_w as f32 * scale) as u32;
            let scaled_h = (logo_h as f32 * scale) as u32;

            let start_x = center_x - (scaled_w as i32 / 2);
            let start_y = center_y - (scaled_h as i32 / 2);

            // Draw logo
            for y in 0..scaled_h {
                for x in 0..scaled_w {
                    let src_x = (x as f32 / scale) as u32;
                    let src_y = (y as f32 / scale) as u32;
                    if src_x < logo_w && src_y < logo_h {
                        let pixel = logo_rgba.get_pixel(src_x, src_y);
                        let px = start_x + x as i32;
                        let py = start_y + y as i32;
                        if px >= 0
                            && py >= 0
                            && px < image.width() as i32
                            && py < image.height() as i32
                        {
                            image.put_pixel(px as u32, py as u32, *pixel);
                        }
                    }
                }
            }
        } else {
            // If logo file not found, draw a placeholder
            println!("Logo file not found: {}, using placeholder", logo_path);
            // Draw a simple placeholder rectangle
            let start_x = center_x - width / 2;
            let start_y = center_y - height / 2;
            for dy in 0..height {
                for dx in 0..width {
                    let px = start_x + dx;
                    let py = start_y + dy;
                    if px >= 0 && py >= 0 && px < image.width() as i32 && py < image.height() as i32
                    {
                        let border = dx < 2 || dx >= width - 2 || dy < 2 || dy >= height - 2;
                        let color = if border {
                            Rgba([100, 100, 100, 255])
                        } else {
                            Rgba([200, 200, 200, 255])
                        };
                        image.put_pixel(px as u32, py as u32, color);
                    }
                }
            }
        }

        Ok(())
    }

    fn substitute_text(&self, text: &str, variables: &HashMap<String, String>) -> String {
        let mut result = text.to_string();
        for (key, value) in variables {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, value);
        }
        result
    }

    fn render_text_simple(
        &self,
        image: &mut RgbaImage,
        text: &str,
        x: i32,
        y: i32,
        font_size: u32,
        color: Rgba<u8>,
    ) {
        // Use rusttype for proper font rendering
        let scale = Scale::uniform(font_size as f32);
        let v_metrics = self.font.v_metrics(scale);
        // Position text correctly - y is the top of the text area
        let baseline_y = y as f32 - v_metrics.ascent;
        let offset = point(x as f32, baseline_y);

        // Layout and render glyphs
        let glyphs: Vec<_> = self.font.layout(text, scale, offset).collect();

        let mut glyph_count = 0;
        let mut pixels_drawn = 0;

        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph_count += 1;
                // Build glyph image
                glyph.draw(|px, py, v| {
                    let px = px as i32 + bounding_box.min.x;
                    let py = py as i32 + bounding_box.min.y;

                    if px >= 0 && py >= 0 && px < image.width() as i32 && py < image.height() as i32
                    {
                        let alpha = (v * 255.0) as u8;
                        if alpha > 10 {
                            // Threshold to avoid very faint pixels
                            // Use solid color for better visibility
                            let pixel_color = Rgba([color[0], color[1], color[2], 255]);
                            image.put_pixel(px as u32, py as u32, pixel_color);
                            pixels_drawn += 1;
                        }
                    }
                });
            }
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
        // More reliable character patterns using thicker lines
        match ch {
            'A' | 'a' => {
                // A shape with thick lines
                let center_x = width / 2;
                // Left diagonal (thick)
                let is_left = (y >= 2 * x - 1 && y <= 2 * x + 1 && x <= center_x) || x <= 2;
                // Right diagonal (thick)
                let is_right =
                    (y >= 2 * (width - x) - 1 && y <= 2 * (width - x) + 1 && x >= center_x)
                        || x >= width - 2;
                // Horizontal bar (thick)
                let is_middle = y >= height / 2 - 1
                    && y <= height / 2 + 1
                    && x >= width / 4
                    && x < 3 * width / 4;
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
                // H shape with thicker lines
                let is_left = x < 2;
                let is_right = x >= width - 2;
                let is_middle = y >= height / 2 - 1 && y <= height / 2 + 1 && x < width;
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
                // O shape with thicker lines
                let is_left = x < 2 && y > 1 && y < height - 2;
                let is_right = x >= width - 2 && y > 1 && y < height - 2;
                let is_top = y < 2 && x > 1 && x < width - 2;
                let is_bottom = y >= height - 2 && x > 1 && x < width - 2;
                is_left || is_right || is_top || is_bottom
            }
            'P' | 'p' => {
                // P shape with thicker lines
                let is_left = x < 2;
                let is_top = y < 2 && x < width;
                let is_middle = y >= height / 2 - 1 && y <= height / 2 + 1 && x < width;
                let is_right = x >= width - 2 && y < height / 2;
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
