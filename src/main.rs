use eframe::egui;
pub mod application;
use application::app::PDSDApp;

/// 程序入口点
fn main() -> Result<(), eframe::Error> {
    // 设置eframe选项
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("电力配电系统设计软件 (PDSD)"),

        // 渲染配置
        renderer: eframe::Renderer::Glow,

        // 其他配置
        vsync: true,

        ..Default::default()
    };

    // 启动应用
    eframe::run_native(
        "电力配电系统设计软件 (PDSD)",
        options,
        Box::new(|cc| {
            // 配置字体以支持中文
            let mut fonts = egui::FontDefinitions::default();
            
            // 使用项目assets目录中的微软雅黑字体支持中文
            fonts.font_data.insert(
                "msyh_font".to_owned(),
                egui::FontData::from_static(
                    include_bytes!("../assets/fonts/msyh.ttc") as &[u8],
                ).into(),
            );
            
            // 将微软雅黑字体添加到默认字体家族
            fonts.families.insert(
                egui::FontFamily::Proportional,
                vec![
                    "msyh_font".to_owned(),
                ],
            );
            
            // 也添加到等宽字体家族以支持代码显示
            fonts.families.insert(
                egui::FontFamily::Monospace,
                vec![
                    "msyh_font".to_owned(),
                ],
            );
            
            cc.egui_ctx.set_fonts(fonts);

            // 创建应用实例
            Ok(Box::new(PDSDApp::default()))
        }),
    )
}
