# PDSD 项目开发指南

本文档帮助开发者构建自己的节点图应用。

## 1. 准备工作与规划阶段

### 1.1 明确项目需求

在开始开发之前，首先需要明确以下几点：

- 你的节点图应用将解决什么问题？
  - 建筑电气设计过程中，配电系统图的设计简化问题。
- 需要支持哪些类型的节点和数据？
  - 节点类型：
    - 电源节点
    - 负载节点
    - 开关节点
    - 连接节点
  - 数据类型：
    - 电压、电流、功率等电气参数
    - 节点位置、连接关系等
- 用户交互流程是什么样的？
  - 用户通过点击节点添加到图中
  - 用户通过拖动节点连接不同节点
  - 节点类型：
    - 电源节点
    - 负载节点
    - 开关节点
    - 连接节点
  - 数据类型：
    - 电压、电流、功率等电气参数
    - 节点位置、连接关系等
- 用户交互流程是什么样的？
  - 用户通过点击节点添加到图中
  - 用户通过拖动节点连接不同节点
  - 用户可以编辑节点属性（如电压、电流等）
- 是否需要状态持久化功能？
  - 是，需要保存用户创建的节点图和相关参数。

### 1.2 环境准备

1. 确保已安装 Rust 开发环境（建议使用 rustup 管理）
2. 熟悉 egui 和 eframe 的基本概念（如果计划创建桌面应用）
3. 克隆或下载 egui_node_graph 仓库作为参考

### 1.3 项目结构设计

创建新的 Rust 项目并设计基本结构：

```bash
cargo new PDSD
cd PDSD
```

对于电气系统节点图应用，建议采用以下文件夹结构：

```
PDSD/
├── Cargo.toml                 # 项目依赖配置
├── Cargo.lock                 # 依赖版本锁定文件
├── src/                       # 源代码目录
│   ├── main.rs                # 程序入口点
│   ├── config.rs              # 应用配置管理
│   ├── core_lib/              # 核心库模块
│   │   ├── mod.rs             # 模块导出
│   │   ├── data_types/        # 数据类型定义
│   │   │   ├── mod.rs
│   │   │   ├── electric_data.rs     # 电气数据类型
│   │   │   └── node_data.rs         # 节点数据结构
│   │   ├── traits/            # 核心trait定义
│   │   │   ├── mod.rs
│   │   │   └── calculation.rs       # 计算相关trait
│   │   ├── algorithm/         # 算法实现
│   │   │   ├── mod.rs
│   │   │   ├── graph_algorithm.rs   # 图算法
│   │   │   └── electrical_calculation.rs  # 电气计算算法
│   │   └── utils/             # 工具函数
│   │       ├── mod.rs
│   │       └── conversion.rs         # 单位转换等
│   ├── application/           # 应用层
│   │   ├── mod.rs
│   │   ├── app.rs             # 应用主逻辑
│   │   └── state.rs           # 应用状态管理
│   ├── editor/                # 编辑器实现
│   │   ├── mod.rs
│   │   ├── ui/                # UI组件
│   │   │   ├── mod.rs
│   │   │   ├── node_editor.rs       # 节点编辑器
│   │   │   └── property_panel.rs    # 属性面板
│   │   ├── business/          # 业务逻辑
│   │   │   ├── mod.rs
│   │   │   ├── node_templates.rs    # 节点模板
│   │   │   └── validation.rs        # 验证逻辑
│   │   └── graph/             # 图管理
│   │       ├── mod.rs
│   │       ├── graph_manager.rs     # 图管理器
│   │       └── serialization.rs     # 序列化功能
│   └── export/                # 导出功能
│       ├── mod.rs
│       ├── report.rs          # 报告生成
│       └── cad_export.rs      # CAD导出
├── tests/                     # 测试目录
│   ├── integration/           # 集成测试
│   └── calculation_tests/     # 计算相关测试
├── assets/                    # 资源文件
│   ├── icons/                 # 图标
│   ├── fonts/                 # 字体
│   └── templates/             # 模板文件
└── docs/                      # 文档
    ├── api/                   # API文档
    └── user_guide/            # 用户指南
```

这种结构设计遵循了关注点分离原则，将核心数据类型、业务逻辑、UI 表现和外部接口清晰分开，便于团队协作和代码维护。

## 2. 核心数据类型定义

### 2.1 设计数据类型枚举（DataType）

在建筑电气配电系统中，我们需要定义与电气参数相关的数据类型。基于本项目的业务逻辑，以下是核心数据类型的实现：

```rust
use egui_node_graph::{DataTypeTrait, NodeTemplateTrait, CategoryTrait, NodeResponse};
use egui::{Context, Ui, Color32, WidgetText};
use eframe::{App, Frame};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::{Read, Write};

// 定义电气系统数据类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ElectricDataType {
    // 电气参数    
    Power,          // 功率(kW) - 对应代码中的pe参数
    Voltage,        // 电压(V) - 对应代码中的U参数
    Cos,            // 余弦值 - 对应代码中的cos参数(功率因数)
    Kx,             // 需要系数 - 对应代码中的kx参数
    CalculatedCurrent, // 计算电流 - 对应代码中的ijs参数
    // 节点数据参数
    CircuitData,    // 回路数据
    DistributionBoxData, // 配电箱数据

    // 标识数据
    String,         // 字符串标识
    Integer,        // 整数标识
}

// 定义电气系统值类型
#[derive(Debug, Clone, PartialEq)]
enum ElectricValueType {
    // 基本数值类型
    Float(f64),
    Integer(i64),
    String(String),

    // 电气系统特定类型
    CircuitData(HashMap<String, f64>),    // 回路参数集合
    DistributionBoxData(HashMap<String, f64>), // 配电箱参数集合
    ThreePhaseData(HashMap<String, f64>), // 三相参数集合
}

// 实现适当的显示和操作方法
impl ElectricValueType {
    // 获取浮点数值
    fn as_float(&self) -> Option<f64> {
        match self {
            ElectricValueType::Float(val) => Some(*val),
            _ => None,
        }
    }

    // 获取整数值
    fn as_integer(&self) -> Option<i64> {
        match self {
            ElectricValueType::Integer(val) => Some(*val),
            _ => None,
        }
    }

    // 获取字符串值
    fn as_string(&self) -> Option<&str> {
        match self {
            ElectricValueType::String(val) => Some(val),
            _ => None,
        }
    }

    // 获取回路数据
    fn as_circuit_data(&self) -> Option<&HashMap<String, f64>> {
        match self {
            ElectricValueType::CircuitData(data) => Some(data),
            _ => None,
        }
    }
}

// 实现节点数据结构
#[derive(Debug, Clone)]
enum ElectricNodeData {
    // 配电回路节点
    CircuitNode(CircuitNodeData),
    // 配电箱节点
    DistributionBoxNode(DistributionBoxNodeData),
    // 干线系统图节点
    TrunkLineNode(TrunkLineNodeData),
    // 电源节点
    PowerSourceNode(PowerSourceNodeData),
    // 计算节点
    CalculationNode(CalculationNodeData),
}

    fn data_type_name(&self) -> &str {
        // 提供数据类型的显示名称
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


// 定义各个节点的数据结构
#[derive(Debug, Clone)]
struct CircuitNodeData {
    // 回路基本信息
    name: String,
    description: String,

    // 电气参数
    rated_power: f64,         // 额定功率(kW)
    power_factor: f64,        // 功率因数
    demand_coefficient: f64,  // 需用系数
    current: f64,             // 计算电流(A)

    // 回路类型和相数
    circuit_type: String,     // 照明/动力/混合
    phase_type: String,       // 单相/三相

    // 保护参数
    protection_current: f64,  // 保护电流(A)
    wire_size: String,        // 导线规格
}
// 配电箱节点数据结构
#[derive(Debug, Clone)]
struct DistributionBoxNodeData {
    // 配电箱基本信息
    name: String,
    description: String,
    box_type: String,         // 配电箱类型

    // 电气参数
    rated_voltage: f64,       // 额定电压(V)
    rated_current: f64,       // 额定电流(A)
    total_power: f64,         // 总功率(kW)

    // 三相负载分布
    phase_a_load: f64,        // A相负载(kW)
    phase_b_load: f64,        // B相负载(kW)
    phase_c_load: f64,        // C相负载(kW)

    // 进线参数
    incoming_current: f64,    // 进线电流(A)
    incoming_wire_size: String, // 进线规格
}

// 干线系统图节点数据结构
#[derive(Debug, Clone)]
struct TrunkLineNodeData {
    // 干线基本信息
    name: String,
    description: String,
    line_type: String,        // 干线类型

    // 电气参数
    length: f64,              // 长度(m)
    resistance: f64,          // 电阻(Ω)
    reactance: f64,           // 电抗(Ω)
    voltage_drop: f64,        // 电压降(V)

    // 负荷参数
    total_current: f64,       // 总电流(A)
    wire_size: String,        // 导线规格

    // 三相参数
    phase_a_current: f64,     // A相电流(A)
    phase_b_current: f64,     // B相电流(A)
    phase_c_current: f64,     // C相电流(A)
}

// 电源节点数据结构
#[derive(Debug, Clone)]
struct PowerSourceNodeData {
    // 电源基本信息
    name: String,
    description: String,
    source_type: String,      // 电源类型

    // 电气参数
    voltage: f64,             // 电压(V)
    frequency: f64,           // 频率(Hz)
    capacity: f64,            // 容量(kVA)

    // 相数信息
    phase_count: u32,         // 相数(1/3)

    // 效率参数
    efficiency: f64,          // 效率
}

// 计算节点数据结构
#[derive(Debug, Clone)]
struct CalculationNodeData {
    // 计算节点基本信息
    name: String,
    calculation_type: String, // 计算类型

    // 计算参数
    result: f64,              // 计算结果
    precision: u32,           // 计算精度

    // 特定计算类型参数
    phase_balance_degree: f64, // 三相平衡度(%)
    voltage_loss_percent: f64, // 电压损失率(%)
}

    ((max_phase - min_phase) / max_phase) * 100.0
}

// 项目数据结构
#[derive(Serialize, Deserialize)]
struct ElectricProjectData<'a> {
    project_name: String,
    project_id: uuid::Uuid,
    graph: &'a egui_node_graph::Graph<ElectricNodeData, ElectricDataType, ElectricValueType>,
    calculation_cache: std::collections::HashMap<String, f64>,
    version: String,
    save_date: chrono::DateTime<chrono::Utc>,
}

// 报告生成器
struct ElectricSystemReport {
    project_name: String,
    project_id: uuid::Uuid,
    // 其他报告相关字段
}

impl ElectricSystemReport {
    fn new(project_name: &str, project_id: uuid::Uuid) -> Self {
        Self {
            project_name: project_name.to_string(),
            project_id,
        }
    }

    fn add_system_overview(&mut self, graph: &egui_node_graph::Graph<ElectricNodeData, ElectricDataType, ElectricValueType>) {
        // 添加系统概览部分到报告
    }

    fn add_nodes_details(&mut self, graph: &egui_node_graph::Graph<ElectricNodeData, ElectricDataType, ElectricValueType>) {
        // 添加节点详细信息到报告
    }

    fn add_calculation_results(&mut self, cache: &std::collections::HashMap<String, f64>) {
        // 添加计算结果到报告
    }

    fn add_phase_balance_analysis(&mut self, graph: &egui_node_graph::Graph<ElectricNodeData, ElectricDataType, ElectricValueType>) {
        // 添加三相平衡分析到报告
    }

    fn add_recommendations(&mut self, graph: &egui_node_graph::Graph<ElectricNodeData, ElectricDataType, ElectricValueType>,
                          cache: &std::collections::HashMap<String, f64>) {
        // 添加优化建议到报告
    }

    fn generate_pdf(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 生成PDF报告
        // 这里需要使用PDF生成库如printpdf或rust-pdf
        Ok(())
    }
}
```

### 2.2 设计参数值类型（ValueType）

在电气配电系统中，我们需要定义与电气参数对应的具体值类型，包括电流、功率等基本参数，以及更复杂的配电回路和配电箱数据：

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 配电回路数据结构
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct CircuitInfo {
    pub id: String,
    pub name: String,
    pub power: f64,           // 功率(kW)
    pub power_factor: f64,    // 功率因数
    pub coefficient: f64,     // 需用系数
    pub current: f64,         // 计算电流(A)
    pub phase: String,        // 相别 (A/B/C)
    pub protection_device: String,  // 保护设备型号
    pub cable_type: String,   // 线缆型号
}

// 配电箱数据结构
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct DistributionBoxInfo {
    pub id: String,
    pub name: String,
    pub incoming_power: f64,  // 进线功率(kW)
    pub incoming_current: f64, // 进线电流(A)
    pub total_circuits: usize,// 回路总数
    pub phase_balance: PhaseBalanceInfo, // 三相平衡信息
    pub main_protection: String, // 主保护设备
    pub monitoring_module: String, // 监测模块
}

// 三相平衡信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct PhaseBalanceInfo {
    pub phase_a_power: f64,   // A相功率
    pub phase_b_power: f64,   // B相功率
    pub phase_c_power: f64,   // C相功率
    pub balance_degree: f64,  // 不平衡度
}

// 定义电气系统参数值类型
#[derive(Debug, Clone, PartialEq)]
enum ElectricValueType {
    Float(f64),              // 用于电流、功率等数值
    Integer(i64),            // 用于整数标识
    String(String),          // 用于型号、名称等文本
    CircuitData(CircuitInfo), // 配电回路数据
    DistributionBoxData(DistributionBoxInfo), // 配电箱数据
    ThreePhaseData(PhaseBalanceInfo), // 三相数据
}

// 如果需要序列化支持，添加相应实现
// 序列化：将内存中的数据结构转换为可存储或传输的格式（如JSON字符串）
// 反序列化：将存储或传输的格式转换回内存中的数据结构
use serde::{Serialize, Deserialize, Serializer, Deserializer};

impl Serialize for MyAppValueType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeMap;

        // 创建一个包含类型和值的映射
        let mut map = serializer.serialize_map(Some(2))?;

        match self {
            MyAppValueType::Float(val) => {
                map.serialize_entry("type", "float")?;
                map.serialize_entry("value", val)?;
            }
            MyAppValueType::Integer(val) => {
                map.serialize_entry("type", "integer")?;
                map.serialize_entry("value", val)?;
            }
            MyAppValueType::Boolean(val) => {
                map.serialize_entry("type", "boolean")?;
                map.serialize_entry("value", val)?;
            }
            MyAppValueType::Vector2(val) => {
                map.serialize_entry("type", "vector2")?;
                map.serialize_entry("value", &[val.x, val.y])?;
            }
            MyAppValueType::Vector3(val) => {
                map.serialize_entry("type", "vector3")?;
                map.serialize_entry("value", &[val.x, val.y, val.z])?;
            }
            MyAppValueType::Color(val) => {
                map.serialize_entry("type", "color")?;
                map.serialize_entry("value", &[val.r(), val.g(), val.b(), val.a()])?;
            }
            MyAppValueType::String(val) => {
                map.serialize_entry("type", "string")?;
                map.serialize_entry("value", val)?;
            }
        }

        map.end()
    }
}

