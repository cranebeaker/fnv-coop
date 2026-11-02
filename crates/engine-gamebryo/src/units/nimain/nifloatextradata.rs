//! `NiMain/nifloatextradata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

/// `NiFloatExtraData::ms_RTTI` (the class's `NiRTTI` record).
const NI_FLOAT_EXTRA_DATA_RTTI: u32 = 0x011f_5f40;

// Translated from 00a9a420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatExtraData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_float_extra_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_FLOAT_EXTRA_DATA_RTTI)
}

/// `NiFloatExtraData`'s vtable.
const NI_FLOAT_EXTRA_DATA_VTABLE: u32 = 0x010a_0ec4;
/// `operator new` (`cdecl`, the size) and the sized delete (`cdecl`, (block, size)).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiExtraData`'s constructor and destructor body (`thiscall`, no stack words), its member
/// copy (`00a7b330`, (clone, cloning process)) and `NiExtraData::LoadBinary` (the stream).
const EXTRA_DATA_CONSTRUCT: u32 = 0x00a7_b2e0;
const EXTRA_DATA_DESTRUCT: u32 = 0x00a7_b300;
const EXTRA_DATA_COPY_MEMBERS: u32 = 0x00a7_b330;
const EXTRA_DATA_LOAD_BINARY: u32 = 0x00a7_b4a0;

/// The extra data's float (`m_fValue`, Xbox PDB) at +0xc.
const VALUE: u32 = 0xc;

// Translated from 00a99d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatExtraData::LoadBinary` (Xbox PDB): `NiExtraData::LoadBinary`, then the 4-byte
/// value (+0xc) from the stream's input (the stream argument's stack word is the byte-count
/// word).
pub fn ni_float_extra_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    e.call(EXTRA_DATA_LOAD_BINARY, &args![this, stream]);
    let input = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr));
    let function = e.get(input, NiBinaryStream::m_pfnRead);
    let done = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 4);
        e.call(
            function,
            &args![input, this.addr() + VALUE, 4u32, count, 1u32],
        )
        .u32()
    });
    let position = e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos);
    e.set(
        input,
        NiBinaryStream::m_uiAbsoluteCurrentPos,
        position.wrapping_add(done),
    );
}

/// Allocates and builds an empty float extra data (0x10 bytes, `operator new`, the
/// `NiExtraData` constructor, value 0, the vtable `010a0ec4`); null when the allocation
/// fails.
fn new_float_extra_data(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(EXTRA_DATA_CONSTRUCT, &args![block]);
    e.mem.set_u32(block + VALUE, x87_store32(0.0));
    e.mem.set_u32(block, NI_FLOAT_EXTRA_DATA_VTABLE);
    block
}

// Translated from 00a9a480 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiFloatExtraData`): a new empty float extra data (value 0), or null.
pub fn fn_00a9a480(e: &mut Engine) -> Ptr {
    Ptr::new(new_float_extra_data(e))
}

// Translated from 00a9a430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatExtraData::CreateClone` (Xbox PDB): a new empty float extra data (null when the
/// allocation fails), `NiExtraData`'s member copy into it (`00a7b330`), then this value
/// copied to its +0xc (written at 0xc for a null clone, as the exe does). Answers the clone.
pub fn ni_float_extra_data_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    let clone = new_float_extra_data(e);
    e.call(
        EXTRA_DATA_COPY_MEMBERS,
        &args![this, clone, cloning_process],
    );
    let value = x87_store32(x87_load32(e.mem.u32(this.addr() + VALUE)));
    e.mem.set_u32(clone.wrapping_add(VALUE), value);
    Ptr::new(clone)
}

// Translated from 00a9a4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatExtraData::_scalar_deleting_destructor_` (Xbox PDB): stores the vtable
/// `010a0ec4`, runs the `NiExtraData` destructor body (`00a7b300`) and, when bit 0 of `flags`
/// is set, the sized delete of 0x10 bytes. Answers `this`.
pub fn ni_float_extra_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), NI_FLOAT_EXTRA_DATA_VTABLE);
    e.call(EXTRA_DATA_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x10u32]);
    }
    this
}

/// `NiExtraData::SaveBinary` (`00a7b2c0`, `thiscall`, the stream) and
/// `NiExtraData::GetViewerStrings` (`00a7b580`, `thiscall`, the string array).
const EXTRA_DATA_SAVE_BINARY: u32 = 0x00a7_b2c0;
const EXTRA_DATA_GET_VIEWER_STRINGS: u32 = 0x00a7_b580;
/// `NiGetViewerString` (`00a7d330`, `cdecl`: the class name) and `NiGetViewerString_ov9`
/// (`00a7d550`, `cdecl`: a name and a float), both answering a new string.
const VIEWER_STRING_CLASS: u32 = 0x00a7_d330;
const VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
/// The string array's grow (`0096ad30`, `thiscall`, the new size) and set-at (`0096ae90`,
/// `thiscall`, (index, address of the string)), both unnamed in the engine map.
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The literal `"m_fValue"`.
const TEXT_M_F_VALUE: u32 = 0x010a_0f58;

