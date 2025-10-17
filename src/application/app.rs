use crate::editor::{DataType, UIValueType, UIUserState, UIResponse};
use crate::editor::business::{CircuitNode, DistributionBoxNodeUI, PowerGraphNode};
use crate::editor::graph::PowerDistributionGraphEditorState;
use crate::editor::business::{all_electric_templates, ElectricNodeTemplate};
use crate::editor::ui::{NodeEditor, custom_connections::draw_custom_connection, node_groups::NodeGroupManager};
use crate::application::debug_logger::DebugLogger;
use eframe::{App, egui};
use uuid::Uuid;
use std::collections::HashMap;
use rand; // 添加随机数库导入

/// 电力配电系统设计应用程序主结构体
pub struct PDSDApp {
    /// 节点图编辑器状态
    pub editor_state: PowerDistributionGraphEditorState,
    /// 项目信息
    pub project_name: String,
    pub project_id: Uuid,
    /// 计算结果缓存
    pub calculation_cache: HashMap<String, f64>,
    /// 错误信息
    pub error_message: Option<String>,
    /// 节点编辑器组件
    pub node_editor: NodeEditor,
    /// 调试日志管理器
    pub debug_logger: DebugLogger,
    /// 节点分组管理器
    pub group_manager: NodeGroupManager,
    /// 节点搜索文本
    pub node_search_text: String,
    /// 是否显示节点查找器
    pub show_node_finder: bool,
}

impl Default for PDSDApp {
    fn default() -> Self {
        Self {
            editor_state: PowerDistributionGraphEditorState::default(),
            project_name: "电气配电系统项目".to_string(),
            project_id: Uuid::new_v4(),
            calculation_cache: HashMap::new(),
            error_message: None,
            node_editor: NodeEditor::new(),
            debug_logger: DebugLogger::new(),
            group_manager: NodeGroupManager::default(),
            node_search_text: String::new(),
            show_node_finder: false,
        }
    }
}

impl App for PDSDApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // 创建应用标题和项目信息
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("{} - 项目ID: {}", 
                    self.project_name, 
                    self.project_id.to_string().split('-').next().unwrap())) 
                    .heading() 
                    .color(egui::Color32::from_rgb(0, 100, 200)));
                
                if ui.button("项目设置").clicked() {
                    // 打开项目设置对话框
                    self.open_project_settings(ui);
                }
            });
            
            ui.separator();

            // 创建工具栏
            ui.horizontal(|ui| {
                // 添加节点按钮
                if ui.button("添加节点").clicked() {
                    // 创建上下文菜单显示可用节点类型，按类别分组
                    ui.menu_button("选择节点类型", |ui| {
                        // 按类别组织节点模板
                        let templates = all_electric_templates();

                        // 收集所有唯一类别
                        let mut categories = std::collections::HashSet::new();
                        for template in &templates {
                            categories.insert(template.category());
                        }

                        // 按类别显示节点
                        for category in categories.into_iter() {
                            ui.menu_button(category, |ui| {
                                for template in &templates {
                                    if template.category() == category {
                                        if ui.button(template.node_label()).clicked() {
                                            // 添加新节点
                                            let _ = self.editor_state.graph.add_node(
                                                template.node_label().to_string(),
                                                template.user_data(&mut self.editor_state.user_state),
                                                |graph, node_id| template.build_node(graph, &mut self.editor_state.user_state, node_id),
                                            );
                                            ui.close_menu();
                                        }
                                    }
                                }
                            });
                        }
                    });
                }

                // 保存和加载按钮
                if ui.button("保存项目").clicked() {
                    // 实现项目保存功能
                    if let Err(e) = self.save_project() {
                        self.error_message = Some(format!("保存失败: {}", e));
                    } else {
                        self.error_message = Some("项目保存成功".to_string());
                    }
                }

                if ui.button("加载项目").clicked() {
                    // 实现项目加载功能
                    if let Err(e) = self.load_project() {
                        self.error_message = Some(format!("加载失败: {}", e));
                    } else {
                        self.error_message = Some("项目加载成功".to_string());
                    }
                }

                // 运行计算按钮
                if ui.button("运行计算").clicked() {
                    self.run_calculations();
                    self.error_message = Some("计算完成".to_string());
                }

                // 生成报告按钮
                if ui.button("生成报告").clicked() {
                    if let Err(e) = self.generate_report() {
                        self.error_message = Some(format!("报告生成失败: {}", e));
                    } else {
                        self.error_message = Some("报告生成成功".to_string());
                    }
                }
            });

            // 显示错误信息
            if let Some(error_msg) = &self.error_message {
                ui.label(egui::RichText::new(error_msg).color(egui::Color32::RED));
                // 清除错误信息计时器
                ui.ctx().request_repaint_after(std::time::Duration::from_secs(3));
                if ui.button("×").clicked() {
                    self.error_message = None;
                }
            }

            // 创建一个可滚动区域
            egui::ScrollArea::both().show(ui, |ui| {
                // 调整容器大小
                let (_id, response) = ui.allocate_space(egui::Vec2::new(
                    ui.available_width(),
                    ui.available_height() - 50.0, // 留出状态栏空间
                ));

                // 绘制节点图编辑器
                let node_responses = egui_node_graph::draw_graph_editor(
                    ui,
                    &mut self.editor_state.graph,
                    &mut self.editor_state.editor_state,
                    &mut self.editor_state.user_state,
                    None, // 可选的背景
                );

                // 处理节点响应事件
                self.handle_node_responses(node_responses);
            });

            // 状态栏显示
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!("节点数量: {}", self.editor_state.graph.nodes.len()));
                ui.label(format!("连接数量: {}", self.editor_state.graph.connections.len()));
                ui.label(format!("计算缓存: {}", self.calculation_cache.len()));
            });
        });
    }
}

