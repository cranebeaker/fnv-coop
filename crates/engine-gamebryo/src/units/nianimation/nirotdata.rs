//! `NiAnimation/nirotdata.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiRotData::ms_RTTI` (the class's `NiRTTI` record).
const NI_ROT_DATA_RTTI: u32 = 0x011f_3f50;

// Translated from 00a48150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRotData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_rot_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_ROT_DATA_RTTI)
}

layout! {
    /// `NiRotData` (Xbox PDB), 0x18 bytes on PC (the same).
    pub struct NiRotData: 0x18 {
        /// `m_uiNumKeys` (Xbox PDB).
        0x08 m_uiNumKeys: u32,
        /// `m_pkKeys` (Xbox PDB): `NiRotKey*`.
        0x0C m_pkKeys: u32,
        /// `m_eType` (Xbox PDB): `NiAnimationKey::KeyType`.
        0x10 m_eType: u32,
        /// `m_ucKeySize` (Xbox PDB).
        0x14 m_ucKeySize: u8,
    }
}

/// `NiRotData`'s vtable.
const NI_ROT_DATA_VTABLE: u32 = 0x0109_84ec;
/// The rotation keys' tables, one entry per key type: the key size in bytes (`011f3764`, one
/// byte each) and the functions that delete an array of keys (`011f38d0`, `cdecl`, keys),
/// compare two keys (`011f3b10`, `cdecl` (key, other key), a bool), create `count` keys
/// from a stream (`011f3de0`, `cdecl` (stream, count)), load them (`011f37b0`, `cdecl`
/// (keys, count, key size)) and save them (`011f3a80`, `cdecl` (stream, keys, count)).
const KEY_SIZES: u32 = 0x011f_3764;
const KEY_DELETERS: u32 = 0x011f_38d0;
const KEY_COMPARERS: u32 = 0x011f_3b10;
const KEY_CREATORS: u32 = 0x011f_3de0;
const KEY_LOADERS: u32 = 0x011f_37b0;
const KEY_SAVERS: u32 = 0x011f_3a80;
/// `NiObject`'s destructor body (`00a5d3d0`), `IsEqual` (`00a5d470`; a bool),
/// `CreateDeepCopy` (`00a5d510`, the address of the copy's smart pointer), `LoadBinary`
/// (`00a5d610`) and `GetViewerStrings` (`00a5d6e0`), and the base `SaveBinary` `00a89af0`:
/// `thiscall`.
const OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
const OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const OBJECT_CREATE_DEEP_COPY: u32 = 0x00a5_d510;
const OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
const OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
const OBJECT_SAVE_BINARY: u32 = 0x00a8_9af0;
/// The sized delete (`00aa1460`, `cdecl`, (block, size)).
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiAnimationKey::CopySequence` (`00a3d630`, `cdecl`, (key content, key type, keys,
/// count, start, end, &new keys, &new count)).
const COPY_SEQUENCE: u32 = 0x00a3_d630;
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

/// Releases a counted object (count at +4; `DeleteThis`, slot +4, at zero).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

/// The entry of a per-key-type table of functions.
fn key_function(e: &Engine, table: u32, key_type: u32) -> u32 {
    e.mem.u32(table.wrapping_add(key_type.wrapping_mul(4)))
}

/// One 4-byte transfer through the stream's input (`NiStream::m_pkIstr` +0x24C, `m_pfnRead`
/// +0x8) or output (`m_pkOstr` +0x250, `m_pfnWrite` +0xC): the component size word at
/// `size_word` set to 4, the bytes moved added to the file's position.
fn transfer_word(
    e: &mut Engine,
    (file_field, function_field): (u32, u32),
    stream: u32,
    buffer: u32,
    size_word: u32,
) {
    let file = e.mem.u32(stream.wrapping_add(file_field));
    let function = e.mem.u32(file.wrapping_add(function_field));
    e.mem.set_u32(size_word, 4);
    let done = e
        .call(function, &args![file, buffer, 4u32, size_word, 1u32])
        .u32();
    let position = e.mem.u32(file.wrapping_add(4));
    e.mem
        .set_u32(file.wrapping_add(4), position.wrapping_add(done));
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

// Translated from 00a48160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRotData::IsEqual` (Xbox PDB; `thiscall`, the other object): `NiObject::IsEqual`, the
/// key counts, types and sizes equal, then each pair of keys equal by the key type's
/// compare function (`011f3b10`).
pub fn ni_rot_data_is_equal(e: &mut Engine, this: Ptr<NiRotData>, other: Ptr<NiRotData>) -> bool {
    if !e.call(OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if e.get(this, NiRotData::m_uiNumKeys) != e.get(other, NiRotData::m_uiNumKeys)
        || e.get(this, NiRotData::m_eType) != e.get(other, NiRotData::m_eType)
        || e.get(this, NiRotData::m_ucKeySize) != e.get(other, NiRotData::m_ucKeySize)
    {
        return false;
    }
    let compare = key_function(e, KEY_COMPARERS, e.get(this, NiRotData::m_eType));
    let mut index = 0u32;
    while index < e.get(this, NiRotData::m_uiNumKeys) {
        let offset = u32::from(e.get(this, NiRotData::m_ucKeySize)).wrapping_mul(index);
        let theirs = e.get(other, NiRotData::m_pkKeys).wrapping_add(offset);
        let mine = e.get(this, NiRotData::m_pkKeys).wrapping_add(offset);
        if !e.call(compare, &args![mine, theirs]).bool() {
            return false;
        }
        index += 1;
    }
    true
}

// Translated from 00a481e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRotData::_scalar_deleting_destructor_` (Xbox PDB; `thiscall`, the flags): the vtable,
/// the keys deleted by their type's deleter (`011f38d0`) when present, the `NiObject`
/// destructor body, and with bit 0 of `flags` the sized delete of 0x18 bytes. Answers
/// `this`.
pub fn ni_rot_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiRotData>,
    flags: u32,
) -> Ptr<NiRotData> {
    let keys = e.get(this, NiRotData::m_pkKeys);
    e.mem.set_u32(this.addr(), NI_ROT_DATA_VTABLE);
    if keys != 0 {
        let deleter = key_function(e, KEY_DELETERS, e.get(this, NiRotData::m_eType));
        e.call(deleter, &args![keys]);
    }
    e.call(OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x18u32]);
    }
    this
}

