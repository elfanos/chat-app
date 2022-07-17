use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub enum StateEnum {
    Message,
    Normal,
    Exit,
}
#[derive(Debug)]
pub struct State {
    state: StateEnum,
}
impl State {
    pub fn create() -> State {
        State {
            state: StateEnum::Normal,
        }
    }
    fn is_message(&mut self) -> StateEnum {
        self.state = StateEnum::Message;
        StateEnum::Message
    }
    fn normal(&mut self) -> StateEnum {
        self.state = StateEnum::Normal;
        StateEnum::Normal
    }
    fn is_exit(&mut self) -> StateEnum {
        self.state = StateEnum::Exit;
        StateEnum::Exit
    }
    pub fn current_state(&mut self) -> Result<&StateEnum, &StateEnum> {
        Ok(&self.state)
    }
    pub fn key_state(&mut self, key: KeyEvent) -> Result<StateEnum, StateEnum> {
        match self.state {
            StateEnum::Normal => match key.code {
                KeyCode::Char('e') => Ok(self.is_message()),
                KeyCode::Char('q') => Ok(self.is_exit()),
                _ => Ok(self.normal()),
            },
            StateEnum::Message => {
                if let KeyCode::Esc = key.code {
                    return Ok(self.normal());
                }
                if let KeyCode::Enter = key.code {
                    return Ok(self.normal());
                }
                Ok(self.is_message())
            }
            StateEnum::Exit => Ok(StateEnum::Exit),
            _ => Ok(self.normal()),
        }
    }
}
