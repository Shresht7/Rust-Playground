use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use crate::components::Input;

pub struct Form {
    fields: Vec<Input>,
}

impl Form {
    pub fn new(fields: Vec<Input>) -> Self {
        Self { fields }
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
