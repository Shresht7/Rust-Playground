mod app;
use app::App;

fn main() -> std::io::Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}
