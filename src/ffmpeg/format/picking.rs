use super::sample_format::*;

pub fn pick_best_format(iter: FormatIter, format: SampleFormat) -> Option<SampleFormat> {
	iter.max_by_key(|f| {
		((f.sample_type() == format.sample_type()) as i32) * 2
			+ ((f.is_planar() == format.is_planar()) as i32)
	})
}