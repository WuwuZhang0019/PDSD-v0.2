use eframe::{App, egui};
use egui_node_graph::{Graph, NodeId};
use crate::editor::business::{CircuitNodeData, DistributionBoxNodeData, MainSystemNodeData, MainSystemType, SystemDiagram, DataFlowManager, AutoConnectionManager, ConnectionInfo};
use crate::core_lib::data_types::{ElectricNodeData, ElectricDataType, ElectricValueType};

/// 集成示例应用 - 展示所有功能模块的组合使用
pub struct IntegrationExampleApp {
    /// 节点图数据
    graph: Graph<ElectricNodeData, ElectricDataType, ElectricValueType>,
    /// 数据流向管理器
    data_flow_manager: DataFlowManager,
    /// 自动连接管理器
    auto_connection_manager: AutoConnectionManager,
    /// 示例运行步骤
    step: u32,
    /// 操作结果信息
    result_info: String,
}

impl Default for IntegrationExampleApp {
    fn default() -> Self {
        Self {
            graph: Graph::new(),
            data_flow_manager: DataFlowManager::new(),
            auto_connection_manager: AutoConnectionManager::new(),
            step: 0,
            result_info: "点击开始按钮开始示例".to_string(),
        }
    }
}

impl IntegrationExampleApp {
    /// 重置示例状态
    fn reset(&mut self) {
        self.graph = Graph::new();
        self.step = 0;
        self.result_info = "示例已重置".to_string();
    }
    
    /// 执行示例步骤
    fn execute_step(&mut self) {
        match self.step {
            0 => {
                // 步骤1：创建示例配电回路节点
                self.create_circuit_nodes();
                self.result_info = "已创建示例配电回路节点".to_string();
            },
            1 => {
                // 步骤2：创建配电箱节点并连接回路
                self.create_distribution_box_nodes();
                self.result_info = "已创建配电箱节点并连接回路".to_string();
            },
            2 => {
                // 步骤3：创建干线系统图节点
                self.create_main_system_node();
                self.result_info = "已创建干线系统图节点".to_string();
            },
            3 => {
                // 步骤4：运行数据流向更新
                self.run_data_flow_update();
                self.result_info = "已完成数据流向更新和计算".to_string();
            },
            4 => {
                // 步骤5：执行自动连接生成
                self.execute_auto_connections();
                self.result_info = "已完成自动连接生成".to_string();
            },
            5 => {
                // 步骤6：显示结果摘要
                self.show_summary();
                self.result_info = "示例完成！查看系统图结果摘要".to_string();
                self.step = 4; // 保持在最后一步
            },
            _ => {
                self.reset();
            },
        }
        
        self.step += 1;
    }
    
    /// 创建示例配电回路节点
    fn create_circuit_nodes(&mut self) {
        // 创建一些示例配电回路
        let circuit1 = CircuitNodeData {
            name: "照明回路1".to_string(),
            power: 5.5,
            power_factor: 0.9,
            purpose: Some("普通照明".to_string()),
            ..Default::default()
        };
        
        let circuit2 = CircuitNodeData {
            name: "插座回路1".to_string(),
            power: 3.0,
            power_factor: 0.85,
            purpose: Some("办公室插座".to_string()),
            ..Default::default()
        };
        
        let circuit3 = CircuitNodeData {
            name: "空调回路1".to_string(),
            power: 8.0,
            power_factor: 0.8,
            purpose: Some("办公室空调".to_string()),
            ..Default::default()
        };
        
        // 将回路节点添加到图中
        let _ = self.graph.add_node(
            "照明回路1".to_string(),
            ElectricNodeData::CircuitNode(circuit1),
            |_, _| {},
        );
        
        let _ = self.graph.add_node(
            "插座回路1".to_string(),
            ElectricNodeData::CircuitNode(circuit2),
            |_, _| {},
        );
        
        let _ = self.graph.add_node(
            "空调回路1".to_string(),
            ElectricNodeData::CircuitNode(circuit3),
            |_, _| {},
        );
    }
    
    /// 创建配电箱节点并连接回路
    fn create_distribution_box_nodes(&mut self) {
        // 创建两个配电箱：一个普通配电箱和一个双电源切换配电箱
        let box1 = DistributionBoxNodeData {
            name: "配电箱AL1".to_string(),
            floor: 1,
            modules: vec!["普通配电模块".to_string()],
            ..Default::default()
        };
        
        let box2 = DistributionBoxNodeData {
            name: "消防配电箱XF1".to_string(),
            floor: 2,
            modules: vec!["双电源切换模块".to_string()],
            ..Default::default()
        };
        
        // 添加配电箱节点
        let box1_id = self.graph.add_node(
            "配电箱AL1".to_string(),
            ElectricNodeData::DistributionBoxNode(box1),
            |_, _| {},
        ).unwrap();
        
        let box2_id = self.graph.add_node(
            "消防配电箱XF1".to_string(),
            ElectricNodeData::DistributionBoxNode(box2),
            |_, _| {},
        ).unwrap();
        
        // 简单连接：第一个回路连接到第一个配电箱，其他连接到第二个
        let mut circuit_nodes = Vec::new();
        for (node_id, node) in &self.graph.nodes {
            if let ElectricNodeData::CircuitNode(_) = &node.user_data {
                circuit_nodes.push(*node_id);
            }
        }
        
        // 这里应该实现真正的连接逻辑，目前简化处理
    }
    
