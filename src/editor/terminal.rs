use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::stdout;

pub fn clear_screen() -> Result<(), std::io::Error> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All))
}

pub fn initialize() -> Result<(), std::io::Error> {
    enable_raw_mode()?;
    clear_screen()
}

pub fn terminate() -> Result<(), std::io::Error> {
    disable_raw_mode()
}
