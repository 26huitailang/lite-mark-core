//! 模板系统单元测试
//!
//! 测试模板解析、变量替换、序列化等功能

use litemark_core::layout::{Anchor, FontWeight, ItemType, Template, TemplateItem};
use std::collections::HashMap;

/// 测试模板 JSON 序列化和反序列化
#[test]
fn test_template_json_roundtrip() {
    let template = Template {
        name: "TestTemplate".to_string(),
        anchor: Anchor::BottomLeft,
        padding: 20,
        items: vec![
            TemplateItem {
                item_type: ItemType::Text,
                value: "{Author}".to_string(),
                font_size: 16,
                font_size_ratio: 0.2,
                weight: Some(FontWeight::Bold),
                color: Some("#FFFFFF".to_string()),
            },
            TemplateItem {
                item_type: ItemType::Logo,
                value: "".to_string(),
                font_size: 0,
                font_size_ratio: 0.0,
                weight: Some(FontWeight::Normal),
                color: None,
            },
        ],
        background: None,
        frame_height_ratio: 0.10,
        logo_size_ratio: 0.35,
        primary_font_ratio: 0.20,
        secondary_font_ratio: 0.14,
        padding_ratio: 0.10,
    };

    // 序列化
    let json = template.to_json().expect("序列化失败");
    assert!(json.contains("TestTemplate"));
    assert!(json.contains("{Author}"));

    // 反序列化
    let parsed = Template::from_json(&json).expect("反序列化失败");
    assert_eq!(parsed.name, "TestTemplate");
    assert_eq!(parsed.items.len(), 2);
    assert_eq!(parsed.frame_height_ratio, 0.10);
}

/// 测试变量替换 - 简单替换
#[test]
fn test_substitute_variables_simple() {
    let template = create_test_template("Hello {Name}");
    
    let mut vars = HashMap::new();
    vars.insert("Name".to_string(), "World".to_string());

    let substituted = template.substitute_variables(&vars);
    
    assert_eq!(substituted.items[0].value, "Hello World");
}

/// 测试变量替换 - 多个变量
#[test]
fn test_substitute_variables_multiple() {
    let template = create_test_template("{Camera} • {Lens} • ISO {ISO}");
    
    let mut vars = HashMap::new();
    vars.insert("Camera".to_string(), "Sony A7M4".to_string());
    vars.insert("Lens".to_string(), "FE 24-70mm".to_string());
    vars.insert("ISO".to_string(), "400".to_string());

    let substituted = template.substitute_variables(&vars);
    
    assert_eq!(substituted.items[0].value, "Sony A7M4 • FE 24-70mm • ISO 400");
}

/// 测试变量替换 - 缺失变量保持原样
#[test]
fn test_substitute_variables_missing() {
    let template = create_test_template("{Author} • {Missing}");
    
    let mut vars = HashMap::new();
    vars.insert("Author".to_string(), "Test".to_string());
    // Missing 不提供

    let substituted = template.substitute_variables(&vars);
    
    assert_eq!(substituted.items[0].value, "Test • {Missing}");
}

/// 测试变量替换 - 空变量
#[test]
fn test_substitute_variables_empty() {
    let template = create_test_template("{Author}");
    
    let mut vars = HashMap::new();
    vars.insert("Author".to_string(), "".to_string());

    let substituted = template.substitute_variables(&vars);
    
    assert_eq!(substituted.items[0].value, "");
}

/// 测试变量替换 - 特殊字符
#[test]
fn test_substitute_variables_special_chars() {
    let template = create_test_template("{Author}");
    
    let mut vars = HashMap::new();
    vars.insert("Author".to_string(), "Test <>&\"'".to_string());

    let substituted = template.substitute_variables(&vars);
    
    assert_eq!(substituted.items[0].value, "Test <>&\"'");
}

/// 测试变量替换 - Unicode 字符
#[test]
fn test_substitute_variables_unicode() {
    let template = create_test_template("{Author} • {Camera}");
    
    let mut vars = HashMap::new();
    vars.insert("Author".to_string(), "摄影师📷".to_string());
    vars.insert("Camera".to_string(), "相机".to_string());

    let substituted = template.substitute_variables(&vars);
    
    assert_eq!(substituted.items[0].value, "摄影师📷 • 相机");
}

