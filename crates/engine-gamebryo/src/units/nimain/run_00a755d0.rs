//! `NiMain/run_00a755d0` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiFrustumPlanes::Set` (`thiscall` on the planes, two pointer words).
const FRUSTUM_PLANES_SET: u32 = 0x00a7_4e10;

// Translated from 00a755d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(object)`, `RET 4`; its one caller passes the global at
/// `011c6330` in ECX): `NiFrustumPlanes::Set` on `this` with the object's +0xDC
/// and +0x68.
pub fn fn_00a755d0(e: &mut Engine, this: Ptr, object: Ptr) {
    let object = object.addr();
    e.call(
        FRUSTUM_PLANES_SET,
        &args![this, object.wrapping_add(0xdc), object.wrapping_add(0x68)],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a755d0, fn_00a755d0(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00a755d0_sets_the_planes_from_the_camera() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(FRUSTUM_PLANES_SET, |_, _| Ret::default());
        e.call(0x00a755d0, &args![0x011c_6330u32, 0x5000u32]);
        let sets: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == FRUSTUM_PLANES_SET)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(sets, vec![vec![0x011c_6330, 0x50dc, 0x5068]]);
    }
}
