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
#[doc = "A `Vector2` representing a coordinate in the terminal `[x,y]` `u32`"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector2 {
    pub x: u32,
    pub y: u32,
}
impl Vector2 {
    #[doc = "create an instance of `Vector2`
## Example:
```
let vector:Vector2 = Vector2::new(0,0);
````
"]

    pub fn new(x: u32, y: u32) -> Self {
        Self { x: x, y: y }
    }
}
#[doc = "
Represents an object that can be `drawn` or converted into a `communicator` for subsequent `drawing`"]
pub trait DrawObjectTrait<'a> {
    #[doc = "
`Create` an implementation of `DrawObjectTrait`
- Receives a unique ID, if the ID is repeated it opens strange behaviors
### Example:
```
let mut object = /*you  implementation of DrawObjectTrait*/::new(1/*ID*/);
```
"]
    fn new(id: impl ToString) -> Self;
    #[doc = "
`Change` the `color` ( *foreground color* )
### Example:
```
you_object.set_color(Color::Red);
```
"]
    fn set_color(&mut self, color: crate::Color);
    #[doc = "
`Change` the `position` ( *Vector2* )
### Example:
```
you_object.set_position(vec2(2,5));
```
"]
    fn set_position(&mut self, position: Vector2);
    #[doc = "`Returns` the `color` ( *foreground color* )
### Example:
```
let color_info = object.color();
```
    "]
    fn color(&self) -> crate::Color;
    #[doc = "
`Change` the `bg color` ( *background color* )
### Example:
```
you_object.set_bg_color(Color::Blue);
```
"]
    fn set_bg_color(&mut self, color: crate::Color);
    #[doc = "`Returns` the `bg color` ( *background color* )
### Example:
```
let color_info = object.bg_color();
```
    "]
    fn bg_color(&self) -> crate::Color;
    #[doc = "`Returns` the `position` ( *Vector2* )
### Example:
```
let position_info = object.position();
```
    "]
    fn position(&self) -> Vector2;
    #[doc = "`Returns` the `ID` as ( *String* )
### Example:
```
let id_info = object.id();
```
    "]
    fn id(&self) -> String;
    #[doc = "`Clear` stored `text`

    "]
    fn clear(&mut self);
    #[doc = "
`Returns` internal `communicator`( *DrawCommunicator* )
    "]
    fn communicator(&self) -> DrawCommunicator;
}
#[doc = "Takes care of very basic window things"]
pub trait DrawTrait<'a> {
    #[doc = "
Create a `DrawTrait` implementation"]
    fn new() -> Self;
    #[doc = "Clean the screen"]
    fn clear_all(&mut self);
    #[doc = "Clear the screen in a specific position"]
    fn clear_position(&mut self, pos: Vector2);
}
#[doc = " Represents the `Terminal` and contains methods to modify and `draw`
### Example of counter:
```
use diatermi::{
    config, is_key_press, is_key_press_event, run, text, vec2, DrawObjectTrait, Event, KeyCode,
    KeyEvent, KeyEventKind, KeyModifiers, Terminal, TerminalsEvents, Text, Vector2,
};

fn main() {
    run(App::new());
}
struct App {
    contador: i32,
    stop: bool,
}
impl TerminalsEvents for App {
    fn new() -> Self {
        Self {
            contador: 0,
            stop: false,
        }
    }
    fn on_loop<'a>(&mut self, terminal: &'a mut Terminal) -> Terminal {
        self.contador += 1;
        let mut contador = text!(1, self.contador);
        config!(terminal, contador.clone());
        let mut info = text!(2, \"info\", vec2!(0, 1));
        config!(terminal, info.clone());
        terminal.clone()
    }
    fn on_event<'a>(&mut self, terminal: &mut Terminal, event: Event) -> Terminal {
        is_key_press_event!(
            event,
            || {
                self.stop = true;
            },
            'q'
        );
        terminal.clone()
    }
    fn on_start<'a>(&mut self, terminal: &mut Terminal) -> Terminal {
        terminal.hide_cursor();
        terminal.clone()
    }
    fn on_end<'a>(&mut self, terminal: &mut Terminal) -> Terminal {
        terminal.show_cursor();
        terminal.clone()
    }
    fn is_stop<'a>(&mut self, terminal: &mut Terminal) -> (bool, Terminal) {
        (self.stop, terminal.clone())
    }
}

```
"]
#[derive(Debug, Clone)]
pub struct Terminal {
    #[doc = " Instance of `DrawTerminal`"]
    pub draw: DrawTerminal,
    #[doc = " Instance of ` DrawHandler`"]
    pub draw_handler: DrawHandler,
}
#[doc = "
`Events` from a console application or a `CLI` are handled `here`
"]
pub trait TerminalsEvents {
    #[doc = "
Create a `terminal` application using `TerminalsEvents`
### How to Use:
```
impl Terminal for AppName{
    fn new()->Self{
        Self{
            //...
        }
    }
    //...
}
```

"]
    fn new() -> Self;
    #[doc = "
The loop of an application `this can be interrupted modifying the is_stop function`
### Example:
```
fn on_loop<'a>(&mut self, terminal: &'a mut Terminal) -> Terminal {
    terminal.clone()
}
````
"]
    fn on_loop<'a>(&mut self, terminal: &'a mut Terminal) -> Terminal;
    #[doc = "

