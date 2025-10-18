use serde::{Deserialize, Serialize};

/// 回路类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitType {
    /// 单相回路
    SinglePhase,
    /// 三相回路
    ThreePhase,
}

impl CircuitType {
    /// 将回路类型转换为字符串
    pub fn to_str(&self) -> &'static str {
        match self {
            CircuitType::SinglePhase => "单相",
            CircuitType::ThreePhase => "三相",
        }
    }
}

/// 回路用途枚举
/// 需根据实际用途进行定义
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitPurpose {
    /// 照明回路
    Lighting,
    /// 动力回路
    Power,
    /// 空调回路
    HVAC,
    /// 特殊用途回路
    Special,
    /// 自定义回路
    Custom(String),
}

impl CircuitPurpose {
    /// 将回路用途转换为字符串
    pub fn to_str(&self) -> String {
        match self {
            CircuitPurpose::Lighting => "照明".to_string(),
            CircuitPurpose::Power => "动力".to_string(),
            CircuitPurpose::HVAC => "空调".to_string(),
            CircuitPurpose::Special => "特殊".to_string(),
            CircuitPurpose::Custom(s) => s.clone(),
        }
    }
}

/// 配电回路节点属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitNodeProperties {
    /// 回路类型
    pub circuit_type: CircuitType,
    /// 回路功率 (kW)
    pub power: f64,
    /// 电压值 (V)
    pub voltage: f64,
    /// 功率因数
    pub power_factor: f64,
    /// 需用系数
    pub demand_factor: f64,
    /// 计算电流 (A)
    pub current: f64,
    /// 1.1倍计算电流
    pub current_1_1x: f64,
    /// 1.25倍计算电流
    pub current_1_25x: f64,
    /// 回路用途
    pub purpose: CircuitPurpose,
    /// 元器件类型
    pub component_type: String,
    /// 元器件电流整定值
    pub component_current: f64,
    /// 线缆规格
    pub cable_spec: String,
    /// 相序标识
    pub phase: Option<char>,
    /// 回路编号
    pub circuit_number: u32,
    /// 回路名称
    pub circuit_name: String,
}

impl Default for CircuitNodeProperties {
    fn default() -> Self {
        Self {
            circuit_type: CircuitType::SinglePhase,
            power: 1.0,
            voltage: 220.0,
            power_factor: 0.85,
            demand_factor: 1.0,
            current: 0.0,
            current_1_1x: 0.0,
            current_1_25x: 0.0,
            purpose: CircuitPurpose::Power,
            component_type: "微型断路器".to_string(),
            component_current: 16.0,
            cable_spec: "BV-2.5mm²".to_string(),
            phase: Some('L'),
            circuit_number: 1,
            circuit_name: "新建回路".to_string(),
        }
    }
}

impl CircuitNodeProperties {
    /// 计算工作电流
    pub fn calculate_current(&mut self) {
        match self.circuit_type {
            CircuitType::SinglePhase => {
                // 单相电流计算：I = P * Kx / (U * Cosφ)
                self.current = self.power * 1000.0 * self.demand_factor 
                    / (self.voltage * self.power_factor);
            },
            CircuitType::ThreePhase => {
                // 三相电流计算：I = P * Kx / (√3 * U * Cosφ)
                self.current = self.power * 1000.0 * self.demand_factor 
                    / (1.732 * self.voltage * self.power_factor);
            },
        }
        
        // 计算衍生电流值
        self.current_1_1x = self.current * 1.1;
        self.current_1_25x = self.current * 1.25;
    }
    
    /// 根据回路用途选择元器件类型
    pub fn select_component_type(&mut self) {
        self.component_type = match &self.purpose {
            CircuitPurpose::Lighting => "微型断路器(照明)".to_string(),
            CircuitPurpose::Power => "微型断路器(动力)".to_string(),
            CircuitPurpose::HVAC => "剩余电流保护器".to_string(),
            CircuitPurpose::Special => "专用保护电器".to_string(),
            CircuitPurpose::Custom(s) => format!("自定义({})", s),
        };
    }
    
    /// 选择元器件电流整定值
    pub fn select_component_current(&mut self) {
        self.component_current = Self::select_component_current_value(self.current_1_1x);
    }
    
    /// 选择线缆规格
    pub fn select_cable_spec(&mut self) {
        self.cable_spec = Self::select_cable_spec_value(self.current);
    }
    
    /// 根据计算电流选择元器件电流整定值
    pub fn select_component_current_value(calculated_current: f64) -> f64 {
        let standard_values = [1.0, 2.0, 4.0, 6.0, 10.0, 16.0, 20.0, 25.0, 32.0, 40.0, 50.0, 63.0, 80.0, 100.0, 125.0];
        
        for &value in &standard_values {
            if value >= calculated_current {
                return value;
            }
        }
        
        *standard_values.last().unwrap() // 返回最大的标准值
    }
    
    /// 根据计算电流选择线缆规格
    pub fn select_cable_spec_value(current: f64) -> String {
        match current {
            x if x <= 6.0 => "BV-2.5mm²".to_string(),
            x if x <= 10.0 => "BV-4mm²".to_string(),
            x if x <= 16.0 => "BV-6mm²".to_string(),
            x if x <= 25.0 => "BV-10mm²".to_string(),
            x if x <= 32.0 => "BV-16mm²".to_string(),
            x if x <= 40.0 => "BV-25mm²".to_string(),
            x if x <= 63.0 => "BV-35mm²".to_string(),
            _ => "BV-50mm²".to_string(),
        }
    }
    
    /// 执行所有自动计算和选型
    pub fn perform_all_calculations(&mut self) {
        self.calculate_current();
        self.select_component_type();
        self.select_component_current();
        self.select_cable_spec();
    }
}