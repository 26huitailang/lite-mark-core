//! 模板组合集成测试
//!
//! 测试所有模板变量组合和内置模板

use litemark_core::exif::ExifData;
use litemark_core::layout::{self, Template};
use std::collections::HashMap;

/// 测试内置模板加载
#[test]
fn test_builtin_templates_loading() {
    let templates = layout::create_builtin_templates();

    assert!(!templates.is_empty(), "应至少有一个内置模板");
    
    // 验证常见模板存在
    let template_names: Vec<&str> = templates.iter().map(|t| t.name.as_str()).collect();
    
    assert!(
        template_names.contains(&"Classic") || template_names.contains(&"classic"),
        "应包含 Classic 模板"
    );
}

/// 测试内置模板的结构完整性
#[test]
fn test_builtin_templates_structure() {
    let templates = layout::create_builtin_templates();

    for template in &templates {
        // 验证基本字段
        assert!(!template.name.is_empty(), "模板名称不应为空");
        assert!(
            template.frame_height_ratio > 0.0 && template.frame_height_ratio < 1.0,
            "frame_height_ratio 应在 (0, 1) 范围内"
        );
        assert!(
            template.primary_font_ratio > 0.0,
            "primary_font_ratio 应大于 0"
        );
    }
}

/// 测试所有 EXIF 字段组合
#[test]
fn test_all_exif_combinations() {
    let combinations: Vec<(&str, Box<dyn Fn() -> ExifData>)> = vec![
        ("empty", Box::new(|| ExifData::new())),
        ("full", Box::new(create_full_exif)),
        ("only_iso", Box::new(|| {
            let mut d = ExifData::new();
            d.iso = Some(100);
            d
        })),
        ("only_camera", Box::new(|| {
            let mut d = ExifData::new();
            d.camera_model = Some("Camera".to_string());
            d
        })),
        ("only_author", Box::new(|| {
            let mut d = ExifData::new();
            d.author = Some("Author".to_string());
            d
        })),
        ("photo_params", Box::new(|| {
            let mut d = ExifData::new();
            d.iso = Some(400);
            d.aperture = Some(2.8);
            d.shutter_speed = Some("1/125".to_string());
            d.focal_length = Some(50.0);
            d
        })),
        ("equipment_info", Box::new(|| {
            let mut d = ExifData::new();
            d.camera_model = Some("Sony A7M4".to_string());
            d.lens_model = Some("FE 24-70mm".to_string());
            d
        })),
        ("temporal_info", Box::new(|| {
            let mut d = ExifData::new();
            d.date_time = Some("2024:01:15 14:30:00".to_string());
            d
        })),
    ];

    for (name, factory) in combinations {
        let exif_data = factory();
        let variables = exif_data.to_variables();

        // 验证变量生成不 panic
        assert!(
            variables.len() <= 8,
            "变量数量不应超过 EXIF 字段数"
        );

        // 验证模板替换不 panic
        let template = create_test_template_with_all_variables();
        let _substituted = template.substitute_variables(&variables);
    }
}

/// 测试变量替换完整性
#[test]
fn test_variable_substitution_completeness() {
    let exif_data = create_full_exif();
    let variables = exif_data.to_variables();

    // 验证所有可能的变量都存在
    let expected_keys = vec![
        "ISO", "Aperture", "Shutter", "Focal",
        "Camera", "Lens", "DateTime", "Author"
    ];

    for key in &expected_keys {
        assert!(
            variables.contains_key(*key),
            "变量 {} 应存在",
            key
        );
    }
}

/// 测试模板变量替换结果
#[test]
fn test_template_substitution_results() {
    let mut exif_data = ExifData::new();
    exif_data.iso = Some(400);
    exif_data.aperture = Some(2.8);
    exif_data.shutter_speed = Some("1/200".to_string());
    exif_data.author = Some("Test".to_string());

    let variables = exif_data.to_variables();

    // 测试不同模板字符串的替换结果
    let test_cases = vec![
        ("{ISO}", "400"),
        ("{Aperture}", "f/2.8"),
        ("{Shutter}", "1/200"),
        ("ISO {ISO}", "ISO 400"),
        ("{Aperture} | {Shutter}", "f/2.8 | 1/200"),
        ("{Author}'s Photo", "Test's Photo"),
        ("Unknown {Missing}", "Unknown {Missing}"), // 缺失变量保持原样
    ];

    for (template_str, expected) in test_cases {
        let template = create_template_with_value(template_str);
        let substituted = template.substitute_variables(&variables);
        assert_eq!(
            substituted.items[0].value,
            expected,
            "模板 '{}' 应替换为 '{}'",
            template_str,
            expected
        );
    }
}

