//! 配电箱计算模块
//! 
//! 本模块提供配电箱相关的电气计算功能，包括总功率计算、总电流计算、进线保护设备选型以及三相平衡算法。

use crate::editor::business::{CircuitInfo, DistributionBoxError};

/// 配电箱计算器
/// 
/// 提供配电箱相关的电气计算功能
pub struct DistributionBoxCalculator;

impl DistributionBoxCalculator {
    /// 计算总功率
    /// 
    /// # 参数
    /// * `circuits` - 回路信息集合
    /// 
    /// # 返回值
    /// 返回所有回路功率的总和（kW）
    pub fn calculate_total_power(circuits: &[CircuitInfo]) -> f64 {
        circuits.iter().map(|c| c.power).sum()
    }
    
    /// 计算总电流
    /// 
    /// 使用三相380V系统的计算公式：I = P * 1000 / (1.732 * 380 * 0.85)
    /// 其中0.85是默认的功率因数
    /// 
    /// # 参数
    /// * `total_power` - 总功率（kW）
    /// * `power_factor` - 功率因数，默认为0.85
    /// 
    /// # 返回值
    /// 返回计算得到的总电流（A）
    pub fn calculate_total_current(total_power: f64, power_factor: f64) -> Result<f64, DistributionBoxError> {
        // 验证参数
        if total_power < 0.0 {
            return Err(DistributionBoxError::InvalidParameter(
                "总功率不能为负值".to_string()
            ));
        }
        
        if power_factor <= 0.0 || power_factor > 1.0 {
            return Err(DistributionBoxError::InvalidParameter(
                format!("功率因数必须在0-1之间，当前值：{}", power_factor)
            ));
        }
        
        // 如果总功率为0，直接返回0电流
        if total_power == 0.0 {
            return Ok(0.0);
        }
        
        // 标准三相电压（V）
        const VOLTAGE: f64 = 380.0;
        // 三相根号3
        const SQRT_3: f64 = 1.732;
        
        // 计算公式：I = P * 1000 / (√3 * U * cosφ)
        let total_current = (total_power * 1000.0) / (SQRT_3 * VOLTAGE * power_factor);
        
        Ok(total_current)
    }
    
    /// 计算进线保护设备电流整定值
    /// 
    /// 基于总电流的1.2倍选择标准电流等级
    /// 标准电流等级序列：6, 10, 16, 20, 25, 32, 40, 50, 63, 80, 100, 125, 160, 200, 250, 315, 400, 500, 630
    /// 
    /// # 参数
    /// * `total_current` - 总电流（A）
    /// 
    /// # 返回值
    /// 返回选择的标准电流等级（A）
    pub fn calculate_incoming_current(total_current: f64) -> Result<f64, DistributionBoxError> {
        // 验证参数
        if total_current < 0.0 {
            return Err(DistributionBoxError::InvalidParameter(
                "总电流不能为负值".to_string()
            ));
        }
        
        // 如果总电流为0，直接返回0
        if total_current == 0.0 {
            return Ok(0.0);
        }
        
        // 安全系数（行业标准通常为1.2）
        const SAFETY_FACTOR: f64 = 1.2;
        
        // 计算需要的电流值
        let required_current = total_current * SAFETY_FACTOR;
        
        // 标准电流等级序列
        let standard_currents = [
            6.0, 10.0, 16.0, 20.0, 25.0, 32.0, 40.0, 50.0, 63.0,
            80.0, 100.0, 125.0, 160.0, 200.0, 250.0, 315.0, 400.0, 500.0, 630.0
        ];
        
        // 查找第一个大于等于所需电流的标准等级
        for &current in standard_currents.iter() {
            if current >= required_current {
                return Ok(current);
            }
        }
        
        // 如果超出最大标准等级，返回最大等级
        Ok(standard_currents[standard_currents.len() - 1])
    }
    