impl PDSDApp {
    // 保存项目
    fn save_project(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 创建项目数据结构
        // 这里需要实现项目数据的序列化和保存逻辑
        // 目前仅作为占位符实现
        println!("保存项目: {}", self.project_name);
        Ok(())
    }

    // 加载项目
    fn load_project(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 这里需要实现项目数据的加载和反序列化逻辑
        // 目前仅作为占位符实现
        println!("加载项目");
        Ok(())
    }

    // 运行电气系统计算
    fn run_calculations(&mut self) {
        println!("运行电气系统计算");
        // 更新调试日志
        self.debug_logger.info("开始执行电气系统计算");
        // 执行图计算逻辑
        self.execute_graph();
        // 在实际应用中，这里可能还需要处理计算结果或更新UI显示
    }

    // 生成报告
    fn generate_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("生成系统报告");
        Ok(())
    }

    // 打开项目设置对话框
    fn open_project_settings(&mut self, ui: &mut egui::Ui) {
        // 实现项目设置对话框将在后续开发
        println!("打开项目设置");
    }

    // 处理节点响应事件
    fn handle_node_responses(&mut self, responses: Vec<egui_node_graph::NodeResponse<UIResponse, PowerGraphNode>>) {
        self.editor_state.handle_node_responses(responses);
    }

    // 构建拓扑排序以确定节点执行顺序
    fn topological_sort(&self) -> Vec<egui_node_graph::NodeId> {
        let mut visited = std::collections::HashSet::new();
        let mut result = Vec::new();

        // 从没有输入的节点开始（通常是输入节点）
        for node_id in self.editor_state.graph.nodes.keys() {
            if !visited.contains(&node_id) && self.is_node_independent(node_id) {
                self.dfs_visit(node_id, &mut visited, &mut result);
            }
        }

        // 确保所有节点都被访问
        for node_id in self.editor_state.graph.nodes.keys() {
            if !visited.contains(&node_id) {
                self.dfs_visit(node_id, &mut visited, &mut result);
            }
        }

        result
    }

    // 深度优先搜索访问节点
    fn dfs_visit(
        &self,
        node_id: egui_node_graph::NodeId,
        visited: &mut std::collections::HashSet<egui_node_graph::NodeId>,
        result: &mut Vec<egui_node_graph::NodeId>,
    ) {
        visited.insert(node_id);

        // 找到所有依赖当前节点的节点
        if let Some(node) = self.editor_state.graph.nodes.get(node_id) {
            for (_, output_id) in &node.outputs {
                // 找到连接到这个输出的所有输入
                for (input_id, connected_output_id) in &self.editor_state.graph.connections {
                    if connected_output_id == output_id {
                        // 找到输入所属的节点
                        if let Some(input_param) = self.editor_state.graph.inputs.get(*input_id) {
                            let input_node_id = input_param.node;
                            if !visited.contains(&input_node_id) {
                                self.dfs_visit(input_node_id, visited, result);
                            }
                        }
                    }
                }
            }
        }

        // 将当前节点添加到结果中（后序遍历确保依赖节点先执行）
        result.push(node_id);
    }

    // 判断节点是否没有依赖（独立节点）
    fn is_node_independent(&self, node_id: egui_node_graph::NodeId) -> bool {
        // 检查节点的所有输入参数
        if let Some(node) = self.editor_state.graph.nodes.get(node_id) {
            for (_, input_id) in &node.inputs {
                // 如果有任何一个输入被连接，则节点不是独立的
                if self.editor_state.graph.connections.contains_key(input_id) {
                    return false;
                }
            }
        }
        true
    }

    // 获取节点输入参数的值
    fn get_input_value(
        &self,
        input_id: egui_node_graph::InputId,
    ) -> Option<UIValueType> {
        // 首先检查是否有连接
        if let Some(output_id) = self.editor_state.graph.connections.get(&input_id) {
            // 如果有连接，需要找到对应的输出节点并获取其计算结果
            if let Some(output_param) = self.editor_state.graph.outputs.get(*output_id) {
                // 找到输出所属的节点
                let output_node_id = self.find_node_by_output(*output_id)?;
                
                // 尝试从缓存中获取计算结果
                let cache_key = format!("{}_{}", output_node_id.0, output_id.0);
                if let Some(cached_value) = self.calculation_cache.get(&cache_key) {
                    // 根据输出参数类型返回适当的值
                    return match output_param.typ {
                        DataType::Float => Some(UIValueType::Float(*cached_value)),
                        DataType::Integer => Some(UIValueType::Integer(*cached_value as i64)),
                        _ => Some(UIValueType::Float(*cached_value)), // 默认返回浮点数
                    };
                }

                // 由于计算可能很复杂，这里返回一个默认值作为示例
                return match output_param.typ {
                    DataType::Float => Some(UIValueType::Float(0.0)),
                    DataType::Integer => Some(UIValueType::Integer(0)),
                    _ => Some(UIValueType::Float(0.0)), // 默认返回浮点数
                };
            }
        } else {
            // 如果没有连接，返回输入参数的默认值
            if let Some(input_param) = self.editor_state.graph.inputs.get(input_id) {
                match input_param.kind {
                    egui_node_graph::InputParamKind::Constant |
                    egui_node_graph::InputParamKind::ConnectionOrConstant => {
                        Some(input_param.value.clone())
                    },
                    _ => None,
                }
            } else {
                None
            }
        }
    }

    // 根据输出ID查找对应的节点
    fn find_node_by_output(&self, output_id: egui_node_graph::OutputId) -> Option<egui_node_graph::NodeId> {
        for (node_id, node) in &self.editor_state.graph.nodes {
            for (_, out_id) in &node.outputs {
                if *out_id == output_id {
                    return Some(node_id);
                }
            }
        }
        None
    }

    // 执行图计算
    fn execute_graph(&mut self) {
        // 获取拓扑排序后的节点执行顺序
        let node_order = self.topological_sort();
        
        // 清空计算缓存
        self.calculation_cache.clear();
        
        // 按照拓扑排序顺序执行每个节点
        for node_id in node_order {
            if let Some(node) = self.editor_state.graph.nodes.get(node_id) {
                self.debug_logger.debug(&format!("执行节点: {} (类型: {:?})", node.label, node.user_data.node_type));
                
                // 收集输入值用于调试日志
                let mut input_values = Vec::new();
                for (_, input_id) in &node.inputs {
                    if let Some(value) = self.get_input_value(*input_id) {
                        input_values.push(value);
                    }
                }
                
                // 为每个输出生成一个示例计算结果
                for (output_name, output_id) in &node.outputs {
                    if let Some(output) = self.editor_state.graph.outputs.get(*output_id) {
                        // 生成示例结果
                        let result_value = self.calculate_output_value(node_id, *output_id);
                        
                        // 缓存计算结果
                        let cache_key = format!("{}_{}", node_id.0, output_id.0);
                        self.calculation_cache.insert(cache_key, result_value);
                        
                        // 记录日志
                        self.debug_logger.debug(&format!("  输出 {} ({:?}) = {}", output_name, output.typ, result_value));
                    }
                }
                
                // 记录节点执行日志
                self.debug_logger.log_node_execution(node_id, &input_values, None);
            }
        }
        
        self.debug_logger.info("图计算执行完成");
    }
    
    // 计算输出值（示例实现）
    fn calculate_output_value(&self, node_id: egui_node_graph::NodeId, output_id: egui_node_graph::OutputId) -> f64 {
        // 获取节点数据
        if let Some(node) = self.editor_state.graph.nodes.get(node_id) {
            // 根据节点类型执行不同的计算逻辑
            match node.user_data.node_type {
                // 这里应该根据实际的节点类型实现具体计算
                // 目前返回随机值作为示例
                _ => {
                    // 简单示例：生成0-100之间的随机值
                    (rand::random::<f64>() * 100.0).round()
                }
            }
        } else {
            0.0
        }
    }
}
