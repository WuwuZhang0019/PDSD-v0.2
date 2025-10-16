use thiserror::Error;

/// PDSD项目的根错误类型
/// 
/// 作为所有模块错误的统一聚合点，提供一致的错误处理入口
/// 支持从标准库错误和第三方库错误自动转换
#[derive(Debug, Error)]
pub enum PdsdError {
    /// 核心库错误
    #[error(transparent)]
    Core(#[from] CoreError),
    
    /// 应用程序错误
    #[error(transparent)]
    Application(#[from] ApplicationError),
    
    /// 编辑器错误
    #[error(transparent)]
    Editor(#[from] EditorError),
    
    /// 导出功能错误
    #[error(transparent)]
    Export(#[from] ExportError),
    
    /// 节点图库错误
    #[error(transparent)]
    NodeGraph(#[from] egui_node_graph::EguiGraphError),
    
    /// 标准库I/O错误
    #[error(transparent)]
    Io(#[from] std::io::Error),
    
    /// JSON序列化/反序列化错误
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    
    /// 通用错误，包含自定义错误消息
    #[error("通用错误: {0}")]
    Generic(String),
    
    /// 未知错误
    #[error("未知错误: {0}")]
    Unknown(String),
}

/// 核心库错误
/// 
/// 定义核心计算和数据处理相关的错误
#[derive(Debug, Error)]
pub enum CoreError {
    /// 计算错误
    #[error("计算错误: {0}")]
    Calculation(String),
    
    /// 验证错误
    #[error("验证错误: {field}: {message}")]
    Validation {
        /// 验证失败的字段
        field: String,
        /// 错误消息
        message: String,
    },
    
    /// 配置错误
    #[error("配置错误: {0}")]
    Configuration(String),
    
    /// 数据类型错误
    #[error("数据类型错误: {0}")]
    DataType(String),
}

/// 应用程序错误
/// 
/// 定义应用程序层面的错误
#[derive(Debug, Error)]
pub enum ApplicationError {
    /// 初始化错误
    #[error("初始化错误: {0}")]
    Initialization(String),
    
    /// 状态管理错误
    #[error("状态管理错误: {0}")]
    StateManagement(String),
    
    /// 配置错误
    #[error("应用配置错误: {0}")]
    Configuration(String),
}

/// 编辑器错误
/// 
/// 定义编辑器功能相关的错误
#[derive(Debug, Error)]
pub enum EditorError {
    /// 节点操作错误
    #[error("节点操作错误: {0}")]
    NodeOperation(String),
    
    /// 连接错误
    #[error("连接错误: {0}")]
    Connection(String),
    
    /// UI状态错误
    #[error("UI状态错误: {0}")]
    UiState(String),
    
    /// 图操作错误
    #[error("图操作错误: {0}")]
    GraphOperation(String),
}

/// 导出功能错误
/// 
/// 定义导出相关的错误
#[derive(Debug, Error)]
pub enum ExportError {
    /// 文件操作错误
    #[error("文件操作错误: {0}")]
    FileOperation(String),
    
    /// 格式转换错误
    #[error("格式转换错误: {0}")]
    FormatConversion(String),
    
    /// 资源访问错误
    #[error("资源访问错误: {0}")]
    ResourceAccess(String),
    
    /// 导出配置错误
    #[error("导出配置错误: {0}")]
    Configuration(String),
}

/// 类型别名，简化错误处理
pub type Result<T> = std::result::Result<T, PdsdError>;

/// 辅助函数：创建通用错误
pub fn generic_error(msg: impl Into<String>) -> PdsdError {
    PdsdError::Generic(msg.into())
}

/// 辅助函数：创建未知错误
pub fn unknown_error(msg: impl Into<String>) -> PdsdError {
    PdsdError::Unknown(msg.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_node_graph::{EguiGraphError, AnyParameterId, NodeId};

    #[test]
    fn test_error_creation() {
        // 测试核心错误创建
        let core_error = CoreError::Calculation("计算溢出".to_string());
        assert_eq!(core_error.to_string(), "计算错误: 计算溢出".to_string());

        // 测试应用错误创建
        let app_error = ApplicationError::Initialization("初始化失败".to_string());
        assert_eq!(app_error.to_string(), "初始化错误: 初始化失败".to_string());

        // 测试编辑器错误创建
        let editor_error = EditorError::NodeOperation("节点不存在".to_string());
        assert_eq!(editor_error.to_string(), "节点操作错误: 节点不存在".to_string());

        // 测试导出错误创建
        let export_error = ExportError::FileOperation("文件未找到".to_string());
        assert_eq!(export_error.to_string(), "文件操作错误: 文件未找到".to_string());
    }

    #[test]
    fn test_error_conversion() {
        // 测试核心错误转换为PdsdError
        let core_error = CoreError::Calculation("计算溢出".to_string());
        let pdsd_error: PdsdError = core_error.into();
        assert_eq!(pdsd_error.to_string(), "计算错误: 计算溢出".to_string());

        // 测试验证错误创建
        let validation_error = CoreError::Validation {
            field: "current".to_string(),
            message: "电流值必须大于0".to_string(),
        };
        assert_eq!(validation_error.to_string(), "验证错误: current: 电流值必须大于0".to_string());

        // 测试节点图库错误转换
        let node_id = NodeId::default();
        let node_graph_error = EguiGraphError::NoParameterNamed(node_id, "test_param".to_string());
        let pdsd_error: PdsdError = node_graph_error.into();
        assert!(pdsd_error.to_string().contains("节点") && pdsd_error.to_string().contains("test_param"));
    }

    #[test]
    fn test_helper_functions() {
        // 测试通用错误辅助函数
        let generic_err = generic_error("发生错误");
        assert_eq!(generic_err.to_string(), "通用错误: 发生错误".to_string());

        // 测试未知错误辅助函数
        let unknown_err = unknown_error("未知情况");
        assert_eq!(unknown_err.to_string(), "未知错误: 未知情况".to_string());
    }
}