// A macro named `print_message`
#[macro_export]
macro_rules! config {
    // Match rule that takes an argument expression
    ($terminal:expr,$object:expr) => {
        $terminal.register($object);
        $terminal.draw_handler.draw($object.communicator().id);
    };
}
use crate::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
#[macro_export]
macro_rules! is_key_press {
    // Match rule that takes an argument expression
    ($object:expr) => {
        Event::Key(KeyEvent {
            code: KeyCode::Char($object),
            kind: KeyEventKind::Press,
            modifiers: KeyModifiers::NONE,
            state,
        })
    };
}
#[macro_export]
macro_rules! is_key_repeat {
    // Match rule that takes an argument expression
    ($object:expr) => {
        Event::Key(KeyEvent {
            code: KeyCode::Char($object),
            kind: KeyEventKind::Repeat,
            modifiers: KeyModifiers::NONE,
            state,
        })
    };
}
#[macro_export]
macro_rules! is_key_release {
    // Match rule that takes an argument expression
    ($object:expr) => {
        Event::Key(KeyEvent {
            code: KeyCode::Char($object),
            kind: KeyEventKind::Release,
            modifiers: KeyModifiers::NONE,
            state,
        })
    };
}
