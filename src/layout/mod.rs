use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Default values for template scaling ratios
fn default_frame_height_ratio() -> f32 {
    0.10
}

fn default_logo_size_ratio() -> f32 {
    0.35
}

fn default_primary_font_ratio() -> f32 {
    0.20
}

fn default_secondary_font_ratio() -> f32 {
    0.14
}

fn default_padding_ratio() -> f32 {
    0.10
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub anchor: Anchor,
    pub padding: u32,
    pub items: Vec<TemplateItem>,
    pub background: Option<Background>,
    /// Frame height as ratio of image short edge (default: 0.10 = 10%)
    #[serde(default = "default_frame_height_ratio")]
    pub frame_height_ratio: f32,
    /// Logo size as ratio of frame height (default: 0.35 = 35%)
    #[serde(default = "default_logo_size_ratio")]
    pub logo_size_ratio: f32,
    /// Primary font size as ratio of frame height (default: 0.20 = 20%)
    #[serde(default = "default_primary_font_ratio")]
    pub primary_font_ratio: f32,
    /// Secondary font size as ratio of frame height (default: 0.14 = 14%)
    #[serde(default = "default_secondary_font_ratio")]
    pub secondary_font_ratio: f32,
    /// Padding as ratio of frame height (default: 0.10 = 10%)
    #[serde(default = "default_padding_ratio")]
    pub padding_ratio: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Anchor {
    #[serde(rename = "top-left")]
    TopLeft,
    #[serde(rename = "top-right")]
    TopRight,
    #[serde(rename = "bottom-left")]
    BottomLeft,
    #[serde(rename = "bottom-right")]
    BottomRight,
    #[serde(rename = "center")]
    Center,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateItem {
    #[serde(rename = "type")]
    pub item_type: ItemType,
    pub value: String,
    #[serde(default)]
    pub font_size: u32,
    /// Font size as ratio of frame height (overrides font_size if set > 0)
    #[serde(default)]
    pub font_size_ratio: f32,
    pub weight: Option<FontWeight>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "logo")]
    Logo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FontWeight {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "bold")]
    Bold,
    #[serde(rename = "light")]
    Light,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Background {
    #[serde(rename = "type")]
    pub bg_type: BackgroundType,
    pub opacity: f32,
    pub radius: Option<u32>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackgroundType {
    #[serde(rename = "rect")]
    Rectangle,
    #[serde(rename = "circle")]
    Circle,
}

impl Template {
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn substitute_variables(&self, variables: &HashMap<String, String>) -> Self {
        let mut template = self.clone();

        for item in &mut template.items {
            item.value = substitute_text(&item.value, variables);
        }

        template
    }
}

fn substitute_text(text: &str, variables: &HashMap<String, String>) -> String {
    let mut result = text.to_string();

    for (key, value) in variables {
        let placeholder = format!("{{{}}}", key);
        result = result.replace(&placeholder, value);
    }

    result
}

pub fn create_builtin_templates() -> Vec<Template> {
    vec![
        Template {
            name: "ClassicParam".to_string(),
            anchor: Anchor::BottomLeft,
            padding: 0,
            items: vec![
                TemplateItem {
                    item_type: ItemType::Logo,
                    value: "".to_string(), // Logo path will be set by user or default
                    font_size: 0,
                    font_size_ratio: 0.0,
                    weight: None,
                    color: None,
                },
                TemplateItem {
                    item_type: ItemType::Text,
                    value: "{Author}".to_string(),
                    font_size: 16,
                    font_size_ratio: 0.20, // 20% of frame height
                    weight: Some(FontWeight::Bold),
                    color: Some("#000000".to_string()),
                },
                TemplateItem {
                    item_type: ItemType::Text,
                    value: "{Aperture} | ISO {ISO} | {Shutter}".to_string(),
                    font_size: 12,
                    font_size_ratio: 0.14, // 14% of frame height
                    weight: Some(FontWeight::Normal),
                    color: Some("#000000".to_string()),
                },
            ],
            background: None, // Frame background is handled separately
            frame_height_ratio: 0.10,
            logo_size_ratio: 0.35,
            primary_font_ratio: 0.20,
            secondary_font_ratio: 0.14,
            padding_ratio: 0.10,
        },
        Template {
            name: "Modern".to_string(),
            anchor: Anchor::TopRight,
            padding: 20,
            items: vec![
                TemplateItem {
                    item_type: ItemType::Text,
                    value: "{Camera} • {Lens}".to_string(),
                    font_size: 16,
                    font_size_ratio: 0.20,
                    weight: Some(FontWeight::Bold),
                    color: Some("#FFFFFF".to_string()),
                },
                TemplateItem {
                    item_type: ItemType::Text,
                    value: "{Focal} • {Aperture} • {Shutter} • ISO {ISO}".to_string(),
                    font_size: 12,
                    font_size_ratio: 0.14,
                    weight: Some(FontWeight::Normal),
                    color: Some("#CCCCCC".to_string()),
                },
            ],
            background: Some(Background {
                bg_type: BackgroundType::Rectangle,
                opacity: 0.2,
                radius: Some(8),
                color: Some("#000000".to_string()),
            }),
            frame_height_ratio: 0.10,
            logo_size_ratio: 0.35,
            primary_font_ratio: 0.20,
            secondary_font_ratio: 0.14,
            padding_ratio: 0.10,
        },
        Template {
            name: "Minimal".to_string(),
            anchor: Anchor::BottomRight,
            padding: 16,
            items: vec![TemplateItem {
                item_type: ItemType::Text,
                value: "{Author}".to_string(),
                font_size: 14,
                font_size_ratio: 0.18,
                weight: Some(FontWeight::Normal),
                color: Some("#FFFFFF".to_string()),
            }],
            background: None,
            frame_height_ratio: 0.08, // Smaller frame for minimal design
            logo_size_ratio: 0.35,
            primary_font_ratio: 0.18,
            secondary_font_ratio: 0.14,
            padding_ratio: 0.10,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_substitution() {
        let template = Template {
            name: "Test".to_string(),
            anchor: Anchor::BottomLeft,
            padding: 24,
            items: vec![TemplateItem {
                item_type: ItemType::Text,
                value: "{Author} • {ISO}".to_string(),
                font_size: 16,
                font_size_ratio: 0.2,
                weight: None,
                color: None,
            }],
            background: None,
            frame_height_ratio: 0.1,
            logo_size_ratio: 0.35,
            primary_font_ratio: 0.2,
            secondary_font_ratio: 0.14,
            padding_ratio: 0.1,
        };

        let mut variables = HashMap::new();
        variables.insert("Author".to_string(), "John Doe".to_string());
        variables.insert("ISO".to_string(), "100".to_string());

        let substituted = template.substitute_variables(&variables);

        assert_eq!(substituted.items[0].value, "John Doe • 100");
    }
}
