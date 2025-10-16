use eframe::egui::{self, RichText, Ui};
use std::time::{Duration, Instant};

/// 错误通知消息
#[derive(Debug, Clone)]
pub struct ErrorNotification {
    /// 错误ID
    pub id: u64,
    /// 错误标题
    pub title: String,
    /// 错误消息
    pub message: String,
    /// 错误级别
    pub level: ErrorLevel,
    /// 创建时间
    pub created_at: Instant,
    /// 是否已读
    pub is_read: bool,
    /// 自动消失时间（毫秒）
    pub auto_dismiss_ms: Option<u64>,
}

impl ErrorNotification {
    /// 创建新的错误通知
    pub fn new(
        id: u64,
        title: impl Into<String>,
        message: impl Into<String>,
        level: ErrorLevel,
        auto_dismiss_ms: Option<u64>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            message: message.into(),
            level,
            created_at: Instant::now(),
            is_read: false,
            auto_dismiss_ms,
        }
    }
    
    /// 检查是否应该自动消失
    pub fn should_dismiss(&self) -> bool {
        if let Some(dismiss_ms) = self.auto_dismiss_ms {
            self.created_at.elapsed() > Duration::from_millis(dismiss_ms)
        } else {
            false
        }
    }
    
    /// 获取级别对应的图标
    pub fn level_icon(&self) -> &str {
        match self.level {
            ErrorLevel::Info => "ℹ️",
            ErrorLevel::Warning => "⚠️",
            ErrorLevel::Error => "❌",
            ErrorLevel::Critical => "🔥",
        }
    }
    
    /// 获取级别对应的颜色
    pub fn level_color(&self) -> egui::Color32 {
        match self.level {
            ErrorLevel::Info => egui::Color32::from_rgb(59, 130, 246),
            ErrorLevel::Warning => egui::Color32::from_rgb(245, 158, 11),
            ErrorLevel::Error => egui::Color32::from_rgb(239, 68, 68),
            ErrorLevel::Critical => egui::Color32::from_rgb(153, 27, 27),
        }
    }
}

/// 错误级别枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorLevel {
    /// 信息级别
    Info,
    /// 警告级别
    Warning,
    /// 错误级别
    Error,
    /// 严重错误级别
    Critical,
}

/// 错误通知管理器
#[derive(Debug, Default)]
pub struct ErrorNotificationManager {
    /// 错误通知列表
    pub notifications: Vec<ErrorNotification>,
    /// 下一个通知ID
    next_id: u64,
    /// 是否显示通知面板
    show_panel: bool,
    /// 最大通知数量
    max_notifications: usize,
}

impl ErrorNotificationManager {
    /// 创建新的错误通知管理器
    pub fn new(max_notifications: usize) -> Self {
        Self {
            notifications: Vec::new(),
            next_id: 0,
            show_panel: false,
            max_notifications,
        }
    }
    
    /// 添加信息通知
    pub fn add_info(&mut self, title: impl Into<String>, message: impl Into<String>, auto_dismiss_ms: Option<u64>) {
        self.add_notification(title, message, ErrorLevel::Info, auto_dismiss_ms);
    }
    
    /// 添加警告通知
    pub fn add_warning(&mut self, title: impl Into<String>, message: impl Into<String>, auto_dismiss_ms: Option<u64>) {
        self.add_notification(title, message, ErrorLevel::Warning, auto_dismiss_ms);
    }
    
    /// 添加错误通知
    pub fn add_error(&mut self, title: impl Into<String>, message: impl Into<String>, auto_dismiss_ms: Option<u64>) {
        self.add_notification(title, message, ErrorLevel::Error, auto_dismiss_ms);
    }
    
    /// 添加严重错误通知
    pub fn add_critical(&mut self, title: impl Into<String>, message: impl Into<String>) {
        // 严重错误不自动消失
        self.add_notification(title, message, ErrorLevel::Critical, None);
    }
    
    /// 添加通知
    fn add_notification(&mut self, title: impl Into<String>, message: impl Into<String>, level: ErrorLevel, auto_dismiss_ms: Option<u64>) {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        
        let notification = ErrorNotification::new(
            id,
            title,
            message,
            level,
            auto_dismiss_ms,
        );
        
        // 添加到列表头部
        self.notifications.insert(0, notification);
        
        // 限制最大通知数量
        if self.notifications.len() > self.max_notifications {
            self.notifications.truncate(self.max_notifications);
        }
        
        // 严重错误自动显示通知面板
        if level == ErrorLevel::Critical {
            self.show_panel = true;
        }
    }
    
    /// 清除所有通知
    pub fn clear_all(&mut self) {
        self.notifications.clear();
    }
    
