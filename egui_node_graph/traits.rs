use super::*;

/// 此trait必须由[`Graph`]的`ValueType`泛型参数实现。该trait允许为节点图的不同类型绘制自定义内联小部件。
///
/// 需要[`Default`] trait约束来使用`std::mem::take`规避借用检查器问题。否则，在`value_widget`期间无法传递
/// `node_data`参数。默认值从不使用，因此实现不重要，但应该构造成本合理。
pub trait WidgetValueTrait: Default {
    type Response;
    type UserState;
    type NodeData;

    /// 此方法仅会在每个具有断开连接输入的输入参数的小部件上调用。要显示已连接输入的UI，请使用[`WidgetValueTrait::value_widget_connected`]。
/// 返回值是自定义响应对象的向量，可用于实现副作用处理。如果不确定，响应Vec可以为空。
    fn value_widget(
        &mut self,
        param_name: &str,
        node_id: NodeId,
        ui: &mut egui::Ui,
        user_state: &mut Self::UserState,
        node_data: &Self::NodeData,
    ) -> Vec<Self::Response>;

    /// 此方法仅会在每个具有已连接输入的输入参数的小部件上调用。要显示断开连接输入的UI，请使用[`WidgetValueTrait::value_widget`]。
/// 返回值是自定义响应对象的向量，可用于实现副作用处理。如果不确定，响应Vec可以为空。
///
/// 默认显示输入名称标签。
    fn value_widget_connected(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut Self::UserState,
        _node_data: &Self::NodeData,
    ) -> Vec<Self::Response> {
        ui.label(param_name);

        Default::default()
    }
}

/// 此trait必须由[`Graph`]的`DataType`泛型参数实现。此trait告诉库如何以视觉方式向用户展示数据类型。
pub trait DataTypeTrait<UserState>: PartialEq + Eq {
    /// 与此数据类型关联的端口颜色
    fn data_type_color(&self, user_state: &mut UserState) -> egui::Color32;

    /// 此数据类型的名称。返回类型指定为Cow<str>，因为某些实现需要分配新字符串来提供答案，而其他实现则不需要。
///
/// ## 示例（借用值）
/// 当您可以从其字段或作为&'static str获取数据类型名称时使用此方法。尽可能首选此方法。
/// ```ignore
/// pub struct DataType { name: String }
///
/// impl DataTypeTrait<()> for DataType {
///     fn name(&self) -> std::borrow::Cow<str> {
///         Cow::Borrowed(&self.name)
///     }
/// }
/// ```
///
/// ## 示例（拥有值）
/// 当您无法从其字段派生数据类型名称时使用此方法。
/// ```ignore
/// pub struct DataType { some_tag: i32 }
///
/// impl DataTypeTrait<()> for DataType {
///     fn name(&self) -> std::borrow::Cow<str> {
///         Cow::Owned(format!("超级神奇类型 #{}", self.some_tag))
///     }
/// }
/// ```
    fn name(&self) -> std::borrow::Cow<str>;
}

