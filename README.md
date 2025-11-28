# Chromebook Keyboard Config Generator (Rust)

A Rust implementation of the Chromebook keyboard configuration file generator, producing binary-compatible output with the original C++ implementation from [VivaldiKeyboardTester](https://github.com/coolstar/VivaldiKeyboardTester).

## Overview

This tool generates `croskbsettings.bin`, a configuration file used by the [CrosKeyboard4](https://github.com/coolstar/croskeyboard4) driver to remap Chromebook keyboard keys on Windows. The Rust implementation produces byte-for-byte identical output to the original C++ code.

## Features

- **Binary-compatible**: Generates identical configuration files to the C++ implementation
- **Documented**: Clear mapping of key remapping configurations

## Key Mappings
- This will go into the Chrultrabook Tools application as the keyboard remapping solution


### Prerequisites
- Rust toolchain (install from [rustup.rs](https://rustup.rs/))

### Building
```bash
cargo build
```

### Running
```bash
cargo run
```

This will generate `croskbsettings.bin` in the current directory.

## Usage

1. Run the program to generate the configuration file:
   ```bash
   cargo run
   ```
2. Copy `croskbsettings.bin` to the appropriate location for your CrosKeyboard4 driver installation usually `C:\Windows\System32\drivers\`.
3. Reboot or run croskbreload.exe

## Binary Format

The configuration file consists of:
- **Header** (17 bytes):
  - Magic number: `'CrKB'` (0x4B724243)
  - Number of remappings: 40
  
- **40 Configuration Entries** (73 bytes each):
  - Modifier key states (Ctrl, Alt, Shift, Search, Assistant)
  - Original key mapping
  - Remapped key
  - Additional keys to inject
  - Vivaldi-to-Fn key flag

Total size: 2,937 bytes

## Implementation Notes

### Memory Layout Compatibility

The Rust implementation uses `#[repr(C, packed(1))]` to ensure exact memory layout matching the C++ structs:

```rust
#[repr(C, packed(1))]
struct RemapCfg {
    left_ctrl: i32,           // 4 bytes
    left_alt: i32,            // 4 bytes
    search: i32,              // 4 bytes
    assistant: i32,           // 4 bytes
    left_shift: i32,          // 4 bytes
    right_ctrl: i32,          // 4 bytes
    right_alt: i32,           // 4 bytes
    right_shift: i32,         // 4 bytes
    original_key: RemapCfgKey,      // 4 bytes
    remap_vivaldi_to_fn_keys: u8,   // 1 byte
    remapped_key: RemapCfgKey,      // 4 bytes
    additional_keys: [RemapCfgKey; 8], // 32 bytes
}
// Total: 73 bytes
```

### Magic Number Handling

The C++ code uses a multi-character literal `'CrKB'` which has implementation-defined behavior. On little-endian systems, this produces the byte sequence `42 4B 72 43` ("BKrC"). The Rust implementation correctly replicates this:

```rust
const CFG_MAGIC: u32 = u32::from_le_bytes(*b"BKrC");
```

## Verification

To verify the generated file matches the C++ output:

### Windows (Command Prompt)
```cmd
fc /b croskbsettings.bin croskbsettings_cpp.bin
```

### Windows (PowerShell)
```powershell
(Get-FileHash croskbsettings.bin).Hash -eq (Get-FileHash croskbsettings_cpp.bin).Hash
```
## Credits

- **Original C++ Implementation**: [coolstar/VivaldiKeyboardTester](https://github.com/coolstar/VivaldiKeyboardTester)
- **CrosKeyboard4 Driver**: [coolstar/croskeyboard4](https://github.com/coolstar/croskeyboard4)
- **Rust Port**: This implementation

## License

This project maintains compatibility with the original VivaldiKeyboardTester project. Please refer to the original repository for licensing information.

## Related Projects

- [CrosKeyboard4](https://github.com/coolstar/croskeyboard4) - Windows keyboard driver for Chromebooks
- [VivaldiKeyboardTester](https://github.com/coolstar/VivaldiKeyboardTester) - Original C++ configuration generator and testing tool

## Contributing

Contributions are welcome! Please ensure that any changes maintain binary compatibility with the C++ implementation. Test by comparing the generated files byte-by-byte.

