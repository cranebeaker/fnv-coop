//! `fallout/ai/run_0092c820` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0040b460` (unnamed in the engine map, `magicitem.cpp` range): `cdecl`, one stack word.
const CALLEE_0040B460: u32 = 0x0040_b460;

// Translated from 0092c870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `__fastcall`: calls `0040b460` (`cdecl`) with the address of the
/// member at `this + 8`.
pub fn fn_0092c870(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_0040B460, &args![this.byte_add(8)]);
}

/// `0044dd60` (unit `run_0044d8c0`, `thiscall`): the release of the counted
/// object a pointer holds.
const RELEASE: u32 = 0x0044_dd60;

// Translated from 0092c820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the smart-pointer assignment from another
/// smart pointer. When the two differ, the held object (if any) is
/// released (`0044dd60`), the other's object is taken and, when non-null,
/// counted (`0092c870`). Returns `this`.
pub fn fn_0092c820(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let slot = this.addr();
    if e.mem.u32(slot) != e.mem.u32(other.addr()) {
        let held = e.mem.u32(slot);
        if held != 0 {
            e.call(RELEASE, &args![held]);
        }
        let new = e.mem.u32(other.addr());
        e.mem.set_u32(slot, new);
        let held = e.mem.u32(slot);
        if held != 0 {
            fn_0092c870(e, Ptr::new(held));
        }
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0092_c870, fn_0092c870(Ptr)),
        entry!(0x0092_c820, fn_0092c820(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_0092c870_passes_the_address_of_the_member_at_offset_8() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(CALLEE_0040B460, |_, _| Ret::default());
        e.call(0x0092_c870, &args![0x4000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == CALLEE_0040B460)
            .map(|c| &c.1)
            .collect();
        assert_eq!(hits, vec![&vec![0x4008]]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_0092c820_releases_the_old_object_and_counts_the_new() {
        let mut e = Engine::new();
        e.register(RELEASE, |_, _| Ret::default());
        e.register(CALLEE_0040B460, |_, _| Ret::default());
        let slot = e.mem.alloc(4);
        let other = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x4000);
        e.mem.set_u32(other, 0x5000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0092_c820, &args![slot, other]).u32(), slot);
        assert_eq!(e.mem.u32(slot), 0x5000);
        let log = e.call_log.take().unwrap();
        let picked: Vec<_> = log
            .iter()
            .filter(|c| c.0 == RELEASE || c.0 == CALLEE_0040B460)
            .cloned()
            .collect();
        assert_eq!(
            picked,
            [(RELEASE, vec![0x4000]), (CALLEE_0040B460, vec![0x5008])]
        );
        // Equal pointers: nothing; a null new pointer is not counted.
        e.call_log = Some(vec![]);
        e.call(0x0092_c820, &args![slot, other]);
        e.mem.set_u32(other, 0);
        e.call(0x0092_c820, &args![slot, other]);
        let log = e.call_log.take().unwrap();
        let picked: Vec<_> = log
            .iter()
            .filter(|c| c.0 == RELEASE || c.0 == CALLEE_0040B460)
            .cloned()
            .collect();
        assert_eq!(picked, [(RELEASE, vec![0x5000])]);
        assert_eq!(e.mem.u32(slot), 0);
    }
}
