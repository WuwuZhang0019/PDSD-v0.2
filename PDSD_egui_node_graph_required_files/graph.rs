use super::*;

#[cfg(feature = "persistence")]
use serde::{Deserialize, Serialize};

/// [`Graph`]中的节点。节点具有输入和输出参数，存储为id。它们还包含一个自定义的`NodeData`结构，可存储用户想要的每个节点的数据。
#[derive(Debug, Clone)]
#[cfg_attr(feature = "persistence", derive(Serialize, Deserialize))]
pub struct Node<NodeData> {
    pub id: NodeId,
    pub label: String,
    pub inputs: Vec<(String, InputId)>,
    pub outputs: Vec<(String, OutputId)>,
    pub user_data: NodeData,
}

/// 三种输入参数类型。这些描述了图对于此参数的内联小部件和连接必须如何行为。
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "persistence", derive(Serialize, Deserialize))]
pub enum InputParamKind {
    /// 不能设置常量值。只能通过传入连接产生值
    ConnectionOnly,
    /// 只能设置常量值。不接受传入连接。
    ConstantOnly,
    /// 接受传入连接和常量值。连接优先于常量值。
    ConnectionOrConstant,
}

#[cfg(feature = "persistence")]
fn shown_inline_default() -> bool {
    true
}

/// 输入参数。输入参数位于节点内部，表示该节点接收的数据。与对应的[`OutputParam`]不同，输入参数还显示一个内联小部件，允许设置其"值"。
/// `DataType`泛型参数用于限制此参数的输入连接范围，而`ValueType`用于表示内联小部件(即常量)值的数据。
#[derive(Debug, Clone)]
#[cfg_attr(feature = "persistence", derive(Serialize, Deserialize))]
pub struct InputParam<DataType, ValueType> {
    pub id: InputId,
    /// 此节点的数据类型。用于确定传入连接。这应该始终与InputParamValue的类型匹配，但实际上并不强制要求。
    pub typ: DataType,
    /// 存储在此参数中的常量值。
    pub value: ValueType,
    /// 输入类型。参见[`InputParamKind`]
    pub kind: InputParamKind,
    /// 对包含此参数的节点的反向引用。
    pub node: NodeId,
    /// 为true时，节点显示在节点图的内联部分。
    #[cfg_attr(feature = "persistence", serde(default = "shown_inline_default"))]
    pub shown_inline: bool,
}

/// 输出参数。输出参数位于节点内部，表示节点产生的数据。输出参数可以链接到其他节点的输入参数。与[`InputParam`]不同，输出参数不能有常量内联值。
#[derive(Debug, Clone)]
#[cfg_attr(feature = "persistence", derive(Serialize, Deserialize))]
pub struct OutputParam<DataType> {
    pub id: OutputId,
    /// 对包含此参数的节点的反向引用。
    pub node: NodeId,
    pub typ: DataType,
}

/// 图，包含节点、输入参数和输出参数。由于图中充满了自引用结构，此类型使用`slotmap`库来表示数据中的所有内部引用。
#[derive(Debug, Clone)]
#[cfg_attr(feature = "persistence", derive(Serialize, Deserialize))]
pub struct Graph<NodeData, DataType, ValueType> {
    /// 图的[`Node`]s
    pub nodes: SlotMap<NodeId, Node<NodeData>>,
    /// 图的[`InputParam`]s
    pub inputs: SlotMap<InputId, InputParam<DataType, ValueType>>,
    /// 图的[`OutputParam`]s
    pub outputs: SlotMap<OutputId, OutputParam<DataType>>,
    // 将节点的输入连接到产生它的前驱节点的输出
    pub connections: SecondaryMap<InputId, OutputId>,
}
