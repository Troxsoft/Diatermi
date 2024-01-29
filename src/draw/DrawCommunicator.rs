use std::io::stdout;

use crate::terminal::Vector2;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DrawCommunicator {
    pub id: String,
    pub draw_text: String,
    pub color: crate::Color,
    pub bg_color: crate::Color,
    pub position: Vector2,
}
#[derive(Debug, Clone)]
pub struct DrawHandler {
    draws: Vec<DrawCommunicator>,
}
impl DrawHandler {
    pub fn new() -> Self {
        Self { draws: Vec::new() }
    }
    pub fn exists(&self, id: impl ToString) -> bool {
        for i in self.draws.clone() {
            if i.id == id.to_string() {
                return true;
            }
        }

        false
    }
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
    pub fn remove(&mut self, id: impl ToString) {
        let mut i = 0;
        while i < self.draws.len() {
            if self.draws[i].id == id.to_string() {
                self.draws.remove(i);
            }
            i += 1;
        }
    }
    pub fn all(&self) -> Vec<DrawCommunicator> {
        return self.draws.clone();
    }
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
    pub fn remove_all(&mut self) {
        self.draws.clear();
    }
}
