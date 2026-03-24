mod app;
mod curl_parser;
mod executor;
mod storage;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 500.0])
            .with_title("Curl Helper"),
        ..Default::default()
    };

    eframe::run_native(
        "Curl Helper",
        options,
        Box::new(|cc| Ok(Box::new(app::CurlHelperApp::new(cc)))),
    )
}
