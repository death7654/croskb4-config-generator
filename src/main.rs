// Crates
use bincode::{self, config};
use mem_cmp::MemEq;
use serde::{Deserialize, Serialize};
use std::alloc::Layout;
use std::alloc::alloc;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::mem;
use std::mem::offset_of;
use std::os::raw::c_ulong;
use std::ptr;

use static_assertions::const_assert;

// local crates
use crate::_RemapCfgKeyState::RemapCfgKeyStateEnforce;
use crate::_RemapCfgKeyState::RemapCfgKeyStateEnforceNot;
use crate::_RemapCfgOverride::RemapCfgOverrideAutoDetect;

// Keycodes and constants
const LOCATION: &str = "C:/Windows/System32/drivers";

// Modifier and Special Keycodes
const K_LCTRL: u16 = 0x1D;
const K_LALT: u16 = 0x38;
const K_LSHFT: u16 = 0x2A;
const K_ASSISTANT: u16 = 0x58;
const K_LWIN: u16 = 0x5B;

const K_RCTRL: u16 = 0x1D;
const K_RALT: u16 = 0x38;
const K_RSHFT: u16 = 0x36;

const K_BACKSP: u16 = 0x0E;
const K_DELETE: u16 = 0x53;
const K_LOCK: u16 = 0x5D;

const K_UP: u16 = 0x48;
const K_DOWN: u16 = 0x50;
const K_LEFT: u16 = 0x4B;
const K_RIGHT: u16 = 0x4D;

const K_PGUP: u16 = 0x49;
const K_HOME: u16 = 0x47;
const K_END: u16 = 0x4F;
const K_PGDN: u16 = 0x51;

const K_NUMLCK: u16 = 0x45;

// Vivaldi Keycodes
const VIVALDI_BACK: u16 = 0x6A;
const VIVALDI_FWD: u16 = 0x69;
const VIVALDI_REFRESH: u16 = 0x67;
const VIVALDI_FULLSCREEN: u16 = 0x11;
const VIVALDI_OVERVIEW: u16 = 0x12;
const VIVALDI_SNAPSHOT: u16 = 0x13;
const VIVALDI_BRIGHTNESS_DN: u16 = 0x14;
const VIVALDI_BRIGHTNESS_UP: u16 = 0x15;
const VIVALDI_PRIVACY_TOGGLE: u16 = 0x16;
const VIVALDI_KBD_BKLIGHT_DOWN: u16 = 0x17;
const VIVALDI_KBD_BKLIGHT_UP: u16 = 0x18;
const VIVALDI_KBD_BKLIGHT_TOGGLE: u16 = 0x1E;
const VIVALDI_PLAY_PAUSE: u16 = 0x1A;
const VIVALDI_MUTE: u16 = 0x20;
const VIVALDI_VOL_DN: u16 = 0x2E;
const VIVALDI_VOL_UP: u16 = 0x30;
const VIVALDI_NEXT_TRACK: u16 = 0x19;
const VIVALDI_PREV_TRACK: u16 = 0x10;
const VIVALDI_MIC_MUTE: u16 = 0x1B;

const KEY_BREAK: u16 = 1;
const KEY_E0: u16 = 2;
const KEY_E1: u16 = 4;

const CFG_MAGIC: u32 = u32::from_le_bytes(*b"CrKB");

const MAX_CURRENT_KEYS: usize = 20;

//memory
// Replicates the RtlZeroMemory macro
fn RtlZeroMemory<T>(ptr: *mut T, count: usize) {
    // Safety: The caller must ensure that `ptr` is a valid,
    // writable pointer to `count` elements of type `T`.
    unsafe {
        ptr::write_bytes(ptr, 0, count);
    }
}

