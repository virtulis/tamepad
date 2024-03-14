use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use lazy_static::lazy_static;
use sdl2::gfx::primitives::DrawRenderer;
use crate::state::State;

use crate::types::{Axis, Button, InputEvent};

lazy_static! {
	static ref EVENT_SENDER: Mutex<Option<sdl2::event::EventSender>> = Mutex::new(None);
}

#[derive(Debug)]
struct CustomEvent {
	state_arc: Option<Arc<RwLock<State>>>
}

pub fn sdl_task(sender: crossbeam_channel::Sender<InputEvent>) -> Result<(), String> {
	
	let mut state_arc: Option<Arc<RwLock<State>>> = None;
	
	sdl2::hint::set("SDL_HINT_RENDER_SCALE_QUALITY", "1");
	
	let sdl_context = sdl2::init()?;
	let joystick_subsystem = sdl_context.joystick()?;
	let game_controller_subsystem = sdl_context.game_controller()?;
	let video_subsys = sdl_context.video()?;
	
	let w_width = 500;
	let w_height = 500;
	let window = video_subsys
		.window(
			"Tamepad",
			w_width,
			w_height,
		)
		.position_centered()
		.opengl()
		.build()
		.map_err(|e| e.to_string())?;
	
	let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
	canvas.set_integer_scale(false).unwrap();
	canvas.set_logical_size(1000, 1000).unwrap();
	
	let controllers = RefCell::new(HashMap::new());

	let maybe_add_controller = |id| {
		let guid = match joystick_subsystem.device_guid(id) {
			Ok(id) => id,
			Err(e) => {
				println!("{} {:?}", id, e);
				return None;
			}
		};
		let name = joystick_subsystem
			.name_for_index(id)
			.unwrap_or(guid.to_string());
		println!("{} {} {}", id, guid, name);
		if !game_controller_subsystem.is_game_controller(id) {
			println!("{} is not a game controller", id);
			return None;
		}
		match game_controller_subsystem.open(id) {
			Ok(c) => {
				println!("Success: opened {} \"{}\"", c.instance_id(), c.name());
				println!("Controller mapping: {}", c.mapping());
				controllers.borrow_mut().insert(id, c);
				Some(InputEvent::Added(id, name))
			}
			Err(e) => {
				println!("failed: {:?}", e);
				None
			}
		}
	};

	let maybe_remove_controller = |id| {
		controllers.borrow_mut().remove(&id).is_some()
	};

	{
		let available = game_controller_subsystem
			.num_joysticks()
			.map_err(|e| format!("can't enumerate joysticks: {}", e))?;
		println!("{} joysticks available", available);
		for id in 0..available {
			maybe_add_controller(id);
		}
	}
	
	let white = sdl2::pixels::Color::RGB(255, 255, 255);
	let black = sdl2::pixels::Color::RGB(0, 0, 0);
	let mut paint = |state: &State| {
		
		canvas.set_draw_color(white.clone());
		canvas.clear();
		canvas.set_draw_color(black.clone());
		
		for btn in &state.config.buttons {
			
			canvas.aa_circle(btn.x as i16, btn.y as i16, btn.border_radius as i16, black.clone()).unwrap();
			if state.button_states[btn.button as usize].down {
				canvas.filled_circle(btn.x as i16, btn.y as i16, btn.fill_radius as i16, black.clone()).unwrap();
			}
			
		}
		
		canvas.present();
		
	};

	let ev = sdl_context.event().unwrap();
	ev.register_custom_event::<CustomEvent>().unwrap();
	
	EVENT_SENDER.lock().unwrap().replace(ev.event_sender());
	
	for event in sdl_context.event_pump()?.wait_iter() {
		use sdl2::event::Event;

		// println!("{:?}", event);

		let res = match event {
			Event::ControllerAxisMotion {
				which, axis, value, ..
			} => {
				sender.send(InputEvent::AxisMoved(which, axis.into(), value))
			}
			Event::ControllerButtonDown { which, button, .. } => {
				sender.send(InputEvent::ButtonDown(which, button.into()))
			}
			Event::ControllerButtonUp { which, button, .. } => {
				sender.send(InputEvent::ButtonUp(which, button.into()))
			}
			Event::ControllerDeviceAdded { which, .. } => {
				if let Some(ev) = maybe_add_controller(which) {
					sender.send(ev)
				}
				else {Ok(())}
			},
			Event::ControllerDeviceRemoved { which, .. } => {
				if maybe_remove_controller(which) {
					sender.send(InputEvent::Removed(which))
				}
				else {Ok(())}
			},
			Event::Quit { .. } => break,
			Event::User { .. } => {
				if let Some(ce) = event.as_user_event_type::<CustomEvent>() {
					if ce.state_arc.is_some() {
						state_arc = ce.state_arc.clone();
					}
				}
				if let Some(arc) = &state_arc {
					let state = arc.read().unwrap();
					paint(&state);
				}
				Ok(())
			},
			ev => {
				// println!("{:?}", ev);
				Ok(())
			},
		};
		if let Err(_) = res { break }
		
	}

	Ok(())
}

pub fn send_sdl_event(state_arc: Option<Arc<RwLock<State>>>) -> bool {
	let sender_opt = EVENT_SENDER.lock().unwrap();
	match sender_opt.as_ref() {
		None => false,
		Some(sender) => {
			let state_arc = state_arc.clone();
			sender.push_custom_event(CustomEvent {
				state_arc
			}).unwrap();
			true
		}
	}
}

macro_rules! id_enum {    ($from:ty, $to:ty, ($($item:tt),*)) => {
		impl From<$from> for $to {
			fn from(value: $from) -> Self {
				match (value) {
					$(<$from>::$item => <$to>::$item,)*
				}
			}
		}
	};
}

id_enum!(
	sdl2::controller::Button,
	Button,
	(
		A,
		B,
		X,
		Y,
		Back,
		Guide,
		Start,
		LeftStick,
		RightStick,
		LeftShoulder,
		RightShoulder,
		DPadUp,
		DPadDown,
		DPadLeft,
		DPadRight,
		Misc1,
		Paddle1,
		Paddle2,
		Paddle3,
		Paddle4,
		Touchpad
	)
);
id_enum!(
	sdl2::controller::Axis,
	Axis,
	(LeftX, LeftY, RightX, RightY, TriggerLeft, TriggerRight)
);
