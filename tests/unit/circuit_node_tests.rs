use std::collections::HashMap;

// 在测试文件中使用正确的导入方式
use power_distribution_system_diagram::editor::business::{CircuitNode, CircuitParameters, CircuitCalculator, VoltageType};
use power_distribution_system_diagram::core_lib::data_types::{CircuitNodeProperties, CircuitType};

#[test]
fn test_circuit_parameters_validation() {
    // 测试有效参数
    let valid_params = CircuitParameters::new(
        "测试回路".to_string(),
        5.0,   // 5kW
        0.8,   // 需要系数
        0.85,  // 功率因数
        VoltageType::SinglePhase
    );
    
    assert!(valid_params.is_valid());
    assert!(valid_params.validation_errors().is_empty());
    
    // 测试无效参数 - 负功率
    let invalid_power_params = CircuitParameters::new(
        "测试回路".to_string(),
        -1.0,  // 无效功率
        0.8,
        0.85,
        VoltageType::SinglePhase
    );
    
    assert!(!invalid_power_params.is_valid());
    assert!(!invalid_power_params.validation_errors().is_empty());
    assert!(invalid_power_params.validation_errors().contains(&"额定功率必须大于0".to_string()));
    
    // 测试无效参数 - 超出范围的需要系数
    let invalid_kx_params = CircuitParameters::new(
        "测试回路".to_string(),
        5.0,
        1.5,   // 无效需要系数
        0.85,
        VoltageType::SinglePhase
    );
    
    assert!(!invalid_kx_params.is_valid());
    assert!(!invalid_kx_params.validation_errors().is_empty());
    assert!(invalid_kx_params.validation_errors().contains(&"需要系数必须在0-1之间".to_string()));
    
    // 测试无效参数 - 超出范围的功率因数
    let invalid_cos_params = CircuitParameters::new(
        "测试回路".to_string(),
        5.0,
        0.8,
        1.5,   // 无效功率因数
        VoltageType::SinglePhase
    );
    
    assert!(!invalid_cos_params.is_valid());
    assert!(!invalid_cos_params.validation_errors().is_empty());
    assert!(invalid_cos_params.validation_errors().contains(&"功率因数必须在0-1之间".to_string()));
}

#[test]
fn test_circuit_calculator() {
    // 测试单相信路计算
    let single_phase_params = CircuitParameters::new(
        "单相信路".to_string(),
        1.0,   // 1kW
        0.8,   // 需要系数
        0.85,  // 功率因数
        VoltageType::SinglePhase
    );
    
    let single_phase_result = CircuitCalculator::calculate_circuit_current(&single_phase_params).unwrap();
    
    // 验证计算结果（1kW * 0.8 / 0.22kV / 0.85 ≈ 5.24A）
    assert!(single_phase_result.ijs > 5.2 && single_phase_result.ijs < 5.3);
    assert_eq!(single_phase_result.voltage, 0.22);
    
    // 测试三相信路计算
    let three_phase_params = CircuitParameters::new(
        "三相信路".to_string(),
        10.0,  // 10kW
        0.8,   // 需要系数
        0.85,  // 功率因数
        VoltageType::ThreePhase
    );
    
    let three_phase_result = CircuitCalculator::calculate_circuit_current(&three_phase_params).unwrap();
    
    // 验证计算结果（10kW * 0.8 / 0.38kV / 0.85 / √3 ≈ 17.86A）
    assert!(three_phase_result.ijs > 17.8 && three_phase_result.ijs < 18.0);
    assert_eq!(three_phase_result.voltage, 0.38);
    
    // 测试计算错误 - 无效参数
    let invalid_params = CircuitParameters::new(
        "无效回路".to_string(),
        -1.0,  // 无效功率
        0.8,
        0.85,
        VoltageType::SinglePhase
    );
    
    // 验证计算失败
    let result = CircuitCalculator::calculate_circuit_current(&invalid_params);
    assert!(result.is_err());
    
    // 测试估算功能
    let single_phase_estimate = CircuitCalculator::estimate_circuit_current(1.0, VoltageType::SinglePhase);
    assert!(single_phase_estimate > 5.2 && single_phase_estimate < 5.3); // 约5.26A
    
    let three_phase_estimate = CircuitCalculator::estimate_circuit_current(10.0, VoltageType::ThreePhase);
    assert!(three_phase_estimate > 17.8 && three_phase_estimate < 18.0); // 约17.86A
}

