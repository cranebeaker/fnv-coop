//! `NiAnimation/nibooldata.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiBoolData` (PC offsets as the code uses them): `NiObject`, then the key
//! count +0x08, the key array +0x0c, the key type +0x10 and the key size
//! byte +0x14 (the layout of `NiFloatData`). The key arrays are handled
//! through the bool-key tables in `.data`, indexed by the key type and
//! filled by the static initializers: `011f3918` (a `cdecl` function
//! deleting a key array), `011f3776` (the key size, one byte per type),
//! `011f3e28` (a `cdecl` function `(stream, count)` that reads a key array
//! and returns it) and `011f37f8` (a `cdecl` function `(keys, count, key
//! size)` run on the array read). Their Xbox PDB names are not confirmed.
//!
//! None of these functions has an exception frame or a `/GS` check.

#[allow(unused_imports)]
use crate::prelude::*;

/// Key count, key array, key type and key size byte (see the module comment).
const NUM_KEYS: u32 = 0x08;
const KEYS: u32 = 0x0c;
const KEY_TYPE: u32 = 0x10;
const KEY_SIZE: u32 = 0x14;

/// The bool-key tables, indexed by key type (see the module comment).
const KEY_DELETE_TABLE: u32 = 0x011f_3918;
const KEY_SIZE_TABLE: u32 = 0x011f_3776;
const KEY_LOAD_TABLE: u32 = 0x011f_3e28;
const KEY_AFTER_LOAD_TABLE: u32 = 0x011f_37f8;

/// `NiObject::LoadBinary` (`00a5d610`, `NiMain/niobject.cpp`).
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;

/// `NiStream::m_uiNifFileVersion` +0xd8 and `m_pkIstr` +0x24c (Xbox PDB
/// names; offsets as the PC code reads them), and the input stream's
/// position (+4) and read function (+8, `cdecl` `(stream, buffer, size,
/// &component size, component count)`, returns the bytes read).
const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;

/// The first file version whose bool data stores its key type
/// (`CMP [stream+0xd8], 0x0a010068`; older files use type 5).
const VERSION_WITH_KEY_TYPE: u32 = 0x0a01_0068;
/// The key type older files imply.
const OLD_FILE_KEY_TYPE: u32 = 5;

