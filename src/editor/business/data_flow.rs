use std::collections::{HashSet, HashMap}; use egui_node_graph::{NodeId, Graph}; use crate::core_lib::data_types::{ElectricNodeData, ElectricDataType, ElectricValueType}; use super::circuit_node::CircuitNodeData; use super::distribution_box_node::DistributionBoxNodeData; use super::main_system_node::MainSystemNodeData; 

/// 可更新节点的接口 pub trait UpdatableNode {
    /// 更新节点数据
    fn update(&mut self, cache: &mut HashMap<String, ElectricValueType>) -> HashMap<String, ElectricValueType>;
    
    /// 获取输出值
    fn get_output(&self, key: &str) -> Option<&ElectricValueType>;
}

/// 电力系统图应用中负责数据流向和实时更新的管理器 pub struct DataFlowManager {
    // 更新状态    nodes_to_update: HashSet<NodeId>,
    // 计算缓存    calculation_cache: HashMap<NodeId, HashMap<String, ElectricValueType>>,
}

impl DataFlowManager {
    /// 创建新的数据流向管理器    pub fn new() -> Self {
        Self {
            nodes_to_update: HashSet::new(),
            calculation_cache: HashMap::new(),
        }
    }

    /// 处理节点更新和数据流动    pub fn propagate_updates(&mut self, graph: &mut Graph<ElectricNodeData, ElectricDataType, ElectricValueType>) {
        // 1. 确定需要更新的节点顺序（拓扑排序）        let execution_order = self.perform_topological_sort(graph);

        // 2. 按顺序更新节点        for node_id in execution_order {
            if self.nodes_to_update.contains(&node_id) {
                self.update_node(graph, node_id);

                // 3. 更新受影响的下游节点                self.mark_downstream_nodes_for_update(graph, node_id);
            }
        }

        // 4. 清除更新标志        self.nodes_to_update.clear();
    }

    /// 更新单个节点    pub fn update_node(&mut self, graph: &mut Graph<ElectricNodeData, ElectricDataType, ElectricValueType>, node_id: NodeId) {
        let node = &mut graph.nodes[node_id];

        match &mut node.user_data {
            ElectricNodeData::CircuitNode(circuit) => {
                // 更新配电回路节点                circuit.calculate_current();                circuit.select_components();
            },
            ElectricNodeData::DistributionBoxNode(box_node) => {
                // 收集所有连接到该配电箱的回路节点                let mut connected_circuits = Vec::new();

                for (input_id, connected_output_id) in &graph.connections {
                    let input = &graph.inputs[*input_id];
                    if input.node == node_id {
                        // 找到输出节点                        for (output_node_id, node) in &graph.nodes {
                            if node.outputs.values().any(|&id| id == *connected_output_id) {
                                if let ElectricNodeData::CircuitNode(circuit) = &node.user_data {
                                    connected_circuits.push(circuit.clone());
                                }
                            }
                        }
                    }
                }

                // 更新配电箱数据                let mut circuit_data = connected_circuits.iter().collect::<Vec<_>>();                box_node.calculate_total_power(&connected_circuits);                box_node.auto_number_circuits(&mut circuit_data);                box_node.balance_three_phases(&mut circuit_data);

                // 更新回路数据                for (i, circuit) in connected_circuits.iter().enumerate() {
                    // 简化处理，实际应用中需要维护节点映射                }
            },
            ElectricNodeData::TrunkLineNode(system_node) => {
                // 这里应该处理干线系统图节点的更新逻辑                // 将系统节点数据转换为MainSystemNodeData类型后处理                let mut system_node_data = MainSystemNodeData::default();
                
                // 收集所有连接的配电箱节点                let mut connected_boxes = Vec::new();

                for (input_id, connected_output_id) in &graph.connections {
                    let input = &graph.inputs[*input_id];
                    if input.node == node_id {
                        // 找到输出节点                        for (output_node_id, node) in &graph.nodes {
                            if node.outputs.values().any(|&id| id == *connected_output_id) {
                                if let ElectricNodeData::DistributionBoxNode(box_node) = &node.user_data {
                                    connected_boxes.push(box_node.clone());
                                }
                            }
                        }
                    }
                }

                // 生成系统图                let diagrams = system_node_data.auto_map_distribution_boxes(&connected_boxes);

                // 存储生成的系统图                for diagram in diagrams {
                    // 实际应用中应将系统图关联到节点                }
            },            
            ElectricNodeData::PowerSourceNode(_) => {
                // 更新电源节点            },            
            ElectricNodeData::CalculationNode(_) => {
                // 更新计算节点            },
        }
    }

    /// 标记下游节点需要更新    pub fn mark_downstream_nodes_for_update(&mut self, graph: &Graph<ElectricNodeData, ElectricDataType, ElectricValueType>, node_id: NodeId) {
        // 找到所有依赖此节点的下游节点        for (input_id, connected_output_id) in &graph.connections {
            // 检查是否是当前节点的输出            let node = &graph.nodes[node_id];            if node.outputs.values().any(|&id| id == *connected_output_id) {                // 找到输入节点                let input = &graph.inputs[*input_id];                let downstream_node_id = input.node;

                // 标记为需要更新                self.nodes_to_update.insert(downstream_node_id);

                // 递归标记                self.mark_downstream_nodes_for_update(graph, downstream_node_id);
            }
        }
    }

    /// 执行拓扑排序    pub fn perform_topological_sort(&self, graph: &Graph<ElectricNodeData, ElectricDataType, ElectricValueType>) -> Vec<NodeId> {
        // 实现拓扑排序算法，确保节点按依赖关系顺序执行        let mut visited = HashSet::new();        let mut result = Vec::new();

        // 从无入度的节点开始        for (node_id, _) in &graph.nodes {
            if !visited.contains(node_id) {
                self.dfs_visit(graph, node_id, &mut visited, &mut result);
            }
        }

        result.reverse(); // 反转结果以获得正确的拓扑顺序        result    }

    /// 深度优先遍历辅助函数    fn dfs_visit(&self, graph: &Graph<ElectricNodeData, ElectricDataType, ElectricValueType>, node_id: &NodeId, visited: &mut HashSet<NodeId>, result: &mut Vec<NodeId>) {
        visited.insert(*node_id);

        // 找到所有依赖此节点的下游节点        for (input_id, connected_output_id) in &graph.connections {
            // 检查是否是当前节点的输出            let node = &graph.nodes[*node_id];            if node.outputs.values().any(|&id| id == *connected_output_id) {                // 找到输入节点                let input = &graph.inputs[*input_id];                let downstream_node_id = input.node;

                if !visited.contains(&downstream_node_id) {
                    self.dfs_visit(graph, &downstream_node_id, visited, result);
                }
            }
        }

        result.push(*node_id);    }

    /// 标记节点需要更新    pub fn mark_node_for_update(&mut self, node_id: NodeId) {
        self.nodes_to_update.insert(node_id);    }
    
    /// 标记多个节点需要更新    pub fn mark_nodes_for_update(&mut self, node_ids: impl IntoIterator<Item = NodeId>) {
        for node_id in node_ids {
            self.mark_node_for_update(node_id);        }
    }
    
    /// 检查节点是否需要更新    pub fn needs_update(&self, node_id: NodeId) -> bool {
        self.nodes_to_update.contains(&node_id)    }
    
    /// 清除待更新节点列表    pub fn clear_updates(&mut self) {
        self.nodes_to_update.clear();    }
    
    /// 缓存计算结果    fn cache_calculation_result(&mut self, node_id: NodeId, key: &str, value: ElectricValueType) {
        let node_cache = self.calculation_cache.entry(node_id).or_insert(HashMap::new());        
        node_cache.insert(key.to_string(), value);    }
    
    /// 获取缓存的计算结果    pub fn get_calculation_result(&self, node_id: NodeId, key: &str) -> Option<&ElectricValueType> {
        self.calculation_cache
            .get(&node_id)?            .get(key)    }
    
    /// 获取节点的所有缓存结果    pub fn get_node_cache(&self, node_id: NodeId) -> Option<&HashMap<String, ElectricValueType>> {
        self.calculation_cache.get(&node_id)    }
    
    /// 清除特定节点的缓存    pub fn clear_node_cache(&mut self, node_id: NodeId) {
        self.calculation_cache.remove(&node_id);    }
    
    /// 清除计算缓存    pub fn clear_cache(&mut self) {
        self.calculation_cache.clear();    }
}

