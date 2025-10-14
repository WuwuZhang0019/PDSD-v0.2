use crate::editor::business::text_assembly_shape::*;
use crate::editor::business::text_library::text_switching_components::BreakerType::{IS, MCCB};
use crate::editor::business::text_library::text_switching_components::Curve::C;
use crate::editor::business::text_library::text_switching_components::FrameCurrent::A100;
use crate::editor::business::text_library::text_switching_components::Pole::P3;
use crate::editor::business::text_library::text_switching_components::SettingValue::A20;
use eframe::egui;
use egui::{Color32, Pos2, Stroke, Vec2};
use crate::editor::business::text_library::text_switching_components::DynamicBreakerText;

/// 绘制断路器形状（与draw_circuit_shape统一逻辑）
pub fn draw_breaker_shape(
    painter: &egui::Painter,
    breaker_screen_pos: Pos2, // 断路器在屏幕上的基准点（外部已转换的屏幕坐标）
    size: f32,                // 基准尺寸（包含缩放）
    params: DynamicBreakerText, // 新增：动态文字参数
) {
    // 比例系数（与draw_circuit_shape保持一致）
    let scale_ratio = size / 100.0;

    // 屏幕坐标中的各个点（相对于基准点）
    let line1_start = breaker_screen_pos + Vec2::new(0.0 * scale_ratio, 0.0);
    let line1_end = breaker_screen_pos + Vec2::new(37.5 * scale_ratio, 0.0); // 原375.0*0.1按比例转换

    let line2_start = breaker_screen_pos + Vec2::new(112.5 * scale_ratio, 0.0); // 原1125.0*0.1
    let line2_end = line2_start + Vec2::new(37.5 * scale_ratio, 0.0);

    let line3_start = breaker_screen_pos + Vec2::new(37.5 * scale_ratio, 10.0 * scale_ratio); // 原100.0*0.1
    let line3_end = breaker_screen_pos + Vec2::new(37.5 * scale_ratio, -10.0 * scale_ratio);

    let line4_start = breaker_screen_pos + Vec2::new(12.4 * scale_ratio, 7.1 * scale_ratio); // 原124.0*0.1和71.0*0.1
    let line4_end = breaker_screen_pos + Vec2::new(26.6 * scale_ratio, -7.1 * scale_ratio);

    let line5_start = breaker_screen_pos + Vec2::new(26.6 * scale_ratio, 7.1 * scale_ratio);
    let line5_end = breaker_screen_pos + Vec2::new(12.4 * scale_ratio, -7.1 * scale_ratio);

    let line6_start = breaker_screen_pos + Vec2::new(30.0 * scale_ratio, 37.5 * scale_ratio); // 原300.0*0.1和375.0*0.1
    let line6_end = breaker_screen_pos + Vec2::new(112.5 * scale_ratio, 0.0);

    // 线条样式（线宽随size变化）
    let stroke = Stroke::new(3.0 * scale_ratio, egui::Color32::from_rgb(0, 255, 0));

    // 绘制所有线段
    painter.line_segment([line1_start, line1_end], stroke);
    painter.line_segment([line2_start, line2_end], stroke);
    painter.line_segment([line3_start, line3_end], stroke);
    painter.line_segment([line4_start, line4_end], stroke);
    painter.line_segment([line5_start, line5_end], stroke);
    painter.line_segment([line6_start, line6_end], stroke);

    let rightmost = line2_end;
    draw_breaker_text(
        painter,
        breaker_screen_pos,
        rightmost,
        params,
    );
    // 输入点位于图形最左端（line1_start）
    let input_port_pos = line1_start;
    // 输出点位于图形最右端（line2_end）
    let output_port_pos = line2_end;
    painter.circle_filled(input_port_pos, 5.0 * scale_ratio, Color32::BLUE);
    painter.circle_filled(output_port_pos, 5.0 * scale_ratio, Color32::RED);
}

/// 绘制隔离开关形状（与draw_circuit_shape统一逻辑）
pub fn draw_disconnector_shape(
    painter: &egui::Painter,
    disconnector_screen_pos: Pos2, // 隔离开关在屏幕上的基准点（外部已转换的屏幕坐标）
    size: f32,                     // 基准尺寸（包含缩放）
) {
    // 比例系数（与其他图形保持一致）
    let scale_ratio = size / 100.0;

    // 屏幕坐标中的各个线段端点（基于scale_ratio缩放）
    let line1_start = disconnector_screen_pos + Vec2::new(0.0 * scale_ratio, 0.0);
    let line1_end = disconnector_screen_pos + Vec2::new(22.5 * scale_ratio, 0.0); // 原225.0按比例缩小10倍

    let line2_start = disconnector_screen_pos + Vec2::new(112.5 * scale_ratio, 0.0); // 原1125.0
    let line2_end = line2_start + Vec2::new(37.5 * scale_ratio, 0.0); // 原375.0

    let line3_start = disconnector_screen_pos + Vec2::new(22.5 * scale_ratio, 10.0 * scale_ratio); // 原100.0
    let line3_end = disconnector_screen_pos + Vec2::new(22.5 * scale_ratio, -10.0 * scale_ratio);

    let line4_start = disconnector_screen_pos + Vec2::new(30.0 * scale_ratio, 37.5 * scale_ratio); // 原300.0和375.0
    let line4_end = disconnector_screen_pos + Vec2::new(112.5 * scale_ratio, 0.0);

    // 圆圈参数
    let circle1_center = disconnector_screen_pos + Vec2::new(30.0 * scale_ratio, 0.0); // 原300.0
    let circle1_radius = 7.5 * scale_ratio; // 原75.0

    // 线条样式（线宽随size变化）
    let stroke = Stroke::new(5.0 * scale_ratio, egui::Color32::GREEN);

    // 绘制所有元素
    painter.line_segment([line1_start, line1_end], stroke);
    painter.line_segment([line2_start, line2_end], stroke);
    painter.line_segment([line3_start, line3_end], stroke);
    painter.line_segment([line4_start, line4_end], stroke);
    painter.circle_stroke(circle1_center, circle1_radius, stroke);

    // 确定最右侧坐标（用于文字定位）
    let rightmost = line2_end;
    // 绘制文字（使用屏幕坐标）
    draw_disconnector_text(painter, disconnector_screen_pos, rightmost, IS, P3, A20);
    
    // 添加输入输出点，分别位于图形的左右两端
    // 输入点位于图形最左端（line1_start）
    let input_port_pos = line1_start;
    // 输出点位于图形最右端（line2_end）
    let output_port_pos = line2_end;
    painter.circle_filled(input_port_pos, 5.0 * scale_ratio, Color32::BLUE);
    painter.circle_filled(output_port_pos, 5.0 * scale_ratio, Color32::RED);
}
