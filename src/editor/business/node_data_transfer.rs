use crate::core_lib::data_types::{ElectricDataType, ElectricNodeData, ElectricValueType};
use egui_node_graph::{DataTypeTrait, Graph, NodeDataTrait, NodeId, OutputId, InputId, NodeResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use rand;

/// 节点数据接口实现，用于支持egui_node_graph框架
pub struct PowerGraphNode {
    /// 节点唯一标识
    pub id: String,
    /// 节点数据
    pub data: ElectricNodeData,
    /// 计算缓存
    pub calculation_cache: HashMap<String, ElectricValueType>,
}

impl Default for PowerGraphNode {
    fn default() -> Self {
        Self {
            id: format!("node_{}", rand::random::<u32>()),
            data: ElectricNodeData::CircuitNode(Default::default()),
            calculation_cache: HashMap::new(),
        }
    }
}

impl Debug for PowerGraphNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PowerGraphNode(id: {}, type: {})
", self.id, self.data.get_type_name())
    }
}

impl Clone for PowerGraphNode {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            data: self.data.clone(),
            calculation_cache: self.calculation_cache.clone(),
        }
    }
}

impl PowerGraphNode {
    /// 获取节点的输出值
    pub fn output_values(&self) -> HashMap<OutputId, ElectricValueType> {
        let mut values = HashMap::new();

        match &self.data {
            ElectricNodeData::CircuitNode(circuit) => {
                // 回路节点输出功率和电流
                values.insert(OutputId::from(0), ElectricValueType::Float(circuit.rated_power));
                values.insert(OutputId::from(1), ElectricValueType::Float(circuit.current));

                // 输出回路数据
                let mut circuit_data = HashMap::new();
                circuit_data.insert("rated_power".to_string(), circuit.rated_power);
                circuit_data.insert("current".to_string(), circuit.current);
                circuit_data.insert("power_factor".to_string(), circuit.power_factor);
                circuit_data.insert("demand_coefficient".to_string(), circuit.demand_coefficient);
                values.insert(OutputId::from(2), ElectricValueType::CircuitData(circuit_data));
            },

            ElectricNodeData::DistributionBoxNode(box_data) => {
                // 配电箱节点输出总功率、总电流和三相数据
                values.insert(OutputId::from(0), ElectricValueType::Float(box_data.total_power));
                values.insert(OutputId::from(1), ElectricValueType::Float(box_data.incoming_current));

                // 输出配电箱数据
                let mut dist_box_data = HashMap::new();
                dist_box_data.insert("total_power".to_string(), box_data.total_power);
                dist_box_data.insert("incoming_current".to_string(), box_data.incoming_current);
                dist_box_data.insert("phase_a_load".to_string(), box_data.phase_a_load);
                dist_box_data.insert("phase_b_load".to_string(), box_data.phase_b_load);
                dist_box_data.insert("phase_c_load".to_string(), box_data.phase_c_load);
                values.insert(OutputId::from(2), ElectricValueType::DistributionBoxData(dist_box_data));
            },

            _ => {},
        }

        values
    }

    /// 处理输入数据并更新节点状态
    pub fn process(&mut self, inputs: HashMap<InputId, ElectricValueType>) -> HashMap<OutputId, ElectricValueType> {
        // 处理输入数据并更新节点状态
        match &mut self.data {
            ElectricNodeData::DistributionBoxNode(box_data) => {
                // 收集所有输入的回路数据
                let mut total_power = 0.0;
                let mut phase_a_load = 0.0;
                let mut phase_b_load = 0.0;
                let mut phase_c_load = 0.0;

                // 假设三相平均分配
                for (_, value) in inputs {
                    if let ElectricValueType::CircuitData(circuit_data) = value {
                        if let Some(power) = circuit_data.get("rated_power") {
                            total_power += power;

                            // 简化的三相分配逻辑
                            let phase_load = power / 3.0;
                            phase_a_load += phase_load;
                            phase_b_load += phase_load;
                            phase_c_load += phase_load;
                        }
                    } else if let ElectricValueType::Float(power) = value {
                        total_power += power;

                        // 简化的三相分配逻辑
                        let phase_load = power / 3.0;
                        phase_a_load += phase_load;
                        phase_b_load += phase_load;
                        phase_c_load += phase_load;
                    }
                }

                // 更新配电箱数据
                box_data.total_power = total_power;
                box_data.phase_a_load = phase_a_load;
                box_data.phase_b_load = phase_b_load;
                box_data.phase_c_load = phase_c_load;

                // 计算进线电流（假设功率因数0.8）
                box_data.incoming_current = (total_power * 1000.0) / (box_data.rated_voltage * 0.8 * 1.732);
            },

            _ => {},
        }

        // 返回处理后的输出值
        self.output_values()
    }
}

