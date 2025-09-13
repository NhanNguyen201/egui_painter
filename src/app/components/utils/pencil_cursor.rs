use egui::{Color32, Key, Painter, Pos2, Stroke, Vec2};

use crate::app::components::utils::draw_tool::{DrawTool, Pencil};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum DirLocked {
    #[default]
    None,
    X,
    Y
}

#[derive(Clone, Copy, PartialEq)]
pub struct PencilCursorSettings {
    pub dir_locked: DirLocked 
}

impl Default for PencilCursorSettings {
    fn default() -> Self {
        Self {
            dir_locked: DirLocked::None
        }
    }
}
impl  PencilCursorSettings {
    pub fn add_lock_event(&mut self, ui: &mut egui::Ui) {
        let key_x = ui.ctx().input(|i| i.key_released(Key::X));
        let key_y = ui.ctx().input(|i| i.key_released(Key::Y));
        
        if key_x {
            
            self.dir_locked = if self.dir_locked == DirLocked::X {
                DirLocked::None
            } else {
                DirLocked::X 
            }
        }

        if key_y {
                self.dir_locked = if self.dir_locked == DirLocked::Y {
                DirLocked::None
            } else {
                DirLocked::Y
            }
        }
    }
}


#[derive(Clone, Copy, PartialEq)]
pub struct PencilCursor {
    pub pos: Pos2,
    pub pencil: Pencil,
    pub radius: f32,
    pub settings: PencilCursorSettings,
}

impl Default for PencilCursor {
    fn default() -> Self {
        Self {
            pos: Pos2::ZERO,
            pencil: DrawTool::default().pencil,
            radius: 10.,
            settings: PencilCursorSettings::default()
        }
    }
}

impl PencilCursor  {
    pub fn ui(&self, painter: &Painter) {
        
        
        match self.pencil {
            
            Pencil::Brush | Pencil::Pen => {
                painter.circle_stroke(self.pos, self.radius / 2., Stroke::new(1., Color32::BLACK));
            },
            Pencil::Eraser => {
                painter.rect_stroke(egui::Rect::from_center_size(self.pos, Vec2::new(self.radius, self.radius)), 0.0, Stroke::new(1., Color32::BLACK), egui::StrokeKind::Middle);
            }
        }
    }
    pub fn update_pos(&mut self, pos: Pos2) {
        match self.settings.dir_locked {
            DirLocked::None => {
                self.pos = pos;
            },
            DirLocked::X => {
                self.pos.x = pos.x
            },
            DirLocked::Y => {
                self.pos.y = pos.y;
            }
        }
    }
    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }
    pub fn set_pencil(&mut self, pencil: Pencil) {
        self.pencil = pencil;
    }
    pub fn get_pos(&self) -> Pos2 {
        self.pos
    }
    
}