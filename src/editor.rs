use crossterm::{
    event::{
        read,
        Event::{self, Key},
        KeyCode::Char,
        KeyEvent, KeyModifiers,
    },
    terminal::ClearType,
};
use std::io::Error;
use terminal::{Shape, Terminal};

mod terminal;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    will_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { will_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::shape()?.width;
        let length = welcome_message.len();
        #[allow(clippy::integer_division)] // Truncation is the intended functionality
        let padding = (width.saturating_sub(length)) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(welcome_message)?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Shape { height, .. } = Terminal::shape()?;
        for row in 0..height {
            Terminal::clear_terminal(ClearType::CurrentLine)?;
            #[allow(clippy::integer_division)] // Truncation is the intended functionality
            if row == height / 2 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            if row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.will_quit {
            Terminal::clear_terminal(ClearType::All)?;
            print!("Goodbye. \r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor(0, 0)?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.will_quit = true;
                }
                _ => (),
            }
        }
    }

    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;

            if self.will_quit {
                break;
            }

            let event = read()?;

            self.evaluate_event(&event);
        }
        Ok(())
    }
}
