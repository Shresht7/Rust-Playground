use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use crate::components::Input;

pub struct Form {
    fields: Vec<Input>,
    selected_index: usize,
}

impl Form {
    pub fn new(mut fields: Vec<Input>) -> Self {
        if !fields.is_empty() {
            fields[0].focus();
        }
        Self {
            fields,
            selected_index: 0,
        }
    }

    pub fn next_focus(&mut self) {
        self.fields[self.selected_index].blur();
        self.selected_index = (self.selected_index + 1) % self.fields.len();
        self.fields[self.selected_index].focus();
    }

    pub fn handle_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Tab => self.next_focus(),
            _ => self.fields[self.selected_index].handle_event(event),
        }
    }
}

impl Widget for &Form {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let form_layout =
            Layout::vertical(self.fields.iter().map(|_| Constraint::Length(3))).split(area);
        self.fields
            .iter()
            .enumerate()
            .for_each(|(i, f)| f.render(form_layout[i], buf));
    }
}
