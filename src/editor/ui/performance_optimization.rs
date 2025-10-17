/// 性能优化模块
/// 用于提高大规模节点图的渲染和交互性能

use egui::{Ui, Checkbox, Slider, CollapsingHeader, Label, Button};
use egui_node_graph::{Graph, GraphEditorState, NodeId};
use std::time::{Instant, Duration};
use std::collections::{HashMap, HashSet};

use crate::editor::business::PowerGraphNode;
use crate::editor::{DataType, UIValueType, UIUserState};

/// 性能优化设置\#[derive(Debug, Clone, Default)]
pub struct PerformanceSettings {
    /// 启用节点视图范围裁剪
    pub enable_view_culling: bool,
    /// 启用节点渲染LOD（细节层次）
    pub enable_lod: bool,
    /// 启用连接优化
    pub enable_connection_optimization: bool,
    /// 启用交互节流
    pub enable_interaction_throttling: bool,
    /// 视图裁剪边距
    pub culling_margin: f32,
    /// 低细节级别距离阈值
    pub low_detail_distance: f32,
    /// 最低细节级别距离阈值
    pub minimal_detail_distance: f32,
    /// 交互节流最小时间间隔（毫秒）
    pub interaction_throttle_ms: u64,
    /// 节点可见性缓存有效期（毫秒）
    pub visibility_cache_ttl_ms: u64,
    /// 仅在鼠标悬停时显示节点详情
    pub show_node_details_on_hover_only: bool,
}

/// 可见性缓存条目\#[derive(Debug, Clone)]
struct VisibilityCacheEntry {
    /// 是否可见
    visible: bool,
    /// 缓存时间戳
    timestamp: Instant,
    /// 细节级别
    detail_level: NodeDetailLevel,
}

/// 节点细节级别\#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum NodeDetailLevel {
    /// 完整细节
    Full,
    /// 中等细节
    Medium,
    /// 低细节
    Low,
    /// 最小细节（仅轮廓）
    Minimal,
}

/// 性能优化管理器\#[derive(Debug, Default)]
pub struct PerformanceOptimizer {
    /// 性能设置
    pub settings: PerformanceSettings,
    /// 节点可见性缓存
    visibility_cache: HashMap<NodeId, VisibilityCacheEntry>,
    /// 最后交互时间
    last_interaction_time: HashMap<String, Instant>,
    /// 视口信息
    current_viewport: Option<egui::Rect>,
    /// 基准点（用于计算节点距离）
    reference_point: Option<egui::Pos2>,
    /// 性能统计
    performance_stats: PerformanceStats,
}

/// 性能统计数据\#[derive(Debug, Default, Clone)]
pub struct PerformanceStats {
    /// 渲染的节点总数
    rendered_nodes: usize,
    /// 被裁剪的节点数
    culled_nodes: usize,
    /// 低细节渲染的节点数
    low_detail_nodes: usize,
    /// 最小细节渲染的节点数
    minimal_detail_nodes: usize,
    /// 渲染连接总数
    rendered_connections: usize,
    /// 优化的连接数
    optimized_connections: usize,
    /// 渲染耗时（毫秒）
    render_time_ms: f64,
    /// 更新耗时（毫秒）
    update_time_ms: f64,
    /// 当前FPS
    current_fps: f32,
}

impl PerformanceOptimizer {
    /// 创建新的性能优化管理器
    pub fn new() -> Self {
        let mut settings = PerformanceSettings::default();
        // 设置默认优化参数
        settings.enable_view_culling = true;
        settings.enable_lod = true;
        settings.enable_connection_optimization = true;
        settings.enable_interaction_throttling = true;
        settings.culling_margin = 200.0;
        settings.low_detail_distance = 300.0;
        settings.minimal_detail_distance = 600.0;
        settings.interaction_throttle_ms = 50;
        settings.visibility_cache_ttl_ms = 100;
        
        Self {
            settings,
            ..Default::default()
        }
    }
    
    /// 更新视口信息
    pub fn update_viewport(&mut self, viewport: egui::Rect) {
        self.current_viewport = Some(viewport);
        // 清空过期的可见性缓存
        self.clear_expired_cache();
    }
    
    /// 设置参考点
    pub fn set_reference_point(&mut self, point: egui::Pos2) {
        self.reference_point = Some(point);
    }
    
