
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


#[derive(Default)]
/**
 * This is a struct containing a path to a file.
 * It takes a generic type which defines what type the file contains.
 */
pub struct JataFile<T: JataType + Default> {

	/** A path to the file */
	path: String,

	/** What is currently believed to be the value */
	value: T
}

impl<T> JataFile<T> where T: JataType + Default {

	/**
	 * The default constructor which doesn't initiate any fields
	 */
	pub fn new(self) -> Self {
		Self::default()
	}

	/**
	 * Checks and returns the current value of the file
	 */
	pub fn check_value(self) -> Option<T> {
		match read_to_string(self.path) {
			Ok(s) => T::read(s),
			Err(_e) => None
		}
	}
}