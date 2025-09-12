use rand::Rng;

pub mod image_color;
pub mod layer;
pub mod draw_tool;
pub mod pencil_cursor;
pub mod create_paint;

pub fn new_rand_id()-> egui::Id {
    let rand = rand::rng().random::<u32>();
   egui::Id::from(rand.to_string())
}