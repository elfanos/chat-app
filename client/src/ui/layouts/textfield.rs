use tui::layout::{Constraint, Direction, Layout, Rect};
pub struct Textfield(pub Vec<Rect>);

impl Textfield {
    pub fn create(f: Rect) -> Textfield {
        Textfield(
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(f),
        )
    }
}
