use eframe::egui::{self, Ui, ScrollArea, CollapsingHeader}; 
use std::collections::HashMap; 
use crate::editor::business::{ElectricNodeTemplate}; 
use crate::editor::UIUserState; 

/// 增强版节点查找器
/// 支持按类别分组和实时搜索
pub fn enhanced_node_finder(
    ui: &mut Ui,
    templates: &[impl ElectricNodeTemplate],
    search_query: &mut String,
    user_state: &mut UIUserState,
    selected_template: &mut Option<Box<dyn ElectricNodeTemplate>>,
) -> bool {
    // 绘制搜索框
    ui.horizontal(|ui| {
        ui.label("搜索节点:");
        ui.text_edit_singleline(search_query);
    });

    // 过滤模板
    let filtered_templates: Vec<_> = templates
        .iter()
        .filter(|template| {
            let label = template.node_label();
            let category = template.category();
            
            label.to_lowercase().contains(&search_query.to_lowercase()) ||
            category.to_lowercase().contains(&search_query.to_lowercase())
        })
        .collect();

    // 按类别分组显示
    let mut categories = HashMap::new();
    for template in &filtered_templates {
        let category = template.category().to_string();
        categories
            .entry(category)
            .or_insert_with(Vec::new)
            .push(template.clone());
    }

    // 显示分类后的节点列表
    let mut selected = false;
    ScrollArea::vertical().show(ui, |ui| {
        for (category, category_templates) in categories {
            CollapsingHeader::new(&category)
                .default_open(true)
                .show(ui, |ui| {
                    for template in category_templates {
                        let template_clone = template.clone();
                        let is_selected = selected_template.as_ref()
                            .map_or(false, |t| t.node_label() == template.node_label());
                            
                        if ui.selectable_label(is_selected, &template.node_label()).clicked() {
                            *selected_template = Some(template_clone);
                            selected = true;
                        }
                    }
                });
        }
    });

    selected
}