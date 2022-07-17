use std::io::{BufReader, BufWriter};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::app_state::AppState;
use crate::controllers::{Handles, Sender, State};
use crate::domain::Reader;
use crate::domain::Writer;
use crate::ui::initialize;

#[derive(Debug)]
pub struct Client {
    ip: String,
}
pub enum ClientStatus {
    Ok,
    Error,
}
impl Client {
    pub(self) fn run_client_stream(stream: TcpStream, messages: Arc<Mutex<Vec<[u8; 50]>>>) {
        let reader = BufReader::new(stream.try_clone().expect("Reader clone..."));
        let mut client_reader = Reader::new(reader);
        let messages_mutext_clone = Arc::clone(&messages);
        thread::spawn(move || {
            client_reader.read(messages_mutext_clone);
        });
        let mut app_state = AppState {
            logs: Vec::new(),
            rooms: Vec::new(),
            users: Vec::new(),
            handles: Handles::create(),
            state: State::create(),
            sender: Sender::create(Writer::new(BufWriter::new(
                stream.try_clone().expect("Writer clone..."),
            ))),
            messages,
        };
        let result = initialize(&mut app_state);
        match result {
            Ok(_) => {}
            Err(error) => println!("some error {:?}", error),
        }
    }
    pub fn new(ip: String) -> Client {
        Client { ip }
    }
    pub fn load(self) -> Result<ClientStatus, ClientStatus> {
        let buffer = [0 as u8; 50];
        let message_from_server = Arc::new(Mutex::new(vec![buffer]));
        let result = match TcpStream::connect(self.ip) {
            Ok(stream) => {
                Client::run_client_stream(stream, message_from_server);
                Ok(ClientStatus::Ok)
            }
            Err(_) => Err(ClientStatus::Error),
        };
        result
    }
}
