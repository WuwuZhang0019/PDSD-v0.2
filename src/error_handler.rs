use crate::{error::PdsdError, application::app::PDSDApp};
use eframe::egui::{self, RichText};

/// 错误级别枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorLevel {
    /// 提示信息
    Info,
    /// 警告信息
    Warning,
    /// 错误信息
    Error,
    /// 严重错误
    Critical,
}

/// UI显示的错误消息
#[derive(Debug, Clone)]
pub struct UiErrorMessage {
    /// 错误级别
    pub level: ErrorLevel,
    /// 错误标题
    pub title: String,
    /// 详细错误消息
    pub message: String,
    /// 错误发生的上下文（模块、函数等）
    pub context: Option<String>,
    /// 是否可恢复的错误
    pub is_recoverable: bool,
    /// 错误代码（可选）
    pub code: Option<&'static str>,
}

impl UiErrorMessage {
    /// 创建新的UI错误消息
    pub fn new(
        level: ErrorLevel,
        title: impl Into<String>,
        message: impl Into<String>,
        context: Option<impl Into<String>>,
        is_recoverable: bool,
        code: Option<&'static str>,
    ) -> Self {
        Self {
            level,
            title: title.into(),
            message: message.into(),
            context: context.map(|c| c.into()),
            is_recoverable,
            code,
        }
    }
    
    /// 创建信息级别的消息
    pub fn info(
        title: impl Into<String>,
        message: impl Into<String>,
        context: Option<impl Into<String>>,
    ) -> Self {
        Self::new(
            ErrorLevel::Info,
            title,
            message,
            context,
            true,
            None,
        )
    }
    
    /// 创建警告级别的消息
    pub fn warning(
        title: impl Into<String>,
        message: impl Into<String>,
        context: Option<impl Into<String>>,
    ) -> Self {
        Self::new(
            ErrorLevel::Warning,
            title,
            message,
            context,
            true,
            None,
        )
    }
    
    /// 创建错误级别的消息
    pub fn error(
        title: impl Into<String>,
        message: impl Into<String>,
        context: Option<impl Into<String>>,
        code: Option<&'static str>,
    ) -> Self {
        Self::new(
            ErrorLevel::Error,
            title,
            message,
            context,
            true,
            code,
        )
    }
    
    /// 创建严重错误级别的消息
    pub fn critical(
        title: impl Into<String>,
        message: impl Into<String>,
        context: Option<impl Into<String>>,
        code: Option<&'static str>,
    ) -> Self {
        Self::new(
            ErrorLevel::Critical,
            title,
            message,
            context,
            false,
            code,
        )
    }
    
    /// 获取级别对应的颜色
    pub fn level_color(&self) -> egui::Color32 {
        match self.level {
            ErrorLevel::Info => egui::Color32::from_rgb(59, 130, 246), // 蓝色
            ErrorLevel::Warning => egui::Color32::from_rgb(245, 158, 11), // 橙色
            ErrorLevel::Error => egui::Color32::from_rgb(239, 68, 68), // 红色
            ErrorLevel::Critical => egui::Color32::from_rgb(153, 27, 27), // 深红色
        }
    }
    
    /// 获取级别对应的标签文本
    pub fn level_text(&self) -> &'static str {
        match self.level {
            ErrorLevel::Info => "信息",
            ErrorLevel::Warning => "警告",
            ErrorLevel::Error => "错误",
            ErrorLevel::Critical => "严重错误",
        }
    }
}

/// 错误处理器
pub trait ErrorHandler {
    /// 处理错误并返回UI友好的错误消息
    fn handle_error(&self, error: &PdsdError) -> UiErrorMessage;
    
    /// 显示错误对话框
    fn show_error_dialog(&self, ctx: &egui::Context, error: &UiErrorMessage);
    
    /// 显示错误通知
    fn show_error_notification(&self, ctx: &egui::Context, error: &UiErrorMessage);
    
    /// 记录错误到日志
    fn log_error(&self, error: &PdsdError, context: Option<&str>);
}

/// PDSD应用的错误处理器实现
pub struct PdsdErrorHandler;

