use tui::layout::{Constraint, Direction, Layout, Rect};
pub struct Container(pub Vec<Rect>);

impl Container {
    pub fn create(f: Rect) -> Container {
        Container(
            Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(5),
                        Constraint::Percentage(5),
                        Constraint::Percentage(60),
                        Constraint::Percentage(30),
                    ]
                    .as_ref(),
                )
                .split(f),
        )
    }
}
