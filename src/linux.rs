use std::{fs::OpenOptions, io};
use std::os::unix::fs::OpenOptionsExt;

use crossbeam_channel::select;
use input_linux::{
	EventKind, EventTime, InputEvent, InputId, KeyEvent, KeyState, SynchronizeEvent, SynchronizeKind, UInputHandle,
};
use lazy_static::lazy_static;
use strum::IntoEnumIterator;

use crate::types::{Action, Key};

lazy_static! {
	// This is very stupid but at least I only do it once.
	static ref KEY_TO_UINPUT: Vec<input_linux::Key> = Key::iter().map(|k| {
		let vec = serde_json::to_vec(&k).unwrap();
		serde_json::from_slice(vec.as_slice()).unwrap_or(input_linux::Key::Unknown)
	}).collect();
}

pub fn linux_actions_task(actions: crossbeam_channel::Receiver<Action>) -> io::Result<()> {
	let uinput_file = OpenOptions::new()
		.read(false)
		.write(true)
		.custom_flags(libc::O_NONBLOCK)
		.open("/dev/uinput")?;
	let uhandle = UInputHandle::new(uinput_file);

	uhandle.set_evbit(EventKind::Key)?;
	for key in Key::iter() {
		uhandle.set_keybit(KEY_TO_UINPUT[key as usize])?;	
	}

	// uhandle.set_evbit(EventKind::Relative)?;
	// uhandle.set_relbit(RelativeAxis::X)?;
	// uhandle.set_relbit(RelativeAxis::Y)?;

	let input_id = InputId {
		bustype: input_linux::sys::BUS_USB,
		vendor: 0x1234,
		product: 0x5678,
		version: 0,
	};
	let device_name = b"Tamepad";
	uhandle.create(&input_id, device_name, 0, &[])?;

	const ZERO: EventTime = EventTime::new(0, 0);
	loop {
		let _ = select! {
			recv(actions) -> act => match act {
				Ok(act) => {
					println!("{}", json5::to_string(&act).unwrap());
					let mut events = Vec::new();
					match act {
						Action::KeyDown(key) => {
							events.push(*InputEvent::from(KeyEvent::new(
								ZERO,
								KEY_TO_UINPUT[key as usize],
								KeyState::PRESSED
							)).as_raw());
						}
						Action::KeyUp(key) => {
							events.push(*InputEvent::from(KeyEvent::new(
								ZERO,
								KEY_TO_UINPUT[key as usize],
								KeyState::RELEASED
							)).as_raw());
						}
						_ => {}
					}
					if !events.is_empty() {
						events.push(*InputEvent::from(SynchronizeEvent::new(ZERO, SynchronizeKind::Report, 0)).as_raw());
						match uhandle.write(events.as_slice()) {
							Ok(_) => {},
							Err(e) => {
								println!("{:?}", e);
							}
						}
					}
				}
				Err(e) => {
					println!("{:?}", e);
					break;
				}
			}
		};
	}

	uhandle.dev_destroy()?;

	Ok(())
}
