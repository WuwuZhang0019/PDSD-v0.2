use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 由于egui不在core_lib中直接使用，需要条件编译
#[cfg(feature = "with_egui")]
extern crate egui;
#[cfg(feature = "with_egui")]
use egui;

/// 电压等级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoltageLevel {
    V400_750,
    V600_1000,
}

impl VoltageLevel {
    pub fn to_str(&self) -> &'static str {
        match self {
            VoltageLevel::V400_750 => "400V/750V",
            VoltageLevel::V600_1000 => "0.6V/1kV",
        }
    }
}

/// 敷设方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayingMethod {
    SC,  //焊接钢管
    JDG, //紧定管
    PC,  //硬塑料导管
    PVC, //塑料导管
    CT,  //桥架
    MR,  //金属线槽
    RC,  //镀锌钢管
}

impl LayingMethod {
    pub fn to_str(&self) -> &'static str {
        match self {
            LayingMethod::SC => "SC",
            LayingMethod::JDG => "JDG",
            LayingMethod::PC => "PC",
            LayingMethod::PVC => "PVC",
            LayingMethod::CT => "CT",
            LayingMethod::MR => "MR",
            LayingMethod::RC => "RC",
        }
    }
}

///穿管规格
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PipeSpecification {
    P15,
    P20,
    P25,
    P32,
    P40,
    P50,
    P65,
    P70,
    P100,
    P150,
    P200,
}

impl PipeSpecification {
    pub fn to_str(&self) -> &'static str {
        match self {
            PipeSpecification::P15 => "15",
            PipeSpecification::P20 => "20",
            PipeSpecification::P25 => "25",
            PipeSpecification::P32 => "32",
            PipeSpecification::P40 => "40",
            PipeSpecification::P50 => "50",
            PipeSpecification::P65 => "65",
            PipeSpecification::P70 => "70",
            PipeSpecification::P100 => "100",
            PipeSpecification::P150 => "150",
            PipeSpecification::P200 => "200",
        }
    }
}

/// 敷设部位
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayingArea {
    WC,  // 沿墙暗敷
    CC,  // 沿顶板暗敷
    FC,  // 沿底板暗敷
    WS,  // 沿墙明敷
    SCE, // 吊顶内明敷
    CE,  // 顶板明敷
    E,   //明敷
}

impl LayingArea {
    pub fn to_str(&self) -> &'static str {
        match self {
            LayingArea::WC => "WC",
            LayingArea::CC => "CC",
            LayingArea::FC => "FC",
            LayingArea::WS => "WS",
            LayingArea::SCE => "SCE",
            LayingArea::CE => "CE",
            LayingArea::E => "E",
        }
    }
}

/// 相序枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhaseSequence {
    L1,
    L2,
    L3,
    L1L2L3,
}

impl PhaseSequence {
    pub fn to_str(&self) -> &'static str {
        match self {
            PhaseSequence::L1 => "L1",
            PhaseSequence::L2 => "L2",
            PhaseSequence::L3 => "L3",
            PhaseSequence::L1L2L3 => "L1L2L3",
        }
    }
}

/// 相序
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Phase {
    L1,    // A相
    L2,    // B相
    L3,    // C相
    ThreePhase, // 三相(空值表示)
}

/// 回路编号 - 用于按照输出端口连接顺序为回路分配编号
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CircuitNumber {
    pub number: u32,
}

impl CircuitNumber {
    /// 创建新的回路编号
    pub fn new(number: u32) -> Self {
        Self { number }
    }

    /// 根据字符串生成回路编号
    pub fn from_str(s: &str) -> Option<Self> {
        // 检查字符串是否以"WL"开头
        if s.starts_with("WL") {
            // 尝试解析"WL"后面的数字部分
            if let Some(num_str) = s.get(2..) {
                if let Ok(num) = num_str.parse::<u32>() {
                    return Some(Self { number: num });
                }
            }
        }
        None
    }

    /// 获取下一个回路编号
    pub fn next(&self) -> Self {
        Self { number: self.number + 1 }
    }

    /// 转换为字符串表示
    pub fn to_str(&self) -> String {
        format!("WL{}", self.number)
    }

    /// 比较两个回路编号的顺序
    pub fn compare(&self, other: &Self) -> std::cmp::Ordering {
        self.number.cmp(&other.number)
    }
}

/// 断路器类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreakerType {
    ACB,      //框架断路器
    MCCB,     // 塑壳断路器
    MCCB_RCBO, //塑壳漏电断路器
    MCB,      // 微型断路器
    RCBO,     //漏电断路器
    IS,       //隔离开关
    ATS,      //双电源自动转换开关
}

