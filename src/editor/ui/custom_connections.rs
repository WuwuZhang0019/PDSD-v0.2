use eframe::egui::{self, Ui, Painter, Pos2, Color32, Stroke, Shape, PathShape}; 

/// 自定义连接绘制函数
/// 提供更美观和专业的连接线条样式
pub fn draw_custom_connection(
    ui: &mut Ui,
    response: &egui::Response,
    start: Pos2,
    end: Pos2,
    color: Color32,
    thickness: f32,
) {
    // 获取绘图上下文
    let painter = ui.painter_at(response.rect);

    // 计算控制点以创建平滑曲线
    let mid_point = (start + end) * 0.5;
    let control_point1 = Pos2::new(mid_point.x - 50.0, mid_point.y);
    let control_point2 = Pos2::new(mid_point.x + 50.0, mid_point.y);

    // 绘制贝塞尔曲线
    painter.add(Shape::Path(PathShape {
        points: vec![start, control_point1, control_point2, end],
        closed: false,
        fill: Color32::TRANSPARENT,
        stroke: Stroke::new(thickness, color),
    }));

    // 绘制箭头（在终点位置）
    draw_arrow(painter, end, start, color, thickness);
}

/// 绘制箭头
fn draw_arrow(
    painter: &Painter,
    from: Pos2,
    to: Pos2,
    color: Color32,
    thickness: f32,
) {
    // 计算箭头方向
    let direction = (to - from).normalize_or_zero();
    let arrow_length = 10.0;
    let arrow_angle = std::f32::consts::PI / 6.0; // 30度

    // 计算箭头两侧点
    let arrow_side1 = from + direction * arrow_length 
        + Pos2::new(
            -direction.y * arrow_length * arrow_angle.cos(),
            direction.x * arrow_length * arrow_angle.sin()
        );
    let arrow_side2 = from + direction * arrow_length 
        + Pos2::new(
            direction.y * arrow_length * arrow_angle.cos(),
            -direction.x * arrow_length * arrow_angle.sin()
        );

    // 绘制箭头
    painter.add(Shape::Path(PathShape {
        points: vec![to, arrow_side1, arrow_side2, to],
        closed: true,
        fill: color,
        stroke: Stroke::new(thickness, color),
    }));
}