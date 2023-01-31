use super::StateEnum;
use crate::domain::Writer;
use crossterm::event::{KeyCode, KeyEvent};
use std::net::TcpStream;

pub struct Sender {
    pub writer: Writer<TcpStream>,
}
impl Sender {
    pub fn create(writer: Writer<TcpStream>) -> Sender {
        Sender { writer }
    }
    pub fn update(
        &mut self,
        key: KeyEvent,
        app_state: Result<&StateEnum, &StateEnum>,
        message: &mut String,
    ) {
        match app_state.unwrap() {
            StateEnum::Message => {
                if let KeyCode::Enter = key.code {
                    self.writer.write(message.as_bytes())
                }
            }
            StateEnum::NoUserName => {
                if let KeyCode::Enter = key.code {
                    self.writer.write(message.as_bytes())
                }
            }
            _ => {}
        }
        // if let StateEnum::Message = app_state.unwrap() {
        //     if let KeyCode::Enter = key.code {
        //         self.writer.write(message.as_bytes())
        //     }
        // }
    }
}
