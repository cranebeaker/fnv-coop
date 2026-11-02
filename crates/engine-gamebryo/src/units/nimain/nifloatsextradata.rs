//! `NiMain/nifloatsextradata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

/// `NiFloatsExtraData::ms_RTTI` (the class's `NiRTTI` record).
const NI_FLOATS_EXTRA_DATA_RTTI: u32 = 0x011f_5b60;

// Translated from 00a89120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_floats_extra_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_FLOATS_EXTRA_DATA_RTTI)
}

/// `NiFloatsExtraData`'s vtable.
const NI_FLOATS_EXTRA_DATA_VTABLE: u32 = 0x0109_ec0c;
/// `operator new` (`cdecl`, the size) and the sized delete (`cdecl`, (block, size)).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// The array allocation and free (`00aa1070` and `00aa10f0`, `cdecl`, one word; unnamed in
/// the engine map).
const ALLOCATE_ARRAY: u32 = 0x00aa_1070;
const FREE_ARRAY: u32 = 0x00aa_10f0;
/// `NiExtraData`'s constructor and destructor body (`thiscall`, no stack words), its member
/// copy (`00a7b330`, (clone, cloning process)), `SaveBinary` (`00a7b2c0`, the stream) and
/// `GetViewerStrings` (`00a7b580`, the string array).
const EXTRA_DATA_CONSTRUCT: u32 = 0x00a7_b2e0;
const EXTRA_DATA_DESTRUCT: u32 = 0x00a7_b300;
const EXTRA_DATA_COPY_MEMBERS: u32 = 0x00a7_b330;
const EXTRA_DATA_SAVE_BINARY: u32 = 0x00a7_b2c0;
const EXTRA_DATA_GET_VIEWER_STRINGS: u32 = 0x00a7_b580;
/// `NiGetViewerString` (`00a7d330`, the class name), `NiGetViewerString_ov8` (`00a7d510`, a
/// name and an unsigned word) and `NiGetViewerString_ov9` (`00a7d550`, a name and a float),
/// all `cdecl`, answering a new string.
const VIEWER_STRING_CLASS: u32 = 0x00a7_d330;
const VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
/// The string array's grow (`0096ad30`, `thiscall`, the new size) and set-at (`0096ae90`,
/// `thiscall`, (index, address of the string)), both unnamed in the engine map.
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The literals `"m_uiSize"` and `"m_pfValue[i]"`.
const TEXT_M_UI_SIZE: u32 = 0x0109_df4c;
const TEXT_M_PF_VALUE_I: u32 = 0x0109_eca0;

/// The number of floats (`m_uiSize`, Xbox PDB) at +0xc and the floats (`m_pfValue`) at +0x10.
const SIZE: u32 = 0xc;
const VALUES: u32 = 0x10;

/// Appends `text` to a viewer-string array as the exe's inlined `Add` does: when the 16-bit
/// size (+0xa) has reached the capacity (+0x8) the array grows to its step (+0xe) plus the
/// size, then the string is stored at the old size through `0096ae90` (a stack word holding
/// it).
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

/// The same append with the set-at inlined (`NiTArray::SetAt`): after the grow the size is
/// read again; storing at or past it makes the size the index plus one and counts a
/// non-null string in the 16-bit used count (+0xc); storing below it adjusts the used count
/// by the change between the old element and `text` (one more for a string over a null,
/// one less for a null over a string). The string goes into the elements (+0x4) at the old
/// size.
fn push_viewer_string_inline(e: &mut Engine, strings: Ptr, text: u32) {
    let base = strings.addr();
    let index = u32::from(e.mem.u16(base + 0xa));
    if index >= u32::from(e.mem.u16(base + 8)) {
        let step = u32::from(e.mem.u16(base + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + index]);
    }
    let size = u32::from(e.mem.u16(base + 0xa));
    if index >= size {
        e.mem.set_u16(base + 0xa, (index + 1) as u16);
        if text != 0 {
            let used = e.mem.u16(base + 0xc);
            e.mem.set_u16(base + 0xc, used.wrapping_add(1));
        }
    } else {
        let old = e.mem.u32(e.mem.u32(base + 4).wrapping_add(4 * index));
        if text != 0 {
            if old == 0 {
                let used = e.mem.u16(base + 0xc);
                e.mem.set_u16(base + 0xc, used.wrapping_add(1));
            }
        } else if old != 0 {
            let used = e.mem.u16(base + 0xc);
            e.mem.set_u16(base + 0xc, used.wrapping_add(0xffff));
        }
    }
    let elements = e.mem.u32(base + 4);
    e.mem.set_u32(elements.wrapping_add(4 * index), text);
}

