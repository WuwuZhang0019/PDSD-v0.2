#![forbid(unsafe_code)]

use slotmap::{SecondaryMap, SlotMap};

pub type SVec<T> = smallvec::SmallVec<[T; 4]>;

/// 包含节点图模型的主要定义。
pub mod graph;
pub use graph::*;

/// 不同id类型（节点、输入、输出）的类型声明
pub mod id_type;
pub use id_type::*;

/// 为Graph类型实现索引trait，允许通过所有三种id类型进行索引
pub mod index_impls;

/// 实现`Graph`的主要方法
pub mod graph_impls;

/// 自定义错误类型，整个crate范围内使用
pub mod error;
pub use error::*;

/// 库中的主要结构体，包含绘制UI图所需的所有必要状态
pub mod ui_state;
pub use ui_state::*;

/// 节点查找器是一个允许创建新节点类型的小部件
pub mod node_finder;
pub use node_finder::*;

/// egui实现的内部细节。大多数egui代码都在这里。
pub mod editor_ui;
pub use editor_ui::*;

/// 用户必须实现的几个trait，以自定义此库的行为。
pub mod traits;
pub use traits::*;

mod utils;

mod color_hex_utils;
