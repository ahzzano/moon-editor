use iced::{widget::text, Element};

use crate::editor::core::EditorState;

pub mod core;

#[derive(Default)]
pub struct EditorView {
    state: EditorState
}

#[derive(Debug, Clone)]
pub enum Message {}

impl EditorView {
    pub fn new() -> Self {
        Self {
            state: EditorState::default()
        }
    }

    pub fn title(&self) -> String {
        String::from("moon")
    }

    pub fn update(&mut self, message: Message) {
        match message {}
    }

    pub fn view(&self) -> Element<'_, Message> {
        text("Hello world!").into()
    }
}
