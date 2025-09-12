use std::{ fmt::{Display, Debug}};

use egui::Id;

use crate::app::components::utils::new_rand_id;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Pencil {
    Brush,
    Pen,
    Eraser
}

impl Display for Pencil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, PartialEq)]
pub struct DrawTool {
    pub pencil: Pencil,
    pub id: Id
}

impl Display for DrawTool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.pencil)
    }
}

impl DrawTool {
    pub fn new(pencil: Pencil) -> DrawTool {
        DrawTool {
            pencil: pencil,
            id: new_rand_id()
        }
    }
}

impl Default for DrawTool {
    fn default() -> Self {
        Self { pencil: Pencil::Brush, id: Id::new("Brush") }
    }
}

#[derive(Clone, PartialEq)]
pub struct Tools {
    pub tools: Vec<DrawTool>,
}

impl Default for Tools {
    fn default() -> Self {
        let mut tools: Vec<DrawTool> = Vec::new();
        let brush = DrawTool::default();
        tools.push(brush.clone());
        tools.push(DrawTool::new(Pencil::Pen));
        tools.push(DrawTool::new(Pencil::Eraser));
        Self {
            tools
        }
    }
}
