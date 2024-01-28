use std::{fmt::format, io::stdout};

use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};

use crate::terminal::{DrawObjectTrait, DrawTrait, Terminal, Vector2};
#[derive(Debug, Clone)]
pub struct Text {
    text_: String,
    color_: crate::Color,
    position_: Vector2,
    bg_color_: crate::Color,
}
impl Text {
    pub fn set_text(&mut self, string: impl ToString) {
        self.text_ = string.to_string()
    }
    pub fn text(&self) -> String {
        self.text_.clone()
    }
}
impl ToString for Text {
    fn to_string(&self) -> String {
        self.text()
    }
}
impl<'a> DrawObjectTrait<'a> for Text {
    fn draw(&self) {
        use crossterm::cursor::MoveTo;
        crossterm::execute!(
            stdout(),
            MoveTo(
                self.position().x.try_into().unwrap(),
                self.position().y.try_into().unwrap()
            ),
            SetForegroundColor(self.color_),
            SetBackgroundColor(self.bg_color_),
            Print(self.text_.clone()),
            ResetColor
        );
    }

    fn set_position(&mut self, position: crate::terminal::Vector2) {
        self.position_ = position
    }

    fn set_color(&mut self, color: crate::Color) {
        self.color_ = color
    }

    fn color(&self) -> crate::Color {
        self.color_
    }

    fn position(&self) -> Vector2 {
        self.position_
    }

    fn set_bg_color(&mut self, color: crate::Color) {
        self.bg_color_ = color
    }

    fn bg_color(&self) -> crate::Color {
        self.bg_color_
    }

    fn new() -> Self {
        Self {
            color_: Color::Cyan,
            text_: "hello :)".to_string(),
            position_: Vector2::new(0, 0),
            bg_color_: Color::Black,
        }
    }

    fn clear(&self) {
        let mut text__: String = "".to_string();
        let mut i = 0;
        while i < self.text().len() {
            text__.push_str("⠀");
            i += 1;
        }
        crossterm::execute!(
            stdout(),
            crossterm::cursor::MoveTo(
                self.position().x.try_into().unwrap(),
                self.position().y.try_into().unwrap()
            ),
            SetForegroundColor(self.color_),
            SetBackgroundColor(self.bg_color_),
            Print(text__),
            ResetColor
        );
    }
}