/// Appends `text` to a viewer-string array as the exe's inlined `Add` does: when the 16-bit
/// size (+0xa) has reached the capacity (+0x8) the array grows to its step (+0xe) plus the
/// size, then the string is stored at the old size (through a stack word holding it).
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

// Translated from 00a9a3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatExtraData::IsEqual` (Xbox PDB): false for a null other object; otherwise true
/// exactly when the two values (+0xc) compare equal (`FUCOMPP`; a NaN on either side is not
/// equal). The base class is not consulted.
pub fn ni_float_extra_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if other.is_null() {
        return false;
    }
    let mine = x87_load32(e.mem.u32(this.addr() + VALUE));
    let theirs = x87_load32(e.mem.u32(other.addr() + VALUE));
    theirs == mine
}

// Translated from 00a9a4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatExtraData::SaveBinary` (Xbox PDB): `NiExtraData::SaveBinary`, then the 4-byte
/// value (+0xc) through the stream's output (`m_pkOstr`, its write function with a count
/// word of 4 and one item), its position advanced by what was written.
pub fn ni_float_extra_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    e.call(EXTRA_DATA_SAVE_BINARY, &args![this, stream]);
    let output = Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkOstr));
    let function = e.get(output, NiBinaryStream::m_pfnWrite);
    let done = e.with_stack(4, |e, count| {
        e.mem.set_u32(count.addr(), 4);
        e.call(
            function,
            &args![output, this.addr() + VALUE, 4u32, count, 1u32],
        )
        .u32()
    });
    let position = e.get(output, NiBinaryStream::m_uiAbsoluteCurrentPos);
    e.set(
        output,
        NiBinaryStream::m_uiAbsoluteCurrentPos,
        position.wrapping_add(done),
    );
}

