use std::time::{SystemTime, UNIX_EPOCH};

/// Simple linear congruential generator.
#[derive(Default)]
pub struct LCG {
	pub last_value: usize,
}

impl LCG {
	/// Generates a random usize.
	fn random_usize(&mut self) -> usize {
		let salt = (SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_nanos()
			/ 4351) as usize;

		self.last_value = ((1255 * self.last_value + salt + 6173) % 29282)
			.try_into()
			.unwrap();
		self.last_value
	}

	/// Generates a random number in range 0..7
	pub fn random_piece(&mut self) -> usize {
		self.random_usize() % 7
	}
}
