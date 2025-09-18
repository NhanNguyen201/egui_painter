pub mod components;


use std::path::PathBuf;

use egui::{Color32, ColorImage, Id, Pos2, Vec2};


use components::{AppComponentExt, canvas::Canvas};
use image::imageops::FilterType;
use image::{ImageBuffer};
use rfd::FileDialog;

use crate::app::components::utils::create_paint::NewPaintSetting;
use crate::app::components::utils::image_color::composite_layers;
use crate::app::components::utils::layer::LayersContainer;
use crate::app::components::{
    color_palette::ColorPalette,
    color_picker::ColorPicker,
    layers_display_container::LayersDisplayContainer,
    tools_bar::ToolBar
};

use crate::app::components::utils::{
    draw_tool::{DrawTool, Tools},
    layer::{Layer, LayerTexture, PaintColor},
    new_rand_id,
    pencil_cursor::PencilCursor
};

pub struct App {
    app_settings: AppSettings,
    app_state: AppState
}

#[derive(Clone, PartialEq)]
pub struct AppSettings {
    layer_size: Vec2,
    color_picker: ColorPicker,
    pencil_cursor: PencilCursor,
    draw_tools: Tools,
    base_dir: Option<PathBuf>,
    new_paint_settings: NewPaintSetting
}

impl Default for AppSettings {
    fn default() -> Self {
        
        
        Self {
            layer_size: Vec2::new(500.0, 500.0),
            color_picker: ColorPicker::default(),
            draw_tools: Tools::default(),
            pencil_cursor: PencilCursor::default(),
            new_paint_settings: NewPaintSetting::default(),
            base_dir: None
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct AppState {
    is_dragging: bool,
    poses: Vec<Pos2>,
    layers_container: LayersContainer,
    
    current_layer: Option<Id>,
  
    color_palette: Vec<PaintColor>,
    current_color: Option<PaintColor>,
    current_stroke_width: f32,
    current_draw_tool: Option<DrawTool>
}


impl AppState {
    pub fn from_settings(settings: AppSettings) -> Self  {
        let default_layer = Layer {
            id: new_rand_id(), 
            name: "Layer 1".to_string(), 
            is_visible: true,
            texture: LayerTexture::new(settings.layer_size.x as usize, settings.layer_size.y as usize)
        };
        let mut palette: Vec<PaintColor> = Vec::new();
        let mut layers_container = LayersContainer::default();
        layers_container.layers.push(default_layer.clone());
        palette.push(PaintColor{color: Color32::BLACK, id: new_rand_id()});
        palette.push(PaintColor{color: Color32::WHITE, id: new_rand_id()});

        palette.push(PaintColor{color: Color32::BLUE, id: new_rand_id()});
        // palette.push(Color32::WHITE);
        // palette.push(Color32::GREEN);
        let default_tool = Tools::default().tools[0].clone();
        
        Self { 
            is_dragging: false,
            poses: Vec::new(),
            current_draw_tool: Some(default_tool),
            layers_container: layers_container,

            color_palette: palette.clone(),
            current_color: Some(palette[0].clone()),
            current_stroke_width: 10.,
           
            
            current_layer: Some(default_layer.id)
        }
    }
        
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ctx.request_repaint();

        egui::CentralPanel::default().show(ctx,  |ui| {
            ui.horizontal(|ui|{
                ToolBar::add(self, ui);
            });
            ui.add_space(10.);

            ui.horizontal_top(|ui| {
                ui.vertical(|ui| {
                    ColorPalette::add(self, ui); 
                    ColorPicker::add(self, ui);
                });
                Canvas::add( self, ui);
                LayersDisplayContainer::add(self, ui);
            });
            ui.add_space(10.);
            ui.horizontal(|ui| {
                ui.label(format!("Current scale: {:.02}", &self.app_state.layers_container.transform.scale.clone()));
            });
        });  
    }
}

impl App { 
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        fonts.font_data.insert(
            "roboto".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
                "../../assets/Roboto-Regular.ttf"
            ))),
        );
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "roboto".to_owned());
        cc.egui_ctx.set_fonts(fonts);
        let app_settings = AppSettings::default();
        Self {

            app_state: AppState::from_settings(app_settings.clone()),
            app_settings,
        }
    }

    pub fn re_new(&mut self) {
        let mut new_settings = AppSettings::default();
        new_settings.layer_size = Vec2::new(self.app_settings.new_paint_settings.width as f32, self.app_settings.new_paint_settings.height as f32);
        self.app_settings = new_settings.clone();
        self.app_state = AppState::from_settings(new_settings);
    
    }

    pub fn set_base_directory(&mut self) {
        let dir_path: Option<PathBuf> = FileDialog::new().pick_folder();
        if let Some(path) = dir_path {
            self.app_settings.base_dir = Some(path);
        }
        
    }

    pub fn save_to_image(&self) -> Result<(), std::io::Error> {
        let path = if let Some(base_dir) = &self.app_settings.base_dir {
            base_dir.join("final_output.png")
        } else {
            PathBuf::from(r"C:\")
        };
        // let path = base_dir.join("final_output.png");
        let color_vec = &self.app_state.layers_container.layers.clone().iter().rev().map(|layer| layer.texture.image_data.clone()).collect::<Vec<ColorImage>>();
        let composites = composite_layers(&color_vec);
        let [width, height] = composites.size;

        let mut buffer  = ImageBuffer::<image::Rgba<u8>, Vec<u8>>::new(width as u32, height as u32);

        for (i, pixel) in composites.pixels.iter().enumerate() {
            let x = (i % width) as u32;
            let y = (i / width) as u32;
            let rgba = pixel.to_array(); // [r, g, b, a]
            buffer.put_pixel(x, y, image::Rgba::from(rgba));
        }
        buffer.save(path).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
    pub fn load_image(&mut self) {
        let file_path: Option<PathBuf> = FileDialog::new()
            .add_filter("Image", &["png", "jpeg", "jpg"])
            .pick_file();
        match file_path {
            Some(path) => {
                let reader = image::ImageReader::open(path.clone()).unwrap();
                if let Ok(image) = reader.decode()  {
                    let image_size = Vec2::new(image.width() as f32, image.height() as f32);
                    let image_ratio = image_size.x / image_size.y;
                    let canvas_ratio = self.app_settings.layer_size.x / self.app_settings.layer_size.y;
                    let scaled_size = if image_ratio > canvas_ratio {
                        Vec2::new(self.app_settings.layer_size.x.clone(), self.app_settings.layer_size.x.clone() / image_ratio)
                    } else {
                        Vec2::new(self.app_settings.layer_size.y.clone() * image_ratio, self.app_settings.layer_size.y.clone() )
                    };
                    let scaled_image = image.resize(scaled_size.x as u32, scaled_size.y as u32, FilterType::Nearest);
                    let color_image = ColorImage::from_rgba_unmultiplied(
                        [scaled_image.width() as _, scaled_image.height() as _],
                        scaled_image.to_rgba8().as_flat_samples().as_slice(),
                    );
                    let offset = (self.app_settings.layer_size - scaled_size) / 2.0;
                    let mut new_image_layer = Layer {
                        id: new_rand_id(), 
                        name: "Image layer".to_string(), 
                        is_visible: true,
                        texture: LayerTexture::new(self.app_settings.layer_size.x as usize, self.app_settings.layer_size.y as usize)
                    };
                    for px in 0..scaled_image.width() {
                        for py in 0..scaled_image.height() {
                                new_image_layer.texture.image_data.pixels[((py as f32 + offset.y).floor() as usize * self.app_settings.layer_size.x  as usize + (px as f32 + offset.x).floor() as usize) as usize] = color_image.pixels[(py * scaled_image.width() + px) as usize];
                        
                        }
                    }
                    if let Some(active_layer) = self.app_state.current_layer  {
                        if let Some(find_index) = self.app_state.layers_container.layers.iter().position(|l| l.id == active_layer) {
                            if find_index == 0 {
                                self.app_state.layers_container.layers.insert(0, new_image_layer);
                            } else {
                                self.app_state.layers_container.layers.insert(find_index - 1, new_image_layer);

                            }
                        }
                        
                    }
                }
            },
            None => {}
        }
    }
}


// fn blend_pixel(bottom: egui::Color32, top: egui::Color32) -> egui::Color32 {
//     let top_a = top.a() as f32 / 255.0;
//     let inv_a = 1.0 - top_a;

//     let r = (top.r() as f32 * top_a + bottom.r() as f32 * inv_a) as u8;
//     let g = (top.g() as f32 * top_a + bottom.g() as f32 * inv_a) as u8;
//     let b = (top.b() as f32 * top_a + bottom.b() as f32 * inv_a) as u8;

    
//     let a = (top.a() as f32 + bottom.a() as f32 * inv_a) as u8;

//     egui::Color32::from_rgba_premultiplied(r, g, b, a)
// }

// ef get_color(colorRGBA1, colorRGBA2):
//     alpha = 255 - ((255 - colorRGBA1[3]) * (255 - colorRGBA2[3]) / 255)      1. - (1. - top.a)(1. - bottom.a)
//     red   = (colorRGBA1[0] * (255 - colorRGBA2[3]) + colorRGBA2[0] * colorRGBA2[3]) / 255   r = top.r * (1. - bottom.r) - bottom.r * bottom.a
//     green = (colorRGBA1[1] * (255 - colorRGBA2[3]) + colorRGBA2[1] * colorRGBA2[3]) / 255   b = top.r * (1. - bottom.r) - bottom.r * bottom.a
//     blue  = (colorRGBA1[2] * (255 - colorRGBA2[3]) + colorRGBA2[2] * colorRGBA2[3]) / 255
//     return (int(red), int(green), int(blue), int(alpha))