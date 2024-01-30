use std::io::stdout;

use crate::terminal::Vector2;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[doc = "
Stores information to paint on the screen. This is created automatically by the `DrawObjectTrait` implementations"]
pub struct DrawCommunicator {
    #[doc = "A unique ID or ?"]
    pub id: String,
    #[doc = "The text to be drawn"]
    pub draw_text: String,
    #[doc = "
The color of the text"]
    pub color: crate::Color,
    #[doc = "The color of the background text"]
    pub bg_color: crate::Color,
    #[doc = "The position of the text"]
    pub position: Vector2,
}
#[derive(Debug, Clone)]
#[doc = "`Manage` the `communicators`"]
pub struct DrawHandler {
    draws: Vec<DrawCommunicator>,
}
impl DrawHandler {
    #[doc = "Create a new `DrawHandler`"]
    pub fn new() -> Self {
        Self { draws: Vec::new() }
    }
    #[doc = "there is a communicator with the ID?"]
    pub fn exists(&self, id: impl ToString) -> bool {
        for i in self.draws.clone() {
            if i.id == id.to_string() {
                return true;
            }
        }

        false
    }
    #[doc = "
Add a communicator"]
    pub fn add(&mut self, communicator: DrawCommunicator) {
        if self.exists(communicator.id.clone()) {
            if communicator.clone() != self.get(communicator.clone().id).unwrap() {
                self.remove(communicator.id.clone());
                self.draws.push(communicator.clone());
            }
        } else {
            self.draws.push(communicator.clone());
        }
    }
    #[doc = "Remove a communicator"]
    pub fn remove(&mut self, id: impl ToString) {
        let mut i = 0;
        while i < self.draws.len() {
            if self.draws[i].id == id.to_string() {
                self.draws.remove(i);
            }
            i += 1;
        }
    }
    #[doc = "
Returns all communicators"]
    pub fn all(&self) -> Vec<DrawCommunicator> {
        return self.draws.clone();
    }
    #[doc = "
Return a communicator that has the same ID"]
    pub fn get(&self, id: impl ToString) -> Option<DrawCommunicator> {
        let mut i = 0;
        while i < self.draws.len() {
            if self.draws[i].id == id.to_string() {
                return Some(self.draws[i].clone());
            }
            i += 1;
        }
        None
    }
    #[doc = "Draw a communicator"]
    pub fn draw(&self, id: impl ToString) {
        let mut i = 0;
        use crossterm::cursor::MoveTo;
        let mut stdout = stdout();
        while i < self.draws.len() {
            if self.draws[i].id == id.to_string() {
                crossterm::execute!(
                    stdout,
                    MoveTo(
                        self.draws[i].position.x.try_into().unwrap(),
                        self.draws[i].position.y.try_into().unwrap()
                    ),
                    crossterm::style::SetForegroundColor(self.draws[i].color),
                    crossterm::style::SetBackgroundColor(self.draws[i].bg_color),
                    crossterm::style::Print(self.draws[i].draw_text.clone()),
                    crossterm::style::ResetColor
                );
            }
            i += 1;
        }
    }
    #[doc = "
Draw all the communicators"]
    pub fn draw_all(&self) {
        let mut i = 0;
        let mut stdout = stdout();
        while i < self.draws.len() {
            use crossterm::cursor::MoveTo;
            crossterm::execute!(
                stdout,
                MoveTo(
                    self.draws[i].position.x.try_into().unwrap(),
                    self.draws[i].position.y.try_into().unwrap()
                ),
                crossterm::style::SetForegroundColor(self.draws[i].color),
                crossterm::style::SetBackgroundColor(self.draws[i].bg_color),
                crossterm::style::Print(self.draws[i].draw_text.clone()),
                crossterm::style::ResetColor
            );
            i += 1;
        }
    }
    #[doc = "
Clear the screen of all communicators"]
    pub fn clear_all(&self) {
        let mut i = 0;
        use crossterm::cursor::MoveTo;
        let mut stdout = stdout();
        while i < self.draws.len() {
            let mut text__: String = "".to_string();
            let mut i = 0;
            while i < self.draws[i].draw_text.len() {
                text__.push_str("⠀");
                i += 1;
            }
            crossterm::execute!(
                stdout,
                crossterm::cursor::MoveTo(
                    self.draws[i].position.x.try_into().unwrap(),
                    self.draws[i].position.y.try_into().unwrap()
                ),
                crossterm::style::SetForegroundColor(self.draws[i].color),
                crossterm::style::SetBackgroundColor(self.draws[i].bg_color),
                crossterm::style::Print(text__),
                crossterm::style::ResetColor
            );

            i += 1;
        }
    }
    #[doc = "
Clean the screen of a communicator"]
    pub fn clear(&self, id: impl ToString) {
        let mut i = 0;
        use crossterm::cursor::MoveTo;
        let mut stdout = stdout();
        while i < self.draws.len() {
            if self.draws[i].id == id.to_string() {
                let mut text__: String = "".to_string();
                let mut i = 0;
                while i < self.draws[i].draw_text.len() {
                    text__.push_str("⠀");
                    i += 1;
                }
                crossterm::execute!(
                    stdout,
                    crossterm::cursor::MoveTo(
                        self.draws[i].position.x.try_into().unwrap(),
                        self.draws[i].position.y.try_into().unwrap()
                    ),
                    crossterm::style::SetForegroundColor(self.draws[i].color),
                    crossterm::style::SetBackgroundColor(self.draws[i].bg_color),
                    crossterm::style::Print(text__),
                    crossterm::style::ResetColor
                );
            }
            i += 1;
        }
    }
    #[doc = "
Remove all communicators
"]
    pub fn remove_all(&mut self) {
        self.draws.clear();
    }
}
