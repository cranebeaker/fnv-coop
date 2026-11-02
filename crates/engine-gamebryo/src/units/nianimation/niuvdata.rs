//! `NiAnimation/niuvdata.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

/// `NiUVData::ms_RTTI` (the class's `NiRTTI` record).
const NI_UV_DATA_RTTI: u32 = 0x011f_3f10;

// Translated from 00a45010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUVData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_uv_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_UV_DATA_RTTI)
}

// Translated from 00a45200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUVData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a45020`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x4c bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_uv_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a4_5020;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x4cu32]);
    }
    this
}

layout! {
    /// `NiUVData` (Xbox PDB), 0x4C bytes on PC as on the Xbox (offsets checked against the
    /// PC code): an `NiObject` and four float key lists (U offset, V offset, U tiling, V
    /// tiling; each a count, the keys and their type), the last values and the key sizes.
    pub struct NiUVData: 0x4C {
        /// `m_uiNumUOffsetKeys` (Xbox PDB).
        0x08 m_uiNumUOffsetKeys: u32,
        /// `m_pkUOffsetKeys` (Xbox PDB): `NiFloatKey *`.
        0x0C m_pkUOffsetKeys: u32,
        /// `m_eUOffsetType` (Xbox PDB): `NiAnimationKey::KeyType`.
        0x10 m_eUOffsetType: u32,
        /// `m_uiNumVOffsetKeys` (Xbox PDB).
        0x14 m_uiNumVOffsetKeys: u32,
        /// `m_pkVOffsetKeys` (Xbox PDB).
        0x18 m_pkVOffsetKeys: u32,
        /// `m_eVOffsetType` (Xbox PDB).
        0x1C m_eVOffsetType: u32,
        /// `m_uiNumUTilingKeys` (Xbox PDB).
        0x20 m_uiNumUTilingKeys: u32,
        /// `m_pkUTilingKeys` (Xbox PDB).
        0x24 m_pkUTilingKeys: u32,
        /// `m_eUTilingType` (Xbox PDB).
        0x28 m_eUTilingType: u32,
        /// `m_uiNumVTilingKeys` (Xbox PDB).
        0x2C m_uiNumVTilingKeys: u32,
        /// `m_pkVTilingKeys` (Xbox PDB).
        0x30 m_pkVTilingKeys: u32,
        /// `m_eVTilingType` (Xbox PDB).
        0x34 m_eVTilingType: u32,
        /// `m_fLastUOffset` (Xbox PDB), as its bit pattern.
        0x38 m_fLastUOffset: u32,
        /// `m_fLastVOffset` (Xbox PDB), as its bit pattern.
        0x3C m_fLastVOffset: u32,
        /// `m_fLastUTiling` (Xbox PDB), as its bit pattern.
        0x40 m_fLastUTiling: u32,
        /// `m_fLastVTiling` (Xbox PDB), as its bit pattern.
        0x44 m_fLastVTiling: u32,
        /// `m_ucUOffsetSize` (Xbox PDB).
        0x48 m_ucUOffsetSize: u8,
        /// `m_ucVOffsetSize` (Xbox PDB).
        0x49 m_ucVOffsetSize: u8,
        /// `m_ucUTilingSize` (Xbox PDB).
        0x4A m_ucUTilingSize: u8,
        /// `m_ucVTilingSize` (Xbox PDB).
        0x4B m_ucVTilingSize: u8,
    }
}

