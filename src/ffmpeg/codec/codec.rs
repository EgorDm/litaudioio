use sys::*;
use crate::ffmpeg::utils::*;
use crate::ffmpeg::format::*;

ff_wrap_struct!(Codec, AVCodec);
ff_wrap!(Codec, AVCodec);

impl Codec {
	pub fn formats(&self) -> Option<FormatIter> {
		if self.as_ref().sample_fmts.is_null() {
			None
		} else {
			Some(FormatIter::new(self.as_ref().sample_fmts))
		}
	}
}
