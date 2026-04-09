use std::error::Error;

mod app;
mod context;
mod ui;
use app::App;

fn main() -> color_eyre::Result<(), Box<dyn Error>> {
    color_eyre::install()?;

    let mut app = App::new();

    ratatui::run(|terminal| {
        app.run(terminal).unwrap();
    });

    if app.print_json {
        app.print_json()?;
    }

    Ok(())
}