/// Allocates and builds an empty floats extra data (0x14 bytes, `operator new`, the
/// `NiExtraData` constructor, the vtable `0109ec0c`, no floats); null when the allocation
/// fails.
fn new_floats_extra_data(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(EXTRA_DATA_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, NI_FLOATS_EXTRA_DATA_VTABLE);
    e.mem.set_u32(block + VALUES, 0);
    e.mem.set_u32(block + SIZE, 0);
    block
}

// Translated from 00a89000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraData::SetValue` (Xbox PDB): for an index below the count (+0xc), stores
/// the float (`FLD`/`FSTP`) at that index of the floats (+0x10) and answers true; false
/// otherwise.
pub fn ni_floats_extra_data_set_value(e: &mut Engine, this: Ptr, index: u32, value: f32) -> bool {
    if index >= e.mem.u32(this.addr() + SIZE) {
        return false;
    }
    let values = e.mem.u32(this.addr() + VALUES);
    e.mem.set_u32(
        values.wrapping_add(index.wrapping_mul(4)),
        x87_quiet_bits(value.to_bits()),
    );
    true
}

// Translated from 00a89020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraData::GetValue` (Xbox PDB): the float at the index (in `ST0`) when the
/// index is below the count (+0xc), 0 otherwise.
pub fn ni_floats_extra_data_get_value(e: &mut Engine, this: Ptr, index: u32) -> f32 {
    if index >= e.mem.u32(this.addr() + SIZE) {
        return 0.0;
    }
    let values = e.mem.u32(this.addr() + VALUES);
    f32::from_bits(x87_quiet_bits(
        e.mem.u32(values.wrapping_add(index.wrapping_mul(4))),
    ))
}

// Translated from 00a89040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its body `NiFloatsExtraData`'s member copy (`thiscall`:
/// the clone and the cloning process). `NiExtraData`'s member copy (`00a7b330`), then: with
/// no floats (null pointer or a count of 0) the clone's floats and count are cleared;
/// otherwise the clone takes the count, a new array of count * 4 bytes (`00aa1070`) and a
/// copy of each float (`FLD`/`FSTP`).
pub fn fn_00a89040(e: &mut Engine, this: Ptr, clone: Ptr, cloning_process: Ptr) {
    e.call(
        EXTRA_DATA_COPY_MEMBERS,
        &args![this, clone, cloning_process],
    );
    let base = this.addr();
    let clone = clone.addr();
    if e.mem.u32(base + VALUES) == 0 || e.mem.u32(base + SIZE) == 0 {
        e.mem.set_u32(clone.wrapping_add(VALUES), 0);
        e.mem.set_u32(clone.wrapping_add(SIZE), 0);
        return;
    }
    let count = e.mem.u32(base + SIZE);
    e.mem.set_u32(clone.wrapping_add(SIZE), count);
    let size = e.mem.u32(base + SIZE).wrapping_mul(4);
    let values = e.call(ALLOCATE_ARRAY, &args![size]).u32();
    e.mem.set_u32(clone.wrapping_add(VALUES), values);
    let mut index = 0u32;
    while index < e.mem.u32(base + SIZE) {
        let from = e.mem.u32(base + VALUES).wrapping_add(4 * index);
        let value = x87_quiet_bits(e.mem.u32(from));
        let to = e
            .mem
            .u32(clone.wrapping_add(VALUES))
            .wrapping_add(4 * index);
        e.mem.set_u32(to, value);
        index += 1;
    }
}

// Translated from 00a890b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraData::IsEqual` (Xbox PDB): false for a null other object or a different
/// count (+0xc); when exactly one side has no floats pointer, false; when neither has one,
/// true; otherwise true exactly when every float compares equal (`FUCOMPP`, a NaN is not
/// equal). The base class is not consulted.
pub fn ni_floats_extra_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if other.is_null() {
        return false;
    }
    let count = e.mem.u32(this.addr() + SIZE);
    if count != e.mem.u32(other.addr() + SIZE) {
        return false;
    }
    let mine = e.mem.u32(this.addr() + VALUES);
    let theirs = e.mem.u32(other.addr() + VALUES);
    if mine != 0 {
        if theirs == 0 {
            return false;
        }
    } else {
        return theirs == 0;
    }
    for index in 0..count {
        let a = x87_load32(e.mem.u32(mine.wrapping_add(4 * index)));
        let b = x87_load32(e.mem.u32(theirs.wrapping_add(4 * index)));
        if b != a {
            return false;
        }
    }
    true
}

