//! CLI 端到端测试
//!
//! 测试命令行接口的完整功能

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// 测试 CLI 帮助信息
#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("litemark-cli").expect("找不到 litemark-cli");
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("litemark-cli"));
}

/// 测试 CLI 版本信息
#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("litemark-cli").expect("找不到 litemark-cli");
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("0."));
}

/// 测试模板列表命令
#[test]
fn test_cli_templates_command() {
    let mut cmd = Command::cargo_bin("litemark-cli").expect("找不到 litemark-cli");
    cmd.arg("templates");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Available templates"));
}

/// 测试单图处理 - 无效输入路径
#[test]
fn test_cli_single_invalid_input() {
    let mut cmd = Command::cargo_bin("litemark-cli").expect("找不到 litemark-cli");
    cmd.arg("add")
        .arg("-i")
        .arg("/nonexistent/path/image.jpg")
        .arg("-o")
        .arg("/tmp/output.jpg")
        .arg("-t")
        .arg("classic");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

/// 测试单图处理 - 无效模板名称
#[test]
fn test_cli_single_invalid_template() {
    let temp_dir = TempDir::new().expect("创建临时目录失败");
    let input_path = create_test_image(&temp_dir, "test.jpg");
    let output_path = temp_dir.path().join("output.jpg");

    let mut cmd = Command::cargo_bin("litemark-cli").expect("找不到 litemark-cli");
    cmd.arg("add")
        .arg("-i")
        .arg(&input_path)
        .arg("-o")
        .arg(&output_path)
        .arg("-t")
        .arg("nonexistent_template");

    // 应失败或警告无效模板
    let output = cmd.output().expect("运行命令失败");
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        !output.status.success()
            || stderr.contains("Error")
            || stderr.contains("error")
            || stdout.contains("Error")
            || stdout.contains("error")
            || stdout.contains("available"),
        "无效模板应产生错误或警告: stderr={}, stdout={}",
        stderr,
        stdout
    );
}

/// 测试单图处理 - 缺少必需参数
#[test]
fn test_cli_single_missing_args() {
    let mut cmd = Command::cargo_bin("litemark-cli").expect("找不到 litemark-cli");
    cmd.arg("add");
    // 缺少 -i 和 -o
    cmd.assert().failure();
}

/// 测试批量处理 - 无效输入目录
#[test]
fn test_cli_batch_invalid_input() {
    let temp_dir = TempDir::new().expect("创建临时目录失败");
    let output_dir = temp_dir.path().join("output");

    let mut cmd = Command::cargo_bin("litemark-cli").expect("找不到 litemark-cli");
    cmd.arg("batch")
        .arg("-i")
        .arg("/nonexistent/directory")
        .arg("-o")
        .arg(&output_dir)
        .arg("-t")
        .arg("classic");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

/// 辅助函数：创建测试图像文件
fn create_test_image(temp_dir: &TempDir, filename: &str) -> std::path::PathBuf {
    use image::{ImageBuffer, Rgb};

    let path = temp_dir.path().join(filename);
    let img = ImageBuffer::from_fn(400, 300, |x, y| {
        Rgb([(x % 256) as u8, (y % 256) as u8, 128])
    });
    img.save(&path).expect("保存测试图像失败");
    path
}

/// 辅助函数：创建损坏的图像文件
fn create_corrupted_image(temp_dir: &TempDir, filename: &str) -> std::path::PathBuf {
    let path = temp_dir.path().join(filename);
    fs::write(&path, b"not a valid image file").expect("写入损坏文件失败");
    path
}