// Translated from 00a48220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRotData::ReplaceAnim` (Xbox PDB; `thiscall`, (keys, count, key type), `RET 0xC`): the
/// old keys deleted by their type's deleter (`011f38d0`); with both `keys` and `count`
/// non-zero they are taken with the type and its key size (`011f3764`), otherwise all four
/// fields are cleared.
pub fn ni_rot_data_replace_anim(
    e: &mut Engine,
    this: Ptr<NiRotData>,
    keys: u32,
    count: u32,
    key_type: u32,
) {
    let old = e.get(this, NiRotData::m_pkKeys);
    if old != 0 {
        let deleter = key_function(e, KEY_DELETERS, e.get(this, NiRotData::m_eType));
        e.call(deleter, &args![old]);
    }
    if keys != 0 && count != 0 {
        let size = e.mem.u8(KEY_SIZES.wrapping_add(key_type));
        e.set(this, NiRotData::m_pkKeys, keys);
        e.set(this, NiRotData::m_ucKeySize, size);
        e.set(this, NiRotData::m_uiNumKeys, count);
        e.set(this, NiRotData::m_eType, key_type);
        return;
    }
    e.set(this, NiRotData::m_uiNumKeys, 0);
    e.set(this, NiRotData::m_pkKeys, 0);
    e.set(this, NiRotData::m_eType, 0);
    e.set(this, NiRotData::m_ucKeySize, 0);
}

