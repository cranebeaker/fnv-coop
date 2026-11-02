//! `NiMain/nibinaryextradata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a7b290`: the jump target of `00a89ae0`.
const JUMP_TARGET_00A89AE0: u32 = 0x00a7_b290;

// Translated from 00a89ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBinaryExtraData::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a7b290` (engine map: `NiExtraData::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiBinaryExtraData@@UAA_NAAVNiStream@@@Z`.
pub fn ni_binary_extra_data_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A89AE0, &args![this, stream]).bool()
}

/// `NiBinaryExtraData`'s vtable (its RTTI names `NiBinaryExtraData`).
const BINARY_EXTRA_DATA_VTABLE: u32 = 0x0102_e394;
/// `00aa13e0` (`cdecl`, the size): the object allocation; `00aa1070` (`cdecl`, the size): the
/// data block allocation; `_memcpy` (`00ec44d0`, `cdecl`).
const NI_NEW: u32 = 0x00aa_13e0;
const ALLOCATE_BLOCK: u32 = 0x00aa_1070;
const MEMCPY: u32 = 0x00ec_44d0;
/// `NiExtraData`'s constructor (`00a7b2e0`, `thiscall`), member copy (`00a7b330`, `thiscall`,
/// (clone, cloning process)), `LoadBinary` (`00a7b4a0`), `SaveBinary` (`00a7b2c0`) and
/// `GetViewerStrings` (`00a7b580`).
const EXTRA_DATA_CONSTRUCT: u32 = 0x00a7_b2e0;
const EXTRA_DATA_COPY_MEMBERS: u32 = 0x00a7_b330;
const EXTRA_DATA_LOAD_BINARY: u32 = 0x00a7_b4a0;
const EXTRA_DATA_SAVE_BINARY: u32 = 0x00a7_b2c0;
const EXTRA_DATA_GET_VIEWER_STRINGS: u32 = 0x00a7_b580;
/// `NiGetViewerString` of a class name (`00a7d330`) and of an unsigned int (`00a7d510`,
/// `cdecl`, name and value).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const GET_VIEWER_STRING_UINT: u32 = 0x00a7_d510;
/// `0096ad30` grows an `NiTArray`; `0096ae90` stores an element (both `thiscall`).
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The first word of `NiBinaryExtraData`'s `NiRTTI` and the member name `0109df4c`.
const BINARY_EXTRA_DATA_RTTI_NAME: u32 = 0x011f_4ab8;
const NAME_SIZE: u32 = 0x0109_df4c;

// The data's bytes are at +0xc (`m_pcBinaryData`, Xbox PDB name unconfirmed here) and their
// count at +0x10, as every function below uses them.

// Translated from 00a7f870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBinaryExtraData::IsEqual` (Xbox PDB): false for a null other; otherwise whether both
/// have the same byte count (+0x10) and the same bytes (+0xc), compared four bytes at a time
/// and the last one to three bytes one by one (the base class is not compared).
pub fn ni_binary_extra_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if other.addr() == 0 {
        return false;
    }
    let mut size = e.mem.u32(this.addr() + 0x10);
    if size != e.mem.u32(other.addr() + 0x10) {
        return false;
    }
    let mut theirs = e.mem.u32(other.addr() + 0xc);
    let mut ours = e.mem.u32(this.addr() + 0xc);
    while size >= 4 {
        if e.mem.u32(ours) != e.mem.u32(theirs) {
            return false;
        }
        size -= 4;
        theirs = theirs.wrapping_add(4);
        ours = ours.wrapping_add(4);
    }
    for index in 0..size {
        if e.mem.u8(theirs.wrapping_add(index)) != e.mem.u8(ours.wrapping_add(index)) {
            return false;
        }
    }
    true
}

