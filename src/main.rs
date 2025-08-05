
use crate::RemapCfgKeyState::{
    RemapCfgKeyStateEnforce, RemapCfgKeyStateEnforceNot, RemapCfgKeyStateNoDetect,
};
use crate::RemapCfgOverride::RemapCfgOverrideAutoDetect;

//crates
use bincode::{self, Decode, Encode, config, encode_into_std_write};
use std::fs::File;
use mem_cmp::MemEq;

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


const LOCATION: &str = "C:/Windows/System32/drivers";

const FUNCTION_KEYS: [u16; 16] = [
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

const KEY_BREAK: u16 = 1;
const KEY_E0: u16 = 2;
const KEY_E1: u16 = 4;

const CFG_MAGIC: u32 = u32::from_le_bytes(*b"CrKB");

const MAX_CURRENT_KEYS: i32 = 20;


//structs
#[repr(C, packed)]
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Decode)]
struct RemapCfgKey {
    make_code: u16,
    flags: u16,
}
impl RemapCfgKey {
    fn new() -> Self {
        RemapCfgKey {
            make_code: 0,
            flags: 0,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize, Encode, Decode, Clone, Copy)]
struct RemappedKeyStruct
{
    original_key: KeyStruct,
    remapped_key: KeyStruct
}
impl  RemappedKeyStruct {
    fn new() -> Self
    {
        RemappedKeyStruct { original_key: KeyStruct::new(), remapped_key: KeyStruct::new() }
    }

}

#[repr(u32)]
#[derive(serde::Serialize, serde::Deserialize, Encode, Decode, Clone, Copy)]
enum RemapCfgOverride {
    RemapCfgOverrideAutoDetect = 0,
    RemapCfgOverrideEnable = 1,
    RemapCfgOverrideDisable = 2,
}
#[repr(u32)]
#[derive(serde::Serialize, serde::Deserialize, Encode, Decode, Clone, Copy)]
enum RemapCfgKeyState {
    RemapCfgKeyStateNoDetect = 0,
    RemapCfgKeyStateEnforce = 1,
    RemapCfgKeyStateEnforceNot = 2,
}
#[repr(C, packed)]
#[derive(serde::Serialize, serde::Deserialize, Decode, Clone, Copy)]
struct RemapConfigKeys {
    left_ctrl: RemapCfgKeyState,
    left_alt: RemapCfgKeyState,
    search: RemapCfgKeyState,
    assistant: RemapCfgKeyState,
    left_shift: RemapCfgKeyState,
    right_ctrl: RemapCfgKeyState,
    right_alt: RemapCfgKeyState,
    right_shift: RemapCfgKeyState,
    original_key: RemapCfgKey,
    remap_vivaldi_to_fn_keys: bool,
    remapped_key: RemapCfgKey,
    additional_keys: [RemapCfgKey; 8],
}
impl RemapConfigKeys {
    fn new() -> Self {
        let cfgkey = RemapCfgKey::new();
        RemapConfigKeys {
            left_ctrl: RemapCfgKeyStateNoDetect,
            left_alt: RemapCfgKeyStateNoDetect,
            search: RemapCfgKeyStateNoDetect,
            assistant: RemapCfgKeyStateNoDetect,
            left_shift: RemapCfgKeyStateNoDetect,
            right_ctrl: RemapCfgKeyStateNoDetect,
            right_alt: RemapCfgKeyStateNoDetect,
            right_shift: RemapCfgKeyStateNoDetect,
            original_key: cfgkey,
            remap_vivaldi_to_fn_keys: false,
            remapped_key: cfgkey,
            additional_keys: [cfgkey; 8],
        }
    }
}
#[repr(C, packed)]
#[derive(serde::Deserialize, Decode)]
struct RemapConfigs {
    magic: u32,
    remappings: u32,
    flip_search_and_assistant_on_pixelbook: bool,
    has_assistant_key: RemapCfgOverride,
    is_non_chrome_ec: RemapCfgOverride,
    cfg: Vec<RemapConfigKeys>,
}
impl RemapConfigs {
    fn new() -> Self {
        let config = RemapConfigKeys::new();
        RemapConfigs {
            magic: 0,
            remappings: 0,
            flip_search_and_assistant_on_pixelbook: false,
            has_assistant_key: RemapCfgOverrideAutoDetect,
            is_non_chrome_ec: RemapCfgOverrideAutoDetect,
            cfg: [config; 40].to_vec(),
        }
    }
    fn default(&mut self) {
        let remap_configuration = self;

        //start remap config
        remap_configuration.magic = CFG_MAGIC;
        remap_configuration.flip_search_and_assistant_on_pixelbook = true;
        remap_configuration.has_assistant_key = RemapCfgOverrideAutoDetect;
        remap_configuration.is_non_chrome_ec = RemapCfgOverrideAutoDetect;
        remap_configuration.remappings = 40;

        //start map vivalid keys (without Ctrl) to F# keys
        remap_configuration.cfg[0].left_ctrl = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[0].original_key.make_code = VIVALDI_BACK as u16;
        remap_configuration.cfg[0].original_key.flags = KEY_E0;
        remap_configuration.cfg[0].remap_vivaldi_to_fn_keys = true;

        remap_configuration.cfg[1].left_ctrl = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[1].original_key.make_code = VIVALDI_FWD as u16;
        remap_configuration.cfg[1].original_key.flags = KEY_E0;
        remap_configuration.cfg[1].remap_vivaldi_to_fn_keys = true;

        remap_configuration.cfg[2].left_ctrl = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[2].original_key.make_code = VIVALDI_REFRESH as u16;
        remap_configuration.cfg[2].original_key.flags = KEY_E0;
        remap_configuration.cfg[2].remap_vivaldi_to_fn_keys = true;

        remap_configuration.cfg[3].left_ctrl = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[3].original_key.make_code = VIVALDI_FULLSCREEN as u16;
        remap_configuration.cfg[3].original_key.flags = KEY_E0;
        remap_configuration.cfg[3].remap_vivaldi_to_fn_keys = true;

        remap_configuration.cfg[4].left_ctrl = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[4].original_key.make_code = VIVALDI_OVERVIEW as u16;
        remap_configuration.cfg[4].original_key.flags = KEY_E0;
        remap_configuration.cfg[4].remap_vivaldi_to_fn_keys = true;

        remap_configuration.cfg[5].left_ctrl = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[5].original_key.make_code = VIVALDI_SNAPSHOT as u16;
        remap_configuration.cfg[5].original_key.flags = KEY_E0;
        remap_configuration.cfg[5].remap_vivaldi_to_fn_keys = true;

        remap_configuration.cfg[6].left_ctrl = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[6].original_key.make_code = VIVALDI_BRIGHTNESS_DN as u16;
        remap_configuration.cfg[6].original_key.flags = KEY_E0;
        remap_configuration.cfg[6].remap_vivaldi_to_fn_keys = true;

        remap_configuration.cfg[7].left_ctrl = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[7].original_key.make_code = VIVALDI_BRIGHTNESS_UP as u16;
        remap_configuration.cfg[7].original_key.flags = KEY_E0;
        remap_configuration.cfg[7].remap_vivaldi_to_fn_keys = true;

        remap_configuration.cfg[8].left_ctrl = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[8].original_key.make_code = VIVALDI_PRIVACY_TOGGLE as u16;
        remap_configuration.cfg[8].original_key.flags = KEY_E0;
        remap_configuration.cfg[8].remap_vivaldi_to_fn_keys = true;

        remap_configuration.cfg[9].left_ctrl = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[9].original_key.make_code = VIVALDI_KBD_BKLIGHT_DOWN as u16;
        remap_configuration.cfg[9].original_key.flags = KEY_E0;
        remap_configuration.cfg[9].remap_vivaldi_to_fn_keys = true;

        remap_configuration.cfg[10].left_ctrl = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[10].original_key.make_code = VIVALDI_KBD_BKLIGHT_UP as u16;
        remap_configuration.cfg[10].original_key.flags = KEY_E0;
        remap_configuration.cfg[10].remap_vivaldi_to_fn_keys = true;

        remap_configuration.cfg[11].left_ctrl = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[11].original_key.make_code = VIVALDI_KBD_BKLIGHT_TOGGLE as u16;
        remap_configuration.cfg[11].original_key.flags = KEY_E0;
        remap_configuration.cfg[11].remap_vivaldi_to_fn_keys = true;

        remap_configuration.cfg[12].left_ctrl = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[12].original_key.make_code = VIVALDI_PLAY_PAUSE as u16;
        remap_configuration.cfg[12].original_key.flags = KEY_E0;
        remap_configuration.cfg[12].remap_vivaldi_to_fn_keys = true;

        remap_configuration.cfg[13].left_ctrl = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[13].original_key.make_code = VIVALDI_MUTE as u16;
        remap_configuration.cfg[13].original_key.flags = KEY_E0;
        remap_configuration.cfg[13].remap_vivaldi_to_fn_keys = true;

        remap_configuration.cfg[14].left_ctrl = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[14].original_key.make_code = VIVALDI_VOL_DN as u16;
        remap_configuration.cfg[14].original_key.flags = KEY_E0;
        remap_configuration.cfg[14].remap_vivaldi_to_fn_keys = true;

        remap_configuration.cfg[15].left_ctrl = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[15].original_key.make_code = VIVALDI_VOL_UP as u16;
        remap_configuration.cfg[15].original_key.flags = KEY_E0;
        remap_configuration.cfg[15].remap_vivaldi_to_fn_keys = true;

        remap_configuration.cfg[16].left_ctrl = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[16].original_key.make_code = VIVALDI_NEXT_TRACK as u16;
        remap_configuration.cfg[16].original_key.flags = KEY_E0;
        remap_configuration.cfg[16].remap_vivaldi_to_fn_keys = true;

        remap_configuration.cfg[17].left_ctrl = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[17].original_key.make_code = VIVALDI_PREV_TRACK as u16;
        remap_configuration.cfg[17].original_key.flags = KEY_E0;
        remap_configuration.cfg[17].remap_vivaldi_to_fn_keys = true;

        remap_configuration.cfg[18].left_ctrl = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[18].original_key.make_code = VIVALDI_MIC_MUTE as u16;
        remap_configuration.cfg[18].original_key.flags = KEY_E0;
        remap_configuration.cfg[18].remap_vivaldi_to_fn_keys = true;

        // ctrl + alt + backspace -> ctrl + alt + delete
        remap_configuration.cfg[19].left_ctrl = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[19].left_alt = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[19].original_key.make_code = K_BACKSP as u16;
        remap_configuration.cfg[19].original_key.flags = 0;
        remap_configuration.cfg[19].remapped_key.make_code = K_DELETE as u16;
        remap_configuration.cfg[19].remapped_key.flags = KEY_E0;

        //map ctrl + backspace -> delete
        remap_configuration.cfg[20].left_ctrl = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[20].left_alt = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[20].original_key.make_code = K_BACKSP as u16;
        remap_configuration.cfg[20].original_key.flags = 0;
        remap_configuration.cfg[20].remapped_key.make_code = K_DELETE as u16;
        remap_configuration.cfg[20].remapped_key.flags = KEY_E0;
        remap_configuration.cfg[20].additional_keys[0].make_code = K_LCTRL as u16;
        remap_configuration.cfg[20].additional_keys[0].flags = KEY_BREAK;

        //map ctrl + fullscreen -> f11
        remap_configuration.cfg[21].left_ctrl = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[21].left_shift = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[21].original_key.make_code = VIVALDI_FULLSCREEN as u16;
        remap_configuration.cfg[21].original_key.flags = KEY_E0;
        remap_configuration.cfg[21].remapped_key.make_code = FUNCTION_KEYS[10];
        remap_configuration.cfg[21].additional_keys[0].flags = KEY_BREAK;

        //map ctrl + shift + fullscreen -> windows + p
        remap_configuration.cfg[22].left_ctrl = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[22].left_shift = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[22].search = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[22].original_key.make_code = VIVALDI_FULLSCREEN as u16;
        remap_configuration.cfg[22].original_key.flags = KEY_E0;
        remap_configuration.cfg[22].remapped_key.make_code = 0x19;
        remap_configuration.cfg[22].additional_keys[0].make_code = K_LCTRL as u16;
        remap_configuration.cfg[22].additional_keys[0].flags = KEY_BREAK;
        remap_configuration.cfg[22].additional_keys[1].make_code = K_LSHFT as u16;
        remap_configuration.cfg[22].additional_keys[1].flags = KEY_BREAK;
        remap_configuration.cfg[22].additional_keys[2].make_code = K_LWIN as u16;
        remap_configuration.cfg[22].additional_keys[2].flags = KEY_E0;

        remap_configuration.cfg[23].left_ctrl = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[23].left_shift = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[23].search = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[23].original_key.make_code = VIVALDI_FULLSCREEN as u16;
        remap_configuration.cfg[23].original_key.flags = KEY_E0;
        remap_configuration.cfg[23].remapped_key.make_code = 0x19;
        remap_configuration.cfg[23].additional_keys[0].make_code = K_LCTRL as u16;
        remap_configuration.cfg[23].additional_keys[0].flags = KEY_BREAK;
        remap_configuration.cfg[23].additional_keys[1].make_code = K_LSHFT as u16;
        remap_configuration.cfg[23].additional_keys[1].flags = KEY_BREAK;

        //map ctrl + overview -> windows + tab
        remap_configuration.cfg[24].left_ctrl = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[24].left_shift = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[24].search = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[24].original_key.make_code = VIVALDI_OVERVIEW as u16;
        remap_configuration.cfg[24].original_key.flags = KEY_E0;
        remap_configuration.cfg[24].remapped_key.make_code = 0x0F;
        remap_configuration.cfg[24].additional_keys[0].make_code = K_LCTRL as u16;
        remap_configuration.cfg[24].additional_keys[0].flags = KEY_BREAK;
        remap_configuration.cfg[24].additional_keys[1].make_code = K_LWIN as u16;
        remap_configuration.cfg[24].additional_keys[1].flags = KEY_E0;

        remap_configuration.cfg[25].left_ctrl = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[25].left_shift = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[25].search = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[25].original_key.make_code = VIVALDI_OVERVIEW as u16;
        remap_configuration.cfg[25].original_key.flags = KEY_E0;
        remap_configuration.cfg[25].remapped_key.make_code = 0x0F;
        remap_configuration.cfg[25].additional_keys[0].make_code = K_LCTRL as u16;
        remap_configuration.cfg[25].additional_keys[0].flags = KEY_BREAK;

        //map ctrl + shift + overview -> windows + shift + s
        remap_configuration.cfg[26].left_ctrl = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[26].left_shift = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[26].search = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[26].original_key.make_code = VIVALDI_OVERVIEW as u16;
        remap_configuration.cfg[26].original_key.flags = KEY_E0;
        remap_configuration.cfg[26].remapped_key.make_code = 0x1F;
        remap_configuration.cfg[26].additional_keys[0].make_code = K_LCTRL as u16;
        remap_configuration.cfg[26].additional_keys[0].flags = KEY_BREAK;
        remap_configuration.cfg[26].additional_keys[1].make_code = K_LWIN as u16;
        remap_configuration.cfg[26].additional_keys[1].flags = KEY_E0;

        remap_configuration.cfg[27].left_ctrl = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[27].left_shift = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[27].search = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[27].original_key.make_code = VIVALDI_OVERVIEW as u16;
        remap_configuration.cfg[27].original_key.flags = KEY_E0;
        remap_configuration.cfg[27].remapped_key.make_code = K_LCTRL as u16;
        remap_configuration.cfg[27].additional_keys[0].make_code = K_LCTRL as u16;
        remap_configuration.cfg[27].additional_keys[0].flags = KEY_BREAK;

        //map ctrl + snapshot -> windows + shift + s
        remap_configuration.cfg[28].left_ctrl = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[28].left_shift = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[28].search = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[28].original_key.make_code = VIVALDI_SNAPSHOT as u16;
        remap_configuration.cfg[28].original_key.flags = KEY_E0;
        remap_configuration.cfg[28].remapped_key.make_code = 0x1F;
        remap_configuration.cfg[28].additional_keys[0].make_code = K_LCTRL as u16;
        remap_configuration.cfg[28].additional_keys[0].flags = KEY_BREAK;
        remap_configuration.cfg[28].additional_keys[1].make_code = K_LWIN as u16;
        remap_configuration.cfg[28].additional_keys[1].flags = KEY_E0;
        remap_configuration.cfg[28].additional_keys[2].make_code = K_LSHFT as u16;

        remap_configuration.cfg[29].left_ctrl = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[29].left_shift = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[29].search = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[29].original_key.make_code = VIVALDI_SNAPSHOT as u16;
        remap_configuration.cfg[29].remapped_key.make_code = 0x1F;
        remap_configuration.cfg[29].additional_keys[0].make_code = K_LCTRL as u16;
        remap_configuration.cfg[29].additional_keys[0].flags = KEY_BREAK;
        remap_configuration.cfg[29].additional_keys[1].make_code = K_LSHFT as u16;

        remap_configuration.cfg[30].left_ctrl = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[30].left_shift = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[30].search = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[30].original_key.make_code = VIVALDI_SNAPSHOT as u16;
        remap_configuration.cfg[30].original_key.flags = KEY_E0;
        remap_configuration.cfg[30].remapped_key.make_code = 0x1f;
        remap_configuration.cfg[30].additional_keys[0].make_code = K_LCTRL as u16;
        remap_configuration.cfg[30].additional_keys[0].flags = KEY_BREAK;
        remap_configuration.cfg[30].additional_keys[1].make_code = K_LWIN as u16;
        remap_configuration.cfg[30].additional_keys[1].flags = KEY_E0;

        remap_configuration.cfg[31].left_ctrl = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[31].left_shift = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[31].search = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[31].original_key.flags = KEY_E0;
        remap_configuration.cfg[31].remapped_key.make_code = 0x1f;
        remap_configuration.cfg[31].additional_keys[0].make_code = K_LCTRL as u16;
        remap_configuration.cfg[31].additional_keys[0].flags = KEY_BREAK;

        //ctrl + alt + brightness -> ctrl + alt + kb brightness

        remap_configuration.cfg[32].left_ctrl = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[32].left_alt = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[32].original_key.make_code = VIVALDI_BRIGHTNESS_DN as u16;
        remap_configuration.cfg[32].original_key.flags = KEY_E0;
        remap_configuration.cfg[32].remapped_key.make_code = VIVALDI_KBD_BKLIGHT_DOWN as u16;
        remap_configuration.cfg[32].remapped_key.flags = KEY_E0;

        remap_configuration.cfg[33].left_ctrl = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[33].left_alt = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[33].original_key.make_code = VIVALDI_BRIGHTNESS_UP as u16;
        remap_configuration.cfg[33].original_key.flags = KEY_E0;
        remap_configuration.cfg[33].remapped_key.make_code = VIVALDI_KBD_BKLIGHT_UP as u16;
        remap_configuration.cfg[33].remapped_key.flags = KEY_E0;

        //ctrl + left -> home
        remap_configuration.cfg[34].left_ctrl = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[34].original_key.make_code = K_LEFT as u16;
        remap_configuration.cfg[34].original_key.flags = KEY_E0;
        remap_configuration.cfg[34].remapped_key.make_code = K_HOME as u16;
        remap_configuration.cfg[34].remapped_key.flags = KEY_E0;
        remap_configuration.cfg[34].additional_keys[0].make_code = K_LCTRL as u16;
        remap_configuration.cfg[34].additional_keys[0].flags = KEY_BREAK;

        //ctrl + right -> end

        remap_configuration.cfg[35].left_ctrl = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[35].original_key.make_code = K_RIGHT as u16;
        remap_configuration.cfg[35].original_key.flags = KEY_E0;
        remap_configuration.cfg[35].remapped_key.make_code = K_END as u16;
        remap_configuration.cfg[35].remapped_key.flags = KEY_E0;
        remap_configuration.cfg[35].additional_keys[0].make_code = K_LCTRL as u16;
        remap_configuration.cfg[35].additional_keys[0].flags = KEY_BREAK;

        //ctrl + up -> page up
        remap_configuration.cfg[36].left_ctrl = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[36].original_key.make_code = K_UP as u16;
        remap_configuration.cfg[36].original_key.flags = KEY_E0;
        remap_configuration.cfg[36].remapped_key.make_code = K_PGUP as u16;
        remap_configuration.cfg[36].remapped_key.flags = KEY_E0;
        remap_configuration.cfg[36].additional_keys[0].make_code = K_LCTRL as u16;
        remap_configuration.cfg[36].additional_keys[0].flags = KEY_BREAK;

        //ctrl + down -> page down
        remap_configuration.cfg[37].left_ctrl = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[37].original_key.make_code = K_DOWN as u16;
        remap_configuration.cfg[37].original_key.flags = KEY_E0;
        remap_configuration.cfg[37].remapped_key.make_code = K_PGDN as u16;
        remap_configuration.cfg[37].remapped_key.flags = KEY_E0;
        remap_configuration.cfg[37].additional_keys[0].make_code = K_LCTRL as u16;
        remap_configuration.cfg[37].additional_keys[0].flags = KEY_BREAK;

        //lock -> windows + L

        remap_configuration.cfg[38].search = RemapCfgKeyStateEnforceNot;
        remap_configuration.cfg[38].original_key.make_code = K_LOCK as u16;
        remap_configuration.cfg[38].original_key.flags = 0;
        remap_configuration.cfg[38].remapped_key.make_code = 0x26;
        remap_configuration.cfg[38].additional_keys[0].make_code = K_LWIN as u16;
        remap_configuration.cfg[38].additional_keys[0].flags = KEY_E0;

        remap_configuration.cfg[39].search = RemapCfgKeyStateEnforce;
        remap_configuration.cfg[39].original_key.make_code = K_LOCK as u16;
        remap_configuration.cfg[39].original_key.flags = 0;
        remap_configuration.cfg[39].remapped_key.make_code = 0x26;
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Encode, Decode)]
struct KeyStruct {
    make_code: u16,
    flags: u16,
    internal_flags: u16,
}

impl KeyStruct {
    fn new() -> Self {
        KeyStruct {
            make_code: 0,
            flags: 0,
            internal_flags: 0,
        }
    }
}


#[derive(serde::Deserialize, Decode)]
struct VivaldiTester {
    legacy_top_row_keys: [u8; 10],
    legacy_vivaldi: [u8; 10],

