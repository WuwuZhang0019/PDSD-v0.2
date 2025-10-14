/// 回路文字标注库
use serde::{Deserialize, Serialize};

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

/// 相序
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

/// 相线
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhaseLine {
    M1_5,
    M2_5,
    M4,
    M6,
    M10,
    M16,
    M25,
    M35,
    M50,
    M70,
    M95,
    M120,
    M150,
    M185,
    M240,
}
impl PhaseLine {
    // 按规则获取最近的截面值（向上取整）
    pub fn to_str(&self) -> &'static str {
        match self {
            // 小截面（≤16）的PE线与相线规格相同
            PhaseLine::M1_5 => "1.5",
            PhaseLine::M2_5 => "2.5",
            PhaseLine::M4 => "4",
            PhaseLine::M6 => "6",
            PhaseLine::M10 => "10",
            PhaseLine::M16 => "16",

            PhaseLine::M25 => "25",
            PhaseLine::M35 => "35",
            PhaseLine::M50 => "50",
            PhaseLine::M70 => "70",
            PhaseLine::M95 => "95",
            PhaseLine::M120 => "120",
            PhaseLine::M150 => "150",
            PhaseLine::M185 => "185",
            PhaseLine::M240 => "240",
        }
    }
    /// 获取当前Phase对应的PE值（核心映射关系）
    pub fn corresponding_pe(&self) -> PE {
        match self {
            PhaseLine::M1_5 => PE::M1_5,
            PhaseLine::M2_5 => PE::M2_5,
            PhaseLine::M4 => PE::M4,
            PhaseLine::M6 => PE::M6,
            PhaseLine::M10 => PE::M10,
            PhaseLine::M16 => PE::M16,

            PhaseLine::M25 => PE::M16,
            PhaseLine::M35 => PE::M16,
            PhaseLine::M50 => PE::M25,
            PhaseLine::M70 => PE::M35,
            PhaseLine::M95 => PE::M50,
            PhaseLine::M120 => PE::M70,
            PhaseLine::M150 => PE::M70,
            PhaseLine::M185 => PE::M95,
            PhaseLine::M240 => PE::M120,
        }
    }
}

/// 双拼电缆
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TwinParallelCables {
    M2x70,
    M2x95,
    M2x120,
    M2x150,
    M2x185,
    M2x240,
}
impl TwinParallelCables {
    // 按规则获取最近的截面值（向上取整）
    pub fn to_str(&self) -> &'static str {
        match self {
            // 双拼类型PE线规则
            TwinParallelCables::M2x70 => "70",
            TwinParallelCables::M2x95 => "95",
            TwinParallelCables::M2x120 => "120",
            TwinParallelCables::M2x150 => "150",
            TwinParallelCables::M2x185 => "185",
            TwinParallelCables::M2x240 => "240",
        }
    }
    /// 获取当前Phase对应的PE值（核心映射关系）
    pub fn corresponding_pe(&self) -> PE {
        match self {
            TwinParallelCables::M2x70 => PE::M35,
            TwinParallelCables::M2x95 => PE::M50,
            TwinParallelCables::M2x120 => PE::M70,
            TwinParallelCables::M2x150 => PE::M70,
            TwinParallelCables::M2x185 => PE::M95,
            TwinParallelCables::M2x240 => PE::M120,
        }
    }
    ///标识是否为双拼电缆
    pub fn is_twin(&self) -> bool {
        true
    }
}

///PE线
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PE {
    M1_5,
    M2_5,
    M4,
    M6,
    M10,
    M16,
    M25,
    M35,
    M50,
    M70,
    M95,
    M120,
}
impl PE {
    // 按规则获取最近的截面值（向上取整）
    pub fn to_str(&self) -> &'static str {
        match self {
            // 双拼类型PE线规则
            PE::M1_5 => "1.5",
            PE::M2_5 => "2.5",
            PE::M4 => "4",
            PE::M6 => "6",
            PE::M10 => "10",
            PE::M16 => "16",
            PE::M25 => "25",
            PE::M35 => "35",
            PE::M50 => "50",
            PE::M70 => "70",
            PE::M95 => "95",
            PE::M120 => "120",
        }
    }
}