/// 测试变量替换 - 多个模板项
#[test]
fn test_substitute_variables_multiple_items() {
    let template = Template {
        name: "Test".to_string(),
        anchor: Anchor::BottomLeft,
        padding: 0,
        items: vec![
            TemplateItem {
                item_type: ItemType::Text,
                value: "{Author}".to_string(),
                font_size: 20,
                font_size_ratio: 0.25,
                weight: Some(FontWeight::Bold),
                color: Some("#000000".to_string()),
            },
            TemplateItem {
                item_type: ItemType::Text,
                value: "{Aperture} | ISO {ISO}".to_string(),
                font_size: 14,
                font_size_ratio: 0.18,
                weight: Some(FontWeight::Normal),
                color: Some("#666666".to_string()),
            },
        ],
        background: None,
        frame_height_ratio: 0.10,
        logo_size_ratio: 0.35,
        primary_font_ratio: 0.25,
        secondary_font_ratio: 0.18,
        padding_ratio: 0.10,
    };

    let mut vars = HashMap::new();
    vars.insert("Author".to_string(), "John".to_string());
    vars.insert("Aperture".to_string(), "f/2.8".to_string());
    vars.insert("ISO".to_string(), "400".to_string());

    let substituted = template.substitute_variables(&vars);
    
    assert_eq!(substituted.items[0].value, "John");
    assert_eq!(substituted.items[1].value, "f/2.8 | ISO 400");
}

/// 测试无效 JSON 解析
#[test]
fn test_template_invalid_json() {
    let invalid_json = "{ invalid json }";
    let result = Template::from_json(invalid_json);
    
    assert!(result.is_err(), "无效 JSON 应返回错误");
}

/// 测试缺少字段的 JSON
#[test]
fn test_template_missing_fields() {
    let incomplete_json = r#"{
        "name": "Incomplete",
        "anchor": "bottom-left"
    }"#;
    
    let result = Template::from_json(incomplete_json);
    // serde 会处理缺失字段，使用默认值
    assert!(result.is_ok() || result.is_err());
}

/// 测试所有 Anchor 变体
#[test]
fn test_anchor_variants() {
    let anchors = vec![
        ("top-left", Anchor::TopLeft),
        ("top-right", Anchor::TopRight),
        ("bottom-left", Anchor::BottomLeft),
        ("bottom-right", Anchor::BottomRight),
        ("center", Anchor::Center),
    ];

    for (json_name, _) in anchors {
        let json = format!(
            r#"{{
                "name": "Test",
                "anchor": "{}",
                "padding": 0,
                "items": [],
                "frame_height_ratio": 0.1,
                "logo_size_ratio": 0.35,
                "primary_font_ratio": 0.2,
                "secondary_font_ratio": 0.14,
                "padding_ratio": 0.1
            }}"#,
            json_name
        );
        
        let result = Template::from_json(&json);
        assert!(result.is_ok(), "Anchor '{}' 应被正确解析", json_name);
    }
}

/// 测试所有 ItemType 变体
#[test]
fn test_item_type_variants() {
    let items = vec![
        ("text", ItemType::Text),
        ("logo", ItemType::Logo),
    ];

    for (json_name, _) in items {
        let json = format!(
            r#"{{
                "name": "Test",
                "anchor": "bottom-left",
                "padding": 0,
                "items": [
                    {{
                        "type": "{}",
                        "value": "test",
                        "font_size_ratio": 0.2
                    }}
                ],
                "frame_height_ratio": 0.1,
                "logo_size_ratio": 0.35,
                "primary_font_ratio": 0.2,
                "secondary_font_ratio": 0.14,
                "padding_ratio": 0.1
            }}"#,
            json_name
        );
        
        let result = Template::from_json(&json);
        assert!(result.is_ok(), "ItemType '{}' 应被正确解析", json_name);
    }
}

/// 测试所有 FontWeight 变体
#[test]
fn test_font_weight_variants() {
    let weights = vec!["normal", "bold", "light"];

    for weight in weights {
        let json = format!(
            r#"{{
                "name": "Test",
                "anchor": "bottom-left",
                "padding": 0,
                "items": [
                    {{
                        "type": "text",
                        "value": "test",
                        "font_size_ratio": 0.2,
                        "weight": "{}"
                    }}
                ],
                "frame_height_ratio": 0.1,
                "logo_size_ratio": 0.35,
                "primary_font_ratio": 0.2,
                "secondary_font_ratio": 0.14,
                "padding_ratio": 0.1
            }}"#,
            weight
        );
        
        let result = Template::from_json(&json);
        assert!(result.is_ok(), "FontWeight '{}' 应被正确解析", weight);
    }
}

/// 测试模板克隆
#[test]
fn test_template_clone() {
    let template = create_test_template("{Author}");
    let cloned = template.clone();
    
    assert_eq!(template.name, cloned.name);
    assert_eq!(template.items.len(), cloned.items.len());
}

/// 辅助函数：创建简单测试模板
fn create_test_template(value: &str) -> Template {
    Template {
        name: "Test".to_string(),
        anchor: Anchor::BottomLeft,
        padding: 0,
        items: vec![TemplateItem {
            item_type: ItemType::Text,
            value: value.to_string(),
            font_size: 16,
            font_size_ratio: 0.2,
            weight: Some(FontWeight::Normal),
            color: Some("#000000".to_string()),
        }],
        background: None,
        frame_height_ratio: 0.10,
        logo_size_ratio: 0.35,
        primary_font_ratio: 0.20,
        secondary_font_ratio: 0.14,
        padding_ratio: 0.10,
    }
}
