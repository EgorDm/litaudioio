use crate::error::Error;
use std::{ptr, mem};
use libc::{c_int};
use crate::sys::*;
use super::format::*;
use super::frame::*;
use super::utils::*;
use litcontainers::*;
use litaudio::*;

pub struct Converter {
	ptr: *mut SwrContext
}

impl Converter {
	pub fn new(src_fmt: AudioFormat, dst_fmt: AudioFormat) -> Result<Self, Error> {
		unsafe {
			let ptr = swr_alloc_set_opts(
				ptr::null_mut(),
				dst_fmt.channel_layout.bits() as i64, dst_fmt.sample_format.into(), dst_fmt.sample_rate as c_int,
				src_fmt.channel_layout.bits() as i64, src_fmt.sample_format.into(), src_fmt.sample_rate as c_int,
				0, ptr::null_mut()
			);
			if ptr.is_null() {
				return Err(Error::from("Couldn't create the SwrContext"));
			}

			ffm_op!(swr_init(ptr))?;
			Ok(Converter { ptr })
		}
	}

	pub fn get_sample_count(&self, input_sample_count: i32) -> i32 {
		unsafe { swr_get_out_samples(self.ptr, input_sample_count) }
	}

	pub fn convert_frame<'a, T, C, CS, L, LS, P>(&mut self, input: &mut Frame, output: &mut AudioSliceMut<'a, T, C, CS, L, LS, P>) -> Result<i32, Error>
		where T: Sample, C: Dim, CS: Dim, L: Dim, LS: Dim, P: SamplePackingType
	{
		unsafe {
			self.convert(
				mem::transmute((*input.as_ptr()).data.as_ptr()), input.nb_samples(),
				mem::transmute(output.as_mut_ptr()), output.sample_count() as i32
			)
		}
	}

	pub fn convert_slice<'a, T, C, CS, L, LS, P>(&mut self, input: &AudioSlice<'a, T, C, CS, L, LS, P>, output: &mut Frame) -> Result<i32, Error>
		where T: Sample, C: Dim, CS: Dim, L: Dim, LS: Dim, P: SamplePackingType
	{
		unsafe {
			self.convert(
				mem::transmute(input.as_ptr()), input.sample_count() as i32,
				mem::transmute((*output.as_mut_ptr()).data.as_ptr()), output.nb_samples(),
			)
		}
	}

	pub fn convert(
		&mut self,
		input: *const *const u8,
		input_size: i32,
		output: *const *mut u8,
		output_size: i32
	) -> Result<i32, Error> {
		unsafe {
			match swr_convert(
				self.ptr,
				mem::transmute(output), output_size,
				mem::transmute(input), input_size
			) {
				e if e < 0 => Err(Error::from_ff(e)),
				r => Ok(r)
			}
		}
	}
}

impl Drop for Converter {
	fn drop(&mut self) {
		unsafe { swr_free(&mut self.ptr) }
	}
}
