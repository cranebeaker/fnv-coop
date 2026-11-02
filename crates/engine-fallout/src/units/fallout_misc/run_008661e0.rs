//! `fallout/misc/run_008661e0` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008661e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards all four cdecl arguments to the executable's checked move routine.
pub fn move_bytes(e: &mut Engine, destination: Ptr, capacity: u32, source: Ptr, size: u32) {
    e.call(0x00ec7ce3, &args![destination, capacity, source, size]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008661e0, move_bytes(Ptr, u32, Ptr, u32))]
}
