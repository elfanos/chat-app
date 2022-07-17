use std::io::{BufWriter, Write};

pub struct Writer<T: Write> {
    writer: BufWriter<T>,
}

impl<T: Write> Writer<T> {
    pub fn new(writer: BufWriter<T>) -> Writer<T> {
        Writer { writer }
    }
    pub fn write(&mut self, input: &[u8]) {
        match self.writer.write(input) {
            Ok(_) => {}
            Err(e) => {
                println!("Something happen with the stream? {}", e);
            }
        }
        match self.writer.flush() {
            Ok(_) => {}
            Err(_) => {
                println!("Something went wrong flushing the stream");
            }
        }
    }
}
