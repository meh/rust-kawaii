use std::error::Error as E;
use std::num::ParseIntError;

#[derive(Clone, Debug)]
pub struct Error(pub String);

pub type Result<T> = ::std::result::Result<T, Error>;

impl From<String> for Error {
	fn from(value: String) -> Error {
		Error(value)
	}
}

impl<'a> From<&'a str> for Error {
	fn from(value: &'a str) -> Error {
		Error(value.into())
	}
}

impl From<ParseIntError> for Error {
	fn from(value: ParseIntError) -> Error {
		Error(value.description().into())
	}
}
