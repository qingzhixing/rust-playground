use ratatui::{
    Frame,
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
};

use crate::app;

pub struct Ui {}

impl Ui {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, frame: &mut Frame, app: &mut app::App) -> color_eyre::Result<()> {
        frame.render_widget(self, frame.area());
        Ok(())
    }
}

impl Widget for &Ui {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        Paragraph::new("Hello, world!")
            .centered()
            .render(area, buffer);
    }
}
