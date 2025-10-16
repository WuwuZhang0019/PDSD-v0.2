//! 回路管理模块
//! 
//! 本模块提供对配电回路的管理功能，包括添加、删除、更新回路，以及自动编号和参数验证等功能。

use crate::editor::business::{CircuitInfo, DistributionBoxError};

/// 回路管理器
/// 
/// 提供对回路集合的管理功能，实现回路的添加、删除、更新、自动编号等操作
pub struct CircuitManager;

impl CircuitManager {
    /// 添加回路到集合
    /// 
    /// # 参数
    /// * `circuits` - 回路集合的可变引用
    /// * `circuit` - 要添加的回路信息
    /// 
    /// # 返回值
    /// * `Ok(())` - 添加成功
    /// * `Err(DistributionBoxError)` - 添加失败，包含错误信息
    pub fn add_circuit(circuits: &mut Vec<CircuitInfo>, circuit: CircuitInfo) -> Result<(), DistributionBoxError> {
        // 验证回路信息
        circuit.validate()?;
        
        // 检查回路数量限制
        if circuits.len() >= 99 {
            return Err(DistributionBoxError::CircuitCountExceeded(
                circuits.len() as u32 + 1
            ));
        }
        
        // 检查回路ID是否已存在
        if circuits.iter().any(|c| c.circuit_id == circuit.circuit_id) {
            return Err(DistributionBoxError::InvalidParameter(
                format!("回路ID '{}' 已存在", circuit.circuit_id)
            ));
        }
        
        // 添加回路
        circuits.push(circuit);
        Ok(())
    }
    
    /// 从集合中删除回路
    /// 
    /// # 参数
    /// * `circuits` - 回路集合的可变引用
    /// * `circuit_id` - 要删除的回路ID
    /// 
    /// # 返回值
    /// * `Ok(())` - 删除成功
    /// * `Err(DistributionBoxError)` - 删除失败，包含错误信息
    pub fn remove_circuit(circuits: &mut Vec<CircuitInfo>, circuit_id: &str) -> Result<(), DistributionBoxError> {
        let initial_len = circuits.len();
        
        // 过滤掉指定ID的回路
        circuits.retain(|c| c.circuit_id != circuit_id);
        
        // 检查是否找到并删除了回路
        if circuits.len() == initial_len {
            return Err(DistributionBoxError::CircuitNotFound(
                circuit_id.to_string()
            ));
        }
        
        Ok(())
    }
    
    /// 更新集合中的回路信息
    /// 
    /// # 参数
    /// * `circuits` - 回路集合的可变引用
    /// * `circuit` - 更新后的回路信息
    /// 
    /// # 返回值
    /// * `Ok(())` - 更新成功
    /// * `Err(DistributionBoxError)` - 更新失败，包含错误信息
    pub fn update_circuit(circuits: &mut Vec<CircuitInfo>, circuit: CircuitInfo) -> Result<(), DistributionBoxError> {
        // 验证回路信息
        circuit.validate()?;
        
        // 查找并更新回路
        let mut found = false;
        let original_id = circuit.circuit_id.clone();
        
        for (i, existing_circuit) in circuits.iter().enumerate() {
            if existing_circuit.circuit_id == original_id {
                // 保留原始编号和相位信息
                let original_number = existing_circuit.number;
                let original_phase = existing_circuit.phase;
                
                // 更新回路信息
                circuits[i] = circuit;
                
                // 恢复编号和相位
                circuits[i].number = original_number;
                circuits[i].phase = original_phase;
                
                found = true;
                break;
            }
        }
        
        // 检查是否找到回路
        if !found {
            return Err(DistributionBoxError::CircuitNotFound(original_id));
        }
        
        Ok(())
    }
    
