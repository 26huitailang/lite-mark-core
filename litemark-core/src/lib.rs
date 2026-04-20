//! LiteMark Core Library
//!
//! 提供纯粹的图像水印处理能力，支持多平台复用（CLI、Web、iOS、Desktop）
//!
//! # 核心模块
//!
//! - `image_io`: 图像编解码模块，支持内存操作
//! - `exif`: EXIF 元数据提取模块
//! - `layout`: 模板引擎模块
//! - `renderer`: 水印渲染引擎模块
//!
//! # 设计原则
//!
//! - 无副作用：所有函数保持纯函数特性，不进行文件系统操作
//! - 平台无关：不依赖特定平台的 API（如 std::fs、环境变量等）
//! - 内存安全：所有图像数据通过内存传递，不涉及路径引用
//! - 可组合性：每个模块可独立使用，也可组合使用

pub mod error;
pub mod image_io;
pub mod exif;
pub mod layout;
pub mod renderer;

// Re-export core types for convenience
pub use error::CoreError;
pub use exif::ExifData;
pub use layout::Template;
pub use renderer::WatermarkRenderer;