/// 此trait必须为[`Graph`]的`NodeData`泛型参数实现。此trait允许自定义节点绘制的某些方面。
pub trait NodeDataTrait
where
    Self: Sized,
{
    /// 必须设置为自定义用户`NodeResponse`类型
    type Response;
    /// 必须设置为自定义用户`UserState`类型
    type UserState;
    /// 必须设置为自定义用户`DataType`类型
    type DataType;
    /// 必须设置为自定义用户`ValueType`类型
    type ValueType;

    /// 在参数后绘制在节点中的附加UI元素。
    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        graph: &Graph<Self, Self::DataType, Self::ValueType>,
        user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<Self::Response, Self>>
    where
        Self::Response: UserResponseTrait;

    /// 在节点顶部栏上绘制的UI。
    fn top_bar_ui(
        &self,
        _ui: &mut egui::Ui,
        _node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<Self::Response, Self>>
    where
        Self::Response: UserResponseTrait,
    {
        Default::default()
    }

    /// 为每个输出绘制的UI
///
/// 默认显示param_name作为简单标签。
    fn output_ui(
        &self,
        ui: &mut egui::Ui,
        _node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        param_name: &str,
    ) -> Vec<NodeResponse<Self::Response, Self>>
    where
        Self::Response: UserResponseTrait,
    {
        ui.label(param_name);

        Default::default()
    }

    /// 在标题栏上设置背景颜色
/// 如果返回值为None，则设置默认颜色。
    fn titlebar_color(
        &self,
        _ui: &egui::Ui,
        _node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> Option<egui::Color32> {
        None
    }

    /// 放置在节点中元素之间的分隔符。
///
/// 在输入、输出和底部UI之间调用。对于在没有显式分隔符的情况下开始失去结构的复杂UI很有用。
/// `param_id`参数是位于分隔符*之前*的输入或输出的ID。
///
/// 默认实现不执行任何操作。
    fn separator(
        &self,
        _ui: &mut egui::Ui,
        _node_id: NodeId,
        _param_id: AnyParameterId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) {
    }

    fn can_delete(
        &self,
        _node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> bool {
        true
    }
}

/// 任何用户类型都可以实现此trait。此trait告诉库如何枚举作为节点查找器一部分呈现给用户的节点模板。
pub trait NodeTemplateIter {
    type Item;
    fn all_kinds(&self) -> Vec<Self::Item>;
}

/// 描述节点的类别。
///
/// 由[`NodeTemplateTrait::node_finder_categories`]用于将节点模板分类到组中。
///
/// 如果程序中所有节点都已知，那么定义一个包含所有类别的枚举并为其实现[`CategoryTrait`]是很有用的。这样可以避免因输入错误现有类别而意外创建新类别，就像使用字符串类型时那样。
pub trait CategoryTrait {
    /// 类别的名称。
    fn name(&self) -> String;
}

impl CategoryTrait for () {
    fn name(&self) -> String {
        String::new()
    }
}

impl<'a> CategoryTrait for &'a str {
    fn name(&self) -> String {
        self.to_string()
    }
}

impl CategoryTrait for String {
    fn name(&self) -> String {
        self.clone()
    }
}

/// 此trait必须由[`GraphEditorState`]的`NodeTemplate`泛型参数实现。它允许自定义节点模板。节点模板描述了可以添加到图中的节点类型、它们的名称以及它们的输入/输出参数。
pub trait NodeTemplateTrait: Clone {
    /// 必须设置为自定义用户`NodeData`类型
    type NodeData;
    /// 必须设置为自定义用户`DataType`类型
    type DataType;
    /// 必须设置为自定义用户`ValueType`类型
    type ValueType;
    /// 必须设置为自定义用户`UserState`类型
    type UserState;
    /// 必须是实现了[`CategoryTrait`] trait的类型。
///
/// 如果您打算简单地输入节点的类别，`&'static str`是一个很好的默认选择。如果您根本不需要类别，请使用`()`。
    type CategoryType;

    /// 返回节点类型的描述性名称，用于节点查找器。
///
/// 返回类型为Cow<str>，以允许更灵活地返回拥有或借用的值。有关更多信息，请参阅`DataTypeTrait::name`的文档
    fn node_finder_label(&self, user_state: &mut Self::UserState) -> std::borrow::Cow<str>;

    /// 节点所属的类别向量。
///
/// 将相似的节点组织到类别中通常很有用，节点查找器将使用这些类别来显示更易于管理的UI，特别是当节点模板众多时。
    fn node_finder_categories(&self, _user_state: &mut Self::UserState) -> Vec<Self::CategoryType> {
        Vec::default()
    }

    /// 返回节点类型的描述性名称，用于图中。
    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String;

    /// 返回此节点类型的用户数据。
    fn user_data(&self, user_state: &mut Self::UserState) -> Self::NodeData;

    /// 当此节点类型被添加到图中时运行此函数。默认情况下，节点将为空，此函数可用于填充其参数。
    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        user_state: &mut Self::UserState,
        node_id: NodeId,
    );
}

/// 在图中绘制节点时的自定义用户响应类型必须实现此trait。
pub trait UserResponseTrait: Clone + std::fmt::Debug {}
