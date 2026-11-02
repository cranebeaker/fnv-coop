//! `(unplaced)/run_00cbc460` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable the constructor stores: its slot 0 is `00cbc520`, which the engine map names
/// `hkpCharacterStateManager::_scalar_deleting_destructor_`.
const CHARACTER_STATE_MANAGER_VTABLE: u32 = 0x010c_9194;
/// `hkReferencedObject::addReference` (`00c90510`, `thiscall`; the engine map has no name for
/// it, its source unit is `hkreferencedobject.cpp`).
const REFERENCED_OBJECT_ADD_REFERENCE: u32 = 0x00c9_0510;
/// `hkReferencedObject::removeReference` (`00c905b0`, `thiscall`; unnamed in the engine map,
/// source unit `hkreferencedobject.cpp`).
const REFERENCED_OBJECT_REMOVE_REFERENCE: u32 = 0x00c9_05b0;

// Translated from 00cbc460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `__fastcall` constructor of the class whose vtable is
/// `010c9194` (`hkpCharacterStateManager` by the vtable's destructor entry): stores the vtable,
/// sets the 16-bit reference count at `+6` to 1 and clears the eleven dwords at `+8..=+0x30`
/// (the registered states). Returns `this`.
pub fn fn_00cbc460(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, CHARACTER_STATE_MANAGER_VTABLE);
    e.mem.set_u16(base.wrapping_add(6), 1);
    for offset in (8..=0x30).step_by(4) {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    this
}

// Translated from 00cbc4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 8` (an object and a slot number): adds a
/// reference to the object (`00c90510`, called even for a null object, which then faults as in
/// the exe), drops the reference to the object already in the slot `+8 + index * 4` (`00c905b0`)
/// when there is one, and stores the object in the slot. `hkpCharacterStateManager::
/// registerState` by its shape; unconfirmed.
pub fn fn_00cbc4e0(e: &mut Engine, this: Ptr, object: Ptr, index: u32) {
    e.call(REFERENCED_OBJECT_ADD_REFERENCE, &args![object]);
    let slot = this
        .addr()
        .wrapping_add(index.wrapping_mul(4))
        .wrapping_add(8);
    let old = e.mem.u32(slot);
    if old != 0 {
        e.call(REFERENCED_OBJECT_REMOVE_REFERENCE, &args![old]);
    }
    e.mem.set_u32(slot, object.addr());
}

// Translated from 00cbc4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): the destructor of the manager
/// [`fn_00cbc460`] builds. Sets the vtable `010c9194`, drops the reference of each of the 11
/// state objects at +8.. (`00c905b0`, skipping null slots, each slot then cleared), and sets
/// the base vtable `0102e388`.
pub fn fn_00cbc4a0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, CHARACTER_STATE_MANAGER_VTABLE);
    for index in 0..11u32 {
        let slot = base + 8 + 4 * index;
        let object = e.mem.u32(slot);
        if object != 0 {
            e.call(REFERENCED_OBJECT_REMOVE_REFERENCE, &args![object]);
            e.mem.set_u32(slot, 0);
        }
    }
    e.mem.set_u32(base, 0x0102_e388);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb_c460, fn_00cbc460(Ptr) -> Ptr),
        entry!(0x00cb_c4e0, fn_00cbc4e0(Ptr, Ptr, u32)),
        entry!(0x00cb_c4a0, fn_00cbc4a0(Ptr)),
    ]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn at_00cbc460_initialises_the_manager() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x38);
        for offset in (0..0x38).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00cb_c460, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x010c_9194);
        // Only the low 16 bits at +4 and the 16-bit count at +6 are in play.
        assert_eq!(e.mem.u32(this + 4), 0x0001_ffff);
        for offset in (8..=0x30).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x34), 0xffff_ffff);
    }

    #[test]
    fn at_00cbc4e0_stores_the_object_adding_a_reference_and_dropping_the_old_one() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(REFERENCED_OBJECT_ADD_REFERENCE, |_, _| Ret::default());
        e.register(REFERENCED_OBJECT_REMOVE_REFERENCE, |_, _| Ret::default());
        let this = e.mem.alloc(0x38);
        e.call(0x00cb_c4e0, &args![this, 0x7000u32, 3u32]);
        assert_eq!(e.mem.u32(this + 8 + 12), 0x7000);
        assert_eq!(
            calls(&e, REFERENCED_OBJECT_ADD_REFERENCE),
            vec![vec![0x7000]]
        );
        assert!(calls(&e, REFERENCED_OBJECT_REMOVE_REFERENCE).is_empty());
        e.call(0x00cb_c4e0, &args![this, 0x8000u32, 3u32]);
        assert_eq!(e.mem.u32(this + 8 + 12), 0x8000);
        assert_eq!(
            calls(&e, REFERENCED_OBJECT_REMOVE_REFERENCE),
            vec![vec![0x7000]]
        );
        // The neighbouring slots are untouched.
        assert_eq!(e.mem.u32(this + 8 + 8), 0);
        assert_eq!(e.mem.u32(this + 8 + 16), 0);
    }

    #[test]
    fn at_00cbc4e0_adds_the_reference_before_dropping_the_old_one() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(REFERENCED_OBJECT_ADD_REFERENCE, |_, _| Ret::default());
        e.register(REFERENCED_OBJECT_REMOVE_REFERENCE, |_, _| Ret::default());
        let this = e.mem.alloc(0x38);
        e.mem.set_u32(this + 8, 0x7000);
        // Storing the same object again still adds and then removes one reference.
        e.call(0x00cb_c4e0, &args![this, 0x7000u32, 0u32]);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| {
                *a == REFERENCED_OBJECT_ADD_REFERENCE || *a == REFERENCED_OBJECT_REMOVE_REFERENCE
            })
            .collect();
        assert_eq!(
            order,
            vec![
                REFERENCED_OBJECT_ADD_REFERENCE,
                REFERENCED_OBJECT_REMOVE_REFERENCE
            ]
        );
        assert_eq!(e.mem.u32(this + 8), 0x7000);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00cbc4a0_drops_every_state_and_sets_the_base_vtable() {
        let mut e = Engine::new();
        e.register(REFERENCED_OBJECT_REMOVE_REFERENCE, |_, _| Ret::default());
        let this = e.mem.alloc(0x34);
        e.mem.set_u32(this + 8, 0x7000);
        e.mem.set_u32(this + 8 + 4 * 10, 0x7a00);
        e.call_log = Some(vec![]);
        e.call(0x00cb_c4a0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (REFERENCED_OBJECT_REMOVE_REFERENCE, vec![0x7000]),
                (REFERENCED_OBJECT_REMOVE_REFERENCE, vec![0x7a00]),
            ]
        );
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x30), 0);
        assert_eq!(e.mem.u32(this), 0x0102_e388);
    }
}
