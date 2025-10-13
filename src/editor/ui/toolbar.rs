/// 工具栏模块
use eframe::egui;
use crate::application::AppState;

/// 工具栏
pub struct Toolbar {
    /// 当前选中的节点类型
    pub selected_node_type: String,
    /// 是否启用网格
    pub grid_enabled: bool,
    /// 缩放级别
    pub zoom_level: f32,
}

impl Toolbar {
    /// 创建新的工具栏
    pub fn new() -> Self {
        Self {
            selected_node_type: "distribution_box".to_string(),
            grid_enabled: true,
            zoom_level: 1.0,
        }
    }
    
    /// 渲染工具栏
    pub fn render(&mut self, ui: &mut egui::Ui, state: &mut AppState) {
        // 创建水平工具栏
        ui.horizontal_wrapped(|ui| {
            // 节点创建部分
            ui.label("添加节点:");
            
            // 节点类型按钮
            if self.node_type_button(ui, "配电箱", "distribution_box") {
                self.selected_node_type = "distribution_box".to_string();
            }
            
            if self.node_type_button(ui, "负载", "load") {
                self.selected_node_type = "load".to_string();
            }
            
            if self.node_type_button(ui, "电缆", "cable") {
                self.selected_node_type = "cable".to_string();
            }
            
            if self.node_type_button(ui, "电源", "power_source") {
                self.selected_node_type = "power_source".to_string();
            }
            
            ui.separator();
            
            // 视图控制部分
            ui.label("视图:");
            
            // 网格切换按钮
            if ui.button(if self.grid_enabled { "隐藏网格" } else { "显示网格" }).clicked() {
                self.grid_enabled = !self.grid_enabled;
            }
            
            // 缩放控制
            ui.horizontal(|ui| {
                ui.label("缩放:");
                if ui.button("-").clicked() && self.zoom_level > 0.1 {
                    self.zoom_level *= 0.9;
                }
                ui.label(format!("{:.1}x", self.zoom_level));
                if ui.button("+").clicked() && self.zoom_level < 3.0 {
                    self.zoom_level *= 1.1;
                }
                if ui.button("重置").clicked() {
                    self.zoom_level = 1.0;
                }
            });
            
            ui.separator();
            
            // 计算部分
            if ui.button("运行计算").clicked() {
                // 触发电气计算
                if let Err(e) = state.perform_calculations() {
                    println!("计算错误: {}", e);
                }
            }
            
            // 导出部分
            if ui.button("导出")
                .on_hover_text("导出项目或图表")
                .clicked() {
                self.show_export_menu(ui, state);
            }
        });
    }
    
    /// 渲染节点类型按钮
    fn node_type_button(&self, ui: &mut egui::Ui, label: &str, node_type: &str) -> bool {
        ui.button(label)
            .fill(if self.selected_node_type == node_type {
                Some(ui.visuals().widgets.active.bg_fill)
            } else {
                None
            })
            .clicked()
    }
    
    /// 显示导出菜单
    fn show_export_menu(&self, ui: &mut egui::Ui, _state: &mut AppState) {
        egui::menu::popup_menu(ui, "export_menu", |ui| {
            if ui.button("导出为图片").clicked() {
                // 实现导出为图片功能
            }
            
            if ui.button("导出为PDF").clicked() {
                // 实现导出为PDF功能
            }
            
            if ui.button("导出为JSON").clicked() {
                // 实现导出为JSON功能
            }
            
            ui.separator();
            
            if ui.button("导出计算报告").clicked() {
                // 实现导出计算报告功能
            }
        });
    }
    
    /// 获取当前选中的节点类型
    pub fn get_selected_node_type(&self) -> &str {
        &self.selected_node_type
    }
    
    /// 获取网格启用状态
    pub fn is_grid_enabled(&self) -> bool {
        self.grid_enabled
    }
    
    /// 获取缩放级别
    pub fn get_zoom_level(&self) -> f32 {
        self.zoom_level
    }
    
    /// 设置缩放级别
    pub fn set_zoom_level(&mut self, zoom: f32) {
        self.zoom_level = zoom.max(0.1).min(3.0); // 限制缩放范围
    }
}