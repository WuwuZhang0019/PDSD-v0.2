//! 配电箱功能单元测试
//! 
//! 本文件包含对配电箱相关功能的全面单元测试

use crate::editor::business::{BoxData, CircuitInfo, CircuitManager, DistributionBoxCalculator, DistributionBoxNode, DistributionBoxTemplate};
use crate::editor::business::{DistributionBoxError};

#[test]
fn test_box_data_basic_operations() {
    // 测试配电箱数据的基本操作
    let mut box_data = BoxData::default();
    
    // 验证默认值
    assert_eq!(box_data.name, "新建配电箱");
    assert_eq!(box_data.floor, 1);
    assert!(box_data.circuits.is_empty());
    assert_eq!(box_data.total_power, 0.0);
    assert_eq!(box_data.total_current, 0.0);
    assert_eq!(box_data.incoming_current, 0.0);
    
    // 创建回路并添加到配电箱
    let circuit1 = CircuitInfo::new(
        "circuit_1".to_string(),
        "照明回路".to_string(),
        5.0,
        10.0
    );
    let circuit2 = CircuitInfo::new(
        "circuit_2".to_string(),
        "插座回路".to_string(),
        3.0,
        6.0
    );
    
    // 添加回路
    assert!(box_data.add_circuit(circuit1.clone()).is_ok());
    assert!(box_data.add_circuit(circuit2.clone()).is_ok());
    assert_eq!(box_data.circuits.len(), 2);
    
    // 测试回路更新
    let mut updated_circuit = circuit1.clone();
    updated_circuit.name = "更新的照明回路".to_string();
    updated_circuit.power = 8.0;
    
    assert!(box_data.update_circuit(updated_circuit.clone()).is_ok());
    assert_eq!(box_data.circuits[0].name, "更新的照明回路");
    assert_eq!(box_data.circuits[0].power, 8.0);
    
    // 测试回路移除
    assert!(box_data.remove_circuit("circuit_1").is_ok());
    assert_eq!(box_data.circuits.len(), 1);
    assert_eq!(box_data.circuits[0].name, "插座回路");
    
    // 测试错误处理：添加重复ID的回路
    let duplicate_circuit = CircuitInfo::new(
        "circuit_2".to_string(),
        "重复ID回路".to_string(),
        2.0,
        4.0
    );
    
    let result = box_data.add_circuit(duplicate_circuit);
    assert!(result.is_err());
    if let Err(DistributionBoxError::CircuitIdExists(id)) = result {
        assert_eq!(id, "circuit_2");
    }
    
    // 测试错误处理：更新不存在的回路
    let non_existent_circuit = CircuitInfo::new(
        "circuit_3".to_string(),
        "不存在的回路".to_string(),
        1.0,
        2.0
    );
    
    let result = box_data.update_circuit(non_existent_circuit);
    assert!(result.is_err());
    if let Err(DistributionBoxError::CircuitNotFound(id)) = result {
        assert_eq!(id, "circuit_3");
    }
}

#[test]
fn test_circuit_manager_auto_numbering() {
    // 测试回路自动编号功能
    let mut circuits = vec![
        CircuitInfo::new("circuit_1".to_string(), "回路1".to_string(), 3.0, 6.0),
        CircuitInfo::new("circuit_2".to_string(), "回路2".to_string(), 5.0, 10.0),
        CircuitInfo::new("circuit_3".to_string(), "回路3".to_string(), 2.0, 4.0),
        CircuitInfo::new("circuit_4".to_string(), "回路4".to_string(), 8.0, 16.0),
    ];
    
    // 执行自动编号（应该按功率降序编号）
    CircuitManager::auto_number_circuits(&mut circuits);
    
    // 验证编号结果
    // 回路4 (8kW) -> 编号1
    let circuit4 = circuits.iter().find(|c| c.circuit_id == "circuit_4").unwrap();
    assert_eq!(circuit4.number, 1);
    
    // 回路2 (5kW) -> 编号2
    let circuit2 = circuits.iter().find(|c| c.circuit_id == "circuit_2").unwrap();
    assert_eq!(circuit2.number, 2);
    
    // 回路1 (3kW) -> 编号3
    let circuit1 = circuits.iter().find(|c| c.circuit_id == "circuit_1").unwrap();
    assert_eq!(circuit1.number, 3);
    
    // 回路3 (2kW) -> 编号4
    let circuit3 = circuits.iter().find(|c| c.circuit_id == "circuit_3").unwrap();
    assert_eq!(circuit3.number, 4);
}

