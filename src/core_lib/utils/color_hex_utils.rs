use std::str::FromStr;

/// 颜色十六进制工具类
/// 提供颜色转换和处理功能
pub struct ColorHexUtils;

impl ColorHexUtils {
    /// 创建新的颜色工具实例
    pub fn new() -> Self {
        Self
    }
    
    /// 将RGB值转换为十六进制颜色字符串
    /// - r: 红色分量(0-255)
    /// - g: 绿色分量(0-255)
    /// - b: 蓝色分量(0-255)
    pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
    
    /// 将RGBA值转换为十六进制颜色字符串
    /// - r: 红色分量(0-255)
    /// - g: 绿色分量(0-255)
    /// - b: 蓝色分量(0-255)
    /// - a: 透明度分量(0-255)
    pub fn rgba_to_hex(r: u8, g: u8, b: u8, a: u8) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
    }
    
    /// 将十六进制颜色字符串转换为RGB值
    /// - hex: 十六进制颜色字符串，格式为"#RRGGBB"或"#RRGGBBAA"
    pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
        // 移除#前缀（如果存在）
        let hex = hex.trim_start_matches('#');
        
        // 检查长度是否为6（RGB）或8（RGBA）
        if hex.len() != 6 && hex.len() != 8 {
            return None;
        }
        
        // 解析RGB部分
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        
        Some((r, g, b))
    }
    
    /// 将十六进制颜色字符串转换为RGBA值
    /// - hex: 十六进制颜色字符串，格式为"#RRGGBB"或"#RRGGBBAA"
    pub fn hex_to_rgba(hex: &str) -> Option<(u8, u8, u8, u8)> {
        // 移除#前缀（如果存在）
        let hex = hex.trim_start_matches('#');
        
        // 解析RGB部分
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        
        // 解析Alpha部分（如果存在）
        let a = if hex.len() >= 8 {
            u8::from_str_radix(&hex[6..8], 16).ok()?
        } else {
            255 // 默认不透明
        };
        
        Some((r, g, b, a))
    }
    
    /// 调整颜色亮度
    /// - hex: 原始颜色的十六进制字符串
    /// - factor: 亮度调整因子（<1.0变暗，>1.0变亮）
    pub fn adjust_brightness(hex: &str, factor: f64) -> Option<String> {
        let (r, g, b) = Self::hex_to_rgb(hex)?;
        
        // 将RGB值转换为0-1范围并调整亮度
        let r = (r as f64 * factor).max(0.0).min(255.0) as u8;
        let g = (g as f64 * factor).max(0.0).min(255.0) as u8;
        let b = (b as f64 * factor).max(0.0).min(255.0) as u8;
        
        Some(Self::rgb_to_hex(r, g, b))
    }
    
    /// 创建颜色的互补色
    /// - hex: 原始颜色的十六进制字符串
    pub fn complementary_color(hex: &str) -> Option<String> {
        let (r, g, b) = Self::hex_to_rgb(hex)?;
        
        // 互补色是每个通道取255减去原始值
        let comp_r = 255 - r;
        let comp_g = 255 - g;
        let comp_b = 255 - b;
        
        Some(Self::rgb_to_hex(comp_r, comp_g, comp_b))
    }
    
    /// 检查颜色是否为亮色（适合深色文本）
    /// - hex: 颜色的十六进制字符串
    pub fn is_light_color(hex: &str) -> Option<bool> {
        let (r, g, b) = Self::hex_to_rgb(hex)?;
        
        // 计算亮度（使用HSP公式）
        let brightness = ((r as f64).powi(2) * 0.299 + 
                         (g as f64).powi(2) * 0.587 + 
                         (b as f64).powi(2) * 0.114).sqrt();
        
        Some(brightness > 127.5)
    }
    
    /// 获取文本的对比度颜色（黑或白）
    /// - background_hex: 背景颜色
    pub fn get_contrast_text_color(background_hex: &str) -> Option<String> {
        if let Some(is_light) = Self::is_light_color(background_hex) {
            if is_light {
                Some("#000000".to_string()) // 浅色背景用黑色文本
            } else {
                Some("#FFFFFF".to_string()) // 深色背景用白色文本
            }
        } else {
            None
        }
    }
    
    /// 预定义的电气系统颜色
    pub fn get_electrical_colors() -> std::collections::HashMap<&'static str, &'static str> {
        let mut colors = std::collections::HashMap::new();
        
        // 相序颜色
        colors.insert("phase_a", "#FF0000");    // A相 - 红色
        colors.insert("phase_b", "#00FF00");    // B相 - 绿色
        colors.insert("phase_c", "#0000FF");    // C相 - 蓝色
        colors.insert("neutral", "#FFFF00");   // 零线 - 黄色
        colors.insert("ground", "#FFA500");    // 地线 - 橙色
        
        // 设备颜色
        colors.insert("transformer", "#808080"); // 变压器 - 灰色
        colors.insert("circuit_breaker", "#C0C0C0"); // 断路器 - 银色
        colors.insert("switch", "#696969");      // 开关 - 深灰色
        colors.insert("load", "#FF6347");        // 负载 - 番茄红
        
        // 状态颜色
        colors.insert("normal", "#00FF00");     // 正常 - 绿色
        colors.insert("warning", "#FFFF00");    // 警告 - 黄色
        colors.insert("error", "#FF0000");      // 错误 - 红色
        colors.insert("offline", "#808080");    // 离线 - 灰色
        
        colors
    }
}

// 为了向后兼容性，保留原来的函数
#[cfg(feature = "with_egui")]
pub fn color_from_hex(hex: &str) -> Result<egui::Color32, String> {
    // Convert a hex string to decimal. E.g. "00" -> 0. "FF" -> 255.
    fn _hex_dec(hex_string: &str) -> Result<u8, String> {
        match u8::from_str_radix(hex_string, 16) {
            Ok(o) => Ok(o),
            Err(e) => Err(format!("Error parsing hex: {}", e)),
        }
    }

    if hex.len() == 9 && hex.starts_with('#') {
        // #FFFFFFFF (Red Green Blue Alpha)
        return Ok(egui::Color32::from_rgba_premultiplied(
            _hex_dec(&hex[1..3])?,
            _hex_dec(&hex[3..5])?,
            _hex_dec(&hex[5..7])?,
            _hex_dec(&hex[7..9])?,
        ));
    } else if hex.len() == 7 && hex.starts_with('#') {
        // #FFFFFF (Red Green Blue)
        return Ok(egui::Color32::from_rgb(
            _hex_dec(&hex[1..3])?,
            _hex_dec(&hex[3..5])?,
            _hex_dec(&hex[5..7])?,
        ));
    }

    Err(format!(
        "Error parsing hex: {}. Example of valid formats: #FFFFFF or #FFFFFFFF",
        hex
    ))
}

#[cfg(feature = "with_egui")]
#[allow(dead_code)]
pub fn color_to_hex(color: egui::Color32) -> String {
    if color.a() < 255 {
        format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            color.r(),
            color.g(),
            color.b(),
            color.a()
        )
    } else {
        format!("#{:02x}{:02x}{:02x}", color.r(), color.g(), color.b())
    }
}