/// 电缆芯数
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoresNum {
    C2_1(PhaseLine, PE),
    C4_1(PhaseLine, Option<TwinParallelCables>, PE),
    C4(PhaseLine, Option<TwinParallelCables>),
}
impl CoresNum {
    /// 创建C2_1
    pub fn new_c2_1(phase: PhaseLine, pe: PE) -> Result<Self, &'static str> {
        // 验证：PE必须是Phase对应的预设值
        if pe != phase.corresponding_pe() {
            return Err("PE值与Phase不匹配，请使用对应的PE值");
        }
        Ok(Self::C2_1(phase, pe))
    }

    /// 创建普通C4_1
    pub fn new_c4_1(phase: PhaseLine, pe: PE) -> Result<Self, &'static str> {
        if pe != phase.corresponding_pe() {
            return Err("PE值与相线不匹配");
        }
        Ok(Self::C4_1(phase, None, pe))
    }
    /// 创建双拼普通C4_1
    pub fn new_c4_1_twin(
        phase: PhaseLine,
        twin: TwinParallelCables,
        pe: PE,
    ) -> Result<Self, &'static str> {
        if pe != twin.corresponding_pe() {
            return Err("PE值与双拼电缆相线不匹配，请使用对应的PE值");
        }
        Ok(Self::C4_1(phase, Some(twin), pe))
    }
    /// C4创建逻辑
    pub fn new_c4(phase: PhaseLine, twin: TwinParallelCables) -> Self {
        Self::C4(phase, Some(twin))
    }

    /// 生成规格字符串
    pub fn to_spec_str(&self) -> String {
        match self {
            CoresNum::C2_1(phase, pe) => {
                format!("2x{}+PE{}", phase.to_str(), pe.to_str())
            }
            CoresNum::C4_1(phase, twin, pe) => match twin {
                None => format!("4x{}+PE{}", phase.to_str(), pe.to_str()),
                Some(twin_cable) => {
                    let inner_str = format!("4x{}+PE{}", twin_cable.to_str(), pe.to_str());
                    format!("2x({})", inner_str)
                }
            },
            CoresNum::C4(phase, twin) => match twin {
                None => format!("4x{}", phase.to_str()),
                Some(twin_cable) => {
                    format!("2x(4x{})", twin_cable.to_str())
                }
            },
        }
    }
}

/// 回路编号类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum CircuitNumber {
    // 类型1：字母+数字（如"N1, N2..."）
    LetterNumber {
        letter: char,        // 字母（如'N'、'P'）
        current_number: u32, // 当前数字（从1开始）
    },
    //类型2： 双字母+数字（如"WL1, WX2..."）
    TwoLettersNumber {
        prefix: &'static str, // 固定双字母前缀（如"WL"、"WX"）
        current_number: u32,
    },
    // 类型3：配电箱编号+'-'+数字（如"1AL-1, 1AL-2..."）
    DistributionBox {
        box_id: String,      // 配电箱编号（如"1AL"、"2AP"）
        current_number: u32, // 当前数字（从1开始）
    },
}
impl CircuitNumber {
    // 创建单字母类型
    pub fn new_single_letter(letter: char) -> Self {
        Self::LetterNumber {
            letter,
            current_number: 1,
        }
    }
    // 创建双字母类型
    pub fn new_two_letters(prefix: &'static str) -> Self {
        // 简单校验：确保前缀是2个字符（可选）
        debug_assert_eq!(prefix.len(), 2, "双字母前缀必须是2个字符");
        Self::TwoLettersNumber {
            prefix,
            current_number: 1,
        }
    }

    // 新增：创建配电箱初始编号（关联函数，接收外部box_id）
    pub fn new_box_initial(box_id: &str) -> Self {
        Self::DistributionBox {
            box_id: box_id.to_string(),
            current_number: 1, // 初始编号从1开始
        }
    }
    // 创建配电箱类型
    pub fn new_box(&self) -> Self {
        match self {
            Self::DistributionBox {
                box_id,
                current_number,
            } => CircuitNumber::DistributionBox {
                box_id: box_id.clone(),
                current_number: current_number + 1,
            },
            _ => todo!("仅配电箱类型可调用new_box方法"),
        }
    }

    // 生成当前编号字符串
    pub fn to_string(&self) -> String {
        match self {
            Self::LetterNumber {
                letter,
                current_number,
            } => {
                format!("{}{}", letter, current_number)
            }
            Self::TwoLettersNumber {
                prefix,
                current_number,
            } => {
                format!("{}{}", prefix, current_number)
            }
            CircuitNumber::DistributionBox {
                box_id,
                current_number,
            } => {
                format!("{}-{}", box_id, current_number)
            }
        }
    }