impl BreakerType {
    pub fn to_str(&self) -> &'static str {
        match self {
            BreakerType::ACB => "ACB",
            BreakerType::MCCB => "MCCB",
            BreakerType::MCCB_RCBO => "MCCB-RCBO",
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
    
    pub fn to_f64(&self) -> f64 {
        match self {
            FrameCurrent::A63 => 63.0,
            FrameCurrent::A100 => 100.0,
            FrameCurrent::A160 => 160.0,
            FrameCurrent::A250 => 250.0,
            FrameCurrent::A400 => 400.0,
            FrameCurrent::A630 => 630.0,
            FrameCurrent::A800 => 800.0,
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
    
    /// 获取极数对应的数值
    pub fn to_u32(&self) -> u32 {
        match self {
            Pole::P1 => 1,
            Pole::P2 => 2,
            Pole::P3 => 3,
            Pole::P4 => 4,
            Pole::P1N => 2,
            Pole::P3N => 4,
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

/// 整定电流值
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SettingValue {
    A6,   //6A
    A10,  //10A
    A16,  //16A
    A20,  //20A
    A25,  //25A
    A32,  //32A
    A40,  //40A
    A50,  //50A
    A63,  //63A
    A80,  //80A
    A100, //100A
    A125, //125A
    A140, //140A
    A160, //160A
    A180, //180A
    A200, //200A
    A225, //225A
    A250, //250A
    A315, //315A
    A350, //350A
    A400, //400A
    A500, //500A
    A630, //630A
    A800, //800A
    A1000, //1000A
    A1250, //1250A
    A1600, //1600A
    A2000, //2000A
}

impl SettingValue {
    pub fn to_str(&self) -> &'static str {
        match self {
            SettingValue::A6 => "6",
            SettingValue::A10 => "10",
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
            SettingValue::A1250 => "1250",
            SettingValue::A1600 => "1600",
            SettingValue::A2000 => "2000",
        }
    }
    
    pub fn to_f64(&self) -> f64 {
        match self {
            SettingValue::A6 => 6.0,
            SettingValue::A10 => 10.0,
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
            SettingValue::A1250 => 1250.0,
            SettingValue::A1600 => 1600.0,
            SettingValue::A2000 => 2000.0,
        }
    }
}

/// 相位配置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhaseConfig {
    SinglePhase,        // 单相
    ThreePhase,         // 三相
    ThreePhaseFourWire, // 三相四线
}

impl Default for PhaseConfig {
    fn default() -> Self {
        PhaseConfig::ThreePhase // 默认三相
    }
}

/// 断路器参数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Breaker {
    pub breaker_type: Option<BreakerType>,      // 断路器类型
    pub model: String,                          // 断路器型号
    pub frame_current: FrameCurrent,            // 壳架电流值(A)
    pub rated_current: SettingValue,            // 整定电流值(A)
    pub pole: Option<Pole>,                     // 极数
    pub breaking_capacity: Option<BreakingCapacity>, // 分断能力
    pub curve: Option<Curve>,                   // 脱扣曲线
    pub deduction_method: Option<DeductionMethod>, // 脱扣方式
}

impl Default for Breaker {
    fn default() -> Self {
        Self {
            breaker_type: Some(BreakerType::MCCB),
            model: "NSX100N".to_string(),
            frame_current: FrameCurrent::A100,
            rated_current: SettingValue::A63,
            pole: Some(Pole::P3),
            breaking_capacity: Some(BreakingCapacity::N),
            curve: Some(Curve::C),
            deduction_method: Some(DeductionMethod::TM),
        }
    }
}

/// 隔离开关参数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Isolator {
    pub model: String,                          // 隔离开关型号
    pub frame_current: FrameCurrent,            // 壳架电流值(A)
    pub rated_current: SettingValue,            // 额定电流(A)
    pub pole: Pole,                             // 极数
    pub insulation_level: f64,                  // 绝缘水平(kV)
}

impl Default for Isolator {
    fn default() -> Self {
        Self {
            model: "GL-100".to_string(),
            frame_current: FrameCurrent::A100,
            rated_current: SettingValue::A100,
            pole: Pole::P3,
            insulation_level: 0.69,
        }
    }
}

/// 双电源开关参数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DualPowerSwitch {
    pub model: String,                          // 双电源开关型号
    pub frame_current: FrameCurrent,            // 壳架电流值(A)
    pub rated_current: SettingValue,            // 额定电流(A)
    pub pole: Pole,                             // 极数
    pub breaking_capacity: Option<BreakingCapacity>, // 分断能力
    pub transfer_time: f64,                     // 切换时间(ms)
    pub operation_type: String,                 // 操作方式(自动/手动)
}

impl Default for DualPowerSwitch {
    fn default() -> Self {
        Self {
            model: "ATSE-100".to_string(),
            frame_current: FrameCurrent::A100,
            rated_current: SettingValue::A100,
            pole: Pole::P4,
            breaking_capacity: Some(BreakingCapacity::N),
            transfer_time: 100.0,
            operation_type: "自动".to_string(),
        }
    }
}

/// 接触器参数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contactor {
    pub model: String,                          // 接触器型号
    pub frame_current: FrameCurrent,            // 壳架电流值(A)
    pub rated_current: SettingValue,            // 额定电流(A)
    pub pole: Pole,                             // 极数
    pub control_voltage: f64,                   // 控制电压(V)
    pub breaking_capacity: Option<BreakingCapacity>, // 分断能力
}

