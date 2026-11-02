//! `(unplaced)/run_006ebf30` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0040f6e0` (`ECX` = the member at `+0x14` of the
/// stored object): the call the constructor makes on a non-null object (its
/// body calls `0040b460` on the member's `+4`; unconfirmed what it does).
const MEMBER_ACQUIRE: u32 = 0x0040_f6e0;

// Translated from 006ebf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map gives it no name. `thiscall` (`RET 4`): stores `object` at
/// `*this`; when it is not null, calls `0040f6e0` on the member at
/// `object + 0x14`. Returns `this`.
pub fn fn_006ebf60(e: &mut Engine, this: Ptr, object: u32) -> Ptr {
    e.mem.set_u32(this.addr(), object);
    if e.mem.u32(this.addr()) != 0 {
        e.call(MEMBER_ACQUIRE, &args![object + 0x14]);
    }
    this
}

/// `00401970` (`ECX` = the member at `+0x14` of the stored object): the
/// call the destructor makes on a non-null object (unconfirmed what it
/// does).
const MEMBER_RELEASE: u32 = 0x0040_1970;

// Translated from 006ebf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map gives it no name. `thiscall` (`RET 4`), the copy form of
/// [`fn_006ebf60`]: stores the object `*other` at `*this`; when it is not
/// null, calls `0040f6e0` on the member at `object + 0x14`. Returns `this`.
pub fn fn_006ebf30(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let object = e.mem.u32(other.addr());
    e.mem.set_u32(this.addr(), object);
    if e.mem.u32(this.addr()) != 0 {
        let member = e.mem.u32(this.addr()).wrapping_add(0x14);
        e.call(MEMBER_ACQUIRE, &args![member]);
    }
    this
}

// Translated from 006ebf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map gives it no name. The destructor of the same holder:
/// when the object at `*this` is not null, calls `00401970` on the member
/// at `object + 0x14`.
pub fn fn_006ebf90(e: &mut Engine, this: Ptr) {
    if e.mem.u32(this.addr()) != 0 {
        let member = e.mem.u32(this.addr()).wrapping_add(0x14);
        e.call(MEMBER_RELEASE, &args![member]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006ebf60, fn_006ebf60(Ptr, u32) -> Ptr),
        entry!(0x006ebf30, fn_006ebf30(Ptr, Ptr) -> Ptr),
        entry!(0x006ebf90, fn_006ebf90(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
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
    fn stores_the_object_and_acquires_its_member() {
        let mut e = Engine::new();
        e.register(MEMBER_ACQUIRE, |_, _| Ret::default());
        let slot = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        let back = e.call(0x006ebf60, &args![slot, 0x5000u32]).u32();
        assert_eq!(back, slot);
        assert_eq!(e.mem.u32(slot), 0x5000);
        assert_eq!(calls_to(&e, MEMBER_ACQUIRE), vec![vec![0x5014]]);
    }

    #[test]
    fn a_null_object_is_stored_without_a_call() {
        let mut e = Engine::new();
        e.register(MEMBER_ACQUIRE, |_, _| Ret::default());
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x006ebf60, &args![slot, 0u32]);
        assert_eq!(e.mem.u32(slot), 0);
        assert!(calls_to(&e, MEMBER_ACQUIRE).is_empty());
    }

    #[test]
    fn copy_takes_the_other_holders_object_and_acquires_it() {
        let mut e = Engine::new();
        e.register(MEMBER_ACQUIRE, |_, _| Ret::default());
        let slot = e.mem.alloc(4);
        let other = e.mem.alloc(4);
        e.mem.set_u32(other, 0x6000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006e_bf30, &args![slot, other]).u32(), slot);
        assert_eq!(e.mem.u32(slot), 0x6000);
        assert_eq!(calls_to(&e, MEMBER_ACQUIRE), vec![vec![0x6014]]);
        e.mem.set_u32(other, 0);
        e.call_log = Some(vec![]);
        e.call(0x006e_bf30, &args![slot, other]);
        assert_eq!(e.mem.u32(slot), 0);
        assert!(calls_to(&e, MEMBER_ACQUIRE).is_empty());
    }

    #[test]
    fn destructor_releases_only_a_held_object() {
        let mut e = Engine::new();
        e.register(MEMBER_RELEASE, |_, _| Ret::default());
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x006e_bf90, &args![slot]);
        assert_eq!(calls_to(&e, MEMBER_RELEASE), vec![vec![0x6014]]);
        e.mem.set_u32(slot, 0);
        e.call_log = Some(vec![]);
        e.call(0x006e_bf90, &args![slot]);
        assert!(calls_to(&e, MEMBER_RELEASE).is_empty());
    }
}
