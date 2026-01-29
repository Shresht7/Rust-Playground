use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use crate::components::Input;

pub struct Form {
    name: Input,
    email: Input,
    password: Input,
}

impl Form {
    pub fn new() -> Self {
        Self {
            name: Input::new("Name"),
            email: Input::new("Email"),
            password: Input::new("Password"),
        }
    }
}

impl Widget for &Form {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let form_layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(area);
        self.name.render(form_layout[0], buf);
        self.email.render(form_layout[1], buf);
        self.password.render(form_layout[2], buf);
    }
}
