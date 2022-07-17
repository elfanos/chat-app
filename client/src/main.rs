use client::Client;
mod app_state;
mod client;
mod controllers;
mod domain;
mod ui;

fn main() {
    let client = Client::new(String::from("localhost:3333"));
    let result = client.load();
    match result {
        Ok(_) => {
            println!("Ended program");
        }
        Err(_) => {
            println!("something went wrong")
        }
    }
}
