//! 配电箱节点模板实现
//! 
//! 本模块实现了配电箱节点的模板，用于在节点编辑器中创建和配置新的配电箱节点。

use std::borrow::Cow;

use egui_node_graph::{Graph, NodeId};
use egui_node_graph::traits::{NodeTemplateTrait};

use crate::core_lib::data_types::{ElectricDataType, ElectricValueType};
use crate::editor::business::{DistributionBoxNodeUI, BoxData, EditorState};

/// 配电箱节点模板
/// 
/// 用于在节点编辑器中创建配电箱节点，定义了节点的类别、名称、输入输出端口等信息
#[derive(Debug, Clone, Default)]
pub struct DistributionBoxTemplate;

impl DistributionBoxTemplate {
    /// 创建新的配电箱节点模板实例
    pub fn new() -> Self {
        Self::default()
    }
}

impl NodeTemplateTrait for DistributionBoxTemplate {
    type NodeData = DistributionBoxNodeUI;
    type DataType = ElectricDataType;
    type ValueType = ElectricValueType;
    type UserState = EditorState;
    type CategoryType = &'static str;
    
    /// 节点查找器标签
    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'static, str> {
        Cow::Borrowed("配电箱")
    }
    
    /// 节点查找器类别
    fn node_finder_categories(&self, _user_state: &mut Self::UserState) -> Vec<Self::CategoryType> {
        vec!["电气节点"]
    }
    
    /// 节点图标签
    fn node_graph_label(&self, _user_state: &mut Self::UserState) -> String {
        "配电箱".to_string()
    }
    
    /// 用户数据
    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        DistributionBoxNodeUI::default()
    }
    
    /// 构建节点
    /// 
    /// 创建节点并配置其输入输出参数
    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        // 添加输入参数
        // 配电箱名称
        graph.add_input_param(
            node_id,
            "配电箱名称",
            ElectricDataType::String,
            ElectricValueType::String("新建配电箱".to_string()),
            egui_node_graph::InputParamKind::ConstantOnly,
            true,
        );
        
        // 所在楼层
        graph.add_input_param(
            node_id,
            "所在楼层",
            ElectricDataType::Integer,
            ElectricValueType::Float(1.0),
            egui_node_graph::InputParamKind::ConstantOnly,
            true,
        );
        
        // 回路数据输入端口（动态，最多99个）
        // 这里先添加3个输入端口，实际使用时可以根据需要添加更多
        for i in 1..=3 {
            graph.add_input_param(
                node_id,
                &format!("回路输入{}", i),
                ElectricDataType::CircuitData,
                ElectricValueType::Float(0.0),
                egui_node_graph::InputParamKind::Dynamic,
                false,
            );
        }
        
        // 添加输出参数
        // 配电箱数据（完整信息）
        graph.add_output_param(
            node_id,
            "配电箱数据",
            ElectricDataType::DistributionBoxData,
        );
        
        // 总电流
        graph.add_output_param(
            node_id,
            "总电流",
            ElectricDataType::Current,
        );
        
        // 总功率
        graph.add_output_param(
            node_id,
            "总功率",
            ElectricDataType::Power,
        );
        
        // 进线电流（保护设备电流整定值）
        graph.add_output_param(
            node_id,
            "进线电流",
            ElectricDataType::Current,
        );
    }
    

}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_template_properties() {
        let template = DistributionBoxTemplate;
        
        // 验证模板属性
        let mut user_state = EditorState;
        assert_eq!(template.node_finder_label(&mut user_state), "配电箱");
        assert_eq!(template.node_graph_label(&mut user_state), "配电箱");
        
        // 验证类别
        let categories = template.node_finder_categories(&mut user_state);
        assert!(categories.contains(&"电气节点"));
    }
    
    #[test]
    fn test_node_creation() {
        let template = DistributionBoxTemplate;
        let mut user_state = EditorState;
        let mut graph = Graph::default();
        
        // 创建节点 - add_node expects label, user_data, and a closure
        let node_data = DistributionBoxNodeUI::default();
        let node_id = graph.add_node("配电箱".to_string(), node_data, |_graph, _node_id| {});

        // 构建节点
        template.build_node(&mut graph, &mut user_state, node_id);

        // 验证节点属性
        let node = &graph[node_id];
        assert_eq!(node.user_data.data.name, "新建配电箱");
        assert_eq!(node.user_data.data.floor, 1);
        assert_eq!(node.user_data.data.circuits.len(), 0);
        assert_eq!(node.user_data.data.total_power, 0.0);
        assert!(node.user_data.errors.is_empty());
    }
}