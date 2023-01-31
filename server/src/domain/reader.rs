use crate::{
    app_state::AppState,
    domain::{communication::broadcast, socket::Socket},
};
use std::{
    io::{BufReader, Read},
    net::TcpStream,
    sync::{Arc, Mutex},
};

use super::writer::Writer;

#[derive(Debug)]
pub struct Reader<T: Read> {
    reader: BufReader<T>,
    key: String,
}

impl<T: Read> Reader<T> {
    pub fn new(reader: BufReader<T>, key: String) -> Reader<T> {
        Reader { reader, key }
    }

    pub fn read(&mut self, context: Arc<Mutex<Socket<AppState>>>) {
        loop {
            let mut buffer = [0 as u8; 50];
            match self.reader.read(&mut buffer) {
                Ok(size) => {
                    match context.lock() {
                        Ok(mut c) => broadcast(&mut buffer, &mut c.map),
                        Err(err) => {
                            println!("what is wrong {:?}", err);
                        }
                    }
                    if size == 0 {
                        println!("Client  closed");
                        match context.lock() {
                            Ok(mut c) => {
                                c.map.remove_entry(&self.key).expect("remove enetry");
                            }
                            Err(err) => {
                                println!("what is wrong {:?}", err);
                            }
                        }
                        break;
                    }
                }
                Err(_) => {
                    println!("Something went wrong");
                }
            }
        }
    }
    pub fn get_id(&mut self) -> String {
        self.key.clone()
    }
}
