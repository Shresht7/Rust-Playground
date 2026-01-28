mod app;
use app::App;

fn main() -> std::io::Result<()> {
    let mut app = App::new("test");
    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}
