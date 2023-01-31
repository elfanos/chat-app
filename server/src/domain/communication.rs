use std::{collections::HashMap, net::TcpStream};

use crate::app_state::AppState;

pub fn broadcast(message: &mut [u8], context: &mut HashMap<String, AppState>) {
    for (_, writer) in context.into_iter() {
        writer.writer.write(message).unwrap();
    }
}
