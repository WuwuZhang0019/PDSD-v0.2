/// 节点模板定义
/// 用于创建和配置不同类型的电气节点

use std::borrow::Cow;
use uuid::Uuid;
use egui_node_graph::{NodeTemplateTrait, NodeId, Graph, CategoryTrait, InputParamKind};
use crate::editor::{DataType, UIValueType, UIUserState};
use crate::core_lib::data_types::ElectricNodeData as CoreElectricNodeData;
use crate::editor::business::{PowerGraphNode};

/// 电气节点模板，用于创建不同类型的节点
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ElectricNodeTemplate {
    // 配电回路相关节点
    CircuitNode,
    CircuitGroupNode,

    // 配电箱相关节点
    DistributionBoxNode,
    MainDistributionBoxNode,
    SubDistributionBoxNode,

    // 干线系统图相关节点
    MainLineNode,
    FeederLineNode,

    // 电源节点
    PowerSourceNode,

    // 计算节点
    CurrentCalculationNode,
    PhaseBalanceNode,
}

// 实现CategoryTrait以支持节点分类
impl egui_node_graph::CategoryTrait for ElectricNodeTemplate {
    fn name(&self) -> String {
        match self {
            ElectricNodeTemplate::CircuitNode |
            ElectricNodeTemplate::CircuitGroupNode => "配电回路".to_string(),

            ElectricNodeTemplate::DistributionBoxNode |
            ElectricNodeTemplate::MainDistributionBoxNode |
            ElectricNodeTemplate::SubDistributionBoxNode => "配电箱".to_string(),

            ElectricNodeTemplate::MainLineNode |
            ElectricNodeTemplate::FeederLineNode => "干线系统".to_string(),

            ElectricNodeTemplate::PowerSourceNode => "电源".to_string(),

            ElectricNodeTemplate::CurrentCalculationNode |
            ElectricNodeTemplate::PhaseBalanceNode => "计算工具".to_string(),
        }
    }
}

impl NodeTemplateTrait for ElectricNodeTemplate {
    type NodeData = PowerGraphNode;
    type DataType = DataType;
    type ValueType = UIValueType;
    type UserState = UIUserState;
    type Category = ElectricNodeTemplate;
    type CategoryType = ElectricNodeTemplate;
    
    /// 节点查找器中显示的标签
    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<str> {
        Cow::Borrowed(self.node_label().as_str())
    }
    
    /// 节点在查找器中的分类
    fn node_finder_categories(&self, _user_state: &mut Self::UserState) -> Vec<ElectricNodeTemplate> {
        vec![*self]  // 返回自身作为分类
    }
    
    /// 节点在图形编辑器中显示的标签
    fn node_graph_label(&self, _user_state: &mut Self::UserState) -> String {
        self.node_label()
    }
    
    /// 节点标签
    fn node_label(&self) -> String {
        match self {
            ElectricNodeTemplate::CircuitNode => "配电回路".to_string(),
            ElectricNodeTemplate::CircuitGroupNode => "回路组".to_string(),
            ElectricNodeTemplate::DistributionBoxNode => "配电箱".to_string(),
            ElectricNodeTemplate::MainDistributionBoxNode => "总配电箱".to_string(),
            ElectricNodeTemplate::SubDistributionBoxNode => "分配电箱".to_string(),
            ElectricNodeTemplate::MainLineNode => "干线".to_string(),
            ElectricNodeTemplate::FeederLineNode => "馈线".to_string(),
            ElectricNodeTemplate::PowerSourceNode => "电源".to_string(),
            ElectricNodeTemplate::CurrentCalculationNode => "电流计算".to_string(),
            ElectricNodeTemplate::PhaseBalanceNode => "三相平衡".to_string(),
        }
    }
    
    /// 节点分类
    fn node_category(&self) -> ElectricNodeTemplate {
        *self
    }
    
    /// 获取分类名称（兼容CategoryTrait的category方法）
    fn category(&self) -> &str {
        match self {
            ElectricNodeTemplate::CircuitNode |
            ElectricNodeTemplate::CircuitGroupNode => "配电回路",

            ElectricNodeTemplate::DistributionBoxNode |
            ElectricNodeTemplate::MainDistributionBoxNode |
            ElectricNodeTemplate::SubDistributionBoxNode => "配电箱",

            ElectricNodeTemplate::MainLineNode |
            ElectricNodeTemplate::FeederLineNode => "干线系统",

            ElectricNodeTemplate::PowerSourceNode => "电源",

            ElectricNodeTemplate::CurrentCalculationNode |
            ElectricNodeTemplate::PhaseBalanceNode => "计算工具",
        }
    }
    
