use ratatui::crossterm;

use crate::context::{Context, CurrentEditing};
use crate::ui::Ui;

pub struct App {
    pub context: Context,
}

impl App {
    pub fn new() -> Self {
        Self {
            context: Context::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> color_eyre::Result<()> {
        let ui = Ui::new(&self.context);
        loop {
            terminal.draw(|frame| {
                ui.draw(frame).unwrap();
            })?;
            if crossterm::event::read()?.is_key_press() {
                break;
            }
        }
        Ok(())
    }

    pub fn save_key_value(&mut self) {
        self.context.pairs.insert(
            self.context.key_input.clone(),
            self.context.value_input.clone(),
        );

        self.context.key_input = String::new();
        self.context.value_input = String::new();
        self.context.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.context.currently_editing {
            self.context.currently_editing = match edit_mode {
                CurrentEditing::Key => Some(CurrentEditing::Value),
                CurrentEditing::Value => Some(CurrentEditing::Key),
            };
        } else {
            self.context.currently_editing = Some(CurrentEditing::Key);
        }
    }

    pub fn print_json(&self) -> color_eyre::Result<()> {
        let output = serde_json::to_string_pretty(&self.context.pairs)?;
        println!("{output}");
        Ok(())
    }
}
