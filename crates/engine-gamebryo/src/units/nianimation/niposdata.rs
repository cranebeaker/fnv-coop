//! `NiAnimation/niposdata.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `NiPosData` (Xbox PDB), 0x18 bytes on PC (the same).
    pub struct NiPosData: 0x18 {
        /// `m_uiNumKeys` (Xbox PDB).
        0x08 m_uiNumKeys: u32,
        /// `m_pkKeys` (Xbox PDB): `NiPosKey*`.
        0x0C m_pkKeys: Ptr,
        /// `m_eType` (Xbox PDB): `NiAnimationKey::KeyType`.
        0x10 m_eType: u32,
        /// `m_ucKeySize` (Xbox PDB).
        0x14 m_ucKeySize: u8,
    }
}

/// `NiPosData`'s vtable (stored by the constructor and destructor).
const NI_POS_DATA_VTABLE: u32 = 0x0109_717c;
/// `00aa13e0` (cdecl, byte count): `NiNew`'s allocator.
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The `NiObject` constructor (`00a5d3a0`, thiscall).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// The `NiObject` destructor (`00a5d3d0`, thiscall).
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
/// `NiObject::LoadBinary` (`00a5d610`, thiscall, stream).
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// The position keys' size in bytes per key type (one byte each).
const KEY_SIZES: u32 = 0x011f_375e;
/// Per key type: the function that creates `count` keys (cdecl: stream,
/// count).
const KEY_CREATORS: u32 = 0x011f_3dc8;
/// Per key type: the function that loads the keys (cdecl: keys, count, key
/// size).
const KEY_LOADERS: u32 = 0x011f_3798;
/// Per key type: the function that deletes an array of keys (cdecl, keys).
const KEY_DELETERS: u32 = 0x011f_38b8;
/// `NiStream`'s input file (+0x24C): read function at +8, position at +4.
const STREAM_INPUT: u32 = 0x24c;

/// Reads a 4-byte word from the stream's input into `destination`, the
/// byte-count word at `count` set to 4 first, and advances the position.
fn read_word(e: &mut Engine, stream: Ptr, destination: u32, count: u32) {
    let input = e.mem.u32(stream.addr().wrapping_add(STREAM_INPUT));
    let read = e.mem.u32(input.wrapping_add(8));
    e.mem.set_u32(count, 4);
    let done = e
        .call(read, &args![input, destination, 4u32, count, 1u32])
        .u32();
    let position = e.mem.u32(input.wrapping_add(4)).wrapping_add(done);
    e.mem.set_u32(input.wrapping_add(4), position);
}

/// Deletes the keys at +0xC (when there are any) through the deleter of
/// the type at +0x10.
fn delete_keys(e: &mut Engine, this: Ptr<NiPosData>) {
    let keys = e.get(this, NiPosData::m_pkKeys).addr();
    if keys != 0 {
        let key_type = e.get(this, NiPosData::m_eType);
        let deleter = e.global::<u32>(KEY_DELETERS.wrapping_add(key_type.wrapping_mul(4)));
        e.call(deleter, &args![keys]);
    }
}

