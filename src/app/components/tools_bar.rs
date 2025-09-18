use std::ops::RangeInclusive;

use egui::{Align2, Color32, CursorIcon, FontFamily, FontId, PointerButton, Pos2, Sense, Stroke, StrokeKind, Vec2};

use crate::app::{components::AppComponentExt, App};


pub struct ToolBar;


impl AppComponentExt for ToolBar {
    type Context = App;
    fn add(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        let button_size = Vec2::new(50., 20.);
        let padding = 2.5;
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                // let is_renew_open= &mut ctx.app_settings.new_paint_settings.is_open;
                let (options_sense, options_painter) = ui.allocate_painter(Vec2::new(ui.available_width(), 50.), Sense::empty());

                let new_paint_rect = egui::Rect::from_center_size(Pos2::new(options_sense.rect.min.x + 50., options_sense.rect.center().y), Vec2::new(80., 25.));
                // let mut new_paint_settings = &mut ctx.app_settings.new_paint_settings;
                options_painter.rect_filled(new_paint_rect, 5., Color32::WHITE);
                options_painter.text(new_paint_rect.center(), Align2::CENTER_CENTER, "New paint", FontId::new(12., FontFamily::Monospace), Color32::BLACK);
                
                let new_paint_sense = ui.allocate_rect(new_paint_rect, Sense::click());
                 if new_paint_sense.hovered() {
                    ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                }
                if new_paint_sense.clicked_by(PointerButton::Primary) {
                    ctx.app_settings.new_paint_settings.is_open = true;
                }

                let add_base_dir_rect = egui::Rect::from_center_size(Pos2::new(options_sense.rect.min.x + 150., options_sense.rect.center().y), Vec2::new(80., 25.));
                // let mut new_paint_settings = &mut ctx.app_settings.new_paint_settings;
                options_painter.rect_filled(add_base_dir_rect, 5., Color32::WHITE);
                options_painter.text(add_base_dir_rect.center(), Align2::CENTER_CENTER, "Set folder", FontId::new(12., FontFamily::Monospace), Color32::BLACK);
                
                let add_base_dir_sense = ui.allocate_rect(add_base_dir_rect, Sense::click());
                 if add_base_dir_sense.hovered() {
                    ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                }
                if add_base_dir_sense.clicked_by(PointerButton::Primary) {
                    ctx.set_base_directory();
                }

                let export_iamge_rect = egui::Rect::from_center_size(Pos2::new(options_sense.rect.min.x + 250., options_sense.rect.center().y), Vec2::new(80., 25.));
                options_painter.rect_filled(export_iamge_rect, 5., Color32::WHITE);
                options_painter.text(export_iamge_rect.center(), Align2::CENTER_CENTER, "Export Image", FontId::new(12., FontFamily::Monospace), Color32::BLACK);
                
                let export_image_sense = ui.allocate_rect(export_iamge_rect, Sense::click());
                if export_image_sense.hovered() {
                    ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                }
                if export_image_sense.clicked_by(PointerButton::Primary) {
                    let _saved = ctx.save_to_image();
                }
                if ctx.app_settings.new_paint_settings.is_open {
                    egui::Window::new("New paint")
                        .anchor(Align2::CENTER_CENTER, Vec2::new(0., -300.))
                        .collapsible(false)
                        .default_size(Vec2::new(300., 400.))
                        .show(ui.ctx(),|ui| {
                            ui.horizontal(|ui| {
                                
                                ui.label("New canva width");
                                ui.add(egui::DragValue::new(&mut ctx.app_settings.new_paint_settings.width).speed(5.).range(RangeInclusive::new(50.0, 1600.)));
                            });
                             ui.horizontal(|ui| {
                                
                                ui.label("New canva height");
                                ui.add(egui::DragValue::new(&mut ctx.app_settings.new_paint_settings.height).speed(5.).range(RangeInclusive::new(50.0, 1200.)));
                            });
                            ui.separator();
                            ui.horizontal(|ui| {
                                let cancel_button = ui.button("Close");
                                if cancel_button.clicked_by(PointerButton::Primary) {
                                    ctx.app_settings.new_paint_settings.is_open = false;
                                }
                                let confirm_button= ui.button("Confirm");
                                if confirm_button.clicked_by(PointerButton::Primary) {
                                    ctx.app_settings.new_paint_settings.is_open = false;
                                    ctx.re_new();
                                }
                            })
                        });
                }
            });
            ui.horizontal(|ui| {
                let (container_response, container_painter) = ui.allocate_painter(Vec2::new(1000., 25.), Sense::click());
                let starting_offset = Vec2::new(100., 0.0);
                
                let load_image_rect = egui::Rect::from_min_max(
                    Pos2::new(container_response.rect.min.x + padding, container_response.rect.min.y + padding), 
                    Pos2::new(container_response.rect.min.x + padding + 80., container_response.rect.min.y + padding + button_size.y)
                );
                container_painter.rect_filled(load_image_rect, 5., Color32::WHITE);
                container_painter.text(load_image_rect.center(), Align2::CENTER_CENTER, "load image", FontId::new(12., FontFamily::Monospace), Color32::BLACK);

                let load_image_sense = ui.allocate_rect(load_image_rect, Sense::click());
                if load_image_sense.hovered() {
                    ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                }
                if load_image_sense.clicked() {
                    ctx.load_image();
                }

                for (tool_idx, tool) in ctx.app_settings.draw_tools.tools.iter().enumerate() {
                    let button_rect = egui::Rect::from_min_max(
                        Pos2::new((button_size.x + 2. * padding) * tool_idx as f32 + container_response.rect.min.x + starting_offset.x, padding + container_response.rect.min.y + starting_offset.y), 
                        Pos2::new((button_size.x + 2. * padding) * tool_idx as f32 + button_size.x + container_response.rect.min.x + starting_offset.x, padding + button_size.y + container_response.rect.min.y + starting_offset.y)
                    );
                    let mut text_color = Color32::BLACK;
                    if let Some(current_tool) = &ctx.app_state.current_draw_tool && current_tool.id == tool.id {
                        text_color = Color32::WHITE;  
                        container_painter.rect(button_rect, 3.0, Color32::BLACK, Stroke::new(2., Color32::WHITE), StrokeKind::Middle);
                        // container_painter.text(button_rect.center(), Align2::CENTER_CENTER, format!("{:#}", tool.pencil.clone()), FontId::new(12., FontFamily::Monospace), Color32::WHITE);
                    } else {
                        container_painter.rect_filled(button_rect, 3.0, Color32::WHITE);
                    }
                    container_painter.text(button_rect.center(), Align2::CENTER_CENTER, format!("{:#}", tool.pencil.clone()), FontId::new(12., FontFamily::Monospace), text_color);
                    
    
                    
                    let button_sense = ui.allocate_rect(button_rect, Sense::click());
                    // let tool_button = ui.button(format!("{:#}", tool.draw_tool.clone()) );
                    if button_sense.hovered() {
                        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                    }
                    if button_sense.clicked_by(PointerButton::Primary) {
                        ctx.app_state.current_draw_tool = Some(tool.clone());
                        ctx.app_settings.pencil_cursor.set_pencil(tool.pencil.clone());
                    }
                }
                // ui.label("This is tool bar");
            });
            ui.add_space(10.);
            ui.horizontal(|ui| {
                let stroke_width_slider_sense= ui.add(egui::Slider::new(&mut ctx.app_state.current_stroke_width, RangeInclusive::new(1., 50.)));
                    if stroke_width_slider_sense.changed() {
                        ctx.app_settings.pencil_cursor.set_radius(ctx.app_state.current_stroke_width);
                    }
                }
            );
        });
        
    }
}
