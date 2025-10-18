use crate::editor::{DataType, UIValueType, UIUserState, UIResponse};
use crate::editor::business::{CircuitNode, DistributionBoxNodeUI, PowerGraphNode, DataFlowManager, AutoConnectionManager, CircuitType};
use crate::core_lib::data_types::ElectricValueType;
use crate::editor::graph::PowerDistributionGraphEditorState;
use crate::editor::business::{all_electric_templates, ElectricNodeTemplate};
use crate::editor::ui::{NodeEditor, custom_connections::draw_custom_connection, node_groups::NodeGroupManager, node_search_ui};
use crate::editor::ui::debug_tools::{log_panel_ui, LOGGER, LogLevel};
use crate::editor::ui::performance_optimization::{PerformanceOptimizer, performance_settings_ui, performance_stats_ui};
use crate::application::debug_logger::DebugLogger;
use eframe::{App, egui};
use uuid::Uuid;
use std::collections::HashMap;
use std::time::{Instant, Duration};
use chrono::Local;
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
    /// 右侧面板激活的标签页
    pub active_right_tab: String,
    /// 数据流向管理器
    pub data_flow_manager: DataFlowManager,
    /// 自动连接管理器
    pub auto_connection_manager: AutoConnectionManager,
    /// 性能优化管理器
    pub performance_optimizer: PerformanceOptimizer,
    /// 上次更新时间
    pub last_update_time: Instant,
    /// 帧率计算相关
    pub frame_count: u32,
    pub last_fps_update_time: Instant,
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
            active_right_tab: "属性".to_string(),
            data_flow_manager: DataFlowManager::new(),
            auto_connection_manager: AutoConnectionManager::new(),
            performance_optimizer: PerformanceOptimizer::new(),
            last_update_time: Instant::now(),
            frame_count: 0,
            last_fps_update_time: Instant::now(),
        }
    }
}