    function_row_count: u8,
    function_row_keys: [KeyStruct; 16],

    remap_config: RemapConfigs,

    left_ctrl_pressed: bool,
    left_alt_pressed: bool,
    left_shift_pressed: bool,
    assistant_pressed: bool,
    search_pressed: bool,

    right_ctrl_pressed: bool,
    right_alt_pressed: bool,
    right_shift_pressed: bool,

    current_keys: [KeyStruct; MAX_CURRENT_KEYS as usize],
    last_key_pressed: KeyStruct,

    remapped_keys: [RemappedKeyStruct; MAX_CURRENT_KEYS as usize],
    num_key_pressed: i32,
    num_remaps: i32,
}

impl VivaldiTester {
    fn new() -> Self {
        VivaldiTester {
            legacy_top_row_keys: [0; 10],
            legacy_vivaldi: [0; 10],
            function_row_count: 13,
            function_row_keys: [KeyStruct::new(); 16],
            remap_config: RemapConfigs::new(),
            left_ctrl_pressed: false,
            left_alt_pressed: false,
            left_shift_pressed: false,
            assistant_pressed: false,
            search_pressed: false,
            right_ctrl_pressed: false,
            right_alt_pressed: false,
            right_shift_pressed: false,
            current_keys: [KeyStruct::new(); MAX_CURRENT_KEYS as usize],
            last_key_pressed: KeyStruct::new(),
            remapped_keys: [RemappedKeyStruct::new(); MAX_CURRENT_KEYS as usize],
            num_key_pressed: 0,
            num_remaps: 0,
        }
    }
    fn find_key_index(&self, original_key: &RemapCfgKey) -> Option<usize> {
        for (i, key) in self.function_row_keys.iter().enumerate() {
            if key.make_code == original_key.make_code {
                return Some(i);
            }
        }
        None
    }
    fn add_remap(&mut self, remap: RemappedKeyStruct) -> bool
    {
        for i in 0..(MAX_CURRENT_KEYS as usize)
        {
            if self.remapped_keys[i].original_key.make_code == remap.original_key.make_code && self.remapped_keys[i].original_key.flags == remap.remapped_key.flags
            {
                if self.remapped_keys[i].mem_eq(&remap)
                {
                    return true; //remap exists
                }
                else
                {
                    return false;// remap exists but is not of the same configuration
                }
            }
        }
        return false;

    }
    fn garbage_collect(&mut self)
    {
        //clear out remap slots
        for _ in 0..MAX_CURRENT_KEYS
        {
            let mut key_remap:[RemappedKeyStruct; MAX_CURRENT_KEYS as usize] = [RemappedKeyStruct::new(); MAX_CURRENT_KEYS as usize];
            let empty_struct: RemappedKeyStruct = RemappedKeyStruct::new();
            let mut j: i32 = 0;
            for k in 0..MAX_CURRENT_KEYS
            {
                if self.remapped_keys[k as usize].mem_eq(&empty_struct)
                {
                    key_remap[j as usize] = self.remapped_keys[k as usize];
                    j+=1;
                }
            }
            self.num_remaps = j;
        }

        //clear out any empty key slots
        for _ in 0..MAX_CURRENT_KEYS
        {
            let mut key_codes:[KeyStruct; MAX_CURRENT_KEYS as usize] = [KeyStruct::new(); MAX_CURRENT_KEYS as usize];
            let mut j: i32 = 0;
            for k in 0..MAX_CURRENT_KEYS
            {
                if self.current_keys[k as usize].flags != 0 || self.current_keys[k as usize].make_code !=0
                {

                    key_codes[j as usize] = self.current_keys[k as usize];
                    j+=1;

                }
            }
            self.num_key_pressed = j;


        }

    }
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    check_sizes();