/// 电能表参数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnergyMeter {
    pub model: String,                          // 电能表型号
    pub rated_current: SettingValue,            // 额定电流(A)
    pub max_current: SettingValue,              // 最大电流(A)
    pub accuracy_class: f64,                    // 精度等级
    pub communication: String,                  // 通讯方式
    pub phase_config: Option<PhaseConfig>,      // 相位配置
}

/// 线缆参数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CableInfo {
    pub model: String,      // 线缆型号
    pub voltage_level: f64, // 电压等级(kV)
    pub size: String,       // 线缆规格(如: BV-2.5mm²)
    pub laying_method: String, // 敷设方式
    pub pipe_diameter: String, // 穿管管径
    pub is_three_phase: bool,  // 单三相判断
}

impl Default for CableInfo {
    fn default() -> Self {
        Self {
            model: "BV".to_string(),
            voltage_level: 0.4,
            size: "2.5mm²".to_string(),
            laying_method: "SC20".to_string(),
            pipe_diameter: "20".to_string(),
            is_three_phase: false,
        }
    }
}

/// 电气元器件类型枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ElectricComponent {
    Breaker(Breaker),        // 断路器
    Isolator(Isolator),             // 隔离开关
    DualPowerSwitch(DualPowerSwitch), // 双电源开关
    Contactor(Contactor),           // 接触器
    EnergyMeter(EnergyMeter),       // 电能表
}

impl ElectricComponent {
    /// 获取元器件的额定电流(A)
    pub fn get_rated_current(&self) -> f64 {
        match self {
            ElectricComponent::Breaker(breaker) => breaker.rated_current.to_f64(),
            ElectricComponent::Isolator(isolator) => isolator.rated_current.to_f64(),
            ElectricComponent::DualPowerSwitch(dps) => dps.rated_current.to_f64(),
            ElectricComponent::Contactor(contactor) => contactor.rated_current.to_f64(),
            ElectricComponent::EnergyMeter(meter) => meter.rated_current.to_f64(),
        }
    }

    /// 获取元器件的型号
    pub fn get_model(&self) -> &str {
        match self {
            ElectricComponent::Breaker(breaker) => &breaker.model,
            ElectricComponent::Isolator(isolator) => &isolator.model,
            ElectricComponent::DualPowerSwitch(dps) => &dps.model,
            ElectricComponent::Contactor(contactor) => &contactor.model,
            ElectricComponent::EnergyMeter(meter) => &meter.model,
        }
    }

    /// 获取元器件的极数
    pub fn get_poles(&self) -> u32 {
        match self {
            ElectricComponent::Breaker(breaker) => breaker.pole.as_ref().map_or(1, |p| p.to_u32()),
            ElectricComponent::Isolator(isolator) => isolator.pole.to_u32(),
            ElectricComponent::DualPowerSwitch(dps) => dps.pole.to_u32(),
            ElectricComponent::Contactor(contactor) => contactor.pole.to_u32(),
            ElectricComponent::EnergyMeter(meter) => {
                meter.phase_config.as_ref().map_or(1, |pc| match pc {
                    PhaseConfig::SinglePhase => 1,
                    PhaseConfig::ThreePhase => 3,
                    PhaseConfig::ThreePhaseFourWire => 4,
                })
            },
        }
    }
}

/// 回路数据复合类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CircuitData {
    pub circuit_id: String,         // 回路编号
    pub phase_sequence: Phase, // 相序
    pub components: Vec<ElectricComponent>, // 元器件参数列表
    pub cable: CableInfo,           // 线缆选型
    
    // 以下为电气参数
    pub rated_power: f64,           // 额定功率(kW) - 对应pe
    pub power_factor: f64,          // 功率因数 - 对应cos
    pub demand_coefficient: f64,    // 需用系数 - 对应kx
    pub calculated_current: f64,    // 计算电流(A) - 对应ijs
    
    pub circuit_type: String,       // 回路类型(照明/动力/混合)
    pub description: String,        // 回路描述
}

