//! 自动识别与连线生成功能测试
//! 
//! 本测试文件用于验证配电箱进线类型自动识别和干线系统图自动连线生成功能。

use crate::editor::business::distribution_box_parameters::{DistributionBoxNode, IncomingType};
use crate::editor::business::main_system_node::{MainSystemNodeUI, MainSystemType, ConnectionType, ComponentType};

/// 测试配电箱进线类型自动识别功能
#[test]
fn test_determine_incoming_type() {
    // 测试单电源配电箱
    let mut single_power_box = DistributionBoxNode::new("普通配电箱1号".to_string(), 1);
    single_power_box.modules.push("断路器".to_string());
    single_power_box.modules.push("浪涌保护器".to_string());
    
    // 测试双电源配电箱
    let mut dual_power_box = DistributionBoxNode::new("消防配电箱1号".to_string(), 1);
    dual_power_box.modules.push("双电源切换".to_string());
    dual_power_box.modules.push("断路器".to_string());
    
    // 测试带英文标识的双电源配电箱
    let mut dual_power_box_en = DistributionBoxNode::new("Fire Distribution Box".to_string(), 2);
    dual_power_box_en.modules.push("dual power transfer switch".to_string());
    
    // 验证识别结果
    assert_eq!(single_power_box.determine_incoming_type(), IncomingType::SinglePower);
    assert_eq!(dual_power_box.determine_incoming_type(), IncomingType::DualPower);
    assert_eq!(dual_power_box_en.determine_incoming_type(), IncomingType::DualPower);
    
    println!("✅ 配电箱进线类型自动识别测试通过！");
}

/// 测试自动连线生成功能
#[test]
fn test_auto_generate_connections() {
    // 创建干线系统图节点
    let mut main_system_node = MainSystemNodeUI::default();
    main_system_node.data.systems = vec![MainSystemType::PowerDistribution];
    main_system_node.data.auto_layout = true;
    
    // 创建多个配电箱（不同楼层、不同进线类型）
    let mut box1 = DistributionBoxNode::new("一层普通配电箱".to_string(), 1);
    box1.total_power = 50.0;
    box1.total_current = 75.8;
    box1.modules.push("断路器".to_string());
    
    let mut box2 = DistributionBoxNode::new("二层消防配电箱".to_string(), 2);
    box2.total_power = 30.0;
    box2.total_current = 45.5;
    box2.modules.push("双电源切换".to_string());
    
    let mut box3 = DistributionBoxNode::new("三层普通配电箱".to_string(), 3);
    box3.total_power = 40.0;
    box3.total_current = 60.6;
    box3.modules.push("断路器".to_string());
    
    // 收集配电箱引用
    let boxes = vec![&box1, &box2, &box3];
    
    // 调用自动连线生成功能
    let diagrams = main_system_node.auto_map_distribution_boxes(&boxes);
    
    // 验证生成的系统图
    assert!(!diagrams.is_empty(), "应该生成至少一个系统图");
    let diagram = &diagrams[0];
    
    // 验证组件数量（3个配电箱 + 1个母线 + 1个备用电源 = 5个组件）
    assert_eq!(diagram.components.len(), 5, "组件数量不正确");
    
    // 验证连接数量（单电源连接:2 + 双电源连接:2 = 4个连接）
    assert_eq!(diagram.connections.len(), 4, "连接数量不正确");
    
    // 验证双电源配电箱的连接类型
    let fire_box_component = diagram.components.iter()
        .find(|c| c.component_type == ComponentType::DistributionBox && c.label.contains("二层"))
        .unwrap();
    
    // 检查双电源配电箱有两个连接
    let dual_power_connections = diagram.connections.iter()
        .filter(|conn| conn.to == fire_box_component.id && conn.connection_type == ConnectionType::DualPower)
        .count();
    
    assert_eq!(dual_power_connections, 2, "双电源配电箱应该有两个连接");
    
    println!("✅ 自动连线生成功能测试通过！");
    println!("生成了 {} 个系统图，包含 {} 个组件和 {} 个连接", 
             diagrams.len(), diagram.components.len(), diagram.connections.len());
}