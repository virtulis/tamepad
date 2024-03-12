use crate::handle::handler_task;
use crate::sdl::sdl_task;

mod handle;
mod types;
mod sdl;

fn main() {
	println!("Hello, world!");

	let (input_sender, input_receiver) = crossbeam_channel::unbounded();

	let sdl = std::thread::spawn(|| sdl_task(input_sender).unwrap());
	let handler = std::thread::spawn(|| handler_task(input_receiver));

	sdl.join().unwrap();
	handler.join().unwrap();
}
