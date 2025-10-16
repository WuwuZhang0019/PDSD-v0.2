//! 配电箱节点参数定义模块
//! 
//! 本模块定义了配电箱节点相关的数据结构，包括配电箱节点数据、回路信息和错误类型等。

use serde::{Serialize, Deserialize};
use thiserror::Error;

/// 配电箱错误类型
#[derive(Debug, Error, Clone, PartialEq)]
pub enum DistributionBoxError {
    /// 回路数量超出限制
    #[error("回路数量超出限制: {0}个回路，最大允许99个")]
    CircuitCountExceeded(u32),
    
    /// 参数无效
    #[error("参数无效: {0}")]
    InvalidParameter(String),
    
    /// 三相平衡失败
    #[error("三相平衡失败，达到最大迭代次数")]
    BalanceFailed,
    
    /// 计算错误
    #[error("计算错误: {0}")]
    CalculationError(String),
    
    /// 回路不存在
    #[error("回路不存在: {0}")]
    CircuitNotFound(String),
    
    /// 数据序列化错误
    #[error("数据序列化错误: {0}")]
    SerializationError(String),
}

/// 回路信息结构体
/// 
/// 存储单个回路的关键信息，包括ID、名称、功率、电流、编号和相位分配
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CircuitInfo {
    /// 回路唯一标识符
    pub circuit_id: String,
    /// 回路名称
    pub name: String,
    /// 回路功率（kW）
    pub power: f64,
    /// 回路电流（A）
    pub current: f64,
    /// 自动分配的编号
    pub number: u32,
    /// 分配的相（L1/L2/L3），None表示未分配
    pub phase: Option<char>,
}

impl CircuitInfo {
    /// 创建新的回路信息实例
    /// 
    /// # 参数
    /// * `circuit_id` - 回路唯一标识符
    /// * `name` - 回路名称
    /// * `power` - 回路功率（kW）
    /// * `current` - 回路电流（A）
    /// 
    /// # 返回值
    /// 返回新创建的CircuitInfo实例
    pub fn new(circuit_id: String, name: String, power: f64, current: f64) -> Self {
        Self {
            circuit_id,
            name,
            power,
            current,
            number: 0, // 初始编号为0，将在自动编号时设置
            phase: None, // 初始未分配相位
        }
    }
    
    /// 验证回路信息是否有效
    /// 
    /// # 返回值
    /// * `Ok(())` - 验证通过
    /// * `Err(DistributionBoxError)` - 验证失败，包含错误信息
    pub fn validate(&self) -> Result<(), DistributionBoxError> {
        // 验证功率必须大于0
        if self.power <= 0.0 {
            return Err(DistributionBoxError::InvalidParameter(
                format!("回路'{}'的功率必须大于0，当前值：{}", self.name, self.power)
            ));
        }
        
        // 验证电流必须大于0
        if self.current <= 0.0 {
            return Err(DistributionBoxError::InvalidParameter(
                format!("回路'{}'的电流必须大于0，当前值：{}", self.name, self.current)
            ));
        }
        
        // 验证回路ID不为空
        if self.circuit_id.is_empty() {
            return Err(DistributionBoxError::InvalidParameter(
                "回路ID不能为空".to_string()
            ));
        }
        
        // 验证回路名称不为空
        if self.name.is_empty() {
            return Err(DistributionBoxError::InvalidParameter(
                "回路名称不能为空".to_string()
            ));
        }
        
        Ok(())
    }
}

/// 配电箱节点数据结构体
/// 
/// 存储配电箱节点的所有相关信息和状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DistributionBoxNode {
    /// 配电箱名称
    pub name: String,
    /// 总功率（kW）
    pub total_power: f64,
    /// 总电流（A）
    pub total_current: f64,
    /// 进线保护设备电流整定值（A）
    pub incoming_current: f64,
    /// 所在楼层
    pub floor: u32,
    /// 包含的模块列表
    pub modules: Vec<String>,
    /// L1, L2, L3各相负载（kW）
    pub phase_loads: [f64; 3],
    /// 管理的回路信息列表
    pub circuits: Vec<CircuitInfo>,
}

impl Default for DistributionBoxNode {
    /// 创建默认的配电箱节点数据
    fn default() -> Self {
        Self {
            name: "新建配电箱".to_string(),
            total_power: 0.0,
            total_current: 0.0,
            incoming_current: 0.0,
            floor: 1,
            modules: Vec::new(),
            phase_loads: [0.0; 3],
            circuits: Vec::new(),
        }
    }
}

impl DistributionBoxNode {
    /// 创建新的配电箱节点数据
    /// 
    /// # 参数
    /// * `name` - 配电箱名称
    /// * `floor` - 所在楼层
    /// 
    /// # 返回值
    /// 返回新创建的DistributionBoxNode实例
    pub fn new(name: String, floor: u32) -> Self {
        Self {
            name,
            floor,
            ..Default::default()
        }
    }
    
