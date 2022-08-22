use pixels::{Pixels, SurfaceTexture};
use winit::{
	event_loop::EventLoop,
	window::{Window, WindowBuilder},
};

use crate::ui::{LOGICAL_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};

/// Fulfills all pre-requisites for the program to run (e.g. creates window,
/// pixel buffer).
pub fn initial_setup() -> (EventLoop<()>, Window, Pixels) {
	let event_loop = EventLoop::new();

	let window = WindowBuilder::new()
		.with_title("Tetrust")
		.with_inner_size(LOGICAL_SIZE)
		.with_min_inner_size(LOGICAL_SIZE)
		.build(&event_loop)
		.unwrap();

	let (width, height) = (SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
	let surface_texture = SurfaceTexture::new(width, height, &window);
	let pixels = Pixels::new(width, height, surface_texture).unwrap();

	(event_loop, window, pixels)
}
