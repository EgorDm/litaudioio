use std::ptr;
use crate::sys::*;
use crate::error::Error;
use super::utils::*;
use super::stream::*;
use std::ffi::CString;

#[derive(Copy, Clone, Debug)]
pub enum Mode {
	Input,
	Output,
}

pub struct FormatContext {
	ptr: *mut AVFormatContext,
	mode: Mode
}

impl FormatContext {
	pub fn new(ptr: *mut AVFormatContext, mode: Mode) -> Self {
		FormatContext { ptr, mode }
	}

	pub fn get_audio_stream(&self) -> Option<Stream> {
		unsafe {
			let index = av_find_best_stream(self.ptr, AVMediaType::AVMEDIA_TYPE_AUDIO, -1, -1, ptr::null_mut(), 0);

			if index >= 0 {
				Some(Stream::new(*(self.as_ref().streams.offset(index as isize))).unwrap())
			} else {
				None
			}
		}
	}

	pub fn get_duration(&self) -> usize {
		self.as_ref().duration as usize
	}

	pub fn get_output_format(&self) -> *mut AVOutputFormat {
		self.as_ref().oformat
	}

	pub fn get_flags(&self) -> i32 {
		unsafe { (*self.as_ref().oformat).flags }
	}

	pub fn dump(&self, url: &str) {
		unsafe {
			let cpath = CString::new(url).unwrap();
			av_dump_format(self.as_mut_ptr(), 0, cpath.as_ptr(), 1);
		}
	}
}

impl FFWrapper<AVFormatContext> for FormatContext {
	fn as_ptr(&self) -> *const AVFormatContext { self.ptr }

	fn as_mut_ptr(&self) -> *mut AVFormatContext { self.ptr }
}

impl Drop for FormatContext {
	fn drop(&mut self) {
		unsafe {
			match self.mode {
				Mode::Input => avformat_close_input(&mut self.ptr),

				Mode::Output => {
					if !self.as_ref().pb.is_null() {
						avio_close((*self.ptr).pb);
					}
					avformat_free_context(self.ptr);
				}
			}
		}
	}
}

pub fn open_read(path: &str) -> Result<FormatContext, Error> {
	let cpath = CString::new(path).unwrap();

	let mut format_context = FormatContext::new(ptr::null_mut(), Mode::Input);

	unsafe {
		ffm_op!(avformat_open_input(&mut format_context.ptr, cpath.as_ptr(), ptr::null_mut(), ptr::null_mut()))?;
		ffm_op!(avformat_find_stream_info(format_context.as_mut_ptr(), ptr::null_mut()))?;
	}
	Ok(format_context)
}

pub fn open_write(path: &str) -> Result<FormatContext, Error> {
	let cpath = CString::new(path).unwrap();

	unsafe {
		let mut output_context: *mut AVIOContext = ptr::null_mut();

		ffm_op!(avio_open(&mut output_context, cpath.as_ptr(), AVIO_FLAG_READ_WRITE))?;
		let ptr = avformat_alloc_context();
		(*ptr).pb = output_context;

		(*ptr).oformat = av_guess_format(ptr::null_mut(), cpath.as_ptr(), ptr::null_mut());
		if (*ptr).oformat.is_null() {
			return Err(Error::from("Could not find output file format"));
		}

		(*ptr).url = av_strdup(cpath.as_ptr());
		if (*ptr).url.is_null() {
			return Err(Error::from("Could not allocate url."));
		}

		Ok(FormatContext::new(ptr, Mode::Output))
	}
}
