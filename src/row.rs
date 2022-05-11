pub struct Row {
	string: String,
}

impl From<&str> for Row {
	fn from(slice: &str) -> Self {
		Self {
			string: slice.to_string(),
		}
	}
}