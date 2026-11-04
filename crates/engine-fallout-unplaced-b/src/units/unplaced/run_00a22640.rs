//! `(unplaced)/run_00a22640` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The functions here work on the PC input object (0x1C04 bytes, built by
//! `00a22660` from `Main`): DirectInput at +0x08, its first device at +0x2C
//! and second device at +0x30, flags at +0x04 and the control bindings read
//! from and written to the `Controls` section of the preferences file. The
//! class has no Xbox PDB name (the Xbox build has no DirectInput); fields are
//! described by what the code does with them.

#[allow(unused_imports)]
use crate::prelude::*;

/// `memset` (CRT, cdecl).
const MEMSET: u32 = 0x00ec_61c0;
/// The `DirectInput8Create` thunk (`JMP [00fdf02c]`; stdcall, five words).
const DIRECT_INPUT8_CREATE: u32 = 0x00ee_7f10;
/// The interface identifier passed to `DirectInput8Create`.
const DIRECT_INPUT_INTERFACE_ID: u32 = 0x0110_0ed4;
/// The device GUID of the device kept at +0x2C.
const FIRST_DEVICE_GUID: u32 = 0x0110_0f84;
/// The data format set on the device at +0x2C.
const FIRST_DEVICE_FORMAT: u32 = 0x0110_0cb4;
/// The device GUID of the device kept at +0x30.
const SECOND_DEVICE_GUID: u32 = 0x0110_0f74;
/// The data format set on the device at +0x30.
const SECOND_DEVICE_FORMAT: u32 = 0x0110_0aac;
/// The setting whose byte value (+4) makes the constructor call `00a252d0`
/// with 1.
const SETTING_011F35D0: u32 = 0x011f_35d0;
/// The setting whose byte value (+4) picks cooperative level 0x16 instead
/// of 0x15 for the device at +0x2C.
const SETTING_011F35DC: u32 = 0x011f_35dc;
/// The setting whose byte value (+4) sets flag 8.
const SETTING_011F35E8: u32 = 0x011f_35e8;
/// The input object's method `00a252d0` (thiscall, one word).
const INPUT_METHOD_A252D0: u32 = 0x00a2_52d0;
/// The input object's method `00a24b70` (thiscall, one word).
const INPUT_METHOD_A24B70: u32 = 0x00a2_4b70;
/// The input object's binding getter `00a24f10` (thiscall: control index).
const BINDING_GET: u32 = 0x00a2_4f10;
/// The input object's binding setter `00a24f60` (thiscall: control index,
/// value).
const BINDING_SET: u32 = 0x00a2_4f60;
/// Constructor of the 0x40-byte object kept at +0x1B80 (thiscall, one word).
const OBJECT_1B80_CONSTRUCTOR: u32 = 0x00a2_62d0;
/// Method `00a26310` of the object at +0x1B80 (thiscall, two words).
const OBJECT_1B80_METHOD: u32 = 0x00a2_6310;
/// `GetActiveWindow` (import slot).
const IMPORT_GET_ACTIVE_WINDOW: u32 = 0x00fd_f2fc;
/// `RegOpenKeyExA` (import slot).
const IMPORT_REG_OPEN_KEY_EX: u32 = 0x00fd_f00c;
/// `RegQueryValueExA` (import slot).
const IMPORT_REG_QUERY_VALUE_EX: u32 = 0x00fd_f008;
/// `RegCloseKey` (import slot).
const IMPORT_REG_CLOSE_KEY: u32 = 0x00fd_f004;
/// `SwapMouseButton` (import slot).
const IMPORT_SWAP_MOUSE_BUTTON: u32 = 0x00fd_f2c4;
/// `GetDoubleClickTime` (import slot).
const IMPORT_GET_DOUBLE_CLICK_TIME: u32 = 0x00fd_f2c0;
/// `GetPrivateProfileStringA` (import slot).
const IMPORT_GET_PRIVATE_PROFILE_STRING: u32 = 0x00fd_f164;
/// `WritePrivateProfileStringA` (import slot).
const IMPORT_WRITE_PRIVATE_PROFILE_STRING: u32 = 0x00fd_f0bc;
/// `HKEY_CURRENT_USER`.
const HKEY_CURRENT_USER: u32 = 0x8000_0001;
/// `KEY_ALL_ACCESS`, the access the key is opened with.
const KEY_ALL_ACCESS: u32 = 0x000f_003f;
/// The string `Control Panel\Mouse`.
const MOUSE_KEY_NAME: u32 = 0x0109_5880;
/// The string `MouseSensitivity`.
const MOUSE_SENSITIVITY_NAME: u32 = 0x0109_586c;
/// `atol` (CRT, cdecl).
const ATOL: u32 = 0x00ec_a6d3;
/// `strtol` (CRT, cdecl; engine map `strtoxl` unit).
const STRTOL: u32 = 0x00ec_8d08;
/// `strcpy_s` (CRT, cdecl).
const STRCPY_S: u32 = 0x00ec_65a6;
/// `strcat_s` (CRT, cdecl).
const STRCAT_S: u32 = 0x00ec_6bd2;
/// The formatter `00406d00` (cdecl: buffer, size, format, arguments).
const FORMAT_STRING: u32 = 0x0040_6d00;
/// The folder string the preferences path starts with.
const PREFERENCES_FOLDER: u32 = 0x0120_2fa0;
/// The word holding the preferences file name's address.
const PREFERENCES_FILE_NAME: u32 = 0x011a_2ff8;
/// The string `Controls` (the profile section).
const CONTROLS_SECTION: u32 = 0x0105_e724;
/// The 28 control names (string addresses), one word each.
const CONTROL_NAMES: u32 = 0x011a_7e60;
/// The format `%08X`.
const BINDING_FORMAT: u32 = 0x0109_5894;
/// Number of controls the loops walk.
const CONTROL_COUNT: u32 = 0x1c;
/// The memory manager object the code passes as `this` to its allocator.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// The memory manager's allocation method (thiscall, one word: the size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// The memory manager's free method (thiscall, one word: the block).
const MEMORY_FREE: u32 = 0x00aa_4060;
/// The cached language index `00a22cb0` fills (-1 until set).
const LANGUAGE_INDEX: u32 = 0x011a_7ed0;
/// The setting whose string value (+4) names the language.
const SETTING_011C3CB4: u32 = 0x011c_3cb4;
/// The case-insensitive compare of at most n characters `00ec7ec0`
/// (`_strnicmp` by Ghidra's name; cdecl, three words).
const STRNICMP: u32 = 0x00ec_7ec0;
/// The language names and the index each one gives (string address,
/// length compared, index), in the order the code tries them.
const LANGUAGES: [(u32, u32, u32); 5] = [
    // "ENGLISH"
    (0x0101_8348, 7, 0),
    // "FRENCH"
    (0x0101_fd00, 6, 2),
    // "ITALIAN"
    (0x0101_fcf0, 7, 4),
    // "GERMAN"
    (0x0101_fd08, 6, 1),
    // "SPANISH"
    (0x0101_fcf8, 7, 3),
];