// Replicates the RtlCopyMemory macro
fn RtlCopyMemory<T>(src: *const T, dst: *mut T, count: usize) {
    // Safety: The caller must ensure that both `src` and `dst` are valid pointers
    // and that the memory regions they point to do not overlap.
    unsafe {
        ptr::copy_nonoverlapping(src, dst, count);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _KeyboardInputData {
    UnitId: u16,
    MakeCode: u16,
    Flags: u16,
    Reserved: u16,
    ExtraInformation: u16,
}
pub type KeyboardInputData = _KeyboardInputData;
pub type PkeyboardInputData = *mut _KeyboardInputData;

const FUNCTION_KEYS: [u16; 10] = [
    0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x40, 0x41, 0x42, 0x43,
    0x44, // 0x57, 0x58, // F13 - F16
         //0x64, 0x64, 0x66, 0x67,
];

pub type PULONG = c_ulong;

#[allow(non_snake_case)]
unsafe extern "C" {
    fn ReceiveKeys_Guarded(
        startPtr: *mut KeyboardInputData,
        endPtr: *mut KeyboardInputData,
        InputDataConsumed: *mut PULONG,
    );
}

#[repr(C, packed(1))]
#[derive(Debug, Copy, Clone)]
struct _RemapCfgKey {
    MakeCode: u16,
    Flags: u16,
}

pub type RemapCfgKey = _RemapCfgKey;
pub type PRemapCfgKey = *mut _RemapCfgKey;

#[derive(Debug, Copy, Clone)]
enum _RemapCfgOverride {
    RemapCfgOverrideAutoDetect,
    RemapCfgOverrideEnable,
    RemapCfgOverrideDisable,
}

pub type RemapCfgOverride = _RemapCfgOverride;
pub type PRemapCfgOverride = *mut _RemapCfgOverride;

#[derive(Debug, Copy, Clone)]
enum _RemapCfgKeyState {
    RemapCfgKeyStateNoDetect,
    RemapCfgKeyStateEnforce,
    RemapCfgKeyStateEnforceNot,
}

pub type RemapCfgKeyState = _RemapCfgKeyState;
pub type PRemapCfgKeyState = *mut _RemapCfgKeyState;

#[repr(C, packed(1))]
#[derive(Debug, Copy, Clone)]
struct _RemapCfg {
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

pub type RemapCfg = _RemapCfgKey;
pub type PRemapCfg = *mut _RemapCfgKey;

#[repr(C, packed(1))]
#[derive(Debug, Copy, Clone)]
struct _RemapCfgs {
    magic: u32,
    remappings: u32,
    FlipSearchAndAssistantOnPixelbook: bool,
    HasAssistantKey: RemapCfgOverride,
    IsNonChromeEC: RemapCfgOverride,
    cfg: [_RemapCfg; 1],
}

pub type RemapCfgs = _RemapCfgs;
pub type PRemapCfgs = *mut _RemapCfgs;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct _KeyStruct {
    MakeCode: u16,
    Flags: u16,
    InternalFlags: u16,
}

pub type KeyStruct = _KeyStruct;
pub type PRKeyStruct = *mut _KeyStruct;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct _RemappedKeyStruct {
    origKey: KeyStruct,
    remappedKey: KeyStruct,
}

pub type RemappedKeyStruct = _RemappedKeyStruct;
pub type PRemappedKeyStruc = *mut _RemappedKeyStruct;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct VivaldiTester {
    legacyTopRowKeys: [u16; 10],
    legacyVivaldi: [u16; 10],

    functionRowCount: u8,
    functionRowKeys: [KeyStruct; 16],

    remapCfgs: PRemapCfgs,

    LeftCtrlPressed: bool,
    LeftAltPressed: bool,
    LeftShiftPressed: bool,
    AssistantPressed: bool,
    SearchPressed: bool,

    RightCtrlPressed: bool,
    RightAltPressed: bool,
    RightShiftPressed: bool,

    currentKeys: [_KeyStruct; MAX_CURRENT_KEYS],
    lastKeyPressed: KeyStruct,

    numKeysPressed: i32,

    remappedKeys: [_RemappedKeyStruct; MAX_CURRENT_KEYS],
    numRemaps: i32,
}

impl VivaldiTester {
    //numkeyspressed should be 0
    fn updateKey(data: KeyStruct) {
        
    }
    fn addRemap(remap: RemappedKeyStruct) -> bool {
        return false;
    }
    fn garbageCollect() {}

    fn checkKey(key: KeyboardInputData, report: [KeyStruct; MAX_CURRENT_KEYS]) -> bool {
        return false;
    }

    fn addKey(key: KeyboardInputData, data: [KeyboardInputData; MAX_CURRENT_KEYS]) -> bool {
        return false;
    }

    fn IdexOfFnKey(originalKey: RemapCfgKey) -> i32 {
        return 0;
    }

    fn RemapLoaded(
        report: [KeyboardInputData; MAX_CURRENT_KEYS],
        dataBefore: [KeyboardInputData; MAX_CURRENT_KEYS],
        dataAfter: [KeyboardInputData; MAX_CURRENT_KEYS],
    ) {
    }

    pub fn VivaldiTester(&mut self, configs: usize) {
        const LEGACY_VIVALDI: [u16; 10] = [
            VIVALDI_BACK,
            VIVALDI_FWD,
            VIVALDI_REFRESH,
            VIVALDI_FULLSCREEN,
            VIVALDI_OVERVIEW,
            VIVALDI_BRIGHTNESS_DN,
            VIVALDI_BRIGHTNESS_UP,
            VIVALDI_MUTE,
            VIVALDI_VOL_DN,
            VIVALDI_VOL_UP,
        ];

        const LEGACY_VIVALDI_PIXELBOOK: [u16; 10] = [
            VIVALDI_BACK,
            VIVALDI_REFRESH,
            VIVALDI_FULLSCREEN,
            VIVALDI_OVERVIEW,
            VIVALDI_BRIGHTNESS_DN,
            VIVALDI_BRIGHTNESS_UP,
            VIVALDI_PLAY_PAUSE,
            VIVALDI_MUTE,
            VIVALDI_VOL_DN,
            VIVALDI_VOL_UP,
        ];
        self.numKeysPressed = 0;

        let key_pointer: *mut [_KeyStruct; MAX_CURRENT_KEYS] = &mut self.currentKeys;
        RtlZeroMemory(key_pointer, mem::size_of_val(&key_pointer));

        let last_key_pointer: *mut _KeyStruct = &mut self.lastKeyPressed;
        RtlZeroMemory(last_key_pointer, mem::size_of_val(&last_key_pointer));

        let remapped_key_pointer: *mut [_RemappedKeyStruct; MAX_CURRENT_KEYS] =
            &mut self.remappedKeys;
        RtlZeroMemory(
            remapped_key_pointer,
            mem::size_of_val(&remapped_key_pointer),
        );
        self.numRemaps = 0;

        let function_row_key_pointer: *mut [_KeyStruct; 16] = &mut self.functionRowKeys;
        RtlZeroMemory(
            function_row_key_pointer,
            mem::size_of_val(&function_row_key_pointer),
        );
        self.functionRowCount = 0;

        let legacy_top_row_key_pointer: *mut [u16; 10] = &mut self.legacyTopRowKeys;
        let function_key_pointer: *mut [u16; 10] = &mut FUNCTION_KEYS;
        RtlCopyMemory(
            legacy_top_row_key_pointer,
            function_key_pointer,
            mem::size_of_val(&legacy_top_row_key_pointer),
        );
        let legacy_vivaldi_pointer: *mut [u16; 10] = &mut self.legacyVivaldi;
        let const_legacy_vivaldi_pointer: *mut [u16; 10] = &mut LEGACY_VIVALDI;
        RtlCopyMemory(
            legacy_vivaldi_pointer,
            const_legacy_vivaldi_pointer,
            mem::size_of_val(&self.legacyVivaldi),
        );

        self.functionRowCount = 13;

        const JINLON_KEYS: [u16; 13] = [
            VIVALDI_BACK,
            VIVALDI_REFRESH,
            VIVALDI_FULLSCREEN,
            VIVALDI_OVERVIEW,
            VIVALDI_SNAPSHOT,
            VIVALDI_BRIGHTNESS_DN,
            VIVALDI_BRIGHTNESS_UP,
            VIVALDI_KBD_BKLIGHT_DOWN,
            VIVALDI_KBD_BKLIGHT_UP,
            VIVALDI_PLAY_PAUSE,
            VIVALDI_MUTE,
            VIVALDI_VOL_DN,
            VIVALDI_VOL_UP,
        ];

        for i in 0..size_of_val(&JINLON_KEYS) {
            self.functionRowKeys[i].MakeCode = JINLON_KEYS[i] as u16;
            self.functionRowKeys[i].Flags = KEY_E0;
        }

        let cfg_size: usize =
            unsafe { &(*(std::ptr::null::<RemapCfgs>())).cfg as *const _ as usize }
                + std::mem::size_of::<RemapCfg>() * 40;

        //const_assert!(offset_of!(RemapCfgs, cfg) == 17);
        //const_assert!(mem::size_of::<RemapCfg>() == 73);

        let cfg_size: usize =
            unsafe { &(*(std::ptr::null::<RemapCfgs>())).cfg as *const _ as usize }
                + mem::size_of::<RemapCfg>() * 40;

        let remap_cfgs: *mut RemapCfgs = unsafe {
            // Define the memory layout based on size and alignment.
            let layout = Layout::from_size_align(cfg_size, mem::align_of::<RemapCfgs>()).unwrap();

            // Allocate the raw memory block.
            let raw_ptr = alloc(layout);

            // Check for allocation
            if raw_ptr.is_null() {
                panic!("Failed to allocate memory");
            }

            // Return the raw pointer, cast to the correct type.
            raw_ptr as *mut RemapCfgs
        };
        RtlZeroMemory(remap_cfgs, cfg_size);

        let cfg_offset = offset_of!(RemapCfgs, cfg);
        let total_size = cfg_offset + mem::size_of::<RemapCfg>() * configs;

        let layout = Layout::from_size_align(total_size, mem::align_of::<RemapCfgs>()).unwrap();
        let raw = unsafe { alloc(layout) as *mut RemapCfgs };
        let remapCfgs: &mut RemapCfgs = unsafe { &mut *raw };

        remapCfgs.magic = CFG_MAGIC;
        remapCfgs.FlipSearchAndAssistantOnPixelbook = true;
        remapCfgs.HasAssistantKey = RemapCfgOverrideAutoDetect;
        remapCfgs.IsNonChromeEC = RemapCfgOverrideAutoDetect;
        remapCfgs.remappings = total_size as u32;

        remapCfgs.cfg[0].LeftCtrl = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[0].originalKey.MakeCode = VIVALDI_BACK;
        remapCfgs.cfg[0].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[0].remapVivaldiToFnKeys = true;

        remapCfgs.cfg[1].LeftCtrl = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[1].originalKey.MakeCode = VIVALDI_FWD;
        remapCfgs.cfg[1].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[1].remapVivaldiToFnKeys = true;

        remapCfgs.cfg[2].LeftCtrl = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[2].originalKey.MakeCode = VIVALDI_REFRESH;
        remapCfgs.cfg[2].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[2].remapVivaldiToFnKeys = true;

        remapCfgs.cfg[3].LeftCtrl = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[3].originalKey.MakeCode = VIVALDI_FULLSCREEN;
        remapCfgs.cfg[3].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[3].remapVivaldiToFnKeys = true;

        remapCfgs.cfg[4].LeftCtrl = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[4].originalKey.MakeCode = VIVALDI_OVERVIEW;
        remapCfgs.cfg[4].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[4].remapVivaldiToFnKeys = true;

        remapCfgs.cfg[5].LeftCtrl = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[5].originalKey.MakeCode = VIVALDI_SNAPSHOT;
        remapCfgs.cfg[5].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[5].remapVivaldiToFnKeys = true;

        remapCfgs.cfg[6].LeftCtrl = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[6].originalKey.MakeCode = VIVALDI_BRIGHTNESS_DN;
        remapCfgs.cfg[6].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[6].remapVivaldiToFnKeys = true;

        remapCfgs.cfg[7].LeftCtrl = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[7].originalKey.MakeCode = VIVALDI_BRIGHTNESS_UP;
        remapCfgs.cfg[7].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[7].remapVivaldiToFnKeys = true;

        remapCfgs.cfg[8].LeftCtrl = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[8].originalKey.MakeCode = VIVALDI_PRIVACY_TOGGLE;
        remapCfgs.cfg[8].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[8].remapVivaldiToFnKeys = true;

        remapCfgs.cfg[9].LeftCtrl = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[9].originalKey.MakeCode = VIVALDI_KBD_BKLIGHT_DOWN;
        remapCfgs.cfg[9].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[9].remapVivaldiToFnKeys = true;

        remapCfgs.cfg[10].LeftCtrl = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[10].originalKey.MakeCode = VIVALDI_KBD_BKLIGHT_UP;
        remapCfgs.cfg[10].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[10].remapVivaldiToFnKeys = true;

        remapCfgs.cfg[11].LeftCtrl = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[11].originalKey.MakeCode = VIVALDI_KBD_BKLIGHT_TOGGLE;
        remapCfgs.cfg[11].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[11].remapVivaldiToFnKeys = true;

        remapCfgs.cfg[12].LeftCtrl = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[12].originalKey.MakeCode = VIVALDI_PLAY_PAUSE;
        remapCfgs.cfg[12].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[12].remapVivaldiToFnKeys = true;

        remapCfgs.cfg[13].LeftCtrl = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[13].originalKey.MakeCode = VIVALDI_MUTE;
        remapCfgs.cfg[13].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[13].remapVivaldiToFnKeys = true;

        remapCfgs.cfg[14].LeftCtrl = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[14].originalKey.MakeCode = VIVALDI_VOL_DN;
        remapCfgs.cfg[14].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[14].remapVivaldiToFnKeys = true;

        remapCfgs.cfg[15].LeftCtrl = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[15].originalKey.MakeCode = VIVALDI_VOL_UP;
        remapCfgs.cfg[15].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[15].remapVivaldiToFnKeys = true;

        remapCfgs.cfg[16].LeftCtrl = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[16].originalKey.MakeCode = VIVALDI_NEXT_TRACK;
        remapCfgs.cfg[16].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[16].remapVivaldiToFnKeys = true;

        remapCfgs.cfg[17].LeftCtrl = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[17].originalKey.MakeCode = VIVALDI_PREV_TRACK;
        remapCfgs.cfg[17].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[17].remapVivaldiToFnKeys = true;

        remapCfgs.cfg[18].LeftCtrl = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[18].originalKey.MakeCode = VIVALDI_MIC_MUTE;
        remapCfgs.cfg[18].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[18].remapVivaldiToFnKeys = true;

        // ctrl + alt + backspace => ctrl + alt + delete

        remapCfgs.cfg[19].LeftCtrl = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[19].LeftAlt = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[19].originalKey.MakeCode = K_BACKSP;
        remapCfgs.cfg[19].originalKey.Flags = 0;
        remapCfgs.cfg[19].remappedKey.MakeCode = K_DELETE;
        remapCfgs.cfg[19].remappedKey.Flags = KEY_E0;

        // ctrl + backspace . delete

        remapCfgs.cfg[20].LeftCtrl = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[20].LeftAlt = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[20].originalKey.MakeCode = K_BACKSP;
        remapCfgs.cfg[20].originalKey.Flags = 0;
        remapCfgs.cfg[20].remappedKey.MakeCode = K_DELETE;
        remapCfgs.cfg[20].remappedKey.Flags = KEY_E0;
        remapCfgs.cfg[20].additionalKeys[0].MakeCode = K_LCTRL;
        remapCfgs.cfg[20].additionalKeys[0].Flags = KEY_BREAK;

        // ctrl + fullscreen => F11

        remapCfgs.cfg[21].LeftCtrl = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[21].LeftShift = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[21].originalKey.MakeCode = VIVALDI_FULLSCREEN;
        remapCfgs.cfg[21].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[21].remappedKey.MakeCode = FUNCTION_KEYS[10];
        remapCfgs.cfg[21].additionalKeys[0].MakeCode = K_LCTRL;
        remapCfgs.cfg[21].additionalKeys[0].Flags = KEY_BREAK;

        // ctrl + shift + fullscreen . windows + p

        remapCfgs.cfg[22].LeftCtrl = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[22].LeftShift = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[22].Search = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[22].originalKey.MakeCode = VIVALDI_FULLSCREEN;
        remapCfgs.cfg[22].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[22].remappedKey.MakeCode = 0x19;
        remapCfgs.cfg[22].additionalKeys[0].MakeCode = K_LCTRL;
        remapCfgs.cfg[22].additionalKeys[0].Flags = KEY_BREAK;
        remapCfgs.cfg[22].additionalKeys[1].MakeCode = K_LSHFT;
        remapCfgs.cfg[22].additionalKeys[1].Flags = KEY_BREAK;
        remapCfgs.cfg[22].additionalKeys[2].MakeCode = K_LWIN;
        remapCfgs.cfg[22].additionalKeys[2].Flags = KEY_E0;

        remapCfgs.cfg[23].LeftCtrl = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[23].LeftShift = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[23].Search = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[23].originalKey.MakeCode = VIVALDI_FULLSCREEN;
        remapCfgs.cfg[23].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[23].remappedKey.MakeCode = 0x19;
        remapCfgs.cfg[23].additionalKeys[0].MakeCode = K_LCTRL;
        remapCfgs.cfg[23].additionalKeys[0].Flags = KEY_BREAK;
        remapCfgs.cfg[23].additionalKeys[1].MakeCode = K_LSHFT;
        remapCfgs.cfg[23].additionalKeys[1].Flags = KEY_BREAK;

        //Map Ctrl + Overview . Windows + Tab

        remapCfgs.cfg[24].LeftCtrl = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[24].LeftShift = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[24].Search = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[24].originalKey.MakeCode = VIVALDI_OVERVIEW;
        remapCfgs.cfg[24].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[24].remappedKey.MakeCode = 0x0F;
        remapCfgs.cfg[24].additionalKeys[0].MakeCode = K_LCTRL;
        remapCfgs.cfg[24].additionalKeys[0].Flags = KEY_BREAK;
        remapCfgs.cfg[24].additionalKeys[1].MakeCode = K_LWIN;
        remapCfgs.cfg[24].additionalKeys[1].Flags = KEY_E0;

        remapCfgs.cfg[25].LeftCtrl = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[25].LeftShift = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[25].Search = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[25].originalKey.MakeCode = VIVALDI_OVERVIEW;
        remapCfgs.cfg[25].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[25].remappedKey.MakeCode = 0x0F;
        remapCfgs.cfg[25].additionalKeys[0].MakeCode = K_LCTRL;
        remapCfgs.cfg[25].additionalKeys[0].Flags = KEY_BREAK;

        //Map Ctrl + Shift + Overview . Windows + Shift + S

        remapCfgs.cfg[26].LeftCtrl = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[26].LeftShift = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[26].Search = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[26].originalKey.MakeCode = VIVALDI_OVERVIEW;
        remapCfgs.cfg[26].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[26].remappedKey.MakeCode = 0x1F;
        remapCfgs.cfg[26].additionalKeys[0].MakeCode = K_LCTRL;
        remapCfgs.cfg[26].additionalKeys[0].Flags = KEY_BREAK;
        remapCfgs.cfg[26].additionalKeys[1].MakeCode = K_LWIN;
        remapCfgs.cfg[26].additionalKeys[1].Flags = KEY_E0;

        remapCfgs.cfg[27].LeftCtrl = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[27].LeftShift = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[27].Search = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[27].originalKey.MakeCode = VIVALDI_OVERVIEW;
        remapCfgs.cfg[27].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[27].remappedKey.MakeCode = 0x1F;
        remapCfgs.cfg[27].additionalKeys[0].MakeCode = K_LCTRL;
        remapCfgs.cfg[27].additionalKeys[0].Flags = KEY_BREAK;

        //Map Ctrl + Snapshot . Windows + Shift + S

        remapCfgs.cfg[28].LeftCtrl = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[28].LeftShift = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[28].Search = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[28].originalKey.MakeCode = VIVALDI_SNAPSHOT;
        remapCfgs.cfg[28].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[28].remappedKey.MakeCode = 0x1F;
        remapCfgs.cfg[28].additionalKeys[0].MakeCode = K_LCTRL;
        remapCfgs.cfg[28].additionalKeys[0].Flags = KEY_BREAK;
        remapCfgs.cfg[28].additionalKeys[1].MakeCode = K_LWIN;
        remapCfgs.cfg[28].additionalKeys[1].Flags = KEY_E0;
        remapCfgs.cfg[28].additionalKeys[2].MakeCode = K_LSHFT;

        remapCfgs.cfg[29].LeftCtrl = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[29].LeftShift = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[29].Search = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[29].originalKey.MakeCode = VIVALDI_SNAPSHOT;
        remapCfgs.cfg[29].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[29].remappedKey.MakeCode = 0x1F;
        remapCfgs.cfg[29].additionalKeys[0].MakeCode = K_LCTRL;
        remapCfgs.cfg[29].additionalKeys[0].Flags = KEY_BREAK;
        remapCfgs.cfg[29].additionalKeys[1].MakeCode = K_LSHFT;

        remapCfgs.cfg[30].LeftCtrl = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[30].LeftShift = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[30].Search = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[30].originalKey.MakeCode = VIVALDI_SNAPSHOT;
        remapCfgs.cfg[30].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[30].remappedKey.MakeCode = 0x1F;
        remapCfgs.cfg[30].additionalKeys[0].MakeCode = K_LCTRL;
        remapCfgs.cfg[30].additionalKeys[0].Flags = KEY_BREAK;
        remapCfgs.cfg[30].additionalKeys[1].MakeCode = K_LWIN;
        remapCfgs.cfg[30].additionalKeys[1].Flags = KEY_E0;

        remapCfgs.cfg[31].LeftCtrl = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[31].LeftShift = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[31].Search = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[31].originalKey.MakeCode = VIVALDI_SNAPSHOT;
        remapCfgs.cfg[31].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[31].remappedKey.MakeCode = 0x1F;
        remapCfgs.cfg[31].additionalKeys[0].MakeCode = K_LCTRL;
        remapCfgs.cfg[31].additionalKeys[0].Flags = KEY_BREAK;

        //Ctrl + Alt + Brightness . Ctrl + Alt + KB Brightness

        remapCfgs.cfg[32].LeftCtrl = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[32].LeftAlt = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[32].originalKey.MakeCode = VIVALDI_BRIGHTNESS_DN;
        remapCfgs.cfg[32].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[32].remappedKey.MakeCode = VIVALDI_KBD_BKLIGHT_DOWN;
        remapCfgs.cfg[32].remappedKey.Flags = KEY_E0;

        remapCfgs.cfg[33].LeftCtrl = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[33].LeftAlt = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[33].originalKey.MakeCode = VIVALDI_BRIGHTNESS_UP;
        remapCfgs.cfg[33].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[33].remappedKey.MakeCode = VIVALDI_KBD_BKLIGHT_UP;
        remapCfgs.cfg[33].remappedKey.Flags = KEY_E0;

        //Ctrl + Left . Home

        remapCfgs.cfg[34].LeftCtrl = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[34].originalKey.MakeCode = K_LEFT;
        remapCfgs.cfg[34].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[34].remappedKey.MakeCode = K_HOME;
        remapCfgs.cfg[34].remappedKey.Flags = KEY_E0;
        remapCfgs.cfg[34].additionalKeys[0].MakeCode = K_LCTRL;
        remapCfgs.cfg[34].additionalKeys[0].Flags = KEY_BREAK;

        //Ctrl + Right . End

        remapCfgs.cfg[35].LeftCtrl = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[35].originalKey.MakeCode = K_RIGHT;
        remapCfgs.cfg[35].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[35].remappedKey.MakeCode = K_END;
        remapCfgs.cfg[35].remappedKey.Flags = KEY_E0;
        remapCfgs.cfg[35].additionalKeys[0].MakeCode = K_LCTRL;
        remapCfgs.cfg[35].additionalKeys[0].Flags = KEY_BREAK;

        //Ctrl + Up . Page Up

        remapCfgs.cfg[36].LeftCtrl = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[36].originalKey.MakeCode = K_UP;
        remapCfgs.cfg[36].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[36].remappedKey.MakeCode = K_PGUP;
        remapCfgs.cfg[36].remappedKey.Flags = KEY_E0;
        remapCfgs.cfg[36].additionalKeys[0].MakeCode = K_LCTRL;
        remapCfgs.cfg[36].additionalKeys[0].Flags = KEY_BREAK;

        //Ctrl + Down . Page Down

        remapCfgs.cfg[37].LeftCtrl = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[37].originalKey.MakeCode = K_DOWN;
        remapCfgs.cfg[37].originalKey.Flags = KEY_E0;
        remapCfgs.cfg[37].remappedKey.MakeCode = K_PGDN;
        remapCfgs.cfg[37].remappedKey.Flags = KEY_E0;
        remapCfgs.cfg[37].additionalKeys[0].MakeCode = K_LCTRL;
        remapCfgs.cfg[37].additionalKeys[0].Flags = KEY_BREAK;

        //Lock . Windows + L

        remapCfgs.cfg[38].Search = RemapCfgKeyStateEnforceNot;
        remapCfgs.cfg[38].originalKey.MakeCode = K_LOCK;
        remapCfgs.cfg[38].originalKey.Flags = 0;
        remapCfgs.cfg[38].remappedKey.MakeCode = 0x26;
        remapCfgs.cfg[38].additionalKeys[0].MakeCode = K_LWIN;
        remapCfgs.cfg[38].additionalKeys[0].Flags = KEY_E0;

        remapCfgs.cfg[39].Search = RemapCfgKeyStateEnforce;
        remapCfgs.cfg[39].originalKey.MakeCode = K_LOCK;
        remapCfgs.cfg[39].originalKey.Flags = 0;
        remapCfgs.cfg[39].remappedKey.MakeCode = 0x26;

        self.remapCfgs = remapCfgs;

        println!("Initialized\n");

        let dumped_settings_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("croskbsettings.bin");

        let file_path = "croskbsettings.bin";
        let cfg_size = total_size;

        let file_result = File::create(file_path);

        match file_result {
            Ok(mut dumped_settings_file) => {
                if let Err(e) = dumped_settings_file.write_all(&remap_cfgs[..cfg_size]) {
                    println!(
                        "Warning: Failed to write settings to croskeyboard4! Error: {}",
                        e
                    );
                } else {
                    println!("Wrote active settings to {}!", file_path);
                }
            }
            Err(e) => {
                println!(
                    "Warning: Failed to write settings for croskeyboard4! Check that your permissions are correct! Error: {}",
                    e
                );
            }
        }
    }



    fn ServiceCallback(
        InputDataStart: PkeyboardInputData,
        InputDataEnd: PkeyboardInputData,
        InputDataConsumed: u32,
    ) {
    }
}

fn main() {}
