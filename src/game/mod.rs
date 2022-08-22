pub mod draw;
pub mod letters;
pub mod piece;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::ui::SCREEN_WIDTH;

use self::piece::PIECES;

pub const FRAMERATE: usize = 200;
pub const TIME_STEP: Duration = Duration::from_nanos(1_000_000_000 / FRAMERATE as u64);
/// Internal framerate (144fps);
pub const ONE_FRAME: Duration = Duration::from_nanos(1_000_000_000 / 144);

/// Contains board, piece bag, etc.
pub struct GameState {
	/// Contains the board, with squares being either empty None, or filled Some(u8).
	/// Filled squares color values will be mapped to a color by the UI.
	pub board: [[Option<u8>; 24]; 10],
	/// The coordinates currently occupied by the piece being dropped.
	pub active_piece: [[u8; 2]; 4],
	/// The color pallete index of the current piece's color.
	pub active_piece_color: usize,
	/// The location the selected piece will end up if hard-dropped.
	pub active_piece_drop_location: [[u8; 2]; 4],
	/// When this reaches 1.0, the selected piece will move down.
	piece_movement: f32,
	/// Time of the last tick. Used for things like block drop speed.
	pub delta_time: Duration,
	pub input: WinitInputHelper,
}

impl Default for GameState {
	fn default() -> Self {
		let random_number = random_usize() % 7;

		Self {
			board: Default::default(),
			active_piece: PIECES[random_number].map(|xy| [xy[0] + 4, xy[1]]),
			active_piece_color: random_number + 2,
			active_piece_drop_location: [[0; 2]; 4],
			piece_movement: 0.0,
			delta_time: Duration::ZERO,
			input: WinitInputHelper::new(),
		}
	}
}

impl GameState {
	pub fn update(&mut self) {
		// Update delta time
		self.delta_time += TIME_STEP;

		while self.delta_time >= ONE_FRAME {
			// Increment timestep, gravity, etc.
			self.delta_time -= ONE_FRAME;
			self.piece_movement += 0.02;
			self.active_piece_drop_location = self.selected_piece_drop_location();

			// Lose game is piece goes too high
			for x in 0..10 {
				for y in 0..4 {
					if self.board[x][y].is_some() {
						panic!();
					}
				}
			}

			// Remove full rows
			for y in 4..24 {
				let mut contains_empty = false;
				for x in 0..10 {
					if self.board[x][y].is_none() {
						contains_empty = true;
					}
				}

				// Remove row
				if !contains_empty {
					for x in 0..10 {
						self.board[x][y] = None;
					}

					// Move all rows above it down
					for y in (4..y).rev() {
						for x in 0..10 {
							self.board[x][y + 1] = self.board[x][y];
							self.board[x][y] = None;
						}
					}
				}
			}

			// Check if piece can be moved down
			if self.piece_can_move(self.active_piece, 0, 1) {
				while self.piece_movement > 1.0 && self.piece_can_move(self.active_piece, 0, 1) {
					self.piece_movement -= 1.0;

					// Move selected piece down every tick (Gravity)
					for position in self.active_piece.iter_mut() {
						position[1] += 1;
					}
				}
			} else {
				// Add selected piece to static board
				for xy in self.active_piece.iter() {
					self.board[xy[0] as usize][xy[1] as usize] = Some(self.active_piece_color as u8);
				}

				// Generate a new piece
				let random_number = random_usize() % 7;
				self.active_piece = PIECES[random_number].map(|xy| [xy[0] + 4, xy[1]]);
				self.active_piece_color = random_number + 2;
			}
		}
	}

