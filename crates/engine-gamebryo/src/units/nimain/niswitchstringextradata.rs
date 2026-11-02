//! `NiMain/niswitchstringextradata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

/// `NiSwitchStringExtraData::ms_RTTI` (the class's `NiRTTI` record).
const NI_SWITCH_STRING_EXTRA_DATA_RTTI: u32 = 0x011f_5eac;

// Translated from 00a93740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchStringExtraData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_switch_string_extra_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SWITCH_STRING_EXTRA_DATA_RTTI)
}

// Translated from 00a938b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchStringExtraData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a93750`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x18 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_switch_string_extra_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a9_3750;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x18u32]);
    }
    this
}

/// `NiSwitchStringExtraData` (Xbox PDB, offsets checked against the PC code): `m_uiSize`
/// +0x0C, `m_ppcValue` +0x10 (`char**`, one heap string per element) and `m_iIndex` +0x14.
const SIZE: u32 = 0x0c;
const VALUES: u32 = 0x10;
const INDEX: u32 = 0x14;
/// The class's vtable (`0109ff64`).
const NI_SWITCH_STRING_EXTRA_DATA_VTABLE: u32 = 0x0109_ff64;
/// `NiNew`'s allocator (`00aa13e0`, cdecl: size), `NiAlloc` (`00aa1070`, cdecl: size) and
/// `NiFree` (`00aa10f0`, cdecl: block).
const NI_NEW: u32 = 0x00aa_13e0;
const NI_ALLOC: u32 = 0x00aa_1070;
const NI_FREE: u32 = 0x00aa_10f0;
/// `_strcpy_s` (`00ec65a6`, cdecl: destination, size, source).
const STRCPY_S: u32 = 0x00ec_65a6;
/// `NiExtraData`'s default constructor body `00a7b2e0`, destructor body `00a7b300` and
/// member copy `00a7b330` (unnamed in the engine map; thiscall, the copy taking the
/// destination and the cloning process), and its `SaveBinary` (`00a7b2c0`), `LoadBinary`
/// (`00a7b4a0`) and `GetViewerStrings` (`00a7b580`), thiscall, one word each.
const EXTRA_DATA_CONSTRUCT_00A7B2E0: u32 = 0x00a7_b2e0;
const EXTRA_DATA_DESTRUCT_00A7B300: u32 = 0x00a7_b300;
const EXTRA_DATA_COPY_MEMBERS_00A7B330: u32 = 0x00a7_b330;
const EXTRA_DATA_SAVE_BINARY: u32 = 0x00a7_b2c0;
const EXTRA_DATA_LOAD_BINARY: u32 = 0x00a7_b4a0;
const EXTRA_DATA_GET_VIEWER_STRINGS: u32 = 0x00a7_b580;
/// The stream's string writer `00a64160` (unnamed in the engine map; thiscall on the
/// `NiStream`, the string) and `NiStream::LoadCString` (`00a64ce0`, thiscall on the
/// stream, the address of the `char*` to fill).
const STREAM_SAVE_STRING_00A64160: u32 = 0x00a6_4160;
const STREAM_LOAD_C_STRING: u32 = 0x00a6_4ce0;
/// `NiGetViewerString` (cdecl, the class name), `NiGetViewerString_ov7` (cdecl, a name and
/// an `int`, `"%s = %d"`), `NiGetViewerString_ov8` (cdecl, a name and an unsigned value,
/// `"%s = %u"`) and `NiGetViewerString_ov11` (cdecl, a name and a string, `"%s = %s"` or
/// `"%s = NULL"`): each returns a new string.
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_SIGNED: u32 = 0x00a7_d4d0;
const VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
const VIEWER_STRING_STRING: u32 = 0x00a7_d5e0;
/// The strings array's `SetSize` (thiscall, the new size) and `SetAt` (thiscall, the index
/// and the address of the element).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `"m_uiSize"`, `"m_ppcValue[i]"` and `"m_iIndex"`.
const TEXT_SIZE: u32 = 0x0109_df4c;
const TEXT_VALUE: u32 = 0x010a_0004;
const TEXT_INDEX: u32 = 0x0109_fff8;

