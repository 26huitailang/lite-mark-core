//! EXIF 模块单元测试
//!
//! 测试 EXIF 数据提取、格式化、变量转换等功能

use litemark_core::exif::ExifData;

/// 测试 ExifData::new() 创建空对象
#[test]
fn test_exif_data_new_empty() {
    let data = ExifData::new();
    
    assert!(data.iso.is_none());
    assert!(data.aperture.is_none());
    assert!(data.shutter_speed.is_none());
    assert!(data.focal_length.is_none());
    assert!(data.camera_model.is_none());
    assert!(data.lens_model.is_none());
    assert!(data.date_time.is_none());
    assert!(data.author.is_none());
}

/// 测试 ExifData::default() 行为与 new() 一致
#[test]
fn test_exif_data_default() {
    let new_data = ExifData::new();
    let default_data = ExifData::default();
    
    assert_eq!(new_data.iso, default_data.iso);
    assert_eq!(new_data.aperture, default_data.aperture);
    assert_eq!(new_data.shutter_speed, default_data.shutter_speed);
}

/// 测试完整 EXIF 数据转换为变量
#[test]
fn test_exif_data_to_variables_full() {
    let mut data = ExifData::new();
    data.iso = Some(400);
    data.aperture = Some(2.8);
    data.shutter_speed = Some("1/200".to_string());
    data.focal_length = Some(85.0);
    data.camera_model = Some("Sony A7M4".to_string());
    data.lens_model = Some("FE 85mm F1.8".to_string());
    data.date_time = Some("2024:01:15 14:30:00".to_string());
    data.author = Some("Test Photographer".to_string());

    let vars = data.to_variables();

    assert_eq!(vars.get("ISO"), Some(&"400".to_string()));
    assert_eq!(vars.get("Aperture"), Some(&"f/2.8".to_string()));
    assert_eq!(vars.get("Shutter"), Some(&"1/200".to_string()));
    assert_eq!(vars.get("Focal"), Some(&"85mm".to_string()));
    assert_eq!(vars.get("Camera"), Some(&"Sony A7M4".to_string()));
    assert_eq!(vars.get("Lens"), Some(&"FE 85mm F1.8".to_string()));
    assert_eq!(vars.get("DateTime"), Some(&"2024:01:15 14:30:00".to_string()));
    assert_eq!(vars.get("Author"), Some(&"Test Photographer".to_string()));
}

/// 测试部分 EXIF 数据转换为变量（缺失字段不应出现在结果中）
#[test]
fn test_exif_data_to_variables_partial() {
    let mut data = ExifData::new();
    data.iso = Some(100);
    data.camera_model = Some("Canon R5".to_string());
    // 其他字段保持 None

    let vars = data.to_variables();

    assert_eq!(vars.len(), 2);
    assert_eq!(vars.get("ISO"), Some(&"100".to_string()));
    assert_eq!(vars.get("Camera"), Some(&"Canon R5".to_string()));
    assert!(!vars.contains_key("Aperture"));
    assert!(!vars.contains_key("Lens"));
}

/// 测试光圈值格式化（小数位处理）
#[test]
fn test_aperture_formatting() {
    let test_cases = vec![
        (1.4, "f/1.4"),
        (2.8, "f/2.8"),
        (5.6, "f/5.6"),
        (11.0, "f/11.0"),
        (16.0, "f/16.0"),
        (0.95, "f/0.9"),  // 边界：极小光圈
        (999.0, "f/999.0"), // 边界：极大光圈
    ];

    for (aperture, expected) in test_cases {
        let mut data = ExifData::new();
        data.aperture = Some(aperture);
        let vars = data.to_variables();
        assert_eq!(vars.get("Aperture"), Some(&expected.to_string()), 
            "光圈值 {} 应格式化为 {}", aperture, expected);
    }
}

/// 测试焦距格式化
#[test]
fn test_focal_length_formatting() {
    let test_cases = vec![
        (24.0, "24mm"),
        (50.0, "50mm"),
        (85.0, "85mm"),
        (200.0, "200mm"),
        (70.0, "70mm"),  // 变焦常见值
        (1.0, "1mm"),    // 边界：极短焦距
        (800.0, "800mm"), // 边界：超长焦
    ];

    for (focal, expected) in test_cases {
        let mut data = ExifData::new();
        data.focal_length = Some(focal);
        let vars = data.to_variables();
        assert_eq!(vars.get("Focal"), Some(&expected.to_string()),
            "焦距 {} 应格式化为 {}", focal, expected);
    }
}

