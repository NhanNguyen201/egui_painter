use egui::{Align2, Color32, CursorIcon, FontFamily, FontId, Frame, Pos2, Sense, Vec2};
use super::AppComponentExt;
use crate::app::App;

#[derive(Clone, Copy, PartialEq)]
pub struct ColorPicker {
    pub is_dragging: bool,
    pub drag_pos: f32,
    pub r_channel: f32,
    pub g_channel: f32,
    pub b_channel: f32,
    pub a_channel: f32,
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self {
            is_dragging: false,
            drag_pos: 0.0,
            r_channel:0.0,
            g_channel: 0.0,
            b_channel: 0.0,
            a_channel: 255.0,
        }
    }
}

impl AppComponentExt for ColorPicker {
    type Context = App;
    fn add(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        Frame::canvas(ui.style()).show(ui, |ui| {
            let color_picker_size = Vec2::new(150.0, 200.);
        
            let max_channel = 255.;
            let (color_picker_rect,  picker_painter) = ui.allocate_painter(color_picker_size, Sense::empty());
            picker_painter.rect_filled(color_picker_rect.rect, 0.0, Color32::from_rgb(125, 125, 125));

            let clone_r_channel = ctx.app_settings.color_picker.r_channel;
            let r_rect_containter = egui::Rect::from_center_size(Pos2::new(color_picker_rect.rect.min.x + 30., color_picker_rect.rect.min.y + 45.), Vec2::new(25., 80.));
            picker_painter.rect_filled(r_rect_containter, 0.0, Color32::from_rgb(175, 175, 175));
            let r_rect_sense = ui.allocate_rect(r_rect_containter, Sense::click_and_drag());
            let r_height = 1. - (clone_r_channel / max_channel);
            let red_rect = egui::Rect::from_min_max(Pos2::new(r_rect_containter.min.x, r_rect_containter.min.y + r_height * r_rect_containter.height()), r_rect_containter.max);
            picker_painter.rect_filled(red_rect, 0.0, Color32::RED);
            picker_painter.text(Pos2::new(r_rect_containter.center().x, r_rect_containter.max.y + 10.), Align2::CENTER_CENTER, format!("r: {:?}", clone_r_channel.floor() as u8), FontId::new(10., FontFamily::Proportional), Color32::WHITE);

            if r_rect_sense.hovered() {
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }

            if r_rect_sense.drag_started() {
                ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);

                ctx.app_settings.color_picker.is_dragging = true;
            }

            if r_rect_sense.clicked() ||(r_rect_sense.dragged() && ctx.app_settings.color_picker.is_dragging) {
                ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);

                let interaction_pos = r_rect_sense.interact_pointer_pos();
                if let Some(pos) = interaction_pos {
                    let y_pos = ((pos.y - r_rect_containter.min.y) / r_rect_containter.height()).clamp(0.0, 1.0);
                    
                    ctx.app_settings.color_picker.r_channel = ((1. - y_pos) * max_channel).floor();
                }
            }
            if r_rect_sense.drag_stopped() {
                ui.ctx().set_cursor_icon(CursorIcon::Alias);

                ctx.app_settings.color_picker.is_dragging = false;
            }

            let clone_g_channel = ctx.app_settings.color_picker.g_channel;
            let g_rect_containter = egui::Rect::from_center_size(Pos2::new(color_picker_rect.rect.min.x + 60., color_picker_rect.rect.min.y + 45.), Vec2::new(25., 80.));
            picker_painter.rect_filled(g_rect_containter, 0.0, Color32::from_rgb(175, 175, 175));
            let g_rect_sense = ui.allocate_rect(g_rect_containter, Sense::click_and_drag());
            let g_height = 1. - (clone_g_channel / max_channel);
            
            let green_rect = egui::Rect::from_min_max(Pos2::new(g_rect_containter.min.x, g_rect_containter.min.y + g_height * g_rect_containter.height()), g_rect_containter.max);

            picker_painter.rect_filled(green_rect, 0.0, Color32::GREEN);
            picker_painter.text(Pos2::new(g_rect_containter.center().x, g_rect_containter.max.y + 10.), Align2::CENTER_CENTER, format!("g: {:?}", clone_g_channel.floor() as u8), FontId::new(10., FontFamily::Proportional), Color32::WHITE);

            
            if g_rect_sense.hovered() {
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }
            if g_rect_sense.drag_started() {
                ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);

                ctx.app_settings.color_picker.is_dragging = true;
            }

            if g_rect_sense.clicked() || (g_rect_sense.dragged() && ctx.app_settings.color_picker.is_dragging) {
                ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);

