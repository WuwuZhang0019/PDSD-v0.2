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

**注：详细的数据类型定义请参考《复合数据类型定义方案》文档，以下为核心内容摘要：**

核心数据类型定义包括：

1. **基础参数类型**：定义了电压等级(VoltageLevel)、敷设方式(LayingMethod)、穿管规格(PipeSpecification)、敷设部位(LayingArea)、相序(PhaseSequence)等枚举类型，以及用于按输出端口连接顺序分配编号的回路编号结构体(CircuitNumber)。

2. **回路数据复合类型(CircuitData)**：包含断路器类型、壳架电流、分断能力、脱扣方式、极数等断路器参数，以及线缆参数、电气参数（额定功率、功率因数、需用系数、计算电流）和回路用途等信息。

3. **配电箱数据复合类型(DistributionBoxData)**：包含配电箱基本信息、电气参数、进线参数、出线回路集合等，支持按输出端口连接顺序自动分配回路编号。

4. **电气元器件类型**：定义了断路器(Breaker)、隔离开关(Isolator)、双电源开关(DualPowerSwitch)、接触器(Contactor)和电能表(EnergyMeter)等电气元器件的详细参数结构。

所有数据类型均实现了序列化和反序列化功能，确保与现有代码保持一致，并提供了丰富的方法用于数据操作和转换。

5. **完整数据类型列表：**

| 数据类型名称 | 中文名称 | 简要描述 |
|------------|---------|--------|
| VoltageLevel | 电压等级 | 定义系统电压等级，如400V/750V等 |
| LayingMethod | 敷设方式 | 定义线缆敷设方式，如焊接钢管、紧定管等 |
| PipeSpecification | 穿管规格 | 定义穿管直径规格，如15mm、20mm等 |
| LayingArea | 敷设部位 | 定义线缆敷设的具体部位，如沿墙暗敷、吊顶内明敷等 |
| PhaseSequence | 相序 | 定义电路相序，如L1、L2、L3、L1L2L3等 |
| CircuitNumber | 回路编号 | 用于按照输出端口连接顺序为回路分配编号 |
| BreakerType | 断路器类型 | 定义各类断路器类型，如框架断路器、塑壳断路器等 |
| FrameCurrent | 壳架电流 | 定义断路器壳架电流等级，如63A、100A等 |
| BreakingCapacity | 分断能力 | 定义断路器分断能力等级，如F(36kA)、N(50kA)等 |
| DeductionMethod | 脱扣方式 | 定义断路器脱扣方式，如热磁型(TM)、单磁型(MA)等 |
| Pole | 极数 | 定义电气设备的极数，如1P、2P、3P、4P、1P+N、3P+N等 |
| Curve | 脱扣曲线 | 定义断路器脱扣曲线类型，如B、C、D等 |
| SettingValue | 整定电流值 | 定义电气设备的整定电流值，如16A、20A等 |
| PhaseConfig | 相位配置 | 定义电气设备的相位配置，如单相、三相、三相四线等 |
| Phase | 相序枚举 | 定义具体的相序，如L1(A相)、L2(B相)、L3(C相)等 |
| ElectricComponent | 电气元器件类型枚举 | 统一表示各类电气元器件，如断路器、隔离开关等 |
| Breaker | 断路器参数 | 定义断路器的详细参数，包含类型、型号、电流等信息 |
| Isolator | 隔离开关参数 | 定义隔离开关的详细参数，包含型号、电流、极数等信息 |
| DualPowerSwitch | 双电源开关参数 | 定义双电源开关的详细参数，包含型号、切换时间等信息 |
| Contactor | 接触器参数 | 定义接触器的详细参数，包含型号、控制电压等信息 |
| EnergyMeter | 电能表参数 | 定义电能表的详细参数，包含型号、精度等级等信息 |
| CableInfo | 线缆参数 | 定义线缆的详细参数，包含型号、规格、敷设方式等信息 |
| CircuitData | 回路数据复合类型 | 定义配电回路的完整数据结构，包含回路信息、元器件列表、电气参数等 |
| DistributionBoxData | 配电箱数据复合类型 | 定义配电箱的完整数据结构，包含基本信息、回路集合、三相负载分布等 |
| ElectricValueType | 电气值类型枚举 | 统一表示系统中各类电气参数值，支持数值、字符串、复合类型等 |