// Translated from 00a49020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPosData`'s create function (no function in Ghidra; read from the
/// bytes): allocates 0x18 bytes, runs the `NiObject` constructor, clears
/// the key fields and stores the vtable; null when the allocation fails.
pub fn fn_00a49020(e: &mut Engine) -> Ptr<NiPosData> {
    let object = e.call(NI_ALLOCATE, &args![0x18u32]).u32();
    if object == 0 {
        return Ptr::new(0);
    }
    e.call(NI_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u32(object + 8, 0);
    e.mem.set_u32(object + 0xc, 0);
    e.mem.set_u32(object + 0x10, 0);
    e.mem.set_u8(object + 0x14, 0);
    e.mem.set_u32(object, NI_POS_DATA_VTABLE);
    Ptr::new(object)
}

// Translated from 00a490e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPosData::ReplaceAnim` (Xbox PDB name in the engine map): deletes the
/// old keys, then takes `keys` (`count` of type `key_type`, with that type's
/// key size) when both are non-zero, otherwise clears all four fields.
pub fn ni_pos_data_replace_anim(
    e: &mut Engine,
    this: Ptr<NiPosData>,
    keys: u32,
    count: u32,
    key_type: u32,
) {
    delete_keys(e, this);
    if keys != 0 && count != 0 {
        let size = e.global::<u8>(KEY_SIZES.wrapping_add(key_type));
        e.set(this, NiPosData::m_pkKeys, Ptr::new(keys));
        e.set(this, NiPosData::m_ucKeySize, size);
        e.set(this, NiPosData::m_uiNumKeys, count);
        e.set(this, NiPosData::m_eType, key_type);
        return;
    }
    e.set(this, NiPosData::m_uiNumKeys, 0);
    e.set(this, NiPosData::m_pkKeys, Ptr::new(0));
    e.set(this, NiPosData::m_eType, 0);
    e.set(this, NiPosData::m_ucKeySize, 0);
}

// Translated from 00a49240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPosData::LoadBinary` (Xbox PDB name in the engine map):
/// `NiObject::LoadBinary`, then the key count (read into the stream
/// argument's own stack word) and, for a non-zero count, the key type, the
/// type's key size (+0x14), new keys from the type's creator, loaded by the
/// type's loader, handed to `ReplaceAnim`.
pub fn ni_pos_data_load_binary(e: &mut Engine, this: Ptr<NiPosData>, stream: Ptr) {
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    // Locals: the byte-count word (+0), the key type (+4), the stream
    // argument's stack word that receives the key count (+8).
    e.with_stack(0xc, |e, locals| {
        let size_word = locals.addr();
        let type_word = size_word + 4;
        let count_word = size_word + 8;
        read_word(e, stream, count_word, size_word);
        let count = e.mem.u32(count_word);
        if count == 0 {
            return;
        }
        read_word(e, stream, type_word, size_word);
        let key_type = e.mem.u32(type_word);
        let size = e.global::<u8>(KEY_SIZES.wrapping_add(key_type));
        e.set(this, NiPosData::m_ucKeySize, size);
        let creator = e.global::<u32>(KEY_CREATORS.wrapping_add(key_type.wrapping_mul(4)));
        let keys = e.call(creator, &args![stream, count]).u32();
        let size = e.get(this, NiPosData::m_ucKeySize) as u32;
        let loader = e.global::<u32>(KEY_LOADERS.wrapping_add(key_type.wrapping_mul(4)));
        e.call(loader, &args![keys, count, size]);
        ni_pos_data_replace_anim(e, this, keys, count, key_type);
    });
}

// Translated from 00a48fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPosData` destructor: stores the
/// vtable, deletes the keys and ends in the `NiObject` destructor (a tail
/// jump; its `EAX` is returned).
pub fn fn_00a48fb0(e: &mut Engine, this: Ptr<NiPosData>) -> u32 {
    e.mem.set_u32(this.addr(), NI_POS_DATA_VTABLE);
    delete_keys(e, this);
    e.call(NI_OBJECT_DESTRUCT, &args![this]).u32()
}

/// Per key type: the function that compares two position keys (`011f3af8`, `cdecl` (key,
/// other key), a bool) and the one that saves the keys (`011f3a68`, `cdecl` (stream, keys,
/// count)).
const KEY_COMPARERS: u32 = 0x011f_3af8;
const KEY_SAVERS: u32 = 0x011f_3a68;
/// `NiAnimationKey::GuaranteeKeyAtStartAndEnd` (`00a3d930`, `cdecl`, (key content, key
/// type, &keys, &count, start, end)) and `NiAnimationKey::CopySequence` (`00a3d630`,
/// `cdecl`, (key content, key type, keys, count, start, end, &new keys, &new count)).
const GUARANTEE_KEY_AT_START_AND_END: u32 = 0x00a3_d930;
const COPY_SEQUENCE: u32 = 0x00a3_d630;
/// `NiObject::IsEqual` (`00a5d470`; a bool), `CreateDeepCopy` (`00a5d510`, the address of
/// the copy's smart pointer) and `GetViewerStrings` (`00a5d6e0`), and the base `SaveBinary`
/// `00a89af0`: `thiscall`, one word.
const NI_OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const NI_OBJECT_CREATE_DEEP_COPY: u32 = 0x00a5_d510;
const NI_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
const NI_OBJECT_SAVE_BINARY: u32 = 0x00a8_9af0;
/// `InterlockedIncrement` and `InterlockedDecrement` (import slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `NiGetViewerString` (`cdecl`, the text) and its count overload (`00a7d510`, `cdecl`,
/// (name, count)); the viewer-string array's `SetSize` (`0096ad30`) and `SetAt`
/// (`0096ae90`, (index, address of the string)); `"m_uiNumKeys"`.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_COUNT: u32 = 0x00a7_d510;
const STRINGS_SET_SIZE: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
const NUM_KEYS_NAME_TEXT: u32 = 0x0109_7164;
/// `NiPosData::ms_RTTI` (the class's `NiRTTI` record; its first word is the class name).
const NI_POS_DATA_RTTI: u32 = 0x011f_3f68;

/// Releases a counted object (count at +4; `DeleteThis`, slot +4, at zero).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

/// A 4-byte write to the stream's output (`NiStream::m_pkOstr` +0x250, `m_pfnWrite` +0xC):
/// the byte-count word at `count` set to 4 first, the bytes written added to the position.
fn write_word(e: &mut Engine, stream: u32, source: u32, count: u32) {
    let output = e.mem.u32(stream.wrapping_add(0x250));
    let write = e.mem.u32(output.wrapping_add(0xc));
    e.mem.set_u32(count, 4);
    let done = e
        .call(write, &args![output, source, 4u32, count, 1u32])
        .u32();
    let position = e.mem.u32(output.wrapping_add(4)).wrapping_add(done);
    e.mem.set_u32(output.wrapping_add(4), position);
}

/// The inlined `Add` of the viewer-string array: `text` into the stack word at `slot`; a
/// full array grown to size + step first; then `SetAt` at the old size with the word's
/// address.
fn add_viewer_string(e: &mut Engine, strings: u32, slot: u32, text: u32) {
    let size = u32::from(e.mem.u16(strings + 0xa));
    e.mem.set_u32(slot, text);
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_SET_SIZE, &args![strings, step + size]);
    }
    e.call(STRINGS_SET_AT, &args![strings, size, slot]);
}

