use super::id_type::{AnyParameterId, NodeId};

#[derive(Debug, thiserror::Error)]
pub enum EguiGraphError {
    #[error("节点 {0:?} 没有名为 {1} 的参数")]
    NoParameterNamed(NodeId, String),

    #[error("在图中找不到参数 {0:?}。")]
    InvalidParameterId(AnyParameterId),
}
