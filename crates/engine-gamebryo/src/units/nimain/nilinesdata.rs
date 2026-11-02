//! `NiMain/nilinesdata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiLinesData::ms_RTTI` (the class's `NiRTTI` record).
const NI_LINES_DATA_RTTI: u32 = 0x011f_5b88;

// Translated from 00a8a7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLinesData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_lines_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_LINES_DATA_RTTI)
}

/// `00a677e0`: the jump target of `00a8a9a0`.
const JUMP_TARGET_00A8A9A0: u32 = 0x00a6_77e0;

/// `00a66f50`: the jump target of `00a8a9b0`.
const JUMP_TARGET_00A8A9B0: u32 = 0x00a6_6f50;

// Translated from 00a8a9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLinesData::LinkObject` (Xbox PDB): a 5-byte jump to `00a677e0` (engine map: `NiGeometryData::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiLinesData@@UAAXAAVNiStream@@@Z`.
pub fn ni_lines_data_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A8A9A0, &args![this, stream]);
}

// Translated from 00a8a9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLinesData::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a66f50` (engine map: `NiGeometryData::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiLinesData@@UAA_NAAVNiStream@@@Z`.
pub fn ni_lines_data_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A8A9B0, &args![this, stream]).bool()
}

/// `NiLinesData`'s vtable (`0109f23c`).
const NI_LINES_DATA_VTABLE: u32 = 0x0109_f23c;
/// `NiLinesData`'s fields as the PC code reads them: the vertex count of `NiGeometryData`
/// (+8, 16 bits) and the connection flags `m_pkFlags` (+0x40, one byte per vertex).
const VERTEX_COUNT: u32 = 0x8;
const FLAGS: u32 = 0x40;
/// `NiGeometryData` functions (`thiscall`): `IsEqual` (`00a67840`), the destructor body
/// (`00a67c70`), `LoadBinary` (`00a67ea0`), `SaveBinary` (`00a68520`) and
/// `GetViewerStrings` (`00a688b0`).
const GEOMETRY_DATA_IS_EQUAL: u32 = 0x00a6_7840;
const GEOMETRY_DATA_DESTRUCT: u32 = 0x00a6_7c70;
const GEOMETRY_DATA_LOAD_BINARY: u32 = 0x00a6_7ea0;
const GEOMETRY_DATA_SAVE_BINARY: u32 = 0x00a6_8520;
const GEOMETRY_DATA_GET_VIEWER_STRINGS: u32 = 0x00a6_88b0;
/// The Ni allocator (`00aa1070`) and free (`00aa10f0`), `cdecl`, and the sized delete
/// (`00aa1460`, `cdecl`: pointer, size).
const NI_ALLOC: u32 = 0x00aa_1070;
const NI_FREE: u32 = 0x00aa_10f0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// Viewer string builders (`cdecl`): `NiGetViewerString` (`00a7d330`, the class name) and
/// `NiGetViewerString_ov10` (`00a7d5a0`, a label and a pointer).
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_POINTER: u32 = 0x00a7_d5a0;
/// `"m_pkFlags"` (`0109f2e0`).
const TEXT_FLAGS: u32 = 0x0109_f2e0;
/// `NiTArray<char *>` resize (`0096ad30`) and `SetAt` (`0096ae90`).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;

// Translated from 00a8a730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLinesData::IsEqual` (Xbox PDB): `NiGeometryData::IsEqual` (`00a67840`), both flag
/// arrays null or both present, and when present the same byte for each of this object's
/// vertices.
pub fn ni_lines_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(GEOMETRY_DATA_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let mine = e.mem.u32(this.addr() + FLAGS);
    let theirs = e.mem.u32(other.addr() + FLAGS);
    if (mine != 0) != (theirs != 0) {
        return false;
    }
    if mine != 0 {
        let count = e.mem.u16(this.addr() + VERTEX_COUNT);
        let mut index: u16 = 0;
        while index < count {
            let at = u32::from(index);
            if e.mem.u8(mine.wrapping_add(at)) != e.mem.u8(theirs.wrapping_add(at)) {
                return false;
            }
            index = index.wrapping_add(1);
        }
    }
    true
}

// Translated from 00a8a7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLinesData::_scalar_deleting_destructor_` (Xbox PDB): the vtable `0109f23c`, the flag
/// array freed (`00aa10f0`), the `NiGeometryData` destructor body (`00a67c70`), then the
/// sized delete of 0x44 bytes when bit 0 of `flags` is set. Answers `this`.
pub fn ni_lines_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let array = e.mem.u32(this.addr() + FLAGS);
    e.mem.set_u32(this.addr(), NI_LINES_DATA_VTABLE);
    e.call(NI_FREE, &args![array]);
    e.call(GEOMETRY_DATA_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x44u32]);
    }
    this
}

