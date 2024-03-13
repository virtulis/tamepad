use std::fs::read_to_string;
use std::ops::Index;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use anyhow::anyhow;
use crossbeam_channel::select;
use indexmap::IndexMap;
use strum::IntoEnumIterator;

use crate::types::{
	Action, Binding, Button, ButtonCombo, ButtonHandler, GamepadConfig, InputEvent, Overlay, StateMapping,
};

#[derive(Debug)]
struct CachedOverlay<'a> {
	overlay: &'a Overlay,
	buttons: IndexMap<Button, &'a ButtonHandler>,
	combos: IndexMap<usize, &'a ButtonHandler>,
}
#[derive(Default, Debug)]
struct ButtonComboList<'a> {
	max_timeout: u64,
	combos: Vec<(usize, &'a ButtonCombo)>,
}

#[derive(Debug, Clone)]
struct ButtonState {
	down: bool,
	handled: bool,
	in_combo: Option<usize>,
	handle_at: Option<Instant>,
}

impl Default for ButtonState {
	fn default() -> Self {
		Self {
			down: false,
			handled: true,
			in_combo: None,
			handle_at: None,
		}
	}
}

#[derive(Debug)]
struct State<'a> {
	pub config: &'a GamepadConfig,
	pub overlays: IndexMap<String, CachedOverlay<'a>>,
	pub current_overlays: Vec<usize>,
	pub button_combos: IndexMap<Button, ButtonComboList<'a>>,
	pub button_states: Vec<ButtonState>,
	pub combo_states: Vec<bool>,
}

