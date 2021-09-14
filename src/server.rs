use ansi_term::Colour;
use std::net::TcpListener;
use std::io::Read;
use std::convert::{TryFrom, TryInto};
use crate::http::Request;

pub struct Server {
	address: String,
}

impl Server {
	pub fn new(address: String) -> Self {
		Self { address }
	}

	pub fn run(self) {
		println!("Listening on {}", Colour::Blue.bold().paint(&self.address));

		let listener = TcpListener::bind(&self.address).unwrap();

		loop {
			match listener.accept() {
				Ok((mut stream, _)) => {
					let mut buffer = [0; 1024];
					match stream.read(&mut buffer) {
						Ok(_) => {
							println!("Received a request : {}", Colour::Green.paint(String::from_utf8_lossy(&buffer)));
							match Request::try_from(&buffer[..]) {
								Ok(request) => {
									dbg!(request);
								}
								Err(e) => println!("Failed to parse request: {}", Colour::Red.paint(e.to_string()))
							}
							let res: &Result<Request, _> = &buffer[..].try_into();
						}
						Err(e) => println!("Failed to read from connection: {}", Colour::Red.paint(e.to_string()))
					}
				}
				Err(e) => println!("Failed to establish a connection: {}", Colour::Red.paint(e.to_string()))
			}
		}
	}
}