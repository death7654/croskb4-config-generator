use std::fs::File;
use std::io::Write;
use std::mem;

// Keycodes and constants
const K_LCTRL: u16 = 0x1D;
const K_LALT: u16 = 0x38;
const K_LSHFT: u16 = 0x2A;
const K_LWIN: u16 = 0x5B;
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

// C++ multi-char literal 'CrKB' on little-endian systems stores bytes as: 42 4B 72 43
// Which reads as "BKrC" in ASCII. We need to match this exact byte sequence.
const CFG_MAGIC: u32 = u32::from_le_bytes(*b"BKrC");

const FUNCTION_KEYS: [u16; 16] = [
    0x3B, 0x3C, 0x3D, 0x3E, 0x3F, 0x40, 0x41, 0x42, 0x43, 0x44, 0x57, 0x58,
    0x64, 0x65, 0x66, 0x67, // F13-F16
];

type RemapCfgKeyState = i32;
const KEY_STATE_NO_DETECT: i32 = 0;
const KEY_STATE_ENFORCE: i32 = 1;
const KEY_STATE_ENFORCE_NOT: i32 = 2;

type RemapCfgOverride = i32;
const REMAP_AUTO_DETECT: i32 = 0;
const REMAP_ENABLE: i32 = 1;
const REMAP_DISABLE: i32 = 2;

#[repr(C, packed(1))]
#[derive(Debug, Copy, Clone)]
struct RemapCfgKey {
    make_code: u16,
    flags: u16,
}

impl RemapCfgKey {
    const fn new() -> Self {
        Self {
            make_code: 0,
            flags: 0,
        }
    }

    const fn with_values(make_code: u16, flags: u16) -> Self {
        Self { make_code, flags }
    }
}

#[repr(C, packed(1))]
#[derive(Copy, Clone)]
struct RemapCfg {
    left_ctrl: i32,
    left_alt: i32,
    search: i32,
    assistant: i32,
    left_shift: i32,
    right_ctrl: i32,
    right_alt: i32,
    right_shift: i32,
    original_key: RemapCfgKey,
    remap_vivaldi_to_fn_keys: u8,
    remapped_key: RemapCfgKey,
    additional_keys: [RemapCfgKey; 8],
}

impl RemapCfg {
    fn new() -> Self {
        Self {
            left_ctrl: KEY_STATE_NO_DETECT,
            left_alt: KEY_STATE_NO_DETECT,
            search: KEY_STATE_NO_DETECT,
            assistant: KEY_STATE_NO_DETECT,
            left_shift: KEY_STATE_NO_DETECT,
            right_ctrl: KEY_STATE_NO_DETECT,
            right_alt: KEY_STATE_NO_DETECT,
            right_shift: KEY_STATE_NO_DETECT,
            original_key: RemapCfgKey::new(),
            remap_vivaldi_to_fn_keys: 0,
            remapped_key: RemapCfgKey::new(),
            additional_keys: [RemapCfgKey::new(); 8],
        }
    }
}

#[repr(C, packed(1))]
struct RemapCfgsHeader {
    magic: u32,
    remappings: u32,
    flip_search_and_assistant_on_pixelbook: u8,
    has_assistant_key: i32,
    is_non_chrome_ec: i32,
}

