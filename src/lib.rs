#[macro_use] extern crate derive_new;
#[macro_use] extern crate bitflags;
extern crate libc;

pub extern crate ffmpeg_sys as sys;

#[macro_use] pub mod error;
pub mod ffmpeg;

pub mod input;
pub mod reader;
pub mod output;
pub mod writer;

use litaudio::*;
use reader::*;
use writer::*;
use error::*;
use std::path::Path;
use litcontainers::StorageConstructor;

pub fn read_audio<T, C, P, S>(path: &Path) -> Result<AudioContainer<T, C, Dynamic, P, S>, Error>
	where T: Sample, C: Dim, P: SamplePackingType, S: AudioStorageMut<T, C, Dynamic, P> + DynamicSampleStorage<T, C> + StorageConstructor<T, C, Dynamic>
{
	let path = match path.to_str() {
		None => return Err(Error::from(format!("Invalid path: {}", path.display()))),
		Some(s) => s
	};
	Reader::open(&path, None)?.read()
}

pub fn write_audio<S, T, C, L, P>(path: &Path, audio: &S) -> Result<(), Error>
	where T: Sample, C: Dim, L: Dim, P: SamplePackingType, S: AudioStorage<T, C, L, P>
{
	let path = match path.to_str() {
		None => return Err(Error::from(format!("Invalid path: {}", path.display()))),
		Some(s) => s
	};
	Writer::open(&path, audio)?.write()
}
