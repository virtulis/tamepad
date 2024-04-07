use std::fs::read_to_string;
use std::ops::Index;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use anyhow::anyhow;
use crossbeam_channel::select;
use indexmap::IndexMap;
use strum::IntoEnumIterator;
use winit::event_loop::EventLoopProxy;

use crate::gui::UIEvent;
use crate::types::{
	Action, Axis, Binding, Button, ButtonCombo, ButtonHandler, GamepadConfig, InputEvent, MainEvent, Overlay,
	StateMapping, StickHandler,
};

#[derive(Debug)]
struct CachedOverlay {
	overlay: Overlay,
	buttons: IndexMap<Button, ButtonHandler>,
	combos: IndexMap<usize, ButtonHandler>,
	sticks: IndexMap<usize, StickHandler>,
}
#[derive(Default, Debug)]
struct ButtonComboList {
	max_timeout: u64,
	combos: Vec<(usize, ButtonCombo)>,
}

#[derive(Debug, Clone)]
pub struct ButtonState {
	pub down: bool,
	handled: bool,
	pub in_combo: Option<usize>,
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

#[derive(Default, Debug, Clone)]
struct AxisState {
	value: f64,
}

#[derive(Default, Debug, Clone)]
pub struct StickState {
	pub degrees: f64,
	pub value: f64,
	pub sector: Option<usize>,
}

#[derive(Debug)]
pub struct CachedConfig {
	pub config: GamepadConfig,
	pub overlays: IndexMap<String, CachedOverlay>,
}

#[derive(Debug)]
pub struct State {
	pub config: Arc<CachedConfig>,
	pub current_overlays: Vec<usize>,
	pub button_combos: IndexMap<Button, ButtonComboList>,
	pub button_states: Vec<ButtonState>,
	pub combo_states: Vec<bool>,
	pub axis_states: Vec<AxisState>,
	pub stick_states: Vec<StickState>,
	active_maps: Vec<(usize, StateMapping)>,
}

impl State {
	pub fn find_button_handler<'a>(
		&self,
		config: &'a CachedConfig,
		btn: &Button,
	) -> Option<(usize, &'a ButtonHandler)> {
		self.current_overlays
			.iter()
			.rev()
			.find_map(|oidx| config.overlays.index(*oidx).buttons.get(btn).map(|h| (*oidx, h)))
	}

	pub fn find_combo_handler<'a>(&self, config: &'a CachedConfig, idx: usize) -> Option<(usize, &'a ButtonHandler)> {
		self.current_overlays
			.iter()
			.rev()
			.find_map(|oidx| config.overlays.index(*oidx).combos.get(&idx).map(|h| (*oidx, h)))
	}

	pub fn find_stick_handler<'a>(&self, config: &'a CachedConfig, idx: usize) -> Option<(usize, &'a StickHandler)> {
		self.current_overlays
			.iter()
			.rev()
			.find_map(|oidx| config.overlays.index(*oidx).sticks.get(&idx).map(|h| (*oidx, h)))
	}
}

