/// 验证工具类
/// 提供数据验证和检查功能
pub struct ValidationUtils;

impl ValidationUtils {
    /// 创建新的验证工具实例
    pub fn new() -> Self {
        Self
    }
    
    /// 验证是否为有效数字（非NaN和非无穷大）
    /// - value: 要验证的浮点数
    pub fn is_valid_number(value: f64) -> bool {
        !value.is_nan() && !value.is_infinite()
    }
    
    /// 验证值是否在指定范围内
    /// - value: 要验证的值
    /// - min: 最小值
    /// - max: 最大值
    pub fn is_in_range<T: PartialOrd>(value: T, min: T, max: T) -> bool {
        value >= min && value <= max
    }
    
    /// 验证字符串是否为空
    /// - s: 要验证的字符串
    pub fn is_not_empty(s: &str) -> bool {
        !s.trim().is_empty()
    }
    
    /// 验证字符串长度是否在指定范围内
    /// - s: 要验证的字符串
    /// - min_length: 最小长度
    /// - max_length: 最大长度
    pub fn is_length_valid(s: &str, min_length: usize, max_length: usize) -> bool {
        let length = s.len();
        length >= min_length && length <= max_length
    }
    
    /// 验证是否为有效的百分比值（0-100）
    /// - percent: 百分比值
    pub fn is_valid_percentage(percent: f64) -> bool {
        Self::is_valid_number(percent) && Self::is_in_range(percent, 0.0, 100.0)
    }
    
    /// 验证是否为有效的功率因数值（0-1）
    /// - power_factor: 功率因数值
    pub fn is_valid_power_factor(power_factor: f64) -> bool {
        Self::is_valid_number(power_factor) && Self::is_in_range(power_factor, 0.0, 1.0)
    }
    
    /// 验证是否为有效的正数值
    /// - value: 要验证的值
    pub fn is_positive(value: f64) -> bool {
        Self::is_valid_number(value) && value > 0.0
    }
    
    /// 验证是否为有效的非负值
    /// - value: 要验证的值
    pub fn is_non_negative(value: f64) -> bool {
        Self::is_valid_number(value) && value >= 0.0
    }
    
    /// 验证是否为有效的电压值（常见电压范围）
    /// - voltage: 电压值(V)
    pub fn is_valid_voltage(voltage: f64) -> bool {
        Self::is_valid_number(voltage) && {
            // 常见电压等级
            let common_voltages = [
                12.0, 24.0, 36.0,   // 低压直流
                110.0, 220.0, 230.0, 240.0,  // 低压交流单相
                380.0, 400.0, 415.0,  // 低压交流三相
                660.0, 690.0,         // 中压
                3300.0, 6600.0, 10000.0, 11000.0 // 高压
            ];
            
            // 检查是否在常见电压等级附近（允许10%误差）
            common_voltages.iter().any(|&v| {
                let lower = v * 0.9;
                let upper = v * 1.1;
                voltage >= lower && voltage <= upper
            })
        }
    }
    
    /// 验证是否为有效的电流值
    /// - current: 电流值(A)
    pub fn is_valid_current(current: f64) -> bool {
        Self::is_valid_number(current) && Self::is_in_range(current, 0.0, 5000.0)
    }
    
    /// 验证是否为有效的功率值
    /// - power: 功率值(kW)
    pub fn is_valid_power(power: f64) -> bool {
        Self::is_valid_number(power) && Self::is_in_range(power, 0.0, 10000.0)
    }
    
    /// 验证是否为有效的导线截面积
    /// - wire_size: 导线截面积(mm²)
    pub fn is_valid_wire_size(wire_size: f64) -> bool {
        Self::is_valid_number(wire_size) && {
            // 标准导线截面积系列
            let standard_sizes = [
                1.0, 1.5, 2.5, 4.0, 6.0, 10.0, 16.0, 25.0, 
                35.0, 50.0, 70.0, 95.0, 120.0, 150.0, 185.0, 240.0, 
                300.0, 400.0, 500.0, 630.0
            ];
            
            standard_sizes.contains(&wire_size)
        }
    }
    
    /// 验证是否为有效的相数
    /// - phase_count: 相数
    pub fn is_valid_phase_count(phase_count: u32) -> bool {
        phase_count == 1 || phase_count == 3
    }
    
    /// 验证是否为有效的需求系数
    /// - demand_coefficient: 需求系数
    pub fn is_valid_demand_coefficient(demand_coefficient: f64) -> bool {
        Self::is_valid_number(demand_coefficient) && Self::is_in_range(demand_coefficient, 0.0, 1.0)
    }
    
    /// 验证是否为有效的电压损失率
    /// - voltage_drop_percent: 电压损失率(%)
    pub fn is_valid_voltage_drop(voltage_drop_percent: f64) -> bool {
        Self::is_valid_number(voltage_drop_percent) && voltage_drop_percent <= 5.0
    }
    
    /// 验证是否为有效的三相不平衡度
    /// - imbalance_percent: 不平衡度(%)
    pub fn is_valid_phase_imbalance(imbalance_percent: f64) -> bool {
        Self::is_valid_number(imbalance_percent) && imbalance_percent <= 20.0
    }
    
    /// 验证是否为有效的连接类型
    /// - connection_type: 连接类型字符串
    pub fn is_valid_connection_type(connection_type: &str) -> bool {
        let valid_types = ["star", "delta", "星型", "三角形", "Y", "Δ", "D"];
        valid_types.contains(&connection_type.to_lowercase().as_str())
    }
    
    /// 验证是否为有效的绝缘等级
    /// - insulation_class: 绝缘等级字符串
    pub fn is_valid_insulation_class(insulation_class: &str) -> bool {
        let valid_classes = ["A", "E", "B", "F", "H", "C"];
        valid_classes.contains(&insulation_class.to_uppercase().as_str())
    }
    
    /// 验证是否为有效的防护等级
    /// - ip_rating: IP防护等级字符串
    pub fn is_valid_ip_rating(ip_rating: &str) -> bool {
        // IP等级格式: IP后跟两位数字
        let ip_rating = ip_rating.to_uppercase();
        if ip_rating.len() != 4 || !ip_rating.starts_with("IP") {
            return false;
        }
        
        // 直接检查字符是否为数字，避免unwrap_or
        // 由于前面已经检查了长度为4，这里可以安全访问
        ip_rating.chars().nth(2).map_or(false, |c| c.is_digit(10)) && 
        ip_rating.chars().nth(3).map_or(false, |c| c.is_digit(10))
    }
    
    /// 验证UUID格式
    /// - uuid: UUID字符串
    pub fn is_valid_uuid(uuid: &str) -> bool {
        // 简单的UUID格式验证 (8-4-4-4-12)
        if uuid.len() != 36 {
            return false;
        }
        
        // 检查连字符位置
        if uuid.chars().nth(8) != Some('-') || 
           uuid.chars().nth(13) != Some('-') || 
           uuid.chars().nth(18) != Some('-') || 
           uuid.chars().nth(23) != Some('-') {
            return false;
        }
        
        // 检查其他字符是否为十六进制字符
        uuid.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
    }
}