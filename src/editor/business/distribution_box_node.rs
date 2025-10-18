//! 配电箱节点实现
//! 
//! 本模块实现了配电箱节点的核心功能，包括数据管理、UI展示、参数计算等。

use std::collections::HashMap;
use std::borrow::Cow;

use egui::Color32;

use egui_node_graph::{Graph, NodeId, NodeResponse};
use egui_node_graph::traits::{NodeDataTrait, UserResponseTrait, NodeTemplateTrait};

use crate::core_lib::data_types::{ElectricDataType, ElectricValueType};
use crate::editor::business::{BoxData, CircuitInfo, DistributionBoxError, DistributionBoxResponse};
use crate::editor::business::{CircuitManager, DistributionBoxCalculator, EditorState};

/// 配电箱节点UI实现
/// 
/// 包装了DistributionBoxNodeUI数据，实现了NodeDataTrait接口，提供UI交互功能
#[derive(Debug, Clone)]
pub struct DistributionBoxNodeUI {
    /// 节点ID
    pub id: String,
    /// 配电箱数据
    pub data: BoxData,
    /// 错误信息
    pub errors: Vec<String>,
}

impl Default for DistributionBoxNodeUI {
    fn default() -> Self {
        Self {
            id: format!("distribution_box_{:x}", rand::random::<u32>()),
            data: BoxData::default(),
            errors: Vec::new(),
        }
    }
}

impl DistributionBoxNodeUI {
    /// 创建新的配电箱节点
    /// 
    /// # 参数
    /// * `id` - 节点唯一标识符
    /// * `data` - 配电箱数据
    /// 
    /// # 返回值
    /// 返回新创建的配电箱节点实例
    pub fn new(id: String, data: BoxData) -> Self {
        Self {
            id,
            data,
            errors: Vec::new(),
        }
    }
    
    /// 更新配电箱名称
    /// 
    /// # 参数
    /// * `name` - 新的配电箱名称
    pub fn update_name(&mut self, name: String) {
        self.data.name = name;
    }
    
    /// 更新配电箱楼层
    /// 
    /// # 参数
    /// * `floor` - 新的楼层号
    pub fn update_floor(&mut self, floor: u32) {
        self.data.floor = floor;
    }
    
    /// 添加回路
    /// 
    /// # 参数
    /// * `circuit` - 要添加的回路信息
    /// 
    /// # 返回值
    /// * `true` - 添加成功
    /// * `false` - 添加失败
    pub fn add_circuit(&mut self, circuit: CircuitInfo) -> bool {
        match self.data.add_circuit(circuit) {
            Ok(_) => {
                self.recalculate();
                true
            },
            Err(err) => {
                self.errors.push(err.to_string());
                false
            }
        }
    }
    
    /// 移除回路
    /// 
    /// # 参数
    /// * `circuit_id` - 要移除的回路ID
    /// 
    /// # 返回值
    /// * `true` - 移除成功
    /// * `false` - 移除失败
    pub fn remove_circuit(&mut self, circuit_id: &str) -> bool {
        match self.data.remove_circuit(circuit_id) {
            Ok(_) => {
                self.recalculate();
                true
            },
            Err(err) => {
                self.errors.push(err.to_string());
                false
            }
        }
    }
    
    /// 更新回路
    /// 
    /// # 参数
    /// * `circuit` - 更新后的回路信息
    /// 
    /// # 返回值
    /// * `true` - 更新成功
    /// * `false` - 更新失败
    pub fn update_circuit(&mut self, circuit: CircuitInfo) -> bool {
        match self.data.update_circuit(circuit) {
            Ok(_) => {
                self.recalculate();
                true
            },
            Err(err) => {
                self.errors.push(err.to_string());
                false
            }
        }
    }
    
