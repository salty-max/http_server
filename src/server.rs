use ansi_term::Colour;
use std::net::TcpListener;

pub struct Server {
	address: String,
}

impl Server {
	pub fn new(address: String) -> Self {
		Self { address }
	}

	pub fn run(self) {
		println!("Listening on {}", Colour::Blue.bold().paint(self.address));

		let listener = TcpListener::bind(&self.address);
	}
}