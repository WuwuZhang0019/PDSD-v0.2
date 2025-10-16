use std::collections::HashMap;
use crate::core_lib::data_types::{ElectricNodeData, ElectricValueType};

/// 电气计算接口
/// 定义所有电气计算功能必须实现的方法
pub trait ElectricalCalculation {
    /// 计算电流
    /// - power: 功率(kW)
    /// - voltage: 电压(V)
    /// - power_factor: 功率因数
    /// - phase_count: 相数(1/3)
    fn calculate_current(power: f64, voltage: f64, power_factor: f64, phase_count: u32) -> f64;
    
    /// 计算功率
    /// - current: 电流(A)
    /// - voltage: 电压(V)
    /// - power_factor: 功率因数
    /// - phase_count: 相数(1/3)
    fn calculate_power(current: f64, voltage: f64, power_factor: f64, phase_count: u32) -> f64;
    
    /// 计算电压降
    /// - current: 电流(A)
    /// - resistance: 电阻(Ω)
    /// - reactance: 电抗(Ω)
    /// - power_factor: 功率因数
    fn calculate_voltage_drop(current: f64, resistance: f64, reactance: f64, power_factor: f64) -> f64;
    
    /// 计算电缆电阻
    /// - length: 长度(m)
    /// - cross_section: 截面积(mm²)
    /// - resistivity: 电阻率(Ω·m)
    fn calculate_resistance(length: f64, cross_section: f64, resistivity: f64) -> f64;
    
    /// 计算三相平衡度
    /// - phase_a: A相功率或电流
    /// - phase_b: B相功率或电流
    /// - phase_c: C相功率或电流
    fn calculate_phase_balance(phase_a: f64, phase_b: f64, phase_c: f64) -> f64;
    
    /// 应用需用系数
    /// - power: 额定功率
    /// - coefficient: 需用系数
    fn apply_demand_coefficient(power: f64, coefficient: f64) -> f64;
}

/// 图计算接口
/// 定义基于图结构的电气计算功能
pub trait GraphCalculation {
    /// 计算整个图的参数
    /// - graph: 图数据结构
    /// - results: 计算结果缓存
    fn calculate_graph(graph: &HashMap<String, ElectricNodeData>, results: &mut HashMap<String, f64>) -> Result<(), String>;
    
    /// 计算单个节点的参数
    /// - node_id: 节点ID
    /// - node_data: 节点数据
    /// - graph: 图数据结构
    /// - results: 计算结果缓存
    fn calculate_node(node_id: &str, node_data: &mut ElectricNodeData, graph: &HashMap<String, ElectricNodeData>, results: &mut HashMap<String, f64>) -> Result<(), String>;
    
    /// 验证图的电气合理性
    /// - graph: 图数据结构
    fn validate_graph(graph: &HashMap<String, ElectricNodeData>) -> Vec<String>;
}

/// 计算结果处理接口
pub trait CalculationResult {
    /// 获取计算结果
    fn get_result(&self) -> f64;
    
    /// 设置计算结果
    fn set_result(&mut self, result: f64);
    
    /// 格式化计算结果
    fn format_result(&self, precision: u32) -> String;
}

/// 默认计算实现
pub struct DefaultElectricalCalculation;

impl ElectricalCalculation for DefaultElectricalCalculation {
    fn calculate_current(power: f64, voltage: f64, power_factor: f64, phase_count: u32) -> f64 {
        // 功率(kW)转换为瓦(W)
        let power_w = power * 1000.0;
        // 三相计算: I = P / (√3 * U * cosφ)
        // 单相计算: I = P / (U * cosφ)
        let sqrt_3 = 3.0f64.sqrt();
        
        if phase_count == 3 {
            power_w / (sqrt_3 * voltage * power_factor)
        } else {
            power_w / (voltage * power_factor)
        }
    }
    
    fn calculate_power(current: f64, voltage: f64, power_factor: f64, phase_count: u32) -> f64 {
        // 三相计算: P = √3 * U * I * cosφ
        // 单相计算: P = U * I * cosφ
        let sqrt_3 = 3.0f64.sqrt();
        let power_w: f64;
        
        if phase_count == 3 {
            power_w = sqrt_3 * voltage * current * power_factor
        } else {
            power_w = voltage * current * power_factor
        }
        
        // 转换为kW
        power_w / 1000.0
    }
    
    fn calculate_voltage_drop(current: f64, resistance: f64, reactance: f64, power_factor: f64) -> f64 {
        // 计算有功部分电压降: I * R * cosφ
        let active_drop = current * resistance * power_factor;
        // 计算无功部分电压降: I * X * sinφ
        let reactive_drop = current * reactance * (1.0 - power_factor.powi(2)).sqrt();
        
        // 总电压降
        (active_drop.powi(2) + reactive_drop.powi(2)).sqrt()
    }
    
    fn calculate_resistance(length: f64, cross_section: f64, resistivity: f64) -> f64 {
        // R = ρ * L / S
        // ρ: 电阻率(Ω·m)
        // L: 长度(m)
        // S: 截面积(m²)
        // 注意：cross_section单位是mm²，需要转换为m²
        let cross_section_m2 = cross_section / 1_000_000.0;
        resistivity * length / cross_section_m2
    }
    
    fn calculate_phase_balance(phase_a: f64, phase_b: f64, phase_c: f64) -> f64 {
        let max_phase = phase_a.max(phase_b).max(phase_c);
        let min_phase = phase_a.min(phase_b).min(phase_c);
        
        if max_phase == 0.0 {
            return 0.0;
        }
        
        // 不平衡度 = (最大值 - 最小值) / 最大值 * 100%
        ((max_phase - min_phase) / max_phase) * 100.0
    }
    
    fn apply_demand_coefficient(power: f64, coefficient: f64) -> f64 {
        power * coefficient
    }
}

/// 字符串计算结果实现
impl CalculationResult for f64 {
    fn get_result(&self) -> f64 {
        *self
    }
    
    fn set_result(&mut self, result: f64) {
        *self = result;
    }
    
    fn format_result(&self, precision: u32) -> String {
        format!("{:.precision$}", self, precision = precision as usize)
    }
}