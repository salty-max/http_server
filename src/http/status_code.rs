use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
	Ok = 200,
	BadRequest = 400,
	Unauthorized = 401,
	Forbidden = 403,
	NotFound = 404,
	InternalServerError = 500,
	BadGateway = 502
}

impl StatusCode {
	pub fn reason_phrase(&self) -> &str {
		match self {
			Self::Ok => "Ok",
			Self::BadRequest => "Bad Request",
			Self::Unauthorized => "Unauthorized",
			Self::Forbidden => "Forbidden",
			Self::NotFound => "Not Found",
			Self::InternalServerError => "Internal Server Error",
			Self::BadGateway => "Bad Gateway",
		}
	}
}

impl Display for StatusCode {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "{}", *self as u16)
	}
}