impl App for PDSDApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 记录更新开始时间
        let update_start_time = Instant::now();
        
        // 更新帧率计算
        self.frame_count += 1;
        let now = Instant::now();
        if now.duration_since(self.last_fps_update_time).as_secs() >= 1 {
            let fps = self.frame_count as f32 / now.duration_since(self.last_fps_update_time).as_secs_f32();
            self.performance_optimizer.update_fps(fps);
            self.frame_count = 0;
            self.last_fps_update_time = now;
        }
        // 左侧组件库面板
            egui::SidePanel::left("node_palette").resizable(true).show(ctx, |ui| {
                ui.heading("组件库");

                // 配电回路节点
                ui.collapsing("配电回路", |ui| {
                    if ui.button("单相配电回路").clicked() {
                        // 添加单相配电回路节点
                        self.add_circuit_node(CircuitType::SinglePhase);
                    }
                    if ui.button("三相配电回路").clicked() {
                        // 添加三相配电回路节点
                        self.add_circuit_node(CircuitType::ThreePhase);
                    }
                });

                // 配电箱节点
                ui.collapsing("配电箱", |ui| {
                    if ui.button("标准配电箱").clicked() {
                        // 添加标准配电箱节点
                        self.add_distribution_box_node();
                    }
                });

                // 干线系统图节点
                ui.collapsing("干线系统", |ui| {
                    if ui.button("干线系统图").clicked() {
                        // 添加干线系统图节点
                        self.add_main_system_node();
                    }
                });
            });

            // 右侧面板 - 可切换的标签页
            egui::SidePanel::right("right_panel").resizable(true).show(ctx, |ui| {
                ui.heading("功能面板");
                
                // 标签页选择
                ui.horizontal(|ui| {
                    if ui.selectable_label(self.active_right_tab == "属性", "属性").clicked() {
                        self.active_right_tab = "属性".to_string();
                    }
                    if ui.selectable_label(self.active_right_tab == "搜索", "搜索").clicked() {
                        self.active_right_tab = "搜索".to_string();
                    }
                    if ui.selectable_label(self.active_right_tab == "性能", "性能").clicked() {
                        self.active_right_tab = "性能".to_string();
                    }
                    if ui.selectable_label(self.active_right_tab == "调试", "调试").clicked() {
                        self.active_right_tab = "调试".to_string();
                    }
                });
                
                ui.separator();
                
                // 根据当前标签页显示不同内容
                match self.active_right_tab.as_str() {
                    "属性" => {
                        // 显示选中节点的属性
                        if let Some(selected_node_id) = self.editor_state.selected_node() {
                            self.draw_node_properties(ui, selected_node_id);
                        } else {
                            ui.label("未选中节点");
                        }
                    },
                    "搜索" => {
                        // 节点搜索面板
                        node_search_ui(
                            ui,
                            &mut self.node_search_text,
                            &mut self.show_node_finder,
                            &self.editor_state.graph
                        );
                    },
                    "性能" => {
                        // 性能设置和统计
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            performance_settings_ui(ui, &mut self.performance_optimizer);
                            ui.add_space(20.0);
                            performance_stats_ui(ui, &self.performance_optimizer);
                        });
                    },
                    "调试" => {
                        // 日志面板
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            // 显示调试控制面板
                            self.show_debug_panel(ui);
                            ui.add_space(20.0);
                            
                            // 显示日志面板
                            log_panel_ui(ui, &LOGGER);
                        });
                    },
                    _ => {}
                }
            });

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
                    // 触发数据流向更新
                    self.data_flow_manager.propagate_updates(&mut self.editor_state.graph);
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
                // 更新性能优化器的视口信息
                let viewport = ui.ctx().input(|i| i.screen_rect);
                self.performance_optimizer.update_viewport(viewport);
                
                // 调整容器大小
                let (_id, response) = ui.allocate_space(egui::Vec2::new(
                    ui.available_width(),
                    ui.available_height() - 50.0, // 留出状态栏空间
                ));

                // 记录渲染开始时间
                let render_start_time = Instant::now();
                
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
                
                // 记录渲染时间
                self.performance_optimizer.update_render_time(render_start_time.elapsed());
            });

            // 状态栏显示
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!("节点数量: {}", self.editor_state.graph.nodes.len()));
                ui.label(format!("连接数量: {}", self.editor_state.graph.connections.len()));
                ui.label(format!("计算缓存: {}", self.calculation_cache.len()));
                ui.label(format!("FPS: {:.1}", self.performance_optimizer.get_stats().current_fps));
            });
        });
        
        // 记录更新时间
        self.performance_optimizer.update_update_time(update_start_time.elapsed());
    }
}

