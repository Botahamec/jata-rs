/* JATA.RS
 * A library to handle object databases which are not stored in memory
 * Instead, the structures are stored in physical storage, using txt files
 * This is most likely not the most efficient way to do this
 * However, I was unable to find any crates which did a better job
 */

use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;
use std::fs::File;
use std::io::prelude::*;

pub mod jata;

#[derive(PartialEq)]
enum JataType {
	Str,
	Int,
	Float,
	Bool,
	StrList,
	IntList,
	FloatList,
	BoolList
}

struct JataProp {
	name: String,
	location: String,
	jtype: JataType,
}

struct JataObject {
	location: String,
	properties: Vec<JataProp>
}

impl JataProp {

	fn new_raw(name: String, location: String, jtype: JataType, value: String) -> Result<JataProp> {
		let mut file = File::create(location.clone())?;
    	file.write_all(value.as_bytes())?;
    	Ok(JataProp{name: name, location: location.clone(), jtype: jtype})
	}

	fn new_str(name: String, location: String, value: String) -> Result<JataProp> {
		JataProp::new_raw(name, location, JataType::Str, value)
	}

	fn new_int(name: String, location: String, value: isize) -> Result<JataProp> {
		JataProp::new_raw(name, location, JataType::Int, format!("{}", value))
	}

	fn new_float(name: String, location: String, value: f32) -> Result<JataProp> {
		JataProp::new_raw(name, location, JataType::Float, format!("{}", value))
	}

	fn new_bool(name: String, location: String, value: bool) -> Result<JataProp> {
		JataProp::new_raw(name, location, JataType::Bool, match value {
			false => String::from("0"),
			true => String::from("1")
		})
	}

	fn new_str_list(name: String, location: String, value: Vec<String>) -> Result<JataProp> {
		JataProp::new_raw(name, location, JataType, value.join("\n"))
	}

	fn new_int_list(name: String, location: String, value: Vec<isize>) -> Result<JataProp> {
		JataProp::new_raw(name, location, JataType::IntList,
							value.iter()
							.map(|i| format!("{}", i))
							.join("\n"))
	}

	fn new_float_list(name: String, location: String, value: Vec<f32>) -> Result<JataProp> {
		JataProp::new_raw(name, location, JataType::FloatList,
							value.iter()
							.map(|f| format!("{}", f))
							.join("\n"))
	}

	fn new_bool_list(name: String, location: String, value: Vec<bool>) -> Result<JataProp> {
		JataProp::new_raw(name, location, JataType::BoolList,
							value.iter()
							.map(|b| match b {
								false => String::from("0"),
								true => String::from("1")
							})
							.join("\n"))
	}

	fn get_raw(&self) -> Result<String> {
		let mut file = File::open(self.location.clone())?;
    	let mut contents = String::new();
    	file.read_to_string(&mut contents)?;
    	Ok(contents)
	}

	fn get_raw_result(&self, jtype: JataType) -> Result<String> {
		if self.jtype == jtype {
			self.get_raw()
		}
		else {
			Err(Error::new(
				ErrorKind::InvalidData,
				format!("The data at {} is not of type: {}",
						self.location,
						stringify!(jtype))
			))
		}
	}

	fn get_str(&self) -> Result<String> {
		self.get_raw_result(JataType::Str)
	}

	fn get_int(&self) -> Result<isize> {
		match self.get_raw_result(JataType::Int) {
			Ok(s) => {
				match s.parse::<isize>() {
					Ok(i) => Ok(i),
					Err(e) => Err(Error::new(ErrorKind::InvalidData, e))
				}
			}
			Err(e) => Err(e)
		}
	}

	fn get_float(&self) -> Result<f32> {
		match self.get_raw_result(JataType::Int) {
			Ok(s) => {
				match s.parse::<f32>() {
					Ok(f) => Ok(f),
					Err(e) => Err(Error::new(ErrorKind::InvalidData, e))
				}
			}
			Err(e) => Err(e)
		}
	}

	fn get_bool(&self) -> Result<bool> {
		match self.get_raw_result(JataType::Bool) {
			Ok(s) => {
				match s.as_str() {
					"0" => Ok(false),
					"1" => Ok(true),
					_ => Err(Error::new(
						ErrorKind::InvalidData,
						format!("The file at {} did not contain 0 or 1",
								self.location)
					))
				}
			}
			Err(e) => Err(e)
		}
	}
}