impl<'de> Deserialize<'de> for MyAppValueType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Type,
            Value,
        }

        struct MyAppValueTypeVisitor;

        impl<'de> Visitor<'de> for MyAppValueTypeVisitor {
            type Value = MyAppValueType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct MyAppValueType")
            }

            fn visit_map<V>(self, mut map: V) -> Result<MyAppValueType, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut value_type = None;
                let mut value = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Type => {
                            if value_type.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            value_type = Some(map.next_value::<String>()?);
                        }
                        Field::Value => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value::<serde_json::Value>()?);
                        }
                    }
                }

                let value_type = value_type.ok_or_else(|| de::Error::missing_field("type"))?;
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;

                match value_type.as_str() {
                    "float" => {
                        let val = value.as_f64()
                            .ok_or_else(|| de::Error::custom("invalid float value"))?;
                        Ok(MyAppValueType::Float(val))
                    }
                    "integer" => {
                        let val = value.as_i64()
                            .ok_or_else(|| de::Error::custom("invalid integer value"))?;
                        Ok(MyAppValueType::Integer(val))
                    }
                    "boolean" => {
                        let val = value.as_bool()
                            .ok_or_else(|| de::Error::custom("invalid boolean value"))?;
                        Ok(MyAppValueType::Boolean(val))
                    }
                    "vector2" => {
                        let arr = value.as_array()
                            .ok_or_else(|| de::Error::custom("invalid vector2 value"))?;
                        if arr.len() != 2 {
                            return Err(de::Error::custom("vector2 must have 2 components"));
                        }
                        let x = arr[0].as_f64().ok_or_else(|| de::Error::custom("invalid x"))? as f32;
                        let y = arr[1].as_f64().ok_or_else(|| de::Error::custom("invalid y"))? as f32;
                        Ok(MyAppValueType::Vector2(egui::Vec2::new(x, y)))
                    }
                    "vector3" => {
                        let arr = value.as_array()
                            .ok_or_else(|| de::Error::custom("invalid vector3 value"))?;
                        if arr.len() != 3 {
                            return Err(de::Error::custom("vector3 must have 3 components"));
                        }
                        let x = arr[0].as_f64().ok_or_else(|| de::Error::custom("invalid x"))? as f32;
                        let y = arr[1].as_f64().ok_or_else(|| de::Error::custom("invalid y"))? as f32;
                        let z = arr[2].as_f64().ok_or_else(|| de::Error::custom("invalid z"))? as f32;
                        Ok(MyAppValueType::Vector3(egui::Vec3::new(x, y, z)))
                    }
                    "color" => {
                        let arr = value.as_array()
                            .ok_or_else(|| de::Error::custom("invalid color value"))?;
                        if arr.len() != 4 {
                            return Err(de::Error::custom("color must have 4 components"));
                        }
                        let r = arr[0].as_u64().ok_or_else(|| de::Error::custom("invalid r"))? as u8;
                        let g = arr[1].as_u64().ok_or_else(|| de::Error::custom("invalid g"))? as u8;
                        let b = arr[2].as_u64().ok_or_else(|| de::Error::custom("invalid b"))? as u8;
                        let a = arr[3].as_u64().ok_or_else(|| de::Error::custom("invalid a"))? as u8;
                        Ok(MyAppValueType::Color(egui::Color32::from_rgba_premultiplied(r, g, b, a)))
                    }
                    "string" => {
                        let val = value.as_str()
                            .ok_or_else(|| de::Error::custom("invalid string value"))?;
                        Ok(MyAppValueType::String(val.to_string()))
                    }
                    _ => Err(de::Error::unknown_variant(&value_type, &["float", "integer", "boolean", "vector2", "vector3", "color", "string"])),
                }
            }
        }

        const FIELDS: &[&str] = &["type", "value"];
        deserializer.deserialize_struct("MyAppValueType", FIELDS, MyAppValueTypeVisitor)
    }
}
```

### 2.3 实现参数控件（WidgetValueTrait）

定义如何在 UI 中编辑电气参数值，包括简单的数值参数和复杂的配电回路、配电箱数据：

```rust
use egui_node_graph::WidgetValueTrait;

impl WidgetValueTrait<ElectricDataType> for ElectricValueType {
    fn value_widget(&mut self, ui: &mut egui::Ui, data_type: &ElectricDataType) -> egui::Response {
        // 根据数据类型显示不同的编辑控件
        match (self, data_type) {
            // 处理基本电气参数
            (ElectricValueType::Float(val),
             ElectricDataType::Current |
             ElectricDataType::Power |
             ElectricDataType::Voltage |
             ElectricDataType::PowerFactor |
             ElectricDataType::Coefficient) => {

                let label = match data_type {
                    ElectricDataType::Current => "电流(A): ",
                    ElectricDataType::Power => "功率(kW): ",
                    ElectricDataType::Voltage => "电压(V): ",
                    ElectricDataType::PowerFactor => "功率因数: ",
                    ElectricDataType::Coefficient => "需用系数: ",
                    _ => "值: "
                };

                ui.horizontal(|ui| {
                    ui.label(label);

                    if *data_type == ElectricDataType::PowerFactor || *data_type == ElectricDataType::Coefficient {
                        // 功率因数和需用系数限制在0-1之间
                        ui.add(egui::Slider::new(val, 0.0..=1.0).text(format!("{:.2}", val)))
                    } else {
                        // 其他数值参数
                        ui.add(egui::DragValue::new(val).clamp_range(0.0..=f64::MAX))
                    }
                }).response
            }

            // 处理整数参数
            (ElectricValueType::Integer(val), ElectricDataType::Integer) => {
                ui.add(egui::DragValue::new(val).clamp_range(0..=i64::MAX))
            }

            // 处理字符串参数
            (ElectricValueType::String(val), ElectricDataType::String) => {
                ui.text_edit_singleline(val)
            }

            // 处理配电回路数据（简化视图）
            (ElectricValueType::CircuitData(circuit), ElectricDataType::CircuitData) => {
                ui.collapsing(format!("回路: {}", circuit.name), |ui| {
                    ui.horizontal(|ui| {
                        ui.label("ID: ");
                        ui.text_edit_singleline(&mut circuit.id);
                    });
                    ui.horizontal(|ui| {
                        ui.label("名称: ");
                        ui.text_edit_singleline(&mut circuit.name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("功率(kW): ");
                        ui.add(egui::DragValue::new(&mut circuit.power).clamp_range(0.0..=f64::MAX));
                    });
                    ui.horizontal(|ui| {
                        ui.label("功率因数: ");
                        ui.add(egui::Slider::new(&mut circuit.power_factor, 0.0..=1.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("需用系数: ");
                        ui.add(egui::Slider::new(&mut circuit.coefficient, 0.0..=1.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("相别: ");
                        egui::ComboBox::from_label("")
                            .selected_text(&circuit.phase)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut circuit.phase, "A".to_string(), "A相");
                                ui.selectable_value(&mut circuit.phase, "B".to_string(), "B相");
                                ui.selectable_value(&mut circuit.phase, "C".to_string(), "C相");
                            });
                    });
                }).response
            }

            // 处理配电箱数据（简化视图）
            (ElectricValueType::DistributionBoxData(box_data), ElectricDataType::DistributionBoxData) => {
                ui.collapsing(format!("配电箱: {}", box_data.name), |ui| {
                    ui.horizontal(|ui| {
                        ui.label("ID: ");
                        ui.text_edit_singleline(&mut box_data.id);
                    });
                    ui.horizontal(|ui| {
                        ui.label("名称: ");
                        ui.text_edit_singleline(&mut box_data.name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("进线功率(kW): ");
                        ui.add(egui::DragValue::new(&mut box_data.incoming_power).clamp_range(0.0..=f64::MAX));
                    });
                    ui.horizontal(|ui| {
                        ui.label("进线电流(A): ");
                        ui.add(egui::DragValue::new(&mut box_data.incoming_current).clamp_range(0.0..=f64::MAX));
                    });

                    // 显示三相平衡信息
                    ui.collapsing("三相平衡信息", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("A相功率(kW): ");
                            ui.add(egui::DragValue::new(&mut box_data.phase_balance.phase_a_power).clamp_range(0.0..=f64::MAX));
                        });
                        ui.horizontal(|ui| {
                            ui.label("B相功率(kW): ");
                            ui.add(egui::DragValue::new(&mut box_data.phase_balance.phase_b_power).clamp_range(0.0..=f64::MAX));
                        });
                        ui.horizontal(|ui| {
                            ui.label("C相功率(kW): ");
                            ui.add(egui::DragValue::new(&mut box_data.phase_balance.phase_c_power).clamp_range(0.0..=f64::MAX));
                        });
                        ui.horizontal(|ui| {
                            ui.label("不平衡度(%): ");
                            ui.add(egui::DragValue::new(&mut box_data.phase_balance.balance_degree).clamp_range(0.0..=100.0));
                        });
                    });
                }).response
            }

            // 类型不匹配的情况
            _ => {
                ui.label("无效的值类型")
                    .on_hover_text("值类型与数据类型不匹配")
            }
        }
    }
}
```

## 3. 节点数据与模板定义

### 3.1 设计节点数据结构（NodeData）

创建存储电气系统节点特有信息的数据结构：

```rust
use egui_node_graph::NodeDataTrait;
use uuid::Uuid;

// 定义节点类型枚举
#[derive(Debug, Clone, PartialEq)]
enum ElectricNodeType {
    CircuitNode,        // 配电回路节点
    DistributionBoxNode, // 配电箱节点
    MainLineNode,        // 干线系统图节点
}

// 定义配电回路数据
#[derive(Debug, Clone)]
struct CircuitInfo {
    id: String,
    name: String,
    power: f64,         // 功率(kW)
    power_factor: f64,  // 功率因数
    coefficient: f64,   // 需用系数
    phase: String,      // 相别: "A", "B", "C"
    calculated_current: f64, // 计算电流(A)
}

// 定义三相平衡信息
#[derive(Debug, Clone)]
struct PhaseBalanceInfo {
    phase_a_power: f64, // A相功率(kW)
    phase_b_power: f64, // B相功率(kW)
    phase_c_power: f64, // C相功率(kW)
    balance_degree: f64, // 不平衡度(%)
}

// 定义配电箱数据
#[derive(Debug, Clone)]
struct DistributionBoxInfo {
    id: String,
    name: String,
    incoming_power: f64,    // 进线功率(kW)
    incoming_current: f64,  // 进线电流(A)
    phase_balance: PhaseBalanceInfo, // 三相平衡信息
    outgoing_circuits_count: usize, // 出线回路数量
}

// 定义干线系统图数据
#[derive(Debug, Clone)]
struct MainLineInfo {
    id: String,
    name: String,
    voltage_level: f64,     // 电压等级(kV)
    total_power: f64,       // 总功率(kW)
    current_calculated: f64, // 计算电流(A)
    cable_type: String,     // 电缆型号
    cable_length: f64,      // 电缆长度(m)
    resistance: f64,        // 线路电阻(Ω)
    reactance: f64,         // 线路电抗(Ω)
}

// 节点特有数据枚举
#[derive(Debug, Clone)]
enum ElectricNodeSpecificData {
    Circuit(CircuitInfo),
    DistributionBox(DistributionBoxInfo),
    MainLine(MainLineInfo),
}

// 定义节点数据结构
#[derive(Debug, Clone, Default)]
struct ElectricNodeData {
    node_type: ElectricNodeType,
    specific_data: ElectricNodeSpecificData,
    // 节点位置和大小
    position: egui::Pos2,
    size: egui::Vec2,
}

// 实现NodeDataTrait以自定义节点UI和行为
impl NodeDataTrait<ElectricDataType, ElectricValueType> for ElectricNodeData {
    // 创建新节点
    fn new_with_id(id: Uuid) -> Self {
        // 创建默认的配电回路节点
        Self {
            node_type: ElectricNodeType::CircuitNode,
            specific_data: ElectricNodeSpecificData::Circuit(CircuitInfo {
                id: format!("CIR-{}", id.to_string().split('-').next().unwrap()),
                name: "新建回路".to_string(),
                power: 0.0,
                power_factor: 0.8,
                coefficient: 0.8,
                phase: "A".to_string(),
                calculated_current: 0.0,
            }),
            position: egui::Pos2::new(0.0, 0.0),
            size: egui::Vec2::new(200.0, 150.0),
        }
    }

    // 获取节点ID
    fn id(&self) -> &Uuid {
        // 根据节点类型获取对应ID
        match &self.specific_data {
            ElectricNodeSpecificData::Circuit(circuit) => {
                // 这里简化处理，实际应该存储UUID
                static dummy_uuid: Uuid = Uuid::nil();
                &dummy_uuid
            },
            ElectricNodeSpecificData::DistributionBox(box_data) => {
                static dummy_uuid: Uuid = Uuid::nil();
                &dummy_uuid
            },
            ElectricNodeSpecificData::MainLine(main_line) => {
                static dummy_uuid: Uuid = Uuid::nil();
                &dummy_uuid
            },
        }
    }

    // 克隆节点
    fn clone_node(&self) -> Box<dyn NodeDataTrait<ElectricDataType, ElectricValueType>> {
        let new_id = Uuid::new_v4();
        let cloned_specific_data = match &self.specific_data {
            ElectricNodeSpecificData::Circuit(circuit) => ElectricNodeSpecificData::Circuit(CircuitInfo {
                id: format!("CIR-{}", new_id.to_string().split('-').next().unwrap()),
                name: format!("{}(副本)", circuit.name),
                power: circuit.power,
                power_factor: circuit.power_factor,
                coefficient: circuit.coefficient,
                phase: circuit.phase.clone(),
                calculated_current: circuit.calculated_current,
            }),
            ElectricNodeSpecificData::DistributionBox(box_data) => {
                ElectricNodeSpecificData::DistributionBox(DistributionBoxInfo {
                    id: format!("DB-{}", new_id.to_string().split('-').next().unwrap()),
                    name: format!("{}(副本)", box_data.name),
                    incoming_power: box_data.incoming_power,
                    incoming_current: box_data.incoming_current,
                    phase_balance: PhaseBalanceInfo {
                        phase_a_power: box_data.phase_balance.phase_a_power,
                        phase_b_power: box_data.phase_balance.phase_b_power,
                        phase_c_power: box_data.phase_balance.phase_c_power,
                        balance_degree: box_data.phase_balance.balance_degree,
                    },
                    outgoing_circuits_count: box_data.outgoing_circuits_count,
                })
            },
            ElectricNodeSpecificData::MainLine(main_line) => ElectricNodeSpecificData::MainLine(MainLineInfo {
                id: format!("ML-{}", new_id.to_string().split('-').next().unwrap()),
                name: format!("{}(副本)", main_line.name),
                voltage_level: main_line.voltage_level,
                total_power: main_line.total_power,
                current_calculated: main_line.current_calculated,
                cable_type: main_line.cable_type.clone(),
                cable_length: main_line.cable_length,
                resistance: main_line.resistance,
                reactance: main_line.reactance,
            }),
        };

        Box::new(Self {
            node_type: self.node_type.clone(),
            specific_data: cloned_specific_data,
            position: self.position,
            size: self.size,
        })
    }

