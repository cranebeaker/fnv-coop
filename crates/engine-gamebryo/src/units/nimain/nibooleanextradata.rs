//! `NiMain/nibooleanextradata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBooleanExtraData::ms_RTTI` (the class's `NiRTTI` record).
const NI_BOOLEAN_EXTRA_DATA_RTTI: u32 = 0x011f_5f5c;

// Translated from 00a9af30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBooleanExtraData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_boolean_extra_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BOOLEAN_EXTRA_DATA_RTTI)
}

/// `NiBooleanExtraData`'s vtable (its RTTI names `NiBooleanExtraData`).
const BOOLEAN_EXTRA_DATA_VTABLE: u32 = 0x010a_112c;
/// `00aa13e0` (`cdecl`, the size): the object allocation; `00aa1460` (`cdecl`, (block,
/// size)): the sized delete.
const NI_NEW: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiExtraData`'s constructor (`00a7b2e0`), destructor body (`00a7b300`), member copy
/// (`00a7b330`, (clone, cloning process)), `LoadBinary` (`00a7b4a0`), `SaveBinary`
/// (`00a7b2c0`) and `GetViewerStrings` (`00a7b580`), all `thiscall`.
const EXTRA_DATA_CONSTRUCT: u32 = 0x00a7_b2e0;
const EXTRA_DATA_DESTRUCT: u32 = 0x00a7_b300;
const EXTRA_DATA_COPY_MEMBERS: u32 = 0x00a7_b330;
const EXTRA_DATA_LOAD_BINARY: u32 = 0x00a7_b4a0;
const EXTRA_DATA_SAVE_BINARY: u32 = 0x00a7_b2c0;
const EXTRA_DATA_GET_VIEWER_STRINGS: u32 = 0x00a7_b580;
/// `NiGetViewerString` of a class name (`00a7d330`) and of a `bool` (`00a7d370`, name and
/// value); the member name `010a11c0`.
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const GET_VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
const NAME_VALUE: u32 = 0x010a_11c0;
/// `0096ad30` grows an `NiTArray`; `0096ae90` stores an element (both `thiscall`).
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;

// The value is the byte at +0xc, as every function below uses it.

// Translated from 00a9af10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBooleanExtraData::IsEqual` (Xbox PDB): false for a null other; otherwise whether both
/// value bytes (+0xc) are the same (the base class is not compared).
pub fn ni_boolean_extra_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if other.addr() == 0 {
        return false;
    }
    e.mem.u8(this.addr() + 0xc) == e.mem.u8(other.addr() + 0xc)
}