/// `NiUVData`'s vtable (`01097f84`).
const NI_UV_DATA_VTABLE: u32 = 0x0109_7f84;
/// The float key function tables, one word per key type: delete (`cdecl`, the keys,
/// `011f38a0`), equal (`cdecl`, two keys, `011f3ae0`), create (`cdecl`, (stream, count),
/// answers the keys, `011f3db0`), fill (`cdecl`, (keys, count, size), `011f3780`) and save
/// (`cdecl`, (stream, keys, count), `011f3a50`); and the key size bytes (`011f3758`).
const FLOAT_KEY_DELETE: u32 = 0x011f_38a0;
const FLOAT_KEY_EQUAL: u32 = 0x011f_3ae0;
const FLOAT_KEY_CREATE: u32 = 0x011f_3db0;
const FLOAT_KEY_FILL: u32 = 0x011f_3780;
const FLOAT_KEY_SAVE: u32 = 0x011f_3a50;
const FLOAT_KEY_SIZE: u32 = 0x011f_3758;
/// `NiObject`'s destructor body (`00a5d3d0`), `IsEqual` (`00a5d470`), `LoadBinary`
/// (`00a5d610`) and `GetViewerStrings` (`00a5d6e0`), and the base `SaveBinary` this class
/// calls (`00a89af0`, unnamed in the engine map), each `thiscall`.
const OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
const OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
const OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
const BASE_SAVE_BINARY: u32 = 0x00a8_9af0;
/// `NiGetViewerString` (`00a7d330`, `cdecl`, the text) and its `(name, unsigned int)` overload
/// (`00a7d510`), and the viewer-string array's grow (`0096ad30`, `thiscall`, the new size)
/// and set (`0096ae90`, `thiscall`, (index, &text)).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_UINT: u32 = 0x00a7_d510;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The viewer-string names `"m_uiNumUOffsetKeys"` (`0109804c`), `"m_uiNumUTilingKeys"`
/// (`01098038`), `"m_uiNumVOffsetKeys"` (`01098024`) and `"m_uiNumVTilingKeys"` (`01098010`).
const NAME_U_OFFSET_KEYS: u32 = 0x0109_804c;
const NAME_U_TILING_KEYS: u32 = 0x0109_8038;
const NAME_V_OFFSET_KEYS: u32 = 0x0109_8024;
const NAME_V_TILING_KEYS: u32 = 0x0109_8010;

/// A key list's fields: the count (+8 + 0xc * channel), the keys after it, the type after
/// those and the size byte (+0x48 + channel); channels U offset, V offset, U tiling, V
/// tiling.
fn channel_fields(channel: u32) -> (u32, u32, u32, u32) {
    let count = 8 + 0xc * channel;
    (count, count + 4, count + 8, 0x48 + channel)
}

/// The function for key type `kind` in the table at `table`.
fn key_function(e: &Engine, table: u32, kind: u32) -> u32 {
    e.mem.u32(table.wrapping_add(kind.wrapping_mul(4)))
}

/// Deletes a channel's keys through their type's delete function, when there are keys.
fn delete_channel_keys(e: &mut Engine, this: Ptr<NiUVData>, channel: u32) {
    let (_, keys_at, type_at, _) = channel_fields(channel);
    let keys = e.mem.u32(this.addr() + keys_at);
    if keys != 0 {
        let kind = e.mem.u32(this.addr() + type_at);
        let delete = key_function(e, FLOAT_KEY_DELETE, kind);
        e.call(delete, &args![keys]);
    }
}

/// The key replacement the four `00a45220`-style functions share: the old keys deleted, then
/// `keys`, `count`, `kind` and the type's size byte (`011f3758`) when `keys` and `count` are
/// both non-zero, else all four zero.
fn set_channel_keys(
    e: &mut Engine,
    this: Ptr<NiUVData>,
    channel: u32,
    keys: u32,
    count: u32,
    kind: u32,
) {
    delete_channel_keys(e, this, channel);
    let (count_at, keys_at, type_at, size_at) = channel_fields(channel);
    let at = this.addr();
    if keys != 0 && count != 0 {
        e.mem.set_u32(at + count_at, count);
        e.mem.set_u32(at + keys_at, keys);
        e.mem.set_u32(at + type_at, kind);
        let size = e.mem.u8(FLOAT_KEY_SIZE.wrapping_add(kind));
        e.mem.set_u8(at + size_at, size);
    } else {
        e.mem.set_u32(at + count_at, 0);
        e.mem.set_u32(at + keys_at, 0);
        e.mem.set_u32(at + type_at, 0);
        e.mem.set_u8(at + size_at, 0);
    }
}

/// Reads 4 bytes from the stream's input into `destination`, the byte-count word at `count`
/// set to 4 first; the answer is added to the input's position.
fn read_word(e: &mut Engine, stream: Ptr<NiStream>, destination: u32, count: u32) {
    let input = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
    let function = e.get(input, NiBinaryStream::m_pfnRead);
    e.mem.set_u32(count, 4);
    let done = e
        .call(function, &args![input, destination, 4u32, count, 1u32])
        .u32();
    let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
    e.set(
        input,
        NiBinaryStream::m_uiAbsoluteCurrentPos,
        position.wrapping_add(done),
    );
}

