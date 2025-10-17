/// UI层数据结构定义
/// 用于节点编辑器和UI组件中的数据表示

use serde::{Deserialize, Serialize};
use egui_node_graph::{NodeId, NodeResponse, Graph, UserResponseTrait, NodeDataTrait, DataTypeTrait};
use egui;
use std::borrow::Cow;
use crate::core_lib::data_types::electric_data::{Phase, CircuitData, DistributionBoxData};

/// UI层的值类型枚举，用于节点编辑器中表示不同类型的数据值
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UIValueType {
    Float(f64),              // 浮点数（用于电流、功率等）
    Integer(i64),            // 整数（用于标识、数量等）
    String(String),          // 字符串（用于文本信息）
    CircuitInfo(CircuitInfo), // 简化的回路信息
    CircuitGroupInfo(CircuitGroupInfo), // 回路组信息
    DistributionBoxInfo(DistributionBoxInfo), // 简化的配电箱信息
    PhaseBalanceInfo(PhaseBalanceInfo), // 三相平衡信息
    PowerSourceInfo(PowerSourceInfo), // 电源信息
    PhaseLoadInfo(PhaseLoadInfo), // 三相负载信息
}

impl Default for UIValueType {
    fn default() -> Self {
        UIValueType::Float(0.0)
    }
}

/// 简化的回路信息结构，用于UI层显示和编辑
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CircuitInfo {
    pub id: String,          // 回路编号
    pub name: String,        // 回路名称
    pub power: f64,          // 功率(kW)
    pub phase: String,       // 相序
    pub circuit_type: String,// 回路类型
    pub current: f64,        // 计算电流(A)
}

impl Default for CircuitInfo {
    fn default() -> Self {
        Self {
            id: "WL1".to_string(),
            name: "默认回路".to_string(),
            power: 1.0,
            phase: "L1".to_string(),
            circuit_type: "照明".to_string(),
            current: 0.0,
        }
    }
}

/// 简化的配电箱信息结构，用于UI层显示和编辑
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistributionBoxInfo {
    pub id: String,          // 配电箱编号
    pub name: String,        // 配电箱名称
    pub box_type: String,    // 配电箱类型
    pub circuit_count: usize,// 回路数量
    pub total_power: f64,    // 总功率(kW)
    pub calculated_current: f64, // 计算电流(A)
    pub phase_a_load: f64,   // A相负载(kW)
    pub phase_b_load: f64,   // B相负载(kW)
    pub phase_c_load: f64,   // C相负载(kW)
}

impl Default for DistributionBoxInfo {
    fn default() -> Self {
        Self {
            id: "MDB-001".to_string(),
            name: "照明配电箱".to_string(),
            box_type: "户内嵌入式".to_string(),
            circuit_count: 0,
            total_power: 0.0,
            calculated_current: 0.0,
            phase_a_load: 0.0,
            phase_b_load: 0.0,
            phase_c_load: 0.0,
        }
    }
}

/// 三相平衡信息结构，用于UI层显示
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhaseBalanceInfo {
    pub phase_a: f64,        // A相负载(kW)
    pub phase_b: f64,        // B相负载(kW)
    pub phase_c: f64,        // C相负载(kW)
    pub balance_degree: f64, // 平衡度百分比
}

impl Default for PhaseBalanceInfo {
    fn default() -> Self {
        Self {
            phase_a: 0.0,
            phase_b: 0.0,
            phase_c: 0.0,
            balance_degree: 0.0,
        }
    }
}

/// 回路组信息结构，用于UI层显示和编辑
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CircuitGroupInfo {
    pub id: String,              // 回路组编号
    pub name: String,            // 回路组名称
    pub circuits: Vec<CircuitInfo>, // 回路列表
    pub total_power: f64,        // 总功率(kW)
    pub calculated_current: f64, // 计算电流(A)
}

impl Default for CircuitGroupInfo {
    fn default() -> Self {
        Self {
            id: "CG-001".to_string(),
            name: "照明回路组".to_string(),
            circuits: Vec::new(),
            total_power: 0.0,
            calculated_current: 0.0,
        }
    }
}

/// 电源信息结构，用于UI层显示
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PowerSourceInfo {
    pub id: String,          // 电源编号
    pub name: String,        // 电源名称
    pub voltage: f64,        // 电压(V)
    pub capacity: f64,       // 容量(kVA)
    pub source_type: String, // 电源类型
}

impl Default for PowerSourceInfo {
    fn default() -> Self {
        Self {
            id: "PS-001".to_string(),
            name: "市电电源".to_string(),
            voltage: 220.0,
            capacity: 100.0,
            source_type: "三相四线制".to_string(),
        }
    }
}

