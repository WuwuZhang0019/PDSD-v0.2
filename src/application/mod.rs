/// 应用程序核心模块
pub mod app;
pub mod state;
pub mod error;

// 重新导出主要结构体和类型
pub use app::PDSDApp;
pub use state::AppState;
pub use error::ApplicationError;
