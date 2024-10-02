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
        Self::draw_rows().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let (_, height) = Terminal::get_size()?;
        for row in 0..height {
            Terminal::move_cursor(0, row)?;
            print!("~");
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.will_quit {
            Terminal::clear_screen()?;
            print!("Goodbye. \r\n");
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
            let event = read()?;

            self.evaluate_event(&event);

            self.refresh_screen()?;

            if self.will_quit {
                break;
            }
        }
        Ok(())
    }
}