// Translated from 00a9a520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatExtraData::GetViewerStrings` (Xbox PDB): `NiExtraData::GetViewerStrings`, then
/// the class name (`NiGetViewerString` of `ms_RTTI`'s name word) and `"m_fValue"` with the
/// value (`NiGetViewerString_ov9`, the float passed as its bits) appended to the array.
pub fn ni_float_extra_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(EXTRA_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(NI_FLOAT_EXTRA_DATA_RTTI);
    let text = e.call(VIEWER_STRING_CLASS, &args![name]).u32();
    push_viewer_string(e, strings, text);
    let value = x87_quiet_bits(e.mem.u32(this.addr() + VALUE));
    let text = e
        .call(VIEWER_STRING_FLOAT, &args![TEXT_M_F_VALUE, value])
        .u32();
    push_viewer_string(e, strings, text);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a9a420, ni_float_extra_data_get_rtti() -> Ptr),
        entry!(
            0x00a9_9d30,
            ni_float_extra_data_load_binary(Ptr, Ptr<NiStream>)
        ),
        entry!(0x00a9_a480, fn_00a9a480() -> Ptr),
        entry!(0x00a9_a430, ni_float_extra_data_create_clone(Ptr, Ptr) -> Ptr),
        entry!(
            0x00a9_a4b0,
            ni_float_extra_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a9_a3f0, ni_float_extra_data_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a9_a4e0,
            ni_float_extra_data_save_binary(Ptr, Ptr<NiStream>)
        ),
        entry!(
            0x00a9_a520,
            ni_float_extra_data_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_float_extra_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a9a420, &args![]).u32(), NI_FLOAT_EXTRA_DATA_RTTI);
    }

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
            e.mem.write(block, &[0xee; 0x10]);
            block.into_ret()
        });
        for double in [
            EXTRA_DATA_CONSTRUCT,
            EXTRA_DATA_DESTRUCT,
            EXTRA_DATA_COPY_MEMBERS,
            EXTRA_DATA_LOAD_BINARY,
            SIZED_DELETE,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn load_binary_reads_the_value() {
        let mut e = engine();
        let stream = e.new_object::<NiStream>();
        let input = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkIstr, input.addr());
        e.set(input, NiBinaryStream::m_pfnRead, 0x0300_0000);
        e.register(0x0300_0000, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            e.mem.set_f32(a[1], 2.5);
            4u32.into_ret()
        });
        let this = e.mem.alloc(0x10);
        e.call(0x00a9_9d30, &args![this, stream]);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_LOAD_BINARY),
            vec![vec![this, stream.addr()]]
        );
        assert_eq!(e.mem.f32(this + 0xc), 2.5);
        assert_eq!(e.get(input, NiBinaryStream::m_uiAbsoluteCurrentPos), 4);
    }

    #[test]
    fn create_object_builds_an_empty_value() {
        let mut e = engine();
        let object = e.call(0x00a9_a480, &args![]).u32();
        assert_eq!(e.mem.u32(object), NI_FLOAT_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u32(object + 0xc), 0);
        assert_eq!(e.mem.u32(object + 4), 0xeeee_eeee);
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_a480, &args![]).u32(), 0);
    }

    #[test]
    fn create_clone_copies_the_value() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.mem.set_f32(this + 0xc, -1.5);
        let clone = e.call(0x00a9_a430, &args![this, 0x77u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_FLOAT_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.f32(clone + 0xc), -1.5);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_COPY_MEMBERS),
            vec![vec![this, clone, 0x77]]
        );
    }

    #[test]
    fn scalar_deleting_destructor_deletes_on_bit_0() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00a9_a4b0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_FLOAT_EXTRA_DATA_VTABLE);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0x10]]);
        e.call(0x00a9_a4b0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE).len(), 1);
        assert_eq!(calls_to(&e, EXTRA_DATA_DESTRUCT).len(), 2);
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

    #[test]
    fn is_equal_compares_the_values() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        e.mem.set_f32(this + 0xc, 1.5);
        e.mem.set_f32(other + 0xc, 1.5);
        assert!(e.call(0x00a9_a3f0, &args![this, other]).bool());
        assert!(!e.call(0x00a9_a3f0, &args![this, 0u32]).bool());
        e.mem.set_f32(other + 0xc, 2.0);
        assert!(!e.call(0x00a9_a3f0, &args![this, other]).bool());
        e.mem.set_f32(this + 0xc, 0.0);
        e.mem.set_f32(other + 0xc, -0.0);
        assert!(e.call(0x00a9_a3f0, &args![this, other]).bool());
        e.mem.set_u32(this + 0xc, 0x7fc0_0000);
        e.mem.set_u32(other + 0xc, 0x7fc0_0000);
        assert!(!e.call(0x00a9_a3f0, &args![this, other]).bool());
    }

    #[test]
    fn save_binary_writes_the_value() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_SAVE_BINARY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let stream = e.new_object::<NiStream>();
        let output = e.new_object::<NiBinaryStream>();
        e.set(stream, NiStream::m_pkOstr, output.addr());
        e.set(output, NiBinaryStream::m_pfnWrite, 0x0300_0000);
        e.set(output, NiBinaryStream::m_uiAbsoluteCurrentPos, 8);
        e.register(0x0300_0000, |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            assert_eq!(e.mem.f32(a[1]), 3.25);
            4u32.into_ret()
        });
        let this = e.mem.alloc(0x10);
        e.mem.set_f32(this + 0xc, 3.25);
        e.call(0x00a9_a4e0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_SAVE_BINARY),
            vec![vec![this, stream.addr()]]
        );
        assert_eq!(
            calls_to(&e, 0x0300_0000)[0][..2],
            [output.addr(), this + 0xc]
        );
        assert_eq!(e.get(output, NiBinaryStream::m_uiAbsoluteCurrentPos), 12);
    }

    #[test]
    fn get_viewer_strings_adds_the_class_and_value() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        e.mem.set_u32(NI_FLOAT_EXTRA_DATA_RTTI, 0x0123_0000);
        e.register(EXTRA_DATA_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(VIEWER_STRING_CLASS, |_, _| 0x0e00_0001u32.into_ret());
        e.register(VIEWER_STRING_FLOAT, |_, _| 0x0e00_0002u32.into_ret());
        e.register(STRINGS_GROW, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x10);
        e.mem.set_f32(this + 0xc, 0.5);
        // Size 2 of capacity 3, step 4: the second string grows the array to 4 + 3.
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 3);
        e.mem.set_u16(strings + 0xa, 2);
        e.mem.set_u16(strings + 0xe, 4);
        e.call(0x00a9_a520, &args![this, strings]);
        assert_eq!(
            calls_to(&e, EXTRA_DATA_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls_to(&e, VIEWER_STRING_CLASS), vec![vec![0x0123_0000]]);
        assert_eq!(
            calls_to(&e, VIEWER_STRING_FLOAT),
            vec![vec![TEXT_M_F_VALUE, 0.5f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, STRINGS_GROW), vec![vec![strings, 7]]);
        let set = calls_to(&e, STRINGS_SET_AT);
        assert_eq!((set[0][1], set[1][1]), (2, 3));
    }
}
