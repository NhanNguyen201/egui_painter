use egui::ColorImage;


pub fn blend_pixel(bottom: egui::Color32, top: egui::Color32) -> egui::Color32 {

    let top_a = top.a() as f32 / 255.0;
    let bottom_a = bottom.a() as f32 / 255.0;

    let out_a = top_a + bottom_a * (1.0 - top_a);

    if out_a == 0.0 {
        return egui::Color32::TRANSPARENT;
    }

    let r = ((top.r() as f32 * top_a + bottom.r() as f32 * bottom_a * (1.0 - top_a)) / out_a).round() as u8;
    let g = ((top.g() as f32 * top_a + bottom.g() as f32 * bottom_a * (1.0 - top_a)) / out_a).round() as u8;
    let b = ((top.b() as f32 * top_a + bottom.b() as f32 * bottom_a * (1.0 - top_a)) / out_a).round() as u8;
    let a = (out_a * 255.0).round() as u8;

    egui::Color32::from_rgba_unmultiplied(r, g, b, a)

}
pub fn composite_layers(layers: &[egui::ColorImage]) -> egui::ColorImage {
    let size = layers[0].size;
    let mut final_pixels = layers[0].pixels.clone();

    for layer in &layers[1..] {
        for i in 0..final_pixels.len() {
            final_pixels[i] = blend_pixel(final_pixels[i], layer.pixels[i]);
        }
    }

    
    ColorImage::new(size, final_pixels)
}