    /// 检查节点是否应该被渲染（考虑视口裁剪）
    pub fn should_render_node(
        &mut self,
        node_id: NodeId,
        node_pos: egui::Pos2,
        node_size: egui::Vec2,
    ) -> (bool, NodeDetailLevel) {
        // 如果禁用了视图裁剪，所有节点都可见
        if !self.settings.enable_view_culling {
            return (true, NodeDetailLevel::Full);
        }
        
        // 检查缓存
        if let Some(entry) = self.get_valid_cache_entry(node_id) {
            return (entry.visible, entry.detail_level);
        }
        
        // 计算节点边界
        let node_rect = egui::Rect::from_min_size(node_pos, node_size);
        
        // 获取视口信息，如果不可用则所有节点都可见
        let viewport = match self.current_viewport {
            Some(rect) => rect,
            None => {
                self.update_visibility_cache(node_id, true, NodeDetailLevel::Full);
                return (true, NodeDetailLevel::Full);
            },
        };
        
        // 添加边距到视口
        let expanded_viewport = viewport.expand(self.settings.culling_margin);
        
        // 检查节点是否在扩展视口内
        let visible = expanded_viewport.intersects(node_rect);
        
        // 确定细节级别
        let detail_level = if visible && self.settings.enable_lod {
            self.calculate_detail_level(node_pos)
        } else if visible {
            NodeDetailLevel::Full
        } else {
            // 不可见节点不需要细节级别
            NodeDetailLevel::Full
        };
        
        // 更新缓存
        self.update_visibility_cache(node_id, visible, detail_level);
        
        // 更新统计数据
        self.performance_stats.rendered_nodes += visible as usize;
        self.performance_stats.culled_nodes += (!visible) as usize;
        
        if visible {
            match detail_level {
                NodeDetailLevel::Low => self.performance_stats.low_detail_nodes += 1,
                NodeDetailLevel::Minimal => self.performance_stats.minimal_detail_nodes += 1,
                _ => {},
            }
        }
        
        (visible, detail_level)
    }
    
    /// 检查连接是否应该被渲染（考虑优化）
    pub fn should_render_connection(
        &mut self,
        start_node_id: NodeId,
        end_node_id: NodeId,
    ) -> bool {
        // 如果禁用了连接优化，所有连接都渲染
        if !self.settings.enable_connection_optimization {
            self.performance_stats.rendered_connections += 1;
            return true;
        }
        
        // 检查两个节点是否都可见
        let start_visible = self.get_valid_cache_entry(start_node_id)
            .map(|entry| entry.visible)
            .unwrap_or(true);
        
        let end_visible = self.get_valid_cache_entry(end_node_id)
            .map(|entry| entry.visible)
            .unwrap_or(true);
        
        // 如果两个节点都不可见，则连接也不渲染
        if !start_visible && !end_visible {
            self.performance_stats.optimized_connections += 1;
            return false;
        }
        
        // 两个节点都可见或一个可见，则渲染连接
        self.performance_stats.rendered_connections += 1;
        return true;
    }
    
    /// 检查交互是否应该被处理（考虑节流）
    pub fn should_process_interaction(&mut self, interaction_id: &str) -> bool {
        // 如果禁用了交互节流，所有交互都处理
        if !self.settings.enable_interaction_throttling {
            return true;
        }
        
        // 检查上次交互时间
        let now = Instant::now();
        let should_process = match self.last_interaction_time.get(interaction_id) {
            Some(last_time) => {
                now.duration_since(*last_time).as_millis() as u64 >= self.settings.interaction_throttle_ms
            },
            None => true,
        };
        
        // 更新最后交互时间
        if should_process {
            self.last_interaction_time.insert(interaction_id.to_string(), now);
        }
        
        should_process
    }
    
    /// 计算节点的细节级别
    fn calculate_detail_level(&self, node_pos: egui::Pos2) -> NodeDetailLevel {
        // 获取参考点（默认为视图中心）
        let reference = match self.reference_point {
            Some(point) => point,
            None => return NodeDetailLevel::Full, // 没有参考点时使用完整细节
        };
        
        // 计算距离
        let distance = reference.distance(node_pos);
        
        // 根据距离确定细节级别
        if distance >= self.settings.minimal_detail_distance {
            NodeDetailLevel::Minimal
        } else if distance >= self.settings.low_detail_distance {
            NodeDetailLevel::Low
        } else {
            NodeDetailLevel::Full
        }
    }
    
    /// 获取有效的缓存条目
    fn get_valid_cache_entry(&self, node_id: NodeId) -> Option<&VisibilityCacheEntry> {
        self.visibility_cache.get(&node_id).and_then(|entry| {
            let age = Instant::now().duration_since(entry.timestamp);
            if age.as_millis() as u64 <= self.settings.visibility_cache_ttl_ms {
                Some(entry)
            } else {
                None
            }
        })
    }
    