/// Writes 4 bytes from `source` to the stream's output, the byte-count word at `count` set to
/// 4 first; the answer is added to the output's position.
fn write_word(e: &mut Engine, stream: Ptr<NiStream>, source: u32, count: u32) {
    let output = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkOstr));
    let function = e.get(output, NiBinaryStream::m_pfnWrite);
    e.mem.set_u32(count, 4);
    let done = e
        .call(function, &args![output, source, 4u32, count, 1u32])
        .u32();
    let position = e.get(output, NiBinaryStream::m_uiAbsoluteCurrentPos);
    e.set(
        output,
        NiBinaryStream::m_uiAbsoluteCurrentPos,
        position.wrapping_add(done),
    );
}

/// Appends `text` to the viewer-string array at `strings` as the exe's inlined `Add` does:
/// when the 16-bit size (+0xA) has reached the capacity (+0x8) the array grows to the step
/// (+0xE) plus the size first; the text goes in through the stack word at `slot`.
fn append_viewer_string(e: &mut Engine, strings: u32, slot: u32, text: u32) {
    e.mem.set_u32(slot, text);
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.call(STRINGS_SET_AT, &args![strings, size, slot]);
}

// Translated from 00a45020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; by its body the `NiUVData`
/// destructor body): the class's vtable (`01097f84`), each of the four key lists deleted
/// through its type's delete function (`011f38a0`) when it has keys, then a tail jump to the
/// `NiObject` destructor body (`00a5d3d0`).
pub fn fn_00a45020(e: &mut Engine, this: Ptr<NiUVData>) {
    e.mem.set_u32(this.addr(), NI_UV_DATA_VTABLE);
    for channel in 0..4 {
        delete_channel_keys(e, this, channel);
    }
    e.call(OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00a45090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUVData::IsEqual` (Xbox PDB; `thiscall`, the other object): equal when `NiObject::IsEqual`
/// (`00a5d470`) is, the counts and types of the four lists and their size bytes are equal, and
/// every key of each list is equal to the other's (the equal function of its type, `011f3ae0`,
/// on keys `size * i` into each list; `this`'s count and size).
pub fn ni_uv_data_is_equal(e: &mut Engine, this: Ptr<NiUVData>, other: Ptr<NiUVData>) -> bool {
    if !e.call(OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let (a, b) = (this.addr(), other.addr());
    let words_equal = [8u32, 0x10, 0x14, 0x1c, 0x20, 0x28, 0x2c, 0x34]
        .iter()
        .all(|&offset| e.mem.u32(a + offset) == e.mem.u32(b + offset));
    let bytes_equal = (0x48..0x4c).all(|offset| e.mem.u8(a + offset) == e.mem.u8(b + offset));
    if !words_equal || !bytes_equal {
        return false;
    }
    for channel in 0..4 {
        let (count_at, keys_at, type_at, size_at) = channel_fields(channel);
        let equal = key_function(e, FLOAT_KEY_EQUAL, e.mem.u32(a + type_at));
        let mut i = 0u32;
        while i < e.mem.u32(a + count_at) {
            let offset = u32::from(e.mem.u8(a + size_at)).wrapping_mul(i);
            let mine = e.mem.u32(a + keys_at).wrapping_add(offset);
            let theirs = e.mem.u32(b + keys_at).wrapping_add(offset);
            if !e.call(equal, &args![mine, theirs]).bool() {
                return false;
            }
            i += 1;
        }
    }
    true
}

// Translated from 00a45220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, (keys, count, type); by its body the U offset key
/// replacement): deletes the current U offset keys through their type's delete function and,
/// when `keys` and `count` are both non-zero, takes them with `kind` and that type's key size
/// (`011f3758`); otherwise the list is left empty.
pub fn fn_00a45220(e: &mut Engine, this: Ptr<NiUVData>, keys: u32, count: u32, kind: u32) {
    set_channel_keys(e, this, 0, keys, count, kind);
}

// Translated from 00a45280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, (keys, count, type)): the V offset key replacement,
/// as `00a45220` for the second list (+0x14).
pub fn fn_00a45280(e: &mut Engine, this: Ptr<NiUVData>, keys: u32, count: u32, kind: u32) {
    set_channel_keys(e, this, 1, keys, count, kind);
}

// Translated from 00a452e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, (keys, count, type)): the U tiling key replacement,
/// as `00a45220` for the third list (+0x20).
pub fn fn_00a452e0(e: &mut Engine, this: Ptr<NiUVData>, keys: u32, count: u32, kind: u32) {
    set_channel_keys(e, this, 2, keys, count, kind);
}

// Translated from 00a45340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, (keys, count, type)): the V tiling key replacement,
/// as `00a45220` for the fourth list (+0x2c).
pub fn fn_00a45340(e: &mut Engine, this: Ptr<NiUVData>, keys: u32, count: u32, kind: u32) {
    set_channel_keys(e, this, 3, keys, count, kind);
}

// Translated from 00a453a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUVData::LoadBinary` (Xbox PDB; `thiscall`, the stream): `NiObject::LoadBinary`
/// (`00a5d610`), then for each list in order (U offset, V offset, U tiling, V tiling) its
/// 4-byte count and, when it is non-zero, its 4-byte type (the size byte set from
/// `011f3758`), keys made and read by the type's create function (`011f3db0`), filled by its
/// fill function (`011f3780`, with the size) and taken by the list's key replacement
/// (`00a45220`, ...). The count goes into the stream argument's stack word; the byte-count
/// words are the two stack locals as the exe uses them.
pub fn ni_uv_data_load_binary(e: &mut Engine, this: Ptr<NiUVData>, stream: Ptr<NiStream>) {
    e.call(OBJECT_LOAD_BINARY, &args![this, stream]);
    // Stack words: the lower local (ESP-8), the upper one (ESP-4) and the stream argument.
    e.with_stack(0xc, |e, words| {
        let lower = words.addr();
        let upper = lower + 4;
        let argument = lower + 8;
        for channel in 0..4 {
            let (count_word, type_word) = if channel == 0 {
                (lower, upper)
            } else {
                (upper, lower)
            };
            read_word(e, stream, argument, count_word);
            if e.mem.u32(argument) == 0 {
                continue;
            }
            read_word(e, stream, type_word, count_word);
            let kind = e.mem.u32(type_word);
            let (_, _, _, size_at) = channel_fields(channel);
            let size = e.mem.u8(FLOAT_KEY_SIZE.wrapping_add(kind));
            e.mem.set_u8(this.addr() + size_at, size);
            let create = key_function(e, FLOAT_KEY_CREATE, kind);
            let count = e.mem.u32(argument);
            let keys = e.call(create, &args![stream, count]).u32();
            let size = u32::from(e.mem.u8(this.addr() + size_at));
            let fill = key_function(e, FLOAT_KEY_FILL, kind);
            let count = e.mem.u32(argument);
            if channel != 3 {
                e.mem.set_u32(upper, keys);
            }
            e.call(fill, &args![keys, count, size]);
            let count = e.mem.u32(argument);
            set_channel_keys(e, this, channel, keys, count, kind);
        }
    });
}

// Translated from 00a45610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUVData::SaveBinary` (Xbox PDB; `thiscall`, the stream): the base `SaveBinary`
/// (`00a89af0`), then for each list in order its 4-byte count and, when it is non-zero, its
/// 4-byte type and the keys through the type's save function (`011f3a50`, (stream, keys,
/// count)). The stream argument's stack word is the byte-count word.
pub fn ni_uv_data_save_binary(e: &mut Engine, this: Ptr<NiUVData>, stream: Ptr<NiStream>) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, word| {
        let at = this.addr();
        for channel in 0..4 {
            let (count_at, keys_at, type_at, _) = channel_fields(channel);
            write_word(e, stream, at + count_at, word.addr());
            if e.mem.u32(at + count_at) == 0 {
                continue;
            }
            write_word(e, stream, at + type_at, word.addr());
            let count = e.mem.u32(at + count_at);
            let keys = e.mem.u32(at + keys_at);
            let save = key_function(e, FLOAT_KEY_SAVE, e.mem.u32(at + type_at));
            e.call(save, &args![stream, keys, count]);
        }
    });
}