// Translated from 00a89130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraData::CreateClone` (Xbox PDB): a new empty floats extra data (null when
/// the allocation fails) and the member copy (`00a89040`) into it, run on the null clone
/// too, as the exe does. Answers the clone.
pub fn ni_floats_extra_data_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    let clone = new_floats_extra_data(e);
    fn_00a89040(e, this, Ptr::new(clone), cloning_process);
    Ptr::new(clone)
}

// Translated from 00a89190 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiFloatsExtraData`): a new empty floats extra data, or null.
pub fn fn_00a89190(e: &mut Engine) -> Ptr {
    Ptr::new(new_floats_extra_data(e))
}

// Translated from 00a891d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraData::_scalar_deleting_destructor_` (Xbox PDB): stores the vtable
/// `0109ec0c`, frees the floats (`00aa10f0`) and clears the pointer, runs the `NiExtraData`
/// destructor body (`00a7b300`) and, when bit 0 of `flags` is set, the sized delete of 0x14
/// bytes. Answers `this`.
pub fn ni_floats_extra_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    let values = e.mem.u32(this.addr() + VALUES);
    e.mem.set_u32(this.addr(), NI_FLOATS_EXTRA_DATA_VTABLE);
    e.call(FREE_ARRAY, &args![values]);
    e.mem.set_u32(this.addr() + VALUES, 0);
    e.call(EXTRA_DATA_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 00a89210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; by its body `NiFloatsExtraData::SaveBinary` (`thiscall`, the
/// stream): `NiExtraData::SaveBinary`, then through the stream's output (`m_pkOstr`, its
/// write function, count word 4, one item) the count (4 bytes at +0xc) and the floats
/// (count * 4 bytes from +0x10's array), the position advanced by each write's result.
pub fn fn_00a89210(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    e.call(EXTRA_DATA_SAVE_BINARY, &args![this, stream]);
    let base = this.addr();
    write_words(e, stream, base + SIZE, 4);
    let size = e.mem.u32(base + SIZE).wrapping_mul(4);
    let values = e.mem.u32(base + VALUES);
    write_words(e, stream, values, size);
}

/// One write through the stream's output: its write function (`m_pfnWrite`, `cdecl`:
/// (output, data, bytes, address of a count word of 4, 1)), the position advanced by its
/// result.
fn write_words(e: &mut Engine, stream: Ptr<NiStream>, data: u32, bytes: u32) {
    let output = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkOstr));
    let function = e.get(output, NiBinaryStream::m_pfnWrite);
    let done = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 4);
        e.call(function, &args![output, data, bytes, count, 1u32])
            .u32()
    });
    let position = e.get(output, NiBinaryStream::m_uiAbsoluteCurrentPos);
    e.set(
        output,
        NiBinaryStream::m_uiAbsoluteCurrentPos,
        position.wrapping_add(done),
    );
}

