use crate::error::Error;
use crate::sys::*;
use super::codec::*;
use super::codec_context::*;
use super::codec_params::*;
use super::super::utils::*;
use super::super::format::*;
use std::ptr;

pub struct DecoderContext {
	ctx: CodecContext
}

wrap_ff_wrap!(DecoderContext, CodecContext, AVCodecContext, ctx, ctx_mut);

impl DecoderContext {
	pub fn create(params: &CodecParameters) -> Result<Self, Error> {
		let codec_ptr = unsafe { avcodec_find_decoder(params.as_ref().codec_id) };
		let codec = Codec::new(codec_ptr).ok_or("Can't find fitting decoder.")?;

		let ptr = unsafe { avcodec_alloc_context3(codec.as_ptr()) };
		unsafe { ffm_op!(avcodec_parameters_to_context(ptr, params.as_ptr()))? };
		let ctx = CodecContext::new(ptr, codec).ok_or("Can't allocate a codec context.")?;
		Ok(DecoderContext { ctx })
	}

	pub fn set_request_sample_fmt(&mut self, sample_format: SampleFormat) {
		self.as_mut_ref().request_sample_fmt = sample_format.into();
	}

	pub fn sample_fmt(&self) -> SampleFormat { SampleFormat::from(self.as_ref().request_sample_fmt) }
}


pub struct DecoderOpen {
	ctx: DecoderContext
}

wrap_ff_wrap!(DecoderOpen, DecoderContext, AVCodecContext, ctx, ctx_mut);

impl DecoderOpen {
	pub fn open(ctx: DecoderContext) -> Result<DecoderOpen, Error> {
		unsafe {
			ffm_op!(avcodec_open2(ctx.as_mut_ptr(), ctx.ctx().codec().as_ptr(), ptr::null_mut()))?;
		}
		Ok(DecoderOpen { ctx })
	}
}

impl Drop for DecoderOpen {
	fn drop(&mut self) {
		unsafe {
			avcodec_close(self.ctx.as_mut_ptr());
		}
	}
}