/// A new boolean extra data (0x10 bytes from `00aa13e0`, `NiExtraData`'s constructor, the
/// vtable `010a112c`, value 0); null when the allocation fails.
fn new_boolean_extra_data(e: &mut Engine) -> u32 {
    let block = e.call(NI_NEW, &args![0x10u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(EXTRA_DATA_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, BOOLEAN_EXTRA_DATA_VTABLE);
    e.mem.set_u8(block + 0xc, 0);
    block
}

// Translated from 00a9af40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBooleanExtraData::CreateClone` (Xbox PDB): a new boolean extra data (null when the
/// allocation fails; the store below still goes to it), `NiExtraData`'s member copy
/// (`00a7b330`), then this one's value byte. Answers the clone.
pub fn ni_boolean_extra_data_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    let clone = new_boolean_extra_data(e);
    e.call(
        EXTRA_DATA_COPY_MEMBERS,
        &args![this, clone, cloning_process],
    );
    let value = e.mem.u8(this.addr() + 0xc);
    e.mem.set_u8(clone.wrapping_add(0xc), value);
    Ptr::new(clone)
}

// Translated from 00a9af90 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; no arguments): by its body the
/// creation function of `NiBooleanExtraData`: a new boolean extra data, or null.
pub fn fn_00a9af90(e: &mut Engine) -> Ptr {
    Ptr::new(new_boolean_extra_data(e))
}

// Translated from 00a9afc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBooleanExtraData::_scalar_deleting_destructor_` (Xbox PDB): the vtable `010a112c`,
/// `NiExtraData`'s destructor body (`00a7b300`), then the sized delete (`00aa1460`) of 0x10
/// bytes when bit 0 of the flags byte is set. Answers `this`.
pub fn ni_boolean_extra_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), BOOLEAN_EXTRA_DATA_VTABLE);
    e.call(EXTRA_DATA_DESTRUCT, &args![this]);
    if flags as u8 & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00a9aff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBooleanExtraData::LoadBinary` (Xbox PDB): `NiExtraData::LoadBinary`, then one byte from
/// the stream's input (`+0x24c`, read function +8; the count word, a local, 1) read into the
/// low byte of the stream argument's own stack word; the value (+0xc) is whether that byte is
/// non-zero.
pub fn ni_boolean_extra_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(EXTRA_DATA_LOAD_BINARY, &args![this, stream]);
    e.with_stack(8, |e, locals| {
        let count = locals.addr();
        let slot = locals.addr() + 4;
        e.mem.set_u32(slot, stream.addr());
        let input = e.mem.u32(stream.addr() + 0x24c);
        let read = e.mem.u32(input + 8);
        e.mem.set_u32(count, 1);
        let done = e.call(read, &args![input, slot, 1u32, count, 1u32]).u32();
        let position = e.mem.u32(input + 4).wrapping_add(done);
        e.mem.set_u32(input + 4, position);
        let value = e.mem.u8(slot) != 0;
        e.mem.set_u8(this.addr() + 0xc, u8::from(value));
    });
}

// Translated from 00a9b040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBooleanExtraData::SaveBinary` (Xbox PDB): `NiExtraData::SaveBinary`, then the value byte
/// written from the low byte of the stream argument's own stack word (the count word, a local,
/// 1).
pub fn ni_boolean_extra_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(EXTRA_DATA_SAVE_BINARY, &args![this, stream]);
    e.with_stack(8, |e, locals| {
        let count = locals.addr();
        let slot = locals.addr() + 4;
        e.mem.set_u32(slot, stream.addr());
        let value = e.mem.u8(this.addr() + 0xc);
        let output = e.mem.u32(stream.addr() + 0x250);
        e.mem.set_u8(slot, value);
        let write = e.mem.u32(output + 0xc);
        e.mem.set_u32(count, 1);
        let done = e.call(write, &args![output, slot, 1u32, count, 1u32]).u32();
        let position = e.mem.u32(output + 4).wrapping_add(done);
        e.mem.set_u32(output + 4, position);
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

// Translated from 00a9b090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBooleanExtraData::GetViewerStrings` (Xbox PDB): `NiExtraData::GetViewerStrings`, the
/// class name (first word of the `NiRTTI` at `011f5f5c`), then the value byte (zero-extended)
/// as a `bool` under the name at `010a11c0` (`00a7d370`), each kept in the `strings`
/// argument's stack word and appended.
pub fn ni_boolean_extra_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(EXTRA_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let class_name = e.global::<u32>(NI_BOOLEAN_EXTRA_DATA_RTTI);
        let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(slot.addr(), text);
        append_viewer_string(e, strings, slot.addr());
        let value = u32::from(e.mem.u8(this.addr() + 0xc));
        let text = e
            .call(GET_VIEWER_STRING_BOOL, &args![NAME_VALUE, value])
            .u32();
        e.mem.set_u32(slot.addr(), text);
        append_viewer_string(e, strings, slot.addr());
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9af30, ni_boolean_extra_data_get_rtti() -> Ptr),
        entry!(0x00a9_af10, ni_boolean_extra_data_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a9_af40, ni_boolean_extra_data_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a9_af90, fn_00a9af90() -> Ptr),
        entry!(
            0x00a9_afc0,
            ni_boolean_extra_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a9_aff0, ni_boolean_extra_data_load_binary(Ptr, Ptr)),
        entry!(0x00a9_b040, ni_boolean_extra_data_save_binary(Ptr, Ptr)),
        entry!(
            0x00a9_b090,
            ni_boolean_extra_data_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_boolean_extra_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a9af30, &args![]).u32(),
            NI_BOOLEAN_EXTRA_DATA_RTTI
        );
    }
}

