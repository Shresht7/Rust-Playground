use ratatui::DefaultTerminal;

mod events;
mod widget;

pub struct App {
    /// Whether the application should exit
    should_exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self { should_exit: false }
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
