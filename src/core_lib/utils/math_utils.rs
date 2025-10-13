/// 数学工具类
/// 提供各种数学计算和辅助功能
pub struct MathUtils;

impl MathUtils {
    /// 创建新的数学工具实例
    pub fn new() -> Self {
        Self
    }
    
    /// 计算两个向量的点积
    /// - v1: 第一个向量
    /// - v2: 第二个向量
    pub fn dot_product(v1: (f64, f64, f64), v2: (f64, f64, f64)) -> f64 {
        v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
    }
    
    /// 计算向量的模长
    /// - v: 三维向量
    pub fn vector_magnitude(v: (f64, f64, f64)) -> f64 {
        (v.0.powi(2) + v.1.powi(2) + v.2.powi(2)).sqrt()
    }
    
    /// 计算二维向量的模长
    /// - v: 二维向量
    pub fn vector_magnitude_2d(v: (f64, f64)) -> f64 {
        (v.0.powi(2) + v.1.powi(2)).sqrt()
    }
    
    /// 计算两个二维点之间的距离
    /// - p1: 第一个点
    /// - p2: 第二个点
    pub fn distance_2d(p1: (f64, f64), p2: (f64, f64)) -> f64 {
        let dx = p2.0 - p1.0;
        let dy = p2.1 - p1.1;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }
    
    /// 计算两个三维点之间的距离
    /// - p1: 第一个点
    /// - p2: 第二个点
    pub fn distance_3d(p1: (f64, f64, f64), p2: (f64, f64, f64)) -> f64 {
        let dx = p2.0 - p1.0;
        let dy = p2.1 - p1.1;
        let dz = p2.2 - p1.2;
        (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
    }
    
    /// 计算三角形的面积（使用海伦公式）
    /// - a: 第一条边长度
    /// - b: 第二条边长度
    /// - c: 第三条边长度
    pub fn triangle_area(a: f64, b: f64, c: f64) -> f64 {
        // 检查三角形是否有效
        if a + b <= c || a + c <= b || b + c <= a {
            return 0.0;
        }
        
        let s = (a + b + c) / 2.0;
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }
    
    /// 计算矩形的面积
    /// - width: 宽度
    /// - height: 高度
    pub fn rectangle_area(width: f64, height: f64) -> f64 {
        width * height
    }
    
    /// 计算圆形的面积
    /// - radius: 半径
    pub fn circle_area(radius: f64) -> f64 {
        std::f64::consts::PI * radius.powi(2)
    }
    
    /// 计算圆形的周长
    /// - radius: 半径
    pub fn circle_circumference(radius: f64) -> f64 {
        2.0 * std::f64::consts::PI * radius
    }
    
    /// 计算两点连线的中点
    /// - p1: 第一个点
    /// - p2: 第二个点
    pub fn midpoint(p1: (f64, f64), p2: (f64, f64)) -> (f64, f64) {
        ((p1.0 + p2.0) / 2.0, (p1.1 + p2.1) / 2.0)
    }
    
    /// 计算两点连线的斜率
    /// - p1: 第一个点
    /// - p2: 第二个点
    pub fn slope(p1: (f64, f64), p2: (f64, f64)) -> Option<f64> {
        if (p2.0 - p1.0).abs() < f64::EPSILON {
            // 垂直线，斜率无穷大
            None
        } else {
            Some((p2.1 - p1.1) / (p2.0 - p1.0))
        }
    }
    
    /// 将角度限制在0-360度范围内
    /// - degrees: 角度(度)
    pub fn normalize_angle(degrees: f64) -> f64 {
        let mut normalized = degrees % 360.0;
        if normalized < 0.0 {
            normalized += 360.0;
        }
        normalized
    }
    
    /// 计算加权平均值
    /// - values: 数值数组
    /// - weights: 权重数组
    pub fn weighted_average(values: &[f64], weights: &[f64]) -> Option<f64> {
        if values.len() != weights.len() || values.is_empty() {
            return None;
        }
        
        let total_weight: f64 = weights.iter().sum();
        if total_weight == 0.0 {
            return None;
        }
        
        let weighted_sum: f64 = values.iter()
            .zip(weights.iter())
            .map(|(&v, &w)| v * w)
            .sum();
        
        Some(weighted_sum / total_weight)
    }
    
    /// 计算移动平均值
    /// - values: 数值数组
    /// - window_size: 窗口大小
    pub fn moving_average(values: &[f64], window_size: usize) -> Option<Vec<f64>> {
        if window_size > values.len() || window_size == 0 {
            return None;
        }
        
        let mut result = Vec::with_capacity(values.len() - window_size + 1);
        
        for i in 0..=values.len() - window_size {
            let window = &values[i..i + window_size];
            let average = window.iter().sum::<f64>() / window_size as f64;
            result.push(average);
        }
        
        Some(result)
    }
    
    /// 计算标准差
    /// - values: 数值数组
    pub fn standard_deviation(values: &[f64]) -> Option<f64> {
        if values.len() < 2 {
            return None;
        }
        
        let mean: f64 = values.iter().sum::<f64>() / values.len() as f64;
        let variance: f64 = values.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / (values.len() - 1) as f64;
        
        Some(variance.sqrt())
    }
    
    /// 计算最大值
    /// - values: 数值数组
    pub fn max(values: &[f64]) -> Option<f64> {
        if values.is_empty() {
            return None;
        }
        
        Some(*values.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap())
    }
    
    /// 计算最小值
    /// - values: 数值数组
    pub fn min(values: &[f64]) -> Option<f64> {
        if values.is_empty() {
            return None;
        }
        
        Some(*values.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).unwrap())
    }
    
