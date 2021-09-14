use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
	data: HashMap<&'buf str, QueryStringValue<'buf>>
}

impl<'buf> QueryString<'buf> {
	pub fn get(&self, key: &str) -> Option<&QueryStringValue> {
		self.data.get(key)
	}
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
	fn from(s: &'buf str) -> Self {
		let mut data = HashMap::new();

		for sub_str in s.split('&') {
			let mut key = sub_str;
			let mut value = "";

			if let Some(i) = sub_str.find('=') {
				key = &sub_str[..i];
				value = &sub_str[i + 1..];
			}

			data.entry(key)
				.and_modify(|current| match current {
					QueryStringValue::Single(prev) => {
						*current = QueryStringValue::Multiple(vec![prev, value]);
					}
					QueryStringValue::Multiple(vec) => vec.push(value)
				})
				.or_insert(QueryStringValue::Single(value));
		}

		Self {data}
	}
}

#[derive(Debug)]
pub enum QueryStringValue<'buf> {
	Single(&'buf str),
	Multiple(Vec<&'buf str>)
}