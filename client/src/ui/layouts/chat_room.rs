use tui::layout::{Constraint, Direction, Layout, Rect};
pub struct ChatRoom(pub Vec<Rect>);

impl ChatRoom {
    pub fn create(f: Rect) -> ChatRoom {
        ChatRoom(
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(f),
        )
    }
}