### 2.2 设计数据结构定义（DataStructure）

在电气配电系统中，我们需要定义与电气参数对应的具体值类型，包括电流、功率等基本参数，以及更复杂的配电回路和配电箱数据。

详细的数据结构定义请参考《数据结构定义方案》文档，该文档包含了完整的电气配电系统数据结构设计，包括：

1. 基础数据类型：功率、电流、相序、回路编号等基本参数
2. 配电回路数据结构：包含自动计算电流、智能选型、回路用途识别等功能的数据模型
3. 配电箱数据结构：包含配电回路接口、保护设备、监测模块、三相平衡算法等功能的数据模型
4. 干线系统图数据结构：自动映射、智能识别与连线生成等功能的数据模型

这些数据结构支持动态连接、双向数据流和自动更新功能，确保整个配电系统设计的数据一致性和实时性。

5. 完整数据结构列表：

| 数据结构名称 | 中文名称 | 简要描述 |
|------------|---------|--------|
| CircuitNumber | 回路编号类型 | 支持字母+数字、双字母+数字、配电箱编号+'-'+数字三种形式的回路编号 |
| CircuitPurpose | 回路用途 | 支持手动输入或从外部参数获取的回路用途定义 |
| CircuitPower | 回路功率 | 支持手动输入或从外部配电箱参数获取的功率定义 |
| VoltageLevel | 电压等级 | 定义系统电压等级，如400V/750V、0.6V/1kV等 |
| PhaseSequence | 相序 | 定义电路相序，如L1、L2、L3、L1L2L3等 |
| PhaseLine | 相线截面积规格 | 定义相线的截面积规格，如1.5mm²、2.5mm²等 |
| PE | PE线截面积规格 | 定义PE线的截面积规格 |
| TwinParallelCables | 双根并联电缆 | 定义双根并联电缆的规格 |
| CoresNum | 电缆芯数与配置 | 定义电缆的芯数和配置，如2芯+PE、4芯等 |
| DistributionBoxData | 配电箱数据结构 | 包含配电箱的基本信息、回路连接、功率电流计算、三相平衡等功能 |
| MonitoringModule | 监测模块 | 定义配电箱监测模块的功能配置，如电压、电流、功率监测等 |
| PhaseBalance | 三相平衡信息 | 记录三相平衡的详细数据，包括各相功率、电流和不平衡度 |
| Dimensions | 箱体尺寸 | 定义配电箱的物理尺寸，包括宽度、高度和深度 |
| TrunkSystemData | 干线系统数据结构 | 定义整个配电干线系统，包含配电箱、连接关系和拓扑分析功能 |
| Connection | 连接关系 | 定义设备间的连接信息，包括源设备、目标设备、线缆规格等 |
| SystemType | 系统类型 | 定义配电系统的类型，如放射式、树干式、环式、混合式等 |
| ConnectionType | 连接类型 | 定义连接的类型，如一次侧、二次侧、母线连接、馈线连接等 |
| LoadCategory | 负载分类 | 定义负载的分类信息，包括名称、描述、总功率和负载等级 |
| LoadLevel | 负载等级 | 定义负载的重要性等级，如一级负荷、二级负荷、三级负荷 |
| SystemTopology | 系统拓扑 | 描述配电系统的拓扑结构，包括根节点、叶节点和分支信息 |
| Branch | 分支信息 | 记录系统拓扑中的分支连接信息 |
| TopologyAnalysisResult | 拓扑分析结果 | 包含拓扑分析的详细结果，如配电箱数量、连接数量、根节点等 |
| DataManager | 数据管理器 | 管理整个系统的数据，包括回路、配电箱和干线系统，提供数据一致性维护 |
| DataChangeType | 数据变更类型 | 定义数据变更的类型，如添加、更新、删除等操作类型 |
| DataChange | 数据变更记录 | 记录数据变更的详细信息，包括变更类型、实体ID、时间戳等 |

### 2.3 实现简化参数控件（WidgetValueTrait）

根据egui_node_graph库的标准接口，实现简化版的参数控件，只支持基本数据类型的显示和编辑：

