use egui::Color32;

pub fn color_from_hex(hex: &str) -> Result<Color32, String> {
    // Convert a hex string to decimal. E.g. "00" -> 0. "FF" -> 255.
    //将十六进制字符串转换为十进制。如"00"-> 0."ff" -> 255。
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