// Translated from 00a8a800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLinesData::LoadBinary` (Xbox PDB): `NiGeometryData::LoadBinary` (`00a67ea0`), then a
/// flag array of one byte per vertex allocated (`00aa1070`), stored at +0x40 and read from
/// the stream's input (`NiStream` +0x24c, read function +8, position +4) with a component
/// size of 1 (the stream argument's stack word holds it).
pub fn ni_lines_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(GEOMETRY_DATA_LOAD_BINARY, &args![this, stream]);
    let count = u32::from(e.mem.u16(this.addr() + VERTEX_COUNT));
    let array = e.call(NI_ALLOC, &args![count]).u32();
    let count = u32::from(e.mem.u16(this.addr() + VERTEX_COUNT));
    e.mem.set_u32(this.addr() + FLAGS, array);
    let input = e.mem.u32(stream.addr() + 0x24c);
    let function = e.mem.u32(input + 8);
    e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 1);
        let done = e
            .call(function, &args![input, array, count, size, 1u32])
            .u32();
        let position = e.mem.u32(input + 4).wrapping_add(done);
        e.mem.set_u32(input + 4, position);
    });
}

// Translated from 00a8a850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLinesData::SaveBinary` (Xbox PDB): `NiGeometryData::SaveBinary` (`00a68520`), then the
/// flag array, one byte per vertex, to the stream's output (`NiStream` +0x250, write function
/// +0xc, position +4) with a component size of 1.
pub fn ni_lines_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(GEOMETRY_DATA_SAVE_BINARY, &args![this, stream]);
    let count = u32::from(e.mem.u16(this.addr() + VERTEX_COUNT));
    let array = e.mem.u32(this.addr() + FLAGS);
    let output = e.mem.u32(stream.addr() + 0x250);
    let function = e.mem.u32(output + 0xc);
    e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 1);
        let done = e
            .call(function, &args![output, array, count, size, 1u32])
            .u32();
        let position = e.mem.u32(output + 4).wrapping_add(done);
        e.mem.set_u32(output + 4, position);
    });
}

/// Adds the string in the word at `slot` to the viewer strings: the array grown by its
/// grow-by count when full (`0096ad30`), then `SetAt` (`0096ae90`) at the old size.
fn add_viewer_string(e: &mut Engine, strings: Ptr, slot: u32) {
    let array = strings.addr();
    let index = u32::from(e.mem.u16(array + 0xa));
    if index >= u32::from(e.mem.u16(array + 8)) {
        let grow = u32::from(e.mem.u16(array + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow + index]);
    }
    e.call(STRING_ARRAY_SET_AT, &args![strings, index, slot]);
}

