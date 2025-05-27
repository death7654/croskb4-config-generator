use winapi;

/*
Resources
https://github.com/coreboot/chrome-ec/blob/1b359bdd91da15ea25aaffd0d940ff63b9d72bc5/include/keyboard_8042_sharedlib.h#L116

    SCANCODE_F1 = 0x0005, /* Translates to 3b in codeset 1 */
    SCANCODE_F2 = 0x0006, /* Translates to 3c in codeset 1 */
    SCANCODE_F3 = 0x0004, /* Translates to 3d in codeset 1 */
    SCANCODE_F4 = 0x000c, /* Translates to 3e in codeset 1 */
    SCANCODE_F5 = 0x0003, /* Translates to 3f in codeset 1 */
    SCANCODE_F6 = 0x000b, /* Translates to 40 in codeset 1 */
    SCANCODE_F7 = 0x0083, /* Translates to 41 in codeset 1 */
    SCANCODE_F8 = 0x000a, /* Translates to 42 in codeset 1 */
    SCANCODE_F9 = 0x0001, /* Translates to 43 in codeset 1 */
    SCANCODE_F10 = 0x0009, /* Translates to 44 in codeset 1 */
    SCANCODE_F11 = 0x0078, /* Translates to 57 in codeset 1 */
    SCANCODE_F12 = 0x0007, /* Translates to 58 in codeset 1 */
    SCANCODE_F13 = 0x000f, /* Translates to 59 in codeset 1 */
    SCANCODE_F14 = 0x0017, /* Translates to 5a in codeset 1 */
    SCANCODE_F15 = 0x001f, /* Translates to 5b in codeset 1 */

    SCANCODE_BACK = 0xe038, /* e06a in codeset 1 */
    SCANCODE_REFRESH = 0xe020, /* e067 in codeset 1 */
    SCANCODE_FORWARD = 0xe030, /* e069 in codeset 1 */
    SCANCODE_FULLSCREEN = 0xe01d, /* e011 in codeset 1 */
    SCANCODE_OVERVIEW = 0xe024, /* e012 in codeset 1 */
    SCANCODE_SNAPSHOT = 0xe02d, /* e013 in codeset 1 */
    SCANCODE_BRIGHTNESS_DOWN = 0xe02c, /* e014 in codeset 1 */
    SCANCODE_BRIGHTNESS_UP = 0xe035, /* e015 in codeset 1 */
    SCANCODE_PRIVACY_SCRN_TOGGLE = 0xe03c, /* e016 in codeset 1 */
    SCANCODE_VOLUME_MUTE = 0xe023, /* e020 in codeset 1 */
    SCANCODE_VOLUME_DOWN = 0xe021, /* e02e in codeset 1 */
    SCANCODE_VOLUME_UP = 0xe032, /* e030 in codeset 1 */
    SCANCODE_KBD_BKLIGHT_DOWN = 0xe043, /* e017 in codeset 1 */
    SCANCODE_KBD_BKLIGHT_UP = 0xe044, /* e018 in codeset 1 */
    SCANCODE_KBD_BKLIGHT_TOGGLE = 0xe01c, /* e01e in codeset 1 */
    SCANCODE_NEXT_TRACK = 0xe04d, /* e019 in codeset 1 */
    SCANCODE_PREV_TRACK = 0xe015, /* e010 in codeset 1 */
    SCANCODE_PLAY_PAUSE = 0xe054, /* e01a in codeset 1 */
    SCANCODE_MICMUTE = 0xe05b, /* e01b in codeset 1 */

    SCANCODE_UP = 0xe075,
    SCANCODE_DOWN = 0xe072,
    SCANCODE_LEFT = 0xe06b,
    SCANCODE_RIGHT = 0xe074,

    SCANCODE_LEFT_CTRL = 0x0014,
    SCANCODE_RIGHT_CTRL = 0xe014,
    SCANCODE_LEFT_ALT = 0x0011,
    SCANCODE_RIGHT_ALT = 0xe011,

    SCANCODE_LEFT_WIN = 0xe01f, /* Also known as GUI or Super key. */
    SCANCODE_RIGHT_WIN = 0xe027,
    SCANCODE_MENU = 0xe02f,

    SCANCODE_POWER = 0xe037,

    SCANCODE_NUMLOCK = 0x0077,
    SCANCODE_CAPSLOCK = 0x0058,
    SCANCODE_SCROLL_LOCK = 0x007e,

    SCANCODE_CTRL_BREAK = 0xe07e,
};

 */

const location: &str = "C:/Windows/System32/drivers";

const function_keys: [i32; 16] = [
    0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x40, 0x41, 0x42, 0x43, 0x44, 0x57, 0x58, //F13 - F16
    0x64, 0x64, 0x66, 0x67,
];
// Modifier and Special Keycodes
const K_LCTRL: u8 = 0x1D;
const K_LALT: u8 = 0x38;
const K_LSHFT: u8 = 0x2A;
const K_ASSISTANT: u8 = 0x58;
const K_LWIN: u8 = 0x5B;

const K_RCTRL: u8 = 0x1D;
const K_RALT: u8 = 0x38;
const K_RSHFT: u8 = 0x36;

const K_BACKSP: u8 = 0x0E;
const K_DELETE: u8 = 0x53;
const K_LOCK: u8 = 0x5D;

const K_UP: u8 = 0x48;
const K_DOWN: u8 = 0x50;
const K_LEFT: u8 = 0x4B;
const K_RIGHT: u8 = 0x4D;

const K_PGUP: u8 = 0x49;
const K_HOME: u8 = 0x47;
const K_END: u8 = 0x4F;
const K_PGDN: u8 = 0x51;

