/// 编辑器模块
use eframe::egui;
use crate::application::AppState;
use crate::editor::ui::{NodeEditor, PropertyPanel, Toolbar, StatusBar};
use crate::editor::graph::PowerDistributionGraphEditorState;
use std::sync::{Arc, Mutex};

/// 主编辑器结构体
pub struct Editor {
    /// 节点编辑器
    node_editor: NodeEditor,
    /// 属性面板
    property_panel: PropertyPanel,
    /// 工具栏
    toolbar: Toolbar,
    /// 状态栏
    status_bar: StatusBar,
    /// 当前选中的节点ID
    selected_node_id: Option<String>,
    /// 是否显示属性面板
    show_property_panel: bool,
    /// 本地节点图编辑器状态
    local_graph_state: Option<PowerDistributionGraphEditorState>,
}

impl Editor {
    /// 创建新的编辑器
    pub fn new() -> Self {
        Self {
            node_editor: NodeEditor::new(),
            property_panel: PropertyPanel::new(),
            toolbar: Toolbar::new(),
            status_bar: StatusBar::new(),
            selected_node_id: None,
            show_property_panel: true,
            local_graph_state: Some(PowerDistributionGraphEditorState::new()),
        }
    }
    
    /// 渲染编辑器界面
    pub fn render(&mut self, ctx: &egui::Context, state: &mut AppState) {
        // 确保本地状态存在
        if self.local_graph_state.is_none() {
            self.local_graph_state = Some(PowerDistributionGraphEditorState::new());
        }
        
        // 从应用状态获取图编辑器状态
        let mut app_graph_state = state.graph_editor_state.lock().unwrap();
        
        // 渲染菜单栏
        self.render_menu_bar(ctx, state);
        
        // 渲染工具栏
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.add_space(5.0);
            self.toolbar.render(ui, state);
            ui.add_space(5.0);
        });
        
        // 渲染主编辑区域和属性面板
        egui::CentralPanel::default().show(ctx, |ui| {
            // 根据是否显示属性面板来调整布局
            if self.show_property_panel {
                // 分割布局：左侧节点编辑器，右侧属性面板
                egui::SidePanel::right("property_panel")
                    .resizable(true)
                    .min_width(250.0)
                    .show(ctx, |ui| {
                        self.property_panel.render(ui, state, &self.selected_node_id);
                    });
            }
            
            // 如果本地状态存在，使用它进行节点编辑器渲染
            if let Some(local_graph_state) = &mut self.local_graph_state {
                // 设置节点编辑器参数
                self.node_editor.set_zoom(self.toolbar.get_zoom_level());
                self.node_editor.set_show_grid(self.toolbar.is_grid_enabled());
                
                // 渲染节点编辑器并处理响应
                let response = self.node_editor.render_with_graph(
                    ui, 
                    state, 
                    self.toolbar.get_selected_node_type(),
                    local_graph_state
                );
                
                // 处理节点选择事件
                if let Some(node_id) = response.selected_node_id {
                    self.selected_node_id = node_id;
                    self.status_bar.update_selected_nodes_count(1);
                }
                
                // 更新状态栏信息
                self.status_bar.update_mouse_pos(response.mouse_pos);
                self.status_bar.update_zoom_level(self.toolbar.get_zoom_level());
            }
        });
        
        // 渲染状态栏
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.add_space(2.0);
            self.status_bar.render(ui, state);
            ui.add_space(2.0);
        });
    }
    
    /// 渲染菜单栏
    fn render_menu_bar(&mut self, ctx: &egui::Context, state: &mut AppState) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // 文件菜单
                self.render_file_menu(ui, state);
                
                // 编辑菜单
                self.render_edit_menu(ui, state);
                
                // 视图菜单
                self.render_view_menu(ui, state);
                
                // 工具菜单
                self.render_tools_menu(ui, state);
                
                // 帮助菜单
                self.render_help_menu(ui, state);
            });
        });
    }
    
    /// 渲染文件菜单
    fn render_file_menu(&self, ui: &mut egui::Ui, _state: &mut AppState) {
        egui::menu::menu_button(ui, "文件", |ui| {
            if ui.button("新建项目").clicked() {
                // 实现新建项目功能
                ui.close();
            }
            
            if ui.button("打开项目").clicked() {
                // 实现打开项目功能
                ui.close();
            }
            
            if ui.button("保存项目").clicked() {
                // 实现保存项目功能
                ui.close();
            }
            
            if ui.button("另存为...").clicked() {
                // 实现另存为功能
                ui.close();
            }
            
            ui.separator();
            
            if ui.button("导入...").clicked() {
                // 实现导入功能
                ui.close();
            }
            
            if ui.button("导出...").clicked() {
                // 实现导出功能
                ui.close();
            }
            
            ui.separator();
            
            if ui.button("退出").clicked() {
                // 实现退出功能
                ui.close();
            }
        });
    }
    
    /// 渲染编辑菜单
    fn render_edit_menu(&mut self, ui: &mut egui::Ui, _state: &mut AppState) {
        egui::menu::menu_button(ui, "编辑", |ui| {
            if ui.button("撤销").clicked() {
                // 实现撤销功能
                ui.close();
            }
            
            if ui.button("重做").clicked() {
                // 实现重做功能
                ui.close();
            }
            
            ui.separator();
            
            if ui.button("复制").clicked() {
                // 实现复制功能
                ui.close();
            }
            
            if ui.button("剪切").clicked() {
                // 实现剪切功能
                ui.close();
            }
            
            if ui.button("粘贴").clicked() {
                // 实现粘贴功能
                ui.close();
            }
            
            ui.separator();
            
            if ui.button("删除选中项").clicked() {
                // 实现删除功能
                ui.close();
            }
        });
    }
    
    /// 渲染视图菜单
    fn render_view_menu(&mut self, ui: &mut egui::Ui, _state: &mut AppState) {
        egui::menu::menu_button(ui, "视图", |ui| {
            // 属性面板显示切换
            ui.checkbox(&mut self.show_property_panel.clone(), "显示属性面板");
            
            // 网格显示切换
            ui.checkbox(&mut self.toolbar.grid_enabled.clone(), "显示网格");
            
            ui.separator();
            
            // 缩放控制
            if ui.button("放大").clicked() {
                self.toolbar.set_zoom_level(self.toolbar.get_zoom_level() * 1.2);
                ui.close();
            }
            
            if ui.button("缩小").clicked() {
                self.toolbar.set_zoom_level(self.toolbar.get_zoom_level() * 0.8);
                ui.close();
            }
            
            if ui.button("重置缩放").clicked() {
                self.toolbar.set_zoom_level(1.0);
                ui.close();
            }
            
            if ui.button("适应屏幕").clicked() {
                // 实现适应屏幕功能
                ui.close();
            }
        });
    }
    
    /// 渲染工具菜单
    fn render_tools_menu(&self, ui: &mut egui::Ui, _state: &mut AppState) {
        egui::menu::menu_button(ui, "工具", |ui| {
            if ui.button("电气计算").clicked() {
                // 实现电气计算功能
                ui.close();
            }
            
            if ui.button("电缆选型").clicked() {
                // 实现电缆选型功能
                ui.close();
            }
            
            if ui.button("短路计算").clicked() {
                // 实现短路计算功能
                ui.close();
            }
            
            ui.separator();
            
            if ui.button("项目设置").clicked() {
                // 实现项目设置功能
                ui.close();
            }
            
            if ui.button("软件设置").clicked() {
                // 实现软件设置功能
                ui.close();
            }
        });
    }
    
    /// 渲染帮助菜单
    fn render_help_menu(&self, ui: &mut egui::Ui, _state: &mut AppState) {
        egui::menu::menu_button(ui, "帮助", |ui| {
            if ui.button("用户手册").clicked() {
                // 实现用户手册功能
                ui.close();
            }
            
            if ui.button("在线帮助").clicked() {
                // 实现在线帮助功能
                ui.close();
            }
            
            ui.separator();
            
            if ui.button("关于").clicked() {
                // 实现关于对话框
                ui.close();
            }
        });
    }
    
    /// 获取当前选中的节点ID
    pub fn get_selected_node_id(&self) -> &Option<String> {
        &self.selected_node_id
    }
    
    /// 设置选中的节点ID
    pub fn set_selected_node_id(&mut self, node_id: Option<String>) {
        self.selected_node_id = node_id;
        self.status_bar.update_selected_nodes_count(node_id.is_some() as usize);
    }
}