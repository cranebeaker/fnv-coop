//! `fallout/misc/saveload/bgssaveloaddefines.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The save game's sized values: a size type (0, 1, 2: one, two or four
//! bytes) chosen by the value, and the variable-sized value whose low two
//! bits are its size type, each followed by a `|` separator in a buffer.

#[allow(unused_imports)]
use crate::prelude::*;

/// `BGSSaveLoadFile::Write` and `::Read` (engine map), `thiscall`: data,
/// size; the count moved.
const SAVE_LOAD_FILE_WRITE: u32 = 0x0084_6330;
const SAVE_LOAD_FILE_READ: u32 = 0x0084_6300;
/// The game's `memcpy` wrapper (`00401460`, `cdecl`: destination, source,
/// count).
const MEMCPY: u32 = 0x0040_1460;
/// The bytes of each size type (`011a2040`, three words: 1, 2, 4).
const SIZE_TYPE_SIZES: u32 = 0x011a_2040;
/// `00845cc0`: the save/load error report (`cdecl`: format, ...).
const SAVE_LOAD_ERROR: u32 = 0x0084_5cc0;
/// The error formats for a value too large to save and for a missing
/// separator.
const FORMAT_VALUE_TOO_LARGE: u32 = 0x0107_ee00;
const FORMAT_NO_SEPARATOR: u32 = 0x0107_ee40;
/// The separator after each value.
const SEPARATOR: u8 = b'|';

// Translated from 00845d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SaveGameDataToFileFunction` (engine map), `cdecl`:
/// `BGSSaveLoadFile::Write(data, size)` on `file`; its result is left in
/// `EAX` and returned.
pub fn save_game_data_to_file_function(e: &mut Engine, file: u32, data: u32, size: u32) -> u32 {
    e.call(SAVE_LOAD_FILE_WRITE, &args![file, data, size]).u32()
}

// Translated from 00845d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadGameDataFromFileFunction` (engine map), `cdecl`:
/// `BGSSaveLoadFile::Read(destination, size)` on `file`; its result is
/// left in `EAX` and returned.
pub fn load_game_data_from_file_function(
    e: &mut Engine,
    file: u32,
    destination: u32,
    size: u32,
) -> u32 {
    e.call(SAVE_LOAD_FILE_READ, &args![file, destination, size])
        .u32()
}

// Translated from 00845d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetSizeTypeForSize` (engine map), `cdecl`: 0 for a value up to `0xff`,
/// 1 up to `0xffff`, else 2.
pub fn get_size_type_for_size(_e: &mut Engine, size: u32) -> u32 {
    if size <= 0xff {
        0
    } else if size <= 0xffff {
        1
    } else {
        2
    }
}

// Translated from 00845d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetSizeTypeSize` (engine map), `cdecl`: the bytes of size type
/// `size_type`, from the table at `011a2040` (read past it as the exe does
/// for other types).
pub fn get_size_type_size(e: &mut Engine, size_type: u32) -> u32 {
    e.global::<u32>(SIZE_TYPE_SIZES.wrapping_add(size_type.wrapping_mul(4)))
}

// Translated from 00845d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SaveGameSizeToFile` (engine map), `cdecl`: writes `value` to `file` as
/// one, two or four bytes for size type 0, 1 or 2 (nothing for others).
pub fn save_game_size_to_file(e: &mut Engine, file: u32, value: u32, size_type: u32) {
    e.with_stack(4, |e, local| {
        let local = local.addr();
        let size = match size_type {
            0 => {
                e.mem.set_u8(local, value as u8);
                1u32
            }
            1 => {
                e.mem.set_u16(local, value as u16);
                2
            }
            2 => {
                e.mem.set_u32(local, value);
                4
            }
            _ => return,
        };
        e.call(SAVE_LOAD_FILE_WRITE, &args![file, local, size]);
    });
}

// Translated from 00845de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SaveGameSizeToBuffer` (engine map), `cdecl`: copies `value` into
/// `buffer` as one, two or four bytes for size type 0, 1 or 2 (nothing for
/// others).
pub fn save_game_size_to_buffer(e: &mut Engine, buffer: u32, value: u32, size_type: u32) {
    e.with_stack(4, |e, local| {
        let local = local.addr();
        let size = match size_type {
            0 => {
                e.mem.set_u8(local, value as u8);
                1u32
            }
            1 => {
                e.mem.set_u16(local, value as u16);
                2
            }
            2 => {
                e.mem.set_u32(local, value);
                4
            }
            _ => return,
        };
        e.call(MEMCPY, &args![buffer, local, size]);
    });
}