/// One read through the stream's read function into `buffer`: the word at
/// `slot` is set to `component` and passed as the component size; the
/// position advances by the bytes read.
fn stream_read(e: &mut Engine, stream: u32, buffer: u32, size: u32, component: u32, slot: u32) {
    let input = e.mem.u32(stream.wrapping_add(STREAM_INPUT));
    let read = e.mem.u32(input.wrapping_add(BINARY_STREAM_READ));
    e.mem.set_u32(slot, component);
    let bytes = e.call(read, &args![input, buffer, size, slot, 1u32]).u32();
    let position = e.mem.u32(input.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        input.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

// Translated from 00a538b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolData::ReplaceAnim` (Xbox PDB): deletes the current key array (when
/// there is one) through its type's delete function, then takes `keys`,
/// `num_keys` and `key_type` with the type's key size when `keys` and
/// `num_keys` are non-zero, and clears all four fields otherwise.
pub fn ni_bool_data_replace_anim(
    e: &mut Engine,
    this: Ptr,
    keys: u32,
    num_keys: u32,
    key_type: u32,
) {
    let this = this.addr();
    let old = e.mem.u32(this.wrapping_add(KEYS));
    if old != 0 {
        let old_type = e.mem.u32(this.wrapping_add(KEY_TYPE));
        let delete = e
            .mem
            .u32(KEY_DELETE_TABLE.wrapping_add(old_type.wrapping_mul(4)));
        e.call(delete, &args![old]);
    }
    if keys != 0 && num_keys != 0 {
        let key_size = e.mem.u8(KEY_SIZE_TABLE.wrapping_add(key_type));
        e.mem.set_u32(this.wrapping_add(KEYS), keys);
        e.mem.set_u8(this.wrapping_add(KEY_SIZE), key_size);
        e.mem.set_u32(this.wrapping_add(NUM_KEYS), num_keys);
        e.mem.set_u32(this.wrapping_add(KEY_TYPE), key_type);
    } else {
        e.mem.set_u32(this.wrapping_add(NUM_KEYS), 0);
        e.mem.set_u32(this.wrapping_add(KEYS), 0);
        e.mem.set_u32(this.wrapping_add(KEY_TYPE), 0);
        e.mem.set_u8(this.wrapping_add(KEY_SIZE), 0);
    }
}

// Translated from 00a53a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolData::LoadBinary` (Xbox PDB): `NiObject::LoadBinary`, then the key
/// count (4 bytes, read into the argument's own stack word). With keys, the
/// key type follows (4 bytes) from file version `0x0a010068` on (type 5
/// before); the type's key size is stored, the type's load function reads
/// the array (`(stream, count)`), the type's after-load function runs on it
/// (`(keys, count, key size)`) and `ReplaceAnim` takes it. A count of 0
/// reads nothing more.
pub fn ni_bool_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    let stream = stream.addr();
    // The frame: the component size word +0, the key type +4, and the
    // argument's own word (the key count) +8.
    e.with_stack(12, |e, frame| {
        let component_size = frame.addr();
        let key_type_word = component_size + 4;
        let num_keys_word = component_size + 8;
        e.mem.set_u32(num_keys_word, stream);
        stream_read(e, stream, num_keys_word, 4, 4, component_size);
        let num_keys = e.mem.u32(num_keys_word);
        if num_keys == 0 {
            return;
        }
        let mut key_type = OLD_FILE_KEY_TYPE;
        if e.mem.u32(stream.wrapping_add(STREAM_VERSION)) >= VERSION_WITH_KEY_TYPE {
            stream_read(e, stream, key_type_word, 4, 4, component_size);
            key_type = e.mem.u32(key_type_word);
        }
        let key_size = e.mem.u8(KEY_SIZE_TABLE.wrapping_add(key_type));
        e.mem.set_u8(this.addr().wrapping_add(KEY_SIZE), key_size);
        let load = e
            .mem
            .u32(KEY_LOAD_TABLE.wrapping_add(key_type.wrapping_mul(4)));
        let keys = e.call(load, &args![stream, num_keys]).u32();
        let key_size = e.mem.u8(this.addr().wrapping_add(KEY_SIZE));
        let after_load = e
            .mem
            .u32(KEY_AFTER_LOAD_TABLE.wrapping_add(key_type.wrapping_mul(4)));
        e.call(after_load, &args![keys, num_keys, u32::from(key_size)]);
        ni_bool_data_replace_anim(e, this, keys, num_keys, key_type);
    });
}

/// `NiBoolData::ms_RTTI` (`NiBoolData::GetRTTI`, `00a537e0`, answers it).
const NI_BOOL_DATA_RTTI: u32 = 0x011f_4054;
/// The bool-key equality table (`011f3b58`, `cdecl (key, other key)`,
/// answers a `bool`) and save table (`011f3ac8`, `cdecl (stream, keys,
/// count)`), indexed by key type like the tables above.
const KEY_EQUAL_TABLE: u32 = 0x011f_3b58;
const KEY_SAVE_TABLE: u32 = 0x011f_3ac8;
/// The key content the exe passes for bool keys (`PUSH 5`).
const BOOL_KEY_CONTENT: u32 = 5;
/// `NiAnimationKey::GuaranteeKeyAtStartAndEnd` (`00a3d930`, engine map;
/// `cdecl (content, type, &keys, &count, start, end)`) and
/// `NiAnimationKey::CopySequence` (`00a3d630`, engine map; `cdecl
/// (content, type, keys, count, start, end, &new keys, &new count)`).
const GUARANTEE_KEY_AT_START_AND_END: u32 = 0x00a3_d930;
const COPY_SEQUENCE: u32 = 0x00a3_d630;
/// `NiObject::IsEqual` (`00a5d470`, engine map; `thiscall`, one word),
/// `NiObject::CreateDeepCopy` (`00a5d510`, engine map; `thiscall`, one
/// word: the `NiPointer` it fills), the base `SaveBinary` (`00a89af0`,
/// `thiscall`, one word) and `NiObject::GetViewerStrings` (`00a5d6e0`,
/// engine map; `thiscall`, one word).
const NI_OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const NI_OBJECT_CREATE_DEEP_COPY: u32 = 0x00a5_d510;
const NI_OBJECT_SAVE_BINARY: u32 = 0x00a8_9af0;
const NI_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiStream::m_pkOstr` (+0x250, offset as the PC code reads it) and the
/// output stream's write function (+0xC, `cdecl (stream, source, size,
/// &component size, component count)`, returns the bytes written).
const STREAM_OUTPUT: u32 = 0x250;
const BINARY_STREAM_WRITE: u32 = 0xc;
/// `NiGetViewerString` (`00a7d330`, `cdecl (text)`) and its unsigned
/// overload (`00a7d510`, `cdecl (name, value)`); the viewer-string array's
/// grow (`0096ad30`) and store (`0096ae90`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `"m_uiNumKeys"`.
const NAME_NUM_KEYS: u32 = 0x0109_7164;

/// Releases a reference-counted object (`InterlockedDecrement` of +4, its
/// vtable +4 deleting it at zero).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32() == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// One write through the stream's output: `size` bytes from `source`, the
/// component size `component` in a stack word; the output's position (+4)
/// advances by the bytes written.
fn stream_write(e: &mut Engine, stream: u32, source: u32, size: u32, component: u32) {
    let output = e.mem.u32(stream.wrapping_add(STREAM_OUTPUT));
    let write = e.mem.u32(output.wrapping_add(BINARY_STREAM_WRITE));
    let bytes = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), component);
        e.call(write, &args![output, source, size, slot, 1u32])
            .u32()
    });
    let position = e.mem.u32(output.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        output.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

/// Appends `text` to a viewer-string array as the exe's inlined `Add`
/// does: when the 16-bit size (+0xA) has reached the capacity (+0x8) the
/// array grows to the step (+0xE) plus the size, then the string is stored
/// at the old size (through a stack slot holding it).
fn push_viewer_string(e: &mut Engine, strings: Ptr, text: u32) {
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

// Translated from 00a537a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolData::GuaranteeKeysAtStartAndEnd` (Xbox PDB, matched by the
/// engine map; `thiscall (this; start, end)`):
/// `NiAnimationKey::GuaranteeKeyAtStartAndEnd(5, type, &keys, &count,
/// start, end)` (`00a3d930`) on a stack copy of the key array pointer and
/// the count field itself; the copy is stored back as the key array.
pub fn ni_bool_data_guarantee_keys_at_start_and_end(
    e: &mut Engine,
    this: Ptr,
    start: f32,
    end: f32,
) {
    let this = this.addr();
    let end = x87_quiet_bits(end.to_bits());
    let start = x87_quiet_bits(start.to_bits());
    let keys = e.with_stack(4, |e, slot| {
        let keys = e.mem.u32(this.wrapping_add(KEYS));
        e.mem.set_u32(slot.addr(), keys);
        let key_type = e.mem.u32(this.wrapping_add(KEY_TYPE));
        e.call(
            GUARANTEE_KEY_AT_START_AND_END,
            &args![
                BOOL_KEY_CONTENT,
                key_type,
                slot,
                this.wrapping_add(NUM_KEYS),
                start,
                end
            ],
        );
        e.mem.u32(slot.addr())
    });
    e.mem.set_u32(this.wrapping_add(KEYS), keys);
}

// Translated from 00a53830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolData::IsEqual` (Xbox PDB, matched by the engine map; `thiscall`,
/// one word: the other object): `NiObject::IsEqual` (`00a5d470`), the same
/// count, key type and key size, then every key pair (keys a key size
/// apart) equal by the type's equality function (`011f3b58`); true for no
/// keys.
pub fn ni_bool_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(NI_OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let this = this.addr();
    let other = other.addr();
    let count = e.mem.u32(this.wrapping_add(NUM_KEYS));
    if count != e.mem.u32(other.wrapping_add(NUM_KEYS)) {
        return false;
    }
    let key_type = e.mem.u32(this.wrapping_add(KEY_TYPE));
    if key_type != e.mem.u32(other.wrapping_add(KEY_TYPE)) {
        return false;
    }
    if e.mem.u8(this.wrapping_add(KEY_SIZE)) != e.mem.u8(other.wrapping_add(KEY_SIZE)) {
        return false;
    }
    let equal = e
        .mem
        .u32(KEY_EQUAL_TABLE.wrapping_add(key_type.wrapping_mul(4)));
    let mut index = 0u32;
    if count != 0 {
        loop {
            let offset = u32::from(e.mem.u8(this.wrapping_add(KEY_SIZE))).wrapping_mul(index);
            let theirs = e.mem.u32(other.wrapping_add(KEYS)).wrapping_add(offset);
            let ours = e.mem.u32(this.wrapping_add(KEYS)).wrapping_add(offset);
            if !e.call(equal, &args![ours, theirs]).bool() {
                return false;
            }
            index += 1;
            if index >= e.mem.u32(this.wrapping_add(NUM_KEYS)) {
                break;
            }
        }
    }
    true
}

// Translated from 00a53910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolData::GetSequenceData` (Xbox PDB, matched by the engine map;
/// `thiscall (this; start, end, &result)`, `result` a `NiPointer`): a deep
/// copy (`NiObject::CreateDeepCopy` into a stack `NiPointer`, then held by
/// a second reference); with keys, `NiAnimationKey::CopySequence(5, type,
/// keys, count, start, end, &new keys, &new count)` (`00a3d630`) and the
/// copy's [`ni_bool_data_replace_anim`] takes them (on the copy as the exe
/// has it, null included). The copy is assigned to `*result` (a
/// `NiPointer` assignment) and both stack references are released.
pub fn ni_bool_data_get_sequence_data(
    e: &mut Engine,
    this: Ptr,
    start: f32,
    end: f32,
    result: Ptr,
) {
    let this = this.addr();
    // The frame: the deep copy's `NiPointer` +0, the new count +4 and the
    // new keys +8.
    e.with_stack(12, |e, frame| {
        let copy_slot = frame.addr();
        let count_slot = copy_slot + 4;
        let keys_slot = copy_slot + 8;
        e.mem.set_u32(copy_slot, 0);
        e.call(NI_OBJECT_CREATE_DEEP_COPY, &args![this, copy_slot]);
        let copy = e.mem.u32(copy_slot);
        if copy != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![copy + 4]);
        }
        let count = e.mem.u32(this.wrapping_add(NUM_KEYS));
        if count != 0 {
            let keys = e.mem.u32(this.wrapping_add(KEYS));
            let end = x87_quiet_bits(end.to_bits());
            e.mem.set_u32(keys_slot, 0);
            let start = x87_quiet_bits(start.to_bits());
            e.mem.set_u32(count_slot, 0);
            let key_type = e.mem.u32(this.wrapping_add(KEY_TYPE));
            e.call(
                COPY_SEQUENCE,
                &args![
                    BOOL_KEY_CONTENT,
                    key_type,
                    keys,
                    count,
                    start,
                    end,
                    keys_slot,
                    count_slot
                ],
            );
            let key_type = e.mem.u32(this.wrapping_add(KEY_TYPE));
            let new_count = e.mem.u32(count_slot);
            let new_keys = e.mem.u32(keys_slot);
            ni_bool_data_replace_anim(e, Ptr::new(copy), new_keys, new_count, key_type);
        }
        let old = e.mem.u32(result.addr());
        if old != copy {
            if old != 0 {
                release(e, old);
            }
            e.mem.set_u32(result.addr(), copy);
            if copy != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![copy + 4]);
            }
        }
        if copy != 0 {
            release(e, copy);
        }
        let held = e.mem.u32(copy_slot);
        if held != 0 {
            release(e, held);
        }
    });
}

