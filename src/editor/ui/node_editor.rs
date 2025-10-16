/// 节点编辑器模块
use eframe::egui;
use crate::application::AppState;
use crate::editor::graph::PowerDistributionGraphEditorState;

/// 节点编辑器响应信息
pub struct NodeEditorResponse {
    /// 选中的节点ID
    pub selected_node_id: Option<String>,
    /// 鼠标位置
    pub mouse_pos: Option<(f32, f32)>,
}

impl Default for NodeEditorResponse {
    fn default() -> Self {
        Self {
            selected_node_id: None,
            mouse_pos: None,
        }
    }
}

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
    
    /// 设置缩放级别
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(0.1, 5.0);
    }
    
    /// 获取缩放级别
    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }
    
    /// 设置是否显示网格
    pub fn set_show_grid(&mut self, show: bool) {
        self.show_grid = show;
    }
    
    /// 获取是否显示网格
    pub fn is_grid_enabled(&self) -> bool {
        self.show_grid
    }
    
    /// 渲染节点编辑器（旧版接口，保持兼容性）
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
    
    /// 使用图状态渲染节点编辑器（新版接口）
    pub fn render_with_graph(
        &mut self, 
        ui: &mut egui::Ui, 
        state: &AppState, 
        selected_node_type: Option<String>,
        graph_state: &mut PowerDistributionGraphEditorState
    ) -> NodeEditorResponse {
        let mut response = NodeEditorResponse::default();
        
        // 创建一个可滚动的区域
        egui::ScrollArea::both()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                // 获取可用空间
                let available_size = ui.available_size();
                
                // 创建绘图区域
                let (mut response_area, painter) = ui.allocate_painter(
                    available_size,
                    egui::Sense::drag(),
                );
                
                // 处理拖动事件
                if response_area.dragged() {
                    let delta = response_area.drag_delta();
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
                    let mouse_pos = response_area.interact_pointer_pos()
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
                
                // 更新鼠标位置信息
                response.mouse_pos = response_area.interact_pointer_pos().map(|pos| (pos.x, pos.y));
                
                // 绘制网格（如果启用）
                if self.show_grid {
                    self.draw_grid(&painter, available_size);
                }
                
                // 使用egui_node_graph渲染节点图
                egui_node_graph::draw_graph_editor(
                    ui, 
                    &mut graph_state.graph, 
                    &mut graph_state.editor_state,
                    self.zoom,
                    &self.pan_offset,
                );
                
                // 检查节点选择变化
                if let Some(selection) = &graph_state.editor_state.selected_nodes {
                    if !selection.is_empty() {
                        // 这里简化处理，只取第一个选中的节点
                        let node_id = selection.iter().next().unwrap();
                        response.selected_node_id = Some(node_id.0.to_string());
                    }
                }
                
                // 如果有选中的节点类型，处理节点添加
                if let Some(node_type) = &selected_node_type {
                    if let Some(node_template) = graph_state.node_templates.get(node_type) {
                        // 处理鼠标点击添加节点
                        if response_area.clicked() {
                            if let Some(mouse_pos) = response_area.interact_pointer_pos() {
                                // 计算节点在世界坐标系中的位置
                                let world_pos = (
                                    mouse_pos.x - self.pan_offset.0,
                                    mouse_pos.y - self.pan_offset.1,
                                );
                                
                                // 创建新节点
                                let mut node = node_template.create_node();
                                graph_state.graph.set_node_position(
                                    graph_state.add_node(node),
                                    world_pos.0, world_pos.1
                                );
                            }
                        }
                    }
                }
            });
        
        response
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
        // 目前暂时留空，因为新的render_with_graph方法使用egui_node_graph来处理
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