Executed when an event is received
### Example:
```
fn on_event<'a>(&mut self, terminal: &mut Terminal, event: Event) -> Terminal {
    is_key_press_event!(
        event,
        || {
            //you code
        },
        'q' // you key char
    );
    terminal.clone()
}
```
"]
    fn on_event<'a>(&mut self, terminal: &'a mut Terminal, event: Event) -> Terminal {
        terminal.clone()
    }
    #[doc = "
Here the code is executed at the `beginning` after the `new` 

### Example:
```
fn on_start<'a>(&mut self, terminal: &mut Terminal) -> Terminal {
    terminal.hide_cursor();
    terminal.clone()
```
}
"]
    fn on_start<'a>(&mut self, terminal: &'a mut Terminal) -> Terminal {
        terminal.clone()
    }
    #[doc = "Here the code is executed at the `end` after the `loop`
### Example:
```
fn on_end<'a>(&mut self, terminal: &mut Terminal) -> Terminal {
    terminal.show_cursor();
    terminal.clone()
```
}
"]
    fn on_end<'a>(&mut self, terminal: &'a mut Terminal) -> Terminal {
        terminal.clone()
    }
    #[doc = "
This function `informs` the application if it has to `stop`

### Example:
```
fn is_stop<'a>(&mut self, terminal: &mut Terminal) -> (bool, Terminal) {
    (true, terminal.clone())
}
```
"]
    fn is_stop<'a>(&mut self, terminal: &'a mut Terminal) -> (bool, Terminal) {
        (false, terminal.clone())
    }
}
#[doc = "
The cursor configuration is saved by this structure.
To apply the configuration please use:
``` 
terminal.set_cursor(/*you config */);
```

"]
pub struct CursorConfig {
    #[doc = "¿Is `cursor` `hide`?"]
    pub hide: bool,
    #[doc = "The cursor style.
This uses a `crossterm` structure but is `re-exported` and  called: `SetCursorStyle`"]
    pub style: crate::SetCursorStyle,
}

impl CursorConfig {
    #[doc = "Create a `CursorConfig` 
### Example:
```
CursorConfig::new(false,SetCursorStyle::DefaultUserShape);
```    
    
"]
    pub fn new(hide: bool, style: crate::SetCursorStyle) -> Self {
        Self {
            hide: hide,
            style: style,
        }
    }
}
#[doc = "`initialize` the `application` in a `loop`
### Example:
```
// you code 
fn main(){
    /*App is the structure that implements TerminalsEvents */
    run(App::new());
}
// you code x2
```
"]
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
    #[doc = "
Create a `Terminal`
"]
    pub fn new() -> Self {
        Terminal {
            draw: DrawTerminal::new(),
            draw_handler: DrawHandler::new(),
        }
    }
    #[doc = "registers the `communicator` of the object with the `handler`. This allows it to be drawn later
### How to use:
```
/* object is what you want to draw. Example a text, and more */
terminal.register(object.clone());
```
    "]
    pub fn register(&mut self, object: impl DrawObjectTrait<'a>) {
        self.draw_handler.add(object.communicator());
    }
    #[doc = "`change` `terminal cursor` `settings`
### Example:
```
terminal.set_cursor(CursorConfig::new(false, SetCursorStyle::BlinkingBar));
```    
"]
    pub fn set_cursor(&self, cursor: CursorConfig) {
        if cursor.hide {
            crossterm::execute!(stdout(), crossterm::cursor::Hide);
        } else {
            crossterm::execute!(stdout(), crossterm::cursor::Show);
        }
        crossterm::execute!(stdout(), cursor.style);
    }
    #[doc = "
`hide` the terminal `cursor`
### How to use?:
```
terminal.hide_cursor();
```
"]
    pub fn hide_cursor(&self) {
        crossterm::execute!(stdout(), crossterm::cursor::Hide);
    }
    #[doc = "
Set `only` the terminal cursor style
### Example:
```
terminal.set_cursor_style(SetCursorStyle::BlinkingBlock);
```
"]
    pub fn set_cursor_style(&self, style: crate::SetCursorStyle) {
        crossterm::execute!(stdout(), style);
    }
    #[doc = "Set `title` of `terminal`
### Example:
```
terminal.set_title(\"Hello from ...\");    
```
"]
    pub fn set_title(&self, title: impl ToString) {
        crossterm::execute!(stdout(), crossterm::terminal::SetTitle(title.to_string()));
    }
    #[doc = "
Change the `rows` and `columns` of the `terminal` 
### Example:
```
terminal.set_size(40,40);
```
"]
    pub fn set_size(&self, columns: u16, rows: u16) {
        crossterm::execute!(stdout(), crossterm::terminal::SetSize(columns, rows));
    }
    #[doc = "`Show` terminal cursor
### How to use?:
```
terminal.show_cursor();
```
    "]
    pub fn show_cursor(&self) {
        crossterm::execute!(stdout(), crossterm::cursor::Show);
    }
}
