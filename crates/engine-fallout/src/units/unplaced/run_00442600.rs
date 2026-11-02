//! `(unplaced)/run_00442600` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The semaphore-like member at +0x0c, `00559450` (`thiscall` on it; a
/// word) and `00442550` (`thiscall` on it; the engine map's name for it is
/// `ReleaseSemaphore`).
const MEMBER: u32 = 0x0c;
const MEMBER_VALUE: u32 = 0x0055_9450;
const MEMBER_RELEASE: u32 = 0x0044_2550;

// Translated from 00442600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `00442550` on the member at +0x0c when
/// `00559450` on it gives 0.
pub fn fn_00442600(e: &mut Engine, this: Ptr) {
    let member = this.addr().wrapping_add(MEMBER);
    if e.call(MEMBER_VALUE, &args![member]).u32() == 0 {
        e.call(MEMBER_RELEASE, &args![member]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0044_2600, fn_00442600(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00442600_releases_only_when_the_value_is_zero() {
        for (value, released) in [(0u32, true), (3, false)] {
            let mut e = Engine::new();
            e.register_double(MEMBER_VALUE, move |_, _| value.into_ret());
            e.register(MEMBER_RELEASE, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x0044_2600, &args![0x4000u32]);
            let log = e.call_log.take().unwrap();
            let releases: Vec<_> = log.iter().filter(|c| c.0 == MEMBER_RELEASE).collect();
            assert_eq!(releases.len(), usize::from(released));
            if released {
                assert_eq!(releases[0].1, [0x400c]);
            }
        }
    }
}