    /// 更新可见性缓存
    fn update_visibility_cache(&mut self, node_id: NodeId, visible: bool, detail_level: NodeDetailLevel) {
        self.visibility_cache.insert(node_id, VisibilityCacheEntry {
            visible,
            timestamp: Instant::now(),
            detail_level,
        });
    }
    
    /// 清空过期的缓存条目
    fn clear_expired_cache(&mut self) {
        let now = Instant::now();
        self.visibility_cache.retain(|_, entry| {
            now.duration_since(entry.timestamp).as_millis() as u64 <= self.settings.visibility_cache_ttl_ms
        });
    }
    
    /// 清空所有缓存
    pub fn clear_cache(&mut self) {
        self.visibility_cache.clear();
        self.last_interaction_time.clear();
    }
    
    /// 重置性能统计
    pub fn reset_stats(&mut self) {
        self.performance_stats = PerformanceStats::default();
    }
    
    /// 获取性能统计
    pub fn get_stats(&self) -> &PerformanceStats {
        &self.performance_stats
    }
    
    /// 更新渲染时间统计
    pub fn update_render_time(&mut self, duration: Duration) {
        self.performance_stats.render_time_ms = duration.as_secs_f64() * 1000.0;
    }
    
    /// 更新更新时间统计
    pub fn update_update_time(&mut self, duration: Duration) {
        self.performance_stats.update_time_ms = duration.as_secs_f64() * 1000.0;
    }
    
    /// 更新FPS统计
    pub fn update_fps(&mut self, fps: f32) {
        self.performance_stats.current_fps = fps;
    }
    
    /// 对节点进行批量优化处理
    pub fn optimize_nodes_batch(
        &mut self,
        graph: &Graph<PowerGraphNode, DataType, UIValueType>,
        editor_state: &GraphEditorState<PowerGraphNode, DataType, UIValueType, UIUserState>,
    ) -> (HashSet<NodeId>, HashMap<NodeId, NodeDetailLevel>) {
        let mut visible_nodes = HashSet::new();
        let mut detail_levels = HashMap::new();
        
        // 重置统计
        self.reset_stats();
        
        // 对每个节点进行可见性和细节级别计算
        for (node_id, _) in &graph.nodes {
            if let Some(pos) = editor_state.node_positions.get(node_id) {
                // 估计节点大小
                let node_size = egui::Vec2::new(200.0, 150.0);
                
                // 检查是否可见以及细节级别
                let (visible, detail_level) = self.should_render_node(*node_id, *pos, node_size);
                
                if visible {
                    visible_nodes.insert(*node_id);
                    detail_levels.insert(*node_id, detail_level);
                }
            }
        }
        
        (visible_nodes, detail_levels)
    }
}

/// 性能设置UI
pub fn performance_settings_ui(ui: &mut Ui, optimizer: &mut PerformanceOptimizer) -> bool {
    let mut changed = false;
    
    ui.group(|ui| {
        ui.heading("性能设置");
        
        // 启用/禁用选项
        ui.checkbox(&mut optimizer.settings.enable_view_culling, "启用视图裁剪")
            .on_change(|_| changed = true);
        
        ui.checkbox(&mut optimizer.settings.enable_lod, "启用细节级别")
            .on_change(|_| changed = true);
        
        ui.checkbox(&mut optimizer.settings.enable_connection_optimization, "启用连接优化")
            .on_change(|_| changed = true);
        
        ui.checkbox(&mut optimizer.settings.enable_interaction_throttling, "启用交互节流")
            .on_change(|_| changed = true);
        
        ui.checkbox(&mut optimizer.settings.show_node_details_on_hover_only, "仅在悬停时显示节点详情")
            .on_change(|_| changed = true);
        
        // 裁剪设置
        if optimizer.settings.enable_view_culling {
            ui.add_space(10.0);
            ui.label("视图裁剪边距:");
            ui.add(Slider::new(&mut optimizer.settings.culling_margin, 0.0..=500.0))
                .on_change(|_| changed = true);
        }
        
        // LOD设置
        if optimizer.settings.enable_lod {
            ui.add_space(10.0);
            ui.label("低细节距离阈值:");
            ui.add(Slider::new(&mut optimizer.settings.low_detail_distance, 100.0..=1000.0))
                .on_change(|_| changed = true);
            
            ui.label("最小细节距离阈值:");
            ui.add(Slider::new(&mut optimizer.settings.minimal_detail_distance, 200.0..=2000.0))
                .on_change(|_| changed = true);
        }
        
        // 交互节流设置
        if optimizer.settings.enable_interaction_throttling {
            ui.add_space(10.0);
            ui.label("交互节流间隔 (毫秒):");
            ui.add(Slider::new(&mut optimizer.settings.interaction_throttle_ms, 10..=200))
                .on_change(|_| changed = true);
        }
        
        // 缓存设置
        ui.add_space(10.0);
        ui.label("可见性缓存有效期 (毫秒):");
        ui.add(Slider::new(&mut optimizer.settings.visibility_cache_ttl_ms, 50..=500))
            .on_change(|_| changed = true);
        
        // 清除缓存按钮
        if ui.button("清除所有缓存").clicked() {
            optimizer.clear_cache();
        }
    });
    
    changed
}

