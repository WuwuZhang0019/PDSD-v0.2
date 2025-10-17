//! 干线系统图节点实现
//! 
//! 本模块实现了干线系统图节点，用于自动生成配电干线图和各种监测系统图。

use std::collections::BTreeMap;
use std::fmt::Display;
use std::borrow::Cow;

use egui::{Ui, Widget};
use egui_node_graph::{Graph, NodeId, NodeTemplateTrait};

use crate::core_lib::data_types::{ElectricDataType, ElectricValueType};
use crate::editor::business::EditorState;

/// 干线系统图类型
#[derive(Debug, Clone, PartialEq)]
pub enum MainSystemType {
    PowerDistribution,     // 配电干线图
    EnergyMonitoring,      // 能耗监测干线图
    ElectricalFireMonitoring, // 电气火灾监控干线图
    FirePowerMonitoring,   // 消防电源监测干线图
}

impl Display for MainSystemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainSystemType::PowerDistribution => write!(f, "配电干线图"),
            MainSystemType::EnergyMonitoring => write!(f, "能耗监测干线图"),
            MainSystemType::ElectricalFireMonitoring => write!(f, "电气火灾监控干线图"),
            MainSystemType::FirePowerMonitoring => write!(f, "消防电源监测干线图"),
        }
    }
}

/// 系统图组件类型
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentType {
    Busbar,           // 母线
    DistributionBox,  // 配电箱
    MonitoringModule, // 监测模块
    PowerSource,      // 电源
}

/// 系统图连接类型
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionType {
    SinglePower, // 单电源
    DualPower,   // 双电源
    Monitoring,  // 监测
}

/// 系统图组件
#[derive(Debug, Clone)]
pub struct DiagramComponent {
    pub id: usize,
    pub component_type: ComponentType,
    pub label: String,
    pub position: (f64, f64),
}

/// 系统图连接
#[derive(Debug, Clone)]
pub struct DiagramConnection {
    pub from: usize,
    pub to: usize,
    pub connection_type: ConnectionType,
}

/// 系统图数据结构
#[derive(Debug, Clone)]
pub struct SystemDiagram {
    pub name: String,
    pub components: Vec<DiagramComponent>,
    pub connections: Vec<DiagramConnection>,
}

impl SystemDiagram {
    /// 创建新的系统图
    pub fn new(name: String) -> Self {
        Self {
            name,
            components: Vec::new(),
            connections: Vec::new(),
        }
    }

    /// 添加组件
    pub fn add_component(&mut self, component_type: ComponentType, label: String) -> usize {
        let id = self.components.len();
        self.components.push(DiagramComponent {
            id,
            component_type,
            label,
            position: (0.0, 0.0), // 初始位置，自动布局时会更新
        });
        id
    }

    /// 添加连接
    pub fn add_connection(&mut self, from: usize, to: usize) {
        self.connections.push(DiagramConnection {
            from,
            to,
            connection_type: ConnectionType::SinglePower, // 默认连接类型
        });
    }

    /// 设置连接类型
    pub fn set_connection_type(&mut self, from: usize, to: usize, connection_type: ConnectionType) {
        for connection in &mut self.connections {
            if connection.from == from && connection.to == to {
                connection.connection_type = connection_type;
                break;
            }
        }
    }
}

/// 干线系统图节点数据结构
#[derive(Debug, Clone)]
pub struct MainSystemNodeData {
    pub name: String,
    pub systems: Vec<MainSystemType>, // 包含的系统图类型
    pub auto_layout: bool,            // 是否自动布局
    pub diagrams: Vec<SystemDiagram>, // 生成的系统图
    pub errors: Vec<String>,          // 错误信息
}

impl Default for MainSystemNodeData {
    fn default() -> Self {
        Self {
            name: "干线系统图".to_string(),
            systems: vec![MainSystemType::PowerDistribution], // 默认只包含配电干线图
            auto_layout: true,
            diagrams: Vec::new(),
            errors: Vec::new(),
        }
    }
}

/// 干线系统图节点UI
#[derive(Debug, Clone, Default)]
pub struct MainSystemNodeUI {
    pub data: MainSystemNodeData,
}

impl MainSystemNodeUI {
    /// 自动映射功能
    pub fn auto_map_distribution_boxes(&mut self, boxes: &[Box<i32>]) -> Vec<SystemDiagram> {
        // 这里简化处理，实际实现应该接收配电箱数据
        let mut diagrams = Vec::new();

        // 生成各类系统图
        for system_type in &self.data.systems {
            match system_type {
                MainSystemType::PowerDistribution => {
                    let diagram = self.generate_power_distribution_diagram();
                    diagrams.push(diagram);
                },
                MainSystemType::EnergyMonitoring => {
                    let diagram = self.generate_energy_monitoring_diagram();
                    diagrams.push(diagram);
                },
                MainSystemType::ElectricalFireMonitoring => {
                    let diagram = self.generate_electrical_fire_monitoring_diagram();
                    diagrams.push(diagram);
                },
                MainSystemType::FirePowerMonitoring => {
                    let diagram = self.generate_fire_power_monitoring_diagram();
                    diagrams.push(diagram);
                },
            }
        }

        // 更新节点数据中的系统图
        self.data.diagrams = diagrams.clone();
        diagrams
    }

