use std::collections::HashMap;
use crate::core_lib::{ElectricNodeData, DefaultElectricalCalculation};

/// 电气计算管理器
/// 负责处理各种电气计算逻辑
pub struct ElectricalCalculator {
    // 电阻率数据（铜、铝等材质）
    resistivity_data: HashMap<String, f64>,
}

impl Default for ElectricalCalculator {
    fn default() -> Self {
        let mut resistivity_data = HashMap::new();
        // 添加常用材料的电阻率 (Ω·m) at 20°C
        resistivity_data.insert("copper".to_string(), 1.72e-8);  // 铜
        resistivity_data.insert("aluminum".to_string(), 2.82e-8); // 铝
        
        Self {
            resistivity_data,
        }
    }
}

impl ElectricalCalculator {
    /// 创建新的电气计算器
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 计算回路电流
    /// - rated_power: 额定功率(kW)
    /// - voltage: 电压(V)
    /// - power_factor: 功率因数
    /// - demand_coefficient: 需用系数
    /// - phase_count: 相数
    pub fn calculate_circuit_current(&self, rated_power: f64, voltage: f64, power_factor: f64, demand_coefficient: f64, phase_count: u32) -> f64 {
        // 应用需用系数
        let calculated_power = rated_power * demand_coefficient;
        // 计算电流
        DefaultElectricalCalculation::calculate_current(calculated_power, voltage, power_factor, phase_count)
    }
    
    /// 计算配电箱的总电流
    /// - phase_loads: 三相负载分布(kW)
    /// - voltage: 电压(V)
    /// - average_power_factor: 平均功率因数
    pub fn calculate_distribution_box_current(&self, phase_loads: [f64; 3], voltage: f64, average_power_factor: f64) -> f64 {
        // 计算总功率
        let total_power = phase_loads[0] + phase_loads[1] + phase_loads[2];
        // 计算三相电流
        DefaultElectricalCalculation::calculate_current(total_power, voltage, average_power_factor, 3)
    }
    
    /// 计算电缆电压损失
    /// - current: 电流(A)
    /// - length: 长度(m)
    /// - wire_size: 导线规格(mm²)
    /// - material: 材料类型
    /// - power_factor: 功率因数
    pub fn calculate_cable_voltage_drop(&self, current: f64, length: f64, wire_size: f64, material: &str, power_factor: f64) -> f64 {
        // 获取电阻率
        let resistivity = *self.resistivity_data.get(material).unwrap_or(&1.72e-8);
        
        // 计算电阻 (每相)
        let resistance = DefaultElectricalCalculation::calculate_resistance(length, wire_size, resistivity);
        
        // 对于低压电缆，电抗约为0.08-0.12 Ω/km，这里取0.1 Ω/km
        let reactance_per_km = 0.1;
        let reactance = (length / 1000.0) * reactance_per_km;
        
        // 计算电压降
        DefaultElectricalCalculation::calculate_voltage_drop(current, resistance, reactance, power_factor)
    }
    
    /// 计算三相不平衡度
    pub fn calculate_three_phase_imbalance(&self, phase_a: f64, phase_b: f64, phase_c: f64) -> f64 {
        DefaultElectricalCalculation::calculate_phase_balance(phase_a, phase_b, phase_c)
    }
    
    /// 选择合适的保护电器
    /// - current: 计算电流(A)
    pub fn select_protective_device(&self, current: f64) -> String {
        // 标准断路器额定电流系列
        let standard_rates = [6.0, 10.0, 16.0, 20.0, 25.0, 32.0, 40.0, 50.0, 63.0, 80.0, 100.0, 125.0];
        
        // 选择大于计算电流的最小标准值
        for &rate in standard_rates.iter() {
            if rate >= current * 1.1 { // 考虑10%余量
                return format!("断路器 {}A", rate);
            }
        }
        
        // 如果超出范围，返回更大的值
        format!("断路器 {}A", current * 1.2)
    }
    
    /// 选择合适的导线规格
    /// - current: 计算电流(A)
    /// - is_three_phase: 是否三相
    pub fn select_wire_size(&self, current: f64, is_three_phase: bool) -> String {
        // 简化的导线载流量表 (铜芯BV线，穿管，环境温度30°C)
        let wire_sizes = [
            (2.5, 16.0),   // 2.5mm²: 16A
            (4.0, 25.0),   // 4mm²: 25A
            (6.0, 32.0),   // 6mm²: 32A
            (10.0, 42.0),  // 10mm²: 42A
            (16.0, 55.0),  // 16mm²: 55A
            (25.0, 70.0),  // 25mm²: 70A
            (35.0, 85.0),  // 35mm²: 85A
            (50.0, 110.0), // 50mm²: 110A
            (70.0, 135.0), // 70mm²: 135A
        ];
        
        // 考虑1.25倍安全系数
        let rated_current = current * 1.25;
        
        // 查找合适的导线规格
        for &(size, max_current) in wire_sizes.iter() {
            if max_current >= rated_current {
                if is_three_phase {
                    return format!("VV-4×{}+1×{}mm²", size, size / 2.0);
                } else {
                    return format!("BV-{}mm²", size);
                }
            }
        }
        
        // 如果超出范围，返回更大的值
        format!("BV-95mm²")
    }
    
    /// 计算电压损失率
    /// - voltage_drop: 电压降(V)
    /// - nominal_voltage: 标称电压(V)
    pub fn calculate_voltage_drop_percent(&self, voltage_drop: f64, nominal_voltage: f64) -> f64 {
        (voltage_drop / nominal_voltage) * 100.0
    }
    
    /// 检查电压损失是否符合规范
    /// - voltage_drop_percent: 电压损失率(%)
    pub fn check_voltage_drop_compliance(&self, voltage_drop_percent: f64) -> bool {
        // 一般规范要求电压损失不超过5%
        voltage_drop_percent <= 5.0
    }
    
    /// 检查三相不平衡是否符合规范
    /// - imbalance_percent: 不平衡度(%)
    pub fn check_phase_imbalance_compliance(&self, imbalance_percent: f64) -> bool {
        // 一般规范要求三相不平衡不超过15%
        imbalance_percent <= 15.0
    }
}