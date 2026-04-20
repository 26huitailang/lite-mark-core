use crate::error::{CoreError, FontError, RenderError};
use crate::layout::{FontWeight, ItemType, RenderMode, Template};
use ab_glyph::{Font, FontRef, Glyph, Point, PxScale, ScaleFont};
use image::{DynamicImage, Rgba, RgbaImage};
use std::collections::HashMap;

/// Parse color string (e.g., "#FFFFFF", "#000000", "#RRGGBBAA") to Rgba
///
/// Supports both 6-digit (RGB) and 8-digit (RGBA) hex formats.
fn parse_color(color_str: &str) -> Result<Rgba<u8>, CoreError> {
    let color_str = color_str.trim_start_matches('#');
    let len = color_str.len();
    if len != 6 && len != 8 {
        return Err(RenderError::InvalidColor(color_str.to_string()).into());
    }

    let r = u8::from_str_radix(&color_str[0..2], 16)
        .map_err(|_| RenderError::InvalidColor(color_str.to_string()))?;
    let g = u8::from_str_radix(&color_str[2..4], 16)
        .map_err(|_| RenderError::InvalidColor(color_str.to_string()))?;
    let b = u8::from_str_radix(&color_str[4..6], 16)
        .map_err(|_| RenderError::InvalidColor(color_str.to_string()))?;
    let a = if len == 8 {
        u8::from_str_radix(&color_str[6..8], 16)
            .map_err(|_| RenderError::InvalidColor(color_str.to_string()))?
    } else {
        255
    };

    Ok(Rgba([r, g, b, a]))
}

/// 字重字体集合
struct FontSet {
    regular: FontRef<'static>,
    bold: Option<FontRef<'static>>,
}

