use litaudio::*;
use super::channel_layout::*;
use super::sample_format::*;

#[derive(new)]
pub struct AudioFormat {
	pub channel_layout: ChannelLayout,
	pub sample_format: SampleFormat,
	pub sample_rate: i32
}

impl AudioFormat {
	pub fn from_storage<T, C, L, P, S>(s: &S) -> Self
		where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorage<T, C, L, P>
	{
		AudioFormat {
			channel_layout: ChannelLayout::default(s.channel_count() as i32),
			sample_format: SampleFormat::from_storage(s),
			sample_rate: s.sample_rate()
		}
	}
}