/// Whether the two zero-terminated strings differ (the code's inline `strcmp`: byte by
/// byte until a difference or the end of the first).
fn strings_differ(e: &Engine, first: u32, second: u32) -> bool {
    let mut i = 0u32;
    loop {
        let byte = e.mem.u8(first.wrapping_add(i));
        if byte != e.mem.u8(second.wrapping_add(i)) {
            return true;
        }
        if byte == 0 {
            return false;
        }
        i = i.wrapping_add(1);
    }
}

/// The length of the zero-terminated string at `text` (the code's inline `strlen`).
fn string_length(e: &Engine, text: u32) -> u32 {
    let mut length = 0u32;
    while e.mem.u8(text.wrapping_add(length)) != 0 {
        length = length.wrapping_add(1);
    }
    length
}

/// Adds the string word at `slot` at the end of the strings array (`SetSize` to the size
/// plus the grow-by step when the array is full, then `SetAt`).
fn add_viewer_string(e: &mut Engine, strings: Ptr<NiTArray>, slot: u32) {
    let index = u32::from(e.get(strings, NiTArray::m_usSize));
    if index >= u32::from(e.get(strings, NiTArray::m_usMaxSize)) {
        let grow = u32::from(e.get(strings, NiTArray::m_usGrowBy));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow + index]);
    }
    e.call(STRING_ARRAY_SET_AT, &args![strings, index, slot]);
}

/// Adds `text` at the end of the strings array with the array's `SetAt` inlined: `SetSize`
/// when full; then, at the old size, a slot at or past the (re-read) size makes the size
/// one more and counts a non-null string, a slot inside it counts the change between a
/// null and a non-null element; then the element is stored.
fn add_viewer_string_inline(e: &mut Engine, strings: Ptr<NiTArray>, text: u32) {
    let index = e.get(strings, NiTArray::m_usSize);
    if index >= e.get(strings, NiTArray::m_usMaxSize) {
        let grow = u32::from(e.get(strings, NiTArray::m_usGrowBy));
        e.call(
            STRING_ARRAY_SET_SIZE,
            &args![strings, grow + u32::from(index)],
        );
    }
    let used = e.get(strings, NiTArray::m_usESize);
    if index >= e.get(strings, NiTArray::m_usSize) {
        e.set(strings, NiTArray::m_usSize, index.wrapping_add(1));
        if text != 0 {
            e.set(strings, NiTArray::m_usESize, used.wrapping_add(1));
        }
    } else {
        let base = e.get(strings, NiTArray::m_pBase);
        let old = e.mem.u32(base + u32::from(index) * 4);
        if text != 0 {
            if old == 0 {
                e.set(strings, NiTArray::m_usESize, used.wrapping_add(1));
            }
        } else if old != 0 {
            e.set(strings, NiTArray::m_usESize, used.wrapping_sub(1));
        }
    }
    let base = e.get(strings, NiTArray::m_pBase);
    e.mem.set_u32(base + u32::from(index) * 4, text);
}

/// One 4-byte transfer (one component of 4 bytes) of the field at `field` through the
/// binary stream `binary` and its function `function` (`m_pfnRead` or `m_pfnWrite`); the
/// component size is kept in the stream argument's stack word, and the stream's position
/// moves on by the result.
fn transfer_u32(e: &mut Engine, binary: Ptr<NiBinaryStream>, function: u32, field: u32) {
    e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), 4);
        let moved = e
            .call(function, &args![binary, field, 4u32, sizes, 1u32])
            .u32();
        let position = e.get(binary, NiBinaryStream::m_uiAbsoluteCurrentPos);
        e.set(
            binary,
            NiBinaryStream::m_uiAbsoluteCurrentPos,
            position.wrapping_add(moved),
        );
    });
}

// Translated from 00a93680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchStringExtraData::IsEqual` (Xbox PDB; thiscall, `RET 4`; the base class's `IsEqual`
/// is not called): false for a null `other`, a different size or index, or one value array
/// null and the other not; then, with values, each element pair must be both null or both
/// strings with the same characters.
pub fn ni_switch_string_extra_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if other.is_null() {
        return false;
    }
    let size = e.mem.u32(this.addr() + SIZE);
    if size != e.mem.u32(other.addr() + SIZE)
        || e.mem.u32(this.addr() + INDEX) != e.mem.u32(other.addr() + INDEX)
    {
        return false;
    }
    let values = e.mem.u32(this.addr() + VALUES);
    if values != 0 {
        if e.mem.u32(other.addr() + VALUES) == 0 {
            return false;
        }
    } else if e.mem.u32(other.addr() + VALUES) != 0 {
        return false;
    }
    if values != 0 {
        for i in 0..size {
            let mine = e.mem.u32(values + i * 4);
            let theirs = e.mem.u32(e.mem.u32(other.addr() + VALUES) + i * 4);
            if mine != 0 {
                if theirs == 0 {
                    return false;
                }
            } else {
                if theirs != 0 {
                    return false;
                }
                continue;
            }
            if strings_differ(e, mine, theirs) {
                return false;
            }
        }
    }
    true
}

