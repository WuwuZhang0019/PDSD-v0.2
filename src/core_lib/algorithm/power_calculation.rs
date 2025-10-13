use crate::core_lib::traits::{ElectricalCalculation, CalculationResult};

/// 功率计算工具
pub struct PowerCalculator;

impl PowerCalculator {
    /// 创建新的功率计算器
    pub fn new() -> Self {
        Self
    }
    
    /// 计算有功功率
    /// - current_a: 电流(A)
    /// - voltage_v: 电压(V)
    /// - power_factor: 功率因数
    /// - phase_count: 相数
    pub fn calculate_active_power(current_a: f64, voltage_v: f64, power_factor: f64, phase_count: u32) -> f64 {
        let sqrt_3 = 3.0_f64.sqrt();
        if phase_count == 1 {
            // 单相
            (current_a * voltage_v * power_factor) / 1000.0 // 转换为kW
        } else {
            // 三相
            (sqrt_3 * current_a * voltage_v * power_factor) / 1000.0 // 转换为kW
        }
    }
    
    /// 计算视在功率
    /// - current_a: 电流(A)
    /// - voltage_v: 电压(V)
    /// - phase_count: 相数
    pub fn calculate_apparent_power(current_a: f64, voltage_v: f64, phase_count: u32) -> f64 {
        let sqrt_3 = 3.0_f64.sqrt();
        if phase_count == 1 {
            // 单相
            (current_a * voltage_v) / 1000.0 // 转换为kVA
        } else {
            // 三相
            (sqrt_3 * current_a * voltage_v) / 1000.0 // 转换为kVA
        }
    }
    
    /// 计算无功功率
    /// - current_a: 电流(A)
    /// - voltage_v: 电压(V)
    /// - power_factor: 功率因数
    /// - phase_count: 相数
    pub fn calculate_reactive_power(current_a: f64, voltage_v: f64, power_factor: f64, phase_count: u32) -> f64 {
        // 计算视在功率
        let apparent_power = Self::calculate_apparent_power(current_a, voltage_v, phase_count);
        
        // 计算无功功率因数 (sin(θ))
        let theta = power_factor.acos();
        let reactive_factor = theta.sin();
        
        // 计算无功功率
        apparent_power * reactive_factor
    }
    
    /// 计算功率因数
    /// - active_power_kw: 有功功率(kW)
    /// - apparent_power_kva: 视在功率(kVA)
    pub fn calculate_power_factor(active_power_kw: f64, apparent_power_kva: f64) -> f64 {
        if apparent_power_kva == 0.0 {
            return 0.0;
        }
        
        let power_factor = active_power_kw / apparent_power_kva;
        // 确保功率因数在0.0到1.0之间
        power_factor.max(0.0).min(1.0)
    }
    
    /// 计算系统的总功率
    /// - loads: 负载功率数组(kW)
    pub fn calculate_total_power(loads: &[f64]) -> f64 {
        loads.iter().sum()
    }
    
    /// 计算配电箱的三相功率分布
    /// - loads: 所有负载的功率(kW)
    /// - phase_assignments: 各负载的相分配 (0=A相, 1=B相, 2=C相)
    pub fn calculate_phase_power_distribution(loads: &[f64], phase_assignments: &[usize]) -> [f64; 3] {
        let mut phase_powers = [0.0, 0.0, 0.0];
        
        // 将负载功率分配到对应相位
        for (i, &load) in loads.iter().enumerate() {
            if let Some(&phase) = phase_assignments.get(i) {
                if phase < 3 {
                    phase_powers[phase] += load;
                }
            }
        }
        
        phase_powers
    }
    
    /// 计算需要的电容器容量来提高功率因数
    /// - active_power_kw: 有功功率(kW)
    /// - current_power_factor: 当前功率因数
    /// - target_power_factor: 目标功率因数
    pub fn calculate_required_capacity(active_power_kw: f64, current_power_factor: f64, target_power_factor: f64) -> f64 {
        let current_theta = current_power_factor.acos();
        let target_theta = target_power_factor.acos();
        
        // 计算需要的无功功率补偿量(kvar)
        active_power_kw * (current_theta.tan() - target_theta.tan())
    }
    
    /// 格式化功率值显示
    /// - power: 功率值
    /// - unit: 功率单位 ("kW", "kVA", "kvar")
    pub fn format_power(power: f64, unit: &str) -> String {
        format!("{:.{}f} {}", power, 2, unit)
    }
    
    /// 检查功率是否超过额定值
    /// - calculated_power: 计算功率(kW)
    /// - rated_power: 额定功率(kW)
    pub fn check_power_rating(calculated_power: f64, rated_power: f64) -> bool {
        calculated_power <= rated_power
    }
    
    /// 计算效率
    /// - output_power: 输出功率(kW)
    /// - input_power: 输入功率(kW)
    pub fn calculate_efficiency(output_power: f64, input_power: f64) -> f64 {
        if input_power == 0.0 {
            return 0.0;
        }
        
        (output_power / input_power) * 100.0
    }
}