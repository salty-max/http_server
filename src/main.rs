#![allow(dead_code)]

use server::Server;
use crate::website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {
    let server = Server::new(String::from("127.0.0.1:1234"));
    server.run(WebsiteHandler);
}