    // 自定义节点底部UI
    fn bottom_ui(&mut self, ui: &mut egui::Ui, node_id: egui_node_graph::NodeId) {
        match (&mut self.specific_data, &self.node_type) {
            (ElectricNodeSpecificData::Circuit(circuit), ElectricNodeType::CircuitNode) => {
                ui.collapsing("回路参数", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("ID: ");
                        ui.text_edit_singleline(&mut circuit.id);
                    });
                    ui.horizontal(|ui| {
                        ui.label("名称: ");
                        ui.text_edit_singleline(&mut circuit.name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("功率(kW): ");
                        ui.add(egui::DragValue::new(&mut circuit.power).clamp_range(0.0..=f64::MAX));
                    });
                    ui.horizontal(|ui| {
                        ui.label("功率因数: ");
                        ui.add(egui::Slider::new(&mut circuit.power_factor, 0.0..=1.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("需用系数: ");
                        ui.add(egui::Slider::new(&mut circuit.coefficient, 0.0..=1.0));
                    });
                    ui.horizontal(|ui| {
                        ui.label("相别: ");
                        egui::ComboBox::from_label("")
                            .selected_text(&circuit.phase)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut circuit.phase, "A".to_string(), "A相");
                                ui.selectable_value(&mut circuit.phase, "B".to_string(), "B相");
                                ui.selectable_value(&mut circuit.phase, "C".to_string(), "C相");
                            });
                    });

                    // 计算并显示电流
                    if ui.button("计算电流").clicked() {
                        // 单相计算：I = P * Kx / (U * cosφ)
                        // 假设电压为220V
                        const VOLTAGE: f64 = 220.0;
                        circuit.calculated_current = (circuit.power * 1000.0 * circuit.coefficient) /
                                                   (VOLTAGE * circuit.power_factor);
                    }

                    ui.horizontal(|ui| {
                        ui.label("计算电流(A): ");
                        ui.label(format!("{:.2}", circuit.calculated_current));
                    });
                });
            },

            (ElectricNodeSpecificData::DistributionBox(box_data), ElectricNodeType::DistributionBoxNode) => {
                ui.collapsing("配电箱参数", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("ID: ");
                        ui.text_edit_singleline(&mut box_data.id);
                    });
                    ui.horizontal(|ui| {
                        ui.label("名称: ");
                        ui.text_edit_singleline(&mut box_data.name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("进线功率(kW): ");
                        ui.add(egui::DragValue::new(&mut box_data.incoming_power).clamp_range(0.0..=f64::MAX));
                    });
                    ui.horizontal(|ui| {
                        ui.label("进线电流(A): ");
                        ui.add(egui::DragValue::new(&mut box_data.incoming_current).clamp_range(0.0..=f64::MAX));
                    });
                    ui.horizontal(|ui| {
                        ui.label("出线回路数: ");
                        ui.add(egui::DragValue::new(&mut box_data.outgoing_circuits_count).clamp_range(0..=100));
                    });

                    // 三相平衡信息
                    ui.collapsing("三相平衡信息", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("A相功率(kW): ");
                            ui.add(egui::DragValue::new(&mut box_data.phase_balance.phase_a_power).clamp_range(0.0..=f64::MAX));
                        });
                        ui.horizontal(|ui| {
                            ui.label("B相功率(kW): ");
                            ui.add(egui::DragValue::new(&mut box_data.phase_balance.phase_b_power).clamp_range(0.0..=f64::MAX));
                        });
                        ui.horizontal(|ui| {
                            ui.label("C相功率(kW): ");
                            ui.add(egui::DragValue::new(&mut box_data.phase_balance.phase_c_power).clamp_range(0.0..=f64::MAX));
                        });

                        // 计算不平衡度
                        if ui.button("计算不平衡度").clicked() {
                            let max_phase = box_data.phase_balance.phase_a_power
                                .max(box_data.phase_balance.phase_b_power)
                                .max(box_data.phase_balance.phase_c_power);
                            let min_phase = box_data.phase_balance.phase_a_power
                                .min(box_data.phase_balance.phase_b_power)
                                .min(box_data.phase_balance.phase_c_power);

                            if max_phase > 0.0 {
                                box_data.phase_balance.balance_degree = ((max_phase - min_phase) / max_phase) * 100.0;
                            }
                        }

                        ui.horizontal(|ui| {
                            ui.label("不平衡度(%): ");
                            ui.label(format!("{:.2}", box_data.phase_balance.balance_degree));
                        });
                    });
                });
            },

            (ElectricNodeSpecificData::MainLine(main_line), ElectricNodeType::MainLineNode) => {
                ui.collapsing("干线参数", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("ID: ");
                        ui.text_edit_singleline(&mut main_line.id);
                    });
                    ui.horizontal(|ui| {
                        ui.label("名称: ");
                        ui.text_edit_singleline(&mut main_line.name);
                    });
                    ui.horizontal(|ui| {
                        ui.label("电压等级(kV): ");
                        ui.add(egui::DragValue::new(&mut main_line.voltage_level).clamp_range(0.0..=f64::MAX));
                    });
                    ui.horizontal(|ui| {
                        ui.label("总功率(kW): ");
                        ui.add(egui::DragValue::new(&mut main_line.total_power).clamp_range(0.0..=f64::MAX));
                    });
                    ui.horizontal(|ui| {
                        ui.label("电缆型号: ");
                        ui.text_edit_singleline(&mut main_line.cable_type);
                    });
                    ui.horizontal(|ui| {
                        ui.label("电缆长度(m): ");
                        ui.add(egui::DragValue::new(&mut main_line.cable_length).clamp_range(0.0..=f64::MAX));
                    });

                    // 计算干线参数
                    if ui.button("计算干线参数").clicked() {
                        // 三相计算：I = P * 1000 / (√3 * U * cosφ)
                        // 假设功率因数为0.8
                        const POWER_FACTOR: f64 = 0.8;
                        let voltage = main_line.voltage_level * 1000.0; // 转换为V
                        main_line.current_calculated = (main_line.total_power * 1000.0) /
                                                      (3.0_f64.sqrt() * voltage * POWER_FACTOR);

                        // 简化的电阻电抗计算
                        main_line.resistance = 0.1 * main_line.cable_length / 1000.0;
                        main_line.reactance = 0.08 * main_line.cable_length / 1000.0;
                    }

                    ui.horizontal(|ui| {
                        ui.label("计算电流(A): ");
                        ui.label(format!("{:.2}", main_line.current_calculated));
                    });
                    ui.horizontal(|ui| {
                        ui.label("线路电阻(Ω): ");
                        ui.label(format!("{:.4}", main_line.resistance));
                    });
                    ui.horizontal(|ui| {
                        ui.label("线路电抗(Ω): ");
                        ui.label(format!("{:.4}", main_line.reactance));
                    });
                });
            },

            _ => {}
        }
    }

    // 自定义顶部栏UI
    fn top_bar_ui(&mut self, ui: &mut egui::Ui, node_id: egui_node_graph::NodeId) -> Option<egui::Response> {
        // 添加节点类型标签
        let node_type_label = match self.node_type {
            ElectricNodeType::CircuitNode => "配电回路",
            ElectricNodeType::DistributionBoxNode => "配电箱",
            ElectricNodeType::MainLineNode => "干线系统",
        };

        ui.label(node_type_label).response
    }

    // 自定义标题栏颜色
    fn title_bar_color(&self) -> Option<egui::Color32> {
        match self.node_type {
            ElectricNodeType::CircuitNode => Some(egui::Color32::from_rgb(100, 180, 200)),
            ElectricNodeType::DistributionBoxNode => Some(egui::Color32::from_rgb(100, 200, 140)),
            ElectricNodeType::MainLineNode => Some(egui::Color32::from_rgb(200, 140, 100)),
        }
    }
```

### 3.2 定义节点模板（NodeTemplate）

创建电气系统专用的节点类型模板：

```rust
use egui_node_graph::{NodeTemplateTrait, InputParamKind};

// 定义电气系统节点模板枚举
#[derive(Debug, Clone, Copy, PartialEq)]
enum ElectricNodeTemplate {
    // 配电回路相关节点
    CircuitNode,
    CircuitGroupNode,

    // 配电箱相关节点
    DistributionBoxNode,
    MainDistributionBoxNode,
    SubDistributionBoxNode,

    // 干线系统图相关节点
    MainLineNode,
    FeederLineNode,

    // 电源节点
    PowerSourceNode,

    // 计算节点
    CurrentCalculationNode,
    PhaseBalanceNode,
}

// 实现CategoryTrait以支持节点分类
impl egui_node_graph::CategoryTrait for ElectricNodeTemplate {
    fn category(&self) -> &str {
        match self {
            ElectricNodeTemplate::CircuitNode |
            ElectricNodeTemplate::CircuitGroupNode => "配电回路",

            ElectricNodeTemplate::DistributionBoxNode |
            ElectricNodeTemplate::MainDistributionBoxNode |
            ElectricNodeTemplate::SubDistributionBoxNode => "配电箱",

            ElectricNodeTemplate::MainLineNode |
            ElectricNodeTemplate::FeederLineNode => "干线系统",

            ElectricNodeTemplate::PowerSourceNode => "电源",

            ElectricNodeTemplate::CurrentCalculationNode |
            ElectricNodeTemplate::PhaseBalanceNode => "计算工具",
        }
    }
}

// 实现NodeTemplateTrait以定义如何构建节点
impl NodeTemplateTrait for ElectricNodeTemplate {
    type NodeData = ElectricNodeData;
    type DataType = ElectricDataType;
    type ValueType = ElectricValueType;
    type Category = Self;
    type UserState = (); // 可以使用自定义类型存储用户状态

    fn node_label(&self) -> String {
        match self {
            ElectricNodeTemplate::CircuitNode => "配电回路",
            ElectricNodeTemplate::CircuitGroupNode => "回路组",
            ElectricNodeTemplate::DistributionBoxNode => "配电箱",
            ElectricNodeTemplate::MainDistributionBoxNode => "总配电箱",
            ElectricNodeTemplate::SubDistributionBoxNode => "分配电箱",
            ElectricNodeTemplate::MainLineNode => "干线",
            ElectricNodeTemplate::FeederLineNode => "馈线",
            ElectricNodeTemplate::PowerSourceNode => "电源",
            ElectricNodeTemplate::CurrentCalculationNode => "电流计算",
            ElectricNodeTemplate::PhaseBalanceNode => "三相平衡",
        }
    }

    fn node_category(&self) -> Self::Category {
        *self
    }

    fn user_data(&self) -> Self::NodeData {
        // 使用UUID::new_v4()生成临时ID
        let temp_id = uuid::Uuid::new_v4();
        self.node_data(temp_id)
    }

    fn node_data(&self, id: uuid::Uuid) -> Self::NodeData {
        let mut node_data = ElectricNodeData::new_with_id(id);

        // 根据模板类型设置节点数据
        match self {
            ElectricNodeTemplate::CircuitNode => {
                node_data.node_type = ElectricNodeType::CircuitNode;
                node_data.specific_data = ElectricNodeSpecificData::Circuit(CircuitInfo {
                    id: format!("CIR-{}", id.to_string().split('-').next().unwrap()),
                    name: "配电回路".to_string(),
                    power: 0.0,
                    power_factor: 0.8,
                    coefficient: 0.8,
                    phase: "A".to_string(),
                    calculated_current: 0.0,
                });
                node_data.size = egui::Vec2::new(200.0, 150.0);
            },

            ElectricNodeTemplate::DistributionBoxNode |
            ElectricNodeTemplate::MainDistributionBoxNode |
            ElectricNodeTemplate::SubDistributionBoxNode => {
                node_data.node_type = ElectricNodeType::DistributionBoxNode;
                let box_type_name = match self {
                    ElectricNodeTemplate::MainDistributionBoxNode => "总配电箱",
                    ElectricNodeTemplate::SubDistributionBoxNode => "分配电箱",
                    _ => "配电箱",
                };
                node_data.specific_data = ElectricNodeSpecificData::DistributionBox(DistributionBoxInfo {
                    id: format!("DB-{}", id.to_string().split('-').next().unwrap()),
                    name: box_type_name.to_string(),
                    incoming_power: 0.0,
                    incoming_current: 0.0,
                    phase_balance: PhaseBalanceInfo {
                        phase_a_power: 0.0,
                        phase_b_power: 0.0,
                        phase_c_power: 0.0,
                        balance_degree: 0.0,
                    },
                    outgoing_circuits_count: 0,
                });
                node_data.size = egui::Vec2::new(220.0, 200.0);
            },

            ElectricNodeTemplate::MainLineNode |
            ElectricNodeTemplate::FeederLineNode => {
                node_data.node_type = ElectricNodeType::MainLineNode;
                let line_type_name = match self {
                    ElectricNodeTemplate::MainLineNode => "干线",
                    _ => "馈线",
                };
                node_data.specific_data = ElectricNodeSpecificData::MainLine(MainLineInfo {
                    id: format!("ML-{}", id.to_string().split('-').next().unwrap()),
                    name: line_type_name.to_string(),
                    voltage_level: 0.4, // 默认0.4kV
                    total_power: 0.0,
                    current_calculated: 0.0,
                    cable_type: "YJV-0.6/1kV".to_string(),
                    cable_length: 0.0,
                    resistance: 0.0,
                    reactance: 0.0,
                });
                node_data.size = egui::Vec2::new(240.0, 220.0);
            },

            _ => {
                // 默认配置
            }
        }

        node_data
    }

    fn build_node(
        &self,
        graph: &mut egui_node_graph::Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        node_id: egui_node_graph::NodeId,
    ) {
        // 根据节点类型添加输入输出参数
        match self {
            ElectricNodeTemplate::CircuitNode => {
                // 配电回路节点：一个输入（上级电源），多个输出（功率、电流、回路数据）
                graph.add_input_param(node_id, "上级电源", InputParamKind::ConnectionOnly, ElectricDataType::Voltage, None);
                graph.add_output_param(node_id, "功率", ElectricDataType::Power);
                graph.add_output_param(node_id, "电流", ElectricDataType::Current);
                graph.add_output_param(node_id, "回路数据", ElectricDataType::CircuitData);
            },

            ElectricNodeTemplate::DistributionBoxNode |
            ElectricNodeTemplate::MainDistributionBoxNode |
            ElectricNodeTemplate::SubDistributionBoxNode => {
                // 配电箱节点：一个输入（上级进线），多个输出（出线电流、三相平衡、配电箱数据）
                graph.add_input_param(node_id, "上级进线", InputParamKind::ConnectionOnly, ElectricDataType::Current, None);
                graph.add_output_param(node_id, "出线电流", ElectricDataType::Current);
                graph.add_output_param(node_id, "三相平衡", ElectricDataType::PhaseBalance);
                graph.add_output_param(node_id, "配电箱数据", ElectricDataType::DistributionBoxData);
            },

            ElectricNodeTemplate::MainLineNode |
            ElectricNodeTemplate::FeederLineNode => {
                // 干线节点：一个输入（电源输入），多个输出（线路电流、电压降、干线数据）
                graph.add_input_param(node_id, "电源输入", InputParamKind::ConnectionOnly, ElectricDataType::Voltage, None);
                graph.add_output_param(node_id, "线路电流", ElectricDataType::Current);
                graph.add_output_param(node_id, "电压降", ElectricDataType::Voltage);
                graph.add_output_param(node_id, "干线数据", ElectricDataType::MainLineData);
            },

            ElectricNodeTemplate::PowerSourceNode => {
                // 电源节点：无输入，输出电压和电源信息
                graph.add_output_param(node_id, "电压", ElectricDataType::Voltage);
                graph.add_output_param(node_id, "电源容量", ElectricDataType::Power);
            },

            ElectricNodeTemplate::CurrentCalculationNode => {
                // 电流计算节点：输入功率、电压、功率因数，输出计算电流
                graph.add_input_param(node_id, "功率", InputParamKind::ConnectionOrConstant, ElectricDataType::Power, Some(ElectricValueType::Float(0.0)));
                graph.add_input_param(node_id, "电压", InputParamKind::ConnectionOrConstant, ElectricDataType::Voltage, Some(ElectricValueType::Float(220.0)));
                graph.add_input_param(node_id, "功率因数", InputParamKind::ConnectionOrConstant, ElectricDataType::PowerFactor, Some(ElectricValueType::Float(0.8)));
                graph.add_input_param(node_id, "需用系数", InputParamKind::ConnectionOrConstant, ElectricDataType::Coefficient, Some(ElectricValueType::Float(0.8)));
                graph.add_output_param(node_id, "计算电流", ElectricDataType::Current);
            },

            ElectricNodeTemplate::PhaseBalanceNode => {
                // 三相平衡节点：输入A/B/C相功率，输出不平衡度和优化建议
                graph.add_input_param(node_id, "A相功率", InputParamKind::ConnectionOrConstant, ElectricDataType::Power, Some(ElectricValueType::Float(0.0)));
                graph.add_input_param(node_id, "B相功率", InputParamKind::ConnectionOrConstant, ElectricDataType::Power, Some(ElectricValueType::Float(0.0)));
                graph.add_input_param(node_id, "C相功率", InputParamKind::ConnectionOrConstant, ElectricDataType::Power, Some(ElectricValueType::Float(0.0)));
                graph.add_output_param(node_id, "不平衡度", ElectricDataType::Coefficient);
                graph.add_output_param(node_id, "平衡建议", ElectricDataType::String);
            },

            _ => {
                // 默认情况：不添加参数
            }
        }
    }
}

