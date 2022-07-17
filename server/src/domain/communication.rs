use std::{collections::HashMap, net::TcpStream};

use super::writer::Writer;

pub fn broadcast(message: &mut [u8], context: &mut HashMap<String, Writer<TcpStream>>) {
    for (_, writer) in context.into_iter() {
        writer.write(message)
    }
}
