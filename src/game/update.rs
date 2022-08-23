use std::time::Instant;

use super::{piece::PIECES, GameState, ONE_FRAME, TIME_STEP};

pub fn update(state: &mut GameState) {
	// Update delta time
	state.delta_time += TIME_STEP;

	while state.delta_time >= ONE_FRAME {
		// Increment timestep, gravity, etc.
		state.delta_time -= ONE_FRAME;
		state.piece_movement += 0.02;
		state.active_piece.drop_location = state.selected_piece_drop_location();

		// Re-fill bag if needed
		if state.bag.len() == 7 {
			for _ in 0..7 {
				state.bag.push_back(state.lcg.random_piece() % 7);
			}
		}

		// Restart game is piece goes too high
		for x in 0..10 {
			for y in 0..4 {
				if state.board[x][y].is_some() {
					*state = GameState::default();
				}
			}
		}

		// Remove full rows
		for y in 4..24 {
			let mut contains_empty = false;
			for x in 0..10 {
				if state.board[x][y].is_none() {
					contains_empty = true;
				}
			}

			// Remove row
			if !contains_empty {
				for x in 0..10 {
					state.board[x][y] = None;
				}

				// Move all rows above it down
				for y in (4..y).rev() {
					for x in 0..10 {
						state.board[x][y + 1] = state.board[x][y];
						state.board[x][y] = None;
					}
				}
			}
		}

		// Check if piece can be moved down
		if state.piece_can_move(state.active_piece.location, 0, 1) {
			while state.piece_movement > 1.0 && state.piece_can_move(state.active_piece.location, 0, 1) {
				state.piece_movement -= 1.0;

				// Move selected piece down every tick (Gravity)
				for position in state.active_piece.location.iter_mut() {
					position[1] += 1;
				}
			}

			state.active_piece.resting_start = None;
		// If the piece can't move down and has been resting for an ammount of time
		} else if let Some(resting_start) = state.active_piece.resting_start {
			if Instant::now().duration_since(resting_start).as_millis() > 500 {
				// Add selected piece to static board
				for xy in state.active_piece.location.iter() {
					state.board[xy[0] as usize][xy[1] as usize] = Some(state.active_piece.color as u8);
				}

				// Load the next piece
				state.active_piece.location = PIECES[state.bag[0]].map(|xy| [xy[0] + 4, xy[1]]);
				state.active_piece.color = state.bag[0] + 2;
				state.bag.pop_front();
			}
		// If the piece can't move down and is not resting
		} else {
			state.active_piece.resting_start = Some(Instant::now());
		}
	}
}