/// The inlined check `setting != NULL && setting.value == 1` on a byte
/// setting (value at +4). The address is a global's, so never null, but the
/// code tests it.
fn byte_setting_is_one(e: &Engine, setting: u32) -> bool {
    if setting != 0 {
        e.mem.u8(setting + 4) == 1
    } else {
        false
    }
}

/// The inlined read of a string setting's value (+4), 0 for a null setting
/// (never the case: the address is a global's).
fn string_setting_value(e: &Engine, setting: u32) -> u32 {
    if setting != 0 {
        e.mem.u32(setting + 4)
    } else {
        0
    }
}

/// Builds the preferences file path in the 0x104-byte buffer at `path`:
/// `strcpy_s` of the folder, then `strcat_s` of the file name.
fn build_preferences_path(e: &mut Engine, path: u32) {
    e.call(STRCPY_S, &[path, 0x104, PREFERENCES_FOLDER]);
    let file_name = e.mem.u32(PREFERENCES_FILE_NAME);
    e.call(STRCAT_S, &[path, 0x104, file_name]);
}

// Translated from 00a22660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The input object's constructor (unnamed in the engine map): creates
/// DirectInput (`DirectInput8Create(instance, 0x800, ...)` into +0x08),
/// creates the first device (+0x2C, flag 4; cooperative level 0x15, or 0x16
/// when the setting at `011f35dc` is 1; its data format; a property with a
/// value of 0x20) and the second device (+0x30, flag 2; its capabilities
/// into +0x1AF8; cooperative level 5, or 6 with flag 8, which the setting at
/// `011f35e8` sets; its data format). Reads `MouseSensitivity` from
/// `HKEY_CURRENT_USER\Control Panel\Mouse` into byte +0x1B50 (6 when
/// missing), the mouse-button swap state (+0x1B4C, restored at once) and the
/// double-click time (+0x1B54); clears the state blocks, builds the 0x40-byte
/// object at +0x1B80 and the 0x24-byte block at +0x1B84, and calls
/// `00a24b70(4)`. Returns `this`.
// Translation incomplete: 00a22660 - C++ exception unwinding (SEH frame state) and the /GS stack cookie check (00ec408c) are not translated.
pub fn fn_00a22660(e: &mut Engine, this: u32, instance: u32) -> u32 {
    // The frame from EBP-0x44 to EBP: `local(k)` is EBP-k.
    e.with_stack(0x44, |e, frame| {
        let local = |k: u32| frame.addr() + 0x44 - k;
        e.mem.set_u32(this + 0x18f4, 0);
        e.mem.set_u32(this + 4, 0);
        e.mem.set_u16(this + 0x1b92, 0);
        e.call(
            DIRECT_INPUT8_CREATE,
            &[instance, 0x800, DIRECT_INPUT_INTERFACE_ID, this + 8, 0],
        );
        if byte_setting_is_one(e, SETTING_011F35D0) {
            e.call(INPUT_METHOD_A252D0, &[this, 1]);
        }
        e.mem.set_u32(this + 0x2c, 0);
        let direct_input = e.mem.u32(this + 8);
        let result = e
            .vcall(direct_input, 0xc, &[FIRST_DEVICE_GUID, local(0x14), 0])
            .u32();
        e.mem.set_u32(local(0x18), result);
        if e.mem.u32(local(0x18)) == 0 {
            let flags = e.mem.u32(this + 4);
            e.mem.set_u32(this + 4, flags | 4);
            let device = e.mem.u32(local(0x14));
            e.mem.set_u32(this + 0x2c, device);
            let level = if byte_setting_is_one(e, SETTING_011F35DC) {
                0x16
            } else {
                0x15
            };
            let window = e.call(IMPORT_GET_ACTIVE_WINDOW, &[]).u32();
            let device = e.mem.u32(this + 0x2c);
            e.vcall(device, 0x34, &[window, level]);
            let device = e.mem.u32(this + 0x2c);
            e.vcall(device, 0x2c, &[FIRST_DEVICE_FORMAT]);
            // A DIPROPDWORD-shaped block: sizes 0x14 and 0x10, two zero
            // words, value 0x20.
            e.mem.set_u32(local(0x2c), 0x14);
            e.mem.set_u32(local(0x28), 0x10);
            e.mem.set_u32(local(0x20), 0);
            e.mem.set_u32(local(0x24), 0);
            e.mem.set_u32(local(0x1c), 0x20);
            let device = e.mem.u32(this + 0x2c);
            e.vcall(device, 0x18, &[1, local(0x2c)]);
        }
        e.mem.set_u32(this + 0x30, 0);
        e.mem.set_u8(this + 0x1b50, 6);
        e.mem.set_u32(local(0x10), 0);
        let opened = e
            .call(
                IMPORT_REG_OPEN_KEY_EX,
                &[
                    HKEY_CURRENT_USER,
                    MOUSE_KEY_NAME,
                    0,
                    KEY_ALL_ACCESS,
                    local(0x10),
                ],
            )
            .u32();
        if opened == 0 {
            e.mem.set_u32(local(0x30), 10);
            let key = e.mem.u32(local(0x10));
            let queried = e
                .call(
                    IMPORT_REG_QUERY_VALUE_EX,
                    &[
                        key,
                        MOUSE_SENSITIVITY_NAME,
                        0,
                        local(0x34),
                        local(0x44),
                        local(0x30),
                    ],
                )
                .u32();
            if queried == 0 {
                let value = e.call(ATOL, &[local(0x44)]).u32();
                e.mem.set_u8(this + 0x1b50, value as u8);
            }
            let key = e.mem.u32(local(0x10));
            e.call(IMPORT_REG_CLOSE_KEY, &[key]);
        }
        let swapped = e.call(IMPORT_SWAP_MOUSE_BUTTON, &[0]).u32();
        e.mem.set_u32(this + 0x1b4c, swapped);
        let swapped = e.mem.u32(this + 0x1b4c);
        e.call(IMPORT_SWAP_MOUSE_BUTTON, &[swapped]);
        let double_click = e.call(IMPORT_GET_DOUBLE_CLICK_TIME, &[]).u32();
        e.mem.set_u32(this + 0x1b54, double_click);
        e.call(MEMSET, &[this + 0x1b60, 0, 0x20]);
        e.call(MEMSET, &[this + 0x1af8, 0, 0x2c]);
        e.mem.set_u32(this + 0x1af8, 0x2c);
        if byte_setting_is_one(e, SETTING_011F35E8) {
            let flags = e.mem.u32(this + 4);
            e.mem.set_u32(this + 4, flags | 8);
        }
        let direct_input = e.mem.u32(this + 8);
        let result = e
            .vcall(direct_input, 0xc, &[SECOND_DEVICE_GUID, local(0x14), 0])
            .u32();
        e.mem.set_u32(local(0x18), result);
        if e.mem.u32(local(0x18)) == 0 {
            let flags = e.mem.u32(this + 4);
            e.mem.set_u32(this + 4, flags | 2);
            let device = e.mem.u32(local(0x14));
            e.mem.set_u32(this + 0x30, device);
            let device = e.mem.u32(this + 0x30);
            e.vcall(device, 0xc, &[this + 0x1af8]);
            let level = if e.mem.u32(this + 4) & 8 != 0 { 6 } else { 5 };
            let window = e.call(IMPORT_GET_ACTIVE_WINDOW, &[]).u32();
            let device = e.mem.u32(this + 0x30);
            let result = e.vcall(device, 0x34, &[window, level]).u32();
            e.mem.set_u32(local(0x18), result);
            // Three failure codes are folded to 1 in the local; nothing
            // reads it afterwards.
            if matches!(
                e.mem.u32(local(0x18)),
                0x8007_0006 | 0x8007_0015 | 0x8007_0057
            ) {
                e.mem.set_u32(local(0x18), 1);
            }
            let device = e.mem.u32(this + 0x30);
            e.vcall(device, 0x2c, &[SECOND_DEVICE_FORMAT]);
        }
        e.call(MEMSET, &[this + 0x34, 0, 0x500]);
        e.call(MEMSET, &[this + 0x18f8, 0, 0x200]);
        e.call(MEMSET, &[this + 0x1b24, 0, 0x28]);
        e.mem.set_u8(this + 0x1b90, 0);
        e.mem.set_u8(this + 0x1b91, 0);
        let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 0x40]).u32();
        let object = if block != 0 {
            e.call(OBJECT_1B80_CONSTRUCTOR, &[block, 0]).u32()
        } else {
            0
        };
        e.mem.set_u32(this + 0x1b80, object);
        let block = e.call(MEMORY_ALLOCATE, &[MEMORY_MANAGER, 0x24]).u32();
        let cleared = if block != 0 {
            e.call(MEMSET, &[block, 0, 0x24]);
            block
        } else {
            0
        };
        e.mem.set_u32(this + 0x1b84, cleared);
        let block = e.mem.u32(this + 0x1b84);
        e.mem.set_u32(block + 8, 0);
        let block = e.mem.u32(this + 0x1b84);
        e.mem.set_u32(block, 0);
        let block = e.mem.u32(this + 0x1b84);
        e.mem.set_u32(block + 4, 2);
        let block = e.mem.u32(this + 0x1b84);
        e.mem.set_u32(block + 0x14, 0);
        let block = e.mem.u32(this + 0x1b84);
        e.mem.set_u32(block + 0x1c, 0);
        let block = e.mem.u32(this + 0x1b84);
        let object = e.mem.u32(this + 0x1b80);
        e.call(OBJECT_1B80_METHOD, &[object, 1, block]);
        let block = e.mem.u32(this + 0x1b84);
        let object = e.mem.u32(this + 0x1b80);
        e.call(OBJECT_1B80_METHOD, &[object, 0, block]);
        e.mem.set_u8(this + 0x1b88, 1);
        e.call(INPUT_METHOD_A24B70, &[this, 4]);
        e.mem.set_u8(this, 0);
        this
    })
}