    /// 计算平均值
    /// - values: 数值数组
    pub fn average(values: &[f64]) -> Option<f64> {
        if values.is_empty() {
            return None;
        }
        
        Some(values.iter().sum::<f64>() / values.len() as f64)
    }
    
    /// 将值限制在指定范围内
    /// - value: 要限制的值
    /// - min: 最小值
    /// - max: 最大值
    pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }
    
    /// 线性插值
    /// - a: 起始值
    /// - b: 结束值
    /// - t: 插值因子 (0.0-1.0)
    pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
        let t_clamped = Self::clamp(t, 0.0, 1.0);
        a + (b - a) * t_clamped
    }
    
    /// 缓动函数 - 平滑开始
    /// - t: 时间因子 (0.0-1.0)
    pub fn ease_in(t: f64) -> f64 {
        let t_clamped = Self::clamp(t, 0.0, 1.0);
        t_clamped * t_clamped
    }
    
    /// 缓动函数 - 平滑结束
    /// - t: 时间因子 (0.0-1.0)
    pub fn ease_out(t: f64) -> f64 {
        let t_clamped = Self::clamp(t, 0.0, 1.0);
        t_clamped * (2.0 - t_clamped)
    }
    
    /// 缓动函数 - 平滑开始和平滑结束
    /// - t: 时间因子 (0.0-1.0)
    pub fn ease_in_out(t: f64) -> f64 {
        let t_clamped = Self::clamp(t, 0.0, 1.0);
        3.0 * t_clamped.powi(2) - 2.0 * t_clamped.powi(3)
    }
    
    /// 检查两个浮点数是否近似相等
    /// - a: 第一个浮点数
    /// - b: 第二个浮点数
    /// - epsilon: 容差
    pub fn approximately_equal(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() <= epsilon
    }
    
    /// 计算对数平均值
    /// - a: 第一个数
    /// - b: 第二个数
    pub fn logarithmic_mean(a: f64, b: f64) -> Option<f64> {
        if a <= 0.0 || b <= 0.0 || (a - b).abs() < f64::EPSILON {
            return None;
        }
        
        Some((a - b) / (a / b).ln())
    }
}