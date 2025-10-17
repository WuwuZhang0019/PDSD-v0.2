/// 核心trait模块
/// 定义电气计算和数据处理的核心接口

pub mod calculation;
pub mod widgets;

// 重新导出常用trait
pub use calculation::{ElectricalCalculation, GraphCalculation, CalculationResult, DefaultElectricalCalculation};
pub use widgets::{ElectricResponse, widget_utils};