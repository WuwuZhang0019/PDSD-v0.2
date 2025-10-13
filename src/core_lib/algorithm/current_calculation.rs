use crate::core_lib::traits::{ElectricalCalculation, CalculationResult};

/// 电流计算工具
pub struct CurrentCalculator;

impl CurrentCalculator {
    /// 创建新的电流计算器
    pub fn new() -> Self {
        Self
    }
    
    /// 计算有功功率对应的电流
    /// - power_kw: 有功功率(kW)
    /// - voltage_v: 电压(V)
    /// - power_factor: 功率因数
    /// - phase_count: 相数
    pub fn calculate_active_current(power_kw: f64, voltage_v: f64, power_factor: f64, phase_count: u32) -> f64 {
        ElectricalCalculation::calculate_current(power_kw, voltage_v, power_factor, phase_count)
    }
    
    /// 计算视在功率对应的电流
    /// - power_kva: 视在功率(kVA)
    /// - voltage_v: 电压(V)
    /// - phase_count: 相数
    pub fn calculate_apparent_current(power_kva: f64, voltage_v: f64, phase_count: u32) -> f64 {
        // 视在功率电流不需要考虑功率因数
        let sqrt_3 = 3.0_f64.sqrt();
        if phase_count == 1 {
            // 单相
            (power_kva * 1000.0) / voltage_v
        } else {
            // 三相
            (power_kva * 1000.0) / (sqrt_3 * voltage_v)
        }
    }
    
    /// 计算无功功率对应的电流
    /// - power_kvar: 无功功率(kvar)
    /// - voltage_v: 电压(V)
    /// - phase_count: 相数
    pub fn calculate_reactive_current(power_kvar: f64, voltage_v: f64, phase_count: u32) -> f64 {
        // 无功功率电流计算类似视在功率
        let sqrt_3 = 3.0_f64.sqrt();
        if phase_count == 1 {
            // 单相
            (power_kvar * 1000.0) / voltage_v
        } else {
            // 三相
            (power_kvar * 1000.0) / (sqrt_3 * voltage_v)
        }
    }
    
    /// 计算短路电流
    /// - voltage_kv: 电压(kV)
    /// - impedance_ohm: 系统阻抗(Ω)
    pub fn calculate_short_circuit_current(voltage_kv: f64, impedance_ohm: f64) -> f64 {
        // 简化的短路电流计算
        voltage_kv * 1000.0 / impedance_ohm
    }
    
    /// 计算峰值电流
    /// - rms_current: 有效值电流(A)
    pub fn calculate_peak_current(rms_current: f64) -> f64 {
        // 对于正弦波，峰值是有效值的√2倍
        rms_current * 2.0_f64.sqrt()
    }
    
    /// 计算断路器脱口电流
    /// - rated_current: 额定电流(A)
    /// - inverse_time_factor: 反时限倍数
    pub fn calculate_tripping_current(rated_current: f64, inverse_time_factor: f64) -> f64 {
        rated_current * inverse_time_factor
    }
    
    /// 计算中性线电流
    /// - phase_currents: 三相电流
    pub fn calculate_neutral_current(phase_currents: [f64; 3], phase_angles: [f64; 3]) -> f64 {
        // 考虑相位角计算中性线电流
        let mut neutral_current_a = 0.0;
        let mut neutral_current_b = 0.0;
        
        // 将各相电流转换为复数形式并相加
        for i in 0..3 {
            neutral_current_a += phase_currents[i] * phase_angles[i].to_radians().cos();
            neutral_current_b += phase_currents[i] * phase_angles[i].to_radians().sin();
        }
        
        // 计算合成电流的大小
        (neutral_current_a.powi(2) + neutral_current_b.powi(2)).sqrt()
    }
    
    /// 校验电流是否超过额定值
    /// - calculated_current: 计算电流(A)
    /// - rated_current: 额定电流(A)
    /// - tolerance_percent: 允许偏差百分比(%)
    pub fn check_current_rating(calculated_current: f64, rated_current: f64, tolerance_percent: f64) -> bool {
        let tolerance = rated_current * tolerance_percent / 100.0;
        calculated_current <= rated_current + tolerance
    }
    
    /// 格式化电流值显示
    /// - current: 电流值(A)
    pub fn format_current(current: f64) -> String {
        current.format_result(1)
    }
}


/*pub fn single_phase_current_calculation(pe: f32, kx: f32, cos: f32) -> f32 {
    const U: f32 = 0.22;
    let ijs: f32 = pe * kx / U / cos;
    ijs
    //(ijs * 100.0).round() / 100.0
}

pub fn three_phase_current_calculation(pe: f32, kx: f32, cos: f32) -> f32 {
    const U: f32 = 0.38;
    let sqrt3: f32 = 3.0_f32.sqrt();
    let ijs: f32 = pe * kx / U / cos / sqrt3;
    ijs
    //(ijs * 100.0).round() / 100.0
}*/