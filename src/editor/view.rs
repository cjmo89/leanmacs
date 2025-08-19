mod buffer;
use buffer::Buffer;

use crate::editor::terminal::Point;
use crate::editor::terminal::Terminal;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}
impl View {
    pub fn render(&self) -> Result<(), std::io::Error> {
        Terminal::move_cursor_to(&Point::new(0, 0))?;
        Terminal::clear_line()?;
        let terminal_size = Terminal::size()?;
        let rows = terminal_size.y;
        let mut buffer_counter = 0;
        for row in 0..rows {
            Terminal::clear_line()?;
            if let Some(s) = self.buffer.lines.get(buffer_counter) {
                Terminal::print(s)?;
                buffer_counter += 1;
            } else {
                Terminal::print("~")?;   
            }
            if row == rows / 3 {
                Self::display_welcome()?;
            }
            Terminal::move_cursor_to(&Point::new(0, row + 1))?;
        }
        Ok(())
    }

    fn display_welcome() -> Result<(), std::io::Error> {
        let size = Terminal::size()?;
        let msg = format!("Welcome to {NAME} -- version {VERSION}");
        let x_pos = size.x / 2 - msg.chars().count() as u16 / 2;
        Terminal::move_cursor_to(&Point::new(x_pos, size.y / 3))?;
        Terminal::print(&msg)?;
        Terminal::flush()?;
        Ok(())
    }
}
