use crossterm::event::KeyEvent;
use ratatui::{buffer::Buffer, layout::Rect};

pub trait Component {
    type Action;
    fn render(&self, area: Rect, buf: &mut Buffer);
    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Self::Action> {
        None
    }
}