// Translated from 00a48f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPosData::GuaranteeKeysAtStartAndEnd` (Xbox PDB; `thiscall`, (start, end), `RET 8`):
/// `NiAnimationKey::GuaranteeKeyAtStartAndEnd` (key content 1, this data's type) on a local
/// copy of the keys pointer and on the key count in place; the local then becomes the keys.
pub fn ni_pos_data_guarantee_keys_at_start_and_end(
    e: &mut Engine,
    this: Ptr<NiPosData>,
    start: f32,
    end: f32,
) {
    e.with_stack(4, |e, local| {
        let keys = e.get(this, NiPosData::m_pkKeys).addr();
        e.mem.set_u32(local.addr(), keys);
        let key_type = e.get(this, NiPosData::m_eType);
        e.call(
            GUARANTEE_KEY_AT_START_AND_END,
            &args![
                1u32,
                key_type,
                local,
                this.addr() + 8,
                x87_quiet_bits(start.to_bits()),
                x87_quiet_bits(end.to_bits())
            ],
        );
        let keys = e.mem.u32(local.addr());
        e.set(this, NiPosData::m_pkKeys, Ptr::new(keys));
    });
}

// Translated from 00a48fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, (keys, count, key type), `RET 0xC`; by its body
/// `ReplaceAnim` without deleting the old keys): with both `keys` and `count` non-zero
/// they are taken with the type and its key size (`011f375e`), otherwise all four fields
/// are cleared.
pub fn fn_00a48fe0(e: &mut Engine, this: Ptr<NiPosData>, keys: u32, count: u32, key_type: u32) {
    if keys != 0 && count != 0 {
        let size = e.global::<u8>(KEY_SIZES.wrapping_add(key_type));
        e.set(this, NiPosData::m_pkKeys, Ptr::new(keys));
        e.set(this, NiPosData::m_uiNumKeys, count);
        e.set(this, NiPosData::m_ucKeySize, size);
        e.set(this, NiPosData::m_eType, key_type);
        return;
    }
    e.set(this, NiPosData::m_uiNumKeys, 0);
    e.set(this, NiPosData::m_pkKeys, Ptr::new(0));
    e.set(this, NiPosData::m_ucKeySize, 0);
    e.set(this, NiPosData::m_eType, 0);
}

