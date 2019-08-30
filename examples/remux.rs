use litaudio::*;
use litaudioio::*;
use std::error::Error;
use std::path::{PathBuf, Path};

fn main() {
	remux(
		&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/test_audio.wav"),
		&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tmp/test_audio.wav")
	);

	remux(
		&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/jazz.mp3"),
		&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tmp/jazz.mp3")
	);
}

fn remux(in_path: &Path, out_path: &Path) {
	let s: Result<AudioDeinterleaved<f32, U2, Dynamic>, litaudioio::error::Error> = read_audio(in_path);

	match s {
		Ok(s) => {
			match write_audio(out_path, &s) {
				Ok(_) => {},
				Err(e) => {
					println!("{}", e.description());
					assert!(false, e.description().to_string());
				}
			}
		},
		Err(e) => {
			assert!(false,  e.description().to_string());
		}
	}
}