//! `fallout shared/havok/run_00624020` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Offset of the link that `fn_00624020` follows.
const LINK: u32 = 0x0c;

// Translated from 00624020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Follows the word at `+0x0c` from `this` until it is 0 and returns the
/// last object reached (`this` itself when its link is already 0). The
/// callers use it on a collision body to reach the root of its parent chain
/// (`SpecificItemCollector::addRayHit` reads the collision filter of the
/// root); the exe does not name the field.
pub fn fn_00624020(e: &mut Engine, this: u32) -> u32 {
    let mut object = this;
    loop {
        let next = e.mem.u32(object + LINK);
        if next == 0 {
            return object;
        }
        object = next;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0062_4020, fn_00624020(u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reaches_the_end_of_the_chain() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0x20);
        let b = e.mem.alloc(0x20);
        let c = e.mem.alloc(0x20);
        e.mem.set_u32(a + 0x0c, b);
        e.mem.set_u32(b + 0x0c, c);
        assert_eq!(e.call(0x0062_4020, &args![a]).u32(), c);
        assert_eq!(e.call(0x0062_4020, &args![b]).u32(), c);
    }

    #[test]
    fn an_object_without_a_link_is_its_own_root() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0x20);
        assert_eq!(e.call(0x0062_4020, &args![a]).u32(), a);
    }
}
