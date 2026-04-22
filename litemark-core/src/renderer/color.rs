use crate::error::{CoreError, RenderError};
use image::Rgba;

/// Parse color string (e.g., "#FFFFFF", "#000000", "#RRGGBBAA") to Rgba
///
/// Supports both 6-digit (RGB) and 8-digit (RGBA) hex formats.
pub(super) fn parse_color(color_str: &str) -> Result<Rgba<u8>, CoreError> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
