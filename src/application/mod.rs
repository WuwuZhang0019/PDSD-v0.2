/// 应用程序核心模块
pub mod app;
pub mod state;
pub mod error;
pub mod debug_logger;
pub mod integration_example;

// 重新导出主要结构体和类型
pub use app::PDSDApp;
pub use state::AppState;
pub use error::ApplicationError;
pub use debug_logger::{DebugLogger, LogLevel, LogEntry};
pub use integration_example::start_integration_example;