                let interaction_pos = g_rect_sense.interact_pointer_pos();
                if let Some(pos) = interaction_pos {
                    let y_pos = ((pos.y - g_rect_containter.min.y) / g_rect_containter.height()).clamp(0.0, 1.0);
                    
                    ctx.app_settings.color_picker.g_channel = ((1. - y_pos) * max_channel).floor();

                }
            }

            if g_rect_sense.drag_stopped() {
                ui.ctx().set_cursor_icon(CursorIcon::Alias);
                
                ctx.app_settings.color_picker.is_dragging = false;

            }


            let clone_b_channel = ctx.app_settings.color_picker.b_channel;
            let b_rect_containter = egui::Rect::from_center_size(Pos2::new(color_picker_rect.rect.min.x + 90., color_picker_rect.rect.min.y + 45.), Vec2::new(25., 80.));
            picker_painter.rect_filled(b_rect_containter, 0.0, Color32::from_rgb(175, 175, 175));
            let b_rect_sense = ui.allocate_rect(b_rect_containter, Sense::click_and_drag());
            let b_height = 1. - (clone_b_channel / max_channel);
            
            let blue_rect = egui::Rect::from_min_max(Pos2::new(b_rect_containter.min.x, b_rect_containter.min.y + b_height * b_rect_containter.height()), b_rect_containter.max);
            picker_painter.rect_filled(blue_rect, 0.0, Color32::BLUE);
            picker_painter.text(Pos2::new(b_rect_containter.center().x, b_rect_containter.max.y + 10.), Align2::CENTER_CENTER, format!("b: {:?}", clone_b_channel.floor() as u8), FontId::new(10., FontFamily::Proportional), Color32::WHITE);


            if b_rect_sense.hovered() {
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }

            if b_rect_sense.drag_started() {
                ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);

                ctx.app_settings.color_picker.is_dragging = true;
            }
            if b_rect_sense.clicked() ||(b_rect_sense.dragged() && ctx.app_settings.color_picker.is_dragging) {
                ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);

                let interaction_pos = b_rect_sense.interact_pointer_pos();
                if let Some(pos) = interaction_pos {
                    let y_pos = ((pos.y - b_rect_containter.min.y) / b_rect_containter.height()).clamp(0.0, 1.0);
                  
                    ctx.app_settings.color_picker.b_channel = ((1. - y_pos) * max_channel).floor();

                }
            }
            if b_rect_sense.drag_stopped() {
                ui.ctx().set_cursor_icon(CursorIcon::Alias);

                ctx.app_settings.color_picker.is_dragging = false;
            }
            
            let clone_a_channel: f32 = ctx.app_settings.color_picker.a_channel;
            let a_rect_containter = egui::Rect::from_center_size(Pos2::new(color_picker_rect.rect.min.x + 120., color_picker_rect.rect.min.y + 45.), Vec2::new(25., 80.));
            picker_painter.rect_filled(a_rect_containter, 0.0, Color32::from_rgb(175, 175, 175));
            let a_rect_sense = ui.allocate_rect(a_rect_containter, Sense::click_and_drag());
            let a_height = 1. - (clone_a_channel / max_channel);
            
            let alpha_rect = egui::Rect::from_min_max(Pos2::new(a_rect_containter.min.x, a_rect_containter.min.y + a_height * a_rect_containter.height()), a_rect_containter.max);
            picker_painter.rect_filled(alpha_rect, 0.0, Color32::BLACK);
            picker_painter.text(Pos2::new(a_rect_containter.center().x, a_rect_containter.max.y + 10.), Align2::CENTER_CENTER, format!("a: {:.02}", clone_a_channel.floor()  / max_channel), FontId::new(10., FontFamily::Proportional), Color32::WHITE);


            if a_rect_sense.hovered() {
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }

            if a_rect_sense.drag_started() {
                ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);

                ctx.app_settings.color_picker.is_dragging = true;
            }
            if a_rect_sense.clicked() || (a_rect_sense.dragged() && ctx.app_settings.color_picker.is_dragging) {
                ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);

                let interaction_pos = a_rect_sense.interact_pointer_pos();
                if let Some(pos) = interaction_pos {
                    let y_pos = ((pos.y - a_rect_containter.min.y) / a_rect_containter.height()).clamp(0.0, 1.0);
                  
                    ctx.app_settings.color_picker.a_channel = ((1. - y_pos) * max_channel).floor();

                }
            }
            if a_rect_sense.drag_stopped() {
                ui.ctx().set_cursor_icon(CursorIcon::Alias);

                ctx.app_settings.color_picker.is_dragging = false;
            }

            picker_painter.text(Pos2::new(color_picker_rect.rect.center().x, r_rect_containter.max.y + 50.), Align2::CENTER_CENTER, "Click to \n overwrite the color", FontId::new(12., FontFamily::Monospace), Color32::WHITE);
            let result_rect = egui::Rect::from_min_max(Pos2::new(r_rect_containter.min.x, r_rect_containter.max.y + 75. ), Pos2::new(b_rect_containter.max.x, b_rect_containter.max.y + 105.));
            let result_color = Color32::from_rgba_unmultiplied(ctx.app_settings.color_picker.r_channel.floor() as u8, ctx.app_settings.color_picker.g_channel.floor() as u8, ctx.app_settings.color_picker.b_channel.floor() as u8, ctx.app_settings.color_picker.a_channel.floor() as u8);
            picker_painter.rect_filled(result_rect, 2., result_color.clone()) ;
            let result_sense = ui.allocate_rect(result_rect, Sense::click());
            if result_sense.hovered() {
                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
            }
            if result_sense.clicked() {
                if let Some(color) = &mut ctx.app_state.current_color {
                    if let Some(find_color) = ctx.app_state.color_palette.iter_mut().find(|c| c.id == color.id) {
                        find_color.color = result_color.clone();
                    };
                    color.color = result_color;
                }
            }
        });

        
    }
}