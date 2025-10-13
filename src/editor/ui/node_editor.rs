/// 节点编辑器模块
use eframe::egui;
use crate::application::AppState;

/// 节点编辑器
pub struct NodeEditor {
    /// 是否显示网格
    pub show_grid: bool,
    /// 缩放级别
    pub zoom: f32,
    /// 平移偏移
    pub pan_offset: (f32, f32),
}

impl NodeEditor {
    /// 创建新的节点编辑器
    pub fn new() -> Self {
        Self {
            show_grid: true,
            zoom: 1.0,
            pan_offset: (0.0, 0.0),
        }
    }
    
    /// 渲染节点编辑器
    pub fn render(&mut self, ui: &mut egui::Ui, state: &AppState, selected_node_id: &Option<String>) {
        // 创建一个可滚动的区域
        egui::ScrollArea::both()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                // 获取可用空间
                let available_size = ui.available_size();
                
                // 创建绘图区域
                let (mut response, painter) = ui.allocate_painter(
                    available_size,
                    egui::Sense::drag(),
                );
                
                // 处理拖动事件
                if response.dragged() {
                    let delta = response.drag_delta();
                    self.pan_offset.0 += delta.x;
                    self.pan_offset.1 += delta.y;
                }
                
                // 处理滚轮缩放
                if let Some(scroll_delta) = ui.input(|i| i.scroll_delta.y) {
                    // 缩放因子
                    let zoom_factor = if scroll_delta > 0.0 {
                        1.1
                    } else {
                        0.9
                    };
                    
                    // 计算鼠标在编辑器中的位置
                    let mouse_pos = response.interact_pointer_pos()
                        .map(|pos| (pos.x, pos.y))
                        .unwrap_or((available_size.x / 2.0, available_size.y / 2.0));
                    
                    // 更新缩放
                    let old_zoom = self.zoom;
                    self.zoom = (self.zoom * zoom_factor).clamp(0.1, 5.0);
                    
                    // 调整平移偏移以实现以鼠标为中心的缩放
                    let zoom_ratio = self.zoom / old_zoom;
                    self.pan_offset.0 = mouse_pos.0 - (mouse_pos.0 - self.pan_offset.0) * zoom_ratio;
                    self.pan_offset.1 = mouse_pos.1 - (mouse_pos.1 - self.pan_offset.1) * zoom_ratio;
                }
                
                // 绘制网格（如果启用）
                if self.show_grid {
                    self.draw_grid(&painter, available_size);
                }
                
                // 绘制节点之间的连线
                self.draw_connections(&painter, state);
            });
    }
    
    /// 绘制网格
    fn draw_grid(&self, painter: &egui::Painter, size: egui::Vec2) {
        let grid_size = 20.0 * self.zoom;
        
        // 计算网格的起始位置（考虑平移偏移）
        let start_x = (self.pan_offset.0 % grid_size).floor();
        let start_y = (self.pan_offset.1 % grid_size).floor();
        
        // 绘制垂直线
        for x in (start_x as i32..size.x as i32).step_by(grid_size as usize) {
            painter.line_segment(
                [
                    egui::pos2(x as f32, 0.0),
                    egui::pos2(x as f32, size.y),
                ],
                egui::Stroke::new(0.5, egui::Color32::LIGHT_GRAY),
            );
        }
        
        // 绘制水平线
        for y in (start_y as i32..size.y as i32).step_by(grid_size as usize) {
            painter.line_segment(
                [
                    egui::pos2(0.0, y as f32),
                    egui::pos2(size.x, y as f32),
                ],
                egui::Stroke::new(0.5, egui::Color32::LIGHT_GRAY),
            );
        }
    }
    
    /// 绘制节点之间的连线
    fn draw_connections(&self, painter: &egui::Painter, _state: &AppState) {
        // 实际应用中，这里应该根据节点之间的连接关系绘制连线
        // 目前暂时留空
    }
    
    /// 重置视图
    pub fn reset_view(&mut self) {
        self.zoom = 1.0;
        self.pan_offset = (0.0, 0.0);
    }
    
    /// 切换网格显示
    pub fn toggle_grid(&mut self) {
        self.show_grid = !self.show_grid;
    }
}