use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

pub struct Input {
    value: String,
    label: String,
    focused: bool,
}

impl Input {
    pub fn new<T: Into<String>>(label: T) -> Self {
        Self {
            value: String::new(),
            label: label.into(),
            focused: false,
        }
    }

    pub fn focus(&mut self) {
        self.focused = true;
    }

    pub fn blur(&mut self) {
        self.focused = false;
    }

    pub fn handle_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(c) => self.value.push(c),
            KeyCode::Backspace => {
                self.value.pop();
            }
            _ => {}
        }
    }
}

impl Widget for &Input {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let style = if self.focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let block = Block::bordered()
            .title(format!(" {} ", self.label.clone()))
            .borders(Borders::ALL)
            .border_style(style)
            .padding(Padding::horizontal(2));
        Paragraph::new(self.value.clone())
            .block(block)
            .render(area, buf);
    }
}
