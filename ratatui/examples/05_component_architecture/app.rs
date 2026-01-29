use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, text::Span, widgets::Widget};

pub struct App {
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            let msg = self.handle_events()?;
            self.update(msg);
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Quit => self.quit(),
            _ => {}
        }
    }

    fn handle_events(&self) -> io::Result<Msg> {
        match event::read()? {
            Event::Key(key_event) if key_event.is_press() => {
                return self.handle_key_events(key_event);
            }
            _ => {}
        }
        Ok(Msg::None)
    }

    fn handle_key_events(&self, key: KeyEvent) -> io::Result<Msg> {
        match key.code {
            KeyCode::Esc => Ok(Msg::Quit),
            _ => Ok(Msg::None),
        }
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let text = Span::from("Hello World");
        text.render(area, buf);
    }
}

pub enum Msg {
    None,
    Quit,
}
