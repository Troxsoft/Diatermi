use diatermi::{
    self,
    draw::{DrawTerminal::DrawTerminal, Text::Text},
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{CursorConfig, DrawObjectTrait, DrawTrait, Terminal, TerminalsEvents, Vector2},
    Color,
};

fn main() {
    let mut terminal = Terminal::new();

    terminal.start(App::new(&mut terminal.clone()));
}
struct App<'a> {
    draw: DrawTerminal<'a>,
    contador: i32,
    stop: bool,
    texto_1: Text,
    texto_2: Text,
}

impl<'a> TerminalsEvents<'a> for App<'a> {
    fn on_loop(&mut self) {

        //self.draw.terminal.set_title(self.texto_1.text());
    }

    fn on_start(&mut self) {
        self.draw.terminal.set_cursor(CursorConfig::new(
            true,
            diatermi::SetCursorStyle::BlinkingBar,
        ));
        self.draw.clear_all();
        self.texto_2.draw();
        self.texto_1.set_text(self.contador);
        self.texto_1.draw();
    }

    fn on_end(&mut self) {
        self.draw.terminal.set_cursor(CursorConfig::new(
            false,
            diatermi::SetCursorStyle::DefaultUserShape,
        ));
    }

    fn new(terminal: &'a mut Terminal) -> Self {
        let mut draw = DrawTerminal::new(terminal);

        return App {
            contador: 0,
            stop: false,
            texto_1: draw.text("", Vector2::new(0, 0)),
            texto_2: draw.text_color("numeros", Vector2::new(0, 1), Color::Green),
            draw: draw,
        };
    }

    fn is_stop(&mut self) -> bool {
        self.stop
    }

    fn on_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                kind,
                state,
            }) => {
                self.stop = true;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('p'),
                modifiers: KeyModifiers::NONE,
                kind,
                state,
            }) => {
                self.contador += 1;
                self.texto_1.set_text(self.contador);
                self.texto_1.draw();
            }
            /*
                        self.texto_1.set_text(self.contador);
            self.texto_1.draw(); */
            _ => {}
        }
    }
}