pub fn state_task(
	events: crossbeam_channel::Receiver<InputEvent>,
	action_sender: crossbeam_channel::Sender<Action>,
) -> Result<(), anyhow::Error> {
	let config = read_to_string("configs/default.json5")?;
	let config: GamepadConfig = json5::from_str(&config)?;

	println!("{:?}", &config);

	let overlays = IndexMap::from_iter(config.overlays.iter().map(|(id, overlay)| {
		let buttons = IndexMap::from_iter(overlay.bindings.iter().filter_map(|b| match b {
			Binding::Button { button, handler } => Some((*button, handler)),
			_ => None,
		}));
		let combos = IndexMap::from_iter(overlay.bindings.iter().filter_map(|b| match b {
			Binding::Combo { combo, handler } => {
				let idx = config.combos.get_index_of(combo);
				match idx {
					Some(idx) => Some((idx, handler)),
					None => {
						println!("Combo not found: {}", combo);
						None
					}
				}
			}
			_ => None,
		}));
		(
			id.clone(),
			CachedOverlay {
				overlay,
				buttons,
				combos,
			},
		)
	}));

	let mut current_overlays = Vec::new();
	let base_overlay = overlays
		.get_index_of(&config.base_overlay)
		.ok_or(anyhow!("Base overlay '{}' not found", &config.base_overlay))?;
	current_overlays.push(base_overlay);

	let mut button_combos = IndexMap::new();
	for (idx, combo) in config.combos.values().enumerate() {
		for btn in &combo.buttons {
			if !button_combos.contains_key(btn) {
				button_combos.insert(*btn, ButtonComboList::default());
			}
			let list = button_combos.get_mut(btn).unwrap();
			list.max_timeout = combo.timeout.max(list.max_timeout);
			list.combos.push((idx, combo));
		}
	}

	println!("button_combos = {:?}", &button_combos);

	let button_states = vec![ButtonState::default(); Button::iter().len()];
	let combo_states = vec![false; Button::iter().len()];

	let state_arc = Arc::new(RwLock::new(State {
		config: &config,
		overlays,
		current_overlays,
		button_combos,
		button_states,
		combo_states,
	}));

	let print_state = |state: &State| {
		let str = Button::iter()
			.map(|btn| {
				let state = &state.button_states[btn as usize];
				let name: &'static str = btn.into();
				format!(
					"{} {}",
					name,
					if !state.down {
						"."
					} else if !state.handled {
						"d"
					} else {
						"D"
					}
				)
			})
			.collect::<Vec<String>>()
			.join("  ");
		println!("{}", str);
	};

	let add_overlay = |state: &mut State, name: &String| {
		println!("add_overlay {:?}", name);
		let idx = config.overlays.get_index_of(name);
		match idx {
			Some(idx) => {
				if !state.current_overlays.contains(&idx) {
					state.current_overlays.push(idx);
				}
			}
			None => {
				println!("Unknown overlay {}", name);
			}
		}
	};
	let remove_overlay = |state: &mut State, name: &String| {
		println!("remove_overlay {:?}", name);
		let idx = config.overlays.get_index_of(name);
		match idx {
			Some(idx) => {
				state.current_overlays.retain(|m| *m != idx);
			}
			None => {
				println!("Unknown overlay {}", name);
			}
		}
	};

	let trigger_handler = |state: &mut State, handler: &ButtonHandler, down: bool| {
		println!("trigger_handler {:?}", handler);
		if let Some(map) = &handler.map {
			match map {
				StateMapping::Key(key) => {
					action_sender
						.send(match down {
							true => Action::KeyDown(*key),
							false => Action::KeyUp(*key),
						})
						.unwrap();
				}
				StateMapping::Overlay(name) => {
					if down {
						add_overlay(state, name);
					} else {
						remove_overlay(state, name);
					}
				}
			}
		}
	};

	let combo_up = |state: &mut State, idx: usize| {
		let combo = state.config.combos.index(idx);
		println!("combo up {:?}", combo);
		state.combo_states[idx] = false;
		for btn in &combo.buttons {
			let s = &mut state.button_states[*btn as usize];
			s.down = false;
			s.handled = true;
			s.handle_at = None;
			s.in_combo = None;
		}
		let handler = state
			.current_overlays
			.iter()
			.rev()
			.find_map(|oidx| state.overlays.index(*oidx).combos.get(&idx));
		if let Some(handler) = handler {
			trigger_handler(state, handler, false);
		}
	};

	let check_combos = |state: &mut State, btn: Button| {
		let combos = state.button_combos.get(&btn);
		if let Some(combos) = combos {
			let combo = combos.combos.iter().find(|(idx, combo)| {
				let all_down = combo.buttons.iter().all(|btn| state.button_states[*btn as usize].down);
				return all_down && !state.combo_states[*idx];
			});
			if let Some((idx, combo)) = combo {
				println!("combo down {:?}", combo);
				state.combo_states[*idx] = true;
				for btn in &combo.buttons {
					let s = &mut state.button_states[*btn as usize];
					s.handled = true;
					s.handle_at = None;
					s.in_combo = Some(*idx);
				}
				let handler = state
					.current_overlays
					.iter()
					.rev()
					.find_map(|oidx| state.overlays.index(*oidx).combos.get(idx));
				if let Some(handler) = handler {
					trigger_handler(state, handler, true);
				}
				return true;
			}
		}
		false
	};

	let do_handle_button = |mut state: &mut State, btn: Button, down: bool| {
		println!("do_handle_button {} {}", btn.into_str(), down);
		let btn_state = state.button_states.get_mut(btn as usize).unwrap();
		btn_state.handled = true;
		btn_state.handle_at = None;
		let handler = state
			.current_overlays
			.iter()
			.rev()
			.find_map(|oidx| state.overlays.index(*oidx).buttons.get(&btn));
		if let Some(handler) = handler {
			trigger_handler(state, handler, down);
		}
	};

	let maybe_handle_button = |mut state: &mut State, btn: Button, down: bool| {
		let prev_state = state.button_states.get(btn as usize).unwrap();
		if prev_state.down == down {
			return;
		}

		if !prev_state.handled {
			do_handle_button(state, btn, !down);
		}

		let prev_state = state.button_states.get(btn as usize).unwrap().clone();

		{
			let s = state.button_states.get_mut(btn as usize).unwrap();
			s.down = down;
			s.handled = false;
		}

		if let Some(idx) = prev_state.in_combo {
			combo_up(state, idx);
		} else if down && state.button_combos.contains_key(&btn) {
			let handled = check_combos(state, btn);
			if !handled {
				let combos = state.button_combos.get(&btn).unwrap();
				let s = state.button_states.get_mut(btn as usize).unwrap();
				s.handle_at = Some(Instant::now() + Duration::from_millis(combos.max_timeout));
			}
		} else {
			do_handle_button(state, btn, down);
		}
	};

	loop {
		let next = {
			let state = state_arc.read().unwrap();
			print_state(&state);
			state.button_states.iter().filter_map(|bs| bs.handle_at).min()
		};
		let timeout = if let Some(at) = next {
			at - Instant::now()
		} else {
			Duration::from_secs(60)
		};
		select! {
			recv(events) -> ev => {
				let ev = ev?;
				println!("{:?}", ev);
				let mut s = state_arc.write().unwrap();
				match ev {
					InputEvent::ButtonDown(_, btn) => {
						maybe_handle_button(&mut s, btn, true);
					}
					InputEvent::ButtonUp(_, btn) => {
						maybe_handle_button(&mut s, btn, false);
					}
					_ => {}
				}
			}
			default(timeout) => {
				println!("timeout {:?}", timeout);
				let mut s = state_arc.write().unwrap();
				let now = Instant::now();
				for btn in Button::iter() {
					let bs = &s.button_states[btn as usize];
					if bs.handled || !bs.handle_at.is_some_and(|at| at <= now) { continue }
					let down = bs.down;
					do_handle_button(&mut s, btn, down);
				}
			}
		}
	}
}
