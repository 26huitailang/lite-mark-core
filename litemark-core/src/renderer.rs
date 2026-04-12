use crate::layout::{ItemType, Template};
use image::{DynamicImage, Rgba, RgbaImage};
use rusttype::{point, Font, Scale};
use std::collections::HashMap;

/// Parse color string (e.g., "#FFFFFF" or "#000000") to Rgba
fn parse_color(color_str: &str) -> Result<Rgba<u8>, Box<dyn std::error::Error>> {
    let color_str = color_str.trim_start_matches('#');
    if color_str.len() != 6 {
        return Err("Invalid color format".into());
    }

    let r = u8::from_str_radix(&color_str[0..2], 16)?;
    let g = u8::from_str_radix(&color_str[2..4], 16)?;
    let b = u8::from_str_radix(&color_str[4..6], 16)?;

    Ok(Rgba([r, g, b, 255]))
}

pub struct WatermarkRenderer {
    font: Font<'static>,
}

impl WatermarkRenderer {
    /// 使用默认嵌入字体创建渲染器
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Self::from_font_bytes(None)
    }

    /// 从字节数据创建渲染器（Core 接口）
    ///
    /// # Arguments
    /// * `font_data` - 字体文件的字节数据，None 表示使用默认字体
    ///
    /// # Examples
    /// ```
    /// // 使用自定义字体
    /// let font_bytes = std::fs::read("custom.ttf")?;
    /// let renderer = WatermarkRenderer::from_font_bytes(Some(&font_bytes))?;
    ///
    /// // 使用默认字体
    /// let renderer = WatermarkRenderer::from_font_bytes(None)?;
    /// ```
    pub fn from_font_bytes(font_data: Option<&[u8]>) -> Result<Self, Box<dyn std::error::Error>> {
        let font = if let Some(data) = font_data {
            // 使用自定义字体数据
            if data.len() < 100 {
                return Err("Font data appears to be invalid or empty".into());
            }

            // Leak the boxed data to obtain a 'static slice reference for rusttype
            let leaked: &'static [u8] = Box::leak(data.to_vec().into_boxed_slice());
            Font::try_from_bytes(leaked).ok_or_else(|| {
                format!("Failed to parse font data (size: {} bytes)", leaked.len())
            })?
        } else {
            // 使用默认嵌入字体
            Self::load_default_font()?
        };

        Ok(WatermarkRenderer { font })
    }

    /// 加载默认嵌入字体
    fn load_default_font() -> Result<Font<'static>, Box<dyn std::error::Error>> {
        // Default font: Source Han Sans CN (embedded at compile time)
        // Supports both Chinese and English characters
        let font_data = include_bytes!("../../assets/fonts/SourceHanSansCN-Regular.otf");

        // Validate font data
        if font_data.len() < 100 {
            return Err("Default font file appears to be invalid or empty".into());
        }

        Ok(Font::try_from_bytes(font_data).ok_or_else(|| {
            format!(
                "Failed to parse default font data (size: {} bytes)",
                font_data.len()
            )
        })?)
    }

    /// 渲染水印到图像（Core 接口，支持字节数组 Logo）
    ///
    /// # Arguments
    /// * `image` - 要添加水印的图像
    /// * `template` - 水印模板
    /// * `variables` - 变量替换映射
    /// * `logo_data` - 可选的 Logo 图像字节数据
    pub fn render_watermark_with_logo_bytes(
        &self,
        image: &mut DynamicImage,
        template: &Template,
        variables: &HashMap<String, String>,
        logo_data: Option<&[u8]>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let substituted_template = template.substitute_variables(variables);

        let original_width = image.width();
        let original_height = image.height();

        // Calculate frame dimensions based on image short edge
        let short_edge = original_width.min(original_height) as f32;

        // Validate and clamp frame_height_ratio (5% to 20%)
        let frame_height_ratio = template.frame_height_ratio.clamp(0.05, 0.20);

        // Calculate frame height with min/max bounds
        let calculated_frame_height = (short_edge * frame_height_ratio) as u32;
        let bottom_frame_height = calculated_frame_height.clamp(80, 800); // Min 80px, Max 800px

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
            template,
            &substituted_template,
            variables,
            frame_y,
            new_width,
            bottom_frame_height,
            logo_data,
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
        original_template: &Template,
        substituted_template: &Template,
        _variables: &HashMap<String, String>,
        frame_y: u32,
        width: u32,
        height: u32,
        logo_data: Option<&[u8]>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Calculate dynamic sizes based on frame height
        let frame_height_f32 = height as f32;

        // Separate logo and text items, and classify text into left/right columns
        let mut left_column_items: Vec<(String, f32, Option<String>)> = Vec::new();
        let mut right_column_items: Vec<(String, f32, Option<String>)> = Vec::new();
        let mut has_logo = false;

        // Priority variables for left column
        let left_priority = ["Author", "Camera", "DateTime"];

        // Use original template items for classification, substituted items for display
        for (i, (orig_item, subst_item)) in original_template
            .items
            .iter()
            .zip(substituted_template.items.iter())
            .enumerate()
        {
            match orig_item.item_type {
                ItemType::Logo => {
                    has_logo = true;
                }
                ItemType::Text => {
                    let substituted_text = &subst_item.value;

                    // Skip this item if it still contains unreplaced placeholders
                    if substituted_text.contains('{') && substituted_text.contains('}') {
                        continue;
                    }

                    // Determine font size
                    let font_size = if orig_item.font_size_ratio > 0.0 {
                        (frame_height_f32 * orig_item.font_size_ratio) as f32
                    } else if i == 0 {
                        frame_height_f32 * original_template.primary_font_ratio
                    } else {
                        frame_height_f32 * original_template.secondary_font_ratio
                    };

                    let font_size = font_size.max(10.0).min(100.0);

                    // Classify into columns
                    let is_left_column = left_priority.iter().any(|&var| {
                        let placeholder = format!("{{{}}}", var);
                        orig_item.value.trim() == placeholder
                    });

                    if is_left_column {
                        left_column_items.push((
                            substituted_text.clone(),
                            font_size,
                            orig_item.color.clone(),
                        ));
                    } else {
                        right_column_items.push((
                            substituted_text.clone(),
                            font_size,
                            orig_item.color.clone(),
                        ));
                    }
                }
            }
        }

        // Calculate padding
        let padding = (frame_height_f32 * original_template.padding_ratio) as u32;
        let padding = padding.max(5).min(50);

        // Layout parameters
        let column1_x = padding;
        let logo_height = (height as f32 / 3.0) as u32;
        let estimated_logo_width = (logo_height as f32 * 2.5) as u32;

        // Right-aligned columns (use saturating_sub to prevent overflow on tiny images)
        let column4_x_end = width.saturating_sub(padding);
        let estimated_column4_width = (width / 3) as u32;
        let column4_x = column4_x_end.saturating_sub(estimated_column4_width);

        let separator_x = column4_x.saturating_sub(padding * 3);
        let logo_center_x = separator_x.saturating_sub(padding * 3).saturating_sub(estimated_logo_width / 2);

        // Render Column 1: Author, Camera, Date (left side)
        let mut current_y = frame_y + padding * 2;
        for (text, font_size, color_opt) in left_column_items.iter() {
            let color = if let Some(color_str) = color_opt {
                parse_color(color_str).unwrap_or(Rgba([0, 0, 0, 255]))
            } else {
                Rgba([0, 0, 0, 255])
            };

            self.render_text_simple(
                image,
                text,
                column1_x as i32,
                current_y as i32,
                *font_size as u32,
                color,
            );
            current_y += *font_size as u32 + padding / 3;
        }

        // Render Column 2: Logo (center-left, vertically centered)
        if has_logo && logo_data.is_some() {
            let logo_y = frame_y + height / 2;
            self.render_logo_from_bytes(
                image,
                logo_data.unwrap(),
                logo_center_x as i32,
                logo_y as i32,
                logo_height,
            )?;
        }

        // Render Column 3: Vertical separator line
        self.render_vertical_line(
            image,
            separator_x,
            frame_y + padding,
            height - padding * 2,
            Rgba([200, 200, 200, 255]),
        );

        // Render Column 4: Other EXIF info (right side)
        current_y = frame_y + padding * 2;
        for (text, font_size, color_opt) in right_column_items.iter() {
            let color = if let Some(color_str) = color_opt {
                parse_color(color_str).unwrap_or(Rgba([0, 0, 0, 255]))
            } else {
                Rgba([0, 0, 0, 255])
            };

            self.render_text_simple(
                image,
                text,
                column4_x as i32,
                current_y as i32,
                *font_size as u32,
                color,
            );
            current_y += *font_size as u32 + padding / 3;
        }

        Ok(())
    }

    /// 从字节数据渲染 Logo
    fn render_logo_from_bytes(
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

        // Draw logo with alpha blending
        for y in 0..scaled_h {
            for x in 0..scaled_w {
                let src_x = ((x as f32 / scaled_w as f32) * logo_w as f32) as u32;
                let src_y = ((y as f32 / scaled_h as f32) * logo_h as f32) as u32;
                if src_x < logo_w && src_y < logo_h {
                    let logo_pixel = logo_rgba.get_pixel(src_x, src_y);
                    let px = start_x + x as i32;
                    let py = start_y + y as i32;
                    if px >= 0 && py >= 0 && px < image.width() as i32 && py < image.height() as i32
                    {
                        let px_u32 = px as u32;
                        let py_u32 = py as u32;

                        let alpha = logo_pixel[3] as f32 / 255.0;

                        if alpha > 0.01 {
                            let bg_pixel = image.get_pixel(px_u32, py_u32);

                            let r = ((logo_pixel[0] as f32 * alpha)
                                + (bg_pixel[0] as f32 * (1.0 - alpha)))
                                as u8;
                            let g = ((logo_pixel[1] as f32 * alpha)
                                + (bg_pixel[1] as f32 * (1.0 - alpha)))
                                as u8;
                            let b = ((logo_pixel[2] as f32 * alpha)
                                + (bg_pixel[2] as f32 * (1.0 - alpha)))
                                as u8;

                            image.put_pixel(px_u32, py_u32, Rgba([r, g, b, 255]));
                        }
                    }
                }
            }
        }

        Ok(())
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
        let scale = Scale::uniform(font_size as f32);
        let v_metrics = self.font.v_metrics(scale);
        let baseline_y = y as f32 + v_metrics.ascent;
        let offset = point(x as f32, baseline_y);

        let glyphs: Vec<_> = self.font.layout(text, scale, offset).collect();

        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|px, py, v| {
                    let px = px as i32 + bounding_box.min.x;
                    let py = py as i32 + bounding_box.min.y;

                    if px >= 0 && py >= 0 && px < image.width() as i32 && py < image.height() as i32
                    {
                        let alpha = (v * 255.0) as u8;
                        if alpha > 10 {
                            let pixel_color = Rgba([color[0], color[1], color[2], 255]);
                            image.put_pixel(px as u32, py as u32, pixel_color);
                        }
                    }
                });
            }
        }
    }

    fn render_vertical_line(
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
                if x + 1 < image.width() {
                    image.put_pixel(x + 1, y, color);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::{Anchor, TemplateItem, FontWeight};
    use image::Rgb;

    #[test]
    fn test_parse_color() {
        let color = parse_color("#FF0000").unwrap();
        assert_eq!(color, Rgba([255, 0, 0, 255]));

        let color = parse_color("#00FF00").unwrap();
        assert_eq!(color, Rgba([0, 255, 0, 255]));

        let color = parse_color("#0000FF").unwrap();
        assert_eq!(color, Rgba([0, 0, 255, 255]));
    }

    #[test]
    fn test_renderer_creation() {
        let renderer = WatermarkRenderer::new();
        assert!(renderer.is_ok());

        let renderer = WatermarkRenderer::from_font_bytes(None);
        assert!(renderer.is_ok());
    }

    #[test]
    fn test_render_watermark_basic() {
        let renderer = WatermarkRenderer::new().unwrap();
        let mut image = DynamicImage::ImageRgb8(image::RgbImage::from_fn(800, 600, |_x, _y| {
            Rgb([128, 128, 128])
        }));

        let template = Template {
            name: "Test".to_string(),
            anchor: Anchor::BottomLeft,
            padding: 20,
            items: vec![
                TemplateItem {
                    item_type: ItemType::Text,
                    value: "{Author}".to_string(),
                    font_size: 16,
                    font_size_ratio: 0.20,
                    weight: Some(FontWeight::Bold),
                    color: Some("#000000".to_string()),
                },
            ],
            background: None,
            frame_height_ratio: 0.10,
            logo_size_ratio: 0.35,
            primary_font_ratio: 0.20,
            secondary_font_ratio: 0.14,
            padding_ratio: 0.10,
        };

        let mut variables = HashMap::new();
        variables.insert("Author".to_string(), "Test Author".to_string());

        let result = renderer.render_watermark_with_logo_bytes(&mut image, &template, &variables, None);
        assert!(result.is_ok());

        // 检查图像尺寸是否增加了边框
        assert!(image.height() > 600);
    }
}