// Translated from 00a8a890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLinesData::GetViewerStrings` (Xbox PDB): after `NiGeometryData::GetViewerStrings`
/// (`00a688b0`), adds the class name and `"m_pkFlags"` with the flag array's address.
pub fn ni_lines_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(GEOMETRY_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let name = e.mem.u32(NI_LINES_DATA_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![name]).u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
        let array = e.mem.u32(this.addr() + FLAGS);
        let text = e
            .call(VIEWER_STRING_POINTER, &args![TEXT_FLAGS, array])
            .u32();
        e.mem.set_u32(slot.addr(), text);
        add_viewer_string(e, strings, slot.addr());
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8_a730, ni_lines_data_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a8_a7c0,
            ni_lines_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a8_a800, ni_lines_data_load_binary(Ptr, Ptr)),
        entry!(0x00a8_a850, ni_lines_data_save_binary(Ptr, Ptr)),
        entry!(0x00a8_a890, ni_lines_data_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00a8a7b0, ni_lines_data_get_rtti() -> Ptr),
        entry!(0x00a8_a9a0, ni_lines_data_link_object(Ptr, Ptr)),
        entry!(0x00a8_a9b0, ni_lines_data_register_streamables(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_lines_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a8a7b0, &args![]).u32(), NI_LINES_DATA_RTTI);
    }

    #[test]
    fn thunk_00a8a9a0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a8_a9a0;
        const TARGET: u32 = 0x00a6_77e0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a8a9b0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a8_a9b0;
        const TARGET: u32 = 0x00a6_6f50;
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
mod a06_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn lines(e: &mut Engine, flags: &[u8]) -> u32 {
        let this = e.mem.alloc(0x44);
        e.mem.set_u16(this + VERTEX_COUNT, flags.len() as u16);
        if !flags.is_empty() {
            let array = e.mem.alloc(flags.len() as u32);
            e.mem.write(array, flags);
            e.mem.set_u32(this + FLAGS, array);
        }
        this
    }

    #[test]
    fn is_equal_compares_the_flags_byte_by_byte() {
        let mut e = Engine::new();
        e.register(GEOMETRY_DATA_IS_EQUAL, |_, _| 1u32.into_ret());
        let a = lines(&mut e, &[1, 0, 1]);
        let b = lines(&mut e, &[1, 0, 1]);
        let c = lines(&mut e, &[1, 1, 1]);
        let none = lines(&mut e, &[]);
        let none2 = lines(&mut e, &[]);
        let eq = |e: &mut Engine, x: u32, y: u32| e.call(0x00a8_a730, &args![x, y]).bool();
        assert!(eq(&mut e, a, b));
        assert!(!eq(&mut e, a, c));
        assert!(!eq(&mut e, a, none));
        assert!(!eq(&mut e, none, a));
        assert!(eq(&mut e, none, none2));
        e.register(GEOMETRY_DATA_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!eq(&mut e, a, b));
    }

    #[test]
    fn scalar_deleting_destructor_frees_the_flags_and_deletes_on_bit_0() {
        let mut e = Engine::new();
        for addr in [NI_FREE, GEOMETRY_DATA_DESTRUCT, SIZED_DELETE] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = lines(&mut e, &[1]);
        let array = e.mem.u32(this + FLAGS);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_a7c0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_LINES_DATA_VTABLE);
        assert_eq!(calls(&e, NI_FREE), vec![vec![array]]);
        assert_eq!(calls(&e, GEOMETRY_DATA_DESTRUCT), vec![vec![this]]);
        assert!(calls(&e, SIZED_DELETE).is_empty());
        e.call(0x00a8_a7c0, &args![this, 1u32]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![this, 0x44]]);
    }

    #[test]
    fn load_and_save_binary_move_one_byte_per_vertex() {
        let mut e = Engine::new();
        let this = lines(&mut e, &[1, 0, 1, 1]);
        let stream = e.mem.alloc(0x260);
        let io = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, io);
        e.mem.set_u32(stream + 0x250, io);
        e.mem.set_u32(io + 8, 0x0340_0000);
        e.mem.set_u32(io + 0xc, 0x0340_0010);
        for addr in [0x0340_0000u32, 0x0340_0010] {
            e.register(addr, |e, a| {
                assert_eq!((e.mem.u32(a[3]), a[4]), (1, 1));
                a[2].into_ret()
            });
        }
        e.register(GEOMETRY_DATA_LOAD_BINARY, |_, _| Ret::default());
        e.register(GEOMETRY_DATA_SAVE_BINARY, |_, _| Ret::default());
        e.register(NI_ALLOC, |_, _| 0x0350_0000u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00a8_a800, &args![this, stream]);
        assert_eq!(calls(&e, NI_ALLOC), vec![vec![4]]);
        assert_eq!(e.mem.u32(this + FLAGS), 0x0350_0000);
        assert_eq!(calls(&e, 0x0340_0000)[0][..3], [io, 0x0350_0000, 4]);
        assert_eq!(e.mem.u32(io + 4), 4);
        e.call(0x00a8_a850, &args![this, stream]);
        assert_eq!(
            calls(&e, GEOMETRY_DATA_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(calls(&e, 0x0340_0010)[0][..3], [io, 0x0350_0000, 4]);
        assert_eq!(e.mem.u32(io + 4), 8);
    }

    #[test]
    fn viewer_strings_add_the_name_and_the_flags_pointer() {
        let mut e = Engine::new();
        e.map(NI_LINES_DATA_RTTI & !0xfff, 0x1000);
        e.mem.set_u32(NI_LINES_DATA_RTTI, 0x0abc_0000);
        let this = lines(&mut e, &[1]);
        let array = e.mem.u32(this + FLAGS);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 0xe, 2);
        for addr in [
            GEOMETRY_DATA_GET_VIEWER_STRINGS,
            STRING_ARRAY_SET_SIZE,
            STRING_ARRAY_SET_AT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(VIEWER_STRING_NAME, |_, _| 0x61u32.into_ret());
        e.register(VIEWER_STRING_POINTER, |_, _| 0x62u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00a8_a890, &args![this, strings]);
        assert_eq!(calls(&e, VIEWER_STRING_NAME), vec![vec![0x0abc_0000]]);
        assert_eq!(
            calls(&e, VIEWER_STRING_POINTER),
            vec![vec![TEXT_FLAGS, array]]
        );
        // The empty array is grown by 2 before each add.
        assert_eq!(calls(&e, STRING_ARRAY_SET_SIZE), vec![vec![strings, 2]; 2]);
        assert_eq!(calls(&e, STRING_ARRAY_SET_AT).len(), 2);
    }
}
