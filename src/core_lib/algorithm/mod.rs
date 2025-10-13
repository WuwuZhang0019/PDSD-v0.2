/// 算法模块
/// 包含各种电气计算算法和图算法

pub mod electrical_calculation;
pub mod current_calculation;
pub mod power_calculation;
pub mod resistance_calculation;
pub mod voltage_drop_calculation;

// 重新导出常用组件
pub use electrical_calculation::ElectricalCalculator;