use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};

mod terminal;

pub struct Editor {
    will_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { will_quit: false }
    }

    pub fn run(&mut self) {
        terminal::initialize().unwrap();
        let result = self.repl();
        terminal::terminate().unwrap();
        result.unwrap();
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.will_quit {
            terminal::clear_screen()?;
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
