use crate::sys::*;
use crate::ffmpeg::utils::*;

ff_wrap_struct!(CodecParameters, AVCodecParameters);
ff_wrap!(CodecParameters, AVCodecParameters);