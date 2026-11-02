//! `NiMain/nicolorextradata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiColorExtraData::ms_RTTI` (the class's `NiRTTI` record).
const NI_COLOR_EXTRA_DATA_RTTI: u32 = 0x011f_5b68;

// Translated from 00a893f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_color_extra_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_COLOR_EXTRA_DATA_RTTI)
}

/// `NiColorExtraData`'s vtable (its RTTI names `NiColorExtraData`).
const COLOR_EXTRA_DATA_VTABLE: u32 = 0x0109_eccc;
/// The four words at `011a9bd0` a new object's color is copied from (`MOV`).
const DEFAULT_COLOR: u32 = 0x011a_9bd0;
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
/// `NiColor::LoadBinary` (`00a59be0`), `NiColorA::LoadBinary` (`00a69800`),
/// `NiColorA::SaveBinary` (`00a698a0`) and `NiColorA::GetViewerString` (`00a69720`), each
/// `thiscall` on a color with one word; `004a4c90` (`thiscall`, the other color): the
/// negation of the comparison `004a4cc0`.
const COLOR_LOAD_BINARY: u32 = 0x00a5_9be0;
const COLOR_A_LOAD_BINARY: u32 = 0x00a6_9800;
const COLOR_A_SAVE_BINARY: u32 = 0x00a6_98a0;
const COLOR_A_GET_VIEWER_STRING: u32 = 0x00a6_9720;
const COLOR_A_COMPARE: u32 = 0x004a_4c90;
/// `NiGetViewerString` of a class name (`00a7d330`); the member name `0109ed60`.
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NAME_COLOR: u32 = 0x0109_ed60;
/// `0096ad30` grows an `NiTArray`; `0096ae90` stores an element (both `thiscall`).
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;

// The color (`NiColorA`, four floats) is at +0xc, as every function below uses it.

// Translated from 00a893b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraData::SetValue` (Xbox PDB): the color passed by value (four words) copied to
/// +0xc.
pub fn ni_color_extra_data_set_value(e: &mut Engine, this: Ptr, r: u32, g: u32, b: u32, a: u32) {
    e.mem.set_u32(this.addr() + 0xc, r);
    e.mem.set_u32(this.addr() + 0x10, g);
    e.mem.set_u32(this.addr() + 0x14, b);
    e.mem.set_u32(this.addr() + 0x18, a);
}

// Translated from 00a893d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraData::SaveBinary` (Xbox PDB): `NiExtraData::SaveBinary`, then the color
/// (`NiColorA::SaveBinary` on +0xc).
pub fn ni_color_extra_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(EXTRA_DATA_SAVE_BINARY, &args![this, stream]);
    e.call(COLOR_A_SAVE_BINARY, &args![this.addr() + 0xc, stream]);
}

/// A new color extra data (0x1c bytes from `00aa13e0`, `NiExtraData`'s constructor, the
/// vtable `0109eccc`, the color first 0.0 four times (`FLDZ`, `FST`) and then the four words
/// at `011a9bd0`); null when the allocation fails.
fn new_color_extra_data(e: &mut Engine) -> u32 {
    let block = e.call(NI_NEW, &args![0x1cu32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(EXTRA_DATA_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, COLOR_EXTRA_DATA_VTABLE);
    for word in 0..4 {
        e.mem.set_u32(block + 0xc + 4 * word, 0.0f32.to_bits());
    }
    for word in 0..4 {
        let value = e.global::<u32>(DEFAULT_COLOR + 4 * word);
        e.mem.set_u32(block + 0xc + 4 * word, value);
    }
    block
}

// Translated from 00a89400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraData::CreateClone` (Xbox PDB): a new color extra data (null when the
/// allocation fails; the stores below still go to it), `NiExtraData`'s member copy
/// (`00a7b330`), then this one's color (four words). Answers the clone.
pub fn ni_color_extra_data_create_clone(e: &mut Engine, this: Ptr, cloning_process: Ptr) -> Ptr {
    let clone = new_color_extra_data(e);
    e.call(
        EXTRA_DATA_COPY_MEMBERS,
        &args![this, clone, cloning_process],
    );
    for word in 0..4 {
        let value = e.mem.u32(this.addr() + 0xc + 4 * word);
        e.mem.set_u32(clone.wrapping_add(0xc + 4 * word), value);
    }
    Ptr::new(clone)
}

// Translated from 00a89490 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; no arguments): by its body the
/// creation function of `NiColorExtraData`: a new color extra data, or null.
pub fn fn_00a89490(e: &mut Engine) -> Ptr {
    Ptr::new(new_color_extra_data(e))
}

// Translated from 00a894f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraData::LoadBinary` (Xbox PDB): `NiExtraData::LoadBinary`; files before version
/// `0x0500000e` (`NiStream` +0xd8) hold a three-component color, read by `NiColor::LoadBinary`
/// into a local of three 0.0 floats and copied in (`FLD`, `FSTP`) with alpha 1.0 (`FLD1`);
/// newer files hold the four components (`NiColorA::LoadBinary` on +0xc).
pub fn ni_color_extra_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(EXTRA_DATA_LOAD_BINARY, &args![this, stream]);
    if e.mem.u32(stream.addr() + 0xd8) >= 0x0500_000e {
        e.call(COLOR_A_LOAD_BINARY, &args![this.addr() + 0xc, stream]);
        return;
    }
    e.with_stack(0xc, |e, color| {
        for word in 0..3 {
            e.mem.set_u32(color.addr() + 4 * word, 0.0f32.to_bits());
        }
        e.call(COLOR_LOAD_BINARY, &args![color, stream]);
        for word in 0..3 {
            let value = x87_quiet_bits(e.mem.u32(color.addr() + 4 * word));
            e.mem.set_u32(this.addr() + 0xc + 4 * word, value);
        }
    });
    e.mem.set_u32(this.addr() + 0x18, 1.0f32.to_bits());
}

// Translated from 00a89560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraData::_scalar_deleting_destructor_` (Xbox PDB): the vtable `0109eccc`,
/// `NiExtraData`'s destructor body (`00a7b300`), then the sized delete (`00aa1460`) of 0x1c
/// bytes when bit 0 of the flags byte is set. Answers `this`.
pub fn ni_color_extra_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), COLOR_EXTRA_DATA_VTABLE);
    e.call(EXTRA_DATA_DESTRUCT, &args![this]);
    if flags as u8 & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x1cu32]);
    }
    this
}

