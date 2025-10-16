use thiserror::Error;

/// 核心库错误类型
/// 
/// 定义与配电系统图核心计算、数据处理和验证相关的所有错误
#[derive(Debug, Error)]
pub enum CoreError {
    /// 计算错误，用于所有与电气计算相关的错误
    #[error("计算错误: {description}")]
    Calculation {
        /// 错误描述
        description: String,
        /// 可选的计算参数信息
        parameter: Option<String>,
    },
    
    /// 数据验证错误
    #[error("验证错误: {field} - {message}")]
    Validation {
        /// 验证失败的字段或参数名称
        field: String,
        /// 详细错误消息
        message: String,
    },
    
    /// 配置错误
    #[error("配置错误: {0}")]
    Configuration(String),
    
    /// 数据类型错误
    #[error("数据类型错误: {expected}，实际: {actual}")]
    DataType {
        /// 期望的数据类型
        expected: String,
        /// 实际的数据类型
        actual: String,
    },
    
    /// 数据缺失错误
    #[error("数据缺失错误: {field} - {reason}")]
    MissingData {
        /// 缺失的字段
        field: String,
        /// 缺失原因
        reason: String,
    },
    
    /// 算法错误
    #[error("算法错误: {algorithm_name} - {description}")]
    Algorithm {
        /// 算法名称
        algorithm_name: String,
        /// 错误描述
        description: String,
    },
    
    /// 数学错误（如除零、溢出等）
    #[error("数学错误: {operation} - {error_type}")]
    Mathematical {
        /// 操作名称
        operation: String,
        /// 错误类型
        error_type: String,
    },
    
    /// 逻辑错误
    #[error("逻辑错误: {0}")]
    Logic(String),
}

/// 核心库错误的辅助方法
impl CoreError {
    /// 创建计算错误
    pub fn calculation(description: impl Into<String>, parameter: Option<impl Into<String>>) -> Self {
        CoreError::Calculation {
            description: description.into(),
            parameter: parameter.map(|p| p.into()),
        }
    }
    
    /// 创建验证错误
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        CoreError::Validation {
            field: field.into(),
            message: message.into(),
        }
    }
    
    /// 创建配置错误
    pub fn configuration(message: impl Into<String>) -> Self {
        CoreError::Configuration(message.into())
    }
    
    /// 创建数据类型错误
    pub fn data_type(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        CoreError::DataType {
            expected: expected.into(),
            actual: actual.into(),
        }
    }
    
    /// 创建数据缺失错误
    pub fn missing_data(field: impl Into<String>, reason: impl Into<String>) -> Self {
        CoreError::MissingData {
            field: field.into(),
            reason: reason.into(),
        }
    }
    
    /// 创建算法错误
    pub fn algorithm(algorithm_name: impl Into<String>, description: impl Into<String>) -> Self {
        CoreError::Algorithm {
            algorithm_name: algorithm_name.into(),
            description: description.into(),
        }
    }
    
    /// 创建数学错误
    pub fn mathematical(operation: impl Into<String>, error_type: impl Into<String>) -> Self {
        CoreError::Mathematical {
            operation: operation.into(),
            error_type: error_type.into(),
        }
    }
    
    /// 创建逻辑错误
    pub fn logic(message: impl Into<String>) -> Self {
        CoreError::Logic(message.into())
    }
}

/// 核心库的结果类型别名
pub type Result<T> = std::result::Result<T, CoreError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculation_error() {
        let error = CoreError::calculation("电流计算溢出", Some("负载功率=1000kW"));
        assert!(error.to_string().contains("计算错误"));
        assert!(error.to_string().contains("电流计算溢出"));
        
        // 测试没有参数的情况
        let error_no_param = CoreError::calculation("电压计算失败", None::<String>);
        assert!(error_no_param.to_string().contains("计算错误"));
    }

    #[test]
    fn test_validation_error() {
        let error = CoreError::validation("电缆截面积", "必须大于0.5mm²");
        assert_eq!(error.to_string(), "验证错误: 电缆截面积 - 必须大于0.5mm²".to_string());
    }

    #[test]
    fn test_data_type_error() {
        let error = CoreError::data_type("浮点数", "字符串");
        assert_eq!(error.to_string(), "数据类型错误: 浮点数，实际: 字符串".to_string());
    }

    #[test]
    fn test_missing_data_error() {
        let error = CoreError::missing_data("短路电流", "计算断路器容量需要短路电流值");
        assert_eq!(error.to_string(), "数据缺失错误: 短路电流 - 计算断路器容量需要短路电流值".to_string());
    }

    #[test]
    fn test_algorithm_error() {
        let error = CoreError::algorithm("三相平衡算法", "节点数量超过算法限制");
        assert_eq!(error.to_string(), "算法错误: 三相平衡算法 - 节点数量超过算法限制".to_string());
    }

    #[test]
    fn test_mathematical_error() {
        let error = CoreError::mathematical("除法", "除零错误");
        assert_eq!(error.to_string(), "数学错误: 除法 - 除零错误".to_string());
    }
}