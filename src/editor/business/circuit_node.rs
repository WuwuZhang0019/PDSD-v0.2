/// 配电回路节点实现

use std::time::Instant;
use std::collections::HashMap;
use std::borrow::Cow;
use rand;

use egui::Color32;
use slotmap::{SlotMap, SecondaryMap};

use egui_node_graph::{Graph, NodeId, InputId, OutputId, AnyParameterId, NodeResponse};
use egui_node_graph::traits::{NodeDataTrait, UserResponseTrait, NodeTemplateTrait};

use crate::core_lib::data_types::{ElectricDataType, ElectricValueType};
use crate::editor::business::{CircuitParameters, CircuitResult, VoltageType};
use crate::editor::business::circuit_calculator::CircuitCalculator;

/// 配电回路节点自定义响应类型
#[derive(Debug, Clone)]
pub enum CircuitNodeResponse {
    /// 参数更新响应
    ParameterUpdated(CircuitParameters),
    /// 计算触发响应
    CalculationTriggered,
    /// 错误响应
    Error(String),
}

/// 用户状态类型
#[derive(Debug, Default)]
pub struct EditorState;

/// 配电回路节点
#[derive(Debug, Clone)]
pub struct CircuitNode {
    /// 节点ID
    pub id: String,
    /// 回路参数
    pub parameters: CircuitParameters,
    /// 计算结果
    pub result: Option<CircuitResult>,
    /// 错误信息
    pub errors: Vec<String>,
}

impl Default for CircuitNode {
    fn default() -> Self {
        Self {
            id: format!("circuit_{:x}", rand::random::<u32>()),
            parameters: CircuitParameters::default(),
            result: None,
            errors: Vec::new(),
        }
    }
}

impl CircuitNode {
    /// 创建新的回路节点
    pub fn new(id: String, parameters: CircuitParameters) -> Self {
        Self {
            id,
            parameters,
            result: None,
            errors: Vec::new(),
        }
    }
    
    /// 更新参数
    pub fn update_parameters(&mut self, parameters: CircuitParameters) {
        self.parameters = parameters;
        self.errors = self.parameters.validation_errors();
        // 参数更新后，需要重新计算
        self.recalculate();
    }
    
    /// 执行计算
    pub fn recalculate(&mut self) {
        // 清除之前的结果
        self.result = None;
        
        // 如果有错误，不进行计算
        if !self.errors.is_empty() {
            return;
        }
        
        // 执行计算
        match CircuitCalculator::calculate_circuit_current(&self.parameters) {
            Ok(result) => {
                self.result = Some(result);
                self.errors.clear();
            },
            Err(err) => {
                self.errors.push(err.to_string());
            },
        }
    }
    
    /// 获取回路数据的哈希映射表示
    pub fn to_circuit_data_map(&self) -> HashMap<String, f64> {
        let mut map = HashMap::new();
        
        // 添加基本参数
        map.insert("pe".to_string(), self.parameters.pe as f64);
        map.insert("kx".to_string(), self.parameters.kx as f64);
        map.insert("cos".to_string(), self.parameters.cos as f64);
        map.insert("voltage_type".to_string(), match self.parameters.voltage_type {
            VoltageType::SinglePhase => 1.0,
            VoltageType::ThreePhase => 3.0,
        });
        
        // 添加计算结果（如果有）
        if let Some(result) = &self.result {
            map.insert("ijs".to_string(), result.ijs as f64);
            map.insert("voltage".to_string(), result.voltage as f64);
        }
        
        map
    }
    
    /// 获取节点标题
    pub fn get_title(&self) -> String {
        format!("回路: {}", self.parameters.name)
    }
}

// 实现NodeDataTrait
impl NodeDataTrait for CircuitNode {
    type Response = CircuitNodeResponse;
    type UserState = EditorState;
    type DataType = ElectricDataType;
    type ValueType = ElectricValueType;
    
    /// 节点底部UI，显示计算结果和错误信息
    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        _node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<Self::Response, Self>> {
        let mut responses = Vec::new();
        
        // 显示计算结果
        if let Some(result) = &self.result {
            ui.separator();
            ui.label("计算结果:");
            ui.horizontal(|ui| {
                ui.label("计算电流: ");
                ui.label(egui::RichText::new(result.formatted_ijs()).color(Color32::GREEN));
            });
            ui.horizontal(|ui| {
                ui.label("电压: ");
                ui.label(result.formatted_voltage());
            });
        }
        
        // 显示错误信息
        if !self.errors.is_empty() {
            ui.separator();
            ui.label(egui::RichText::new("错误:").color(Color32::RED));
            for error in &self.errors {
                ui.label(egui::RichText::new(format!("- {}", error)).color(Color32::RED));
            }
        }
        
        responses
    }
    