// Translated from 00845ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadGameSizeFromBuffer` (engine map), `cdecl`: reads one, two or four
/// bytes from `buffer` for size type 0, 1 or 2 into the word at `output`
/// (zero-extended) and returns that count; 0 for other types.
pub fn load_game_size_from_buffer(e: &mut Engine, buffer: u32, output: Ptr, size_type: u32) -> u32 {
    e.with_stack(4, |e, local| {
        let local = local.addr();
        match size_type {
            0 => {
                e.mem.set_u8(local, 0);
                e.call(MEMCPY, &args![local, buffer, 1u32]);
                let value = u32::from(e.mem.u8(local));
                e.mem.set_u32(output.addr(), value);
                1
            }
            1 => {
                e.mem.set_u16(local, 0);
                e.call(MEMCPY, &args![local, buffer, 2u32]);
                let value = u32::from(e.mem.u16(local));
                e.mem.set_u32(output.addr(), value);
                2
            }
            2 => {
                e.call(MEMCPY, &args![output, buffer, 4u32]);
                4
            }
            _ => 0,
        }
    })
}

// Translated from 00845f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, `cdecl`: the bytes a variable-sized `value` takes with its
/// separator: 2 below `0x40`, 3 below `0x4000`, 5 below `0x40000000`,
/// else 1 (such a value cannot be saved).
pub fn fn_00845f70(_e: &mut Engine, value: u32) -> u32 {
    let size = if value < 0x40 {
        1
    } else if value < 0x4000 {
        2
    } else if value < 0x4000_0000 {
        4
    } else {
        0
    };
    size + 1
}

// Translated from 00845fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SaveVariableSizedValueToBuffer` (engine map), `cdecl`: writes `value`
/// shifted left by two with its size type in the low bits (one byte below
/// `0x40`, two below `0x4000`, four below `0x40000000`), then the `|`
/// separator; a larger value is reported (`00845cc0`) and only the
/// separator is written, at `buffer`.
pub fn save_variable_sized_value_to_buffer(e: &mut Engine, buffer: u32, value: u32) {
    let size = e.with_stack(4, |e, local| {
        let local = local.addr();
        if value < 0x40 {
            e.mem.set_u8(local, (value << 2) as u8);
            e.call(MEMCPY, &args![buffer, local, 1u32]);
            1
        } else if value < 0x4000 {
            e.mem.set_u16(local, (value << 2 | 1) as u16);
            e.call(MEMCPY, &args![buffer, local, 2u32]);
            2
        } else if value < 0x4000_0000 {
            e.mem.set_u32(local, value << 2 | 2);
            e.call(MEMCPY, &args![buffer, local, 4u32]);
            4
        } else {
            e.call(SAVE_LOAD_ERROR, &args![FORMAT_VALUE_TOO_LARGE, value]);
            0
        }
    });
    e.mem.set_u8(buffer.wrapping_add(size), SEPARATOR);
}

// Translated from 00846080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadVariableSizedValueFromBuffer` (engine map), `cdecl`: reads a
/// variable-sized value (its low two bits give one, two or four bytes; 3
/// reads nothing) into the word at `output`, shifted right by two, checks
/// the `|` after it (reporting `00845cc0` when it is not there) and
/// returns the bytes taken with the separator.
pub fn load_variable_sized_value_from_buffer(e: &mut Engine, buffer: u32, output: Ptr) -> u32 {
    let tag = e.mem.u8(buffer) & 3;
    let size = e.with_stack(4, |e, local| {
        let local = local.addr();
        match tag {
            0 => {
                e.mem.set_u8(local, 0);
                e.call(MEMCPY, &args![local, buffer, 1u32]);
                let value = e.mem.u8(local) >> 2;
                e.mem.set_u32(output.addr(), u32::from(value));
                1
            }
            1 => {
                e.mem.set_u16(local, 0);
                e.call(MEMCPY, &args![local, buffer, 2u32]);
                let value = e.mem.u16(local) >> 2;
                e.mem.set_u32(output.addr(), u32::from(value));
                2
            }
            2 => {
                e.mem.set_u32(local, 0);
                e.call(MEMCPY, &args![local, buffer, 4u32]);
                let value = e.mem.u32(local) >> 2;
                e.mem.set_u32(output.addr(), value);
                4
            }
            _ => 0,
        }
    });
    if e.mem.u8(buffer.wrapping_add(size)) != SEPARATOR {
        e.call(SAVE_LOAD_ERROR, &args![FORMAT_NO_SEPARATOR]);
    }
    size + 1
}

