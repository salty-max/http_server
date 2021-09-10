pub struct Request {
	path: String,
	query_string: Option<String>,
	method: Method,
}

impl Request {
	pub fn new(path: String, query_string: Option<String>, method: Method) -> Self {
		Self { path, query_string, method }
	}
}