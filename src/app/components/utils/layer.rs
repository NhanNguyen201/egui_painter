
use egui::{Color32, Id, Pos2, Vec2};
use egui::{ColorImage, TextureHandle};
use rand::random_range;
use crate::app::components::utils::draw_tool::Pencil;
use crate::app::components::utils::new_rand_id;

#[derive(Clone, PartialEq)]
pub struct LayersContainer {
    pub layers: Vec<Layer>,
    pub transform: Transform
}

impl Default for LayersContainer {
    fn default() -> Self {
        Self {
            layers: Vec::new(),
            transform: Transform::default()
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Transform {
    pub position: Pos2,
    pub scale: f32
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Pos2::ZERO,
            scale: 1.
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Layer {
    pub id: Id,
    pub name: String,
    pub is_visible: bool,
    pub texture: LayerTexture
}

#[derive(Clone, PartialEq)]
pub struct LayerTexture {
    pub texture_handle: Option<TextureHandle>,
    pub image_data: ColorImage,
    pub width: usize,
    pub height: usize,
}

fn random_draw() -> bool {
    random_range(0.0..1.0) > 0.5
}

impl LayerTexture {
    pub fn new(width: usize, height: usize) -> Self {
        let mut filled: Vec<Color32>  = vec![];
        filled.resize_with(width * height,|| Color32::from_rgba_unmultiplied(255, 255, 255, 0));
        
        let image_data = ColorImage::new([width, height], filled);
        Self {
            texture_handle: None,
            image_data,
            width,
            height,
        }
    }
    pub fn paint_at(&mut self, pos: Pos2, tool: Pencil, brush_size: f32, color: Color32) {
        let x = pos.x as usize;
        let y = pos.y as usize;
        
        // Simple circular brush
        match tool {
            Pencil::Brush => {
                let radius = (brush_size / 2.0) as i32;
                for dy in -radius..=radius {
                    for dx in -radius..=radius {
                        let px = x as i32 + dx;
                        let py = y as i32 + dy;
                        
                        if px >= 0 && py >= 0 && 
                        (px as usize) < self.width && 
                        (py as usize) < self.height &&
                            random_draw()
                        {
                            let dist_sq = dx * dx + dy * dy;
                            if dist_sq <= radius * radius {
                                let idx = py as usize * self.width + px as usize;
                                self.image_data.pixels[idx] = color;
                            }
                        }
                    }
                }
            },
            Pencil::Pen => {
                let radius = (brush_size / 2.0) as i32;
                for dy in -radius..=radius {
                    for dx in -radius..=radius {
                        let px = x as i32 + dx;
                        let py = y as i32 + dy;
                        
                        if px >= 0 && py >= 0 && 
                        (px as usize) < self.width && 
                        (py as usize) < self.height 
                        {
                            let dist_sq = dx * dx + dy * dy;
                            if dist_sq <= radius * radius {
                                let idx = py as usize * self.width + px as usize;
                                self.image_data.pixels[idx] = color;
                            }
                        }
                    }
                }
            },
            Pencil::Eraser => {
                let x = pos.x as usize;
                let y = pos.y as usize;
                let radius = (brush_size / 2.0) as i32;
                for dy in -radius..=radius {
                    for dx in -radius..=radius {
                        let px = x as i32 + dx;
                        let py = y as i32 + dy;
                        
                        if px >= 0 && py >= 0 && 
                        (px as usize) < self.width && 
                        (py as usize) < self.height {
                            let idx = py as usize * self.width + px as usize;

                            self.image_data.pixels[idx] = Color32::from_white_alpha(0);
                        }
                    }
                }
            }
        }
        
    }
    
}


#[derive(Clone, PartialEq)]
pub struct PaintColor {
    pub color: Color32,
    pub id: Id
}

impl Default for PaintColor {
    fn default() -> Self {
        Self {
            color: Color32::BLACK,
            id: new_rand_id()
        }
    }
}