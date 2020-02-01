
use std::fs::read_to_string;

/**
 * This allows a type to be used with Jata.
 * This makes sure that a file can be translated to a string.
 * It also needs to be able to parse a String into itself
 */
pub trait JataType where Self: std::marker::Sized {

	/**
	 * This takes a string and converts it to the type.
	 * Returns None if the String is not a correct representation of the type.
	 */
	fn read(representation: String) -> Option<Self>;

	/**
	 * Converts the type to a String.
	*/
	fn to_str(self) -> String;
}


#[derive(Clone, Default)]
/**
 * This is a struct containing a path to a file.
 * It takes a generic type which defines what type the file contains.
 */
pub struct JataFile<T: JataType + Default + Clone> {

	/** A path to the file */
	path: String,

	/** What is currently believed to be the value */
	value: T
}

impl<T> JataFile<T> where T: JataType + Default + Clone {

	/**
	 * The default constructor which doesn't initiate any fields
	 */
	pub fn new() -> Self {
		Self::default()
	}

	/**
	 * Checks and returns the current value of the file.
	 * Returns None if there is an error when reading the file,
	 * or if the file isn't a valid representation of the type.
	 */
	pub fn check_value(self) -> Option<T> {
		match read_to_string(self.path) {
			Ok(s) => T::read(s),
			Err(_e) => None
		}
	}

	/**
	 * Checks the value of the file and sets the current value if applicable.
	 * Returns the value if applicable.
	 */
	pub fn reset_value(&mut self) -> Option<T> {
		match read_to_string(self.path.clone()) {
			Ok(s) => match T::read(s) {
				Some(t) => {self.value = t.clone(); return Some(t)},
				None => None
			},
			Err(_e) => None
		}
	}
}