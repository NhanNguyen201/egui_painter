
use std::ops::RangeInclusive;

use egui::{Align, Align2, Color32, ColorImage, CursorIcon, Frame, Layout, PointerButton, Pos2, Sense, Stroke, StrokeKind, TextureHandle, Vec2, Window};
use image::imageops::FilterType;
use image::DynamicImage;

use crate::app::components::utils::layer::{Layer, LayerTexture, Transform};
use crate::app::components::utils::new_rand_id;
use crate::app::components::AppComponentExt;
use crate::app::App;

#[derive(Clone, PartialEq)]
pub struct CropRect {
    pub top: f32,
    pub left: f32,
    pub bottom: f32,
    pub right: f32
}

#[derive(Clone, PartialEq)]
pub struct DragModifier {
    pub is_dragging: bool,
    pub drag_pos: Pos2,
    pub drag_offset: Vec2
}

impl Default for DragModifier {
    fn default() -> Self {
        Self {
            is_dragging: false,
            drag_offset: Vec2::ZERO,
            drag_pos: Pos2::ZERO
        }
    }
}
impl Default for CropRect {
    fn default() -> Self {
        Self { top: 0.0, left: 0.0, bottom: 0.0, right: 0.0 }
    }
}

pub trait FitIn {
    fn fit_in(&self, target: egui::Rect) -> Option<egui::Rect>;
}

impl FitIn for egui::Rect {
    fn fit_in(&self, target: egui::Rect) -> Option<egui::Rect> {
        if self.min.x < target.min.x && self.min.y < target.min.y && self.max.x > target.max.x && self.max.y > target.max.y {
            None
        } else {
            Some(self.intersect(target))
        }
    }
}
#[derive(Clone, PartialEq)]
pub struct ImportImageWidget {
    pub is_open: bool,
    pub texture: Option<Texture>,
    pub original_scale: f32,
    pub transform: Transform,
    pub crop: CropRect,
    pub drag_modifier: DragModifier,
    pub draw_rect: Option<egui::Rect>,
    pub background_layer_rect: Option<egui::Rect>,
    pub texture_rect: Option<egui::Rect>,
    pub scale_factor: f32
}

