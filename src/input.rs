use crate::sys::*;
use crate::ffmpeg::*;
use crate::error::Error;

pub struct Input {
	format_ctx: FormatContext,
	stream: Stream,
	codec_ctx: DecoderOpen,
}

impl Input {
	pub fn open<F>(path: &str, format_picker: F) -> Result<Self, Error>
		where F: Fn(FormatIter) -> Option<SampleFormat>
	{
		let format_ctx = open_read(path)?;
		let stream = format_ctx.get_audio_stream()
			.ok_or("Could not find any audio stream in the file")?;
		let codec_params = stream.parameters();
		let mut codec_ctx = DecoderContext::create(&codec_params)?;

		let format_iter = codec_ctx.ctx().codec().formats()
			.ok_or("Could not find appropriate sample format")?;
		let sample_format = format_picker(format_iter)
			.ok_or("Could not find appropriate sample format")?;

		codec_ctx.set_request_sample_fmt(sample_format);
		let codec_ctx = DecoderOpen::open(codec_ctx)?;

		Ok(Input { format_ctx, stream, codec_ctx })
	}

	pub fn format_ctx(&self) -> &FormatContext { &self.format_ctx }

	pub fn stream(&self) -> &Stream { &self.stream }

	pub fn codec_ctx(&self) -> &DecoderOpen { &self.codec_ctx }

	pub fn estimated_sample_count(&self) -> usize {
		(self.format_ctx.get_duration() * self.codec_ctx.ctx().ctx().sample_rate() as usize) / AV_TIME_BASE as usize
	}

	pub fn channel_layout(&self) -> ChannelLayout { self.codec_ctx.ctx().ctx().channel_layout() }

	pub fn sample_format(&self) -> SampleFormat { self.codec_ctx.ctx().sample_fmt() }

	pub fn sample_rate(&self) -> i32 { self.codec_ctx.ctx().ctx().sample_rate() }

	pub fn converter(&self, dst_fmt: AudioFormat)
		-> Result<Converter, Error> {
		Converter::new(
			AudioFormat::new(self.channel_layout(), self.sample_format(), self.sample_rate()),
			dst_fmt
		)
	}
}