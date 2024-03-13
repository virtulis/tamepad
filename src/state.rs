use crossbeam_channel::select;

use crate::types::{Action, Button, InputEvent, Key};

pub fn state_task(
	events: crossbeam_channel::Receiver<InputEvent>,
	action_sender: crossbeam_channel::Sender<Action>,
) -> Result<(), anyhow::Error> {
	loop {
		let _ = select! {
			recv(events) -> ev => {
				let ev = ev?;
				println!("{:?}", ev);
				match ev {
					InputEvent::ButtonDown(_, btn) => {
						if btn == Button::A {
							if action_sender.send(Action::KeyDown(Key::A)).is_err() { break; };
						}
					}
					InputEvent::ButtonUp(_, btn) => {
						if btn == Button::A {
							if action_sender.send(Action::KeyUp(Key::A)).is_err() { break };
						}
					}
					_ => {}
				}
			}
		};
	}
	Ok(())
}
