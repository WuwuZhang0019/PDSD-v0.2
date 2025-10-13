use crate::core_lib::traits::ElectricalCalculation;

/// 电阻计算工具
pub struct ResistanceCalculator;

impl ResistanceCalculator {
    /// 创建新的电阻计算器
    pub fn new() -> Self {
        Self
    }
    
    /// 计算导线电阻
    /// - length_m: 导线长度(m)
    /// - cross_section_area_mm2: 截面积(mm²)
    /// - resistivity_ohm_m: 电阻率(Ω·m)
    pub fn calculate_wire_resistance(length_m: f64, cross_section_area_mm2: f64, resistivity_ohm_m: f64) -> f64 {
        // 转换截面积到平方米 (1 mm² = 1e-6 m²)
        let cross_section_area_m2 = cross_section_area_mm2 * 1e-6;
        
        // 使用公式 R = ρ * L / A
        ElectricalCalculation::calculate_resistance(length_m, cross_section_area_mm2, resistivity_ohm_m)
    }
    
    /// 计算温度修正后的电阻
    /// - resistance_20c: 20°C时的电阻(Ω)
    /// - temperature_c: 当前温度(°C)
    /// - temperature_coefficient: 温度系数(1/°C)
    pub fn calculate_temperature_corrected_resistance(resistance_20c: f64, temperature_c: f64, temperature_coefficient: f64) -> f64 {
        resistance_20c * (1.0 + temperature_coefficient * (temperature_c - 20.0))
    }
    
    /// 计算串联电阻
    /// - resistances: 电阻值数组(Ω)
    pub fn calculate_series_resistance(resistances: &[f64]) -> f64 {
        resistances.iter().sum()
    }
    
    /// 计算并联电阻
    /// - resistances: 电阻值数组(Ω)
    pub fn calculate_parallel_resistance(resistances: &[f64]) -> f64 {
        if resistances.is_empty() {
            return 0.0;
        }
        
        let reciprocal_sum: f64 = resistances.iter()
            .map(|&r| if r > 0.0 { 1.0 / r } else { 0.0 })
            .sum();
        
        if reciprocal_sum > 0.0 {
            1.0 / reciprocal_sum
        } else {
            0.0
        }
    }
    
    /// 计算导体的电阻率
    /// - resistance: 电阻(Ω)
    /// - length_m: 长度(m)
    /// - cross_section_area_mm2: 截面积(mm²)
    pub fn calculate_resistivity(resistance: f64, length_m: f64, cross_section_area_mm2: f64) -> f64 {
        if length_m == 0.0 || cross_section_area_mm2 == 0.0 {
            return 0.0;
        }
        
        // ρ = R * A / L
        resistance * cross_section_area_mm2 / length_m
    }
    
    /// 计算导体的电导
    /// - resistance: 电阻(Ω)
    pub fn calculate_conductance(resistance: f64) -> f64 {
        if resistance == 0.0 {
            return f64::INFINITY;
        }
        1.0 / resistance
    }
    
    /// 计算导体的电导率
    /// - conductance: 电导(S)
    /// - length_m: 长度(m)
    /// - cross_section_area_mm2: 截面积(mm²)
    pub fn calculate_conductivity(conductance: f64, length_m: f64, cross_section_area_mm2: f64) -> f64 {
        if length_m == 0.0 || cross_section_area_mm2 == 0.0 {
            return 0.0;
        }
        
        // σ = G * L / A
        conductance * length_m / cross_section_area_mm2
    }
    
    /// 获取常用材料的电阻率 (Ω·mm²/m) at 20°C
    pub fn get_standard_resistivity(material: &str) -> f64 {
        match material.to_lowercase().as_str() {
            "copper" | "铜" => 0.0172,
            "aluminum" | "铝" => 0.0282,
            "steel" | "钢" => 0.12,
            "gold" | "金" => 0.024,
            "silver" | "银" => 0.016,
            "iron" | "铁" => 0.097,
            _ => 0.0172, // 默认铜的电阻率
        }
    }
    
    /// 获取常用材料的温度系数 (1/°C)
    pub fn get_temperature_coefficient(material: &str) -> f64 {
        match material.to_lowercase().as_str() {
            "copper" | "铜" => 0.00393,
            "aluminum" | "铝" => 0.00403,
            "steel" | "钢" => 0.0045,
            "gold" | "金" => 0.0034,
            "silver" | "银" => 0.0038,
            "iron" | "铁" => 0.0050,
            _ => 0.00393, // 默认铜的温度系数
        }
    }
    
    /// 计算导体的截面积
    /// - resistance: 电阻(Ω)
    /// - length_m: 长度(m)
    /// - resistivity_ohm_mm2_per_m: 电阻率(Ω·mm²/m)
    pub fn calculate_cross_section_area(resistance: f64, length_m: f64, resistivity_ohm_mm2_per_m: f64) -> f64 {
        if resistance == 0.0 {
            return f64::INFINITY;
        }
        
        // A = ρ * L / R
        (resistivity_ohm_mm2_per_m * length_m) / resistance
    }
    
    /// 计算导体的最大允许长度
    /// - max_resistance: 最大允许电阻(Ω)
    /// - cross_section_area_mm2: 截面积(mm²)
    /// - resistivity_ohm_mm2_per_m: 电阻率(Ω·mm²/m)
    pub fn calculate_max_allowable_length(max_resistance: f64, cross_section_area_mm2: f64, resistivity_ohm_mm2_per_m: f64) -> f64 {
        if resistivity_ohm_mm2_per_m == 0.0 {
            return f64::INFINITY;
        }
        
        // L = R * A / ρ
        (max_resistance * cross_section_area_mm2) / resistivity_ohm_mm2_per_m
    }
}