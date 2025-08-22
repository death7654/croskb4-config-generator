// Crates
use bincode::{self, config};
use mem_cmp::MemEq;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::ops::Rem;
use std::os::raw::c_ulong;
use std::ptr;
use std::slice;

// Keycodes and constants
const LOCATION: &str = "C:/Windows/System32/drivers";

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

const FUNCTION_KEYS: [u8; 16] = [
    0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x40, 0x41, 0x42, 0x43, 0x44, 0x57, 0x58, // F13 - F16
    0x64, 0x64, 0x66, 0x67,
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

enum _RemapCfgOverride {
    RemapCfgOverrideAutoDetect,
    RemapCfgOverrideEnable,
    RemapCfgOverrideDisable,
}

pub type RemapCfgOverride = _RemapCfgOverride;
pub type PRemapCfgOverride = *mut _RemapCfgOverride;

enum _RemapCfgKeyState {
    RemapCfgKeyStateNoDetect,
    RemapCfgKeyStateEnforce,
    RemapCfgKeyStateEnforceNot,
}

pub type RemapCfgKeyState = _RemapCfgKeyState;
pub type PRemapCfgKeyState = *mut _RemapCfgKeyState;

#[repr(C, packed(1))]
#[derive(Debug, Copy, Clone)]
struct _RemapCfgKey {
    LeftCtrl: RemapCfgKeyState,
    LeftAlt: RemapCfgKeyState,
    Search: RemapCfgKeyState,
    Assistant: RemapCfgKeyState,
    LeftShift: RemapCfgKeyState,
    RightCtrl: RemapCfgKeyState,
    RightAlt: RemapCfgKeyState,
    RightShift: RemapCfgKeyState,

    originalKey: bool,

    remappedKey: RemapCfgKey,
    additionalKey: [RemapCfgKey; 8],
}

pub type RemapCfg = _RemapCfgKey;
pub type PRemapCfg = *mut _RemapCfgKey;

#[repr(C, packed(1))]
#[derive(Debug, Copy, Clone)]
struct _RemapCfgs {
    magic: u32,
    remap: u32,
    FlipSearchAndAssistantOnPixelbook: bool,
    HasAssistantKey: RemapCfgOverride,
    IsNonChromeEC: RemapCfgOverride,
    cfg: [RemapCfg; 1],
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
    legacyTopRowKeys: [u8; 10],
    legacyVivaldi: [u8; 10],

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

    currentKeys: [MAX_CURRENT_KEYS; KeyStruct],
    lastKeyPressed: KeyStruct,

    numKeysPressed: i32,

    remappedKeys: [MAX_CURRENT_KEYS; RemappedKeyStruct],
    numRemaps: i32,
}

impl VivaldiTester {
    //numkeyspressed should be 0
    fn updateKey(key: KeyStruct) {}
    fn addRemap(remap: RemappedKeyStruct) -> bool {}
    fn garbageCollect() {}

    fn checkKey(key: KeyboardInputData, report: [MAX_CURRENT_KEYS; KeyStruct]) -> bool {}

    fn addKey(key: KeyboardInputData, data: [MAX_CURRENT_KEYS; KeyboardInputData]) -> bool {}

    fn IdexOfFnKey(originalKey: RemapCfgKey) -> i32 {}

    fn RemapLoaded(
        report: [MAX_CURRENT_KEYS; KeyboardInputData],
        dataBefore: [MAX_CURRENT_KEYS; KeyboardInputData],
        dataAfter: [MAX_CURRENT_KEYS; KeyboardInputData],
    ) {
    }

    pub fn VivaldiTester(&self) {
        const LEGACY_VIVALDI: [u8; 10] = [
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

        const LEGACY_VIVALDI_PIXELBOOK: [u8; 10] = [
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

        RtlZeroMemory(self.currentKeys, mem::size_of_val(self.currentKeys));
        RtlZeroMemory(self.lastKeyPressed, mem::size_of_val(self.lastKeyPressed));

        RtlZeroMemory(self.remappedKeys, mem::size_of_val(self.remappedKeys));
        self.numRemaps = 0;

        self.functionRowCount = 0;
        RtlZeroMemory(self.functionRowKeys, mem::size_of_val(self.functionRowKeys));

        RtlCopyMemory(
            self.legacyTopRowKeys,
            FUNCTION_KEYS,
            mem::size_of_val(self.legacyTopRowKeys),
        );
        RtlCopyMemory(
            self.legacyVivaldi,
            LEGACY_VIVALDI,
            mem::size_of_val(self.legacyVivaldi),
        );

        self.functionRowCount = 13;

        const JINLON_KEYS: [u8; 13] = [
            VIVALDI_BACK,
            VIVALDI_REFRESH,
            VIVALDI_FULLSCREEN,
            VIVALDI_OVERVIEW,
            VIVALDI_SNAPSHOT,
            VIVALDI_BRIGHTNESSDN,
            VIVALDI_BRIGHTNESSUP,
            VIVALDI_KBD_BKLIGHT_DOWN,
            VIVALDI_KBD_BKLIGHT_UP,
            VIVALDI_PLAYPAUSE,
            VIVALDI_MUTE,
            VIVALDI_VOLDN,
            VIVALDI_VOLUP,
        ];

        for i in 0..size_of_val(JINLON_KEYS) {
            self.functionRowKeys[i].MakeCode = JINLON_KEYS[i];
            self.functionRowKeys[i].Flags = KEY_E0;
        }

        let cfg_size: usize =
            unsafe { &(*(std::ptr::null::<RemapCfgs>())).cfg as *const _ as usize }
                + std::mem::size_of::<RemapCfg>() * 40;

        const_assert!(offset_of!(RemapCfgs, cfg) == 17);
        const_assert!(mem::size_of::<RemapCfg>() == 73);

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
    }

    fn ServiceCallback(
        InputDataStart: PkeyboardInputData,
        InputDataEnd: PkeyboardInputData,
        InputDataConsumed: u32,
    ) {
    }
}

fn main() {}
