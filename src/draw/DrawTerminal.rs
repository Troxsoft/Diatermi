use std::io::stdout;

use crate::terminal::{DrawObjectTrait, DrawTrait, Terminal, Vector2};

use super::Text::Text;

#[derive(Debug)]
pub struct DrawTerminal<'a> {
    pub terminal: &'a Terminal,
}
impl<'a> DrawTerminal<'a> {
    pub fn text(&self, text: impl ToString, position: Vector2) -> Text {
        let mut text_ = Text::new();
        text_.set_position(position);
        text_.set_text(text);
        text_.clone()
    }
    pub fn text_color(&self, text: impl ToString, position: Vector2, color: crate::Color) -> Text {
        let mut text_ = Text::new();
        text_.set_position(position);
        text_.set_color(color);
        text_.set_text(text);
        text_.clone()
    }
}
impl<'a> DrawTrait<'a> for DrawTerminal<'a> {
    fn new(terminal: &'a Terminal) -> Self {
        return Self { terminal: terminal };
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

    fn draw(&self, object: impl crate::terminal::DrawObjectTrait<'a>) {
        object.draw()
    }
}