// Translated from 00a22bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The input object's release (unnamed in the engine map): with flag 1 calls
/// `00a252d0(0)`; unacquires (vtable +0x20) and releases (+0x08) the devices
/// at +0x2C and +0x30 that are set, releases DirectInput (+0x08), and frees
/// the block at +0x1B84 when set.
pub fn fn_00a22bf0(e: &mut Engine, this: u32) {
    if e.mem.u32(this + 4) & 1 != 0 {
        e.call(INPUT_METHOD_A252D0, &[this, 0]);
    }
    for device in [0x2c, 0x30] {
        if e.mem.u32(this + device) != 0 {
            let object = e.mem.u32(this + device);
            e.vcall(object, 0x20, &[]);
            let object = e.mem.u32(this + device);
            e.vcall(object, 8, &[]);
        }
    }
    let direct_input = e.mem.u32(this + 8);
    e.vcall(direct_input, 8, &[]);
    if e.mem.u32(this + 0x1b84) != 0 {
        let block = e.mem.u32(this + 0x1b84);
        e.call(MEMORY_FREE, &[MEMORY_MANAGER, block]);
    }
}

// Translated from 00a22cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The language index (unnamed in the engine map): while the cached index
/// at `011a7ed0` is -1, compares the language setting's string (`011c3cb4`
/// +4) case-insensitively with `ENGLISH` (0), `FRENCH` (2), `ITALIAN` (4),
/// `GERMAN` (1) and `SPANISH` (3) and caches the first match's index (no
/// match keeps -1). Returns the cached index.
pub fn fn_00a22cb0(e: &mut Engine) -> i32 {
    if e.mem.u32(LANGUAGE_INDEX) == u32::MAX {
        for (name, length, index) in LANGUAGES {
            let value = string_setting_value(e, SETTING_011C3CB4);
            if e.call(STRNICMP, &[value, name, length]).u32() == 0 {
                e.mem.set_u32(LANGUAGE_INDEX, index);
                break;
            }
        }
    }
    e.mem.u32(LANGUAGE_INDEX) as i32
}

