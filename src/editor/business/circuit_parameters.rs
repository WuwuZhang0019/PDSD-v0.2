/// 配电回路参数与结果数据结构定义

use std::time::Instant;
use serde::{Deserialize, Serialize};

/// 电压类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoltageType {
    /// 单相电压 (220V)
    SinglePhase,
    /// 三相电压 (380V)
    ThreePhase,
}

impl Default for VoltageType {
    fn default() -> Self {
        VoltageType::SinglePhase
    }
}

impl VoltageType {
    /// 获取电压值（单位：kV）
    pub fn voltage_value(&self) -> f32 {
        match self {
            VoltageType::SinglePhase => 0.22,
            VoltageType::ThreePhase => 0.38,
        }
    }
    
    /// 获取电压类型的中文描述
    pub fn display_name(&self) -> &'static str {
        match self {
            VoltageType::SinglePhase => "单相",
            VoltageType::ThreePhase => "三相",
        }
    }
    
    /// 获取所有可用的电压类型
    pub fn all_types() -> Vec<VoltageType> {
        vec![VoltageType::SinglePhase, VoltageType::ThreePhase]
    }
}

/// 配电回路参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitParameters {
    /// 回路名称
    pub name: String,
    /// 额定功率 (kW)
    pub pe: f32,
    /// 需要系数
    pub kx: f32,
    /// 功率因数
    pub cos: f32,
    /// 电压类型
    pub voltage_type: VoltageType,
}

impl Default for CircuitParameters {
    fn default() -> Self {
        Self {
            name: "新回路".to_string(),
            pe: 1.0,
            kx: 0.8,
            cos: 0.85,
            voltage_type: VoltageType::SinglePhase,
        }
    }
}

impl CircuitParameters {
    /// 创建新的回路参数实例
    pub fn new(name: String, pe: f32, kx: f32, cos: f32, voltage_type: VoltageType) -> Self {
        Self {
            name,
            pe,
            kx,
            cos,
            voltage_type,
        }
    }
    
    /// 验证参数是否有效
    pub fn is_valid(&self) -> bool {
        self.pe > 0.0 && 
        self.kx > 0.0 && self.kx <= 1.0 && 
        self.cos > 0.0 && self.cos <= 1.0
    }
    
    /// 获取参数验证错误信息
    pub fn validation_errors(&self) -> Vec<String> {
        let mut errors = Vec::new();
        
        if self.pe <= 0.0 {
            errors.push("额定功率必须大于0".to_string());
        }
        
        if self.kx <= 0.0 || self.kx > 1.0 {
            errors.push("需要系数必须在0-1之间".to_string());
        }
        
        if self.cos <= 0.0 || self.cos > 1.0 {
            errors.push("功率因数必须在0-1之间".to_string());
        }
        
        errors
    }
}

/// 配电回路计算结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitResult {
    /// 计算电流 (A)
    pub ijs: f32,
    /// 电压值 (kV)
    pub voltage: f32,
    /// 计算时间戳
    pub calculation_time: Instant,
}

impl CircuitResult {
    /// 创建新的计算结果实例
    pub fn new(ijs: f32, voltage: f32) -> Self {
        Self {
            ijs,
            voltage,
            calculation_time: Instant::now(),
        }
    }
    
    /// 格式化计算电流显示
    pub fn formatted_ijs(&self) -> String {
        format!("{:.2}A", self.ijs)
    }
    
    /// 格式化电压显示
    pub fn formatted_voltage(&self) -> String {
        format!("{:.2}kV", self.voltage)
    }
}

/// 回路完整数据（包含参数和计算结果）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitData {
    /// 回路标识
    pub id: String,
    /// 回路参数
    pub parameters: CircuitParameters,
    /// 计算结果（可选）
    pub result: Option<CircuitResult>,
}

impl Default for CircuitData {
    fn default() -> Self {
        Self {
            id: format!("circuit_{:x}", rand::random::<u32>()),
            parameters: CircuitParameters::default(),
            result: None,
        }
    }
}

impl CircuitData {
    /// 创建新的回路数据实例
    pub fn new(id: String, parameters: CircuitParameters) -> Self {
        Self {
            id,
            parameters,
            result: None,
        }
    }
    
    /// 更新参数
    pub fn update_parameters(&mut self, parameters: CircuitParameters) {
        self.parameters = parameters;
        // 参数更新后，清除计算结果，需要重新计算
        self.result = None;
    }
    
    /// 设置计算结果
    pub fn set_result(&mut self, result: CircuitResult) {
        self.result = Some(result);
    }
    
    /// 检查是否需要重新计算
    pub fn needs_recalculation(&self) -> bool {
        self.result.is_none() || !self.parameters.is_valid()
    }
}