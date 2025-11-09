use iced::{widget::text, Element};

#[derive(Default)]
pub struct EditorView;

#[derive(Debug, Clone)]
pub enum Message {}

impl EditorView {
    pub fn new() -> Self {
        Self
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