```rust
use egui_node_graph::WidgetValueTrait;

// 定义简化的响应类型
type ElectricResponse = ();

impl WidgetValueTrait for ElectricValueType {
    // 定义必要的关联类型
    type Response = ElectricResponse;
    type UserState = (); // 简化版不使用用户状态
    type NodeData = ();  // 简化版不使用节点数据
    
    // 实现核心方法 - 简化版只处理基本类型
    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut Self::UserState,
        _node_data: &Self::NodeData,
    ) -> Vec<Self::Response> {
        // 根据值类型显示不同的简单控件
        match self {
            // 处理浮点数类型（如电流、功率等）
            ElectricValueType::Float(val) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(egui::DragValue::new(val).clamp_range(0.0..=f64::MAX));
                });
            },
            
            // 处理整数类型
            ElectricValueType::Integer(val) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(egui::DragValue::new(val).clamp_range(0..=i64::MAX));
                });
            },
            
            // 处理字符串类型
            ElectricValueType::String(val) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.text_edit_singleline(val);
                });
            },
            
            // 简化处理复杂类型 - 只显示基本信息
            ElectricValueType::CircuitData(circuit) => {
                ui.label(format!("{}: 回路-{}", param_name, circuit.name));
            },
            
            ElectricValueType::DistributionBoxData(box_data) => {
                ui.label(format!("{}: 配电箱-{}", param_name, box_data.name));
            },
            
            // 其他类型简单显示
            _ => {
                ui.label(format!("{}: {}", param_name, self));
            }
        }
        
        // 返回空的响应列表（简化版不需要副作用处理）
        Vec::new()
    }
    
    // 使用默认的已连接输入控件实现
    // fn value_widget_connected 方法使用默认实现
}

// 为ElectricValueType实现Default特性，这是WidgetValueTrait的要求
impl Default for ElectricValueType {
    fn default() -> Self {
        ElectricValueType::Float(0.0) // 使用浮点数0.0作为默认值
    }
}
```

## 3. 节点数据与模板定义

### 3.1 设计节点数据结构（NodeData）

节点数据结构的详细定义请参考《03UI层数据结构定义方案》文档。该文档包含了完整的UI层数据结构实现，包括：

1. UI层值类型 (UIValueType)：支持浮点数、整数、字符串、回路信息、配电箱信息和三相平衡信息等类型
2. 简化的回路信息结构 (CircuitInfo)：包含回路编号、名称、功率、相序等基本信息
3. 简化的配电箱信息结构 (DistributionBoxInfo)：包含配电箱编号、名称、类型、回路数量等信息
4. 三相平衡信息结构 (PhaseBalanceInfo)：记录各相负载和平衡度
5. 节点数据类型 (ElectricNodeData)：实现了NodeDataTrait接口，支持节点UI自定义和交互
6. 业务层与UI层数据转换机制：提供双向数据转换功能

请查阅《03UI层数据结构定义方案》获取完整实现代码和详细说明。


### 3.2 定义节点模板（NodeTemplate）
"定义节点模板"功能的具体作用是：
1. 标准化节点创建 ：通过 ElectricNodeTemplate 枚举定义各种电气系统节点类型（如配电回路、配电箱、干线等），确保节点创建的一致性和规范性。
2. 节点分类管理 ：实现 CategoryTrait 接口，将节点按功能分类（配电回路、配电箱、干线系统、电源、计算工具等），方便用户在编辑器中查找和使用。
3. 节点属性预配置 ：在 NodeTemplateTrait 实现中，为不同类型的节点预设默认属性值（如名称、尺寸、初始数据等），减少用户手动配置的工作量。
4. 统一节点外观定义 ：为每种节点类型设置标准尺寸和样式，保证界面的一致性和专业性。
5. 输入输出接口标准化 ：通过 build_node 方法，为每种节点类型定义标准的输入输出参数接口，确保节点间数据交互的规范性。
6. 节点行为封装 ：将节点的UI交互逻辑、数据处理方式等封装在模板中，使节点具有统一的行为模式。
7. 模板管理机制 ：提供 all_electric_templates 函数，统一管理所有可用的节点模板，便于扩展和维护。
这种节点模板机制使电气系统图的设计更加高效、规范，同时也为后续功能扩展提供了良好的架构基础。

