use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Widget},
};

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
