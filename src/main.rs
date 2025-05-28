use crate::RemapCfgKeyState::{RemapCfgKeyStateNoDetect, RemapCfgKeyStateEnforceNot, RemapCfgKeyStateEnforce };
use crate::RemapCfgOverride::RemapCfgOverrideAutoDetect;
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

const function_keys: [u16; 16] = [
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

const KEY_BREAK:u16 = 1;
const KEY_E0:u16 = 2;
const KEY_E1: u16 = 4;

const CFG_MAGIC: u32 = u32::from_le_bytes(*b"CrKB");

//structs
#[derive(Clone, Copy)]
struct RemapCfgKey {
    MakeCode: u16,
    Flags: u16,
}
impl RemapCfgKey {
    fn new() -> Self {
        RemapCfgKey {
            MakeCode: 0,
            Flags: 0,
        }
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

struct remap_config_keys {
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
impl remap_config_keys {
    fn new() -> Self {
        let cfgkey = RemapCfgKey::new();
        remap_config_keys {
            LeftCtrl: RemapCfgKeyStateNoDetect,
            LeftAlt: RemapCfgKeyStateNoDetect,
            Search: RemapCfgKeyStateNoDetect,
            Assistant: RemapCfgKeyStateNoDetect,
            LeftShift: RemapCfgKeyStateNoDetect,
            RightCtrl: RemapCfgKeyStateNoDetect,
            RightAlt: RemapCfgKeyStateNoDetect,
            RightShift: RemapCfgKeyStateNoDetect,
            originalKey: cfgkey,
            remapVivaldiToFnKeys: false,
            remappedKey: cfgkey,
            additionalKeys: [cfgkey; 8],
        }
    }
}

struct remap_configs {
    magic: u32,
    remappings: u32,
    FlipSearchAndAssistantOnPixelbook: bool,
    HasAssistantKey: RemapCfgOverride,
    IsNonChromeEC: RemapCfgOverride,
    cfg: [remap_config_keys; 1],
}
impl remap_configs {
    fn new() -> Self {
        let cfg = [remap_config_keys::new(); 1];
        remap_configs {
            magic: 0,
            remappings: 0,
            FlipSearchAndAssistantOnPixelbook: false,
            HasAssistantKey: RemapCfgOverrideAutoDetect,
            IsNonChromeEC: RemapCfgOverrideAutoDetect,
            cfg,
        }
    }
    fn default(&mut self)
    {
        let remap_configuration = self;

    //start remap config
    remap_configuration.magic = CFG_MAGIC;
    remap_configuration.FlipSearchAndAssistantOnPixelbook = true;
    remap_configuration.HasAssistantKey = RemapCfgOverrideAutoDetect;
    remap_configuration.IsNonChromeEC = RemapCfgOverrideAutoDetect;
    remap_configuration.remappings = 40;

    //start map vivalid keys (without Ctrl) to F# keys
    remap_configuration.cfg[0].LeftCtrl = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[0].originalKey.MakeCode = VIVALDI_BACK as u16;
    remap_configuration.cfg[0].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[0].remapVivaldiToFnKeys = true;

    remap_configuration.cfg[1].LeftCtrl = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[1].originalKey.MakeCode = VIVALDI_FWD as u16;
    remap_configuration.cfg[1].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[1].remapVivaldiToFnKeys = true;

    remap_configuration.cfg[2].LeftCtrl = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[2].originalKey.MakeCode = VIVALDI_REFRESH as u16;
    remap_configuration.cfg[2].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[2].remapVivaldiToFnKeys = true;

    remap_configuration.cfg[3].LeftCtrl = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[3].originalKey.MakeCode = VIVALDI_FULLSCREEN as u16;
    remap_configuration.cfg[3].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[3].remapVivaldiToFnKeys = true;

    remap_configuration.cfg[4].LeftCtrl = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[4].originalKey.MakeCode = VIVALDI_OVERVIEW as u16;
    remap_configuration.cfg[4].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[4].remapVivaldiToFnKeys = true;

    remap_configuration.cfg[5].LeftCtrl = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[5].originalKey.MakeCode = VIVALDI_SNAPSHOT as u16;
    remap_configuration.cfg[5].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[5].remapVivaldiToFnKeys = true;

    remap_configuration.cfg[6].LeftCtrl = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[6].originalKey.MakeCode = VIVALDI_BRIGHTNESS_DN as u16;
    remap_configuration.cfg[6].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[6].remapVivaldiToFnKeys = true;

    remap_configuration.cfg[7].LeftCtrl = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[7].originalKey.MakeCode = VIVALDI_BRIGHTNESS_UP as u16;
    remap_configuration.cfg[7].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[7].remapVivaldiToFnKeys = true;

    remap_configuration.cfg[8].LeftCtrl = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[8].originalKey.MakeCode = VIVALDI_PRIVACY_TOGGLE as u16;
    remap_configuration.cfg[8].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[8].remapVivaldiToFnKeys = true;

    remap_configuration.cfg[9].LeftCtrl = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[9].originalKey.MakeCode = VIVALDI_KBD_BKLIGHT_DOWN as u16;
    remap_configuration.cfg[9].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[9].remapVivaldiToFnKeys = true;

    remap_configuration.cfg[10].LeftCtrl = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[10].originalKey.MakeCode = VIVALDI_KBD_BKLIGHT_UP as u16;
    remap_configuration.cfg[10].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[10].remapVivaldiToFnKeys = true;

    remap_configuration.cfg[11].LeftCtrl = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[11].originalKey.MakeCode = VIVALDI_KBD_BKLIGHT_TOGGLE as u16;
    remap_configuration.cfg[11].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[11].remapVivaldiToFnKeys = true;

    remap_configuration.cfg[12].LeftCtrl = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[12].originalKey.MakeCode = VIVALDI_PLAY_PAUSE as u16;
    remap_configuration.cfg[12].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[12].remapVivaldiToFnKeys = true;

    remap_configuration.cfg[13].LeftCtrl = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[13].originalKey.MakeCode = VIVALDI_MUTE as u16;
    remap_configuration.cfg[13].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[13].remapVivaldiToFnKeys = true;

    remap_configuration.cfg[14].LeftCtrl = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[14].originalKey.MakeCode = VIVALDI_VOL_DN as u16;
    remap_configuration.cfg[14].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[14].remapVivaldiToFnKeys = true;

    remap_configuration.cfg[15].LeftCtrl = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[15].originalKey.MakeCode = VIVALDI_VOL_UP as u16;
    remap_configuration.cfg[15].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[15].remapVivaldiToFnKeys = true;

    remap_configuration.cfg[16].LeftCtrl = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[16].originalKey.MakeCode = VIVALDI_NEXT_TRACK as u16;
    remap_configuration.cfg[16].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[16].remapVivaldiToFnKeys = true;

    remap_configuration.cfg[17].LeftCtrl = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[17].originalKey.MakeCode = VIVALDI_PREV_TRACK as u16;
    remap_configuration.cfg[17].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[17].remapVivaldiToFnKeys = true;

    remap_configuration.cfg[18].LeftCtrl = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[18].originalKey.MakeCode = VIVALDI_MIC_MUTE as u16;
    remap_configuration.cfg[18].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[18].remapVivaldiToFnKeys = true;

    // ctrl + alt + backspace -> ctrl + alt + delete
    remap_configuration.cfg[19].LeftCtrl = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[19].LeftAlt = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[19].originalKey.MakeCode = K_BACKSP as u16;
    remap_configuration.cfg[19].originalKey.Flags = 0;
    remap_configuration.cfg[19].remappedKey.MakeCode = K_DELETE as u16;
    remap_configuration.cfg[19].remappedKey.Flags = KEY_E0;


    //map ctrl + backspace -> delete
    remap_configuration.cfg[20].LeftCtrl = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[20].LeftAlt = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[20].originalKey.MakeCode = K_BACKSP as u16;
    remap_configuration.cfg[20].originalKey.Flags = 0;
    remap_configuration.cfg[20].remappedKey.MakeCode = K_DELETE as u16;
    remap_configuration.cfg[20].remappedKey.Flags = KEY_E0;
    remap_configuration.cfg[20].additionalKeys[0].MakeCode = K_LCTRL as u16;
    remap_configuration.cfg[20].additionalKeys[0].Flags = KEY_BREAK;

    //map ctrl + fullscreen -> f11
    remap_configuration.cfg[21].LeftCtrl = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[21].LeftShift = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[21].originalKey.MakeCode = VIVALDI_FULLSCREEN as u16;
    remap_configuration.cfg[21].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[21].remappedKey.MakeCode = function_keys[10];
    remap_configuration.cfg[21].additionalKeys[0].Flags = KEY_BREAK;

    //map ctrl + shift + fullscreen -> windows + p
    remap_configuration.cfg[22].LeftCtrl = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[22].LeftShift = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[22].Search = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[22].originalKey.MakeCode = VIVALDI_FULLSCREEN as u16;
    remap_configuration.cfg[22].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[22].remappedKey.MakeCode = 0x19;
    remap_configuration.cfg[22].additionalKeys[0].MakeCode = K_LCTRL as u16;
    remap_configuration.cfg[22].additionalKeys[0].Flags = KEY_BREAK;
    remap_configuration.cfg[22].additionalKeys[1].MakeCode = K_LSHFT as u16;
    remap_configuration.cfg[22].additionalKeys[1].Flags = KEY_BREAK;
    remap_configuration.cfg[22].additionalKeys[2].MakeCode = K_LWIN as u16;
    remap_configuration.cfg[22].additionalKeys[2].Flags = KEY_E0;

    remap_configuration.cfg[23].LeftCtrl = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[23].LeftShift = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[23].Search = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[23].originalKey.MakeCode = VIVALDI_FULLSCREEN as u16;
    remap_configuration.cfg[23].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[23].remappedKey.MakeCode = 0x19;
    remap_configuration.cfg[23].additionalKeys[0].MakeCode = K_LCTRL as u16;
    remap_configuration.cfg[23].additionalKeys[0].Flags = KEY_BREAK;
    remap_configuration.cfg[23].additionalKeys[1].MakeCode = K_LSHFT as u16;
    remap_configuration.cfg[23].additionalKeys[1].Flags = KEY_BREAK;

    //map ctrl + overview -> windows + tab
    remap_configuration.cfg[24].LeftCtrl = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[24].LeftShift = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[24].Search = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[24].originalKey.MakeCode = VIVALDI_OVERVIEW as u16;
    remap_configuration.cfg[24].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[24].remappedKey.MakeCode = 0x0F;
    remap_configuration.cfg[24].additionalKeys[0].MakeCode = K_LCTRL as u16;
    remap_configuration.cfg[24].additionalKeys[0].Flags = KEY_BREAK;
    remap_configuration.cfg[24].additionalKeys[1].MakeCode = K_LWIN as u16;
    remap_configuration.cfg[24].additionalKeys[1].Flags = KEY_E0;

    remap_configuration.cfg[25].LeftCtrl = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[25].LeftShift = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[25].Search = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[25].originalKey.MakeCode = VIVALDI_OVERVIEW as u16;
    remap_configuration.cfg[25].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[25].remappedKey.MakeCode = 0x0F;
    remap_configuration.cfg[25].additionalKeys[0].MakeCode = K_LCTRL as u16;
    remap_configuration.cfg[25].additionalKeys[0].Flags = KEY_BREAK;

    //map ctrl + shift + overview -> windows + shift + s
    remap_configuration.cfg[26].LeftCtrl = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[26].LeftShift = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[26].Search = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[26].originalKey.MakeCode = VIVALDI_OVERVIEW as u16;
    remap_configuration.cfg[26].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[26].remappedKey.MakeCode = 0x1F;
    remap_configuration.cfg[26].additionalKeys[0].MakeCode = K_LCTRL as u16;
    remap_configuration.cfg[26].additionalKeys[0].Flags = KEY_BREAK;
    remap_configuration.cfg[26].additionalKeys[1].MakeCode = K_LWIN as u16;
    remap_configuration.cfg[26].additionalKeys[1].Flags = KEY_E0;

    remap_configuration.cfg[27].LeftCtrl = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[27].LeftShift = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[27].Search = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[27].originalKey.MakeCode = VIVALDI_OVERVIEW as u16;
    remap_configuration.cfg[27].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[27].remappedKey.MakeCode = K_LCTRL as u16;
    remap_configuration.cfg[27].additionalKeys[0].MakeCode =K_LCTRL as u16;
    remap_configuration.cfg[27].additionalKeys[0].Flags = KEY_BREAK;

    //map ctrl + snapshot -> windows + shift + s
    remap_configuration.cfg[28].LeftCtrl = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[28].LeftShift = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[28].Search = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[28].originalKey.MakeCode = VIVALDI_SNAPSHOT as u16;
    remap_configuration.cfg[28].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[28].remappedKey.MakeCode = 0x1F;
    remap_configuration.cfg[28].additionalKeys[0].MakeCode = K_LCTRL as u16;
    remap_configuration.cfg[28].additionalKeys[0].Flags = KEY_BREAK;
    remap_configuration.cfg[28].additionalKeys[1].MakeCode = K_LWIN as u16;
    remap_configuration.cfg[28].additionalKeys[1].Flags = KEY_E0;
    remap_configuration.cfg[28].additionalKeys[2].MakeCode = K_LSHFT as u16;

    remap_configuration.cfg[29].LeftCtrl = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[29].LeftShift = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[29].Search = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[29].originalKey.MakeCode = VIVALDI_SNAPSHOT as u16;
    remap_configuration.cfg[29].remappedKey.MakeCode = 0x1F;
    remap_configuration.cfg[29].additionalKeys[0].MakeCode = K_LCTRL as u16;
    remap_configuration.cfg[29].additionalKeys[0].Flags = KEY_BREAK;
    remap_configuration.cfg[29].additionalKeys[1].MakeCode = K_LSHFT as u16;

    remap_configuration.cfg[30].LeftCtrl = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[30].LeftShift = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[30].Search = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[30].originalKey.MakeCode = VIVALDI_SNAPSHOT as u16;
    remap_configuration.cfg[30].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[30].remappedKey.MakeCode = 0x1f;
    remap_configuration.cfg[30].additionalKeys[0].MakeCode = K_LCTRL as u16;
    remap_configuration.cfg[30].additionalKeys[0].Flags = KEY_BREAK;
    remap_configuration.cfg[30].additionalKeys[1].MakeCode = K_LWIN as u16;
    remap_configuration.cfg[30].additionalKeys[1].Flags = KEY_E0;

    remap_configuration.cfg[31].LeftCtrl = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[31].LeftShift = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[31].Search = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[31].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[31].remappedKey.MakeCode = 0x1f;
    remap_configuration.cfg[31].additionalKeys[0].MakeCode = K_LCTRL as u16;
    remap_configuration.cfg[31].additionalKeys[0].Flags = KEY_BREAK;

    //ctrl + alt + brightness -> ctrl + alt + kb brightness

    remap_configuration.cfg[32].LeftCtrl = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[32].LeftAlt = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[32].originalKey.MakeCode = VIVALDI_BRIGHTNESS_DN as u16;
    remap_configuration.cfg[32].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[32].remappedKey.MakeCode = VIVALDI_KBD_BKLIGHT_DOWN as u16;
    remap_configuration.cfg[32].remappedKey.Flags = KEY_E0;

    remap_configuration.cfg[33].LeftCtrl = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[33].LeftAlt = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[33].originalKey.MakeCode = VIVALDI_BRIGHTNESS_UP as u16;
    remap_configuration.cfg[33].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[33].remappedKey.MakeCode = VIVALDI_KBD_BKLIGHT_UP as u16;
    remap_configuration.cfg[33].remappedKey.Flags = KEY_E0;

    //ctrl + left -> home
    remap_configuration.cfg[34].LeftCtrl = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[34].originalKey.MakeCode = K_LEFT as u16;
    remap_configuration.cfg[34].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[34].remappedKey.MakeCode = K_HOME as u16;
    remap_configuration.cfg[34].remappedKey.Flags = KEY_E0;
    remap_configuration.cfg[34].additionalKeys[0].MakeCode = K_LCTRL as u16;
    remap_configuration.cfg[34].additionalKeys[0].Flags = KEY_BREAK;

    //ctrl + right -> end

    remap_configuration.cfg[35].LeftCtrl = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[35].originalKey.MakeCode = K_RIGHT as u16;
    remap_configuration.cfg[35].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[35].remappedKey.MakeCode = K_END as u16;
    remap_configuration.cfg[35].remappedKey.Flags = KEY_E0;
    remap_configuration.cfg[35].additionalKeys[0].MakeCode = K_LCTRL as u16;
    remap_configuration.cfg[35].additionalKeys[0].Flags = KEY_BREAK;

    //ctrl + up -> page up
    remap_configuration.cfg[36].LeftCtrl = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[36].originalKey.MakeCode = K_UP as u16;
    remap_configuration.cfg[36].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[36].remappedKey.MakeCode = K_PGUP as u16;
    remap_configuration.cfg[36].remappedKey.Flags = KEY_E0;
    remap_configuration.cfg[36].additionalKeys[0].MakeCode = K_LCTRL as u16;
    remap_configuration.cfg[36].additionalKeys[0].Flags = KEY_BREAK;

    //ctrl + down -> page down
    remap_configuration.cfg[37].LeftCtrl = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[37].originalKey.MakeCode = K_DOWN as u16;
    remap_configuration.cfg[37].originalKey.Flags = KEY_E0;
    remap_configuration.cfg[37].remappedKey.MakeCode = K_PGDN as u16;
    remap_configuration.cfg[37].remappedKey.Flags = KEY_E0;
    remap_configuration.cfg[37].additionalKeys[0].MakeCode = K_LCTRL as u16;
    remap_configuration.cfg[37].additionalKeys[0].Flags = KEY_BREAK;


    //lock -> windows + L

    remap_configuration.cfg[38].Search = RemapCfgKeyStateEnforceNot;
    remap_configuration.cfg[38].originalKey.MakeCode = K_LOCK as u16;
    remap_configuration.cfg[38].originalKey.Flags = 0;
    remap_configuration.cfg[38].remappedKey.MakeCode = 0x26;
    remap_configuration.cfg[38].additionalKeys[0].MakeCode = K_LWIN as u16;
    remap_configuration.cfg[38].additionalKeys[0].Flags = KEY_E0;

    remap_configuration.cfg[39].Search = RemapCfgKeyStateEnforce;
    remap_configuration.cfg[39].originalKey.MakeCode = K_LOCK as u16;
    remap_configuration.cfg[39].originalKey.Flags = 0;
    remap_configuration.cfg[39].remappedKey.MakeCode = 0x26;


    }
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

    remap_config: remap_configs,

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
impl KeyStruct {
    fn new() -> Self {
        KeyStruct {
            MakeCode: 0,
            Flags: 0,
            InternalFlags: 0,
        }
    }
}
impl vivaldi_tester {
    fn new() -> Self {
        vivaldi_tester {
            legacy_top_row_keys: [0; 10],
            legacy_vivaldi: [0; 10],
            function_row_count: 13,
            function_row_keys: [KeyStruct::new(); 16],
            remap_config: remap_configs::new(),
            left_ctrl_pressed: false,
            left_alt_pressed: false,
            left_shift_pressed: false,
            assistant_pressed: false,
            search_pressed: false,
            right_ctrl_pressed: false,
            right_alt_pressed: false,
            right_shift_pressed: false,
            current_keys: [KeyStruct::new(); max_current_keys as usize],
            num_key_pressed: 0,
            num_remaps: 0,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut reamp_config = vivaldi_tester::new();
    //sets the config to default
    reamp_config.remap_config.default();

    
}