fn create_config() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    const NUM_CONFIGS: usize = 40;
    
    let header_size = mem::size_of::<RemapCfgsHeader>();
    let cfg_size = mem::size_of::<RemapCfg>();
    let total_size = header_size + cfg_size * NUM_CONFIGS;

    println!("Header size: {} (should be 17)", header_size);
    println!("RemapCfg size: {} (should be 73)", cfg_size);
    println!("Total size: {} bytes", total_size);

    if header_size != 17 {
        return Err(format!("ERROR: Header size is {}, expected 17!", header_size).into());
    }
    if cfg_size != 73 {
        return Err(format!("ERROR: RemapCfg size is {}, expected 73!", cfg_size).into());
    }

    let mut buffer = vec![0u8; total_size];
    
    // Write header
    unsafe {
        let header = buffer.as_mut_ptr() as *mut RemapCfgsHeader;
        (*header).magic = CFG_MAGIC;
        (*header).remappings = NUM_CONFIGS as u32;
        (*header).flip_search_and_assistant_on_pixelbook = 1;
        (*header).has_assistant_key = REMAP_AUTO_DETECT;
        (*header).is_non_chrome_ec = REMAP_AUTO_DETECT;
    }

    let cfg_array = unsafe {
        let ptr = buffer.as_mut_ptr().add(header_size);
        std::slice::from_raw_parts_mut(ptr as *mut RemapCfg, NUM_CONFIGS)
    };

    for cfg in cfg_array.iter_mut() {
        *cfg = RemapCfg::new();
    }

    // Map Vivaldi keys (without Ctrl) to F# keys (configs 0-18)
    let vivaldi_keys = [
        VIVALDI_BACK, VIVALDI_FWD, VIVALDI_REFRESH, VIVALDI_FULLSCREEN,
        VIVALDI_OVERVIEW, VIVALDI_SNAPSHOT, VIVALDI_BRIGHTNESS_DN,
        VIVALDI_BRIGHTNESS_UP, VIVALDI_PRIVACY_TOGGLE, VIVALDI_KBD_BKLIGHT_DOWN,
        VIVALDI_KBD_BKLIGHT_UP, VIVALDI_KBD_BKLIGHT_TOGGLE, VIVALDI_PLAY_PAUSE,
        VIVALDI_MUTE, VIVALDI_VOL_DN, VIVALDI_VOL_UP, VIVALDI_NEXT_TRACK,
        VIVALDI_PREV_TRACK, VIVALDI_MIC_MUTE,
    ];

    for (i, &key) in vivaldi_keys.iter().enumerate() {
        cfg_array[i].left_ctrl = KEY_STATE_ENFORCE_NOT;
        cfg_array[i].original_key = RemapCfgKey::with_values(key, KEY_E0);
        cfg_array[i].remap_vivaldi_to_fn_keys = 1;
    }

    // Config 19: Ctrl + Alt + Backspace -> Ctrl + Alt + Delete
    cfg_array[19].left_ctrl = KEY_STATE_ENFORCE;
    cfg_array[19].left_alt = KEY_STATE_ENFORCE;
    cfg_array[19].original_key = RemapCfgKey::with_values(K_BACKSP, 0);
    cfg_array[19].remapped_key = RemapCfgKey::with_values(K_DELETE, KEY_E0);

    // Config 20: Ctrl + Backspace -> Delete
    cfg_array[20].left_ctrl = KEY_STATE_ENFORCE;
    cfg_array[20].left_alt = KEY_STATE_ENFORCE_NOT;
    cfg_array[20].original_key = RemapCfgKey::with_values(K_BACKSP, 0);
    cfg_array[20].remapped_key = RemapCfgKey::with_values(K_DELETE, KEY_E0);
    cfg_array[20].additional_keys[0] = RemapCfgKey::with_values(K_LCTRL, KEY_BREAK);

    // Config 21: Ctrl + Fullscreen -> F11
    cfg_array[21].left_ctrl = KEY_STATE_ENFORCE;
    cfg_array[21].left_shift = KEY_STATE_ENFORCE_NOT;
    cfg_array[21].original_key = RemapCfgKey::with_values(VIVALDI_FULLSCREEN, KEY_E0);
    cfg_array[21].remapped_key = RemapCfgKey::with_values(FUNCTION_KEYS[10], 0);
    cfg_array[21].additional_keys[0] = RemapCfgKey::with_values(K_LCTRL, KEY_BREAK);

    // Config 22: Ctrl + Shift + Fullscreen -> Windows + P
    cfg_array[22].left_ctrl = KEY_STATE_ENFORCE;
    cfg_array[22].left_shift = KEY_STATE_ENFORCE;
    cfg_array[22].search = KEY_STATE_ENFORCE_NOT;
    cfg_array[22].original_key = RemapCfgKey::with_values(VIVALDI_FULLSCREEN, KEY_E0);
    cfg_array[22].remapped_key = RemapCfgKey::with_values(0x19, 0);
    cfg_array[22].additional_keys[0] = RemapCfgKey::with_values(K_LCTRL, KEY_BREAK);
    cfg_array[22].additional_keys[1] = RemapCfgKey::with_values(K_LSHFT, KEY_BREAK);
    cfg_array[22].additional_keys[2] = RemapCfgKey::with_values(K_LWIN, KEY_E0);

    // Config 23
    cfg_array[23].left_ctrl = KEY_STATE_ENFORCE;
    cfg_array[23].left_shift = KEY_STATE_ENFORCE;
    cfg_array[23].search = KEY_STATE_ENFORCE;
    cfg_array[23].original_key = RemapCfgKey::with_values(VIVALDI_FULLSCREEN, KEY_E0);
    cfg_array[23].remapped_key = RemapCfgKey::with_values(0x19, 0);
    cfg_array[23].additional_keys[0] = RemapCfgKey::with_values(K_LCTRL, KEY_BREAK);
    cfg_array[23].additional_keys[1] = RemapCfgKey::with_values(K_LSHFT, KEY_BREAK);

    // Config 24: Ctrl + Overview -> Windows + Tab
    cfg_array[24].left_ctrl = KEY_STATE_ENFORCE;
    cfg_array[24].left_shift = KEY_STATE_ENFORCE_NOT;
    cfg_array[24].search = KEY_STATE_ENFORCE_NOT;
    cfg_array[24].original_key = RemapCfgKey::with_values(VIVALDI_OVERVIEW, KEY_E0);
    cfg_array[24].remapped_key = RemapCfgKey::with_values(0x0F, 0);
    cfg_array[24].additional_keys[0] = RemapCfgKey::with_values(K_LCTRL, KEY_BREAK);
    cfg_array[24].additional_keys[1] = RemapCfgKey::with_values(K_LWIN, KEY_E0);

    // Config 25
    cfg_array[25].left_ctrl = KEY_STATE_ENFORCE;
    cfg_array[25].left_shift = KEY_STATE_ENFORCE_NOT;
    cfg_array[25].search = KEY_STATE_ENFORCE;
    cfg_array[25].original_key = RemapCfgKey::with_values(VIVALDI_OVERVIEW, KEY_E0);
    cfg_array[25].remapped_key = RemapCfgKey::with_values(0x0F, 0);
    cfg_array[25].additional_keys[0] = RemapCfgKey::with_values(K_LCTRL, KEY_BREAK);

    // Config 26: Ctrl + Shift + Overview -> Windows + Shift + S
    cfg_array[26].left_ctrl = KEY_STATE_ENFORCE;
    cfg_array[26].left_shift = KEY_STATE_ENFORCE;
    cfg_array[26].search = KEY_STATE_ENFORCE_NOT;
    cfg_array[26].original_key = RemapCfgKey::with_values(VIVALDI_OVERVIEW, KEY_E0);
    cfg_array[26].remapped_key = RemapCfgKey::with_values(0x1F, 0);
    cfg_array[26].additional_keys[0] = RemapCfgKey::with_values(K_LCTRL, KEY_BREAK);
    cfg_array[26].additional_keys[1] = RemapCfgKey::with_values(K_LWIN, KEY_E0);

    // Config 27
    cfg_array[27].left_ctrl = KEY_STATE_ENFORCE;
    cfg_array[27].left_shift = KEY_STATE_ENFORCE;
    cfg_array[27].search = KEY_STATE_ENFORCE;
    cfg_array[27].original_key = RemapCfgKey::with_values(VIVALDI_OVERVIEW, KEY_E0);
    cfg_array[27].remapped_key = RemapCfgKey::with_values(0x1F, 0);
    cfg_array[27].additional_keys[0] = RemapCfgKey::with_values(K_LCTRL, KEY_BREAK);

    // Config 28: Ctrl + Snapshot -> Windows + Shift + S
    cfg_array[28].left_ctrl = KEY_STATE_ENFORCE;
    cfg_array[28].left_shift = KEY_STATE_ENFORCE_NOT;
    cfg_array[28].search = KEY_STATE_ENFORCE_NOT;
    cfg_array[28].original_key = RemapCfgKey::with_values(VIVALDI_SNAPSHOT, KEY_E0);
    cfg_array[28].remapped_key = RemapCfgKey::with_values(0x1F, 0);
    cfg_array[28].additional_keys[0] = RemapCfgKey::with_values(K_LCTRL, KEY_BREAK);
    cfg_array[28].additional_keys[1] = RemapCfgKey::with_values(K_LWIN, KEY_E0);
    cfg_array[28].additional_keys[2] = RemapCfgKey::with_values(K_LSHFT, 0);

    // Config 29
    cfg_array[29].left_ctrl = KEY_STATE_ENFORCE;
    cfg_array[29].left_shift = KEY_STATE_ENFORCE_NOT;
    cfg_array[29].search = KEY_STATE_ENFORCE;
    cfg_array[29].original_key = RemapCfgKey::with_values(VIVALDI_SNAPSHOT, KEY_E0);
    cfg_array[29].remapped_key = RemapCfgKey::with_values(0x1F, 0);
    cfg_array[29].additional_keys[0] = RemapCfgKey::with_values(K_LCTRL, KEY_BREAK);
    cfg_array[29].additional_keys[1] = RemapCfgKey::with_values(K_LSHFT, 0);

    // Config 30
    cfg_array[30].left_ctrl = KEY_STATE_ENFORCE;
    cfg_array[30].left_shift = KEY_STATE_ENFORCE;
    cfg_array[30].search = KEY_STATE_ENFORCE_NOT;
    cfg_array[30].original_key = RemapCfgKey::with_values(VIVALDI_SNAPSHOT, KEY_E0);
    cfg_array[30].remapped_key = RemapCfgKey::with_values(0x1F, 0);
    cfg_array[30].additional_keys[0] = RemapCfgKey::with_values(K_LCTRL, KEY_BREAK);
    cfg_array[30].additional_keys[1] = RemapCfgKey::with_values(K_LWIN, KEY_E0);

    // Config 31
    cfg_array[31].left_ctrl = KEY_STATE_ENFORCE;
    cfg_array[31].left_shift = KEY_STATE_ENFORCE;
    cfg_array[31].search = KEY_STATE_ENFORCE;
    cfg_array[31].original_key = RemapCfgKey::with_values(VIVALDI_SNAPSHOT, KEY_E0);
    cfg_array[31].remapped_key = RemapCfgKey::with_values(0x1F, 0);
    cfg_array[31].additional_keys[0] = RemapCfgKey::with_values(K_LCTRL, KEY_BREAK);

    // Config 32-33: Ctrl + Alt + Brightness -> Ctrl + Alt + KB Brightness
    cfg_array[32].left_ctrl = KEY_STATE_ENFORCE;
    cfg_array[32].left_alt = KEY_STATE_ENFORCE;
    cfg_array[32].original_key = RemapCfgKey::with_values(VIVALDI_BRIGHTNESS_DN, KEY_E0);
    cfg_array[32].remapped_key = RemapCfgKey::with_values(VIVALDI_KBD_BKLIGHT_DOWN, KEY_E0);

    cfg_array[33].left_ctrl = KEY_STATE_ENFORCE;
    cfg_array[33].left_alt = KEY_STATE_ENFORCE;
    cfg_array[33].original_key = RemapCfgKey::with_values(VIVALDI_BRIGHTNESS_UP, KEY_E0);
    cfg_array[33].remapped_key = RemapCfgKey::with_values(VIVALDI_KBD_BKLIGHT_UP, KEY_E0);

    // Config 34-37: Ctrl + Arrow keys -> Home/End/PageUp/PageDown
    cfg_array[34].left_ctrl = KEY_STATE_ENFORCE;
    cfg_array[34].original_key = RemapCfgKey::with_values(K_LEFT, KEY_E0);
    cfg_array[34].remapped_key = RemapCfgKey::with_values(K_HOME, KEY_E0);
    cfg_array[34].additional_keys[0] = RemapCfgKey::with_values(K_LCTRL, KEY_BREAK);

    cfg_array[35].left_ctrl = KEY_STATE_ENFORCE;
    cfg_array[35].original_key = RemapCfgKey::with_values(K_RIGHT, KEY_E0);
    cfg_array[35].remapped_key = RemapCfgKey::with_values(K_END, KEY_E0);
    cfg_array[35].additional_keys[0] = RemapCfgKey::with_values(K_LCTRL, KEY_BREAK);

    cfg_array[36].left_ctrl = KEY_STATE_ENFORCE;
    cfg_array[36].original_key = RemapCfgKey::with_values(K_UP, KEY_E0);
    cfg_array[36].remapped_key = RemapCfgKey::with_values(K_PGUP, KEY_E0);
    cfg_array[36].additional_keys[0] = RemapCfgKey::with_values(K_LCTRL, KEY_BREAK);

    cfg_array[37].left_ctrl = KEY_STATE_ENFORCE;
    cfg_array[37].original_key = RemapCfgKey::with_values(K_DOWN, KEY_E0);
    cfg_array[37].remapped_key = RemapCfgKey::with_values(K_PGDN, KEY_E0);
    cfg_array[37].additional_keys[0] = RemapCfgKey::with_values(K_LCTRL, KEY_BREAK);

    // Config 38-39: Lock -> Windows + L
    cfg_array[38].search = KEY_STATE_ENFORCE_NOT;
    cfg_array[38].original_key = RemapCfgKey::with_values(K_LOCK, 0);
    cfg_array[38].remapped_key = RemapCfgKey::with_values(0x26, 0);
    cfg_array[38].additional_keys[0] = RemapCfgKey::with_values(K_LWIN, KEY_E0);

    cfg_array[39].search = KEY_STATE_ENFORCE;
    cfg_array[39].original_key = RemapCfgKey::with_values(K_LOCK, 0);
    cfg_array[39].remapped_key = RemapCfgKey::with_values(0x26, 0);

    Ok(buffer)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating Vivaldi keyboard configuration...\n");
    
    let config_data = create_config()?;
    
    // Check file size
    println!("Generated config size: {} bytes", config_data.len());
    println!("Expected: 17 (header) + 73 * 40 (configs) = 2937 bytes\n");
    
    let mut file = File::create("croskbsettings.bin")?;
    file.write_all(&config_data)?;
    
    println!("Successfully wrote {} bytes to croskbsettings.bin", config_data.len());
    
    Ok(())
}