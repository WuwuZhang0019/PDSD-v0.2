/// 属性面板模块
use eframe::egui;
use crate::application::AppState;

/// 属性面板
pub struct PropertyPanel {
    /// 是否显示高级属性
    pub show_advanced_properties: bool,
}

impl PropertyPanel {
    /// 创建新的属性面板
    pub fn new() -> Self {
        Self {
            show_advanced_properties: false,
        }
    }
    
    /// 渲染属性面板
    pub fn render(&mut self, ui: &mut egui::Ui, state: &AppState, selected_node_id: &Option<String>) {
        ui.heading("属性面板");
        ui.separator();
        
        if let Some(node_id) = selected_node_id {
            self.render_node_properties(ui, state, node_id);
        } else {
            ui.label("请选择一个节点以查看其属性");
            
            // 如果没有选中节点，可以显示项目属性
            if ui.collapsing("项目属性", |ui| {
                self.render_project_properties(ui, state);
            }).header_response.clicked() {
                // 处理点击事件
            }
        }
    }
    
    /// 渲染节点属性
    fn render_node_properties(&mut self, ui: &mut egui::Ui, state: &AppState, node_id: &str) {
        if let Ok(nodes) = state.nodes.lock() {
            if let Some(node) = nodes.iter().find(|n| n.id == node_id) {
                ui.collapsing("基本信息", |ui| {
                    self.render_basic_properties(ui, node);
                });
                
                // 根据节点类型显示特定属性
                self.render_type_specific_properties(ui, node);
                
                // 高级属性
                ui.collapsing("高级属性", |ui| {
                    self.show_advanced_properties = true;
                    self.render_advanced_properties(ui, node);
                });
                
                // 操作按钮
                ui.separator();
                if ui.button("复制节点").clicked() {
                    // 处理复制节点
                }
                if ui.button("删除节点").clicked() {
                    // 处理删除节点
                }
            } else {
                ui.label("未找到节点信息");
            }
        } else {
            ui.label("无法访问节点数据");
        }
    }
    
    /// 渲染基本属性
    fn render_basic_properties(&self, ui: &mut egui::Ui, node: &crate::application::state::GraphNode) {
        ui.label(format!("节点ID: {}", node.id));
        
        // 节点名称（可编辑）
        let mut name = node.name.clone();
        if ui.text_edit_singleline(&mut name).changed() {
            // 在实际应用中，这里应该更新节点名称
            // 由于我们在只读上下文中，所以这里只是演示
        }
        
        // 节点类型（只读）
        ui.label(format!("节点类型: {}", self.get_node_type_display_name(&node.node_type)));
        
        // 位置信息
        let mut x = node.x.to_string();
        let mut y = node.y.to_string();
        ui.horizontal(|ui| {
            ui.label("X坐标:");
            if ui.text_edit_singleline(&mut x).changed() {
                // 处理X坐标变更
            }
        });
        ui.horizontal(|ui| {
            ui.label("Y坐标:");
            if ui.text_edit_singleline(&mut y).changed() {
                // 处理Y坐标变更
            }
        });
    }
    
    /// 渲染类型特定属性
    fn render_type_specific_properties(&self, ui: &mut egui::Ui, node: &crate::application::state::GraphNode) {
        match node.node_type.as_str() {
            "distribution_box" => {
                self.render_distribution_box_properties(ui, node);
            }
            "load" => {
                self.render_load_properties(ui, node);
            }
            "cable" => {
                self.render_cable_properties(ui, node);
            }
            "power_source" => {
                self.render_power_source_properties(ui, node);
            }
            _ => {
                ui.label("未知节点类型");
            }
        }
    }
    
    /// 渲染配电箱属性
    fn render_distribution_box_properties(&self, ui: &mut egui::Ui, _node: &crate::application::state::GraphNode) {
        ui.label("配电箱属性");
        
        // 额定电流
        let mut rated_current = String::new();
        ui.horizontal(|ui| {
            ui.label("额定电流 (A):");
            ui.text_edit_singleline(&mut rated_current);
        });
        
        // 型号
        let mut model = String::new();
        ui.horizontal(|ui| {
            ui.label("型号:");
            ui.text_edit_singleline(&mut model);
        });
        
        // IP等级
        let mut ip_rating = String::new();
        ui.horizontal(|ui| {
            ui.label("IP等级:");
            ui.text_edit_singleline(&mut ip_rating);
        });
    }
    
