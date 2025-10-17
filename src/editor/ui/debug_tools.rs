/// 调试工具和日志系统
/// 用于开发过程中的问题诊断和性能监控

use egui::{Ui, ScrollArea, TextEdit, Button, Checkbox, Separator, CollapsingHeader};
use std::collections::VecDeque;
use std::sync::Mutex;
use chrono::{Local, DateTime};
use serde::{Serialize, Deserialize};

/// 日志级别\#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    /// 错误
    Error,
    /// 警告
    Warning,
    /// 信息
    Info,
    /// 调试
    Debug,
    /// 追踪
    Trace,
}

impl LogLevel {
    /// 获取日志级别对应的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            LogLevel::Error => "错误",
            LogLevel::Warning => "警告",
            LogLevel::Info => "信息",
            LogLevel::Debug => "调试",
            LogLevel::Trace => "追踪",
        }
    }
    
    /// 获取日志级别对应的颜色
    pub fn color(&self) -> egui::Color32 {
        match self {
            LogLevel::Error => egui::Color32::RED,
            LogLevel::Warning => egui::Color32::YELLOW,
            LogLevel::Info => egui::Color32::BLUE,
            LogLevel::Debug => egui::Color32::GREEN,
            LogLevel::Trace => egui::Color32::GRAY,
        }
    }
}

/// 日志条目\#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// 时间戳
    pub timestamp: DateTime<Local>,
    /// 日志级别
    pub level: LogLevel,
    /// 消息内容
    pub message: String,
    /// 来源模块
    pub module: String,
    /// 来源文件
    pub file: String,
    /// 行号
    pub line: u32,
    /// 附加数据（可选）
    pub data: Option<String>,
}

/// 全局日志系统
pub struct Logger {
    /// 日志缓冲区
    logs: Mutex<VecDeque<LogEntry>>,
    /// 最大日志条目数
    max_entries: usize,
    /// 当前日志级别过滤
    log_level: Mutex<LogLevel>,
    /// 是否自动滚动
    auto_scroll: Mutex<bool>,
}

impl Logger {
    /// 创建新的日志系统
    pub fn new(max_entries: usize) -> Self {
        Self {
            logs: Mutex::new(VecDeque::with_capacity(max_entries)),
            max_entries,
            log_level: Mutex::new(LogLevel::Info),
            auto_scroll: Mutex::new(true),
        }
    }
    
    /// 记录错误日志
    pub fn error(&self, message: &str, module: &str, file: &str, line: u32, data: Option<String>) {
        self.log(LogLevel::Error, message, module, file, line, data);
    }
    
    /// 记录警告日志
    pub fn warn(&self, message: &str, module: &str, file: &str, line: u32, data: Option<String>) {
        self.log(LogLevel::Warning, message, module, file, line, data);
    }
    
    /// 记录信息日志
    pub fn info(&self, message: &str, module: &str, file: &str, line: u32, data: Option<String>) {
        self.log(LogLevel::Info, message, module, file, line, data);
    }
    
    /// 记录调试日志
    pub fn debug(&self, message: &str, module: &str, file: &str, line: u32, data: Option<String>) {
        self.log(LogLevel::Debug, message, module, file, line, data);
    }
    
    /// 记录追踪日志
    pub fn trace(&self, message: &str, module: &str, file: &str, line: u32, data: Option<String>) {
        self.log(LogLevel::Trace, message, module, file, line, data);
    }
    
    /// 通用日志记录方法
    pub fn log(&self, level: LogLevel, message: &str, module: &str, file: &str, line: u32, data: Option<String>) {
        // 检查是否应该记录此级别的日志
        if level > *self.log_level.lock().unwrap() {
            return;
        }
        
        let entry = LogEntry {
            timestamp: Local::now(),
            level,
            message: message.to_string(),
            module: module.to_string(),
            file: file.to_string(),
            line,
            data: data.map(|s| s.to_string()),
        };
        
        let mut logs = self.logs.lock().unwrap();
        
        // 添加新日志
        logs.push_back(entry);
        
        // 如果超出最大条目数，移除最旧的日志
        if logs.len() > self.max_entries {
            logs.pop_front();
        }
    }
    
    /// 设置日志级别
    pub fn set_log_level(&self, level: LogLevel) {
        *self.log_level.lock().unwrap() = level;
    }
    
