mod terminal;
use crossterm::event::{read, Event, KeyEvent, KeyModifiers};

use crate::editor::terminal::Point;
use crate::keystack::{Actions, KeyCombination, KeyStack};
use crossterm::event::Event::Key;
use terminal::Terminal;

pub struct Editor {
    key_stack: KeyStack,
    close: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            key_stack: KeyStack::default(),
            close: false,
        }
    }
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    pub fn display_welcome() {
        let size = Terminal::size().unwrap();
        let msg = "Welcome to Leanmacs 0.01";
        let x_pos = size.x / 2 - msg.chars().count() as u16 / 2;
        Terminal::move_cursor_to(&Point::new(x_pos, size.y / 3)).unwrap();
        Terminal::print(msg).unwrap();
        Terminal::flush().unwrap();
    }
    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.close {
                break;
            }
            Self::display_welcome();
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
            Terminal::clear_line()?;
            Terminal::print("Goodbye. \r\n")?;
            Terminal::flush()?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(&Point::new(0, 0))?;
        }
        Terminal::show_cursor()?;
        Terminal::flush()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        Terminal::move_cursor_to(&Point::new(0, 0))?;
        let point = Terminal::size()?;
        let rows = point.y;
        for row in 0..rows {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            Terminal::move_cursor_to(&Point::new(0, row + 1))?;
        }
        Ok(())
    }
}
