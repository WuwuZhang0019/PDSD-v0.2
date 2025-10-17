use std::fmt::Debug;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::time::Duration;
use egui_node_graph::{NodeId, InputId, OutputId};
use crate::editor::UIValueType;

/// 日志级别枚举
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

/// 日志条目结构
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Local>,
}

/// 调试日志管理器
pub struct DebugLogger {
    log_sender: Sender<LogEntry>,
    log_receiver: Receiver<LogEntry>,
    pub logs: Vec<LogEntry>,
    pub max_logs: usize,
    pub current_level: LogLevel,
    is_recording: bool,
}

impl Default for DebugLogger {
    fn default() -> Self {
        let (sender, receiver) = channel();
        
        Self {
            log_sender: sender,
            log_receiver: receiver,
            logs: Vec::new(),
            max_logs: 1000,
            current_level: LogLevel::Info,
            is_recording: true,
        }
    }
}

impl DebugLogger {
    /// 创建新的调试日志管理器
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 记录节点执行日志
    pub fn log_node_execution(
        &self, 
        node_id: NodeId, 
        inputs: &[UIValueType], 
        result: Option<UIValueType>
    ) {
        if !self.is_recording || self.current_level > LogLevel::Debug {
            return;
        }
        
        let message = format!(
            "执行节点: {:?}\n输入: {:?}\n结果: {:?}", 
            node_id, inputs, result
        );
        
        self.log(LogLevel::Debug, &message);
    }
    
    /// 记录错误日志
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
    
    /// 记录警告日志
    pub fn warning(&self, message: &str) {
        self.log(LogLevel::Warning, message);
    }
    
    /// 记录信息日志
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }
    
    /// 记录调试日志
    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }
    
    /// 记录跟踪日志
    pub fn trace(&self, message: &str) {
        self.log(LogLevel::Trace, message);
    }
    
    /// 内部日志记录方法
    fn log(&self, level: LogLevel, message: &str) {
        if !self.is_recording || level < self.current_level {
            return;
        }
        
        let entry = LogEntry {
            level,
            message: message.to_string(),
            timestamp: chrono::Local::now(),
        };
        
        // 忽略发送失败的情况
        let _ = self.log_sender.send(entry);
    }
    
    /// 更新日志缓冲区，获取新的日志条目
    pub fn update(&mut self) {
        while let Ok(entry) = self.log_receiver.try_recv() {
            self.logs.push(entry);
            
            // 限制日志数量
            if self.logs.len() > self.max_logs {
                self.logs.remove(0);
            }
        }
    }
    
    /// 清除所有日志
    pub fn clear(&mut self) {
        self.logs.clear();
    }
    
    /// 设置日志级别
    pub fn set_level(&mut self, level: LogLevel) {
        self.current_level = level;
    }
    
    /// 开始/停止记录日志
    pub fn set_recording(&mut self, recording: bool) {
        self.is_recording = recording;
    }
}

/// 将节点执行结果格式化为字符串
pub fn format_node_execution_result<T: Debug>(result: Option<T>) -> String {
    match result {
        Some(value) => format!("{:?}", value),
        None => "无结果".to_string(),
    }
}