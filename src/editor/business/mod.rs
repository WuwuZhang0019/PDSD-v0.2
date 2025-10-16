/// 编辑器业务逻辑模块

pub mod circuit_parameters;
pub mod circuit_calculator;
pub mod circuit_node;
pub mod distribution_box_parameters;
pub mod circuit_manager;
pub mod distribution_box_calculator;
pub mod distribution_box_node;
pub mod distribution_box_template;
pub mod node_data_transfer;

// 条件导出测试模块
#[cfg(test)]
pub mod distribution_box_tests;

// 导出主要数据结构和函数
pub use circuit_parameters::{CircuitParameters, CircuitResult, VoltageType, CircuitData};
pub use circuit_calculator::{CircuitCalculator, CircuitCalculationError};
pub use circuit_node::{CircuitNode, CircuitNodeTemplate, CircuitNodeResponse, EditorState};
pub use distribution_box_parameters::{DistributionBoxNode as BoxData, CircuitInfo, DistributionBoxError, DistributionBoxResponse};
pub use circuit_manager::CircuitManager;
pub use distribution_box_calculator::DistributionBoxCalculator;
pub use distribution_box_node::DistributionBoxNode;
pub use distribution_box_template::DistributionBoxTemplate;
pub use node_data_transfer::{PowerGraphNode, PowerGraphState};

/// 业务逻辑模块初始化函数
pub fn initialize() {
    // 初始化日志或其他全局资源
    #[cfg(debug_assertions)]
    println!("业务逻辑模块初始化完成");
}