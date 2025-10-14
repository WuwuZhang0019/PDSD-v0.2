use egui::{Pos2, Stroke, Vec2};

/// 绘制回路形状
pub fn draw_circuit_shape(
    painter: &egui::Painter,
    circuit_screen_pos: egui::Pos2,
    size: f32,
) -> (Pos2, Pos2) {
    let len_ratio = size / 100.0;

    let line1_start = circuit_screen_pos + Vec2::new(0.0 * len_ratio, 0.0);
    let line1_end = circuit_screen_pos + Vec2::new(160.0 * len_ratio, 0.0);

    let line2_start = circuit_screen_pos + Vec2::new(310.0 * len_ratio, 0.0);
    let line2_end = circuit_screen_pos + Vec2::new(1170.0 * len_ratio, 0.0);

    let line3_start = circuit_screen_pos + Vec2::new(1200.0 * len_ratio, 0.0);
    let line3_end = circuit_screen_pos + Vec2::new(1380.0 * len_ratio, 0.0);

    // 设置线条样式（线宽随缩放变化）
    let stroke = Stroke::new(5.0 * (size / 100.0), egui::Color32::WHITE);
    let stroke1 = Stroke::new(1.0 * (size / 100.0), egui::Color32::WHITE);

    // 绘制所有线段
    painter.line_segment([line1_start, line1_end], stroke);
    painter.line_segment([line2_start, line2_end], stroke);
    painter.line_segment([line3_start, line3_end], stroke1);

    (line1_end, line2_end)
}