/// 性能统计UI
pub fn performance_stats_ui(ui: &mut Ui, optimizer: &PerformanceOptimizer) {
    ui.group(|ui| {
        ui.heading("性能统计");
        
        let stats = optimizer.get_stats();
        
        CollapsingHeader::new("渲染统计")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(format!("当前FPS: {:.1}", stats.current_fps));
                ui.label(format!("渲染节点: {}", stats.rendered_nodes));
                ui.label(format!("裁剪节点: {}", stats.culled_nodes));
                ui.label(format!("低细节节点: {}", stats.low_detail_nodes));
                ui.label(format!("最小细节节点: {}", stats.minimal_detail_nodes));
                ui.label(format!("渲染连接: {}", stats.rendered_connections));
                ui.label(format!("优化连接: {}", stats.optimized_connections));
            });
        
        CollapsingHeader::new("性能指标")
            .default_open(false)
            .show(ui, |ui| {
                ui.label(format!("渲染耗时: {:.2} ms", stats.render_time_ms));
                ui.label(format!("更新耗时: {:.2} ms", stats.update_time_ms));
                
                // 计算渲染性能比率
                let total_time = stats.render_time_ms + stats.update_time_ms;
                if total_time > 0.0 {
                    let render_ratio = (stats.render_time_ms / total_time) * 100.0;
                    let update_ratio = (stats.update_time_ms / total_time) * 100.0;
                    
                    ui.label(format!("渲染时间占比: {:.1}%", render_ratio));
                    ui.label(format!("更新时间占比: {:.1}%", update_ratio));
                }
            });
    });
}

/// 视图优化管理器\#[derive(Debug, Default)]
pub struct ViewOptimizer {
    /// 可见区域历史记录
    view_history: VecDeque<egui::Rect>,
    /// 视图稳定性评估
    view_stability: f32,
    /// 最大历史记录数
    max_history: usize,
}

impl ViewOptimizer {
    /// 创建新的视图优化器
    pub fn new(max_history: usize) -> Self {
        Self {
            view_history: VecDeque::with_capacity(max_history),
            view_stability: 1.0,
            max_history,
        }
    }
    
    /// 更新当前视图
    pub fn update_view(&mut self, view: egui::Rect) {
        // 添加新视图到历史记录
        self.view_history.push_back(view);
        
        // 限制历史记录大小
        if self.view_history.len() > self.max_history {
            self.view_history.pop_front();
        }
        
        // 计算视图稳定性
        self.calculate_view_stability();
    }
    
    /// 计算视图稳定性（0.0-1.0，值越高表示视图越稳定）
    fn calculate_view_stability(&mut self) {
        if self.view_history.len() < 2 {
            self.view_stability = 1.0;
            return;
        }
        
        // 获取最近两次视图
        let prev_view = self.view_history[self.view_history.len() - 2];
        let current_view = *self.view_history.back().unwrap();
        
        // 计算视图变化程度
        let translation_diff = (prev_view.min - current_view.min).length();
        let size_diff = (prev_view.size() - current_view.size()).length();
        
        // 归一化并计算稳定性分数
        // 假设典型的平移变化不超过100像素，缩放变化不超过50像素
        let normalized_translation = (translation_diff / 100.0).min(1.0);
        let normalized_size = (size_diff / 50.0).min(1.0);
        
        // 稳定性 = 1.0 - 变化程度
        self.view_stability = 1.0 - (normalized_translation * 0.7 + normalized_size * 0.3);
    }
    
    /// 获取当前视图稳定性
    pub fn get_view_stability(&self) -> f32 {
        self.view_stability
    }
    
    /// 根据视图稳定性调整渲染质量
    pub fn adjust_render_quality(&self, base_quality: f32) -> f32 {
        // 视图越稳定，渲染质量可以越高
        // 视图不稳定时，降低渲染质量以提高性能
        base_quality * (0.5 + 0.5 * self.view_stability)
    }
}