impl PDSDApp {
    // 绘制节点属性面板
    fn draw_node_properties(&mut self, ui: &mut egui::Ui, node_id: egui_node_graph::NodeId) {
        if let Some(node) = self.editor_state.graph.nodes.get_mut(node_id) {
            match &mut node.user_data {
                PowerGraphNode::CircuitNode(circuit) => {
                    ui.heading("配电回路属性");
                    ui.text_edit_singleline(&mut circuit.name);
                    ui.add(egui::Slider::new(&mut circuit.power, 0.1..=100.0).text("功率 (kW)"));
                    if ui.button("更新计算").clicked() {
                        // 执行计算逻辑
                        self.data_flow_manager.mark_node_for_update(node_id);
                        self.run_calculations();
                    }

                    // 显示计算结果
                    ui.group(|ui| {
                        ui.label(format!("计算电流: {:.2} A", circuit.current));
                        ui.label(format!("1.1倍电流: {:.2} A", circuit.current_1_1x));
                        ui.label(format!("1.25倍电流: {:.2} A", circuit.current_1_25x));
                        ui.label(format!("元器件类型: {}", circuit.component_type));
                        ui.label(format!("元器件电流: {:.0} A", circuit.component_current));
                        ui.label(format!("线缆规格: {}", circuit.cable_spec));
                        if let Some(phase) = circuit.phase {
                            ui.label(format!("相序: {}", phase));
                        }
                        ui.label(format!("回路编号: {}", circuit.circuit_number));
                    });
                },
                PowerGraphNode::DistributionBoxNode(box_node) => {
                    ui.heading("配电箱属性");
                    ui.text_edit_singleline(&mut box_node.name);
                    ui.add(egui::Slider::new(&mut box_node.floor, 1..=50).text("所在楼层"));

                    // 显示计算结果
                    ui.group(|ui| {
                        ui.label(format!("总功率: {:.2} kW", box_node.total_power));
                        ui.label(format!("总电流: {:.2} A", box_node.total_current));
                        ui.label(format!("进线保护电流: {:.0} A", box_node.incoming_current));
                        ui.label(format!("L1相负载: {:.2} kW", box_node.phase_loads[0]));
                        ui.label(format!("L2相负载: {:.2} kW", box_node.phase_loads[1]));
                        ui.label(format!("L3相负载: {:.2} kW", box_node.phase_loads[2]));
                    });

                    // 三相平衡控制
                    if ui.button("重新平衡三相").clicked() {
                        self.data_flow_manager.mark_node_for_update(node_id);
                        self.run_calculations();
                    }
                },
                PowerGraphNode::TrunkLineNode(system_node) => {
                    ui.heading("干线系统图属性");
                    ui.checkbox(&mut system_node.auto_layout, "自动布局");

                    // 显示系统类型选择
                    ui.label("包含系统图:");
                    // 这里需要根据实际的数据结构实现系统类型选择
                    
                    if ui.button("更新系统图").clicked() {
                        self.data_flow_manager.mark_node_for_update(node_id);
                        self.run_calculations();
                    }
                },
                _ => {
                    // 其他类型节点的属性显示
                    ui.label(format!("节点名称: {}", node.label));
                    ui.label(format!("节点类型: {:?}", node.user_data.node_type));
                }
            }
        }
    }
    
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

