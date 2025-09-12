use egui::{Color32, Frame, Key, Pos2, Sense, TextureOptions, Vec2};

use super::AppComponentExt;
use crate::app::{
    components::utils::pencil_cursor::DirLocked,
     App
};
pub struct Canvas;


// let frame_style = egui::Style::clone_from(&mut self, source);
impl AppComponentExt for Canvas {
    type Context = App;
    fn add(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        Frame::canvas(ui.style()).outer_margin(0.).show(ui, |ui| {
            let padding = 5.;
            // Make the canvas painter clip to the canvas area
            let cursor = &mut ctx.app_settings.pencil_cursor;
            
            let (canva_response, painter) = ui.allocate_painter(ctx.app_settings.canvas_size.clone() + Vec2::new(padding * 2., padding * 2.), Sense::click());
            let canvas_rect = egui::Rect::from_min_max(canva_response.rect.min + Vec2::new(padding, padding), canva_response.rect.max - Vec2::new(padding, padding));
            let canva_sense = ui.allocate_rect(canvas_rect, Sense::click_and_drag());
            painter.rect_filled(canvas_rect,0.0, Color32::from_rgb(200, 200, 200));
            
            if canva_sense.drag_started() {
                ctx.app_state.is_dragging = true;
            }
           
            if canva_sense.clicked() || (canva_sense.is_pointer_button_down_on() &&  ctx.app_state.is_dragging) {
             
                let pos = cursor.get_pos() - canvas_rect.min.to_vec2();
                let color = ctx.app_state.current_color.clone().unwrap_or_default().color;
                if let Some(layer_id) = ctx.app_state.current_layer {
                    if let Some(layer) = ctx.app_state.layers.iter_mut().find(|layer| layer.id == layer_id) {
                        layer.texture.paint_at(pos, ctx.app_state.current_draw_tool.clone().unwrap().pencil, ctx.app_state.current_stroke_width, color);
                    }
                }
            }
            if canva_sense.drag_stopped() {
                ctx.app_state.is_dragging = false
            }
           
            for layer in ctx.app_state.layers.clone().iter_mut().filter(|layer| layer.is_visible).rev(){
                let texture_handle = &layer.texture.texture_handle;
                if texture_handle.is_none() {

                    layer.texture.texture_handle = Some(ui.ctx().load_texture(layer.name.clone(), layer.texture.image_data.clone(), TextureOptions::LINEAR))
                } 
                painter.image(layer.texture.texture_handle.as_ref().unwrap().id(), canvas_rect, egui::Rect::from_min_max(Pos2::ZERO, Pos2::new(1., 1.)), Color32::from_white_alpha(255));
                
            }
          
            if ctx.app_state.current_draw_tool.clone().is_some() && canva_sense.hovered(){
                let pos = canva_sense.hover_pos().unwrap_or_default();

                let key_x = ui.ctx().input(|i| i.key_released(Key::X));
                let key_y = ui.ctx().input(|i| i.key_released(Key::Y));
                
                if key_x {
                   
                    cursor.settings.dir_locked = if cursor.settings.dir_locked == DirLocked::X {
                        DirLocked::None
                    } else {
                        DirLocked::X 
                    }
                }

                if key_y {
                     cursor.settings.dir_locked = if cursor.settings.dir_locked == DirLocked::Y {
                        DirLocked::None
                    } else {
                        DirLocked::Y
                    }
                }
                
                cursor.update_pos(pos);
                cursor.ui(&painter);
               
            }
            
        });
    }
}
