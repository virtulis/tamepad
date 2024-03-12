#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Button {
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
	Touchpad,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Axis {
	LeftX,
	LeftY,
	RightX,
	RightY,
	TriggerLeft,
	TriggerRight,
}

#[derive(Clone, Debug)]
pub enum InputEvent {
	ButtonDown(u32, Button),
	ButtonUp(u32, Button),
	Axis(u32, Axis, i16),
	Added(u32, String),
	Removed(u32),
}

#[derive(Clone, Debug)]
pub enum Action {
	KeyDown(input_linux::Key),
	KeyUp(input_linux::Key)
}
