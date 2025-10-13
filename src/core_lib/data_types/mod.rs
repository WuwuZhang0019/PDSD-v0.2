/// 核心数据类型模块
/// 定义电气系统的基础数据类型和节点数据结构

pub mod electric_data;
pub mod node_data;

// 重新导出常用类型
pub use electric_data::{ElectricDataType, ElectricValueType, PhaseBalanceInfo};
pub use node_data::{
    ElectricNodeData,
    CircuitNodeData,
    DistributionBoxNodeData,
    TrunkLineNodeData,
    PowerSourceNodeData,
    CalculationNodeData
};