use std::net::TcpStream;

use crate::{domain::writer::Writer, modal::User};

pub struct AppState {
    pub user: User,
    pub writer: Writer<TcpStream>,
}

impl AppState {
    pub fn create(user: User, writer: Writer<TcpStream>) -> AppState {
        AppState { user, writer }
    }
}
