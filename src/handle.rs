use crossbeam_channel::select;

use crate::types::InputEvent;

pub fn handler_task(events: crossbeam_channel::Receiver<InputEvent>) {
	loop {
		let _ = select! {
			recv(events) -> ev => match ev {
				Ok(ev) => {
					println!("{:?}", ev);
				}
				Err(e) => {
					println!("{:?}", e);
					break;
				}
			}
		};
	}
}
