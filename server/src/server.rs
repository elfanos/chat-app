use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::io::{BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::app_state::AppState;
use crate::domain::reader::Reader;
use crate::domain::socket::Socket;
use crate::domain::writer::Writer;
use crate::modal::User;

fn generate_socket_id() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

#[derive(Debug)]
pub struct Server {
    ip: String,
}

impl Server {
    pub fn new(ip: String) -> Self {
        Self { ip }
    }
    pub(self) fn create_client_connection(
        stream: TcpStream,
        socket: &mut Arc<Mutex<Socket<AppState>>>,
    ) {
        println!("New connection: {}", stream.peer_addr().unwrap());
        let mut reader = Reader::new(
            BufReader::new(stream.try_clone().expect("Creating listener from client")),
            generate_socket_id(),
        );
        let data_mutex_clone = Arc::clone(&socket);

        thread::spawn(move || {
            data_mutex_clone.lock().unwrap().add_connection(
                AppState::create(
                    User::create("user".to_string()),
                    Writer::new(BufWriter::new(
                        stream.try_clone().expect("Creating a writer to client"),
                    )),
                ),
                reader.get_id().clone(),
            );
            reader.read(data_mutex_clone)
        });
    }
    pub fn run(&self) {
        let listener = TcpListener::bind(&self.ip).unwrap();

        let socket = Socket::new(HashMap::<String, AppState>::new());

        let mut data = Arc::new(Mutex::new(socket));

        println!("RUNNING SERVER ON {}", &self.ip);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    Server::create_client_connection(stream, &mut data);
                }
                Err(error) => println!("Failed to read from connection: {}", error),
            }
        }
        drop(listener);
    }
}
