use egui::{Color32, Event, Frame, PointerButton, Pos2, Sense, TextureOptions, Vec2};

use super::AppComponentExt;
use crate::app::{components::utils::layer, App};
pub struct Canvas;


// let frame_style = egui::Style::clone_from(&mut self, source);
impl AppComponentExt for Canvas {
    type Context = App;
    fn add(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        Frame::canvas(ui.style()).outer_margin(0.).show(ui, |ui| {
           
            let container_canvas_size =  Vec2::new(900., 600.);
            let background_block_size = 50.;
            let background_block_color = Vec::from([Color32::from_rgb(150, 150, 150), Color32::from_rgb(50, 50, 50)]);
            // Make the canvas canvas_container_painter clip to the canvas area
            let cursor = &mut ctx.app_settings.pencil_cursor;
            
            let (canva_container_response, canvas_container_painter) = ui.allocate_painter(container_canvas_size, Sense::click());
            // paint the background
            for background_row_block in 0..(container_canvas_size.y / background_block_size).floor() as usize {
                for background_col_block in 0..(container_canvas_size.x / background_block_size).floor() as usize  {
                    let mut color = background_block_color[0].clone();
                    if background_row_block % 2  == background_col_block % 2 {
                        color = background_block_color[1].clone();
                    } 
                    canvas_container_painter.rect_filled(
                        egui::Rect::from_min_max(
                            canva_container_response.rect.min + Vec2::new(background_block_size * background_col_block as f32, background_block_size * background_row_block as f32), 
                            canva_container_response.rect.min + Vec2::new(background_block_size * background_col_block as f32 + background_block_size, background_block_size * background_row_block as f32 + background_block_size)
                        ), 
                        0.0, 
                        color);
                }
            }; 

            // let canvas_rect = egui::Rect::from_center_size(canva_container_response.rect.center(), ctx.app_settings.layer_size);
            // let example_rect= egui::Rect::from_center_size(canva_container_response.rect.center(), Vec2::new(2000., 1000.0));
            let raw_canvas_rect = egui::Rect::from_center_size(
                canva_container_response.rect.center() + ctx.app_state.layers_container.transform.position.to_vec2(), 
                ctx.app_settings.layer_size * ctx.app_state.layers_container.transform.scale
            );
            let clamped_canvas_rect = egui::Rect::from_min_max(
                Pos2::new(raw_canvas_rect.min.x.max(canva_container_response.rect.min.x), raw_canvas_rect.min.y.max(canva_container_response.rect.min.y)),  
                Pos2::new(raw_canvas_rect.max.x.min(canva_container_response.rect.max.x), raw_canvas_rect.max.y.min(canva_container_response.rect.max.y))
            );
            let clamped_canvas_uv = egui::Rect::from_min_max(
                Pos2::new(
                    ((clamped_canvas_rect.min.x - raw_canvas_rect.min.x) / raw_canvas_rect.width()).max(0.), 
                    ((clamped_canvas_rect.min.y - raw_canvas_rect.min.y) / raw_canvas_rect.height()).max(0.)
                ), 
                Pos2::new(
                    ((clamped_canvas_rect.max.x - raw_canvas_rect.min.x) / raw_canvas_rect.width()).min(1.), 
                    ((clamped_canvas_rect.max.y - raw_canvas_rect.min.y) / raw_canvas_rect.height()).min(1.)
                )
            );
            // canvas_container_painter.
            let clamped_canva_sense = ui.allocate_rect(clamped_canvas_rect, Sense::click_and_drag());
            canvas_container_painter.rect_filled(clamped_canvas_rect,0.0, Color32::from_rgb(250, 250, 250));
            
            if  clamped_canva_sense.drag_started_by(PointerButton::Primary) {
                ctx.app_state.is_dragging = true;
            }
           
            if clamped_canva_sense.clicked_by(PointerButton::Primary) || (clamped_canva_sense.dragged_by(PointerButton::Primary) &&  ctx.app_state.is_dragging) {
             
                let pos = (cursor.get_pos() - raw_canvas_rect.min.to_vec2()) / ctx.app_state.layers_container.transform.scale;
                let brush_size = ctx.app_state.current_stroke_width;
                let color = ctx.app_state.current_color.clone().unwrap_or_default().color;
                if let Some(layer_id) = ctx.app_state.current_layer {
                    if let Some(layer) = ctx.app_state.layers_container.layers.iter_mut().find(|layer| layer.id == layer_id) {
                        layer.texture.paint_at(
                            pos, 
                            ctx.app_state.current_draw_tool.clone().unwrap().pencil, 
                            brush_size, 
                            color
                        );
                    }
                }
            }
            if clamped_canva_sense.drag_stopped_by(PointerButton::Primary) {
                ctx.app_state.is_dragging = false
            }

            if  clamped_canva_sense.drag_started_by(PointerButton::Secondary) {
                ctx.app_state.layers_container.is_dragged = true;
                ctx.app_state.layers_container.dragged_offset = clamped_canva_sense.interact_pointer_pos().unwrap_or_default() - ctx.app_state.layers_container.transform.position;
            }
           
            if  clamped_canva_sense.dragged_by(PointerButton::Secondary) &&  ctx.app_state.layers_container.is_dragged {
             
                
                if let Some(drag_pos) = clamped_canva_sense.interact_pointer_pos() {
                  
                    ctx.app_state.layers_container.transform.position = drag_pos -  ctx.app_state.layers_container.dragged_offset;
                } 
                
            }
            if clamped_canva_sense.drag_stopped_by(PointerButton::Secondary) {
                ctx.app_state.layers_container.is_dragged = false;

            }

           
            if ui.input(|i| i.events.len() > 0) {
               
                let events = ui.input(|i| i.events.clone());
                for event in events.iter() {
                    if let Event::MouseWheel { unit: _, delta, modifiers: _ } = event {
                        ctx.app_state.layers_container.transform.scale +=  delta.y * 0.05;
                    }
                }
               
            }
            for layer in ctx.app_state.layers_container.layers.clone().iter_mut().filter(|layer| layer.is_visible).rev(){
                let texture_handle = &layer.texture.texture_handle;
                if texture_handle.is_none() {

                    layer.texture.texture_handle = Some(ui.ctx().load_texture(layer.name.clone(), layer.texture.image_data.clone(), TextureOptions::LINEAR))
                } 
                canvas_container_painter.image(
                    layer.texture.texture_handle.as_ref().unwrap().id(), 
                    clamped_canvas_rect, 
                    clamped_canvas_uv, 
                    Color32::from_white_alpha(255)
                );
                
            }
          
            if ctx.app_state.current_draw_tool.clone().is_some() && clamped_canva_sense.hovered(){
                let pos = clamped_canva_sense.hover_pos().unwrap_or_default();

                
                cursor.settings.add_lock_event(ui);
                cursor.set_radius(ctx.app_state.current_stroke_width * ctx.app_state.layers_container.transform.scale);
                cursor.update_pos(pos);
                if !ctx.app_state.layers_container.is_dragged {
                    cursor.ui(&canvas_container_painter)
                };
               
            }
            
        });
    }
}