    /// 清除指定ID的通知
    pub fn clear_by_id(&mut self, id: u64) {
        self.notifications.retain(|n| n.id != id);
    }
    
    /// 标记所有通知为已读
    pub fn mark_all_as_read(&mut self) {
        for notification in &mut self.notifications {
            notification.is_read = true;
        }
    }
    
    /// 获取未读通知数量
    pub fn unread_count(&self) -> usize {
        self.notifications.iter().filter(|n| !n.is_read).count()
    }
    
    /// 自动清除过期通知
    pub fn cleanup_expired(&mut self) {
        self.notifications.retain(|n| !n.should_dismiss());
    }
    
    /// 切换通知面板显示状态
    pub fn toggle_panel(&mut self) {
        self.show_panel = !self.show_panel;
        if self.show_panel {
            // 显示面板时标记所有通知为已读
            self.mark_all_as_read();
        }
    }
}

/// 错误显示组件
pub struct ErrorDisplayComponent {
    /// 通知管理器
    pub notification_manager: ErrorNotificationManager,
}

impl Default for ErrorDisplayComponent {
    fn default() -> Self {
        Self {
            notification_manager: ErrorNotificationManager::new(50),
        }
    }
}

impl ErrorDisplayComponent {
    /// 创建新的错误显示组件
    pub fn new() -> Self {
        Default::default()
    }
    
    /// 更新组件状态
    pub fn update(&mut self) {
        // 清理过期通知
        self.notification_manager.cleanup_expired();
    }
    
    /// 显示通知图标
    pub fn show_notification_icon(&mut self, ui: &mut Ui) {
        let unread_count = self.notification_manager.unread_count();
        let mut style = ui.style_mut();
        
        // 创建通知图标按钮
        let button_response = ui.add(
            egui::Button::new(
                if unread_count > 0 {
                    RichText::new(format!("🔔 ({})", unread_count))
                        .color(egui::Color32::from_rgb(239, 68, 68))
                } else {
                    RichText::new("🔔")
                }
            )
            .frame(false)
            .small()
        );
        
        // 处理点击事件
        if button_response.clicked() {
            self.notification_manager.toggle_panel();
        }
        
        // 显示悬停提示
        if button_response.hovered() {
            ui.tooltip_text(format!("{}/{} 条通知", unread_count, self.notification_manager.notifications.len()));
        }
        
        // 重置样式
        *style = ui.ctx().style();
    }
    
    /// 显示通知面板
    pub fn show_notification_panel(&mut self, ctx: &egui::Context) {
        if self.notification_manager.show_panel {
            egui::Window::new("通知中心")
                .title_bar(true)
                .resizable(true)
                .default_width(400.0)
                .default_height(500.0)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        // 标题栏
                        ui.horizontal(|ui| {
                            ui.heading("通知中心");
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("清除全部").clicked() {
                                    self.notification_manager.clear_all();
                                }
                            });
                        });
                        
                        ui.separator();
                        
