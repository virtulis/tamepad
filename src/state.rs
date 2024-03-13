use std::cell::RefCell;
use std::fs::read_to_string;
use std::time::{Duration, Instant};

use anyhow::anyhow;
use crossbeam_channel::select;
use indexmap::IndexMap;
use strum::IntoEnumIterator;

use crate::types::{Action, Button, ButtonCombo, GamepadConfig, InputEvent};

#[derive(Default, Debug)]
struct ButtonComboList<'a> {
	max_timeout: u64,
	combos: Vec<(String, &'a ButtonCombo)>,
}

#[derive(Default, Debug, Clone)]
struct ButtonState {
	down: bool,
	handled: bool,
	handle_at: Option<Instant>,
}

pub fn state_task(
	events: crossbeam_channel::Receiver<InputEvent>,
	action_sender: crossbeam_channel::Sender<Action>,
) -> Result<(), anyhow::Error> {
	
	let config = read_to_string("configs/default.json5")?;
	let config: GamepadConfig = json5::from_str(&config)?;
	
	println!("{:?}", &config);
	
	let mut current_overlays = Vec::new();
	let base_overlay = config.overlays.get(&config.base_overlay)
		.ok_or(anyhow!("Base overlay '{}' not found", &config.base_overlay))?;
	current_overlays.push(base_overlay);
	
	let mut button_combos = IndexMap::new();
	for (key, combo) in &config.combos {
		for btn in &combo.buttons {
			if !button_combos.contains_key(&btn) {
				button_combos.insert(btn, ButtonComboList::default());
			}
			let list = button_combos.get_mut(&btn).unwrap();
			list.max_timeout = combo.timeout.max(list.max_timeout);
			list.combos.push((key.clone(), combo));
		}
	}
	
	println!("button_combos = {:?}", &button_combos);
	
	let mut button_states = RefCell::new(vec![
		ButtonState::default();
		Button::iter().len()
	]);
	
	let print_button_states = || {
		let states = button_states.borrow();
		let str = Button::iter().enumerate().map(|(i, btn)| {
			let state = &states[i];
			let name: &'static str = btn.into();
			format!(
				"{} {}",
				name,
				if !state.down { "." }
				else if !state.handled { "d" }
				else { "D" }
			)
		}).collect::<Vec<String>>().join("  ");
		println!("{}", str);
	};
	
	let handle_button = |btn: Button, down: bool, state: &mut ButtonState| {
		println!("handle_button {}", btn.into_str());
		state.down = down;
		if let Some(combos) = button_combos.get(&btn) {
			state.handled = false;
			state.handle_at = Some(Instant::now() + Duration::from_millis(combos.max_timeout));
		}
		else {
			state.handled = true;
		}
	};
	
	loop {
		print_button_states();
		let _ = select! {
			recv(events) -> ev => {
				let ev = ev?;
				println!("{:?}", ev);
				match ev {
					InputEvent::ButtonDown(_, btn) => {
						let mut states = button_states.borrow_mut();
						let state = states.get_mut(btn as usize).unwrap();
						handle_button(btn, true, state);
					}
					InputEvent::ButtonUp(_, btn) => {
						let mut states = button_states.borrow_mut();
						let state = states.get_mut(btn as usize).unwrap();
						handle_button(btn, false, state);
					}
					_ => {}
				}
			}
		};
	}
	
	Ok(())
	
}