impl CircuitData {
    /// 添加一个元器件到回路
    pub fn add_component(&mut self, component: ElectricComponent) {
        self.components.push(component);
    }
    
    /// 查找回路中的断路器
    pub fn find_breaker(&self) -> Option<&Breaker> {
        self.components.iter().find_map(|comp| match comp {
            ElectricComponent::Breaker(breaker) => Some(breaker),
            _ => None,
        })
    }
    
    pub fn get_circuit_breaker(&self) -> Option<&Breaker> {
        self.find_breaker()
    }
    
    /// 查找回路中的隔离开关
    pub fn find_isolator(&self) -> Option<&Isolator> {
        self.components.iter().find_map(|comp| match comp {
            ElectricComponent::Isolator(isolator) => Some(isolator),
            _ => None,
        })
    }
    
    /// 更新回路中的断路器
    pub fn update_breaker(&mut self, breaker: Breaker) -> bool {
        if let Some(pos) = self.components.iter().position(|comp| matches!(comp, ElectricComponent::Breaker(_))) {
            self.components[pos] = ElectricComponent::Breaker(breaker);
            true
        } else {
            // 如果不存在断路器，则添加一个新的
            self.components.push(ElectricComponent::Breaker(breaker));
            true
        }
    }
    
    /// 添加或替换断路器
    pub fn set_breaker(&mut self, breaker: Breaker) {
        let _ = self.update_breaker(breaker);
    }
    
    /// 删除指定类型的元器件
    pub fn remove_component_of_type(&mut self, component_type: &str) {
        self.components.retain(|comp| {
            let comp_type_str = match comp {
                ElectricComponent::Breaker(_) => "breaker",
                ElectricComponent::Isolator(_) => "isolator",
                ElectricComponent::DualPowerSwitch(_) => "dual_power_switch",
                ElectricComponent::Contactor(_) => "contactor",
                ElectricComponent::EnergyMeter(_) => "energy_meter",
            };
            comp_type_str != component_type.to_lowercase()
        });
    }
}

impl Default for CircuitData {
    fn default() -> Self {
        Self {
            circuit_id: "WL1".to_string(),
            phase_sequence: Phase::L1,
            components: vec![ElectricComponent::Breaker(Breaker::default())],
            cable: CableInfo::default(),
            rated_power: 1.0,
            power_factor: 0.8,
            demand_coefficient: 0.8,
            calculated_current: 0.0,
            circuit_type: "照明".to_string(),
            description: "默认回路".to_string(),
        }
    }
}

/// 电气系统数据类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElectricDataType {
    // 电气参数
    Current,        // 电流(A)
    Power,          // 功率(kW)
    Voltage,        // 电压(V)
    PowerFactor,    // 功率因数
    Coefficient,    // 需用系数
    // 配电系统参数
    CircuitData,    // 回路数据
    DistributionBoxData, // 配电箱数据
    ThreePhaseData, // 三相数据
    // 标识数据
    String,         // 字符串标识
    Integer,        // 整数标识
}

impl ElectricDataType {
    /// 获取数据类型的显示颜色
    #[cfg(feature = "with_egui")]
    pub fn data_type_color(&self) -> egui::Color32 {
        // 根据数据类型返回不同的颜色，用于端口显示
        match self {
            ElectricDataType::Current => egui::Color32::from_rgb(255, 100, 100),      // 红色 - 电流
            ElectricDataType::Power => egui::Color32::from_rgb(100, 200, 100),        // 绿色 - 功率
            ElectricDataType::Voltage => egui::Color32::from_rgb(100, 100, 255),      // 蓝色 - 电压
            ElectricDataType::PowerFactor => egui::Color32::from_rgb(255, 255, 100),  // 黄色 - 功率因数
            ElectricDataType::Coefficient => egui::Color32::from_rgb(255, 100, 255),  // 紫色 - 系数
            ElectricDataType::CircuitData => egui::Color32::from_rgb(200, 150, 100),  // 棕色 - 回路数据
            ElectricDataType::DistributionBoxData => egui::Color32::from_rgb(100, 200, 200), // 青色 - 配电箱数据
            ElectricDataType::ThreePhaseData => egui::Color32::from_rgb(200, 100, 200), // 紫红色 - 三相数据
            ElectricDataType::String => egui::Color32::from_rgb(200, 200, 200),       // 灰色 - 字符串
            ElectricDataType::Integer => egui::Color32::from_rgb(150, 150, 150),       // 深灰色 - 整数
        }
    }

