use diatermi::{
    self, config,
    draw::{DrawTerminal::DrawTerminal, Text::Text},
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    is_key_press,
    terminal::{run, CursorConfig, DrawObjectTrait, DrawTrait, Terminal, TerminalsEvents, Vector2},
    Color,
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
        let mut contador = Text::new(1);
        contador.set_text(self.contador);
        config!(terminal, contador.clone());
        let mut info = Text::new(2);
        info.set_text("info");
        info.set_position(Vector2::new(0, 1));
        config!(terminal, info.clone());
        terminal.clone()
    }
    fn on_event<'a>(&mut self, terminal: &mut Terminal, event: Event) -> Terminal {
        match event {
            is_key_press!('q') => {
                self.stop = true;
            }
            _ => {}
        }
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