// Translated from 00a49060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPosData::IsEqual` (Xbox PDB; `thiscall`, the other object): `NiObject::IsEqual`, the
/// key counts, types and sizes equal, then each pair of keys equal by the key type's compare
/// function (`011f3af8`).
pub fn ni_pos_data_is_equal(e: &mut Engine, this: Ptr<NiPosData>, other: Ptr<NiPosData>) -> bool {
    if !e.call(NI_OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if e.get(this, NiPosData::m_uiNumKeys) != e.get(other, NiPosData::m_uiNumKeys)
        || e.get(this, NiPosData::m_eType) != e.get(other, NiPosData::m_eType)
        || e.get(this, NiPosData::m_ucKeySize) != e.get(other, NiPosData::m_ucKeySize)
    {
        return false;
    }
    let key_type = e.get(this, NiPosData::m_eType);
    let compare = e.global::<u32>(KEY_COMPARERS.wrapping_add(key_type.wrapping_mul(4)));
    let mut index = 0u32;
    while index < e.get(this, NiPosData::m_uiNumKeys) {
        let offset = u32::from(e.get(this, NiPosData::m_ucKeySize)).wrapping_mul(index);
        let theirs = e
            .get(other, NiPosData::m_pkKeys)
            .addr()
            .wrapping_add(offset);
        let mine = e.get(this, NiPosData::m_pkKeys).addr().wrapping_add(offset);
        if !e.call(compare, &args![mine, theirs]).bool() {
            return false;
        }
        index += 1;
    }
    true
}

// Translated from 00a49140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPosData::GetSequenceData` (Xbox PDB; `thiscall`, (start, end, address of the result's
/// smart pointer), `RET 0xC`): a deep copy (`NiObject::CreateDeepCopy` into a local smart
/// pointer, then held in a counted temporary); with keys, `NiAnimationKey::CopySequence`
/// (key content 1) of this data's keys between the times gives the copy's keys
/// (`ReplaceAnim` run on the copy, null or not); the copy is assigned (counted) to the
/// result's pointer, then the temporary and the local are released.
pub fn ni_pos_data_get_sequence_data(
    e: &mut Engine,
    this: Ptr<NiPosData>,
    start: f32,
    end: f32,
    result: Ptr,
) {
    // Locals: the deep copy's smart pointer, the new count, the new keys.
    e.with_stack(0xc, |e, locals| {
        let copy_slot = locals.addr();
        let count_slot = copy_slot + 4;
        let keys_slot = copy_slot + 8;
        e.mem.set_u32(copy_slot, 0);
        e.call(NI_OBJECT_CREATE_DEEP_COPY, &args![this, copy_slot]);
        let copy = e.mem.u32(copy_slot);
        if copy != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![copy.wrapping_add(4)]);
        }
        let count = e.get(this, NiPosData::m_uiNumKeys);
        if count != 0 {
            e.mem.set_u32(keys_slot, 0);
            e.mem.set_u32(count_slot, 0);
            let keys = e.get(this, NiPosData::m_pkKeys).addr();
            let key_type = e.get(this, NiPosData::m_eType);
            e.call(
                COPY_SEQUENCE,
                &args![
                    1u32,
                    key_type,
                    keys,
                    count,
                    x87_quiet_bits(start.to_bits()),
                    x87_quiet_bits(end.to_bits()),
                    keys_slot,
                    count_slot
                ],
            );
            let key_type = e.get(this, NiPosData::m_eType);
            let new_count = e.mem.u32(count_slot);
            let new_keys = e.mem.u32(keys_slot);
            ni_pos_data_replace_anim(e, Ptr::new(copy), new_keys, new_count, key_type);
        }
        let old = e.mem.u32(result.addr());
        if old != copy {
            if old != 0 {
                release(e, old);
            }
            e.mem.set_u32(result.addr(), copy);
            if copy != 0 {
                e.call(INTERLOCKED_INCREMENT, &args![copy.wrapping_add(4)]);
            }
        }
        // The temporary's release (skipped by the exe for a null copy).
        if copy != 0 {
            release(e, copy);
        }
        // The local smart pointer's destructor.
        let local = e.mem.u32(copy_slot);
        if local != 0 {
            release(e, local);
        }
    });
}

