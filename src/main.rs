use anyhow::anyhow;

use crate::actions::actions_task;
use crate::sdl::sdl_task;
use crate::state::state_task;

mod actions;
mod sdl;
mod state;
mod types;

fn main() -> Result<(), anyhow::Error> {
	println!("Hello, world!");

	let (input_sender, input_receiver) = crossbeam_channel::unbounded();
	let (actions_sender, actions_receiver) = crossbeam_channel::unbounded();

	let sdl = std::thread::spawn(|| sdl_task(input_sender));
	let state = std::thread::spawn(|| state_task(input_receiver, actions_sender));
	let actions = std::thread::spawn(|| actions_task(actions_receiver));
	
	sdl.join().unwrap().map_err(|e| anyhow!("SDL thread: {:?}", e))?;
	state.join().unwrap()?;
	actions.join().unwrap()?;
	
	Ok(())
	
}
