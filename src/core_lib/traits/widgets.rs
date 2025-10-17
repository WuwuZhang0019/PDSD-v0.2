/// UI控件相关的trait定义
/// 实现电气参数的UI控件显示和交互功能

use egui_node_graph::*;
use crate::core_lib::data_types::electric_data::ElectricValueType;
use egui::Ui;

/// 电气响应类型 - 简化版不使用复杂响应
pub type ElectricResponse = ();

/// 为ElectricValueType实现WidgetValueTrait接口
/// 用于在节点编辑器中显示和编辑电气参数
impl WidgetValueTrait for ElectricValueType {
    // 定义必要的关联类型
    type Response = ElectricResponse;
    type UserState = (); // 简化版不使用用户状态
    type NodeData = ();  // 简化版不使用节点数据
    
    /// 渲染参数控件的核心方法
    /// 根据电气参数类型显示不同的UI控件
    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut Ui,
        _user_state: &mut Self::UserState,
        _node_data: &Self::NodeData,
    ) -> Vec<Self::Response> {
        // 根据值类型显示不同的简单控件
        match self {
            // 处理浮点数类型（如电流、功率等）
            ElectricValueType::Float(val) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(egui::DragValue::new(val).clamp_range(0.0..=f64::MAX));
                });
            },
            
            // 处理整数类型
            ElectricValueType::Integer(val) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(egui::DragValue::new(val).clamp_range(0..=i64::MAX));
                });
            },
            
            // 处理字符串类型
            ElectricValueType::String(val) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.text_edit_singleline(val);
                });
            },
            
            // 处理布尔类型
            ElectricValueType::Boolean(val) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.checkbox(val, "");
                });
            },
            
            // 简化处理复杂类型 - 只显示基本信息
            ElectricValueType::CircuitData(circuit) => {
                ui.label(format!("{}: 回路-{} (功率: {:.2}kW)", 
                    param_name, 
                    circuit.name, 
                    circuit.rated_power
                ));
                
                // 显示回路的简要信息
                ui.collapsing("查看详情", |ui| {
                    ui.label(format!("回路编号: {}", circuit.number.get_full_number()));
                    ui.label(format!("计算电流: {:.2}A", circuit.calculated_current));
                    ui.label(format!("功率因数: {:.2}", circuit.power_factor));
                    ui.label(format!("需用系数: {:.2}", circuit.demand_coefficient));
                    
                    // 显示线缆信息
                    if let Some(cable) = &circuit.cable_info {
                        ui.label(format!("线缆型号: {}", cable.model));
                    }
                });
            },
            
            ElectricValueType::DistributionBoxData(box_data) => {
                ui.label(format!("{}: 配电箱-{} (总功率: {:.2}kW)", 
                    param_name, 
                    box_data.name, 
                    box_data.total_power
                ));
                
                // 显示配电箱简要信息
                ui.collapsing("查看详情", |ui| {
                    ui.label(format!("总电流: {:.2}A", box_data.total_current));
                    ui.label(format!("电压等级: {:.1}kV", box_data.voltage_level));
                    ui.label(format!("相数: {}", box_data.phase_count));
                    ui.label(format!("出线回路数: {}", box_data.outgoing_circuits.len()));
                });
            },
            
            ElectricValueType::PhaseBalanceInfo(balance_info) => {
                ui.label(format!("{}: 三相平衡信息", param_name));
                
                ui.collapsing("查看详情", |ui| {
                    ui.label(format!("A相负载: {:.2}kW", balance_info.phase_a_load));
                    ui.label(format!("B相负载: {:.2}kW", balance_info.phase_b_load));
                    ui.label(format!("C相负载: {:.2}kW", balance_info.phase_c_load));
                    ui.label(format!("不平衡度: {:.2}%", balance_info.unbalance_degree));
                });
            },
            
            // 其他类型简单显示
            _ => {
                ui.label(format!("{}: {}", param_name, self));
            }
        }
        
        // 返回空的响应列表（简化版不需要副作用处理）
        Vec::new()
    }
    
    /// 当输入参数已连接时的显示控件
    /// 简化版只显示连接提示
    fn value_widget_connected(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut Ui,
        _user_state: &mut Self::UserState,
        _node_data: &Self::NodeData,
    ) -> Vec<Self::Response> {
        ui.horizontal(|ui| {
            ui.label(param_name);
            ui.label("[已连接]")
                .on_hover_text("此参数通过连接线获取数据");
        });
        Vec::new()
    }
    
    /// 获取值的简短描述，用于在连接线上显示
    fn value_to_text(&self) -> String {
        match self {
            ElectricValueType::Float(val) => format!("{:.2}", val),
            ElectricValueType::Integer(val) => format!("{}", val),
            ElectricValueType::String(val) => val.clone(),
            ElectricValueType::Boolean(val) => format!("{}", val),
            ElectricValueType::CircuitData(circuit) => format!("回路{}", circuit.name),
            ElectricValueType::DistributionBoxData(box_data) => format!("配电箱{}", box_data.name),
            ElectricValueType::PhaseBalanceInfo(_) => "三相平衡".to_string(),
            _ => "".to_string(),
        }
    }
}

/// 为ElectricValueType实现Default特性
/// 提供默认值，这是WidgetValueTrait的要求
impl Default for ElectricValueType {
    fn default() -> Self {
        ElectricValueType::Float(0.0) // 使用浮点数0.0作为默认值
    }
}

/// 提供UI控件的辅助函数
pub mod widget_utils {
    use egui::{Ui, Label, RichText};
    
    /// 显示电气参数的标签，支持带单位的显示
    pub fn electric_param_label(ui: &mut Ui, label: &str, value: f64, unit: &str) {
        ui.horizontal(|ui| {
            ui.label(format!("{}:", label));
            ui.label(RichText::new(format!("{:.2} {}", value, unit)).strong());
        });
    }
    
    /// 显示警告信息
    pub fn warning_label(ui: &mut Ui, text: &str) {
        ui.label(RichText::new(text).color(egui::Color32::YELLOW));
    }
    
    /// 显示错误信息
    pub fn error_label(ui: &mut Ui, text: &str) {
        ui.label(RichText::new(text).color(egui::Color32::RED));
    }
    
    /// 显示成功信息
    pub fn success_label(ui: &mut Ui, text: &str) {
        ui.label(RichText::new(text).color(egui::Color32::GREEN));
    }
}