// Translated from 00a49300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPosData::SaveBinary` (Xbox PDB; `thiscall`, the stream): the base `SaveBinary`
/// (`00a89af0`), the key count (4 bytes); for a non-zero count the key type (4 bytes) and
/// the keys by the type's saver (`011f3a68`). The byte-count word is the stream argument's
/// own stack slot.
pub fn ni_pos_data_save_binary(e: &mut Engine, this: Ptr<NiPosData>, stream: u32) {
    e.call(NI_OBJECT_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, count| {
        let count = count.addr();
        write_word(e, stream, this.addr() + 8, count);
        if e.get(this, NiPosData::m_uiNumKeys) == 0 {
            return;
        }
        write_word(e, stream, this.addr() + 0x10, count);
        let keys_count = e.get(this, NiPosData::m_uiNumKeys);
        let keys = e.get(this, NiPosData::m_pkKeys).addr();
        let key_type = e.get(this, NiPosData::m_eType);
        let saver = e.global::<u32>(KEY_SAVERS.wrapping_add(key_type.wrapping_mul(4)));
        e.call(saver, &args![stream, keys, keys_count]);
    });
}

// Translated from 00a49380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPosData::GetViewerStrings` (Xbox PDB; `thiscall`, the viewer-string array):
/// `NiObject::GetViewerStrings`, then the class name (the first word of the `NiRTTI` record
/// at `011f3f68`) and `"m_uiNumKeys"` with the key count appended.
pub fn ni_pos_data_get_viewer_strings(e: &mut Engine, this: Ptr<NiPosData>, strings: u32) {
    e.call(NI_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let class_name = e.global::<u32>(NI_POS_DATA_RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
        add_viewer_string(e, strings, slot, text);
        let count = e.get(this, NiPosData::m_uiNumKeys);
        let text = e
            .call(
                NI_GET_VIEWER_STRING_COUNT,
                &args![NUM_KEYS_NAME_TEXT, count],
            )
            .u32();
        add_viewer_string(e, strings, slot, text);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4_9020, fn_00a49020() -> Ptr<NiPosData>),
        entry!(
            0x00a4_90e0,
            ni_pos_data_replace_anim(Ptr<NiPosData>, u32, u32, u32)
        ),
        entry!(0x00a4_9240, ni_pos_data_load_binary(Ptr<NiPosData>, Ptr)),
        entry!(0x00a4_8fb0, fn_00a48fb0(Ptr<NiPosData>) -> u32),
        entry!(
            0x00a4_8f70,
            ni_pos_data_guarantee_keys_at_start_and_end(Ptr<NiPosData>, f32, f32)
        ),
        entry!(0x00a4_8fe0, fn_00a48fe0(Ptr<NiPosData>, u32, u32, u32)),
        entry!(
            0x00a4_9060,
            ni_pos_data_is_equal(Ptr<NiPosData>, Ptr<NiPosData>) -> bool
        ),
        entry!(
            0x00a4_9140,
            ni_pos_data_get_sequence_data(Ptr<NiPosData>, f32, f32, Ptr)
        ),
        entry!(0x00a4_9300, ni_pos_data_save_binary(Ptr<NiPosData>, u32)),
        entry!(
            0x00a4_9380,
            ni_pos_data_get_viewer_strings(Ptr<NiPosData>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Key tables for type 2: size 0x10, creator 0x7000_0010, loader
    /// 0x7000_0020, deleter 0x7000_0030.
    fn key_tables(e: &mut Engine) {
        e.map(KEY_SIZES, 4);
        e.set_global(KEY_SIZES + 2, 0x10u8);
        for (table, function) in [
            (KEY_CREATORS, 0x7000_0010u32),
            (KEY_LOADERS, 0x7000_0020),
            (KEY_DELETERS, 0x7000_0030),
        ] {
            e.map(table, 0x10);
            e.set_global(table + 8, function);
        }
        e.register(0x7000_0010, |_, a| Ret {
            eax: 0x4400 + a[1],
            ..Ret::default()
        });
        e.register(0x7000_0020, |_, _| Ret::default());
        e.register(0x7000_0030, |_, _| Ret::default());
    }

    #[test]
    fn fn_00a49020_creates_empty_pos_data() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.register(NI_OBJECT_CONSTRUCT, |_, _| Ret::default());
        assert_eq!(e.call(0x00a4_9020, &[]).u32(), 0);
        e.register(NI_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xaa; 0x18]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let data = e.call(0x00a4_9020, &[]).u32();
        assert_eq!(e.mem.u32(data), NI_POS_DATA_VTABLE);
        assert_eq!(e.mem.bytes(data + 8, 0xd), vec![0; 0xd]);
        assert_eq!(e.mem.u8(data + 0x15), 0xaa);
    }

    #[test]
    fn replace_anim_deletes_the_old_keys_and_takes_or_clears() {
        let mut e = Engine::new();
        key_tables(&mut e);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0xc, 0x5500);
        e.mem.set_u32(this + 0x10, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a4_90e0, &args![this, 0x6600u32, 5u32, 2u32]);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .contains(&(0x7000_0030, vec![0x5500])));
        assert_eq!(e.mem.u32(this + 8), 5);
        assert_eq!(e.mem.u32(this + 0xc), 0x6600);
        assert_eq!(e.mem.u32(this + 0x10), 2);
        assert_eq!(e.mem.u8(this + 0x14), 0x10);
        e.call(0x00a4_90e0, &args![this, 0x6600u32, 0u32, 2u32]);
        assert_eq!(e.mem.bytes(this + 8, 0xd), vec![0; 0xd]);
        e.call_log = Some(vec![]);
        e.call(0x00a4_90e0, &args![this, 0u32, 3u32, 2u32]);
        assert!(!e
            .call_log
            .take()
            .unwrap()
            .iter()
            .any(|c| c.0 == 0x7000_0030));
    }

    #[test]
    fn load_binary_reads_count_and_type_then_builds_the_keys() {
        let mut e = Engine::new();
        key_tables(&mut e);
        e.register(NI_OBJECT_LOAD_BINARY, |_, _| Ret::default());
        // The input hands out the words 3 then 2.
        let mut words = vec![2u32, 3];
        e.register_double(0x7000_0008, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_u32(a[1], words.pop().unwrap());
            Ret {
                eax: 4,
                ..Ret::default()
            }
        });
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 8, 0x7000_0008);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, input);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x00a4_9240, &args![this, stream]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x7000_0010, vec![stream, 3])));
        assert!(log.contains(&(0x7000_0020, vec![0x4403, 3, 0x10])));
        assert_eq!(e.mem.u32(this + 8), 3);
        assert_eq!(e.mem.u32(this + 0xc), 0x4403);
        assert_eq!(e.mem.u32(input + 4), 8);
    }

    #[test]
    fn load_binary_stops_after_a_zero_count() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(0x7000_0008, |e, a| {
            e.mem.set_u32(a[1], 0);
            Ret {
                eax: 4,
                ..Ret::default()
            }
        });
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(input + 8, 0x7000_0008);
        let stream = e.mem.alloc(0x250);
        e.mem.set_u32(stream + 0x24c, input);
        let this = e.mem.alloc(0x18);
        e.call(0x00a4_9240, &args![this, stream]);
        assert_eq!(e.mem.u32(input + 4), 4, "one read");
    }

    #[test]
    fn fn_00a48fb0_deletes_the_keys_and_destroys_the_object() {
        let mut e = Engine::new();
        key_tables(&mut e);
        e.register(NI_OBJECT_DESTRUCT, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0xc, 0x5500);
        e.mem.set_u32(this + 0x10, 2);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_8fb0, &args![this]).u32(), this + 1);
        assert_eq!(e.mem.u32(this), NI_POS_DATA_VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x7000_0030, vec![0x5500]));
        assert_eq!(log[2], (NI_OBJECT_DESTRUCT, vec![this]));
    }
}