/// 测试缺失字段检测
#[test]
fn test_get_missing_fields_all_present() {
    let mut data = ExifData::new();
    data.iso = Some(100);
    data.aperture = Some(2.8);
    data.shutter_speed = Some("1/125".to_string());
    data.focal_length = Some(50.0);
    data.camera_model = Some("Camera".to_string());
    data.lens_model = Some("Lens".to_string());
    data.date_time = Some("2024:01:01".to_string());
    data.author = Some("Author".to_string());

    let missing = data.get_missing_fields();
    assert!(missing.is_empty(), "所有字段都存在时不应有缺失字段");
}

#[test]
fn test_get_missing_fields_all_missing() {
    let data = ExifData::new();
    let missing = data.get_missing_fields();
    
    assert_eq!(missing.len(), 8);
    assert!(missing.contains(&"ISO".to_string()));
    assert!(missing.contains(&"Aperture".to_string()));
    assert!(missing.contains(&"Shutter".to_string()));
    assert!(missing.contains(&"Focal".to_string()));
    assert!(missing.contains(&"Camera".to_string()));
    assert!(missing.contains(&"Lens".to_string()));
    assert!(missing.contains(&"DateTime".to_string()));
    assert!(missing.contains(&"Author".to_string()));
}

#[test]
fn test_get_missing_fields_partial() {
    let mut data = ExifData::new();
    data.iso = Some(100);
    data.author = Some("Test".to_string());

    let missing = data.get_missing_fields();
    
    assert_eq!(missing.len(), 6);
    assert!(!missing.contains(&"ISO".to_string()));
    assert!(!missing.contains(&"Author".to_string()));
    assert!(missing.contains(&"Aperture".to_string()));
}

/// 测试从空字节提取（无 EXIF）
#[test]
fn test_extract_from_empty_bytes() {
    let empty_data: &[u8] = &[];
    let result = litemark_core::exif::extract_from_bytes(empty_data);
    
    assert!(result.is_ok(), "空数据应返回 Ok");
    let exif_data = result.unwrap();
    assert!(exif_data.iso.is_none());
}

/// 测试从无效数据提取（损坏的 EXIF）
#[test]
fn test_extract_from_invalid_data() {
    // 随机数据，不是有效的图片
    let invalid_data: Vec<u8> = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x00, 0x00];
    let result = litemark_core::exif::extract_from_bytes(&invalid_data);
    
    // 应该返回 Ok 而不是 Err（降级处理）
    assert!(result.is_ok());
}

/// 测试变量包含特殊字符
#[test]
fn test_exif_variables_special_chars() {
    let mut data = ExifData::new();
    data.author = Some("摄影师📷 Test".to_string());
    data.camera_model = Some("Canon EOS R5 <special>".to_string());

    let vars = data.to_variables();
    
    assert_eq!(vars.get("Author"), Some(&"摄影师📷 Test".to_string()));
    assert_eq!(vars.get("Camera"), Some(&"Canon EOS R5 <special>".to_string()));
}

/// 测试 ISO 值边界
#[test]
fn test_iso_boundaries() {
    let test_values = vec![
        (0, "0"),      // 理论最小值
        (50, "50"),    // 低 ISO
        (100, "100"),  // 标准 ISO
        (6400, "6400"), // 高 ISO
        (102400, "102400"), // 极高 ISO
        (999999, "999999"), // 超大值
    ];

    for (iso, expected) in test_values {
        let mut data = ExifData::new();
        data.iso = Some(iso);
        let vars = data.to_variables();
        assert_eq!(vars.get("ISO"), Some(&expected.to_string()),
            "ISO {} 应格式化为 {}", iso, expected);
    }
}

/// 测试 ExifData 克隆
#[test]
fn test_exif_data_clone() {
    let mut original = ExifData::new();
    original.iso = Some(200);
    original.camera_model = Some("Nikon Z6".to_string());

    let cloned = original.clone();
    
    assert_eq!(original.iso, cloned.iso);
    assert_eq!(original.camera_model, cloned.camera_model);
}

/// 测试 ExifData Debug 输出
#[test]
fn test_exif_data_debug() {
    let mut data = ExifData::new();
    data.iso = Some(100);
    
    let debug_str = format!("{:?}", data);
    assert!(debug_str.contains("ExifData"));
    assert!(debug_str.contains("iso: Some(100)"));
}
