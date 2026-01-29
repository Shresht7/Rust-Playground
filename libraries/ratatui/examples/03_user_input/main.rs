mod app;
use app::App;

fn main() -> std::io::Result<()> {
    let mut app = App::default();
    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}
