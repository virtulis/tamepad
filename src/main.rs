extern crate core;

use std::sync::{Arc, Mutex};

use sdl2::event::EventSender;
use signal_hook::iterator::Signals;

use crate::gui::{gui_loop, init_gui, UIEvent};
use crate::linux::linux_actions_task;
use crate::sdl::sdl_task;
use crate::state::state_task;
use crate::types::{MainEvent, write_schema};

mod gui;
mod linux;
mod sdl;
mod state;
mod types;

fn main() {
	println!("Hello, world!");

	let (event_loop, current_context, gl_display, window, surface) = init_gui();

	let (main_sender, main_receiver) = crossbeam_channel::unbounded();
	let (input_sender, input_receiver) = crossbeam_channel::unbounded();
	let (actions_sender, actions_receiver) = crossbeam_channel::unbounded();
	let ui_loop_proxy = event_loop.create_proxy();

	let sdl_sender: Arc<Mutex<Option<EventSender>>> = Arc::new(Mutex::new(None));
	let sdl_sender_copy = sdl_sender.clone();
	let sdl = std::thread::spawn(move || {
		sdl_task(input_sender, sdl_sender_copy).unwrap_or_else(|e| {
			println!("SDL thread: {:?}", e);
		})
	});
	
	let ui = ui_loop_proxy.clone();
	let mr = main_receiver.clone();
	let state = std::thread::spawn(move || {
		state_task(input_receiver, actions_sender, ui, mr).unwrap_or_else(|e| {
			println!("State thread: {:?}", e);
		})
	});
	
	let actions = std::thread::spawn(move || {
		linux_actions_task(actions_receiver).unwrap_or_else(|e| {
			println!("Actions thread: {:?}", e);
		})
	});
	
	let ui = ui_loop_proxy.clone();
	std::thread::spawn(move || {
		let mut signals = Signals::new(&[
			signal_hook::consts::signal::SIGTERM,
			signal_hook::consts::signal::SIGINT,
		]).unwrap();
		for _s in signals.forever() {
			ui.send_event(UIEvent::Quit).unwrap()
		}
	});

	write_schema();
	
	gui_loop(event_loop, current_context, gl_display, window, surface).unwrap();
	
	main_sender.send(MainEvent::Quit).unwrap();
	{
		let lock = sdl_sender.lock();
		lock.unwrap().as_mut().unwrap().push_event(sdl2::event::Event::Quit { timestamp: 0 }).unwrap();
	}

	sdl.join().unwrap();
	state.join().unwrap();
	actions.join().unwrap();
	
	std::process::exit(0);
	
}