pub struct WatermarkRenderer {
    fonts: FontSet,
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
            fonts: FontSet {
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
            fonts: FontSet {
                regular: regular_font,
                bold: bold_font,
            },
        })
    }

    fn parse_font_data(data: &[u8]) -> Result<FontRef<'static>, CoreError> {
        if data.len() < 100 {
            return Err(FontError::InvalidData { size: data.len() }.into());
        }

        let leaked: &'static [u8] = Box::leak(data.to_vec().into_boxed_slice());
        FontRef::try_from_slice(leaked).map_err(|e| {
            FontError::ParseFailed {
                reason: format!("font data parse error: {} (size: {} bytes)", e, leaked.len()),
            }
            .into()
        })
    }

    /// 加载默认嵌入字体
    fn load_default_font() -> Result<FontRef<'static>, CoreError> {
        // Default font: Source Han Sans CN (embedded at compile time)
        // Supports both Chinese and English characters
        let font_data = include_bytes!("../../assets/fonts/SourceHanSansCN-Regular.otf");

        if font_data.len() < 100 {
            return Err(FontError::InvalidData {
                size: font_data.len(),
            }
            .into());
        }

        FontRef::try_from_slice(font_data).map_err(|e| {
            FontError::ParseFailed {
                reason: format!("default font parse error: {} (size: {} bytes)", e, font_data.len()),
            }
            .into()
        })
    }

    /// 根据字重选择字体
    fn select_font(&self, weight: Option<&FontWeight>) -> &FontRef<'static> {
        match weight {
            Some(FontWeight::Bold) => self.fonts.bold.as_ref().unwrap_or(&self.fonts.regular),
            _ => &self.fonts.regular,
        }
    }

    /// 计算文字像素宽度
    fn text_width(&self, text: &str, font_size: u32, weight: Option<&FontWeight>) -> f32 {
        let font = self.select_font(weight);
        let scale = PxScale::from(font_size as f32);
        let scaled_font = font.as_scaled(scale);

        text.chars()
            .map(|c| scaled_font.h_advance(font.glyph_id(c)))
            .sum()
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

    fn render_gradient_background(
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
                    let blended = Self::blend_pixel(original, overlay);
                    image.put_pixel(x, frame_y + y, blended);
                }
            }
        }

        Ok(())
    }

    fn render_minimal_line(
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
                        64.0 * orig_item.font_size_ratio
                    } else if i == 0 {
                        64.0 * original_template.primary_font_ratio
                    } else {
                        64.0 * original_template.secondary_font_ratio
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

        // Calculate overlay box dimensions
        let padding = 24u32;
        let line_spacing = 8u32;

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
        let radius = 12u32;

        // Position: bottom-right with margin
        let margin = 32u32;
        let box_x = width.saturating_sub(box_width + margin);
        let box_y = height.saturating_sub(box_height + margin);

        // Draw rounded rectangle background
        let bg_color = Rgba([0, 0, 0, 160]); // Semi-transparent black
        self.render_rounded_rect(&mut rgba_image, box_x, box_y, box_width, box_height, radius, bg_color);

        // Render text inside overlay
        let mut current_y = box_y + padding + 4; // Slight optical adjustment
        for (text, font_size, color_opt, weight_opt) in &text_items {
            let color = if let Some(color_str) = color_opt {
                parse_color(color_str).unwrap_or(Rgba([255, 255, 255, 255]))
            } else {
                Rgba([255, 255, 255, 255])
            };

            self.render_text_simple(
                &mut rgba_image,
                text,
                (box_x + padding) as i32,
                current_y as i32,
                *font_size as u32,
                color,
                weight_opt.as_ref(),
            );
            current_y += *font_size as u32 + line_spacing;
        }

        // Render logo if present (top-left of overlay box)
        if has_logo && logo_data.is_some() {
            let logo_size = 32u32;
            let logo_x = (box_x + padding) as i32;
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

    fn render_rounded_rect(
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
                    let blended = Self::blend_pixel(*bg, color);
                    image.put_pixel(px, py, blended);
                }
            }
        }
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
                parse_color(color_str).unwrap_or(Rgba([0, 0, 0, 255]))
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

                        image.put_pixel(px_u32, py_u32, Rgba([r, g, b, 255]));
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
        weight: Option<&FontWeight>,
    ) {
        let font = self.select_font(weight);
        let scale = PxScale::from(font_size as f32);
        let scaled_font = font.as_scaled(scale);

        let mut glyph_x = x as f32;
        let baseline_y = y as f32 + scaled_font.ascent();

        for c in text.chars() {
            let glyph_id = font.glyph_id(c);
            let glyph = Glyph {
                id: glyph_id,
                scale,
                position: Point {
                    x: glyph_x,
                    y: baseline_y,
                },
            };

            if let Some(outlined) = scaled_font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                outlined.draw(|px, py, v| {
                    let px = px as i32 + bounds.min.x as i32;
                    let py = py as i32 + bounds.min.y as i32;

                    if px >= 0
                        && py >= 0
                        && px < image.width() as i32
                        && py < image.height() as i32
                        && v > 0.01
                    {
                        let px_u32 = px as u32;
                        let py_u32 = py as u32;
                        let bg = image.get_pixel(px_u32, py_u32);

                        let r =
                            ((color[0] as f32 * v) + (bg[0] as f32 * (1.0 - v))) as u8;
                        let g =
                            ((color[1] as f32 * v) + (bg[1] as f32 * (1.0 - v))) as u8;
                        let b =
                            ((color[2] as f32 * v) + (bg[2] as f32 * (1.0 - v))) as u8;

                        image.put_pixel(px_u32, py_u32, Rgba([r, g, b, 255]));
                    }
                });
            }

            glyph_x += scaled_font.h_advance(glyph_id);
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
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::{Anchor, TemplateItem};
    use image::Rgb;

    #[test]
    fn test_parse_color_rgb() {
        let color = parse_color("#FF0000").unwrap();
        assert_eq!(color, Rgba([255, 0, 0, 255]));

        let color = parse_color("#00FF00").unwrap();
        assert_eq!(color, Rgba([0, 255, 0, 255]));

        let color = parse_color("#0000FF").unwrap();
        assert_eq!(color, Rgba([0, 0, 255, 255]));
    }

    #[test]
    fn test_parse_color_rgba() {
        let color = parse_color("#FF000080").unwrap();
        assert_eq!(color, Rgba([255, 0, 0, 128]));

        let color = parse_color("#00000000").unwrap();
        assert_eq!(color, Rgba([0, 0, 0, 0]));
    }

    #[test]
    fn test_parse_color_invalid() {
        assert!(parse_color("#GGGGGG").is_err());
        assert!(parse_color("#FFF").is_err());
        assert!(parse_color("not-a-color").is_err());
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