    /// 获取当前日志级别
    pub fn get_log_level(&self) -> LogLevel {
        *self.log_level.lock().unwrap()
    }
    
    /// 设置自动滚动
    pub fn set_auto_scroll(&self, auto_scroll: bool) {
        *self.auto_scroll.lock().unwrap() = auto_scroll;
    }
    
    /// 获取自动滚动设置
    pub fn get_auto_scroll(&self) -> bool {
        *self.auto_scroll.lock().unwrap()
    }
    
    /// 获取所有日志条目
    pub fn get_logs(&self) -> Vec<LogEntry> {
        self.logs.lock().unwrap().clone().into_iter().collect()
    }
    
    /// 清空日志
    pub fn clear_logs(&self) {
        self.logs.lock().unwrap().clear();
    }
    
    /// 导出日志到文件
    pub fn export_logs(&self, file_path: &str) -> Result<(), std::io::Error> {
        let logs = self.get_logs();
        let mut file = std::fs::File::create(file_path)?;
        
        for log in logs {
            let log_str = format!(
                "[{}] [{}] [{}:{}] {}\n",
                log.timestamp.format("%Y-%m-%d %H:%M:%S%.3f"),
                log.level.display_name(),
                log.file,
                log.line,
                log.message
            );
            file.write_all(log_str.as_bytes())?;
        }
        
        Ok(())
    }
}

// 创建全局日志实例
lazy_static::lazy_static! {
    pub static ref LOGGER: Logger = Logger::new(1000);
}

/// 日志宏定义
#[macro_export]
macro_rules! log_error {
    ($message:expr $(, $data:expr)?) => {
        $crate::editor::ui::debug_tools::LOGGER.error(
            $message,
            module_path!(),
            file!(),
            line!(),
            $(Some($data.to_string()))?
        );
    };
}

#[macro_export]
macro_rules! log_warn {
    ($message:expr $(, $data:expr)?) => {
        $crate::editor::ui::debug_tools::LOGGER.warn(
            $message,
            module_path!(),
            file!(),
            line!(),
            $(Some($data.to_string()))?
        );
    };
}

#[macro_export]
macro_rules! log_info {
    ($message:expr $(, $data:expr)?) => {
        $crate::editor::ui::debug_tools::LOGGER.info(
            $message,
            module_path!(),
            file!(),
            line!(),
            $(Some($data.to_string()))?
        );
    };
}

#[macro_export]
macro_rules! log_debug {
    ($message:expr $(, $data:expr)?) => {
        $crate::editor::ui::debug_tools::LOGGER.debug(
            $message,
            module_path!(),
            file!(),
            line!(),
            $(Some($data.to_string()))?
        );
    };
}

#[macro_export]
macro_rules! log_trace {
    ($message:expr $(, $data:expr)?) => {
        $crate::editor::ui::debug_tools::LOGGER.trace(
            $message,
            module_path!(),
            file!(),
            line!(),
            $(Some($data.to_string()))?
        );
    };
}

/// 日志面板UI
pub fn log_panel_ui(ui: &mut Ui, logger: &Logger) {
    ui.group(|ui| {
        ui.heading("日志面板");
        
        // 日志控制栏
        ui.horizontal(|ui| {
            // 日志级别选择
            ui.label("日志级别:");
            
            let current_level = logger.get_log_level();
            let mut new_level = current_level;
            
            if ui.radio(current_level == LogLevel::Error, "错误").clicked() {
                new_level = LogLevel::Error;
            }
            if ui.radio(current_level == LogLevel::Warning, "警告").clicked() {
                new_level = LogLevel::Warning;
            }
            if ui.radio(current_level == LogLevel::Info, "信息").clicked() {
                new_level = LogLevel::Info;
            }
            if ui.radio(current_level == LogLevel::Debug, "调试").clicked() {
                new_level = LogLevel::Debug;
            }
            if ui.radio(current_level == LogLevel::Trace, "追踪").clicked() {
                new_level = LogLevel::Trace;
            }
            
            if new_level != current_level {
                logger.set_log_level(new_level);
            }
            
            // 清空按钮
            if ui.button("清空").clicked() {
                logger.clear_logs();
            }
            
            // 自动滚动
            let mut auto_scroll = logger.get_auto_scroll();
            if ui.checkbox(&mut auto_scroll, "自动滚动").changed() {
                logger.set_auto_scroll(auto_scroll);
            }
        });
        
        Separator::default().ui(ui);
        
        // 日志内容显示
        ScrollArea::vertical().show(ui, |ui| {
            let logs = logger.get_logs();
            
            for log in logs {
                ui.collapsing(format!("[{}] {} - {}", 
                    log.timestamp.format("%H:%M:%S%.3f"),
                    log.level.display_name(),
                    log.message
                ), |ui| {
                    ui.label("详细信息:");
                    ui.label(format!("时间: {}", log.timestamp.format("%Y-%m-%d %H:%M:%S%.3f")));
                    ui.label(format!("级别: {}", log.level.display_name()));
                    ui.label(format!("模块: {}", log.module));
                    ui.label(format!("位置: {}:{}", log.file, log.line));
                    
                    if let Some(data) = &log.data {
                        ui.label("附加数据:");
                        ui.code(data);
                    }
                });
            }
        });
    });
}

