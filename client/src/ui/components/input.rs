use crate::controllers::StateEnum;
use tui::{
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
};

#[derive(Debug)]
pub struct Focus(pub Vec<Span<'static>>, pub Style);

impl Focus {
    pub fn view() -> Focus {
        Focus(vec![], Style::default().fg(Color::Yellow))
    }
}

pub struct Stale(pub Vec<Span<'static>>, pub Style);

impl Stale {
    pub fn view() -> Stale {
        Stale(vec![], Style::default())
    }
}

pub enum Input {
    Focus(Focus),
    Stale(Stale),
}

impl Input {
    pub fn create(app_state: Result<&StateEnum, &StateEnum>) -> Input {
        if let StateEnum::Message = app_state.unwrap() {
            return Input::Focus(Focus::view());
        }
        Input::Stale(Stale::view())
    }

    pub fn get_view(self, text: String) -> Paragraph<'static> {
        let (_, style) = match self {
            Input::Focus(Focus(msg, style)) => (msg, style),
            Input::Stale(Stale(msg, style)) => (msg, style),
        };
        Paragraph::new(text)
            .style(style)
            .block(Block::default().borders(Borders::ALL).title("Input"))
    }
}
