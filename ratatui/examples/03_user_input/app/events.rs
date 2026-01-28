use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

impl super::App {
    pub(super) fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, event: KeyEvent) -> std::io::Result<()> {
        match event.code {
            KeyCode::Esc => self.exit(),
            _ => {}
        }
        Ok(())
    }
}
