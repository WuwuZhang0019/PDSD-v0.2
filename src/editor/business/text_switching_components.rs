/// 开关元器件文字标注库
use serde::{Deserialize, Serialize};

/// 开关元器件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreakerType {
    ACB,      //框架断路器
    MCCB,     // 塑壳断路器
    MCCBRCBO, //塑壳漏电断路器
    MCB,      // 微型断路器
    RCBO,     //漏电断路器
    IS,       //隔离开关
    ATS,      //双电源自动转换开关
}
// 为枚举实现“显示字符串”转换（UI展示用）
impl BreakerType {
    pub fn to_str(&self) -> &'static str {
        match self {
            BreakerType::ACB => "ACB",
            BreakerType::MCCB => "MCCB",
            BreakerType::MCCBRCBO => "MCCB-RCBO",
            BreakerType::MCB => "MCB",
            BreakerType::RCBO => "RCBO",
            BreakerType::IS => "IS",
            BreakerType::ATS => "ATS-PC(CB)",
        }
    }
}
/// 壳架电流
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrameCurrent {
    A63,
    A100,
    A160,
    A250,
    A400,
    A630,
    A800,
}
impl FrameCurrent {
    pub fn to_str(&self) -> &'static str {
        match self {
            FrameCurrent::A63 => "63",
            FrameCurrent::A100 => "100",
            FrameCurrent::A160 => "160",
            FrameCurrent::A250 => "250",
            FrameCurrent::A400 => "400",
            FrameCurrent::A630 => "630",
            FrameCurrent::A800 => "800",
        }
    }
}
/// 分断能力
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreakingCapacity {
    F, //36kA
    N, //50kA
    H, //70kA
    S, //100kA
    L, //150kA
}
impl BreakingCapacity {
    pub fn to_str(&self) -> &'static str {
        match self {
            BreakingCapacity::F => "F",
            BreakingCapacity::N => "N",
            BreakingCapacity::H => "H",
            BreakingCapacity::S => "S",
            BreakingCapacity::L => "L",
        }
    }
}
/// 脱扣方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeductionMethod {
    TM, //热磁型
    MA, //单磁型
}
impl DeductionMethod {
    pub fn to_str(&self) -> &'static str {
        match self {
            DeductionMethod::TM => "TM",
            DeductionMethod::MA => "MA",
        }
    }
}
/// 极数
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Pole {
    P1,
    P2,
    P3,
    P4,
    P1N,
    P3N,
}
impl Pole {
    pub fn to_str(&self) -> &'static str {
        match self {
            Pole::P1 => "1P",
            Pole::P2 => "2P",
            Pole::P3 => "3P",
            Pole::P4 => "4P",
            Pole::P1N => "1P+N",
            Pole::P3N => "3P+N",
        }
    }
}
/// 脱扣曲线
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Curve {
    B,
    C,
    D,
}
impl Curve {
    pub fn to_str(&self) -> &'static str {
        match self {
            Curve::B => "B",
            Curve::C => "C",
            Curve::D => "D",
        }
    }
}
/// 整定值
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingValue {
    A16,
    A20,
    A25,
    A32,
    A40,
    A50,
    A63,
    A80,
    A100,
    A125,
    A140,
    A160,
    A180,
    A200,
    A225,
    A250,
    A315,
    A350,
    A400,
    A500,
    A630,
    A800,
    A1000,
}
impl SettingValue {
    pub fn to_str(&self) -> &'static str {
        match self {
            SettingValue::A16 => "16",
            SettingValue::A20 => "20",
            SettingValue::A25 => "25",
            SettingValue::A32 => "32",
            SettingValue::A40 => "40",
            SettingValue::A50 => "50",
            SettingValue::A63 => "63",
            SettingValue::A80 => "80",
            SettingValue::A100 => "100",
            SettingValue::A125 => "125",
            SettingValue::A140 => "140",
            SettingValue::A160 => "160",
            SettingValue::A180 => "180",
            SettingValue::A200 => "200",
            SettingValue::A225 => "225",
            SettingValue::A250 => "250",
            SettingValue::A315 => "315",
            SettingValue::A350 => "350",
            SettingValue::A400 => "400",
            SettingValue::A500 => "500",
            SettingValue::A630 => "630",
            SettingValue::A800 => "800",
            SettingValue::A1000 => "1000",
        }
    }
}

