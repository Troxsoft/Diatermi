use std::{clone, io::stdout};

use crossterm::{
    self,
    event::{poll, read, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
#[derive(Debug, Clone, Copy)]
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
    fn new() -> Self;
    fn draw(&self);
    fn set_color(&mut self, color: crate::Color);
    fn set_position(&mut self, position: Vector2);
    fn color(&self) -> crate::Color;
    fn set_bg_color(&mut self, color: crate::Color);
    fn bg_color(&self) -> crate::Color;
    fn position(&self) -> Vector2;

    fn clear(&self);
}

pub trait DrawTrait<'a> {
    fn new(terminal: &'a Terminal) -> Self;

    fn clear_all(&mut self);
    fn clear_position(&mut self, pos: Vector2);
    fn draw(&self, object: impl DrawObjectTrait<'a>);
}
#[derive(Debug, Clone)]

pub struct Terminal {}

pub trait TerminalsEvents<'a> {
    fn new(terminal: &'a mut Terminal) -> Self;
    fn on_loop(&mut self);
    fn on_event(&mut self, event: Event);
    fn on_start(&mut self);
    fn on_end(&mut self);
    fn is_stop(&mut self) -> bool;
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

impl Terminal {
    pub fn new() -> Self {
        Terminal {}
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

    pub fn start<'a>(&mut self, mut app: impl TerminalsEvents<'a>) {
        enable_raw_mode().unwrap();
        crossterm::execute!(
            stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
        );
        crossterm::execute!(stdout(), crossterm::event::EnableMouseCapture);
        app.on_start();

        loop {
            if app.is_stop() {
                break;
            }
            if poll(std::time::Duration::from_micros(4000)).unwrap() {
                app.on_event(read().unwrap());
            } else {
                app.on_loop();
            }
        }
        app.on_end();
        crossterm::execute!(stdout(), crossterm::event::DisableMouseCapture);
        disable_raw_mode().unwrap();
    }
}
