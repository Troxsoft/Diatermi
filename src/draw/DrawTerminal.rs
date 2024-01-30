use std::io::stdout;

use crate::terminal::{DrawObjectTrait, DrawTrait, Terminal, Vector2};

use super::{DrawCommunicator::DrawHandler, Text::Text};

#[derive(Debug, Clone)]
#[doc = "
Manage basic things like deleting all text from the terminal or deleting something specific"]
pub struct DrawTerminal {}
impl<'a> DrawTrait<'a> for DrawTerminal {
    fn new() -> Self {
        return Self {};
    }

    fn clear_all(&mut self) {
        crossterm::execute!(
            stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        );
    }

    fn clear_position(&mut self, pos: crate::terminal::Vector2) {
        crossterm::execute!(
            stdout(),
            crossterm::cursor::MoveTo(pos.x.try_into().unwrap(), pos.y.try_into().unwrap()),
            crossterm::style::ResetColor,
            crossterm::style::Print(" ")
        );
    }
}
