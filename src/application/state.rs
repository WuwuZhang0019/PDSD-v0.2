/// 应用状态管理模块
use std::sync::{Arc, Mutex};

/// 应用程序的全局状态
#[derive(Clone)]
pub struct AppState {
    /// 项目信息
    pub project_info: Arc<Mutex<ProjectInfo>>,
    /// 当前编辑的图表节点
    pub nodes: Arc<Mutex<Vec<GraphNode>>>,
    /// 应用配置
    pub config: Arc<Mutex<AppConfig>>,
    /// 计算结果
    pub calculation_results: Arc<Mutex<CalculationResults>>,
}

impl AppState {
    /// 创建新的应用状态实例
    pub fn new() -> Self {
        Self {
            project_info: Arc::new(Mutex::new(ProjectInfo::default())),
            nodes: Arc::new(Mutex::new(Vec::new())),
            config: Arc::new(Mutex::new(AppConfig::default())),
            calculation_results: Arc::new(Mutex::new(CalculationResults::default())),
        }
    }
}

/// 项目信息
#[derive(Default, Debug)]
pub struct ProjectInfo {
    /// 项目名称
    pub name: String,
    /// 项目描述
    pub description: String,
    /// 项目编号
    pub project_id: String,
    /// 创建日期
    pub creation_date: String,
    /// 最后修改日期
    pub last_modified: String,
}

/// 应用配置
#[derive(Default, Debug)]
pub struct AppConfig {
    /// 主题设置
    pub theme: Theme,
    /// 语言设置
    pub language: String,
    /// 自动保存间隔（秒）
    pub auto_save_interval: u64,
    /// 是否启用自动计算
    pub auto_calculate: bool,
    /// 网格大小
    pub grid_size: f32,
}

/// UI主题
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    /// 浅色主题
    Light,
    /// 深色主题
    Dark,
    /// 跟随系统
    System,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Light
    }
}

/// 计算结果
#[derive(Default, Debug)]
pub struct CalculationResults {
    /// 总功率
    pub total_power: Option<f64>,
    /// 总电流
    pub total_current: Option<f64>,
    /// 电压损失
    pub voltage_drop: Option<f64>,
    /// 三相不平衡度
    pub phase_imbalance: Option<f64>,
    /// 短路电流
    pub short_circuit_current: Option<f64>,
    /// 计算是否成功
    pub calculation_success: bool,
    /// 错误信息
    pub error_message: String,
}

// 导入GraphNode类型
#[derive(Debug, Clone, Default)]
pub struct GraphNode {
    /// 节点ID
    pub id: String,
    /// 节点名称
    pub name: String,
    /// 节点类型
    pub node_type: String,
    /// X坐标
    pub x: f32,
    /// Y坐标
    pub y: f32,
    /// 节点属性
    pub properties: std::collections::HashMap<String, String>,
}