// Translated from 00a93750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the body is `NiSwitchStringExtraData`'s destructor (thiscall):
/// sets the class's vtable (`0109ff64`), frees each string (`NiFree`) and then the array,
/// clears the array pointer and ends in `NiExtraData`'s destructor body (`00a7b300`).
pub fn fn_00a93750(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, NI_SWITCH_STRING_EXTRA_DATA_VTABLE);
    let mut i = 0u32;
    while i < e.mem.u32(base + SIZE) {
        let text = e.mem.u32(e.mem.u32(base + VALUES) + i * 4);
        e.call(NI_FREE, &args![text]);
        i += 1;
    }
    let values = e.mem.u32(base + VALUES);
    e.call(NI_FREE, &args![values]);
    e.mem.set_u32(base + VALUES, 0);
    e.call(EXTRA_DATA_DESTRUCT_00A7B300, &args![this]);
}

// Translated from 00a937a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the body is `NiSwitchStringExtraData`'s member copy (thiscall,
/// `RET 8`: the destination and the cloning process): `NiExtraData`'s copy (`00a7b330`),
/// then without values or with size 0 the destination gets neither (its index is left as
/// it is); otherwise the size, the index when it is from 0 to below the size (signed
/// compares) and -1 otherwise, a new pointer array (`NiAlloc`, four bytes per element) and
/// a copy of each string (`NiAlloc` of its length plus one, `_strcpy_s`), a null element
/// staying null.
pub fn fn_00a937a0(e: &mut Engine, this: Ptr, dest: Ptr, cloning: u32) {
    e.call(
        EXTRA_DATA_COPY_MEMBERS_00A7B330,
        &args![this, dest, cloning],
    );
    let (source, target) = (this.addr(), dest.addr());
    if e.mem.u32(source + VALUES) == 0 || e.mem.u32(source + SIZE) == 0 {
        e.mem.set_u32(target + VALUES, 0);
        e.mem.set_u32(target + SIZE, 0);
        return;
    }
    let size = e.mem.u32(source + SIZE);
    e.mem.set_u32(target + SIZE, size);
    let index = e.mem.u32(source + INDEX) as i32;
    if index > -1 && index < e.mem.u32(source + SIZE) as i32 {
        e.mem.set_u32(target + INDEX, index as u32);
    } else {
        e.mem.set_u32(target + INDEX, u32::MAX);
    }
    let size = e.mem.u32(source + SIZE);
    let array = e.call(NI_ALLOC, &args![size.wrapping_mul(4)]).u32();
    e.mem.set_u32(target + VALUES, array);
    let mut i = 0u32;
    while i < e.mem.u32(source + SIZE) {
        let text = e.mem.u32(e.mem.u32(source + VALUES) + i * 4);
        if text == 0 {
            let values = e.mem.u32(target + VALUES);
            e.mem.set_u32(values + i * 4, 0);
        } else {
            let bytes = string_length(e, text).wrapping_add(1);
            let copy = e.call(NI_ALLOC, &args![bytes]).u32();
            let values = e.mem.u32(target + VALUES);
            e.mem.set_u32(values + i * 4, copy);
            let text = e.mem.u32(e.mem.u32(source + VALUES) + i * 4);
            let copy = e.mem.u32(e.mem.u32(target + VALUES) + i * 4);
            e.call(STRCPY_S, &args![copy, bytes, text]);
        }
        i += 1;
    }
}