/// 干线信息结构，用于UI层显示和编辑
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MainLineInfo {
    pub id: String,              // 干线编号
    pub name: String,            // 干线名称
    pub start_current: f64,      // 起始电流(A)
    pub end_current: f64,        // 末端电流(A)
    pub voltage_drop: f64,       // 电压降(V)
    pub cable_size: String,      // 电缆规格
}

impl Default for MainLineInfo {
    fn default() -> Self {
        Self {
            id: "ML-001".to_string(),
            name: "主干线".to_string(),
            start_current: 0.0,
            end_current: 0.0,
            voltage_drop: 0.0,
            cable_size: "YJV-4x50+1x25mm²".to_string(),
        }
    }
}

/// 三相负载信息结构，用于UI层显示和编辑
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhaseLoadInfo {
    pub phase_a_power: f64,  // A相功率(kW)
    pub phase_b_power: f64,  // B相功率(kW)
    pub phase_c_power: f64,  // C相功率(kW)
    pub voltage: f64,        // 电压(V)
}

impl Default for PhaseLoadInfo {
    fn default() -> Self {
        Self {
            phase_a_power: 0.0,
            phase_b_power: 0.0,
            phase_c_power: 0.0,
            voltage: 380.0,
        }
    }
}

/// 数据类型定义，用于节点编辑器中标识不同的数据类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataType {
    Float,                  // 浮点数类型
    Integer,                // 整数类型
    String,                 // 字符串类型
    CircuitInfo,            // 回路信息类型
    CircuitGroupInfo,       // 回路组信息类型
    DistributionBoxInfo,    // 配电箱信息类型
    PhaseBalanceInfo,       // 三相平衡信息类型
    PowerSourceInfo,        // 电源信息类型
    PhaseLoadInfo,          // 三相负载信息类型
    
    // 新增电气专业数据类型
    Voltage,                // 电压类型
    Current,                // 电流类型
    Power,                  // 功率类型
    PowerFactor,            // 功率因数类型
    Coefficient,            // 系数类型
    MainLineInfo,           // 干线信息类型
}

/// UI用户状态，用于在节点编辑器操作期间存储临时状态
#[derive(Debug, Clone, Default)]
pub struct UIUserState {
    pub selected_node: Option<NodeId>,
    pub error_message: Option<String>,
    pub message_queue: Vec<String>,
    pub show_property_panel: bool,
    pub show_log_panel: bool,
    pub zoom_level: f32,
}

/// UI响应类型，用于在UI操作后返回事件
#[derive(Debug, Clone)]
pub enum UIResponse {
    ValueChanged(NodeId, String, UIValueType),
    CircuitInfoUpdated(NodeId, CircuitInfo),
    DistributionBoxInfoUpdated(NodeId, DistributionBoxInfo),
    NodeSelected(NodeId),
    NodeDeleted(NodeId),
    ConnectionAdded(NodeId, String, NodeId, String),
    ConnectionRemoved(NodeId, String, NodeId, String),
}

impl UserResponseTrait for UIResponse {}

/// 电气节点数据类型，用于表示节点编辑器中的节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElectricNodeData {
    pub node_type: ElectricNodeType, // 节点类型
    pub id: String,                  // 节点ID
    pub name: String,                // 节点名称
    pub description: String,         // 节点描述
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ElectricNodeType {
    CircuitNode,                  // 回路节点
    CircuitGroupNode,             // 回路组节点
    DistributionBoxNode,          // 通用配电箱节点
    MainDistributionBoxNode,      // 主配电箱节点
    SubDistributionBoxNode,       // 子配电箱节点
    MainLineNode,                 // 主线路节点
    FeederLineNode,               // 馈线节点
    PowerSourceNode,              // 电源节点
    CurrentCalculationNode,       // 电流计算节点
    PhaseBalanceNode,             // 相平衡节点
}

impl Default for ElectricNodeData {
    fn default() -> Self {
        Self {
            node_type: ElectricNodeType::CircuitNode,
            id: "NODE-0001".to_string(),
            name: "新节点".to_string(),
            description: "".to_string(),
        }
    }
}

// 从业务层CircuitData转换为UI层CircuitInfo
impl From<&CircuitData> for CircuitInfo {
    fn from(circuit_data: &CircuitData) -> Self {
        CircuitInfo {
            id: circuit_data.circuit_id.clone(),
            name: circuit_data.description.clone(),
            power: circuit_data.rated_power,
            phase: match circuit_data.phase_sequence {
                Phase::L1 => "L1",
                Phase::L2 => "L2",
                Phase::L3 => "L3",
                Phase::ThreePhase => "三相",
                _ => "L1",
            }.to_string(),
            circuit_type: circuit_data.circuit_type.clone(),
            current: circuit_data.calculated_current,
        }
    }
}