    /// 生成配电干线系统图
    fn generate_power_distribution_diagram(&self) -> SystemDiagram {
        let mut diagram = SystemDiagram::new("配电干线系统图".to_string());

        // 创建母线
        let busbar = diagram.add_component(ComponentType::Busbar, "主母线".to_string());

        // 创建一个示例配电箱
        let box_component = diagram.add_component(
            ComponentType::DistributionBox,
            "示例配电箱\n楼层:1\n功率:10.00kW\n电流:15.20A".to_string()
        );

        // 添加连线
        diagram.add_connection(busbar, box_component);

        // 设置连接类型为单电源
        diagram.set_connection_type(busbar, box_component, ConnectionType::SinglePower);

        if self.data.auto_layout {
            self.auto_layout_diagram(&mut diagram);
        }

        diagram
    }

    /// 生成能耗监测干线系统图
    fn generate_energy_monitoring_diagram(&self) -> SystemDiagram {
        let mut diagram = SystemDiagram::new("能耗监测干线系统图".to_string());

        // 创建母线
        let busbar = diagram.add_component(ComponentType::Busbar, "主母线".to_string());

        // 创建一个示例监测模块
        let module = diagram.add_component(
            ComponentType::MonitoringModule,
            "能耗监测模块\n楼层:1".to_string()
        );

        // 添加连线
        diagram.add_connection(busbar, module);
        diagram.set_connection_type(busbar, module, ConnectionType::Monitoring);

        if self.data.auto_layout {
            self.auto_layout_diagram(&mut diagram);
        }

        diagram
    }

    /// 生成电气火灾监控干线系统图
    fn generate_electrical_fire_monitoring_diagram(&self) -> SystemDiagram {
        let mut diagram = SystemDiagram::new("电气火灾监控干线系统图".to_string());

        // 创建母线
        let busbar = diagram.add_component(ComponentType::Busbar, "主母线".to_string());

        // 创建一个示例监测模块
        let module = diagram.add_component(
            ComponentType::MonitoringModule,
            "电气火灾监控模块\n楼层:1".to_string()
        );

        // 添加连线
        diagram.add_connection(busbar, module);
        diagram.set_connection_type(busbar, module, ConnectionType::Monitoring);

        if self.data.auto_layout {
            self.auto_layout_diagram(&mut diagram);
        }

        diagram
    }

    /// 生成消防电源监测干线系统图
    fn generate_fire_power_monitoring_diagram(&self) -> SystemDiagram {
        let mut diagram = SystemDiagram::new("消防电源监测干线系统图".to_string());

        // 创建母线
        let busbar = diagram.add_component(ComponentType::Busbar, "主母线".to_string());

        // 创建一个示例监测模块
        let module = diagram.add_component(
            ComponentType::MonitoringModule,
            "消防电源监测模块\n楼层:1".to_string()
        );

        // 添加连线
        diagram.add_connection(busbar, module);
        diagram.set_connection_type(busbar, module, ConnectionType::Monitoring);

        if self.data.auto_layout {
            self.auto_layout_diagram(&mut diagram);
        }

        diagram
    }

    /// 自动布局算法
    fn auto_layout_diagram(&self, diagram: &mut SystemDiagram) {
        // 简化的自动布局实现
        // 实际应用中应根据图论算法进行优化布局
        let mut x_pos = 100.0;
        let y_pos = 100.0;
        
        for component in &mut diagram.components {
            component.position = (x_pos, y_pos);
            x_pos += 200.0; // 组件之间水平间距
        }
    }