/// The save/load game singleton pointer (`011ddf38`).
const SAVE_LOAD_GAME: u32 = 0x011d_df38;
/// `BGSSaveLoadGame::SaveGameWarningFunction` (`0084ab00`, `thiscall`,
/// `RET 8`: format, argument list).
const SAVE_GAME_WARNING_FUNCTION: u32 = 0x0084_ab00;
/// `00401170` (`thiscall`): the form's type.
const FORM_TYPE: u32 = 0x0040_1170;

// Translated from 00845cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SaveGameWarning` (engine map; cdecl with variable arguments): with a
/// save/load game (`011ddf38`), hands it `format` and the address of the
/// argument words after it (`BGSSaveLoadGame::SaveGameWarningFunction`).
pub fn save_game_warning(e: &mut Engine, format: u32, arguments: &[u32]) {
    let list = e.mem.alloc(4 * arguments.len().max(1) as u32);
    for (i, word) in arguments.iter().enumerate() {
        e.mem.set_u32(list + 4 * i as u32, *word);
    }
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    if game != 0 {
        e.call(SAVE_GAME_WARNING_FUNCTION, &args![game, format, list]);
    }
    e.mem.free(list);
}

/// The uniform form of `00845cc0`: the format, then any number of argument
/// words.
fn save_game_warning_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(
        !words.is_empty(),
        "SaveGameWarning needs the format, got no argument words"
    );
    save_game_warning(e, words[0], &words[1..]);
    Ret::default()
}

// Translated from 00845e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadGameSizeFromFile` (engine map; cdecl): reads a value of size type
/// 0, 1 or 2 (one, two or four bytes, `BGSSaveLoadFile::Read`) from `file`
/// into the word at `output` (zero-extended) and answers its byte count; 0
/// (nothing read) for any other size type.
pub fn load_game_size_from_file(e: &mut Engine, file: u32, output: Ptr, size_type: u32) -> u32 {
    match size_type {
        0 => {
            let value = e.with_stack(4, |e, local| {
                e.mem.set_u8(local.addr(), 0);
                e.call(SAVE_LOAD_FILE_READ, &args![file, local, 1u32]);
                e.mem.u8(local.addr())
            });
            e.mem.set_u32(output.addr(), u32::from(value));
            1
        }
        1 => {
            let value = e.with_stack(4, |e, local| {
                e.mem.set_u16(local.addr(), 0);
                e.call(SAVE_LOAD_FILE_READ, &args![file, local, 2u32]);
                e.mem.u16(local.addr())
            });
            e.mem.set_u32(output.addr(), u32::from(value));
            2
        }
        2 => {
            e.call(SAVE_LOAD_FILE_READ, &args![file, output, 4u32]);
            4
        }
        _ => 0,
    }
}

