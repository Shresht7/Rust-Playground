use ratatui::{Frame, buffer::Buffer, layout::Rect, text::Text, widgets::Widget};

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
        let text = Text::from(self.value.clone());
        text.render(area, buf);
    }
}
