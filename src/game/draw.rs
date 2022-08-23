use crate::ui::{text::draw_text, COLOR_PALLETE, SCREEN_HEIGHT, SCREEN_WIDTH};

use super::{piece::PIECES, point_in_rectangle, usize_to_xy, GameState};

pub fn draw(game_state: &GameState, frame: &mut [u8]) {
	// Iterate over all RGBA values (pixels) in the buffer
	for (index, pixel) in frame.chunks_exact_mut(4).enumerate() {
		let (x, y) = usize_to_xy(index);
		let point: (u8, u8) = (x.try_into().unwrap(), y.try_into().unwrap());

		// Default / clear color;
		let mut color;

		// Background checkers
		if (x + y) % 2 == 0 {
			color = COLOR_PALLETE[2];
		} else {
			color = COLOR_PALLETE[3];
		}

		// Darken edges with a nice little gradient :)
		for offset in 0..3 {
			if !point_in_rectangle(
				point,
				(1 + offset, 1 + offset),
				(SCREEN_WIDTH - 1 - offset, SCREEN_HEIGHT - 1 - offset),
			) {
				color = color.map(|v| (v as f32 * 0.75) as u8);
			}
		}

		// The board
		if point_in_rectangle(point, (49, 0), (79, 72)) {
			// Convert the pixel coordinate into a position on the board
			// Note that each board position is represented by 3x3 pixels
			let (x, y) = ((x - 49) / 3, y / 3);

			if let Some(tile) = game_state.board[x][y] {
				color = COLOR_PALLETE[tile as usize];
			} else if game_state
				.active_piece
				.location
				.contains(&[x as u8, y as u8])
			{
				// The piece currently being dropped
				color = COLOR_PALLETE[game_state.active_piece.color];
			} else {
				// Otherwise, background color
				color = COLOR_PALLETE[0];

				// Drop location highlighting
				if game_state
					.active_piece
					.drop_location
					.contains(&[x as u8, y as u8])
				{
					let mut block_color = COLOR_PALLETE[game_state.active_piece.color];
					block_color[3] = 128;
					color = block_color;
				}

				// Out of bounds highlighting
				if y < 4 {
					color[0] = color[0].saturating_mul(2);
				}
			}
		} else if point_in_rectangle(point, (48, 0), (80, 72)) {
			// Border
			color = COLOR_PALLETE[1];
		}

		// Incoming piece background
		if point_in_rectangle(point, (85, 5), (89, 41)) {
			color = COLOR_PALLETE[0];
		}
		// Bag / incoming piece display
		if point_in_rectangle(point, (86, 5), (88, 41)) {
			let (x, y) = (x - 86, y - 5);

			if y % 5 != 0 {
				let piece_index = game_state.bag[y / 5];
				// Converts the local y (within this rectangle) into a value between
				// 0 and 4, ignoring every 5th pixel (for spacing)
				let piece_y = y - ((y as f32 / 5.0).floor() as usize * 5) - 1;

				// Checks if the piece contains this position, drawing it if so
				if PIECES[piece_index].contains(&[x as u8, piece_y as u8]) {
					color = COLOR_PALLETE[piece_index + 2];
				}
			}
		}

		pixel.copy_from_slice(&color);
	}

	draw_text((5, 5), "TETRUST", frame);
}
