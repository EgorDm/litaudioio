use sys::*;
use crate::ffmpeg::utils::*;
use crate::ffmpeg::format::*;
use super::codec::*;

pub struct CodecContext {
	ptr: *mut AVCodecContext,
	codec: Codec,
}

ff_wrap!(CodecContext, AVCodecContext);

impl CodecContext {
	pub fn new(ptr: *mut AVCodecContext, codec: Codec) -> Option<CodecContext> {
		if ptr.is_null() {
			None
		} else {
			Some(Self { ptr, codec })
		}
	}

	pub fn codec(&self) -> &Codec {
		&self.codec
	}

	pub fn channel_layout(&self) -> ChannelLayout {
		ChannelLayout::from_bits_truncate(self.as_ref().channel_layout)
	}

	pub fn set_channel_layout(&mut self, channel_layout: ChannelLayout) {
		self.as_mut_ref().channels = channel_layout.channels();
		self.as_mut_ref().channel_layout = channel_layout.bits();
	}

	pub fn sample_rate(&self) -> i32 {
		self.as_ref().sample_rate
	}

	pub fn set_sample_rate(&mut self, sample_rate: i32) {
		self.as_mut_ref().sample_rate = sample_rate;
	}

	pub fn sample_format(&self) -> SampleFormat {
		SampleFormat::from(self.as_ref().sample_fmt)
	}

	pub fn set_sample_format(&mut self, sample_format: SampleFormat) {
		self.as_mut_ref().sample_fmt = sample_format.into();
	}

	pub fn frame_size(&self) -> i32 { self.as_ref().frame_size }
}

impl Drop for CodecContext {
	fn drop(&mut self) { unsafe { avcodec_free_context(&mut self.as_mut_ptr()); } }
}