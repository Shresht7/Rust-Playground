use ratatui::DefaultTerminal;

mod events;
mod widget;

pub struct App {
    value: String,
    history: Vec<String>,
    /// Whether the application should exit
    should_exit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            value: String::new(),
            history: Vec::new(),
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

    fn push_char(&mut self, c: char) {
        self.value.push(c);
    }

    fn pop_char(&mut self) {
        self.value.pop();
    }

    fn submit(&mut self) {
        self.history.push(self.value.clone());
        self.clear();
    }

    fn clear(&mut self) {
        self.value = String::new();
    }

    fn exit(&mut self) {
        self.should_exit = true;
    }
}
