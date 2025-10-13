use std::collections::HashSet;

use crate::color_hex_utils::*;
use crate::utils::ColorUtils;

use super::*;
use egui::epaint::{CubicBezierShape, RectShape};
use egui::*;

pub type PortLocations = std::collections::HashMap<AnyParameterId, Pos2>;
pub type NodeRects = std::collections::HashMap<NodeId, Rect>;

const DISTANCE_TO_CONNECT: f32 = 10.0;

/// 节点在绘制时向父图传达某些事件。有一个特殊的`User`变体，用户可以在节点UI中执行某些自定义操作时将其用作返回值。
#[derive(Clone, Debug)]
pub enum NodeResponse<UserResponse: UserResponseTrait, NodeData: NodeDataTrait> {
    ConnectEventStarted(NodeId, AnyParameterId),
    ConnectEventEnded {
        output: OutputId,
        input: InputId,
    },
    CreatedNode(NodeId),
    SelectNode(NodeId),
    /// 作为此库的用户，请优先监听`DeleteNodeFull`，它还将包含被删除节点的用户数据。
    DeleteNodeUi(NodeId),
    /// 当节点被删除时发出。在从绘制函数返回此响应后，节点将不再存在于图中，但其内容会随事件一起传递。
    DeleteNodeFull {
        node_id: NodeId,
        node: Node<NodeData>,
    },
    DisconnectEvent {
        output: OutputId,
        input: InputId,
    },
    /// 当节点被交互时发出，并且应该被提升
    RaiseNode(NodeId),
    MoveNode {
        node: NodeId,
        drag_delta: Vec2,
    },
    User(UserResponse),
}

/// [`draw_graph_editor`]的返回值。此值可用于使用户代码对绘制图时发生的特定事件做出反应。
#[derive(Clone, Debug)]
pub struct GraphResponse<UserResponse: UserResponseTrait, NodeData: NodeDataTrait> {
    /// 在此帧渲染图期间发生的事件。查看[`UserResponse`]类型以了解每个事件的描述。
    pub node_responses: Vec<NodeResponse<UserResponse, NodeData>>,
    /// 鼠标当前是否悬停在图编辑器上？请注意，即使节点查找器浮动在图编辑器矩形之外，它也被视为图编辑器的一部分。
    pub cursor_in_editor: bool,
    /// 鼠标当前是否悬停在节点查找器上？
    pub cursor_in_finder: bool,
}
impl<UserResponse: UserResponseTrait, NodeData: NodeDataTrait> Default
    for GraphResponse<UserResponse, NodeData>
{
    fn default() -> Self {
        Self {
            node_responses: Default::default(),
            cursor_in_editor: false,
            cursor_in_finder: false,
        }
    }
}
pub struct GraphNodeWidget<'a, NodeData, DataType, ValueType> {
    pub position: &'a mut Pos2,
    pub graph: &'a mut Graph<NodeData, DataType, ValueType>,
    pub port_locations: &'a mut PortLocations,
    pub node_rects: &'a mut NodeRects,
    pub node_id: NodeId,
    pub ongoing_drag: Option<(NodeId, AnyParameterId)>,
    pub selected: bool,
    pub pan: egui::Vec2,
}

impl<NodeData, DataType, ValueType, NodeTemplate, UserResponse, UserState, CategoryType>
    GraphEditorState<NodeData, DataType, ValueType, NodeTemplate, UserState>
