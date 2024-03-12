use std::{fs::OpenOptions, io};
use std::os::unix::fs::OpenOptionsExt;

use crossbeam_channel::select;
use input_linux::{EventKind, EventTime, InputEvent, InputId, KeyEvent, KeyState, SynchronizeEvent, SynchronizeKind, UInputHandle};

use crate::types::Action;

pub fn actions_task(actions: crossbeam_channel::Receiver<Action>) -> io::Result<()> {
	
	let uinput_file = OpenOptions::new()
		.read(false)
		.write(true)
		.custom_flags(libc::O_NONBLOCK)
		.open("/dev/uinput")?;
	let uhandle = UInputHandle::new(uinput_file);
	
	uhandle.set_evbit(EventKind::Key)?;
	uhandle.set_keybit(input_linux::Key::A)?;
	
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
				Ok(ev) => {
					println!("{:?}", ev);
					let mut events = Vec::new();
					match ev {
						Action::KeyDown(key) => {
							events.push(*InputEvent::from(KeyEvent::new(
								ZERO,
								key,
								KeyState::PRESSED
							)).as_raw());
						}
						Action::KeyUp(key) => {
							events.push(*InputEvent::from(KeyEvent::new(
								ZERO,
								key,
								KeyState::RELEASED
							)).as_raw());
						}
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
