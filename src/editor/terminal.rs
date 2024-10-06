use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{queue, Command};
use std::fmt::Display;
use std::io::{stdout, Error, Write};

pub struct Terminal;

#[derive(Clone, Copy)]
pub struct Shape {
    pub width: usize,
    pub height: usize,
}

impl Terminal {
    pub fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    /// Returns the shape of the current terminal window
    ///
    /// @`EdgeCases`
    ///     usize < u16: width/height truncated to usize
    ///
    /// @Returns
    ///     `Shape` - named struct holding width and height
    ///     `std::io::Error` - error reading terminal dimensions
    pub fn shape() -> Result<Shape, Error> {
        let (width_u16, height_u16) = size()?;
        #[allow(clippy::as_conversions)] // clippy::as_conversions: See docstring above
        let width = width_u16 as usize;
        #[allow(clippy::as_conversions)] // clippy::as_conversions: See docstring above
        let height = height_u16 as usize;
        Ok(Shape { width, height })
    }

    pub fn clear_terminal(clear_type: ClearType) -> Result<(), Error> {
        Self::queue_command(Clear(clear_type))
    }

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_terminal(ClearType::All)?;
        Self::move_cursor(0, 0)?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()
    }

    pub fn move_cursor(w: u16, h: u16) -> Result<(), Error> {
        Self::queue_command(MoveTo(w, h))
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)
    }

    pub fn print<T: Display>(input: T) -> Result<(), Error> {
        Self::queue_command(Print(input))
    }
}
