mod terminal;
mod view;

use crossterm::event::{read, Event, KeyEvent, KeyModifiers};

use crate::editor::terminal::Point;
use crate::keystack::{Actions, KeyCombination, KeyStack};
use crossterm::event::Event::Key;
use terminal::Terminal;
use view::View;

pub struct Editor {
    key_stack: KeyStack,
    close: bool,
    view: View,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            key_stack: KeyStack::default(),
            close: false,
            view: View::default(),
        }
    }
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.close {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            if modifiers.contains(KeyModifiers::ALT) || modifiers.contains(KeyModifiers::CONTROL) {
                let action = self.key_stack.push(KeyCombination {
                    modifier: *modifiers,
                    key: *code,
                });
                self.close = action == Actions::Close;
            }
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        if self.close {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye. \r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_cursor_to(&Point::new(0, 0))?;
        }
        Terminal::show_cursor()?;
        Terminal::flush()?;
        Ok(())
    }
}
