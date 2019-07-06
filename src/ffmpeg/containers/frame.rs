use std::mem;
use std::ptr;
use crate::sys::*;
use crate::error::Error;
use super::super::codec::*;
use super::super::format::*;
use super::super::utils::*;

ff_wrap_struct!(Frame, AVFrame);
ff_wrap!(Frame, AVFrame);

impl Frame {
	pub fn empty() -> Option<Self> {
		Frame::new(unsafe {av_frame_alloc()})
	}

	pub fn send(&mut self, ctx: &mut EncoderOpen) -> Result<i32, Error> {
		unsafe {
			ffm_ret!(avcodec_send_frame(ctx.as_mut_ptr(), self.as_ptr()))
		}
	}

	pub fn send_flush(ctx: &mut EncoderOpen) -> Result<i32, Error> {
		unsafe {
			ffm_ret!(avcodec_send_frame(ctx.as_mut_ptr(), ptr::null_mut()))
		}
	}

	pub fn recieve(&mut self, ctx: &DecoderOpen) -> Result<(), Error> {
		unsafe {
			ffm_op!(avcodec_receive_frame(ctx.as_mut_ptr(), self.ptr))
		}
	}

	pub fn nb_samples(&self) -> i32 { self.as_ref().nb_samples }

	pub fn set_nb_samples(&mut self, nb_samples: i32) {
		self.as_mut_ref().nb_samples = nb_samples;
	}

	pub fn set_channel_layout(&mut self, channel_layout: ChannelLayout) {
		unsafe { (*self.ptr).channel_layout = channel_layout.bits(); }
	}

	pub fn set_sample_rate(&mut self, sample_rate: i32) {
		self.as_mut_ref().sample_rate = sample_rate;
	}

	pub fn set_sample_format(&mut self, sample_format: SampleFormat) {
		let sf: AVSampleFormat = sample_format.into();
		self.as_mut_ref().format = unsafe {mem::transmute::<AVSampleFormat, i32>(sf)};
	}

	pub fn data_ptr(&self, i: usize) -> *const u8 { self.as_ref().data[i] }

	pub fn data_mut_ptr(&mut self, i: usize) -> *mut u8 { self.as_mut_ref().data[i] }
}

impl Drop for Frame {
	fn drop(&mut self) {
		unsafe {
			av_frame_free(&mut self.as_mut_ptr());
		}
	}
}