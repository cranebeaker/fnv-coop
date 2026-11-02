//! `NiAnimation/nifloatdata.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiFloatData` (Xbox PDB, 0x18 bytes; PC offsets as the code uses them):
//! `NiObject`, then `m_uiNumKeys` +0x08, `m_pkKeys` +0x0c, `m_eType` +0x10
//! and `m_ucKeySize` +0x14. The key arrays are handled through the
//! animation key tables in `.data`, indexed by the key type (filled by the
//! static initializers): `011f38a0` (a `cdecl` function deleting a key
//! array), `011f3758` (the key size, one byte per type), `011f3db0` (a
//! `cdecl` function `(stream, count)` that reads a key array and returns
//! it) and `011f3780` (a `cdecl` function `(keys, count, key size)` run on
//! the array read). Their Xbox PDB names are not confirmed.
//!
//! None of these functions has an exception frame or a `/GS` check.

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of `NiFloatData` (RTTI `.?AVNiFloatData@@`).
const NI_FLOAT_DATA_VTABLE: u32 = 0x0102_dabc;
/// `NiFloatData` size (Xbox PDB and the `CreateObject` allocation).
const NI_FLOAT_DATA_SIZE: u32 = 0x18;
/// `m_uiNumKeys`, `m_pkKeys`, `m_eType`, `m_ucKeySize` (Xbox PDB).
const NUM_KEYS: u32 = 0x08;
const KEYS: u32 = 0x0c;
const KEY_TYPE: u32 = 0x10;
const KEY_SIZE: u32 = 0x14;

/// The animation key tables, indexed by key type (see the module comment).
const KEY_DELETE_TABLE: u32 = 0x011f_38a0;
const KEY_SIZE_TABLE: u32 = 0x011f_3758;
const KEY_LOAD_TABLE: u32 = 0x011f_3db0;
const KEY_AFTER_LOAD_TABLE: u32 = 0x011f_3780;

/// `00aa13e0` (`cdecl`, one word): allocates from the Ni allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `NiObject::NiObject` (`00a5d3a0`, `NiMain/niobject.cpp`).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// `NiObject::~NiObject` (`00a5d3d0`, `NiMain/niobject.cpp`).
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
/// `NiObject::LoadBinary` (`00a5d610`, `NiMain/niobject.cpp`).
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;

/// `NiStream::m_pkIstr` (Xbox PDB) +0x24c, and the input stream's
/// `m_uiAbsoluteCurrentPos` (+4) and `m_pfnRead` (+8, `cdecl`
/// `(stream, buffer, size, &component size, component count)`, returns the
/// bytes read).
const STREAM_INPUT: u32 = 0x24c;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;

/// Deletes the key array (when there is one) through the delete function
/// of its current key type.
fn delete_keys(e: &mut Engine, this: u32) {
    let keys = e.mem.u32(this.wrapping_add(KEYS));
    if keys != 0 {
        let key_type = e.mem.u32(this.wrapping_add(KEY_TYPE));
        let delete = e
            .mem
            .u32(KEY_DELETE_TABLE.wrapping_add(key_type.wrapping_mul(4)));
        e.call(delete, &args![keys]);
    }
}

