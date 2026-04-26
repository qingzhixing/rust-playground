use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::context::{Context, CurrentEditing, CurrentScreen};
use crate::ui::Ui;

pub struct App {
    pub exiting: bool,
    pub print_json: bool,
    pub context: Context,
}

impl App {
    pub fn new() -> Self {
        Self {
            exiting: false,
            print_json: false,
            context: Context::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> color_eyre::Result<()> {
        while !self.exiting {
            let ui = Ui::new(&self.context);
            terminal.draw(|frame| {
                ui.draw(frame).unwrap();
            })?;
            match event::read()? {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) -> color_eyre::Result<()> {
        match self.context.current_screen {
            // Main screen
            CurrentScreen::Main => match key_event.code {
                KeyCode::Char('e') => {
                    self.context.current_screen = CurrentScreen::Editing;
                    self.context.currently_editing = Some(CurrentEditing::Key);
                }
                KeyCode::Char('q') => {
                    self.context.current_screen = CurrentScreen::Exiting;
                }
                _ => {}
            },

            // Exiting screen
            CurrentScreen::Exiting => match key_event.code {
                KeyCode::Char('y') => {
                    self.exiting = true;
                    self.print_json = true;
                }
                KeyCode::Char('n') | KeyCode::Char('q') => {
                    self.exiting = true;
                    self.print_json = false;
                }
                _ => {}
            },

            // Editing screen
            CurrentScreen::Editing => match key_event.code {
                KeyCode::Enter => {
                    self.submit_editing();
                }
                KeyCode::Backspace => {
                    self.delete_char();
                }
                KeyCode::Esc => {
                    self.context.current_screen = CurrentScreen::Main;
                    self.context.currently_editing = None;
                }
                KeyCode::Tab => {
                    self.toggle_editing();
                }
                KeyCode::Char(value) => {
                    self.input_char(value);
                }
                _ => {}
            },
        }
        Ok(())
    }

    fn submit_editing(&mut self) {
        if let Some(editing) = &self.context.currently_editing {
            match editing {
                CurrentEditing::Key => {
                    self.context.currently_editing = Some(CurrentEditing::Value);
                }
                CurrentEditing::Value => {
                    self.save_key_value();
                    self.context.current_screen = CurrentScreen::Main;
                }
            }
        }
    }

    fn delete_char(&mut self) {
        if let Some(editing) = &self.context.currently_editing {
            match editing {
                CurrentEditing::Key => {
                    self.context.key_input.pop();
                }
                CurrentEditing::Value => {
                    self.context.value_input.pop();
                }
            }
        }
    }

    fn input_char(&mut self, c: char) {
        if let Some(editing) = &self.context.currently_editing {
            match editing {
                CurrentEditing::Key => {
                    self.context.key_input.push(c);
                }
                CurrentEditing::Value => {
                    self.context.value_input.push(c);
                }
            }
        }
    }

    fn save_key_value(&mut self) {
        self.context.pairs.insert(
            self.context.key_input.clone(),
            self.context.value_input.clone(),
        );

        self.context.key_input = String::new();
        self.context.value_input = String::new();
        self.context.currently_editing = None;
    }

    fn toggle_editing(&mut self) {
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