    /// 添加回路到配电箱
    /// 
    /// # 参数
    /// * `circuit` - 要添加的回路信息
    /// 
    /// # 返回值
    /// * `Ok(())` - 添加成功
    /// * `Err(DistributionBoxError)` - 添加失败，包含错误信息
    pub fn add_circuit(&mut self, circuit: CircuitInfo) -> Result<(), DistributionBoxError> {
        // 验证回路信息
        circuit.validate()?;
        
        // 检查回路数量限制
        if self.circuits.len() >= 99 {
            return Err(DistributionBoxError::CircuitCountExceeded(
                self.circuits.len() as u32 + 1
            ));
        }
        
        // 检查回路ID是否已存在
        if self.circuits.iter().any(|c| c.circuit_id == circuit.circuit_id) {
            return Err(DistributionBoxError::InvalidParameter(
                format!("回路ID '{}' 已存在", circuit.circuit_id)
            ));
        }
        
        // 添加回路
        self.circuits.push(circuit);
        Ok(())
    }
    
    /// 从配电箱移除回路
    /// 
    /// # 参数
    /// * `circuit_id` - 要移除的回路ID
    /// 
    /// # 返回值
    /// * `Ok(())` - 移除成功
    /// * `Err(DistributionBoxError)` - 移除失败，包含错误信息
    pub fn remove_circuit(&mut self, circuit_id: &str) -> Result<(), DistributionBoxError> {
        let initial_len = self.circuits.len();
        
        // 过滤掉指定ID的回路
        self.circuits.retain(|c| c.circuit_id != circuit_id);
        
        // 检查是否找到并移除了回路
        if self.circuits.len() == initial_len {
            return Err(DistributionBoxError::CircuitNotFound(
                circuit_id.to_string()
            ));
        }
        
        Ok(())
    }
    
    /// 更新配电箱中的回路信息
    /// 
    /// # 参数
    /// * `updated_circuit` - 更新后的回路信息
    /// 
    /// # 返回值
    /// * `Ok(())` - 更新成功
    /// * `Err(DistributionBoxError)` - 更新失败，包含错误信息
    pub fn update_circuit(&mut self, updated_circuit: CircuitInfo) -> Result<(), DistributionBoxError> {
        // 验证回路信息
        updated_circuit.validate()?;
        
        // 查找并更新回路
        let mut found = false;
        
        for (i, circuit) in self.circuits.iter_mut().enumerate() {
            if circuit.circuit_id == updated_circuit.circuit_id {
                // 保留原始编号和相位信息
                let original_number = circuit.number;
                let original_phase = circuit.phase;
                
                // 更新回路信息
                self.circuits[i] = updated_circuit;
                
                // 恢复编号和相位
                self.circuits[i].number = original_number;
                self.circuits[i].phase = original_phase;
                
                found = true;
                break;
            }
        }
        
        // 检查是否找到回路
        if !found {
            return Err(DistributionBoxError::CircuitNotFound(
                updated_circuit.circuit_id.to_string()
            ));
        }
        
        Ok(())
    }
    
    /// 获取回路数量
    /// 
    /// # 返回值
    /// 返回当前配电箱中管理的回路数量
    pub fn circuit_count(&self) -> usize {
        self.circuits.len()
    }
    
    /// 检查是否包含指定ID的回路
    /// 
    /// # 参数
    /// * `circuit_id` - 要检查的回路ID
    /// 
    /// # 返回值
    /// 如果包含指定ID的回路则返回true，否则返回false
    pub fn contains_circuit(&self, circuit_id: &str) -> bool {
        self.circuits.iter().any(|c| c.circuit_id == circuit_id)
    }
    
    /// 重置计算结果
    /// 
    /// 将总功率、总电流、进线电流和相负载重置为零
    pub fn reset_calculations(&mut self) {
        self.total_power = 0.0;
        self.total_current = 0.0;
        self.incoming_current = 0.0;
        self.phase_loads = [0.0; 3];
    }
}

/// 配电箱节点响应类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DistributionBoxResponse {
    /// 节点更新响应
    NodeUpdated,
    /// 参数变更响应
    ParameterChanged(String, f64),
    /// 回路添加响应
    CircuitAdded(CircuitInfo),
    /// 回路移除响应
    CircuitRemoved(String),
    /// 回路更新响应
    CircuitUpdated(CircuitInfo),
    /// 计算完成响应
    CalculationCompleted,
    /// 错误响应
    Error(DistributionBoxError),
}

/// 编辑器状态类型别名
/// 
/// 用于NodeDataTrait中的UserState类型
pub type EditorState = (); // 实际实现时将替换为真实的编辑器状态类型