// Translated from 00a7f8e0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; no arguments): by its body the
/// creation function of `NiBinaryExtraData`: a 0x14-byte object from `00aa13e0`, constructed by
/// `NiExtraData`'s constructor (`00a7b2e0`), given the vtable `0102e394`, no bytes and a count
/// of 0; null when the allocation fails.
pub fn fn_00a7f8e0(e: &mut Engine) -> Ptr {
    let block = e.call(NI_NEW, &args![0x14u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(EXTRA_DATA_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, BINARY_EXTRA_DATA_VTABLE);
    e.mem.set_u32(block + 0x10, 0);
    e.mem.set_u32(block + 0xc, 0);
    Ptr::new(block)
}

// Translated from 00a7f920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBinaryExtraData::CreateClone` (Xbox PDB): a new object as [`fn_00a7f8e0`] makes it (null
/// when the allocation fails; the stores below still go to it), the base's member copy
/// (`00a7b330`), then for a byte count above 0 a copy of the bytes in a new block
/// (`00aa1070`, `memcpy`) and the count; for no bytes the clone keeps none.
pub fn ni_binary_extra_data_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    let clone = fn_00a7f8e0(e).addr();
    e.call(
        EXTRA_DATA_COPY_MEMBERS,
        &args![this, clone, cloning_process],
    );
    let size = e.mem.u32(this.addr() + 0x10);
    if size == 0 {
        e.mem.set_u32(clone.wrapping_add(0x10), 0);
        e.mem.set_u32(clone.wrapping_add(0xc), 0);
        return Ptr::new(clone);
    }
    let bytes = e.call(ALLOCATE_BLOCK, &args![size]).u32();
    e.mem.set_u32(clone.wrapping_add(0xc), bytes);
    let size = e.mem.u32(this.addr() + 0x10);
    let source = e.mem.u32(this.addr() + 0xc);
    e.call(MEMCPY, &args![bytes, source, size]);
    let size = e.mem.u32(this.addr() + 0x10);
    e.mem.set_u32(clone.wrapping_add(0x10), size);
    Ptr::new(clone)
}

/// One transfer through the stream's input (`+0x24c`, read function at +8) or output
/// (`+0x250`, write function at +0xc): `function(binary, address, size, &count, 1)` with the
/// count word at `count` set to `count_value` first; the answer is added to the position (+4).
fn transfer(
    e: &mut Engine,
    stream: Ptr,
    write: bool,
    address: u32,
    size: u32,
    count: u32,
    count_value: u32,
) {
    let binary = if write {
        e.mem.u32(stream.addr() + 0x250)
    } else {
        e.mem.u32(stream.addr() + 0x24c)
    };
    let function = e.mem.u32(binary + if write { 0xc } else { 8 });
    e.mem.set_u32(count, count_value);
    let done = e
        .call(function, &args![binary, address, size, count, 1u32])
        .u32();
    let position = e.mem.u32(binary + 4).wrapping_add(done);
    e.mem.set_u32(binary + 4, position);
}

// Translated from 00a7f9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBinaryExtraData::LoadBinary` (Xbox PDB): `NiExtraData::LoadBinary` (`00a7b4a0`), the
/// 4-byte count into +0x10, then for a count above 0 a block of that many bytes (`00aa1070`)
/// at +0xc read as `count` bytes of size 1. The stream argument's stack word is the
/// byte-count word (4, then 1).
pub fn ni_binary_extra_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(EXTRA_DATA_LOAD_BINARY, &args![this, stream]);
    e.with_stack(4, |e, count| {
        let size_at = this.addr() + 0x10;
        transfer(e, stream, false, size_at, 4, count.addr(), 4);
        let size = e.mem.u32(size_at);
        if size == 0 {
            return;
        }
        let bytes = e.call(ALLOCATE_BLOCK, &args![size]).u32();
        let size = e.mem.u32(size_at);
        e.mem.set_u32(this.addr() + 0xc, bytes);
        transfer(e, stream, false, bytes, size, count.addr(), 1);
    });
}

// Translated from 00a7fa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBinaryExtraData::SaveBinary` (Xbox PDB): `NiExtraData::SaveBinary` (`00a7b2c0`), the
/// 4-byte count (+0x10), then for a count above 0 the bytes (+0xc) written as `count` bytes of
/// size 1.
pub fn ni_binary_extra_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(EXTRA_DATA_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, count| {
        let size_at = this.addr() + 0x10;
        transfer(e, stream, true, size_at, 4, count.addr(), 4);
        let size = e.mem.u32(size_at);
        if size == 0 {
            return;
        }
        let bytes = e.mem.u32(this.addr() + 0xc);
        transfer(e, stream, true, bytes, size, count.addr(), 1);
    });
}

/// Appends the string pointer held in the word at `slot` to the viewer-string array (grown to
/// count + `m_usGrowBy` by `0096ad30` when full, stored by `0096ae90`).
fn append_viewer_string(e: &mut Engine, strings: Ptr, slot: u32) {
    let count = e.mem.u16(strings.addr() + 0xa) as u32;
    let capacity = e.mem.u16(strings.addr() + 8) as u32;
    if count >= capacity {
        let grow_by = e.mem.u16(strings.addr() + 0xe) as u32;
        e.call(ARRAY_SET_SIZE, &args![strings, grow_by + count]);
    }
    e.call(ARRAY_SET_AT, &args![strings, count, slot]);
}

