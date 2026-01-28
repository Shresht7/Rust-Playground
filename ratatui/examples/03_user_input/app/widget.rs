use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

impl super::App {
    pub(super) fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &super::App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let layout = Layout::vertical([Constraint::Length(3)]).split(area);
        let block = Block::bordered()
            .title(" label ".dark_gray())
            .padding(Padding::horizontal(2))
            .borders(Borders::ALL);
        let text = Paragraph::new(self.value.clone()).block(block);
        text.render(layout[0], buf);
    }
}
