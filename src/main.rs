// Crates
use bincode::{self, config};
use mem_cmp::MemEq;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::mem;
use std::ops::Rem;
use std::os::raw::c_ulong;
use std::ptr;
use std::slice;

use static_assertions::const_assert;


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

const FUNCTION_KEYS: [u8; 10] = [
    0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x40, 0x41, 0x42, 0x43, 0x44,// 0x57, 0x58, // F13 - F16
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

    currentKeys: [_KeyStruct; MAX_CURRENT_KEYS],
    lastKeyPressed: KeyStruct,

    numKeysPressed: i32,

    remappedKeys: [_RemappedKeyStruct; MAX_CURRENT_KEYS],
    numRemaps: i32,
}

impl VivaldiTester {
    //numkeyspressed should be 0
    fn updateKey(key: KeyStruct) {}
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
        dataBefore:[KeyboardInputData; MAX_CURRENT_KEYS],
        dataAfter:[KeyboardInputData; MAX_CURRENT_KEYS],
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

        let  key_pointer: *mut [_KeyStruct; MAX_CURRENT_KEYS] =   &mut self.currentKeys;
        RtlZeroMemory(key_pointer, mem::size_of_val(&key_pointer));

        let last_key_pointer: *mut _KeyStruct = &mut self.lastKeyPressed;
        RtlZeroMemory(last_key_pointer, mem::size_of_val(&last_key_pointer));

        let remapped_key_pointer: *mut [_RemappedKeyStruct; MAX_CURRENT_KEYS] = &mut self.remappedKeys;
        RtlZeroMemory(remapped_key_pointer, mem::size_of_val(&remapped_key_pointer));
        self.numRemaps = 0;

        let function_row_key_pointer: *mut [_KeyStruct; 16] = &mut self.functionRowKeys;
        RtlZeroMemory(function_row_key_pointer, mem::size_of_val(&function_row_key_pointer));
        self.functionRowCount = 0;


        let legacy_top_row_key_pointer: *mut [u8; 10] = &mut self.legacyTopRowKeys;
        let function_key_pointer: *mut [u8; 10] = &mut FUNCTION_KEYS;
        RtlCopyMemory(
            legacy_top_row_key_pointer,
            function_key_pointer,
            mem::size_of_val(&legacy_top_row_key_pointer),
        );
        let legacy_vivaldi_pointer: *mut [u8; 10] = &mut self.legacyVivaldi;
        let const_legacy_vivaldi_pointer:*mut [u8;10] = &mut LEGACY_VIVALDI;
        RtlCopyMemory(
            legacy_vivaldi_pointer,
            const_legacy_vivaldi_pointer,
            mem::size_of_val(&self.legacyVivaldi),
        );

        self.functionRowCount = 13;

        const JINLON_KEYS: [u8; 13] = [
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

        self.remapCfgs.magic = CFG_MAGIC;
        self.remapCfgs.FlipSearchAndAssistantOnPixelbook = true;
        self.remapCfgs.HasAssistantKey = RemapCfgOverrideAutoDetect;
    }

    fn ServiceCallback(
        InputDataStart: PkeyboardInputData,
        InputDataEnd: PkeyboardInputData,
        InputDataConsumed: u32,
    ) {
    }
}

fn main() {}
