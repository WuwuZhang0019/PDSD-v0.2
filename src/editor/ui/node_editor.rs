/// 节点编辑器UI组件
/// 实现配电系统图的节点编辑器界面

use eframe::egui::{self, Ui, ScrollArea, Window}; 
use egui_node_graph::{draw_graph_editor, NodeResponse}; 
use crate::editor::{DataType, UIValueType, UIUserState, UIResponse};
use crate::editor::business::{PowerGraphNode}; 
use crate::editor::graph::PowerDistributionGraphEditorState;
use crate::editor::business::{ElectricNodeTemplate, get_all_node_templates};

/// 节点编辑器UI组件
pub struct NodeEditor {
    /// 节点图编辑器状态
    pub editor_state: PowerDistributionGraphEditorState,
    /// 节点搜索框的文本
    pub node_search_text: String,
    /// 是否显示节点查找器
    pub show_node_finder: bool,
}

impl Default for NodeEditor {
    fn default() -> Self {
        Self {
            editor_state: PowerDistributionGraphEditorState::default(),
            node_search_text: String::new(),
            show_node_finder: false,
        }
    }
}

impl NodeEditor {
    /// 创建新的节点编辑器UI组件
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 显示节点编辑器UI
    pub fn show(&mut self, ctx: &egui::Context) {
        // 创建节点编辑器窗口
        Window::new("配电系统图编辑器")
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                self.show_ui(ui);
            });
    }
    
    /// 显示节点编辑器的内容UI
    fn show_ui(&mut self, ui: &mut Ui) {
        // 顶部工具栏
        ui.horizontal(|ui| {
            // 添加节点按钮
            if ui.button("添加节点").clicked() {
                self.show_node_finder = true;
            }
            
            // 清除按钮
            if ui.button("清除").clicked() {
                self.editor_state.graph.clear();
            }
        });
        
        ui.separator();
        
        // 节点图编辑区域
        ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // 绘制节点编辑器
                let responses = draw_graph_editor(
                    ui,
                    &mut self.editor_state.graph,
                    &mut self.editor_state.editor_state,
                    &mut self.editor_state.user_state,
                    None, // 可选的背景
                );
                
                // 处理节点响应
                self.handle_node_responses(responses);
            });
            
        // 如果需要显示节点查找器
        if self.show_node_finder {
            self.show_node_finder(ui);
        }
        
        // 显示选中节点的详细信息（如果有）
        if let Some(node_id) = self.editor_state.selected_node_id {
            self.show_node_details(ui, node_id);
        }
    }
    
    /// 处理节点响应
    fn handle_node_responses(&mut self, responses: Vec<NodeResponse<UIResponse, PowerGraphNode>>) {
        self.editor_state.handle_node_responses(responses);
    }
    
    /// 显示节点查找器
    fn show_node_finder(&mut self, ui: &mut Ui) {
        // 创建节点查找器弹窗
        let mut close_finder = false;
        
        Window::new("选择节点类型")
            .default_size([300.0, 400.0])
            .show(ui.ctx(), |ui| {
                // 搜索框
                ui.text_edit_singleline(&mut self.node_search_text);
                ui.separator();
                
                // 显示所有可用的节点模板
                ScrollArea::vertical().show(ui, |ui| {
                    let node_templates = get_all_node_templates();
                    
                    for template in node_templates {
                        let label = template.node_finder_label(&mut self.editor_state.user_state);
                        
                        // 如果有搜索文本，过滤结果
                        if !self.node_search_text.is_empty() {
                            if !label.contains(&self.node_search_text) {
                                continue;
                            }
                        }
                        
                        // 显示节点模板按钮
                        if ui.button(label).clicked() {
                            // 使用模板创建新节点
                            let node_id = self.editor_state.graph.add_node_from_template(
                                template,
                                &mut self.editor_state.user_state,
                                ui.next_widget_position() - egui::vec2(50.0, 50.0), // 居中放置节点
                            );
                            
                            // 将节点添加到编辑器状态中
                            self.editor_state.selected_node_id = Some(node_id);
                            close_finder = true;
                        }
                    }
                });
                
                // 关闭按钮
                if ui.button("取消").clicked() {
                    close_finder = true;
                }
            });
            
        if close_finder {
            self.show_node_finder = false;
            self.node_search_text.clear();
        }
    }
    
    /// 显示选中节点的详细信息
    fn show_node_details(&self, ui: &mut Ui, node_id: crate::editor::graph::egui_node_graph::NodeId) {
        // 获取节点信息
        if let Some(node) = self.editor_state.graph.nodes.get(node_id) {
            Window::new("节点详情")
                .default_size([300.0, 400.0])
                .show(ui.ctx(), |ui| {
                    ui.label(format!("节点ID: {:?}", node_id));
                    ui.label(format!("节点类型: {:?}", node.node_type));
                    ui.label(format!("节点名称: {}", node.name));
                    ui.label(format!("节点描述: {}", node.description));
                    
                    ui.separator();
                    
                    // 显示节点输入参数
                    ui.label("输入参数:");
                    for (input_id, input) in &self.editor_state.graph.inputs {
                        if input.node == node_id {
                            ui.label(format!("- {} ({:?})", 
                                input.name, 
                                input.data_type
                            ));
                            
                            // 如果有连接，显示连接的输出
                            if let Some(connected_output_id) = self.editor_state.graph.connections.get(input_id) {
                                ui.label(format!("  已连接到输出ID: {:?}", connected_output_id));
                            }
                        }
                    }
                    
                    ui.separator();
                    
                    // 显示节点输出参数
                    ui.label("输出参数:");
                    for (output_id, output) in &self.editor_state.graph.outputs {
                        if output.node == node_id {
                            ui.label(format!("- {} ({:?})", 
                                output.name, 
                                output.data_type
                            ));
                        }
                    }
                });
        }
    }
}