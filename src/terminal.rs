use std::{clone, io::stdout};

use crossterm::{
    self,
    event::{poll, read, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::draw::{
    DrawCommunicator::{DrawCommunicator, DrawHandler},
    DrawTerminal::DrawTerminal,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector2 {
    pub x: u32,
    pub y: u32,
}
impl Vector2 {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x: x, y: y }
    }
}
pub trait DrawObjectTrait<'a> {
    fn new(id: impl ToString) -> Self;
    fn set_color(&mut self, color: crate::Color);
    fn set_position(&mut self, position: Vector2);
    fn color(&self) -> crate::Color;
    fn set_bg_color(&mut self, color: crate::Color);
    fn bg_color(&self) -> crate::Color;
    fn position(&self) -> Vector2;
    fn id(&self) -> String;
    fn clear(&mut self);
    fn communicator(&self) -> DrawCommunicator;
}

pub trait DrawTrait<'a> {
    fn new() -> Self;

    fn clear_all(&mut self);
    fn clear_position(&mut self, pos: Vector2);
}
#[derive(Debug, Clone)]

pub struct Terminal {
    pub draw: DrawTerminal,
    pub draw_handler: DrawHandler,
}

pub trait TerminalsEvents {
    fn new() -> Self;
    fn on_loop<'a>(&mut self, terminal: &'a mut Terminal) -> Terminal;
    fn on_event<'a>(&mut self, terminal: &'a mut Terminal, event: Event) -> Terminal;
    fn on_start<'a>(&mut self, terminal: &'a mut Terminal) -> Terminal;
    fn on_end<'a>(&mut self, terminal: &'a mut Terminal) -> Terminal;
    fn is_stop<'a>(&mut self, terminal: &'a mut Terminal) -> (bool, Terminal);
}

pub struct CursorConfig {
    pub hide: bool,
    pub style: crate::SetCursorStyle,
}

impl CursorConfig {
    pub fn new(hide: bool, style: crate::SetCursorStyle) -> Self {
        Self {
            hide: hide,
            style: style,
        }
    }
}
pub fn run(mut app: impl TerminalsEvents) {
    enable_raw_mode().unwrap();
    let mut terminal = Terminal::new();
    crossterm::execute!(
        stdout(),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        crossterm::cursor::MoveTo(0, 0)
    );
    crossterm::execute!(stdout(), crossterm::event::EnableMouseCapture);
    terminal = app.on_start(&mut terminal.clone()).clone();

    loop {
        let aj = app.is_stop(&mut terminal.clone());
        terminal = aj.1.clone();
        if aj.0 {
            break;
        }
        if poll(std::time::Duration::from_micros(4000)).unwrap() {
            terminal = app.on_event(&mut terminal.clone(), read().unwrap()).clone();
        } else {
            terminal = app.on_loop(&mut terminal.clone()).clone();
        }
    }
    terminal = app.on_end(&mut terminal.clone()).clone();
    crossterm::execute!(stdout(), crossterm::event::DisableMouseCapture);
    disable_raw_mode().unwrap();
}
impl<'a> Terminal {
    pub fn new() -> Self {
        Terminal {
            draw: DrawTerminal::new(),
            draw_handler: DrawHandler::new(),
        }
    }
    pub fn register(&mut self, object: impl DrawObjectTrait<'a>) {
        self.draw_handler.add(object.communicator());
    }
    pub fn set_cursor(&self, cursor: CursorConfig) {
        if cursor.hide {
            crossterm::execute!(stdout(), crossterm::cursor::Hide);
        } else {
            crossterm::execute!(stdout(), crossterm::cursor::Show);
        }
        crossterm::execute!(stdout(), cursor.style);
    }
    pub fn hide_cursor(&self) {
        crossterm::execute!(stdout(), crossterm::cursor::Hide);
    }
    pub fn set_cursor_style(&self, style: crate::SetCursorStyle) {
        crossterm::execute!(stdout(), style);
    }
    pub fn set_title(&self, title: impl ToString) {
        crossterm::execute!(stdout(), crossterm::terminal::SetTitle(title.to_string()));
    }

    pub fn set_size(&self, columns: u16, rows: u16) {
        crossterm::execute!(stdout(), crossterm::terminal::SetSize(columns, rows));
    }
    pub fn show_cursor(&self) {
        crossterm::execute!(stdout(), crossterm::cursor::Show);
    }
}