// Translated from 00a7fa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBinaryExtraData::GetViewerStrings` (Xbox PDB): `NiExtraData::GetViewerStrings`
/// (`00a7b580`), then the class name (first word of the `NiRTTI` at `011f4ab8`) and the byte
/// count (+0x10) under the name at `0109df4c` (`00a7d510`), each kept in the `strings`
/// argument's stack word and appended.
pub fn ni_binary_extra_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(EXTRA_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let class_name = e.global::<u32>(BINARY_EXTRA_DATA_RTTI_NAME);
        let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(slot.addr(), text);
        append_viewer_string(e, strings, slot.addr());
        let size = e.mem.u32(this.addr() + 0x10);
        let text = e
            .call(GET_VIEWER_STRING_UINT, &args![NAME_SIZE, size])
            .u32();
        e.mem.set_u32(slot.addr(), text);
        append_viewer_string(e, strings, slot.addr());
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_9ae0, ni_binary_extra_data_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a7_f870, ni_binary_extra_data_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a7_f8e0, fn_00a7f8e0() -> Ptr),
        entry!(0x00a7_f920, ni_binary_extra_data_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a7_f9a0, ni_binary_extra_data_load_binary(Ptr, Ptr)),
        entry!(0x00a7_fa20, ni_binary_extra_data_save_binary(Ptr, Ptr)),
        entry!(
            0x00a7_fa90,
            ni_binary_extra_data_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00a89ae0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a8_9ae0;
        const TARGET: u32 = 0x00a7_b290;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }
}

