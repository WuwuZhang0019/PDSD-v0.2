use super::super::{DataFlowManager, CircuitNodeData, DistributionBoxNodeData};
use crate::core_lib::data_types::{ElectricNodeData, ElectricDataType, ElectricValueType};
use egui_node_graph::{Graph, NodeId};

#[test]
fn test_data_flow_manager_creation() {
    // 测试创建数据流向管理器
    let manager = DataFlowManager::new();
    // 验证初始状态
    assert_eq!(manager.nodes_to_update.len(), 0);
    assert_eq!(manager.calculation_cache.len(), 0);
}

#[test]
fn test_mark_node_for_update() {
    let mut manager = DataFlowManager::new();
    let node_id = NodeId(1);
    
    // 标记节点需要更新
    manager.mark_node_for_update(node_id);
    
    // 验证节点被标记
    assert!(manager.nodes_to_update.contains(&node_id));
    
    // 清除更新标记
    manager.clear_updates();
    assert!(!manager.nodes_to_update.contains(&node_id));
}

#[test]
fn test_cache_management() {
    let mut manager = DataFlowManager::new();
    let node_id = NodeId(1);
    let value = ElectricValueType::Float(42.0);
    
    // 缓存计算结果
    manager.cache_calculation_result(node_id, "test_value", value.clone());
    
    // 获取缓存结果
    let cached_value = manager.get_calculation_result(node_id, "test_value");
    assert!(cached_value.is_some());
    assert_eq!(cached_value.unwrap(), &value);
    
    // 清除特定节点缓存
    manager.clear_node_cache(node_id);
    assert!(manager.get_calculation_result(node_id, "test_value").is_none());
    
    // 缓存多个结果
    manager.cache_calculation_result(node_id, "value1", ElectricValueType::Float(10.0));
    manager.cache_calculation_result(node_id, "value2", ElectricValueType::Float(20.0));
    
    // 获取所有缓存
    let node_cache = manager.get_node_cache(node_id);
    assert!(node_cache.is_some());
    assert_eq!(node_cache.unwrap().len(), 2);
    
    // 清除所有缓存
    manager.clear_cache();
    assert!(manager.get_node_cache(node_id).is_none());
}

#[test]
fn test_topological_sort() {
    // 创建一个简单的图进行测试
    let mut graph = Graph::<ElectricNodeData, ElectricDataType, ElectricValueType>::default();
    
    // 创建一些测试节点
    let node1 = graph.add_node(
        "节点1".to_string(),
        ElectricNodeData::CircuitNode(CircuitNodeData::default()),
        |_, _| {},
    );
    
    let node2 = graph.add_node(
        "节点2".to_string(),
        ElectricNodeData::CircuitNode(CircuitNodeData::default()),
        |_, _| {},
    );
    
    let node3 = graph.add_node(
        "节点3".to_string(),
        ElectricNodeData::DistributionBoxNode(DistributionBoxNodeData::default()),
        |_, _| {},
    );
    
    let manager = DataFlowManager::new();
    
    // 执行拓扑排序
    let sorted_nodes = manager.perform_topological_sort(&graph);
    
    // 验证排序结果包含所有节点
    assert_eq!(sorted_nodes.len(), 3);
    assert!(sorted_nodes.contains(&node1));
    assert!(sorted_nodes.contains(&node2));
    assert!(sorted_nodes.contains(&node3));
}

#[test]
fn test_propagate_updates() {
    // 创建一个简单的测试图
    let mut graph = Graph::<ElectricNodeData, ElectricDataType, ElectricValueType>::default();
    
    // 创建测试节点
    let node1 = graph.add_node(
        "节点1".to_string(),
        ElectricNodeData::CircuitNode(CircuitNodeData::default()),
        |_, _| {},
    );
    
    let mut manager = DataFlowManager::new();
    
    // 标记节点需要更新
    manager.mark_node_for_update(node1);
    
    // 执行更新传播
    manager.propagate_updates(&mut graph);
    
    // 验证更新标记已清除
    assert!(!manager.needs_update(node1));
}

#[test]
fn test_updatable_node_trait() {
    // 测试CircuitNodeData的UpdatableNode实现
    let mut circuit = CircuitNodeData::default();
    let mut cache = std::collections::HashMap::new();
    
    // 设置一些测试值
    circuit.power = 10.0;
    circuit.power_factor = 0.8;
    
    // 执行更新
    let results = circuit.update(&mut cache);
    
    // 验证结果
    assert!(!results.is_empty());
    assert!(cache.contains_key("power"));
    assert!(cache.contains_key("current"));
    
    // 测试获取输出
    let power_output = circuit.get_output("power");
    assert!(power_output.is_some());
    if let Some(ElectricValueType::Float(power)) = power_output {
        assert_eq!(*power, 10.0);
    }
    
    // 测试不存在的输出
    let invalid_output = circuit.get_output("invalid_key");
    assert!(invalid_output.is_none());
}