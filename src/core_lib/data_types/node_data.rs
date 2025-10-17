use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::process::id as process_id;

use super::electric_data::{Breaker, CableInfo, CircuitData, CircuitNumber, Phase, PhaseBalanceInfo};

/// 物理尺寸
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: f64,    // 宽度(mm)
    pub height: f64,   // 高度(mm)
    pub depth: f64,    // 深度(mm)
}

impl Default for Dimensions {
    fn default() -> Self {
        Self {
            width: 400.0,
            height: 600.0,
            depth: 200.0,
        }
    }
}

/// 配电箱数据结构
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistributionBoxData {
    pub id: String,                      // 唯一标识符
    pub name: String,                    // 配电箱名称
    pub number: CircuitNumber,           // 配电箱编号
    pub incoming_circuit: Option<String>, // 进线回路ID
    pub outgoing_circuits: Vec<String>,  // 出线回路ID列表
    pub total_power: f64,                // 总功率(kW)
    pub total_current: f64,              // 总电流(A)
    pub voltage_level: f64,              // 电压等级(kV)
    pub phase_count: u8,                 // 相数(1或3)
    pub main_protection: Breaker,        // 主保护设备
    pub monitoring_module: Option<String>, // 监测模块
    pub phase_balance: PhaseBalanceInfo, // 三相平衡信息
    pub location: String,                // 安装位置
    pub dimensions: Dimensions,          // 箱体尺寸
    pub manufacturer: Option<String>,    // 制造商
}

impl Default for DistributionBoxData {
    fn default() -> Self {
        Self {
            id: generate_unique_id(),
            name: "配电箱".to_string(),
            number: CircuitNumber::new(1),
            incoming_circuit: None,
            outgoing_circuits: Vec::new(),
            total_power: 0.0,
            total_current: 0.0,
            voltage_level: 0.4,
            phase_count: 3,
            main_protection: Breaker::default(),
            monitoring_module: None,
            phase_balance: PhaseBalanceInfo::default(),
            location: "" .to_string(),
            dimensions: Dimensions::default(),
            manufacturer: None,
        }
    }
}

impl DistributionBoxData {
    /// 添加出线回路
    pub fn add_outgoing_circuit(&mut self, circuit_id: String) {
        if !self.outgoing_circuits.contains(&circuit_id) {
            self.outgoing_circuits.push(circuit_id);
        }
    }
    
    /// 移除出线回路
    pub fn remove_outgoing_circuit(&mut self, circuit_id: &str) {
        self.outgoing_circuits.retain(|id| id != circuit_id);
    }
    
    /// 更新总功率和总电流
    pub fn update_total_power_and_current(&mut self, circuits: &HashMap<String, CircuitData>) {
        let mut total_p = 0.0;
        let mut total_i = 0.0;
        
        for circuit_id in &self.outgoing_circuits {
            if let Some(circuit) = circuits.get(circuit_id) {
                total_p += circuit.rated_power * circuit.demand_coefficient;
                total_i += circuit.calculated_current;
            }
        }
        
        self.total_power = total_p;
        self.total_current = total_i;
    }
    
    /// 获取下一个可用的回路编号
    pub fn get_next_circuit_number(&self) -> CircuitNumber {
        let last_number = self.outgoing_circuits.iter()
            .filter_map(|id| CircuitNumber::from_str(id))
            .max_by(|a, b| a.compare(b))
            .unwrap_or_else(|| CircuitNumber::new(0));
        
        last_number.next()
    }
    
    /// 获取配电箱的完整编号字符串
    pub fn get_full_number(&self) -> String {
        format!("AL{}", self.number.number)
    }
}

/// 生成唯一标识符
fn generate_unique_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    let pid = process_id();
    let thread_id = std::thread::current().id();
    format!("{}_{}_{:?}", timestamp, pid, thread_id)
}

/// 配电回路节点数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitNodeData {
    // 回路基本信息
    pub name: String,
    pub description: String,
    
    // 电气参数
    pub rated_power: f64,         // 额定功率(kW)
    pub power_factor: f64,        // 功率因数
    pub demand_coefficient: f64,  // 需用系数
    pub current: f64,             // 计算电流(A)
    
    // 回路类型和相数
    pub circuit_type: String,     // 照明/动力/混合
    pub phase_type: String,       // 单相/三相
    
    // 保护参数
    pub protection_current: f64,  // 保护电流(A)
    pub wire_size: String,        // 导线规格
}

/// 配电箱节点数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionBoxNodeData {
    // 配电箱基本信息
    pub name: String,
    pub description: String,
    pub box_type: String,         // 配电箱类型
    
    // 电气参数
    pub rated_voltage: f64,       // 额定电压(V)
    pub rated_current: f64,       // 额定电流(A)
    pub total_power: f64,         // 总功率(kW)
    
    // 三相负载分布
    pub phase_a_load: f64,        // A相负载(kW)
    pub phase_b_load: f64,        // B相负载(kW)
    pub phase_c_load: f64,        // C相负载(kW)
    
    // 进线参数
    pub incoming_current: f64,    // 进线电流(A)
    pub incoming_wire_size: String, // 进线规格
}

/// 干线系统图节点数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrunkLineNodeData {
    // 干线基本信息
    pub name: String,
    pub description: String,
    pub line_type: String,        // 干线类型
    
    // 电气参数
    pub length: f64,              // 长度(m)
    pub resistance: f64,          // 电阻(Ω)
    pub reactance: f64,           // 电抗(Ω)
    pub voltage_drop: f64,        // 电压降(V)
    
    // 负荷参数
    pub total_current: f64,       // 总电流(A)
    pub wire_size: String,        // 导线规格
    
    // 三相参数
    pub phase_a_current: f64,     // A相电流(A)
    pub phase_b_current: f64,     // B相电流(A)
    pub phase_c_current: f64,     // C相电流(A)
}

