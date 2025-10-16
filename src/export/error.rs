use thiserror::Error;

/// 导出功能错误类型
/// 
/// 定义与PDSD导出功能相关的错误，包括文件操作、格式转换、资源访问等
#[derive(Debug, Error)]
pub enum ExportError {
    /// 文件操作错误
    #[error("文件操作错误: {operation} - {path}: {reason}")]
    FileOperation {
        /// 操作类型
        operation: String,
        /// 文件路径
        path: String,
        /// 错误原因
        reason: String,
    },
    
    /// 格式转换错误
    #[error("格式转换错误: 从 {from_format} 到 {to_format} - {description}")]
    FormatConversion {
        /// 源格式
        from_format: String,
        /// 目标格式
        to_format: String,
        /// 错误描述
        description: String,
    },
    
    /// 资源访问错误
    #[error("资源访问错误: {resource_type} - {resource_id}: {error}")]
    ResourceAccess {
        /// 资源类型
        resource_type: String,
        /// 资源ID
        resource_id: String,
        /// 错误描述
        error: String,
    },
    
    /// 导出配置错误
    #[error("导出配置错误: {setting} - {message}")]
    Configuration {
        /// 配置项
        setting: String,
        /// 错误消息
        message: String,
    },
    
    /// 导出数据错误
    #[error("导出数据错误: {data_type} - {issue}")]
    DataError {
        /// 数据类型
        data_type: String,
        /// 问题描述
        issue: String,
    },
    
    /// 导出过程取消
    #[error("导出过程已取消: {reason}")]
    Cancelled {
        /// 取消原因
        reason: Option<String>,
    },
    
    /// 导出模板错误
    #[error("导出模板错误: {template_name} - {description}")]
    Template {
        /// 模板名称
        template_name: String,
        /// 错误描述
        description: String,
    },
    
    /// 外部工具错误
    #[error("外部工具错误: {tool_name} - {error_message}")]
    ExternalTool {
        /// 工具名称
        tool_name: String,
        /// 错误消息
        error_message: String,
    },
}

/// 导出错误的辅助方法
impl ExportError {
    /// 创建文件操作错误
    pub fn file_operation(
        operation: impl Into<String>,
        path: impl Into<String>,
        reason: impl Into<String>
    ) -> Self {
        ExportError::FileOperation {
            operation: operation.into(),
            path: path.into(),
            reason: reason.into(),
        }
    }
    
    /// 创建格式转换错误
    pub fn format_conversion(
        from_format: impl Into<String>,
        to_format: impl Into<String>,
        description: impl Into<String>
    ) -> Self {
        ExportError::FormatConversion {
            from_format: from_format.into(),
            to_format: to_format.into(),
            description: description.into(),
        }
    }
    
    /// 创建资源访问错误
    pub fn resource_access(
        resource_type: impl Into<String>,
        resource_id: impl Into<String>,
        error: impl Into<String>
    ) -> Self {
        ExportError::ResourceAccess {
            resource_type: resource_type.into(),
            resource_id: resource_id.into(),
            error: error.into(),
        }
    }
    
    /// 创建配置错误
    pub fn configuration(setting: impl Into<String>, message: impl Into<String>) -> Self {
        ExportError::Configuration {
            setting: setting.into(),
            message: message.into(),
        }
    }
    
    /// 创建数据错误
    pub fn data_error(data_type: impl Into<String>, issue: impl Into<String>) -> Self {
        ExportError::DataError {
            data_type: data_type.into(),
            issue: issue.into(),
        }
    }
    
    /// 创建取消错误
    pub fn cancelled(reason: Option<impl Into<String>>) -> Self {
        ExportError::Cancelled {
            reason: reason.map(|r| r.into()),
        }
    }
    
    /// 创建模板错误
    pub fn template(template_name: impl Into<String>, description: impl Into<String>) -> Self {
        ExportError::Template {
            template_name: template_name.into(),
            description: description.into(),
        }
    }
    
    /// 创建外部工具错误
    pub fn external_tool(
        tool_name: impl Into<String>,
        error_message: impl Into<String>
    ) -> Self {
        ExportError::ExternalTool {
            tool_name: tool_name.into(),
            error_message: error_message.into(),
        }
    }
}

/// 导出模块的结果类型别名
pub type Result<T> = std::result::Result<T, ExportError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_operation_error() {
        let error = ExportError::file_operation(
            "写入", 
            "output/report.pdf", 
            "磁盘空间不足"
        );
        assert_eq!(error.to_string(), "文件操作错误: 写入 - output/report.pdf: 磁盘空间不足".to_string());
    }

    #[test]
    fn test_format_conversion_error() {
        let error = ExportError::format_conversion(
            "SVG", 
            "PNG", 
            "无法初始化渲染引擎"
        );
        assert_eq!(error.to_string(), "格式转换错误: 从 SVG 到 PNG - 无法初始化渲染引擎".to_string());
    }

    #[test]
    fn test_resource_access_error() {
        let error = ExportError::resource_access(
            "字体", 
            "simhei.ttf", 
            "找不到指定资源"
        );
        assert_eq!(error.to_string(), "资源访问错误: 字体 - simhei.ttf: 找不到指定资源".to_string());
    }

    #[test]
    fn test_configuration_error() {
        let error = ExportError::configuration(
            "页面尺寸", 
            "无效的尺寸值: 210x297a"
        );
        assert_eq!(error.to_string(), "导出配置错误: 页面尺寸 - 无效的尺寸值: 210x297a".to_string());
    }

    #[test]
    fn test_data_error() {
        let error = ExportError::data_error(
            "节点数据", 
            "包含无效的电气参数"
        );
        assert_eq!(error.to_string(), "导出数据错误: 节点数据 - 包含无效的电气参数".to_string());
    }

    #[test]
    fn test_cancelled_error() {
        // 有原因的情况
        let error_with_reason = ExportError::cancelled(Some("用户取消操作"));
        assert!(error_with_reason.to_string().contains("导出过程已取消: Some(\"用户取消操作\")"));
        
        // 无原因的情况
        let error_no_reason = ExportError::cancelled(None::<String>);
        assert!(error_no_reason.to_string().contains("导出过程已取消: None"));
    }

    #[test]
    fn test_template_error() {
        let error = ExportError::template(
            "标准报表", 
            "模板文件损坏"
        );
        assert_eq!(error.to_string(), "导出模板错误: 标准报表 - 模板文件损坏".to_string());
    }
}