// 提供一个获取所有可用模板的函数
fn all_electric_templates() -> Vec<ElectricNodeTemplate> {
    vec![
        // 配电回路相关节点
        ElectricNodeTemplate::CircuitNode,
        ElectricNodeTemplate::CircuitGroupNode,

        // 配电箱相关节点
        ElectricNodeTemplate::DistributionBoxNode,
        ElectricNodeTemplate::MainDistributionBoxNode,
        ElectricNodeTemplate::SubDistributionBoxNode,

        // 干线系统图相关节点
        ElectricNodeTemplate::MainLineNode,
        ElectricNodeTemplate::FeederLineNode,

        // 电源节点
        ElectricNodeTemplate::PowerSourceNode,

        // 计算节点
        ElectricNodeTemplate::CurrentCalculationNode,
        ElectricNodeTemplate::PhaseBalanceNode,
    ]
}
```

## 4. 应用集成与实现

### 4.1 创建电气系统应用状态结构体

```rust
use egui_node_graph::GraphEditorState;
use uuid::Uuid;

// 定义电气系统应用状态
struct ElectricNodeGraphApp {
    // 图数据 - 使用电气系统专用数据类型
    graph: egui_node_graph::Graph<ElectricNodeData, ElectricDataType, ElectricValueType>,
    // 编辑器状态
    editor_state: GraphEditorState,
    // 项目信息
    project_name: String,
    project_id: Uuid,
    // 计算结果缓存
    calculation_cache: std::collections::HashMap<String, f64>,
    // 错误信息
    error_message: Option<String>,
}

impl Default for ElectricNodeGraphApp {
    fn default() -> Self {
        Self {
            graph: Default::default(),
            editor_state: GraphEditorState::default(),
            project_name: "电气配电系统项目".to_string(),
            project_id: Uuid::new_v4(),
            calculation_cache: std::collections::HashMap::new(),
            error_message: None,
        }
    }
}
```

### 4.2 实现电气系统应用的 UI 和逻辑

```rust
use eframe::egui;
use eframe::{App, Frame};

impl eframe::App for ElectricNodeGraphApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // 创建应用标题和项目信息
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("{} - 项目ID: {}", self.project_name, self.project_id.to_string().split('-').next().unwrap())).heading().color(egui::Color32::from_rgb(0, 100, 200)));
                if ui.button("项目设置").clicked() {
                    // 打开项目设置对话框
                    self.open_project_settings(ui);
                }
            });
            ui.separator();

            // 创建工具栏
            ui.horizontal(|ui| {
                // 添加节点按钮
                if ui.button("添加节点").clicked() {
                    // 创建上下文菜单显示可用节点类型，按类别分组
                    ui.menu_button("选择节点类型", |ui| {
                        // 按类别组织节点模板
                        let templates = all_electric_templates();

                        // 收集所有唯一类别
                        let mut categories = std::collections::HashSet::new();
                        for template in &templates {
                            categories.insert(template.category().to_string());
                        }

                        // 按类别显示节点
                        for category in categories.into_iter() {
                            ui.menu_button(category, |ui| {
                                for template in &templates {
                                    if template.category() == category {
                                        if ui.button(template.node_label()).clicked() {
                                            // 添加新节点
                                            let _ = self.graph.add_node(
                                                template.node_label(),
                                                template.user_data(),
                                                |graph, node_id| template.build_node(graph, &mut (), node_id),
                                            );
                                            ui.close_menu();
                                        }
                                    }
                                }
                            });
                        }
                    });
                }

                // 保存和加载按钮
                if ui.button("保存项目").clicked() {
                    // 实现项目保存功能
                    if let Err(e) = self.save_project() {
                        self.error_message = Some(format!("保存失败: {}", e));
                    } else {
                        self.error_message = Some("项目保存成功".to_string());
                    }
                }

                if ui.button("加载项目").clicked() {
                    // 实现项目加载功能
                    if let Err(e) = self.load_project() {
                        self.error_message = Some(format!("加载失败: {}", e));
                    } else {
                        self.error_message = Some("项目加载成功".to_string());
                    }
                }

                // 运行计算按钮
                if ui.button("运行计算").clicked() {
                    self.run_calculations();
                    self.error_message = Some("计算完成".to_string());
                }

                // 生成报告按钮
                if ui.button("生成报告").clicked() {
                    if let Err(e) = self.generate_report() {
                        self.error_message = Some(format!("报告生成失败: {}", e));
                    } else {
                        self.error_message = Some("报告生成成功".to_string());
                    }
                }
            });

            // 显示错误信息
            if let Some(error_msg) = &self.error_message {
                ui.label(egui::RichText::new(error_msg).color(egui::Color32::RED));
                // 清除错误信息计时器
                ui.ctx().request_repaint_after(std::time::Duration::from_secs(3));
                if ui.button("×").clicked() {
                    self.error_message = None;
                }
            }

            // 创建一个可滚动区域
            egui::ScrollArea::both().show(ui, |ui| {
                // 调整容器大小
                let (_id, response) = ui.allocate_space(egui::Vec2::new(
                    ui.available_width(),
                    ui.available_height() - 50.0, // 留出状态栏空间
                ));

                // 绘制节点图编辑器
                let node_responses = egui_node_graph::draw_graph_editor(
                    ui,
                    &mut self.graph,
                    &mut self.editor_state,
                    response.rect,
                    &all_electric_templates(),
                    &mut (), // 用户状态
                );

                // 处理节点响应事件
                self.handle_node_responses(node_responses);
            });

            // 状态栏显示
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!("节点数量: {}", self.graph.nodes.len()));
                ui.label(format!("连接数量: {}", self.graph.connections.len()));
                ui.label(format!("计算缓存: {}", self.calculation_cache.len()));
            });
        });
    }
}

impl ElectricNodeGraphApp {
    // 保存项目
    fn save_project(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 创建项目数据结构
        let project_data = ElectricProjectData {
            project_name: self.project_name.clone(),
            project_id: self.project_id,
            graph: &self.graph,
            calculation_cache: self.calculation_cache.clone(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            save_date: chrono::Utc::now(),
        };

        // 序列化并保存到文件
        let json_data = serde_json::to_string_pretty(&project_data)?;
        let file_path = format!("{}_{}.json",
            self.project_name.replace(' ', '_'),
            chrono::Utc::now().format("%Y%m%d_%H%M%S"));
        std::fs::write(file_path, json_data)?;

        Ok(())
    }

    // 加载项目
    fn load_project(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 这里应该使用文件选择对话框，但为简化示例，直接读取
        // 在实际应用中，应使用文件对话框库如rfd或native-dialog
        let file_path = "电气配电系统项目.json";

        let json_data = std::fs::read_to_string(file_path)?;
        let project_data: ElectricProjectData = serde_json::from_str(&json_data)?;

        // 恢复项目数据
        self.project_name = project_data.project_name;
        self.project_id = project_data.project_id;
        self.graph = project_data.graph;
        self.calculation_cache = project_data.calculation_cache;

        Ok(())
    }

    // 运行电气系统计算
    fn run_calculations(&mut self) {
        // 1. 构建依赖图并确定计算顺序（拓扑排序）
        let execution_order = self.determine_calculation_order();

        // 2. 清除计算缓存
        self.calculation_cache.clear();

        // 3. 按照计算顺序执行每个节点
        for node_id in execution_order {
            self.calculate_node(node_id);
        }

        // 4. 计算三相平衡
        self.calculate_phase_balance();

        // 5. 更新所有节点的显示数据
        self.update_node_display_data();
    }

    // 生成报告
    fn generate_report(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 创建报告生成器
        let mut report = ElectricSystemReport::new(&self.project_name, self.project_id);

        // 添加系统概览
        report.add_system_overview(&self.graph);

        // 添加节点详情
        report.add_nodes_details(&self.graph);

        // 添加计算结果
        report.add_calculation_results(&self.calculation_cache);

        // 添加三相平衡分析
        report.add_phase_balance_analysis(&self.graph);

        // 添加推荐建议
        report.add_recommendations(&self.graph, &self.calculation_cache);

        // 生成PDF报告
        let file_path = format!("报告_{}_{}.pdf",
            self.project_name.replace(' ', '_'),
            chrono::Utc::now().format("%Y%m%d_%H%M%S"));
        report.generate_pdf(&file_path)?;

        Ok(())
    }

    // 打开项目设置对话框
    fn open_project_settings(&mut self, ui: &mut egui::Ui) {
        // 这里可以实现项目设置对话框
        // 例如修改项目名称、项目参数等
    }

    // 确定计算顺序（拓扑排序）
    fn determine_calculation_order(&self) -> Vec<egui_node_graph::NodeId> {
        // 实现拓扑排序算法确定节点计算顺序
        // 从电源节点开始，依次计算下游节点
        let mut order = Vec::new();
        let mut visited = std::collections::HashSet::new();

        // 实现简化版拓扑排序
        for node_id in self.graph.nodes.keys() {
            self.traverse_node(node_id, &mut visited, &mut order);
        }

        order
    }

    // 遍历节点进行拓扑排序
    fn traverse_node(&self, node_id: egui_node_graph::NodeId, visited: &mut std::collections::HashSet<egui_node_graph::NodeId>, order: &mut Vec<egui_node_graph::NodeId>) {
        if visited.contains(&node_id) {
            return;
        }

        // 先访问所有依赖的上游节点
        // 这里需要实现依赖关系分析

        // 然后将当前节点加入顺序
        visited.insert(node_id);
        order.push(node_id);
    }

    // 计算单个节点
    fn calculate_node(&mut self, node_id: egui_node_graph::NodeId) {
        // 根据节点类型执行特定计算
        if let Some(node) = self.graph.nodes.get(node_id) {
            match &node.user_data.node_type {
                ElectricNodeType::CircuitNode => self.calculate_circuit_node(node_id, node),
                ElectricNodeType::DistributionBoxNode => self.calculate_distribution_box_node(node_id, node),
                ElectricNodeType::MainLineNode => self.calculate_main_line_node(node_id, node),
                ElectricNodeType::PowerSourceNode => self.calculate_power_source_node(node_id, node),
                _ => {}
            }
        }
    }

    // 计算配电回路节点
    fn calculate_circuit_node(&mut self, node_id: egui_node_graph::NodeId, node: &egui_node_graph::Node<ElectricNodeData, ElectricDataType, ElectricValueType>) {
        if let ElectricNodeSpecificData::Circuit(circuit_info) = &node.user_data.specific_data {
            // 根据功率、电压、功率因数、需用系数计算电流
            // 这里使用实际的电流计算公式
            let voltage = self.get_input_voltage(node_id).unwrap_or(220.0); // 默认220V
            let current = calculate_current(circuit_info.power, voltage, circuit_info.power_factor, circuit_info.coefficient);

            // 更新计算结果
            let cache_key = format!("circuit_current_{}", node_id.0);
            self.calculation_cache.insert(cache_key, current);

            // 更新节点数据
            // 注意：在实际应用中，需要通过可变引用更新节点数据
        }
    }

    // 获取输入电压
    fn get_input_voltage(&self, node_id: egui_node_graph::NodeId) -> Option<f64> {
        // 从输入参数或上游连接获取电压值
        // 这里需要实现实际的电压获取逻辑
        None
    }

    // 计算三相平衡
    fn calculate_phase_balance(&mut self) {
        // 遍历所有配电箱节点，计算三相平衡
        for node_id in self.graph.nodes.keys() {
            if let Some(node) = self.graph.nodes.get(node_id) {
                if let ElectricNodeType::DistributionBoxNode = node.user_data.node_type {
                    // 收集A/B/C相功率数据
                    let (phase_a, phase_b, phase_c) = self.collect_phase_powers(node_id);

                    // 计算不平衡度
                    let balance_degree = calculate_phase_balance_degree(phase_a, phase_b, phase_c);

                    // 存储结果
                    let cache_key = format!("balance_degree_{}", node_id.0);
                    self.calculation_cache.insert(cache_key, balance_degree);
                }
            }
        }
    }

    // 收集相功率
    fn collect_phase_powers(&self, distribution_box_id: egui_node_graph::NodeId) -> (f64, f64, f64) {
        // 实现收集配电箱连接的各相功率数据
        (0.0, 0.0, 0.0) // 默认值，需要实际实现
    }

    // 更新节点显示数据
    fn update_node_display_data(&mut self) {
        // 更新所有节点的显示数据，如计算结果、状态等
    }

    // 处理节点响应事件
    fn handle_node_responses(&mut self, responses: Vec<egui_node_graph::NodeResponse<ElectricNodeTemplate>>) {
        for response in responses {
            match response {
                egui_node_graph::NodeResponse::NodeAdded(node_id, template) => {
                    println!("添加节点: {:?} 节点ID: {:?}", template, node_id);
                    // 节点添加后的额外逻辑
                    self.on_node_added(node_id, &template);
                }
                egui_node_graph::NodeResponse::NodeRemoved(node_id) => {
                    println!("删除节点ID: {:?}", node_id);
                    // 节点删除后的清理逻辑
                    self.on_node_removed(node_id);
                }
                egui_node_graph::NodeResponse::ConnectionAdded(output_id, input_id) => {
                    println!("添加连接: {:?} -> {:?}", output_id, input_id);
                    // 电气系统连接类型兼容性检查
                    if let Some(output_node) = self.graph.nodes.get(&output_id.node_id) {
                        if let Some(input_node) = self.graph.nodes.get(&input_id.node_id) {
                            if let Some(output) = output_node.outputs.get(output_id.index) {
                                if let Some(input) = input_node.inputs.get(input_id.index) {
                                    // 电气系统专用的连接规则
                                    if !self.is_connection_valid(&output_node.user_data, &input_node.user_data, output.data_type, input.data_type) {
                                        // 不兼容的连接，移除连接并显示错误
                                        self.graph.remove_connection(output_id, input_id);
                                        self.error_message = Some(format!("连接规则不允许: {} -> {}",
                                            output_node.label, input_node.label));
                                    } else if output.data_type != input.data_type {
                                        // 数据类型不匹配
                                        self.graph.remove_connection(output_id, input_id);
                                        self.error_message = Some("连接数据类型不兼容".to_string());
                                    } else {
                                        // 连接成功，更新依赖关系
                                        self.update_dependencies();
                                        // 触发相关节点重新计算
                                        self.invalidate_affected_nodes(input_id.node_id);
                                    }
                                }
                            }
                        }
                    }
                }
                egui_node_graph::NodeResponse::ConnectionRemoved(input_id) => {
                    println!("删除连接: {:?}", input_id);
                    // 更新依赖关系
                    self.update_dependencies();
                    // 清除相关计算结果缓存
                    self.invalidate_affected_nodes(input_id.node_id);
                }
                egui_node_graph::NodeResponse::ParameterValueChanged(param_id, param_type) => {
                    println!("参数变更: {:?} 类型: {:?}", param_id, param_type);
                    // 找到参数所属的节点
                    if let Some(node_id) = self.find_node_by_param_id(param_id) {
                        // 使节点及下游节点失效
                        self.invalidate_affected_nodes(node_id);
                    }
                }
                egui_node_graph::NodeResponse::NodeDoubleClicked(node_id) => {
                    println!("双击节点: {:?}", node_id);
                    // 打开节点属性编辑对话框
                    self.open_node_properties_dialog(node_id);
                }
                egui_node_graph::NodeResponse::NodeSelected(node_ids) => {
                    println!("选中节点: {:?}", node_ids);
                    // 更新选中状态
                    self.selected_nodes = node_ids;
                }
                _ => {}
            }
        }
    }

    // 检查连接是否有效（电气系统规则）
    fn is_connection_valid(&self, output_node_data: &ElectricNodeData, input_node_data: &ElectricNodeData,
                          output_type: ElectricDataType, input_type: ElectricDataType) -> bool {
        // 定义电气系统的连接规则
        match (output_node_data.node_type, input_node_data.node_type) {
            // 电源节点只能连接到干线或配电箱
            (ElectricNodeType::PowerSourceNode, ElectricNodeType::MainLineNode) => true,
            (ElectricNodeType::PowerSourceNode, ElectricNodeType::DistributionBoxNode) => true,

            // 干线可以连接到配电箱
            (ElectricNodeType::MainLineNode, ElectricNodeType::DistributionBoxNode) => true,

            // 配电箱可以连接到配电回路
            (ElectricNodeType::DistributionBoxNode, ElectricNodeType::CircuitNode) => true,

            // 计算节点的连接规则
            (_, ElectricNodeType::CalculationNode) => true, // 任何节点都可以连接到计算节点
            (ElectricNodeType::CalculationNode, _) => true, // 计算节点可以连接到任何节点

            // 其他连接类型需要根据具体的数据类型判断
            _ => {
                // 电压/电流/功率信号可以在不同节点间传递
                matches!((output_type, input_type),
                    (ElectricDataType::Voltage, ElectricDataType::Voltage) |
                    (ElectricDataType::Current, ElectricDataType::Current) |
                    (ElectricDataType::Power, ElectricDataType::Power) |
                    (ElectricDataType::PhaseInfo, ElectricDataType::PhaseInfo))
            }
        }
    }

    // 更新节点依赖关系
    fn update_dependencies(&mut self) {
        // 实现节点依赖关系图的更新
        // 这对于确定计算顺序很重要
    }

    // 使受影响的节点计算结果失效
    fn invalidate_affected_nodes(&mut self, start_node_id: egui_node_graph::NodeId) {
        // 清除当前节点及所有下游节点的计算结果缓存
        let affected_nodes = self.find_downstream_nodes(start_node_id);

        for node_id in affected_nodes {
            // 清除与该节点相关的所有缓存项
            let cache_keys: Vec<String> = self.calculation_cache.keys()
                .filter(|key| key.contains(&node_id.0.to_string()))
                .cloned()
                .collect();

            for key in cache_keys {
                self.calculation_cache.remove(&key);
            }
        }
    }

    // 查找下游节点
    fn find_downstream_nodes(&self, start_node_id: egui_node_graph::NodeId) -> Vec<egui_node_graph::NodeId> {
        let mut downstream = Vec::new();
        let mut visited = std::collections::HashSet::new();

        self.find_downstream_recursive(start_node_id, &mut downstream, &mut visited);
        downstream
    }

    // 递归查找下游节点
    fn find_downstream_recursive(&self, node_id: egui_node_graph::NodeId, downstream: &mut Vec<egui_node_graph::NodeId>,
                               visited: &mut std::collections::HashSet<egui_node_graph::NodeId>) {
        if visited.contains(&node_id) {
            return;
        }

        visited.insert(node_id);

        // 找到所有使用此节点输出的下游节点
        for connection in &self.graph.connections {
            if connection.output.node_id == node_id {
                let target_node_id = connection.input.node_id;
                downstream.push(target_node_id);
                self.find_downstream_recursive(target_node_id, downstream, visited);
            }
        }
    }

    // 查找参数所属的节点
    fn find_node_by_param_id(&self, param_id: egui_node_graph::ParamId) -> Option<egui_node_graph::NodeId> {
        // 遍历所有节点查找参数ID
        for (node_id, node) in &self.graph.nodes {
            // 检查输入参数
            for (input_id, _) in &node.inputs {
                if input_id == &param_id {
                    return Some(*node_id);
                }
            }
            // 检查输出参数
            for (output_id, _) in &node.outputs {
                if output_id == &param_id {
                    return Some(*node_id);
                }
            }
        }
        None
    }

    // 打开节点属性编辑对话框
    fn open_node_properties_dialog(&mut self, node_id: egui_node_graph::NodeId) {
        // 这里可以实现节点属性对话框
        // 例如编辑节点参数、调整计算选项等
    }

    // 节点添加后的处理
    fn on_node_added(&mut self, node_id: egui_node_graph::NodeId, template: &ElectricNodeTemplate) {
        // 根据节点类型初始化默认值或连接
        match template {
            ElectricNodeTemplate::CircuitNode => self.initialize_circuit_node(node_id),
            ElectricNodeTemplate::DistributionBoxNode => self.initialize_distribution_box_node(node_id),
            ElectricNodeTemplate::PowerSourceNode => self.initialize_power_source_node(node_id),
            _ => {}
        }
    }

    // 初始化配电回路节点
    fn initialize_circuit_node(&mut self, node_id: egui_node_graph::NodeId) {
        // 设置默认参数值
    }

    // 初始化配电箱节点
    fn initialize_distribution_box_node(&mut self, node_id: egui_node_graph::NodeId) {
        // 设置默认参数值和三相平衡信息
    }

    // 初始化电源节点
    fn initialize_power_source_node(&mut self, node_id: egui_node_graph::NodeId) {
        // 设置默认电压、频率等值
    }

    // 节点删除后的清理
    fn on_node_removed(&mut self, node_id: egui_node_graph::NodeId) {
        // 清理相关计算缓存
        self.invalidate_affected_nodes(node_id);

        // 清理选中状态
        if let Some(pos) = self.selected_nodes.iter().position(|&id| id == node_id) {
            self.selected_nodes.remove(pos);
        }
    }
}
```

### 4.3 实现图的执行引擎

这是一个复杂但关键的部分，实现图的实际执行逻辑：

```rust
impl MyNodeGraphApp {
    // 构建拓扑排序以确定节点执行顺序
    fn topological_sort(&self) -> Vec<egui_node_graph::NodeId> {
        let mut visited = std::collections::HashSet::new();
        let mut result = Vec::new();

        // 从没有输入的节点开始（通常是输入节点）
        for node_id in self.graph.nodes.keys() {
            if !visited.contains(&node_id) && self.is_node_independent(node_id) {
                self.dfs_visit(node_id, &mut visited, &mut result);
            }
        }

        // 确保所有节点都被访问
        for node_id in self.graph.nodes.keys() {
            if !visited.contains(&node_id) {
                self.dfs_visit(node_id, &mut visited, &mut result);
            }
        }

        result
    }