#[cfg(test)]
mod n015_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::calls_to;

    fn data(e: &mut Engine, bytes: &[u8]) -> u32 {
        let object = e.mem.alloc(0x14);
        let block = e.mem.alloc(bytes.len() as u32 + 1);
        e.mem.write(block, bytes);
        e.mem.set_u32(object + 0xc, block);
        e.mem.set_u32(object + 0x10, bytes.len() as u32);
        object
    }

    #[test]
    fn is_equal_compares_count_and_bytes() {
        let mut e = Engine::new();
        let this = data(&mut e, &[1, 2, 3, 4, 5, 6, 7]);
        let same = data(&mut e, &[1, 2, 3, 4, 5, 6, 7]);
        assert!(e.call(0x00a7_f870, &args![this, same]).bool());
        for index in [1usize, 4, 6] {
            let mut bytes = [1, 2, 3, 4, 5, 6, 7];
            bytes[index] = 9;
            let other = data(&mut e, &bytes);
            assert!(!e.call(0x00a7_f870, &args![this, other]).bool(), "{index}");
        }
        let shorter = data(&mut e, &[1, 2, 3, 4, 5, 6]);
        assert!(!e.call(0x00a7_f870, &args![this, shorter]).bool());
        assert!(!e.call(0x00a7_f870, &args![this, 0u32]).bool());
        let empty = data(&mut e, &[]);
        let also_empty = data(&mut e, &[]);
        assert!(e.call(0x00a7_f870, &args![empty, also_empty]).bool());
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ALLOCATE_BLOCK, |e, a| e.mem.alloc(a[0]).into_ret());
        for double in [
            EXTRA_DATA_CONSTRUCT,
            EXTRA_DATA_COPY_MEMBERS,
            EXTRA_DATA_LOAD_BINARY,
            EXTRA_DATA_SAVE_BINARY,
            EXTRA_DATA_GET_VIEWER_STRINGS,
            MEMCPY,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn creation_function_builds_an_empty_object() {
        let mut e = engine();
        let object = e.call(0x00a7_f8e0, &args![]).u32();
        assert_eq!(e.mem.u32(object), BINARY_EXTRA_DATA_VTABLE);
        assert_eq!((e.mem.u32(object + 0xc), e.mem.u32(object + 0x10)), (0, 0));
        assert_eq!(calls_to(&e, EXTRA_DATA_CONSTRUCT), vec![vec![object]]);
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a7_f8e0, &args![]).u32(), 0);
    }

    #[test]
    fn create_clone_copies_the_bytes() {
        let mut e = engine();
        let this = data(&mut e, &[1, 2, 3]);
        let clone = e.call(0x00a7_f920, &args![this, 0x66u32]).u32();
        assert_eq!(
            calls_to(&e, EXTRA_DATA_COPY_MEMBERS),
            vec![vec![this, clone, 0x66]]
        );
        let bytes = e.mem.u32(clone + 0xc);
        assert_eq!(calls_to(&e, ALLOCATE_BLOCK), vec![vec![3]]);
        assert_eq!(
            calls_to(&e, MEMCPY),
            vec![vec![bytes, e.mem.u32(this + 0xc), 3]]
        );
        assert_eq!(e.mem.u32(clone + 0x10), 3);
        let empty = data(&mut e, &[]);
        let clone = e.call(0x00a7_f920, &args![empty, 0x66u32]).u32();
        assert_eq!((e.mem.u32(clone + 0xc), e.mem.u32(clone + 0x10)), (0, 0));
        assert_eq!(calls_to(&e, ALLOCATE_BLOCK).len(), 1);
    }

    /// A stream whose input and output answer the size asked and record the count words.
    fn stream(e: &mut Engine) -> (u32, u32) {
        let stream = e.mem.alloc(0x300);
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, binary);
        e.mem.set_u32(stream + 0x250, binary);
        e.mem.set_u32(binary + 8, 0x0300_0000);
        e.mem.set_u32(binary + 0xc, 0x0300_0004);
        e.register(0x0300_0000, |e, a| {
            if a[2] == 4 {
                e.mem.set_u32(a[1], 5);
            }
            let count = e.mem.u32(a[3]);
            e.mem.set_u32(0x0300_1000, count);
            a[2].into_ret()
        });
        e.register(0x0300_0004, |e, a| {
            let count = e.mem.u32(a[3]);
            e.mem.set_u32(0x0300_1004, count);
            a[2].into_ret()
        });
        e.map(0x0300_1000, 8);
        (stream, binary)
    }

    #[test]
    fn load_binary_reads_count_then_bytes() {
        let mut e = engine();
        let (stream, binary) = stream(&mut e);
        let this = e.mem.alloc(0x14);
        e.call(0x00a7_f9a0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(e.mem.u32(this + 0x10), 5);
        let reads = calls_to(&e, 0x0300_0000);
        assert_eq!(reads.len(), 2);
        assert_eq!(
            (reads[1][1], reads[1][2], reads[1][4]),
            (e.mem.u32(this + 0xc), 5, 1)
        );
        assert_eq!(e.mem.u32(0x0300_1000), 1);
        assert_eq!(e.mem.u32(binary + 4), 9);
    }

    #[test]
    fn save_binary_writes_count_then_bytes() {
        let mut e = engine();
        let (stream, binary) = stream(&mut e);
        let this = data(&mut e, &[1, 2]);
        e.call(0x00a7_fa20, &args![this, stream]);
        let writes = calls_to(&e, 0x0300_0004);
        assert_eq!(writes.len(), 2);
        assert_eq!((writes[0][1], writes[0][2]), (this + 0x10, 4));
        assert_eq!((writes[1][1], writes[1][2]), (e.mem.u32(this + 0xc), 2));
        assert_eq!(e.mem.u32(binary + 4), 6);
        let empty = data(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x00a7_fa20, &args![empty, stream]);
        assert_eq!(calls_to(&e, 0x0300_0004).len(), 1);
    }

    #[test]
    fn viewer_strings_add_class_and_size() {
        let mut e = engine();
        e.map(0x011f_4000, 0x1000);
        e.mem.set_u32(BINARY_EXTRA_DATA_RTTI_NAME, 0x0102_0000);
        e.register(GET_VIEWER_STRING, |_, _| 0x7000u32.into_ret());
        e.register(GET_VIEWER_STRING_UINT, |_, _| 0x8000u32.into_ret());
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(ARRAY_SET_AT, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(0x011f_4f00 + 4 * a[1], value);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        let this = data(&mut e, &[1, 2, 3]);
        e.call(0x00a7_fa90, &args![this, strings]);
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x0102_0000]]);
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING_UINT),
            vec![vec![NAME_SIZE, 3]]
        );
        assert!(calls_to(&e, ARRAY_SET_SIZE).is_empty());
        assert_eq!(calls_to(&e, ARRAY_SET_AT).len(), 2);
        assert_eq!(e.mem.u32(0x011f_4f00), 0x8000);
    }
}
