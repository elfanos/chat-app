use crate::controllers::StateEnum;
use tui::{
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
};

#[derive(Debug)]
pub struct Focus(pub Vec<Span<'static>>, pub Style);

impl Focus {
    pub fn view() -> Focus {
        Focus(vec![], Style::default().fg(Color::Yellow))
    }
}
pub enum UserName {
    Focus(Focus),
}
impl UserName {
    pub fn create(app_state: Result<&StateEnum, &StateEnum>) -> UserName {
        UserName::Focus(Focus::view())
    }

    pub fn get_view(self, text: String) -> Paragraph<'static> {
        let (msg, style) = match self {
            UserName::Focus(Focus(msg, style)) => (msg, style),
        };
        Paragraph::new(text)
            .style(style)
            .block(Block::default().borders(Borders::ALL).title("Username"))
    }
}
