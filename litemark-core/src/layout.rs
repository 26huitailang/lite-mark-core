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
    let mut templates = vec![];

    // Load all templates from JSON files
    for template_name in ["classic", "compact", "professional"] {
        let template_path = format!("templates/{}.json", template_name);
        if let Ok(content) = std::fs::read_to_string(&template_path) {
            if let Ok(template) = Template::from_json(&content) {
                templates.push(template);
            }
        }
    }

    templates
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
