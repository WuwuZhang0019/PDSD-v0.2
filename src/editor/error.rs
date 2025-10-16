use thiserror::Error;

/// 编辑器错误类型
/// 
/// 定义与PDSD节点编辑器相关的错误，包括节点操作、连接管理、UI状态等
#[derive(Debug, Error)]
pub enum EditorError {
    /// 节点操作错误
    #[error("节点操作错误: {operation} - {description}")]
    NodeOperation {
        /// 执行的操作
        operation: String,
        /// 错误描述
        description: String,
        /// 可选的节点ID
        node_id: Option<String>,
    },
    
    /// 连接错误
    #[error("连接错误: {type_} - {message}")]
    Connection {
        /// 连接类型或操作
        type_: String,
        /// 错误消息
        message: String,
        /// 源节点ID（可选）
        source_node: Option<String>,
        /// 目标节点ID（可选）
        target_node: Option<String>,
    },
    
    /// UI状态错误
    #[error("UI状态错误: {component} - {state}")]
    UiState {
        /// UI组件名称
        component: String,
        /// 状态描述
        state: String,
    },
    
    /// 图操作错误
    #[error("图操作错误: {action} - {description}")]
    GraphOperation {
        /// 操作名称
        action: String,
        /// 错误描述
        description: String,
    },
    
    /// 选择错误
    #[error("选择错误: {reason}")]
    Selection {
        /// 错误原因
        reason: String,
    },
    
    /// 拖拽操作错误
    #[error("拖拽操作错误: {phase} - {issue}")]
    DragDrop {
        /// 拖拽阶段
        phase: String,
        /// 问题描述
        issue: String,
    },
    
    /// 布局错误
    #[error("布局错误: {algorithm} - {description}")]
    Layout {
        /// 布局算法
        algorithm: String,
        /// 错误描述
        description: String,
    },
    
    /// 工具操作错误
    #[error("工具操作错误: {tool_name} - {error}")]
    ToolOperation {
        /// 工具名称
        tool_name: String,
        /// 错误描述
        error: String,
    },
}

/// 编辑器错误的辅助方法
impl EditorError {
    /// 创建节点操作错误
    pub fn node_operation(
        operation: impl Into<String>,
        description: impl Into<String>,
        node_id: Option<impl Into<String>>
    ) -> Self {
        EditorError::NodeOperation {
            operation: operation.into(),
            description: description.into(),
            node_id: node_id.map(|id| id.into()),
        }
    }
    
    /// 创建连接错误
    pub fn connection(
        type_: impl Into<String>,
        message: impl Into<String>,
        source_node: Option<impl Into<String>>,
        target_node: Option<impl Into<String>>
    ) -> Self {
        EditorError::Connection {
            type_: type_.into(),
            message: message.into(),
            source_node: source_node.map(|id| id.into()),
            target_node: target_node.map(|id| id.into()),
        }
    }
    
    /// 创建UI状态错误
    pub fn ui_state(component: impl Into<String>, state: impl Into<String>) -> Self {
        EditorError::UiState {
            component: component.into(),
            state: state.into(),
        }
    }
    
    /// 创建图操作错误
    pub fn graph_operation(action: impl Into<String>, description: impl Into<String>) -> Self {
        EditorError::GraphOperation {
            action: action.into(),
            description: description.into(),
        }
    }
    
    /// 创建选择错误
    pub fn selection(reason: impl Into<String>) -> Self {
        EditorError::Selection {
            reason: reason.into(),
        }
    }
    
    /// 创建拖拽操作错误
    pub fn drag_drop(phase: impl Into<String>, issue: impl Into<String>) -> Self {
        EditorError::DragDrop {
            phase: phase.into(),
            issue: issue.into(),
        }
    }
    
    /// 创建布局错误
    pub fn layout(algorithm: impl Into<String>, description: impl Into<String>) -> Self {
        EditorError::Layout {
            algorithm: algorithm.into(),
            description: description.into(),
        }
    }
    
    /// 创建工具操作错误
    pub fn tool_operation(tool_name: impl Into<String>, error: impl Into<String>) -> Self {
        EditorError::ToolOperation {
            tool_name: tool_name.into(),
            error: error.into(),
        }
    }
}

/// 编辑器的结果类型别名
pub type Result<T> = std::result::Result<T, EditorError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_operation_error() {
        // 有节点ID的情况
        let error_with_id = EditorError::node_operation(
            "创建", 
            "节点类型不存在", 
            Some("panel_1")
        );
        assert!(error_with_id.to_string().contains("节点操作错误: 创建 - 节点类型不存在"));
        
        // 没有节点ID的情况
        let error_no_id = EditorError::node_operation(
            "删除", 
            "无法删除锁定的节点", 
            None::<String>
        );
        assert!(error_no_id.to_string().contains("节点操作错误: 删除 - 无法删除锁定的节点"));
    }

    #[test]
    fn test_connection_error() {
        let error = EditorError::connection(
            "创建", 
            "输入端口已连接", 
            Some("source_1"), 
            Some("target_1")
        );
        assert!(error.to_string().contains("连接错误: 创建 - 输入端口已连接"));
    }

    #[test]
    fn test_ui_state_error() {
        let error = EditorError::ui_state("属性面板", "无法显示未知节点类型的属性");
        assert_eq!(error.to_string(), "UI状态错误: 属性面板 - 无法显示未知节点类型的属性".to_string());
    }

    #[test]
    fn test_graph_operation_error() {
        let error = EditorError::graph_operation("保存", "图包含无效连接");
        assert_eq!(error.to_string(), "图操作错误: 保存 - 图包含无效连接".to_string());
    }

    #[test]
    fn test_selection_error() {
        let error = EditorError::selection("无法选择锁定的节点");
        assert_eq!(error.to_string(), "选择错误: 无法选择锁定的节点".to_string());
    }

    #[test]
    fn test_layout_error() {
        let error = EditorError::layout("自动布局", "节点数量过多，无法计算布局");
        assert_eq!(error.to_string(), "布局错误: 自动布局 - 节点数量过多，无法计算布局".to_string());
    }
}