/// 应用主逻辑模块
use eframe::{egui, App};
use crate::application::AppState;
use serde_json;
use std::sync::Arc;

/// 电力配电系统设计应用程序主结构体
pub struct PDSDApp {
    /// 应用状态
    state: Arc<AppState>,
}

impl PDSDApp {
    /// 创建新的应用程序实例
    pub fn new() -> Self {
        // 创建新的应用状态
        let state = AppState::new();
        
        // 将state包装在Arc中
        let state_arc = Arc::new(state);
        
        Self {
            state: state_arc,
        }
    }
}

impl App for PDSDApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // 设置窗口标题
        frame.set_window_title("电力配电系统设计软件 (PDSD)");
        
        // 简单的UI渲染
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("电力配电系统设计软件 (PDSD)");
            ui.label("欢迎使用电力配电系统设计软件");
            ui.label("应用程序正在开发中...");
        });
    }
    
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // 保存应用状态到存储
        if let Ok(nodes) = self.state.nodes.lock() {
            if let Ok(json) = serde_json::to_string(&*nodes) {
                storage.set_string("nodes", json);
            }
        }
        
        // 保存项目信息
        if let Ok(project_info) = self.state.project_info.lock() {
            if let Ok(json) = serde_json::to_string(&*project_info) {
                storage.set_string("project_info", json);
            }
        }
        
        // 保存应用配置
        if let Ok(config) = self.state.config.lock() {
            if let Ok(json) = serde_json::to_string(&*config) {
                storage.set_string("config", json);
            }
        }
    }
}