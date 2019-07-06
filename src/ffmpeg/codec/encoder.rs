use crate::error::Error;
use crate::sys::*;
use crate::ffmpeg::format_context::*;
use crate::ffmpeg::utils::*;
use super::codec_context::*;
use super::codec::*;
use super::super::stream::*;
use std::ptr;

pub struct EncoderContext {
	ctx: CodecContext
}

impl EncoderContext {
	pub fn create(format_ctx: &FormatContext) -> Result<Self, Error> {
		let codec_ptr = unsafe { avcodec_find_encoder((*format_ctx.get_output_format()).audio_codec) };
		let codec = Codec::new(codec_ptr).ok_or(Error::from("Can't find fitting encoder."))?;

		let ptr = unsafe { avcodec_alloc_context3(codec.as_ptr()) };
		let ctx = CodecContext::new(ptr, codec).ok_or(Error::from("Can't allocate a codec context."))?;
		Ok(EncoderContext { ctx })
	}
}

wrap_ff_wrap!(EncoderContext, CodecContext, AVCodecContext, ctx, ctx_mut);


pub struct EncoderOpen {
	ctx: EncoderContext
}

wrap_ff_wrap!(EncoderOpen, EncoderContext, AVCodecContext, ctx, ctx_mut);

impl EncoderOpen {
	pub fn open(ctx: EncoderContext, stream: &mut Stream) -> Result<EncoderOpen, Error> {
		unsafe {
			ffm_op!(avcodec_open2(ctx.as_mut_ptr(), ctx.ctx().codec().as_ptr(), ptr::null_mut()))?;
			ffm_op!(avcodec_parameters_from_context(stream.parameters().as_mut_ptr(), ctx.as_mut_ptr()))?;
		}
		Ok(EncoderOpen { ctx })
	}
}
