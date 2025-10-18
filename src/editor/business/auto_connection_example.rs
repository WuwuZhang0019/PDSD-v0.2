// 自动识别与连线生成功能示例
// 本文件展示了如何使用DistributionBoxNode的determine_incoming_type方法和MainSystemNodeUI的auto_generate_connections方法

use crate::editor::business::{DistributionBoxNode, MainSystemNodeUI, MainSystemResponse};

pub fn demonstrate_auto_connections() {
    // 创建一些配电箱示例
    let mut single_power_box = DistributionBoxNode::new("照明配电箱".to_string(), 1);
    single_power_box.total_power = 15.0;  // 15kW
    single_power_box.total_current = 22.8; // 22.8A
    // 单电源配电箱不添加双电源切换模块
    
    let mut dual_power_box = DistributionBoxNode::new("消防配电箱".to_string(), 2);
    dual_power_box.total_power = 30.0;    // 30kW
    dual_power_box.total_current = 45.6;  // 45.6A
    // 双电源配电箱添加双电源切换模块
    dual_power_box.modules.push("双电源切换".to_string());
    
    let mut energy_monitoring_box = DistributionBoxNode::new("能耗监测配电箱".to_string(), 3);
    energy_monitoring_box.total_power = 20.0;  // 20kW
    energy_monitoring_box.total_current = 30.4; // 30.4A
    energy_monitoring_box.modules.push("能耗监测".to_string());
    
    let mut fire_monitoring_box = DistributionBoxNode::new("电气火灾监控配电箱".to_string(), 4);
    fire_monitoring_box.total_power = 10.0;  // 10kW
    fire_monitoring_box.total_current = 15.2; // 15.2A
    fire_monitoring_box.modules.push("电气火灾监控".to_string());
    
    let mut fire_power_box = DistributionBoxNode::new("消防电源监测配电箱".to_string(), 5);
    fire_power_box.total_power = 25.0;  // 25kW
    fire_power_box.total_current = 38.0; // 38.0A
    fire_power_box.modules.push("消防电源监测".to_string());
    
    // 演示determine_incoming_type方法
    println!("配电箱进线类型判断:");
    println!("照明配电箱: {:?}", single_power_box.determine_incoming_type());
    println!("消防配电箱: {:?}", dual_power_box.determine_incoming_type());
    
    // 创建所有配电箱的向量
    let boxes = vec![
        &single_power_box,
        &dual_power_box,
        &energy_monitoring_box,
        &fire_monitoring_box,
        &fire_power_box
    ];
    
    // 演示auto_map_distribution_boxes方法
    println!("\n生成系统图:");
    let mut system_node = MainSystemNodeUI::default();
    
    // 选择所有类型的系统图
    system_node.data.systems.push(crate::editor::business::MainSystemType::PowerDistribution);
    system_node.data.systems.push(crate::editor::business::MainSystemType::EnergyMonitoring);
    system_node.data.systems.push(crate::editor::business::MainSystemType::ElectricalFireMonitoring);
    system_node.data.systems.push(crate::editor::business::MainSystemType::FirePowerMonitoring);
    
    // 启用自动布局
    system_node.data.auto_layout = true;
    
    // 生成系统图
    let diagrams = system_node.auto_map_distribution_boxes(&boxes);
    
    println!("\n已生成 {} 个系统图:", diagrams.len());
    for (i, diagram) in diagrams.iter().enumerate() {
        println!("{}. {}", i + 1, diagram.name);
        println!("   包含 {} 个组件和 {} 条连接", diagram.components.len(), diagram.connections.len());
    }
    
    println!("\n自动识别与连线生成功能演示完成！");
}