impl ErrorHandler for PdsdErrorHandler {
    fn handle_error(&self, error: &PdsdError) -> UiErrorMessage {
        match error {
            PdsdError::Core(core_error) => {
                // 处理核心库错误
                match core_error {
                    crate::error::CoreError::Calculation(description) => {
                        UiErrorMessage::error(
                            "计算错误",
                            description.clone(),
                            None::<&str>,
                            Some("CORE-CALC"),
                        )
                    },
                    crate::error::CoreError::Validation { field, message } => {
                        UiErrorMessage::error(
                            "验证失败",
                            message.clone(),
                            Some(format!("字段: {}", field.clone())),
                            Some("CORE-VAL"),
                        )
                    },
                    crate::error::CoreError::Configuration(message) => {
                        UiErrorMessage::error(
                            "配置错误",
                            message.clone(),
                            None::<&str>,
                            Some("CORE-CONF"),
                        )
                    },
                    crate::error::CoreError::DataType(message) => {
                        UiErrorMessage::error(
                            "数据类型错误",
                            message.clone(),
                            None::<&str>,
                            Some("CORE-TYPE"),
                        )
                    },
                    // 确保覆盖所有可能的CoreError变体
                    crate::error::CoreError::DataType(_) => {
                        // 已经在上面处理过，这里不会执行
                        unreachable!()
                    }
                }
            },
            PdsdError::Application(app_error) => {
                // 处理应用程序错误
                match app_error {
                    crate::error::ApplicationError::Initialization(message) => {
                        UiErrorMessage::error(
                            "初始化失败",
                            message.clone(),
                            None::<&str>,
                            Some("APP-INIT"),
                        )
                    },
                    crate::error::ApplicationError::StateManagement(message) => {
                        UiErrorMessage::error(
                            "状态管理错误",
                            message.clone(),
                            None::<&str>,
                            Some("APP-STATE"),
                        )
                    },
                    crate::error::ApplicationError::Configuration(message) => {
                        UiErrorMessage::error(
                            "配置错误",
                            message.clone(),
                            None::<&str>,
                            Some("APP-CONF"),
                        )
                    },
                    crate::error::ApplicationError::Configuration(message) => {
                        UiErrorMessage::error(
                            "配置错误",
                            message.clone(),
                            None::<&str>,
                            Some("APP-CONF"),
                        )
                    },
                    // error.rs中的ApplicationError没有NotImplemented变体，暂时移除
                    // 添加一个通配符匹配来处理所有其他情况
                    _ => UiErrorMessage::error(
                        "应用程序错误",
                        app_error.to_string(),
                        None::<&str>,
                        Some("APP-GENERAL"),
                    )
                }
            },
            PdsdError::NodeGraph(node_graph_error) => {
                // 处理节点图库错误
                UiErrorMessage::error(
                    "节点图错误",
                    node_graph_error.to_string(),
                    None::<&str>,
                    Some("NODE-GRAPH"),
                )
            },
            PdsdError::Editor(editor_error) => {
                // 处理编辑器错误
                match editor_error {
                    crate::error::EditorError::NodeOperation(message) => {
                        UiErrorMessage::error(
                            "节点操作失败",
                            message.clone(),
                            None::<&str>,
                            Some("EDITOR-NODE"),
                        )
                    },
                    crate::error::EditorError::Connection(message) => {
                        UiErrorMessage::error(
                            "连接失败",
                            message.clone(),
                            None::<&str>,
                            Some("EDITOR-CONN"),
                        )
                    },
                    crate::error::EditorError::UiState(message) => {
                        UiErrorMessage::error(
                            "UI状态错误",
                            message.clone(),
                            None::<&str>,
                            Some("EDITOR-UI"),
                        )
                    },
                    crate::error::EditorError::GraphOperation(message) => {
                        UiErrorMessage::error(
                            "图操作错误",
                            message.clone(),
                            None::<&str>,
                            Some("EDITOR-GRAPH"),
                        )
                    },
                }
            },
            PdsdError::Export(export_error) => {
                // 处理导出错误
                match export_error {
                    crate::error::ExportError::FileOperation(message) => {
                        UiErrorMessage::error(
                            "文件操作失败",
                            message.clone(),
                            None::<&str>,
                            Some("EXPORT-FILE"),
                        )
                    },
                    crate::error::ExportError::FormatConversion(message) => {
                        UiErrorMessage::error(
                            "格式转换失败",
                            message.clone(),
                            None::<&str>,
                            Some("EXPORT-FORMAT"),
                        )
                    },
                    crate::error::ExportError::ResourceAccess(message) => {
                        UiErrorMessage::error(
                            "资源访问失败",
                            message.clone(),
                            None::<&str>,
                            Some("EXPORT-RES"),
                        )
                    },
                    crate::error::ExportError::Configuration(message) => {
                        UiErrorMessage::error(
                            "导出配置错误",
                            message.clone(),
                            None::<&str>,
                            Some("EXPORT-CONF"),
                        )
                    },
                }
            },
            PdsdError::Io(io_error) => {
                UiErrorMessage::error(
                    "文件I/O错误",
                    io_error.to_string(),
                    Some("系统文件操作".to_string()),
                    Some("IO-ERROR"),
                )
            },
            PdsdError::Serde(serde_error) => {
                UiErrorMessage::error(
                    "序列化/反序列化错误",
                    serde_error.to_string(),
                    Some("数据处理".to_string()),
                    Some("SERDE-ERROR"),
                )
            },
            PdsdError::Generic(message) => {
                UiErrorMessage::error(
                    "通用错误",
                    message.clone(),
                    None::<&str>,
                    Some("GENERIC"),
                )
            },
            PdsdError::Unknown(message) => {
                UiErrorMessage::error(
                    "未知错误",
                    message.clone(),
                    None::<&str>,
                    Some("UNKNOWN"),
                )
            }
        }
    }
    
