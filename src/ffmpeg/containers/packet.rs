use std::mem;
use crate::sys::*;
use crate::error::Error;
use super::super::format_context::FormatContext;
use super::super::codec::*;
use super::super::utils::*;

pub struct Packet(AVPacket);

impl Packet {
	#[inline]
	pub fn empty() -> Self {
		unsafe {
			let mut pkt: AVPacket = mem::zeroed();
			av_init_packet(&mut pkt);
			Packet(pkt)
		}
	}

	fn as_mut_ptr(&mut self) -> *mut AVPacket { &mut self.0 }

	pub fn read(&mut self, fmt: &FormatContext) -> Result<(), Error> {
		unsafe { ffm_op!(av_read_frame(fmt.as_mut_ptr(), self.as_mut_ptr())) }
	}

	pub fn write(&mut self, fmt: &FormatContext) -> Result<(), Error> {
		unsafe { ffm_op!(av_write_frame(fmt.as_mut_ptr(), self.as_mut_ptr())) }
	}

	pub fn send(&mut self, ctx: &DecoderOpen) -> Result<(), Error> {
		unsafe { ffm_op!(avcodec_send_packet(ctx.as_mut_ptr(), self.as_mut_ptr())) }
	}

	pub fn recieve(&mut self, ctx: &mut EncoderOpen) -> Result<(), Error> {
		unsafe { ffm_op!(avcodec_receive_packet(ctx.as_mut_ptr(), self.as_mut_ptr())) }
	}

	pub fn stream_id(&self) -> i32 {
		self.0.stream_index
	}

	pub fn reset(&mut self) {
		unsafe { av_packet_unref(&mut self.0); }
	}
}

impl Drop for Packet {
	fn drop(&mut self) {
		unsafe {
			av_packet_unref(&mut self.0);
		}
	}
}
