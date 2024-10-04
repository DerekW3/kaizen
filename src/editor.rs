use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use terminal::Terminal;

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

    fn draw_rows() -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        let terminal_dims = Terminal::get_size()?;
        for row in 0..terminal_dims.height {
            Terminal::print_string("~")?;
            if row + 1 < terminal_dims.height {
                Terminal::print_string("\r\n")?;
            }
        }
        Terminal::show_cursor()?;
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.will_quit {
            Terminal::clear_screen()?;
            print!("Goodbye. \r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor(0, 0)?;
        }
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

    fn repl(&mut self) -> Result<(), std::io::Error> {
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