impl Default for DataFlowManager {
    fn default() -> Self {
        Self::new()    }
}

// 为CircuitNodeData实现UpdatableNode trait impl UpdatableNode for CircuitNodeData {
    fn update(&mut self, cache: &mut HashMap<String, ElectricValueType>) -> HashMap<String, ElectricValueType> {
        // 执行计算        self.calculate_current();        self.select_components();        
        // 更新缓存        let mut results = HashMap::new();        
        // 将计算结果存储到缓存        cache.insert("power".to_string(), ElectricValueType::Float(self.power));        cache.insert("current".to_string(), ElectricValueType::Float(self.calculated_current));        cache.insert("power_factor".to_string(), ElectricValueType::Float(self.power_factor));        
        // 复制一份结果返回        results.insert("power".to_string(), ElectricValueType::Float(self.power));        results.insert("current".to_string(), ElectricValueType::Float(self.calculated_current));        results.insert("power_factor".to_string(), ElectricValueType::Float(self.power_factor));        
        results    }
    
    fn get_output(&self, key: &str) -> Option<&ElectricValueType> {
        // 这里简化处理，实际应从缓存中获取        match key {
            "power" => Some(&ElectricValueType::Float(self.power)),            "current" => Some(&ElectricValueType::Float(self.calculated_current)),            "power_factor" => Some(&ElectricValueType::Float(self.power_factor)),            _ => None,
        }
    }
}

