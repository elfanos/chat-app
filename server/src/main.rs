use server::Server;

mod domain;
mod server;

fn main() {
    let ip = String::from("0.0.0.0:3333");
    let server = Server::new(ip);
    server.run();
}
