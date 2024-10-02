use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use terminal::Terminal;

mod terminal;

pub struct Editor {
    will_quit: bool,
    term: terminal::Terminal,
}

impl Editor {
    pub fn default() -> Self {
        Editor {
            will_quit: false,
            term: Terminal::new(),
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
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