                        // 通知列表
                        if self.notification_manager.notifications.is_empty() {
                            ui.centered_and_justified(|ui| {
                                ui.label("暂无通知");
                            });
                        } else {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                for notification in &mut self.notification_manager.notifications {
                                    self.show_notification_item(ui, notification);
                                }
                            });
                        }
                    });
                });
        }
    }
    
    /// 显示通知项
    fn show_notification_item(&mut self, ui: &mut Ui, notification: &mut ErrorNotification) {
        let bg_color = if !notification.is_read {
            Some(egui::Color32::from_rgba_premultiplied(239, 68, 68, 30))
        } else {
            None
        };
        
        egui::Frame::default()
            .fill(bg_color.unwrap_or(ui.style().visuals.panel_fill))
            .inner_margin(egui::style::Margin::same(8.0))
            .rounding(8.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // 图标
                    ui.label(notification.level_icon());
                    
                    ui.vertical(|ui| {
                        // 标题
                        ui.label(
                            RichText::new(&notification.title)
                                .color(notification.level_color())
                                .strong()
                        );
                        
                        // 消息
                        ui.label(&notification.message);
                        
                        // 时间信息
                        let time_passed = notification.created_at.elapsed();
                        let time_str = if time_passed.as_secs() < 60 {
                            format!("{}秒前", time_passed.as_secs())
                        } else if time_passed.as_secs() < 3600 {
                            format!("{}分钟前", time_passed.as_secs() / 60)
                        } else {
                            format!("{}小时前", time_passed.as_secs() / 3600)
                        };
                        ui.label(RichText::new(time_str).small().secondary());
                    });
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Top), |ui| {
                        if ui.button("✕").clicked() {
                            self.notification_manager.clear_by_id(notification.id);
                        }
                    });
                });
            });
        
        ui.add_space(4.0);
        
        // 标记为已读
        notification.is_read = true;
    }
    
    /// 显示错误对话框
    pub fn show_error_dialog(
        &self,
        ctx: &egui::Context,
        title: impl Into<String>,
        message: impl Into<String>,
        details: Option<impl Into<String>>,
        level: ErrorLevel,
    ) {
        let title = title.into();
        let message = message.into();
        let details = details.map(|d| d.into());
        
        egui::Window::new(title.clone())
            .collapsible(false)
            .resizable(true)
            .default_width(450.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // 标题和图标
                    ui.horizontal(|ui| {
                        ui.label(match level {
                            ErrorLevel::Info => "ℹ️",
                            ErrorLevel::Warning => "⚠️",
                            ErrorLevel::Error => "❌",
                            ErrorLevel::Critical => "🔥",
                        });
                        ui.heading(
                            RichText::new(title)
                                .color(match level {
                                    ErrorLevel::Info => egui::Color32::from_rgb(59, 130, 246),
                                    ErrorLevel::Warning => egui::Color32::from_rgb(245, 158, 11),
                                    ErrorLevel::Error => egui::Color32::from_rgb(239, 68, 68),
                                    ErrorLevel::Critical => egui::Color32::from_rgb(153, 27, 27),
                                })
                        );
                    });
                    
                    ui.separator();
                    
                    // 错误消息
                    ui.label(message);
                    
                    // 详细信息
                    if let Some(details) = details {
                        ui.collapsing("详细信息", |ui| {
                            ui.text_edit_multiline(&mut details.clone());
                        });
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("确定").clicked() {
                            ui.close_menu();
                        }
                    });
                });
            });
    }
    
    /// 显示信息对话框
    pub fn show_info_dialog(
        &self,
        ctx: &egui::Context,
        title: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.show_error_dialog(ctx, title, message, None, ErrorLevel::Info);
    }
    
    /// 显示警告对话框
    pub fn show_warning_dialog(
        &self,
        ctx: &egui::Context,
        title: impl Into<String>,
        message: impl Into<String>,
        details: Option<impl Into<String>>,
    ) {
        self.show_error_dialog(ctx, title, message, details, ErrorLevel::Warning);
    }
    
    /// 显示错误对话框
    pub fn show_error_dialog(
        &self,
        ctx: &egui::Context,
        title: impl Into<String>,
        message: impl Into<String>,
        details: Option<impl Into<String>>,
    ) {
        self.show_error_dialog(ctx, title, message, details, ErrorLevel::Error);
    }
    
    /// 显示严重错误对话框
    pub fn show_critical_dialog(
        &self,
        ctx: &egui::Context,
        title: impl Into<String>,
        message: impl Into<String>,
        details: Option<impl Into<String>>,
    ) {
        self.show_error_dialog(ctx, title, message, details, ErrorLevel::Critical);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_notification_manager() {
        let mut manager = ErrorNotificationManager::new(10);
        
        // 添加通知
        manager.add_info("测试", "这是一条信息通知", Some(1000));
        manager.add_warning("测试", "这是一条警告通知", Some(2000));
        manager.add_error("测试", "这是一条错误通知", Some(3000));
        manager.add_critical("测试", "这是一条严重错误通知");
        
        assert_eq!(manager.notifications.len(), 4);
        assert_eq!(manager.unread_count(), 4);
        
        // 标记为已读
        manager.mark_all_as_read();
        assert_eq!(manager.unread_count(), 0);
        
        // 清除通知
        let first_id = manager.notifications[0].id;
        manager.clear_by_id(first_id);
        assert_eq!(manager.notifications.len(), 3);
        
        // 清除所有
        manager.clear_all();
        assert_eq!(manager.notifications.len(), 0);
    }
    
    #[test]
    fn test_error_display_component() {
        let mut component = ErrorDisplayComponent::new();
        
        // 添加各种通知
        component.notification_manager.add_info("信息", "这是信息", Some(1000));
        component.notification_manager.add_warning("警告", "这是警告", Some(2000));
        component.notification_manager.add_error("错误", "这是错误", Some(3000));
        component.notification_manager.add_critical("严重错误", "这是严重错误");
        
        assert_eq!(component.notification_manager.notifications.len(), 4);
        
        // 测试自动显示面板
        assert!(component.notification_manager.show_panel);
        
        // 切换面板
        component.notification_manager.toggle_panel();
        assert!(!component.notification_manager.show_panel);
    }
}