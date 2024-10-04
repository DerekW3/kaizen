use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::queue;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::stdout;

pub struct Terminal {}

impl Terminal {
    pub fn get_size() -> Result<(u16, u16), std::io::Error> {
        size()
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        queue!(stdout, Hide)?;
        queue!(stdout, Clear(ClearType::All))?;
        queue!(stdout, Show)
    }

    pub fn clear_line() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        queue!(stdout, Hide)?;
        queue!(stdout, Clear(ClearType::CurrentLine))?;
        queue!(stdout, Show)
    }

    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor(0, 0)?;
        Ok(())
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    pub fn move_cursor(w: u16, h: u16) -> Result<(), std::io::Error> {
        queue!(stdout(), MoveTo(w, h))?;
        Ok(())
    }
}
