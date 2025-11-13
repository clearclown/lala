mod core;
mod gui;

use gui::EditorApp;

fn main() -> eframe::Result {
    // ロガーの初期化（オプション）
    env_logger::init();

    // ネイティブウィンドウのオプション設定
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_min_inner_size([640.0, 480.0])
            .with_title("Lala Editor"),
        ..Default::default()
    };

    // アプリケーションの起動
    eframe::run_native(
        "Lala Editor",
        native_options,
        Box::new(|_cc| Ok(Box::new(EditorApp::new()))),
    )
}