    /// 获取数据类型的显示名称
    pub fn data_type_name(&self) -> &str {
        match self {
            ElectricDataType::Current => "电流(A)",
            ElectricDataType::Power => "功率(kW)",
            ElectricDataType::Voltage => "电压(V)",
            ElectricDataType::PowerFactor => "功率因数",
            ElectricDataType::Coefficient => "需用系数",
            ElectricDataType::CircuitData => "回路数据",
            ElectricDataType::DistributionBoxData => "配电箱数据",
            ElectricDataType::ThreePhaseData => "三相数据",
            ElectricDataType::String => "字符串",
            ElectricDataType::Integer => "整数",
        }
    }
}

// 三相平衡信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhaseBalanceInfo {
    pub phase_a_power: f64,   // A相功率
    pub phase_b_power: f64,   // B相功率
    pub phase_c_power: f64,   // C相功率
    pub balance_degree: f64,  // 不平衡度
}

// 定义电气系统参数值类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ElectricValueType {
    Float(f64),              // 用于电流、功率等数值
    Integer(i64),            // 用于整数标识
    String(String),          // 用于型号、名称等文本
    CircuitData(CircuitData), // 配电回路数据
    DistributionBoxData(HashMap<String, f64>), // 配电箱数据
    ThreePhaseData(PhaseBalanceInfo), // 三相数据
}

impl ElectricValueType {
    /// 获取浮点数值
    pub fn as_float(&self) -> Option<f64> {
        match self {
            ElectricValueType::Float(val) => Some(*val),
            _ => None,
        }
    }
    
    /// 获取整数值
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            ElectricValueType::Integer(val) => Some(*val),
            _ => None,
        }
    }
    
    /// 获取字符串值
    pub fn as_string(&self) -> Option<&str> {
        match self {
            ElectricValueType::String(val) => Some(val),
            _ => None,
        }
    }
    
    /// 获取回路数据
    pub fn as_circuit_data(&self) -> Option<&CircuitData> {
        match self {
            ElectricValueType::CircuitData(data) => Some(data),
            _ => None,
        }
    }
    
    /// 获取配电箱数据
    pub fn as_distribution_box_data(&self) -> Option<&HashMap<String, f64>> {
        match self {
            ElectricValueType::DistributionBoxData(data) => Some(data),
            _ => None,
        }
    }
    
    /// 获取三相数据
    pub fn as_three_phase_data(&self) -> Option<&PhaseBalanceInfo> {
        match self {
            ElectricValueType::ThreePhaseData(data) => Some(data),
            _ => None,
        }
    }
}

// 为了在egui_node_graph中使用，需要实现相应的trait
#[cfg(feature = "with_egui")]
impl egui_node_graph::DataTypeTrait for ElectricDataType {
    fn data_type_color(&self) -> egui::Color32 {
        // 根据数据类型返回不同的颜色，用于端口显示
        match self {
            ElectricDataType::Current => egui::Color32::from_rgb(255, 100, 100),      // 红色 - 电流
            ElectricDataType::Power => egui::Color32::from_rgb(100, 200, 100),        // 绿色 - 功率
            ElectricDataType::Voltage => egui::Color32::from_rgb(100, 100, 255),      // 蓝色 - 电压
            ElectricDataType::PowerFactor => egui::Color32::from_rgb(255, 255, 100),  // 黄色 - 功率因数
            ElectricDataType::Coefficient => egui::Color32::from_rgb(255, 100, 255),  // 紫色 - 系数
            ElectricDataType::CircuitData => egui::Color32::from_rgb(200, 150, 100),  // 棕色 - 回路数据
            ElectricDataType::DistributionBoxData => egui::Color32::from_rgb(100, 200, 200), // 青色 - 配电箱数据
            ElectricDataType::ThreePhaseData => egui::Color32::from_rgb(200, 100, 200), // 紫红色 - 三相数据
            ElectricDataType::String => egui::Color32::from_rgb(200, 200, 200),       // 灰色 - 字符串
            ElectricDataType::Integer => egui::Color32::from_rgb(150, 150, 150),       // 深灰色 - 整数
        }
    }
    
    fn data_type_name(&self) -> &str {
        self.data_type_name()
    }
}

// 实现默认值
impl Default for PhaseBalanceInfo {
    fn default() -> Self {
        Self {
            phase_a_power: 0.0,
            phase_b_power: 0.0,
            phase_c_power: 0.0,
            balance_degree: 0.0,
        }
    }
}