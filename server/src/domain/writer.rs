use std::io::{BufWriter, Write};

#[derive(Debug)]

pub struct Writer<T: Write> {
    writer: BufWriter<T>,
}

impl<T: Write> Writer<T> {
    pub fn new(writer: BufWriter<T>) -> Writer<T> {
        Writer { writer }
    }

    pub fn write(&mut self, message: &mut [u8]) -> Result<(), ()> {
        match self.writer.write(message) {
            Ok(_) => {
                println!("could actually writer stuff writing to client");
            }
            Err(_) => {
                println!("Could not writer stuff");
            }
        }
        match self.writer.flush() {
            Ok(_) => {
                println!("Flushing buffer writer");
                Ok(())
            }
            Err(_) => {
                println!("Something went wrong flushing the stream");
                Err(())
            }
        }
    }
}