    // 深度优先搜索访问节点
    fn dfs_visit(
        &self,
        node_id: egui_node_graph::NodeId,
        visited: &mut std::collections::HashSet<egui_node_graph::NodeId>,
        result: &mut Vec<egui_node_graph::NodeId>,
    ) {
        visited.insert(node_id);

        // 找到所有依赖当前节点的节点
        for (_, output_id) in &self.graph.nodes[node_id].outputs {
            // 找到连接到这个输出的所有输入
            for (input_id, connected_output_id) in &self.graph.connections {
                if connected_output_id == output_id {
                    // 找到输入所属的节点
                    let input_node_id = self.graph.inputs[input_id].node;
                    if !visited.contains(&input_node_id) {
                        self.dfs_visit(input_node_id, visited, result);
                    }
                }
            }
        }

        // 将当前节点添加到结果中（后序遍历确保依赖节点先执行）
        result.push(node_id);
    }

    // 判断节点是否没有依赖（独立节点）
    fn is_node_independent(&self, node_id: egui_node_graph::NodeId) -> bool {
        // 检查节点的所有输入参数
        for (_, input_id) in &self.graph.nodes[node_id].inputs {
            // 如果有任何一个输入被连接，则节点不是独立的
            if self.graph.connections.contains_key(input_id) {
                return false;
            }
        }
        true
    }

    // 获取节点输入参数的值
    fn get_input_value(
        &self,
        input_id: egui_node_graph::InputId,
    ) -> Option<MyAppValueType> {
        // 首先检查是否有连接
        if let Some(output_id) = self.graph.connections.get(input_id) {
            // 如果有连接，需要找到对应的输出节点并获取其计算结果
            // 这里简化处理，实际应用中应该缓存计算结果
            let output_param = &self.graph.outputs[*output_id];

            // 找到输出所属的节点
            let output_node_id = self.find_node_by_output(*output_id)?;
            let node_data = &self.graph.nodes[output_node_id];

            // 根据节点类型和输出索引计算结果
            // 这是一个简化的示例，实际应用中应该有更复杂的逻辑
            // let result = self.calculate_node_output(output_node_id, output_id);
            // return Some(result);

            // 由于计算可能很复杂，这里返回一个默认值作为示例
            return match output_param.typ {
                MyAppDataType::Float => Some(MyAppValueType::Float(0.0)),
                MyAppDataType::Integer => Some(MyAppValueType::Integer(0)),
                MyAppDataType::Boolean => Some(MyAppValueType::Boolean(false)),
                _ => None,
            };
        } else {
            // 如果没有连接，返回输入参数的默认值
            let input_param = &self.graph.inputs[input_id];
            match input_param.kind {
                egui_node_graph::InputParamKind::Constant |
                egui_node_graph::InputParamKind::ConnectionOrConstant => {
                    Some(input_param.value.clone())
                },
                _ => None,
            }
        }
    }

    // 根据输出ID查找对应的节点
    fn find_node_by_output(&self, output_id: egui_node_graph::OutputId) -> Option<egui_node_graph::NodeId> {
        for (node_id, node) in &self.graph.nodes {
            for (_, out_id) in &node.outputs {
                if *out_id == output_id {
                    return Some(node_id);
                }
            }
        }
        None
    }
}
```

## 5. 进阶定制与扩展

### 5.1 自定义连接样式

你可以通过扩展或修改库的 UI 渲染逻辑来自定义连接线条的样式。一种方法是创建一个自定义的连接绘制函数：

```rust
fn draw_custom_connection(
    ui: &mut egui::Ui,
    response: &egui::Response,
    start: egui::Pos2,
    end: egui::Pos2,
    color: egui::Color32,
    thickness: f32,
) {
    // 实现自定义的连接线绘制逻辑
    // 例如绘制带箭头的曲线

    // 获取绘图上下文
    let painter = ui.painter_at(response.rect);

    // 计算控制点以创建平滑曲线
    let mid_point = (start + end) * 0.5;
    let control_point1 = egui::Pos2::new(mid_point.x - 50.0, mid_point.y);
    let control_point2 = egui::Pos2::new(mid_point.x + 50.0, mid_point.y);

    // 绘制贝塞尔曲线
    painter.add(egui::Shape::Path(egui::PathShape {
        points: vec![start, control_point1, control_point2, end],
        closed: false,
        fill: egui::Color32::TRANSPARENT,
        stroke: egui::Stroke::new(thickness, color),
    }));

    // 绘制箭头
    // ...
}
```

### 5.2 添加节点分组和注释功能

你可以扩展节点图以支持节点分组和注释功能：

```rust
// 定义分组数据结构
struct NodeGroup {
    id: GroupId,
    label: String,
    color: egui::Color32,
    rect: egui::Rect,
    node_ids: Vec<egui_node_graph::NodeId>,
}

// 定义注释数据结构
struct Annotation {
    id: AnnotationId,
    text: String,
    position: egui::Pos2,
    size: egui::Vec2,
}

// 在应用状态中添加分组和注释支持
struct EnhancedNodeGraphApp {
    // 原有的字段
    graph: egui_node_graph::Graph<MyAppNodeData, MyAppDataType, MyAppValueType>,
    editor_state: GraphEditorState,

    // 新增的字段
    groups: Vec<NodeGroup>,
    annotations: Vec<Annotation>,
}

// 在UI中渲染分组和注释
fn draw_groups_and_annotations(
    ui: &mut egui::Ui,
    response: &egui::Response,
    groups: &[NodeGroup],
    annotations: &[Annotation],
) {
    let painter = ui.painter_at(response.rect);

    // 绘制分组背景
    for group in groups {
        painter.rect(
            group.rect,
            8.0, // 圆角半径
            group.color,
            egui::Stroke::new(1.0, egui::Color32::WHITE),
        );
        painter.text(
            group.rect.min,
            egui::Align2::LEFT_TOP,
            &group.label,
            egui::FontId::monospace(12.0),
            egui::Color32::WHITE,
        );
    }

    // 绘制注释
    for annotation in annotations {
        let rect = egui::Rect::from_min_size(annotation.position, annotation.size);
        painter.rect(
            rect,
            4.0, // 圆角半径
            egui::Color32::from_rgba_premultiplied(255, 255, 255, 200),
            egui::Stroke::new(1.0, egui::Color32::BLACK),
        );
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            &annotation.text,
            egui::FontId::monospace(12.0),
            egui::Color32::BLACK,
        );
    }
}
```

### 5.3 实现节点搜索和自动完成

你可以增强节点查找器，添加更强大的搜索和自动完成功能：

```rust
fn enhanced_node_finder(
    ui: &mut egui::Ui,
    templates: &[MyAppNodeTemplate],
    search_query: &mut String,
    selected_template: &mut Option<MyAppNodeTemplate>,
) -> bool {
    // 绘制搜索框
    ui.horizontal(|ui| {
        ui.label("Search nodes:");
        ui.text_edit_singleline(search_query);
    });

    // 过滤模板
    let filtered_templates: Vec<_> = templates
        .iter()
        .filter(|template| {
            template.node_label().to_lowercase().contains(&search_query.to_lowercase()) ||
            template.category().to_lowercase().contains(&search_query.to_lowercase())
        })
        .collect();

    // 按类别分组显示
    let mut categories = std::collections::HashMap::new();
    for template in &filtered_templates {
        categories
            .entry(template.category())
            .or_insert_with(Vec::new)
            .push(*template);
    }

    // 显示分类后的节点列表
    let mut selected = false;
    for (category, category_templates) in categories {
        ui.collapsing(category, |ui| {
            for template in category_templates {
                if ui.selectable_label(
                    selected_template == Some(template),
                    &template.node_label(),
                ).clicked() {
                    *selected_template = Some(template);
                    selected = true;
                }
            }
        });
    }

    selected
}
```

## 6. 实际业务逻辑集成示例

### 6.1 计算图应用示例

以下是如何将上述框架应用于具体业务场景的示例。假设我们要构建一个简单的数学计算图应用：

```rust
// 扩展MyAppNodeData以支持更多计算节点类型
#[derive(Debug, Clone, PartialEq)]
enum MathOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Sin,
    Cos,
    Tan,
    Log,
    Exp,
    // 添加更多数学函数...
}

// 实现计算逻辑
fn calculate_math_operation(op: MathOperation, inputs: &[f64]) -> Option<f64> {
    match (op, inputs) {
        (MathOperation::Add, [a, b]) => Some(a + b),
        (MathOperation::Subtract, [a, b]) => Some(a - b),
        (MathOperation::Multiply, [a, b]) => Some(a * b),
        (MathOperation::Divide, [a, b]) if *b != 0.0 => Some(a / b),
        (MathOperation::Sin, [x]) => Some(x.sin()),
        (MathOperation::Cos, [x]) => Some(x.cos()),
        (MathOperation::Tan, [x]) => Some(x.tan()),
        (MathOperation::Log, [x]) if *x > 0.0 => Some(x.ln()),
        (MathOperation::Exp, [x]) => Some(x.exp()),
        _ => None,
    }
}

