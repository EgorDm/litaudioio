use std::{mem};
use crate::sys::*;
use crate::ffmpeg::*;
use crate::error::Error;

pub struct Output {
	format_ctx: FormatContext,
	stream: Stream,
	codec_ctx: EncoderOpen,
}

impl Output {
	pub fn open<F>(path: &str, format_picker: F, channel_layout: ChannelLayout, sample_rate: i32)
		-> Result<Self, Error>
		where F: Fn(FormatIter) -> Option<SampleFormat>
	{
		let format_ctx = open_write(&path)?;
		let mut codec_ctx = EncoderContext::create(&format_ctx)?;

		let format_iter = codec_ctx.ctx().codec().formats()
			.ok_or("Could not find appropriate sample format")?;
		let sample_format = format_picker(format_iter)
			.ok_or("Could not find appropriate sample format")?;

		codec_ctx.ctx_mut().set_channel_layout(channel_layout);
		codec_ctx.ctx_mut().set_sample_format(sample_format);
		codec_ctx.ctx_mut().set_sample_rate(sample_rate);

		let mut stream = Stream::from_format(&format_ctx)?;
		stream.as_mut_ref().time_base.den = sample_rate;
		stream.as_mut_ref().time_base.num = 1;

		if (format_ctx.get_flags() & AVFMT_GLOBALHEADER) != 0 {
			codec_ctx.as_mut_ref().flags |= unsafe { mem::transmute::<u32, i32>(AV_CODEC_FLAG_GLOBAL_HEADER) };
		}

		let codec_ctx = EncoderOpen::open(codec_ctx, &mut stream)?;

		Ok(Output { format_ctx, stream, codec_ctx })
	}

	pub fn format_ctx(&self) -> &FormatContext { &self.format_ctx }

	pub fn stream(&self) -> &Stream { &self.stream }

	pub fn codec_ctx(&mut self) -> &mut EncoderOpen { &mut self.codec_ctx }

	pub fn channel_layout(&self) -> ChannelLayout { self.codec_ctx.ctx().ctx().channel_layout() }

	pub fn sample_format(&self) -> SampleFormat { self.codec_ctx.ctx().ctx().sample_format() }

	pub fn sample_rate(&self) -> i32 { self.codec_ctx.ctx().ctx().sample_rate() }

	pub fn frame_size(&self) -> usize {
		(match self.codec_ctx.ctx().ctx().frame_size() {
			0 => AV_CODEC_CAP_VARIABLE_FRAME_SIZE as i32,
			v => v
		} as usize)
	}

	pub fn converter(&self, src_fmt: AudioFormat)
		-> Result<Converter, Error> {
		Converter::new(
			src_fmt,
			AudioFormat::new(self.channel_layout(), self.sample_format(), self.sample_rate())
		)
	}

	pub fn new_frame(&self) -> Result<Frame, Error> {
		let mut frame = Frame::empty().unwrap();
		frame.set_channel_layout(self.channel_layout());
		frame.set_sample_format(self.sample_format());
		frame.set_sample_rate(self.sample_rate());
		frame.set_nb_samples(self.frame_size() as i32);

		unsafe { ffm_op!(av_frame_get_buffer(frame.as_mut_ptr(), 0))? };

		Ok(frame)
	}
}
