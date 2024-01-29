use std::{fmt::format, io::stdout};

use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};

use crate::terminal::{DrawObjectTrait, DrawTrait, Terminal, Vector2};

use super::DrawCommunicator::DrawCommunicator;
#[derive(Debug, Clone)]
pub struct Text {
    communicator: DrawCommunicator,
}
impl Text {
    pub fn set_text(&mut self, string: impl ToString) {
        self.communicator.draw_text = string.to_string()
    }
    pub fn text(&self) -> String {
        self.communicator.draw_text.clone()
    }
}
impl ToString for Text {
    fn to_string(&self) -> String {
        self.text()
    }
}
impl<'a> DrawObjectTrait<'a> for Text {
    fn set_position(&mut self, position: crate::terminal::Vector2) {
        self.communicator.position = position
    }

    fn set_color(&mut self, color: crate::Color) {
        self.communicator.color = color
    }

    fn color(&self) -> crate::Color {
        self.communicator.color
    }

    fn position(&self) -> Vector2 {
        self.communicator.position
    }

    fn set_bg_color(&mut self, color: crate::Color) {
        self.communicator.bg_color = color
    }

    fn bg_color(&self) -> crate::Color {
        self.communicator.bg_color
    }

    fn new(id: impl ToString) -> Self {
        Self {
            communicator: DrawCommunicator {
                bg_color: Color::Blue,
                color: Color::Black,
                draw_text: "text value".to_string(),
                id: id.to_string(),
                position: Vector2::new(0, 0),
            },
        }
    }

    fn clear(&mut self) {
        self.communicator.draw_text = "".to_string();
    }

    fn id(&self) -> String {
        self.communicator.id.clone()
    }

    fn communicator(&self) -> super::DrawCommunicator::DrawCommunicator {
        self.communicator.clone()
    }
}
