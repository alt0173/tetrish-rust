/// List of pieces in the form of their 4 tiles as XY coordinates within a
/// 4x4 grid, with the top left corner being 0,0.
///
/// Note that the first coordinate of the piece is the pivot point.
pub const PIECES: [[[u8; 2]; 4]; 7] = [
	// I
	[[1, 1], [1, 2], [1, 0], [1, 3]],
	// J
	[[1, 1], [1, 2], [1, 0], [0, 2]],
	// L
	[[1, 1], [1, 2], [1, 0], [0, 0]],
	// O
	[[1, 1], [1, 0], [0, 0], [0, 1]],
	// S
	[[1, 1], [0, 0], [0, 1], [1, 2]],
	// T
	[[1, 1], [1, 0], [1, 2], [0, 1]],
	// Z
	[[1, 1], [1, 0], [0, 1], [0, 2]],
];