// Translated from 00a53b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolData::SaveBinary` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the stream): the base `SaveBinary` (`00a89af0`),
/// the key count (4 bytes); with keys, the key type (4 bytes) and the
/// type's save function (`011f3ac8`, `(stream, keys, count)`).
pub fn ni_bool_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_OBJECT_SAVE_BINARY, &args![this, stream]);
    let this = this.addr();
    let stream = stream.addr();
    stream_write(e, stream, this.wrapping_add(NUM_KEYS), 4, 4);
    if e.mem.u32(this.wrapping_add(NUM_KEYS)) == 0 {
        return;
    }
    stream_write(e, stream, this.wrapping_add(KEY_TYPE), 4, 4);
    let count = e.mem.u32(this.wrapping_add(NUM_KEYS));
    let keys = e.mem.u32(this.wrapping_add(KEYS));
    let key_type = e.mem.u32(this.wrapping_add(KEY_TYPE));
    let save = e
        .mem
        .u32(KEY_SAVE_TABLE.wrapping_add(key_type.wrapping_mul(4)));
    e.call(save, &args![stream, keys, count]);
}

// Translated from 00a53ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolData::GetViewerStrings` (Xbox PDB, matched by the engine map;
/// `thiscall`, one word: the strings array): `NiObject::GetViewerStrings`
/// (`00a5d6e0`), then the class name (the first word of the `NiRTTI`
/// record) and `m_uiNumKeys`, each appended.
pub fn ni_bool_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(NI_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name: u32 = e.global(NI_BOOL_DATA_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings, text);
    let count = e.mem.u32(this.addr().wrapping_add(NUM_KEYS));
    let text = e
        .call(NI_GET_VIEWER_STRING_UNSIGNED, &args![NAME_NUM_KEYS, count])
        .u32();
    push_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a5_37a0,
            ni_bool_data_guarantee_keys_at_start_and_end(Ptr, f32, f32)
        ),
        entry!(0x00a5_3830, ni_bool_data_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a5_3910,
            ni_bool_data_get_sequence_data(Ptr, f32, f32, Ptr)
        ),
        entry!(0x00a5_3b20, ni_bool_data_save_binary(Ptr, Ptr)),
        entry!(0x00a5_3ba0, ni_bool_data_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00a5_38b0, ni_bool_data_replace_anim(Ptr, u32, u32, u32)),
        entry!(0x00a5_3a50, ni_bool_data_load_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const DELETE: u32 = 0x00ee_0100;
    const LOAD: u32 = 0x00ee_0200;
    const AFTER_LOAD: u32 = 0x00ee_0300;
    const READ: u32 = 0x00ee_0400;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            ret(eax)
        });
    }

    /// Key types 3 and 5: delete `DELETE`, sizes 0x08 and 0x0c, load
    /// `LOAD`, after-load `AFTER_LOAD`.
    fn tables(e: &mut Engine, log: &Log) {
        e.map(KEY_SIZE_TABLE, 0x10);
        e.map(KEY_DELETE_TABLE, 0x40);
        e.map(KEY_LOAD_TABLE, 0x40);
        e.map(KEY_AFTER_LOAD_TABLE, 0x40);
        for (key_type, size) in [(3u32, 0x08u8), (5, 0x0c)] {
            e.mem.set_u8(KEY_SIZE_TABLE + key_type, size);
            e.mem.set_u32(KEY_DELETE_TABLE + 4 * key_type, DELETE);
            e.mem.set_u32(KEY_LOAD_TABLE + 4 * key_type, LOAD);
            e.mem
                .set_u32(KEY_AFTER_LOAD_TABLE + 4 * key_type, AFTER_LOAD);
        }
        logged(e, log, DELETE, 0);
        logged(e, log, LOAD, 0x7000);
        logged(e, log, AFTER_LOAD, 0);
    }

    #[test]
    fn replace_anim_takes_keys_and_count_or_clears_everything() {
        let mut e = Engine::new();
        let log = Log::default();
        tables(&mut e, &log);
        let this = e.mem.alloc(0x18);
        // No old keys: nothing deleted.
        e.call(0x00a5_38b0, &args![this, 0x6000u32, 5u32, 3u32]);
        assert_eq!(e.mem.u32(this + KEYS), 0x6000);
        assert_eq!(e.mem.u32(this + NUM_KEYS), 5);
        assert_eq!(e.mem.u32(this + KEY_TYPE), 3);
        assert_eq!(e.mem.u8(this + KEY_SIZE), 0x08);
        assert!(log.borrow().is_empty());
        for (keys, count) in [(0u32, 5u32), (0x6100, 0)] {
            e.mem.set_u32(this + KEYS, 0x6000);
            e.mem.set_u32(this + KEY_TYPE, 3);
            log.borrow_mut().clear();
            e.call(0x00a5_38b0, &args![this, keys, count, 3u32]);
            assert_eq!(*log.borrow(), [(DELETE, vec![0x6000])]);
            assert_eq!(e.mem.bytes(this + 8, 13), vec![0; 13]);
        }
    }

    /// `LoadBinary` on a stream of file `version` whose reads hand out
    /// `words` in turn.
    fn load(version: u32, words: &'static [u32]) -> (Engine, Log, u32, u32) {
        let mut e = Engine::new();
        let log = Log::default();
        tables(&mut e, &log);
        logged(&mut e, &log, NI_OBJECT_LOAD_BINARY, 0);
        let stream = e.mem.alloc(0x5c4);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        e.mem.set_u32(input + BINARY_STREAM_READ, READ);
        let l = log.clone();
        let mut next = words.iter();
        e.register_double(READ, move |e, a| {
            l.borrow_mut()
                .push((READ, vec![a[0], a[2], e.mem.u32(a[3]), a[4]]));
            e.mem.set_u32(a[1], *next.next().unwrap());
            ret(4)
        });
        let this = e.mem.alloc(0x18);
        e.call(0x00a5_3a50, &args![this, stream]);
        (e, log, this, stream)
    }

    #[test]
    fn load_binary_reads_the_count_and_type_in_new_files() {
        let (e, log, this, stream) = load(0x0a01_0068, &[4, 3]);
        let input = e.mem.u32(stream + STREAM_INPUT);
        assert_eq!(e.mem.u32(input + BINARY_STREAM_POSITION), 8);
        assert_eq!(
            *log.borrow(),
            [
                (NI_OBJECT_LOAD_BINARY, vec![this, stream]),
                (READ, vec![input, 4, 4, 1]),
                (READ, vec![input, 4, 4, 1]),
                (LOAD, vec![stream, 4]),
                (AFTER_LOAD, vec![0x7000, 4, 0x08]),
            ]
        );
        assert_eq!(e.mem.u32(this + KEYS), 0x7000);
        assert_eq!(e.mem.u32(this + KEY_TYPE), 3);
    }

    #[test]
    fn load_binary_uses_type_5_in_old_files_and_stops_without_keys() {
        let (e, log, this, stream) = load(0x0a01_0067, &[2]);
        let input = e.mem.u32(stream + STREAM_INPUT);
        assert_eq!(e.mem.u32(input + BINARY_STREAM_POSITION), 4);
        assert_eq!(log.borrow()[2], (LOAD, vec![stream, 2]));
        assert_eq!(log.borrow()[3], (AFTER_LOAD, vec![0x7000, 2, 0x0c]));
        assert_eq!(e.mem.u32(this + KEY_TYPE), 5);
        assert_eq!(e.mem.u8(this + KEY_SIZE), 0x0c);

        let (e, log, this, _) = load(0x0a01_0068, &[0]);
        assert_eq!(log.borrow().len(), 2);
        assert_eq!(e.mem.u32(this + KEYS), 0);
    }
}