#[test]
fn test_circuit_node_validate() {
    // 创建有效节点
    let mut node = CircuitNode::new_single_phase("测试回路", 5.0);
    node.validate();
    
    // 验证有效节点没有错误
    assert!(node.errors.is_empty());
    
    // 测试无效功率
    node.parameters.pe = -1.0;
    node.validate();
    assert!(!node.errors.is_empty());
    assert!(node.errors.contains(&"功率必须大于0".to_string()));
    
    // 测试无效需求系数
    node.parameters.pe = 5.0; // 恢复有效功率
    node.parameters.kx = 1.5;
    node.validate();
    assert!(!node.errors.is_empty());
    assert!(node.errors.contains(&"需求系数必须在0和1之间".to_string()));
    
    // 测试无效功率因数
    node.parameters.kx = 0.8; // 恢复有效需求系数
    node.parameters.cos = 1.5;
    node.validate();
    assert!(!node.errors.is_empty());
    assert!(node.errors.contains(&"功率因数必须在0和1之间".to_string()));
    
    // 测试空名称
    node.parameters.cos = 0.8; // 恢复有效功率因数
    node.parameters.name = "".to_string();
    node.validate();
    assert!(!node.errors.is_empty());
    assert!(node.errors.contains(&"回路名称不能为空".to_string()));
}

#[test]
fn test_circuit_node_properties_sync() {
    // 创建单相信路节点
    let mut node = CircuitNode::new_single_phase("测试回路", 5.0);
    
    // 验证属性初始值
    assert_eq!(node.properties.power, 5.0);
    assert_eq!(node.properties.circuit_type, CircuitType::SinglePhase);
    assert_eq!(node.properties.voltage, 220.0);
    
    // 更新参数并验证同步
    node.parameters.pe = 10.0;
    node.parameters.kx = 0.9;
    node.parameters.cos = 0.95;
    node.sync_parameters_to_properties();
    
    assert_eq!(node.properties.power, 10.0);
    assert_eq!(node.properties.demand_factor, 0.9);
    assert_eq!(node.properties.power_factor, 0.95);
    
    // 测试回路类型切换
    node.set_circuit_type(CircuitType::ThreePhase);
    assert_eq!(node.parameters.voltage_type, VoltageType::ThreePhase);
    assert_eq!(node.properties.circuit_type, CircuitType::ThreePhase);
    assert_eq!(node.properties.voltage, 380.0);
    
    // 切回单相信路
    node.set_circuit_type(CircuitType::SinglePhase);
    assert_eq!(node.parameters.voltage_type, VoltageType::SinglePhase);
    assert_eq!(node.properties.circuit_type, CircuitType::SinglePhase);
    assert_eq!(node.properties.voltage, 220.0);
}
    let three_phase_estimate = CircuitCalculator::estimate_circuit_current(10.0, VoltageType::ThreePhase);
    
    assert!(single_phase_estimate > 0.0);
    assert!(three_phase_estimate > 0.0);
}

