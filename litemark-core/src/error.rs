use thiserror::Error;

/// 根错误类型，覆盖 Core 库所有可能的失败场景
#[derive(Error, Debug)]
pub enum CoreError {
    #[error("图像处理错误: {0}")]
    Image(#[from] ImageError),

    #[error("EXIF 解析错误: {0}")]
    Exif(#[from] ExifError),

    #[error("字体错误: {0}")]
    Font(#[from] FontError),

    #[error("模板错误: {0}")]
    Template(#[from] TemplateError),

    #[error("渲染错误: {0}")]
    Render(#[from] RenderError),
}

/// 图像编解码错误
#[derive(Error, Debug)]
pub enum ImageError {
    #[error("解码失败: {0}")]
    Decode(String),

    #[error("编码失败: 格式 {format:?}")]
    Encode { format: String },

    #[error("不支持的图像格式")]
    UnsupportedFormat,

    #[error("HEIC/HEIF 解码失败: {0}")]
    HeicDecode(String),
}

/// EXIF 元数据解析错误
#[derive(Error, Debug)]
pub enum ExifError {
    #[error("EXIF 数据损坏或格式无效")]
    InvalidData,

    #[error("缺少必需的 EXIF 字段: {0}")]
    MissingField(String),
}

/// 字体加载与渲染错误
#[derive(Error, Debug)]
pub enum FontError {
    #[error("字体数据无效或损坏 (大小: {size} bytes)")]
    InvalidData { size: usize },

    #[error("无法解析字体: {reason}")]
    ParseFailed { reason: String },

    #[error("请求的字重 '{weight}' 未加载，已回退到常规体")]
    WeightNotAvailable { weight: String },
}

/// 模板解析与验证错误
#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("JSON 解析失败: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("模板验证失败: {0}")]
    Validation(String),

    #[error("未知模板: '{0}'")]
    UnknownTemplate(String),
}

/// 水印渲染过程中的错误
#[derive(Error, Debug)]
pub enum RenderError {
    #[error("Logo 加载失败: {0}")]
    LogoLoadFailed(String),

    #[error("布局计算失败: {0}")]
    LayoutFailed(String),

    #[error("颜色值无效: '{0}'")]
    InvalidColor(String),
}