    /// 顶部栏UI，显示节点名称
    fn top_bar_ui(
        &self,
        ui: &mut egui::Ui,
        _node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<Self::Response, Self>> {
        let mut responses = Vec::new();
        
        ui.label(self.get_title());
        
        responses
    }
    
    /// 标题栏颜色
    fn titlebar_color(
        &self,
        _ui: &egui::Ui,
        _node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> Option<Color32> {
        Some(Color32::from_rgb(100, 150, 200)) // 蓝色系表示回路节点
    }
    
    /// 输出UI，显示端口信息
    fn output_ui(
        &self,
        ui: &mut egui::Ui,
        _node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        param_name: &str,
    ) -> Vec<NodeResponse<Self::Response, Self>> {
        let mut responses = Vec::new();
        
        ui.horizontal(|ui| {
            ui.label(param_name);
            
            // 如果是电流输出，显示当前计算值
            if param_name == "计算电流" && self.result.is_some() {
                ui.label(format!("({})
", self.result.as_ref().unwrap().formatted_ijs()));
            } else if param_name == "回路数据" {
                // 对于回路数据，可以显示功率信息
                ui.label(format!("({:.2}kW)", self.parameters.pe));
            }
        });
        
        responses
    }
}

// 实现UserResponseTrait
impl UserResponseTrait for CircuitNodeResponse {
    fn apply<NodeData: NodeDataTrait<Response = Self>>(
        self,
        node_id: NodeId,
        graph: &mut Graph<NodeData, NodeData::DataType, NodeData::ValueType>,
        user_state: &mut NodeData::UserState,
    ) {
        match self {
            CircuitNodeResponse::ParameterUpdated(params) => {
                if let Some(node) = graph.nodes.get_mut(node_id) {
                    if let Some(circuit_node) = node.user_data.as_any().downcast_mut::<CircuitNode>() {
                        circuit_node.update_parameters(params);
                    }
                }
            },
            CircuitNodeResponse::CalculationTriggered => {
                if let Some(node) = graph.nodes.get_mut(node_id) {
                    if let Some(circuit_node) = node.user_data.as_any().downcast_mut::<CircuitNode>() {
                        circuit_node.recalculate();
                    }
                }
            },
            CircuitNodeResponse::Error(_) => {
                // 错误已经在UI中显示，这里可以记录日志
            },
        }
    }
}

/// 配电回路节点模板
#[derive(Debug, Clone, Default)]
pub struct CircuitNodeTemplate;

impl CircuitNodeTemplate {
    /// 创建新的回路节点模板实例
    pub fn new() -> Self {
        Self::default()
    }
}

impl NodeTemplateTrait for CircuitNodeTemplate {
    type NodeData = CircuitNode;
    type DataType = ElectricDataType;
    type ValueType = ElectricValueType;
    type Category = &'static str;
    type UserState = EditorState;
    
    /// 节点查找器标签
    fn node_finder_label(&self) -> Cow<'static, str> {
        Cow::Borrowed("配电回路")
    }
    
    /// 节点查找器类别
    fn node_finder_categories(&self) -> Vec<Self::Category> {
        vec!["电气节点"]
    }
    
    /// 用户数据
    fn user_data(&self) -> Self::NodeData {
        CircuitNode::default()
    }
    
    /// 构建节点
    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        user_state: &mut Self::UserState,
    ) -> NodeId {
        // 创建新节点
        let node_data = self.user_data();
        let node_label = node_data.get_title();
        
        // 创建节点
        let node_id = graph.add_node(node_data, &node_label);
        
        // 添加输入参数
        graph.add_input_param(
            node_id,
            "回路名称",
            ElectricDataType::String,
            ElectricValueType::String("新回路".to_string()),
            egui_node_graph::InputParamKind::ConstantOnly,
            true,
        );
        
        graph.add_input_param(
            node_id,
            "额定功率(kW)",
            ElectricDataType::Power,
            ElectricValueType::Float(1.0),
            egui_node_graph::InputParamKind::ConstantOnly,
            true,
        );
        
        graph.add_input_param(
            node_id,
            "需要系数",
            ElectricDataType::Coefficient,
            ElectricValueType::Float(0.8),
            egui_node_graph::InputParamKind::ConstantOnly,
            true,
        );
        
        graph.add_input_param(
            node_id,
            "功率因数",
            ElectricDataType::PowerFactor,
            ElectricValueType::Float(0.85),
            egui_node_graph::InputParamKind::ConstantOnly,
            true,
        );
        
        graph.add_input_param(
            node_id,
            "电压类型",
            ElectricDataType::String,
            ElectricValueType::String("单相".to_string()),
            egui_node_graph::InputParamKind::ConstantOnly,
            true,
        );
        
        // 添加输出参数
        graph.add_output_param(
            node_id,
            "计算电流",
            ElectricDataType::Current,
        );
        
        graph.add_output_param(
            node_id,
            "回路数据",
            ElectricDataType::CircuitData,
        );
        
        node_id
    }
}

// 为CircuitNode实现Any trait，用于在UserResponseTrait中进行类型转换
impl std::any::Any for CircuitNode {
    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<CircuitNode>()
    }
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_circuit_node_creation() {
        let node = CircuitNode::default();
        assert_eq!(node.parameters.name, "新回路");
        assert_eq!(node.parameters.pe, 1.0);
        assert_eq!(node.parameters.kx, 0.8);
        assert_eq!(node.parameters.cos, 0.85);
        assert_eq!(node.parameters.voltage_type, VoltageType::SinglePhase);
        assert!(node.result.is_none());
        assert!(node.errors.is_empty());
    }
    
    #[test]
    fn test_parameter_update() {
        let mut node = CircuitNode::default();
        let new_params = CircuitParameters::new(
            "测试回路".to_string(),
            5.0,
            0.7,
            0.9,
            VoltageType::ThreePhase
        );
        
        node.update_parameters(new_params.clone());
        
        assert_eq!(node.parameters.name, new_params.name);
        assert_eq!(node.parameters.pe, new_params.pe);
        assert_eq!(node.parameters.kx, new_params.kx);
        assert_eq!(node.parameters.cos, new_params.cos);
        assert_eq!(node.parameters.voltage_type, new_params.voltage_type);
        assert!(node.errors.is_empty());
    }
    
    #[test]
    fn test_recalculation() {
        let mut node = CircuitNode::default();
        node.recalculate();
        
        assert!(node.result.is_some());
        if let Some(result) = &node.result {
            // 验证计算结果的合理性
            assert!(result.ijs > 0.0);
            assert!(result.ijs < 100.0); // 1kW单相负载电流应该小于100A
        }
    }
}