// Translated from 00a89590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraData::IsEqual` (Xbox PDB): false for a null other; otherwise the negation of
/// `004a4c90` on the two colors (+0xc). The base class is not compared.
pub fn ni_color_extra_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if other.addr() == 0 {
        return false;
    }
    !e.call(
        COLOR_A_COMPARE,
        &args![this.addr() + 0xc, other.addr() + 0xc],
    )
    .bool()
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

// Translated from 00a895c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiColorExtraData::GetViewerStrings` (Xbox PDB): `NiExtraData::GetViewerStrings`, the class
/// name (first word of the `NiRTTI` at `011f5b68`), then the color's viewer string
/// (`NiColorA::GetViewerString` on +0xc with the name at `0109ed60`), each kept in the
/// `strings` argument's stack word and appended.
pub fn ni_color_extra_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(EXTRA_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let class_name = e.global::<u32>(NI_COLOR_EXTRA_DATA_RTTI);
        let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(slot.addr(), text);
        append_viewer_string(e, strings, slot.addr());
        let text = e
            .call(
                COLOR_A_GET_VIEWER_STRING,
                &args![this.addr() + 0xc, NAME_COLOR],
            )
            .u32();
        e.mem.set_u32(slot.addr(), text);
        append_viewer_string(e, strings, slot.addr());
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a893f0, ni_color_extra_data_get_rtti() -> Ptr),
        entry!(
            0x00a8_93b0,
            ni_color_extra_data_set_value(Ptr, u32, u32, u32, u32)
        ),
        entry!(0x00a8_93d0, ni_color_extra_data_save_binary(Ptr, Ptr)),
        entry!(0x00a8_9400, ni_color_extra_data_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a8_9490, fn_00a89490() -> Ptr),
        entry!(0x00a8_94f0, ni_color_extra_data_load_binary(Ptr, Ptr)),
        entry!(0x00a8_9560, ni_color_extra_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a8_9590, ni_color_extra_data_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a8_95c0,
            ni_color_extra_data_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_color_extra_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a893f0, &args![]).u32(), NI_COLOR_EXTRA_DATA_RTTI);
    }
}

