use eframe::egui::{self, Ui, Painter, Pos2, Color32, Rect, Align2, FontId, Stroke, Id}; 
use egui_node_graph::NodeId; 
use slotmap::{new_key_type, Key}; 

// 定义分组和注释的唯一标识符
new_key_type! {
    pub struct GroupId;
    pub struct AnnotationId;
}

/// 节点分组数据结构
pub struct NodeGroup {
    pub id: GroupId,
    pub label: String,
    pub color: Color32,
    pub rect: Rect,
    pub node_ids: Vec<NodeId>,
    pub is_selected: bool,
}

impl Default for NodeGroup {
    fn default() -> Self {
        Self {
            id: GroupId::null(),
            label: "新分组".to_string(),
            color: Color32::from_rgba_premultiplied(100, 150, 255, 100),
            rect: Rect::NOTHING,
            node_ids: Vec::new(),
            is_selected: false,
        }
    }
}

/// 注释数据结构
pub struct Annotation {
    pub id: AnnotationId,
    pub text: String,
    pub position: Pos2,
    pub size: egui::Vec2,
    pub color: Color32,
    pub is_selected: bool,
}

impl Default for Annotation {
    fn default() -> Self {
        Self {
            id: AnnotationId::null(),
            text: "新注释".to_string(),
            position: Pos2::ZERO,
            size: egui::vec2(200.0, 100.0),
            color: Color32::from_rgba_premultiplied(255, 255, 230, 200),
            is_selected: false,
        }
    }
}

/// 节点分组和注释管理器
pub struct NodeGroupManager {
    pub groups: Vec<NodeGroup>,
    pub annotations: Vec<Annotation>,
    pub next_group_id: usize,
    pub next_annotation_id: usize,
}

impl Default for NodeGroupManager {
    fn default() -> Self {
        Self {
            groups: Vec::new(),
            annotations: Vec::new(),
            next_group_id: 1,
            next_annotation_id: 1,
        }
    }
}

impl NodeGroupManager {
    /// 创建新的节点分组
    pub fn create_group(&mut self, label: &str, color: Color32) -> GroupId {
        let id = GroupId::from_usize(self.next_group_id);
        self.next_group_id += 1;
        
        let new_group = NodeGroup {
            id,
            label: label.to_string(),
            color,
            rect: Rect::from_min_size(Pos2::new(100.0, 100.0), egui::vec2(300.0, 200.0)),
            node_ids: Vec::new(),
            is_selected: false,
        };
        
        self.groups.push(new_group);
        id
    }
    
    /// 创建新的注释
    pub fn create_annotation(&mut self, text: &str, position: Pos2) -> AnnotationId {
        let id = AnnotationId::from_usize(self.next_annotation_id);
        self.next_annotation_id += 1;
        
        let new_annotation = Annotation {
            id,
            text: text.to_string(),
            position,
            size: egui::vec2(200.0, 100.0),
            color: Color32::from_rgba_premultiplied(255, 255, 230, 200),
            is_selected: false,
        };
        
        self.annotations.push(new_annotation);
        id
    }
    
    /// 将节点添加到分组
    pub fn add_node_to_group(&mut self, group_id: GroupId, node_id: NodeId) {
        if let Some(group) = self.groups.iter_mut().find(|g| g.id == group_id) {
            if !group.node_ids.contains(&node_id) {
                group.node_ids.push(node_id);
            }
        }
    }
    
    /// 从分组中移除节点
    pub fn remove_node_from_group(&mut self, group_id: GroupId, node_id: NodeId) {
        if let Some(group) = self.groups.iter_mut().find(|g| g.id == group_id) {
            group.node_ids.retain(|&id| id != node_id);
        }
    }
    
    /// 绘制分组和注释
    pub fn draw_groups_and_annotations(
        &self,
        ui: &mut Ui,
        response: &egui::Response,
    ) {
        let painter = ui.painter_at(response.rect);

        // 绘制分组背景
        for group in &self.groups {
            painter.rect(
                group.rect,
                8.0, // 圆角半径
                group.color,
                Stroke::new(1.0, Color32::WHITE),
            );
            painter.text(
                group.rect.min,
                Align2::LEFT_TOP,
                &group.label,
                FontId::monospace(12.0),
                Color32::WHITE,
            );
        }

        // 绘制注释
        for annotation in &self.annotations {
            let rect = Rect::from_min_size(annotation.position, annotation.size);
            painter.rect(
                rect,
                4.0, // 圆角半径
                annotation.color,
                Stroke::new(1.0, Color32::BLACK),
            );
            painter.text(
                rect.center(),
                Align2::CENTER_CENTER,
                &annotation.text,
                FontId::monospace(12.0),
                Color32::BLACK,
            );
        }
    }
}