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
use litcontainers::{StorageConstructor, Container, StorageMut};

pub fn read_audio<T, P, S>(path: &Path) -> Result<Container<T, AudioContainer<T, P, S>>, Error>
	where T: Sample, P: SamplePackingType, S: StorageMut<T> + DynamicSampleStorage<T> + StorageConstructor<T>
{
	let path = match path.to_str() {
		None => return Err(Error::from(format!("Invalid path: {}", path.display()))),
		Some(s) => s
	};
	Reader::open(&path, None)?.read().map(|v| v.into())
}

pub fn write_audio<S, T, P>(path: &Path, audio: &S) -> Result<(), Error>
	where T: Sample, P: SamplePackingType, S: AudioStorage<T, P>
{
	let path = match path.to_str() {
		None => return Err(Error::from(format!("Invalid path: {}", path.display()))),
		Some(s) => s
	};
	Writer::open(&path, audio)?.write()
}