    /// 执行计算
    pub fn recalculate(&mut self) {
        // 清除之前的错误和计算结果
        self.errors.clear();
        self.data.reset_calculations();
        
        // 如果没有回路，直接返回
        if self.data.circuits.is_empty() {
            return;
        }
        
        // 验证所有回路数据
        if let Err(err) = CircuitManager::validate_circuits(&self.data.circuits) {
            self.errors.push(err.to_string());
            return;
        }
        
        // 自动编号
        let mut circuits = self.data.circuits.clone();
        CircuitManager::auto_number_circuits(&mut circuits);
        
        // 执行三相平衡
        match DistributionBoxCalculator::balance_three_phases(&mut circuits) {
            Ok(phase_loads) => {
                // 更新相位负载
                self.data.phase_loads = phase_loads;
                
                // 更新回路数据
                self.data.circuits = circuits;
                
                // 计算总功率
                self.data.total_power = DistributionBoxCalculator::calculate_total_power(&self.data.circuits);
                
                // 计算总电流（默认功率因数0.85）
                match DistributionBoxCalculator::calculate_total_current(self.data.total_power, 0.85) {
                    Ok(total_current) => {
                        self.data.total_current = total_current;
                        
                        // 计算进线保护设备电流整定值
                        match DistributionBoxCalculator::calculate_incoming_current(total_current) {
                            Ok(incoming_current) => {
                                self.data.incoming_current = incoming_current;
                            },
                            Err(err) => {
                                self.errors.push(format!("进线电流计算错误: {}", err));
                            }
                        }
                    },
                    Err(err) => {
                        self.errors.push(format!("总电流计算错误: {}", err));
                    }
                }
            },
            Err(err) => {
                self.errors.push(format!("三相平衡失败: {}", err));
            }
        }
    }
    
    /// 获取节点标题
    pub fn get_title(&self) -> String {
        format!("配电箱: {}", self.data.name)
    }
    
    /// 将配电箱数据转换为哈希映射
    pub fn to_data_map(&self) -> HashMap<String, f64> {
        let mut map = HashMap::new();
        
        // 添加基本参数
        map.insert("total_power".to_string(), self.data.total_power);
        map.insert("total_current".to_string(), self.data.total_current);
        map.insert("incoming_current".to_string(), self.data.incoming_current);
        map.insert("floor".to_string(), self.data.floor as f64);
        map.insert("circuit_count".to_string(), self.data.circuits.len() as f64);
        
        // 添加各相负载
        map.insert("phase_load_l1".to_string(), self.data.phase_loads[0]);
        map.insert("phase_load_l2".to_string(), self.data.phase_loads[1]);
        map.insert("phase_load_l3".to_string(), self.data.phase_loads[2]);
        
        map
    }
}

// 实现NodeDataTrait接口
impl NodeDataTrait for DistributionBoxNodeUI {
    type Response = DistributionBoxResponse;
    type UserState = EditorState;
    type DataType = ElectricDataType;
    type ValueType = ElectricValueType;
    
    /// 节点底部UI，显示计算结果和错误信息
    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        _node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<Self::Response, Self>> {
        let mut responses = Vec::new();
        
        // 显示基本信息
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("楼层: ");
            ui.label(format!("{}", self.data.floor));
        });
        
        // 显示计算结果
        if !self.data.circuits.is_empty() {
            ui.separator();
            ui.label("计算结果:");
            ui.horizontal(|ui| {
                ui.label("回路数量: ");
                ui.label(format!("{}", self.data.circuits.len()));
            });
            ui.horizontal(|ui| {
                ui.label("总功率: ");
                ui.label(egui::RichText::new(format!("{:.2} kW", self.data.total_power)).color(Color32::GREEN));
            });
            ui.horizontal(|ui| {
                ui.label("总电流: ");
                ui.label(egui::RichText::new(format!("{:.2} A", self.data.total_current)).color(Color32::GREEN));
            });
            ui.horizontal(|ui| {
                ui.label("进线电流: ");
                ui.label(egui::RichText::new(format!("{:.0} A", self.data.incoming_current)).color(Color32::BLUE));
            });
            
            // 显示三相负载分布
            ui.separator();
            ui.label("三相负载分布:");
            ui.horizontal(|ui| {
                ui.label("L1: ");
                ui.label(format!("{:.2} kW", self.data.phase_loads[0]));
            });
            ui.horizontal(|ui| {
                ui.label("L2: ");
                ui.label(format!("{:.2} kW", self.data.phase_loads[1]));
            });
            ui.horizontal(|ui| {
                ui.label("L3: ");
                ui.label(format!("{:.2} kW", self.data.phase_loads[2]));
            });
            
            // 显示平衡度
            let total_load = self.data.phase_loads.iter().sum::<f64>();
            if total_load > 0.0 {
                let balance_degree = DistributionBoxCalculator::calculate_balance_degree(&self.data.phase_loads);
                ui.horizontal(|ui| {
                    ui.label("平衡度: ");
                    let balance_text = if balance_degree < 0.01 {
                        egui::RichText::new("优秀")
                    } else if balance_degree < 0.05 {
                        egui::RichText::new("良好")
                    } else {
                        egui::RichText::new("一般")
                    };
                    ui.label(balance_text);
                });
            }
            
            // 显示回路列表（最多显示5个）
            ui.separator();
            ui.label("回路列表:");
            let display_circuits = if self.data.circuits.len() > 5 {
                &self.data.circuits[0..5]
            } else {
                &self.data.circuits
            };
            
            for circuit in display_circuits {
                ui.horizontal(|ui| {
                    ui.label(format!("#{:02} ", circuit.number));
                    ui.label(format!("{} ", circuit.name));
                    ui.label(format!("{:.2}kW ", circuit.power));
                    if let Some(phase) = circuit.phase {
                        ui.label(format!("(相: L{})", phase));
                    }
                });
            }
            
            if self.data.circuits.len() > 5 {
                ui.label(format!("... 还有 {} 个回路", self.data.circuits.len() - 5));
            }
        }
        
        // 显示错误信息
        if !self.errors.is_empty() {
            ui.separator();
            ui.label(egui::RichText::new("错误:").color(Color32::RED));
            for error in &self.errors {
                ui.label(egui::RichText::new(format!("- {}", error)).color(Color32::RED));
            }
        }
    }
}

