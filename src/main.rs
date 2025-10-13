use eframe::egui;
mod application;

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
            let fonts = egui::FontDefinitions::default();
            cc.egui_ctx.set_fonts(fonts);
            
            // 创建应用实例
            Box::new(PDSDApp::new())
        }),
    )
}