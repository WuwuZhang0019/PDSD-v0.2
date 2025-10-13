/// 应用主逻辑模块
use eframe::{egui, App, CreationContext};
use crate::application::{AppState};
use crate::editor::Editor;
use std::sync::Arc;

/// 电力配电系统设计应用程序主结构体
pub struct PDSDApp {
    /// 应用状态
    state: Arc<AppState>,
    /// 编辑器实例
    editor: Editor,
}

impl PDSDApp {
    /// 创建新的应用程序实例
    pub fn new(cc: &CreationContext<'_>, state: AppState) -> Result<Self, Box<dyn std::error::Error>> {
        // 将state包装在Arc中
        let state_arc = Arc::new(state);
        
        // 初始化编辑器
        let editor = Editor::new();
        
        Ok(Self {
            state: state_arc,
            editor,
        })
    }
}

impl App for PDSDApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // 设置窗口标题
        frame.set_window_title("电力配电系统设计软件 (PDSD)");
        
        // 克隆状态以便传递给编辑器
        let state_clone = Arc::clone(&self.state);
        
        // 渲染编辑器
        self.editor.render(ctx, frame, &state_clone);
    }
    
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // 保存应用状态到存储
        // 实际应用中应该实现序列化和保存逻辑
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
    
    fn load(&mut self, storage: &dyn eframe::Storage) {
        // 从存储加载应用状态
        // 实际应用中应该实现反序列化和加载逻辑
        if let Some(json) = storage.get_string("nodes") {
            if let Ok(nodes) = serde_json::from_str(&json) {
                if let Ok(mut nodes_lock) = self.state.nodes.lock() {
                    *nodes_lock = nodes;
                }
            }
        }
        
        // 加载项目信息
        if let Some(json) = storage.get_string("project_info") {
            if let Ok(project_info) = serde_json::from_str(&json) {
                if let Ok(mut project_lock) = self.state.project_info.lock() {
                    *project_lock = project_info;
                }
            }
        }
        
        // 加载应用配置
        if let Some(json) = storage.get_string("config") {
            if let Ok(config) = serde_json::from_str(&json) {
                if let Ok(mut config_lock) = self.state.config.lock() {
                    *config_lock = config;
                }
            }
        }
    }
}