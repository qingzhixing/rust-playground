use std::error::Error;

mod app;
mod ui;
use app::App;

fn main() -> color_eyre::Result<(), Box<dyn Error>> {
    color_eyre::install()?;
    ratatui::run(|terminal| {
        App::new().run(terminal).unwrap();
    });
    Ok(())
}
