use std::cell::RefCell;
use std::collections::HashMap;

use crate::types::{Axis, Button, InputEvent};

pub fn sdl_task(sender: crossbeam_channel::Sender<InputEvent>) -> Result<(), String> {
	let sdl_context = sdl2::init()?;
	let joystick_subsystem = sdl_context.joystick()?;
	let game_controller_subsystem = sdl_context.game_controller()?;

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

	for event in sdl_context.event_pump()?.wait_iter() {
		use sdl2::event::Event;

		// println!("{:?}", event);

		match event {
			Event::ControllerAxisMotion {
				which, axis, value, ..
			} => {
				sender.send(InputEvent::Axis(which, axis.into(), value)).unwrap();
			}
			Event::ControllerButtonDown { which, button, .. } => {
				sender.send(InputEvent::ButtonDown(which, button.into())).unwrap();
			}
			Event::ControllerButtonUp { which, button, .. } => {
				sender.send(InputEvent::ButtonUp(which, button.into())).unwrap();
			}
			Event::ControllerDeviceAdded { which, .. } => {
				if let Some(ev) = maybe_add_controller(which) {
					sender.send(ev).unwrap();
				}
			},
			Event::ControllerDeviceRemoved { which, .. } => {
				if maybe_remove_controller(which) {
					sender.send(InputEvent::Removed(which)).unwrap();
				}
			},
			Event::Quit { .. } => break,
			_ => (),
		}
	}

	Ok(())
}

macro_rules! id_enum {
    ($from:ty, $to:ty, ($($item:tt),*)) => {
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