#[cfg(test)]
mod n002_tests {
    use super::*;

    const DELETE_SLOT: u32 = 0x00ee_1010;
    const EQUAL: u32 = 0x00ee_1020;
    const SAVE: u32 = 0x00ee_1030;
    const KEY_DELETE: u32 = 0x00ee_1040;
    const VTABLE: u32 = 0x00ee_2000;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], value);
            word(value)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], value);
            word(value)
        });
        e.register(DELETE_SLOT, |_, _| Ret::default());
        e.put_vtable(VTABLE, &[0, DELETE_SLOT]);
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn bool_data(e: &mut Engine, references: u32, count: u32, keys: u32) -> u32 {
        let object = e.mem.alloc(0x18);
        e.mem.set_u32(object, VTABLE);
        e.mem.set_u32(object + 4, references);
        e.mem.set_u32(object + NUM_KEYS, count);
        e.mem.set_u32(object + KEYS, keys);
        e.mem.set_u32(object + KEY_TYPE, 3);
        e.mem.set_u8(object + KEY_SIZE, 2);
        object
    }

    #[test]
    fn guarantee_keys_passes_the_array_through_a_stack_copy() {
        let mut e = engine();
        e.register(GUARANTEE_KEY_AT_START_AND_END, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x6000);
            e.mem.set_u32(a[2], 0x6100);
            e.mem.set_u32(a[3], 7);
            Ret::default()
        });
        let this = bool_data(&mut e, 1, 2, 0x6000);
        e.call(0x00a5_37a0, &args![this, 1.0f32, 2.0f32]);
        let call = &calls(&e, GUARANTEE_KEY_AT_START_AND_END)[0];
        assert_eq!(call[..2], [5, 3]);
        assert_eq!(
            call[3..],
            [this + NUM_KEYS, 1.0f32.to_bits(), 2.0f32.to_bits()]
        );
        assert_eq!(e.mem.u32(this + KEYS), 0x6100);
        assert_eq!(e.mem.u32(this + NUM_KEYS), 7);
    }

    #[test]
    fn is_equal_compares_header_then_every_key() {
        let mut e = engine();
        e.map(KEY_EQUAL_TABLE, 0x10);
        e.mem.set_u32(KEY_EQUAL_TABLE + 12, EQUAL);
        e.register(EQUAL, |e, a| {
            word(u32::from(e.mem.u8(a[0]) == e.mem.u8(a[1])))
        });
        e.register(NI_OBJECT_IS_EQUAL, |_, _| word(1));
        let ours = e.mem.alloc(6);
        let theirs = e.mem.alloc(6);
        e.mem.write(ours, &[1, 0, 2, 0, 3, 0]);
        e.mem.write(theirs, &[1, 0, 2, 0, 3, 0]);
        let this = bool_data(&mut e, 1, 3, ours);
        let other = bool_data(&mut e, 1, 3, theirs);
        assert!(e.call(0x00a5_3830, &args![this, other]).bool());
        assert_eq!(
            calls(&e, EQUAL),
            vec![
                vec![ours, theirs],
                vec![ours + 2, theirs + 2],
                vec![ours + 4, theirs + 4]
            ]
        );
        e.mem.set_u8(theirs + 2, 9);
        assert!(!e.call(0x00a5_3830, &args![this, other]).bool());
        // Header differences.
        e.mem.set_u8(theirs + 2, 2);
        e.mem.set_u8(other + KEY_SIZE, 4);
        assert!(!e.call(0x00a5_3830, &args![this, other]).bool());
        e.mem.set_u8(other + KEY_SIZE, 2);
        e.mem.set_u32(other + KEY_TYPE, 1);
        assert!(!e.call(0x00a5_3830, &args![this, other]).bool());
        e.mem.set_u32(other + KEY_TYPE, 3);
        e.mem.set_u32(other + NUM_KEYS, 2);
        assert!(!e.call(0x00a5_3830, &args![this, other]).bool());
        // No keys: equal without a key compare.
        e.mem.set_u32(other + NUM_KEYS, 0);
        e.mem.set_u32(this + NUM_KEYS, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a5_3830, &args![this, other]).bool());
        assert!(calls(&e, EQUAL).is_empty());
        e.register(NI_OBJECT_IS_EQUAL, |_, _| word(0));
        assert!(!e.call(0x00a5_3830, &args![this, other]).bool());
    }

    #[test]
    fn get_sequence_data_hands_out_a_copy_of_the_range() {
        let mut e = engine();
        e.map(KEY_DELETE_TABLE, 0x10);
        e.map(KEY_SIZE_TABLE, 0x10);
        e.mem.set_u32(KEY_DELETE_TABLE + 12, KEY_DELETE);
        e.mem.set_u8(KEY_SIZE_TABLE + 3, 2);
        e.register(KEY_DELETE, |_, _| Ret::default());
        let this = bool_data(&mut e, 1, 4, 0x6000);
        let copy = bool_data(&mut e, 0, 4, 0x6800);
        e.register_double(NI_OBJECT_CREATE_DEEP_COPY, move |e, a| {
            e.mem.set_u32(a[1], copy);
            let references = e.mem.u32(copy + 4);
            e.mem.set_u32(copy + 4, references + 1);
            Ret::default()
        });
        e.register(COPY_SEQUENCE, |e, a| {
            assert_eq!((e.mem.u32(a[6]), e.mem.u32(a[7])), (0, 0));
            e.mem.set_u32(a[6], 0x6900);
            e.mem.set_u32(a[7], 2);
            Ret::default()
        });
        let old = bool_data(&mut e, 1, 0, 0);
        let result = e.mem.alloc(4);
        e.mem.set_u32(result, old);
        e.call(0x00a5_3910, &args![this, 1.0f32, 2.0f32, result]);
        assert_eq!(
            calls(&e, COPY_SEQUENCE)[0][..6],
            [5, 3, 0x6000, 4, 1.0f32.to_bits(), 2.0f32.to_bits()]
        );
        assert_eq!(calls(&e, KEY_DELETE), vec![vec![0x6800]]);
        assert_eq!(e.mem.u32(copy + KEYS), 0x6900);
        assert_eq!(e.mem.u32(copy + NUM_KEYS), 2);
        assert_eq!(e.mem.u32(result), copy);
        // The result's reference is the one left.
        assert_eq!(e.mem.u32(copy + 4), 1);
        assert_eq!(calls(&e, DELETE_SLOT), vec![vec![old]]);

        // No keys: no sequence copy; the same result is kept (the deep
        // copy double hands out the same object again).
        e.mem.set_u32(this + NUM_KEYS, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a5_3910, &args![this, 1.0f32, 2.0f32, result]);
        assert!(calls(&e, COPY_SEQUENCE).is_empty());
        assert_eq!(e.mem.u32(result), copy);
        assert_eq!(e.mem.u32(copy + 4), 1);
        assert!(calls(&e, DELETE_SLOT).is_empty());
    }

    #[test]
    fn save_binary_writes_count_type_and_keys() {
        const WRITE: u32 = 0x00ee_1050;
        let mut e = engine();
        e.register(NI_OBJECT_SAVE_BINARY, |_, _| Ret::default());
        e.register(WRITE, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            word(4)
        });
        e.register(SAVE, |_, _| Ret::default());
        e.map(KEY_SAVE_TABLE, 0x10);
        e.mem.set_u32(KEY_SAVE_TABLE + 12, SAVE);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + BINARY_STREAM_WRITE, WRITE);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        let this = bool_data(&mut e, 1, 2, 0x6000);
        e.call(0x00a5_3b20, &args![this, stream]);
        assert_eq!(calls(&e, NI_OBJECT_SAVE_BINARY), vec![vec![this, stream]]);
        let writes = calls(&e, WRITE);
        assert_eq!(writes.len(), 2);
        assert_eq!(writes[0][..3], [output, this + NUM_KEYS, 4]);
        assert_eq!(writes[1][..3], [output, this + KEY_TYPE, 4]);
        assert_eq!(calls(&e, SAVE), vec![vec![stream, 0x6000, 2]]);
        assert_eq!(e.mem.u32(output + 4), 8);
        // No keys: only the count.
        e.mem.set_u32(this + NUM_KEYS, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a5_3b20, &args![this, stream]);
        assert_eq!(calls(&e, WRITE).len(), 1);
        assert!(calls(&e, SAVE).is_empty());
    }

    #[test]
    fn get_viewer_strings_appends_name_and_count() {
        let mut e = engine();
        e.map(NI_BOOL_DATA_RTTI, 4);
        e.set_global(NI_BOOL_DATA_RTTI, 0x0109_2222u32);
        e.register(NI_OBJECT_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| word(0xb1));
        e.register(NI_GET_VIEWER_STRING_UNSIGNED, |_, _| word(0xb2));
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        let this = bool_data(&mut e, 1, 6, 0);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xe, 2);
        e.call(0x00a5_3ba0, &args![this, strings]);
        assert_eq!(calls(&e, NI_GET_VIEWER_STRING), vec![vec![0x0109_2222]]);
        assert_eq!(
            calls(&e, NI_GET_VIEWER_STRING_UNSIGNED),
            vec![vec![NAME_NUM_KEYS, 6]]
        );
        // Size 0 of 1 fits; size 1 grows to 1 + 2.
        assert_eq!(calls(&e, STRINGS_GROW), vec![vec![strings, 3]]);
        let sets = calls(&e, STRINGS_SET_AT);
        assert_eq!((sets[0][1], sets[1][1]), (0, 1));
    }
}
