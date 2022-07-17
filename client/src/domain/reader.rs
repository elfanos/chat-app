use std::{
    borrow::Cow,
    io::{BufReader, Read},
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub struct Reader<T: Read> {
    reader: BufReader<T>,
}

impl<T: Read> Reader<T> {
    pub fn new(reader: BufReader<T>) -> Reader<T> {
        Reader { reader }
    }

    pub fn read(&mut self, message: Arc<Mutex<Vec<[u8; 50]>>>) {
        loop {
            let mut buffer = [0 as u8; 50];
            match self.reader.read(&mut buffer) {
                Ok(size) => {
                    match message.lock() {
                        Ok(mut c) => c.push(buffer),
                        Err(err) => {
                            println!("what is wrong {:?}", err);
                        }
                    }
                    if size == 0 {
                        println!("Server closed");
                        break;
                    }
                }
                Err(_) => {
                    println!("It is not okay");
                }
            }
        }
    }
}
