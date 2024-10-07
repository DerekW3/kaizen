use crate::editor::terminal::{Shape, Terminal};

pub struct Cursor {
    x: u16,
    y: u16,
}

impl Cursor {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn col(&self) -> u16 {
        self.x
    }

    pub fn row(&self) -> u16 {
        self.y
    }
}
