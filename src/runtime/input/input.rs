use {
	ecs::{
		Component,
		System,
		World,
	},
	engine::{
		Builder,
		Engine,
		Event,
		Module,
	},
	serde::{
		Deserialize,
		Serialize,
	},
	std::collections::HashMap,
};

use std::ops::DerefMut;

pub use os::input::*;

pub struct GameInput {
	event_queue: Vec<Event>,
}

impl Module for GameInput {
	fn new() -> Self {
		Self {
			event_queue: Vec::with_capacity(256),
		}
	}

	fn depends_on(builder: &mut Builder) -> &mut Builder {
		builder
			.register(InputManager::variant())
			.process_input(|event| {
				let input: &mut GameInput = unsafe { Engine::module_mut().unwrap() };
				input.event_queue.push(*event);
			})
	}
}

#[derive(Clone, Copy, Debug)]
pub enum InputState {
	Button(bool),
	Axis1D(f32),
}

impl InputState {
	pub fn button(self) -> bool {
		match self {
			Self::Button(b) => b,
			_ => unreachable!(),
		}
	}

	pub fn axis1d(self) -> f32 {
		match self {
			Self::Axis1D(x) => x,
			_ => unreachable!(),
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InputManager {
	#[serde(skip)]
	current: HashMap<Input, InputState>,
	#[serde(skip)]
	last: HashMap<Input, InputState>,
	#[serde(skip)]
	last_focus: bool,
	#[serde(skip)]
	focus: bool,
}

impl InputManager {
	pub fn is_button_down(&self, button: Input) -> bool {
		match self.current.get(&button) {
			Some(input_state) => input_state.button(),
			None => false,
		}
	}

	pub fn was_button_pressed(&self, button: Input) -> bool {
		let down = match self.current.get(&button) {
			Some(input_state) => input_state.button(),
			None => false,
		};
		let last = match self.last.get(&button) {
			Some(input_state) => input_state.button(),
			None => false,
		};

		!last && down
	}

	pub fn was_button_released(&self, button: Input) -> bool {
		let down = match self.current.get(&button) {
			Some(input_state) => input_state.button(),
			None => false,
		};
		let last = match self.last.get(&button) {
			Some(input_state) => input_state.button(),
			None => false,
		};

		last && !down
	}

	pub fn current_axis1d(&self, axis: Input) -> f32 {
		match self.current.get(&axis) {
			Some(input_state) => input_state.axis1d(),
			None => 0.0,
		}
	}

	pub fn last_axis1d(&self, axis: Input) -> f32 {
		match self.last.get(&axis) {
			Some(input_state) => input_state.axis1d(),
			None => 0.0,
		}
	}

	pub fn delta_axis1d(&self, axis: Input) -> f32 {
		self.current_axis1d(axis) - self.last_axis1d(axis)
	}

	pub fn has_focus(&self) -> bool {
		self.focus
	}

	pub fn lost_focus(&self) -> bool {
		!self.focus && self.last_focus
	}

	pub fn gained_focus(&self) -> bool {
		self.focus && !self.last_focus
	}
}

impl Default for InputManager {
	fn default() -> Self {
		Self {
			current: HashMap::with_capacity(256),
			last: HashMap::with_capacity(256),
			last_focus: true,
			focus: true,
		}
	}
}

impl Component for InputManager {}

#[derive(Clone)]
pub struct InputSystem;
impl System for InputSystem {
	fn run(&self, world: &World, _dt: f32) {
		let mut input_managers = world.write::<InputManager>();
		let mut input_manager = input_managers.get_mut_or_default(world.singleton);

		// Swap current state to last for new current state
		input_manager.last = input_manager.current.clone();
		input_manager.last_focus = input_manager.focus;

		let InputManager { current, focus, .. } = input_manager.deref_mut();

		// Reset all current axis input to 0
		for (_, value) in current.iter_mut() {
			if let InputState::Axis1D(x) = value {
				*x = 0.0;
			}
		}

		// Process input into state maps
		let input: &mut GameInput = unsafe { Engine::module_mut().unwrap() };
		for e in input.event_queue.drain(..) {
			match e {
				Event::Key { key, pressed } => match current.get_mut(&key) {
					Some(input_state) => {
						if let InputState::Button(b) = input_state {
							*b = pressed;
						} else {
							unreachable!();
						}
					}
					None => {
						current.insert(key, InputState::Button(pressed));
					}
				},
				Event::MouseButton {
					mouse_button,
					pressed,
				} => match current.get_mut(&mouse_button) {
					Some(input_state) => {
						if let InputState::Button(b) = input_state {
							*b = pressed;
						} else {
							unreachable!();
						}
					}
					None => {
						current.insert(mouse_button, InputState::Button(pressed));
					}
				},
				Event::MouseMotion(x, y) => {
					let mut set_motion = |input, value| match current.get_mut(&input) {
						Some(input_state) => {
							if let InputState::Axis1D(x) = input_state {
								*x = value;
							} else {
								unreachable!();
							}
						}
						None => {
							current.insert(input, InputState::Axis1D(value));
						}
					};
					if x != 0.0 {
						set_motion(os::MOUSE_AXIS_X, x);
					}
					if y != 0.0 {
						set_motion(os::MOUSE_AXIS_Y, y);
					}
				}
				Event::FocusLost => *focus = false,
				Event::FocusGained => *focus = true,
				_ => {}
			}
		}
	}
}