// Translated from 00a48280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRotData::GetSequenceData` (Xbox PDB; `thiscall`, (start, end, address of the result's
/// smart pointer), `RET 0xC`): a deep copy (`NiObject::CreateDeepCopy` into a local smart
/// pointer, then held in a counted temporary); with keys, `NiAnimationKey::CopySequence`
/// (key content 2) of this data's keys between the times gives the copy's keys
/// (`ReplaceAnim` run on the copy, null or not); the copy is assigned (counted) to the
/// result's pointer, then the temporary and the local are released.
pub fn ni_rot_data_get_sequence_data(
    e: &mut Engine,
    this: Ptr<NiRotData>,
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
        e.call(OBJECT_CREATE_DEEP_COPY, &args![this, copy_slot]);
        let copy = e.mem.u32(copy_slot);
        if copy != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![copy.wrapping_add(4)]);
        }
        let count = e.get(this, NiRotData::m_uiNumKeys);
        if count != 0 {
            e.mem.set_u32(keys_slot, 0);
            e.mem.set_u32(count_slot, 0);
            let keys = e.get(this, NiRotData::m_pkKeys);
            let key_type = e.get(this, NiRotData::m_eType);
            e.call(
                COPY_SEQUENCE,
                &args![
                    2u32,
                    key_type,
                    keys,
                    count,
                    x87_quiet_bits(start.to_bits()),
                    x87_quiet_bits(end.to_bits()),
                    keys_slot,
                    count_slot
                ],
            );
            let key_type = e.get(this, NiRotData::m_eType);
            let new_count = e.mem.u32(count_slot);
            let new_keys = e.mem.u32(keys_slot);
            ni_rot_data_replace_anim(e, Ptr::new(copy), new_keys, new_count, key_type);
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

// Translated from 00a48380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRotData::LoadBinary` (Xbox PDB; `thiscall`, the stream): `NiObject::LoadBinary`, the
/// key count (4 bytes, into the stream argument's own stack slot); for a non-zero count the
/// key type (4 bytes, a local), its key size (`011f3764`) stored, keys made by the type's
/// creator (`011f3de0`), loaded by its loader (`011f37b0`) and handed to `ReplaceAnim`.
pub fn ni_rot_data_load_binary(e: &mut Engine, this: Ptr<NiRotData>, stream: u32) {
    e.call(OBJECT_LOAD_BINARY, &args![this, stream]);
    // Locals: the component size word, the key type, the stream argument's slot.
    e.with_stack(0xc, |e, locals| {
        let size_word = locals.addr();
        let type_slot = size_word + 4;
        let count_slot = size_word + 8;
        transfer_word(e, (0x24c, 8), stream, count_slot, size_word);
        if e.mem.u32(count_slot) == 0 {
            return;
        }
        transfer_word(e, (0x24c, 8), stream, type_slot, size_word);
        let key_type = e.mem.u32(type_slot);
        let size = e.mem.u8(KEY_SIZES.wrapping_add(key_type));
        let count = e.mem.u32(count_slot);
        e.set(this, NiRotData::m_ucKeySize, size);
        let creator = key_function(e, KEY_CREATORS, key_type);
        let keys = e.call(creator, &args![stream, count]).u32();
        let size = u32::from(e.get(this, NiRotData::m_ucKeySize));
        let loader = key_function(e, KEY_LOADERS, key_type);
        let count = e.mem.u32(count_slot);
        e.call(loader, &args![keys, count, size]);
        let count = e.mem.u32(count_slot);
        ni_rot_data_replace_anim(e, this, keys, count, key_type);
    });
}

// Translated from 00a48440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRotData::SaveBinary` (Xbox PDB; `thiscall`, the stream): the base `SaveBinary`
/// (`00a89af0`), the key count (4 bytes); for a non-zero count the key type (4 bytes) and
/// the keys by the type's saver (`011f3a80`). The component size word is the stream
/// argument's own stack slot.
pub fn ni_rot_data_save_binary(e: &mut Engine, this: Ptr<NiRotData>, stream: u32) {
    e.call(OBJECT_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, size_word| {
        let size_word = size_word.addr();
        transfer_word(e, (0x250, 0xc), stream, this.addr() + 8, size_word);
        if e.get(this, NiRotData::m_uiNumKeys) == 0 {
            return;
        }
        transfer_word(e, (0x250, 0xc), stream, this.addr() + 0x10, size_word);
        let count = e.get(this, NiRotData::m_uiNumKeys);
        let keys = e.get(this, NiRotData::m_pkKeys);
        let saver = key_function(e, KEY_SAVERS, e.get(this, NiRotData::m_eType));
        e.call(saver, &args![stream, keys, count]);
    });
}

// Translated from 00a484c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRotData::GetViewerStrings` (Xbox PDB; `thiscall`, the viewer-string array):
/// `NiObject::GetViewerStrings`, then the class name (the first word of the `NiRTTI` record
/// at `011f3f50`) and `"m_uiNumKeys"` with the key count appended.
pub fn ni_rot_data_get_viewer_strings(e: &mut Engine, this: Ptr<NiRotData>, strings: u32) {
    e.call(OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let class_name = e.global::<u32>(NI_ROT_DATA_RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
        add_viewer_string(e, strings, slot, text);
        let count = e.get(this, NiRotData::m_uiNumKeys);
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
        entry!(0x00a48150, ni_rot_data_get_rtti() -> Ptr),
        entry!(
            0x00a4_8160,
            ni_rot_data_is_equal(Ptr<NiRotData>, Ptr<NiRotData>) -> bool
        ),
        entry!(
            0x00a4_81e0,
            ni_rot_data_scalar_deleting_destructor(Ptr<NiRotData>, u32) -> Ptr<NiRotData>
        ),
        entry!(
            0x00a4_8220,
            ni_rot_data_replace_anim(Ptr<NiRotData>, u32, u32, u32)
        ),
        entry!(
            0x00a4_8280,
            ni_rot_data_get_sequence_data(Ptr<NiRotData>, f32, f32, Ptr)
        ),
        entry!(0x00a4_8380, ni_rot_data_load_binary(Ptr<NiRotData>, u32)),
        entry!(0x00a4_8440, ni_rot_data_save_binary(Ptr<NiRotData>, u32)),
        entry!(
            0x00a4_84c0,
            ni_rot_data_get_viewer_strings(Ptr<NiRotData>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_rot_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a48150, &args![]).u32(), NI_ROT_DATA_RTTI);
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

    /// The key tables mapped, each type's entry `0x0098_0000 + table offset + 4 * type`, and
    /// the key size of type `t` `0x10 + t`.
    fn tables(e: &mut Engine) {
        e.map(0x011f_3000, 0x1000);
        for table in [
            KEY_DELETERS,
            KEY_COMPARERS,
            KEY_CREATORS,
            KEY_LOADERS,
            KEY_SAVERS,
        ] {
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

    fn rot_data(e: &mut Engine, count: u32, keys: u32, key_type: u32, size: u8) -> Ptr<NiRotData> {
        let this = e.new_object::<NiRotData>();
        let table = e.mem.alloc(8);
        e.mem.set_u32(table + 4, 0x0099_0004);
        e.mem.set_u32(this.addr(), table);
        e.set(this, NiRotData::m_uiNumKeys, count);
        e.set(this, NiRotData::m_pkKeys, keys);
        e.set(this, NiRotData::m_eType, key_type);
        e.set(this, NiRotData::m_ucKeySize, size);
        this
    }

    #[test]
    fn is_equal_compares_header_then_keys() {
        let mut e = Engine::new();
        tables(&mut e);
        e.register(OBJECT_IS_EQUAL, |_, _| true.into_ret());
        // Keys are equal when their first words are.
        e.register(function(KEY_COMPARERS, 2), |e, a| {
            (e.mem.u32(a[0]) == e.mem.u32(a[1])).into_ret()
        });
        let keys_a = e.mem.alloc(0x20);
        let keys_b = e.mem.alloc(0x20);
        for (i, v) in [1u32, 2].iter().enumerate() {
            e.mem.set_u32(keys_a + 0x10 * i as u32, *v);
            e.mem.set_u32(keys_b + 0x10 * i as u32, *v);
        }
        let a = rot_data(&mut e, 2, keys_a, 2, 0x10);
        let b = rot_data(&mut e, 2, keys_b, 2, 0x10);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a4_8160, &[a.addr(), b.addr()]).bool());
        assert_eq!(
            calls_to(&e, function(KEY_COMPARERS, 2)),
            vec![vec![keys_a, keys_b], vec![keys_a + 0x10, keys_b + 0x10]]
        );
        e.mem.set_u32(keys_b + 0x10, 3);
        assert!(!e.call(0x00a4_8160, &[a.addr(), b.addr()]).bool());
        for (count, key_type, size) in [(1u32, 2u32, 0x10u8), (2, 3, 0x10), (2, 2, 0x11)] {
            let c = rot_data(&mut e, count, keys_a, key_type, size);
            assert!(!e.call(0x00a4_8160, &[a.addr(), c.addr()]).bool());
        }
        e.register(OBJECT_IS_EQUAL, |_, _| false.into_ret());
        assert!(!e.call(0x00a4_8160, &[a.addr(), a.addr()]).bool());
    }

    #[test]
    fn destructor_deletes_the_keys_and_on_bit_0_the_object() {
        let mut e = Engine::new();
        tables(&mut e);
        e.register(function(KEY_DELETERS, 3), |_, _| Ret::default());
        e.register(OBJECT_DESTRUCT, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let this = rot_data(&mut e, 1, 0x4000, 3, 0x13);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a4_81e0, &[this.addr(), 1]).u32(), this.addr());
        assert_eq!(calls_to(&e, function(KEY_DELETERS, 3)), vec![vec![0x4000]]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this.addr(), 0x18]]);
        assert_eq!(e.mem.u32(this.addr()), NI_ROT_DATA_VTABLE);
        e.set(this, NiRotData::m_pkKeys, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_81e0, &[this.addr(), 0]);
        assert!(calls_to(&e, function(KEY_DELETERS, 3)).is_empty());
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
    }

    #[test]
    fn replace_anim_takes_keys_or_clears() {
        let mut e = Engine::new();
        tables(&mut e);
        e.register(function(KEY_DELETERS, 1), |_, _| Ret::default());
        let this = rot_data(&mut e, 1, 0x4000, 1, 0x11);
        e.call_log = Some(vec![]);
        e.call(0x00a4_8220, &[this.addr(), 0x5000, 4, 5]);
        assert_eq!(calls_to(&e, function(KEY_DELETERS, 1)), vec![vec![0x4000]]);
        assert_eq!(e.get(this, NiRotData::m_pkKeys), 0x5000);
        assert_eq!(e.get(this, NiRotData::m_uiNumKeys), 4);
        assert_eq!(e.get(this, NiRotData::m_eType), 5);
        assert_eq!(e.get(this, NiRotData::m_ucKeySize), 0x15);
        e.register(function(KEY_DELETERS, 5), |_, _| Ret::default());
        e.call(0x00a4_8220, &[this.addr(), 0x5000, 0, 5]);
        assert_eq!(e.get(this, NiRotData::m_pkKeys), 0);
        assert_eq!(e.get(this, NiRotData::m_uiNumKeys), 0);
        assert_eq!(e.get(this, NiRotData::m_eType), 0);
        assert_eq!(e.get(this, NiRotData::m_ucKeySize), 0);
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
        e.register(function(KEY_DELETERS, 2), |_, _| Ret::default());
        let copy = rot_data(&mut e, 0, 0, 0, 0);
        e.register_double(OBJECT_CREATE_DEEP_COPY, move |e, a| {
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
        let this = rot_data(&mut e, 5, 0x6000, 2, 0x12);
        let old = rot_data(&mut e, 0, 0, 0, 0);
        e.mem.set_u32(old.addr() + 4, 1);
        let result = e.mem.alloc(4);
        e.mem.set_u32(result, old.addr());
        e.call_log = Some(vec![]);
        e.call(
            0x00a4_8280,
            &[this.addr(), 1.0f32.to_bits(), 2.0f32.to_bits(), result],
        );
        let copied = calls_to(&e, COPY_SEQUENCE);
        assert_eq!(
            copied[0][..6],
            [2, 2, 0x6000, 5, 1.0f32.to_bits(), 2.0f32.to_bits()]
        );
        assert_eq!(e.mem.u32(result), copy.addr());
        assert_eq!(e.get(copy, NiRotData::m_pkKeys), 0x7000);
        assert_eq!(e.get(copy, NiRotData::m_uiNumKeys), 3);
        assert_eq!(e.get(copy, NiRotData::m_ucKeySize), 0x12);
        // The result holds the only reference; the old data was deleted.
        assert_eq!(e.mem.u32(copy.addr() + 4), 1);
        assert_eq!(calls_to(&e, 0x0099_0004), vec![vec![old.addr()]]);
        // No keys: no copy of the sequence, the result already the copy.
        let empty = rot_data(&mut e, 0, 0, 2, 0);
        e.call_log = Some(vec![]);
        e.call(
            0x00a4_8280,
            &[empty.addr(), 1.0f32.to_bits(), 2.0f32.to_bits(), result],
        );
        assert!(calls_to(&e, COPY_SEQUENCE).is_empty());
        assert_eq!(e.mem.u32(copy.addr() + 4), 1);
    }

    /// A stream whose input reads answer `words` and whose output writes are recorded.
    fn test_stream(e: &mut Engine, words: Vec<u32>) -> (u32, u32) {
        let stream = e.mem.alloc(0x300);
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 8, 0x0300_0000);
        e.mem.set_u32(file + 0xc, 0x0300_0004);
        e.mem.set_u32(stream + 0x24c, file);
        e.mem.set_u32(stream + 0x250, file);
        let mut words = words;
        e.register_double(0x0300_0000, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            e.mem.set_u32(a[1], words.remove(0));
            a[2].into_ret()
        });
        e.register(0x0300_0004, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 4);
            a[2].into_ret()
        });
        (stream, file)
    }

    #[test]
    fn load_binary_creates_and_loads_the_keys() {
        let mut e = Engine::new();
        tables(&mut e);
        e.register(OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(function(KEY_CREATORS, 3), |_, _| 0x7000u32.into_ret());
        e.register(function(KEY_LOADERS, 3), |_, _| Ret::default());
        let (stream, file) = test_stream(&mut e, vec![4, 3]);
        let this = rot_data(&mut e, 0, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_8380, &[this.addr(), stream]);
        assert_eq!(
            calls_to(&e, function(KEY_CREATORS, 3)),
            vec![vec![stream, 4]]
        );
        assert_eq!(
            calls_to(&e, function(KEY_LOADERS, 3)),
            vec![vec![0x7000, 4, 0x13]]
        );
        assert_eq!(e.get(this, NiRotData::m_pkKeys), 0x7000);
        assert_eq!(e.get(this, NiRotData::m_uiNumKeys), 4);
        assert_eq!(e.get(this, NiRotData::m_eType), 3);
        assert_eq!(e.get(this, NiRotData::m_ucKeySize), 0x13);
        assert_eq!(e.mem.u32(file + 4), 8);
        // A zero count reads nothing more.
        let (stream, file) = test_stream(&mut e, vec![0]);
        let fresh = rot_data(&mut e, 0, 0, 0, 0);
        e.call(0x00a4_8380, &[fresh.addr(), stream]);
        assert_eq!(e.mem.u32(file + 4), 4);
        assert_eq!(e.get(fresh, NiRotData::m_pkKeys), 0);
    }

    #[test]
    fn save_binary_writes_count_type_and_keys() {
        let mut e = Engine::new();
        tables(&mut e);
        e.register(OBJECT_SAVE_BINARY, |_, _| Ret::default());
        e.register(function(KEY_SAVERS, 6), |_, _| Ret::default());
        let (stream, file) = test_stream(&mut e, vec![]);
        let this = rot_data(&mut e, 2, 0x7000, 6, 0x16);
        e.call_log = Some(vec![]);
        e.call(0x00a4_8440, &[this.addr(), stream]);
        let writes: Vec<u32> = calls_to(&e, 0x0300_0004).iter().map(|a| a[1]).collect();
        assert_eq!(writes, [this.addr() + 8, this.addr() + 0x10]);
        assert_eq!(
            calls_to(&e, function(KEY_SAVERS, 6)),
            vec![vec![stream, 0x7000, 2]]
        );
        assert_eq!(e.mem.u32(file + 4), 8);
        let empty = rot_data(&mut e, 0, 0, 6, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_8440, &[empty.addr(), stream]);
        assert_eq!(calls_to(&e, 0x0300_0004).len(), 1);
        assert!(calls_to(&e, function(KEY_SAVERS, 6)).is_empty());
    }

    #[test]
    fn get_viewer_strings_adds_class_and_key_count() {
        let mut e = Engine::new();
        e.map(NI_ROT_DATA_RTTI & !0xfff, 0x1000);
        e.set_global(NI_ROT_DATA_RTTI, 0x0444_0000u32);
        e.register(OBJECT_GET_VIEWER_STRINGS, |_, _| Ret::default());
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
        e.mem.set_u16(strings + 8, 4);
        let this = rot_data(&mut e, 7, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_84c0, &[this.addr(), strings]);
        assert_eq!(calls_to(&e, NI_GET_VIEWER_STRING), vec![vec![0x0444_0000]]);
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_COUNT),
            vec![vec![NUM_KEYS_NAME_TEXT, 7]]
        );
        assert_eq!(e.mem.u32(base), 0x0600_0001);
        assert_eq!(e.mem.u32(base + 4), 0x0600_0002);
    }
}
