use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Paragraph, Widget},
};

use crate::components::{Form, Input};

enum Screen {
    Form,
    Results,
}

pub struct App {
    form: Form,
    should_quit: bool,
    screen: Screen,
    submitted_data: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            form: Form::new(vec![
                Input::new("Name"),
                Input::new("Email"),
                Input::new("Password"),
            ]),
            should_quit: false,
            screen: Screen::Form,
            submitted_data: Vec::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            let msg = self.handle_events()?;
            self.update(msg);
        }
        Ok(())
    }

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Quit => self.quit(),
            Msg::None => {}
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<Msg> {
        match event::read()? {
            Event::Key(key_event) if key_event.is_press() => {
                return self.handle_key_events(key_event);
            }
            _ => Ok(Msg::None),
        }
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> io::Result<Msg> {
        match self.screen {
            Screen::Form => match key.code {
                KeyCode::Esc => Ok(Msg::Quit),
                _ => {
                    if self.form.handle_event(key) {
                        self.submitted_data = self.form.get_values();
                        self.screen = Screen::Results;
                    }
                    Ok(Msg::None)
                }
            },
            Screen::Results => match key.code {
                KeyCode::Char('e') => {
                    self.screen = Screen::Form;
                    Ok(Msg::None)
                }
                KeyCode::Char('q') | KeyCode::Esc => Ok(Msg::Quit),
                _ => Ok(Msg::None),
            },
        }
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self.screen {
            Screen::Form => {
                let main = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
                    .spacing(4)
                    .split(area);

                Paragraph::new(" FORM ".bold())
                    .centered()
                    .render(main[0], buf);

                let layout = Layout::horizontal([
                    Constraint::Fill(1),
                    Constraint::Min(40),
                    Constraint::Fill(1),
                ])
                .split(main[1]);
                self.form.render(layout[1], buf);
            }
            Screen::Results => {
                let main = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
                    .spacing(4)
                    .split(area);

                Paragraph::new(" RESULTS ".bold())
                    .centered()
                    .render(main[0], buf);

                let text: String = self.submitted_data.join("\n");
                Paragraph::new(text).centered().render(main[1], buf);
            }
        }
    }
}

pub enum Msg {
    None,
    Quit,
}
