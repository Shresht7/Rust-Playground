use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

mod component;
mod input;
use input::Input;
mod messages;
use messages::Messages;

use crate::app::component::Component;

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
        frame.render_widget(self, frame.area());
    }

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Quit => self.quit(),
            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<Msg> {
        match event::read()? {
            Event::Key(key_event) if key_event.is_press() => {
                return self.handle_key_events(key_event);
            }
            _ => {}
        }
        Ok(Msg::None)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> io::Result<Msg> {
        if let Some(action) = self.input.handle_key_events(key) {
            match action {
                input::Action::Submit(s) => self.messages.push(s),
            }
        }
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
        let [area_input, area_messages] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(area);
        self.input.render(area_input, buf);
        self.messages.render(area_messages, buf);
    }
}

pub enum Msg {
    None,
    Quit,
}
