use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Span,
    widgets::{List, ListItem, Widget},
};

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
    fn render(self, area: Rect, buf: &mut Buffer) {
        let list_items = self.list.iter().map(|item| ListItem::new(Span::from(item)));
        List::new(list_items).render(area, buf);
    }
}