// 为DistributionBoxNodeData实现UpdatableNode trait impl UpdatableNode for DistributionBoxNodeData {
    fn update(&mut self, cache: &mut HashMap<String, ElectricValueType>) -> HashMap<String, ElectricValueType> {
        // 执行计算        self.calculate_total_power(&[]); // 简化处理，实际应有连接的回路        self.balance_three_phases(&mut []); // 简化处理        
        // 更新缓存        let mut results = HashMap::new();        
        // 将计算结果存储到缓存        cache.insert("total_power".to_string(), ElectricValueType::Float(self.total_power));        cache.insert("total_current".to_string(), ElectricValueType::Float(self.calculated_current));        
        // 复制一份结果返回        results.insert("total_power".to_string(), ElectricValueType::Float(self.total_power));        results.insert("total_current".to_string(), ElectricValueType::Float(self.calculated_current));        
        results    }
    
    fn get_output(&self, key: &str) -> Option<&ElectricValueType> {
        // 这里简化处理，实际应从缓存中获取        match key {
            "total_power" => Some(&ElectricValueType::Float(self.total_power)),            "total_current" => Some(&ElectricValueType::Float(self.calculated_current)),            _ => None,
        }
    }
}

// 为MainSystemNodeData实现UpdatableNode trait impl UpdatableNode for MainSystemNodeData {
    fn update(&mut self, cache: &mut HashMap<String, ElectricValueType>) -> HashMap<String, ElectricValueType> {
        // 执行自动映射        // 简化处理，实际应有连接的配电箱        
        // 更新缓存        let mut results = HashMap::new();        
        // 将计算结果存储到缓存        cache.insert("system_info".to_string(), ElectricValueType::String(self.name.clone()));        
        // 复制一份结果返回        results.insert("system_info".to_string(), ElectricValueType::String(self.name.clone()));        
        results    }
    
    fn get_output(&self, key: &str) -> Option<&ElectricValueType> {
        // 这里简化处理，实际应从缓存中获取        match key {
            "system_info" => Some(&ElectricValueType::String(self.name.clone())),            _ => None,
        }
    }
}