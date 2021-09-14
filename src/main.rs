#![allow(dead_code)]

use server::Server;

mod server;
mod http;

fn main() {
    let server = Server::new(String::from("127.0.0.1:1234"));
    server.run();
}