/// 相位配置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    SinglePhase, // 单相
    ThreePhase,  // 三相
}
impl Default for Phase {
    fn default() -> Self {
        Phase::ThreePhase // 默认三相
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicBreakerText {
    pub load_current: f32,                           // 负载电流值(A)
    pub phase: Phase,                                // 相位配置
    pub breaker_type: Option<BreakerType>,           // 断路器类型(可选)
    pub setting_value: Option<SettingValue>,         // 整定值(可选)
    pub frame_current: Option<FrameCurrent>,         // 壳架电流(可选)
    pub curve: Option<Curve>,                        // 脱扣曲线(可选)
    pub pole: Option<Pole>,                          // 极数(可选)
    pub breaking_capacity: Option<BreakingCapacity>, // 分断能力(可选)
    pub deduction_method: Option<DeductionMethod>,   // 脱扣方式(可选)
}
impl Default for DynamicBreakerText {
    fn default() -> Self {
        Self {
            load_current: 0.0,
            phase: Phase::default(),
            breaker_type: None,
            setting_value: None,
            frame_current: None,
            curve: None,
            pole: None,
            breaking_capacity: None,
            deduction_method: None,
        }
    }
}
impl DynamicBreakerText {
    /// 根据负载电流自动计算整定值
    pub fn compute_setting_value(load_current: f32) -> SettingValue {
        // 整定值必须大于负载电流（1.25倍安全系数）
        let required_current = load_current * 1.25;

        // 查找最接近的标准整定值
        let setting_values = [
            SettingValue::A16,
            SettingValue::A20,
            SettingValue::A25,
            SettingValue::A32,
            SettingValue::A40,
            SettingValue::A50,
            SettingValue::A63,
            SettingValue::A80,
            SettingValue::A100,
            SettingValue::A125,
            SettingValue::A140,
            SettingValue::A160,
            SettingValue::A180,
            SettingValue::A200,
            SettingValue::A225,
            SettingValue::A250,
            SettingValue::A315,
            SettingValue::A350,
            SettingValue::A400,
            SettingValue::A500,
            SettingValue::A630,
            SettingValue::A800,
            SettingValue::A1000,
        ];

        // 返回第一个大于等于所需电流的标准值
        setting_values
            .into_iter()
            .find(|&sv| sv.to_f32() >= required_current)
            .unwrap_or(SettingValue::A1000) // 默认最大整定值
    }

    /// 根据参数智能计算断路器类型
    pub fn compute_breaker_type(&self) -> BreakerType {
        // 使用用户自定义类型（如有）
        if let Some(t) = self.breaker_type {
            return t;
        }

        // 根据负载电流智能选择默认类型
        if self.load_current <= 63.0 {
            BreakerType::MCB
        } else if self.load_current <= 250.0 {
            BreakerType::MCCB
        } else {
            BreakerType::ACB
        }
    }

    /// 获取或计算所有参数
    pub fn resolved_params(
        &self,
    ) -> (
        BreakerType,
        FrameCurrent,
        Pole,
        Curve,
        SettingValue,
        BreakingCapacity,
        DeductionMethod,
    ) {
        (
            self.compute_breaker_type(),
            self.frame_current.unwrap_or(FrameCurrent::A630),
            self.pole.unwrap_or(match self.phase {
                Phase::SinglePhase => Pole::P1N,
                Phase::ThreePhase => Pole::P3N,
            }),
            self.curve.unwrap_or(Curve::C),
            self.setting_value
                .unwrap_or_else(|| Self::compute_setting_value(self.load_current)),
            self.breaking_capacity.unwrap_or(BreakingCapacity::N),
            self.deduction_method.unwrap_or(DeductionMethod::TM),
        )
    }
}
/// 增加枚举值转浮点数的功能
impl SettingValue {
    pub fn to_f32(&self) -> f32 {
        match self {
            SettingValue::A16 => 16.0,
            SettingValue::A20 => 20.0,
            SettingValue::A25 => 25.0,
            SettingValue::A32 => 32.0,
            SettingValue::A40 => 40.0,
            SettingValue::A50 => 50.0,
            SettingValue::A63 => 63.0,
            SettingValue::A80 => 80.0,
            SettingValue::A100 => 100.0,
            SettingValue::A125 => 125.0,
            SettingValue::A140 => 140.0,
            SettingValue::A160 => 160.0,
            SettingValue::A180 => 180.0,
            SettingValue::A200 => 200.0,
            SettingValue::A225 => 225.0,
            SettingValue::A250 => 250.0,
            SettingValue::A315 => 315.0,
            SettingValue::A350 => 350.0,
            SettingValue::A400 => 400.0,
            SettingValue::A500 => 500.0,
            SettingValue::A630 => 630.0,
            SettingValue::A800 => 800.0,
            SettingValue::A1000 => 1000.0,
        }
    }
}