pub fn state_task(
	events: crossbeam_channel::Receiver<InputEvent>,
	action_sender: crossbeam_channel::Sender<Action>,
	ui_event_proxy: EventLoopProxy<UIEvent>,
	main_events: crossbeam_channel::Receiver<MainEvent>,
) -> Result<(), anyhow::Error> {
	let config = read_to_string("configs/default.json5")?;
	let config: GamepadConfig = json5::from_str(&config)?;

	println!("{:?}", &config);

	let overlays: IndexMap<String, CachedOverlay> = IndexMap::from_iter(config.overlays.iter().map(|(id, overlay)| {
		let buttons = IndexMap::from_iter(overlay.bindings.iter().filter_map(|b| match b {
			Binding::Button { button, handler } => Some((*button, handler.clone())),
			_ => None,
		}));
		let combos = IndexMap::from_iter(overlay.bindings.iter().filter_map(|b| match b {
			Binding::Combo { combo, handler } => {
				let idx = config.combos.get_index_of(combo);
				match idx {
					Some(idx) => Some((idx, handler.clone())),
					None => {
						println!("Combo not found: {}", combo);
						None
					}
				}
			}
			_ => None,
		}));
		let sticks = IndexMap::from_iter(overlay.bindings.iter().filter_map(|b| match b {
			Binding::Stick { stick, handler } => {
				let mut handler = handler.clone();
				handler
					.circle
					.iter_mut()
					.for_each(|ch| ch.sectors.sort_by(|a, b| a.from_degrees.partial_cmp(&b.from_degrees).unwrap()));
				Some((*stick as usize, handler))
			}
			_ => None,
		}));
		(
			id.clone(),
			CachedOverlay {
				overlay: overlay.clone(),
				buttons,
				combos,
				sticks,
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
			list.combos.push((idx, combo.clone()));
		}
	}

	println!("button_combos = {:?}", &button_combos);

	let button_states = vec![ButtonState::default(); Button::iter().len()];
	let combo_states = vec![false; Button::iter().len()];
	let axis_states = vec![AxisState::default(); Axis::iter().len()];
	let stick_states = vec![StickState::default(); 2];

	let cached_config = Arc::new(CachedConfig { config, overlays });

	let state_arc = Arc::new(RwLock::new(State {
		config: cached_config.clone(),
		current_overlays,
		button_combos,
		button_states,
		combo_states,
		axis_states,
		stick_states,
		active_maps: Vec::new(),
	}));

	let print_state = |state: &State| {
		let str = Button::iter()
			.filter_map(|btn| {
				let state = &state.button_states[btn as usize];
				let name: &'static str = btn.into();
				if !state.down {
					return None;
				}
				Some(format!("{} {}", name, if !state.handled { "d" } else { "D" }))
			})
			.collect::<Vec<String>>()
			.join("  ");
		println!("{}", str);
	};

	let add_overlay = |state: &mut State, name: &String| {
		println!("add_overlay {:?}", name);
		let idx = cached_config.overlays.get_index_of(name);
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
		let idx = cached_config.overlays.get_index_of(name);
		match idx {
			Some(idx) => {
				state.current_overlays.retain(|m| *m != idx);
			}
			None => {
				println!("Unknown overlay {}", name);
			}
		}
	};

	let trigger_mapping = |state: &mut State, map: &StateMapping, down: bool, oidx| {
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
		if down {
			state.active_maps.push((oidx, map.clone()));
		} else {
			state.active_maps.retain(|(oidx, other)| *other != *map);
		}
	};

	let trigger_handler = |state: &mut State, handler: &ButtonHandler, down: bool, oidx| {
		// println!("trigger_handler {:?}", handler);
		if let Some(map) = &handler.map {
			trigger_mapping(state, &map, down, oidx);
		}
	};

	let combo_up = |state: &mut State, idx: usize| {
		let combo = cached_config.config.combos.index(idx);
		println!("combo up {:?}", combo);
		state.combo_states[idx] = false;
		for btn in &combo.buttons {
			let s = &mut state.button_states[*btn as usize];
			s.down = false;
			s.handled = true;
			s.handle_at = None;
			s.in_combo = None;
		}
		if let Some((oidx, handler)) = state.find_combo_handler(&cached_config, idx) {
			trigger_handler(state, &handler, false, oidx)
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
				if let Some((oidx, handler)) = state.find_combo_handler(&cached_config, *idx) {
					trigger_handler(state, &handler, true, oidx)
				}
			}
		}
		false
	};

	let do_handle_button = |mut state: &mut State, btn: Button, down: bool| {
		// println!("do_handle_button {} {}", btn.into_str(), down);
		let btn_state = state.button_states.get_mut(btn as usize).unwrap();
		btn_state.handled = true;
		btn_state.handle_at = None;
		if let Some((oidx, handler)) = state.find_button_handler(&cached_config, &btn) {
			trigger_handler(state, &handler, down, oidx)
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

	let update_stick = |state: &mut State, stick: usize, x: f64, y: f64| {
		let s = &mut state.stick_states[stick];
		let degrees = (y.atan2(x) + std::f64::consts::PI).to_degrees();
		let value = x.hypot(y);

		s.degrees = degrees;
		s.value = value;
		let prev_sector_idx = s.sector;

		// println!("stick {} {:>7.1} {:>6.2}", stick, degrees, value);

		if let Some((oidx, handler)) = state.find_stick_handler(&cached_config, stick) {
			if let Some(ch) = &handler.circle {
				let sector = if value > ch.min_value {
					ch.sectors
						.iter()
						.enumerate()
						.rfind(|(i, s)| s.from_degrees <= degrees)
						.or_else(|| ch.sectors.iter().enumerate().last())
				} else {
					None
				};
				let sector_idx = sector.map(|(i, _)| i);
				let sector = sector.map(|(_, s)| s);
				state.stick_states[stick].sector = sector_idx;

				if sector_idx != prev_sector_idx {
					// println!("stick {} {:>7.1} {:>6.2} {:?} {:?}", stick, degrees, value, sector_idx, sector);
					if let Some(psidx) = prev_sector_idx {
						let prev_sector = &ch.sectors[psidx];
						if let Some(map) = prev_sector.map.clone() {
							trigger_mapping(state, &map, false, oidx);
						}
					}
					if let Some(sector) = sector {
						if let Some(map) = sector.map.clone() {
							trigger_mapping(state, &map, true, oidx);
						}
					}
				}
			}
		}
	};
	// ff
	let update_axis = |state: &mut State, axis: Axis, value| {
		state.axis_states[axis as usize].value = value;
		if axis == Axis::LeftX || axis == Axis::LeftY {
			update_stick(
				state,
				0,
				state.axis_states[Axis::LeftX as usize].value,
				state.axis_states[Axis::LeftY as usize].value,
			);
		} else if axis == Axis::RightX || axis == Axis::RightY {
			update_stick(
				state,
				1,
				state.axis_states[Axis::RightX as usize].value,
				state.axis_states[Axis::RightY as usize].value,
			);
		}
		// println!("axis = {:?}", state.axis);
	};

	let mut state_sent = false;

	loop {
		if !state_sent {
			ui_event_proxy.send_event(UIEvent::StateReset(cached_config.clone(), state_arc.clone()))?;
			state_sent = true;
		} else {
			ui_event_proxy.send_event(UIEvent::StateUpdated)?;
		}

		let next = {
			let state = state_arc.read().unwrap();
			// print_state(&state);
			// println!("{:?}", state.active_maps);
			state.button_states.iter().filter_map(|bs| bs.handle_at).min()
		};

		let timeout = if let Some(at) = next {
			at - Instant::now()
		} else {
			Duration::from_secs(1)
		};

		select! {
			recv(main_events) -> _ev => {
				// currently just quit
				return Ok(());
			}
			recv(events) -> ev => {
				match ev {
					Ok(ev) => {
						// println!("{:?}", ev);
						let mut s = state_arc.write().unwrap();
						match ev {
							InputEvent::ButtonDown(_, btn) => {
								maybe_handle_button(&mut s, btn, true);
							}
							InputEvent::ButtonUp(_, btn) => {
								maybe_handle_button(&mut s, btn, false);
							}
							InputEvent::AxisMoved(_, axis, value) => {
								update_axis(&mut s, axis, (value as f64) / 32768.0);
							}
							_ => {}
						}
					},
					Err(_) => {
						return Ok(());
					}
				}
			}
			default(timeout) => {
				// println!("timeout {:?}", timeout);
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
