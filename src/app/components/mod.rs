pub mod canvas;
pub mod color_palette;
pub mod utils;
pub mod layers_display_container;
pub mod tools_bar;
pub mod color_picker;

pub trait AppComponentExt {
    type Context;
    fn add(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui);
}