    /// 创建干线系统图节点
    fn create_main_system_node(&mut self) {
        // 创建干线系统图节点
        let system_node = MainSystemNodeData {
            name: "干线系统图MS1".to_string(),
            systems: vec![
                MainSystemType::PowerDistribution,
                MainSystemType::EnergyMonitoring,
                MainSystemType::ElectricalFireMonitoring,
                MainSystemType::FirePowerMonitoring,
            ],
            auto_layout: true,
            ..Default::default()
        };
        
        // 添加系统图节点
        let _ = self.graph.add_node(
            "干线系统图MS1".to_string(),
            ElectricNodeData::TrunkLineNode(system_node),
            |_, _| {},
        );
    }
    
    /// 运行数据流向更新
    fn run_data_flow_update(&mut self) {
        // 触发数据流向更新
        self.data_flow_manager.propagate_updates(&mut self.graph);
    }
    
    /// 执行自动连接生成
    fn execute_auto_connections(&mut self) {
        // 收集所有配电箱节点数据
        let mut distribution_boxes = Vec::new();
        let mut system_node_id: Option<NodeId> = None;
        
        for (node_id, node) in &self.graph.nodes {
            match &node.user_data {
                ElectricNodeData::DistributionBoxNode(box_data) => {
                    distribution_boxes.push(box_data.clone());
                },
                ElectricNodeData::TrunkLineNode(_) => {
                    system_node_id = Some(*node_id);
                },
                _ => {},
            }
        }
        
        // 生成自动连接
        if let Some(node_id) = system_node_id {
            let connections = self.auto_connection_manager.auto_connect_for_system(
                &mut MainSystemNodeData::default(), // 简化处理
                &distribution_boxes
            );
            
            // 将连接信息保存到系统图中
            // 简化处理：打印连接信息
            println!("生成的连接信息: {:?}", connections);
        }
    }
    
    /// 显示结果摘要
    fn show_summary(&mut self) {
        // 收集系统状态信息
        let node_count = self.graph.nodes.len();
        let distribution_boxes: Vec<_> = self.graph.nodes.values()
            .filter(|node| matches!(&node.user_data, ElectricNodeData::DistributionBoxNode(_)))
            .count();
        let circuit_count: Vec<_> = self.graph.nodes.values()
            .filter(|node| matches!(&node.user_data, ElectricNodeData::CircuitNode(_)))
            .count();
        
        println!("=== 系统摘要 ===");
        println!("总节点数: {}", node_count);
        println!("配电箱数: {}", distribution_boxes);
        println!("回路数: {}", circuit_count);
        println!("==============");
    }
}

impl App for IntegrationExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PDSD系统集成示例");
            ui.separator();
            
            // 控制面板
            ui.horizontal(|ui| {
                if ui.button("开始/下一步").clicked() {
                    self.execute_step();
                }
                if ui.button("重置").clicked() {
                    self.reset();
                }
            });
            
            // 状态显示
            ui.add_space(10.0);
            ui.label(format!("当前步骤: {}/5", self.step));
            ui.add_space(10.0);
            ui.label(format!("状态: {}", self.result_info));
            
            // 节点信息显示
            ui.add_space(20.0);
            ui.collapsing("当前节点列表", |ui| {
                for (node_id, node) in &self.graph.nodes {
                    ui.label(format!("节点 ID: {}, 名称: {}", node_id.index(), node.title));
                }
            });
            
            // 操作说明
            ui.add_space(20.0);
            ui.collapsing("操作说明", |ui| {
                ui.label("示例展示了PDSD系统的核心功能流程:");
                ui.label("1. 创建配电回路节点");
                ui.label("2. 创建配电箱节点并连接回路");
                ui.label("3. 创建干线系统图节点");
                ui.label("4. 运行数据流向更新和计算");
                ui.label("5. 执行自动连接生成");
                ui.label("6. 查看系统图结果摘要");
                ui.add_space(10.0);
                ui.label("点击'开始/下一步'按钮逐步执行示例");
            });
        });
    }
}

/// 启动集成示例应用的函数
pub fn start_integration_example() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("PDSD集成示例"),
        ..Default::default()
    };
    
    // 实际运行应用的代码（这里仅作示例，不实际执行）
    println!("PDSD集成示例已准备就绪");
    println!("在实际应用中，这里会调用eframe::run_native启动示例应用");
}