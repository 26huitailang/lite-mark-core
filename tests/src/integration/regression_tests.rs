//! 回归测试套件
//!
//! 使用 JSON Lines 格式定义回归测试用例

use litemark_core::exif::ExifData;
use litemark_core::layout::{RenderMode, self, Anchor, FontWeight, ItemType, Template, TemplateItem};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// 回归测试用例
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RegressionCase {
    id: String,
    description: String,
    template: String,
    variables: HashMap<String, String>,
    expected: ExpectedResult,
}

/// 预期结果
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExpectedResult {
    success: bool,
    #[serde(default)]
    output_contains: Option<String>,
    #[serde(default)]
    text_contains: Option<String>,
}

/// 加载回归测试用例
fn load_regression_cases() -> Vec<RegressionCase> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let file_path = Path::new(manifest_dir).join("fixtures/regressions.jsonl");

    // 如果文件不存在，返回默认测试用例
    if !file_path.exists() {
        return get_default_regression_cases();
    }

    let content = fs::read_to_string(&file_path).expect("读取回归测试文件失败");
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line).expect("解析回归测试用例失败"))
        .collect()
}

/// 默认回归测试用例
fn get_default_regression_cases() -> Vec<RegressionCase> {
    vec![
        RegressionCase {
            id: "R001".to_string(),
            description: "基本变量替换".to_string(),
            template: "{Author}".to_string(),
            variables: [("Author".to_string(), "Test".to_string())].into(),
            expected: ExpectedResult {
                success: true,
                output_contains: Some("Test".to_string()),
                text_contains: None,
            },
        },
        RegressionCase {
            id: "R002".to_string(),
            description: "多变量替换".to_string(),
            template: "{Camera} • {Lens}".to_string(),
            variables: [
                ("Camera".to_string(), "Sony".to_string()),
                ("Lens".to_string(), "85mm".to_string()),
            ]
            .into(),
            expected: ExpectedResult {
                success: true,
                output_contains: Some("Sony".to_string()),
                text_contains: Some("85mm".to_string()),
            },
        },
        RegressionCase {
            id: "R003".to_string(),
            description: "缺失变量保持原样".to_string(),
            template: "{Author} • {Missing}".to_string(),
            variables: [("Author".to_string(), "Test".to_string())].into(),
            expected: ExpectedResult {
                success: true,
                output_contains: Some("Test".to_string()),
                text_contains: Some("{Missing}".to_string()),
            },
        },
        RegressionCase {
            id: "R004".to_string(),
            description: "Unicode 字符".to_string(),
            template: "{Author}".to_string(),
            variables: [("Author".to_string(), "摄影师📷".to_string())].into(),
            expected: ExpectedResult {
                success: true,
                output_contains: Some("摄影师".to_string()),
                text_contains: None,
            },
        },
        RegressionCase {
            id: "R005".to_string(),
            description: "特殊字符".to_string(),
            template: "{Author}".to_string(),
            variables: [("Author".to_string(), "<>&\"'".to_string())].into(),
            expected: ExpectedResult {
                success: true,
                output_contains: Some("<>&\"'".to_string()),
                text_contains: None,
            },
        },
        RegressionCase {
            id: "R006".to_string(),
            description: "空变量".to_string(),
            template: "{Author}".to_string(),
            variables: [("Author".to_string(), "".to_string())].into(),
            expected: ExpectedResult {
                success: true,
                output_contains: Some("".to_string()),
                text_contains: None,
            },
        },
        RegressionCase {
            id: "R007".to_string(),
            description: "长文本".to_string(),
            template: "{Author}".to_string(),
            variables: [(
                "Author".to_string(),
                "Very Long Author Name That Tests Long Text Handling".to_string(),
            )]
            .into(),
            expected: ExpectedResult {
                success: true,
                output_contains: Some("Very Long".to_string()),
                text_contains: None,
            },
        },
        RegressionCase {
            id: "R008".to_string(),
            description: "数字变量".to_string(),
            template: "ISO {ISO}".to_string(),
            variables: [("ISO".to_string(), "6400".to_string())].into(),
            expected: ExpectedResult {
                success: true,
                output_contains: Some("6400".to_string()),
                text_contains: None,
            },
        },
    ]
}

/// 运行所有回归测试
#[test]
fn test_regression_suite() {
    let cases = load_regression_cases();

    println!("运行 {} 个回归测试用例", cases.len());

    let mut failures = Vec::new();

    for case in cases {
        match run_regression_case(&case) {
            Ok(()) => {
                println!("✅ {}: {}", case.id, case.description);
            }
            Err(e) => {
                println!("❌ {}: {} - {}", case.id, case.description, e);
                failures.push((case.id, case.description, e));
            }
        }
    }

    if !failures.is_empty() {
        let error_msg = failures
            .iter()
            .map(|(id, desc, err)| format!("\n  {} ({}): {}", id, desc, err))
            .collect::<String>();
        panic!("回归测试失败: {}", error_msg);
    }
}

