pub mod editor;

use editor::EditorView;

#[tokio::main]
async fn main() {
    let _ = iced::application(EditorView::title, EditorView::update, EditorView::view)
        .subscription(EditorView::subscription)
        .run_with(EditorView::new);
}
