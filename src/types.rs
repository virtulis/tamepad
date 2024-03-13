use fs_extra::file::write_all;
use schemars::{JsonSchema, schema_for};
use serde::{Serialize, Deserialize};
use strum::EnumIter;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, JsonSchema)]
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
	
	// These usually? behave as buttons, so emulate buttons
	TriggerLeft,
	TriggerRight,
	
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

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum InputEvent {
	ButtonDown(u32, Button),
	ButtonUp(u32, Button),
	Axis(u32, Axis, i16),
	Added(u32, String),
	Removed(u32),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Action {
	KeyDown(Key),
	KeyUp(Key)
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum Binding {
	// Action(Action),
	Overlay(String),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, JsonSchema)]
pub enum Overlay {
	Button(Button),
	Axis(Axis)
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
	
	UnknownC3,
	UnknownC4,
	UnknownC5,
	UnknownC6,
	UnknownC7,
	
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
	
	UnknownF9,
	UnknownFA,
	UnknownFB,
	UnknownFC,
	UnknownFD,
	UnknownFE,
	
	/// Code 255 is reserved for special needs of AT keyboard driver
	ReservedFF,
	
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
	
	Unknown10A,
	Unknown10B,
	Unknown10C,
	Unknown10D,
	Unknown10E,
	Unknown10F,
	
	//ButtonMouse,
	ButtonLeft,
	ButtonRight,
	ButtonMiddle,
	ButtonSide,
	ButtonExtra,
	ButtonForward,
	ButtonBack,
	ButtonTask,
	
	Unknown118,
	Unknown119,
	Unknown11A,
	Unknown11B,
	Unknown11C,
	Unknown11D,
	Unknown11E,
	Unknown11F,
	
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
	
	Unknown12C,
	Unknown12D,
	Unknown12E,
	
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
	
	Unknown152,
	Unknown153,
	Unknown154,
	Unknown155,
	Unknown156,
	Unknown157,
	Unknown158,
	Unknown159,
	Unknown15A,
	Unknown15B,
	Unknown15C,
	Unknown15D,
	Unknown15E,
	Unknown15F,
	
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
	
	Unknown1BB,
	Unknown1BC,
	Unknown1BD,
	Unknown1BE,
	Unknown1BF,
	
	DelEol,
	DelEos,
	InsLine,
	DelLine,
	
	Unknown1C4,
	Unknown1C5,
	Unknown1C6,
	Unknown1C7,
	Unknown1C8,
	Unknown1C9,
	Unknown1CA,
	Unknown1CB,
	Unknown1CC,
	Unknown1CD,
	Unknown1CE,
	Unknown1CF,
	
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
	
	Unknown1E5,
	Unknown1E6,
	Unknown1E7,
	Unknown1E8,
	Unknown1E9,
	Unknown1EA,
	Unknown1EB,
	Unknown1EC,
	Unknown1ED,
	Unknown1EE,
	Unknown1EF,
	Unknown1F0,
	
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
	
	Unknown1FB,
	Unknown1FC,
	Unknown1FD,
	Unknown1FE,
	Unknown1FF,
	
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
	
	Unknown21F,
	
	ButtonDpadUp,
	ButtonDpadDown,
	ButtonDpadLeft,
	ButtonDpadRight,
	
	Unknown224,
	Unknown225,
	Unknown226,
	Unknown227,
	Unknown228,
	Unknown229,
	Unknown22A,
	Unknown22B,
	Unknown22C,
	Unknown22D,
	Unknown22E,
	Unknown22F,
	
	/// Ambient light sensor
	AlsToggle,
	RotateLockToggle,
	
	Unknown232,
	Unknown233,
	Unknown234,
	Unknown235,
	Unknown236,
	Unknown237,
	Unknown238,
	Unknown239,
	Unknown23A,
	Unknown23B,
	Unknown23C,
	Unknown23D,
	Unknown23E,
	Unknown23F,
	
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
	
	Unknown24E,
	Unknown24F,
	
	/// Set Brightness to Minimum
	BrightnessMin,
	/// Set Brightness to Maximum
	BrightnessMax,
	
	Unknown252,
	Unknown253,
	Unknown254,
	Unknown255,
	Unknown256,
	Unknown257,
	Unknown258,
	Unknown259,
	Unknown25A,
	Unknown25B,
	Unknown25C,
	Unknown25D,
	Unknown25E,
	Unknown25F,
	
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
	
	Unknown28A,
	Unknown28B,
	Unknown28C,
	Unknown28D,
	Unknown28E,
	Unknown28F,
	
	Macro1,
	Macro2,
	Macro3,
	Macro4,
	Macro5,
	Macro6,
	Macro7,
	Macro8,
	Macro9,
	Macro10,
	Macro11,
	Macro12,
	Macro13,
	Macro14,
	Macro15,
	Macro16,
	Macro17,
	Macro18,
	Macro19,
	Macro20,
	Macro21,
	Macro22,
	Macro23,
	Macro24,
	Macro25,
	Macro26,
	Macro27,
	Macro28,
	Macro29,
	Macro30,
	
	Unknown2AE,
	Unknown2AF,
	
	MacroRecordStart,
	MacroRecordStop,
	MacroPresetCycle,
	MacroPreset1,
	MacroPreset2,
	MacroPreset3,
	
	Unknown2B6,
	Unknown2B7,
	
	KbdLcdMenu1,
	KbdLcdMenu2,
	KbdLcdMenu3,
	KbdLcdMenu4,
	KbdLcdMenu5,
	
	Unknown2BD,
	Unknown2BE,
	Unknown2BF,
	
	ButtonTriggerHappy1,
	ButtonTriggerHappy2,
	ButtonTriggerHappy3,
	ButtonTriggerHappy4,
	ButtonTriggerHappy5,
	ButtonTriggerHappy6,
	ButtonTriggerHappy7,
	ButtonTriggerHappy8,
	ButtonTriggerHappy9,
	ButtonTriggerHappy10,
	ButtonTriggerHappy11,
	ButtonTriggerHappy12,
	ButtonTriggerHappy13,
	ButtonTriggerHappy14,
	ButtonTriggerHappy15,
	ButtonTriggerHappy16,
	ButtonTriggerHappy17,
	ButtonTriggerHappy18,
	ButtonTriggerHappy19,
	ButtonTriggerHappy20,
	ButtonTriggerHappy21,
	ButtonTriggerHappy22,
	ButtonTriggerHappy23,
	ButtonTriggerHappy24,
	ButtonTriggerHappy25,
	ButtonTriggerHappy26,
	ButtonTriggerHappy27,
	ButtonTriggerHappy28,
	ButtonTriggerHappy29,
	ButtonTriggerHappy30,
	ButtonTriggerHappy31,
	ButtonTriggerHappy32,
	ButtonTriggerHappy33,
	ButtonTriggerHappy34,
	ButtonTriggerHappy35,
	ButtonTriggerHappy36,
	ButtonTriggerHappy37,
	ButtonTriggerHappy38,
	ButtonTriggerHappy39,
	ButtonTriggerHappy40,
	
	Unknown2E8,
	Unknown2E9,
	Unknown2EA,
	Unknown2EB,
	Unknown2EC,
	Unknown2ED,
	Unknown2EE,
	Unknown2EF,
	
	Unknown2F0,
	Unknown2F1,
	Unknown2F2,
	Unknown2F3,
	Unknown2F4,
	Unknown2F5,
	Unknown2F6,
	Unknown2F7,
	Unknown2F8,
	Unknown2F9,
	Unknown2FA,
	Unknown2FB,
	Unknown2FC,
	Unknown2FD,
	Unknown2FE,
	Unknown2FF,
	
}

pub fn write_schema() {
	let schema = schema_for!(Overlay);
	let str  = format!("{}", serde_json::to_string_pretty(&schema).unwrap());
	write_all("configs/schema.json", &str).unwrap();
}
