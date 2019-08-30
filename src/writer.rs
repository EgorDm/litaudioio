use std::{ptr, cmp};
use crate::sys::*;
use crate::ffmpeg::*;
use crate::error::Error;
use crate::output::Output;
use litaudio::*;
use litcontainers::*;
use std::cmp::min;
use std::marker::PhantomData;

pub struct Writer<'a: 'b, 'b, T, P, S>
	where T: Sample, P: SamplePackingType, S: AudioStorage<T, P>
{
	output: Output,
	input: &'a S,
	cursor: Slice<'b, T, S::Rows, S::RowStride, Dynamic, S::ColStride>,
	converter: Option<Converter>,
	sample_count: usize,
	max_frame_size: usize,
	_phantoms: PhantomData<(P)>
}

impl<'a: 'b, 'b, T, P, S> Writer<'a, 'b, T, P, S>
	where T: Sample, P: SamplePackingType, S: AudioStorage<T, P>
{
	pub fn open(path: &str, audio: &'a S) -> Result<Self, Error> {
		let output = Output::open(
			&path,
			|i| pick_best_format(i, SampleFormat::from_type::<T, P>()),
			ChannelLayout::default(audio.rows() as i32),
			audio.sample_rate()
		)?;
		output.format_ctx().dump(&path);

		let use_converter = output.sample_format() != SampleFormat::from_type::<T, P>();
		let converter = match use_converter {
			false => None,
			true => Some(output.converter(AudioFormat::from_storage(audio))?)
		};

		let cursor = SliceBase::new(
			unsafe {
				PtrStorage::new(
					std::ptr::null(),
					Size::new(audio.channel_dim(), Dynamic::new(0)),
					audio.strides()
				)
			}
		).into();

		let max_frame_size = output.frame_size();
		Ok(Writer { output, input: audio, cursor, converter, sample_count: 0, max_frame_size, _phantoms: PhantomData })
	}

	pub fn write(mut self) -> Result<(), Error> {
		unsafe { ffm_op!(avformat_write_header(self.output.format_ctx().as_mut_ptr(), ptr::null_mut()))?; }

		let mut frame = self.output.new_frame()?;
		let mut packet = Packet::empty();

		while match self.write_frame(&mut packet, Some(&mut frame)) {
			Err(Error::FFM(FFError::Eof)) => false,
			Err(e) => return Err(e),
			Ok(_) => true
		} {}

		while match self.write_frame(&mut packet, None) {
			Err(Error::FFM(FFError::Eof)) => false,
			Err(e) => return Err(e),
			Ok(_) => true
		} {}

		unsafe { ffm_op!(av_write_trailer(self.output.format_ctx().as_mut_ptr()))? };

		Ok(())
	}

	pub fn write_frame(&mut self, packet: &mut Packet, frame: Option<&mut Frame>) -> Result<(), Error> {
		let mut frame_cap = 0;

		match frame {
			None => {
				match Frame::send_flush(&mut self.output.codec_ctx()) {
					Err(Error::FFM(FFError::Again)) => {},
					Err(e) => return Err(e),
					_ => {}
				}
			},
			Some(frame) => {
				// TODO: fill frame fn?
				let buffer_size = self.input.samples() - self.sample_count;
				if buffer_size <= 0 {
					return Err(Error::from(FFError::Eof))
				}

				frame.set_nb_samples(self.max_frame_size as i32);
				self.cursor.storage_mut().storage_mut().shift_col_to(self.input, self.sample_count, cmp::min(self.max_frame_size, buffer_size));
				frame_cap = self.copy_cursor_to_frame(frame)?;
				frame.set_nb_samples(frame_cap);

				match frame.send(&mut self.output.codec_ctx()) {
					Err(Error::FFM(FFError::Again)) => {},
					Err(e) => return Err(e),
					_ => {}
				}
			}
		}

		match packet.recieve(&mut self.output.codec_ctx()) {
			Err(Error::FFM(FFError::Again)) => {},
			Err(e) => return Err(e),
			_ => {}
		}

		packet.write(&self.output.format_ctx())?;
		packet.reset();

		self.sample_count += frame_cap as usize;

		Ok(())
	}

	pub fn copy_cursor_to_frame(&mut self, frame: &mut Frame) -> Result<i32, Error> {
		Ok(match self.converter {
			None => {
				let sample_count = min(frame.nb_samples() as usize, self.cursor.samples());
				match self.input.packing_type() {
					SamplePacking::Interleaved => {
						unsafe {
							ptr::copy_nonoverlapping(
								self.cursor.as_row_ptr(0),
								frame.data_mut_ptr(0) as *mut T,
								sample_count * self.cursor.rows()
							);
						}
					},
					SamplePacking::Deinterleaved => {
						for c in 0..self.cursor.rows() {
							unsafe {
								ptr::copy_nonoverlapping(
									self.cursor.as_row_ptr(c),
									frame.data_mut_ptr(c) as *mut T,
									sample_count
								);
							}
						}
					}
				}
				sample_count as i32
			},
			Some(ref mut converter) => {
				converter.convert_slice(&self.cursor, frame)?
			}
		})
	}
}