// 从业务层DistributionBoxData转换为UI层DistributionBoxInfo
impl From<&DistributionBoxData> for DistributionBoxInfo {
    fn from(box_data: &DistributionBoxData) -> Self {
        DistributionBoxInfo {
            id: box_data.id.clone(),
            name: box_data.name.clone(),
            box_type: "配电箱".to_string(),
            circuit_count: box_data.outgoing_circuits.len(),
            total_power: box_data.total_power,
            calculated_current: box_data.total_current,
            phase_a_load: box_data.phase_balance.phase_a_load,
            phase_b_load: box_data.phase_balance.phase_b_load,
            phase_c_load: box_data.phase_balance.phase_c_load,
        }
    }
}

// 从业务层计算三相平衡信息
impl From<&DistributionBoxData> for PhaseBalanceInfo {
    fn from(box_data: &DistributionBoxData) -> Self {
        PhaseBalanceInfo {
            phase_a: box_data.phase_balance.phase_a_load,
            phase_b: box_data.phase_balance.phase_b_load,
            phase_c: box_data.phase_balance.phase_c_load,
            balance_degree: box_data.phase_balance.unbalance_degree,
        }
    }
}

/// 从UI层CircuitInfo更新业务层CircuitData
pub fn update_circuit_data_from_ui(circuit_data: &mut CircuitData, circuit_info: &CircuitInfo) {
    circuit_data.circuit_id = circuit_info.id.clone();
    circuit_data.description = circuit_info.name.clone();
    circuit_data.rated_power = circuit_info.power;
    circuit_data.phase_sequence = match circuit_info.phase.as_str() {
        "L1" => Phase::L1,
        "L2" => Phase::L2,
        "L3" => Phase::L3,
        "三相" => Phase::ThreePhase,
        _ => Phase::L1,
    };
    circuit_data.circuit_type = circuit_info.circuit_type.clone();
    // 注意：calculated_current应该通过业务层的计算方法更新，而不是直接赋值
}

/// 从UI层DistributionBoxInfo更新业务层DistributionBoxData
pub fn update_box_data_from_ui(box_data: &mut DistributionBoxData, box_info: &DistributionBoxInfo) {
    box_data.id = box_info.id.clone();
    box_data.name = box_info.name.clone();
    // 注意：circuit_count、total_power、calculated_current等应由业务层计算，而不是直接赋值
}

// TODO: 暂时注释掉部分实现代码
// 后续将根据需求重新实现
/*
/// 为UIValueType实现WidgetValueTrait，提供参数控件的UI渲染支持
impl WidgetValueTrait for UIValueType {
    // 使用基本类型作为关联类型
    type Response = ();
    type UserState = ();
    type NodeData = ();
    
    // 实现核心方法 - 处理不同类型的值
    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut Self::UserState,
        _node_data: &Self::NodeData,
    ) -> Vec<Self::Response> {
        // 简化版本，只处理基本类型
        match self {
            UIValueType::Float(val) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(egui::DragValue::new(val));
                });
            },
            UIValueType::Integer(val) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(egui::DragValue::new(val));
                });
            },
            UIValueType::String(val) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.text_edit_singleline(val);
                });
            },
            _ => {
                // 其他类型简单显示
                ui.label(format!("{}: {}", param_name, self));
            }
        }
        
        Vec::new()
    }
}

// 为UIValueType实现Display trait，用于简化显示
impl std::fmt::Display for UIValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UIValueType::Float(val) => write!(f, "Float({:.2})", val),
            UIValueType::Integer(val) => write!(f, "Integer({})", val),
            UIValueType::String(val) => write!(f, "String({})", val),
            UIValueType::CircuitInfo(circuit) => write!(f, "Circuit({})", circuit.name),
            UIValueType::CircuitGroupInfo(group) => write!(f, "CircuitGroup({})", group.name),
            UIValueType::DistributionBoxInfo(box_data) => write!(f, "DistributionBox({})", box_data.name),
            UIValueType::PhaseBalanceInfo(_) => write!(f, "PhaseBalance"),
            UIValueType::PowerSourceInfo(source) => write!(f, "PowerSource({})", source.name),
            UIValueType::PhaseLoadInfo(_) => write!(f, "PhaseLoad"),
        }
    }
}
*/