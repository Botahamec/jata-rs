
use std::fs::read_to_string;
use std::fs::write;
use std::io::Error;

use std::str::FromStr;
use std::string::ToString;

/**
 * This is a struct containing a path to a file.
 * It takes a generic type which defines what type the file contains.
 */
pub struct JataFile<T: ToString + FromStr> {

	/** A path to the file */
	pub path: String,

	/** What is currently believed to be the value */
	pub value: T
}

impl<T> JataFile<T> where T: ToString + FromStr {

	/**
	 * Checks and returns the current value of the file.
	 * Returns Ok if everything is ok.
	 * Returns Ok(Err) if there was a problem parsing the file.
	 * Returns Err if there was a problem reading the file
	 */
	pub fn check_value(&self) -> std::io::Result<Result<T, T::Err>> {
		match read_to_string(&self.path) {
			Ok(s) => Ok(T::from_str(&s)),
			Err(e) => Err(e)
		}
	}

	/**
	 * Checks the value of the file and sets the current value if applicable.
	 * Returns Ok if the check was successful.
	 * Returns Err if there was a problem with reading the file.
	 * Returns Ok(Err) if there was a problem parsing the file.
	 */
	pub fn reset_value(&mut self) -> std::io::Result<Result<(), T::Err>> {
		match self.check_value() {
			Ok(r) => match r {
					Ok(t) => {
						self.value = t;
						Ok(Ok(()))
					},
					Err(e) => Ok(Err(e))
				},
			Err(e) => Err(Error::from(e))
		}
	}

	/**
	 * Sets the value and writes the value to the file.
	 * Returns Ok if the write was successful
	 * Returns Err if there was an error in writing to the file
	 */
	pub fn set_value(&mut self, value: T) -> std::io::Result<()> {
		self.value = value;
		let string_rep = self.value.to_string();
		match write(self.path.clone(), string_rep) {
			Ok(_o) => Ok(()),
			Err(e) => Err(e)
		}
	}
}