// 实现UserResponseTrait接口
impl UserResponseTrait for DistributionBoxResponse {}

impl DistributionBoxResponse {
    /// 应用响应到节点图
    pub fn apply(
        self,
        node_id: NodeId,
        graph: &mut Graph<DistributionBoxNodeUI, ElectricDataType, ElectricValueType>,
        user_state: &mut EditorState,
    ) {
        match self {
            DistributionBoxResponse::NodeUpdated => {
                // 节点已更新，可能需要刷新UI
            },
            DistributionBoxResponse::ParameterChanged(name, value) => {
                // 参数变更处理
            },
            DistributionBoxResponse::CircuitAdded(circuit) => {
                // 添加回路
                if let Some(node) = graph.nodes.get_mut(node_id) {
                    node.user_data.add_circuit(circuit);
                }
            },
            DistributionBoxResponse::CircuitRemoved(circuit_id) => {
                // 移除回路
                if let Some(node) = graph.nodes.get_mut(node_id) {
                    node.user_data.remove_circuit(&circuit_id);
                }
            },
            DistributionBoxResponse::CircuitUpdated(circuit) => {
                // 更新回路
                if let Some(node) = graph.nodes.get_mut(node_id) {
                    node.user_data.update_circuit(circuit);
                }
            },
            DistributionBoxResponse::CalculationCompleted => {
                // 计算完成
            },
            DistributionBoxResponse::Error(err) => {
                // 错误处理
                if let Some(node) = graph.nodes.get_mut(node_id) {
                    node.user_data.errors.push(err.to_string());
                }
            },
        }
    }
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_distribution_box_node_creation() {
        let node = DistributionBoxNodeUI::default();
        assert_eq!(node.data.name, "新建配电箱");
        assert_eq!(node.data.floor, 1);
        assert_eq!(node.data.circuits.len(), 0);
        assert_eq!(node.data.total_power, 0.0);
        assert_eq!(node.data.total_current, 0.0);
        assert_eq!(node.data.incoming_current, 0.0);
        assert!(node.errors.is_empty());
    }
    
    #[test]
    fn test_add_circuit() {
        let mut node = DistributionBoxNodeUI::default();
        let circuit = CircuitInfo::new(
            "circuit_1".to_string(),
            "测试回路1".to_string(),
            5.0,
            10.0
        );
        
        let result = node.add_circuit(circuit);
        
        assert!(result);
        assert_eq!(node.data.circuits.len(), 1);
        assert_eq!(node.data.circuits[0].name, "测试回路1");
        assert_eq!(node.data.circuits[0].power, 5.0);
        assert!(node.errors.is_empty());
    }
    
