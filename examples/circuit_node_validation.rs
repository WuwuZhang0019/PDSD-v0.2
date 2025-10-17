// 配电回路节点功能验证脚本

use power_distribution_system_diagram::editor::business::{CircuitNode};
use power_distribution_system_diagram::core_lib::data_types::{CircuitNodeProperties, CircuitType};

fn main() {
    println!("=== 配电回路节点功能验证 ===");
    
    // 1. 测试单相信路节点创建
    println!("\n1. 创建单相信路节点");
    let mut single_phase_node = CircuitNode::new_single_phase("照明回路", 5.0);
    println!("   - 节点ID: {}", single_phase_node.id);
    println!("   - 回路名称: {}", single_phase_node.parameters.name);
    println!("   - 回路类型: {}", single_phase_node.properties.circuit_type.to_str());
    println!("   - 功率: {}kW", single_phase_node.parameters.pe);
    println!("   - 计算电流: {:.2}A", single_phase_node.properties.current);
    
    // 2. 测试三相信路节点创建
    println!("\n2. 创建三相信路节点");
    let mut three_phase_node = CircuitNode::new_three_phase("动力回路", 20.0);
    println!("   - 节点ID: {}", three_phase_node.id);
    println!("   - 回路名称: {}", three_phase_node.parameters.name);
    println!("   - 回路类型: {}", three_phase_node.properties.circuit_type.to_str());
    println!("   - 功率: {}kW", three_phase_node.parameters.pe);
    println!("   - 计算电流: {:.2}A", three_phase_node.properties.current);
    
    // 3. 测试参数更新
    println!("\n3. 更新回路参数");
    single_phase_node.parameters.pe = 8.0;
    single_phase_node.parameters.kx = 0.9;
    single_phase_node.sync_parameters_to_properties();
    single_phase_node.recalculate();
    println!("   - 更新后功率: {}kW", single_phase_node.parameters.pe);
    println!("   - 更新后需求系数: {}", single_phase_node.parameters.kx);
    println!("   - 更新后计算电流: {:.2}A", single_phase_node.properties.current);
    
    // 4. 测试回路类型切换
    println!("\n4. 切换回路类型");
    single_phase_node.set_circuit_type(CircuitType::ThreePhase);
    println!("   - 切换后类型: {}", single_phase_node.properties.circuit_type.to_str());
    println!("   - 切换后电压: {}V", single_phase_node.properties.voltage);
    println!("   - 切换后计算电流: {:.2}A", single_phase_node.properties.current);
    
    // 5. 测试元器件选型
    println!("\n5. 查看元器件选型");
    println!("   - 元器件类型: {}", three_phase_node.properties.component_type);
    println!("   - 元器件电流: {}A", three_phase_node.properties.component_current);
    println!("   - 线缆规格: {}", three_phase_node.properties.cable_spec);
    
    // 6. 测试参数验证
    println!("\n6. 测试参数验证");
    let mut invalid_node = CircuitNode::new_single_phase("无效回路", 5.0);
    invalid_node.parameters.pe = -1.0;
    invalid_node.validate();
    println!("   - 无效功率时错误数: {}", invalid_node.errors.len());
    
    // 恢复有效参数
    invalid_node.parameters.pe = 5.0;
    invalid_node.validate();
    println!("   - 有效参数时错误数: {}", invalid_node.errors.len());
    
    println!("\n=== 验证完成 ===");
}