    // 生成下一个编号
    pub fn next(&self) -> Self {
        match self {
            Self::LetterNumber {
                letter,
                current_number,
            } => Self::LetterNumber {
                letter: *letter,
                current_number: current_number + 1,
            },
            Self::TwoLettersNumber {
                prefix,
                current_number,
            } => Self::TwoLettersNumber {
                prefix,
                current_number: current_number + 1,
            },
            Self::DistributionBox {
                box_id,
                current_number,
            } => Self::DistributionBox {
                box_id: box_id.clone(),
                current_number: current_number + 1,
            },
        }
    }
}

/// 回路用途,支持手动输入或从外部参数获取
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CircuitPurpose {
    // 手动输入的用途
    Manual(String),
    // 从外部参数获取的用途（未来关联到具体参数源）
    External {
        source_id: String, // 外部参数的唯一标识（如设备ID、配置项ID）
        value: String,     // 用途值（从外部获取后存储）
    },
}
impl CircuitPurpose {
    /// 创建手动输入的回路用途
    pub fn from_manual(input: &str) -> Self {
        Self::Manual(input.to_string())
    }
    /// 创建从外部参数获取的回路用途
    /// source_id: 外部参数的标识（如"device_123"）
    /// value: 从外部获取的用途值
    pub fn from_external(source_id: &str, value: &str) -> Self {
        Self::External {
            source_id: source_id.to_string(),
            value: value.to_string(),
        }
    }
    /// 获取用途的字符串表示（统一接口）
    pub fn to_string(&self) -> &str {
        match self {
            Self::Manual(s) => s,
            Self::External { value, .. } => value,
        }
    }
    /// 更新用途值（支持两种类型的更新）
    pub fn update(&mut self, new_value: &str) {
        match self {
            Self::Manual(s) => *s = new_value.to_string(),
            Self::External { value, .. } => *value = new_value.to_string(),
        }
    }
    /// 如果是外部来源，返回其标识（用于追溯来源）
    pub fn external_source(&self) -> Option<&str> {
        match self {
            Self::External { source_id, .. } => Some(source_id),
            _ => None,
        }
    }
}

/// 回路功率：支持手动输入或从外部配电箱参数获取
#[derive(Debug, Clone, PartialEq)]
pub enum CircuitPower {
    // 手动输入的功率（仅需数值，自动添加kW单位）
    Manual {
        value: f32,  // 功率数值（如10.5表示10.5kW）
    },
    // 从外部配电箱参数获取的功率
    External {
        source_id: String,  // 外部配电箱标识
        value: f32,         // 功率数值（自动以kW为单位）
    },
}
impl CircuitPower {
    /// 创建手动输入的回路功率（仅需数值，自动添加kW单位）
    pub fn from_manual(input: &str) -> Result<Self, &'static str> {
        // 尝试将输入解析为数值
        let value = input.parse().map_err(|_| {
            "无效的功率数值，请输入数字（如10、7.5）"
        })?;

        // 验证数值有效性（功率不能为负）
        if value < 0.0 {
            return Err("功率数值不能为负数");
        }

        Ok(Self::Manual { value })
    }

    /// 创建从外部配电箱获取的回路功率（自动以kW为单位）
    pub fn from_external(source_id: &str, value: f32) -> Result<Self, &'static str> {
        if value < 0.0 {
            return Err("外部传入的功率不能为负数");
        }

        Ok(Self::External {
            source_id: source_id.to_string(),
            value,
        })
    }

    /// 获取带单位的功率字符串（自动添加kW）
    pub fn to_string(&self) -> String {
        let value = match self {
            Self::Manual { value } => *value,
            Self::External { value, .. } => *value,
        };

        // 整数数值显示为整数形式（如10 -> "10kW"），否则保留小数（如7.5 -> "7.5kW"）
        if value.fract() == 0.0 {
            format!("{}kW", value as u32)
        } else {
            format!("{}kW", value)
        }
    }

    /// 更新功率数值（保持kW单位）
    pub fn update(&mut self, new_value: &str) -> Result<(), &'static str> {
        let new_value = new_value.parse().map_err(|_| {
            "无效的更新数值，请输入数字（如15、12.3）"
        })?;

        if new_value < 0.0 {
            return Err("更新的功率不能为负数");
        }

        match self {
            Self::Manual { value } => *value = new_value,
            Self::External { value, .. } => *value = new_value,
        }

        Ok(())
    }

    /// 获取原始数值（不带单位）
    pub fn get_value(&self) -> f32 {
        match self {
            Self::Manual { value } => *value,
            Self::External { value, .. } => *value,
        }
    }

    /// 获取外部来源标识（如果是外部获取的）
    pub fn external_source(&self) -> Option<&str> {
        match self {
            Self::External { source_id, .. } => Some(source_id),
            _ => None,
        }
    }
}

