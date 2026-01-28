use ratatui::DefaultTerminal;

mod events;
mod widget;

pub struct App {
    value: String,
    /// Whether the application should exit
    should_exit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            value: String::new(),
            should_exit: false,
        }
    }
}

impl App {
    pub fn new<T: Into<String>>(value: T) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.should_exit = true;
    }
}
