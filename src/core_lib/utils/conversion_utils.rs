/// 转换工具类
/// 提供各种单位转换和数据格式转换功能
pub struct ConversionUtils;

impl ConversionUtils {
    /// 创建新的转换工具实例
    pub fn new() -> Self {
        Self
    }
    
    /// 将伏特(V)转换为千伏(kV)
    /// - voltage_v: 电压(V)
    pub fn v_to_kv(voltage_v: f64) -> f64 {
        voltage_v / 1000.0
    }
    
    /// 将千伏(kV)转换为伏特(V)
    /// - voltage_kv: 电压(kV)
    pub fn kv_to_v(voltage_kv: f64) -> f64 {
        voltage_kv * 1000.0
    }
    
    /// 将瓦特(W)转换为千瓦(kW)
    /// - power_w: 功率(W)
    pub fn w_to_kw(power_w: f64) -> f64 {
        power_w / 1000.0
    }
    
    /// 将千瓦(kW)转换为瓦特(W)
    /// - power_kw: 功率(kW)
    pub fn kw_to_w(power_kw: f64) -> f64 {
        power_kw * 1000.0
    }
    
    /// 将米(m)转换为千米(km)
    /// - length_m: 长度(m)
    pub fn m_to_km(length_m: f64) -> f64 {
        length_m / 1000.0
    }
    
    /// 将千米(km)转换为米(m)
    /// - length_km: 长度(km)
    pub fn km_to_m(length_km: f64) -> f64 {
        length_km * 1000.0
    }
    
    /// 将摄氏度(°C)转换为开尔文(K)
    /// - temp_c: 温度(°C)
    pub fn celsius_to_kelvin(temp_c: f64) -> f64 {
        temp_c + 273.15
    }
    
    /// 将开尔文(K)转换为摄氏度(°C)
    /// - temp_k: 温度(K)
    pub fn kelvin_to_celsius(temp_k: f64) -> f64 {
        temp_k - 273.15
    }
    
    /// 将功率因数转换为角度(度)
    /// - power_factor: 功率因数
    pub fn power_factor_to_degrees(power_factor: f64) -> f64 {
        // 确保功率因数在有效范围内
        let power_factor = power_factor.max(-1.0).min(1.0);
        power_factor.acos().to_degrees()
    }
    
    /// 将角度(度)转换为功率因数
    /// - degrees: 角度(度)
    pub fn degrees_to_power_factor(degrees: f64) -> f64 {
        degrees.to_radians().cos()
    }
    
    /// 将毫米(mm)转换为米(m)
    /// - mm: 长度(mm)
    pub fn mm_to_m(mm: f64) -> f64 {
        mm / 1000.0
    }
    
    /// 将米(m)转换为毫米(mm)
    /// - m: 长度(m)
    pub fn m_to_mm(m: f64) -> f64 {
        m * 1000.0
    }
    
    /// 将平方毫米(mm²)转换为平方米(m²)
    /// - mm2: 面积(mm²)
    pub fn mm2_to_m2(mm2: f64) -> f64 {
        mm2 * 1e-6
    }
    
    /// 将平方米(m²)转换为平方毫米(mm²)
    /// - m2: 面积(m²)
    pub fn m2_to_mm2(m2: f64) -> f64 {
        m2 * 1e6
    }
    
    /// 将千克(kg)转换为牛顿(N)
    /// - kg: 质量(kg)
    pub fn kg_to_n(kg: f64) -> f64 {
        kg * 9.81 // 标准重力加速度
    }
    
    /// 将牛顿(N)转换为千克力(kgf)
    /// - n: 力(N)
    pub fn n_to_kgf(n: f64) -> f64 {
        n / 9.81
    }
    
    /// 将焦耳(J)转换为千瓦时(kWh)
    /// - j: 能量(J)
    pub fn j_to_kwh(j: f64) -> f64 {
        j / 3.6e6
    }
    
    /// 将千瓦时(kWh)转换为焦耳(J)
    /// - kwh: 能量(kWh)
    pub fn kwh_to_j(kwh: f64) -> f64 {
        kwh * 3.6e6
    }
    
    /// 将欧姆(Ω)转换为毫欧(mΩ)
    /// - ohm: 电阻(Ω)
    pub fn ohm_to_milliohm(ohm: f64) -> f64 {
        ohm * 1000.0
    }
    
    /// 将毫欧(mΩ)转换为欧姆(Ω)
    /// - milliohm: 电阻(mΩ)
    pub fn milliohm_to_ohm(milliohm: f64) -> f64 {
        milliohm / 1000.0
    }
    
    /// 将西门子(S)转换为毫西门子(mS)
    /// - s: 电导(S)
    pub fn s_to_ms(s: f64) -> f64 {
        s * 1000.0
    }
    
    /// 将毫西门子(mS)转换为西门子(S)
    /// - ms: 电导(mS)
    pub fn ms_to_s(ms: f64) -> f64 {
        ms / 1000.0
    }
    
    /// 将特斯拉(T)转换为高斯(G)
    /// - t: 磁感应强度(T)
    pub fn t_to_g(t: f64) -> f64 {
        t * 10000.0
    }
    
    /// 将高斯(G)转换为特斯拉(T)
    /// - g: 磁感应强度(G)
    pub fn g_to_t(g: f64) -> f64 {
        g / 10000.0
    }
    
    /// 将弧度转换为度
    /// - radians: 弧度值
    pub fn radians_to_degrees(radians: f64) -> f64 {
        radians.to_degrees()
    }
    
    /// 将度转换为弧度
    /// - degrees: 角度值(度)
    pub fn degrees_to_radians(degrees: f64) -> f64 {
        degrees.to_radians()
    }
    
    /// 将百分比转换为小数
    /// - percent: 百分比值
    pub fn percent_to_decimal(percent: f64) -> f64 {
        percent / 100.0
    }
    
    /// 将小数转换为百分比
    /// - decimal: 小数值
    pub fn decimal_to_percent(decimal: f64) -> f64 {
        decimal * 100.0
    }
    
    /// 格式化数值显示
    /// - value: 数值
    /// - decimals: 小数位数
    pub fn format_number(value: f64, decimals: usize) -> String {
        format!("{:.precision$}", value, precision = decimals)
    }
    
    /// 格式化大数值显示（添加千位分隔符）
    /// - value: 数值
    pub fn format_large_number(value: f64) -> String {
        // 使用科学计数法表示非常大的数
        if value.abs() >= 1e6 {
            format!("{:.2e}", value)
        } else {
            // 对于一般数值，添加千位分隔符
            let s = format!("{:.2}", value);
            let parts: Vec<&str> = s.split('.').collect();
            let mut integer_part = parts[0].to_string();
            
            // 添加千位分隔符
            let mut result = String::new();
            let mut count = 0;
            for c in integer_part.chars().rev() {
                if count > 0 && count % 3 == 0 {
                    result.push(',');
                }
                result.push(c);
                count += 1;
            }
            
            result = result.chars().rev().collect();
            
            if parts.len() > 1 {
                format!("{}.{}", result, parts[1])
            } else {
                result
            }
        }
    }
}