## 4. 应用集成与实现

### 4.1 创建电气系统应用状态结构体（ElectricNodeGraphApp）

`ElectricNodeGraphApp` 结构体是电气配电系统图设计工具的核心应用状态容器，其功能和作用如下：

#### 核心功能概述
1. **应用状态统一管理**：集中存储和管理整个电气系统图设计应用的所有状态数据，包括图形数据、UI状态、项目信息等
2. **数据持久化支持**：通过封装关键数据结构，支持项目的保存、加载和状态恢复
3. **计算缓存机制**：提供计算结果缓存，优化重复计算的性能
4. **错误状态追踪**：维护错误信息状态，用于向用户显示操作反馈
#### 各字段详细说明
1. **graph**：核心图形数据结构
   - 类型：`egui_node_graph::Graph<ElectricNodeData, ElectricDataType, ElectricValueType>`
   - 作用：存储整个电气系统图的节点、连接和数据流信息，使用专用的电气系统数据类型确保类型安全
2. **editor_state**：编辑器UI状态
   - 类型：`GraphEditorState`
   - 作用：管理节点编辑器的界面状态，包括视图位置、缩放级别、选中的节点等UI相关信息
3. **project_name**：项目名称
   - 类型：`String`
   - 作用：存储当前电气项目的名称，用于标识和展示
4. **project_id**：项目唯一标识符
   - 类型：`Uuid`
   - 作用：为每个项目生成全局唯一标识符，确保数据的唯一性和可追溯性
5. **calculation_cache**：计算结果缓存
   - 类型：`HashMap<String, f64>`
   - 作用：缓存计算结果，避免重复计算，提升应用性能，特别是在复杂的电气计算场景中
6. **error_message**：错误信息
   - 类型：`Option<String>`
   - 作用：存储操作过程中可能出现的错误信息，用于向用户提供反馈
#### Default实现的作用
`Default` trait实现提供了应用状态的初始默认值：
- 创建空的图形数据结构
- 初始化编辑器状态
- 设置默认项目名称
- 生成新的项目UUID
- 创建空的计算缓存
- 初始无错误状态
这种设计使得应用能够从一个已知的、有效的初始状态开始运行，确保系统的稳定性和一致性。
#### 整体架构意义
`ElectricNodeGraphApp`结构体采用了集中式状态管理模式，将复杂的电气系统图设计工具的所有状态封装在单一结构体中，便于状态的传递、更新和持久化。这种设计符合现代GUI应用开发的最佳实践，特别适合节点编辑器这类复杂交互式应用。

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

### 4.2 实现电气系统应用的 UI 和逻辑（update 方法）

本章节实现了电气系统应用的核心交互界面和业务逻辑，主要包括：

1. **应用界面布局**：
   - 实现了应用标题栏和项目信息显示
   - 创建了包含添加节点、保存/加载项目、运行计算、生成报告等功能的工具栏
   - 设计了错误信息提示区域和状态栏
   - 实现了可滚动的节点编辑器主界面

2. **节点操作功能**：
   - 实现了按类别组织的节点模板选择和添加功能
   - 处理节点的添加、删除、双击编辑等交互事件
   - 实现了节点选中状态管理

3. **项目管理功能**：
   - 提供项目数据的JSON序列化保存功能
   - 支持从文件加载项目数据并恢复应用状态
   - 实现了项目设置对话框入口

4. **电气计算逻辑**：
   - 实现基于拓扑排序的节点计算顺序确定
   - 提供针对不同类型节点的专用计算方法
   - 实现计算结果缓存和依赖更新机制
   - 提供三相平衡分析功能

5. **连接管理**：
   - 实现了基于电气规则的连接有效性检查
   - 处理连接添加和删除事件
   - 支持参数值变更的传播机制

6. **报告生成**：
   - 实现系统概览、节点详情、计算结果等多维度报告生成
   - 支持PDF格式报告导出

详细实现代码请参考：《05实现应用的 UI 和逻辑.md》


### 4.3 实现图的执行引擎（topological_sort 方法）

图的执行引擎是电气节点图应用的核心计算组件，负责管理节点之间的依赖关系并确保按照正确顺序执行计算。主要功能包括：

