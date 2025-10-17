/// 节点搜索和过滤功能实现
/// 用于快速定位和筛选图中的节点

use egui::{Ui, TextEdit, Checkbox, ScrollArea, Response, Button};
use egui_node_graph::{NodeId, Graph, GraphEditorState};
use std::collections::HashSet;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use std::sync::LazyLock;

use crate::editor::business::{PowerGraphNode, ElectricNodeTemplate};
use crate::editor::{DataType, UIValueType, UIUserState};

/// 静态模糊匹配器
static FUZZY_MATCHER: LazyLock<SkimMatcherV2> = LazyLock::new(SkimMatcherV2::default);

/// 搜索结果排序方式\#[derive(Debug, Clone, PartialEq, Eq)]
enum SearchResultOrder {
    /// 按相关性排序
    Relevance,
    /// 按节点类型排序
    Type,
    /// 按字母顺序排序
    Alphabetical,
}

/// 节点搜索器\#[derive(Debug, Default)]
pub struct NodeSearcher {
    /// 搜索关键词
    pub search_text: String,
    /// 搜索结果
    pub results: Vec<(NodeId, i64)>, // 节点ID和匹配分数
    /// 搜索结果排序方式
    pub order: SearchResultOrder,
    /// 过滤条件
    pub filters: NodeFilters,
    /// 上一次的搜索关键词（用于检测变化）
    pub last_search_text: String,
    /// 上一次的过滤条件（用于检测变化）
    pub last_filters: NodeFilters,
}

/// 节点过滤条件\#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct NodeFilters {
    /// 按节点类型过滤
    pub node_types: HashSet<ElectricNodeTemplate>,
    /// 按数据类型过滤（基于节点的输入/输出）
    pub data_types: HashSet<DataType>,
    /// 只显示选中的节点
    pub only_selected: bool,
    /// 只显示有连接的节点
    pub only_connected: bool,
    /// 只显示错误节点
    pub only_errors: bool,
    /// 只显示当前分组的节点
    pub only_current_group: Option<String>,
}

impl NodeSearcher {
    /// 创建新的节点搜索器
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 更新搜索结果
    pub fn update_search(
        &mut self,
        graph: &Graph<PowerGraphNode, DataType, UIValueType>,
        editor_state: &GraphEditorState<PowerGraphNode, DataType, UIValueType, UIUserState>,
    ) {
        // 检查搜索条件是否有变化
        let search_changed = 
            self.search_text != self.last_search_text || 
            self.filters != self.last_filters;
        
        if !search_changed && !self.search_text.is_empty() {
            return; // 没有变化，不需要重新搜索
        }
        
        // 更新最后搜索条件
        self.last_search_text = self.search_text.clone();
        self.last_filters = self.filters.clone();
        
        // 清空之前的结果
        self.results.clear();
        
        // 如果搜索文本为空且没有应用过滤器，则不执行搜索
        if self.search_text.is_empty() && 
           self.filters.node_types.is_empty() &&
           self.filters.data_types.is_empty() &&
           !self.filters.only_selected &&
           !self.filters.only_connected &&
           !self.filters.only_errors &&
           self.filters.only_current_group.is_none() {
            return;
        }
        
        // 遍历所有节点进行搜索和过滤
        for (node_id, node) in &graph.nodes {
            // 应用过滤器
            if !self.apply_filters(node_id, node, graph, editor_state) {
                continue;
            }
            
            // 计算匹配分数
            let score = self.calculate_match_score(node);
            
            // 只添加有匹配分数的结果
            if score.is_some() {
                self.results.push((node_id, score.unwrap()));
            }
        }
        
        // 排序结果
        self.sort_results();
    }
    
    /// 应用过滤条件
    fn apply_filters(
        &self,
        node_id: &NodeId,
        node: &PowerGraphNode,
        graph: &Graph<PowerGraphNode, DataType, UIValueType>,
        editor_state: &GraphEditorState<PowerGraphNode, DataType, UIValueType, UIUserState>,
    ) -> bool {
        // 按节点类型过滤
        if !self.filters.node_types.is_empty() {
            // 获取节点类型（需要根据实际情况实现）
            let node_type = self.get_node_type(node);
            if !self.filters.node_types.contains(&node_type) {
                return false;
            }
        }
        
        // 按数据类型过滤
        if !self.filters.data_types.is_empty() {
            let mut has_matching_data_type = false;
            
            // 检查输入端口数据类型
            for (_input_id, input) in &node.inputs {
                if self.filters.data_types.contains(&input.data_type) {
                    has_matching_data_type = true;
                    break;
                }
            }
            
            // 检查输出端口数据类型
            if !has_matching_data_type {
                for (_output_id, output) in &node.outputs {
                    if self.filters.data_types.contains(&output.data_type) {
                        has_matching_data_type = true;
                        break;
                    }
                }
            }
            
            if !has_matching_data_type {
                return false;
            }
        }
        
        // 只显示选中的节点
        if self.filters.only_selected && 
           !editor_state.selected_nodes.contains(node_id) {
            return false;
        }
        
        // 只显示有连接的节点
        if self.filters.only_connected {
            let has_connections = graph.input_connections.iter()
                .any(|(input_node_id, _, _)| input_node_id == node_id) ||
                graph.output_connections.iter()
                .any(|(output_node_id, _, _, _)| output_node_id == node_id);
                
            if !has_connections {
                return false;
            }
        }
        
        // 只显示错误节点（需要根据实际情况实现错误检测逻辑）
        if self.filters.only_errors && !self.has_node_errors(node) {
            return false;
        }
        
        // TODO: 实现按分组过滤
        
        true
    }
    
