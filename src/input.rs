fn normalize(s: &str) -> String {
	s.to_lowercase()
		.replace(|c: char| !c.is_alphanumeric() && !c.is_whitespace(), " ")
		.split_whitespace() //	avoid usage tabs, new lines etc
		.collect::<Vec<_>>()
		.join(" ")
		.trim()
		.to_string()
}

#[derive(Default, Debug)]
pub struct Input {
	pub text: String,
	pub empedding: Option<Vec<f32>>,
}

impl Input {
	pub fn new(text: &str) -> Self {
		Self {
			text: normalize(text),
			empedding: None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_normalize() {
		let input = "Hello, World! This is a Test.";
		let expected = "hello world this is a test";
		assert_eq!(normalize(input), expected);
	}
}