// 扩展MyNodeGraphApp的execute_node方法
impl MyNodeGraphApp {
    fn execute_node(&mut self, node_id: egui_node_graph::NodeId) -> Option<MyAppValueType> {
        let node = &self.graph.nodes[node_id];
        let node_data = &node.user_data;

        match node_data.node_type {
            MyAppNodeType::Math => {
                // 收集输入值
                let mut input_values = Vec::new();
                for (_, input_id) in &node.inputs {
                    if let Some(value) = self.get_input_value(*input_id) {
                        if let MyAppValueType::Float(val) = value {
                            input_values.push(val);
                        }
                    }
                }

                // 执行计算
                if let Some(result) = calculate_math_operation(node_data.math_op.clone(), &input_values) {
                    // 存储结果以便输出端口使用
                    // 这里简化处理，实际应用中应该使用缓存
                    return Some(MyAppValueType::Float(result));
                }
            },
            MyAppNodeType::Input => {
                // 对于输入节点，直接返回参数值
                if let Some((_, input_id)) = &node.inputs.first() {
                    if let Some(value) = self.get_input_value(*input_id) {
                        return Some(value);
                    }
                }
            },
            // 处理其他类型的节点...
            _ => {},
        }

        None
    }
}
```

### 6.2 图像处理节点图示例

以下是一个简化的图像处理节点图应用示例：

```rust
// 定义图像处理相关的数据类型
#[derive(Debug, Clone, PartialEq)]
enum ImageOperation {
    Blur,
    Sharpen,
    Brightness,
    Contrast,
    Saturation,
    // 添加更多图像处理操作...
}

// 定义图像数据类型
struct ImageData {
    width: usize,
    height: usize,
    pixels: Vec<egui::Color32>,
}

// 实现图像处理逻辑
fn process_image(
    op: ImageOperation,
    image: &ImageData,
    params: &[f64],
) -> ImageData {
    // 实现各种图像处理操作
    // ...

    // 返回处理后的图像
    image.clone() // 简化处理，返回原图
}

// 扩展MyAppDataType以支持图像类型
#[derive(Debug, Clone, PartialEq, Eq)]
enum EnhancedDataType {
    // 原有类型...
    Image,
}

// 在应用中集成图像处理功能
struct ImageProcessingApp {
    // 基础图数据和状态
    graph: egui_node_graph::Graph<EnhancedNodeData, EnhancedDataType, EnhancedValueType>,
    editor_state: GraphEditorState,

    // 图像处理特定数据
    images: std::collections::HashMap<egui_node_graph::NodeId, ImageData>,
    loaded_images: std::collections::HashMap<String, ImageData>,
}
```

## 7. 调试与优化

### 7.1 添加调试功能

在开发过程中，添加调试功能可以帮助你更好地理解图的执行和数据流：

```rust
// 添加调试日志
fn log_node_execution(node_id: egui_node_graph::NodeId, inputs: &[MyAppValueType], result: Option<MyAppValueType>) {
    println!("Executing node: {:?}", node_id);
    println!("Inputs: {:?}", inputs);
    println!("Result: {:?}", result);
    println!("------------------------");
}

// 在执行节点时添加日志
impl MyNodeGraphApp {
    fn execute_node_with_debug(&mut self, node_id: egui_node_graph::NodeId) -> Option<MyAppValueType> {
        let node = &self.graph.nodes[node_id];

        // 收集输入值用于日志
        let mut input_values = Vec::new();
        for (_, input_id) in &node.inputs {
            if let Some(value) = self.get_input_value(*input_id) {
                input_values.push(value);
            }
        }

        // 执行节点
        let result = self.execute_node(node_id);

        // 记录日志
        log_node_execution(node_id, &input_values, result.clone());

        result
    }
}
```

### 7.2 性能优化技巧

对于大型节点图，性能可能是一个问题。以下是一些优化技巧：

1. **缓存计算结果**：避免重复计算已经计算过的节点

```rust
struct OptimizedNodeGraphApp {
    // 原有字段...

    // 计算结果缓存
    result_cache: std::collections::HashMap<egui_node_graph::OutputId, MyAppValueType>,
    // 节点更新标志
    node_needs_update: std::collections::HashSet<egui_node_graph::NodeId>,
}

impl OptimizedNodeGraphApp {
    // 执行图时使用缓存
    fn execute_graph_optimized(&mut self) {
        // 只重新计算需要更新的节点及其依赖
        let mut nodes_to_update = self.node_needs_update.clone();

        // 确定受影响的节点
        self.propagate_update_needs(&mut nodes_to_update);

        // 按拓扑顺序执行需要更新的节点
        let execution_order = self.topological_sort();
        for node_id in execution_order {
            if nodes_to_update.contains(&node_id) {
                self.execute_node_and_cache(node_id);
            }
        }

        // 清除更新标志
        self.node_needs_update.clear();
    }

    // 执行节点并缓存结果
    fn execute_node_and_cache(&mut self, node_id: egui_node_graph::NodeId) {
        let node = &self.graph.nodes[node_id];

        // 执行节点计算
        let node_result = self.execute_node(node_id);

        // 缓存结果到所有输出端口
        if let Some(result) = node_result {
            for (_, output_id) in &node.outputs {
                self.result_cache.insert(*output_id, result.clone());
            }
        }
    }

    // 传播更新需求
    fn propagate_update_needs(&self, nodes_to_update: &mut std::collections::HashSet<egui_node_graph::NodeId>) {
        // 找到所有依赖于需要更新节点的节点
        let mut newly_affected = std::collections::HashSet::new();

        for node_id in nodes_to_update.iter() {
            // 找到所有依赖此节点的节点
            for (_, output_id) in &self.graph.nodes[*node_id].outputs {
                for (input_id, connected_output_id) in &self.graph.connections {
                    if connected_output_id == output_id {
                        let dependent_node_id = self.graph.inputs[*input_id].node;
                        if !nodes_to_update.contains(&dependent_node_id) {
                            newly_affected.insert(dependent_node_id);
                        }
                    }
                }
            }
        }

        // 添加新发现的受影响节点并递归传播
        for node_id in newly_affected {
            nodes_to_update.insert(node_id);
        }

        if !newly_affected.is_empty() {
            self.propagate_update_needs(nodes_to_update);
        }
    }

    // 当节点或连接发生变化时，标记节点需要更新
    fn mark_node_for_update(&mut self, node_id: egui_node_graph::NodeId) {
        self.node_needs_update.insert(node_id);
        // 清除该节点的缓存结果
        let node = &self.graph.nodes[node_id];
        for (_, output_id) in &node.outputs {
            self.result_cache.remove(output_id);
        }
    }
}
```

2. **虚拟滚动**：对于包含大量节点的图，实现虚拟滚动以减少渲染开销
3. **延迟渲染**：仅在视口中可见的区域渲染节点和连接
4. **并行计算**：对于独立节点的计算，可以使用并行处理提高性能

## 8. 建筑电气配电系统图设计工具实现

本节将根据专利文档《一种基于节点编辑器的建筑电气配电系统图的高效设计工具》，详细介绍如何在 egui_node_graph 基础上实现建筑电气配电系统图的高效设计功能。

### 8.1 配电回路节点实现

配电回路节点是系统的基本构成单元，分为单相和三相两种类型。以下是具体实现：

```rust
// 定义配电回路类型
enum CircuitType {
    SinglePhase,
    ThreePhase,
}

// 定义回路用途
enum CircuitPurpose {
    Lighting,
    Power,
    HVAC,
    Special,
    Custom(String),
}

// 定义配电回路节点数据结构
#[derive(Debug, Clone)]
struct CircuitNodeData {
    node_type: MyAppNodeType,
    circuit_type: CircuitType,
    power: f64,            // 功率值(kW)
    voltage: f64,          // 电压值(V)
    current: f64,          // 计算电流值(A)
    current_1_1x: f64,     // 1.1倍计算电流
    current_1_25x: f64,    // 1.25倍计算电流
    purpose: CircuitPurpose,
    component_type: String, // 元器件类型
    component_current: f64, // 元器件电流整定值
    cable_spec: String,    // 线缆规格
    phase: Option<char>,   // 相序 (L1, L2, L3)
    circuit_number: u32,   // 回路编号
}

impl CircuitNodeData {
    // 自动计算电流
    fn calculate_current(&mut self) {
        match self.circuit_type {
            CircuitType::SinglePhase => {
                // 单相电流计算公式: I = P / (U * cosφ)
                // 这里简化处理，假设功率因数为0.85
                self.current = self.power * 1000.0 / (self.voltage * 0.85);
            },
            CircuitType::ThreePhase => {
                // 三相电流计算公式: I = P / (√3 * U * cosφ)
                // 这里简化处理，假设功率因数为0.85
                self.current = self.power * 1000.0 / (1.732 * self.voltage * 0.85);
            },
        }

        // 计算1.1倍和1.25倍电流值
        self.current_1_1x = self.current * 1.1;
        self.current_1_25x = self.current * 1.25;
    }

    // 智能选型
    fn select_components(&mut self) {
        // 1. 选择元器件
        self.component_current = self.select_component_current();
        self.component_type = self.select_component_type();

        // 2. 选择线缆规格
        self.cable_spec = self.select_cable_spec();
    }

    // 根据计算电流选择元器件电流整定值
    fn select_component_current(&self) -> f64 {
        // 根据电流查询内置选型表，选择合适的整定值
        // 这里简化处理，实际应用中应查询完整的选型表
        let standard_values = [1, 2, 4, 6, 10, 16, 20, 25, 32, 40, 50, 63, 80, 100, 125];

        for &value in &standard_values {
            if value as f64 >= self.current_1_1x {
                return value as f64;
            }
        }

        125.0 // 默认值
    }

    // 根据回路用途选择元器件类型
    fn select_component_type(&self) -> String {
        match self.purpose {
            CircuitPurpose::Lighting => "微型断路器(照明)".to_string(),
            CircuitPurpose::Power => "微型断路器(动力)".to_string(),
            CircuitPurpose::HVAC => "剩余电流保护器".to_string(),
            CircuitPurpose::Special => "专用保护电器".to_string(),
            CircuitPurpose::Custom(ref s) => format!("自定义({})
", s),
        }
    }

    // 选择线缆规格
    fn select_cable_spec(&self) -> String {
        // 根据电流选择线缆规格，简化处理
        if self.current <= 6.0 {
            "BV-2.5mm²".to_string()
        } else if self.current <= 10.0 {
            "BV-4mm²".to_string()
        } else if self.current <= 16.0 {
            "BV-6mm²".to_string()
        } else if self.current <= 25.0 {
            "BV-10mm²".to_string()
        } else {
            "BV-16mm²".to_string()
        }
    }
}
```

### 8.2 配电箱节点实现

配电箱节点负责管理多个配电回路节点，实现自动编号和三相平衡功能：

```rust
// 定义配电箱节点数据结构
#[derive(Debug, Clone)]
struct DistributionBoxNodeData {
    node_type: MyAppNodeType,
    name: String,          // 配电箱名称
    total_power: f64,      // 总功率
    total_current: f64,    // 总电流
    incoming_current: f64, // 进线保护设备电流整定值
    floor: u32,            // 所在楼层
    modules: Vec<String>,  // 包含的模块 (能耗监测、电气火灾监测等)
    phase_loads: [f64; 3], // L1, L2, L3各相负载
}

impl DistributionBoxNodeData {
    // 自动编号功能
    fn auto_number_circuits(&self, circuit_nodes: &mut [CircuitNodeData]) {
        // 按功率大小或其他规则对回路进行排序
        // 然后依次分配编号
        let mut sorted_indices: Vec<usize> = (0..circuit_nodes.len()).collect();
        sorted_indices.sort_by(|&i, &j| {
            circuit_nodes[i].power.partial_cmp(&circuit_nodes[j].power).unwrap_or(std::cmp::Ordering::Equal)
        });

        // 分配编号
        for (idx, &original_idx) in sorted_indices.iter().enumerate() {
            circuit_nodes[original_idx].circuit_number = (idx + 1) as u32;
        }
    }

    // 三相平衡算法
    fn balance_three_phases(&mut self, circuit_nodes: &mut [CircuitNodeData]) {
        // 1. 初始化分配：将回路按顺序分配到各相
        let phases = ['L', '1', '2', '3'];
        for (i, circuit) in circuit_nodes.iter_mut().enumerate() {
            let phase_idx = i % 3;
            circuit.phase = Some(phases[phase_idx + 1]);
            self.phase_loads[phase_idx] += circuit.power;
        }

        // 2. 迭代优化平衡
        let mut iterations = 0;
        let max_iterations = 100;
        let tolerance = 0.01; // 允许的不平衡度

        while iterations < max_iterations {
            let total_power: f64 = self.phase_loads.iter().sum();
            let avg_power = total_power / 3.0;

            // 检查是否已平衡
            let max_diff = self.phase_loads.iter().map(|&p| (p - avg_power).abs()).fold(0.0, f64::max);
            if max_diff / avg_power < tolerance {
                break;
            }

            // 找到负载最大的相和最小的相
            let mut max_phase = 0;
            let mut min_phase = 0;

            for i in 1..3 {
                if self.phase_loads[i] > self.phase_loads[max_phase] {
                    max_phase = i;
                }
                if self.phase_loads[i] < self.phase_loads[min_phase] {
                    min_phase = i;
                }
            }

            // 从最大负载相转移一个回路到最小负载相
            if let Some((circuit_idx, _)) = circuit_nodes
                .iter()
                .enumerate()
                .filter(|(_, c)| c.phase == Some(phases[max_phase + 1]))
                .find(|(_, c)| {
                    // 尝试转移后，最大负载相仍大于最小负载相
                    let new_max = self.phase_loads[max_phase] - c.power;
                    let new_min = self.phase_loads[min_phase] + c.power;
                    new_max >= new_min
                })
                .next()
            {
                let circuit = &mut circuit_nodes[circuit_idx];
                self.phase_loads[max_phase] -= circuit.power;
                self.phase_loads[min_phase] += circuit.power;
                circuit.phase = Some(phases[min_phase + 1]);
            }

            iterations += 1;
        }
    }

    // 计算总功率和电流
    fn calculate_total_power(&mut self, circuit_nodes: &[CircuitNodeData]) {
        self.total_power = circuit_nodes.iter().map(|c| c.power).sum();

        // 计算总电流 (假设三相380V，功率因数0.85)
        self.total_current = self.total_power * 1000.0 / (1.732 * 380.0 * 0.85);

        // 确定进线保护设备电流整定值
        self.calculate_incoming_current();
    }

    // 计算进线保护设备电流整定值
    fn calculate_incoming_current(&mut self) {
        // 根据总电流和规范要求确定进线保护设备电流整定值
        let standard_values = [100, 125, 160, 200, 250, 315, 400, 500, 630];

        // 进线电流通常考虑1.2倍的安全系数
        let required_current = self.total_current * 1.2;

        for &value in &standard_values {
            if value as f64 >= required_current {
                self.incoming_current = value as f64;
                return;
            }
        }

        self.incoming_current = 630.0; // 默认值
    }
}
```

### 8.3 干线系统图节点实现

干线系统图节点负责自动生成配电干线图和各种监测系统图：

```rust
// 定义干线系统图类型
enum MainSystemType {
    PowerDistribution,     // 配电干线图
    EnergyMonitoring,      // 能耗监测干线图
    ElectricalFireMonitoring, // 电气火灾监控干线图
    FirePowerMonitoring,   // 消防电源监测干线图
}

// 定义干线系统图节点数据结构
#[derive(Debug, Clone)]
struct MainSystemNodeData {
    node_type: MyAppNodeType,
    systems: Vec<MainSystemType>, // 包含的系统图类型
    auto_layout: bool,           // 是否自动布局
}

impl MainSystemNodeData {
    // 自动映射功能
    fn auto_map_distribution_boxes(&self, boxes: &[DistributionBoxNodeData]) -> Vec<SystemDiagram> {
        let mut diagrams = Vec::new();

        // 按楼层对配电箱进行分组
        let mut boxes_by_floor: std::collections::BTreeMap<u32, Vec<&DistributionBoxNodeData>> = std::collections::BTreeMap::new();
        for box_node in boxes {
            boxes_by_floor.entry(box_node.floor).or_default().push(box_node);
        }

        // 生成各类系统图
        for &system_type in &self.systems {
            match system_type {
                MainSystemType::PowerDistribution => {
                    let diagram = self.generate_power_distribution_diagram(&boxes_by_floor);
                    diagrams.push(diagram);
                },
                MainSystemType::EnergyMonitoring => {
                    // 只包含有能耗监测模块的配电箱
                    let filtered_boxes: std::collections::BTreeMap<u32, Vec<&DistributionBoxNodeData>> = boxes_by_floor
                        .iter()
                        .filter(|(_, floor_boxes)| {
                            floor_boxes.iter().any(|b| b.modules.contains(&"能耗监测".to_string()))
                        })
                        .map(|(&floor, boxes)| (floor, boxes.clone()))
                        .collect();

                    let diagram = self.generate_energy_monitoring_diagram(&filtered_boxes);
                    diagrams.push(diagram);
                },
                MainSystemType::ElectricalFireMonitoring => {
                    // 类似能耗监测图生成...
                },
                MainSystemType::FirePowerMonitoring => {
                    // 类似能耗监测图生成...
                },
            }
        }

        diagrams
    }

