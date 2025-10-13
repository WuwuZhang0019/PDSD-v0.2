use super::*;
use std::marker::PhantomData;

#[cfg(feature = "persistence")]
use serde::{Deserialize, Serialize};

#[derive(Default, Copy, Clone)]
#[cfg_attr(feature = "persistence", derive(Serialize, Deserialize))]
pub struct PanZoom {
    pub pan: egui::Vec2,
    pub zoom: f32,
}

#[derive(Clone)]
#[cfg_attr(feature = "persistence", derive(Serialize, Deserialize))]
pub struct GraphEditorState<NodeData, DataType, ValueType, NodeTemplate, UserState> {
    pub graph: Graph<NodeData, DataType, ValueType>,
    /// 节点按此顺序绘制。绘制顺序很重要，因为最后绘制的节点位于顶部。
    pub node_order: Vec<NodeId>,
    /// 正在进行的连接交互：鼠标已从端口拖开，用户正在按住点击
    pub connection_in_progress: Option<(NodeId, AnyParameterId)>,
    /// 当前选中的节点。某些接口操作依赖于当前选中的节点。
    pub selected_nodes: Vec<NodeId>,
    /// 正在进行框选的鼠标拖动起始位置。
    pub ongoing_box_selection: Option<egui::Pos2>,
    /// 每个节点的位置。
    pub node_positions: SecondaryMap<NodeId, egui::Pos2>,
    /// 节点查找器用于创建新节点。
    pub node_finder: Option<NodeFinder<NodeTemplate>>,
    /// 图视口的平移。
    pub pan_zoom: PanZoom,
    pub _user_state: PhantomData<fn() -> UserState>,
}

impl<NodeData, DataType, ValueType, NodeKind, UserState>
    GraphEditorState<NodeData, DataType, ValueType, NodeKind, UserState>
{
    pub fn new(default_zoom: f32) -> Self {
        Self {
            pan_zoom: PanZoom {
                pan: egui::Vec2::ZERO,
                zoom: default_zoom,
            },
            ..Default::default()
        }
    }
}
impl<NodeData, DataType, ValueType, NodeKind, UserState> Default
    for GraphEditorState<NodeData, DataType, ValueType, NodeKind, UserState>
{
    fn default() -> Self {
        Self {
            graph: Default::default(),
            node_order: Default::default(),
            connection_in_progress: Default::default(),
            selected_nodes: Default::default(),
            ongoing_box_selection: Default::default(),
            node_positions: Default::default(),
            node_finder: Default::default(),
            pan_zoom: Default::default(),
            _user_state: Default::default(),
        }
    }
}

impl PanZoom {
    pub fn adjust_zoom(
        &mut self,
        zoom_delta: f32,
        point: egui::Vec2,
        zoom_min: f32,
        zoom_max: f32,
    ) {
        let zoom_clamped = (self.zoom + zoom_delta).clamp(zoom_min, zoom_max);
        let zoom_delta = zoom_clamped - self.zoom;

        self.zoom += zoom_delta;
        self.pan += point * zoom_delta;
    }
}