// Translated from 00a3cc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatData::~NiFloatData` (unnamed in the engine map; the
/// destructor its scalar deleting destructor `0052cf9a` calls): stores the
/// vtable, deletes the key array through the delete function of its key
/// type, then the `NiObject` destructor (`00a5d3d0`, a tail jump).
pub fn ni_float_data_destruct(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    e.mem.set_u32(this, NI_FLOAT_DATA_VTABLE);
    delete_keys(e, this);
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00a3cd00 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiFloatData::CreateObject` (the loader `00a2d8a0` registers for
/// `"NiFloatData"`; Ghidra has no function here): allocates 0x18 bytes
/// (`00aa13e0`); when that succeeds runs the `NiObject` constructor, clears
/// the key count, keys and type and the key size byte, and stores the
/// vtable. Returns the object, or 0.
pub fn ni_float_data_create_object(e: &mut Engine) -> Ptr {
    let object = e.call(NI_ALLOCATE, &args![NI_FLOAT_DATA_SIZE]).u32();
    if object == 0 {
        return Ptr::new(0);
    }
    e.call(NI_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object.wrapping_add(NUM_KEYS), 0);
    e.mem.set_u32(object.wrapping_add(KEYS), 0);
    e.mem.set_u32(object.wrapping_add(KEY_TYPE), 0);
    e.mem.set_u8(object.wrapping_add(KEY_SIZE), 0);
    e.mem.set_u32(object, NI_FLOAT_DATA_VTABLE);
    Ptr::new(object)
}

// Translated from 00a3cdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatData::ReplaceAnim` (Xbox PDB): deletes the current key array
/// (through its type's delete function), then takes `keys`, `num_keys` and
/// `key_type` with the type's key size when all three are non-zero, and
/// clears all four fields otherwise.
pub fn ni_float_data_replace_anim(
    e: &mut Engine,
    this: Ptr,
    keys: u32,
    num_keys: u32,
    key_type: u32,
) {
    let this = this.addr();
    delete_keys(e, this);
    if keys != 0 && num_keys != 0 && key_type != 0 {
        let key_size = e.mem.u8(KEY_SIZE_TABLE.wrapping_add(key_type));
        e.mem.set_u32(this.wrapping_add(KEYS), keys);
        e.mem.set_u8(this.wrapping_add(KEY_SIZE), key_size);
        e.mem.set_u32(this.wrapping_add(NUM_KEYS), num_keys);
        e.mem.set_u32(this.wrapping_add(KEY_TYPE), key_type);
    } else {
        e.mem.set_u32(this.wrapping_add(NUM_KEYS), 0);
        e.mem.set_u32(this.wrapping_add(KEYS), 0);
        e.mem.set_u8(this.wrapping_add(KEY_SIZE), 0);
        e.mem.set_u32(this.wrapping_add(KEY_TYPE), 0);
    }
}

/// One 4-byte read through the stream's `m_pfnRead` into `buffer`, the
/// component size word at `component_size` set to 4 first; advances the
/// position by the bytes read.
fn read_word(e: &mut Engine, stream: u32, buffer: u32, component_size: u32) {
    let input = e.mem.u32(stream.wrapping_add(STREAM_INPUT));
    let read = e.mem.u32(input.wrapping_add(BINARY_STREAM_READ));
    e.mem.set_u32(component_size, 4);
    let bytes = e
        .call(read, &args![input, buffer, 4u32, component_size, 1u32])
        .u32();
    let position = e.mem.u32(input.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        input.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

// Translated from 00a3cf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatData::LoadBinary` (Xbox PDB): `NiObject::LoadBinary`, then the
/// key count (4 bytes, read into the argument's own stack word). With keys,
/// the key type (4 bytes) follows; the type's key size is stored, the
/// type's load function reads the array (`(stream, count)`), the type's
/// after-load function runs on it (`(keys, count, key size)`), and
/// `ReplaceAnim` takes it. A count of 0 reads nothing more.
pub fn ni_float_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    let stream = stream.addr();
    // The frame: the component size word +0, the key type +4, and the
    // argument's own word (the key count) +8.
    e.with_stack(12, |e, frame| {
        let component_size = frame.addr();
        let key_type_word = component_size + 4;
        let num_keys_word = component_size + 8;
        e.mem.set_u32(num_keys_word, stream);
        read_word(e, stream, num_keys_word, component_size);
        let num_keys = e.mem.u32(num_keys_word);
        if num_keys == 0 {
            return;
        }
        read_word(e, stream, key_type_word, component_size);
        let key_type = e.mem.u32(key_type_word);
        let key_size = e.mem.u8(KEY_SIZE_TABLE.wrapping_add(key_type));
        e.mem.set_u8(this.addr().wrapping_add(KEY_SIZE), key_size);
        let load = e
            .mem
            .u32(KEY_LOAD_TABLE.wrapping_add(key_type.wrapping_mul(4)));
        let keys = e.call(load, &args![stream, num_keys]).u32();
        let after_load = e
            .mem
            .u32(KEY_AFTER_LOAD_TABLE.wrapping_add(key_type.wrapping_mul(4)));
        let key_size = e.mem.u8(this.addr().wrapping_add(KEY_SIZE));
        e.call(after_load, &args![keys, num_keys, u32::from(key_size)]);
        ni_float_data_replace_anim(e, this, keys, num_keys, key_type);
    });
}

/// `NiFloatData`'s `NiRTTI` record (its first word is the class name; the
/// static initializer `00fb4c50` builds it with `"NiFloatData"`).
const NI_FLOAT_DATA_RTTI: u32 = 0x011f_3748;
/// The float rows of the key comparison (`cdecl` `(key, other key)`, true in
/// `AL` when equal) and key save (`cdecl` `(stream, keys, count)`) tables,
/// indexed by key type like the others (module comment).
const KEY_EQUAL_TABLE: u32 = 0x011f_3ae0;
const KEY_SAVE_TABLE: u32 = 0x011f_3a50;
/// `NiAnimationKey::GuaranteeKeyAtStartAndEnd` (`00a3d930`, `cdecl`, six
/// words: key content, key type, address of the keys, address of the key
/// count, start, end).
const GUARANTEE_KEY_AT_START_AND_END: u32 = 0x00a3_d930;
/// `NiAnimationKey::CopySequence` (`00a3d630`, `cdecl`, eight words: key
/// content, key type, keys, key count, start, end, address of the new keys,
/// address of the new count).
const COPY_SEQUENCE: u32 = 0x00a3_d630;
/// The float key content (the first argument of the two above).
const FLOAT_KEY_CONTENT: u32 = 0;
/// `NiObject::IsEqual` (`00a5d470`), `NiObject::CreateDeepCopy` (`00a5d510`;
/// `thiscall`: address of the result pointer) and
/// `NiObject::GetViewerStrings` (`00a5d6e0`), `NiMain/niobject.cpp`.
const NI_OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const NI_OBJECT_CREATE_DEEP_COPY: u32 = 0x00a5_d510;
const NI_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
/// `00a89af0` (`thiscall`, `RET 4`: stream; an empty body): the base
/// `SaveBinary` call.
const BASE_SAVE_BINARY: u32 = 0x00a8_9af0;
/// `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiRefObject::DeleteThis` (virtual slot +0x04).
const SLOT_DELETE_THIS: u32 = 0x04;
/// `NiGetViewerString` (`00a7d330`, `cdecl`: the class name) and
/// `NiGetViewerString_ov8` (engine map; `00a7d510`, `cdecl`: label, value).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_VALUE: u32 = 0x00a7_d510;
/// The viewer-string array's grow (`0096ad30`; `thiscall`: new size) and
/// store (`0096ae90`; `thiscall`: index, address of the string).
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// `"m_uiNumKeys"`.
const LABEL_NUM_KEYS: u32 = 0x0109_7164;
/// `NiStream::m_pkOstr` (Xbox PDB) +0x250 and the output stream's
/// `m_pfnWrite` (+0xc, `cdecl` like the read function).
const STREAM_OUTPUT: u32 = 0x250;
const BINARY_STREAM_WRITE: u32 = 0xc;

/// The inline `NiPointer` release: `InterlockedDecrement` of the count at
/// `+4` and, when it reaches zero, the object's `DeleteThis`.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

/// One 4-byte write of `buffer` through the stream's `m_pfnWrite`, the
/// component size word at `component_size` set to 4 first; advances the
/// position by the bytes written.
fn write_word(e: &mut Engine, stream: u32, buffer: u32, component_size: u32) {
    let output = e.mem.u32(stream.wrapping_add(STREAM_OUTPUT));
    let write = e.mem.u32(output.wrapping_add(BINARY_STREAM_WRITE));
    e.mem.set_u32(component_size, 4);
    let bytes = e
        .call(write, &args![output, buffer, 4u32, component_size, 1u32])
        .u32();
    let position = e.mem.u32(output.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        output.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

/// Appends `text` to the viewer-string array at `strings`: when the 16-bit
/// size (+0xA) has reached the capacity (+0x8) the array grows to the size
/// plus the step (+0xE) first; the string goes in at the old size through a
/// stack word holding it.
fn add_viewer_string(e: &mut Engine, strings: u32, text: u32) {
    let size = u32::from(e.mem.u16(strings.wrapping_add(0xa)));
    if size >= u32::from(e.mem.u16(strings.wrapping_add(8))) {
        let step = u32::from(e.mem.u16(strings.wrapping_add(0xe)));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

// Translated from 00a3cc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatData::GuaranteeKeysAtStartAndEnd` (Xbox PDB):
/// `NiAnimationKey::GuaranteeKeyAtStartAndEnd` on the float keys (content
/// 0, the key type, a stack copy of the key pointer, the address of
/// `m_uiNumKeys`, `start` and `end` copied through the x87 stack); the key
/// pointer it leaves becomes `m_pkKeys`.
pub fn ni_float_data_guarantee_keys_at_start_and_end(
    e: &mut Engine,
    this: Ptr,
    start: f32,
    end: f32,
) {
    let this = this.addr();
    e.with_stack(4, |e, keys| {
        let current = e.mem.u32(this.wrapping_add(KEYS));
        e.mem.set_u32(keys.addr(), current);
        let key_type = e.mem.u32(this.wrapping_add(KEY_TYPE));
        e.call(
            GUARANTEE_KEY_AT_START_AND_END,
            &args![
                FLOAT_KEY_CONTENT,
                key_type,
                keys,
                this.wrapping_add(NUM_KEYS),
                x87_quiet_bits(start.to_bits()),
                x87_quiet_bits(end.to_bits())
            ],
        );
        let keys = e.mem.u32(keys.addr());
        e.mem.set_u32(this.wrapping_add(KEYS), keys);
    });
}

// Translated from 00a3ccb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xC`: keys, key count, key
/// type): like `ReplaceAnim` without deleting the old array. With all three
/// non-zero it stores them and the type's key size (`011f3758`); otherwise
/// it clears the count, keys, type and key size.
pub fn fn_00a3ccb0(e: &mut Engine, this: Ptr, keys: u32, num_keys: u32, key_type: u32) {
    let this = this.addr();
    if keys != 0 && num_keys != 0 && key_type != 0 {
        let key_size = e.mem.u8(KEY_SIZE_TABLE.wrapping_add(key_type));
        e.mem.set_u32(this.wrapping_add(KEYS), keys);
        e.mem.set_u32(this.wrapping_add(NUM_KEYS), num_keys);
        e.mem.set_u32(this.wrapping_add(KEY_TYPE), key_type);
        e.mem.set_u8(this.wrapping_add(KEY_SIZE), key_size);
    } else {
        e.mem.set_u32(this.wrapping_add(NUM_KEYS), 0);
        e.mem.set_u32(this.wrapping_add(KEYS), 0);
        e.mem.set_u32(this.wrapping_add(KEY_TYPE), 0);
        e.mem.set_u8(this.wrapping_add(KEY_SIZE), 0);
    }
}

// Translated from 00a3cd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatData::IsEqual` (Xbox PDB): `NiObject::IsEqual`, the same key
/// count, type and key size, then every key pair equal by the type's
/// comparison function (`011f3ae0`, `(key, other key)`, the stride the key
/// size).
pub fn ni_float_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(NI_OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let this = this.addr();
    let other = other.addr();
    let num_keys = e.mem.u32(this.wrapping_add(NUM_KEYS));
    if num_keys != e.mem.u32(other.wrapping_add(NUM_KEYS)) {
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
    if num_keys != 0 {
        loop {
            let offset = u32::from(e.mem.u8(this.wrapping_add(KEY_SIZE))).wrapping_mul(index);
            let other_key = e.mem.u32(other.wrapping_add(KEYS)).wrapping_add(offset);
            let key = e.mem.u32(this.wrapping_add(KEYS)).wrapping_add(offset);
            if !e.call(equal, &args![key, other_key]).bool() {
                return false;
            }
            index = index.wrapping_add(1);
            if index >= e.mem.u32(this.wrapping_add(NUM_KEYS)) {
                break;
            }
        }
    }
    true
}

// Translated from 00a3ce20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatData::GetSequenceData` (Xbox PDB; `RET 0xC`: start, end, address
/// of the result pointer). A deep copy (`NiObject::CreateDeepCopy` into a
/// local pointer starting at 0), held counted; with keys,
/// `NiAnimationKey::CopySequence` (content 0, `start` and `end` copied
/// through the x87 stack) makes the keys between `start` and `end`, and the
/// copy's `ReplaceAnim` takes them with this data's type. The copy is then
/// assigned to the result pointer (the old one released, the new one
/// counted, when they differ) and both local holds are released.
pub fn ni_float_data_get_sequence_data(
    e: &mut Engine,
    this: Ptr,
    start: f32,
    end: f32,
    result: Ptr,
) {
    let source = this.addr();
    // The frame: the deep copy's pointer +0, the new key count +4, the new
    // keys +8.
    e.with_stack(12, |e, frame| {
        let deep_copy = frame.addr();
        let new_count = deep_copy + 4;
        let new_keys = deep_copy + 8;
        e.mem.set_u32(deep_copy, 0);
        e.call(NI_OBJECT_CREATE_DEEP_COPY, &args![this, deep_copy]);
        let copy = e.mem.u32(deep_copy);
        if copy != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![copy.wrapping_add(4)]);
        }
        let num_keys = e.mem.u32(source.wrapping_add(NUM_KEYS));
        if num_keys != 0 {
            let keys = e.mem.u32(source.wrapping_add(KEYS));
            e.mem.set_u32(new_keys, 0);
            e.mem.set_u32(new_count, 0);
            let key_type = e.mem.u32(source.wrapping_add(KEY_TYPE));
            e.call(
                COPY_SEQUENCE,
                &args![
                    FLOAT_KEY_CONTENT,
                    key_type,
                    keys,
                    num_keys,
                    x87_quiet_bits(start.to_bits()),
                    x87_quiet_bits(end.to_bits()),
                    new_keys,
                    new_count
                ],
            );
            let key_type = e.mem.u32(source.wrapping_add(KEY_TYPE));
            let count = e.mem.u32(new_count);
            let keys = e.mem.u32(new_keys);
            ni_float_data_replace_anim(e, Ptr::new(copy), keys, count, key_type);
        }
        let slot = result.addr();
        let old = e.mem.u32(slot);
        if old != copy {
            if old != 0 {
                release(e, old);
            }
            e.mem.set_u32(slot, copy);
            if copy != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![copy.wrapping_add(4)]);
            }
        }
        if copy != 0 {
            release(e, copy);
        }
        let held = e.mem.u32(deep_copy);
        if held != 0 {
            release(e, held);
        }
    });
}

// Translated from 00a3cfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatData::SaveBinary` (Xbox PDB): the base call (`00a89af0`), the
/// key count (4 bytes through the output stream's write function, the
/// component size in the argument's own stack word) and, with keys, the key
/// type (4 bytes) and the keys through the type's save function
/// (`011f3a50`, `(stream, keys, count)`).
pub fn ni_float_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    let this = this.addr();
    let stream = stream.addr();
    e.with_stack(4, |e, component_size| {
        let component_size = component_size.addr();
        write_word(e, stream, this.wrapping_add(NUM_KEYS), component_size);
        if e.mem.u32(this.wrapping_add(NUM_KEYS)) == 0 {
            return;
        }
        write_word(e, stream, this.wrapping_add(KEY_TYPE), component_size);
        let num_keys = e.mem.u32(this.wrapping_add(NUM_KEYS));
        let keys = e.mem.u32(this.wrapping_add(KEYS));
        let key_type = e.mem.u32(this.wrapping_add(KEY_TYPE));
        let save = e
            .mem
            .u32(KEY_SAVE_TABLE.wrapping_add(key_type.wrapping_mul(4)));
        e.call(save, &args![stream, keys, num_keys]);
    });
}

// Translated from 00a3d050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatData::GetViewerStrings` (Xbox PDB): `NiObject::GetViewerStrings`,
/// then the class name (the `NiRTTI` record's first word through
/// `NiGetViewerString`) and the key count
/// (`NiGetViewerString_ov8("m_uiNumKeys", count)`).
pub fn ni_float_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let strings = strings.addr();
    e.call(NI_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(NI_FLOAT_DATA_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![name]).u32();
    add_viewer_string(e, strings, text);
    let num_keys = e.mem.u32(this.addr().wrapping_add(NUM_KEYS));
    let text = e
        .call(NI_GET_VIEWER_STRING_VALUE, &args![LABEL_NUM_KEYS, num_keys])
        .u32();
    add_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a3_cc80, ni_float_data_destruct(Ptr)),
        entry!(0x00a3_cd00, ni_float_data_create_object() -> Ptr),
        entry!(0x00a3_cdc0, ni_float_data_replace_anim(Ptr, u32, u32, u32)),
        entry!(0x00a3_cf10, ni_float_data_load_binary(Ptr, Ptr)),
        entry!(
            0x00a3_cc40,
            ni_float_data_guarantee_keys_at_start_and_end(Ptr, f32, f32)
        ),
        entry!(0x00a3_ccb0, fn_00a3ccb0(Ptr, u32, u32, u32)),
        entry!(0x00a3_cd40, ni_float_data_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a3_ce20,
            ni_float_data_get_sequence_data(Ptr, f32, f32, Ptr)
        ),
        entry!(0x00a3_cfd0, ni_float_data_save_binary(Ptr, Ptr)),
        entry!(0x00a3_d050, ni_float_data_get_viewer_strings(Ptr, Ptr)),
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

    /// Key type 3: delete `DELETE`, size 0x0c, load `LOAD`, after-load
    /// `AFTER_LOAD`.
    fn tables(e: &mut Engine, log: &Log) {
        e.map(KEY_SIZE_TABLE, 0x10);
        e.map(KEY_DELETE_TABLE, 0x40);
        e.map(KEY_LOAD_TABLE, 0x40);
        e.map(KEY_AFTER_LOAD_TABLE, 0x40);
        e.mem.set_u8(KEY_SIZE_TABLE + 3, 0x0c);
        e.mem.set_u32(KEY_DELETE_TABLE + 12, DELETE);
        e.mem.set_u32(KEY_LOAD_TABLE + 12, LOAD);
        e.mem.set_u32(KEY_AFTER_LOAD_TABLE + 12, AFTER_LOAD);
        logged(e, log, DELETE, 0);
        logged(e, log, LOAD, 0x7000);
        logged(e, log, AFTER_LOAD, 0);
    }

    #[test]
    fn destructor_deletes_the_keys_then_runs_the_object_destructor() {
        let mut e = Engine::new();
        let log = Log::default();
        tables(&mut e, &log);
        logged(&mut e, &log, NI_OBJECT_DESTRUCT, 0);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + KEYS, 0x5000);
        e.mem.set_u32(this + KEY_TYPE, 3);
        e.call(0x00a3_cc80, &args![this]);
        assert_eq!(e.mem.u32(this), NI_FLOAT_DATA_VTABLE);
        assert_eq!(
            *log.borrow(),
            [(DELETE, vec![0x5000]), (NI_OBJECT_DESTRUCT, vec![this])]
        );
        log.borrow_mut().clear();
        e.mem.set_u32(this + KEYS, 0);
        e.call(0x00a3_cc80, &args![this]);
        assert_eq!(*log.borrow(), [(NI_OBJECT_DESTRUCT, vec![this])]);
    }

    #[test]
    fn create_object_builds_an_empty_data_object_or_returns_null() {
        let mut e = Engine::new();
        let log = Log::default();
        let block = e.mem.alloc(0x18);
        e.mem.write(block, &[0xaa; 0x18]);
        logged(&mut e, &log, NI_ALLOCATE, block);
        logged(&mut e, &log, NI_OBJECT_CONSTRUCT, block);
        assert_eq!(e.call(0x00a3_cd00, &args![]).u32(), block);
        assert_eq!(e.mem.u32(block), NI_FLOAT_DATA_VTABLE);
        assert_eq!(e.mem.bytes(block + 8, 13), vec![0; 13]);
        assert_eq!(e.mem.u8(block + 0x15), 0xaa);
        assert_eq!(
            *log.borrow(),
            [
                (NI_ALLOCATE, vec![0x18]),
                (NI_OBJECT_CONSTRUCT, vec![block])
            ]
        );
        logged(&mut e, &log, NI_ALLOCATE, 0);
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00a3_cd00, &args![]).u32(), 0);
        assert_eq!(*log.borrow(), [(NI_ALLOCATE, vec![0x18])]);
    }

    #[test]
    fn replace_anim_takes_the_new_keys_only_when_all_three_are_set() {
        let mut e = Engine::new();
        let log = Log::default();
        tables(&mut e, &log);
        let this = e.mem.alloc(0x18);
        e.call(0x00a3_cdc0, &args![this, 0x6000u32, 5u32, 3u32]);
        assert_eq!(e.mem.u32(this + KEYS), 0x6000);
        assert_eq!(e.mem.u32(this + NUM_KEYS), 5);
        assert_eq!(e.mem.u32(this + KEY_TYPE), 3);
        assert_eq!(e.mem.u8(this + KEY_SIZE), 0x0c);
        assert!(log.borrow().is_empty());
        for (keys, count, key_type) in [(0u32, 5u32, 3u32), (0x6100, 0, 3), (0x6100, 5, 0)] {
            e.mem.set_u32(this + KEYS, 0x6000);
            e.mem.set_u32(this + KEY_TYPE, 3);
            e.mem.set_u32(this + NUM_KEYS, 5);
            log.borrow_mut().clear();
            e.call(0x00a3_cdc0, &args![this, keys, count, key_type]);
            assert_eq!(*log.borrow(), [(DELETE, vec![0x6000])]);
            assert_eq!(e.mem.bytes(this + 8, 13), vec![0; 13]);
        }
    }

    /// `LoadBinary` on a stream whose reads hand out `words` in turn.
    fn load(words: &'static [u32]) -> (Engine, Log, u32, u32) {
        let mut e = Engine::new();
        let log = Log::default();
        tables(&mut e, &log);
        logged(&mut e, &log, NI_OBJECT_LOAD_BINARY, 0);
        let stream = e.mem.alloc(0x5c4);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + STREAM_INPUT, input);
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
        e.call(0x00a3_cf10, &args![this, stream]);
        (e, log, this, stream)
    }

    #[test]
    fn load_binary_reads_the_count_type_and_keys() {
        let (e, log, this, stream) = load(&[5, 3]);
        let input = e.mem.u32(stream + STREAM_INPUT);
        assert_eq!(e.mem.u32(input + BINARY_STREAM_POSITION), 8);
        assert_eq!(
            *log.borrow(),
            [
                (NI_OBJECT_LOAD_BINARY, vec![this, stream]),
                (READ, vec![input, 4, 4, 1]),
                (READ, vec![input, 4, 4, 1]),
                (LOAD, vec![stream, 5]),
                (AFTER_LOAD, vec![0x7000, 5, 0x0c]),
            ]
        );
        assert_eq!(e.mem.u32(this + KEYS), 0x7000);
        assert_eq!(e.mem.u32(this + NUM_KEYS), 5);
        assert_eq!(e.mem.u32(this + KEY_TYPE), 3);
        assert_eq!(e.mem.u8(this + KEY_SIZE), 0x0c);
    }

    #[test]
    fn load_binary_without_keys_reads_only_the_count() {
        let (e, log, this, stream) = load(&[0]);
        let input = e.mem.u32(stream + STREAM_INPUT);
        assert_eq!(e.mem.u32(input + BINARY_STREAM_POSITION), 4);
        assert_eq!(
            *log.borrow(),
            [
                (NI_OBJECT_LOAD_BINARY, vec![this, stream]),
                (READ, vec![input, 4, 4, 1]),
            ]
        );
        assert_eq!(e.mem.u32(this + KEYS), 0);
    }
}