// Translated from 00846190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetLocationCellOrWorld` (engine map; cdecl): false for a null form; a
/// form of type 0x41 is stored at `type_41_out`, one of type 0x39 at
/// `type_39_out` (true); false for any other type.
pub fn get_location_cell_or_world(
    e: &mut Engine,
    form: u32,
    type_39_out: Ptr,
    type_41_out: Ptr,
) -> bool {
    if form == 0 {
        return false;
    }
    if e.call(FORM_TYPE, &args![form]).u32() == 0x41 {
        e.mem.set_u32(type_41_out.addr(), form);
        return true;
    }
    if e.call(FORM_TYPE, &args![form]).u32() == 0x39 {
        e.mem.set_u32(type_39_out.addr(), form);
        return true;
    }
    false
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        (0x0084_5cc0, save_game_warning_entry as AbiFn),
        entry!(0x0084_5e50, load_game_size_from_file(u32, Ptr, u32) -> u32),
        entry!(0x0084_6190, get_location_cell_or_world(u32, Ptr, Ptr) -> bool),
        entry!(0x0084_5d00, save_game_data_to_file_function(u32, u32, u32) -> u32),
        entry!(0x0084_5d20, load_game_data_from_file_function(u32, u32, u32) -> u32),
        entry!(0x0084_5d40, get_size_type_for_size(u32) -> u32),
        entry!(0x0084_5d70, get_size_type_size(u32) -> u32),
        entry!(0x0084_5d80, save_game_size_to_file(u32, u32, u32)),
        entry!(0x0084_5de0, save_game_size_to_buffer(u32, u32, u32)),
        entry!(0x0084_5ee0, load_game_size_from_buffer(u32, Ptr, u32) -> u32),
        entry!(0x0084_5f70, fn_00845f70(u32) -> u32),
        entry!(0x0084_5fc0, save_variable_sized_value_to_buffer(u32, u32)),
        entry!(0x0084_6080, load_variable_sized_value_from_buffer(u32, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn memcpy(e: &mut Engine) {
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(SAVE_LOAD_ERROR, |_, _| Ret::default());
    }

    #[test]
    fn file_wrappers_pass_their_words() {
        let mut e = Engine::new();
        e.register(SAVE_LOAD_FILE_WRITE, |_, a| a[2].into_ret());
        e.register(SAVE_LOAD_FILE_READ, |_, a| (a[2] + 1).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0084_5d00, &args![1u32, 2u32, 3u32]).u32(), 3);
        assert_eq!(e.call(0x0084_5d20, &args![1u32, 2u32, 3u32]).u32(), 4);
        assert_eq!(calls_to(&e, SAVE_LOAD_FILE_WRITE), vec![vec![1, 2, 3]]);
        assert_eq!(calls_to(&e, SAVE_LOAD_FILE_READ), vec![vec![1, 2, 3]]);
    }

    #[test]
    fn size_types_and_their_sizes() {
        let mut e = Engine::new();
        for (size, kind) in [
            (0u32, 0u32),
            (0xff, 0),
            (0x100, 1),
            (0xffff, 1),
            (0x10000, 2),
        ] {
            assert_eq!(e.call(0x0084_5d40, &args![size]).u32(), kind);
        }
        e.map(0x011a_2000, 0x1000);
        for (i, size) in [1u32, 2, 4].iter().enumerate() {
            e.mem.set_u32(SIZE_TYPE_SIZES + 4 * i as u32, *size);
        }
        assert_eq!(e.call(0x0084_5d70, &args![2u32]).u32(), 4);
        for (value, size) in [
            (0x3fu32, 2u32),
            (0x40, 3),
            (0x3fff, 3),
            (0x4000, 5),
            (0x4000_0000, 1),
        ] {
            assert_eq!(e.call(0x0084_5f70, &args![value]).u32(), size);
        }
    }

    #[test]
    fn sizes_to_file_and_buffer() {
        let mut e = Engine::new();
        memcpy(&mut e);
        e.register(SAVE_LOAD_FILE_WRITE, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            let value = bytes.iter().rev().fold(0u32, |v, b| v << 8 | u32::from(*b));
            value.into_ret()
        });
        e.call_log = Some(vec![]);
        for (kind, size) in [(0u32, 1u32), (1, 2), (2, 4)] {
            e.call(0x0084_5d80, &args![0x7700u32, 0x1234_5678u32, kind]);
            let write = calls_to(&e, SAVE_LOAD_FILE_WRITE);
            assert_eq!(write.last().unwrap()[2], size);
        }
        e.call(0x0084_5d80, &args![0x7700u32, 1u32, 3u32]);
        assert_eq!(calls_to(&e, SAVE_LOAD_FILE_WRITE).len(), 3);

        let buffer = e.mem.alloc(8);
        e.call(0x0084_5de0, &args![buffer, 0x1234_5678u32, 1u32]);
        assert_eq!(e.mem.u32(buffer), 0x5678);
        e.call(0x0084_5de0, &args![buffer, 0x1234_5678u32, 2u32]);
        assert_eq!(e.mem.u32(buffer), 0x1234_5678);
        e.call(0x0084_5de0, &args![buffer, 0xabu32, 0u32]);
        assert_eq!(e.mem.u32(buffer), 0x1234_56ab);
        e.call(0x0084_5de0, &args![buffer, 0u32, 5u32]);
        assert_eq!(e.mem.u32(buffer), 0x1234_56ab);

        let output = e.mem.alloc(4);
        assert_eq!(e.call(0x0084_5ee0, &args![buffer, output, 0u32]).u32(), 1);
        assert_eq!(e.mem.u32(output), 0xab);
        assert_eq!(e.call(0x0084_5ee0, &args![buffer, output, 1u32]).u32(), 2);
        assert_eq!(e.mem.u32(output), 0x56ab);
        assert_eq!(e.call(0x0084_5ee0, &args![buffer, output, 2u32]).u32(), 4);
        assert_eq!(e.mem.u32(output), 0x1234_56ab);
        assert_eq!(e.call(0x0084_5ee0, &args![buffer, output, 3u32]).u32(), 0);
    }

    #[test]
    fn variable_sized_values_round_trip() {
        let mut e = Engine::new();
        memcpy(&mut e);
        let buffer = e.mem.alloc(8);
        let output = e.mem.alloc(4);
        for (value, size) in [(0x3fu32, 1u32), (0x3fff, 2), (0x3fff_ffff, 4)] {
            e.call(0x0084_5fc0, &args![buffer, value]);
            assert_eq!(e.mem.u8(buffer + size), b'|');
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0084_6080, &args![buffer, output]).u32(), size + 1);
            assert_eq!(e.mem.u32(output), value);
            assert!(calls_to(&e, SAVE_LOAD_ERROR).is_empty());
        }
        // Too large: reported, only the separator written.
        e.mem.set_u8(buffer, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x0084_5fc0, &args![buffer, 0x4000_0000u32]);
        assert_eq!(
            calls_to(&e, SAVE_LOAD_ERROR),
            vec![vec![FORMAT_VALUE_TOO_LARGE, 0x4000_0000]]
        );
        assert_eq!(e.mem.u8(buffer), b'|');
        // Tag 3 reads nothing; the byte is not a separator: reported.
        e.mem.set_u8(buffer, 3);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0084_6080, &args![buffer, output]).u32(), 1);
        assert_eq!(
            calls_to(&e, SAVE_LOAD_ERROR),
            vec![vec![FORMAT_NO_SEPARATOR]]
        );
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn save_game_warning_forwards_the_arguments() {
        let mut e = Engine::new();
        e.map(0x011d_d000, 0x1000);
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let log = seen.clone();
        e.register_double(SAVE_GAME_WARNING_FUNCTION, move |e, a| {
            log.borrow_mut()
                .push(vec![a[0], a[1], e.mem.u32(a[2]), e.mem.u32(a[2] + 4)]);
            Ret::default()
        });
        e.call(0x0084_5cc0, &args![0x0107_0000u32, 7u32, 9u32]);
        assert!(seen.borrow().is_empty());
        e.set_global::<u32>(SAVE_LOAD_GAME, 0x5000);
        e.call(0x0084_5cc0, &args![0x0107_0000u32, 7u32, 9u32]);
        e.call(0x0084_5cc0, &args![0x0107_0004u32]);
        let seen = seen.borrow();
        assert_eq!(seen[0], vec![0x5000, 0x0107_0000, 7, 9]);
        assert_eq!(seen[1][..2], [0x5000, 0x0107_0004]);
    }

    fn reads(e: &mut Engine) {
        e.register(SAVE_LOAD_FILE_READ, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[1] + i, 0xf1 + i as u8);
            }
            Ret::default()
        });
    }

    #[test]
    fn load_game_size_from_file_reads_by_size_type() {
        let mut e = Engine::new();
        reads(&mut e);
        let output = e.mem.alloc(4);
        e.mem.set_u32(output, 0xdead_beef);
        let read =
            |e: &mut Engine, kind: u32| e.call(0x0084_5e50, &args![0x700u32, output, kind]).u32();
        assert_eq!(read(&mut e, 0), 1);
        assert_eq!(e.mem.u32(output), 0xf1);
        assert_eq!(read(&mut e, 1), 2);
        assert_eq!(e.mem.u32(output), 0xf2f1);
        assert_eq!(read(&mut e, 2), 4);
        assert_eq!(e.mem.u32(output), 0xf4f3_f2f1);
        e.mem.set_u32(output, 5);
        assert_eq!(read(&mut e, 3), 0);
        assert_eq!(e.mem.u32(output), 5);
    }

    #[test]
    fn get_location_cell_or_world_sorts_by_type() {
        let mut e = Engine::new();
        e.register(FORM_TYPE, |e, a| e.mem.u32(a[0]).into_ret());
        let first = e.mem.alloc(4);
        let second = e.mem.alloc(4);
        let world = e.mem.alloc(4);
        e.mem.set_u32(world, 0x41);
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, 0x39);
        let other = e.mem.alloc(4);
        e.mem.set_u32(other, 0x3b);
        let sort =
            |e: &mut Engine, form: u32| e.call(0x0084_6190, &args![form, first, second]).bool();
        assert!(!sort(&mut e, 0));
        assert!(sort(&mut e, world));
        assert_eq!((e.mem.u32(first), e.mem.u32(second)), (0, world));
        assert!(sort(&mut e, cell));
        assert_eq!((e.mem.u32(first), e.mem.u32(second)), (cell, world));
        assert!(!sort(&mut e, other));
    }
}
