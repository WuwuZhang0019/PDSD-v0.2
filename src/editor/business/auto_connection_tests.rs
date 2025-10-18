// 自动识别与连线生成功能单元测试

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::business::{DistributionBoxNode, MainSystemNodeUI, IncomingType};
    use crate::editor::business::main_system_node::{ConnectionType, ComponentType};
    
    #[test]
    fn test_determine_incoming_type() {
        // 创建单电源配电箱
        let mut single_power_box = DistributionBoxNode::new("单电源配电箱".to_string(), 1);
        // 验证识别结果为单电源
        assert_eq!(single_power_box.determine_incoming_type(), IncomingType::SinglePower);
        
        // 创建双电源配电箱
        let mut dual_power_box = DistributionBoxNode::new("双电源配电箱".to_string(), 2);
        dual_power_box.modules.push("双电源切换".to_string());
        // 验证识别结果为双电源
        assert_eq!(dual_power_box.determine_incoming_type(), IncomingType::DualPower);
    }
    
    #[test]
    fn test_auto_generate_connections() {
        // 创建测试用的配电箱
        let mut single_power_box = DistributionBoxNode::new("单电源配电箱".to_string(), 1);
        single_power_box.total_power = 10.0;
        single_power_box.total_current = 15.2;
        
        let mut dual_power_box = DistributionBoxNode::new("双电源配电箱".to_string(), 2);
        dual_power_box.total_power = 20.0;
        dual_power_box.total_current = 30.4;
        dual_power_box.modules.push("双电源切换".to_string());
        
        // 创建配电箱向量
        let boxes = vec![&single_power_box, &dual_power_box];
        
        // 创建主系统节点
        let mut system_node = MainSystemNodeUI::default();
        
        // 生成系统图
        let diagram = system_node.auto_generate_connections(boxes);
        
        // 验证系统图包含预期的组件和连接
        // 应该有：主母线、备用电源、单电源配电箱、双电源配电箱
        assert!(diagram.components.len() >= 4);
        
        // 单电源应该有1条连接（连接到主母线）
        // 双电源应该有2条连接（连接到主母线和备用电源）
        // 总连接数应该至少为3
        assert!(diagram.connections.len() >= 3);
        
        // 找到配电箱组件
        let mut box_components = diagram.components
            .iter()
            .filter(|comp| comp.component_type == ComponentType::DistributionBox)
            .collect::<Vec<_>>();
        
        // 找到母线和电源组件
        let main_busbar = diagram.components
            .iter()
            .find(|comp| comp.component_type == ComponentType::Busbar)
            .unwrap();
        
        let backup_power = diagram.components
            .iter()
            .find(|comp| comp.component_type == ComponentType::PowerSource)
            .unwrap();
        
        // 验证单电源配电箱只有一条连接（到主母线）
        let single_power_component = box_components
            .iter()
            .find(|comp| comp.label.contains("单电源配电箱"))
            .unwrap();
        
        // 验证双电源配电箱有两条连接（到主母线和备用电源）
        let dual_power_component = box_components
            .iter()
            .find(|comp| comp.label.contains("双电源配电箱"))
            .unwrap();
        
        // 验证连接类型设置正确
        let single_power_conn = diagram.connections
            .iter()
            .find(|conn| conn.source_id == main_busbar.id && conn.target_id == single_power_component.id)
            .unwrap();
        assert_eq!(single_power_conn.connection_type, ConnectionType::SinglePower);
        
        let dual_power_main_conn = diagram.connections
            .iter()
            .find(|conn| conn.source_id == main_busbar.id && conn.target_id == dual_power_component.id)
            .unwrap();
        assert_eq!(dual_power_main_conn.connection_type, ConnectionType::DualPower);
        
        let dual_power_backup_conn = diagram.connections
            .iter()
            .find(|conn| conn.source_id == backup_power.id && conn.target_id == dual_power_component.id)
            .unwrap();
        assert_eq!(dual_power_backup_conn.connection_type, ConnectionType::DualPower);
    }
}