    /// 渲染节点UI
    pub fn ui(&mut self, ui: &mut Ui) -> Option<MainSystemResponse> {
        // 节点名称编辑
        ui.label("节点名称：");
        ui.text_edit_singleline(&mut self.data.name);
        ui.separator();
        
        // 系统图类型选择
        ui.label("选择系统图类型：");
        
        let mut power_distribution_selected = self.data.systems.contains(&MainSystemType::PowerDistribution);
        let mut energy_monitoring_selected = self.data.systems.contains(&MainSystemType::EnergyMonitoring);
        let mut electrical_fire_selected = self.data.systems.contains(&MainSystemType::ElectricalFireMonitoring);
        let mut fire_power_selected = self.data.systems.contains(&MainSystemType::FirePowerMonitoring);
        
        ui.checkbox(&mut power_distribution_selected, "配电干线图");
        ui.checkbox(&mut energy_monitoring_selected, "能耗监测干线图");
        ui.checkbox(&mut electrical_fire_selected, "电气火灾监控干线图");
        ui.checkbox(&mut fire_power_selected, "消防电源监测干线图");
        
        // 更新系统图类型列表
        self.data.systems.clear();
        if power_distribution_selected {
            self.data.systems.push(MainSystemType::PowerDistribution);
        }
        if energy_monitoring_selected {
            self.data.systems.push(MainSystemType::EnergyMonitoring);
        }
        if electrical_fire_selected {
            self.data.systems.push(MainSystemType::ElectricalFireMonitoring);
        }
        if fire_power_selected {
            self.data.systems.push(MainSystemType::FirePowerMonitoring);
        }
        
        // 自动布局选项
        ui.checkbox(&mut self.data.auto_layout, "启用自动布局");
        
        ui.separator();
        
        // 生成系统图按钮
        if ui.button("生成系统图").clicked() {
            // 这里简化处理，实际应该从输入获取配电箱数据
            let dummy_boxes: Vec<Box<i32>> = Vec::new();
            self.auto_map_distribution_boxes(&dummy_boxes);
            return Some(MainSystemResponse::DiagramGenerated);
        }
        
        // 显示已生成的系统图数量
        if !self.data.diagrams.is_empty() {
            ui.label(format!("已生成 {} 个系统图", self.data.diagrams.len()));
        }
        
        // 显示错误信息
        for error in &self.data.errors {
            ui.label(egui::RichText::new(error).color(egui::Color32::RED));
        }
        
        None
    }
}

/// 干线系统图节点响应
#[derive(Debug, Clone)]
pub enum MainSystemResponse {
    DiagramGenerated,     // 系统图已生成
    UpdateSettings,       // 更新设置
    Error(String),        // 错误信息
}

impl crate::editor::business::UserResponseTrait for MainSystemResponse {}

// 为MainSystemResponse实现apply方法
impl MainSystemResponse {
    pub fn apply(
        self,
        node_id: NodeId,
        graph: &mut Graph<MainSystemNodeUI, ElectricDataType, ElectricValueType>,
        _user_state: &mut EditorState,
    ) {
        match self {
            MainSystemResponse::DiagramGenerated => {
                // 系统图已生成，可以刷新UI或通知其他组件
            },
            MainSystemResponse::UpdateSettings => {
                // 更新设置后的处理
            },
            MainSystemResponse::Error(err) => {
                // 错误处理
                if let Some(node) = graph.nodes.get_mut(node_id) {
                    node.user_data.data.errors.push(err);
                }
            },
        }
    }
}

// 实现NodeDataTrait，处理节点数据的输入输出
impl crate::editor::business::NodeDataTrait for MainSystemNodeUI {
    type DataType = ElectricDataType;
    type ValueType = ElectricValueType;
    type UserState = EditorState;
    type Response = MainSystemResponse;
    
    /// 渲染节点主体UI
    fn node_ui(&mut self, ui: &mut Ui) -> Option<Self::Response> {
        self.ui(ui)
    }
    
    /// 渲染节点底部UI（用于显示输出结果）
    fn bottom_ui(&self, ui: &mut Ui) {
        // 显示已生成的系统图信息
        if !self.data.diagrams.is_empty() {
            ui.label("已生成系统图：");
            for diagram in &self.data.diagrams {
                ui.label(format!("- {}", diagram.name));
            }
        }
    }
    
    /// 参数更新处理
    fn update_params(
        &mut self,
        params: Self::ValueType,
        _user_state: &mut Self::UserState,
    ) -> Result<(), Self::DataType> {
        match params {
            ElectricValueType::String(value) => {
                self.data.name = value;
            },
            ElectricValueType::Bool(value) => {
                self.data.auto_layout = value;
            },
            _ => {},
        }
        
        Ok(())
    }
    
    /// 输入数据处理
    fn input_data(
        &mut self,
        input_idx: usize,
        data: &Self::DataType,
        _user_state: &mut Self::UserState,
    ) -> Result<(), Self::DataType> {
        // 处理来自配电箱节点的输入数据
        if let ElectricDataType::DistributionBoxData = data {
            // 实际应用中，这里应该解析配电箱数据并用于生成系统图
            // 现在简化处理
        }
        
        Ok(())
    }
    
    /// 输出数据获取
    fn output_data(
        &self,
        output_idx: usize,
        _user_state: &mut Self::UserState,
    ) -> Result<Self::DataType, Self::DataType> {
        match output_idx {
            0 => {
                // 输出系统图数据（简化处理，输出名称列表）
                let diagram_names: Vec<String> = self.data.diagrams.iter().map(|d| d.name.clone()).collect();
                Ok(ElectricDataType::String(format!("系统图列表: {:?}", diagram_names)))
            },
            1 => {
                // 输出系统图数量
                Ok(ElectricDataType::Integer(self.data.diagrams.len() as i64))
            },
            _ => {
                Err(ElectricDataType::String("无效的输出端口索引".to_string()))
            }
        }
    }
}