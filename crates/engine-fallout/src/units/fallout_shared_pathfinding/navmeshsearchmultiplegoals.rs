//! `fallout shared/pathfinding/navmeshsearchmultiplegoals.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator delete` (`00401030`, cdecl, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `006abfc0` (run_006abf60 unit): the destructor body of
/// `BSSimpleArray<PathingLocation,1024>`.
const LOCATION_ARRAY_DTOR_006ABFC0: u32 = 0x006a_bfc0;

// Translated from 006abfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathingLocation,1024>::scalar deleting destructor` (Xbox
/// PDB): `006abfc0`, then `operator delete` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn bs_simple_array_pathing_location_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(LOCATION_ARRAY_DTOR_006ABFC0, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x006a_bfe0,
        bs_simple_array_pathing_location_scalar_deleting_destructor(Ptr, u32) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_006abfe0_destroys_then_deletes_when_asked() {
        let mut e = Engine::new();
        e.register(LOCATION_ARRAY_DTOR_006ABFC0, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        for flags in [0u32, 1, 2, 3] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006a_bfe0, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            let destroyed: Vec<_> = log
                .iter()
                .filter(|c| c.0 == LOCATION_ARRAY_DTOR_006ABFC0)
                .collect();
            assert_eq!(destroyed.len(), 1);
            assert_eq!(destroyed[0].1, vec![this]);
            let deleted = log.iter().filter(|c| c.0 == OPERATOR_DELETE).count();
            assert_eq!(deleted, (flags & 1) as usize);
        }
    }
}