/// 测试模板 JSON 序列化完整性
#[test]
fn test_template_json_roundtrip() {
    let original = create_test_template_with_all_variables();

    // 序列化
    let json = original.to_json().expect("序列化失败");

    // 反序列化
    let restored = Template::from_json(&json).expect("反序列化失败");

    // 验证关键字段
    assert_eq!(original.name, restored.name);
    assert_eq!(original.items.len(), restored.items.len());
    assert_eq!(original.frame_height_ratio, restored.frame_height_ratio);
}

/// 测试不同 Anchor 位置的模板
#[test]
fn test_template_anchor_positions() {
    use litemark_core::layout::Anchor;

    let anchors = vec![
        Anchor::TopLeft,
        Anchor::TopRight,
        Anchor::BottomLeft,
        Anchor::BottomRight,
        Anchor::Center,
    ];

    for anchor in anchors {
        let template = Template {
            name: "Test".to_string(),
            anchor,
            padding: 20,
            items: vec![],
            background: None,
            frame_height_ratio: 0.1,
            logo_size_ratio: 0.35,
            primary_font_ratio: 0.2,
            secondary_font_ratio: 0.14,
            padding_ratio: 0.1,
        };

        // 验证可以序列化和反序列化
        let json = template.to_json().expect("序列化失败");
        let _restored = Template::from_json(&json).expect("反序列化失败");
    }
}

/// 测试模板比例参数的边界值
#[test]
fn test_template_ratio_boundaries() {
    let ratios = vec![
        0.01f32,  // 极小
        0.05,     // 小
        0.10,     // 默认值附近
        0.20,     // 中等
        0.50,     // 大
    ];

    for ratio in ratios {
        let template = Template {
            name: "Test".to_string(),
            anchor: litemark_core::layout::Anchor::BottomLeft,
            padding: 20,
            items: vec![],
            background: None,
            frame_height_ratio: ratio,
            logo_size_ratio: ratio,
            primary_font_ratio: ratio,
            secondary_font_ratio: ratio * 0.7,
            padding_ratio: ratio * 0.5,
        };

        // 验证可以序列化
        let result = template.to_json();
        assert!(result.is_ok(), "比例 {} 应可序列化", ratio);
    }
}

/// 辅助函数：创建包含所有变量的完整 EXIF 数据
fn create_full_exif() -> ExifData {
    let mut data = ExifData::new();
    data.iso = Some(400);
    data.aperture = Some(2.8);
    data.shutter_speed = Some("1/200".to_string());
    data.focal_length = Some(85.0);
    data.camera_model = Some("Sony A7M4".to_string());
    data.lens_model = Some("FE 85mm F1.8".to_string());
    data.date_time = Some("2024:01:15 14:30:00".to_string());
    data.author = Some("Test Photographer".to_string());
    data
}

/// 辅助函数：创建包含所有变量的测试模板
fn create_test_template_with_all_variables() -> Template {
    use litemark_core::layout::{Anchor, FontWeight, ItemType, TemplateItem};

    Template {
        name: "TestAllVars".to_string(),
        anchor: Anchor::BottomLeft,
        padding: 20,
        items: vec![
            TemplateItem {
                item_type: ItemType::Text,
                value: "{Author}".to_string(),
                font_size: 16,
                font_size_ratio: 0.2,
                weight: Some(FontWeight::Bold),
                color: Some("#000000".to_string()),
            },
            TemplateItem {
                item_type: ItemType::Text,
                value: "{Camera} • {Lens}".to_string(),
                font_size: 14,
                font_size_ratio: 0.16,
                weight: Some(FontWeight::Normal),
                color: Some("#333333".to_string()),
            },
            TemplateItem {
                item_type: ItemType::Text,
                value: "{Aperture} | ISO {ISO} | {Shutter} | {Focal}".to_string(),
                font_size: 12,
                font_size_ratio: 0.14,
                weight: Some(FontWeight::Normal),
                color: Some("#666666".to_string()),
            },
            TemplateItem {
                item_type: ItemType::Text,
                value: "{DateTime}".to_string(),
                font_size: 10,
                font_size_ratio: 0.12,
                weight: Some(FontWeight::Light),
                color: Some("#999999".to_string()),
            },
        ],
        background: None,
        frame_height_ratio: 0.15,
        logo_size_ratio: 0.35,
        primary_font_ratio: 0.2,
        secondary_font_ratio: 0.14,
        padding_ratio: 0.1,
    }
}

/// 辅助函数：创建包含指定值的模板
fn create_template_with_value(value: &str) -> Template {
    use litemark_core::layout::{Anchor, FontWeight, ItemType, TemplateItem};

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
            color: None,
        }],
        background: None,
        frame_height_ratio: 0.1,
        logo_size_ratio: 0.35,
        primary_font_ratio: 0.2,
        secondary_font_ratio: 0.14,
        padding_ratio: 0.1,
    }
}
