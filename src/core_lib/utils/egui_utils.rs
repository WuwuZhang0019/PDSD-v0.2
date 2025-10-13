use once_cell::sync::OnceCell;
use egui;
use std::sync::Arc; // 引入 Arc 智能指针

static FONTS_INITIALIZED: OnceCell<()> = OnceCell::new();

/// 初始化egui字体（全局只执行一次）
pub fn init_egui_fonts(ctx: &egui::Context, font_id: &str, font_data: &'static [u8]) {
    FONTS_INITIALIZED.get_or_init(|| {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            font_id.to_owned(),
            Arc::new(egui::FontData::from_static(font_data)),
        );
        if let Some(families) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
            families.push(font_id.to_owned());
        }
        ctx.set_fonts(fonts);
    });
}