    /// 创建节点的用户数据（使用UUID生成临时ID）
    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        // 使用UUID::new_v4()生成临时ID
        let temp_id = Uuid::new_v4();
        self.node_data(temp_id)
    }
    
    /// 根据ID创建节点数据
    fn node_data(&self, id: Uuid) -> Self::NodeData {
        let node_id = format!("{}-{}", self.node_prefix(), id.to_string().split('-').next().unwrap());
        let node_name = self.node_label();
        let description = self.node_description();
        
        // 创建对应的core_lib::data_types::ElectricNodeData
        let core_node_data = match self {
            ElectricNodeTemplate::CircuitNode => crate::core_lib::data_types::ElectricNodeData::CircuitNode(Default::default()),
            ElectricNodeTemplate::CircuitGroupNode => crate::core_lib::data_types::ElectricNodeData::CircuitNode(Default::default()),
            ElectricNodeTemplate::DistributionBoxNode |
            ElectricNodeTemplate::MainDistributionBoxNode |
            ElectricNodeTemplate::SubDistributionBoxNode => crate::core_lib::data_types::ElectricNodeData::DistributionBoxNode(Default::default()),
            ElectricNodeTemplate::MainLineNode |
            ElectricNodeTemplate::FeederLineNode => crate::core_lib::data_types::ElectricNodeData::TrunkLineNode(Default::default()),
            ElectricNodeTemplate::PowerSourceNode => crate::core_lib::data_types::ElectricNodeData::PowerSourceNode(Default::default()),
            ElectricNodeTemplate::CurrentCalculationNode |
            ElectricNodeTemplate::PhaseBalanceNode => crate::core_lib::data_types::ElectricNodeData::CalculationNode(Default::default()),
        };
        
        PowerGraphNode {
            id: node_id,
            data: core_node_data,
            calculation_cache: std::collections::HashMap::new(),
        }
    }
    
    /// 获取节点ID前缀
    fn node_prefix(&self) -> &str {
        match self {
            ElectricNodeTemplate::CircuitNode => "CIR",
            ElectricNodeTemplate::CircuitGroupNode => "CIG",
            ElectricNodeTemplate::DistributionBoxNode => "DB",
            ElectricNodeTemplate::MainDistributionBoxNode => "MDB",
            ElectricNodeTemplate::SubDistributionBoxNode => "SDB",
            ElectricNodeTemplate::MainLineNode => "ML",
            ElectricNodeTemplate::FeederLineNode => "FL",
            ElectricNodeTemplate::PowerSourceNode => "PS",
            ElectricNodeTemplate::CurrentCalculationNode => "CC",
            ElectricNodeTemplate::PhaseBalanceNode => "PB",
        }
    }
    
    /// 获取节点描述
    fn node_description(&self) -> String {
        match self {
            ElectricNodeTemplate::CircuitNode => "用于表示配电回路的节点".to_string(),
            ElectricNodeTemplate::CircuitGroupNode => "用于表示配电回路组的节点".to_string(),
            ElectricNodeTemplate::DistributionBoxNode => "用于表示通用配电箱的节点".to_string(),
            ElectricNodeTemplate::MainDistributionBoxNode => "用于表示主配电箱的节点".to_string(),
            ElectricNodeTemplate::SubDistributionBoxNode => "用于表示子配电箱的节点".to_string(),
            ElectricNodeTemplate::MainLineNode => "用于表示主线路的节点".to_string(),
            ElectricNodeTemplate::FeederLineNode => "用于表示馈线的节点".to_string(),
            ElectricNodeTemplate::PowerSourceNode => "用于表示电源的节点".to_string(),
            ElectricNodeTemplate::CurrentCalculationNode => "用于进行电流计算的节点".to_string(),
            ElectricNodeTemplate::PhaseBalanceNode => "用于进行相平衡分析的节点".to_string(),
        }
    }
    
    /// 构建节点，添加输入输出参数
    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        // 根据节点类型添加输入输出参数
        match self {
            ElectricNodeTemplate::CircuitNode => {
                // 配电回路节点：一个输入（上级电源），多个输出（功率、电流、回路数据）
                graph.add_input_param(node_id, "上级电源", InputParamKind::ConnectionOnly, DataType::Voltage, None);
                graph.add_output_param(node_id, "功率", DataType::Power);
                graph.add_output_param(node_id, "电流", DataType::Current);
                graph.add_output_param(node_id, "回路数据", DataType::CircuitInfo);
            },

            ElectricNodeTemplate::CircuitGroupNode => {
                // 回路组节点：输入各回路信息，输出组总功率和电流
                graph.add_input_param(node_id, "回路数据1", InputParamKind::ConnectionOrConstant, DataType::CircuitInfo, Some(UIValueType::CircuitInfo(Default::default())));
                graph.add_input_param(node_id, "回路数据2", InputParamKind::ConnectionOrConstant, DataType::CircuitInfo, Some(UIValueType::CircuitInfo(Default::default())));
                graph.add_input_param(node_id, "回路数据3", InputParamKind::ConnectionOrConstant, DataType::CircuitInfo, Some(UIValueType::CircuitInfo(Default::default())));
                graph.add_output_param(node_id, "总功率", DataType::Power);
                graph.add_output_param(node_id, "总电流", DataType::Current);
                graph.add_output_param(node_id, "回路组数据", DataType::CircuitGroupInfo);
            },

            ElectricNodeTemplate::DistributionBoxNode |
            ElectricNodeTemplate::MainDistributionBoxNode |
            ElectricNodeTemplate::SubDistributionBoxNode => {
                // 配电箱节点：一个输入（上级进线），多个输出（出线电流、三相平衡、配电箱数据）
                graph.add_input_param(node_id, "上级进线", InputParamKind::ConnectionOnly, DataType::Current, None);
                graph.add_output_param(node_id, "出线电流", DataType::Current);
                graph.add_output_param(node_id, "三相平衡", DataType::PhaseBalanceInfo);
                graph.add_output_param(node_id, "配电箱数据", DataType::DistributionBoxInfo);
            },

            ElectricNodeTemplate::MainLineNode |
            ElectricNodeTemplate::FeederLineNode => {
                // 干线节点：一个输入（电源输入），多个输出（线路电流、电压降、干线数据）
                graph.add_input_param(node_id, "电源输入", InputParamKind::ConnectionOnly, DataType::Voltage, None);
                graph.add_output_param(node_id, "线路电流", DataType::Current);
                graph.add_output_param(node_id, "电压降", DataType::Voltage);
                graph.add_output_param(node_id, "干线数据", DataType::MainLineInfo);
            },

            ElectricNodeTemplate::PowerSourceNode => {
                // 电源节点：无输入，输出电压和电源信息
                graph.add_output_param(node_id, "电压", DataType::Voltage);
                graph.add_output_param(node_id, "电源容量", DataType::Power);
                graph.add_output_param(node_id, "电源信息", DataType::PowerSourceInfo);
            },

            ElectricNodeTemplate::CurrentCalculationNode => {
                // 电流计算节点：输入功率、电压、功率因数，输出计算电流
                graph.add_input_param(node_id, "功率", InputParamKind::ConnectionOrConstant, DataType::Power, Some(UIValueType::Float(0.0)));
                graph.add_input_param(node_id, "电压", InputParamKind::ConnectionOrConstant, DataType::Voltage, Some(UIValueType::Float(220.0)));
                graph.add_input_param(node_id, "功率因数", InputParamKind::ConnectionOrConstant, DataType::PowerFactor, Some(UIValueType::Float(0.8)));
                graph.add_input_param(node_id, "需用系数", InputParamKind::ConnectionOrConstant, DataType::Coefficient, Some(UIValueType::Float(0.8)));
                graph.add_output_param(node_id, "计算电流", DataType::Current);
            },

            ElectricNodeTemplate::PhaseBalanceNode => {
                // 三相平衡节点：输入A/B/C相功率，输出不平衡度和优化建议
                graph.add_input_param(node_id, "A相功率", InputParamKind::ConnectionOrConstant, DataType::Power, Some(UIValueType::Float(0.0)));
                graph.add_input_param(node_id, "B相功率", InputParamKind::ConnectionOrConstant, DataType::Power, Some(UIValueType::Float(0.0)));
                graph.add_input_param(node_id, "C相功率", InputParamKind::ConnectionOrConstant, DataType::Power, Some(UIValueType::Float(0.0)));
                graph.add_output_param(node_id, "不平衡度", DataType::Coefficient);
                graph.add_output_param(node_id, "平衡建议", DataType::String);
                graph.add_output_param(node_id, "相平衡结果", DataType::PhaseBalanceInfo);
            },
        }
    }
}

/// 初始化所有节点模板
pub fn get_all_node_templates() -> Vec<ElectricNodeTemplate> {
    all_electric_templates()
}

/// 提供一个获取所有可用模板的函数（文档规范中定义的函数名）
pub fn all_electric_templates() -> Vec<ElectricNodeTemplate> {
    vec![
        // 配电回路相关节点
        ElectricNodeTemplate::CircuitNode,
        ElectricNodeTemplate::CircuitGroupNode,

        // 配电箱相关节点
        ElectricNodeTemplate::DistributionBoxNode,
        ElectricNodeTemplate::MainDistributionBoxNode,
        ElectricNodeTemplate::SubDistributionBoxNode,

        // 干线系统图相关节点
        ElectricNodeTemplate::MainLineNode,
        ElectricNodeTemplate::FeederLineNode,

        // 电源节点
        ElectricNodeTemplate::PowerSourceNode,

        // 计算节点
        ElectricNodeTemplate::CurrentCalculationNode,
        ElectricNodeTemplate::PhaseBalanceNode,
    ]
}