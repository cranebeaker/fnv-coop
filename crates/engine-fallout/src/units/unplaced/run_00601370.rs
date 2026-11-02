//! `(unplaced)/run_00601370` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The `TESNPC` destructor (the vtable it stores first, `0104a2f4`, has the
//! RTTI name `.?AVTESNPC@@`), its scalar deleting form and the destructor of
//! the `NiTArray` member at +0x1FC. The engine map gives none of them a name.

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtables the destructor stores, by offset (`TESNPC` and its
/// component bases).
const VTABLES: [(u32, u32); 15] = [
    (0x000, 0x0104_a2f4),
    (0x030, 0x0104_a284),
    (0x064, 0x0104_a270),
    (0x070, 0x0104_a25c),
    (0x07c, 0x0104_a23c),
    (0x090, 0x0104_a21c),
    (0x0b0, 0x0104_a204),
    (0x0b8, 0x0104_a1f0),
    (0x0c4, 0x0104_a1dc),
    (0x0d0, 0x0104_a1c8),
    (0x0dc, 0x0104_a1a4),
    (0x0f4, 0x0104_a190),
    (0x100, 0x0104_a160),
    (0x104, 0x0104_a14c),
    (0x10c, 0x0104_a138),
];
/// `00601750` (`thiscall`, no Xbox PDB name): the first clean-up step.
const CLEAN_UP_00601750: u32 = 0x0060_1750;
/// `00461580` (`thiscall`, one word): whether the word at +4 has any of the
/// given bits.
const FLAGS_TEST_00461580: u32 = 0x0046_1580;
/// `00905330` (`thiscall`, one word: the address of a word holding the
/// form; no Xbox PDB name), on the object at `011cb764`.
const REMOVE_00905330: u32 = 0x0090_5330;
/// The object `00905330` runs on.
const OBJECT_011CB764: u32 = 0x011c_b764;
/// `0060b9b0` (`thiscall`): the destructor of an `NiTArray` whose vtable is
/// `0104aae0` (RTTI `.?AV?$NiTArray@PAUFaceGenUndo@@...`).
const FACE_GEN_UNDO_ARRAY_DESTROY: u32 = 0x0060_b9b0;
/// `0046ffb0` (`thiscall`): runs `00470470` on `this` (a simple list's
/// clean-up).
const LIST_DESTROY_0046FFB0: u32 = 0x0046_ffb0;
/// `0045cec0` (`thiscall`): when the word at `this` is set, runs `00401970`
/// on it.
const RELEASE_0045CEC0: u32 = 0x0045_cec0;
/// `` `eh vector destructor iterator' `` (`00ec5fce`, `stdcall`: array,
/// element size, count, destructor).
const EH_VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// The element destructor passed for the array at +0x134.
const ELEMENT_DESTRUCTOR_00449680: u32 = 0x0044_9680;
/// `005f77b0` (`thiscall`): the base destructor (engine map unit
/// `fallout shared/tescreature.cpp`, no name).
const BASE_DESTRUCTOR_005F77B0: u32 = 0x005f_77b0;
/// The game's `operator delete` (`00401030`, `cdecl`).
const FREE: u32 = 0x0040_1030;

// Translated from 006013c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006013c0 - the C++ exception unwinding frame (FS:[0], handler 00efacf6) is not translated
/// The `TESNPC` destructor (`thiscall`): stores the class's 15 vtables,
/// runs `00601750`; when the word at +0x34 has bit 0x4 (`00461580(this+0x30,
/// 4)`), hands the form to `00905330` on `011cb764`; then destroys the
/// members at +0x1FC ([`fn_006013a0`]), +0x1DC (`0046ffb0`), +0x1CC,
/// +0x1C8, +0x1C4 (`0045cec0`) and the four 0x20-byte elements at +0x134
/// (`eh vector destructor iterator` with `00449680`), and runs the base
/// destructor `005f77b0`. The exception-unwinding frame is not translated.
pub fn fn_006013c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for (offset, vtable) in VTABLES.iter() {
        e.mem.set_u32(base + offset, *vtable);
    }
    e.call(CLEAN_UP_00601750, &args![this]);
    if e.call(FLAGS_TEST_00461580, &args![base + 0x30, 4u32])
        .bool()
    {
        e.with_stack(4, |e, form| {
            e.mem.set_u32(form.addr(), base);
            e.call(REMOVE_00905330, &args![OBJECT_011CB764, form]);
        });
    }
    fn_006013a0(e, Ptr::new(base + 0x1fc));
    e.call(LIST_DESTROY_0046FFB0, &args![base + 0x1dc]);
    e.call(RELEASE_0045CEC0, &args![base + 0x1cc]);
    e.call(RELEASE_0045CEC0, &args![base + 0x1c8]);
    e.call(RELEASE_0045CEC0, &args![base + 0x1c4]);
    e.call(
        EH_VECTOR_DESTRUCTOR_ITERATOR,
        &args![base + 0x134, 0x20u32, 4u32, ELEMENT_DESTRUCTOR_00449680],
    );
    e.call(BASE_DESTRUCTOR_005F77B0, &args![this]);
}