// Translated from 00a93870 (decompiled, FalloutNV.exe 1.4.0.525)
/// The class's stream loader (Gamebryo's `CreateObject`; Ghidra made no function of it):
/// registered with `NiStream::RegisterLoader` under `"NiSwitchStringExtraData"` at
/// `00a8a0d3`. A new 0x18-byte object (`00aa13e0`) with `NiExtraData`'s default
/// constructor body, the class's vtable, no values, size 0 and index -1; null when the
/// allocation fails.
pub fn ni_switch_string_extra_data_create_object(e: &mut Engine) -> Ptr {
    let object = e.call(NI_NEW, &args![0x18u32]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(EXTRA_DATA_CONSTRUCT_00A7B2E0, &args![object]);
    e.mem.set_u32(object, NI_SWITCH_STRING_EXTRA_DATA_VTABLE);
    e.mem.set_u32(object + VALUES, 0);
    e.mem.set_u32(object + SIZE, 0);
    e.mem.set_u32(object + INDEX, u32::MAX);
    Ptr::new(object)
}

// Translated from 00a938d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchStringExtraData::CreateClone` (Xbox PDB; thiscall, `RET 4`: the cloning process): a
/// new 0x18-byte object built as [`ni_switch_string_extra_data_create_object`] builds it,
/// then the member copy ([`fn_00a937a0`]) into it. When the allocation fails the copy still
/// runs, with a null destination, and the result is null.
pub fn ni_switch_string_extra_data_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let object = e.call(NI_NEW, &args![0x18u32]).u32();
    if object == 0 {
        fn_00a937a0(e, this, Ptr::NULL, cloning);
        return Ptr::NULL;
    }
    e.call(EXTRA_DATA_CONSTRUCT_00A7B2E0, &args![object]);
    e.mem.set_u32(object, NI_SWITCH_STRING_EXTRA_DATA_VTABLE);
    e.mem.set_u32(object + VALUES, 0);
    e.mem.set_u32(object + SIZE, 0);
    e.mem.set_u32(object + INDEX, u32::MAX);
    fn_00a937a0(e, this, Ptr::new(object), cloning);
    Ptr::new(object)
}

// Translated from 00a939d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchStringExtraData::SaveBinary` (Xbox PDB; thiscall, `RET 4`): `NiExtraData::SaveBinary`,
/// the size (4 bytes through the stream being written), each string through the stream's
/// string writer (`00a64160`), then the index (4 bytes; the stream field read again).
pub fn ni_switch_string_extra_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    e.call(EXTRA_DATA_SAVE_BINARY, &args![this, stream]);
    let binary = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkOstr));
    let write = e.get(binary, NiBinaryStream::m_pfnWrite);
    transfer_u32(e, binary, write, this.addr() + SIZE);
    let mut i = 0u32;
    while i < e.mem.u32(this.addr() + SIZE) {
        let text = e.mem.u32(e.mem.u32(this.addr() + VALUES) + i * 4);
        e.call(STREAM_SAVE_STRING_00A64160, &args![stream, text]);
        i += 1;
    }
    let binary = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkOstr));
    let write = e.get(binary, NiBinaryStream::m_pfnWrite);
    transfer_u32(e, binary, write, this.addr() + INDEX);
}

// Translated from 00a93930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchStringExtraData::LoadBinary` (Xbox PDB; thiscall, `RET 4`): `NiExtraData::LoadBinary`,
/// the size (4 bytes through the stream being read); with size 0 no array, otherwise a new
/// pointer array (`NiAlloc`, four bytes per element) whose elements are each read with
/// `NiStream::LoadCString` (not cleared first); then the index (4 bytes; the stream field
/// read again).
pub fn ni_switch_string_extra_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    e.call(EXTRA_DATA_LOAD_BINARY, &args![this, stream]);
    let base = this.addr();
    let binary = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
    let read = e.get(binary, NiBinaryStream::m_pfnRead);
    transfer_u32(e, binary, read, base + SIZE);
    let size = e.mem.u32(base + SIZE);
    if size == 0 {
        e.mem.set_u32(base + VALUES, 0);
    } else {
        let array = e.call(NI_ALLOC, &args![size.wrapping_mul(4)]).u32();
        e.mem.set_u32(base + VALUES, array);
        let mut i = 0u32;
        while i < e.mem.u32(base + SIZE) {
            let slot = e.mem.u32(base + VALUES) + i * 4;
            e.call(STREAM_LOAD_C_STRING, &args![stream, slot]);
            i += 1;
        }
    }
    let binary = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
    let read = e.get(binary, NiBinaryStream::m_pfnRead);
    transfer_u32(e, binary, read, base + INDEX);
}