where
    NodeData: NodeDataTrait<
        Response = UserResponse,
        UserState = UserState,
        DataType = DataType,
        ValueType = ValueType,
    >,
    UserResponse: UserResponseTrait,
    ValueType:
        WidgetValueTrait<Response = UserResponse, UserState = UserState, NodeData = NodeData>,
    NodeTemplate: NodeTemplateTrait<
        NodeData = NodeData,
        DataType = DataType,
        ValueType = ValueType,
        UserState = UserState,
        CategoryType = CategoryType,
    >,
    DataType: DataTypeTrait<UserState>,
    CategoryType: CategoryTrait,
{
    #[must_use]
    pub fn draw_graph_editor(
        &mut self,
        ui: &mut Ui,
        all_kinds: impl NodeTemplateIter<Item = NodeTemplate>,
        user_state: &mut UserState,
        prepend_responses: Vec<NodeResponse<UserResponse, NodeData>>,
    ) -> GraphResponse<UserResponse, NodeData> {
        // 这使图形编辑器尽可能使用可用空间
        // (对于窗口，它将使用可调整大小设置的限制
        // 对于面板，它将完全填充)

        let editor_rect = ui.max_rect();
        let resp = ui.allocate_rect(editor_rect, Sense::hover());

        let cursor_pos = ui
            .ctx()
            .input(|i| i.pointer.hover_pos().unwrap_or(Pos2::ZERO));
        let mut cursor_in_editor = resp.hovered();
        let mut cursor_in_finder = false;

        // 在绘制节点时填充节点度量信息

        let mut port_locations = PortLocations::new();
        let mut node_rects = NodeRects::new();

        // 节点绘制返回的响应有副作用，最好在函数末尾执行
       
        let mut delayed_responses: Vec<NodeResponse<UserResponse, NodeData>> = prepend_responses;

        // 用于检测背景何时被点击
        
        let mut click_on_background = false;

        // 用于检测背景中的拖动事件
       
        let mut drag_started_on_background = false;
        let mut drag_released_on_background = false;

        debug_assert_eq!(
            self.node_order.iter().copied().collect::<HashSet<_>>(),
            self.graph.iter_nodes().collect::<HashSet<_>>(),
            "GraphEditor的node_order字段处于不一致状态。它拥有的节点数量与图形中的节点数量不匹配。"
        );

        // 在节点之前分配矩形，否则会阻塞与节点的交互
        

        // 检查端口与鼠标之间的距离是否达到连接距离
        

        // 不允许自环连接
        

        // 稍后执行的操作
        
        let r = ui.allocate_rect(ui.min_rect(), Sense::click().union(Sense::drag()));
        if r.clicked() {
            click_on_background = true;
        } else if r.drag_started() {
            drag_started_on_background = true;
        } else if r.drag_released() {
            drag_released_on_background = true;
        }

        /* 绘制节点 */
        for node_id in self.node_order.iter().copied() {
            let responses = GraphNodeWidget {
                position: self.node_positions.get_mut(node_id).unwrap(),
                graph: &mut self.graph,
                port_locations: &mut port_locations,
                node_rects: &mut node_rects,
                node_id,
                ongoing_drag: self.connection_in_progress,
                selected: self
                    .selected_nodes
                    .iter()
                    .any(|selected| *selected == node_id),
                pan: self.pan_zoom.pan + editor_rect.min.to_vec2(),
            }
            .show(ui, user_state);

            // 稍后执行的操作
            delayed_responses.extend(responses);
        }

        /* 如果打开了节点查找器，则绘制它 */
        let mut should_close_node_finder = false;
        if let Some(ref mut node_finder) = self.node_finder {
            let mut node_finder_area = Area::new("node_finder").order(Order::Foreground);
            if let Some(pos) = node_finder.position {
                node_finder_area = node_finder_area.current_pos(pos);
            }
            node_finder_area.show(ui.ctx(), |ui| {
                if let Some(node_kind) = node_finder.show(ui, all_kinds, user_state) {
                    let new_node = self.graph.add_node(
                        node_kind.node_graph_label(user_state),
                        node_kind.user_data(user_state),
                        |graph, node_id| node_kind.build_node(graph, user_state, node_id),
                    );
                    self.node_positions.insert(
                        new_node,
                        cursor_pos - self.pan_zoom.pan - editor_rect.min.to_vec2(),
                    );
                    self.node_order.push(new_node);

                    should_close_node_finder = true;
                    delayed_responses.push(NodeResponse::CreatedNode(new_node));
                }
                let finder_rect = ui.min_rect();
                // 如果光标不在主编辑器中，检查光标是否在查找器中
                // 如果光标在查找器中，则我们可以认为它也在编辑器中
                if finder_rect.contains(cursor_pos) {
                    cursor_in_editor = true;
                    cursor_in_finder = true;
                }
            });
        }
        if should_close_node_finder {
            self.node_finder = None;
        }

        /* 绘制连接 */
        if let Some((_, ref locator)) = self.connection_in_progress {
            let port_type = self.graph.any_param_type(*locator).unwrap();
            let connection_color = port_type.data_type_color(user_state);
            let start_pos = port_locations[locator];

            // 查找要连接的端口
            fn snap_to_ports<
                NodeData,
                UserState,
                DataType: DataTypeTrait<UserState>,
                ValueType,
                Key: slotmap::Key + Into<AnyParameterId>,
                Value,
            >(
                graph: &Graph<NodeData, DataType, ValueType>,
                port_type: &DataType,
                ports: &SlotMap<Key, Value>,
                port_locations: &PortLocations,
                cursor_pos: Pos2,
            ) -> Pos2 {
                ports
                    .iter()
                    .find_map(|(port_id, _)| {
                        let compatible_ports = graph
                            .any_param_type(port_id.into())
                            .map(|other| other == port_type)
                            .unwrap_or(false);

                        if compatible_ports {
                            port_locations.get(&port_id.into()).and_then(|port_pos| {
                                if port_pos.distance(cursor_pos) < DISTANCE_TO_CONNECT {
                                    Some(*port_pos)
                                } else {
                                    None
                                }
                            })
                        } else {
                            None
                        }
                    })
                    .unwrap_or(cursor_pos)
            }

            let (src_pos, dst_pos) = match locator {
                AnyParameterId::Output(_) => (
                    start_pos,
                    snap_to_ports(
                        &self.graph,
                        port_type,
                        &self.graph.inputs,
                        &port_locations,
                        cursor_pos,
                    ),
                ),
                AnyParameterId::Input(_) => (
                    snap_to_ports(
                        &self.graph,
                        port_type,
                        &self.graph.outputs,
                        &port_locations,
                        cursor_pos,
                    ),
                    start_pos,
                ),
            };
            draw_connection(ui.painter(), src_pos, dst_pos, connection_color);
        }

        for (input, output) in self.graph.iter_connections() {
            let port_type = self
                .graph
                .any_param_type(AnyParameterId::Output(output))
                .unwrap();
            let connection_color = port_type.data_type_color(user_state);
            let src_pos = port_locations[&AnyParameterId::Output(output)];
            let dst_pos = port_locations[&AnyParameterId::Input(input)];
            draw_connection(ui.painter(), src_pos, dst_pos, connection_color);
        }

        /* 处理绘制节点产生的响应 */

        // 一些响应在处理时会生成额外的响应。这些响应
        // 存储在这里，以便报告给用户
        let mut extra_responses: Vec<NodeResponse<UserResponse, NodeData>> = Vec::new();

        for response in delayed_responses.iter() {
            match response {
                NodeResponse::ConnectEventStarted(node_id, port) => {
                    self.connection_in_progress = Some((*node_id, *port));
                }
                NodeResponse::ConnectEventEnded { input, output } => {
                    self.graph.add_connection(*output, *input)
                }
                NodeResponse::CreatedNode(_) => {
                    // 为用户提供的便利NodeResponse
                }
                NodeResponse::SelectNode(node_id) => {
                    self.selected_nodes = Vec::from([*node_id]);
                }
                NodeResponse::DeleteNodeUi(node_id) => {
                    let (node, disc_events) = self.graph.remove_node(*node_id);
                    // 首先传递断开连接的响应，以便用户代码可以在节点移除响应之前执行清理
                    extra_responses.extend(
                        disc_events
                            .into_iter()
                            .map(|(input, output)| NodeResponse::DisconnectEvent { input, output }),
                    );
                    // 将完整节点作为响应传递，以便库用户可以
                    // 监听它并获取他们的用户数据
                    extra_responses.push(NodeResponse::DeleteNodeFull {
                        node_id: *node_id,
                        node,
                    });
                    self.node_positions.remove(*node_id);
                    // 确保不会保留对旧节点的引用
                    self.selected_nodes.retain(|id| *id != *node_id);
                    self.node_order.retain(|id| *id != *node_id);
                }
                NodeResponse::DisconnectEvent { input, output } => {
                    let other_node = self.graph.get_output(*output).node;
                    self.graph.remove_connection(*input);
                    self.connection_in_progress =
                        Some((other_node, AnyParameterId::Output(*output)));
                }
                NodeResponse::RaiseNode(node_id) => {
                    let old_pos = self
                        .node_order
                        .iter()
                        .position(|id| *id == *node_id)
                        .expect("要提升的节点应该在`node_order`中");
                    self.node_order.remove(old_pos);
                    self.node_order.push(*node_id);
                }
                NodeResponse::MoveNode { node, drag_delta } => {
                    self.node_positions[*node] += *drag_delta;
                    // 处理多节点选择移动
                    if self.selected_nodes.contains(node) && self.selected_nodes.len() > 1 {
                        for n in self.selected_nodes.iter().copied() {
                            if n != *node {
                                self.node_positions[n] += *drag_delta;
                            }
                        }
                    }
                }
                NodeResponse::User(_) => {
                    // 这些由用户代码处理
                }
                NodeResponse::DeleteNodeFull { .. } => {
                    unreachable!("UI永远不应该产生DeleteNodeFull事件")
                }
            }
        }

        // 处理框选
        if let Some(box_start) = self.ongoing_box_selection {
            let selection_rect = Rect::from_two_pos(cursor_pos, box_start);
            let bg_color = Color32::from_rgba_unmultiplied(200, 200, 200, 20);
            let stroke_color = Color32::from_rgba_unmultiplied(200, 200, 200, 180);
            ui.painter().rect(
                selection_rect,
                2.0,
                bg_color,
                Stroke::new(3.0, stroke_color),
            );

            self.selected_nodes = node_rects
                .into_iter()
                .filter_map(|(node_id, rect)| {
                    if selection_rect.intersects(rect) {
                        Some(node_id)
                    } else {
                        None
                    }
                })
                .collect();
        }

        // 推送在响应处理期间生成的任何响应
        // 这些仅对最终用户提供信息，无需在此处进行特殊处理
        delayed_responses.extend(extra_responses);

        /* 鼠标输入处理 */

        // 这会锁定上下文，所以不要长时间持有
        let mouse = &ui.ctx().input(|i| i.pointer.clone());

        if mouse.any_released() && self.connection_in_progress.is_some() {
            self.connection_in_progress = None;
        }

        if mouse.secondary_released() && cursor_in_editor && !cursor_in_finder {
            self.node_finder = Some(NodeFinder::new_at(cursor_pos));
        }
        if ui.ctx().input(|i| i.key_pressed(Key::Escape)) {
            self.node_finder = None;
        }

        if r.dragged() && ui.ctx().input(|i| i.pointer.middle_down()) {
            self.pan_zoom.pan += ui.ctx().input(|i| i.pointer.delta());
        }

        // 如果编辑器背景被点击，或者鼠标点击了UI之外的区域，则取消选择并停用查找器
        if click_on_background || (mouse.any_click() && !cursor_in_editor) {
            self.selected_nodes = Vec::new();
            self.node_finder = None;
        }

        if drag_started_on_background && mouse.primary_down() {
            self.ongoing_box_selection = Some(cursor_pos);
        }
        if mouse.primary_released() || drag_released_on_background {
            self.ongoing_box_selection = None;
        }

        GraphResponse {
            node_responses: delayed_responses,
            cursor_in_editor,
            cursor_in_finder,
        }
    }
}