    let mut remap_configuration = VivaldiTester::new();

    // Reset to default settings
    remap_configuration.remap_config.default();

    // Serialize remap_config to binary bytes using bincode v2
    let config = config::standard();

    // Write to file
    let mut dumped_settings_file = File::create("croskbsettings.bin").expect("Failed to open file");

    let result = encode_into_std_write(
        &remap_configuration.remap_config,
        &mut dumped_settings_file,
        config,
    );
    match result {
        Err(e) => {
            println!("Unable to write to file {}", e);
        }
        Ok(_) => {}
    }

    Ok(())
}
fn check_sizes() {
    assert_eq!(std::mem::size_of::<RemapCfgKey>(), 4);
    assert_eq!(std::mem::size_of::<RemapCfgKeyState>(), 4);
    assert_eq!(std::mem::size_of::<RemapCfg>(), /* compute expected: 4*8 (enums) = 32 + 4 (original_key) +1 (bool) +4 (remapped_key) +32 (additional_keys) = 73 bytes? Actually: 8 enums *4 =32, original_key 4 =>36, bool 1 =>37, remapped_key 4 =>41, additional_keys 32 =>73. */ 73);
    // But C++ code warns "if sizeof(RemapCfg) != 73 -> warning". Indeed they check sizeof ==73.
    assert_eq!(std::mem::size_of::<RemapCfg>(), 73);

    // For header: sizeof header before cfg array:
    // magic(4) + remappings(4) + bool(1) + RemapCfgOverride(4) + RemapCfgOverride(4) = 17 bytes?
    // C++ code checks: if offsetof(RemapCfgs, cfg) != 17 => warning. Yes, 4+4+1+4+4 =17.
    assert_eq!(std::mem::size_of::<RemapCfgsHeader>(), 17);
}

