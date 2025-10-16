//! 配电箱节点模板实现
//! 
//! 本模块实现了配电箱节点的模板，用于在节点编辑器中创建和配置新的配电箱节点。

use std::borrow::Cow;

use egui_node_graph::{Graph, NodeId};
use egui_node_graph::traits::{NodeTemplateTrait};

use crate::core_lib::data_types::{ElectricDataType, ElectricValueType};
use crate::editor::business::{DistributionBoxNode, BoxData, EditorState};

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
    type NodeData = DistributionBoxNode;
    type DataType = ElectricDataType;
    type ValueType = ElectricValueType;
    type Category = &'static str;
    type UserState = EditorState;
    
    /// 节点查找器标签
    fn node_finder_label(&self) -> Cow<'static, str> {
        Cow::Borrowed("配电箱")
    }
    
    /// 节点查找器类别
    fn node_finder_categories(&self) -> Vec<Self::Category> {
        vec!["电气节点"]
    }
    
    /// 获取用户数据（节点实例）
    fn user_data(&self) -> Self::NodeData {
        DistributionBoxNode::default()
    }
    
    /// 构建节点
    /// 
    /// 创建节点并配置其输入输出参数
    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> NodeId {
        // 创建新节点
        let node_data = self.user_data();
        let node_label = node_data.get_title();
        
        // 添加节点到图中
        let node_id = graph.add_node(node_data, &node_label);
        
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
        
        // 返回创建的节点ID
        node_id
    }
    
    /// 节点类别
    fn node_category(&self) -> &str {
        "电气节点"
    }
    
    /// 节点名称
    fn node_name(&self) -> &str {
        "配电箱"
    }
    
    /// 创建节点实例（替代build_node的旧接口）
    fn create_node(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        DistributionBoxNode::default()
    }
    
    /// 获取输入端口类型
    fn input_types(&self) -> Vec<(&str, Self::DataType)> {
        vec![
            ("配电箱名称", ElectricDataType::String),
            ("所在楼层", ElectricDataType::Integer),
            ("回路输入1", ElectricDataType::CircuitData),
            ("回路输入2", ElectricDataType::CircuitData),
            ("回路输入3", ElectricDataType::CircuitData),
        ]
    }
    
    /// 获取输出端口类型
    fn output_types(&self) -> Vec<(&str, Self::DataType)> {
        vec![
            ("配电箱数据", ElectricDataType::DistributionBoxData),
            ("总电流", ElectricDataType::Current),
            ("总功率", ElectricDataType::Power),
            ("进线电流", ElectricDataType::Current),
        ]
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
        assert_eq!(template.node_finder_label(), "配电箱");
        assert_eq!(template.node_category(), "电气节点");
        assert_eq!(template.node_name(), "配电箱");
        
        // 验证类别
        let categories = template.node_finder_categories();
        assert!(categories.contains(&"电气节点"));
        
        // 验证输入输出类型
        let input_types = template.input_types();
        assert_eq!(input_types.len(), 5);
        assert!(input_types.iter().any(|(name, _)| *name == "配电箱名称"));
        assert!(input_types.iter().any(|(name, _)| *name == "所在楼层"));
        
        let output_types = template.output_types();
        assert_eq!(output_types.len(), 4);
        assert!(output_types.iter().any(|(name, _)| *name == "配电箱数据"));
        assert!(output_types.iter().any(|(name, _)| *name == "总电流"));
        assert!(output_types.iter().any(|(name, _)| *name == "总功率"));
        assert!(output_types.iter().any(|(name, _)| *name == "进线电流"));
    }
    
    #[test]
    fn test_node_creation() {
        let template = DistributionBoxTemplate;
        let mut user_state = EditorState;
        
        // 创建节点
        let node = template.create_node(&mut user_state);
        
        // 验证节点属性
        assert_eq!(node.data.name, "新建配电箱");
        assert_eq!(node.data.floor, 1);
        assert_eq!(node.data.circuits.len(), 0);
        assert_eq!(node.data.total_power, 0.0);
        assert!(node.errors.is_empty());
    }
}