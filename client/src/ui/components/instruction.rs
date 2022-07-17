use crate::controllers::StateEnum;
use tui::{
    style::{Modifier, Style},
    text::{Span, Spans, Text},
    widgets::Paragraph,
};

#[derive(Debug)]
pub struct Start(pub Vec<Span<'static>>, pub Style);

impl Start {
    pub fn view() -> Start {
        Start(
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        )
    }
}

pub struct Messaging(pub Vec<Span<'static>>, pub Style);

impl Messaging {
    pub fn view() -> Messaging {
        Messaging(
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        )
    }
}

pub enum Instruction {
    Start(Start),
    Messaging(Messaging),
}

impl Instruction {
    pub fn create(app_state: Result<&StateEnum, &StateEnum>) -> Instruction {
        if let StateEnum::Message = app_state.unwrap() {
            return Instruction::Messaging(Messaging::view());
        }
        Instruction::Start(Start::view())
    }

    pub fn get_view(self) -> Paragraph<'static> {
        let (msg, style) = match self {
            Instruction::Start(Start(msg, style)) => (msg, style),
            Instruction::Messaging(Messaging(msg, style)) => (msg, style),
        };
        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        Paragraph::new(text)
    }
}
