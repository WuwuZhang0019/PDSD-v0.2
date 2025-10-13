/// 状态栏模块
use eframe::egui;
use crate::application::AppState;

/// 状态栏
pub struct StatusBar {
    /// 当前鼠标位置
    pub mouse_pos: Option<(f32, f32)>,
    /// 当前缩放级别
    pub zoom_level: f32,
    /// 选中的节点数量
    pub selected_nodes_count: usize,
    /// 当前项目状态
    pub project_status: String,
}

impl StatusBar {
    /// 创建新的状态栏
    pub fn new() -> Self {
        Self {
            mouse_pos: None,
            zoom_level: 1.0,
            selected_nodes_count: 0,
            project_status: "就绪".to_string(),
        }
    }
    
    /// 渲染状态栏
    pub fn render(&mut self, ui: &mut egui::Ui, state: &AppState) {
        // 使用水平布局渲染状态栏内容
        ui.horizontal(|ui| {
            // 项目状态
            ui.label(format!("状态: {}", self.project_status));
            
            ui.separator();
            
            // 鼠标位置信息
            if let Some((x, y)) = self.mouse_pos {
                ui.label(format!("位置: ({:.1}, {:.1})", x, y));
            } else {
                ui.label("位置: 未定义");
            }
            
            ui.separator();
            
            // 缩放级别
            ui.label(format!("缩放: {:.1}x", self.zoom_level));
            
            ui.separator();
            
            // 节点统计信息
            let nodes_count = if let Ok(nodes) = state.nodes.lock() {
                nodes.len()
            } else {
                0
            };
            ui.label(format!("节点: {}", nodes_count));
            
            ui.separator();
            
            // 选中节点信息
            if self.selected_nodes_count > 0 {
                ui.label(format!("选中: {}", self.selected_nodes_count));
            } else {
                ui.label("未选中节点");
            }
            
            ui.separator();
            
            // 项目信息
            if let Ok(project_info) = state.project_info.lock() {
                ui.label(format!("项目: {}", project_info.name));
            }
            
            // 右侧显示版本信息
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("PDSD v0.2.0");
            });
        });
    }
    
    /// 更新鼠标位置
    pub fn update_mouse_pos(&mut self, pos: Option<(f32, f32)>) {
        self.mouse_pos = pos;
    }
    
    /// 更新缩放级别
    pub fn update_zoom_level(&mut self, zoom: f32) {
        self.zoom_level = zoom;
    }
    
    /// 更新选中节点数量
    pub fn update_selected_nodes_count(&mut self, count: usize) {
        self.selected_nodes_count = count;
    }
    
    /// 设置项目状态信息
    pub fn set_project_status(&mut self, status: &str) {
        self.project_status = status.to_string();
    }
    
    /// 显示临时消息
    pub fn show_temp_message(&mut self, message: &str, duration: std::time::Duration) {
        // 在实际应用中，这里应该启动一个定时器来恢复原始状态
        self.project_status = message.to_string();
        
        // 这里只是简单设置，实际应用中需要使用异步定时器
        // std::thread::spawn(move || {
        //     std::thread::sleep(duration);
        //     self.project_status = "就绪".to_string();
        // });
    }
}