impl Default for ImportImageWidget {
    fn default() -> Self {
        Self {
            is_open: false,
            texture: None,
            original_scale: 1.0,
            transform: Transform::default(),
            crop: CropRect::default(),
            drag_modifier: DragModifier::default(),
            draw_rect: None,
            background_layer_rect: None,
            texture_rect: None,
            scale_factor: 1.0
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Texture {
    pub dyn_image: DynamicImage,
    pub texture_handle: TextureHandle
}


impl AppComponentExt for ImportImageWidget {
    type Context = App;
    fn add(ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        if ctx.app_settings.import_image_widget.is_open == false || ctx.app_settings.import_image_widget.texture.is_none() {
            return;
        }
        let max_size = 500.;
        let widget = &mut ctx.app_settings.import_image_widget ;
        let texture = &mut widget.texture;
        let original_scale = &mut widget.original_scale;
        let transform = &mut widget.transform;
        let crop = &mut widget.crop;
        let drag_modifier = &mut widget.drag_modifier;
        let cloned_texture = texture.clone().unwrap();
        let container_size = Vec2::new(1000., 800.);
        Window::new("Import image")
            .title_bar(false)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    Frame::canvas(ui.style()).outer_margin(0.).show(ui, |ui| { 
                        let (container_sense, container_painter) =  ui.allocate_painter(container_size, Sense::click());
                        
                        let scaled_image_size = Vec2::new(
                            cloned_texture.dyn_image.width() as f32 * *original_scale * transform.scale,
                            cloned_texture.dyn_image.height() as f32 * *original_scale * transform.scale,
                        );
                        let unclamped_rect = egui::Rect::from_center_size(
                            container_sense.rect.center() + transform.position.to_vec2(), 
                            scaled_image_size
                        );
                        let texture_rect = egui::Rect {
                            min: Pos2::new(
                                unclamped_rect.min.x.max(container_sense.rect.min.x).min(container_sense.rect.max.x),
                                unclamped_rect.min.y.max(container_sense.rect.min.y ).min(container_sense.rect.max.y)
                            ),
                            max: Pos2::new(
                                unclamped_rect.max.x.min(container_sense.rect.max.x).max(container_sense.rect.min.x),
                                unclamped_rect.max.y.min(container_sense.rect.max.y).max(container_sense.rect.min.y)
                            ),
                        };
                        let uv_rect = egui::Rect::from_min_max(
                            egui::Pos2::new(
                                ((texture_rect.min.x - (unclamped_rect.min.x)) / scaled_image_size.x).max(0.0),
                                ((texture_rect.min.y - (unclamped_rect.min.y)) / scaled_image_size.y).max(0.0)
                            ),
                            egui::Pos2::new(
                                ((texture_rect.max.x - (unclamped_rect.min.x)) / scaled_image_size.x).min(1.0),
                                ((texture_rect.max.y - (unclamped_rect.min.y)) / scaled_image_size.y).min(1.0)
                            )
                        );
                        widget.texture_rect = Some(texture_rect.clone());
                        let image_drag_sense = ui.allocate_rect(texture_rect, Sense::click_and_drag());
                        if image_drag_sense.hovered() {
                            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                        }
                        if image_drag_sense.drag_started_by(PointerButton::Primary) {
                            drag_modifier.is_dragging = true;
                            drag_modifier.drag_pos = image_drag_sense.interact_pointer_pos().unwrap_or(Pos2::ZERO);
                            drag_modifier.drag_offset = transform.position.to_vec2() - drag_modifier.drag_pos.to_vec2();
                        }
                        if image_drag_sense.dragged_by(PointerButton::Primary) && drag_modifier.is_dragging {
                            let current_pos = image_drag_sense.interact_pointer_pos().unwrap_or(Pos2::ZERO);
                            let new_pos = current_pos.to_vec2() + drag_modifier.drag_offset;
                            transform.position = egui::Pos2::new(new_pos.x, new_pos.y).into();
                        }
                        if image_drag_sense.drag_stopped() {
                            drag_modifier.is_dragging = false;
                        }
                        container_painter.image(cloned_texture.texture_handle.id(), texture_rect, uv_rect, Color32::WHITE);
                        // Layer preview
                        let layer_ratio = ctx.app_settings.layer_size.x / ctx.app_settings.layer_size.y;
                        let scaled_layer_size = if layer_ratio > 1. {
                            Vec2::new(max_size, max_size / layer_ratio)
                        } else {
                            Vec2::new(max_size * layer_ratio, max_size)
                        };
                        let background_layer_rect = egui::Rect::from_center_size(container_sense.rect.center(), scaled_layer_size);
                        widget.background_layer_rect = Some(background_layer_rect.clone());
                        container_painter.rect(background_layer_rect, 0., Color32::from_rgba_unmultiplied(100, 100, 100, 65), Stroke::new(1., Color32::from_rgb(200, 200, 200)), StrokeKind::Middle);
                        // Crop display 
                        let crop_rect = egui::Rect::from_min_max(
                            Pos2::new(background_layer_rect.min.x + crop.left, background_layer_rect.min.y + crop.top), 
                            Pos2::new(background_layer_rect.max.x + crop.right, background_layer_rect.max.y + crop.bottom)
                        );
                        container_painter.rect_stroke(crop_rect, 0., Stroke::new(2., Color32::from_rgba_unmultiplied(0, 225, 225, 225)), StrokeKind::Middle);
                        
                        
                        let corner_size = Vec2::new(15., 15.);
                        let top_left_rect = egui::Rect::from_center_size(crop_rect.min, corner_size);
                        container_painter.rect(top_left_rect, 1., Color32::from_rgba_unmultiplied(255, 255, 255, 255), Stroke::new(2., Color32::from_rgba_unmultiplied(255, 0, 225, 125)), StrokeKind::Middle);
                        let top_left_sense = ui.allocate_rect(top_left_rect, Sense::click_and_drag());
                        if top_left_sense.hovered() {
                            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                        };
                        if top_left_sense.drag_started_by(PointerButton::Primary) {
                            drag_modifier.is_dragging = true;
                        }
                        if top_left_sense.dragged_by(PointerButton::Primary) && drag_modifier.is_dragging {
                            let mouse_pos = top_left_sense.interact_pointer_pos().unwrap_or(Pos2::ZERO).clone();
                            crop.top = (mouse_pos.y - background_layer_rect.min.y).min(background_layer_rect.height() + crop.bottom).max(0.);
                            crop.left = (mouse_pos.x - background_layer_rect.min.x).min(background_layer_rect.width() + crop.right).max(0.);

                        }
                        if top_left_sense.drag_stopped() {
                            drag_modifier.is_dragging = false;
                        }

                        let bottom_right_rect = egui::Rect::from_center_size(crop_rect.max, corner_size);
                        container_painter.rect(bottom_right_rect, 1., Color32::from_rgba_unmultiplied(255, 255, 255, 255), Stroke::new(2., Color32::from_rgba_unmultiplied(225, 0, 225, 125)), StrokeKind::Middle);
                        let bottom_right_sense = ui.allocate_rect(bottom_right_rect, Sense::click_and_drag());
                        if bottom_right_sense.hovered() {
                            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                        };
                        if bottom_right_sense.drag_started_by(PointerButton::Primary) {
                            drag_modifier.is_dragging = true;
                        }
                        if bottom_right_sense.dragged_by(PointerButton::Primary) && drag_modifier.is_dragging {
                            let mouse_pos = bottom_right_sense.interact_pointer_pos().unwrap_or(Pos2::ZERO).clone();
                            crop.bottom = (mouse_pos.y - background_layer_rect.max.y).min(0.).max((background_layer_rect.height() - crop.top.clone()) * -1.);
                            crop.right = (mouse_pos.x - background_layer_rect.max.x).min(0.).max((background_layer_rect.width() - crop.left.clone()) * -1.);

                        }
                        if bottom_right_sense.drag_stopped() {
                            drag_modifier.is_dragging = false;
                        }
                        if let Some(draw_rect) = crop_rect.fit_in(texture_rect).or(texture_rect.fit_in(crop_rect)) {
                            container_painter.rect(draw_rect, 0.0, Color32::from_rgba_unmultiplied(255, 255, 255, 125), Stroke::new(2., Color32::from_rgb(255, 0, 255)), StrokeKind::Middle);
                            widget.draw_rect = Some(draw_rect);
                            // container_painter.rect_stroke(draw_rect, 0., Stroke::new(2., Color32::from_rgba_unmultiplied(225, 0, 0, 125)), StrokeKind::Middle);
                        }
                    });
                    ui.add_space(5.);
                    ui.with_layout(Layout::bottom_up(Align::Min),|ui| {
                        ui.set_width(300.);
                        ui.add_space(5.);
                        ui.allocate_ui_with_layout(Vec2::new(ui.available_width(), 100.), Layout::left_to_right(Align::Max),|ui|{
                            let cancel_button = ui.button("Cancel");
                            if cancel_button.clicked() {
                               widget.is_open = false;
                               *texture = None;
                               widget.draw_rect = None;
                               *transform = Transform::default();
                               *crop = CropRect::default();
                               *drag_modifier = DragModifier::default();
                               *original_scale = 1.;
                             
                            //    ctx.app_settings.import_image_widget = ImportImageWidget::default();
                            }
                            let confirm_button = ui.button("Confirm");
                            if confirm_button.clicked() {
                                if let Some(draw_rect) = widget.draw_rect.clone()  {
                                    let scale_factor = widget.scale_factor.clone();
                                    let mut new_image_layer: Layer = Layer {
                                        id: new_rand_id(), 
                                        name: "Image layer".to_string(), 
                                        is_visible: true,
                                        texture: LayerTexture::new(ctx.app_settings.layer_size.x.floor() as usize, ctx.app_settings.layer_size.y.floor() as usize)
                                    };

                                  
                                    let offset_to_layer = (draw_rect.clone().min.to_vec2() - widget.background_layer_rect.unwrap().clone().min.to_vec2()) * scale_factor.clone();
                                    let scaled_raw_rect_size = draw_rect.size().clone() * scale_factor.clone();

                                    let image_scale = original_scale.clone() * transform.scale.clone();
                                    let offset_to_image =  (draw_rect.clone().min.to_vec2() - widget.texture_rect.unwrap().clone().min.to_vec2()) * scale_factor.clone();
                                    // offset_to_image.x = offset_to_image.x / image_scale.clone();
                                    let scaled_dyn_image = cloned_texture.dyn_image.resize(
                                        (scale_factor.clone() * image_scale.clone() * cloned_texture.texture_handle.size()[0] as f32).ceil() as u32,
                                        (scale_factor.clone() * image_scale.clone() * cloned_texture.texture_handle.size()[1] as f32).ceil() as u32, 
                                        FilterType::Nearest
                                    );
                                    let scaled_color_image = ColorImage::from_rgba_unmultiplied(
                                        [scaled_dyn_image.width() as _, scaled_dyn_image.height() as _],
                                        scaled_dyn_image.to_rgba8().as_flat_samples().as_slice(),
                                    );
                                    for p_x in 0..scaled_raw_rect_size.clone().x.floor() as u32 {
                                        for p_y in 0..scaled_raw_rect_size.clone().y.floor() as u32 {
                                            let offset = offset_to_layer.clone();
                                            let x_cord = p_x as f32 + offset.clone().x;
                                            let y_cord = p_y as f32 + offset.clone().y;
                                            let img_x_cord = p_x as f32 + offset_to_image.clone().x;
                                            let img_y_cord = p_y as f32 + offset_to_image.clone().y;
                                            // if x_cord < 0. || y_cord < 0. || x_cord >= new_image_layer.texture.layer_size.x || y_cord >= new_image_layer.texture.layer_size.y {
                                            //     continue;
                                            // }
                                            // println!("point: x: {:#}, y: {:#}", x_cord, y_cord);
                                            let point_position = (y_cord * new_image_layer.texture.layer_size.clone().x).floor() as usize + x_cord.floor() as usize;
                                            let image_point_position = (img_y_cord  * scaled_color_image.size[0] as f32).floor() as usize + img_x_cord.floor() as usize; 
                                            new_image_layer.texture.image_data.pixels[point_position] = scaled_color_image.pixels[image_point_position];
                                            
                                           
                                        }
                                    }
                                    if let Some(active_layer) = ctx.app_state.current_layer  {
                                        if let Some(find_index) = ctx.app_state.layers_container.layers.iter().position(|l| l.id == active_layer) {
                                            ctx.app_state.current_layer = Some(new_image_layer.id.clone());
                                            if find_index == 0 {
                                                ctx.app_state.layers_container.layers.insert(0, new_image_layer);
                                            } else {
                                                ctx.app_state.layers_container.layers.insert(find_index - 1, new_image_layer);

                                            }
                                        }
                                        
                                    } else {
                                        ctx.app_state.layers_container.layers.insert(0, new_image_layer);

                                    }
                                }
                                widget.is_open = false;
                                *texture = None;
                                widget.draw_rect = None;
                                *transform = Transform::default();
                                *crop = CropRect::default();
                                *drag_modifier = DragModifier::default();
                                *original_scale = 1.;

                            }
                        });
                        ui.allocate_ui_with_layout(Vec2::new(ui.available_width(), ui.available_height()), Layout::top_down(Align::Min), |ui| {
                            ui.add_space(5.);
                            ui.add(egui::DragValue::new(&mut transform.scale).speed(0.05).prefix("Image scale: ").range(RangeInclusive::new(0.1, 4.0)).clamp_existing_to_range(false));
                            ui.label(format!("Position: x: {:.1}, y: {:.1}", transform.position.x, transform.position.y));
                        });
                    });
                });
                
            });
    }
}