#[cfg(test)]
mod n015_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::calls_to;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        for double in [
            EXTRA_DATA_CONSTRUCT,
            EXTRA_DATA_DESTRUCT,
            EXTRA_DATA_COPY_MEMBERS,
            EXTRA_DATA_LOAD_BINARY,
            EXTRA_DATA_SAVE_BINARY,
            EXTRA_DATA_GET_VIEWER_STRINGS,
            SIZED_DELETE,
            ARRAY_SET_SIZE,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn is_equal_compares_the_value() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        e.mem.set_u8(this + 0xc, 1);
        assert!(!e.call(0x00a9_af10, &args![this, other]).bool());
        e.mem.set_u8(other + 0xc, 1);
        assert!(e.call(0x00a9_af10, &args![this, other]).bool());
        assert!(!e.call(0x00a9_af10, &args![this, 0u32]).bool());
    }

    #[test]
    fn creation_clone_and_destructor() {
        let mut e = engine();
        let made = e.call(0x00a9_af90, &args![]).u32();
        assert_eq!(e.mem.u32(made), BOOLEAN_EXTRA_DATA_VTABLE);
        assert_eq!(calls_to(&e, NI_NEW), vec![vec![0x10]]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u8(this + 0xc, 1);
        let clone = e.call(0x00a9_af40, &args![this, 0x66u32]).u32();
        assert_eq!(e.mem.u32(clone), BOOLEAN_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u8(clone + 0xc), 1);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_COPY_MEMBERS),
            vec![vec![this, clone, 0x66]]
        );
        assert_eq!(e.call(0x00a9_afc0, &args![this, 2u32]).u32(), this);
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        e.call(0x00a9_afc0, &args![this, 3u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0x10]]);
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_af90, &args![]).u32(), 0);
    }

    #[test]
    fn load_and_save_one_byte() {
        let mut e = engine();
        let stream = e.mem.alloc(0x300);
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, binary);
        e.mem.set_u32(stream + 0x250, binary);
        e.mem.set_u32(binary + 8, 0x0300_0000);
        e.mem.set_u32(binary + 0xc, 0x0300_0004);
        e.register(0x0300_0000, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (1, 1, 1));
            e.mem.set_u8(a[1], 7);
            1u32.into_ret()
        });
        e.register(0x0300_0004, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (1, 1, 1));
            let byte = e.mem.u8(a[1]);
            e.mem.set_u8(0x011f_5f00, byte);
            1u32.into_ret()
        });
        let this = e.mem.alloc(0x10);
        e.call(0x00a9_aff0, &args![this, stream]);
        assert_eq!(e.mem.u8(this + 0xc), 1);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_LOAD_BINARY),
            vec![vec![this, stream]]
        );
        // A read of nothing leaves the stream pointer's low byte.
        e.register(0x0300_0000, |_, _| 0u32.into_ret());
        e.call(0x00a9_aff0, &args![this, stream]);
        assert_eq!(e.mem.u8(this + 0xc), u8::from(stream & 0xff != 0));
        e.mem.set_u8(this + 0xc, 1);
        e.call(0x00a9_b040, &args![this, stream]);
        assert_eq!(e.mem.u8(0x011f_5f00), 1);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(e.mem.u32(binary + 4), 2);
    }

    #[test]
    fn viewer_strings_add_class_and_value() {
        let mut e = engine();
        e.mem.set_u32(NI_BOOLEAN_EXTRA_DATA_RTTI, 0x0102_0000);
        e.register(GET_VIEWER_STRING, |_, _| 0x7000u32.into_ret());
        e.register(GET_VIEWER_STRING_BOOL, |_, _| 0x8000u32.into_ret());
        e.register(ARRAY_SET_AT, |_, _| Ret::default());
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        let this = e.mem.alloc(0x10);
        e.mem.set_u8(this + 0xc, 1);
        e.call(0x00a9_b090, &args![this, strings]);
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x0102_0000]]);
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING_BOOL),
            vec![vec![NAME_VALUE, 1]]
        );
        assert_eq!(calls_to(&e, ARRAY_SET_AT).len(), 2);
    }
}
