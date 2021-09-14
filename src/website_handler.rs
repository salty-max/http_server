use crate::server::Handler;
use crate::http::{
	request::{Request},
	response::Response,
	status_code::StatusCode,
	method::Method
};

pub struct WebsiteHandler {
	public_path: String
}

impl WebsiteHandler {
	pub fn new(public_path: String) -> Self {
		Self { public_path }
	}
}

impl Handler for WebsiteHandler {
	fn handle_request(&mut self, request: &Request) -> Response {
		match request.method() {
			Method::GET => match request.path() {
				"/" => Response::new(StatusCode::Ok, Some(String::from("<h1>It works!</h1>"))),
				"/hello" => Response::new(StatusCode::Ok, Some(String::from("<h1>Hello World!</h1>"))),
				_ => Response::new(StatusCode::NotFound, None),
			}
			_ => Response::new(StatusCode::NotFound, None),
		}
	}
}