#[cfg(test)]
mod n003_tests {
    use super::*;

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const DELETE: u32 = 0x00ee_0100;
    const EQUAL: u32 = 0x00ee_0200;
    const SAVE: u32 = 0x00ee_0300;
    const WRITE: u32 = 0x00ee_0400;
    const DELETE_THIS: u32 = 0x00ee_0500;

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

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Key type 3: delete `DELETE`, size 0x0c, comparison `EQUAL` (equal
    /// when the first words match), save `SAVE`; counting `Interlocked`
    /// doubles and a logged `DeleteThis`.
    fn engine(log: &Log) -> Engine {
        let mut e = Engine::new();
        e.map(KEY_SIZE_TABLE, 0x10);
        e.map(KEY_DELETE_TABLE, 0x40);
        e.map(KEY_EQUAL_TABLE, 0x40);
        e.map(KEY_SAVE_TABLE, 0x40);
        e.mem.set_u8(KEY_SIZE_TABLE + 3, 0x0c);
        e.mem.set_u32(KEY_DELETE_TABLE + 12, DELETE);
        e.mem.set_u32(KEY_EQUAL_TABLE + 12, EQUAL);
        e.mem.set_u32(KEY_SAVE_TABLE + 12, SAVE);
        logged(&mut e, log, DELETE, 0);
        logged(&mut e, log, SAVE, 0);
        logged(&mut e, log, DELETE_THIS, 0);
        let l = log.clone();
        e.register_double(EQUAL, move |e, a| {
            l.borrow_mut().push((EQUAL, a.to_vec()));
            ret((e.mem.u32(a[0]) == e.mem.u32(a[1])) as u32)
        });
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            ret(count)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            ret(count)
        });
        e
    }

    /// A counted data object (count `count`) with a `DeleteThis` double.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + SLOT_DELETE_THIS, DELETE_THIS);
        let object = e.mem.alloc(0x18);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn guarantee_keys_passes_the_key_pointer_and_takes_it_back() {
        let log = Log::default();
        let mut e = engine(&log);
        let l = log.clone();
        e.register_double(GUARANTEE_KEY_AT_START_AND_END, move |e, a| {
            l.borrow_mut().push((
                GUARANTEE_KEY_AT_START_AND_END,
                vec![a[0], a[1], e.mem.u32(a[2]), a[3], a[4], a[5]],
            ));
            e.mem.set_u32(a[2], 0x7100);
            e.mem.set_u32(a[3], 9);
            ret(0)
        });
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + KEYS, 0x7000);
        e.mem.set_u32(this + KEY_TYPE, 3);
        e.call(0x00a3_cc40, &args![this, 0x7f80_0001u32, 2.0f32]);
        assert_eq!(
            calls(&log, GUARANTEE_KEY_AT_START_AND_END),
            vec![vec![
                0,
                3,
                0x7000,
                this + NUM_KEYS,
                0x7fc0_0001,
                2.0f32.to_bits()
            ]]
        );
        assert_eq!(e.mem.u32(this + KEYS), 0x7100);
        assert_eq!(e.mem.u32(this + NUM_KEYS), 9);
    }

    #[test]
    fn set_anim_stores_without_deleting_or_clears() {
        let log = Log::default();
        let mut e = engine(&log);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + KEYS, 0x5000);
        e.call(0x00a3_ccb0, &args![this, 0x6000u32, 4u32, 3u32]);
        assert!(calls(&log, DELETE).is_empty());
        assert_eq!(e.mem.u32(this + KEYS), 0x6000);
        assert_eq!(e.mem.u32(this + NUM_KEYS), 4);
        assert_eq!(e.mem.u32(this + KEY_TYPE), 3);
        assert_eq!(e.mem.u8(this + KEY_SIZE), 0x0c);
        for (keys, count, key_type) in [(0u32, 4u32, 3u32), (0x6000, 0, 3), (0x6000, 4, 0)] {
            e.mem.write(this + 8, &[0xaa; 13]);
            e.call(0x00a3_ccb0, &args![this, keys, count, key_type]);
            assert_eq!(e.mem.bytes(this + 8, 13), vec![0; 13]);
        }
        assert!(calls(&log, DELETE).is_empty());
    }

    /// Data of type 3 with three keys (stride 0x0c) whose first words are
    /// `firsts`.
    fn keyed(e: &mut Engine, firsts: [u32; 3]) -> u32 {
        let this = e.mem.alloc(0x18);
        let keys = e.mem.alloc(0x24);
        for (i, v) in firsts.iter().enumerate() {
            e.mem.set_u32(keys + 0xc * i as u32, *v);
        }
        e.mem.set_u32(this + NUM_KEYS, 3);
        e.mem.set_u32(this + KEYS, keys);
        e.mem.set_u32(this + KEY_TYPE, 3);
        e.mem.set_u8(this + KEY_SIZE, 0x0c);
        this
    }

    #[test]
    fn is_equal_compares_the_header_then_every_key() {
        let log = Log::default();
        let mut e = engine(&log);
        e.register(NI_OBJECT_IS_EQUAL, |e, a| ret(e.mem.u32(a[0] + 4)));
        let this = keyed(&mut e, [1, 2, 3]);
        let other = keyed(&mut e, [1, 2, 3]);
        e.mem.set_u32(this + 4, 1);
        assert!(e.call(0x00a3_cd40, &args![this, other]).bool());
        let this_keys = e.mem.u32(this + KEYS);
        let other_keys = e.mem.u32(other + KEYS);
        assert_eq!(
            calls(&log, EQUAL),
            vec![
                vec![this_keys, other_keys],
                vec![this_keys + 0xc, other_keys + 0xc],
                vec![this_keys + 0x18, other_keys + 0x18],
            ]
        );
        // A key differs: false at it.
        log.borrow_mut().clear();
        e.mem.set_u32(other_keys + 0xc, 7);
        assert!(!e.call(0x00a3_cd40, &args![this, other]).bool());
        assert_eq!(calls(&log, EQUAL).len(), 2);
        // Header differences and the base.
        log.borrow_mut().clear();
        for (offset, value) in [(NUM_KEYS, 2u32), (KEY_TYPE, 4)] {
            let other = keyed(&mut e, [1, 2, 3]);
            e.mem.set_u32(other + offset, value);
            assert!(!e.call(0x00a3_cd40, &args![this, other]).bool());
        }
        let other = keyed(&mut e, [1, 2, 3]);
        e.mem.set_u8(other + KEY_SIZE, 8);
        assert!(!e.call(0x00a3_cd40, &args![this, other]).bool());
        e.mem.set_u32(this + 4, 0);
        assert!(!e.call(0x00a3_cd40, &args![this, other]).bool());
        assert!(calls(&log, EQUAL).is_empty());
        // No keys: equal without comparisons.
        let empty = e.mem.alloc(0x18);
        let other = e.mem.alloc(0x18);
        e.mem.set_u32(empty + 4, 1);
        assert!(e.call(0x00a3_cd40, &args![empty, other]).bool());
    }

    #[test]
    fn get_sequence_data_hands_out_a_trimmed_deep_copy() {
        let log = Log::default();
        let mut e = engine(&log);
        let copy = counted(&mut e, 1);
        let old = counted(&mut e, 1);
        let l = log.clone();
        e.register_double(NI_OBJECT_CREATE_DEEP_COPY, move |e, a| {
            l.borrow_mut()
                .push((NI_OBJECT_CREATE_DEEP_COPY, vec![a[0], e.mem.u32(a[1])]));
            e.mem.set_u32(a[1], copy);
            ret(0)
        });
        let l = log.clone();
        e.register_double(COPY_SEQUENCE, move |e, a| {
            l.borrow_mut().push((
                COPY_SEQUENCE,
                vec![
                    a[0],
                    a[1],
                    a[2],
                    a[3],
                    a[4],
                    a[5],
                    e.mem.u32(a[6]),
                    e.mem.u32(a[7]),
                ],
            ));
            e.mem.set_u32(a[6], 0x7700);
            e.mem.set_u32(a[7], 2);
            ret(0)
        });
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + NUM_KEYS, 5);
        e.mem.set_u32(this + KEYS, 0x7000);
        e.mem.set_u32(this + KEY_TYPE, 3);
        let result = e.mem.alloc(4);
        e.mem.set_u32(result, old);
        e.call(0x00a3_ce20, &args![this, 1.0f32, 0x7f80_0001u32, result]);
        assert_eq!(calls(&log, NI_OBJECT_CREATE_DEEP_COPY), vec![vec![this, 0]]);
        assert_eq!(
            calls(&log, COPY_SEQUENCE),
            vec![vec![0, 3, 0x7000, 5, 1.0f32.to_bits(), 0x7fc0_0001, 0, 0]]
        );
        assert_eq!(e.mem.u32(copy + KEYS), 0x7700);
        assert_eq!(e.mem.u32(copy + NUM_KEYS), 2);
        assert_eq!(e.mem.u32(copy + KEY_TYPE), 3);
        assert_eq!(e.mem.u32(result), copy);
        assert_eq!(calls(&log, DELETE_THIS), vec![vec![old]]);
        // Counted by the hold and the result, released by the hold and the
        // deep copy's local pointer.
        assert_eq!(e.mem.u32(copy + 4), 1);
        // No keys: no sequence copy; the same pointer is not counted again,
        // so the two releases delete it.
        e.mem.set_u32(this + NUM_KEYS, 0);
        e.call(0x00a3_ce20, &args![this, 1.0f32, 2.0f32, result]);
        assert_eq!(calls(&log, COPY_SEQUENCE).len(), 1);
        assert_eq!(e.mem.u32(copy + 4), 0);
        assert_eq!(calls(&log, DELETE_THIS), vec![vec![old], vec![copy]]);
    }

    #[test]
    fn save_binary_writes_the_count_type_and_keys() {
        let log = Log::default();
        let mut e = engine(&log);
        logged(&mut e, &log, BASE_SAVE_BINARY, 0);
        let l = log.clone();
        e.register_double(WRITE, move |e, a| {
            l.borrow_mut()
                .push((WRITE, vec![a[0], a[1], a[2], e.mem.u32(a[3]), a[4]]));
            ret(4)
        });
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + BINARY_STREAM_WRITE, WRITE);
        let stream = e.mem.alloc(0x260);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + NUM_KEYS, 2);
        e.mem.set_u32(this + KEYS, 0x7000);
        e.mem.set_u32(this + KEY_TYPE, 3);
        e.call(0x00a3_cfd0, &args![this, stream]);
        assert_eq!(
            *log.borrow(),
            [
                (BASE_SAVE_BINARY, vec![this, stream]),
                (WRITE, vec![output, this + NUM_KEYS, 4, 4, 1]),
                (WRITE, vec![output, this + KEY_TYPE, 4, 4, 1]),
                (SAVE, vec![stream, 0x7000, 2]),
            ]
        );
        assert_eq!(e.mem.u32(output + BINARY_STREAM_POSITION), 8);
        // Without keys only the count.
        log.borrow_mut().clear();
        e.mem.set_u32(this + NUM_KEYS, 0);
        e.call(0x00a3_cfd0, &args![this, stream]);
        assert_eq!(calls(&log, WRITE).len(), 1);
        assert!(calls(&log, SAVE).is_empty());
    }

    #[test]
    fn get_viewer_strings_adds_the_name_and_the_count() {
        let log = Log::default();
        let mut e = engine(&log);
        logged(&mut e, &log, NI_OBJECT_GET_VIEWER_STRINGS, 0);
        logged(&mut e, &log, NI_GET_VIEWER_STRING, 0x7100);
        logged(&mut e, &log, NI_GET_VIEWER_STRING_VALUE, 0x7200);
        logged(&mut e, &log, STRINGS_GROW, 0);
        let l = log.clone();
        e.register_double(STRINGS_SET_AT, move |e, a| {
            l.borrow_mut()
                .push((STRINGS_SET_AT, vec![a[0], a[1], e.mem.u32(a[2])]));
            let size = e.mem.u16(a[0] + 0xa) + 1;
            e.mem.set_u16(a[0] + 0xa, size);
            ret(0)
        });
        e.map(NI_FLOAT_DATA_RTTI, 4);
        e.mem.set_u32(NI_FLOAT_DATA_RTTI, 0x5151);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + NUM_KEYS, 6);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xe, 3);
        e.call(0x00a3_d050, &args![this, strings]);
        assert_eq!(
            *log.borrow(),
            [
                (NI_OBJECT_GET_VIEWER_STRINGS, vec![this, strings]),
                (NI_GET_VIEWER_STRING, vec![0x5151]),
                (STRINGS_SET_AT, vec![strings, 0, 0x7100]),
                (NI_GET_VIEWER_STRING_VALUE, vec![LABEL_NUM_KEYS, 6]),
                (STRINGS_GROW, vec![strings, 4]),
                (STRINGS_SET_AT, vec![strings, 1, 0x7200]),
            ]
        );
    }
}
