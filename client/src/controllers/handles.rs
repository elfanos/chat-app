use super::StateEnum;
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub struct Messages(pub Vec<String>);

#[derive(Debug)]
pub struct Message(pub String);

pub struct Handles {
    pub messages: Messages,
    pub message: Message,
}
impl Handles {
    pub fn create() -> Handles {
        Handles {
            messages: Messages(vec![]),
            message: Message("".to_string()),
        }
    }
    pub fn update(&mut self, key: KeyEvent, app_state: Result<&StateEnum, &StateEnum>) {
        match app_state.unwrap() {
            StateEnum::Message => match key.code {
                KeyCode::Char(c) => {
                    self.message.0.push(c);
                }
                KeyCode::Backspace => {
                    self.message.0.pop();
                }
                _ => (),
            },
            StateEnum::NoUserName => match key.code {
                KeyCode::Char(c) => {
                    self.message.0.push(c);
                }
                KeyCode::Backspace => {
                    self.message.0.pop();
                }
                _ => (),
            },
            _ => {
                self.message.0.clear();
            }
        }
        // if let StateEnum::Message = app_state.unwrap() {
        //     match key.code {
        //         KeyCode::Char(c) => {
        //             self.message.0.push(c);
        //         }
        //         KeyCode::Backspace => {
        //             self.message.0.pop();
        //         }
        //         _ => (),
        //     }
        // } else {
        //     self.message.0.clear();
        // }
    }
}
