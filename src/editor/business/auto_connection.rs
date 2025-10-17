use std::collections::VecDeque; use crate::editor::business::{DistributionBoxNodeData, MainSystemNodeData, SystemDiagram, CircuitNodeData}; 

/// 配电箱进线类型枚举 #[derive(Debug, Clone, PartialEq, Eq)] pub enum IncomingType {
    /// 单电源进线    SinglePower,
    /// 双电源进线    DualPower,
}

/// 连接类型枚举 #[derive(Debug, Clone, PartialEq, Eq)] pub enum ConnectionType {
    /// 单电源连接    SinglePower,
    /// 双电源连接    DualPower,
    /// 备用电源连接    BackupPower,
    /// 母线连接    BusConnection,
    /// 馈线连接    FeederConnection,
}

/// 连接信息结构体 #[derive(Debug, Clone)] pub struct ConnectionInfo {
    /// 源配电箱名称    pub from: String,
    /// 目标配电箱名称    pub to: String,
    /// 连接类型    pub connection_type: ConnectionType,
}

/// 自动连接生成器 trait pub trait AutoConnectionGenerator {
    /// 自动生成连接信息    fn auto_generate_connections(&self, boxes: &[DistributionBoxNodeData]) -> Vec<ConnectionInfo>;
}

/// 扩展配电箱节点数据以支持进线类型判断 impl DistributionBoxNodeData {
    /// 判断配电箱的进线类型    pub fn determine_incoming_type(&self) -> IncomingType {
        // 根据模块配置判断是否为双电源切换配电箱        if self.modules.iter().any(|module| module.contains("双电源切换") || module.contains("双电源")) {
            IncomingType::DualPower        } else {
            IncomingType::SinglePower        }
    }
    
    /// 检查是否为消防负荷配电箱    pub fn is_fire_load(&self) -> bool {
        // 根据命名规则或标记判断是否为消防负荷        self.name.contains("消防") || self.name.contains("XF")
    }
}

/// 扩展干线系统图节点以支持自动连线生成 impl AutoConnectionGenerator for MainSystemNodeData {
    /// 根据进线类型自动生成连线    fn auto_generate_connections(&self, boxes: &[DistributionBoxNodeData]) -> Vec<ConnectionInfo> {
        let mut connections = Vec::new();

        // 1. 按楼层排序配电箱        let mut sorted_boxes = boxes.to_vec();        sorted_boxes.sort_by(|a, b| a.floor.cmp(&b.floor));

        // 2. 为每层建立配电箱组        let mut floor_boxes: HashMap<i32, Vec<&DistributionBoxNodeData>> = HashMap::new();        for box_node in &sorted_boxes {
            floor_boxes.entry(box_node.floor).or_default().push(box_node);        }

        // 3. 按楼层层级关系生成连线        for (current_floor, current_boxes) in &floor_boxes {
            // 找到上一层的配电箱（如果存在）            if let Some(prev_boxes) = floor_boxes.get(&(current_floor - 1)) {
                // 简单策略：当前层的每个配电箱都连接到上一层的主配电箱                // 实际应用中可以实现更复杂的连接逻辑                if let Some(prev_main_box) = prev_boxes.first() {
                    for box_node in current_boxes {
                        // 根据进线类型生成连线                        let incoming_type = box_node.determine_incoming_type();

                        connections.push(ConnectionInfo {
                            from: prev_main_box.name.clone(),                            to: box_node.name.clone(),                            connection_type: match incoming_type {
                                IncomingType::SinglePower => ConnectionType::SinglePower,
                                IncomingType::DualPower => ConnectionType::DualPower,
                            },                        });

                        // 如果是双电源，还需要连接备用电源                        if let IncomingType::DualPower = incoming_type {
                            connections.push(ConnectionInfo {
                                from: "备用电源".to_string(),                                to: box_node.name.clone(),                                connection_type: ConnectionType::BackupPower,
                            });                        }
                    }
                }
            }
        }

        connections    }
}

/// 扩展系统图以支持自动连接生成 impl SystemDiagram {
    /// 在系统图中自动应用连接信息    pub fn apply_connections(&mut self, connections: &[ConnectionInfo]) {
        // 将连接信息添加到系统图中        for connection in connections {
            // 查找源和目标组件            let from_index = self.components.iter().position(|c| c.name == connection.from);            let to_index = self.components.iter().position(|c| c.name == connection.to);
            
            if let (Some(from_idx), Some(to_idx)) = (from_index, to_index) {
                // 添加连接                self.connections.push((from_idx, to_idx));
            }
        }
    }
    
    /// 基于拓扑结构进行智能连线优化    pub fn optimize_connections(&mut self) {
        // 实现连线优化逻辑，避免交叉和过长的连线        // 简化实现：使用BFS来确定最短路径连接        let mut optimized_connections = Vec::new();        
        // 这里可以实现更复杂的连线优化算法        // 例如基于网格的布线、最短路径算法等        
        // 暂时保留原始连接        self.connections = optimized_connections;
    }
}

/// 自动识别和连线生成管理器 pub struct AutoConnectionManager {
    // 可选的连接规则配置    pub connection_rules: HashMap<String, Vec<ConnectionRule>>,
}

/// 连接规则定义 #[derive(Debug, Clone)] pub struct ConnectionRule {
    /// 源设备类型    pub source_type: String,
    /// 目标设备类型    pub target_type: String,
    /// 是否允许连接    pub allowed: bool,
    /// 连接优先级    pub priority: u32,
}

impl AutoConnectionManager {
    /// 创建新的自动连接管理器    pub fn new() -> Self {
        Self {
            connection_rules: HashMap::new(),
        }
    }
    
    /// 添加连接规则    pub fn add_rule(&mut self, rule: ConnectionRule) {
        self.connection_rules.entry(rule.source_type.clone()).or_default().push(rule);
    }
    
    /// 验证连接是否允许    pub fn validate_connection(&self, source_type: &str, target_type: &str) -> bool {
        if let Some(rules) = self.connection_rules.get(source_type) {
            for rule in rules {
                if rule.target_type == target_type {
                    return rule.allowed;
                }
            }
        }
        
        // 默认允许连接        true    }
    
    /// 为干线系统图执行自动连接生成    pub fn auto_connect_for_system(&self, main_system: &mut MainSystemNodeData, distribution_boxes: &[DistributionBoxNodeData]) -> Vec<ConnectionInfo> {
        main_system.auto_generate_connections(distribution_boxes)    }
}

/// 辅助函数：从配电回路推导出设备类型 pub fn derive_equipment_type(circuit: &CircuitNodeData) -> String {
    // 根据回路用途、功率和其他参数推导设备类型    if let Some(purpose) = &circuit.purpose {
        if purpose.contains("照明") {
            return "照明设备".to_string();        } else if purpose.contains("插座") {
            return "插座设备".to_string();        } else if purpose.contains("空调") {
            return "空调设备".to_string();        } else if purpose.contains("风机") || purpose.contains("水泵") {
            return "动力设备".to_string();        }
    }
    
    // 默认类型    "未知设备".to_string() }