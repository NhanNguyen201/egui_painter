#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NewPaintSetting {
    pub width: usize,
    pub height: usize,
    pub is_open: bool
}

impl Default for NewPaintSetting {
    fn default() -> Self {
        Self {
            width: 500,
            height: 500,
            is_open: false
        }
    }
}