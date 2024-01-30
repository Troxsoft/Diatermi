// A macro named `print_message`
#[doc = "
Configure an object and draw it
### ¿What?:
- config! -> `terminal.register(/*you object */)`
- config! -> `terminal.draw_handler.draw(/*you object id*/)`
"]
#[macro_export]
macro_rules! config {
    // Match rule that takes an argument expression
    ($terminal:expr,$object:expr) => {
        $terminal.register($object);
        $terminal.draw_handler.draw($object.communicator().id);
    };
}
use crate::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
#[doc = "
Executes the function if the key is pressed
### ¿What?:
- `is_key_press!` = 
```
Event::Key(KeyEvent {
    code: KeyCode::Char(/*you key*/),
    kind: KeyEventKind::Press,
    modifiers: KeyModifiers::NONE,
    state,
})
```
"]
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
#[doc = "
Executes the function if it is pressed, otherwise if another function is supplied it will execute it
### ¿What?:
- `is_key_press_event!` = 
```
match event {
    Event::Key(KeyEvent {
        code: KeyCode::Char(/* you key code */),
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        state,
    }) => {/*execute function */},
    _ => {}
}
```
- `is_key_press_event` = 
```
match event {
    Event::Key(KeyEvent {
        code: KeyCode::Char(/* you key code */),
        kind: KeyEventKind::Press,
        modifiers: KeyModifiers::NONE,
        state,
    }) => {/*execute function */},
    _ => {/*execute other function */}
}
```
"]
#[macro_export]
macro_rules! is_key_press_event {
    // Match rule that takes an argument expression
    ($event:expr,$func:expr,$object:expr) => {
        match $event {
            Event::Key(KeyEvent {
                code: KeyCode::Char($object),
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::NONE,
                state,
            }) => $func(),
            _ => {}
        }
    };
    ($event:expr,$func:expr,$func2:expr,$object:expr) => {
        match $event {
            Event::Key(KeyEvent {
                code: KeyCode::Char($object),
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::NONE,
                state,
            }) => $func(),
            _ => $func2(),
        }
    };
}
#[doc = "
Executes the function if the key is pressed repeatedly
### ¿What?:
- `is_key_repeat!` = 
```
Event::Key(KeyEvent {
    code: KeyCode::Char(/*you key*/),
    kind: KeyEventKind::Repeat,
    modifiers: KeyModifiers::NONE,
    state,
})
```
"]
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
#[doc = "
It executes the function if a key is pressed repeatedly, otherwise if it was provided, the other function will be executed.
### ¿What?:
- `is_key_repeat_event!` = 
```
match event {
    Event::Key(KeyEvent {
        code: KeyCode::Char(/* you key code */),
        kind: KeyEventKind::Repeat,
        modifiers: KeyModifiers::NONE,
        state,
    }) => {/*execute function */},
    _ => {}
}
```
- `is_key_repeat_event!` = 
```
match event {
    Event::Key(KeyEvent {
        code: KeyCode::Char(/* you key code */),
        kind: KeyEventKind::Repeat,
        modifiers: KeyModifiers::NONE,
        state,
    }) => {/*execute function */},
    _ => {/*execute other function */}
}
```
"]
#[macro_export]
macro_rules! is_key_repeat_event {
    // Match rule that takes an argument expression
    ($event:expr,$func:expr,$object:expr) => {
        match $event {
            Event::Key(KeyEvent {
                code: KeyCode::Char($object),
                kind: KeyEventKind::Repeat,
                modifiers: KeyModifiers::NONE,
                state,
            }) => $func(),
            _ => {}
        }
    };
    ($event:expr,$func:expr,$func2:expr,$object:expr) => {
        match $event {
            Event::Key(KeyEvent {
                code: KeyCode::Char($object),
                kind: KeyEventKind::Repeat,
                modifiers: KeyModifiers::NONE,
                state,
            }) => $func(),
            _ => $func2(),
        }
    };
}
#[doc = "
executes the function if the pressed key rises
### ¿What?:
- `is_key_release!` = 
```
Event::Key(KeyEvent {
    code: KeyCode::Char(/*you key*/),
    kind: KeyEventKind::Release,
    modifiers: KeyModifiers::NONE,
    state,
})
```
"]
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
#[doc = "

executes the function if the pressed key rises or if you have another function it will execute it otherwise
### ¿What?:
- `is_key_release_event!` = 
```
match event {
    Event::Key(KeyEvent {
        code: KeyCode::Char(/* you key code */),
        kind: KeyEventKind::Release,
        modifiers: KeyModifiers::NONE,
        state,
    }) => {/*execute function */},
    _ => {}
}
```
- `is_key_release_event!` = 
```
match event {
    Event::Key(KeyEvent {
        code: KeyCode::Char(/* you key code */),
        kind: KeyEventKind::Release,
        modifiers: KeyModifiers::NONE,
        state,
    }) => {/*execute function */},
    _ => {/*execute other function */}
}
```
"]
#[macro_export]
macro_rules! is_key_release_event {
    // Match rule that takes an argument expression
    ($event:expr,$func:expr,$object:expr) => {
        match $event {
            Event::Key(KeyEvent {
                code: KeyCode::Char($object),
                kind: KeyEventKind::Release,
                modifiers: KeyModifiers::NONE,
                state,
            }) => $func(),
            _ => {}
        }
    };
    ($event:expr,$func:expr,$func2:expr,$object:expr) => {
        match $event {
            Event::Key(KeyEvent {
                code: KeyCode::Char($object),
                kind: KeyEventKind::Release,
                modifiers: KeyModifiers::NONE,
                state,
            }) => $func(),
            _ => $func2(),
        }
    };
}
#[doc = "Create a `Vector2`
### Example:
```
vec2(0,1);
```
"]
#[macro_export]
macro_rules! vec2 {
    // Match rule that takes an argument expression
    ($x:expr,$y:expr) => {
        Vector2::new($x, $y);
    };
}
#[doc = "
Create a `text` with the possible `arguments`:
- id(*)
- text
- position
### Example:
```
let mut info = text!(0,\"this is a example\",vec2(0,0));
```
### Example 2:
```
let mut info = text!(0,\"this is a example\");
```
### Example 3:
```
let mut info = text!(0);
```
"]
#[macro_export]
macro_rules! text {
    // Match rule that takes an argument expression
    ($id:expr) => {
        Text::new($id);
    };
    ($id:expr,$text:expr) => {
        Text::new($id).with_text($text);
    };
    ($id:expr,$text:expr,$position:expr) => {
        Text::new($id).with_text($text).with_position($position);
    };
}
