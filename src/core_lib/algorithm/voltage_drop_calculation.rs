use crate::core_lib::traits::{ElectricalCalculation, CalculationResult};

/// 电压损失计算工具
pub struct VoltageDropCalculator;

impl VoltageDropCalculator {
    /// 创建新的电压损失计算器
    pub fn new() -> Self {
        Self
    }
    
    /// 计算电缆的电压损失
    /// - current_a: 电流(A)
    /// - length_m: 电缆长度(m)
    /// - resistance_per_km_ohm: 每公里电阻(Ω/km)
    /// - reactance_per_km_ohm: 每公里电抗(Ω/km)
    /// - power_factor: 功率因数
    pub fn calculate_cable_voltage_drop(current_a: f64, length_m: f64, resistance_per_km_ohm: f64, reactance_per_km_ohm: f64, power_factor: f64) -> f64 {
        // 计算每相电阻和电抗
        let resistance = (resistance_per_km_ohm * length_m) / 1000.0;
        let reactance = (reactance_per_km_ohm * length_m) / 1000.0;
        
        // 使用默认电气计算实现电压损失计算
        ElectricalCalculation::calculate_voltage_drop(current_a, resistance, reactance, power_factor)
    }
    
    /// 计算三相系统的电压损失
    /// - current_a: 电流(A)
    /// - length_m: 电缆长度(m)
    /// - resistance_per_km_ohm: 每公里电阻(Ω/km)
    /// - reactance_per_km_ohm: 每公里电抗(Ω/km)
    /// - power_factor: 功率因数
    /// - line_voltage_v: 线电压(V)
    pub fn calculate_three_phase_voltage_drop(current_a: f64, length_m: f64, resistance_per_km_ohm: f64, reactance_per_km_ohm: f64, power_factor: f64, line_voltage_v: f64) -> f64 {
        // 计算相电压
        let phase_voltage_v = line_voltage_v / 3.0_f64.sqrt();
        
        // 计算每相电阻和电抗
        let resistance = (resistance_per_km_ohm * length_m) / 1000.0;
        let reactance = (reactance_per_km_ohm * length_m) / 1000.0;
        
        // 计算电压损失
        let voltage_drop = Self::calculate_voltage_drop_per_phase(current_a, resistance, reactance, power_factor);
        
        // 返回线电压损失 (对于三相系统，线电压损失等于相电压损失)
        voltage_drop
    }
    
    /// 计算单相系统的电压损失
    /// - current_a: 电流(A)
    /// - length_m: 电缆长度(m)
    /// - resistance_per_km_ohm: 每公里电阻(Ω/km)
    /// - reactance_per_km_ohm: 每公里电抗(Ω/km)
    /// - power_factor: 功率因数
    pub fn calculate_single_phase_voltage_drop(current_a: f64, length_m: f64, resistance_per_km_ohm: f64, reactance_per_km_ohm: f64, power_factor: f64) -> f64 {
        // 计算每相电阻和电抗（对于单相，考虑往返）
        let resistance = 2.0 * (resistance_per_km_ohm * length_m) / 1000.0;
        let reactance = 2.0 * (reactance_per_km_ohm * length_m) / 1000.0;
        
        // 计算电压损失
        Self::calculate_voltage_drop_per_phase(current_a, resistance, reactance, power_factor)
    }
    
    /// 计算每相电压损失
    /// - current_a: 电流(A)
    /// - resistance_ohm: 电阻(Ω)
    /// - reactance_ohm: 电抗(Ω)
    /// - power_factor: 功率因数
    fn calculate_voltage_drop_per_phase(current_a: f64, resistance_ohm: f64, reactance_ohm: f64, power_factor: f64) -> f64 {
        // 计算电阻分量和电抗分量
        let resistance_drop = current_a * resistance_ohm * power_factor;
        
        // 计算sin(θ)
        let theta = power_factor.acos();
        let reactive_factor = theta.sin();
        
        let reactance_drop = current_a * reactance_ohm * reactive_factor;
        
        // 计算总电压损失
        resistance_drop + reactance_drop
    }
    
