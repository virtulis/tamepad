use std::f32::consts::{PI, TAU};
use std::num::NonZeroU32;
use std::sync::{Arc, RwLock};

use femtovg::{Canvas, Color, Paint, Path, Renderer, Solidity};
use femtovg::renderer::OpenGl;
use glutin::{
	config::ConfigTemplateBuilder, context::ContextAttributesBuilder, context::PossiblyCurrentContext,
	display::GetGlDisplay, prelude::*,
};
use glutin::display::Display;
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop, EventLoopBuilder};
use winit::window::{Window, WindowBuilder};

use crate::state::{CachedConfig, State};
use crate::types::{ButtonHandler, StateMapping};

#[derive(Debug, Clone)]
pub enum UIEvent {
	StateReset(Arc<CachedConfig>, Arc<RwLock<State>>),
	StateUpdated,
}

pub fn init_gui() -> (
	EventLoop<UIEvent>,
	PossiblyCurrentContext,
	Display,
	Window,
	Surface<WindowSurface>,
) {
	let event_loop = EventLoopBuilder::<UIEvent>::with_user_event().build().unwrap();
	let window_builder = WindowBuilder::new()
		.with_inner_size(winit::dpi::PhysicalSize::new(500, 500))
		.with_min_inner_size(winit::dpi::PhysicalSize::new(100, 100))
		.with_resizable(true)
		.with_transparent(true)
		.with_decorations(false)
		.with_title("Tamepad");

	let template = ConfigTemplateBuilder::new().with_alpha_size(8);

	let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

	let (window, gl_config) = display_builder
		.build(&event_loop, template, |mut configs| configs.next().unwrap())
		.unwrap();

	let window = window.unwrap();

	let gl_display = gl_config.display();

	let context_attributes = ContextAttributesBuilder::new().build(Some(window.raw_window_handle()));

	let mut not_current_gl_context =
		Some(unsafe { gl_display.create_context(&gl_config, &context_attributes).unwrap() });

	let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
		window.raw_window_handle(),
		NonZeroU32::new(500).unwrap(),
		NonZeroU32::new(500).unwrap(),
	);

	let surface = unsafe { gl_config.display().create_window_surface(&gl_config, &attrs).unwrap() };

	let current_context = not_current_gl_context.take().unwrap().make_current(&surface).unwrap();

	(event_loop, current_context, gl_display, window, surface)
}
pub fn gui_loop(
	event_loop: EventLoop<UIEvent>,
	context: PossiblyCurrentContext,
	gl_display: Display,
	window: Window,
	surface: Surface<WindowSurface>,
) -> Result<(), anyhow::Error> {
	let renderer = unsafe { OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s) as *const _) }
		.expect("Cannot create renderer");

	let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");

	canvas.add_font("assets/DejaVuSansMono.ttf").unwrap();

	let mut state_refs = None;

	event_loop
		.run(|event, _target| match event {
			Event::WindowEvent { window_id, event } => {
				// println!("{:?}", event);
				match event {
					WindowEvent::RedrawRequested => {
						println!("redraw? {:?}", state_refs.is_some());
						if let Some((config, state)) = &state_refs {
							render_gui(&context, &surface, &window, &mut canvas, &state, &config);
						}
					}
					_ => {}
				}
			}
			Event::UserEvent(uev) => match uev {
				UIEvent::StateReset(cfg, arc) => {
					state_refs.replace((cfg, arc));
					window.request_redraw();
				}
				UIEvent::StateUpdated => {
					window.request_redraw();
				}
			},
			_ => {}
		})
		.unwrap();

	Ok(())
}

