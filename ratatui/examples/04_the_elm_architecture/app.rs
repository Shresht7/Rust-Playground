use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};

pub struct App {
    input: Input,
    messages: Messages,
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            input: Input::new(),
            messages: Messages::new(),
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
            Msg::Submit(msg) => {
                self.input.clear();
                self.messages.push(msg)
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
            KeyCode::Enter => Ok(Msg::Submit(self.input.value.clone())),
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
        let [area_input, area_messages] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);

        self.input.render(area_input, buf);
        self.messages.render(area_messages, buf);
    }
}

pub enum Msg {
    None,
    Quit,
    Input(char),
    Backspace,
    Submit(String),
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

    pub fn clear(&mut self) {
        self.value.clear();
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

pub struct Messages {
    list: Vec<String>,
}

impl Messages {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn push(&mut self, msg: String) {
        self.list.push(msg);
    }
}

impl Widget for &Messages {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let list_items = self.list.iter().map(|item| ListItem::new(Span::from(item)));
        List::new(list_items).render(area, buf);
    }
}
