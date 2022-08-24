pub mod alphabet;
pub mod event_loop;
pub mod numbers;
pub mod text;

use winit::dpi::LogicalSize;

pub const SCREEN_WIDTH: u8 = 128;
pub const SCREEN_HEIGHT: u8 = 72;
pub const LOGICAL_SIZE: LogicalSize<f64> =
	LogicalSize::new(SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64);

/// Maps numbers to the rgba color at their index.
///
/// (Black, White, Light Blue, Blue, Orange, Yellow, Green, Purple, Red).
pub const COLOR_PALLETE: [[u8; 4]; 9] = [
	[37, 38, 39, 255],    // Black
	[255, 249, 251, 255], // White
	[58, 174, 216, 255],  // Light Blue
	[75, 136, 162, 255],  // Blue
	[254, 127, 45, 255],  // Orange
	[255, 189, 0, 255],   // Yellow
	[67, 100, 54, 255],   // Green
	[158, 123, 155, 255], // Purple
	[187, 10, 33, 255],   // Red
];
