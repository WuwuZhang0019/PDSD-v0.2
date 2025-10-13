use once_cell::sync::Lazy;
use std::path::PathBuf;

/// 应用程序全局配置
pub struct AppConfig {
    /// 窗口标题
    pub window_title: &'static str,
    /// 默认窗口尺寸
    pub default_window_size: (f32, f32),
    /// 最小窗口尺寸
    pub min_window_size: (f32, f32),
    /// 网格配置
    pub grid: GridConfig,
    /// 节点配置
    pub node: NodeConfig,
    /// 主题配置
    pub theme: ThemeConfig,
    /// 文件配置
    pub file: FileConfig,
}

/// 网格配置
pub struct GridConfig {
    /// 网格大小（世界坐标）
    pub size: f32,
    /// 网格线颜色
    pub line_color: egui::Color32,
    /// 网格线宽度
    pub line_width: f32,
}

/// 节点配置
pub struct NodeConfig {
    /// 默认节点大小
    pub default_size: f32,
    /// 节点圆角半径
    pub corner_radius: f32,
    /// 选中状态边框宽度
    pub selected_stroke_width: f32,
    /// 选中状态边框颜色
    pub selected_stroke_color: egui::Color32,
    /// 选中状态边框类型
    pub selected_stroke_kind: egui::StrokeKind,
    /// 普通状态边框宽度
    pub normal_stroke_width: f32,
    /// 普通状态边框颜色
    pub normal_stroke_color: egui::Color32,
    /// 普通状态边框类型
    pub normal_stroke_kind: egui::StrokeKind,
}

/// 主题配置
pub struct ThemeConfig {
    /// 背景颜色
    pub background_color: egui::Color32,
    /// 节点背景颜色
    pub node_background_color: egui::Color32,
    /// 文本颜色
    pub text_color: egui::Color32,
    /// 端点颜色
    pub port_colors: PortColors,
}

/// 端点颜色配置
pub struct PortColors {
    /// 输入端口颜色
    pub input: egui::Color32,
    /// 输出端口颜色
    pub output: egui::Color32,
    /// 端口边框颜色
    pub stroke: egui::Color32,
}

/// 文件配置
pub struct FileConfig {
    /// 默认保存目录
    pub default_save_dir: PathBuf,
    /// 支持的文件扩展名
    pub supported_extensions: Vec<&'static str>,
    /// 自动保存间隔（秒）
    pub auto_save_interval: u64,
}

/// 全局配置实例
pub static CONFIG: Lazy<AppConfig> = Lazy::new(|| AppConfig {
    window_title: "建筑电气配电系统图设计工具",
    default_window_size: (1200.0, 800.0),
    min_window_size: (800.0, 600.0),
    
    grid: GridConfig {
        size: 50.0,
        line_color: egui::Color32::from_gray(200),
        line_width: 0.5,
    },
    
    node: NodeConfig {
        default_size: 40.0,
        corner_radius: 8.0,
        selected_stroke_width: 2.0,
        selected_stroke_color: egui::Color32::from_rgb(0, 255, 255),
        selected_stroke_kind: egui::StrokeKind::Outside,
        normal_stroke_width: 2.0,
        normal_stroke_color: egui::Color32::from_rgba_premultiplied(30, 30, 30, 200),
        normal_stroke_kind: egui::StrokeKind::Middle,
    },
    
    theme: ThemeConfig {
        background_color: egui::Color32::from_rgb(30, 30, 30),
        node_background_color: egui::Color32::from_rgba_premultiplied(70, 130, 180, 200),
        text_color: egui::Color32::WHITE,
        port_colors: PortColors {
            input: egui::Color32::GREEN,
            output: egui::Color32::RED,
            stroke: egui::Color32::BLACK,
        },
    },
    
    file: FileConfig {
        default_save_dir: PathBuf::from("./saved_diagrams"),
        supported_extensions: vec!["json", "yaml", "toml"],
        auto_save_interval: 300, // 5分钟
    },
});

impl AppConfig {
    /// 获取网格大小（根据缩放级别）
    pub fn get_grid_size_with_zoom(&self, zoom: f32) -> f32 {
        self.grid.size * zoom
    }
    
    /// 获取节点大小（根据缩放级别）
    pub fn get_node_size_with_zoom(&self, zoom: f32) -> f32 {
        self.node.default_size * zoom
    }
    
    /// 创建选中状态的Stroke
    pub fn create_selected_stroke(&self) -> egui::Stroke {
        egui::Stroke::new(self.node.selected_stroke_width, self.node.selected_stroke_color)
    }
    
    /// 创建普通状态的Stroke
    pub fn create_normal_stroke(&self) -> egui::Stroke {
        egui::Stroke::new(self.node.normal_stroke_width, self.node.normal_stroke_color)
    }
}

/// 开发模式配置（用于调试）
pub static DEV_CONFIG: Lazy<AppConfig> = Lazy::new(|| AppConfig {
    window_title: "[DEV] 建筑电气配电系统图设计工具",
    default_window_size: CONFIG.default_window_size,
    min_window_size: CONFIG.min_window_size,
    grid: GridConfig {
        size: CONFIG.grid.size,
        line_color: CONFIG.grid.line_color,
        line_width: CONFIG.grid.line_width,
    },
    node: NodeConfig {
        default_size: CONFIG.node.default_size,
        corner_radius: CONFIG.node.corner_radius,
        selected_stroke_width: CONFIG.node.selected_stroke_width,
        selected_stroke_color: CONFIG.node.selected_stroke_color,
        selected_stroke_kind: CONFIG.node.selected_stroke_kind,
        normal_stroke_width: CONFIG.node.normal_stroke_width,
        normal_stroke_color: CONFIG.node.normal_stroke_color,
        normal_stroke_kind: CONFIG.node.normal_stroke_kind,
    },
    theme: ThemeConfig {
        background_color: CONFIG.theme.background_color,
        node_background_color: CONFIG.theme.node_background_color,
        text_color: CONFIG.theme.text_color,
        port_colors: PortColors {
            input: CONFIG.theme.port_colors.input,
            output: CONFIG.theme.port_colors.output,
            stroke: CONFIG.theme.port_colors.stroke,
        },
    },
    file: FileConfig {
        default_save_dir: CONFIG.file.default_save_dir.clone(),
        supported_extensions: CONFIG.file.supported_extensions.clone(),
        auto_save_interval: CONFIG.file.auto_save_interval,
    },
});

/// 配置工具函数
pub mod utils {
    use super::*;
    
    /// 检查是否处于开发模式
    pub fn is_dev_mode() -> bool {
        cfg!(debug_assertions) || std::env::var("RUST_LOG").is_ok()
    }
    
    /// 获取当前配置（根据开发模式）
    pub fn get_config() -> &'static AppConfig {
        if is_dev_mode() {
            &DEV_CONFIG
        } else {
            &CONFIG
        }
    }
    
    /// 确保保存目录存在
    pub fn ensure_save_directory() -> std::io::Result<()> {
        let config = get_config();
        if !config.file.default_save_dir.exists() {
            std::fs::create_dir_all(&config.file.default_save_dir)?;
        }
        Ok(())
    }
}