    // 调试面板UI
    fn show_debug_panel(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.heading("调试信息面板");
            
            // 调试控制面板
            ui.horizontal(|ui| {
                // 日志记录控制
                if ui.checkbox(&mut self.debug_logger.is_recording, "启用日志记录").changed() {
                    log_info!(format!("日志记录已{}", 
                        if self.debug_logger.is_recording { "开启" } else { "关闭" }));
                }
                
                // 日志级别控制
                ui.label("日志级别:");
                if ui.button("错误").clicked() {
                    self.debug_logger.set_level(crate::application::debug_logger::LogLevel::Error);
                    log_info!("日志级别已设置为: 错误");
                }
                if ui.button("警告").clicked() {
                    self.debug_logger.set_level(crate::application::debug_logger::LogLevel::Warning);
                    log_info!("日志级别已设置为: 警告");
                }
                if ui.button("信息").clicked() {
                    self.debug_logger.set_level(crate::application::debug_logger::LogLevel::Info);
                    log_info!("日志级别已设置为: 信息");
                }
                if ui.button("调试").clicked() {
                    self.debug_logger.set_level(crate::application::debug_logger::LogLevel::Debug);
                    log_info!("日志级别已设置为: 调试");
                }
                if ui.button("追踪").clicked() {
                    self.debug_logger.set_level(crate::application::debug_logger::LogLevel::Trace);
                    log_info!("日志级别已设置为: 追踪");
                }
            });
            
            ui.separator();
            
            // 应用状态信息
            egui::CollapsingHeader::new("应用状态信息")
                .default_open(false)
                .show(ui, |ui| {
                    ui.label(format!("节点总数: {}", self.editor_state.graph.nodes.len()));
                    ui.label(format!("连接总数: {}", self.editor_state.graph.connections.len()));
                    ui.label(format!("计算缓存条目: {}", self.calculation_cache.len()));
                    ui.label(format!("当前FPS: {:.1}", self.performance_optimizer.get_stats().current_fps));
                    
                    // 最近计算的节点信息
                    if let Some(selected_node_id) = self.editor_state.selected_node() {
                        ui.label(format!("选中节点: {:?}", selected_node_id));
                    }
                });
            
            ui.separator();
            
            // 调试操作按钮
            ui.horizontal(|ui| {
                if ui.button("清空所有日志").clicked() {
                    LOGGER.clear_logs();
                    self.debug_logger.clear();
                    log_info!("所有日志已清空");
                }
                
                if ui.button("导出日志").clicked() {
                    // 这里应该打开文件对话框让用户选择保存位置
                    // 目前使用一个固定的文件名作为示例
                    let log_file = format!("pdsd_logs_{}.txt", chrono::Local::now().format("%Y%m%d_%H%M%S"));
                    if let Ok(_) = LOGGER.export_logs(&log_file) {
                        log_info!(format!("日志已导出到: {}", log_file));
                        self.error_message = Some(format!("日志已导出到: {}", log_file));
                    } else {
                        log_error!("日志导出失败");
                        self.error_message = Some("日志导出失败".to_string());
                    }
                }
                
                if ui.button("清除缓存").clicked() {
                    self.calculation_cache.clear();
                    self.performance_optimizer.clear_cache();
                    log_info!("所有缓存已清除");
                    self.error_message = Some("所有缓存已清除".to_string());
                }
            });
        });
    }
    
    // 记录节点执行情况
    fn log_node_execution(&self, node_id: egui_node_graph::NodeId, inputs: &[UIValueType], 
                        result: Option<UIValueType>, level: LogLevel) {
        // 根据日志级别记录执行信息
        let node_info = match self.editor_state.graph.nodes.get(node_id) {
            Some(node) => format!("{} (ID: {:?})", node.label, node_id),
            None => format!("节点ID: {:?}", node_id)
        };
        
        let log_message = format!("执行节点: {}", node_info);
        let input_data = format!("输入参数: {:?}", inputs);
        let result_data = format!("执行结果: {:?}", result);
        
        // 使用全局日志系统记录
        match level {
            LogLevel::Error => LOGGER.error(&log_message, module_path!(), file!(), line!(), Some(format!("{}\n{}", input_data, result_data))),
            LogLevel::Warning => LOGGER.warn(&log_message, module_path!(), file!(), line!(), Some(format!("{}\n{}", input_data, result_data))),
            LogLevel::Info => LOGGER.info(&log_message, module_path!(), file!(), line!(), Some(format!("{}\n{}", input_data, result_data))),
            LogLevel::Debug => LOGGER.debug(&log_message, module_path!(), file!(), line!(), Some(format!("{}\n{}", input_data, result_data))),
            LogLevel::Trace => LOGGER.trace(&log_message, module_path!(), file!(), line!(), Some(format!("{}\n{}", input_data, result_data))),
        }
        
        // 同时更新调试日志管理器
        let full_message = format!("{}\n{}\n{}", log_message, input_data, result_data);
        match level {
            LogLevel::Error => self.debug_logger.error(&full_message),
            LogLevel::Warning => self.debug_logger.warning(&full_message),
            LogLevel::Info => self.debug_logger.info(&full_message),
            LogLevel::Debug => self.debug_logger.debug(&full_message),
            LogLevel::Trace => self.debug_logger.trace(&full_message),
        }
    }
    
    // 运行电气系统计算
    fn run_calculations(&mut self) {
        // 使用多级日志系统记录信息
        log_info!("开始执行电气系统计算");
        self.debug_logger.info("开始执行电气系统计算");
        
        // 记录计算开始时间
        let calc_start_time = Instant::now();
        
        // 1. 标记所有节点需要更新
        for node_id in self.editor_state.graph.nodes.keys() {
            self.data_flow_manager.mark_node_for_update(*node_id);
        }
        
        // 2. 执行图计算逻辑
        self.execute_graph();
        
        // 3. 触发数据流向更新
        self.data_flow_manager.propagate_updates(&mut self.editor_state.graph);
        
        // 记录计算耗时
        let calc_duration = calc_start_time.elapsed();
        log_info!(format!("电气系统计算完成，耗时: {:.2} ms", calc_duration.as_secs_f64() * 1000.0));
        self.debug_logger.info("电气系统计算完成");
        
        // 记录操作耗时到性能监控器
        self.performance_optimizer.update_update_time(calc_duration);
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
        for response in responses {
            match response {
                egui_node_graph::NodeResponse::NodeSelected(node_id) => {
                    // 处理节点选择
                    self.editor_state.set_selected_node(Some(node_id));
                },
                egui_node_graph::NodeResponse::NodeDeselected => {
                    // 处理节点取消选择
                    self.editor_state.set_selected_node(None);
                },
                egui_node_graph::NodeResponse::ConnectionCreated(from, to) => {
                    // 处理连接创建
                    self.handle_new_connection(from, to);
                },
                egui_node_graph::NodeResponse::NodeMoved(node_id, _position) => {
                    // 处理节点移动，检查是否拖入配电箱
                    if let Some(target_box) = self.check_if_node_dropped_into_box(node_id) {
                        self.move_circuit_to_box(node_id, target_box);
                    }
                },
                egui_node_graph::NodeResponse::NodeDeleted(node_id) => {
                    // 处理节点删除
                    self.handle_node_deletion(node_id);
                },
                _ => {}
            }
        }
    }
    
    // 检查节点是否拖入配电箱
    fn check_if_node_dropped_into_box(&self, node_id: egui_node_graph::NodeId) -> Option<egui_node_graph::NodeId> {
        // 1. 获取被移动节点的位置
        let node_pos = match self.editor_state.editor_state.node_positions.get(&node_id) {
            Some(pos) => pos,
            None => return None,
        };

        // 2. 检查是否在某个配电箱节点范围内
        for (box_id, box_node) in &self.editor_state.graph.nodes {
            if let Some(box_pos) = self.editor_state.editor_state.node_positions.get(box_id) {
                if let PowerGraphNode::DistributionBoxNode(_) = &box_node.user_data {
                    // 简化处理，实际应用中应检查精确的边界
                    if (node_pos.0 - box_pos.0).abs() < 100.0 &&
                       (node_pos.1 - box_pos.1).abs() < 100.0 {
                        return Some(*box_id);
                    }
                }
            }
        }

        None
    }

    // 移动配电回路到配电箱
    fn move_circuit_to_box(&mut self, circuit_id: egui_node_graph::NodeId, box_id: egui_node_graph::NodeId) {
        // 1. 检查节点类型
        if !matches!(&self.editor_state.graph.nodes[circuit_id].user_data, PowerGraphNode::CircuitNode(_)) ||
           !matches!(&self.editor_state.graph.nodes[box_id].user_data, PowerGraphNode::DistributionBoxNode(_)) {
            return;
        }

        // 2. 创建连接（如果存在输入和输出端口）
        if let Some(circuit_output) = self.editor_state.graph.nodes[circuit_id].outputs.values().next() {
            if let Some(box_input) = self.editor_state.graph.nodes[box_id].inputs.values().next() {
                // 移除可能存在的旧连接
                self.editor_state.graph.connections.remove(box_input);
                // 创建新连接
                self.editor_state.graph.connections.insert(*box_input, *circuit_output);

                // 3. 标记节点需要更新
                self.data_flow_manager.mark_node_for_update(box_id);

                // 4. 触发更新传播
                self.run_calculations();
            }
        }
    }

    // 处理新连接创建
    fn handle_new_connection(&mut self, from: egui_node_graph::Connection, to: egui_node_graph::Connection) {
        // 记录连接创建日志
        self.debug_logger.info(&format!("创建连接: 从节点{}到节点{}", 
            from.node_id.0, to.node_id.0));
        
        // 标记目标节点需要更新
        self.data_flow_manager.mark_node_for_update(to.node_id);
    }

    // 处理节点删除
    fn handle_node_deletion(&mut self, node_id: egui_node_graph::NodeId) {
        // 记录节点删除日志
        if let Some(node) = self.editor_state.graph.nodes.get(&node_id) {
            self.debug_logger.info(&format!("删除节点: {}, 类型: {:?}", 
                node.label, node.user_data.node_type));
        }
        
        // 清除相关缓存
        self.data_flow_manager.clear_node_cache(node_id);
        
        // 从计算缓存中移除相关条目
        let cache_keys_to_remove: Vec<String> = self.calculation_cache.keys()
            .filter(|key| key.starts_with(&node_id.0.to_string()))
            .cloned()
            .collect();
            
        for key in cache_keys_to_remove {
            self.calculation_cache.remove(&key);
        }
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

    // 执行图计算 - 利用数据流向管理器优化计算顺序和缓存
    fn execute_graph(&mut self) {
        log_debug!("开始执行图计算");
        
        // 使用数据流向管理器执行拓扑排序和更新
        // 这里我们复用数据流向管理器中的拓扑排序结果，避免重复计算
        
        // 1. 获取拓扑排序后的节点执行顺序
        let execution_order = self.data_flow_manager.perform_topological_sort(&self.editor_state.graph);
        
        // 2. 按顺序执行每个节点的计算
        for node_id in execution_order {
            // 检查是否需要更新此节点（使用性能优化器进行交互节流）
            if self.performance_optimizer.should_process_interaction(&format!("node_calc_{:?}", node_id)) {
                self.execute_node_calculation(node_id);
            }
        }
        
        log_debug!("图计算执行完成");
    }
    
    // 执行单个节点的计算
    fn execute_node_calculation(&mut self, node_id: egui_node_graph::NodeId) {
        // 记录节点计算开始
        log_trace!(format!("开始计算节点: {:?}", node_id));
        self.debug_logger.trace(&format!("开始计算节点: {:?}", node_id));
        
        // 收集输入参数
        let mut inputs = Vec::new();
        if let Some(node) = self.editor_state.graph.nodes.get(node_id) {
            // 记录计算开始时间
            let calc_start_time = Instant::now();
            
            // 根据节点类型执行不同的计算逻辑
            match &node.user_data {
                PowerGraphNode::CircuitNode(circuit) => {
                    // 执行回路计算
                    log_debug!(format!("计算回路节点: {}, 功率: {:.2}kW", 
                        circuit.name, circuit.power));
                    self.debug_logger.debug(&format!("计算回路节点: {}, 功率: {:.2}kW", 
                        circuit.name, circuit.power));
                },
                PowerGraphNode::DistributionBoxNode(box_node) => {
                    // 执行配电箱计算
                    log_debug!(format!("计算配电箱节点: {}, 回路数: {}", 
                        box_node.name, box_node.circuits.len()));
                    self.debug_logger.debug(&format!("计算配电箱节点: {}, 回路数: {}", 
                        box_node.name, box_node.circuits.len()));
                },
                PowerGraphNode::TrunkLineNode(system_node) => {
                    // 执行干线系统图计算
                    log_debug!(format!("计算干线系统图节点: {}, 类型: {:?}", 
                        system_node.name, system_node.system_type));
                    self.debug_logger.debug(&format!("计算干线系统图节点: {}, 类型: {:?}", 
                        system_node.name, system_node.system_type));
                },
                _ => {
                    // 其他类型节点的计算
                    log_debug!(format!("计算其他类型节点: {:?}", node_id));
                    self.debug_logger.debug(&format!("计算其他类型节点: {:?}", node_id));
                }
            }
            
            // 记录计算耗时
            let calc_duration = calc_start_time.elapsed();
            log_trace!(format!("节点 {:?} 计算耗时: {:.2} ms", node_id, calc_duration.as_secs_f64() * 1000.0));
            
            // 3. 从数据流向管理器获取计算结果并更新本地缓存
            if let Some(node_cache) = self.data_flow_manager.get_node_cache(node_id) {
                // 记录缓存条目数
                log_trace!(format!("节点 {:?} 获取到 {} 个缓存条目", node_id, node_cache.len()));
                
                // 将数据流向管理器中的计算结果同步到应用的计算缓存
                for (key, value) in node_cache {
                    // 转换缓存键格式
                    let cache_key = format!("{}_{}", node_id.0, key);
                    
                    // 根据值类型提取数值
                    match value {
                        ElectricValueType::Float(val) => {
                            self.calculation_cache.insert(cache_key, *val);
                            log_trace!(format!("缓存节点 {:?} 输出 {} = {:.2}", node_id, key, val));
                        },
                        ElectricValueType::Integer(val) => {
                            self.calculation_cache.insert(cache_key, *val as f64);
                            log_trace!(format!("缓存节点 {:?} 输出 {} = {}", node_id, key, val));
                        },
                        _ => {
                            // 不处理非数值类型的缓存
                            log_trace!(format!("忽略节点 {:?} 非数值类型输出: {}", node_id, key));
                        }
                    }
                }
            }
            
            // 记录节点执行情况
            self.log_node_execution(node_id, &inputs, Some(UIValueType::Float(0.0)), LogLevel::Debug);
        } else {
            log_warn!(format!("尝试计算不存在的节点: {:?}", node_id));
            self.debug_logger.warning(&format!("尝试计算不存在的节点: {:?}", node_id));
        }
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
    
    // 添加配电回路节点
    fn add_circuit_node(&mut self, circuit_type: CircuitType) {
        // 获取所有电路节点模板
        let templates = all_electric_templates();
        
        // 查找匹配的模板
        let template_name = match circuit_type {
            CircuitType::SinglePhase => "单相配电回路",
            CircuitType::ThreePhase => "三相配电回路",
        };
        
        if let Some(template) = templates.iter().find(|t| t.node_label() == template_name) {
            // 添加新节点
            let _ = self.editor_state.graph.add_node(
                template.node_label().to_string(),
                template.user_data(&mut self.editor_state.user_state),
                |graph, node_id| template.build_node(graph, &mut self.editor_state.user_state, node_id),
            );
            
            // 记录日志
            self.debug_logger.info(&format!("添加节点: {}", template_name));
        }
    }
    
    // 添加配电箱节点
    fn add_distribution_box_node(&mut self) {
        // 获取所有模板
        let templates = all_electric_templates();
        
        // 查找配电箱模板
        if let Some(template) = templates.iter().find(|t| t.node_label() == "标准配电箱") {
            // 添加新节点
            let _ = self.editor_state.graph.add_node(
                template.node_label().to_string(),
                template.user_data(&mut self.editor_state.user_state),
                |graph, node_id| template.build_node(graph, &mut self.editor_state.user_state, node_id),
            );
            
            // 记录日志
            self.debug_logger.info("添加节点: 标准配电箱");
        }
    }
    
    // 添加干线系统图节点
    fn add_main_system_node(&mut self) {
        // 获取所有模板
        let templates = all_electric_templates();
        
        // 查找干线系统图模板
        if let Some(template) = templates.iter().find(|t| t.node_label() == "干线系统图") {
            // 添加新节点
            let _ = self.editor_state.graph.add_node(
                template.node_label().to_string(),
                template.user_data(&mut self.editor_state.user_state),
                |graph, node_id| template.build_node(graph, &mut self.editor_state.user_state, node_id),
            );
            
            // 记录日志
            self.debug_logger.info("添加节点: 干线系统图");
        }
    }
}
