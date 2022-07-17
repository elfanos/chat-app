use crate::controllers::{Handles, Sender, State};
use std::sync::{Arc, Mutex};
pub struct AppState {
    pub logs: Vec<String>,
    pub rooms: Vec<String>,
    pub users: Vec<String>,
    pub sender: Sender,
    pub handles: Handles,
    pub state: State,
    pub messages: Arc<Mutex<Vec<[u8; 50]>>>,
}
