use egui::Color32;

/// 将带有前导'#'的十六进制字符串转换为egui::Color32。
/// - 前三个通道被解释为R、G、B。
/// - 如果存在第四个通道，则用作alpha值。
/// - 十六进制值可以使用大写和小写字符。
///
/// *改编自：https://docs.rs/raster/0.1.0/src/raster/lib.rs.html#425-725。
/// 归功于原始作者。*
pub fn color_from_hex(hex: &str) -> Result<Color32, String> {
    // Convert a hex string to decimal. Eg. "00" -> 0. "FF" -> 255.
    fn _hex_dec(hex_string: &str) -> Result<u8, String> {
        match u8::from_str_radix(hex_string, 16) {
            Ok(o) => Ok(o),
            Err(e) => Err(format!("Error parsing hex: {}", e)),
        }
    }

    if hex.len() == 9 && hex.starts_with('#') {
        // #FFFFFFFF (Red Green Blue Alpha)
        return Ok(Color32::from_rgba_premultiplied(
            _hex_dec(&hex[1..3])?,
            _hex_dec(&hex[3..5])?,
            _hex_dec(&hex[5..7])?,
            _hex_dec(&hex[7..9])?,
        ));
    } else if hex.len() == 7 && hex.starts_with('#') {
        // #FFFFFF (Red Green Blue)
        return Ok(Color32::from_rgb(
            _hex_dec(&hex[1..3])?,
            _hex_dec(&hex[3..5])?,
            _hex_dec(&hex[5..7])?,
        ));
    }

    Err(format!(
        "Error parsing hex: {}. Example of valid formats: #FFFFFF or #ffffffff",
        hex
    ))
}

/// 将Color32转换为其标准十六进制表示形式。
/// - 颜色字符串将以'#'开头。
/// - 如果alpha通道完全不透明，将省略它。
/// - 从'a'到'f'的字符将以小写形式写入。
#[allow(dead_code)]
pub fn color_to_hex(color: Color32) -> String {
    if color.a() < 255 {
        format!(
            "#{:02x?}{:02x?}{:02x?}{:02x?}",
            color.r(),
            color.g(),
            color.b(),
            color.a()
        )
    } else {
        format!("#{:02x?}{:02x?}{:02x?}", color.r(), color.g(), color.b())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_color_from_and_to_hex() {
        assert_eq!(
            color_from_hex("#00ff00").unwrap(),
            Color32::from_rgb(0, 255, 0)
        );
        assert_eq!(
            color_from_hex("#5577AA").unwrap(),
            Color32::from_rgb(85, 119, 170)
        );
        assert_eq!(
            color_from_hex("#E2e2e277").unwrap(),
            Color32::from_rgba_premultiplied(226, 226, 226, 119)
        );
        assert!(color_from_hex("abcdefgh").is_err());

        assert_eq!(
            color_to_hex(Color32::from_rgb(0, 255, 0)),
            "#00ff00".to_string()
        );
        assert_eq!(
            color_to_hex(Color32::from_rgb(85, 119, 170)),
            "#5577aa".to_string()
        );
        assert_eq!(
            color_to_hex(Color32::from_rgba_premultiplied(226, 226, 226, 119)),
            "#e2e2e277".to_string()
        );
    }
}
