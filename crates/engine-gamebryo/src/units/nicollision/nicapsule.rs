//! `NiCollision/nicapsule.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSegment::operator==` (`00eb60e0`, thiscall on one segment, the other).
const NI_SEGMENT_EQUAL: u32 = 0x00eb_60e0;

// Translated from 00eae690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCapsule::operator==` (Xbox PDB): the segments must be equal (`NiSegment::operator==` on
/// this capsule's segment with the other's) and the radii (+0x18) equal (`FUCOMPP`, unordered is
/// unequal).
pub fn ni_capsule_operator_(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(NI_SEGMENT_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let mine = x87_load32(e.mem.u32(this.addr().wrapping_add(0x18)));
    let theirs = x87_load32(e.mem.u32(other.addr().wrapping_add(0x18)));
    theirs == mine
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ea_e690, ni_capsule_operator_(Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn needs_equal_segments_and_radii() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0x1c);
        let b = e.mem.alloc(0x1c);
        e.mem.set_f32(a + 0x18, 2.0);
        e.mem.set_f32(b + 0x18, 2.0);
        e.register(NI_SEGMENT_EQUAL, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ea_e690, &args![a, b]).bool());
        assert_eq!(
            e.call_log.as_ref().unwrap()[1],
            (NI_SEGMENT_EQUAL, vec![a, b])
        );
        e.mem.set_f32(b + 0x18, 2.5);
        assert!(!e.call(0x00ea_e690, &args![a, b]).bool());
        e.mem.set_f32(b + 0x18, 2.0);
        e.register(NI_SEGMENT_EQUAL, |_, _| Ret::default());
        assert!(!e.call(0x00ea_e690, &args![a, b]).bool());
    }
}
