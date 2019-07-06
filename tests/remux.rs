#[cfg(test)]
mod remux_tests {
	use litaudio::{Dynamic, AudioDeinterleaved, U2};
	use litio::*;
	use std::error::Error;
	use std::path::PathBuf;

	#[test]
	fn it_works() {
		let mut in_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
		in_path.push( "assets/jazz.mp3");

		let s: Result<AudioDeinterleaved<f32, U2, Dynamic>, litio::error::Error> = read_audio(in_path.as_path());

		let mut out_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
		out_path.push("tmp/jazz.mp3");

		match s {
			Ok(s) => {
				match write_audio(out_path.as_path(), &s) {
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
}