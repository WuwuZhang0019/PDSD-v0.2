/// 电流计算工具
pub struct CurrentCalculator;

impl CurrentCalculator {
    /// 创建新的电流计算器
    pub fn new() -> Self {
        Self
    }

    /// 计算单相电路电流
    /// - pe: 额定功率
    /// - kx: 需要系数
    /// - cos: 余弦值
    /// 返回值 ijs: 计算电流
    pub fn single_phase_current_calculation(pe: f32, kx: f32, cos: f32) -> f32 {
        const U: f32 = 0.22; // 额定电压(单位: kV)
        let ijs: f32 = pe * kx / U / cos;
        ijs
    }

    /// 计算三相电路电流
    /// - pe: 额定功率
    /// - kx: 需要系数
    /// - cos: 余弦值
    /// 返回值 ijs: 计算电流
    pub fn three_phase_current_calculation(pe: f32, kx: f32, cos: f32) -> f32 {
        const U: f32 = 0.38; // 额定电压(单位: kV)
        let sqrt3: f32 = 3.0_f32.sqrt();
        let ijs: f32 = pe * kx / U / cos / sqrt3;
        ijs
    }
}
