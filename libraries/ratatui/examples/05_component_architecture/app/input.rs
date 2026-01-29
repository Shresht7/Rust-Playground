use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::app::component::Component;

pub struct Input {
    value: String,
}

impl Input {
    pub fn new() -> Self {
        Self {
            value: String::new(),
        }
    }

    pub fn value(&self) -> String {
        self.value.clone()
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

pub enum Action {
    Submit(String),
}

impl Component for Input {
    type Action = Action;

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().borders(Borders::ALL);
        Paragraph::new(self.value()).block(block).render(area, buf);
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Self::Action> {
        match key.code {
            KeyCode::Char(c) => self.push(c),
            KeyCode::Backspace => self.pop(),
            KeyCode::Enter => {
                let value = self.value();
                self.clear();
                return Some(Action::Submit(value));
            }
            _ => {}
        }
        None
    }
}