    // 生成配电干线系统图
    fn generate_power_distribution_diagram(&self, boxes_by_floor: &std::collections::BTreeMap<u32, Vec<&DistributionBoxNodeData>>) -> SystemDiagram {
        let mut diagram = SystemDiagram::new("配电干线系统图".to_string());

        // 创建母线
        let busbar = diagram.add_component(ComponentType::Busbar, "主母线".to_string());

        // 按楼层自下而上添加配电箱
        let mut last_component = busbar;
        for (floor, boxes) in boxes_by_floor.iter().rev() {
            for box_node in boxes {
                let box_component = diagram.add_component(
                    ComponentType::DistributionBox,
                    format!("{}\n楼层:{}\n功率:{:.2}kW\n电流:{:.2}A",
                            box_node.name, floor, box_node.total_power, box_node.incoming_current)
                );

                // 添加连线
                diagram.add_connection(last_component, box_component);

                // 判断进线类型并生成相应的连线
                let connection_type = if box_node.modules.contains(&"双电源切换".to_string()) {
                    ConnectionType::DualPower
                } else {
                    ConnectionType::SinglePower
                };

                diagram.set_connection_type(last_component, box_component, connection_type);
            }
        }

        if self.auto_layout {
            self.auto_layout_diagram(&mut diagram);
        }

        diagram
    }

    // 自动布局算法
    fn auto_layout_diagram(&self, diagram: &mut SystemDiagram) {
        // 实现干线系统图的自动布局算法
        // 这里简化处理，实际应用中应根据图论算法进行优化布局
    }
}

// 系统图数据结构
struct SystemDiagram {
    name: String,
    components: Vec<DiagramComponent>,
    connections: Vec<DiagramConnection>,
}

enum ComponentType {
    Busbar,
    DistributionBox,
    MonitoringModule,
    // 其他组件类型...
}

enum ConnectionType {
    SinglePower,
    DualPower,
    Monitoring,
    // 其他连接类型...
}

struct DiagramComponent {
    id: usize,
    component_type: ComponentType,
    label: String,
    position: (f64, f64),
}

struct DiagramConnection {
    from: usize,
    to: usize,
    connection_type: ConnectionType,
}

impl SystemDiagram {
    fn new(name: String) -> Self {
        Self {
            name,
            components: Vec::new(),
            connections: Vec::new(),
        }
    }

    fn add_component(&mut self, component_type: ComponentType, label: String) -> usize {
        let id = self.components.len();
        self.components.push(DiagramComponent {
            id,
            component_type,
            label,
            position: (0.0, 0.0), // 初始位置，自动布局时会更新
        });
        id
    }

    fn add_connection(&mut self, from: usize, to: usize) {
        self.connections.push(DiagramConnection {
            from,
            to,
            connection_type: ConnectionType::SinglePower, // 默认连接类型
        });
    }

    fn set_connection_type(&mut self, from: usize, to: usize, connection_type: ConnectionType) {
        for connection in &mut self.connections {
            if connection.from == from && connection.to == to {
                connection.connection_type = connection_type;
                break;
            }
        }
    }
}
```

### 8.4 数据流向与实时更新实现

实现节点间的数据流动和实时更新机制：

```rust
// 扩展MyNodeGraphApp以支持数据流向和实时更新
struct PowerDistributionApp {
    // 基础图数据和状态
    graph: egui_node_graph::Graph<PowerNodeData, PowerDataType, PowerValueType>,
    editor_state: GraphEditorState,

    // 组件库
    circuit_nodes: Vec<CircuitNodeData>,
    distribution_boxes: Vec<DistributionBoxNodeData>,
    main_system_nodes: Vec<MainSystemNodeData>,

    // 更新状态
    nodes_to_update: std::collections::HashSet<egui_node_graph::NodeId>,

    // 计算缓存
    calculation_cache: std::collections::HashMap<egui_node_graph::NodeId, PowerValueType>,
}

// 统一的节点数据枚举
#[derive(Debug, Clone)]
enum PowerNodeData {
    Circuit(CircuitNodeData),
    DistributionBox(DistributionBoxNodeData),
    MainSystem(MainSystemNodeData),
}

// 配电系统特有的数据类型
#[derive(Debug, Clone, PartialEq, Eq)]
enum PowerDataType {
    Power,
    Current,
    ComponentSpec,
    CableSpec,
    CircuitInfo,
    DistributionBoxInfo,
    SystemDiagram,
}

// 配电系统特有的值类型
#[derive(Debug, Clone)]
enum PowerValueType {
    Float(f64),
    String(String),
    CircuitInfo(CircuitNodeData),
    DistributionBoxInfo(DistributionBoxNodeData),
    SystemDiagram(SystemDiagram),
}

impl PowerDistributionApp {
    // 处理节点更新和数据流动
    fn propagate_updates(&mut self) {
        // 1. 确定需要更新的节点顺序（拓扑排序）
        let execution_order = self.perform_topological_sort();

        // 2. 按顺序更新节点
        for node_id in execution_order {
            if self.nodes_to_update.contains(&node_id) {
                self.update_node(node_id);

                // 3. 更新受影响的下游节点
                self.mark_downstream_nodes_for_update(node_id);
            }
        }

        // 4. 清除更新标志
        self.nodes_to_update.clear();
    }

    // 更新单个节点
    fn update_node(&mut self, node_id: egui_node_graph::NodeId) {
        let node = &mut self.graph.nodes[node_id];

        match &mut node.user_data {
            PowerNodeData::Circuit(circuit) => {
                // 更新配电回路节点
                circuit.calculate_current();
                circuit.select_components();
            },
            PowerNodeData::DistributionBox(box_node) => {
                // 收集所有连接到该配电箱的回路节点
                let mut connected_circuits = Vec::new();

                for (input_id, connected_output_id) in &self.graph.connections {
                    let input = &self.graph.inputs[*input_id];
                    if input.node == node_id {
                        // 找到输出节点
                        for (output_node_id, node) in &self.graph.nodes {
                            if node.outputs.values().any(|&id| id == *connected_output_id) {
                                if let PowerNodeData::Circuit(circuit) = &node.user_data {
                                    connected_circuits.push(circuit.clone());
                                }
                            }
                        }
                    }
                }

                // 更新配电箱数据
                let mut circuit_data = connected_circuits.iter_mut().collect::<Vec<_>>();
                box_node.calculate_total_power(&connected_circuits);
                box_node.auto_number_circuits(&mut circuit_data);
                box_node.balance_three_phases(&mut circuit_data);

                // 更新回路数据
                for (i, circuit) in connected_circuits.iter().enumerate() {
                    // 这里应该找到对应的节点并更新
                    // 简化处理，实际应用中需要维护节点映射
                }
            },
            PowerNodeData::MainSystem(system_node) => {
                // 收集所有连接的配电箱节点
                let mut connected_boxes = Vec::new();

                for (input_id, connected_output_id) in &self.graph.connections {
                    let input = &self.graph.inputs[*input_id];
                    if input.node == node_id {
                        // 找到输出节点
                        for (output_node_id, node) in &self.graph.nodes {
                            if node.outputs.values().any(|&id| id == *connected_output_id) {
                                if let PowerNodeData::DistributionBox(box_node) = &node.user_data {
                                    connected_boxes.push(box_node.clone());
                                }
                            }
                        }
                    }
                }

                // 生成系统图
                let diagrams = system_node.auto_map_distribution_boxes(&connected_boxes);

                // 存储生成的系统图
                for diagram in diagrams {
                    // 实际应用中应将系统图关联到节点
                }
            },
        }
    }

    // 标记下游节点需要更新
    fn mark_downstream_nodes_for_update(&mut self, node_id: egui_node_graph::NodeId) {
        // 找到所有依赖此节点的下游节点
        for (input_id, connected_output_id) in &self.graph.connections {
            // 检查是否是当前节点的输出
            let node = &self.graph.nodes[node_id];
            if node.outputs.values().any(|&id| id == *connected_output_id) {
                // 找到输入节点
                let input = &self.graph.inputs[*input_id];
                let downstream_node_id = input.node;

                // 标记为需要更新
                self.nodes_to_update.insert(downstream_node_id);

                // 递归标记
                self.mark_downstream_nodes_for_update(downstream_node_id);
            }
        }
    }

    // 执行拓扑排序
    fn perform_topological_sort(&self) -> Vec<egui_node_graph::NodeId> {
        // 实现拓扑排序算法，确保节点按依赖关系顺序执行
        // 简化实现，实际应用中应使用更完善的算法
        let mut visited = std::collections::HashSet::new();
        let mut result = Vec::new();

        // 从无入度的节点开始
        for (node_id, node) in &self.graph.nodes {
            if !visited.contains(node_id) {
                self.dfs_visit(node_id, &mut visited, &mut result);
            }
        }

        result.reverse(); // 反转结果以获得正确的拓扑顺序
        result
    }

    // 深度优先遍历辅助函数
    fn dfs_visit(&self, node_id: &egui_node_graph::NodeId, visited: &mut std::collections::HashSet<egui_node_graph::NodeId>, result: &mut Vec<egui_node_graph::NodeId>) {
        visited.insert(*node_id);

        // 找到所有依赖此节点的下游节点
        for (input_id, connected_output_id) in &self.graph.connections {
            // 检查是否是当前节点的输出
            let node = &self.graph.nodes[*node_id];
            if node.outputs.values().any(|&id| id == *connected_output_id) {
                // 找到输入节点
                let input = &self.graph.inputs[*input_id];
                let downstream_node_id = input.node;

                if !visited.contains(&downstream_node_id) {
                    self.dfs_visit(&downstream_node_id, visited, result);
                }
            }
        }

        result.push(*node_id);
    }

    // 实现WidgetValueTrait以支持自定义控件
    // ...
}
```

### 8.5 交互式用户界面实现

实现直观的用户界面，支持拖拽、连接和实时编辑：

```rust
impl eframe::App for PowerDistributionApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("node_palette").show(ctx, |ui| {
            ui.heading("组件库");

            // 配电回路节点
            ui.collapsing("配电回路", |ui| {
                if ui.button("单相配电回路").clicked() {
                    self.add_circuit_node(CircuitType::SinglePhase);
                }
                if ui.button("三相配电回路").clicked() {
                    self.add_circuit_node(CircuitType::ThreePhase);
                }
            });

            // 配电箱节点
            ui.collapsing("配电箱", |ui| {
                if ui.button("标准配电箱").clicked() {
                    self.add_distribution_box_node();
                }
            });

            // 干线系统图节点
            ui.collapsing("干线系统", |ui| {
                if ui.button("干线系统图").clicked() {
                    self.add_main_system_node();
                }
            });
        });

        // 主编辑区域
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                let mut response = egui_node_graph::draw_graph_editor(
                    ui,
                    &mut self.graph,
                    &mut self.editor_state,
                    &mut PowerDistributionNodeTemplateProvider {},
                    &mut PowerDistributionWidgetValueProvider {},
                );

                // 处理节点选择
                if let Some(selected_node) = response.selected_node {
                    self.show_node_properties(ui, selected_node);
                }

                // 处理连接创建
                if let Some((from, to)) = response.created_connection {
                    self.handle_new_connection(from, to);
                }

                // 处理节点移动
                if let Some(moved_nodes) = response.moved_nodes {
                    for node_id in moved_nodes {
                        // 可以添加特殊处理，如配电回路节点拖入配电箱的逻辑
                        if let Some(target_box) = self.check_if_node_dropped_into_box(node_id) {
                            self.move_circuit_to_box(node_id, target_box);
                        }
                    }
                }

                // 处理节点删除
                if let Some(deleted_node) = response.deleted_node {
                    self.handle_node_deletion(deleted_node);
                }
            });
        });

        // 右侧属性面板
        egui::SidePanel::right("properties").show(ctx, |ui| {
            ui.heading("属性编辑");
            // 属性编辑内容在show_node_properties中处理
        });

        // 底部状态栏
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("节点数: {}", self.graph.nodes.len()));
                ui.label(format!("连接数: {}", self.graph.connections.len()));
            });
        });
    }
}

// 检查节点是否拖入配电箱
impl PowerDistributionApp {
    fn check_if_node_dropped_into_box(&self, node_id: egui_node_graph::NodeId) -> Option<egui_node_graph::NodeId> {
        // 1. 获取被移动节点的位置
        let node_pos = self.editor_state.node_positions.get(&node_id)?;

        // 2. 检查是否在某个配电箱节点范围内
        for (box_id, box_node) in &self.graph.nodes {
            if let Some(box_pos) = self.editor_state.node_positions.get(box_id) {
                if let PowerNodeData::DistributionBox(_) = &box_node.user_data {
                    // 简化处理，实际应用中应检查精确的边界
                    if (node_pos.0 - box_pos.0).abs() < 100.0 &&
                       (node_pos.1 - box_pos.1).abs() < 100.0 {
                        return Some(*box_id);
                    }
                }
            }
        }

        None
    }

    // 移动配电回路到配电箱
    fn move_circuit_to_box(&mut self, circuit_id: egui_node_graph::NodeId, box_id: egui_node_graph::NodeId) {
        // 1. 检查节点类型
        if !matches!(&self.graph.nodes[circuit_id].user_data, PowerNodeData::Circuit(_)) ||
           !matches!(&self.graph.nodes[box_id].user_data, PowerNodeData::DistributionBox(_)) {
            return;
        }

        // 2. 创建连接
        let circuit_output_id = *self.graph.nodes[circuit_id].outputs.values().next()?;
        let box_input_id = *self.graph.nodes[box_id].inputs.values().next()?;

        self.graph.connections.insert(box_input_id, circuit_output_id);

        // 3. 标记节点需要更新
        self.nodes_to_update.insert(box_id);

        // 4. 触发更新传播
        self.propagate_updates();
    }

