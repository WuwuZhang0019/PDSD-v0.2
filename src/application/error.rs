use thiserror::Error;

/// 应用程序错误类型
/// 
/// 定义与PDSD应用程序层面相关的错误，包括初始化、配置和状态管理等
#[derive(Debug, Error)]
pub enum ApplicationError {
    /// 应用初始化错误
    #[error("初始化错误: {component} - {description}")]
    Initialization {
        /// 初始化失败的组件
        component: String,
        /// 错误描述
        description: String,
    },
    
    /// 状态管理错误
    #[error("状态管理错误: {action} - {description}")]
    StateManagement {
        /// 执行的操作
        action: String,
        /// 错误描述
        description: String,
    },
    
    /// 应用配置错误
    #[error("配置错误: {section} - {message}")]
    Configuration {
        /// 配置部分
        section: String,
        /// 错误消息
        message: String,
    },
    
    /// 资源加载错误
    #[error("资源加载错误: {resource_type} - {resource_name}")]
    ResourceLoading {
        /// 资源类型
        resource_type: String,
        /// 资源名称
        resource_name: String,
        /// 可选的详细错误
        source: Option<String>,
    },
    
    /// 命令执行错误
    #[error("命令执行错误: {command} - {description}")]
    CommandExecution {
        /// 执行的命令
        command: String,
        /// 错误描述
        description: String,
    },
    
    /// 版本兼容性错误
    #[error("版本兼容性错误: 项目文件版本 {file_version} 与应用版本 {app_version} 不兼容")]
    VersionCompatibility {
        /// 文件版本
        file_version: String,
        /// 应用版本
        app_version: String,
    },
    
    /// 功能未实现错误
    #[error("功能未实现: {feature_name} - {reason}")]
    NotImplemented {
        /// 功能名称
        feature_name: String,
        /// 未实现原因
        reason: Option<String>,
    },
    
    /// 权限错误
    #[error("权限错误: {action} - {resource}")]
    Permission {
        /// 尝试的操作
        action: String,
        /// 访问的资源
        resource: String,
    },
}

/// 应用程序错误的辅助方法
impl ApplicationError {
    /// 创建初始化错误
    pub fn initialization(component: impl Into<String>, description: impl Into<String>) -> Self {
        ApplicationError::Initialization {
            component: component.into(),
            description: description.into(),
        }
    }
    
    /// 创建状态管理错误
    pub fn state_management(action: impl Into<String>, description: impl Into<String>) -> Self {
        ApplicationError::StateManagement {
            action: action.into(),
            description: description.into(),
        }
    }
    
    /// 创建配置错误
    pub fn configuration(section: impl Into<String>, message: impl Into<String>) -> Self {
        ApplicationError::Configuration {
            section: section.into(),
            message: message.into(),
        }
    }
    
    /// 创建资源加载错误
    pub fn resource_loading(
        resource_type: impl Into<String>,
        resource_name: impl Into<String>,
        source: Option<impl Into<String>>
    ) -> Self {
        ApplicationError::ResourceLoading {
            resource_type: resource_type.into(),
            resource_name: resource_name.into(),
            source: source.map(|s| s.into()),
        }
    }
    
    /// 创建命令执行错误
    pub fn command_execution(command: impl Into<String>, description: impl Into<String>) -> Self {
        ApplicationError::CommandExecution {
            command: command.into(),
            description: description.into(),
        }
    }
    
    /// 创建版本兼容性错误
    pub fn version_compatibility(
        file_version: impl Into<String>,
        app_version: impl Into<String>
    ) -> Self {
        ApplicationError::VersionCompatibility {
            file_version: file_version.into(),
            app_version: app_version.into(),
        }
    }
    
    /// 创建功能未实现错误
    pub fn not_implemented(
        feature_name: impl Into<String>,
        reason: Option<impl Into<String>>
    ) -> Self {
        ApplicationError::NotImplemented {
            feature_name: feature_name.into(),
            reason: reason.map(|r| r.into()),
        }
    }
    
    /// 创建权限错误
    pub fn permission(action: impl Into<String>, resource: impl Into<String>) -> Self {
        ApplicationError::Permission {
            action: action.into(),
            resource: resource.into(),
        }
    }
}

/// 应用程序的结果类型别名
pub type Result<T> = std::result::Result<T, ApplicationError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization_error() {
        let error = ApplicationError::initialization("字体系统", "无法加载默认字体");
        assert_eq!(error.to_string(), "初始化错误: 字体系统 - 无法加载默认字体".to_string());
    }

    #[test]
    fn test_state_management_error() {
        let error = ApplicationError::state_management("保存", "无法序列化状态数据");
        assert_eq!(error.to_string(), "状态管理错误: 保存 - 无法序列化状态数据".to_string());
    }

    #[test]
    fn test_configuration_error() {
        let error = ApplicationError::configuration("UI", "找不到主题配置文件");
        assert_eq!(error.to_string(), "配置错误: UI - 找不到主题配置文件".to_string());
    }

    #[test]
    fn test_resource_loading_error() {
        // 有source的情况
        let error_with_source = ApplicationError::resource_loading(
            "图标", 
            "save.svg", 
            Some("文件不存在")
        );
        assert!(error_with_source.to_string().contains("资源加载错误: 图标 - save.svg"));
        
        // 没有source的情况
        let error_no_source = ApplicationError::resource_loading(
            "模板", 
            "default.json", 
            None::<String>
        );
        assert!(error_no_source.to_string().contains("资源加载错误: 模板 - default.json"));
    }

    #[test]
    fn test_version_compatibility_error() {
        let error = ApplicationError::version_compatibility("2.0", "1.5");
        assert_eq!(error.to_string(), "版本兼容性错误: 项目文件版本 2.0 与应用版本 1.5 不兼容".to_string());
    }

    #[test]
    fn test_not_implemented_error() {
        // 有reason的情况
        let error_with_reason = ApplicationError::not_implemented(
            "导入DWG", 
            Some("此功能在后续版本中提供")
        );
        assert!(error_with_reason.to_string().contains("功能未实现: 导入DWG - Some(\"此功能在后续版本中提供\")"));
        
        // 没有reason的情况
        let error_no_reason = ApplicationError::not_implemented(
            "批量处理", 
            None::<String>
        );
        assert!(error_no_reason.to_string().contains("功能未实现: 批量处理 - None"));
    }
}