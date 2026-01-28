use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct App {
    input: Input,
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            input: Input::new(),
            should_quit: false,
        }
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
        frame.render_widget(self, frame.area())
    }

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Quit => self.quit(),
            Msg::Input(c) => {
                self.input.push(c);
            }
            Msg::Backspace => {
                self.input.pop();
            }
            Msg::None => {}
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
            KeyCode::Char(c) => Ok(Msg::Input(c)),
            KeyCode::Backspace => Ok(Msg::Backspace),
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
        self.input.render(area, buf);
    }
}

pub enum Msg {
    None,
    Quit,
    Input(char),
    Backspace,
}

pub struct Input {
    value: String,
}

impl Input {
    pub fn new() -> Self {
        Self {
            value: String::new(),
        }
    }

    pub fn push(&mut self, c: char) {
        self.value.push(c);
    }

    pub fn pop(&mut self) {
        self.value.pop();
    }
}

impl Widget for &Input {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered().borders(Borders::ALL);
        Paragraph::new(self.value.clone())
            .block(block)
            .render(area, buf);
    }
}
