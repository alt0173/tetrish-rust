use winit::{event::Event, event_loop::ControlFlow};

use crate::{game::GameState, setup::initial_setup};

pub fn run_event_loop() {
	let (event_loop, window, mut pixels) = initial_setup();
	let mut game_state = GameState::default();

	event_loop.run(move |event, _, control_flow| {
		// Draw the frame, closing the program on failure
		if let Event::RedrawRequested(_) = event {
			game_state.draw(pixels.get_frame());

			pixels
				.render()
				.unwrap_or_else(|_| *control_flow = ControlFlow::Exit);
		}

		// Handle input events
		if game_state.input.update(&event) {
			// Close the window
			if game_state.input.quit() {
				*control_flow = ControlFlow::Exit;
			}

			// Resize the window
			if let Some(size) = game_state.input.window_resized() {
				pixels.resize_surface(size.width, size.height);
			}

			// Update game state and request redraw
			game_state.update();
			game_state.handle_input();
			window.request_redraw();
		}
	});
}