/// 电源节点数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerSourceNodeData {
    // 电源基本信息
    pub name: String,
    pub description: String,
    pub source_type: String,      // 电源类型
    
    // 电气参数
    pub voltage: f64,             // 电压(V)
    pub frequency: f64,           // 频率(Hz)
    pub capacity: f64,            // 容量(kVA)
    
    // 相数信息
    pub phase_count: u32,         // 相数(1/3)
    
    // 效率参数
    pub efficiency: f64,          // 效率
}

/// 计算节点数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationNodeData {
    // 计算节点基本信息
    pub name: String,
    pub calculation_type: String, // 计算类型
    
    // 计算参数
    pub result: f64,              // 计算结果
    pub precision: u32,           // 计算精度
    
    // 特定计算类型参数
    pub phase_balance_degree: f64, // 三相平衡度(%)
    pub voltage_loss_percent: f64, // 电压损失率(%)
}

/// 节点数据枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElectricNodeData {
    // 配电回路节点
    CircuitNode(CircuitNodeData),
    // 配电箱节点
    DistributionBoxNode(DistributionBoxNodeData),
    // 干线系统图节点
    TrunkLineNode(TrunkLineNodeData),
    // 电源节点
    PowerSourceNode(PowerSourceNodeData),
    // 计算节点
    CalculationNode(CalculationNodeData),
}

// 实现各种节点数据的默认值
impl Default for CircuitNodeData {
    fn default() -> Self {
        Self {
            name: "回路".to_string(),
            description: "".to_string(),
            rated_power: 1.0,
            power_factor: 0.8,
            demand_coefficient: 0.8,
            current: 0.0,
            circuit_type: "照明".to_string(),
            phase_type: "单相".to_string(),
            protection_current: 10.0,
            wire_size: "BV-2.5mm²".to_string(),
        }
    }
}

impl Default for DistributionBoxNodeData {
    fn default() -> Self {
        Self {
            name: "配电箱".to_string(),
            description: "".to_string(),
            box_type: "照明配电箱".to_string(),
            rated_voltage: 220.0,
            rated_current: 100.0,
            total_power: 0.0,
            phase_a_load: 0.0,
            phase_b_load: 0.0,
            phase_c_load: 0.0,
            incoming_current: 0.0,
            incoming_wire_size: "BV-25mm²".to_string(),
        }
    }
}

impl Default for TrunkLineNodeData {
    fn default() -> Self {
        Self {
            name: "干线".to_string(),
            description: "".to_string(),
            line_type: "电缆干线".to_string(),
            length: 100.0,
            resistance: 0.0,
            reactance: 0.0,
            voltage_drop: 0.0,
            total_current: 0.0,
            wire_size: "VV-4×25+1×16mm²".to_string(),
            phase_a_current: 0.0,
            phase_b_current: 0.0,
            phase_c_current: 0.0,
        }
    }
}

impl Default for PowerSourceNodeData {
    fn default() -> Self {
        Self {
            name: "电源".to_string(),
            description: "".to_string(),
            source_type: "三相交流电源".to_string(),
            voltage: 380.0,
            frequency: 50.0,
            capacity: 100.0,
            phase_count: 3,
            efficiency: 0.9,
        }
    }
}

impl Default for CalculationNodeData {
    fn default() -> Self {
        Self {
            name: "计算器".to_string(),
            calculation_type: "三相平衡度".to_string(),
            result: 0.0,
            precision: 2,
            phase_balance_degree: 0.0,
            voltage_loss_percent: 0.0,
        }
    }
}

// 为ElectricNodeData实现一些辅助方法
impl ElectricNodeData {
    /// 获取节点名称
    pub fn get_name(&self) -> &str {
        match self {
            ElectricNodeData::CircuitNode(data) => &data.name,
            ElectricNodeData::DistributionBoxNode(data) => &data.name,
            ElectricNodeData::TrunkLineNode(data) => &data.name,
            ElectricNodeData::PowerSourceNode(data) => &data.name,
            ElectricNodeData::CalculationNode(data) => &data.name,
        }
    }
    
    /// 设置节点名称
    pub fn set_name(&mut self, name: String) {
        match self {
            ElectricNodeData::CircuitNode(data) => data.name = name,
            ElectricNodeData::DistributionBoxNode(data) => data.name = name,
            ElectricNodeData::TrunkLineNode(data) => data.name = name,
            ElectricNodeData::PowerSourceNode(data) => data.name = name,
            ElectricNodeData::CalculationNode(data) => data.name = name,
        }
    }
    
    /// 获取节点类型名称
    pub fn get_type_name(&self) -> &str {
        match self {
            ElectricNodeData::CircuitNode(_) => "回路",
            ElectricNodeData::DistributionBoxNode(_) => "配电箱",
            ElectricNodeData::TrunkLineNode(_) => "干线",
            ElectricNodeData::PowerSourceNode(_) => "电源",
            ElectricNodeData::CalculationNode(_) => "计算器",
        }
    }
    
    /// 计算三相不平衡度
    pub fn calculate_phase_balance(phase_a: f64, phase_b: f64, phase_c: f64) -> f64 {
        let max_phase = phase_a.max(phase_b).max(phase_c);
        let min_phase = phase_a.min(phase_b).min(phase_c);
        
        if max_phase == 0.0 {
            return 0.0;
        }
        
        ((max_phase - min_phase) / max_phase) * 100.0
    }
}