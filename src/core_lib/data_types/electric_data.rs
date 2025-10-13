use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 由于egui不在core_lib中直接使用，需要条件编译
#[cfg(feature = "with_egui")]
extern crate egui;
#[cfg(feature = "with_egui")]
use egui;

/// 电气系统数据类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElectricDataType {
    // 电气参数
    Current,        // 电流(A)
    Power,          // 功率(kW)
    Voltage,        // 电压(V)
    PowerFactor,    // 功率因数
    Coefficient,    // 需用系数
    // 配电系统参数
    CircuitData,    // 回路数据
    DistributionBoxData, // 配电箱数据
    ThreePhaseData, // 三相数据
    // 标识数据
    String,         // 字符串标识
    Integer,        // 整数标识
}

impl ElectricDataType {
    /// 获取数据类型的显示颜色
    #[cfg(feature = "with_egui")]
    pub fn data_type_color(&self) -> egui::Color32 {
        // 根据数据类型返回不同的颜色，用于端口显示
        match self {
            ElectricDataType::Current => egui::Color32::from_rgb(255, 100, 100),      // 红色 - 电流
            ElectricDataType::Power => egui::Color32::from_rgb(100, 200, 100),        // 绿色 - 功率
            ElectricDataType::Voltage => egui::Color32::from_rgb(100, 100, 255),      // 蓝色 - 电压
            ElectricDataType::PowerFactor => egui::Color32::from_rgb(255, 255, 100),  // 黄色 - 功率因数
            ElectricDataType::Coefficient => egui::Color32::from_rgb(255, 100, 255),  // 紫色 - 系数
            ElectricDataType::CircuitData => egui::Color32::from_rgb(200, 150, 100),  // 棕色 - 回路数据
            ElectricDataType::DistributionBoxData => egui::Color32::from_rgb(100, 200, 200), // 青色 - 配电箱数据
            ElectricDataType::ThreePhaseData => egui::Color32::from_rgb(200, 100, 200), // 紫红色 - 三相数据
            ElectricDataType::String => egui::Color32::from_rgb(200, 200, 200),       // 灰色 - 字符串
            ElectricDataType::Integer => egui::Color32::from_rgb(150, 150, 150),       // 深灰色 - 整数
        }
    }

    /// 获取数据类型的显示名称
    pub fn data_type_name(&self) -> &str {
        match self {
            ElectricDataType::Current => "电流(A)",
            ElectricDataType::Power => "功率(kW)",
            ElectricDataType::Voltage => "电压(V)",
            ElectricDataType::PowerFactor => "功率因数",
            ElectricDataType::Coefficient => "需用系数",
            ElectricDataType::CircuitData => "回路数据",
            ElectricDataType::DistributionBoxData => "配电箱数据",
            ElectricDataType::ThreePhaseData => "三相数据",
            ElectricDataType::String => "字符串",
            ElectricDataType::Integer => "整数",
        }
    }
}

// 三相平衡信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhaseBalanceInfo {
    pub phase_a_power: f64,   // A相功率
    pub phase_b_power: f64,   // B相功率
    pub phase_c_power: f64,   // C相功率
    pub balance_degree: f64,  // 不平衡度
}

// 定义电气系统参数值类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ElectricValueType {
    Float(f64),              // 用于电流、功率等数值
    Integer(i64),            // 用于整数标识
    String(String),          // 用于型号、名称等文本
    CircuitData(HashMap<String, f64>), // 配电回路数据
    DistributionBoxData(HashMap<String, f64>), // 配电箱数据
    ThreePhaseData(PhaseBalanceInfo), // 三相数据
}

impl ElectricValueType {
    /// 获取浮点数值
    pub fn as_float(&self) -> Option<f64> {
        match self {
            ElectricValueType::Float(val) => Some(*val),
            _ => None,
        }
    }
    
    /// 获取整数值
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            ElectricValueType::Integer(val) => Some(*val),
            _ => None,
        }
    }
    
    /// 获取字符串值
    pub fn as_string(&self) -> Option<&str> {
        match self {
            ElectricValueType::String(val) => Some(val),
            _ => None,
        }
    }
    
    /// 获取回路数据
    pub fn as_circuit_data(&self) -> Option<&HashMap<String, f64>> {
        match self {
            ElectricValueType::CircuitData(data) => Some(data),
            _ => None,
        }
    }
    
    /// 获取配电箱数据
    pub fn as_distribution_box_data(&self) -> Option<&HashMap<String, f64>> {
        match self {
            ElectricValueType::DistributionBoxData(data) => Some(data),
            _ => None,
        }
    }
    
    /// 获取三相数据
    pub fn as_three_phase_data(&self) -> Option<&PhaseBalanceInfo> {
        match self {
            ElectricValueType::ThreePhaseData(data) => Some(data),
            _ => None,
        }
    }
}

// 为了在egui_node_graph中使用，需要实现相应的trait
#[cfg(feature = "with_egui")]
impl egui_node_graph::DataTypeTrait for ElectricDataType {
    fn data_type_color(&self) -> egui::Color32 {
        // 根据数据类型返回不同的颜色，用于端口显示
        match self {
            ElectricDataType::Current => egui::Color32::from_rgb(255, 100, 100),      // 红色 - 电流
            ElectricDataType::Power => egui::Color32::from_rgb(100, 200, 100),        // 绿色 - 功率
            ElectricDataType::Voltage => egui::Color32::from_rgb(100, 100, 255),      // 蓝色 - 电压
            ElectricDataType::PowerFactor => egui::Color32::from_rgb(255, 255, 100),  // 黄色 - 功率因数
            ElectricDataType::Coefficient => egui::Color32::from_rgb(255, 100, 255),  // 紫色 - 系数
            ElectricDataType::CircuitData => egui::Color32::from_rgb(200, 150, 100),  // 棕色 - 回路数据
            ElectricDataType::DistributionBoxData => egui::Color32::from_rgb(100, 200, 200), // 青色 - 配电箱数据
            ElectricDataType::ThreePhaseData => egui::Color32::from_rgb(200, 100, 200), // 紫红色 - 三相数据
            ElectricDataType::String => egui::Color32::from_rgb(200, 200, 200),       // 灰色 - 字符串
            ElectricDataType::Integer => egui::Color32::from_rgb(150, 150, 150),       // 深灰色 - 整数
        }
    }
    
    fn data_type_name(&self) -> &str {
        self.data_type_name()
    }
}

// 由于egui不在core_lib中直接使用，需要条件编译
#[cfg(feature = "with_egui")]
use egui;

// 实现默认值
impl Default for PhaseBalanceInfo {
    fn default() -> Self {
        Self {
            phase_a_power: 0.0,
            phase_b_power: 0.0,
            phase_c_power: 0.0,
            balance_degree: 0.0,
        }
    }
}