/// 运行单个回归测试用例
fn run_regression_case(case: &RegressionCase) -> Result<(), String> {
    // 创建模板
    let template = Template {
        name: format!("Regression-{}", case.id),
        anchor: Anchor::BottomLeft,
        padding: 0,
        items: vec![TemplateItem {
            item_type: ItemType::Text,
            value: case.template.clone(),
            font_size: 16,
            font_size_ratio: 0.2,
            weight: Some(FontWeight::Normal),
            color: Some("#000000".to_string()),
        }],
        background: None,
        frame_height_ratio: 0.1,
        logo_size_ratio: 0.35,
        primary_font_ratio: 0.2,
        secondary_font_ratio: 0.14,
        padding_ratio: 0.,
        render_mode: RenderMode::BottomFrame,
    };

    // 执行变量替换
    let result = template.substitute_variables(&case.variables);
    let output = &result.items[0].value;

    // 验证结果
    if case.expected.success {
        // 验证输出包含预期内容
        if let Some(ref expected_contains) = case.expected.output_contains
            && !output.contains(expected_contains) {
                return Err(format!(
                    "输出不包含 '{}', 实际输出: '{}'",
                    expected_contains, output
                ));
            }

        if let Some(ref expected_text) = case.expected.text_contains
            && !output.contains(expected_text) {
                return Err(format!(
                    "输出不包含 '{}', 实际输出: '{}'",
                    expected_text, output
                ));
            }
    }

    Ok(())
}

/// 测试 EXIF 数据格式化回归
#[test]
fn test_exif_formatting_regression() {
    // 测试 ISO 格式化
    {
        let mut data = ExifData::new();
        data.iso = Some(100);
        let vars = data.to_variables();
        assert_eq!(vars.get("ISO"), Some(&"100".to_string()));
    }
    {
        let mut data = ExifData::new();
        data.iso = Some(6400);
        let vars = data.to_variables();
        assert_eq!(vars.get("ISO"), Some(&"6400".to_string()));
    }

    // 测试光圈格式化
    {
        let mut data = ExifData::new();
        data.aperture = Some(1.4);
        let vars = data.to_variables();
        let value = vars.get("Aperture").expect("应有 Aperture 变量");
        assert!(value.contains("f/1.4"), "光圈应格式化为 f/1.4，实际是 {}", value);
    }
    {
        let mut data = ExifData::new();
        data.aperture = Some(22.0);
        let vars = data.to_variables();
        let value = vars.get("Aperture").expect("应有 Aperture 变量");
        assert!(value.contains("f/22") || value.contains("f/22.0"), 
                "光圈应格式化为 f/22 或 f/22.0，实际是 {}", value);
    }

    // 测试焦距格式化
    {
        let mut data = ExifData::new();
        data.focal_length = Some(24.0);
        let vars = data.to_variables();
        let value = vars.get("Focal").expect("应有 Focal 变量");
        assert!(value.contains("24mm"), "焦距应格式化为 24mm，实际是 {}", value);
    }
    {
        let mut data = ExifData::new();
        data.focal_length = Some(200.0);
        let vars = data.to_variables();
        let value = vars.get("Focal").expect("应有 Focal 变量");
        assert!(value.contains("200mm"), "焦距应格式化为 200mm，实际是 {}", value);
    }
}

/// 测试内置模板加载回归
#[test]
fn test_builtin_templates_regression() {
    let templates = layout::create_builtin_templates();

    // 至少应有一些模板
    assert!(!templates.is_empty(), "应至少有一个内置模板");

    // 所有模板都应可序列化
    for template in &templates {
        let json = template.to_json().unwrap_or_else(|_| panic!("模板 '{}' 应可序列化",
            template.name));
        assert!(!json.is_empty());

        // 应可反序列化
        let restored = Template::from_json(&json).unwrap_or_else(|_| panic!("模板 '{}' 应可反序列化",
            template.name));
        assert_eq!(template.name, restored.name);
    }
}

/// 测试空 EXIF 数据处理回归
#[test]
fn test_empty_exif_regression() {
    let empty_exif = ExifData::new();
    let vars = empty_exif.to_variables();

    // 空 EXIF 应产生空变量集合
    assert!(vars.is_empty(), "空 EXIF 应产生空变量");

    // get_missing_fields 应返回所有字段
    let missing = empty_exif.get_missing_fields();
    assert_eq!(missing.len(), 8, "应有 8 个缺失字段");
}

/// 测试从空字节提取 EXIF 回归
#[test]
fn test_extract_from_empty_bytes_regression() {
    let empty_data: &[u8] = &[];
    let result = litemark_core::exif::extract_from_bytes(empty_data);

    assert!(result.is_ok(), "空数据提取应返回 Ok");
    let exif = result.unwrap();
    assert!(exif.iso.is_none(), "空数据提取的 ISO 应为 None");
}
