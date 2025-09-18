use eframe::*;

mod app;
use app::App;
fn main() -> eframe::Result {
     let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1700.0, 900.0])
        .with_resizable(true),
        // .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native("My Drawing App", options, Box::new(|cc| Ok(Box::new(App::new(cc)))))
}