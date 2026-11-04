//! `fallout shared/run_004b07b0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004b0850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004b07b0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004b0850(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004b_07b0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004b08e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `004b0cf0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_004b08e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004b_0cf0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

use crate::types::NiTArray;

/// The vtable of `NiTArray<BSAnimNoteReceiver::BSAnimNoteReceiverType *, ...>` (its class
/// confirmed by the RTTI before it), the array free `004ede70` (`cdecl`, `(block)`), and
/// the constructor steps `0096a2d0` and `00470470` (`thiscall`, no arguments).
const NOTE_RECEIVER_ARRAY_VTABLE: u32 = 0x0101_fc8c;
const ARRAY_FREE_004EDE70: u32 = 0x004e_de70;
const BASE_CONSTRUCTOR_0096A2D0: u32 = 0x0096_a2d0;
const CONSTRUCTOR_STEP_00470470: u32 = 0x0047_0470;

// Translated from 004b07b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTArray<BSAnimNoteReceiver::BSAnimNoteReceiverType *, ...>` destructor body
/// (unnamed in the map): the class's vtable and the element block (`m_pBase`) freed
/// (`004ede70`).
pub fn fn_004b07b0(e: &mut Engine, this: Ptr<NiTArray>) {
    e.mem.set_u32(this.addr(), NOTE_RECEIVER_ARRAY_VTABLE);
    let elements = e.get(this, NiTArray::m_pBase);
    e.call(ARRAY_FREE_004EDE70, &args![elements]);
}

// Translated from 004b07e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, no arguments; on an `NiTArray` of pointers): removes and
/// answers the last element: 0 when `m_usSize` is 0; otherwise the size drops by one, the
/// element there is taken and its slot cleared, and a non-null element lowers `m_usESize`.
pub fn fn_004b07e0(e: &mut Engine, this: Ptr<NiTArray>) -> u32 {
    let size = e.get(this, NiTArray::m_usSize);
    if size == 0 {
        return 0;
    }
    let size = size.wrapping_sub(1);
    e.set(this, NiTArray::m_usSize, size);
    let slot = e
        .get(this, NiTArray::m_pBase)
        .wrapping_add(u32::from(size) * 4);
    let element = e.mem.u32(slot);
    e.mem.set_u32(slot, 0);
    if element != 0 {
        let used = e.get(this, NiTArray::m_usESize).wrapping_sub(1);
        e.set(this, NiTArray::m_usESize, used);
    }
    element
}

// Translated from 004b0880 (decompiled, FalloutNV.exe 1.4.0.525)
/// A constructor (unnamed in the map; `thiscall`, no arguments): `0096a2d0` then
/// `00470470` on `this`. Answers `this`.
// Translation incomplete: 004b0880 - SEH exception unwinding is unsupported.
pub fn fn_004b0880(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCTOR_0096A2D0, &args![this]);
    e.call(CONSTRUCTOR_STEP_00470470, &args![this]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004b_0850, fn_004b0850(Ptr, u32) -> Ptr),
        entry!(0x004b_08e0, fn_004b08e0(Ptr, u32) -> Ptr),
        entry!(0x004b07b0, fn_004b07b0(Ptr<NiTArray>)),
        entry!(0x004b07e0, fn_004b07e0(Ptr<NiTArray>) -> u32),
        entry!(0x004b0880, fn_004b0880(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004b0850_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004b_07b0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004b_0850, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004b_0850)
                .collect();
            let mut expected = vec![(0x004b_07b0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_004b08e0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004b_0cf0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004b_08e0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004b_08e0)
                .collect();
            let mut expected = vec![(0x004b_0cf0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn note_receiver_array_destructor_body_frees_the_elements() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(ARRAY_FREE_004EDE70, |_, _| Ret::default());
        let this = e.new_object::<NiTArray>();
        e.set(this, NiTArray::m_pBase, 0x7000);
        e.call(0x004b07b0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), NOTE_RECEIVER_ARRAY_VTABLE);
        let frees: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == ARRAY_FREE_004EDE70)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(frees, vec![vec![0x7000]]);
    }

    #[test]
    fn fn_004b07e0_removes_the_last_element() {
        let mut e = Engine::new();
        let this = e.new_object::<NiTArray>();
        let base = e.mem.alloc(12);
        e.mem.set_u32(base, 0x11);
        e.mem.set_u32(base + 8, 0x33);
        e.set(this, NiTArray::m_pBase, base);
        e.set(this, NiTArray::m_usSize, 3);
        e.set(this, NiTArray::m_usESize, 2);
        assert_eq!(e.call(0x004b07e0, &args![this]).u32(), 0x33);
        assert_eq!(e.mem.u32(base + 8), 0);
        assert_eq!(e.get(this, NiTArray::m_usESize), 1);
        // A null element leaves the used count.
        assert_eq!(e.call(0x004b07e0, &args![this]).u32(), 0);
        assert_eq!(e.get(this, NiTArray::m_usESize), 1);
        assert_eq!(e.call(0x004b07e0, &args![this]).u32(), 0x11);
        assert_eq!(e.get(this, NiTArray::m_usSize), 0);
        assert_eq!(e.call(0x004b07e0, &args![this]).u32(), 0);
        assert_eq!(e.get(this, NiTArray::m_usSize), 0);
    }

    #[test]
    fn fn_004b0880_runs_both_constructor_steps() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(BASE_CONSTRUCTOR_0096A2D0, |_, _| Ret::default());
        e.register(CONSTRUCTOR_STEP_00470470, |_, _| Ret::default());
        assert_eq!(e.call(0x004b0880, &args![0x5000u32]).u32(), 0x5000);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (BASE_CONSTRUCTOR_0096A2D0, vec![0x5000]),
                (CONSTRUCTOR_STEP_00470470, vec![0x5000])
            ]
        );
    }
}
