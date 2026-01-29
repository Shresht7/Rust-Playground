use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

pub struct Input {
    value: String,
    label: String,
}

impl Input {
    pub fn new<T: Into<String>>(label: T) -> Self {
        Self {
            value: String::new(),
            label: label.into(),
        }
    }
}

impl Widget for &Input {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered()
            .title(format!(" {} ", self.label.clone()))
            .borders(Borders::ALL)
            .padding(Padding::horizontal(2));
        Paragraph::new(self.value.clone())
            .block(block)
            .render(area, buf);
    }
}