    /// 计算节点与搜索关键词的匹配分数
    fn calculate_match_score(&self, node: &PowerGraphNode) -> Option<i64> {
        if self.search_text.is_empty() {
            return Some(0); // 没有搜索关键词时，所有通过过滤器的节点都有基础分数
        }
        
        // 获取节点的搜索文本
        let search_text = self.get_node_search_text(node);
        
        // 使用模糊匹配算法
        FUZZY_MATCHER.fuzzy_match(&search_text, &self.search_text)
    }
    
    /// 排序搜索结果
    fn sort_results(&mut self) {
        match self.order {
            SearchResultOrder::Relevance => {
                // 按匹配分数降序排序
                self.results.sort_by(|a, b| b.1.cmp(&a.1));
            },
            SearchResultOrder::Type => {
                // 按节点类型排序（需要实现节点类型比较）
                // 这里简化为按节点ID排序
                self.results.sort_by(|a, b| a.0.cmp(&b.0));
            },
            SearchResultOrder::Alphabetical => {
                // 按字母顺序排序（需要实现节点名称比较）
                // 这里简化为按节点ID排序
                self.results.sort_by(|a, b| a.0.cmp(&b.0));
            },
        }
    }
    
    /// 获取节点类型（需要根据实际情况实现）
    fn get_node_type(&self, node: &PowerGraphNode) -> ElectricNodeTemplate {
        // 这是一个简化实现，实际应该根据node.data来判断
        ElectricNodeTemplate::CircuitNode
    }
    
    /// 获取节点的搜索文本（名称、描述等）
    fn get_node_search_text(&self, node: &PowerGraphNode) -> String {
        // 组合节点ID和其他属性作为搜索文本
        format!("{} {:?}", node.id, node.data)
    }
    
    /// 检查节点是否有错误
    fn has_node_errors(&self, _node: &PowerGraphNode) -> bool {
        // 简化实现，实际应该检查节点的错误状态
        false
    }
}

/// 搜索UI组件
pub fn node_search_ui(
    ui: &mut Ui,
    searcher: &mut NodeSearcher,
    graph: &Graph<PowerGraphNode, DataType, UIValueType>,
    editor_state: &mut GraphEditorState<PowerGraphNode, DataType, UIValueType, UIUserState>,
) -> Response {
    let mut response = Response::default();
    
    ui.group(|ui| {
        ui.heading("节点搜索");
        
        // 搜索框
        ui.horizontal(|ui| {
            ui.label("搜索:");
            if ui.text_edit_singleline(&mut searcher.search_text).changed() {
                // 搜索文本变化时更新搜索
                searcher.update_search(graph, editor_state);
            }
            
            // 清除按钮
            if ui.button("清除").clicked() {
                searcher.search_text.clear();
                searcher.results.clear();
            }
        });
        
        // 排序选项
        ui.horizontal(|ui| {
            ui.label("排序:");
            
            if ui.radio(searcher.order == SearchResultOrder::Relevance, "相关性").clicked() {
                searcher.order = SearchResultOrder::Relevance;
                searcher.sort_results();
            }
            
            if ui.radio(searcher.order == SearchResultOrder::Type, "类型").clicked() {
                searcher.order = SearchResultOrder::Type;
                searcher.sort_results();
            }
            
            if ui.radio(searcher.order == SearchResultOrder::Alphabetical, "字母顺序").clicked() {
                searcher.order = SearchResultOrder::Alphabetical;
                searcher.sort_results();
            }
        });
        
        // 过滤选项
        ui.collapsing("过滤选项", |ui| {
            ui.checkbox(&mut searcher.filters.only_selected, "只显示选中节点");
            ui.checkbox(&mut searcher.filters.only_connected, "只显示有连接的节点");
            ui.checkbox(&mut searcher.filters.only_errors, "只显示错误节点");
            
            // TODO: 添加更多过滤选项
        });
        
        // 搜索结果
        ui.separator();
        ui.label(format!("找到 {} 个结果", searcher.results.len()));
        
        ScrollArea::vertical().show(ui, |ui| {
            for (node_id, score) in &searcher.results {
                let node_response = ui.horizontal(|ui| {
                    // 显示节点信息
                    let node_name = if let Some(node) = graph.nodes.get(node_id) {
                        node.id.clone()
                    } else {
                        "未知节点".to_string()
                    };
                    
                    ui.label(node_name);
                    
                    // 显示匹配分数（仅用于调试）
                    if ui.button("查看").clicked() {
                        // 选中并居中显示节点
                        editor_state.selected_nodes.clear();
                        editor_state.selected_nodes.insert(*node_id);
                        // TODO: 实现居中显示节点的功能
                    }
                });
                
                response = response.union(node_response.response);
            }
        });
    });
    
    response
}