/// 核心trait模块
/// 定义电气计算和数据处理的核心接口

pub mod calculation;

// 重新导出常用trait
pub use calculation::{ElectricalCalculation, GraphCalculation, CalculationResult, DefaultElectricalCalculation};