const K_NUMLCK: u8 = 0x45;

// Vivaldi Keycodes
const VIVALDI_BACK: u8 = 0x6A;
const VIVALDI_FWD: u8 = 0x69;
const VIVALDI_REFRESH: u8 = 0x67;
const VIVALDI_FULLSCREEN: u8 = 0x11;
const VIVALDI_OVERVIEW: u8 = 0x12;
const VIVALDI_SNAPSHOT: u8 = 0x13;
const VIVALDI_BRIGHTNESS_DN: u8 = 0x14;
const VIVALDI_BRIGHTNESS_UP: u8 = 0x15;
const VIVALDI_PRIVACY_TOGGLE: u8 = 0x16;
const VIVALDI_KBD_BKLIGHT_DOWN: u8 = 0x17;
const VIVALDI_KBD_BKLIGHT_UP: u8 = 0x18;
const VIVALDI_KBD_BKLIGHT_TOGGLE: u8 = 0x1E;
const VIVALDI_PLAY_PAUSE: u8 = 0x1A;
const VIVALDI_MUTE: u8 = 0x20;
const VIVALDI_VOL_DN: u8 = 0x2E;
const VIVALDI_VOL_UP: u8 = 0x30;
const VIVALDI_NEXT_TRACK: u8 = 0x19;
const VIVALDI_PREV_TRACK: u8 = 0x10;
const VIVALDI_MIC_MUTE: u8 = 0x1B;

// Wilco platform variants
const WILCO_FULLSCREEN: u8 = 0x55;
const WILCO_OVERVIEW: u8 = 0x56;
const WILCO_BRIGHTNESS_DN: u8 = 0x15;
const WILCO_BRIGHTNESS_UP: u8 = 0x11;
const WILCO_PROJECT: u8 = 0x0B;

// CrosKB HID values
const CROSKBHID_BRIGHTNESS_UP: u8 = 0x01;
const CROSKBHID_BRIGHTNESS_DN: u8 = 0x02;
const CROSKBHID_KBLT_UP: u8 = 0x04;
const CROSKBHID_KBLT_DN: u8 = 0x08;
const CROSKBHID_KBLT_TOGGLE: u8 = 0x10;

const cfg_magic: &str = "CrKB";

//structs
struct RemapCfgKey {
    MakeCode: u16,
    Flags: u16,
}
impl RemapCfgKey
{
    fn new() -> Self
    {
        RemapCfgKey { MakeCode: 0, Flags: 0 }
    }
}

enum RemapCfgOverride {
    RemapCfgOverrideAutoDetect,
    RemapCfgOverrideEnable,
    RemapCfgOverrideDisable,
}

enum RemapCfgKeyState {
    RemapCfgKeyStateNoDetect,
    RemapCfgKeyStateEnforce,
    RemapCfgKeyStateEnforceNot,
}

struct RemapCfg {
    LeftCtrl: RemapCfgKeyState,
    LeftAlt: RemapCfgKeyState,
    Search: RemapCfgKeyState,
    Assistant: RemapCfgKeyState,
    LeftShift: RemapCfgKeyState,
    RightCtrl: RemapCfgKeyState,
    RightAlt: RemapCfgKeyState,
    RightShift: RemapCfgKeyState,
    originalKey: RemapCfgKey,
    remapVivaldiToFnKeys: bool,
    remappedKey: RemapCfgKey,
    additionalKeys: [RemapCfgKey; 8],
}

struct RemapCfgs {
    magic: u32,
    remappings: u32,
    FlipSearchAndAssistantOnPixelbook: bool,
    HasAssistantKey: RemapCfgOverride,
    IsNonChromeEC: RemapCfgOverride,
    cfg: [RemapCfg; 1],
}
#[derive(Copy, Clone)]
struct KeyStruct {
    MakeCode: u16,
    Flags: u16,
    InternalFlags: u16,
}

const max_current_keys: i32 = 20;

struct vivaldi_tester {
    legacy_top_row_keys: [u8; 10],
    legacy_vivaldi: [u8; 10],

    function_row_count: u8,
    function_row_keys: [KeyStruct; 16],

    remap_config: *mut RemapCfgKey,

    left_ctrl_pressed: bool,
    left_alt_pressed: bool,
    left_shift_pressed: bool,
    assistant_pressed: bool,
    search_pressed: bool,

    right_ctrl_pressed: bool,
    right_alt_pressed: bool,
    right_shift_pressed: bool,

    current_keys: [KeyStruct; max_current_keys as usize],

    num_key_pressed: i32,
    num_remaps: i32,
}
impl KeyStruct
{
    fn new() -> Self
    {
        KeyStruct { MakeCode: 0, Flags: 0, InternalFlags: 0 }
    }
}
impl vivaldi_tester {
    fn new() -> Self {
        let key = KeyStruct::new();
        let mut remap_cfg = RemapCfgKey::new();
        let mut remap_config = &mut remap_cfg as *mut RemapCfgKey;
        vivaldi_tester {
            legacy_top_row_keys: [0;10],
            legacy_vivaldi: [0;10],
            function_row_count: 13,
            function_row_keys: [key;16],
            remap_config,
            left_ctrl_pressed: false,   
            left_alt_pressed: false,
            left_shift_pressed: false,
            assistant_pressed: false,
            search_pressed: false,
            right_ctrl_pressed: false,
            right_alt_pressed: false,
            right_shift_pressed: false,
            current_keys: [key; max_current_keys as usize],
            num_key_pressed: 0,
            num_remaps: 0,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
