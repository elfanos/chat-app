use tui::layout::{Constraint, Direction, Layout, Rect};
pub struct UserContainer(pub Vec<Rect>);

impl UserContainer {
    pub fn create(f: Rect) -> UserContainer {
        UserContainer(
            Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(2),
                        Constraint::Percentage(10),
                        Constraint::Percentage(88),
                    ]
                    .as_ref(),
                )
                .split(f),
        )
    }
}
