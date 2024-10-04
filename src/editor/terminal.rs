use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{stdout, Error, Write};

pub struct Terminal;
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Terminal {
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    pub fn get_size() -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size { width, height })
    }

    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor(0, 0)?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()
    }

    pub fn move_cursor(w: u16, h: u16) -> Result<(), Error> {
        queue!(stdout(), MoveTo(w, h))?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }

    pub fn print_string(input: &str) -> Result<(), Error> {
        queue!(stdout(), Print(input))?;
        Ok(())
    }
}