// Translated from 006013a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the `NiTArray` member at `this` (`thiscall`): `0060b9b0(this)`.
pub fn fn_006013a0(e: &mut Engine, this: Ptr) {
    e.call(FACE_GEN_UNDO_ARRAY_DESTROY, &args![this]);
}

// Translated from 00601370 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESNPC` scalar deleting destructor (`thiscall`, one word: flags;
/// referenced from the `TESNPC` vtable at `0104a304`): [`fn_006013c0`], then
/// with flag bit 0 the memory is freed (`00401030`). Returns `this`.
pub fn fn_00601370(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006013c0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0060_13c0, fn_006013c0(Ptr)),
        entry!(0x0060_13a0, fn_006013a0(Ptr)),
        entry!(0x0060_1370, fn_00601370(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn doubles(e: &mut Engine, flagged: bool) {
        for callee in [
            CLEAN_UP_00601750,
            FACE_GEN_UNDO_ARRAY_DESTROY,
            LIST_DESTROY_0046FFB0,
            RELEASE_0045CEC0,
            EH_VECTOR_DESTRUCTOR_ITERATOR,
            BASE_DESTRUCTOR_005F77B0,
            FREE,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register_double(FLAGS_TEST_00461580, move |_, a| {
            assert_eq!(a[1], 4);
            word(u32::from(flagged))
        });
        e.register(REMOVE_00905330, |e, a| {
            assert_eq!(a[0], OBJECT_011CB764);
            word(e.mem.u32(a[1]))
        });
    }

    fn calls(log: &[(u32, Vec<u32>)]) -> Vec<(u32, Vec<u32>)> {
        log.iter()
            .filter(|c| c.0 != FLAGS_TEST_00461580)
            .skip(1)
            .cloned()
            .collect()
    }

    #[test]
    fn fn_006013c0_destroys_the_members_in_order() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x200);
        doubles(&mut e, true);
        e.call_log = Some(vec![]);
        e.call(0x0060_13c0, &args![this]);
        let log = e.call_log.take().unwrap();
        for (offset, vtable) in VTABLES.iter() {
            assert_eq!(e.mem.u32(this + offset), *vtable, "vtable at {offset:#x}");
        }
        let removed = log.iter().find(|c| c.0 == REMOVE_00905330).unwrap();
        assert_eq!(removed.1[0], OBJECT_011CB764);
        assert_eq!(
            calls(&log)
                .into_iter()
                .filter(|c| c.0 != REMOVE_00905330)
                .collect::<Vec<_>>(),
            vec![
                (CLEAN_UP_00601750, vec![this]),
                (FACE_GEN_UNDO_ARRAY_DESTROY, vec![this + 0x1fc]),
                (LIST_DESTROY_0046FFB0, vec![this + 0x1dc]),
                (RELEASE_0045CEC0, vec![this + 0x1cc]),
                (RELEASE_0045CEC0, vec![this + 0x1c8]),
                (RELEASE_0045CEC0, vec![this + 0x1c4]),
                (
                    EH_VECTOR_DESTRUCTOR_ITERATOR,
                    vec![this + 0x134, 0x20, 4, ELEMENT_DESTRUCTOR_00449680]
                ),
                (BASE_DESTRUCTOR_005F77B0, vec![this]),
            ]
        );

        let mut e = Engine::new();
        let this = e.mem.alloc(0x200);
        doubles(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x0060_13c0, &args![this]);
        assert!(e
            .call_log
            .take()
            .unwrap()
            .iter()
            .all(|c| c.0 != REMOVE_00905330));
    }

    #[test]
    fn fn_006013a0_destroys_the_array() {
        let mut e = Engine::new();
        e.register(FACE_GEN_UNDO_ARRAY_DESTROY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0060_13a0, &args![0x500u32]);
        assert_eq!(
            e.call_log.take().unwrap()[1],
            (FACE_GEN_UNDO_ARRAY_DESTROY, vec![0x500])
        );
    }

    #[test]
    fn fn_00601370_frees_only_with_bit_0() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x200);
        doubles(&mut e, false);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0060_1370, &args![this, 1u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.last().unwrap(), &(FREE, vec![this]));
        e.call_log = Some(vec![]);
        e.call(0x0060_1370, &args![this, 2u32]);
        assert!(e.call_log.take().unwrap().iter().all(|c| c.0 != FREE));
    }
}
