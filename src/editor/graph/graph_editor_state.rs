use egui_node_graph::{Graph, GraphEditorState, NodeId, UiNodeTemplate};
use crate::core_lib::data_types::{ElectricDataType, ElectricValueType};
use crate::editor::business::node_data_transfer::{PowerGraphNode, PowerGraphState};
use crate::editor::business::{CircuitNodeTemplate, DistributionBoxTemplate};
use std::collections::HashMap;
use std::sync::{Arc, Mutex}; 

/// 电力配电系统图编辑器状态
pub struct PowerDistributionGraphEditorState {
    /// 节点图数据
    pub graph: Graph<PowerGraphNode, ElectricDataType, ElectricValueType>,
    /// 节点编辑器UI状态
    pub editor_state: GraphEditorState<PowerGraphNode, ElectricDataType, ElectricValueType, CircuitNodeTemplate, PowerGraphState>,
    /// 电力图专用状态
    pub power_graph_state: PowerGraphState,
    /// 节点模板集合
    pub node_templates: HashMap<String, Box<dyn UiNodeTemplate<PowerGraphNode, ElectricDataType, ElectricValueType>>>,
    /// 当前选中的节点ID
    pub selected_node_id: Option<NodeId>,
} 

impl Default for PowerDistributionGraphEditorState {
    fn default() -> Self {
        let mut state = Self {
            graph: Graph::default(),
            editor_state: GraphEditorState::default(),
            power_graph_state: PowerGraphState::default(),
            node_templates: HashMap::new(),
            selected_node_id: None,
        };
        
        // 注册节点模板
        state.register_node_templates();
        
        state
    }
}

impl PowerDistributionGraphEditorState {
    /// 创建新的图编辑器状态
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 注册节点模板
    pub fn register_node_templates(&mut self) {
        // 注意：这里应该注册与PowerGraphNode兼容的模板
        // 目前的实现需要修改，暂时注释掉以避免编译错误
        // TODO: 实现正确的节点模板注册机制
        // 注册回路节点模板
        // let circuit_template = CircuitNodeTemplate::new();
        // self.node_templates.insert("回路".to_string(), Box::new(circuit_template));
        
        // 注册配电箱节点模板
        // let distribution_box_template = DistributionBoxTemplate::new();
        // self.node_templates.insert("配电箱".to_string(), Box::new(distribution_box_template));
    }
    
    /// 执行节点数据传播和更新
    pub fn propagate_updates(&mut self) {
        // 1. 确定需要更新的节点顺序（拓扑排序）
        let execution_order = self.power_graph_state.perform_topological_sort(&self.graph);
        
        // 2. 按顺序更新节点
        for node_id in execution_order {
            if self.power_graph_state.nodes_to_update.contains(&node_id) {
                self.update_node(node_id);
                
                // 3. 更新受影响的下游节点
                self.power_graph_state.mark_downstream_nodes_for_update(&self.graph, node_id);
            }
        }
        
        // 4. 清除更新标志
        self.power_graph_state.nodes_to_update.clear();
    }
    
    /// 更新单个节点
    pub fn update_node(&mut self, node_id: NodeId) {
        if let Some(node) = self.graph.nodes.get_mut(node_id) {
            // 收集所有输入数据
            let mut inputs = HashMap::new();
            
            for (input_id, connected_output_id) in &self.graph.connections {
                let input = &self.graph.inputs[*input_id];
                if input.node == node_id {
                    // 获取连接的输出节点及其值
                    if let Some(output_node_id) = self.graph.find_node_by_output_id(*connected_output_id) {
                        if let Some(output_node) = self.graph.nodes.get(output_node_id) {
                            let output_values = output_node.output_values();
                            if let Some(value) = output_values.get(connected_output_id) {
                                inputs.insert(*input_id, value.clone());
                            }
                        }
                    }
                }
            }
            
            // 处理节点数据
            node.process(inputs);
            
            // 存储计算结果到缓存
            if !self.power_graph_state.calculation_cache.contains_key(&node_id) {
                self.power_graph_state.calculation_cache.insert(node_id, HashMap::new());
            }
            
            // 更新缓存
            if let Some(cache) = self.power_graph_state.calculation_cache.get_mut(&node_id) {
                let output_values = node.output_values();
                for (output_id, value) in output_values {
                    cache.insert(format!("output_{}", output_id.0), value);
                }
            }
        }
    }
    
    /// 添加节点并标记为需要更新
    pub fn add_node(&mut self, node: PowerGraphNode) -> NodeId {
        let node_id = self.graph.add_node(node);
        self.power_graph_state.mark_node_for_update(node_id);
        node_id
    }
    
    /// 创建连接并触发更新
    pub fn connect_nodes(&mut self, output_id: OutputId, input_id: InputId) -> bool {
        if self.graph.connect(output_id, input_id) {
            // 找到输入节点并标记为需要更新
            if let Some(input_node_id) = self.graph.inputs.get(input_id).map(|input| input.node) {
                self.power_graph_state.mark_node_for_update(input_node_id);
                self.propagate_updates();
            }
            true
        } else {
            false
        }
    }
    
    /// 断开连接并触发更新
    pub fn disconnect_nodes(&mut self, input_id: InputId) -> bool {
        if let Some(connected_output_id) = self.graph.connections.get(&input_id) {
            if self.graph.disconnect(input_id) {
                // 找到输入节点并标记为需要更新
                if let Some(input_node_id) = self.graph.inputs.get(input_id).map(|input| input.node) {
                    self.power_graph_state.mark_node_for_update(input_node_id);
                    self.propagate_updates();
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}