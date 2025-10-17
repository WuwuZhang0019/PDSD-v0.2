/// 核心数据类型模块
/// 定义电气系统的基础数据类型和节点数据结构

pub mod electric_data;
pub mod node_data;

// 重新导出常用类型 - 基础参数类型
pub use electric_data::{
    VoltageLevel, LayingMethod, PipeSpecification, LayingArea, PhaseSequence, Phase,
    CircuitNumber, BreakerType, FrameCurrent, BreakingCapacity, DeductionMethod,
    Pole, Curve, SettingValue, PhaseConfig
};

// 重新导出常用类型 - 组件参数类型
pub use electric_data::{
    Breaker, Isolator, DualPowerSwitch, Contactor, EnergyMeter, CableInfo,
    ElectricComponent
};

// 重新导出常用类型 - 数据结构类型
pub use electric_data::{
    ElectricDataType, ElectricValueType, PhaseBalanceInfo, CircuitData
};

// 重新导出节点数据类型
pub use node_data::{
    Dimensions, DistributionBoxData,
    ElectricNodeData,
    CircuitNodeData,
    DistributionBoxNodeData,
    TrunkLineNodeData,
    PowerSourceNodeData,
    CalculationNodeData
};