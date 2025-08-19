use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{stdout, Write};

pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(&Point::new(0, 0))?;
        Self::flush()
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    pub fn move_cursor_to(p: &Point) -> Result<(), std::io::Error> {
        queue!(stdout(), MoveTo(p.x, p.y))
    }

    pub fn clear_line() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::All))
    }

    // Returns the size of the terminal.
    // as a Point.
    pub fn size() -> Result<Point, std::io::Error> {
        let size = size()?;
        Ok(Point {
            x: size.0,
            y: size.1,
        })
    }

    pub fn hide_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Hide)
    }

    pub fn show_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Show)
    }

    pub fn print(c: &str) -> Result<(), std::io::Error> {
        queue!(stdout(), Print(c))
    }

    pub fn flush() -> Result<(), std::io::Error> {
        stdout().flush()
    }
}