    /// 渲染负载属性
    fn render_load_properties(&self, ui: &mut egui::Ui, _node: &crate::application::state::GraphNode) {
        ui.label("负载属性");
        
        // 有功功率
        let mut active_power = String::new();
        ui.horizontal(|ui| {
            ui.label("有功功率 (kW):");
            ui.text_edit_singleline(&mut active_power);
        });
        
        // 功率因数
        let mut power_factor = String::new();
        ui.horizontal(|ui| {
            ui.label("功率因数:");
            ui.text_edit_singleline(&mut power_factor);
        });
        
        // 负载类型
        egui::ComboBox::from_label("负载类型")
            .selected_text("三相平衡")
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut "", "三相平衡", "三相平衡");
                ui.selectable_value(&mut "", "单相", "单相");
                ui.selectable_value(&mut "", "两相", "两相");
            });
    }
    
    /// 渲染电缆属性
    fn render_cable_properties(&self, ui: &mut egui::Ui, _node: &crate::application::state::GraphNode) {
        ui.label("电缆属性");
        
        // 截面积
        let mut cross_section = String::new();
        ui.horizontal(|ui| {
            ui.label("截面积 (mm²):");
            ui.text_edit_singleline(&mut cross_section);
        });
        
        // 长度
        let mut length = String::new();
        ui.horizontal(|ui| {
            ui.label("长度 (m):");
            ui.text_edit_singleline(&mut length);
        });
        
        // 电缆型号
        let mut cable_model = String::new();
        ui.horizontal(|ui| {
            ui.label("电缆型号:");
            ui.text_edit_singleline(&mut cable_model);
        });
    }
    
    /// 渲染电源属性
    fn render_power_source_properties(&self, ui: &mut egui::Ui, _node: &crate::application::state::GraphNode) {
        ui.label("电源属性");
        
        // 电压
        let mut voltage = String::new();
        ui.horizontal(|ui| {
            ui.label("电压 (V):");
            ui.text_edit_singleline(&mut voltage);
        });
        
        // 频率
        let mut frequency = String::new();
        ui.horizontal(|ui| {
            ui.label("频率 (Hz):");
            ui.text_edit_singleline(&mut frequency);
        });
        
        // 相数
        egui::ComboBox::from_label("相数")
            .selected_text("三相")
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut "", "三相", "三相");
                ui.selectable_value(&mut "", "单相", "单相");
            });
    }
    
    /// 渲染高级属性
    fn render_advanced_properties(&self, ui: &mut egui::Ui, node: &crate::application::state::GraphNode) {
        ui.label("自定义属性");
        
        // 显示所有自定义属性
        for (key, value) in &node.properties {
            ui.horizontal(|ui| {
                ui.label(format!("{}:", key));
                let mut editable_value = value.clone();
                if ui.text_edit_singleline(&mut editable_value).changed() {
                    // 处理属性值变更
                }
            });
        }
        
        // 添加新属性的功能
        ui.horizontal(|ui| {
            let mut new_key = String::new();
            let mut new_value = String::new();
            
            ui.text_edit_singleline(&mut new_key);
            ui.text_edit_singleline(&mut new_value);
            if ui.button("添加").clicked() && !new_key.is_empty() {
                // 处理添加新属性
            }
        });
    }
    
    /// 渲染项目属性
    fn render_project_properties(&self, ui: &mut egui::Ui, state: &AppState) {
        if let Ok(mut project_info) = state.project_info.lock() {
            ui.text_edit_singleline(&mut project_info.name);
            ui.text_edit_multiline(&mut project_info.description);
            ui.text_edit_singleline(&mut project_info.project_id);
        }
    }
    
    /// 获取节点类型的显示名称
    fn get_node_type_display_name(&self, node_type: &str) -> &str {
        match node_type {
            "distribution_box" => "配电箱",
            "load" => "负载",
            "cable" => "电缆",
            "power_source" => "电源",
            _ => "未知",
        }
    }
}