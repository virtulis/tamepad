use std::collections::HashMap;

use fs_extra::file::write_all;
use indexmap::IndexMap;
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString, IntoStaticStr};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, JsonSchema, EnumIter, EnumString, IntoStaticStr)]
#[repr(usize)]
pub enum Button {
	
	A = 0,
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
	
	// These usually? behave as buttons, so emulate buttons
	TriggerLeft,
	TriggerRight,
	
}

impl Button {
	
	pub fn into_str(self) -> &'static str {
		self.into()
	}
	
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, JsonSchema)]
pub enum Axis {
	
	LeftX,
	LeftY,
	RightX,
	RightY,
	
	TriggerLeft,
	TriggerRight,
	
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, JsonSchema)]
pub enum Stick {
	Left,
	Right,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum InputEvent {
	ButtonDown(u32, Button),
	ButtonUp(u32, Button),
	Axis(u32, Axis, i16),
	Added(u32, String),
	Removed(u32),
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Action {
	KeyDown(Key),
	KeyUp(Key),
	AddOverlay(String),
	RemoveOverlay(String)
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum StateMapping {
	Key(Key),
	Overlay(String),
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ButtonCombo {
	pub timeout: u64,
	pub buttons: Vec<Button>, 
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ButtonHandler {
	pub map: Option<StateMapping>,
	pub down: Option<Action>,
	pub up: Option<Action>,
	pub label: Option<String>
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CircleHandler {
	pub min_value: i16,
	pub positions: Vec<CirclePosition>
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CirclePosition {
	pub from_degrees: f32,
	pub state: Option<StateMapping>,
	pub enter: Option<Action>,
	pub exit: Option<Action>,
	pub label: Option<String>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StickHandler {
	
	pub button_map: Option<StateMapping>,
	pub button_down: Option<Action>,
	pub button_up: Option<Action>,
	pub button_label: Option<String>,
	
	pub circle: Option<CircleHandler>
	
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Binding {
	Button {
		button: Button,
		#[serde(flatten)]
		handler: ButtonHandler
	},
	Combo {
		combo: String,
		#[serde(flatten)]
		handler: ButtonHandler
	},
	Stick {
		stick: Stick,
		#[serde(flatten)]
		handler: StickHandler
	},
	Axis { axis: Axis },
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Overlay {
	pub label: Option<String>,
	pub bindings: Vec<Binding>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GamepadConfig {
	pub combos: IndexMap<String, ButtonCombo>,
	pub overlays: IndexMap<String, Overlay>,
	pub base_overlay: String,
}

/// Copied from input-linux, mapped to it by name
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, JsonSchema, EnumIter)]
pub enum Key {
	
	Reserved = 0,
	
	Esc,
	Num1,
	Num2,
	Num3,
	Num4,
	Num5,
	Num6,
	Num7,
	Num8,
	Num9,
	Num0,
	Minus,
	Equal,
	Backspace,
	Tab,
	Q,
	W,
	E,
	R,
	T,
	Y,
	U,
	I,
	O,
	P,
	LeftBrace,
	RightBrace,
	Enter,
	LeftCtrl,
	A,
	S,
	D,
	F,
	G,
	H,
	J,
	K,
	L,
	Semicolon,
	Apostrophe,
	Grave,
	LeftShift,
	Backslash,
	Z,
	X,
	C,
	V,
	B,
	N,
	M,
	Comma,
	Dot,
	Slash,
	RightShift,
	KpAsterisk,
	LeftAlt,
	Space,
	CapsLock,
	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	NumLock,
	ScrollLock,
	Kp7,
	Kp8,
	Kp9,
	KpMinus,
	Kp4,
	Kp5,
	Kp6,
	KpPlus,
	Kp1,
	Kp2,
	Kp3,
	Kp0,
	KpDot,
	
	Unknown54,
	
	ZenkakuHankaku,
	NonUsBackslashAndPipe,
	F11,
	F12,
	Ro,
	Katakana,
	Hiragana,
	Henkan,
	KatakanaHiragana,
	Muhenkan,
	KpJpComma,
	KpEnter,
	RightCtrl,
	KpSlash,
	Sysrq,
	RightAlt,
	LineFeed,
	Home,
	Up,
	PageUp,
	Left,
	Right,
	End,
	Down,
	PageDown,
	Insert,
	Delete,
	Macro,
	
	Mute,
	VolumeDown,
	VolumeUp,
	/// SC System Power Down
	Power,
	KpEqual,
	KpPlusMinus,
	Pause,
	/// AL Compiz Scale (Expose)
	Scale,
	
	KpComma,
	/// KeyHangeul / KeyHanguel
	Hangul,
	// KeyHangeul = KeyHangul
	// KeyHanguel = KeyHangul
	Hanja,
	Yen,
	LeftMeta,
	RightMeta,
	Compose,
	
	/// AC Stop
	Stop,
	Again,
	/// AC Properties
	Props,
	/// AC Undo
	Undo,
	Front,
	/// AC Copy
	Copy,
	/// AC Open
	Open,
	/// AC Paste
	Paste,
	/// AC Search
	Find,
	/// AC Cut
	Cut,
	/// AL Integrated Help Center
	Help,
	/// Menu (show menu)
	Menu,
	/// AL Calculator
	Calc,
	Setup,
	/// SC System Sleep
	Sleep,
	/// System Wake Up
	Wakeup,
	/// AL Local Machine Browser
	File,
	SendFile,
	DeleteFile,
	Xfer,
	Prog1,
	Prog2,
	/// AL Internet Browser
	WWW,
	MSDOS,
	/// AL Terminal Lock/Screensaver
	/// KeyScreenLock
	Coffee,
	// KeyScreenLock,
	/// Display orientation for e.g. tablets (aka KeyDirectionKey)
	RotateDisplay,
	// KeyDirectionKey = KeyRotateDisplay
	CycleWindows,
	Mail,
	/// AC Bookmarks
	Bookmarks,
	Computer,
	/// AC Back
	Back,
	/// AC Forward
	Forward,
	CloseCD,
	EjectCD,
	EjectCloseCD,
	NextSong,
	PlayPause,
	PreviousSong,
	StopCD,
	Record,
	Rewind,
	/// Media Select Telephone
	Phone,
	Iso,
	/// AL Consumer Control Configuration
	Config,
	/// AC Home
	Homepage,
	/// AC Refresh
	Refresh,
	/// AC Exit
	Exit,
	Move,
	Edit,
	ScrollUp,
	ScrollDown,
	KpLeftParen,
	KpRightParen,
	/// AC New
	New,
	/// AC Redo/Repeat
	Redo,
	
	F13,
	F14,
	F15,
	F16,
	F17,
	F18,
	F19,
	F20,
	F21,
	F22,
	F23,
	F24,
	
	PlayCD,
	PauseCD,
	Prog3,
	Prog4,
	/// AC Desktop Show All Applications
	AllApplications,
	Suspend,
	/// AC Close
	Close,
	Play,
	FastForward,
	BassBoost,
	/// AC Print
	Print,
	Hp,
	Camera,
	Sound,
	Question,
	Email,
	Chat,
	Search,
	Connect,
	/// AL Checkbook/Finance
	Finance,
	Sport,
	Shop,
	Alterase,
	/// AC Cancel
	Cancel,
	BrightnessDown,
	BrightnessUp,
	Media,
	
	/// Cycle between available video outputs (Monitor/LCD/TV-out/etc)
	SwitchVideoMode,
	IllumToggle,
	IllumDown,
	IllumUp,
	
	/// AC Send
	Send,
	/// AC Reply
	Reply,
	/// AC Forward Msg
	ForwardMail,
	/// AC Save
	Save,
	Documents,
	
	Battery,
	
	Bluetooth,
	WLAN,
	UWB,
	
	Unknown,
	
	/// drive next video source
	VideoNext,
	/// drive previous video source
	VideoPrev,
	/// brightness up, after max is min
	BrightnessCycle,
	/// Set Auto Brightness: manual brightness control is off, rely on ambient
	/// (aka KeyBrightnessZero)
	BrightnessAuto,
	// KeyBrightnessZero = KeyBrightnessAuto
	/// display device to off state
	DisplayOff,
	
	/// Wireless WAN (LTE, UMTS, GSM, etc.)
	/// (aka KeyWiMAX)
	WWAN,
	// KeyWiMAX = KeyWWAN
	/// Key that controls all radios
	Rfkill,
	
	/// Mute / unmute the microphone
	MicMute,
	
	//ButtonMisc,
	Button0,
	Button1,
	Button2,
	Button3,
	Button4,
	Button5,
	Button6,
	Button7,
	Button8,
	Button9,
	
	//ButtonMouse,
	ButtonLeft,
	ButtonRight,
	ButtonMiddle,
	ButtonSide,
	ButtonExtra,
	ButtonForward,
	ButtonBack,
	ButtonTask,
	
	//ButtonJoystick,
	ButtonTrigger,
	ButtonThumb,
	ButtonThumb2,
	ButtonTop,
	ButtonTop2,
	ButtonPinkie,
	ButtonBase,
	ButtonBase2,
	ButtonBase3,
	ButtonBase4,
	ButtonBase5,
	ButtonBase6,
	
	ButtonDead,
	
	//ButtonGamepad,
	/// aka ButtonA
	ButtonSouth,
	// ButtonA = ButtonSouth
	/// aka ButtonB
	ButtonEast,
	// ButtonB = ButtonEast
	ButtonC,
	/// aka ButtonX
	ButtonNorth,
	// ButtonX = ButtonNorth
	/// aka ButtonY
	ButtonWest,
	// ButtonY = ButtonWest
	ButtonZ,
	ButtonTL,
	ButtonTR,
	ButtonTL2,
	ButtonTR2,
	ButtonSelect,
	ButtonStart,
	ButtonMode,
	ButtonThumbl,
	ButtonThumbr,
	
	Unknown13F,
	
	//ButtonDigi,
	ButtonToolPen,
	ButtonToolRubber,
	ButtonToolBrush,
	ButtonToolPencil,
	ButtonToolAirbrush,
	ButtonToolFinger,
	ButtonToolMouse,
	ButtonToolLens,
	/// Five fingers on trackpad
	ButtonToolQuintTap,
	ButtonStylus3,
	ButtonTouch,
	ButtonStylus,
	ButtonStylus2,
	ButtonToolDoubleTap,
	ButtonToolTripleTap,
	/// Four fingers on trackpad
	ButtonToolQuadtap,
	
	ButtonWheel,
	//ButtonGearDown,
	ButtonGearUp,
	
	Ok,
	Select,
	Goto,
	Clear,
	Power2,
	Option,
	/// AL OEM Features/Tips/Tutorial
	Info,
	Time,
	Vendor,
	Archive,
	/// Media Select Program Guide
	Program,
	Channel,
	Favorites,
	EPG,
	/// Media Select Home
	PVR,
	MHP,
	Language,
	Title,
	Subtitle,
	Angle,
	FullScreen,
	Mode,
	Keyboard,
	AspectRatio,
	/// Media Select Computer
	PC,
	/// Media Select TV
	TV,
	/// Media Select Cable
	TV2,
	/// Media Select VCR
	VCR,
	/// VCR Plus
	VCR2,
	/// Media Select Satellite
	Sat,
	Sat2,
	/// Media Select CD
	CD,
	/// Media Select Tape
	Tape,
	Radio,
	/// Media Select Tuner
	Tuner,
	Player,
	Text,
	/// Media Select DVD
	Dvd,
	Aux,
	Mp3,
	/// AL Audio Browser
	Audio,
	/// AL Movie Browser
	Video,
	Directory,
	List,
	/// Media Select Messages
	Memo,
	Calendar,
	Red,
	Green,
	Yellow,
	Blue,
	/// Channel Increment
	ChannelUp,
	/// Channel Decrement
	ChannelDown,
	First,
	/// Recall Last
	Last,
	Ab,
	Next,
	Restart,
	Slow,
	Shuffle,
	Break,
	Previous,
	Digits,
	Teen,
	Twen,
	/// Media Select Video Phone
	Videophone,
	/// Media Select Games
	Games,
	/// AC Zoom In
	ZoomIn,
	/// AC Zoom Out
	ZoomOut,
	/// AC Zoom
	ZoomReset,
	/// AL Word Processor
	WordProcessor,
	/// AL Text Editor
	Editor,
	/// AL Spreadsheet
	Spreadsheet,
	/// AL Graphics Editor
	GraphicsEditor,
	/// AL Presentation App
	Presentation,
	/// AL Database App
	Database,
	/// AL Newsreader
	News,
	/// AL Voicemail
	Voicemail,
	/// AL Contacts/Address Book
	AddressBook,
	/// AL Instant Messaging
	Messenger,
	/// Turn display (LCD) on and off (aka KeyBrightnessToggle)
	DisplayToggle,
	// KeyBrightnessToggle = KeyDisplayToggle
	/// AL Spell Check
	SpellCheck,
	/// AL Logoff
	Logoff,
	
	Dollar,
	Euro,
	
	/// Consumer - transport controls
	FrameBack,
	FrameForward,
	/// GenDesc - system context menu
	ContextMenu,
	/// Consumer - transport control
	MediaRepeat,
	/// 10 channels up (10+)
	TenChannelsUp,
	/// 10 channels down (10-)
	TenChannelsDown,
	/// AL Image Browser
	Images,
	
	DelEol,
	DelEos,
	InsLine,
	DelLine,
	
	Fn,
	FnEsc,
	FnF1,
	FnF2,
	FnF3,
	FnF4,
	FnF5,
	FnF6,
	FnF7,
	FnF8,
	FnF9,
	FnF10,
	FnF11,
	FnF12,
	Fn1,
	Fn2,
	FnD,
	FnE,
	FnF,
	FnS,
	FnB,
	
	BrlDot1,
	BrlDot2,
	BrlDot3,
	BrlDot4,
	BrlDot5,
	BrlDot6,
	BrlDot7,
	BrlDot8,
	BrlDot9,
	BrlDot10,
	
	/// used by phones, remote controls,
	Numeric0,
	/// and other keypads
	Numeric1,
	Numeric2,
	Numeric3,
	Numeric4,
	Numeric5,
	Numeric6,
	Numeric7,
	Numeric8,
	Numeric9,
	NumericStar,
	NumericPound,
	/// Phone key A - HUT Telephony 0xb9
	NumericA,
	NumericB,
	NumericC,
	NumericD,
	
	CameraFocus,
	/// WiFi Protected Setup key
	WpsButton,
	
	/// Request switch touchpad on or off
	TouchpadToggle,
	TouchpadOn,
	TouchpadOff,
	
	CameraZoomin,
	CameraZoomout,
	CameraUp,
	CameraDown,
	CameraLeft,
	CameraRight,
	
	AttendantOn,
	AttendantOff,
	/// Attendant call on or off
	AttendantToggle,
	/// Reading light on or off
	LightsToggle,
	
	ButtonDpadUp,
	ButtonDpadDown,
	ButtonDpadLeft,
	ButtonDpadRight,
	
	/// Ambient light sensor
	AlsToggle,
	RotateLockToggle,
	
	/// AL Button Configuration
	ButtonConfig,
	/// AL Task/Project Manager
	TaskManager,
	/// AL Log/Journal/Timecard
	Journal,
	/// AL Control Panel
	ControlPanel,
	/// AL Select Task/Application
	AppSelect,
	/// AL Screen Saver
	Screensaver,
	/// Listening Voice Command
	Voicecommand,
	/// AL Context-aware desktop assistant
	Assistant,
	/// AC Next Keyboard Layout Select
	KbdLayoutNext,
	/// Show/hide emoji picker (HUTRR101)
	EmojiPicker,
	/// Start or Stop Voice Dictation Session (HUTRR99)
	Dictate,
	/// Enables programmatic access to camera devices. (HUTRR72)
	CameraAccessEnable,
	/// Disables programmatic access to camera devices. (HUTRR72)
	CameraAccessDisable,
	/// Toggles the current state of the camera access control. (HUTRR72)
	CameraAccessToggle,
	
	/// Set Brightness to Minimum
	BrightnessMin,
	/// Set Brightness to Maximum
	BrightnessMax,
	
	InputAssistPrev,
	InputAssistNext,
	InputAssistPrevGroup,
	InputAssistNextGroup,
	InputAssistAccept,
	InputAssistCancel,
	
	/// Diagonal movement keys
	RightUp,
	RightDown,
	LeftUp,
	LeftDown,
	
	/// Show Device's Root Menu
	RootMenu,
	/// Show Top Menu of the Media (e.g. DVD)
	MediaTopMenu,
	Numeric11,
	Numeric12,
	
	/// Toggle Audio Description: refers to an audio service that helps blind and
	/// visually impaired consumers understand the action in a program. Note: in
	/// some countries this is referred to as "Video Description".
	
	AudioDesc,
	Audio3dMode,
	NextFavorite,
	StopRecord,
	PauseRecord,
	/// Video on Demand
	Vod,
	Unmute,
	FastReverse,
	SlowReverse,
	
	/// Control a data application associated with the currently viewed channel,
	/// e.g. teletext or data broadcast application (MHEG, MHP, HbbTV, etc.)
	
	Data,
	OnscreenKeyboard,
	PrivacyScreenToggle,
	SelectiveScreenshot,
	
	/// Move the focus to the next user controllable element within a UI container
	NextElement,
	/// Move the focus to the previous user controllable element within a UI container
	PreviousElement,
	
	/// Toggle Autopilot engagement
	AutopilotEngageToggle,
	
	/// Marine navigation shortcut key
	MarkWaypoint,
	/// Marine navigation shortcut key
	SOS,
	/// Marine navigation shortcut key
	NavChart,
	/// Marine navigation shortcut key
	FishingChart,
	/// Marine navigation shortcut key
	SingleRangeRadar,
	/// Marine navigation shortcut key
	DualRangeRadar,
	/// Marine navigation shortcut key
	RadarOverlay,
	/// Marine navigation shortcut key
	TraditionalSonar,
	/// Marine navigation shortcut key
	ClearVuSonar,
	/// Marine navigation shortcut key
	SideVuSonar,
	/// Marine navigation shortcut key
	NavInfo,
	/// Marine navigation shortcut key
	BrightnessMenu,
	
	KbdLcdMenu1,
	KbdLcdMenu2,
	KbdLcdMenu3,
	KbdLcdMenu4,
	KbdLcdMenu5,
	
}

pub fn write_schema() {
	
	let schema = schema_for!(GamepadConfig);
	let str  = format!("{}", serde_json::to_string_pretty(&schema).unwrap());
	let str = str.replace("  ", "\t"); // idfk
	write_all("configs/schema.json", &str).unwrap();
	
}
