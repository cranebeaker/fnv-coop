//! `(unplaced)/run_00d4ded0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c90510` / `00c905b0` (unit `hkreferencedobject.cpp`; thiscall, no arguments): take and
/// drop a reference.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;

// Translated from 00d4ded0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00d4ded0 - the first word pushed to the object's slot +0x24 is a 1 byte over the rest of ECX (PUSH ECX then MOV byte ptr [ESP],1); after an old object was released ECX is what 00c905b0 left, which is not modelled (passed as 0 there; exact when no old object)
/// Unnamed in the engine map (nothing references it): sets the referenced object at +0x18:
/// a reference taken on the new one (when not null), the old one's dropped (when not null),
/// the new one stored; then for a non-null object its vtable slot +0x24 is called with a
/// byte 1 (by value) and the address of an 8-byte local, whose two words' low halves go to
/// the 16-bit fields at +0x1c and +0x1e.
pub fn fn_00d4ded0(e: &mut Engine, this: Ptr, object: Ptr) {
    let this = this.addr();
    let object = object.addr();
    if object != 0 {
        e.call(ADD_REFERENCE, &args![object]);
    }
    let old = e.mem.u32(this + 0x18);
    if old != 0 {
        e.call(REMOVE_REFERENCE, &args![old]);
    }
    e.mem.set_u32(this + 0x18, object);
    if object == 0 {
        return;
    }
    e.with_stack(8, |e, result| {
        let result = result.addr();
        // The byte 1 over ECX's upper bytes: 0 when there was no old object (ECX = 0), not
        // modelled after a release (see the incomplete note).
        let flag = 1u32;
        let current = e.mem.u32(this + 0x18);
        e.vcall(current, 0x24, &args![flag, result]);
        let first = e.mem.u16(result);
        let second = e.mem.u16(result + 4);
        e.mem.set_u16(this + 0x1c, first);
        e.mem.set_u16(this + 0x1e, second);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d4_ded0, fn_00d4ded0(Ptr, Ptr))]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    #[test]
    fn swaps_the_reference_and_reads_the_two_sizes() {
        let mut e = Engine::new();
        e.register(ADD_REFERENCE, |_, _| Ret::default());
        e.register(REMOVE_REFERENCE, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x28);
        e.mem.set_u32(vtable + 0x24, 0x0079_0024);
        e.register(0x0079_0024, |e, a| {
            assert_eq!(a[1] & 0xff, 1);
            e.mem.set_u32(a[2], 0x0001_0203);
            e.mem.set_u32(a[2] + 4, 0x0004_0506);
            Ret::default()
        });
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x18, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x00d4_ded0, &args![this, object]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ADD_REFERENCE, vec![object]));
        assert_eq!(log[2], (REMOVE_REFERENCE, vec![0x6000]));
        assert_eq!(e.mem.u32(this + 0x18), object);
        assert_eq!(e.mem.u16(this + 0x1c), 0x0203);
        assert_eq!(e.mem.u16(this + 0x1e), 0x0506);

        // A null object: only the old one is released.
        e.call_log = Some(vec![]);
        e.call(0x00d4_ded0, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[1], (REMOVE_REFERENCE, vec![object]));
        assert_eq!(e.mem.u32(this + 0x18), 0);
    }
}