// Translated from 00a22e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the 28 control bindings (unnamed in the engine map): builds the
/// preferences path, then for each control formats `00a24f10(index)` as
/// `%08X` and writes it under the control's name in the `Controls`
/// section (`WritePrivateProfileStringA`).
// Translation incomplete: 00a22e10 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_00a22e10(e: &mut Engine, this: u32) {
    // The frame from EBP-0x160 to EBP: `local(k)` is EBP-k.
    e.with_stack(0x160, |e, frame| {
        let local = |k: u32| frame.addr() + 0x160 - k;
        let path = local(0x110);
        let text = local(0x158);
        build_preferences_path(e, path);
        for index in 0..CONTROL_COUNT {
            let binding = e.call(BINDING_GET, &[this, index]).u32();
            e.call(FORMAT_STRING, &[text, 0x40, BINDING_FORMAT, binding]);
            let name = e.mem.u32(CONTROL_NAMES + index * 4);
            e.call(
                IMPORT_WRITE_PRIVATE_PROFILE_STRING,
                &[CONTROLS_SECTION, name, text, path],
            );
        }
    })
}

// Translated from 00a22ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the 28 control bindings (unnamed in the engine map): builds the
/// preferences path, then for each control reads its value from the
/// `Controls` section (`GetPrivateProfileStringA`, 0x40 bytes, no default)
/// and, when one was read, sets `00a24f60(index, strtol(value, .., 16))`.
// Translation incomplete: 00a22ef0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_00a22ef0(e: &mut Engine, this: u32) {
    // The frame from EBP-0x168 to EBP: `local(k)` is EBP-k.
    e.with_stack(0x168, |e, frame| {
        let local = |k: u32| frame.addr() + 0x168 - k;
        let path = local(0x110);
        let value = local(0x158);
        let end = local(0x160);
        build_preferences_path(e, path);
        for index in 0..CONTROL_COUNT {
            let size = 0x40;
            let name = e.mem.u32(CONTROL_NAMES + index * 4);
            let read = e
                .call(
                    IMPORT_GET_PRIVATE_PROFILE_STRING,
                    &[CONTROLS_SECTION, name, 0, value, size, path],
                )
                .u32();
            if read != 0 {
                e.mem.set_u32(end, 0);
                let binding = e.call(STRTOL, &[value, end, 0x10]).u32();
                e.call(BINDING_SET, &[this, index, binding]);
            }
        }
    })
}

/// `BSStringT<char>`'s set from a C string `004037f0` (thiscall, two words; so other units
/// describe it).
const STRING_SET_TEXT: u32 = 0x0040_37f0;

