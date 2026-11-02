//! `fallout shared/bgscliproundsform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00475100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSClipRoundsForm::InitializeDataComponent` (Xbox PDB, matched by the
/// engine map): the clip rounds byte (+4) 12.
pub fn bgs_clip_rounds_form_initialize_data_component(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 4, 0xc);
}

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The `BaseFormComponent` and `BGSClipRoundsForm` type descriptors (`01183040`,
/// `011868d4`).
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_BGS_CLIP_ROUNDS_FORM: u32 = 0x0118_68d4;
/// `00401170` (`ECX` = object): the byte at +4, zero-extended.
const BYTE_AT_4: u32 = 0x0040_1170;
/// `004f15a0` (`ECX` = object, `RET 4`): stores the low byte of its argument at +4.
const SET_BYTE_AT_4: u32 = 0x004f_15a0;

/// `__RTDynamicCast` of `other` from `BaseFormComponent` to `BGSClipRoundsForm`.
fn cast_to_clip_rounds_form(e: &mut Engine, other: Ptr) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![
            other,
            0u32,
            TYPE_BASE_FORM_COMPONENT,
            TYPE_BGS_CLIP_ROUNDS_FORM,
            0u32
        ],
    )
    .u32()
}

// Translated from 00475120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSClipRoundsForm::CopyComponent` (Xbox PDB; `RET 4`): casts `other` to a
/// `BGSClipRoundsForm` and, when it is one, copies its clip rounds byte (+4, read through
/// `00401170` and stored through `004f15a0`).
pub fn bgs_clip_rounds_form_copy_component(e: &mut Engine, this: Ptr, other: Ptr) {
    let cast = cast_to_clip_rounds_form(e, other);
    if cast == 0 {
        return;
    }
    let rounds = e.call(BYTE_AT_4, &args![cast]).u32();
    e.call(SET_BYTE_AT_4, &args![this, rounds]);
}

// Translated from 00475170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSClipRoundsForm::CompareComponent` (Xbox PDB; `RET 4`, result in `AL`): true
/// ("differs") when `other` is not a `BGSClipRoundsForm` or its clip rounds byte (+4,
/// through `00401170`) differs from this one's.
pub fn bgs_clip_rounds_form_compare_component(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let cast = cast_to_clip_rounds_form(e, other);
    if cast == 0 {
        return true;
    }
    let ours = e.call(BYTE_AT_4, &args![this]).u32();
    let theirs = e.call(BYTE_AT_4, &args![cast]).u32();
    ours != theirs
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00475100,
            bgs_clip_rounds_form_initialize_data_component(Ptr)
        ),
        entry!(0x0047_5120, bgs_clip_rounds_form_copy_component(Ptr, Ptr)),
        entry!(
            0x0047_5170,
            bgs_clip_rounds_form_compare_component(Ptr, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_rounds_start_at_twelve() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0xffff_ffff);
        e.call(0x0047_5100, &args![this]);
        assert_eq!(e.mem.u32(this + 4), 0xffff_ff0c);
    }
}

#[cfg(test)]
mod tests_n036 {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(BYTE_AT_4, |e, a| u32::from(e.mem.u8(a[0] + 4)).into_ret());
        e.register(SET_BYTE_AT_4, |e, a| {
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            Ret::default()
        });
        e
    }

    #[test]
    fn copy_component_copies_the_rounds_of_a_clip_rounds_form() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        e.mem.set_u8(other + 4, 30);
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0047_5120, &args![this, other]);
        assert_eq!(e.mem.u8(this + 4), 0);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(
            RT_DYNAMIC_CAST,
            vec![
                other,
                0,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_BGS_CLIP_ROUNDS_FORM,
                0
            ]
        )));
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.call(0x0047_5120, &args![this, other]);
        assert_eq!(e.mem.u8(this + 4), 30);
    }

    #[test]
    fn compare_component_differs_unless_the_same_rounds() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        e.mem.set_u8(this + 4, 6);
        e.mem.set_u8(other + 4, 6);
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        assert!(!e.call(0x0047_5170, &args![this, other]).bool());
        e.mem.set_u8(other + 4, 7);
        assert!(e.call(0x0047_5170, &args![this, other]).bool());
        e.mem.set_u8(other + 4, 6);
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert!(e.call(0x0047_5170, &args![this, other]).bool());
    }
}