// Translated from 00a89280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraData::GetViewerStrings` (Xbox PDB): `NiExtraData::GetViewerStrings`, the
/// class name (`NiGetViewerString` of `ms_RTTI`'s name word), `"m_uiSize"` with the count,
/// then `"m_pfValue[i]"` with each float (the count read again each round), the last ones
/// appended with the array's set-at inlined.
pub fn ni_floats_extra_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(EXTRA_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(NI_FLOATS_EXTRA_DATA_RTTI);
    let text = e.call(VIEWER_STRING_CLASS, &args![name]).u32();
    push_viewer_string(e, strings, text);
    let count = e.mem.u32(this.addr() + SIZE);
    let text = e
        .call(VIEWER_STRING_UNSIGNED, &args![TEXT_M_UI_SIZE, count])
        .u32();
    push_viewer_string(e, strings, text);
    let mut index = 0u32;
    while index < e.mem.u32(this.addr() + SIZE) {
        let values = e.mem.u32(this.addr() + VALUES);
        let value = x87_quiet_bits(e.mem.u32(values.wrapping_add(4 * index)));
        let text = e
            .call(VIEWER_STRING_FLOAT, &args![TEXT_M_PF_VALUE_I, value])
            .u32();
        push_viewer_string_inline(e, strings, text);
        index += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a89120, ni_floats_extra_data_get_rtti() -> Ptr),
        entry!(
            0x00a8_9000,
            ni_floats_extra_data_set_value(Ptr, u32, f32) -> bool
        ),
        entry!(0x00a8_9020, ni_floats_extra_data_get_value(Ptr, u32) -> f32),
        entry!(0x00a8_9040, fn_00a89040(Ptr, Ptr, Ptr)),
        entry!(0x00a8_90b0, ni_floats_extra_data_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a8_9130,
            ni_floats_extra_data_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00a8_9190, fn_00a89190() -> Ptr),
        entry!(
            0x00a8_91d0,
            ni_floats_extra_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a8_9210, fn_00a89210(Ptr, Ptr<NiStream>)),
        entry!(
            0x00a8_9280,
            ni_floats_extra_data_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_floats_extra_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a89120, &args![]).u32(),
            NI_FLOATS_EXTRA_DATA_RTTI
        );
    }
}

