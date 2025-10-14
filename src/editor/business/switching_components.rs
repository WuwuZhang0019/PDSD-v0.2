use crate::editor::business::text_library::text_switching_components::{BreakerType, Curve, DynamicBreakerText, FrameCurrent, Pole, SettingValue};
/// 开关元器件的文字与形状拼接
use egui::{Align2, Color32, FontFamily, FontId, Painter, Pos2};

/// 绘制断路器上方的文字
pub fn draw_breaker_text(
    painter: &Painter,
    base_pos: Pos2,
    rightmost: Pos2,
    params: DynamicBreakerText, // 使用动态参数结构体
) {
    // 获取所有参数（包含智能计算）
    let (breaker_type, frame_current, pole, curve, setting_value, _, _) =
        params.resolved_params();

    // 计算断路器总宽度（用于动态调整字体大小）
    let breaker_width = rightmost.x - base_pos.x;
    // 动态计算字体大小
    let font_size = breaker_width * 0.13;
    let font_id = FontId::new(font_size, FontFamily::Proportional);

    // 计算文字水平居中位置
    let center_x = (base_pos.x + rightmost.x) / 2.0;

    // 第一行文字位置
    let text1_pos = Pos2::new(center_x, base_pos.y - breaker_width * 0.25);
    // 第二行文字位置
    let text2_pos = Pos2::new(center_x, text1_pos.y + font_size);
    // 第一行文字：显示断路器类型和壳架电流
    let first_line = format!("{}{}",
                             breaker_type.to_str(), frame_current.to_str()
    );
    painter.text(
        text1_pos,
        Align2::CENTER_CENTER,
        first_line,
        font_id.clone(),
        Color32::WHITE,
    );
    // 第二行文字：显示极数、脱扣曲线和整定值
    let second_line = format!("{}-{} {}A",
                              pole.to_str(), curve.to_str(), setting_value.to_str()
    );
    painter.text(
        text2_pos,
        Align2::CENTER_CENTER,
        second_line,
        font_id.clone(),
        Color32::WHITE,
    );
}

/// 绘制隔离开关上方的文字
pub fn draw_disconnector_text(
    painter: &Painter,
    base_pos: Pos2,
    rightmost: Pos2,
    break_type: BreakerType,
    pole: Pole,                  // 极数枚举（用户选择）
    setting_value: SettingValue, // 整定值枚举（计算结果）
) {
    // 计算断路器总宽度（用于动态调整字体大小）
    let breaker_width = rightmost.x - base_pos.x;
    // 动态计算字体大小（断路器宽度的5%，确保与尺寸匹配）
    let font_size = breaker_width * 0.1;
    let font_id = FontId::new(font_size, FontFamily::Proportional);
    // 计算文字水平居中位置（断路器左右中点）
    let center_x = (base_pos.x + rightmost.x) / 2.0;
    // 第一行文字位置（断路器上方，距离顶部有一定间距）
    let text1_pos = Pos2::new(center_x, base_pos.y - breaker_width * 0.25);
    // 第二行文字位置（与第一行间隔约1个字体高度）
    let text2_pos = Pos2::new(center_x, text1_pos.y + font_size);
    // 第一行文字："MCCB 100"
    painter.text(
        text1_pos,
        Align2::CENTER_CENTER,
        break_type.to_str(),
        font_id.clone(),
        Color32::WHITE,
    );
    // 第二行文字：组合极数、脱扣曲线和整定值
    let second_line = format!("{}-{}A", pole.to_str(), setting_value.to_str());
    // 第二行文字："3P-C 20A"
    painter.text(
        text2_pos,
        Align2::CENTER_CENTER,
        second_line,
        font_id.clone(),
        Color32::WHITE,
    );
}