    /// 自动为回路分配编号
    /// 
    /// 按功率降序排序回路，并依次分配从1开始的连续编号
    /// 
    /// # 参数
    /// * `circuits` - 回路集合的可变引用
    pub fn auto_number_circuits(circuits: &mut [CircuitInfo]) {
        // 创建索引向量，用于排序
        let mut indices: Vec<usize> = (0..circuits.len()).collect();
        
        // 按功率降序排序索引
        indices.sort_by(|&i, &j| {
            circuits[j].power.partial_cmp(&circuits[i].power)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // 为排序后的回路分配编号
        for (rank, &index) in indices.iter().enumerate() {
            circuits[index].number = (rank + 1) as u32;
        }
    }
    
    /// 验证回路信息
    /// 
    /// # 参数
    /// * `circuit` - 要验证的回路信息
    /// 
    /// # 返回值
    /// * `Ok(())` - 验证通过
    /// * `Err(DistributionBoxError)` - 验证失败，包含错误信息
    pub fn validate_circuit(circuit: &CircuitInfo) -> Result<(), DistributionBoxError> {
        // 验证功率必须大于0
        if circuit.power <= 0.0 {
            return Err(DistributionBoxError::InvalidParameter(
                format!("回路'{}'的功率必须大于0，当前值：{}", circuit.name, circuit.power)
            ));
        }
        
        // 验证电流必须大于0
        if circuit.current <= 0.0 {
            return Err(DistributionBoxError::InvalidParameter(
                format!("回路'{}'的电流必须大于0，当前值：{}", circuit.name, circuit.current)
            ));
        }
        
        // 验证回路ID不为空
        if circuit.circuit_id.is_empty() {
            return Err(DistributionBoxError::InvalidParameter(
                "回路ID不能为空".to_string()
            ));
        }
        
        // 验证回路名称不为空
        if circuit.name.is_empty() {
            return Err(DistributionBoxError::InvalidParameter(
                "回路名称不能为空".to_string()
            ));
        }
        
        // 验证回路编号（如果已分配）
        if circuit.number > 0 && circuit.number > 99 {
            return Err(DistributionBoxError::InvalidParameter(
                format!("回路编号必须在1-99之间，当前值：{}", circuit.number)
            ));
        }
        
        // 验证相位（如果已分配）
        if let Some(phase) = circuit.phase {
            if phase != 'L' && phase != '1' && phase != '2' && phase != '3' {
                return Err(DistributionBoxError::InvalidParameter(
                    format!("无效的相位标识：{}，有效标识为 L, 1, 2, 3", phase)
                ));
            }
        }
        
        Ok(())
    }
    
    /// 查找回路
    /// 
    /// # 参数
    /// * `circuits` - 回路集合
    /// * `circuit_id` - 要查找的回路ID
    /// 
    /// # 返回值
    /// 找到则返回回路的不可变引用，否则返回None
    pub fn find_circuit<'a>(circuits: &'a [CircuitInfo], circuit_id: &str) -> Option<&'a CircuitInfo> {
        circuits.iter().find(|c| c.circuit_id == circuit_id)
    }
    
    /// 查找回路（可变引用）
    /// 
    /// # 参数
    /// * `circuits` - 回路集合的可变引用
    /// * `circuit_id` - 要查找的回路ID
    /// 
    /// # 返回值
    /// 找到则返回回路的可变引用，否则返回None
    pub fn find_circuit_mut<'a>(circuits: &'a mut [CircuitInfo], circuit_id: &str) -> Option<&'a mut CircuitInfo> {
        circuits.iter_mut().find(|c| c.circuit_id == circuit_id)
    }
    
    /// 批量验证回路集合
    /// 
    /// # 参数
    /// * `circuits` - 要验证的回路集合
    /// 
    /// # 返回值
    /// * `Ok(())` - 所有回路验证通过
    /// * `Err(DistributionBoxError)` - 任何一个回路验证失败，包含错误信息
    pub fn validate_circuits(circuits: &[CircuitInfo]) -> Result<(), DistributionBoxError> {
        for circuit in circuits {
            Self::validate_circuit(circuit)?;
        }
        Ok(())
    }
    
    /// 重置回路编号
    /// 
    /// 将所有回路的编号重置为0
    /// 
    /// # 参数
    /// * `circuits` - 回路集合的可变引用
    pub fn reset_circuit_numbers(circuits: &mut [CircuitInfo]) {
        for circuit in circuits {
            circuit.number = 0;
        }
    }
    
    /// 重置回路相位分配
    /// 
    /// 清除所有回路的相位分配
    /// 
    /// # 参数
    /// * `circuits` - 回路集合的可变引用
    pub fn reset_circuit_phases(circuits: &mut [CircuitInfo]) {
        for circuit in circuits {
            circuit.phase = None;
        }
    }
    
    /// 获取回路集合的总功率
    /// 
    /// # 参数
    /// * `circuits` - 回路集合
    /// 
    /// # 返回值
    /// 返回所有回路功率的总和（kW）
    pub fn calculate_total_power(circuits: &[CircuitInfo]) -> f64 {
        circuits.iter().map(|c| c.power).sum()
    }
    
    /// 获取回路集合的总电流
    /// 
    /// # 参数
    /// * `circuits` - 回路集合
    /// 
    /// # 返回值
    /// 返回所有回路电流的总和（A）
    pub fn calculate_total_current(circuits: &[CircuitInfo]) -> f64 {
        circuits.iter().map(|c| c.current).sum()
    }
}