#[cfg(test)]
mod n016_tests {
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

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| {
            let block = e.mem.alloc(a[0]);
            e.mem.write(block, &[0xee; 0x14]);
            block.into_ret()
        });
        e.register(ALLOCATE_ARRAY, |e, a| e.mem.alloc(a[0]).into_ret());
        for double in [
            EXTRA_DATA_CONSTRUCT,
            EXTRA_DATA_DESTRUCT,
            EXTRA_DATA_COPY_MEMBERS,
            EXTRA_DATA_SAVE_BINARY,
            EXTRA_DATA_GET_VIEWER_STRINGS,
            SIZED_DELETE,
            FREE_ARRAY,
            STRINGS_GROW,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    /// A floats extra data holding `values`.
    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let this = e.mem.alloc(0x14);
        let array = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(array + 4 * i as u32, *v);
        }
        e.mem.set_u32(this + SIZE, values.len() as u32);
        e.mem.set_u32(this + VALUES, array);
        this
    }

    #[test]
    fn set_value_stores_below_the_count() {
        let mut e = engine();
        let this = floats(&mut e, &[1.0, 2.0]);
        assert!(e
            .call(0x00a8_9000, &args![this, 1u32, 7.5f32.to_bits()])
            .bool());
        assert_eq!(e.mem.f32(e.mem.u32(this + VALUES) + 4), 7.5);
        assert!(!e
            .call(0x00a8_9000, &args![this, 2u32, 7.5f32.to_bits()])
            .bool());
        // A signalling NaN is quieted by the FLD/FSTP copy.
        e.call(0x00a8_9000, &args![this, 0u32, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(e.mem.u32(this + VALUES)), 0x7fc0_0001);
    }

    #[test]
    fn get_value_reads_below_the_count() {
        let mut e = engine();
        let this = floats(&mut e, &[1.0, -2.5]);
        assert_eq!(e.call(0x00a8_9020, &args![this, 1u32]).f32(), -2.5);
        assert_eq!(e.call(0x00a8_9020, &args![this, 2u32]).f32(), 0.0);
    }

    #[test]
    fn copy_members_copies_the_floats() {
        let mut e = engine();
        let this = floats(&mut e, &[1.0, 2.0, 3.0]);
        let clone = e.mem.alloc(0x14);
        e.call(0x00a8_9040, &args![this, clone, 0x77u32]);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_COPY_MEMBERS),
            vec![vec![this, clone, 0x77]]
        );
        assert_eq!(calls_to(&e, ALLOCATE_ARRAY), vec![vec![12]]);
        assert_eq!(e.mem.u32(clone + SIZE), 3);
        let array = e.mem.u32(clone + VALUES);
        assert_ne!(array, e.mem.u32(this + VALUES));
        assert_eq!(e.mem.f32(array + 8), 3.0);
        // No floats: both words cleared, nothing allocated.
        e.mem.set_u32(this + SIZE, 0);
        e.call(0x00a8_9040, &args![this, clone, 0x77u32]);
        assert_eq!((e.mem.u32(clone + SIZE), e.mem.u32(clone + VALUES)), (0, 0));
        e.mem.set_u32(this + SIZE, 3);
        e.mem.set_u32(this + VALUES, 0);
        e.mem.set_u32(clone + SIZE, 9);
        e.call(0x00a8_9040, &args![this, clone, 0x77u32]);
        assert_eq!(e.mem.u32(clone + SIZE), 0);
        assert_eq!(calls_to(&e, ALLOCATE_ARRAY).len(), 1);
    }

    #[test]
    fn is_equal_compares_counts_pointers_and_floats() {
        let mut e = engine();
        let this = floats(&mut e, &[1.0, 2.0]);
        let other = floats(&mut e, &[1.0, 2.0]);
        assert!(e.call(0x00a8_90b0, &args![this, other]).bool());
        assert!(!e.call(0x00a8_90b0, &args![this, 0u32]).bool());
        e.mem.set_f32(e.mem.u32(other + VALUES) + 4, 2.5);
        assert!(!e.call(0x00a8_90b0, &args![this, other]).bool());
        let short = floats(&mut e, &[1.0]);
        assert!(!e.call(0x00a8_90b0, &args![this, short]).bool());
        // One side without floats: not equal; both without: equal.
        let saved = e.mem.u32(other + VALUES);
        e.mem.set_u32(other + VALUES, 0);
        assert!(!e.call(0x00a8_90b0, &args![this, other]).bool());
        assert!(!e.call(0x00a8_90b0, &args![other, this]).bool());
        e.mem.set_u32(this + VALUES, 0);
        assert!(e.call(0x00a8_90b0, &args![this, other]).bool());
        // A NaN is never equal.
        e.mem.set_u32(this + VALUES, saved);
        e.mem.set_u32(other + VALUES, saved);
        e.mem.set_u32(saved, 0x7fc0_0000);
        assert!(!e.call(0x00a8_90b0, &args![this, other]).bool());
    }

    #[test]
    fn create_clone_builds_and_copies() {
        let mut e = engine();
        let this = floats(&mut e, &[4.0]);
        let clone = e.call(0x00a8_9130, &args![this, 0x77u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_FLOATS_EXTRA_DATA_VTABLE);
        assert_eq!(calls_to(&e, EXTRA_DATA_CONSTRUCT), vec![vec![clone]]);
        assert_eq!(e.mem.u32(clone + SIZE), 1);
        assert_eq!(e.mem.f32(e.mem.u32(clone + VALUES)), 4.0);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_COPY_MEMBERS),
            vec![vec![this, clone, 0x77]]
        );
    }

    #[test]
    fn create_object_builds_an_empty_one() {
        let mut e = engine();
        let object = e.call(0x00a8_9190, &args![]).u32();
        assert_eq!(e.mem.u32(object), NI_FLOATS_EXTRA_DATA_VTABLE);
        assert_eq!(
            (e.mem.u32(object + SIZE), e.mem.u32(object + VALUES)),
            (0, 0)
        );
        assert_eq!(e.mem.u32(object + 4), 0xeeee_eeee);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a8_9190, &args![]).u32(), 0);
    }

    #[test]
    fn scalar_deleting_destructor_frees_the_floats() {
        let mut e = engine();
        let this = floats(&mut e, &[4.0]);
        let array = e.mem.u32(this + VALUES);
        assert_eq!(e.call(0x00a8_91d0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_FLOATS_EXTRA_DATA_VTABLE);
        assert_eq!(calls_to(&e, FREE_ARRAY), vec![vec![array]]);
        assert_eq!(e.mem.u32(this + VALUES), 0);
        assert_eq!(calls_to(&e, EXTRA_DATA_DESTRUCT), vec![vec![this]]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0x14]]);
        e.call(0x00a8_91d0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE).len(), 1);
    }

    #[test]
    fn save_binary_writes_the_count_then_the_floats() {
        let mut e = engine();
        let stream = e.new_object::<NiStream>();
        let output = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkOstr, output.addr());
        e.set(output, NiBinaryStream::m_pfnWrite, 0x0300_0000);
        e.register(0x0300_0000, |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[4]), (4, 1));
            a[2].into_ret()
        });
        let this = floats(&mut e, &[1.0, 2.0]);
        e.call(0x00a8_9210, &args![this, stream]);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_SAVE_BINARY),
            vec![vec![this, stream.addr()]]
        );
        let writes: Vec<Vec<u32>> = calls_to(&e, 0x0300_0000)
            .into_iter()
            .map(|w| w[..3].to_vec())
            .collect();
        assert_eq!(
            writes,
            vec![
                vec![output.addr(), this + SIZE, 4],
                vec![output.addr(), e.mem.u32(this + VALUES), 8]
            ]
        );
        assert_eq!(e.get(output, NiBinaryStream::m_uiAbsoluteCurrentPos), 12);
    }

    #[test]
    fn get_viewer_strings_lists_each_float() {
        let mut e = engine();
        e.map(0x011f_5000, 0x1000);
        e.mem.set_u32(NI_FLOATS_EXTRA_DATA_RTTI, 0x0123_0000);
        e.register(VIEWER_STRING_CLASS, |_, _| 0x0e00_0001u32.into_ret());
        e.register(VIEWER_STRING_UNSIGNED, |_, _| 0x0e00_0002u32.into_ret());
        // The first float's string is null, the second's not.
        e.register(VIEWER_STRING_FLOAT, |e, a| {
            if e.mem.u32(0x011f_5f00) == 0 {
                e.mem.set_u32(0x011f_5f00, 1);
                return 0u32.into_ret();
            }
            assert_eq!(a[0], TEXT_M_PF_VALUE_I);
            0x0e00_0003u32.into_ret()
        });
        e.register(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        let this = floats(&mut e, &[0.5, 1.5]);
        // Capacity 4, size 1: no grow; elements 1 and 2 by set-at, 3 and 4 inlined.
        let strings = e.mem.alloc(0x10);
        let elements = e.mem.alloc(0x20);
        e.mem.set_u32(strings + 4, elements);
        e.mem.set_u16(strings + 8, 4);
        e.mem.set_u16(strings + 0xa, 1);
        e.mem.set_u16(strings + 0xc, 1);
        e.mem.set_u16(strings + 0xe, 2);
        e.call(0x00a8_9280, &args![this, strings]);
        assert_eq!(
            calls_to(&e, VIEWER_STRING_UNSIGNED),
            vec![vec![TEXT_M_UI_SIZE, 2]]
        );
        assert_eq!(
            calls_to(&e, VIEWER_STRING_FLOAT),
            vec![
                vec![TEXT_M_PF_VALUE_I, 0.5f32.to_bits()],
                vec![TEXT_M_PF_VALUE_I, 1.5f32.to_bits()]
            ]
        );
        let set: Vec<u32> = calls_to(&e, STRINGS_SET_AT).iter().map(|c| c[1]).collect();
        assert_eq!(set, vec![1, 2]);
        // Index 3 (null, size 4) and index 4 (grows: step 2 + 4).
        assert_eq!(calls_to(&e, STRINGS_GROW), vec![vec![strings, 6]]);
        assert_eq!(e.mem.u16(strings + 0xa), 5);
        assert_eq!(e.mem.u16(strings + 0xc), 2);
        assert_eq!(e.mem.u32(elements + 12), 0);
        assert_eq!(e.mem.u32(elements + 16), 0x0e00_0003);
    }

    #[test]
    fn inline_set_at_counts_strings_below_the_size() {
        let mut e = engine();
        let strings = e.mem.alloc(0x10);
        let elements = e.mem.alloc(0x20);
        e.mem.set_u32(strings + 4, elements);
        e.mem.set_u16(strings + 8, 8);
        e.mem.set_u16(strings + 0xa, 2);
        e.mem.set_u16(strings + 0xc, 5);
        // A grow double that shrinks the size below the index: the store counts against
        // the old element.
        e.register(STRINGS_GROW, |e, a| {
            e.mem.set_u16(a[0] + 0xa, 3);
            Ret::default()
        });
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u32(elements + 8, 0x0e00_0009);
        push_viewer_string_inline(&mut e, Ptr::new(strings), 0);
        assert_eq!(e.mem.u16(strings + 0xc), 4);
        assert_eq!(e.mem.u16(strings + 0xa), 3);
        e.mem.set_u16(strings + 0xa, 2);
        push_viewer_string_inline(&mut e, Ptr::new(strings), 0x0e00_000a);
        assert_eq!(e.mem.u16(strings + 0xc), 5);
        assert_eq!(e.mem.u32(elements + 8), 0x0e00_000a);
    }
}