/// 性能监控工具\#[derive(Debug, Default)]
pub struct PerformanceMonitor {
    /// 帧率记录
    pub fps_history: VecDeque<f32>,
    /// 各操作耗时记录
    pub operation_times: std::collections::HashMap<String, VecDeque<f64>>,
    /// 最大历史记录数
    max_history: usize,
}

impl PerformanceMonitor {
    /// 创建新的性能监控工具
    pub fn new(max_history: usize) -> Self {
        Self {
            fps_history: VecDeque::with_capacity(max_history),
            operation_times: std::collections::HashMap::new(),
            max_history,
        }
    }
    
    /// 记录帧率
    pub fn record_fps(&mut self, fps: f32) {
        self.fps_history.push_back(fps);
        if self.fps_history.len() > self.max_history {
            self.fps_history.pop_front();
        }
    }
    
    /// 记录操作耗时
    pub fn record_operation_time(&mut self, operation_name: &str, time_ms: f64) {
        let times = self.operation_times.entry(operation_name.to_string())
            .or_insert_with(|| VecDeque::with_capacity(self.max_history));
            
        times.push_back(time_ms);
        if times.len() > self.max_history {
            times.pop_front();
        }
    }
    
    /// 获取平均帧率
    pub fn get_average_fps(&self) -> f32 {
        if self.fps_history.is_empty() {
            0.0
        } else {
            self.fps_history.iter().sum::<f32>() / self.fps_history.len() as f32
        }
    }
    
    /// 获取操作的平均耗时
    pub fn get_average_operation_time(&self, operation_name: &str) -> Option<f64> {
        self.operation_times.get(operation_name)
            .filter(|times| !times.is_empty())
            .map(|times| times.iter().sum::<f64>() / times.len() as f64)
    }
}

/// 性能监控UI
pub fn performance_monitor_ui(ui: &mut Ui, monitor: &PerformanceMonitor) {
    ui.group(|ui| {
        ui.heading("性能监控");
        
        // 帧率信息
        CollapsingHeader::new("帧率监控")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(format!("平均帧率: {:.1} FPS", monitor.get_average_fps()));
                
                // 简单的帧率图表
                ui.label("帧率历史:");
                let fps_values: Vec<f32> = monitor.fps_history.iter().copied().collect();
                if !fps_values.is_empty() {
                    egui::plot::Plot::new(Id::new("fps_plot"))
                        .view_aspect(2.0)
                        .height(100.0)
                        .show(ui, |plot_ui| {
                            let line = egui::plot::Line::new(
                                fps_values.into_iter().enumerate()
                                    .map(|(i, y)| [i as f64, y as f64])
                                    .collect()
                            );
                            plot_ui.line(line);
                        });
                }
            });
        
        // 操作耗时信息
        CollapsingHeader::new("操作耗时")
            .default_open(false)
            .show(ui, |ui| {
                for (operation_name, times) in &monitor.operation_times {
                    if let Some(avg_time) = monitor.get_average_operation_time(operation_name) {
                        ui.label(format!("{}: 平均 {:.2} ms", operation_name, avg_time));
                    }
                }
            });
    });
}
"},"query_language":"Chinese"}}}