// Translated from 00a93a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSwitchStringExtraData::GetViewerStrings` (Xbox PDB; thiscall, `RET 4`):
/// `NiExtraData::GetViewerStrings`, the class name (the first word of `ms_RTTI`) and
/// `"m_uiSize"` with the size (each held in the argument's stack word, added through
/// `SetSize`/`SetAt`), then `"m_ppcValue[i]"` with each string and `"m_iIndex"` with the
/// index, added with the array's `SetAt` inlined.
pub fn ni_switch_string_extra_data_get_viewer_strings(
    e: &mut Engine,
    this: Ptr,
    strings: Ptr<NiTArray>,
) {
    e.call(EXTRA_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let class_name = e.global::<u32>(NI_SWITCH_STRING_EXTRA_DATA_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![class_name]).u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
        let size = e.mem.u32(this.addr() + SIZE);
        let text = e
            .call(VIEWER_STRING_UNSIGNED, &args![TEXT_SIZE, size])
            .u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
    });
    let mut i = 0u32;
    while i < e.mem.u32(this.addr() + SIZE) {
        let value = e.mem.u32(e.mem.u32(this.addr() + VALUES) + i * 4);
        let text = e
            .call(VIEWER_STRING_STRING, &args![TEXT_VALUE, value])
            .u32();
        add_viewer_string_inline(e, strings, text);
        i += 1;
    }
    let index = e.mem.u32(this.addr() + INDEX);
    let text = e
        .call(VIEWER_STRING_SIGNED, &args![TEXT_INDEX, index])
        .u32();
    add_viewer_string_inline(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a93740, ni_switch_string_extra_data_get_rtti() -> Ptr),
        entry!(0x00a938b0, ni_switch_string_extra_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a9_3680, ni_switch_string_extra_data_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a9_3750, fn_00a93750(Ptr)),
        entry!(0x00a9_37a0, fn_00a937a0(Ptr, Ptr, u32)),
        entry!(0x00a9_3870, ni_switch_string_extra_data_create_object() -> Ptr),
        entry!(
            0x00a9_38d0,
            ni_switch_string_extra_data_create_clone(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a9_39d0,
            ni_switch_string_extra_data_save_binary(Ptr, Ptr<NiStream>)
        ),
        entry!(
            0x00a9_3930,
            ni_switch_string_extra_data_load_binary(Ptr, Ptr<NiStream>)
        ),
        entry!(
            0x00a9_3a60,
            ni_switch_string_extra_data_get_viewer_strings(Ptr, Ptr<NiTArray>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_switch_string_extra_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a93740, &args![]).u32(),
            NI_SWITCH_STRING_EXTRA_DATA_RTTI
        );
    }

    #[test]
    fn ni_switch_string_extra_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a9_3750, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00a938b0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a938b0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x18]]);
        assert_eq!(calls_to(&e, 0x00a9_3750), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod n020_tests {
    use super::*;

    /// A zero-terminated copy of `text` on the heap (0 for `None`).
    fn text(e: &mut Engine, text: Option<&str>) -> u32 {
        let Some(text) = text else { return 0 };
        let block = e.mem.alloc(text.len() as u32 + 1);
        e.mem.write(block, text.as_bytes());
        block
    }

    /// An object whose values are `texts` and whose index is `index`.
    fn object(e: &mut Engine, texts: &[Option<&str>], index: u32) -> u32 {
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + SIZE, texts.len() as u32);
        e.mem.set_u32(this + INDEX, index);
        let values = e.mem.alloc(4 * texts.len() as u32 + 4);
        for (i, t) in texts.iter().enumerate() {
            let p = text(e, *t);
            e.mem.set_u32(values + 4 * i as u32, p);
        }
        e.mem.set_u32(this + VALUES, values);
        this
    }

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn c_string(e: &Engine, mut at: u32) -> String {
        let mut s = String::new();
        while e.mem.u8(at) != 0 {
            s.push(e.mem.u8(at) as char);
            at += 1;
        }
        s
    }

    #[test]
    fn is_equal_also_compares_the_index() {
        let mut e = Engine::new();
        let a = object(&mut e, &[Some("ab"), None], 1);
        let b = object(&mut e, &[Some("ab"), None], 1);
        assert_eq!(e.call(0x00a9_3680, &args![a, 0u32]).u32() & 0xff, 0);
        assert_eq!(e.call(0x00a9_3680, &args![a, b]).u32() & 0xff, 1);
        let c = object(&mut e, &[Some("ab"), None], 0);
        assert_eq!(e.call(0x00a9_3680, &args![a, c]).u32() & 0xff, 0);
        let d = object(&mut e, &[Some("aB"), None], 1);
        assert_eq!(e.call(0x00a9_3680, &args![a, d]).u32() & 0xff, 0);
        let f = object(&mut e, &[None, None], 1);
        assert_eq!(e.call(0x00a9_3680, &args![f, a]).u32() & 0xff, 0);
        e.mem.set_u32(f + VALUES, 0);
        assert_eq!(e.call(0x00a9_3680, &args![a, f]).u32() & 0xff, 0);
    }

    #[test]
    fn destructor_body_frees_each_string_then_the_array() {
        let mut e = Engine::new();
        e.register(NI_FREE, |_, _| Ret::default());
        e.register(EXTRA_DATA_DESTRUCT_00A7B300, |_, _| Ret::default());
        let this = object(&mut e, &[Some("a")], 0);
        let values = e.mem.u32(this + VALUES);
        let first = e.mem.u32(values);
        e.call_log = Some(vec![]);
        e.call(0x00a9_3750, &args![this]);
        assert_eq!(calls_to(&e, NI_FREE), vec![vec![first], vec![values]]);
        assert_eq!(e.mem.u32(this), NI_SWITCH_STRING_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + VALUES), 0);
        assert_eq!(calls_to(&e, EXTRA_DATA_DESTRUCT_00A7B300), vec![vec![this]]);
    }

    fn allocator_doubles(e: &mut Engine) {
        e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(STRCPY_S, |e, a| {
            let bytes = e.mem.bytes(a[2], a[1]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(EXTRA_DATA_COPY_MEMBERS_00A7B330, |_, _| Ret::default());
        e.register(EXTRA_DATA_CONSTRUCT_00A7B2E0, |_, _| Ret::default());
    }

    #[test]
    fn member_copy_keeps_an_index_inside_the_values_only() {
        let mut e = Engine::new();
        allocator_doubles(&mut e);
        let this = object(&mut e, &[Some("one"), Some("two")], 1);
        let dest = e.mem.alloc(0x18);
        e.call(0x00a9_37a0, &args![this, dest, 0u32]);
        assert_eq!(e.mem.u32(dest + INDEX), 1);
        let values = e.mem.u32(dest + VALUES);
        assert_eq!(c_string(&e, e.mem.u32(values + 4)), "two");
        for bad in [2u32, u32::MAX, 0x8000_0000] {
            e.mem.set_u32(this + INDEX, bad);
            e.call(0x00a9_37a0, &args![this, dest, 0u32]);
            assert_eq!(e.mem.u32(dest + INDEX), u32::MAX);
        }
        // Without values: no size, no array, the index left alone.
        e.mem.set_u32(dest + INDEX, 7);
        e.mem.set_u32(this + VALUES, 0);
        e.call(0x00a9_37a0, &args![this, dest, 0u32]);
        assert_eq!(
            (
                e.mem.u32(dest + SIZE),
                e.mem.u32(dest + VALUES),
                e.mem.u32(dest + INDEX)
            ),
            (0, 0, 7)
        );
    }

    #[test]
    fn create_object_and_clone_start_with_index_minus_one() {
        let mut e = Engine::new();
        allocator_doubles(&mut e);
        e.call_log = Some(vec![]);
        let made = e.call(0x00a9_3870, &args![]).u32();
        assert_eq!(e.mem.u32(made), NI_SWITCH_STRING_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u32(made + INDEX), u32::MAX);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x18]]);
        let this = object(&mut e, &[Some("q")], 0);
        let clone = e.call(0x00a9_38d0, &args![this, 3u32]).u32();
        assert_eq!(e.mem.u32(clone + INDEX), 0);
        assert_eq!(c_string(&e, e.mem.u32(e.mem.u32(clone + VALUES))), "q");
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_3870, &args![]).u32(), 0);
        // The copy into a null destination writes at address 0 + its offsets.
        e.map(0, 0x18);
        e.mem.set_u32(this + VALUES, 0);
        assert_eq!(e.call(0x00a9_38d0, &args![this, 3u32]).u32(), 0);
    }

    /// The stream double's transfer function: a read of a zero word gives 2.
    const TRANSFER: u32 = 0x0070_2000;

    fn stream(e: &mut Engine) -> u32 {
        e.register(TRANSFER, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            if e.mem.u32(a[1]) == 0 {
                e.mem.set_u32(a[1], 2);
            }
            4u32.into_ret()
        });
        let stream = e.mem.alloc(0x5c4);
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + 8, TRANSFER);
        e.mem.set_u32(binary + 0xc, TRANSFER);
        e.mem.set_u32(stream + 0x24c, binary);
        e.mem.set_u32(stream + 0x250, binary);
        stream
    }

    #[test]
    fn save_binary_writes_size_strings_and_index() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_SAVE_BINARY, |_, _| Ret::default());
        e.register(STREAM_SAVE_STRING_00A64160, |_, _| Ret::default());
        let stream = stream(&mut e);
        let this = object(&mut e, &[Some("a"), None], 1);
        let first = e.mem.u32(e.mem.u32(this + VALUES));
        e.call_log = Some(vec![]);
        e.call(0x00a9_39d0, &args![this, stream]);
        let fields: Vec<u32> = calls_to(&e, TRANSFER).iter().map(|w| w[1]).collect();
        assert_eq!(fields, vec![this + SIZE, this + INDEX]);
        assert_eq!(
            calls_to(&e, STREAM_SAVE_STRING_00A64160),
            vec![vec![stream, first], vec![stream, 0]]
        );
        let binary = e.mem.u32(stream + 0x250);
        assert_eq!(e.mem.u32(binary + 4), 8);
    }

    #[test]
    fn load_binary_reads_size_strings_and_index() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_LOAD_BINARY, |_, _| Ret::default());
        e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(STREAM_LOAD_C_STRING, |e, a| {
            e.mem.set_u32(a[1], 0x0abc_0000);
            Ret::default()
        });
        let stream = stream(&mut e);
        let this = e.mem.alloc(0x18);
        e.call(0x00a9_3930, &args![this, stream]);
        assert_eq!(e.mem.u32(this + SIZE), 2);
        assert_eq!(e.mem.u32(this + INDEX), 2);
        let values = e.mem.u32(this + VALUES);
        assert_eq!(e.mem.u32(values + 4), 0x0abc_0000);
        // A size of 0: no array, the index still read.
        let empty = e.mem.alloc(0x18);
        e.register_double(TRANSFER, move |e, a| {
            e.mem
                .set_u32(a[1], if a[1] == empty + SIZE { 0 } else { 5 });
            4u32.into_ret()
        });
        e.mem.set_u32(empty + VALUES, 0x1234);
        e.call(0x00a9_3930, &args![empty, stream]);
        assert_eq!(e.mem.u32(empty + VALUES), 0);
        assert_eq!(e.mem.u32(empty + INDEX), 5);
    }

    #[test]
    fn viewer_strings_end_with_the_index() {
        let mut e = Engine::new();
        e.map(NI_SWITCH_STRING_EXTRA_DATA_RTTI, 4);
        e.set_global(NI_SWITCH_STRING_EXTRA_DATA_RTTI, 0x0104_0000u32);
        e.register(EXTRA_DATA_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING_NAME, |_, a| (a[0] + 1).into_ret());
        e.register(VIEWER_STRING_UNSIGNED, |_, a| {
            (0x0500_0000 + a[1]).into_ret()
        });
        e.register(VIEWER_STRING_STRING, |_, a| a[1].into_ret());
        e.register(VIEWER_STRING_SIGNED, |_, a| {
            assert_eq!(a[0], TEXT_INDEX);
            (0x0600_0000 + (a[1] & 0xff)).into_ret()
        });
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(strings + 4, base);
        e.mem.set_u16(strings + 0xe, 1);
        let this = object(&mut e, &[Some("v")], u32::MAX);
        let first = e.mem.u32(e.mem.u32(this + VALUES));
        e.call(0x00a9_3a60, &args![this, strings]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(base + 4 * i)).collect();
        assert_eq!(words, vec![0x0104_0001, 0x0500_0001, first, 0x0600_00ff]);
        assert_eq!(e.mem.u16(strings + 0xa), 4);
        assert_eq!(e.mem.u16(strings + 0xc), 2);
    }
}