    #[test]
    fn test_update_name() {
        let mut node = DistributionBoxNodeUI::default();
        node.update_name("测试配电箱".to_string());
        
        assert_eq!(node.data.name, "测试配电箱");
        assert_eq!(node.get_title(), "配电箱: 测试配电箱");
    }
    
    #[test]
    fn test_recalculate() {
        let mut node = DistributionBoxNodeUI::default();
        
        // 添加几个回路
        let circuit1 = CircuitInfo::new(
            "circuit_1".to_string(),
            "回路1".to_string(),
            3.0,
            6.0
        );
        let circuit2 = CircuitInfo::new(
            "circuit_2".to_string(),
            "回路2".to_string(),
            4.0,
            8.0
        );
        let circuit3 = CircuitInfo::new(
            "circuit_3".to_string(),
            "回路3".to_string(),
            5.0,
            10.0
        );
        
        node.add_circuit(circuit1);
        node.add_circuit(circuit2);
        node.add_circuit(circuit3);
        
        // 手动触发重新计算
        node.recalculate();
        
        // 验证计算结果
        assert_eq!(node.data.circuits.len(), 3);
        assert_eq!(node.data.total_power, 12.0); // 3+4+5=12kW
        assert!(node.data.total_current > 0.0);
        assert!(node.data.incoming_current >= node.data.total_current);
        assert!(node.errors.is_empty());
        
        // 验证编号
        let mut numbers: Vec<u32> = node.data.circuits.iter().map(|c| c.number).collect();
        numbers.sort();
        assert_eq!(numbers, vec![1, 2, 3]);
    }
}

// 继续实现NodeDataTrait接口的方法
impl NodeDataTrait for DistributionBoxNodeUI {
    type Response = DistributionBoxResponse;
    type UserState = EditorState;
    type DataType = ElectricDataType;
    type ValueType = ElectricValueType;
    
