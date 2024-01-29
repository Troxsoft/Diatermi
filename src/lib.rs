pub mod draw;
pub mod terminal;
pub mod utils;
pub use crossterm::cursor::SetCursorStyle;
pub use crossterm::event;
pub use crossterm::style::Color;
pub use draw::{
    DrawCommunicator::{DrawCommunicator, DrawHandler},
    DrawTerminal::DrawTerminal,
    Text::Text,
};
pub use terminal::{
    run, CursorConfig, DrawObjectTrait, DrawTrait, Terminal, TerminalsEvents, Vector2,
};
pub use utils::*;