1. **节点执行顺序确定**：
   - 通过`topological_sort`方法实现拓扑排序算法，确保节点按照依赖关系正确执行
   - 先从无依赖的独立节点开始，再处理有依赖的节点，避免计算错误

2. **依赖关系管理**：
   - `is_node_independent`方法检测节点是否独立（无输入连接），为拓扑排序提供起点
   - `dfs_visit`方法通过深度优先搜索遍历整个图结构，确保所有依赖关系被正确处理

3. **数据流动控制**：
   - 实现了基于有向图的节点间数据传输机制
   - `find_node_by_output`方法支持通过输出ID追踪到对应的输出节点，建立数据溯源

4. **参数值获取机制**：
   - `get_input_value`方法智能处理节点输入参数的取值逻辑
   - 优先从连接的上游节点获取计算结果
   - 若无连接则使用参数的默认值
   - 支持不同数据类型（Float、Integer、Boolean等）的处理

5. **计算框架基础**：
   - 提供了计算结果缓存的扩展接口
   - 预留了针对不同节点类型的专用计算方法接口
   - 为复杂电气系统的计算分析提供了可扩展的执行框架

详细实现代码请参考：《06图的执行引擎.md》


## 5. 进阶定制与扩展

### 5.1 自定义连接样式

已完成实现。支持多种连接样式（默认、虚线、粗线、高亮），根据数据类型自动设置不同颜色，并添加箭头指示数据流向。

实现文件：`src/editor/ui/custom_connections.rs`

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

    // 绘制箭头指示流向
    draw_arrowhead(painter, end, start, color, thickness);
}
```

### 5.2 添加节点分组和注释功能

已完成实现。支持创建、编辑、折叠和删除节点分组，自动调整分组大小以包含所有节点。

实现文件：`src/editor/ui/node_groups.rs`

```rust
// 定义分组数据结构
struct NodeGroup {
    id: GroupId,
    label: String,
    color: egui::Color32,
    rect: egui::Rect,
    node_ids: Vec<egui_node_graph::NodeId>,
    is_collapsed: bool,
}

// 分组管理和渲染实现
impl NodeGroupManager {
    fn update_group_rect(&mut self, graph: &Graph<N, E, V>) {
        // 自动调整分组大小以包含所有节点
    }
    
