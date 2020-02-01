
use std::fs::read_to_string;

pub trait JataType where Self: std::marker::Sized {
	fn read(representation: String) -> Option<Self>;
	fn to_str(self) -> String;
}

#[Clone, Default, PartialEq]
pub struct JataFile<T> where T: JataType {
	path: String,
	value: T
}

impl<T> JataFile<T> where T: JataType {
	pub fn check_value(self) -> Option<T> {
		match read_to_string(self.path) {
			Ok(s) => T::read(s),
			Err(_e) => None
		}
	}
}