#[cfg(test)]
mod n015_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::calls_to;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011a_9000, 0x1000);
        e.map(0x011f_5000, 0x1000);
        for (index, value) in [0.25f32, 0.5, 0.75, 1.0].into_iter().enumerate() {
            e.mem.set_f32(DEFAULT_COLOR + 4 * index as u32, value);
        }
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        for double in [
            EXTRA_DATA_CONSTRUCT,
            EXTRA_DATA_DESTRUCT,
            EXTRA_DATA_COPY_MEMBERS,
            EXTRA_DATA_LOAD_BINARY,
            EXTRA_DATA_SAVE_BINARY,
            EXTRA_DATA_GET_VIEWER_STRINGS,
            COLOR_A_LOAD_BINARY,
            COLOR_A_SAVE_BINARY,
            SIZED_DELETE,
            ARRAY_SET_SIZE,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn set_value_and_save_binary() {
        let mut e = engine();
        let this = e.mem.alloc(0x1c);
        e.call(0x00a8_93b0, &args![this, 1u32, 2u32, 3u32, 4u32]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(this + 0xc + 4 * i)).collect();
        assert_eq!(words, vec![1, 2, 3, 4]);
        e.call(0x00a8_93d0, &args![this, 0x55u32]);
        assert_eq!(calls_to(&e, EXTRA_DATA_SAVE_BINARY), vec![vec![this, 0x55]]);
        assert_eq!(
            calls_to(&e, COLOR_A_SAVE_BINARY),
            vec![vec![this + 0xc, 0x55]]
        );
    }

    #[test]
    fn creation_clone_and_destructor() {
        let mut e = engine();
        let made = e.call(0x00a8_9490, &args![]).u32();
        assert_eq!(e.mem.u32(made), COLOR_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.f32(made + 0xc), 0.25);
        assert_eq!(e.mem.f32(made + 0x18), 1.0);
        let this = e.mem.alloc(0x1c);
        e.call(0x00a8_93b0, &args![this, 5u32, 6u32, 7u32, 8u32]);
        let clone = e.call(0x00a8_9400, &args![this, 0x66u32]).u32();
        assert_eq!(
            calls_to(&e, EXTRA_DATA_COPY_MEMBERS),
            vec![vec![this, clone, 0x66]]
        );
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(clone + 0xc + 4 * i)).collect();
        assert_eq!(words, vec![5, 6, 7, 8]);
        assert_eq!(e.call(0x00a8_9560, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), COLOR_EXTRA_DATA_VTABLE);
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        e.call(0x00a8_9560, &args![this, 1u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0x1c]]);
        assert_eq!(calls_to(&e, EXTRA_DATA_DESTRUCT).len(), 2);
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a8_9490, &args![]).u32(), 0);
    }

    #[test]
    fn load_binary_by_version() {
        let mut e = engine();
        e.register(COLOR_LOAD_BINARY, |e, a| {
            e.mem.set_f32(a[0], 0.1);
            e.mem.set_f32(a[0] + 4, 0.2);
            e.mem.set_u32(a[0] + 8, 0x7f80_0001);
            Ret::default()
        });
        let stream = e.mem.alloc(0x300);
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(stream + 0xd8, 0x0500_000d);
        e.call(0x00a8_94f0, &args![this, stream]);
        assert_eq!(e.mem.f32(this + 0xc), 0.1);
        assert_eq!(e.mem.f32(this + 0x10), 0.2);
        assert_eq!(e.mem.u32(this + 0x14), 0x7fc0_0001);
        assert_eq!(e.mem.f32(this + 0x18), 1.0);
        assert!(calls_to(&e, COLOR_A_LOAD_BINARY).is_empty());
        e.mem.set_u32(stream + 0xd8, 0x0500_000e);
        e.call(0x00a8_94f0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, COLOR_A_LOAD_BINARY),
            vec![vec![this + 0xc, stream]]
        );
        assert_eq!(calls_to(&e, EXTRA_DATA_LOAD_BINARY).len(), 2);
    }

    #[test]
    fn is_equal_negates_the_comparison() {
        let mut e = engine();
        e.register(COLOR_A_COMPARE, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x1c);
        let other = e.mem.alloc(0x1c);
        assert!(e.call(0x00a8_9590, &args![this, other]).bool());
        assert_eq!(
            calls_to(&e, COLOR_A_COMPARE),
            vec![vec![this + 0xc, other + 0xc]]
        );
        e.register(COLOR_A_COMPARE, |_, _| 1u32.into_ret());
        assert!(!e.call(0x00a8_9590, &args![this, other]).bool());
        assert!(!e.call(0x00a8_9590, &args![this, 0u32]).bool());
    }

    #[test]
    fn viewer_strings_add_class_and_color() {
        let mut e = engine();
        e.mem.set_u32(NI_COLOR_EXTRA_DATA_RTTI, 0x0102_0000);
        e.register(GET_VIEWER_STRING, |_, _| 0x7000u32.into_ret());
        e.register(COLOR_A_GET_VIEWER_STRING, |_, _| 0x8000u32.into_ret());
        e.register(ARRAY_SET_AT, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(0x011f_5f00 + 4 * a[1], value);
            let count = e.mem.u16(a[0] + 0xa) + 1;
            e.mem.set_u16(a[0] + 0xa, count);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        let this = e.mem.alloc(0x1c);
        e.call(0x00a8_95c0, &args![this, strings]);
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x0102_0000]]);
        assert_eq!(
            calls_to(&e, COLOR_A_GET_VIEWER_STRING),
            vec![vec![this + 0xc, NAME_COLOR]]
        );
        assert_eq!(
            (e.mem.u32(0x011f_5f00), e.mem.u32(0x011f_5f04)),
            (0x7000, 0x8000)
        );
    }
}