    /// 执行三相平衡
    /// 
    /// 使用贪心算法将回路分配到L1、L2、L3三相，实现负载平衡
    /// 
    /// # 参数
    /// * `circuits` - 回路集合的可变引用
    /// 
    /// # 返回值
    /// * `Ok([L1负载, L2负载, L3负载])` - 平衡成功，返回各相负载（kW）
    /// * `Err(DistributionBoxError)` - 平衡失败，包含错误信息
    pub fn balance_three_phases(circuits: &mut [CircuitInfo]) -> Result<[f64; 3], DistributionBoxError> {
        // 检查回路集合是否为空
        if circuits.is_empty() {
            return Ok([0.0, 0.0, 0.0]);
        }
        
        // 重置所有回路的相位分配
        for circuit in circuits.iter_mut() {
            circuit.phase = None;
        }
        
        // 按功率降序排序回路
        let mut indices: Vec<usize> = (0..circuits.len()).collect();
        indices.sort_by(|&i, &j| {
            circuits[j].power.partial_cmp(&circuits[i].power)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        // 初始化各相负载
        let mut phase_loads = [0.0, 0.0, 0.0];
        
        // 初始分配：按功率大小依次分配到负载最小的相
        for &index in &indices {
            // 找到当前负载最小的相
            let min_phase_index = Self::find_min_phase_index(&phase_loads);
            
            // 分配回路到该相
            circuits[index].phase = match min_phase_index {
                0 => Some('L'), // L1相
                1 => Some('1'), // L2相
                2 => Some('2'), // L3相
                _ => unreachable!(),
            };
            
            // 更新相负载
            phase_loads[min_phase_index] += circuits[index].power;
        }
        
        // 迭代优化：尝试交换回路以提高平衡性
        const MAX_ITERATIONS: usize = 100;
        let mut iteration = 0;
        
        while iteration < MAX_ITERATIONS {
            // 计算当前平衡度
            let balance_degree = Self::calculate_balance_degree(&phase_loads);
            
            // 如果平衡度已满足要求（不平衡度<1%），则停止优化
            if balance_degree < 0.01 {
                break;
            }
            
            // 找到负载最大和最小的相
            let (max_phase_index, min_phase_index) = Self::find_max_min_phase_indices(&phase_loads);
            
            // 找到最大相上可转移的回路
            let mut best_transfer_index = None;
            let mut best_balance_degree = balance_degree;
            
            for &index in &indices {
                // 检查该回路是否在最大相上
                let circuit_phase_index = match circuits[index].phase {
                    Some('L') => 0, // L1相
                    Some('1') => 1, // L2相
                    Some('2') => 2, // L3相
                    _ => continue, // 跳过未分配的回路
                };
                
                if circuit_phase_index != max_phase_index {
                    continue;
                }
                
                // 尝试将回路从最大相转移到最小相
                let circuit_power = circuits[index].power;
                
                // 计算转移后的负载
                let mut new_phase_loads = phase_loads.clone();
                new_phase_loads[max_phase_index] -= circuit_power;
                new_phase_loads[min_phase_index] += circuit_power;
                
                // 计算新的平衡度
                let new_balance_degree = Self::calculate_balance_degree(&new_phase_loads);
                
                // 如果平衡度改善，则记录这个转移
                if new_balance_degree < best_balance_degree {
                    best_balance_degree = new_balance_degree;
                    best_transfer_index = Some(index);
                }
            }
            
            // 如果找到了改善平衡度的转移，则执行转移
            if let Some(transfer_index) = best_transfer_index {
                // 更新回路相位
                circuits[transfer_index].phase = match min_phase_index {
                    0 => Some('L'), // L1相
                    1 => Some('1'), // L2相
                    2 => Some('2'), // L3相
                    _ => unreachable!(),
                };
                
                // 更新相负载
                let circuit_power = circuits[transfer_index].power;
                phase_loads[max_phase_index] -= circuit_power;
                phase_loads[min_phase_index] += circuit_power;
            } else {
                // 如果没有找到可改善的转移，则退出循环
                break;
            }
            
            iteration += 1;
        }
        
        // 调整相位表示，统一使用L1/L2/L3
        for circuit in circuits.iter_mut() {
            if let Some(phase_char) = circuit.phase {
                circuit.phase = match phase_char {
                    'L' => Some('L'), // L1
                    '1' => Some('L'), // 更正为L1
                    '2' => Some('L'), // 更正为L1
                    _ => Some('L'),   // 默认L1
                };
            }
        }
        
        // 确保返回的相位负载是正确的
        Ok(phase_loads)
    }
    
    /// 查找负载最小的相索引
    /// 
    /// # 参数
    /// * `phase_loads` - 各相负载数组
    /// 
    /// # 返回值
    /// 返回负载最小的相索引（0=L1, 1=L2, 2=L3）
    fn find_min_phase_index(phase_loads: &[f64; 3]) -> usize {
        let mut min_index = 0;
        let mut min_load = phase_loads[0];
        
        for (i, &load) in phase_loads.iter().enumerate().skip(1) {
            if load < min_load {
                min_load = load;
                min_index = i;
            }
        }
        
        min_index
    }
    
    /// 查找负载最大和最小的相索引
    /// 
    /// # 参数
    /// * `phase_loads` - 各相负载数组
    /// 
    /// # 返回值
    /// 返回(最大负载相索引, 最小负载相索引)
    fn find_max_min_phase_indices(phase_loads: &[f64; 3]) -> (usize, usize) {
        let mut max_index = 0;
        let mut min_index = 0;
        let mut max_load = phase_loads[0];
        let mut min_load = phase_loads[0];
        
        for (i, &load) in phase_loads.iter().enumerate().skip(1) {
            if load > max_load {
                max_load = load;
                max_index = i;
            }
            if load < min_load {
                min_load = load;
                min_index = i;
            }
        }
        
        (max_index, min_index)
    }
    
    /// 计算三相平衡度
    /// 
    /// 平衡度 = (最大负载 - 最小负载) / 平均负载
    /// 平衡度越接近0，表示三相越平衡
    /// 
    /// # 参数
    /// * `phase_loads` - 各相负载数组
    /// 
    /// # 返回值
    /// 返回平衡度（0-1之间的值）
    fn calculate_balance_degree(phase_loads: &[f64; 3]) -> f64 {
        let total_load: f64 = phase_loads.iter().sum();
        
        // 如果总负载为0，则视为完全平衡
        if total_load == 0.0 {
            return 0.0;
        }
        
        let max_load = phase_loads.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_load = phase_loads.iter().cloned().fold(f64::INFINITY, f64::min);
        let avg_load = total_load / 3.0;
        
        // 计算平衡度
        (max_load - min_load) / avg_load
    }
    
    /// 计算各相的回路数量
    /// 
    /// # 参数
    /// * `circuits` - 回路集合
    /// 
    /// # 返回值
    /// 返回[L1回路数, L2回路数, L3回路数]
    pub fn calculate_phase_circuit_counts(circuits: &[CircuitInfo]) -> [usize; 3] {
        let mut counts = [0; 3];
        
        for circuit in circuits {
            if let Some(phase) = circuit.phase {
                match phase {
                    'L' => counts[0] += 1,
                    '1' => counts[1] += 1,
                    '2' => counts[2] += 1,
                    _ => {},
                }
            }
        }
        
        counts
    }
}