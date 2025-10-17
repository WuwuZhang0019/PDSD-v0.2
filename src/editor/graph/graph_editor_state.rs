use egui_node_graph::{Graph, GraphEditorState, NodeId, InputId, OutputId};
use crate::editor::{DataType, UIValueType, UIUserState};
use crate::editor::business::{get_all_node_templates, PowerGraphNode};
use std::collections::HashMap;

/// 电力配电系统图编辑器状态
pub struct PowerDistributionGraphEditorState {
    /// 节点图数据
    pub graph: Graph<PowerGraphNode, DataType, UIValueType>,
    /// 节点编辑器UI状态
    pub editor_state: GraphEditorState<PowerGraphNode, DataType, UIValueType, UIUserState>,
    /// UI状态数据
    pub user_state: UIUserState,
    /// 当前选中的节点ID
    pub selected_node_id: Option<NodeId>,
}
 

impl Default for PowerDistributionGraphEditorState {
    fn default() -> Self {
        let mut state = Self {
            graph: Graph::default(),
            editor_state: GraphEditorState::default(),
            user_state: UIUserState::default(),
            selected_node_id: None,
        };
        
        state
    }
}


impl PowerDistributionGraphEditorState {
    /// 创建新的图编辑器状态
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 获取所有可用的节点模板
    pub fn get_node_templates(&self) -> Vec<impl egui_node_graph::NodeTemplateTrait<NodeData=PowerGraphNode, DataType=DataType, ValueType=UIValueType, UserState=UIUserState>> {
        get_all_node_templates()
    }
    
    /// 添加节点
    pub fn add_node(&mut self, node_data: PowerGraphNode) -> NodeId {
        self.graph.add_node(node_data)
    }
    
    /// 创建连接
    pub fn connect_nodes(&mut self, output_id: OutputId, input_id: InputId) -> bool {
        self.graph.connect(output_id, input_id)
    }
    
    /// 断开连接
    pub fn disconnect_nodes(&mut self, input_id: InputId) -> bool {
        self.graph.disconnect(input_id)
    }
    
    /// 处理节点响应
    pub fn handle_node_responses(&mut self, responses: Vec<egui_node_graph::NodeResponse<crate::editor::UIResponse, PowerGraphNode>>) {
        for response in responses {
            match response {
                egui_node_graph::NodeResponse::User(user_response) => {
                    match user_response {
                        crate::editor::UIResponse::ValueChanged(node_id, param_name, new_value) => {
                            // 处理值变化事件
                            println!("Node {} parameter '{}' changed to {:?}", node_id, param_name, new_value);
                            // 可以在这里触发下游节点的更新
                            self.mark_downstream_for_update(node_id);
                        },
                        crate::editor::UIResponse::NodeSelected(node_id) => {
                            // 处理节点选择事件
                            self.selected_node_id = Some(node_id);
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }
    
    /// 标记下游节点为需要更新
    fn mark_downstream_for_update(&self, _node_id: NodeId) {
        // 实现拓扑排序和节点更新逻辑
        // 这部分将在后续实现
    }
}