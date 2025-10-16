/// 配电回路计算集成模块

use crate::core_lib::algorithm::current_calculation::CurrentCalculator;
use crate::editor::business::circuit_parameters::{CircuitParameters, CircuitResult, VoltageType};

/// 回路计算错误类型
#[derive(Debug, PartialEq)]
pub enum CircuitCalculationError {
    /// 参数无效
    InvalidParameters(Vec<String>),
    /// 计算过程中的数值错误
    CalculationError(String),
}

impl std::fmt::Display for CircuitCalculationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CircuitCalculationError::InvalidParameters(errors) => {
                write!(f, "参数无效: {}", errors.join(", "))
            },
            CircuitCalculationError::CalculationError(message) => {
                write!(f, "计算错误: {}", message)
            },
        }
    }
}

impl std::error::Error for CircuitCalculationError {}

/// 回路计算器
pub struct CircuitCalculator;

impl CircuitCalculator {
    /// 创建新的回路计算器实例
    pub fn new() -> Self {
        Self
    }
    
    /// 计算回路电流
    /// - parameters: 回路参数
    /// - 返回值: 成功时返回计算结果，失败时返回错误
    pub fn calculate_circuit_current(
        parameters: &CircuitParameters
    ) -> Result<CircuitResult, CircuitCalculationError> {
        // 首先验证参数
        if !parameters.is_valid() {
            return Err(CircuitCalculationError::InvalidParameters(
                parameters.validation_errors()
            ));
        }
        
        // 获取电压类型
        let voltage_type = parameters.voltage_type;
        let voltage = voltage_type.voltage_value();
        
        // 创建电流计算器
        let calculator = CurrentCalculator::new();
        
        // 根据电压类型选择计算方法
        let ijs = match voltage_type {
            VoltageType::SinglePhase => {
                calculator.single_phase_current_calculation(
                    parameters.pe,
                    parameters.kx,
                    parameters.cos
                )
            },
            VoltageType::ThreePhase => {
                calculator.three_phase_current_calculation(
                    parameters.pe,
                    parameters.kx,
                    parameters.cos
                )
            },
        };
        
        // 检查计算结果是否有效（避免NaN或无穷大）
        if !ijs.is_finite() {
            return Err(CircuitCalculationError::CalculationError(
                "计算结果无效，请检查输入参数".to_string()
            ));
        }
        
        // 创建并返回计算结果
        Ok(CircuitResult::new(ijs, voltage))
    }
    
    /// 批量计算多个回路
    /// - circuits: 回路参数列表
    /// - 返回值: 计算结果列表，与输入顺序一致
    pub fn calculate_multiple_circuits(
        circuits: &[CircuitParameters]
    ) -> Vec<Result<CircuitResult, CircuitCalculationError>> {
        circuits.iter()
            .map(|params| Self::calculate_circuit_current(params))
            .collect()
    }
    
    /// 估算回路的标准电流（用于快速预览）
    /// 不进行完整验证，仅提供快速估算
    pub fn estimate_circuit_current(
        pe: f32,
        voltage_type: VoltageType
    ) -> f32 {
        // 使用默认的需要系数和功率因数进行快速估算
        const DEFAULT_KX: f32 = 0.8;
        const DEFAULT_COS: f32 = 0.85;
        
        let calculator = CurrentCalculator::new();
        
        match voltage_type {
            VoltageType::SinglePhase => {
                calculator.single_phase_current_calculation(pe, DEFAULT_KX, DEFAULT_COS)
            },
            VoltageType::ThreePhase => {
                calculator.three_phase_current_calculation(pe, DEFAULT_KX, DEFAULT_COS)
            },
        }
    }
    
    /// 验证计算结果的合理性
    /// 用于检查计算结果是否在合理范围内
    pub fn validate_calculation_result(
        result: &CircuitResult
    ) -> bool {
        // 电流应该在合理范围内（0-1000A）
        result.ijs > 0.0 && result.ijs < 1000.0
    }
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_single_phase_current() {
        // 测试单相信路计算
        let params = CircuitParameters::new(
            "测试回路".to_string(),
            1.0,  // 1kW
            0.8,  // 需要系数
            0.85, // 功率因数
            VoltageType::SinglePhase
        );
        
        let result = CircuitCalculator::calculate_circuit_current(&params).unwrap();
        
        // 验证计算结果（1kW * 0.8 / 0.22kV / 0.85 ≈ 5.24A）
        assert!(result.ijs > 5.2 && result.ijs < 5.3);
        assert_eq!(result.voltage, 0.22);
    }
    
    #[test]
    fn test_calculate_three_phase_current() {
        // 测试三相信路计算
        let params = CircuitParameters::new(
            "测试回路".to_string(),
            10.0, // 10kW
            0.8,  // 需要系数
            0.85, // 功率因数
            VoltageType::ThreePhase
        );
        
        let result = CircuitCalculator::calculate_circuit_current(&params).unwrap();
        
        // 验证计算结果（10kW * 0.8 / 0.38kV / 0.85 / √3 ≈ 17.86A）
        assert!(result.ijs > 17.8 && result.ijs < 18.0);
        assert_eq!(result.voltage, 0.38);
    }
    
    #[test]
    fn test_invalid_parameters() {
        // 测试无效参数
        let params = CircuitParameters::new(
            "无效回路".to_string(),
            -1.0, // 负功率（无效）
            0.8,
            0.85,
            VoltageType::SinglePhase
        );
        
        let result = CircuitCalculator::calculate_circuit_current(&params);
        
        assert!(matches!(result, Err(CircuitCalculationError::InvalidParameters(_))));
    }
    
    #[test]
    fn test_estimate_current() {
        // 测试快速估算
        let estimate_single = CircuitCalculator::estimate_circuit_current(
            1.0, VoltageType::SinglePhase
        );
        
        let estimate_three = CircuitCalculator::estimate_circuit_current(
            10.0, VoltageType::ThreePhase
        );
        
        assert!(estimate_single > 0.0);
        assert!(estimate_three > 0.0);
    }
}