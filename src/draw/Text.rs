use std::{fmt::format, io::stdout};

use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};

use crate::terminal::{DrawObjectTrait, DrawTrait, Terminal, Vector2};

use super::DrawCommunicator::DrawCommunicator;
#[derive(Debug, Clone)]
pub struct Text {
    communicator: DrawCommunicator,
}
impl Text {
    #[doc = "Change the text
### Example:
```
object.set_text(2+2);    
```
"]
    pub fn set_text(&mut self, string: impl ToString) {
        self.communicator.draw_text = string.to_string()
    }
    #[doc = "
Returns the text
### Example:
```
let user_info = object.text();
```
"]
    pub fn text(&self) -> String {
        self.communicator.draw_text.clone()
    }
    #[doc = "
Returns the same object but with the text changed"]
    pub fn with_text(&self, text: impl ToString) -> Self {
        Self {
            communicator: DrawCommunicator {
                bg_color: self.communicator.bg_color,
                color: self.communicator.color,
                id: self.communicator.id.clone(),
                draw_text: text.to_string().clone(),
                position: self.communicator.position,
            },
        }
    }
    #[doc = "
Returns the same object but with the position changed"]
    pub fn with_position(&self, position: Vector2) -> Self {
        Self {
            communicator: DrawCommunicator {
                bg_color: self.communicator.bg_color,
                color: self.communicator.color,
                id: self.communicator.id.clone(),
                draw_text: self.communicator.draw_text.clone(),
                position: position.clone(),
            },
        }
    }
    #[doc = "Returns the same object but with the color changed"]
    pub fn with_color(&self, color: Color) -> Self {
        Self {
            communicator: DrawCommunicator {
                bg_color: self.communicator.bg_color,
                color: color.clone(),
                id: self.communicator.id.clone(),
                draw_text: self.communicator.draw_text.clone(),
                position: self.communicator.position.clone(),
            },
        }
    }
    #[doc = "
Returns the same object but with the background color changed"]

    pub fn with_bg_color(&self, bg_color: Color) -> Self {
        Self {
            communicator: DrawCommunicator {
                bg_color: bg_color.clone(),
                color: self.communicator.color.clone(),
                id: self.communicator.id.clone(),
                draw_text: self.communicator.draw_text.clone(),
                position: self.communicator.position.clone(),
            },
        }
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
