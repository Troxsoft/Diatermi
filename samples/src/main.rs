use diatermi::{
    config, is_key_press, is_key_press_event, run, text, vec2, Color, CursorConfig,
    DrawObjectTrait, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, SetCursorStyle,
    Terminal, TerminalsEvents, Text, Vector2,
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
        let mut contador: Text = text!(1, self.contador);
        config!(terminal, contador.clone());
        let mut info: Text = text!(2, "info", vec2!(0, 1))
            .with_color(Color::Red)
            .with_bg_color(Color::Grey);
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
