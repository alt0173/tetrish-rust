use crate::{
	game::{letters::ALPHABET, xy_to_usize},
	ui::COLOR_PALLETE,
};

/// Takes in text and a point (the bottom left of the text) and draws it.
///
/// Supports 26-character uppercase latin alphabet. Each character is 5x5
/// pixels, with 1 between, so total space consumed is equal to the length
/// of the input text, times 6, minus 1.
pub fn draw_text<S: ToString>(point: (u8, u8), text: S, frame: &mut [u8]) {
	for (index, letter) in text.to_string().to_ascii_lowercase().chars().enumerate() {
		// Convert character to ascii index, then subtract 96 to get it's position
		// in the alphabet.
		let character_index = (letter as u32 - 96) as usize;
		let character = ALPHABET[character_index - 1];

		// Position of the left edge of the letter
		let left = point.0 as usize + (index * 5) + index;

		// Iterate over all position in the letter's 5x5 area
		for (x_index, x) in (left..left + 5).enumerate() {
			for (y_index, y) in ((point.1)..(point.1 + 5)).enumerate() {
				let pixel = &mut frame
					.chunks_exact_mut(4)
					.nth(xy_to_usize((x.try_into().unwrap(), y.try_into().unwrap())))
					.unwrap();

				if character[y_index][x_index] {
					pixel.copy_from_slice(&COLOR_PALLETE[5]);
				}
			}
		}
	}
}
