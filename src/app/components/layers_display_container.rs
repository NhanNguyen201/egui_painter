use egui::{Align2, Color32, CursorIcon, FontFamily, FontId, Frame, Pos2, Sense, Vec2};
use super::AppComponentExt;
use crate::app::{components::utils::{layer::{Layer, LayerTexture}, new_rand_id}, App};


pub struct LayersDisplayContainer;



impl AppComponentExt for LayersDisplayContainer {
    type Context = App;
    fn add(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        let container_height: f32 = 450.;
        let layer_size =  Vec2::new(150., 30.);
        
        Frame::canvas(ui.style()).show(ui, |ui| {
            ui.set_max_width(200.);
          
            ui.vertical(|ui| {
                let (title_rect, title_painter) = ui.allocate_painter(Vec2::new(ui.available_width(), 50.0), Sense::click());
                title_painter.rect_filled( egui::Rect { min: title_rect.rect.min + Vec2::new(5., 5.), max: title_rect.rect.max - Vec2::new(5., 5.)}, 2., Color32::WHITE);
                title_painter.text(Pos2::new(title_rect.rect.min.x + 20., title_rect.rect.center().y), Align2::LEFT_CENTER, "Layers ", FontId::new(18., FontFamily::Monospace), Color32::BLACK);
                let add_layer_rect = egui::Rect::from_center_size( Pos2::new(title_rect.rect.max.x - 70., title_rect.rect.center().y), Vec2::new(70., 25.));
                let add_layer_sense = ui.allocate_rect(add_layer_rect.clone(), Sense::click());
                title_painter.rect_filled(add_layer_rect, 2., Color32::from_rgb(50, 50, 50));
                title_painter.text(Pos2::new(add_layer_rect.min.x + 10., add_layer_rect.center().y), Align2::LEFT_CENTER, "Add layer ", FontId::new(12., FontFamily::Proportional), Color32::WHITE);
                if add_layer_sense.hovered() {
                    ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                }
                if add_layer_sense.clicked() {
                    let new_layer: Layer = Layer {id: new_rand_id(), name: format!("Layer {}", (ctx.app_state.layers_container.layers.len() + 1).to_string()), is_visible: true, texture: LayerTexture::new(ctx.app_settings.canvas_size.x as usize, ctx.app_settings.canvas_size.y as usize)};
                    ctx.app_state.layers_container.layers.insert(0, new_layer.clone());
                    ctx.app_state.current_layer = Some(new_layer.id);
                }
                ui.add_space(10.);
                ui.separator();
                egui::ScrollArea::vertical().max_height(container_height).show(ui, |ui| {
                    // ui.ctx().set_style(style);
                    ui.set_height(container_height);
                    ui.set_width(ui.available_width());
                    ui.vertical(|ui| {
                        for (_layer_idx, layer) in ctx.app_state.layers_container.layers.iter_mut().enumerate() {
                            
                            let (layer_rect, layer_painter) = ui.allocate_painter(layer_size, Sense::click());
                            layer_painter.rect_filled(layer_rect.rect, 0.0, Color32::from_rgb(100, 100, 100));
                            layer_painter.text(Pos2::new(layer_rect.rect.min.x + 10., layer_rect.rect.center().y), Align2::LEFT_CENTER, format!("{}", layer.name), FontId::new(16., FontFamily::Proportional), Color32::WHITE);
                            
                            if let Some(active_layer) = ctx.app_state.current_layer {
                                if layer.id == active_layer {
                                   
                                    layer_painter.circle_filled( Pos2::new(layer_rect.rect.min.x , layer_rect.rect.center().y), 3., Color32::BLACK);
                                }
                            }
                             if layer_rect.hovered() {
                                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                             }
                            if layer_rect.clicked() {
                                ctx.app_state.current_layer = Some(layer.id);
                            }
                        
                            let visible_rect_container = egui::Rect::from_center_size(Pos2::new(layer_rect.rect.max.x - 30., layer_rect.rect.center().y), Vec2::new(30., 20.));
                            layer_painter.rect_filled(visible_rect_container, 2., Color32::WHITE);

                            if layer.is_visible {
                                layer_painter.text(visible_rect_container.center(), Align2::CENTER_CENTER, format!("{}", egui_phosphor::regular::EYE), FontId::new(16., FontFamily::Proportional), Color32::BLACK);
                            } else {
                                layer_painter.text(visible_rect_container.center(), Align2::CENTER_CENTER, format!("{}", egui_phosphor::regular::EYE_SLASH), FontId::new(16., FontFamily::Proportional), Color32::BLACK);

                            }
                            let visible_sense =  ui.allocate_rect(visible_rect_container, Sense::click());
                          
                            if visible_sense.clicked() {
                                layer.is_visible = !layer.is_visible;
                            }
                            
                        }
                    });
                });
            });
        });
    }
}