// Translated from 00a457d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiUVData::GetViewerStrings` (Xbox PDB; `thiscall`, the strings): `NiObject`'s strings
/// (`00a5d6e0`), then appended: the class name (the first word of the `NiRTTI` record at
/// `011f3f10`, through `NiGetViewerString`) and the four key counts as `(name, count)`
/// strings (`00a7d510`): U offset, U tiling, V offset, V tiling. Each text goes in through
/// the strings argument's stack word.
pub fn ni_uv_data_get_viewer_strings(e: &mut Engine, this: Ptr<NiUVData>, strings: u32) {
    e.call(OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let class_name = e.mem.u32(NI_UV_DATA_RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
        append_viewer_string(e, strings, slot.addr(), text);
        for (name, count) in [
            (NAME_U_OFFSET_KEYS, NiUVData::m_uiNumUOffsetKeys),
            (NAME_U_TILING_KEYS, NiUVData::m_uiNumUTilingKeys),
            (NAME_V_OFFSET_KEYS, NiUVData::m_uiNumVOffsetKeys),
            (NAME_V_TILING_KEYS, NiUVData::m_uiNumVTilingKeys),
        ] {
            let value = e.get(this, count);
            let text = e.call(NI_GET_VIEWER_STRING_UINT, &args![name, value]).u32();
            append_viewer_string(e, strings, slot.addr(), text);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a45010, ni_uv_data_get_rtti() -> Ptr),
        entry!(0x00a45200, ni_uv_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a4_5020, fn_00a45020(Ptr<NiUVData>)),
        entry!(
            0x00a4_5090,
            ni_uv_data_is_equal(Ptr<NiUVData>, Ptr<NiUVData>) -> bool
        ),
        entry!(0x00a4_5220, fn_00a45220(Ptr<NiUVData>, u32, u32, u32)),
        entry!(0x00a4_5280, fn_00a45280(Ptr<NiUVData>, u32, u32, u32)),
        entry!(0x00a4_52e0, fn_00a452e0(Ptr<NiUVData>, u32, u32, u32)),
        entry!(0x00a4_5340, fn_00a45340(Ptr<NiUVData>, u32, u32, u32)),
        entry!(
            0x00a4_53a0,
            ni_uv_data_load_binary(Ptr<NiUVData>, Ptr<NiStream>)
        ),
        entry!(
            0x00a4_5610,
            ni_uv_data_save_binary(Ptr<NiUVData>, Ptr<NiStream>)
        ),
        entry!(
            0x00a4_57d0,
            ni_uv_data_get_viewer_strings(Ptr<NiUVData>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_uv_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a45010, &args![]).u32(), NI_UV_DATA_RTTI);
    }

    #[test]
    fn ni_uv_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a4_5020, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a45200, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a45200, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x4c]]);
        assert_eq!(calls_to(&e, 0x00a4_5020), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod n006_tests {
    use super::*;

    const DELETE: u32 = 0x0300_0400;
    const EQUAL: u32 = 0x0300_0410;
    const CREATE: u32 = 0x0300_0420;
    const FILL: u32 = 0x0300_0430;
    const SAVE: u32 = 0x0300_0440;
    const READ: u32 = 0x0300_0450;
    const WRITE: u32 = 0x0300_0460;
    const WRITE_LOG: u32 = 0x0300_0470;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The key tables for type 3: each function a double, size byte 0x14.
    fn tables(e: &mut Engine) {
        e.map(0x011f_3700, 0x800);
        for (table, f) in [
            (FLOAT_KEY_DELETE, DELETE),
            (FLOAT_KEY_EQUAL, EQUAL),
            (FLOAT_KEY_CREATE, CREATE),
            (FLOAT_KEY_FILL, FILL),
            (FLOAT_KEY_SAVE, SAVE),
        ] {
            e.mem.set_u32(table + 0xc, f);
            e.register(f, |_, _| Ret::default());
        }
        e.mem.set_u8(FLOAT_KEY_SIZE + 3, 0x14);
    }

    fn uv_data(e: &mut Engine) -> Ptr<NiUVData> {
        let this = e.new_object::<NiUVData>();
        for channel in 0..4 {
            let (count_at, keys_at, type_at, size_at) = channel_fields(channel);
            e.mem.set_u32(this.addr() + count_at, 1 + channel);
            e.mem
                .set_u32(this.addr() + keys_at, 0x7000 + 0x100 * channel);
            e.mem.set_u32(this.addr() + type_at, 3);
            e.mem.set_u8(this.addr() + size_at, 0x14);
        }
        this
    }

    #[test]
    fn fn_00a45020_deletes_every_list_then_the_object() {
        let mut e = Engine::new();
        tables(&mut e);
        e.register(OBJECT_DESTRUCT, |_, _| Ret::default());
        let this = uv_data(&mut e);
        e.set(this, NiUVData::m_pkVOffsetKeys, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_5020, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), NI_UV_DATA_VTABLE);
        assert_eq!(
            calls_to(&e, DELETE),
            vec![vec![0x7000], vec![0x7200], vec![0x7300]]
        );
        assert_eq!(calls_to(&e, OBJECT_DESTRUCT), vec![vec![this.addr()]]);
    }

    #[test]
    fn ni_uv_data_is_equal_compares_fields_then_keys() {
        let mut e = Engine::new();
        tables(&mut e);
        e.register(OBJECT_IS_EQUAL, |_, _| 1u32.into_ret());
        e.register(EQUAL, |_, a| (a[1] != 0x7314).into_ret());
        let a = uv_data(&mut e);
        let b = uv_data(&mut e);
        e.call_log = Some(vec![]);
        // The fourth list's second key differs.
        assert!(!e.call(0x00a4_5090, &args![a, b]).bool());
        let keys: Vec<u32> = calls_to(&e, EQUAL).iter().map(|c| c[0]).collect();
        assert_eq!(
            keys,
            vec![0x7000, 0x7100, 0x7114, 0x7200, 0x7214, 0x7228, 0x7300, 0x7314]
        );
        e.register(EQUAL, |_, _| 1u32.into_ret());
        assert!(e.call(0x00a4_5090, &args![a, b]).bool());
        e.set(b, NiUVData::m_ucVTilingSize, 0x10);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a4_5090, &args![a, b]).bool());
        assert!(calls_to(&e, EQUAL).is_empty());
    }

    #[test]
    fn the_key_replacements_take_or_clear_their_list() {
        let mut e = Engine::new();
        tables(&mut e);
        for (channel, addr) in [0x00a4_5220u32, 0x00a4_5280, 0x00a4_52e0, 0x00a4_5340]
            .iter()
            .enumerate()
        {
            let channel = channel as u32;
            let this = uv_data(&mut e);
            let (count_at, keys_at, type_at, size_at) = channel_fields(channel);
            e.call_log = Some(vec![]);
            e.call(*addr, &args![this, 0x9000u32, 7u32, 3u32]);
            assert_eq!(calls_to(&e, DELETE), vec![vec![0x7000 + 0x100 * channel]]);
            let at = this.addr();
            assert_eq!(
                (
                    e.mem.u32(at + count_at),
                    e.mem.u32(at + keys_at),
                    e.mem.u32(at + type_at)
                ),
                (7, 0x9000, 3)
            );
            assert_eq!(e.mem.u8(at + size_at), 0x14);
            // No keys: the list is cleared.
            e.call(*addr, &args![this, 0u32, 7u32, 3u32]);
            assert_eq!(
                (
                    e.mem.u32(at + count_at),
                    e.mem.u32(at + keys_at),
                    e.mem.u32(at + type_at)
                ),
                (0, 0, 0)
            );
            assert_eq!(e.mem.u8(at + size_at), 0);
        }
    }

    #[test]
    fn ni_uv_data_load_binary_reads_each_present_list() {
        let mut e = Engine::new();
        tables(&mut e);
        e.register(OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(CREATE, |_, a| (0x8000 + a[1]).into_ret());
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkIstr, input.addr());
        e.set(input, NiBinaryStream::m_pfnRead, READ);
        // The words read: U offset 2 keys of type 3, V offset none, U tiling 1 of type 3,
        // V tiling none.
        let mut words = vec![2u32, 3, 0, 1, 3, 0].into_iter();
        e.register_double(READ, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            e.mem.set_u32(a[1], words.next().unwrap());
            4u32.into_ret()
        });
        let this = e.new_object::<NiUVData>();
        e.call_log = Some(vec![]);
        e.call(0x00a4_53a0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, OBJECT_LOAD_BINARY),
            vec![vec![this.addr(), stream.addr()]]
        );
        assert_eq!(
            calls_to(&e, CREATE),
            vec![vec![stream.addr(), 2], vec![stream.addr(), 1]]
        );
        assert_eq!(
            calls_to(&e, FILL),
            vec![vec![0x8002, 2, 0x14], vec![0x8001, 1, 0x14]]
        );
        assert_eq!(e.get(this, NiUVData::m_uiNumUOffsetKeys), 2);
        assert_eq!(e.get(this, NiUVData::m_pkUOffsetKeys), 0x8002);
        assert_eq!(e.get(this, NiUVData::m_uiNumUTilingKeys), 1);
        assert_eq!(e.get(this, NiUVData::m_pkUTilingKeys), 0x8001);
        assert_eq!(e.get(this, NiUVData::m_ucUTilingSize), 0x14);
        assert_eq!(e.get(this, NiUVData::m_uiNumVOffsetKeys), 0);
        assert_eq!(e.mem.u32(input.addr() + 4), 24);
    }

    #[test]
    fn ni_uv_data_save_binary_writes_each_list() {
        let mut e = Engine::new();
        tables(&mut e);
        e.register(BASE_SAVE_BINARY, |_, _| Ret::default());
        let stream = e.new_object::<NiStream>();
        let output = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkOstr, output.addr());
        e.set(output, NiBinaryStream::m_pfnWrite, WRITE);
        e.register(WRITE_LOG, |_, _| Ret::default());
        e.register(WRITE, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            let word = e.mem.u32(a[1]);
            e.call(WRITE_LOG, &args![word]);
            4u32.into_ret()
        });
        let this = uv_data(&mut e);
        e.set(this, NiUVData::m_uiNumVOffsetKeys, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a4_5610, &args![this, stream]);
        let written: Vec<u32> = calls_to(&e, WRITE_LOG).iter().map(|c| c[0]).collect();
        assert_eq!(written, vec![1, 3, 0, 3, 3, 4, 3]);
        assert_eq!(
            calls_to(&e, SAVE),
            vec![
                vec![stream.addr(), 0x7000, 1],
                vec![stream.addr(), 0x7200, 3],
                vec![stream.addr(), 0x7300, 4]
            ]
        );
    }

    #[test]
    fn ni_uv_data_get_viewer_strings_adds_the_class_and_counts() {
        let mut e = Engine::new();
        e.map(NI_UV_DATA_RTTI, 4);
        e.mem.set_u32(NI_UV_DATA_RTTI, 0x0108_0000);
        for f in [OBJECT_GET_VIEWER_STRINGS, STRINGS_GROW] {
            e.register(f, |_, _| Ret::default());
        }
        e.register(NI_GET_VIEWER_STRING, |_, a| a[0].into_ret());
        e.register(NI_GET_VIEWER_STRING_UINT, |_, a| (a[0] + a[1]).into_ret());
        e.register(STRINGS_SET_AT, |e, a| {
            // Records the text the slot holds.
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            let text = e.mem.u32(a[2]);
            e.call(WRITE_LOG, &args![text]);
            Ret::default()
        });
        e.register(WRITE_LOG, |_, _| Ret::default());
        let this = uv_data(&mut e);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 3);
        e.mem.set_u16(strings + 0xe, 4);
        e.call_log = Some(vec![]);
        e.call(0x00a4_57d0, &args![this, strings]);
        let texts: Vec<u32> = calls_to(&e, WRITE_LOG).iter().map(|c| c[0]).collect();
        assert_eq!(
            texts,
            vec![
                0x0108_0000,
                NAME_U_OFFSET_KEYS + 1,
                NAME_U_TILING_KEYS + 3,
                NAME_V_OFFSET_KEYS + 2,
                NAME_V_TILING_KEYS + 4
            ]
        );
        // From size 3 on the array counts as full (the double keeps the capacity): it grows
        // to the step plus the size before each of the last two.
        assert_eq!(
            calls_to(&e, STRINGS_GROW),
            vec![vec![strings, 7], vec![strings, 8]]
        );
    }
}
