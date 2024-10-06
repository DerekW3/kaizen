use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;
use terminal::{Shape, Terminal};

mod terminal;

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

    fn draw_rows() -> Result<(), Error> {
        let Shape { height, .. } = Terminal::get_size()?;
        for row in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.will_quit {
            Terminal::clear_screen()?;
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