fn render_gui<T: Renderer>(
	context: &PossiblyCurrentContext,
	surface: &Surface<WindowSurface>,
	window: &Window,
	canvas: &mut Canvas<T>,
	state: &Arc<RwLock<State>>,
	config: &Arc<CachedConfig>,
) {
	let size = window.inner_size();
	canvas.set_size(size.width, size.height, window.scale_factor() as f32);
	let scale = size.width.min(size.height) as f32 * window.scale_factor() as f32 / 1000.0;

	canvas.clear_rect(0, 0, canvas.width(), canvas.height(), Color::rgbaf(0., 0., 0., 0.));

	let mut bg = Path::new();
	bg.rounded_rect(0., 0., canvas.width() as f32, canvas.height() as f32, 100. * scale);
	canvas.fill_path(&bg, &Paint::color(Color::rgbaf(0.9, 0.9, 0.9, 0.5)));

	let btn_fill = Paint::color(Color::rgbaf(1., 1., 1., 1.));
	let btn_stroke = Paint::color(Color::rgbaf(0., 0., 0., 1.));
	let active_fill = Paint::color(Color::rgbaf(0.7, 0.5, 1.0, 1.));
	let in_combo_fill = Paint::color(Color::rgbaf(0.7, 0.5, 1.0, 0.3));
	let text_fill = Paint::color(Color::rgbaf(0., 0., 0., 1.));

	let draw_label = |canvas: &mut Canvas<T>, s: &str, x, y, x_off| {
		let m = canvas.measure_text(x, y, s, &text_fill).unwrap();
		let dy = y * scale + m.height() * 0.5;
		let dx = (x + x_off) * scale - if x_off >= 0. { 0. } else { m.width() };
		println!("{:.1} {:.1}", dx, dy);
		canvas.fill_text(dx, dy, s, &text_fill).unwrap();
	};

	let draw_handler_label = |canvas: &mut Canvas<T>, hdl: &ButtonHandler, x, y, x_off| {
		if let Some(s) = &hdl.label {
			draw_label(canvas, s, x, y, x_off);
		} else if let Some(m) = &hdl.map {
			let s = match m {
				StateMapping::Key(key) => key.into_static_str(),
				StateMapping::Overlay(name) => name.as_str(),
			};
			draw_label(canvas, s, x, y, x_off);
		}
	};

	{
		let state = state.read().unwrap();

		for c in &state.config.config.buttons {
			let btn = c.button;

			let mut p = Path::new();
			p.arc(
				c.draw.x * scale,
				c.draw.y * scale,
				c.draw.border_radius * scale,
				0.,
				TAU,
				Solidity::Solid,
			);
			canvas.fill_path(&p, &btn_fill);
			canvas.stroke_path(&p, &btn_stroke);

			let bs = &state.button_states[c.button as usize];
			if bs.down {
				let mut p = Path::new();
				p.arc(
					c.draw.x * scale,
					c.draw.y * scale,
					c.draw.fill_radius * scale,
					0.,
					TAU,
					Solidity::Solid,
				);
				canvas.fill_path(
					&p,
					if bs.in_combo.is_some() {
						&in_combo_fill
					} else {
						&active_fill
					},
				);
			}

			let hdl = state.find_button_handler(config, &btn);
			if let Some((_, hdl)) = hdl {
				draw_handler_label(canvas, hdl, c.draw.x, c.draw.y, c.draw.label_offset);
			}
		}

		for (c_idx, (_, c)) in state.config.config.combos.iter().enumerate() {
			let mut p = Path::new();
			p.arc(
				c.draw.x * scale,
				c.draw.y * scale,
				c.draw.border_radius * scale,
				0.,
				TAU,
				Solidity::Solid,
			);
			canvas.fill_path(&p, &btn_fill);
			canvas.stroke_path(&p, &btn_stroke);

			if state.combo_states[c_idx] {
				let mut p = Path::new();
				p.arc(
					c.draw.x * scale,
					c.draw.y * scale,
					c.draw.fill_radius * scale,
					0.,
					TAU,
					Solidity::Solid,
				);
				canvas.fill_path(&p, &active_fill);
			}

			let hdl = state.find_combo_handler(config, c_idx);
			if let Some((_, hdl)) = hdl {
				if let Some(s) = &hdl.label {
					draw_label(canvas, s, c.draw.x, c.draw.y, c.draw.label_offset);
				} else if let Some(m) = &hdl.map {
					let s = match m {
						StateMapping::Key(key) => key.into_static_str(),
						StateMapping::Overlay(name) => name.as_str(),
					};
					draw_label(canvas, s, c.draw.x, c.draw.y, c.draw.label_offset);
				}
			}
		}

		for (s_idx, c) in config.config.sticks.iter().enumerate() {
			let ss = &state.stick_states[s_idx];

			let x = c.draw.x * scale;
			let y = c.draw.y * scale;
			let r = c.draw.border_radius * scale;

			let mut p = Path::new();
			p.arc(x, y, r, 0., TAU, Solidity::Solid);
			canvas.fill_path(&p, &btn_fill);
			canvas.stroke_path(&p, &btn_stroke);

			let hdl = state.find_stick_handler(config, s_idx);
			if let Some((_, hdl)) = hdl {
				if let Some(ch) = &hdl.circle {
					for (sec_idx, sec) in ch.sectors.iter().enumerate() {
						let next = ch.sectors.get(sec_idx + 1).or(ch.sectors.get(0)).unwrap();

						let mut p = Path::new();
						p.move_to(
							x - r * sec.from_degrees.to_radians().cos() as f32,
							y - r * sec.from_degrees.to_radians().sin() as f32,
						);
						p.line_to(x, y);
						p.line_to(
							x - r * next.from_degrees.to_radians().cos() as f32,
							y - r * next.from_degrees.to_radians().sin() as f32,
						);
						p.close();
						p.arc(
							x,
							y,
							r,
							next.from_degrees.to_radians() as f32 + PI,
							sec.from_degrees.to_radians() as f32 + PI,
							Solidity::Solid,
						);
						if ss.sector.is_some_and(|a| a == sec_idx) {
							canvas.fill_path(&p, &active_fill);
						}
						canvas.stroke_path(&p, &btn_stroke);
					}
				}
			}
			
			let mut p = Path::new();
			p.arc(x, y, c.center.border_radius * scale, 0., TAU, Solidity::Solid);
			canvas.fill_path(&p, &btn_fill);
			canvas.stroke_path(&p, &btn_stroke);

			let px = x - ss.value as f32 * r * ss.degrees.to_radians().cos() as f32;
			let py = y - ss.value as f32 * r * ss.degrees.to_radians().sin() as f32;
			let mut p = Path::new();
			p.arc(px, py, c.point.border_radius * scale, 0., TAU, Solidity::Solid);
			canvas.fill_path(&p, &btn_fill);
			canvas.stroke_path(&p, &btn_stroke);

			if ss.sector.is_some() {
				let mut p = Path::new();
				p.arc(px, py, c.point.fill_radius * scale, 0., TAU, Solidity::Solid);
				canvas.fill_path(&p, &active_fill);
			}
		}
	}

	canvas.flush();
	surface.swap_buffers(context).expect("Could not swap buffers");
}
