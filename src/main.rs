extern crate core;

use crate::gui::{gui_loop, init_gui};
use crate::linux::linux_actions_task;
use crate::sdl::sdl_task;
use crate::state::state_task;
use crate::types::write_schema;

mod gui;
mod linux;
mod sdl;
mod state;
mod types;

fn main() -> Result<(), anyhow::Error> {
	println!("Hello, world!");

	let (event_loop, current_context, gl_display, window, surface) = init_gui();

	let (input_sender, input_receiver) = crossbeam_channel::unbounded();
	let (actions_sender, actions_receiver) = crossbeam_channel::unbounded();
	let ui_loop_proxy = event_loop.create_proxy();

	let sdl = std::thread::spawn(|| {
		sdl_task(input_sender).unwrap_or_else(|e| {
			println!("SDL thread: {:?}", e);
		})
	});
	let state = std::thread::spawn(|| {
		state_task(input_receiver, actions_sender, ui_loop_proxy).unwrap_or_else(|e| {
			println!("State thread: {:?}", e);
		})
	});
	let actions = std::thread::spawn(|| {
		linux_actions_task(actions_receiver).unwrap_or_else(|e| {
			println!("Actions thread: {:?}", e);
		})
	});

	write_schema();

	gui_loop(event_loop, current_context, gl_display, window, surface).unwrap();

	sdl.join().unwrap();
	state.join().unwrap();
	actions.join().unwrap();

	Ok(())
}