    /// 节点底部UI，显示计算结果和错误信息
    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        _node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<Self::Response, Self>> {
        let mut responses = Vec::new();
        
        // 显示基本信息
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("楼层: ");
            ui.label(format!("{}", self.data.floor));
        });
        
        // 显示计算结果
        if !self.data.circuits.is_empty() {
            ui.separator();
            ui.label("计算结果:");
            ui.horizontal(|ui| {
                ui.label("回路数量: ");
                ui.label(format!("{}", self.data.circuits.len()));
            });
            ui.horizontal(|ui| {
                ui.label("总功率: ");
                ui.label(egui::RichText::new(format!("{:.2} kW", self.data.total_power)).color(Color32::GREEN));
            });
            ui.horizontal(|ui| {
                ui.label("总电流: ");
                ui.label(egui::RichText::new(format!("{:.2} A", self.data.total_current)).color(Color32::GREEN));
            });
            ui.horizontal(|ui| {
                ui.label("进线电流: ");
                ui.label(egui::RichText::new(format!("{:.0} A", self.data.incoming_current)).color(Color32::BLUE));
            });
        }
        
        responses
    }
    
    /// 顶部栏UI，显示节点名称
    fn top_bar_ui(
        &self,
        ui: &mut egui::Ui,
        _node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<Self::Response, Self>> {
        let mut responses = Vec::new();
        
        ui.label(self.get_title());
        
        responses
    }
}
    
    /// 标题栏颜色
    fn titlebar_color(
        &self,
        _ui: &egui::Ui,
        _node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> Option<Color32> {
        Some(Color32::from_rgb(200, 150, 100)) // 棕色系表示配电箱节点
    }
    
    /// 输出UI
    fn output_ui(
        &self,
        ui: &mut egui::Ui,
        _node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        param_name: &str,
    ) -> Vec<NodeResponse<Self::Response, Self>> {
        let mut responses = Vec::new();
        
        ui.horizontal(|ui| {
            ui.label(param_name);
            
            // 显示对应的值
            match param_name {
                "总电流" => {
                    ui.label(format!("({:.2}A)", self.data.total_current));
                },
                "总功率" => {
                    ui.label(format!("({:.2}kW)", self.data.total_power));
                },
                "进线电流" => {
                    ui.label(format!("({:.0}A)", self.data.incoming_current));
                },
                "配电箱数据" => {
                    ui.label(format!("({}回路)", self.data.circuits.len()));
                },
                _ => {},
            }
        });
        
        responses
    }

// 实现UserResponseTrait接口
impl UserResponseTrait for DistributionBoxResponse {}

impl DistributionBoxResponse {
    /// 应用响应到节点图
    pub fn apply(
        self,
        node_id: NodeId,
        graph: &mut Graph<DistributionBoxNodeUI, ElectricDataType, ElectricValueType>,
        user_state: &mut EditorState,
    ) {
        match self {
            DistributionBoxResponse::NodeUpdated => {
                // 节点已更新，可能需要刷新UI
            },
            DistributionBoxResponse::ParameterChanged(name, value) => {
                // 参数变更处理
            },
            DistributionBoxResponse::CircuitAdded(circuit) => {
                // 添加回路
                if let Some(node) = graph.nodes.get_mut(node_id) {
                    node.user_data.add_circuit(circuit);
                }
            },
            DistributionBoxResponse::CircuitRemoved(circuit_id) => {
                // 移除回路
                if let Some(node) = graph.nodes.get_mut(node_id) {
                    node.user_data.remove_circuit(&circuit_id);
                }
            },
            DistributionBoxResponse::CircuitUpdated(circuit) => {
                // 更新回路
                if let Some(node) = graph.nodes.get_mut(node_id) {
                    node.user_data.update_circuit(circuit);
                }
            },
            DistributionBoxResponse::CalculationCompleted => {
                // 计算完成
            },
            DistributionBoxResponse::Error(err) => {
                // 错误处理
                if let Some(node) = graph.nodes.get_mut(node_id) {
                    node.user_data.errors.push(err.to_string());
                }
            },
        }
    }
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_distribution_box_node_creation() {
        let node = DistributionBoxNodeUI::default();
        assert_eq!(node.data.name, "新建配电箱");
        assert_eq!(node.data.floor, 1);
        assert_eq!(node.data.circuits.len(), 0);
        assert_eq!(node.data.total_power, 0.0);
        assert_eq!(node.data.total_current, 0.0);
        assert_eq!(node.data.incoming_current, 0.0);
        assert!(node.errors.is_empty());
    }
    
    #[test]
    fn test_add_circuit() {
        let mut node = DistributionBoxNodeUI::default();
        let circuit = CircuitInfo::new(
            "circuit_1".to_string(),
            "测试回路1".to_string(),
            5.0,
            10.0
        );
        
        let result = node.add_circuit(circuit);
        
        assert!(result);
        assert_eq!(node.data.circuits.len(), 1);
        assert_eq!(node.data.circuits[0].name, "测试回路1");
        assert_eq!(node.data.circuits[0].power, 5.0);
        assert!(node.errors.is_empty());
    }
    
    #[test]
    fn test_update_name() {
        let mut node = DistributionBoxNodeUI::default();
        node.update_name("测试配电箱".to_string());
        
        assert_eq!(node.data.name, "测试配电箱");
        assert_eq!(node.get_title(), "配电箱: 测试配电箱");
    }
    
    #[test]
    fn test_recalculate() {
        let mut node = DistributionBoxNodeUI::default();
        
        // 添加几个回路
        let circuit1 = CircuitInfo::new(
            "circuit_1".to_string(),
            "回路1".to_string(),
            3.0,
            6.0
        );
        let circuit2 = CircuitInfo::new(
            "circuit_2".to_string(),
            "回路2".to_string(),
            4.0,
            8.0
        );
        let circuit3 = CircuitInfo::new(
            "circuit_3".to_string(),
            "回路3".to_string(),
            5.0,
            10.0
        );
        
        node.add_circuit(circuit1);
        node.add_circuit(circuit2);
        node.add_circuit(circuit3);
        
        // 手动触发重新计算
        node.recalculate();
        
        // 验证计算结果
        assert_eq!(node.data.circuits.len(), 3);
        assert_eq!(node.data.total_power, 12.0); // 3+4+5=12kW
        assert!(node.data.total_current > 0.0);
        assert!(node.data.incoming_current >= node.data.total_current);
        assert!(node.errors.is_empty());
        
        // 验证编号
        let mut numbers: Vec<u32> = node.data.circuits.iter().map(|c| c.number).collect();
        numbers.sort();
        assert_eq!(numbers, vec![1, 2, 3]);
    }
}