#[test]
fn test_distribution_box_calculator() {
    // 测试配电箱计算器功能
    let circuits = vec![
        CircuitInfo::new("circuit_1".to_string(), "回路1".to_string(), 5.0, 10.0),
        CircuitInfo::new("circuit_2".to_string(), "回路2".to_string(), 3.0, 6.0),
        CircuitInfo::new("circuit_3".to_string(), "回路3".to_string(), 4.0, 8.0),
    ];
    
    // 测试总功率计算
    let total_power = DistributionBoxCalculator::calculate_total_power(&circuits);
    assert_eq!(total_power, 12.0); // 5+3+4=12kW
    
    // 测试总电流计算（三相380V，功率因数0.85）
    let total_current = DistributionBoxCalculator::calculate_total_current(total_power, 0.85).unwrap();
    // 计算公式：I = 12000 / (1.732 * 380 * 0.85) ≈ 21.08A
    assert!((total_current - 21.08).abs() < 0.01);
    
    // 测试进线电流计算（标准电流等级）
    let incoming_current = DistributionBoxCalculator::calculate_incoming_current(total_current).unwrap();
    // 21.08 * 1.2 = 25.3A，应该选择32A
    assert_eq!(incoming_current, 32.0);
    
    // 测试错误处理：负值总功率
    let result = DistributionBoxCalculator::calculate_total_current(-5.0, 0.85);
    assert!(result.is_err());
    if let Err(DistributionBoxError::InvalidParameter(msg)) = result {
        assert!(msg.contains("总功率不能为负值"));
    }
    
    // 测试错误处理：无效功率因数
    let result = DistributionBoxCalculator::calculate_total_current(10.0, 1.5);
    assert!(result.is_err());
    if let Err(DistributionBoxError::InvalidParameter(msg)) = result {
        assert!(msg.contains("功率因数必须在0-1之间"));
    }
}

#[test]
fn test_three_phase_balancing() {
    // 测试三相平衡算法
    let mut circuits = vec![
        CircuitInfo::new("circuit_1".to_string(), "回路1".to_string(), 10.0, 20.0),
        CircuitInfo::new("circuit_2".to_string(), "回路2".to_string(), 8.0, 16.0),
        CircuitInfo::new("circuit_3".to_string(), "回路3".to_string(), 7.0, 14.0),
        CircuitInfo::new("circuit_4".to_string(), "回路4".to_string(), 6.0, 12.0),
        CircuitInfo::new("circuit_5".to_string(), "回路5".to_string(), 5.0, 10.0),
        CircuitInfo::new("circuit_6".to_string(), "回路6".to_string(), 4.0, 8.0),
    ];
    
    // 执行三相平衡
    let phase_loads = DistributionBoxCalculator::balance_three_phases(&mut circuits).unwrap();
    
    // 验证所有回路都被分配了相位
    for circuit in &circuits {
        assert!(circuit.phase.is_some());
    }
    
    // 计算各相负载总和（应该等于总功率30kW）
    let total_phase_loads: f64 = phase_loads.iter().sum();
    assert_eq!(total_phase_loads, 30.0);
    
    // 计算平衡度（应该小于0.2，即20%）
    let balance_degree = DistributionBoxCalculator::calculate_balance_degree(&phase_loads);
    assert!(balance_degree < 0.2);
    
    // 验证各相的回路数量
    let phase_counts = DistributionBoxCalculator::calculate_phase_circuit_counts(&circuits);
    assert_eq!(phase_counts.iter().sum::<usize>(), 6); // 总共6个回路
}