	pub fn handle_input(&mut self) {
		// Move down
		if self.input.key_held(VirtualKeyCode::S) && self.piece_can_move(self.active_piece, 0, 1) {
			self.piece_movement += 0.25;
		}

		// Move left
		if self.input.key_pressed(VirtualKeyCode::A) && self.piece_can_move(self.active_piece, -1, 0) {
			self.active_piece = self.active_piece.map(|xy| [xy[0] - 1, xy[1]]);
		}

		// Move right
		if self.input.key_pressed(VirtualKeyCode::D) && self.piece_can_move(self.active_piece, 1, 0) {
			self.active_piece = self.active_piece.map(|xy| [xy[0] + 1, xy[1]]);
		}

		// Hard drop
		if self.input.key_pressed(VirtualKeyCode::LShift) {
			self.active_piece = self.active_piece_drop_location;
		}

		// Rotation
		if self.input.key_pressed(VirtualKeyCode::Q) || self.input.key_pressed(VirtualKeyCode::E) {
			let mut selected_piece_destination = [[0; 2]; 4];
			let pivot = self.active_piece[0];

			// Compute the piece's destination after rotation
			for (xy, destination) in self
				.active_piece
				.iter_mut()
				.zip(selected_piece_destination.iter_mut())
			{
				// The jank here is to prevent underflow :|
				if self.input.key_pressed(VirtualKeyCode::Q) {
					destination[0] = (xy[1] + pivot[0]).checked_sub(pivot[1]).unwrap_or(255);
					destination[1] = (pivot[0] + pivot[1]).checked_sub(xy[0]).unwrap_or(255);
				} else {
					destination[0] = (pivot[0] + pivot[1]).checked_sub(xy[1]).unwrap_or(255);
					destination[1] = (xy[0] + pivot[1]).checked_sub(pivot[0]).unwrap_or(255);
				}
			}

			if self.is_space_open(&selected_piece_destination) {
				self.active_piece = selected_piece_destination;
			}
		}
	}

	/// Turns the game state into pixels and writes it to a passed buffer.
	pub fn draw(&self, frame: &mut [u8]) {
		draw::draw(&self, frame);
	}

	// Takes in a list of coordinates on the board and returns true if they are
	// all within bounds and empty
	pub fn is_space_open(&self, locations: &[[u8; 2]; 4]) -> bool {
		for xy in locations {
			let (x, y) = (xy[0], xy[1]);

			// If the position would be out of bounds or
			// If there is a occupied tile at the target position
			if x >= 10 || y >= 24 || self.board[x as usize][y as usize].is_some() {
				return false;
			}
		}

		true
	}

	/// Checks the board and returns true if the given piece can be moved by a
	/// given offset.
	fn piece_can_move(&self, piece: [[u8; 2]; 4], x_offset: i8, y_offset: i8) -> bool {
		// Maps each position, plus the offset, to true if it is a valid position,
		// and false if it is not.
		!piece
			.iter()
			.map(|xy| {
				let (x, y) = (xy[0] as i8, xy[1] as i8);

				if x + x_offset >= 10 || x + x_offset < 0 || y + y_offset >= 24 {
					// If the position would be out of bounds
					false
				} else {
					// If there is a occupied tile at the target position
					self.board[usize::from((x + x_offset) as u8)][usize::from((y + y_offset) as u8)].is_none()
				}
			})
			.collect::<Vec<bool>>()
			.contains(&false)
	}

	/// Returns the location of the selected piece if it was hard dropped.
	fn selected_piece_drop_location(&self) -> [[u8; 2]; 4] {
		let mut down: u8 = 0;

		while self.piece_can_move(self.active_piece, 0, down as i8) {
			down += 1;
		}

		self.active_piece.map(|xy| [xy[0], xy[1] + down - 1])
	}
}

/// Converts a 1d coordinate (e.g. index) to 2d (x, y).
fn usize_to_xy(input: usize) -> (usize, usize) {
	let x = input % (SCREEN_WIDTH as usize);
	let y = input / (SCREEN_WIDTH as usize);

	(x, y)
}

/// Converts a 2d coordinate (x, y) to 1d (e.g. index).
pub fn xy_to_usize(input: (u8, u8)) -> usize {
	input.0 as usize + input.1 as usize * SCREEN_WIDTH as usize
}

/// Returns true if a point is within a given rectangle, defined by a minimum
/// and maxmimum point. All coordinates are 2D XY.
fn point_in_rectangle(point: (u8, u8), min: (u8, u8), max: (u8, u8)) -> bool {
	point.0 >= min.0 && point.1 >= min.1 && point.0 < max.0 && point.1 < max.1
}

fn random_usize() -> usize {
	let seed = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_nanos()
		/ 85819;
	((1255 * seed + 6173) % 29282).try_into().unwrap()
}