use egui::{ Align2, Color32, CursorIcon, FontFamily, FontId, Frame, PointerButton, Pos2, Sense, Stroke, StrokeKind, Vec2};

use super::AppComponentExt;
use crate::app::{components::utils::{layer::PaintColor, new_rand_id}, App};

pub struct ColorPalette;



impl AppComponentExt for ColorPalette {
    type Context = App;
    fn add(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        let palette_size: Vec2 = Vec2::new(150., 350.);
        let padding: f32 = 2.5;
        let color_block_size: f32 = 20.;
        Frame::canvas(ui.style()).show(ui, |ui| {
            let (palette_response, palette_painter) = ui.allocate_painter(palette_size, Sense::click());
            let palette = palette_response.rect;
            palette_painter.rect_filled(palette, 0., Color32::from_rgb(200, 200, 200));
            let block_per_row = (palette_size.x / (color_block_size + padding * 2.)).floor();

            for (color_idx, paint_color) in ctx.app_state.color_palette.iter_mut().enumerate() {

                let row = (color_idx as f32 / block_per_row).floor();
                let col = color_idx % block_per_row.floor()  as usize;

                let color_rect = egui::Rect::from_min_max(
                    Pos2::new(palette.min.x + (color_block_size + padding * 2.) * col as f32 + padding, palette.min.y + (color_block_size + padding * 2.) * row  + padding),
                    Pos2::new(palette.min.x + (color_block_size + padding * 2.) * col as f32 + padding + color_block_size, palette.min.y + (color_block_size + padding * 2.) * row + padding + color_block_size) 
                );
                palette_painter.rect_filled(color_rect, 2., paint_color.color);
                // let color_block = ui.al
                let color_block_sense = ui.allocate_rect(color_rect, Sense::click());
                if let Some(active_color) = ctx.app_state.current_color.as_ref() {
                    if active_color.id == paint_color.id {
                        palette_painter.rect_stroke(
                            egui::Rect::from_min_max(color_rect.min - Vec2::new(2.5, 2.5), color_rect.max + Vec2::new(2.5, 2.5)), 
                            0., 
                            Stroke::new(2., Color32::BLACK), 
                            StrokeKind::Middle
                        );
                    }
                }
                if color_block_sense.hovered() {
                    ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                }
                if color_block_sense.clicked_by(PointerButton::Primary) {
                    ctx.app_settings.color_picker.r_channel = paint_color.color.r() as f32;
                    ctx.app_settings.color_picker.g_channel = paint_color.color.g() as f32;
                    ctx.app_settings.color_picker.b_channel = paint_color.color.b() as f32;
                    ctx.app_settings.color_picker.a_channel = paint_color.color.a() as f32;
                    ctx.app_state.current_color = Some(paint_color.clone());
                }

            }
            let add_color_row = ((ctx.app_state.color_palette.len() ) as f32 / block_per_row).floor();
            let add_color_col = (ctx.app_state.color_palette.len()) % block_per_row.floor() as usize;
            
            let add_color_rect = egui::Rect::from_min_max(
                Pos2::new(palette.min.x + (color_block_size + padding * 2.) * add_color_col as f32 + padding, palette.min.y + (color_block_size + padding * 2.) * add_color_row + padding),
                Pos2::new(palette.min.x + (color_block_size + padding * 2.) * add_color_col as f32 + padding + color_block_size, palette.min.y + (color_block_size + padding * 2.) * add_color_row + padding + color_block_size) 
            );
            palette_painter.rect_filled(add_color_rect, 2., Color32::WHITE);
            palette_painter.text(add_color_rect.center(), Align2::CENTER_CENTER, "+", FontId::new(12., FontFamily::Proportional), Color32::BLACK);
            let add_color_sense = ui.allocate_rect(add_color_rect, Sense::click());
            if add_color_sense.clicked_by(PointerButton::Primary) {
                
                ctx.app_state.color_palette.push(PaintColor { color: Color32::BLACK, id: new_rand_id() });
                
            }
   
          
        });
    }
}