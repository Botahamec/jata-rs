
<<<<<<< HEAD
use std::fs::read_to_string;
=======
#![allow(dead_code)]

use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;
use std::fs::File;
use std::io::prelude::*;
>>>>>>> e782e0e24d2329b39718671f76d7ca062cf16166

pub trait JataType where Self: std::marker::Sized {
	fn read(representation: String) -> Option<Self>;
	fn to_str(self) -> String;
}

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
<<<<<<< HEAD
=======

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
						format!("The file at {} did not contain 0 or 1, but instead contained {}",
								self.location,
								s)
					))
				}
			}
			Err(e) => Err(e)
		}
	}

	fn get_str_list(&self) -> Result<Vec<String>> {
		let vec = self.get_raw_result(JataType::StrList)?;
		Ok(vec.split_terminator("\n")
			.map(|s| String::from(s))
			.collect::<Vec<String>>())
	}

	fn get_int_list(&self) -> Result<Vec<isize>> {
		let string = self.get_raw_result(JataType::IntList)?;
		let vec = string.split_terminator("\n");
		let mut ints : Vec<isize> = Vec::new();
		for result in vec {
			match result.parse::<isize>() {
				Ok(i) => ints.push(i),
				Err(e) => return Err(Error::new(ErrorKind::InvalidData, e))
			}
		}
		Ok(ints)
	}

	fn get_float_list(&self) -> Result<Vec<f32>> {
		let string = self.get_raw_result(JataType::IntList)?;
		let vec = string.split_terminator("\n");
		let mut floats : Vec<f32> = Vec::new();
		for result in vec {
			match result.parse::<f32>() {
				Ok(f) => floats.push(f),
				Err(e) => return Err(Error::new(ErrorKind::InvalidData, e))
			}
		}
		Ok(floats)
	}

	fn get_bool_list(&self) -> Result<Vec<bool>> {
		let string = self.get_raw_result(JataType::BoolList)?;
		let vec = string.split_terminator("\n");
		let mut bools : Vec<bool> = Vec::new();
		for result in vec {
			match result {
				"0" => bools.push(false),
				"1" => bools.push(true),
				_ => return Err(Error::new(ErrorKind::InvalidData,
								format!("The data at {} did not contain a 0 or 1, but instead contained {}",
								self.location,
								result)
				))
			}
		}
		Ok(bools)
	}
>>>>>>> e782e0e24d2329b39718671f76d7ca062cf16166
}