fn draw_connection(painter: &Painter, src_pos: Pos2, dst_pos: Pos2, color: Color32) {
    let connection_stroke = egui::Stroke { width: 5.0, color };

    let control_scale = ((dst_pos.x - src_pos.x) / 2.0).max(30.0);
    let src_control = src_pos + Vec2::X * control_scale;
    let dst_control = dst_pos - Vec2::X * control_scale;

    let bezier = CubicBezierShape::from_points_stroke(
        [src_pos, src_control, dst_control, dst_pos],
        false,
        Color32::TRANSPARENT,
        connection_stroke,
    );

    painter.add(bezier);
}

#[derive(Clone, Copy, Debug)]
struct OuterRectMemory(Rect);

impl<'a, NodeData, DataType, ValueType, UserResponse, UserState>
    GraphNodeWidget<'a, NodeData, DataType, ValueType>
where
    NodeData: NodeDataTrait<
        Response = UserResponse,
        UserState = UserState,
        DataType = DataType,
        ValueType = ValueType,
    >,
    UserResponse: UserResponseTrait,
    ValueType:
        WidgetValueTrait<Response = UserResponse, UserState = UserState, NodeData = NodeData>,
    DataType: DataTypeTrait<UserState>,
{
    pub const MAX_NODE_SIZE: [f32; 2] = [200.0, 200.0];

    pub fn show(
        self,
        ui: &mut Ui,
        user_state: &mut UserState,
    ) -> Vec<NodeResponse<UserResponse, NodeData>> {
        let mut child_ui = ui.child_ui_with_id_source(
            Rect::from_min_size(*self.position + self.pan, Self::MAX_NODE_SIZE.into()),
            Layout::default(),
            self.node_id,
        );

        Self::show_graph_node(self, &mut child_ui, user_state)
    }

    /// 绘制此节点。同时填充所有端口的位置列表。
    /// 返回指示多个事件的响应。
    fn show_graph_node(
        self,
        ui: &mut Ui,
        user_state: &mut UserState,
    ) -> Vec<NodeResponse<UserResponse, NodeData>> {
        let margin = egui::vec2(15.0, 5.0);
        let mut responses = Vec::<NodeResponse<UserResponse, NodeData>>::new();

        let background_color;
        let text_color;
        if ui.visuals().dark_mode {
            background_color = color_from_hex("#3f3f3f").unwrap();
            text_color = color_from_hex("#fefefe").unwrap();
        } else {
            background_color = color_from_hex("#ffffff").unwrap();
            text_color = color_from_hex("#505050").unwrap();
        }

        ui.visuals_mut().widgets.noninteractive.fg_stroke = Stroke::new(2.0, text_color);

        // 预分配要绘制在内容下方的形状
        // 预分配要绘制在内容下方的形状
        let outline_shape = ui.painter().add(Shape::Noop);
        let background_shape = ui.painter().add(Shape::Noop);

        let outer_rect_bounds = ui.available_rect_before_wrap();

        let mut inner_rect = outer_rect_bounds.shrink2(margin);

        // Make sure we don't shrink to the negative:
        inner_rect.max.x = inner_rect.max.x.max(inner_rect.min.x);
        inner_rect.max.y = inner_rect.max.y.max(inner_rect.min.y);

        let mut child_ui = ui.child_ui(inner_rect, *ui.layout());

        // Get interaction rect from memory, it may expand after the window response on resize.
        let interaction_rect = ui
            .ctx()
            .memory_mut(|mem| {
                mem.data
                    .get_temp::<OuterRectMemory>(child_ui.id())
                    .map(|stored| stored.0)
            })
            .unwrap_or(outer_rect_bounds);
        // After 0.20, layers added over others can block hover interaction. Call this first
        // before creating the node content.
        let window_response = ui.interact(
            interaction_rect,
            Id::new((self.node_id, "window")),
            Sense::click_and_drag(),
        );

        let mut title_height = 0.0;

        let mut input_port_heights = vec![];
        let mut output_port_heights = vec![];

        child_ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.add(Label::new(
                    RichText::new(&self.graph[self.node_id].label)
                        .text_style(TextStyle::Button)
                        .color(text_color),
                ));
                responses.extend(
                    self.graph[self.node_id]
                        .user_data
                        .top_bar_ui(ui, self.node_id, self.graph, user_state)
                        .into_iter(),
                );
                ui.add_space(8.0); // The size of the little cross icon
            });
            ui.add_space(margin.y);
            title_height = ui.min_size().y;

            // First pass: Draw the inner fields. Compute port heights
            let inputs = self.graph[self.node_id].inputs.clone();
            for (param_name, param_id) in inputs {
                if self.graph[param_id].shown_inline {
                    let height_before = ui.min_rect().bottom();
                    // 注意：我们希望将`user_data`传递给
                    // `value_widget`，但不能这样做，因为这需要
                    // 借用graph两次。在这里，我们假设值可以廉价地替换，
                    // 并使用`std::mem::take`暂时用一个虚拟值替换它。
                    // 这要求`ValueType`实现Default，但结果是完全安全的替代方案。
                    let mut value = std::mem::take(&mut self.graph[param_id].value);

                    if self.graph.connection(param_id).is_some() {
                        let node_responses = value.value_widget_connected(
                            &param_name,
                            self.node_id,
                            ui,
                            user_state,
                            &self.graph[self.node_id].user_data,
                        );

                        responses.extend(node_responses.into_iter().map(NodeResponse::User));
                    } else {
                        let node_responses = value.value_widget(
                            &param_name,
                            self.node_id,
                            ui,
                            user_state,
                            &self.graph[self.node_id].user_data,
                        );

                        responses.extend(node_responses.into_iter().map(NodeResponse::User));
                    }

                    self.graph[self.node_id].user_data.separator(
                        ui,
                        self.node_id,
                        AnyParameterId::Input(param_id),
                        self.graph,
                        user_state,
                    );

                    self.graph[param_id].value = value;

                    let height_after = ui.min_rect().bottom();
                    input_port_heights.push((height_before + height_after) / 2.0);
                }
            }

            let outputs = self.graph[self.node_id].outputs.clone();
            for (param_name, param_id) in outputs {
                let height_before = ui.min_rect().bottom();
                responses.extend(
                    self.graph[self.node_id]
                        .user_data
                        .output_ui(ui, self.node_id, self.graph, user_state, &param_name)
                        .into_iter(),
                );

                self.graph[self.node_id].user_data.separator(
                    ui,
                    self.node_id,
                    AnyParameterId::Output(param_id),
                    self.graph,
                    user_state,
                );

                let height_after = ui.min_rect().bottom();
                output_port_heights.push((height_before + height_after) / 2.0);
            }

            responses.extend(
                self.graph[self.node_id]
                    .user_data
                    .bottom_ui(ui, self.node_id, self.graph, user_state)
                    .into_iter(),
            );
        });

        // 第二遍，再次迭代绘制端口。这发生在child_ui外部，因为我们希望端口溢出节点背景。

        let outer_rect = child_ui.min_rect().expand2(margin);
        let port_left = outer_rect.left();
        let port_right = outer_rect.right();

        // Save expanded rect to memory.
        ui.ctx().memory_mut(|mem| {
            mem.data
                .insert_temp(child_ui.id(), OuterRectMemory(outer_rect))
        });

        #[allow(clippy::too_many_arguments)]
        fn draw_port<NodeData, DataType, ValueType, UserResponse, UserState>(
            ui: &mut Ui,
            graph: &Graph<NodeData, DataType, ValueType>,
            node_id: NodeId,
            user_state: &mut UserState,
            port_pos: Pos2,
            responses: &mut Vec<NodeResponse<UserResponse, NodeData>>,
            param_id: AnyParameterId,
            port_locations: &mut PortLocations,
            ongoing_drag: Option<(NodeId, AnyParameterId)>,
            is_connected_input: bool,
        ) where
            DataType: DataTypeTrait<UserState>,
            UserResponse: UserResponseTrait,
            NodeData: NodeDataTrait,
        {
            let port_type = graph.any_param_type(param_id).unwrap();

            let port_rect = Rect::from_center_size(port_pos, egui::vec2(10.0, 10.0));

            let sense = if ongoing_drag.is_some() {
                Sense::hover()
            } else {
                Sense::click_and_drag()
            };

            let resp = ui.allocate_rect(port_rect, sense);

            // 检查端口与鼠标之间的距离是否达到连接距离
            let close_enough = if let Some(pointer_pos) = ui.ctx().pointer_hover_pos() {
                port_rect.center().distance(pointer_pos) < DISTANCE_TO_CONNECT
            } else {
                false
            };

            let port_color = if close_enough {
                Color32::WHITE
            } else {
                port_type.data_type_color(user_state)
            };
            ui.painter()
                .circle(port_rect.center(), 5.0, port_color, Stroke::NONE);

            if resp.drag_started() {
                if is_connected_input {
                    let input = param_id.assume_input();
                    let corresp_output = graph
                        .connection(input)
                        .expect("Connection data should be valid");
                    responses.push(NodeResponse::DisconnectEvent {
                        input: param_id.assume_input(),
                        output: corresp_output,
                    });
                } else {
                    responses.push(NodeResponse::ConnectEventStarted(node_id, param_id));
                }
            }

            if let Some((origin_node, origin_param)) = ongoing_drag {
                if origin_node != node_id {
                    // Don't allow self-loops
                    if graph.any_param_type(origin_param).unwrap() == port_type
                        && close_enough
                        && ui.input(|i| i.pointer.any_released())
                    {
                        match (param_id, origin_param) {
                            (AnyParameterId::Input(input), AnyParameterId::Output(output))
                            | (AnyParameterId::Output(output), AnyParameterId::Input(input)) => {
                                responses.push(NodeResponse::ConnectEventEnded { input, output });
                            }
                            _ => { /* Ignore in-in or out-out connections */ }
                        }
                    }
                }
            }

            port_locations.insert(param_id, port_rect.center());
        }

        // Input ports
        for ((_, param), port_height) in self.graph[self.node_id]
            .inputs
            .iter()
            .zip(input_port_heights.into_iter())
        {
            let should_draw = match self.graph[*param].kind() {
                InputParamKind::ConnectionOnly => true,
                InputParamKind::ConstantOnly => false,
                InputParamKind::ConnectionOrConstant => true,
            };

            if should_draw {
                let pos_left = pos2(port_left, port_height);
                draw_port(
                    ui,
                    self.graph,
                    self.node_id,
                    user_state,
                    pos_left,
                    &mut responses,
                    AnyParameterId::Input(*param),
                    self.port_locations,
                    self.ongoing_drag,
                    self.graph.connection(*param).is_some(),
                );
            }
        }

        // Output ports
        for ((_, param), port_height) in self.graph[self.node_id]
            .outputs
            .iter()
            .zip(output_port_heights.into_iter())
        {
            let pos_right = pos2(port_right, port_height);
            draw_port(
                ui,
                self.graph,
                self.node_id,
                user_state,
                pos_right,
                &mut responses,
                AnyParameterId::Output(*param),
                self.port_locations,
                self.ongoing_drag,
                false,
            );
        }

        // 绘制背景形状
        // 注意：这段代码比实际需要的更复杂，因为egui不支持绘制具有不对称圆角的矩形。

        let (shape, outline) = {
            let rounding_radius = 4.0;
            let rounding = Rounding::same(rounding_radius);

            let titlebar_height = title_height + margin.y;
            let titlebar_rect =
                Rect::from_min_size(outer_rect.min, vec2(outer_rect.width(), titlebar_height));
            let titlebar = Shape::Rect(RectShape {
                rect: titlebar_rect,
                rounding,
                fill: self.graph[self.node_id]
                    .user_data
                    .titlebar_color(ui, self.node_id, self.graph, user_state)
                    .unwrap_or_else(|| background_color.lighten(0.8)),
                stroke: Stroke::NONE,
            });

            let body_rect = Rect::from_min_size(
                outer_rect.min + vec2(0.0, titlebar_height - rounding_radius),
                vec2(outer_rect.width(), outer_rect.height() - titlebar_height),
            );
            let body = Shape::Rect(RectShape {
                rect: body_rect,
                rounding: Rounding::none(),
                fill: background_color,
                stroke: Stroke::NONE,
            });

            let bottom_body_rect = Rect::from_min_size(
                body_rect.min + vec2(0.0, body_rect.height() - titlebar_height * 0.5),
                vec2(outer_rect.width(), titlebar_height),
            );
            let bottom_body = Shape::Rect(RectShape {
                rect: bottom_body_rect,
                rounding,
                fill: background_color,
                stroke: Stroke::NONE,
            });

            let node_rect = titlebar_rect.union(body_rect).union(bottom_body_rect);
            let outline = if self.selected {
                Shape::Rect(RectShape {
                    rect: node_rect.expand(1.0),
                    rounding,
                    fill: Color32::WHITE.lighten(0.8),
                    stroke: Stroke::NONE,
                })
            } else {
                Shape::Noop
            };

            // Take note of the node rect, so the editor can use it later to compute intersections.
            self.node_rects.insert(self.node_id, node_rect);

            (Shape::Vec(vec![titlebar, body, bottom_body]), outline)
        };

        ui.painter().set(background_shape, shape);
        ui.painter().set(outline_shape, outline);

        // --- 交互 ---

        // 标题栏按钮
        let can_delete = self.graph.nodes[self.node_id].user_data.can_delete(
            self.node_id,
            self.graph,
            user_state,
        );

        if can_delete && Self::close_button(ui, outer_rect).clicked() {
            responses.push(NodeResponse::DeleteNodeUi(self.node_id));
        };

        // Movement
        let drag_delta = window_response.drag_delta();
        if drag_delta.length_sq() > 0.0 {
            responses.push(NodeResponse::MoveNode {
                node: self.node_id,
                drag_delta,
            });
            responses.push(NodeResponse::RaiseNode(self.node_id));
        }

        // 节点选择
        //
        // 技巧：仅当没有其他响应处于活动状态时才设置选择响应
        // 这可以防止一些问题
        if responses.is_empty() && window_response.clicked_by(PointerButton::Primary) {
            responses.push(NodeResponse::SelectNode(self.node_id));
            responses.push(NodeResponse::RaiseNode(self.node_id));
        }

        responses
    }

    fn close_button(ui: &mut Ui, node_rect: Rect) -> Response {
        // Measurements
        let margin = 8.0;
        let size = 10.0;
        let stroke_width = 2.0;
        let offs = margin + size / 2.0;

        let position = pos2(node_rect.right() - offs, node_rect.top() + offs);
        let rect = Rect::from_center_size(position, vec2(size, size));
        let resp = ui.allocate_rect(rect, Sense::click());

        let dark_mode = ui.visuals().dark_mode;
        let color = if resp.clicked() {
            if dark_mode {
                color_from_hex("#ffffff").unwrap()
            } else {
                color_from_hex("#000000").unwrap()
            }
        } else if resp.hovered() {
            if dark_mode {
                color_from_hex("#dddddd").unwrap()
            } else {
                color_from_hex("#222222").unwrap()
            }
        } else {
            #[allow(clippy::collapsible_else_if)]
            if dark_mode {
                color_from_hex("#aaaaaa").unwrap()
            } else {
                color_from_hex("#555555").unwrap()
            }
        };
        let stroke = Stroke {
            width: stroke_width,
            color,
        };

        ui.painter()
            .line_segment([rect.left_top(), rect.right_bottom()], stroke);
        ui.painter()
            .line_segment([rect.right_top(), rect.left_bottom()], stroke);

        resp
    }
}