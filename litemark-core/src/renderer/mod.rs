pub mod color;
mod draw;
mod logo;
mod text;

use crate::layout::{Anchor, FontWeight, ItemType, RenderMode, Template};
use image::{DynamicImage, Rgba, RgbaImage};
use std::collections::HashMap;

pub struct WatermarkRenderer {
    fonts: text::FontSet,
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
    /// ```no_run
    /// use litemark_core::WatermarkRenderer;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // 使用自定义字体
    /// let font_bytes = std::fs::read("custom.ttf")?;
    /// let renderer = WatermarkRenderer::from_font_bytes(Some(&font_bytes))?;
    ///
    /// // 使用默认字体
    /// let renderer = WatermarkRenderer::from_font_bytes(None)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_font_bytes(font_data: Option<&[u8]>) -> Result<Self, Box<dyn std::error::Error>> {
        let regular_font = if let Some(data) = font_data {
            Self::parse_font_data(data)?
        } else {
            Self::load_default_font()?
        };

        Ok(WatermarkRenderer {
            fonts: text::FontSet {
                regular: regular_font,
                bold: None,
            },
        })
    }

    /// 从字节数据创建渲染器，同时指定常规体和粗体字体
    ///
    /// # Arguments
    /// * `regular_data` - 常规体字体的字节数据
    /// * `bold_data` - 粗体字体的字节数据
    pub fn from_font_bytes_with_bold(
        regular_data: Option<&[u8]>,
        bold_data: Option<&[u8]>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let regular_font = if let Some(data) = regular_data {
            Self::parse_font_data(data)?
        } else {
            Self::load_default_font()?
        };

        let bold_font = if let Some(data) = bold_data {
            Some(Self::parse_font_data(data)?)
        } else {
            None
        };

        Ok(WatermarkRenderer {
            fonts: text::FontSet {
                regular: regular_font,
                bold: bold_font,
            },
        })
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

        // Overlay mode renders directly on the original image
        if template.render_mode == RenderMode::Overlay {
            return self.render_overlay(
                image,
                template,
                &substituted_template,
                variables,
                logo_data,
            );
        }

        let original_width = image.width();
        let original_height = image.height();

        // Calculate frame dimensions based on image short edge
        let short_edge = original_width.min(original_height) as f32;

        // Validate and clamp frame_height_ratio (5% to 20%)
        let frame_height_ratio = template.frame_height_ratio.clamp(0.05, 0.20);

        // Calculate frame height with min/max bounds
        let calculated_frame_height = (short_edge * frame_height_ratio) as u32;
        let bottom_frame_height = calculated_frame_height.max(80); // Min 80px

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

        // Render based on render mode
        let frame_y = original_height;
        match template.render_mode {
            RenderMode::BottomFrame => {
                self.render_frame_background(&mut frame_image, frame_y, new_width, bottom_frame_height)?;
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
            }
            RenderMode::GradientFrame => {
                self.render_gradient_background(&mut frame_image, frame_y, new_width, bottom_frame_height)?;
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
            }
            RenderMode::Minimal => {
                self.render_frame_background(&mut frame_image, frame_y, new_width, bottom_frame_height)?;
                self.render_minimal_line(&mut frame_image, frame_y, new_width)?;
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
            }
            RenderMode::Overlay => {
                unreachable!("Overlay mode is handled above");
            }
        }

        // Convert back to DynamicImage
        *image = DynamicImage::ImageRgba8(frame_image);

        Ok(())
    }

    fn render_overlay(
        &self,
        image: &mut DynamicImage,
        original_template: &Template,
        substituted_template: &Template,
        _variables: &HashMap<String, String>,
        logo_data: Option<&[u8]>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let width = image.width();
        let height = image.height();
        let img = image.to_rgba8();
        let mut rgba_image = img;

        // Collect text items
        let mut text_items: Vec<(String, f32, Option<String>, Option<FontWeight>)> = Vec::new();
        let mut has_logo = false;

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
                    let text = &subst_item.value;
                    if text.contains('{') && text.contains('}') {
                        continue;
                    }

                    let font_size = if orig_item.font_size_ratio > 0.0 {
                        let short_edge = width.min(height) as f32;
                        let base_font = (short_edge * 0.042).max(20.0);
                        base_font * orig_item.font_size_ratio
                    } else if i == 0 {
                        let short_edge = width.min(height) as f32;
                        let base_font = (short_edge * 0.042).max(20.0);
                        base_font * original_template.primary_font_ratio
                    } else {
                        let short_edge = width.min(height) as f32;
                        let base_font = (short_edge * 0.042).max(20.0);
                        base_font * original_template.secondary_font_ratio
                    };
                    let font_size = font_size.max(10.0);

                    text_items.push((
                        text.clone(),
                        font_size,
                        orig_item.color.clone(),
                        orig_item.weight.clone(),
                    ));
                }
            }
        }

        // Calculate overlay box dimensions (scale with image size)
        let padding = (height as f32 * 0.006).max(6.0).min(20.0) as u32;
        let line_spacing = (height as f32 * 0.002).max(3.0).min(8.0) as u32;

        let mut max_text_width = 0.0f32;
        let mut total_text_height = 0.0f32;
        for (text, font_size, _, weight) in &text_items {
            let w = self.text_width(text, *font_size as u32, weight.as_ref());
            max_text_width = max_text_width.max(w);
            total_text_height += *font_size;
        }
        if text_items.len() > 1 {
            total_text_height += (text_items.len() - 1) as f32 * line_spacing as f32;
        }

        let box_width = (max_text_width + padding as f32 * 2.0).ceil() as u32;
        let box_height = (total_text_height + padding as f32 * 2.0).ceil() as u32;

        // Position based on anchor
        let margin = (width.min(height) as f32 * 0.008).max(6.0).min(24.0) as u32;
        let box_x = match original_template.anchor {
            Anchor::BottomLeft => margin,
            Anchor::BottomCenter => (width.saturating_sub(box_width)) / 2,
            _ => width.saturating_sub(box_width + margin),
        };
        let box_y = height.saturating_sub(box_height + margin);

        // Draw background or gradient mask
        if let Some(bg) = &original_template.background {
            let color = if let Some(color_str) = &bg.color {
                color::parse_color(color_str).unwrap_or(Rgba([0, 0, 0, 255]))
            } else {
                Rgba([0, 0, 0, 255])
            };
            let alpha = (color[3] as f32 * bg.opacity.clamp(0.0, 1.0)).min(255.0) as u8;
            let bg_color = Rgba([color[0], color[1], color[2], alpha]);
            let radius = bg.radius.unwrap_or(12);
            self.render_rounded_rect(&mut rgba_image, box_x, box_y, box_width, box_height, radius, bg_color);
        } else {
            // No background box: draw a subtle bottom gradient mask for readability
            let mask_height = (box_height + margin * 2).min(height);
            let mask_y_start = height.saturating_sub(mask_height);
            for dy in 0..mask_height {
                let y = mask_y_start + dy;
                if y >= height { continue; }
                let progress = dy as f32 / mask_height as f32;
                let alpha = (progress * progress * 180.0) as u8;
                let overlay = Rgba([0, 0, 0, alpha]);
                for x in 0..width {
                    let original = *rgba_image.get_pixel(x, y);
                    let blended = draw::blend_pixel(original, overlay);
                    rgba_image.put_pixel(x, y, blended);
                }
            }
        }

        // Render text
        let mut current_y = box_y + padding + 4;
        for (text, font_size, color_opt, weight_opt) in &text_items {
            let color = if let Some(color_str) = color_opt {
                color::parse_color(color_str).unwrap_or(Rgba([255, 255, 255, 255]))
            } else {
                Rgba([255, 255, 255, 255])
            };

            let text_w = self.text_width(text, *font_size as u32, weight_opt.as_ref());
            let x = match original_template.anchor {
                Anchor::BottomCenter | Anchor::Center => {
                    (box_x as f32 + (box_width as f32 - text_w) / 2.0) as i32
                }
                Anchor::BottomLeft => (box_x + padding) as i32,
                _ => (box_x + padding) as i32,
            };

            self.render_text_simple(
                &mut rgba_image,
                text,
                x,
                current_y as i32,
                *font_size as u32,
                color,
                weight_opt.as_ref(),
            );
            current_y += *font_size as u32 + line_spacing;
        }

        // Render logo if present
        if has_logo && logo_data.is_some() {
            let logo_size = 32u32;
            let logo_x = match original_template.anchor {
                Anchor::BottomCenter | Anchor::Center => {
                    (box_x + box_width / 2 - logo_size / 2) as i32
                }
                _ => (box_x + padding) as i32,
            };
            let logo_y = (box_y + padding) as i32;
            self.render_logo_from_bytes(
                &mut rgba_image,
                logo_data.unwrap(),
                logo_x + logo_size as i32 / 2,
                logo_y + logo_size as i32 / 2,
                logo_size,
            )?;
        }

        *image = DynamicImage::ImageRgba8(rgba_image);
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
        // (text, font_size, color, weight)
        let mut left_column_items: Vec<(String, f32, Option<String>, Option<FontWeight>)> =
            Vec::new();
        let mut right_column_items: Vec<(String, f32, Option<String>, Option<FontWeight>)> =
            Vec::new();
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
                        frame_height_f32 * orig_item.font_size_ratio
                    } else if i == 0 {
                        frame_height_f32 * original_template.primary_font_ratio
                    } else {
                        frame_height_f32 * original_template.secondary_font_ratio
                    };

                    let font_size = font_size.max(10.0);

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
                            orig_item.weight.clone(),
                        ));
                    } else {
                        right_column_items.push((
                            substituted_text.clone(),
                            font_size,
                            orig_item.color.clone(),
                            orig_item.weight.clone(),
                        ));
                    }
                }
            }
        }

        // Calculate padding
        let padding = (frame_height_f32 * original_template.padding_ratio) as u32;
        let padding = padding.max(5);

        // Layout parameters
        let column1_x = padding;
        let logo_height = (height as f32 / 3.0) as u32;
        let estimated_logo_width = (logo_height as f32 * 2.5) as u32;

        // Right-aligned columns (use saturating_sub to prevent overflow on tiny images)
        let column4_x_end = width.saturating_sub(padding);
        let estimated_column4_width = width / 3;
        let column4_x = column4_x_end.saturating_sub(estimated_column4_width);

        let separator_x = column4_x.saturating_sub(padding * 3);
        let logo_center_x = separator_x
            .saturating_sub(padding * 3)
            .saturating_sub(estimated_logo_width / 2);

        // Render Column 1: Author, Camera, Date (left side)
        let mut current_y = frame_y + padding * 2;
        for (text, font_size, color_opt, weight_opt) in left_column_items.iter() {
            let color = if let Some(color_str) = color_opt {
                color::parse_color(color_str).unwrap_or(Rgba([0, 0, 0, 255]))
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
                weight_opt.as_ref(),
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
            Rgba([232, 232, 232, 255]), // #E8E8E8 - lighter separator
        );

        // Render Column 4: Other EXIF info (right side, right-aligned)
        current_y = frame_y + padding * 2;
        for (text, font_size, color_opt, weight_opt) in right_column_items.iter() {
            let color = if let Some(color_str) = color_opt {
                color::parse_color(color_str).unwrap_or(Rgba([0, 0, 0, 255]))
            } else {
                Rgba([0, 0, 0, 255])
            };

            // Calculate text width for right alignment
            let text_w = self.text_width(text, *font_size as u32, weight_opt.as_ref());
            let x = column4_x_end as f32 - text_w;
            let x = x.max(column4_x as f32) as i32; // Don't overflow into separator area

            self.render_text_simple(
                image,
                text,
                x,
                current_y as i32,
                *font_size as u32,
                color,
                weight_opt.as_ref(),
            );
            current_y += *font_size as u32 + padding / 3;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::{Anchor, TemplateItem};

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
            image::Rgb([128, 128, 128])
        }));

        let template = Template {
            name: "Test".to_string(),
            anchor: Anchor::BottomLeft,
            padding: 20,
            items: vec![TemplateItem {
                item_type: ItemType::Text,
                value: "{Author}".to_string(),
                font_size: 16,
                font_size_ratio: 0.20,
                weight: Some(FontWeight::Bold),
                color: Some("#000000".to_string()),
            }],
            background: None,
            frame_height_ratio: 0.10,
            logo_size_ratio: 0.35,
            primary_font_ratio: 0.20,
            secondary_font_ratio: 0.14,
            padding_ratio: 0.10,
            render_mode: RenderMode::BottomFrame,
        };

        let mut variables = HashMap::new();
        variables.insert("Author".to_string(), "Test Author".to_string());

        let result =
            renderer.render_watermark_with_logo_bytes(&mut image, &template, &variables, None);
        assert!(result.is_ok());

        // 检查图像尺寸是否增加了边框
        assert!(image.height() > 600);
    }
}
