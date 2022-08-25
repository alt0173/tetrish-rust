#![windows_subsystem = "windows"]

use ui::event_loop::run_event_loop;

pub mod game;
pub mod setup;
pub mod ui;

fn main() {
	run_event_loop();
}
