/// 配电回路节点实现

use std::time::Instant;
use std::collections::HashMap;
use std::borrow::Cow;
use std::fmt;
use rand;
use uuid::Uuid;

use egui::Color32;
use slotmap::{SlotMap, SecondaryMap};

use egui_node_graph::{Graph, NodeId, InputId, OutputId, AnyParameterId, NodeResponse};
use egui_node_graph::traits::{NodeDataTrait, UserResponseTrait, NodeTemplateTrait};

use crate::core_lib::data_types::{ElectricDataType, ElectricValueType, CircuitNodeProperties, CircuitType, CircuitPurpose};
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
    /// 回路属性（与core_lib定义一致）
    pub properties: CircuitNodeProperties,
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
            properties: CircuitNodeProperties::default(),
            result: None,
            errors: Vec::new(),
        }
    }
}

impl CircuitNode {
    /// 创建新的回路节点
    pub fn new(id: String, parameters: CircuitParameters) -> Self {
        let mut node = Self {
            id,
            parameters,
            properties: CircuitNodeProperties::default(),
            result: None,
            errors: Vec::new(),
        };
        
        // 同步参数到properties
        node.sync_parameters_to_properties();
        node
    }
    
    /// 创建新的单相回路节点
    pub fn new_single_phase(name: &str, power: f64) -> Self {
        let params = CircuitParameters {
            name: name.to_string(),
            pe: power,
            kx: 1.0,
            cos: 0.85,
            voltage_type: VoltageType::SinglePhase,
        };
        
        let mut node = Self {
            id: format!("circuit_{:x}", rand::random::<u32>()),
            parameters: params,
            properties: CircuitNodeProperties::default(),
            result: None,
            errors: Vec::new(),
        };
        
        node.properties.circuit_type = CircuitType::SinglePhase;
        node.properties.voltage = 220.0;
        node.sync_parameters_to_properties();
        
        // 初始化时先验证再计算
        node.validate();
        node.recalculate();
        node
    }
    
    /// 创建新的三相回路节点
    pub fn new_three_phase(name: &str, power: f64) -> Self {
        let params = CircuitParameters {
            name: name.to_string(),
            pe: power,
            kx: 0.8,
            cos: 0.85,
            voltage_type: VoltageType::ThreePhase,
        };
        
        let mut node = Self {
            id: format!("circuit_{:x}", rand::random::<u32>()),
            parameters: params,
            properties: CircuitNodeProperties::default(),
            result: None,
            errors: Vec::new(),
        };
        
        node.properties.circuit_type = CircuitType::ThreePhase;
        node.properties.voltage = 380.0;
        node.properties.phase = None;
        node.sync_parameters_to_properties();
        
        // 初始化时先验证再计算
        node.validate();
        node.recalculate();
        node
    }
    
    /// 将CircuitParameters同步到CircuitNodeProperties
    pub fn sync_parameters_to_properties(&mut self) {
        self.properties.power = self.parameters.pe;
        self.properties.demand_factor = self.parameters.kx;
        self.properties.power_factor = self.parameters.cos;
        
        // 根据电压类型设置回路类型
        if self.parameters.voltage_type == VoltageType::SinglePhase {
            self.properties.circuit_type = CircuitType::SinglePhase;
            self.properties.voltage = 220.0;
        } else {
            self.properties.circuit_type = CircuitType::ThreePhase;
            self.properties.voltage = 380.0;
        }
        
        // 设置其他属性
        self.properties.circuit_name = self.parameters.name.clone();
    }
    
