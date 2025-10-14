use egui::{CornerRadius, Pos2, Rect, Stroke, StrokeKind, Vec2};

/// 绘制配电箱图例形状（与draw_circuit_shape统一逻辑：接收屏幕坐标和尺寸，内部不处理坐标转换）
pub fn draw_box_legend_shape(
    painter: &egui::Painter,
    box_screen_pos: Pos2,  // 配电箱在屏幕上的基准点（外部已转换好的屏幕坐标）
    size: f32              // 基准尺寸（包含缩放，与draw_circuit_shape的size一致）
) {
    // 基于size计算比例系数（适配原0.1*数值的尺寸逻辑）
    // 原base_size=0.1，这里通过比例系数将尺寸与size绑定，确保随缩放同步变化
    let scale_ratio = size / 100.0;  // 与draw_circuit_shape的len_ratio保持一致

    // 屏幕坐标中的矩形定义（相对于屏幕基准点box_screen_pos）
    // 原坐标：0.0*0.1 改为 0.0*scale_ratio，-120.0*0.1 改为 -12.0*scale_ratio（缩小10倍适配size）
    let rect_start = box_screen_pos + Vec2::new(0.0 * scale_ratio, -12.0 * scale_ratio);
    let rect_end = box_screen_pos + Vec2::new(60.0 * scale_ratio, 12.0 * scale_ratio);
    let screen_rect = Rect::from_min_max(rect_start, rect_end);

    // 斜线的起点和终点（矩形对角线，基于屏幕坐标）
    let line_start = rect_start;
    let line_end = rect_end;

    // 线条样式（线宽随size同步变化，与draw_circuit_shape保持视觉协调）
    let stroke = Stroke::new(3.0 * scale_ratio, egui::Color32::GREEN);

    // 绘制矩形边框
    painter.rect_stroke(
        screen_rect,
        CornerRadius::from(0.0),
        stroke,
        StrokeKind::Middle,
    );

    // 绘制斜线
    painter.line_segment([line_start, line_end], stroke);
}