    fn draw_groups(&self, ui: &mut egui::Ui, response: &egui::Response) {
        // 分组渲染逻辑
    }
}
```

### 5.3 实现节点搜索和自动完成

已完成实现。支持模糊搜索节点，可按节点类型、数据类型等条件过滤，支持多种排序方式。

实现文件：`src/editor/ui/node_search.rs`

```rust
fn enhanced_node_finder(
    ui: &mut egui::Ui,
    templates: &[ElectricNodeTemplate],
    search_query: &mut String,
    selected_template: &mut Option<ElectricNodeTemplate>,
    filter_options: &SearchFilterOptions,
) -> bool {
    // 绘制搜索框和过滤选项
    ui.horizontal(|ui| {
        ui.label("搜索节点:");
        ui.text_edit_singleline(search_query);
        ui.menu_button("过滤", |ui| {
            // 过滤选项UI
        });
    });

    // 高级过滤和搜索逻辑
    let filtered_templates = filter_and_search_templates(templates, search_query, filter_options);
    
    // 分类显示和排序
    display_sorted_templates_by_category(ui, &filtered_templates, selected_template)
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

已完成实现。支持多级别日志（错误、警告、信息、调试、追踪），可导出日志文件，集成到UI界面中。

实现文件：`src/editor/ui/debug_tools.rs`

```rust
// 多级别日志系统
pub enum LogLevel { Error, Warning, Info, Debug, Trace }

impl ElectricNodeGraphApp {
    fn log_node_execution(&self, node_id: egui_node_graph::NodeId, inputs: &[ElectricValueType], 
                         result: Option<ElectricValueType>, level: LogLevel) {
        // 根据日志级别记录执行信息
        self.logger.log(level, format!("Executing node: {:?}", node_id));
        
        // 日志UI显示和文件导出功能
    }
    
    // 调试面板UI
    fn show_debug_panel(&mut self, ui: &mut egui::Ui) {
        // 调试信息展示界面
    }
}
```

### 7.2 性能优化技巧

已完成实现。包括视图裁剪、细节层次(LOD)、连接优化、交互节流等技术，大幅提升大规模节点图的性能。

实现文件：`src/editor/ui/performance_optimization.rs`

```rust
struct OptimizedElectricNodeGraphApp {
    // 核心数据结构
    graph: egui_node_graph::Graph<ElectricNodeData, ElectricDataType, ElectricValueType>,
    editor_state: GraphEditorState,
    
    // 性能优化组件
    result_cache: std::collections::HashMap<egui_node_graph::OutputId, ElectricValueType>,
    node_needs_update: std::collections::HashSet<egui_node_graph::NodeId>,
    visible_nodes: std::collections::HashSet<egui_node_graph::NodeId>,
    render_lod_level: u32,
    interaction_throttler: Throttler,
    
    // 性能监控
    performance_metrics: PerformanceMetrics,
}

impl OptimizedElectricNodeGraphApp {
    // 视口裁剪优化
    fn update_visible_nodes(&mut self, view_rect: egui::Rect) {
        // 仅标记视口内可见节点
    }
    
    // 细节层次渲染
    fn render_with_lod(&self, ui: &mut egui::Ui, zoom_level: f32) {
        // 根据缩放级别调整渲染细节
    }
    
    // 并行计算独立节点
    fn execute_independent_nodes_parallel(&mut self, independent_nodes: &[egui_node_graph::NodeId]) {
        // 使用线程池并行计算无依赖节点
    }
    
    // 性能监控和显示
    fn show_performance_panel(&self, ui: &mut egui::Ui) {
        // 显示帧率、渲染时间、更新时间等性能指标
    }
}
```

### 7.3 大规模图的性能优化

已完成实现多种高级优化技术：

1. **视图裁剪**：仅渲染视口内可见的节点和连接
2. **细节层次(LOD)**：根据缩放级别调整节点和连接的渲染细节
3. **连接优化**：使用空间索引加速连接线查询
4. **交互节流**：限制高频操作的处理频率
5. **增量更新**：只处理发生变化的节点和连接
6. **并行计算**：对独立节点使用多线程并行计算

这些优化技术大幅提升了大规模节点图（1000+节点）的性能表现。

## 8. 建筑电气配电系统图设计工具实现

本节将根据专利文档《一种基于节点编辑器的建筑电气配电系统图的高效设计工具》，详细介绍如何在 egui_node_graph 基础上实现建筑电气配电系统图的高效设计功能。

### 8.1 配电回路节点实现

配电回路节点是电气系统图的基本构成单元，支持单相和三相两种工作模式。主要实现内容包括：

1. **类型定义**：
   - `CircuitType`枚举：区分SinglePhase（单相）和ThreePhase（三相）回路
   - `CircuitPurpose`枚举：定义Lighting（照明）、Power（动力）、HVAC（空调）、Special（特殊）等回路用途

2. **数据结构**：`CircuitNodeData`结构体封装了完整的回路参数，包括功率、电压、电流、相序、回路编号、元器件类型及线缆规格等

3. **核心功能**：
   - 自动计算电流：根据回路类型（单相/三相）应用不同计算公式，同时计算1.1倍和1.25倍安全电流值
   - 智能元器件选型：根据计算电流和回路用途自动选择合适的元器件类型和电流整定值
   - 线缆规格选择：基于计算电流推荐适当的线缆规格（BV-2.5mm²至BV-16mm²）

详细实现代码请参考：《8.1配电回路节点实现.md》


### 8.2 配电箱节点实现
配电箱节点负责管理多个配电回路，核心功能包括：

1. **数据结构**：通过`DistributionBoxNodeData`结构体管理配电箱属性，包含名称、总功率、总电流、进线保护设备电流整定值、楼层信息、模块配置和各相负载分配

2. **自动编号**：实现回路自动编号功能，支持按功率大小或其他规则对回路进行排序并分配编号

3. **三相平衡**：提供智能三相平衡算法，通过初始分配和迭代优化，将负载合理分配到L1、L2、L3三相，确保三相负载平衡

4. **电气计算**：自动计算总功率和总电流，并根据电气规范要求确定进线保护设备的电流整定值

详细实现请参考《8.2配电箱节点实现.md》文档。


### 8.3 干线系统图节点实现
干线系统图节点负责自动生成配电干线图和各类监测系统图，核心功能包括：

1. **系统类型支持**：通过`MainSystemType`枚举支持配电干线图、能耗监测、电气火灾监控、消防电源监测等多种系统

2. **自动映射功能**：`auto_map_distribution_boxes`方法可按楼层对配电箱进行分组，并根据配置生成各类系统图

3. **图生成算法**：`generate_power_distribution_diagram`方法自动创建母线、添加配电箱组件、生成连接线，并支持识别双电源切换功能

4. **自动布局**：提供图的自动布局算法，优化组件排列和连接关系

详细实现请参考《8.3干线系统图节点实现.md》文档。

### 8.4 数据流向与实时更新实现
数据流向与实时更新机制实现了节点间的数据流动和状态同步，核心功能包括：

1. **统一数据模型**：通过`PowerNodeData`、`PowerDataType`和`PowerValueType`枚举实现统一的数据表示

2. **更新传播机制**：`propagate_updates`方法通过拓扑排序确定更新顺序，确保数据按依赖关系正确流动

3. **节点更新逻辑**：`update_node`方法根据节点类型执行相应的更新操作，如回路计算、配电箱三相平衡、系统图生成等

4. **依赖追踪**：`mark_downstream_nodes_for_update`方法自动追踪并标记受影响的下游节点，实现级联更新

详细实现请参考《8.4数据流向与实时更新实现.md》文档。

### 8.5 交互式用户界面实现
交互式用户界面提供了直观的操作体验，支持拖拽、连接和实时编辑功能，核心特点包括：

1. **界面布局**：实现左侧组件库、中央编辑区、右侧属性面板的标准节点编辑器布局

2. **节点操作**：支持节点选择、移动、删除和连接创建等基本操作

3. **智能交互**：实现节点拖入配电箱自动创建连接的功能，简化用户操作

4. **属性编辑**：针对不同类型节点提供专用的属性编辑界面，支持实时计算和结果显示

详细实现请参考《8.5交互式用户界面实现.md》文档。

### 8.6 实现自动识别与连线生成
自动识别与连线生成功能增强了干线系统图的智能性，核心功能包括：

1. **进线类型识别**：`determine_incoming_type`方法根据配电箱模块配置自动识别单电源或双电源进线类型

2. **智能连线生成**：`auto_generate_connections`方法按楼层排序配电箱，自动生成箱间连线，并为双电源配电箱连接备用电源

3. **连接信息管理**：通过`ConnectionInfo`和`ConnectionType`实现对不同类型连接的管理和可视化

4. **完整应用示例**：提供了整合所有组件的完整应用代码，展示了从数据类型定义到节点实现再到应用运行的完整流程

详细实现请参考《8.6实现自动识别与连线生成.md》文档。


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

- 1.核心数据类型定义 ：
- 定义了完整的ElectricDataType枚举，包含电流、功率、电压等电气参数类型
- 实现了ElectricValueType值类型和ElectricNodeData节点数据结构
- 添加了CircuitNodeData、DistributionBoxNodeData等节点专用数据结构

- 2.节点模板实现 ：
- 将通用MyAppNodeTemplate替换为电气系统专用ElectricNodeTemplate
- 实现了CategoryTrait和NodeTemplateTrait的电气系统专用版本
- 配置了配电回路、配电箱、干线系统图、电源和计算节点的参数

- 3.应用结构优化 ：
- 将MyNodeGraphApp重命名为ElectricNodeGraphApp，更新为电气系统专用类型
- 实现了App trait的专用版本，包括项目信息显示、分类节点菜单
- 添加了电气系统连接规则检查、自定义节点UI渲染和计算报告功能

- 4.辅助功能增强 ：
- 添加了电流计算、三相平衡度计算等电气专用计算函数
- 实现了项目数据结构和报告生成器
- 添加了必要的导入语句和依赖管理

- 5.文档格式修复 ：
- 修复了文档末尾的多余引号
- 确保了代码块和格式的一致性
