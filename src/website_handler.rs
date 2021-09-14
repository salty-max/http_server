use crate::server::Handler;
use crate::http::request::{Request};
use crate::http::response::Response;
use crate::http::status_code::StatusCode;

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
	fn handle_request(&mut self, request: &Request) -> Response {
		Response::new(StatusCode::Ok, Some(String::from("<h1>It works!</h1>")))
	}
}