    // 显示节点属性
    fn show_node_properties(&mut self, ui: &mut egui::Ui, node_id: egui_node_graph::NodeId) {
        if let Some(node) = self.graph.nodes.get_mut(node_id) {
            match &mut node.user_data {
                PowerNodeData::Circuit(circuit) => {
                    ui.heading("配电回路属性");
                    ui.add(egui::Slider::new(&mut circuit.power, 0.1..=100.0).text("功率 (kW)"));
                    if ui.button("更新计算").clicked() {
                        circuit.calculate_current();
                        circuit.select_components();

                        // 标记下游节点需要更新
                        self.nodes_to_update.insert(node_id);
                        self.propagate_updates();
                    }

                    // 显示计算结果
                    ui.group(|ui| {
                        ui.label(format!("计算电流: {:.2} A", circuit.current));
                        ui.label(format!("1.1倍电流: {:.2} A", circuit.current_1_1x));
                        ui.label(format!("1.25倍电流: {:.2} A", circuit.current_1_25x));
                        ui.label(format!("元器件类型: {}", circuit.component_type));
                        ui.label(format!("元器件电流: {:.0} A", circuit.component_current));
                        ui.label(format!("线缆规格: {}", circuit.cable_spec));
                        if let Some(phase) = circuit.phase {
                            ui.label(format!("相序: {}", phase));
                        }
                        ui.label(format!("回路编号: {}", circuit.circuit_number));
                    });
                },
                PowerNodeData::DistributionBox(box_node) => {
                    ui.heading("配电箱属性");
                    ui.text_edit_singleline(&mut box_node.name);
                    ui.add(egui::Slider::new(&mut box_node.floor, 1..=50).text("所在楼层"));

                    // 显示计算结果
                    ui.group(|ui| {
                        ui.label(format!("总功率: {:.2} kW", box_node.total_power));
                        ui.label(format!("总电流: {:.2} A", box_node.total_current));
                        ui.label(format!("进线保护电流: {:.0} A", box_node.incoming_current));
                        ui.label(format!("L1相负载: {:.2} kW", box_node.phase_loads[0]));
                        ui.label(format!("L2相负载: {:.2} kW", box_node.phase_loads[1]));
                        ui.label(format!("L3相负载: {:.2} kW", box_node.phase_loads[2]));
                    });

                    // 三相平衡控制
                    if ui.button("重新平衡三相").clicked() {
                        // 实现重新平衡逻辑
                        self.nodes_to_update.insert(node_id);
                        self.propagate_updates();
                    }
                },
                PowerNodeData::MainSystem(system_node) => {
                    ui.heading("干线系统图属性");
                    ui.checkbox(&mut system_node.auto_layout, "自动布局");

                    // 系统类型选择
                    ui.label("包含系统图:");
                    ui.checkbox(&mut system_node.systems.contains(&MainSystemType::PowerDistribution), "配电干线图");
                    ui.checkbox(&mut system_node.systems.contains(&MainSystemType::EnergyMonitoring), "能耗监测干线图");
                    ui.checkbox(&mut system_node.systems.contains(&MainSystemType::ElectricalFireMonitoring), "电气火灾监控干线图");
                    ui.checkbox(&mut system_node.systems.contains(&MainSystemType::FirePowerMonitoring), "消防电源监测干线图");

                    if ui.button("更新系统图").clicked() {
                        self.nodes_to_update.insert(node_id);
                        self.propagate_updates();
                    }
                },
            }
        }
    }
}
```

### 8.6 实现自动识别与连线生成

为干线系统图节点添加自动识别配电箱进线类型和生成连线的功能：

````rust
impl DistributionBoxNodeData {
    // 判断进线类型
    fn determine_incoming_type(&self) -> IncomingType {
        if self.modules.contains(&"双电源切换".to_string()) {
            IncomingType::DualPower
        } else {
            IncomingType::SinglePower
        }
    }
}

enum IncomingType {
    SinglePower,
    DualPower,
}

impl MainSystemNodeData {
    // 根据进线类型自动生成连线
    fn auto_generate_connections(&self, boxes: &[DistributionBoxNodeData]) -> Vec<ConnectionInfo> {
        let mut connections = Vec::new();

        // 1. 按楼层排序配电箱
        let mut sorted_boxes = boxes.to_vec();
        sorted_boxes.sort_by(|a, b| a.floor.cmp(&b.floor));

        // 2. 生成连线
        let mut previous_boxes: Vec<&DistributionBoxNodeData> = Vec::new();

        for box_node in &sorted_boxes {
            // 确定父配电箱（通常是上一楼层的配电箱）
            if let Some(parent_box) = previous_boxes.iter()
                .filter(|b| b.floor < box_node.floor)
                .max_by_key(|b| b.floor) {

                // 根据进线类型生成连线
                let incoming_type = box_node.determine_incoming_type();

                connections.push(ConnectionInfo {
                    from: parent_box.name.clone(),
                    to: box_node.name.clone(),
                    connection_type: match incoming_type {
                        IncomingType::SinglePower => ConnectionType::SinglePower,
                        IncomingType::DualPower => ConnectionType::DualPower,
                    },
                });

                // 如果是双电源，还需要连接备用电源
                if let IncomingType::DualPower = incoming_type {
                    connections.push(ConnectionInfo {
                        from: "备用电源".to_string(),
                        to: box_node.name.clone(),
                        connection_type: ConnectionType::BackupPower,
                    });
                }
            }

            previous_boxes.push(box_node);
        }

        connections
    }
}

struct ConnectionInfo {
    from: String,
    to: String,
    connection_type: ConnectionType,
}

enum ConnectionType {
    SinglePower,
    DualPower,
    BackupPower,
    // 其他连接类型...
}

## 5. 完整应用示例

以下是一个基于本指南构建的简单电气节点图应用的完整示例代码，展示了如何将所有组件整合在一起：

```rust
use eframe::egui;
use egui_node_graph::{Graph, NodeTemplateTrait, DataTypeTrait, NodeResponse};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// 1. 定义数据类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ElectricDataType {
    Current,    // 电流(A)
    Power,      // 功率(kW)
    Voltage,    // 电压(V)
    PowerFactor,
}

impl DataTypeTrait for ElectricDataType {
    fn data_type_color(&self) -> egui::Color32 {
        match self {
            Self::Current => egui::Color32::RED,
            Self::Power => egui::Color32::GREEN,
            Self::Voltage => egui::Color32::BLUE,
            Self::PowerFactor => egui::Color32::YELLOW,
        }
    }

    fn data_type_name(&self) -> &str {
        match self {
            Self::Current => "电流(A)",
            Self::Power => "功率(kW)",
            Self::Voltage => "电压(V)",
            Self::PowerFactor => "功率因数",
        }
    }
}

// 2. 定义值类型
#[derive(Debug, Clone, PartialEq)]
enum ElectricValueType {
    Float(f64),
    String(String),
}

// 3. 定义节点数据
#[derive(Debug, Clone)]
enum ElectricNodeData {
    CircuitNode { name: String, power: f64, current: f64 },
    DistributionBoxNode { name: String, total_current: f64 },
    PowerSourceNode { name: String, voltage: f64 },
}

// 4. 定义节点模板
#[derive(Debug, Clone, PartialEq)]
enum ElectricNodeTemplate {
    CircuitNode,
    DistributionBoxNode,
    PowerSourceNode,
}

impl NodeTemplateTrait for ElectricNodeTemplate {
    type NodeData = ElectricNodeData;
    type DataType = ElectricDataType;
    type ValueType = ElectricValueType;

    fn node_type_name(&self) -> &str {
        match self {
            Self::CircuitNode => "配电回路",
            Self::DistributionBoxNode => "配电箱",
            Self::PowerSourceNode => "电源",
        }
    }

    fn node_template_category(&self) -> String {
        match self {
            Self::CircuitNode => "配电回路",
            Self::DistributionBoxNode => "配电箱",
            Self::PowerSourceNode => "电源",
        }
    }

    fn node_template_outputs(&self) -> Vec<(String, Self::DataType)> {
        match self {
            Self::CircuitNode => vec![("电流输出".to_string(), Self::DataType::Current)],
            Self::DistributionBoxNode => vec![("总电流".to_string(), Self::DataType::Current)],
            Self::PowerSourceNode => vec![("电压输出".to_string(), Self::DataType::Voltage)],
        }
    }

    fn node_template_inputs(&self) -> Vec<(String, Self::DataType)> {
        match self {
            Self::CircuitNode => vec![("电压输入".to_string(), Self::DataType::Voltage)],
            Self::DistributionBoxNode => vec![],
            Self::PowerSourceNode => vec![],
        }
    }

    fn node_template_params(&self) -> Vec<(String, Self::ValueType)> {
        match self {
            Self::CircuitNode => vec![("额定功率(kW)".to_string(), Self::ValueType::Float(10.0))],
            Self::DistributionBoxNode => vec![],
            Self::PowerSourceNode => vec![("电压值(V)".to_string(), Self::ValueType::Float(220.0))],
        }
    }

    fn construct_node_data(&self, params: &HashMap<String, Self::ValueType>) -> Self::NodeData {
        match self {
            Self::CircuitNode => {
                let power = params.get("额定功率(kW)").and_then(|v| {
                    if let Self::ValueType::Float(f) = v { Some(*f) } else { None }
                }).unwrap_or(10.0);
                ElectricNodeData::CircuitNode { name: "回路1".to_string(), power, current: 0.0 }
            },
            Self::DistributionBoxNode => {
                ElectricNodeData::DistributionBoxNode { name: "配电箱1".to_string(), total_current: 0.0 }
            },
            Self::PowerSourceNode => {
                let voltage = params.get("电压值(V)").and_then(|v| {
                    if let Self::ValueType::Float(f) = v { Some(*f) } else { None }
                }).unwrap_or(220.0);
                ElectricNodeData::PowerSourceNode { name: "电源1".to_string(), voltage }
            },
        }
    }

    fn update_node_data(&self, node_data: &mut Self::NodeData, params: &HashMap<String, Self::ValueType>) {
        // 更新节点数据逻辑
    }

    fn node_label(&self, node_data: &Self::NodeData) -> String {
        match node_data {
            ElectricNodeData::CircuitNode { name, .. } => name.clone(),
            ElectricNodeData::DistributionBoxNode { name, .. } => name.clone(),
            ElectricNodeData::PowerSourceNode { name, .. } => name.clone(),
        }
    }
}

// 5. 定义应用结构体
struct ElectricNodeGraphApp {
    graph: Graph<ElectricNodeData, ElectricDataType, ElectricValueType>,
    editor_state: egui_node_graph::GraphEditorState,
    templates: Vec<ElectricNodeTemplate>,
}

impl Default for ElectricNodeGraphApp {
    fn default() -> Self {
        Self {
            graph: Graph::default(),
            editor_state: egui_node_graph::GraphEditorState::default(),
            templates: vec![
                ElectricNodeTemplate::CircuitNode,
                ElectricNodeTemplate::DistributionBoxNode,
                ElectricNodeTemplate::PowerSourceNode,
            ],
        }
    }
}

// 6. 实现App trait
impl eframe::App for ElectricNodeGraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("电气节点图设计工具");

            // 渲染节点编辑器
            egui_node_graph::draw_graph_editor(
                ui,
                &mut self.editor_state,
                &mut self.graph,
                &self.templates,
                |_, _| {}, // 自定义UI
            );

            // 计算按钮
            if ui.button("运行计算").clicked() {
                self.run_calculations();
            }
        });
    }
}

impl ElectricNodeGraphApp {
    fn run_calculations(&mut self) {
        // 实现简单的计算逻辑
        // 例如：从电源获取电压，计算回路电流，汇总到配电箱

        // 查找所有电源节点和电压值
        let mut voltage_map = HashMap::new();
        for (node_id, node) in &self.graph.nodes {
            if let ElectricNodeData::PowerSourceNode { name, voltage } = &node.data {
                voltage_map.insert(name.clone(), voltage);
            }
        }

        // 计算回路电流并汇总到配电箱
        for (node_id, node) in &mut self.graph.nodes.iter_mut() {
            if let ElectricNodeData::CircuitNode { name, power, current } = node.data.as_mut() {
                // 查找该回路连接的电源电压
                let mut voltage = 220.0; // 默认值
                for output_id in &node.inputs.values() {
                    if let Some(conn) = self.graph.connections.get(output_id) {
                        if let ElectricNodeData::PowerSourceNode { name: source_name, .. } =
                            &self.graph.nodes.get(&conn.from_node).unwrap().data {
                            if let Some(&v) = voltage_map.get(source_name) {
                                voltage = v;
                                break;
                            }
                        }
                    }
                }

                // 计算电流: I = P / U
                *current = *power * 1000.0 / voltage; // 转换为瓦特
            }
        }
    }
}

// 7. 主函数
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        ..Default::default()
    };

    eframe::run_native(
        "电气节点图设计工具",
        options,
        Box::new(|_| Ok(Box::new(ElectricNodeGraphApp::default()))),
    )
}
````

## 6. 依赖配置

在项目的 Cargo.toml 文件中，需要添加以下依赖：

```toml
[package]
name = "electric_node_graph_app"
version = "0.1.0"
edition = "2021"

[dependencies]
eframe = "0.32.3"
egui = "0.32.3"
egui_node_graph = "0.5.0"  # 根据实际版本调整
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.3.0", features = ["serde", "v4"] }
chrono = { version = "0.4.24", features = ["serde"] }
```

## 7. 开发建议

1. **循序渐进**：从基础节点类型开始，逐步添加功能和复杂性
2. **单元测试**：为电气计算逻辑编写全面的单元测试，确保计算准确性
3. **模块化**：严格按照建议的文件夹结构组织代码，保持良好的模块化
4. **文档先行**：在实现复杂功能前，先编写详细的设计文档
5. **性能优化**：对于大型电路图，考虑使用并行计算和缓存机制提高性能

通过遵循本指南，您可以构建一个功能完善、性能优良的电气节点图应用，满足建筑电气配电系统设计的需求。

```

## 9. 总结

通过本指南，你学习了如何详细复用egui_node_graph库构建自己的节点图应用，特别是实现了一个基于专利技术的建筑电气配电系统图高效设计工具。主要实现内容包括：

1. **核心节点类型实现**：
   - 配电回路节点：实现自动计算电流、智能选型、回路用途识别等功能
   - 配电箱节点：实现自动编号、三相平衡算法、数据汇总等功能
   - 干线系统图节点：实现自动映射、智能识别与连线生成等功能

2. **数据流向与实时更新机制**：
   - 实现了节点间的数据流动机制
   - 实现了拓扑排序和依赖解析
   - 实现了自动更新和反馈机制

3. **交互式用户界面**：
   - 实现了直观的拖放和连接操作
   - 实现了实时属性编辑和反馈
   - 实现了特殊的节点交互逻辑（如回路拖入配电箱）

4. **自动识别与连线生成**：
   - 实现了基于配电箱进线类型的自动连线生成
   - 实现了干线系统图的自动布局

这些功能共同构成了一个高效、智能的建筑电气配电系统图设计工具，能够显著提高设计师的工作效率，减少人为错误，确保设计的准确性和合规性。

egui_node_graph库提供了灵活的基础架构，通过实现各种trait和自定义组件，我们能够完全控制节点图的外观和行为，使其适应复杂的建筑电气设计需求。

当你基于本指南实现自己的系统时，建议从简单的节点类型开始，逐步添加功能和复杂性，并根据实际项目需求进行优化和调整。

- 1.
核心数据类型定义 ：

- 定义了完整的ElectricDataType枚举，包含电流、功率、电压等电气参数类型
- 实现了ElectricValueType值类型和ElectricNodeData节点数据结构
- 添加了CircuitNodeData、DistributionBoxNodeData等节点专用数据结构
- 2.
节点模板实现 ：

- 将通用MyAppNodeTemplate替换为电气系统专用ElectricNodeTemplate
- 实现了CategoryTrait和NodeTemplateTrait的电气系统专用版本
- 配置了配电回路、配电箱、干线系统图、电源和计算节点的参数
- 3.
应用结构优化 ：

- 将MyNodeGraphApp重命名为ElectricNodeGraphApp，更新为电气系统专用类型
- 实现了App trait的专用版本，包括项目信息显示、分类节点菜单
- 添加了电气系统连接规则检查、自定义节点UI渲染和计算报告功能
- 4.
辅助功能增强 ：

- 添加了电流计算、三相平衡度计算等电气专用计算函数
- 实现了项目数据结构和报告生成器
- 添加了必要的导入语句和依赖管理
- 5.
文档格式修复 ：

- 修复了文档末尾的多余引号
- 确保了代码块和格式的一致性
```
