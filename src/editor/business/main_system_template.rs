//! 干线系统图节点模板定义

use std::borrow::Cow;

use egui_node_graph::{NodeTemplateTrait, NodeId, Graph, InputParamKind};

use crate::editor::business::{EditorState, MainSystemNodeUI};
use crate::core_lib::data_types::{ElectricDataType, ElectricValueType};

/// 干线系统图节点模板
#[derive(Debug, Clone, Default)]
pub struct MainSystemTemplate;

impl MainSystemTemplate {
    /// 创建新的干线系统图节点模板实例
    pub fn new() -> Self {
        Self::default()
    }
}

impl NodeTemplateTrait for MainSystemTemplate {
    type NodeData = MainSystemNodeUI;
    type DataType = ElectricDataType;
    type ValueType = ElectricValueType;
    type UserState = EditorState;
    type CategoryType = &'static str;
    
    /// 节点查找器标签
    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'static, str> {
        Cow::Borrowed("干线系统图")
    }
    
    /// 节点查找器分类
    fn node_finder_categories(&self, _user_state: &mut Self::UserState) -> Vec<Self::CategoryType> {
        vec!["干线系统"]
    }
    
    /// 节点图标签
    fn node_graph_label(&self, _user_state: &mut Self::UserState) -> Cow<'static, str> {
        Cow::Borrowed("干线系统图节点")
    }
    
    /// 节点描述
    fn node_description(&self, _user_state: &mut Self::UserState) -> Cow<'static, str> {
        Cow::Borrowed("用于自动生成配电干线图和各种监测系统图的节点")
    }
    
    /// 用户数据创建
    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        MainSystemNodeUI::default()
    }
    
    /// 构建节点，添加输入输出参数
    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        // 输入参数：接收配电箱数据
        graph.add_input_param(
            node_id,
            "配电箱数据", 
            InputParamKind::Connection, 
            ElectricDataType::DistributionBoxData,
            None
        );
        
        // 输出参数：系统图数据
        graph.add_output_param(
            node_id,
            "系统图数据",
            ElectricDataType::SystemDiagramData
        );
        
        // 输出参数：系统图数量
        graph.add_output_param(
            node_id,
            "系统图数量",
            ElectricDataType::Integer
        );
    }
    
    /// 节点预览图标（可选）
    fn node_finder_icon(&self, _user_state: &mut Self::UserState) -> Option<&'static str> {
        None // 可以在这里指定图标
    }
    
    /// 节点默认宽度
    fn node_width(&self, _user_state: &mut Self::UserState) -> f32 {
        250.0
    }
}