#[test]
fn test_distribution_box_node_recalculation() {
    // 测试配电箱节点的完整计算流程
    let mut box_node = DistributionBoxNode::default();
    
    // 添加多个回路
    let circuits = vec![
        CircuitInfo::new("circuit_1".to_string(), "照明回路1".to_string(), 5.0, 10.0),
        CircuitInfo::new("circuit_2".to_string(), "插座回路1".to_string(), 3.0, 6.0),
        CircuitInfo::new("circuit_3".to_string(), "空调回路1".to_string(), 7.0, 14.0),
        CircuitInfo::new("circuit_4".to_string(), "照明回路2".to_string(), 4.0, 8.0),
        CircuitInfo::new("circuit_5".to_string(), "插座回路2".to_string(), 2.0, 4.0),
    ];
    
    for circuit in circuits {
        assert!(box_node.add_circuit(circuit));
    }
    
    // 手动触发重新计算
    box_node.recalculate();
    
    // 验证计算结果
    assert_eq!(box_node.data.circuits.len(), 5);
    assert_eq!(box_node.data.total_power, 21.0); // 5+3+7+4+2=21kW
    
    // 验证总电流（基于21kW，功率因数0.85）
    // 计算值应为：21000 / (1.732 * 380 * 0.85) ≈ 36.89A
    assert!((box_node.data.total_current - 36.89).abs() < 0.01);
    
    // 验证进线电流（1.2倍总电流，向上取标准值）
    // 36.89 * 1.2 = 44.27A，应选择50A
    assert_eq!(box_node.data.incoming_current, 50.0);
    
    // 验证所有回路都有编号
    let numbers: Vec<u32> = box_node.data.circuits.iter().map(|c| c.number).collect();
    assert!(numbers.contains(&1));
    assert!(numbers.contains(&2));
    assert!(numbers.contains(&3));
    assert!(numbers.contains(&4));
    assert!(numbers.contains(&5));
    
    // 验证所有回路都有相位分配
    for circuit in &box_node.data.circuits {
        assert!(circuit.phase.is_some());
    }
    
    // 验证数据映射功能
    let data_map = box_node.to_data_map();
    assert_eq!(data_map.get("total_power").copied().unwrap(), 21.0);
    assert_eq!(data_map.get("total_current").copied().unwrap(), box_node.data.total_current);
    assert_eq!(data_map.get("incoming_current").copied().unwrap(), 50.0);
    assert_eq!(data_map.get("floor").copied().unwrap(), 1.0);
    assert_eq!(data_map.get("circuit_count").copied().unwrap(), 5.0);
}

#[test]
fn test_distribution_box_template() {
    // 测试配电箱节点模板
    let template = DistributionBoxTemplate;
    
    // 验证模板属性
    assert_eq!(template.node_finder_label(), "配电箱");
    assert_eq!(template.node_category(), "电气节点");
    assert_eq!(template.node_name(), "配电箱");
    
    // 验证类别
    let categories = template.node_finder_categories();
    assert!(categories.contains(&"电气节点"));
    
    // 验证输入输出端口
    let input_types = template.input_types();
    assert_eq!(input_types.len(), 5);
    assert!(input_types.iter().any(|(name, _)| *name == "配电箱名称"));
    assert!(input_types.iter().any(|(name, _)| *name == "所在楼层"));
    assert!(input_types.iter().any(|(name, _)| *name == "回路输入1"));
    
    let output_types = template.output_types();
    assert_eq!(output_types.len(), 4);
    assert!(output_types.iter().any(|(name, _)| *name == "配电箱数据"));
    assert!(output_types.iter().any(|(name, _)| *name == "总电流"));
    assert!(output_types.iter().any(|(name, _)| *name == "总功率"));
    assert!(output_types.iter().any(|(name, _)| *name == "进线电流"));
}

#[test]
fn test_edge_cases() {
    // 测试边界情况
    let mut box_node = DistributionBoxNode::default();
    
    // 1. 空配电箱计算
    box_node.recalculate();
    assert_eq!(box_node.data.total_power, 0.0);
    assert_eq!(box_node.data.total_current, 0.0);
    assert_eq!(box_node.data.incoming_current, 0.0);
    
    // 2. 添加零功率回路
    let zero_power_circuit = CircuitInfo::new(
        "circuit_zero".to_string(),
        "零功率回路".to_string(),
        0.0,
        0.0
    );
    assert!(box_node.add_circuit(zero_power_circuit));
    assert_eq!(box_node.data.circuits.len(), 1);
    assert_eq!(box_node.data.total_power, 0.0);
    
    // 3. 测试极端大功率
    let high_power_circuit = CircuitInfo::new(
        "circuit_high".to_string(),
        "大功率回路".to_string(),
        1000.0, // 1000kW
        2000.0
    );
    assert!(box_node.add_circuit(high_power_circuit));
    box_node.recalculate();
    
    // 验证计算结果的合理性
    assert_eq!(box_node.data.total_power, 1000.0);
    // 总电流应该很大但不是NaN或无穷大
    assert!(box_node.data.total_current > 0.0);
    assert!(box_node.data.total_current.is_finite());
    // 进线电流应该是标准值
    assert!(box_node.data.incoming_current > box_node.data.total_current);
}