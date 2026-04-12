//! 测试套件健康检查工具
//!
//! 验证测试套件自身的完整性

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn main() -> Result<()> {
    println!("🏥 LiteMark 测试套件健康检查\n");

    let mut checks_passed = 0;
    let mut checks_failed = 0;

    // 检查 1: 测试图片目录结构
    print!("检查测试图片目录... ");
    match check_test_images() {
        Ok(count) => {
            println!("✅ 找到 {} 个测试图片", count);
            checks_passed += 1;
        }
        Err(e) => {
            println!("❌ {}", e);
            checks_failed += 1;
        }
    }

    // 检查 2: 模板文件
    print!("检查模板文件... ");
    match check_templates() {
        Ok(count) => {
            println!("✅ 找到 {} 个模板", count);
            checks_passed += 1;
        }
        Err(e) => {
            println!("❌ {}", e);
            checks_failed += 1;
        }
    }

    // 检查 3: 单元测试文件
    print!("检查单元测试... ");
    match check_unit_tests() {
        Ok(count) => {
            println!("✅ 找到 {} 个单元测试文件", count);
            checks_passed += 1;
        }
        Err(e) => {
            println!("❌ {}", e);
            checks_failed += 1;
        }
    }

    // 检查 4: 集成测试文件
    print!("检查集成测试... ");
    match check_integration_tests() {
        Ok(count) => {
            println!("✅ 找到 {} 个集成测试文件", count);
            checks_passed += 1;
        }
        Err(e) => {
            println!("❌ {}", e);
            checks_failed += 1;
        }
    }

    // 检查 5: E2E 测试文件
    print!("检查 E2E 测试... ");
    match check_e2e_tests() {
        Ok(count) => {
            println!("✅ 找到 {} 个 E2E 测试文件", count);
            checks_passed += 1;
        }
        Err(e) => {
            println!("❌ {}", e);
            checks_failed += 1;
        }
    }

    // 检查 6: 报告模板
    print!("检查报告模板... ");
    match check_report_assets() {
        Ok(_) => {
            println!("✅ 报告模板文件完整");
            checks_passed += 1;
        }
        Err(e) => {
            println!("❌ {}", e);
            checks_failed += 1;
        }
    }

    // 检查 7: 二进制文件
    print!("检查可执行工具... ");
    match check_binaries() {
        Ok(count) => {
            println!("✅ 找到 {} 个可执行工具", count);
            checks_passed += 1;
        }
        Err(e) => {
            println!("❌ {}", e);
            checks_failed += 1;
        }
    }

    // 检查 8: 依赖 crates
    print!("检查依赖... ");
    match check_dependencies() {
        Ok(_) => {
            println!("✅ 依赖检查通过");
            checks_passed += 1;
        }
        Err(e) => {
            println!("❌ {}", e);
            checks_failed += 1;
        }
    }

    // 总结
    println!("\n{}", "=".repeat(50));
    println!("健康检查完成: {} 通过, {} 失败", checks_passed, checks_failed);

    if checks_failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}

/// 检查测试图片
fn check_test_images() -> Result<usize> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let images_dir = Path::new(manifest_dir).join("fixtures/images");

    if !images_dir.exists() {
        return Err(anyhow::anyhow!("测试图片目录不存在: {}", images_dir.display()));
    }

    let mut count = 0;
    for entry in WalkDir::new(&images_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if matches!(ext.to_lowercase().as_str(), "jpg" | "jpeg" | "png" | "webp") {
                count += 1;
            }
        }
    }

    if count == 0 {
        return Err(anyhow::anyhow!("未找到测试图片"));
    }

    Ok(count)
}

/// 检查模板文件
fn check_templates() -> Result<usize> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let templates_dir = Path::new(manifest_dir).join("fixtures/templates");

    // 检查内置模板目录
    let builtin_dir = Path::new(manifest_dir).join("../templates");

    let mut count = 0;

    if templates_dir.exists() {
        for entry in fs::read_dir(&templates_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                // 验证 JSON 有效性
                let content = fs::read_to_string(&path)?;
                serde_json::from_str::<serde_json::Value>(&content)
                    .with_context(|| format!("无效的 JSON: {}", path.display()))?;
                count += 1;
            }
        }
    }

    if builtin_dir.exists() {
        for entry in fs::read_dir(&builtin_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                count += 1;
            }
        }
    }

    Ok(count)
}

/// 检查单元测试
fn check_unit_tests() -> Result<usize> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let unit_dir = Path::new(manifest_dir).join("src/unit");

    if !unit_dir.exists() {
        return Err(anyhow::anyhow!("单元测试目录不存在"));
    }

    let count = fs::read_dir(&unit_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            path.is_file() && path.extension().map(|ext| ext == "rs").unwrap_or(false)
        })
        .count();

    if count == 0 {
        return Err(anyhow::anyhow!("未找到单元测试文件"));
    }

    Ok(count)
}

/// 检查集成测试
fn check_integration_tests() -> Result<usize> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let integration_dir = Path::new(manifest_dir).join("src/integration");

    if !integration_dir.exists() {
        return Err(anyhow::anyhow!("集成测试目录不存在"));
    }

    let count = fs::read_dir(&integration_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            path.is_file() && path.extension().map(|ext| ext == "rs").unwrap_or(false)
        })
        .count();

    if count == 0 {
        return Err(anyhow::anyhow!("未找到集成测试文件"));
    }

    Ok(count)
}

/// 检查 E2E 测试
fn check_e2e_tests() -> Result<usize> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let e2e_dir = Path::new(manifest_dir).join("src/e2e");

    if !e2e_dir.exists() {
        return Err(anyhow::anyhow!("E2E 测试目录不存在"));
    }

    let count = fs::read_dir(&e2e_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            path.is_file() && path.extension().map(|ext| ext == "rs").unwrap_or(false)
        })
        .count();

    Ok(count)
}

/// 检查报告资源
fn check_report_assets() -> Result<()> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let assets_dir = Path::new(manifest_dir).join("assets");

    let required_files = vec!["report_template.html", "styles.css"];

    for file in required_files {
        let path = assets_dir.join(file);
        if !path.exists() {
            return Err(anyhow::anyhow!("缺少报告资源文件: {}", file));
        }
    }

    Ok(())
}

/// 检查可执行工具
fn check_binaries() -> Result<usize> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let bin_dir = Path::new(manifest_dir).join("src/bin");

    if !bin_dir.exists() {
        return Err(anyhow::anyhow!("二进制目录不存在"));
    }

    let count = fs::read_dir(&bin_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            path.is_file() && path.extension().map(|ext| ext == "rs").unwrap_or(false)
        })
        .count();

    if count == 0 {
        return Err(anyhow::anyhow!("未找到可执行工具"));
    }

    Ok(count)
}

/// 检查依赖
fn check_dependencies() -> Result<()> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let cargo_toml = Path::new(manifest_dir).join("Cargo.toml");

    let content = fs::read_to_string(&cargo_toml)?;
    
    // 验证基本依赖存在
    let required_deps = vec!["litemark-core", "litemark-cli", "image", "serde", "tempfile"];
    
    for dep in required_deps {
        if !content.contains(dep) {
            return Err(anyhow::anyhow!("缺少依赖: {}", dep));
        }
    }

    Ok(())
}
