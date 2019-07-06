use sys::*;
use super::codec::codec_params::*;
use super::utils::*;
use super::format_context::*;
use crate::error::Error;
use std::ptr;

ff_wrap_struct!(Stream, AVStream);
ff_wrap!(Stream, AVStream);

impl Stream {
	pub fn from_format(fmt: &FormatContext) -> Result<Self, Error> {
		let ptr = unsafe { avformat_new_stream(fmt.as_mut_ptr(), ptr::null_mut()) };
		Stream::new(ptr).ok_or(Error::from("Could not create new stream."))
	}

	pub fn parameters(&self) -> CodecParameters {
		CodecParameters::new(self.as_ref().codecpar).unwrap()
	}

	pub fn id(&self) -> i32 { self.as_ref().id }
}