// 测试用例
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c4_1_normal() {
        // 普通电缆（无双拼）
        let c4_1 = CoresNum::new_c4_1(PhaseLine::M16, PE::M16).unwrap();
        assert_eq!(c4_1.to_spec_str(), "4x16+PE16");

        let c4_1 = CoresNum::new_c4_1(PhaseLine::M25, PE::M16).unwrap();
        assert_eq!(c4_1.to_spec_str(), "4x25+PE16");
    }

    #[test]
    fn test_c4_1_twin() {
        // 双拼电缆
        let c4_1 = CoresNum::new_c4_1_twin(PhaseLine::M70, TwinParallelCables::M2x70, PE::M35).unwrap();
        assert_eq!(c4_1.to_spec_str(), "2x(4x70+PE35)");

        let c4_1 =
            CoresNum::new_c4_1_twin(PhaseLine::M120, TwinParallelCables::M2x120, PE::M70).unwrap();
        assert_eq!(c4_1.to_spec_str(), "2x(4x120+PE70)");
    }
    #[test]
    fn test_two_letters_case() {
        let mut circuit = CircuitNumber::new_two_letters("WL");
        assert_eq!(circuit.to_string(), "WL1");

        circuit = circuit.next();
        assert_eq!(circuit.to_string(), "WL2");

        let mut circuit = CircuitNumber::new_two_letters("WX");
        assert_eq!(circuit.to_string(), "WX1");
    }

    #[test]
    fn test_mixed_cases() {
        // 单字母类型
        let single = CircuitNumber::new_single_letter('N');
        assert_eq!(single.to_string(), "N1");

        // 双字母类型
        let two_letters = CircuitNumber::new_two_letters("PE");
        assert_eq!(two_letters.to_string(), "PE1");

        // 配电箱类型测试（模拟外部传入box_id）
        let box_id_from_external = "1AL"; // 模拟从外部获取的box_id

        // 1. 用新增的关联函数创建初始编号
        let mut box_type = CircuitNumber::new_box_initial(box_id_from_external);
        assert_eq!(box_type.to_string(), "1AL-1");

        // 2. 用保留的new_box方法自增（依赖实例self）
        box_type = box_type.new_box();
        assert_eq!(box_type.to_string(), "1AL-2");

        // 再次自增
        box_type = box_type.new_box();
        assert_eq!(box_type.to_string(), "1AL-3");
    }



    #[test]
    fn test_manual_purpose() {
        // 手动输入场景
        let mut purpose = CircuitPurpose::from_manual("照明回路");
        assert_eq!(purpose.to_string(), "照明回路");

        // 更新手动输入的值
        purpose.update("应急照明回路");
        assert_eq!(purpose.to_string(), "应急照明回路");

        // 确认不是外部来源
        assert_eq!(purpose.external_source(), None);
    }

    #[test]
    fn test_external_purpose() {
        // 模拟从外部参数获取（如从设备配置中读取）
        let mut purpose = CircuitPurpose::from_external(
            "device_456",  // 外部设备ID（未来实际参数源）
            "动力回路"     // 从外部获取的用途值
        );

        // 验证用途值
        assert_eq!(purpose.to_string(), "动力回路");
        // 验证外部来源标识
        assert_eq!(purpose.external_source(), Some("device_456"));

        // 更新外部来源的用途值（例如外部参数变化时）
        purpose.update("消防动力回路");
        assert_eq!(purpose.to_string(), "消防动力回路");
    }

    // 模拟未来外部参数系统的调用场景
    #[test]
    fn test_future_external_integration() {
        // 模拟未来的外部参数系统（目前仅返回测试数据）
        struct FutureExternalSystem;
        impl FutureExternalSystem {
            // 模拟从外部系统查询参数
            fn get_circuit_purpose(&self, device_id: &str) -> Option<String> {
                // 实际项目中这里会连接数据库或配置系统
                match device_id {
                    "panel_789" => Some("空调回路".to_string()),
                    _ => None,
                }
            }
        }

        // 从未来系统获取参数并创建回路用途
        let external_system = FutureExternalSystem;
        let device_id = "panel_789";
        if let Some(purpose_value) = external_system.get_circuit_purpose(device_id) {
            let purpose = CircuitPurpose::from_external(device_id, &purpose_value);
            assert_eq!(purpose.to_string(), "空调回路");
            assert_eq!(purpose.external_source(), Some(device_id));
        }
    }

    #[test]
    #[test]
    fn test_manual_power() {
        // 有效手动输入：只传数值（不带单位）
        let mut power = CircuitPower::from_manual("10").unwrap();  // 正确：仅数值
        assert_eq!(power.to_string(), "10kW");  // 系统自动加单位

        // 更新手动输入：同样只传数值
        assert!(power.update("15").is_ok());  // 正确：仅数值
        assert_eq!(power.to_string(), "15kW");

        // 无效格式验证（修正逻辑）
        assert!(CircuitPower::from_manual("abc").is_err());  // 非数字：正确
        assert!(CircuitPower::from_manual("10kW").is_err());  // 带单位：错误（符合设计）
        assert!(CircuitPower::from_manual("").is_err());      // 空值：正确
        assert!(CircuitPower::from_manual("-5").is_err());    // 负数：正确
    }

    #[test]
    fn test_external_power() {
        // 模拟从外部配电箱获取功率
        let mut power = CircuitPower::from_external("panel_101", 7.5).unwrap();

        // 验证功率值和来源
        assert_eq!(power.to_string(), "7.5kW");
        assert_eq!(power.external_source(), Some("panel_101"));

        // 更新外部功率
        assert!(power.update("12").is_ok());
        assert_eq!(power.to_string(), "12kW");

        // 无效的外部功率
        assert!(CircuitPower::from_external("panel_102", -2314.0).is_err());
    }

    // 模拟未来与配电箱系统的集成
    #[test]
    fn test_manual_power_with_default_unit() {
        // 整数数值
        let power = CircuitPower::from_manual("10").unwrap();
        assert_eq!(power.to_string(), "10kW");
        assert_eq!(power.get_value(), 10.0);

        // 小数数值
        let power = CircuitPower::from_manual("7.5").unwrap();
        assert_eq!(power.to_string(), "7.5kW");

        // 更新数值
        let mut power = CircuitPower::from_manual("5").unwrap();
        assert!(power.update("8.2").is_ok());
        assert_eq!(power.to_string(), "8.2kW");

        // 无效输入验证
        assert!(CircuitPower::from_manual("abc").is_err());  // 非数字
        assert!(CircuitPower::from_manual("-3").is_err());   // 负数
    }

    #[test]
    fn test_external_power_with_default_unit() {
        // 从外部获取功率
        let power = CircuitPower::from_external("panel_301", 15.0).unwrap();
        assert_eq!(power.to_string(), "15kW");
        assert_eq!(power.get_value(), 15.0);
        assert_eq!(power.external_source(), Some("panel_301"));

        // 外部传入小数
        let power = CircuitPower::from_external("panel_302", 22.5).unwrap();
        assert_eq!(power.to_string(), "22.5kW");

        // 更新外部功率
        let mut power = CircuitPower::from_external("panel_303", 10.0).unwrap();
        assert!(power.update("18").is_ok());
        assert_eq!(power.to_string(), "18kW");
    }

    // 模拟未来与配电箱系统集成
    #[test]
    fn test_future_panel_integration() {
        // 模拟未来的配电箱系统，返回数值（单位默认kW）
        struct FutureDistributionPanel {
            id: String,
        }

        impl FutureDistributionPanel {
            // 从配电箱参数获取回路功率（返回数值，默认单位kW）
            fn get_circuit_power(&self, circuit_id: u32) -> Option<f32> {
                // 实际场景中，这里会从配电箱的参数系统读取
                match (self.id.as_str(), circuit_id) {
                    ("panel_400", 1) => Some(20.0),   // 20kW
                    ("panel_400", 2) => Some(12.5),   // 12.5kW
                    _ => None,
                }
            }
        }

        // 从未来系统获取功率并创建实例
        let panel = FutureDistributionPanel {
            id: "panel_400".to_string(),
        };

        if let Some(power_value) = panel.get_circuit_power(1) {
            let circuit_power = CircuitPower::from_external(&panel.id, power_value).unwrap();
            assert_eq!(circuit_power.to_string(), "20kW");  // 自动添加单位
        }
    }
}
