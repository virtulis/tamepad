use strum::EnumString;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, EnumString)]
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, EnumString)]
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
	ButtonUp(u32, Button),
	ButtonDown(u32, Button),
	Axis(u32, Axis, i16),
	Added(u32, String),
	Removed(u32),
}
