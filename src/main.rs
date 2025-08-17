
// Crates
use bincode::{self, config};
use std::fs::File;
use std::ops::Rem;
use std::slice;
use mem_cmp::MemEq;
use serde::{Deserialize, Serialize};
use std::ptr;
use std::os::raw::c_ulong;



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
pub struct _KeyboardInputData
{
    UnitId: u16,
    MakeCode: u16,
    Flags: u16,
    Reserved: u16,
    ExtraInformation: u16,
}
pub type KeyboardInputData = _KeyboardInputData; 
pub type PkeyboardInputData = *mut _KeyboardInputData;


const FUNCTION_KEYS: [u8; 16] = [
    0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x40, 0x41, 0x42, 0x43, 0x44, 0x57, 0x58,
    // F13 - F16
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
struct _RemapCfgKey
{
    MakeCode: u16,
    Flags: u16
}

pub type RemapCfgKey = _RemapCfgKey;
pub type PRemapCfgKey = *mut _RemapCfgKey;

enum RemapCfgOverride
{
    RemapCfgOverrideAutoDetect,
    RemapCfgOverrideEnable,
    RemapCfgOverrideDisable
}

fn main()
{

}