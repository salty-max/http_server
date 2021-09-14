use ansi_term::Colour;
use std::net::TcpListener;
use std::io::{Read, Write};
use std::convert::{TryFrom, TryInto};
use crate::http::{
	request::{Request, ParseError},
	response::Response,
	status_code::StatusCode
};

pub trait Handler {
	fn handle_request(&mut self, request: &Request) -> Response;
	
	fn handle_bad_request(&mut self, e: &ParseError) -> Response {
		println!("Failed to parse request: {}", Colour::Red.paint(e.to_string()));
		Response::new(StatusCode::BadRequest, None)
	}
}

pub struct Server {
	address: String,
}

impl Server {
	pub fn new(address: String) -> Self {
		Self { address }
	}

	pub fn run(self, mut handler: impl Handler) {
		println!("Listening on {}", Colour::Blue.bold().paint(&self.address));

		let listener = TcpListener::bind(&self.address).unwrap();

		loop {
			match listener.accept() {
				Ok((mut stream, _)) => {
					let mut buffer = [0; 1024];
					match stream.read(&mut buffer) {
						Ok(_) => {
							println!("Received a request : {}", Colour::Green.paint(String::from_utf8_lossy(&buffer)));
							let response = match Request::try_from(&buffer[..]) {
								Ok(request) => {
									handler.handle_request(&request)
								}
								Err(e) => {
									handler.handle_bad_request(&e)
								}
							};

							if let Err(e) = response.send(&mut stream) {
								println!("Failed to send response: {}", Colour::Red.paint(e.to_string()));
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