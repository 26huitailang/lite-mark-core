use crate::error::{CoreError, FontError};
use crate::layout::FontWeight;
use ab_glyph::{Font, FontRef, Glyph, Point, PxScale, ScaleFont};
use image::{Rgba, RgbaImage};

/// 字重字体集合
pub(super) struct FontSet {
    pub(super) regular: FontRef<'static>,
    pub(super) bold: Option<FontRef<'static>>,
}

impl super::WatermarkRenderer {
    pub(super) fn parse_font_data(data: &[u8]) -> Result<FontRef<'static>, CoreError> {
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
    pub(super) fn load_default_font() -> Result<FontRef<'static>, CoreError> {
        // Default font: Source Han Sans CN (embedded at compile time)
        // Supports both Chinese and English characters
        let font_data = include_bytes!("../../../assets/fonts/SourceHanSansCN-Regular.otf");

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
    pub(super) fn select_font(&self, weight: Option<&FontWeight>) -> &FontRef<'static> {
        match weight {
            Some(FontWeight::Bold) => self.fonts.bold.as_ref().unwrap_or(&self.fonts.regular),
            _ => &self.fonts.regular,
        }
    }

    /// 计算文字像素宽度
    pub(super) fn text_width(&self, text: &str, font_size: u32, weight: Option<&FontWeight>) -> f32 {
        let font = self.select_font(weight);
        let scale = PxScale::from(font_size as f32);
        let scaled_font = font.as_scaled(scale);

        text.chars()
            .map(|c| scaled_font.h_advance(font.glyph_id(c)))
            .sum()
    }

    pub(super) fn render_text_simple(
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
}
