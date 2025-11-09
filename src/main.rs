pub mod editor;

use editor::Editor;

#[tokio::main]
async fn main() {
    let _ = iced::application(Editor::title, Editor::update, Editor::view).run();
}
