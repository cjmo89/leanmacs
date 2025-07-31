#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]
use crate::editor::Editor;

mod editor;
mod keystack;

fn main() {
    Editor::default().run();
}
