use std::sync::Arc;

use iced::{
    Element, Subscription, Task,
    keyboard::{self, Key, Modifiers},
    widget::{column, text},
};

use crate::editor::core::{EditorState, load_file};

pub mod core;

#[derive(Default)]
pub struct EditorView {
    state: EditorState,
}

#[derive(Debug)]
pub enum Message {
    KeyPress(Key, Modifiers),
    Saved,
    FileOpened(Result<Arc<String>, tokio::io::ErrorKind>),
}

impl EditorView {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                state: EditorState::default(),
            },
            Task::perform(
                load_file(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR"))),
                Message::FileOpened,
            ),
        )
    }

    pub fn title(&self) -> String {
        String::from("moon")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::KeyPress(key, mods) => {
                if mods.is_empty() {
                    match key.as_ref() {
                        Key::Character(s) => {
                            self.state.write_line(s);

                        }
                        Key::Named(keyboard::key::Named::Backspace) => {
                            self.state.backspace();
                        }
                        Key::Named(keyboard::key::Named::Space) => {
                            self.state.write_line(" ");
                        }
                        _ => {}
                        
                    }
                }
                Task::none()
            }
            Message::FileOpened(Ok(content)) => {
                println!("Loading content...");
                self.state.load_lines(&content);
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let binding = self.state.get_lines();

        let lines = binding.iter().map(|line| text!("{}", line).into());

        column(lines).into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, mods| Some(Message::KeyPress(key, mods)))
    }
}
