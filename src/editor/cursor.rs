use crate::editor::terminal::{Shape, Terminal};
use core::cmp::min;
use std::io::Error;

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

    pub fn move_up(&mut self) -> Result<(), Error> {
        self.y = self.y.saturating_sub(1);
        Terminal::move_cursor(self.x, self.y)?;
        Ok(())
    }

    pub fn move_down(&mut self) -> Result<(), Error> {
        let Shape { height, .. } = Terminal::shape()?;
        self.y = min(
            self.y.saturating_add(1),
            u16::try_from(height).unwrap_or(u16::MAX),
        );
        Terminal::move_cursor(self.x, self.y)?;
        Ok(())
    }
}
