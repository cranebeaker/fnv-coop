//! `fallout shared/run_004a4ba0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004a4ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004a4bd0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004a4ba0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004a_4bd0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// The vtable of `BGSPrimitive` (RTTI `.?AVBGSPrimitive@@`).
const BGS_PRIMITIVE_VTABLE: u32 = 0x0101_e72c;
/// `NiPointer` assignment (`0066b0d0`, thiscall on the pointer, the new
/// value: releases the old object, counts the new one).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiPointer`'s destructor (`0045cec0`, thiscall on the pointer: releases
/// a non-null object).
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// The three `NiPointer` members (+0x24, +0x28, +0x2c, as the code
/// addresses them).
const POINTER_24: u32 = 0x24;
const POINTER_28: u32 = 0x28;
const POINTER_2C: u32 = 0x2c;

// Translated from 004a4bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a4bd0 - the function's C++ exception frame (FS:[0], unwind states of the three members) is not translated
/// `BGSPrimitive`'s destructor body (by the vtable it stores; Ghidra's
/// library match is only the shape): the vtable, the three `NiPointer`
/// members cleared (`004a4c50`), then their destructors in reverse order
/// (+0x2c, +0x28, +0x24).
pub fn fn_004a4bd0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, BGS_PRIMITIVE_VTABLE);
    fn_004a4c50(e, this);
    e.call(NI_POINTER_DESTRUCT, &args![base + POINTER_2C]);
    e.call(NI_POINTER_DESTRUCT, &args![base + POINTER_28]);
    e.call(NI_POINTER_DESTRUCT, &args![base + POINTER_24]);
}

// Translated from 004a4c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the three `NiPointer` members at +0x24, +0x28 and +0x2c (in that
/// order, each assigned null through `0066b0d0`); unnamed in the engine map.
pub fn fn_004a4c50(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for member in [POINTER_24, POINTER_28, POINTER_2C] {
        e.call(NI_POINTER_ASSIGN, &args![base + member, 0u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004a_4ba0, fn_004a4ba0(Ptr, u32) -> Ptr),
        entry!(0x004a_4bd0, fn_004a4bd0(Ptr)),
        entry!(0x004a_4c50, fn_004a4c50(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004a4ba0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004a_4bd0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004a_4ba0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004a_4ba0)
                .collect();
            let mut expected = vec![(0x004a_4bd0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    fn calls_except(e: &Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a != top)
            .cloned()
            .collect()
    }

    #[test]
    fn fn_004a4c50_assigns_null_to_the_three_pointers() {
        let mut e = Engine::new();
        e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(0x004a_4c50, &args![this]);
        assert_eq!(
            calls_except(&e, 0x004a_4c50),
            vec![
                (NI_POINTER_ASSIGN, vec![this + 0x24, 0]),
                (NI_POINTER_ASSIGN, vec![this + 0x28, 0]),
                (NI_POINTER_ASSIGN, vec![this + 0x2c, 0]),
            ]
        );
    }

    #[test]
    fn fn_004a4bd0_sets_the_vtable_clears_then_destroys_the_pointers() {
        let mut e = Engine::new();
        e.register(NI_POINTER_ASSIGN, |_, _| Ret::default());
        e.register(NI_POINTER_DESTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        e.call(0x004a_4bd0, &args![this]);
        assert_eq!(e.mem.u32(this), BGS_PRIMITIVE_VTABLE);
        assert_eq!(
            calls_except(&e, 0x004a_4bd0),
            vec![
                (NI_POINTER_ASSIGN, vec![this + 0x24, 0]),
                (NI_POINTER_ASSIGN, vec![this + 0x28, 0]),
                (NI_POINTER_ASSIGN, vec![this + 0x2c, 0]),
                (NI_POINTER_DESTRUCT, vec![this + 0x2c]),
                (NI_POINTER_DESTRUCT, vec![this + 0x28]),
                (NI_POINTER_DESTRUCT, vec![this + 0x24]),
            ]
        );
    }
}