impl NodeDataTrait for PowerGraphNode {
    type Response = (); // 简单的响应类型
    type UserState = (); // 简单的用户状态类型
    type DataType = ElectricDataType;
    type ValueType = ElectricValueType;

    // 必需实现的bottom_ui方法
    fn bottom_ui(
        &self,
        _ui: &mut egui::Ui,
        _node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<Self::Response, Self>> {
        Default::default() // 返回空向量
    }
}

/// 节点图编辑器状态管理
pub struct PowerGraphState {
    /// 需要更新的节点集合
    pub nodes_to_update: std::collections::HashSet<NodeId>,
    /// 计算缓存
    pub calculation_cache: HashMap<NodeId, HashMap<String, ElectricValueType>>,
}

impl Default for PowerGraphState {
    fn default() -> Self {
        Self {
            nodes_to_update: std::collections::HashSet::new(),
            calculation_cache: HashMap::new(),
        }
    }
}

impl PowerGraphState {
    /// 标记节点为需要更新
    pub fn mark_node_for_update(&mut self, node_id: NodeId) {
        self.nodes_to_update.insert(node_id);
    }

    /// 标记下游节点为需要更新
    pub fn mark_downstream_nodes_for_update(&mut self, graph: &egui_node_graph::Graph<PowerGraphNode, ElectricDataType, ElectricValueType>, node_id: NodeId) {
        // 找到所有依赖此节点的下游节点
        for (input_id, connected_output_id) in &graph.connections {
            // 检查是否是当前节点的输出
            if let Some(output_node_id) = graph.find_node_by_output_id(*connected_output_id) {
                if output_node_id == node_id {
                    // 找到输入节点
                    if let Some(input_node_id) = graph.inputs.get(*input_id).map(|input| input.node) {
                        // 标记为需要更新
                        self.nodes_to_update.insert(input_node_id);

                        // 递归标记下游节点
                        self.mark_downstream_nodes_for_update(graph, input_node_id);
                    }
                }
            }
        }
    }

    /// 执行拓扑排序，确定节点更新顺序
    pub fn perform_topological_sort(&self, graph: &egui_node_graph::Graph<PowerGraphNode, ElectricDataType, ElectricValueType>) -> Vec<NodeId> {
        let mut visited = std::collections::HashSet::new();
        let mut result = Vec::new();

        // 从无入度的节点开始
        for (node_id, _) in &graph.nodes {
            if !visited.contains(node_id) {
                self.dfs_visit(graph, node_id, &mut visited, &mut result);
            }
        }

        result.reverse(); // 反转结果以获得正确的拓扑顺序
        result
    }

    /// 深度优先遍历辅助函数
    fn dfs_visit(&self, graph: &egui_node_graph::Graph<PowerGraphNode, ElectricDataType, ElectricValueType>, node_id: &NodeId, visited: &mut std::collections::HashSet<NodeId>, result: &mut Vec<NodeId>) {
        visited.insert(*node_id);

        // 找到所有依赖此节点的下游节点
        for (input_id, connected_output_id) in &graph.connections {
            // 检查是否是当前节点的输出
            if let Some(output_node_id) = graph.find_node_by_output_id(*connected_output_id) {
                if output_node_id == *node_id {
                    // 找到输入节点
                    if let Some(input_node_id) = graph.inputs.get(*input_id).map(|input| input.node) {
                        if !visited.contains(&input_node_id) {
                            self.dfs_visit(graph, &input_node_id, visited, result);
                        }
                    }
                }
            }
        }

        result.push(*node_id);
    }
}