    fn show_error_dialog(&self, ctx: &egui::Context, error: &UiErrorMessage) {
        // 显示模态对话框
        egui::Window::new(format!("{}", error.level_text()))
            .collapsible(false)
            .resizable(true)
            .default_width(450.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // 显示错误标题和级别
                    ui.heading(
                        RichText::new(&error.title)
                            .color(error.level_color())
                            .strong()
                    );
                    
                    ui.separator();
                    
                    // 显示错误消息
                    ui.label("详细信息:");
                    ui.text_edit_multiline(&mut error.message.clone());
                    
                    // 显示上下文信息
                    if let Some(context) = &error.context {
                        ui.collapsing("错误上下文", |ui| {
                            ui.label(context);
                        });
                    }
                    
                    // 显示错误代码
                    if let Some(code) = error.code {
                        ui.label(RichText::new(format!("错误代码: {}", code)).small());
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("确定").clicked() {
                            ui.close();
                        }
                    });
                });
            });
    }
    
    fn show_error_notification(&self, _ctx: &egui::Context, error: &UiErrorMessage) {
        // 使用egui 0.32.3支持的方式显示通知
        let message = format!("{}: {}", error.title, error.message);
        
        // 在egui 0.32.3中，我们使用一个简单的方式来处理通知
        // 对于简单实现，我们可以直接打印到控制台
        println!("[NOTIFICATION] {}", message);
    }
    
    fn log_error(&self, error: &PdsdError, context: Option<&str>) {
        match error {
            PdsdError::Core(core_error) => {
                let context_str = context.unwrap_or("核心库");
                println!("[错误] [{}] {}", context_str, core_error);
                // 在实际应用中，这里应该使用适当的日志记录库
            },
            PdsdError::NodeGraph(node_graph_error) => {
                let context_str = context.unwrap_or("节点图库");
                println!("[错误] [{}] {}", context_str, node_graph_error);
                // 在实际应用中，这里应该使用适当的日志记录库
            },
            _ => {
                // 处理其他类型的错误
                eprintln!("[ERROR] {}", error);
                if let Some(ctx) = context {
                    eprintln!("[CONTEXT] {}", ctx);
                }
            }
        }
    }
}