#[test]
fn test_circuit_node_functionality() {
    // 测试节点创建
    let mut node = CircuitNode::default();
    
    assert_eq!(node.parameters.name, "新回路");
    assert_eq!(node.parameters.pe, 1.0);
    assert_eq!(node.parameters.kx, 0.8);
    assert_eq!(node.parameters.cos, 0.85);
    assert_eq!(node.parameters.voltage_type, VoltageType::SinglePhase);
    assert!(node.result.is_none());
    assert!(node.errors.is_empty());
    
    // 测试参数更新
    let new_params = CircuitParameters::new(
        "更新后的回路".to_string(),
        5.0,
        0.7,
        0.9,
        VoltageType::ThreePhase
    );
    
    node.update_parameters(new_params.clone());
    
    assert_eq!(node.parameters.name, new_params.name);
    assert_eq!(node.parameters.pe, new_params.pe);
    assert_eq!(node.parameters.kx, new_params.kx);
    assert_eq!(node.parameters.cos, new_params.cos);
    assert_eq!(node.parameters.voltage_type, new_params.voltage_type);
    assert!(node.errors.is_empty());
    assert!(node.result.is_some()); // 参数更新后应该自动计算
    
    // 测试手动重新计算
    node.recalculate();
    assert!(node.result.is_some());
    
    // 测试获取回路数据映射
    let data_map = node.to_circuit_data_map();
    
    assert_eq!(data_map.get("pe").unwrap(), &5.0);
    assert_eq!(data_map.get("kx").unwrap(), &0.7);
    assert_eq!(data_map.get("cos").unwrap(), &0.9);
    assert_eq!(data_map.get("voltage_type").unwrap(), &3.0); // 三相为3.0
    assert!(data_map.contains_key("ijs"));
    assert!(data_map.contains_key("voltage"));
    
    // 测试无效参数处理
    let invalid_params = CircuitParameters::new(
        "无效回路".to_string(),
        -5.0,  // 无效功率
        0.7,
        0.9,
        VoltageType::ThreePhase
    );
    
    node.update_parameters(invalid_params);
    
    assert!(!node.errors.is_empty());
    assert!(node.result.is_none()); // 无效参数不应有计算结果
}

#[test]
fn test_circuit_node_title_and_formatting() {
    let mut node = CircuitNode::default();
    
    // 测试默认标题
    assert_eq!(node.get_title(), "回路: 新回路");
    
    // 测试更新名称后的标题
    let new_params = CircuitParameters::new(
        "照明回路".to_string(),
        1.0,
        0.8,
        0.85,
        VoltageType::SinglePhase
    );
    
    node.update_parameters(new_params);
    assert_eq!(node.get_title(), "回路: 照明回路");
    
    // 测试结果格式化
    if let Some(result) = &node.result {
        let formatted_ijs = result.formatted_ijs();
        let formatted_voltage = result.formatted_voltage();
        
        // 验证格式是否正确
        assert!(formatted_ijs.contains("A"));
        assert!(formatted_voltage.contains("kV"));
    }
}

#[test]
fn test_voltage_type_functions() {
    // 测试单电压类型
    let single_phase = VoltageType::SinglePhase;
    assert_eq!(single_phase.voltage_value(), 0.22);
    assert_eq!(single_phase.display_name(), "单相");
    
    // 测试三相电压类型
    let three_phase = VoltageType::ThreePhase;
    assert_eq!(three_phase.voltage_value(), 0.38);
    assert_eq!(three_phase.display_name(), "三相");
    
    // 测试所有类型获取
    let all_types = VoltageType::all_types();
    assert_eq!(all_types.len(), 2);
    assert!(all_types.contains(&VoltageType::SinglePhase));
    assert!(all_types.contains(&VoltageType::ThreePhase));
}

#[test]
fn test_multiple_circuits_calculation() {
    // 创建多个回路参数
    let circuits = vec![
        CircuitParameters::new("回路1".to_string(), 1.0, 0.8, 0.85, VoltageType::SinglePhase),
        CircuitParameters::new("回路2".to_string(), 2.0, 0.8, 0.85, VoltageType::SinglePhase),
        CircuitParameters::new("回路3".to_string(), 5.0, 0.8, 0.85, VoltageType::ThreePhase),
        CircuitParameters::new("无效回路".to_string(), -1.0, 0.8, 0.85, VoltageType::SinglePhase),
    ];
    
    // 批量计算
    let results = CircuitCalculator::calculate_multiple_circuits(&circuits);
    
    // 验证结果数量
    assert_eq!(results.len(), circuits.len());
    
    // 验证有效结果
    assert!(results[0].is_ok());
    assert!(results[1].is_ok());
    assert!(results[2].is_ok());
    assert!(results[3].is_err()); // 无效回路应该返回错误
    
    // 验证具体计算值
    if let Ok(result1) = &results[0] {
        assert!(result1.ijs > 0.0);
    }
    
    if let Ok(result2) = &results[1] {
        // 回路2的功率是回路1的2倍，电流也应该大约是2倍
        if let Ok(result1) = &results[0] {
            assert!(result2.ijs > result1.ijs * 1.9 && result2.ijs < result1.ijs * 2.1);
        }
    }
}