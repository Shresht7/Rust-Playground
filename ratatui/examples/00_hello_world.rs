fn main() {
    ratatui::run(|terminal| app(terminal)).unwrap();
}

fn app(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| draw(frame))?; // Draw the frame
        if crossterm::event::read().unwrap().is_key_press() {
            break; // Break the infinite draw loop
        }
    }
    Ok(())
}

fn draw(frame: &mut ratatui::Frame) {
    frame.render_widget("Hello Ratatui! 🐀", frame.area())
}