// Translated from 00a22640 (decompiled, FalloutNV.exe 1.4.0.525)
/// No references in Ghidra (thiscall, no stack words; it sits just before the constructor
/// `00a22660`): the string set `004037f0` on the member at +0x38 with two zero words. Ends in
/// that call, so its `EAX` is the answer.
pub fn fn_00a22640(e: &mut Engine, this: u32) -> u32 {
    e.call(STRING_SET_TEXT, &[this.wrapping_add(0x38), 0, 0])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a2_2640, fn_00a22640(u32) -> u32),
        entry!(0x00a22660, fn_00a22660(u32, u32) -> u32),
        entry!(0x00a22bf0, fn_00a22bf0(u32)),
        entry!(0x00a22cb0, fn_00a22cb0() -> i32),
        entry!(0x00a22e10, fn_00a22e10(u32)),
        entry!(0x00a22ef0, fn_00a22ef0(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    // Addresses of the COM doubles.
    const CREATE_DEVICE: u32 = 0x0077_1000;
    const SET_COOPERATIVE_LEVEL: u32 = 0x0077_1004;
    const SET_DATA_FORMAT: u32 = 0x0077_1008;
    const SET_PROPERTY: u32 = 0x0077_100c;
    const GET_CAPABILITIES: u32 = 0x0077_1010;
    const UNACQUIRE: u32 = 0x0077_1014;
    const RELEASE: u32 = 0x0077_1018;
    const DEVICE_VTABLE: u32 = 0x0077_2000;
    const DIRECT_INPUT_VTABLE: u32 = 0x0077_3000;
    const DIRECT_INPUT: u32 = 0x0077_4000;
    const FIRST_DEVICE: u32 = 0x0077_4100;
    const SECOND_DEVICE: u32 = 0x0077_4200;

    /// A vtable of 0x40 bytes with `slots` filled.
    fn put_slots(e: &mut Engine, table: u32, slots: &[(u32, u32)]) {
        e.map(table, 0x40);
        for (slot, target) in slots {
            e.mem.set_u32(table + slot, *target);
        }
    }

    struct Setup {
        sensitivity: Option<&'static [u8]>,
        create_fails: [bool; 2],
        level_result: u32,
        settings: [u8; 3],
        fail_alloc: u32,
    }

    /// The words of the property block the first device receives.
    type Seen = Rc<RefCell<Vec<u32>>>;

    fn input_engine(s: &Setup) -> (Engine, Seen) {
        let mut e = Engine::new();
        e.map(0x011f_3000, 0x1000);
        e.mem.set_u8(SETTING_011F35D0 + 4, s.settings[0]);
        e.mem.set_u8(SETTING_011F35DC + 4, s.settings[1]);
        e.mem.set_u8(SETTING_011F35E8 + 4, s.settings[2]);
        put_slots(
            &mut e,
            DIRECT_INPUT_VTABLE,
            &[(0x8, RELEASE), (0xc, CREATE_DEVICE)],
        );
        put_slots(
            &mut e,
            DEVICE_VTABLE,
            &[
                (0x8, RELEASE),
                (0xc, GET_CAPABILITIES),
                (0x18, SET_PROPERTY),
                (0x20, UNACQUIRE),
                (0x2c, SET_DATA_FORMAT),
                (0x34, SET_COOPERATIVE_LEVEL),
            ],
        );
        e.map(DIRECT_INPUT, 0x300);
        e.mem.set_u32(DIRECT_INPUT, DIRECT_INPUT_VTABLE);
        e.mem.set_u32(FIRST_DEVICE, DEVICE_VTABLE);
        e.mem.set_u32(SECOND_DEVICE, DEVICE_VTABLE);
        e.register(DIRECT_INPUT8_CREATE, |e, a| {
            e.mem.set_u32(a[3], DIRECT_INPUT);
            ret(0)
        });
        let fails = s.create_fails;
        e.register_double(CREATE_DEVICE, move |e, a| {
            let (device, fail) = if a[1] == FIRST_DEVICE_GUID {
                (FIRST_DEVICE, fails[0])
            } else {
                (SECOND_DEVICE, fails[1])
            };
            if fail {
                ret(0x8000_4005)
            } else {
                e.mem.set_u32(a[2], device);
                ret(0)
            }
        });
        let level_result = s.level_result;
        e.register_double(SET_COOPERATIVE_LEVEL, move |_, _| ret(level_result));
        // The property block the first device receives.
        let property: Seen = Rc::default();
        let seen = property.clone();
        e.register_double(SET_PROPERTY, move |e, a| {
            *seen.borrow_mut() = (0..5).map(|i| e.mem.u32(a[2] + i * 4)).collect();
            ret(0)
        });
        e.register(SET_DATA_FORMAT, |_, _| ret(0));
        e.register(GET_CAPABILITIES, |_, _| ret(0));
        e.register(INPUT_METHOD_A252D0, |_, _| ret(0));
        e.register(INPUT_METHOD_A24B70, |_, _| ret(0));
        e.register(IMPORT_GET_ACTIVE_WINDOW, |_, _| ret(0x0000_beef));
        let sensitivity = s.sensitivity;
        e.register_double(IMPORT_REG_OPEN_KEY_EX, move |e, a| {
            if sensitivity.is_some() {
                e.mem.set_u32(a[4], 0x0000_0444);
                ret(0)
            } else {
                ret(2)
            }
        });
        e.register_double(IMPORT_REG_QUERY_VALUE_EX, move |e, a| {
            assert_eq!(e.mem.u32(a[5]), 10);
            match sensitivity {
                Some(text) if !text.is_empty() => {
                    e.mem.write(a[4], text);
                    ret(0)
                }
                _ => ret(2),
            }
        });
        e.register(IMPORT_REG_CLOSE_KEY, |_, _| ret(0));
        e.register(ATOL, |e, a| {
            let text = String::from_utf8(e.mem.cstr(a[0])).unwrap();
            ret(text.parse::<i32>().unwrap() as u32)
        });
        e.register(IMPORT_SWAP_MOUSE_BUTTON, |_, a| ret(a[0] ^ 1));
        e.register(IMPORT_GET_DOUBLE_CLICK_TIME, |_, _| ret(500));
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            ret(a[0])
        });
        let fail_alloc = s.fail_alloc;
        e.register_double(MEMORY_ALLOCATE, move |e, a| {
            if a[1] == fail_alloc {
                ret(0)
            } else {
                ret(e.mem.alloc(a[1]))
            }
        });
        e.register(OBJECT_1B80_CONSTRUCTOR, |_, a| ret(a[0]));
        e.register(OBJECT_1B80_METHOD, |_, _| ret(0));
        e.register(RELEASE, |_, _| ret(0));
        e.register(UNACQUIRE, |_, _| ret(0));
        (e, property)
    }

    #[test]
    fn constructor_creates_both_devices_and_reads_the_mouse_settings() {
        let (mut e, property) = input_engine(&Setup {
            sensitivity: Some(b"14\0"),
            create_fails: [false, false],
            level_result: 0x8007_0057,
            settings: [1, 1, 1],
            fail_alloc: u32::MAX,
        });
        let this = e.mem.alloc(0x1c04);
        e.mem.set_u8(this, 0xff);
        e.mem.set_u32(this + 0x18f4, 9);
        e.call_log = Some(vec![]);
        let result = e.call(0x00a2_2660, &[this, 0x0040_0000]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u8(this), 0);
        assert_eq!(e.mem.u32(this + 0x18f4), 0);
        assert_eq!(
            calls_to(&e, DIRECT_INPUT8_CREATE),
            vec![vec![
                0x0040_0000,
                0x800,
                DIRECT_INPUT_INTERFACE_ID,
                this + 8,
                0
            ]]
        );
        assert_eq!(calls_to(&e, INPUT_METHOD_A252D0), vec![vec![this, 1]]);
        assert_eq!(e.mem.u32(this + 4), 4 | 8 | 2);
        assert_eq!(e.mem.u32(this + 0x2c), FIRST_DEVICE);
        assert_eq!(e.mem.u32(this + 0x30), SECOND_DEVICE);
        assert_eq!(
            calls_to(&e, SET_COOPERATIVE_LEVEL),
            vec![
                vec![FIRST_DEVICE, 0xbeef, 0x16],
                vec![SECOND_DEVICE, 0xbeef, 6]
            ]
        );
        assert_eq!(
            calls_to(&e, SET_DATA_FORMAT),
            vec![
                vec![FIRST_DEVICE, FIRST_DEVICE_FORMAT],
                vec![SECOND_DEVICE, SECOND_DEVICE_FORMAT]
            ]
        );
        assert_eq!(calls_to(&e, SET_PROPERTY).len(), 1);
        assert_eq!(calls_to(&e, SET_PROPERTY)[0][1], 1);
        assert_eq!(*property.borrow(), vec![0x14, 0x10, 0, 0, 0x20]);
        assert_eq!(
            calls_to(&e, GET_CAPABILITIES),
            vec![vec![SECOND_DEVICE, this + 0x1af8]]
        );
        assert_eq!(e.mem.u32(this + 0x1af8), 0x2c);
        // Registry, mouse buttons, double-click time.
        let open = calls_to(&e, IMPORT_REG_OPEN_KEY_EX);
        assert_eq!(
            &open[0][..4],
            &[HKEY_CURRENT_USER, MOUSE_KEY_NAME, 0, KEY_ALL_ACCESS]
        );
        let query = calls_to(&e, IMPORT_REG_QUERY_VALUE_EX);
        assert_eq!(&query[0][..3], &[0x444, MOUSE_SENSITIVITY_NAME, 0]);
        assert_eq!(calls_to(&e, IMPORT_REG_CLOSE_KEY), vec![vec![0x444]]);
        assert_eq!(e.mem.u8(this + 0x1b50), 14);
        assert_eq!(
            calls_to(&e, IMPORT_SWAP_MOUSE_BUTTON),
            vec![vec![0], vec![1]]
        );
        assert_eq!(e.mem.u32(this + 0x1b4c), 1);
        assert_eq!(e.mem.u32(this + 0x1b54), 500);
        // The two blocks.
        let object = e.mem.u32(this + 0x1b80);
        let block = e.mem.u32(this + 0x1b84);
        assert_eq!(calls_to(&e, OBJECT_1B80_CONSTRUCTOR), vec![vec![object, 0]]);
        assert_eq!(
            calls_to(&e, OBJECT_1B80_METHOD),
            vec![vec![object, 1, block], vec![object, 0, block]]
        );
        assert_eq!(e.mem.u32(block + 4), 2);
        assert_eq!(e.mem.u32(block), 0);
        assert_eq!(e.mem.u8(this + 0x1b88), 1);
        assert_eq!(calls_to(&e, INPUT_METHOD_A24B70), vec![vec![this, 4]]);
        let cleared: Vec<Vec<u32>> = calls_to(&e, MEMSET)
            .into_iter()
            .filter(|a| a[0] != block)
            .collect();
        assert_eq!(
            cleared,
            vec![
                vec![this + 0x1b60, 0, 0x20],
                vec![this + 0x1af8, 0, 0x2c],
                vec![this + 0x34, 0, 0x500],
                vec![this + 0x18f8, 0, 0x200],
                vec![this + 0x1b24, 0, 0x28],
            ]
        );
    }

    #[test]
    fn constructor_without_devices_registry_or_object_keeps_the_defaults() {
        let (mut e, _) = input_engine(&Setup {
            sensitivity: Some(b""),
            create_fails: [true, true],
            level_result: 0,
            settings: [0, 0, 0],
            fail_alloc: 0x40,
        });
        let this = e.mem.alloc(0x1c04);
        e.call_log = Some(vec![]);
        e.call(0x00a2_2660, &[this, 7]);
        assert!(calls_to(&e, INPUT_METHOD_A252D0).is_empty());
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert!(calls_to(&e, SET_COOPERATIVE_LEVEL).is_empty());
        assert!(calls_to(&e, ATOL).is_empty());
        // The key opened but the value is missing: closed, default kept.
        assert_eq!(calls_to(&e, IMPORT_REG_CLOSE_KEY).len(), 1);
        assert_eq!(e.mem.u8(this + 0x1b50), 6);
        assert_eq!(e.mem.u32(this + 0x1b80), 0);
        assert!(calls_to(&e, OBJECT_1B80_CONSTRUCTOR).is_empty());
        assert_eq!(calls_to(&e, OBJECT_1B80_METHOD)[0][0], 0);
    }

    #[test]
    fn constructor_with_low_levels_and_no_key() {
        let (mut e, _) = input_engine(&Setup {
            sensitivity: None,
            create_fails: [false, false],
            level_result: 0,
            settings: [0, 0, 0],
            fail_alloc: u32::MAX,
        });
        let this = e.mem.alloc(0x1c04);
        e.call_log = Some(vec![]);
        e.call(0x00a2_2660, &[this, 7]);
        assert_eq!(
            calls_to(&e, SET_COOPERATIVE_LEVEL),
            vec![
                vec![FIRST_DEVICE, 0xbeef, 0x15],
                vec![SECOND_DEVICE, 0xbeef, 5]
            ]
        );
        assert_eq!(e.mem.u32(this + 4), 4 | 2);
        assert!(calls_to(&e, IMPORT_REG_QUERY_VALUE_EX).is_empty());
        assert!(calls_to(&e, IMPORT_REG_CLOSE_KEY).is_empty());
        assert_eq!(e.mem.u8(this + 0x1b50), 6);
    }

    #[test]
    fn release_unacquires_set_devices_and_frees_the_block() {
        let (mut e, _) = input_engine(&Setup {
            sensitivity: None,
            create_fails: [false, false],
            level_result: 0,
            settings: [0, 0, 0],
            fail_alloc: u32::MAX,
        });
        e.register(MEMORY_FREE, |_, _| ret(0));
        let this = e.mem.alloc(0x1c04);
        e.mem.set_u32(this + 4, 1);
        e.mem.set_u32(this + 8, DIRECT_INPUT);
        e.mem.set_u32(this + 0x2c, FIRST_DEVICE);
        e.mem.set_u32(this + 0x30, SECOND_DEVICE);
        e.mem.set_u32(this + 0x1b84, 0x0012_3450);
        e.call_log = Some(vec![]);
        e.call(0x00a2_2bf0, &[this]);
        assert_eq!(calls_to(&e, INPUT_METHOD_A252D0), vec![vec![this, 0]]);
        assert_eq!(
            calls_to(&e, UNACQUIRE),
            vec![vec![FIRST_DEVICE], vec![SECOND_DEVICE]]
        );
        assert_eq!(
            calls_to(&e, RELEASE),
            vec![vec![FIRST_DEVICE], vec![SECOND_DEVICE], vec![DIRECT_INPUT]]
        );
        assert_eq!(
            calls_to(&e, MEMORY_FREE),
            vec![vec![MEMORY_MANAGER, 0x0012_3450]]
        );
        // Flag 1 clear, no devices, no block: only DirectInput is released.
        let other = e.mem.alloc(0x1c04);
        e.mem.set_u32(other + 8, DIRECT_INPUT);
        e.call_log = Some(vec![]);
        e.call(0x00a2_2bf0, &[other]);
        assert!(calls_to(&e, INPUT_METHOD_A252D0).is_empty());
        assert!(calls_to(&e, UNACQUIRE).is_empty());
        assert_eq!(calls_to(&e, RELEASE), vec![vec![DIRECT_INPUT]]);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
    }

    /// An engine for the language lookup: the setting's string is `language`
    /// and the compare double is `_strnicmp`.
    fn language_engine(language: &[u8], cached: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011a_7000, 0x1000);
        e.map(0x011c_3000, 0x1000);
        e.map(0x0101_8000, 0x1000);
        e.map(0x0101_f000, 0x1000);
        for (name, text) in [
            (0x0101_8348, &b"ENGLISH\0"[..]),
            (0x0101_fd00, &b"FRENCH\0"[..]),
            (0x0101_fcf0, &b"ITALIAN\0"[..]),
            (0x0101_fd08, &b"GERMAN\0"[..]),
            (0x0101_fcf8, &b"SPANISH\0"[..]),
        ] {
            e.mem.write(name, text);
        }
        let value = e.mem.alloc(16);
        e.mem.write(value, language);
        e.mem.set_u32(SETTING_011C3CB4 + 4, value);
        e.mem.set_u32(LANGUAGE_INDEX, cached);
        e.register(STRNICMP, |e, a| {
            let left = e.mem.bytes(a[0], a[2]);
            let right = e.mem.bytes(a[1], a[2]);
            let mut result = 0;
            for (l, r) in left.iter().zip(&right) {
                let (l, r) = (l.to_ascii_lowercase(), r.to_ascii_lowercase());
                if l != r {
                    result = if l < r { u32::MAX } else { 1 };
                    break;
                }
                if l == 0 {
                    break;
                }
            }
            ret(result)
        });
        e
    }

    #[test]
    fn language_index_matches_each_name_and_caches_it() {
        for (language, index, compares) in [
            (&b"english\0"[..], 0, 1),
            (&b"French\0"[..], 2, 2),
            (&b"ITALIAN\0"[..], 4, 3),
            (&b"german\0"[..], 1, 4),
            (&b"Spanish\0"[..], 3, 5),
            (&b"POLISH\0"[..], -1, 5),
        ] {
            let mut e = language_engine(language, u32::MAX);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00a2_2cb0, &[]).u32() as i32, index);
            assert_eq!(calls_to(&e, STRNICMP).len(), compares);
            assert_eq!(e.mem.u32(LANGUAGE_INDEX) as i32, index);
        }
        // A cached index is returned without comparing.
        let mut e = language_engine(b"english\0", 3);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2_2cb0, &[]).u32(), 3);
        assert!(calls_to(&e, STRNICMP).is_empty());
    }

    /// An engine for the bindings' file functions: the preferences path is
    /// built by the `strcpy_s`/`strcat_s` doubles from a folder `C:\` and the
    /// file name `P.ini`; control `i`'s name is the string `K<i>`.
    fn bindings_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011a_2000, 0x1000);
        e.map(0x011a_7000, 0x1000);
        e.map(0x0120_2000, 0x1000);
        e.mem.write(PREFERENCES_FOLDER, b"C:\\\0");
        let file = e.mem.alloc(8);
        e.mem.write(file, b"P.ini\0");
        e.mem.set_u32(PREFERENCES_FILE_NAME, file);
        for i in 0..CONTROL_COUNT {
            let name = e.mem.alloc(8);
            e.mem.write(name, format!("K{i}\0").as_bytes());
            e.mem.set_u32(CONTROL_NAMES + i * 4, name);
        }
        e.register(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.write(a[0], &text);
            e.mem.set_u8(a[0] + text.len() as u32, 0);
            ret(0)
        });
        e.register(STRCAT_S, |e, a| {
            let start = a[0] + e.mem.cstr(a[0]).len() as u32;
            let text = e.mem.cstr(a[2]);
            e.mem.write(start, &text);
            e.mem.set_u8(start + text.len() as u32, 0);
            ret(0)
        });
        e
    }

    fn text_at(e: &Engine, addr: u32) -> String {
        String::from_utf8(e.mem.cstr(addr)).unwrap()
    }

    /// Key, value and file of each profile write.
    type Written = Rc<RefCell<Vec<(String, String, String)>>>;

    #[test]
    fn write_bindings_formats_each_control_into_the_controls_section() {
        let mut e = bindings_engine();
        e.register(BINDING_GET, |_, a| ret(a[1] * 0x10));
        e.register(FORMAT_STRING, |e, a| {
            assert_eq!(a[1], 0x40);
            let text = format!("{:08X}\0", a[3]);
            e.mem.write(a[0], text.as_bytes());
            ret(8)
        });
        let written: Written = Rc::default();
        let log = written.clone();
        e.register_double(IMPORT_WRITE_PRIVATE_PROFILE_STRING, move |e, a| {
            assert_eq!(a[0], CONTROLS_SECTION);
            log.borrow_mut()
                .push((text_at(e, a[1]), text_at(e, a[2]), text_at(e, a[3])));
            ret(1)
        });
        e.call_log = Some(vec![]);
        e.call(0x00a2_2e10, &[0x0055_0000]);
        assert_eq!(calls_to(&e, BINDING_GET).len(), 28);
        assert_eq!(calls_to(&e, BINDING_GET)[5], vec![0x0055_0000, 5]);
        assert!(calls_to(&e, FORMAT_STRING)
            .iter()
            .all(|a| a[2] == BINDING_FORMAT));
        let written = written.borrow();
        assert_eq!(written.len(), 28);
        assert_eq!(
            written[3],
            (
                "K3".to_string(),
                "00000030".to_string(),
                "C:\\P.ini".to_string()
            )
        );
        assert_eq!(written[27].1, "000001B0");
    }

    #[test]
    fn read_bindings_sets_only_the_controls_with_a_value() {
        let mut e = bindings_engine();
        e.register(IMPORT_GET_PRIVATE_PROFILE_STRING, |e, a| {
            assert_eq!(a[0], CONTROLS_SECTION);
            assert_eq!(a[2], 0);
            assert_eq!(a[4], 0x40);
            assert_eq!(text_at(e, a[5]), "C:\\P.ini");
            // Controls K2 and K10 have values.
            match text_at(e, a[1]).as_str() {
                "K2" => {
                    e.mem.write(a[3], b"1F\0");
                    ret(2)
                }
                "K10" => {
                    e.mem.write(a[3], b"A0\0");
                    ret(2)
                }
                _ => ret(0),
            }
        });
        e.register(STRTOL, |e, a| {
            assert_eq!(a[2], 0x10);
            assert_eq!(e.mem.u32(a[1]), 0);
            let text = text_at(e, a[0]);
            ret(u32::from_str_radix(&text, 16).unwrap())
        });
        e.register(BINDING_SET, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x00a2_2ef0, &[0x0066_0000]);
        assert_eq!(calls_to(&e, IMPORT_GET_PRIVATE_PROFILE_STRING).len(), 28);
        assert_eq!(calls_to(&e, STRTOL).len(), 2);
        assert_eq!(
            calls_to(&e, BINDING_SET),
            vec![vec![0x0066_0000, 2, 0x1f], vec![0x0066_0000, 10, 0xa0]]
        );
    }
}

#[cfg(test)]
mod n078_tests {
    use super::*;

    #[test]
    fn string_member_00a22640_is_set_empty_and_its_answer_passed_on() {
        let mut e = Engine::new();
        e.register(STRING_SET_TEXT, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a2_2640, &[0x0066_0000]).u32(), 0x1234);
        let log: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == STRING_SET_TEXT)
            .map(|(_, args)| args.clone())
            .collect();
        assert_eq!(log, vec![vec![0x0066_0038, 0, 0]]);
    }
}