    /// 更新参数
    pub fn update_parameters(&mut self, parameters: CircuitParameters) {
        self.parameters = parameters;
        
        // 同步参数到properties
        self.sync_parameters_to_properties();
        
        // 验证并重新计算
        self.validate();
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
        
        // 使用CircuitNodeProperties进行计算
        self.properties.perform_all_calculations();
        
        // 执行原有计算逻辑
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
    
    /// 获取回路属性数据
    pub fn get_properties(&self) -> &CircuitNodeProperties {
        &self.properties
    }
    
    /// 获取回路属性数据（可变）
    pub fn get_properties_mut(&mut self) -> &mut CircuitNodeProperties {
        &mut self.properties
    }
    
    /// 获取回路类型
    pub fn get_circuit_type(&self) -> CircuitType {
        self.properties.circuit_type
    }
    
    /// 设置回路类型
    pub fn set_circuit_type(&mut self, circuit_type: CircuitType) {
        self.properties.circuit_type = circuit_type;
        
        // 根据回路类型更新电压类型
        match circuit_type {
            CircuitType::SinglePhase => {
                self.parameters.voltage_type = VoltageType::SinglePhase;
                self.properties.voltage = 220.0;
            },
            CircuitType::ThreePhase => {
                self.parameters.voltage_type = VoltageType::ThreePhase;
                self.properties.voltage = 380.0;
            },
        }
        
        // 重新验证和计算
        self.validate();
        self.recalculate();
    }
    
    /// 验证节点参数的有效性
    pub fn validate(&mut self) {
        self.errors.clear();
        
        // 验证功率是否有效
        if self.parameters.pe <= 0.0 {
            self.errors.push("功率必须大于0".to_string());
        }
        
        // 验证需求系数是否有效
        if self.parameters.kx <= 0.0 || self.parameters.kx > 1.0 {
            self.errors.push("需求系数必须在0和1之间".to_string());
        }
        
        // 验证功率因数是否有效
        if self.parameters.cos <= 0.0 || self.parameters.cos > 1.0 {
            self.errors.push("功率因数必须在0和1之间".to_string());
        }
        
        // 验证名称是否为空
        if self.parameters.name.trim().is_empty() {
            self.errors.push("回路名称不能为空".to_string());
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
        
        ui.separator();
        
        // 显示回路类型和基本信息
        ui.label("回路信息:");
        ui.horizontal(|ui| {
            ui.label("回路类型: ");
            ui.label(self.properties.circuit_type.to_str());
        });
        
        // 显示计算结果
        ui.label("计算结果:");
        ui.horizontal(|ui| {
            ui.label("计算电流: ");
            ui.label(egui::RichText::new(format!("{:.2} A", self.properties.current)).color(Color32::GREEN));
        });
        
        ui.horizontal(|ui| {
            ui.label("1.1倍电流: ");
            ui.label(egui::RichText::new(format!("{:.2} A", self.properties.current_1_1x)).color(Color32::BLUE));
        });
        
        ui.horizontal(|ui| {
            ui.label("1.25倍电流: ");
            ui.label(egui::RichText::new(format!("{:.2} A", self.properties.current_1_25x)).color(Color32::YELLOW));
        });
        
        // 显示选型结果
        ui.label("选型结果:");
        ui.horizontal(|ui| {
            ui.label("元器件类型: ");
            ui.label(&self.properties.component_type);
        });
        
        ui.horizontal(|ui| {
            ui.label("元器件电流: ");
            ui.label(egui::RichText::new(format!("{:.0} A", self.properties.component_current)).color(Color32::PURPLE));
        });
        
        ui.horizontal(|ui| {
            ui.label("线缆规格: ");
            ui.label(egui::RichText::new(&self.properties.cable_spec).color(Color32::CYAN));
        });
        
        // 如果是单相回路，显示相序
        if let Some(phase) = self.properties.phase {
            ui.horizontal(|ui| {
                ui.label("相序: ");
                ui.label(format!("{}", phase));
            });
        }
        
        // 显示原有的计算结果
        if let Some(result) = &self.result {
            ui.label("详细数据:");
            ui.horizontal(|ui| {
                ui.label("原计算电流: ");
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
impl UserResponseTrait for CircuitNodeResponse {}

// 为CircuitNode实现Display trait
impl fmt::Display for CircuitNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CircuitNode '{}': PE={}kW, KX={}, COS={}, Type={}", 
            self.parameters.name,
            self.parameters.pe,
            self.parameters.kx,
            self.parameters.cos,
            match self.parameters.voltage_type {
                VoltageType::SinglePhase => "Single Phase",
                VoltageType::ThreePhase => "Three Phase"
            }
        )?;
        
        // 添加计算结果信息（如果有）
        if let Some(result) = &self.result {
            write!(f, ", Current={}A", result.formatted_ijs())?;
        }
        
        Ok(())
    }
}

// 为CircuitNodeResponse实现apply方法（不是trait的一部分）
impl CircuitNodeResponse {
    /// 应用响应到节点图
    pub fn apply(
        self,
        node_id: NodeId,
        graph: &mut Graph<CircuitNode, ElectricDataType, ElectricValueType>,
        user_state: &mut EditorState,
    ) {
        match self {
            CircuitNodeResponse::ParameterUpdated(params) => {
                if let Some(node) = graph.nodes.get_mut(node_id) {
                    node.user_data.update_parameters(params);
                }
            },
            CircuitNodeResponse::CalculationTriggered => {
                if let Some(node) = graph.nodes.get_mut(node_id) {
                    node.user_data.recalculate();
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
    type CategoryType = &'static str;
    type UserState = EditorState;
    
    /// 节点查找器标签
    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'static, str> {
        Cow::Borrowed("配电回路")
    }
    
    /// 节点图标签
    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        self.node_finder_label(user_state).into_owned()
    }
    
    /// 节点查找器类别
    fn node_finder_categories(&self, _user_state: &mut Self::UserState) -> Vec<Self::CategoryType> {
        vec!["电气节点"]
    }
    
    /// 用户数据
    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        CircuitNode::default()
    }
    
    /// 构建节点
    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        // 创建新节点数据
        let node_data = self.user_data(user_state);
        let node_label = node_data.get_title();
        
        // 设置节点数据
        graph.nodes[node_id].user_data = node_data;
        graph.nodes[node_id].label = node_label;
        
        // 添加输入参数
        graph.add_input_param(
            node_id,
            "回路名称".to_string(),
            ElectricDataType::String,
            ElectricValueType::String("新回路".to_string()),
            egui_node_graph::InputParamKind::ConstantOnly,
            true,
        );
        
        graph.add_input_param(
            node_id,
            "额定功率(kW)".to_string(),
            ElectricDataType::Power,
            ElectricValueType::Float(1.0),
            egui_node_graph::InputParamKind::ConstantOnly,
            true,
        );
        
        graph.add_input_param(
            node_id,
            "需要系数".to_string(),
            ElectricDataType::Coefficient,
            ElectricValueType::Float(0.8),
            egui_node_graph::InputParamKind::ConstantOnly,
            true,
        );
        
        graph.add_input_param(
            node_id,
            "功率因数".to_string(),
            ElectricDataType::PowerFactor,
            ElectricValueType::Float(0.85),
            egui_node_graph::InputParamKind::ConstantOnly,
            true,
        );
        
        graph.add_input_param(
            node_id,
            "电压类型".to_string(),
            ElectricDataType::String,
            ElectricValueType::String("单相".to_string()),
            egui_node_graph::InputParamKind::ConstantOnly,
            true,
        );
        
        // 添加输出参数
        graph.add_output_param(
            node_id,
            "计算电流".to_string(),
            ElectricDataType::Current,
        );
        
        graph.add_output_param(
            node_id,
            "回路数据".to_string(),
            ElectricDataType::CircuitData,
        );
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