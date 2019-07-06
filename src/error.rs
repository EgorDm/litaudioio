use std::fmt;
use std::fmt::Formatter;
use libc::{c_int};
use crate::ffmpeg;
use std::error;
use crate::ffmpeg::FFError;

#[derive(Debug)]
pub enum Error {
	FFM(ffmpeg::utils::FFError),
	CustomError(String)
}

impl Error {
	pub fn from_ff(e: c_int) -> Self {
		Error::FFM(ffmpeg::utils::FFError::from(e))
	}
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		f.write_str(error::Error::description(self))
	}
}

impl std::error::Error for Error {
	fn description(&self) -> &str {
		match self {
			Error::FFM(e) => e.description(),
			Error::CustomError(e) => &e
		}
	}
}

impl From<FFError> for Error {
	fn from(e: FFError) -> Self {
		Error::FFM(e)
	}
}

impl From<&str> for Error {
	fn from(e: &str) -> Self {
		Error::CustomError(e.to_string())
	}
}

impl From<String> for Error {
	fn from(e: String) -> Self {
		Error::CustomError(e)
	}
}



macro_rules! ffm_op (
	($e: expr) => {
		match $e {
			0 => Ok(()),
			e => Err(Error::from_ff(e))
		}
	}
);

macro_rules! ffm_ret (
	($e: expr) => {
		match $e {
			r if r >= 0 => Ok(r),
			e => Err(Error::from_ff(e))
		}
	}
);