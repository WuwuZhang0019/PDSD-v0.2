/// 核心库模块
/// 提供电气系统图的基础数据类型、算法和工具函数

pub mod data_types;
pub mod traits;
pub mod algorithm;
pub mod utils;

// 导入错误模块
mod error;

// 重新导出常用组件
pub use data_types::*;
pub use traits::*;
pub use error::CoreError;