/// 为PDSDApp实现错误处理的辅助方法
impl PDSDApp {
    /// 处理错误并显示
    pub fn handle_app_error(&mut self, ctx: &egui::Context, error: PdsdError, show_dialog: bool, context: Option<&str>) {
        let error_handler = PdsdErrorHandler;
        
        // 记录错误
        error_handler.log_error(&error, context);
        
        // 转换为UI错误消息
        let ui_error = error_handler.handle_error(&error);
        
        // 根据配置显示错误
        if show_dialog {
            error_handler.show_error_dialog(ctx, &ui_error);
        } else {
            error_handler.show_error_notification(ctx, &ui_error);
        }
    }
    
    /// 显示错误通知
    pub fn show_error(&mut self, ctx: &egui::Context, message: impl Into<String>) {
        let error = PdsdError::Generic(message.into());
        self.handle_app_error(ctx, error, false, None);
    }
    
    /// 显示警告通知
    pub fn show_warning(&mut self, ctx: &egui::Context, message: impl Into<String>) {
        let ui_error = UiErrorMessage::warning("警告", message.into(), None::<&str>);
        let error_handler = PdsdErrorHandler;
        error_handler.show_error_notification(ctx, &ui_error);
    }
    
    /// 显示信息通知
    pub fn show_info(&mut self, ctx: &egui::Context, message: impl Into<String>) {
        let ui_error = UiErrorMessage::info("信息", message.into(), None::<&str>);
        let error_handler = PdsdErrorHandler;
        error_handler.show_error_notification(ctx, &ui_error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CoreError;
    use egui_node_graph::{EguiGraphError, NodeId};
    
    #[test]
    fn test_error_handling() {
        let error_handler = PdsdErrorHandler;
        
        // 测试核心库计算错误处理
        let core_error = CoreError::Calculation("电流计算溢出".to_string());
        let pdsd_error = PdsdError::from(core_error);
        let ui_error = error_handler.handle_error(&pdsd_error);
        
        assert_eq!(ui_error.title, "计算错误");
        assert_eq!(ui_error.message, "电流计算溢出");
        assert!(ui_error.context.is_some());
        assert_eq!(ui_error.code, Some("CORE-CALC"));
        assert_eq!(ui_error.level, ErrorLevel::Error);
    }
    
    #[test]
    fn test_node_graph_error_handling() {
        let error_handler = PdsdErrorHandler;
        
        // 测试节点图库错误处理
        let node_id = NodeId::default();
        let node_graph_error = EguiGraphError::NoParameterNamed(node_id, "test_param".to_string());
        let pdsd_error = PdsdError::from(node_graph_error);
        let ui_error = error_handler.handle_error(&pdsd_error);
        
        assert_eq!(ui_error.title, "节点图错误");
        assert!(ui_error.message.contains("节点"));
        assert!(ui_error.message.contains("test_param"));
        assert_eq!(ui_error.code, Some("NODE-GRAPH"));
        assert_eq!(ui_error.level, ErrorLevel::Error);
    }
    
    #[test]
    fn test_ui_error_message_constructors() {
        // 测试信息消息构造
        let info = UiErrorMessage::info("操作成功", "文件已保存", Some("模块: 文件系统".to_string()));
        assert_eq!(info.level, ErrorLevel::Info);
        assert_eq!(info.title, "操作成功");
        
        // 测试警告消息构造
        let warning = UiErrorMessage::warning("注意", "文件已存在", Some("操作: 保存".to_string()));
        assert_eq!(warning.level, ErrorLevel::Warning);
        
        // 测试错误消息构造
        let error = UiErrorMessage::error("错误", "无法保存", Some("原因: 权限不足".to_string()), Some("ERR-001"));
        assert_eq!(error.level, ErrorLevel::Error);
        assert_eq!(error.code, Some("ERR-001"));
        
        // 测试严重错误构造
        let critical = UiErrorMessage::critical("严重错误", "程序崩溃", Some("模块: 内核".to_string()), Some("CRITICAL-001"));
        assert_eq!(critical.level, ErrorLevel::Critical);
        assert!(!critical.is_recoverable);
    }
}