    /// 计算电压损失百分比
    /// - voltage_drop_v: 电压损失(V)
    /// - nominal_voltage_v: 标称电压(V)
    pub fn calculate_voltage_drop_percentage(voltage_drop_v: f64, nominal_voltage_v: f64) -> f64 {
        if nominal_voltage_v == 0.0 {
            return 0.0;
        }
        
        (voltage_drop_v / nominal_voltage_v) * 100.0
    }
    
    /// 获取电缆的电阻和电抗
    /// - wire_size_mm2: 导线截面积(mm²)
    /// - is_three_phase: 是否三相
    pub fn get_cable_impedance(wire_size_mm2: f64, is_three_phase: bool) -> (f64, f64) {
        // 简化的铜芯电缆阻抗表 (Ω/km at 20°C)
        let (resistance_per_km, reactance_per_km) = match wire_size_mm2 {
            2.5 => (7.41, 0.30),
            4.0 => (4.64, 0.28),
            6.0 => (3.08, 0.27),
            10.0 => (1.83, 0.25),
            16.0 => (1.15, 0.23),
            25.0 => (0.727, 0.22),
            35.0 => (0.524, 0.21),
            50.0 => (0.387, 0.20),
            70.0 => (0.273, 0.19),
            95.0 => (0.206, 0.19),
            120.0 => (0.164, 0.18),
            150.0 => (0.132, 0.18),
            185.0 => (0.108, 0.17),
            240.0 => (0.0839, 0.17),
            300.0 => (0.0680, 0.16),
            _ => (0.0172 * 1000.0 / wire_size_mm2, 0.20), // 估算值
        };
        
        (resistance_per_km, reactance_per_km)
    }
    
    /// 选择合适的电缆截面积以满足电压损失要求
    /// - current_a: 电流(A)
    /// - length_m: 电缆长度(m)
    /// - nominal_voltage_v: 标称电压(V)
    /// - max_voltage_drop_percent: 最大允许电压损失百分比(%)
    /// - power_factor: 功率因数
    /// - is_three_phase: 是否三相
    pub fn select_wire_size_for_voltage_drop(current_a: f64, length_m: f64, nominal_voltage_v: f64, max_voltage_drop_percent: f64, power_factor: f64, is_three_phase: bool) -> f64 {
        // 标准导线截面积系列
        let wire_sizes = [2.5, 4.0, 6.0, 10.0, 16.0, 25.0, 35.0, 50.0, 70.0, 95.0, 120.0, 150.0, 185.0, 240.0];
        
        // 对于三相系统，使用线电压
        let voltage_to_use = if is_three_phase {
            nominal_voltage_v
        } else {
            nominal_voltage_v
        };
        
        // 查找满足电压损失要求的最小导线截面积
        for &size in wire_sizes.iter() {
            let (resistance_per_km, reactance_per_km) = Self::get_cable_impedance(size, is_three_phase);
            
            let voltage_drop = if is_three_phase {
                Self::calculate_three_phase_voltage_drop(current_a, length_m, resistance_per_km, reactance_per_km, power_factor, voltage_to_use)
            } else {
                Self::calculate_single_phase_voltage_drop(current_a, length_m, resistance_per_km, reactance_per_km, power_factor)
            };
            
            let voltage_drop_percent = Self::calculate_voltage_drop_percentage(voltage_drop, voltage_to_use);
            
            if voltage_drop_percent <= max_voltage_drop_percent {
                return size;
            }
        }
        
        // 如果所有标准截面积都不满足要求，返回最大的标准截面积
        wire_sizes[wire_sizes.len() - 1]
    }
    
    /// 格式化电压损失显示
    /// - voltage_drop_v: 电压损失(V)
    /// - percentage: 电压损失百分比(%)
    pub fn format_voltage_drop(voltage_drop_v: f64, percentage: f64) -> String {
        format!("{:.2}V ({:.2}%)", voltage_drop_v, percentage)
    }
    
    /// 检查电压损失是否符合规范
    /// - voltage_drop_percent: 电压损失百分比(%)
    pub fn check_voltage_drop_compliance(voltage_drop_percent: f64) -> (bool, String) {
        if voltage_drop_percent <= 3.0 {
            (true, "符合规范 (≤3%)".to_string())
        } else if voltage_drop_percent <= 5.0 {
            (true, "基本符合规范 (≤5%)".to_string())
        } else {
            (false, "不符合规范 (>5%)".to_string())
        }
    }
}