#[cfg(test)]
mod n005_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The key tables mapped: each type's entry `0x0098_0000 + table offset + 4 * type`, the
    /// key size of type `t` `0x10 + t`.
    fn tables(e: &mut Engine) {
        e.map(0x011f_3000, 0x1000);
        for table in [KEY_COMPARERS, KEY_SAVERS, KEY_DELETERS] {
            for key_type in 0..8u32 {
                let entry = 0x0098_0000 + (table & 0xfff) + 4 * key_type;
                e.mem.set_u32(table + 4 * key_type, entry);
            }
        }
        for key_type in 0..8u32 {
            e.mem.set_u8(KEY_SIZES + key_type, 0x10 + key_type as u8);
        }
    }

    fn function(table: u32, key_type: u32) -> u32 {
        0x0098_0000 + (table & 0xfff) + 4 * key_type
    }

    fn pos_data(e: &mut Engine, count: u32, keys: u32, key_type: u32, size: u8) -> Ptr<NiPosData> {
        let this = e.new_object::<NiPosData>();
        let table = e.mem.alloc(8);
        e.mem.set_u32(table + 4, 0x0099_0004);
        e.mem.set_u32(this.addr(), table);
        e.set(this, NiPosData::m_uiNumKeys, count);
        e.set(this, NiPosData::m_pkKeys, Ptr::new(keys));
        e.set(this, NiPosData::m_eType, key_type);
        e.set(this, NiPosData::m_ucKeySize, size);
        this
    }

    #[test]
    fn guarantee_keys_works_on_a_copy_of_the_keys_pointer() {
        let mut e = Engine::new();
        e.register(GUARANTEE_KEY_AT_START_AND_END, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0x4000);
            e.mem.set_u32(a[2], 0x5000);
            let count = e.mem.u32(a[3]) + 1;
            e.mem.set_u32(a[3], count);
            Ret::default()
        });
        let this = pos_data(&mut e, 2, 0x4000, 3, 0x13);
        e.call_log = Some(vec![]);
        e.call(0x00a4_8f70, &[this.addr(), 1.0f32.to_bits(), 0x7f80_0001]);
        let call = &calls_to(&e, GUARANTEE_KEY_AT_START_AND_END)[0];
        assert_eq!(call[..2], [1, 3]);
        assert_eq!(call[3..], [this.addr() + 8, 1.0f32.to_bits(), 0x7fc0_0001]);
        assert_eq!(e.get(this, NiPosData::m_pkKeys).addr(), 0x5000);
        assert_eq!(e.get(this, NiPosData::m_uiNumKeys), 3);
    }

    #[test]
    fn set_anim_00a48fe0_takes_keys_without_deleting() {
        let mut e = Engine::new();
        tables(&mut e);
        let this = pos_data(&mut e, 1, 0x4000, 1, 0x11);
        e.call_log = Some(vec![]);
        e.call(0x00a4_8fe0, &[this.addr(), 0x5000, 4, 2]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.get(this, NiPosData::m_pkKeys).addr(), 0x5000);
        assert_eq!(e.get(this, NiPosData::m_uiNumKeys), 4);
        assert_eq!(e.get(this, NiPosData::m_eType), 2);
        assert_eq!(e.get(this, NiPosData::m_ucKeySize), 0x12);
        e.call(0x00a4_8fe0, &[this.addr(), 0, 4, 2]);
        assert_eq!(e.get(this, NiPosData::m_pkKeys).addr(), 0);
        assert_eq!(e.get(this, NiPosData::m_uiNumKeys), 0);
        assert_eq!(e.get(this, NiPosData::m_eType), 0);
        assert_eq!(e.get(this, NiPosData::m_ucKeySize), 0);
    }

    #[test]
    fn is_equal_compares_header_then_keys() {
        let mut e = Engine::new();
        tables(&mut e);
        e.register(NI_OBJECT_IS_EQUAL, |_, _| true.into_ret());
        e.register(function(KEY_COMPARERS, 1), |e, a| {
            (e.mem.u32(a[0]) == e.mem.u32(a[1])).into_ret()
        });
        let keys_a = e.mem.alloc(0x20);
        let keys_b = e.mem.alloc(0x20);
        e.mem.set_u32(keys_a + 0x10, 5);
        e.mem.set_u32(keys_b + 0x10, 5);
        let a = pos_data(&mut e, 2, keys_a, 1, 0x10);
        let b = pos_data(&mut e, 2, keys_b, 1, 0x10);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a4_9060, &[a.addr(), b.addr()]).bool());
        assert_eq!(
            calls_to(&e, function(KEY_COMPARERS, 1)),
            vec![vec![keys_a, keys_b], vec![keys_a + 0x10, keys_b + 0x10]]
        );
        e.mem.set_u32(keys_b + 0x10, 6);
        assert!(!e.call(0x00a4_9060, &[a.addr(), b.addr()]).bool());
        for (count, key_type, size) in [(3u32, 1u32, 0x10u8), (2, 2, 0x10), (2, 1, 0x14)] {
            let c = pos_data(&mut e, count, keys_a, key_type, size);
            assert!(!e.call(0x00a4_9060, &[a.addr(), c.addr()]).bool());
        }
        e.register(NI_OBJECT_IS_EQUAL, |_, _| false.into_ret());
        assert!(!e.call(0x00a4_9060, &[a.addr(), a.addr()]).bool());
    }

    fn counting(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]) - 1;
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(0x0099_0004, |_, _| Ret::default());
    }

    #[test]
    fn sequence_data_copies_the_keys_between_the_times() {
        let mut e = Engine::new();
        tables(&mut e);
        counting(&mut e);
        let copy = pos_data(&mut e, 0, 0, 0, 0);
        e.register_double(NI_OBJECT_CREATE_DEEP_COPY, move |e, a| {
            e.mem.set_u32(a[1], copy.addr());
            let count = e.mem.u32(copy.addr() + 4) + 1;
            e.mem.set_u32(copy.addr() + 4, count);
            Ret::default()
        });
        e.register(COPY_SEQUENCE, |e, a| {
            e.mem.set_u32(a[6], 0x7000);
            e.mem.set_u32(a[7], 3);
            Ret::default()
        });
        let this = pos_data(&mut e, 5, 0x6000, 2, 0x12);
        let result = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(
            0x00a4_9140,
            &[this.addr(), 1.0f32.to_bits(), 2.0f32.to_bits(), result],
        );
        assert_eq!(
            calls_to(&e, COPY_SEQUENCE)[0][..6],
            [1, 2, 0x6000, 5, 1.0f32.to_bits(), 2.0f32.to_bits()]
        );
        assert_eq!(e.mem.u32(result), copy.addr());
        assert_eq!(e.get(copy, NiPosData::m_pkKeys).addr(), 0x7000);
        assert_eq!(e.get(copy, NiPosData::m_uiNumKeys), 3);
        assert_eq!(e.get(copy, NiPosData::m_ucKeySize), 0x12);
        assert_eq!(e.mem.u32(copy.addr() + 4), 1);
        assert!(calls_to(&e, 0x0099_0004).is_empty());
    }

    #[test]
    fn save_binary_writes_count_type_and_keys() {
        let mut e = Engine::new();
        tables(&mut e);
        e.register(NI_OBJECT_SAVE_BINARY, |_, _| Ret::default());
        e.register(function(KEY_SAVERS, 6), |_, _| Ret::default());
        e.register(0x0300_0004, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            a[2].into_ret()
        });
        let stream = e.mem.alloc(0x300);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + 0xc, 0x0300_0004);
        e.mem.set_u32(stream + 0x250, output);
        let this = pos_data(&mut e, 2, 0x7000, 6, 0x16);
        e.call_log = Some(vec![]);
        e.call(0x00a4_9300, &[this.addr(), stream]);
        let writes: Vec<u32> = calls_to(&e, 0x0300_0004).iter().map(|a| a[1]).collect();
        assert_eq!(writes, [this.addr() + 8, this.addr() + 0x10]);
        assert_eq!(
            calls_to(&e, function(KEY_SAVERS, 6)),
            vec![vec![stream, 0x7000, 2]]
        );
        assert_eq!(e.mem.u32(output + 4), 8);
        let empty = pos_data(&mut e, 0, 0, 6, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_9300, &[empty.addr(), stream]);
        assert_eq!(calls_to(&e, 0x0300_0004).len(), 1);
    }

    #[test]
    fn get_viewer_strings_adds_class_and_key_count() {
        let mut e = Engine::new();
        e.map(NI_POS_DATA_RTTI & !0xfff, 0x1000);
        e.set_global(NI_POS_DATA_RTTI, 0x0444_0000u32);
        e.register(NI_OBJECT_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| 0x0600_0001u32.into_ret());
        e.register(NI_GET_VIEWER_STRING_COUNT, |_, _| 0x0600_0002u32.into_ret());
        e.register(STRINGS_SET_SIZE, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let text = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], text);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x10);
        e.mem.set_u32(strings + 4, base);
        e.mem.set_u16(strings + 8, 1);
        e.mem.set_u16(strings + 0xe, 1);
        let this = pos_data(&mut e, 9, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_9380, &[this.addr(), strings]);
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_COUNT),
            vec![vec![NUM_KEYS_NAME_TEXT, 9]]
        );
        assert_eq!(e.mem.u32(base), 0x0600_0001);
        assert_eq!(e.mem.u32(base + 4), 0x0600_0002);
        // Full after the first: grown by the step (1 + 1).
        assert_eq!(calls_to(&e, STRINGS_SET_SIZE), vec![vec![strings, 2]]);
    }
}
