extern crate core;

use crate::linux::linux_actions_task;
use crate::sdl::sdl_task;
use crate::state::state_task;
use crate::types::write_schema;

mod linux;
mod sdl;
mod state;
mod types;

fn main() -> Result<(), anyhow::Error> {
	println!("Hello, world!");

	let (input_sender, input_receiver) = crossbeam_channel::unbounded();
	let (actions_sender, actions_receiver) = crossbeam_channel::unbounded();

	let sdl = std::thread::spawn(|| sdl_task(input_sender).unwrap_or_else(|e| {
		println!("SDL thread: {:?}", e);
	}));
	let state = std::thread::spawn(|| state_task(input_receiver, actions_sender).unwrap_or_else(|e| {
		println!("State thread: {:?}", e);
	}));
	let actions = std::thread::spawn(|| linux_actions_task(actions_receiver).unwrap_or_else(|e| {
		println!("Actions thread: {:?}", e);
	}));
	
	write_schema();
	
	sdl.join().unwrap();
	state.join().unwrap();
	actions.join().unwrap();
	
	Ok(())
	
}
