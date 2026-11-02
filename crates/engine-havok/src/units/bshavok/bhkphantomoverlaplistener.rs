//! `BSHavok/bhkphantomoverlaplistener.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The pooled allocator object at `011f6238` and its release `(block)` method (`00aa4060`).
const POOL: u32 = 0x011f_6238;
const POOL_RELEASE: u32 = 0x00aa_4060;
/// The vtable the destructor leaves the object with (all purecall slots).
const VTABLE_PURECALL: u32 = 0x010c_925c;

// Translated from 00cbf6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpPhantomOverlapListener::_scalar_deleting_destructor_` (Ghidra name): sets the
/// vtable at `0x010c925c`, then releases the object's pool block (`00aa4060`) when bit 0 of
/// the low byte of `flags` is set. Answers `this`.
pub fn hkp_phantom_overlap_listener_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_PURECALL);
    if flags as u8 & 1 != 0 {
        e.call(POOL_RELEASE, &args![POOL, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cbf6e0, hkp_phantom_overlap_listener_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hkp_phantom_overlap_listener_scalar_deleting_destructor_releases_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(POOL_RELEASE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        let releases = |e: &Engine| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|call| call.0 == POOL_RELEASE)
                .map(|call| call.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00cbf6e0, &args![this, 0xfeu32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_PURECALL);
        assert!(releases(&e).is_empty());
        // Only the low byte counts.
        e.mem.set_u32(this, 0);
        assert_eq!(e.call(0x00cbf6e0, &args![this, 0x100u32]).u32(), this);
        assert!(releases(&e).is_empty());
        e.mem.set_u32(this, 0);
        assert_eq!(e.call(0x00cbf6e0, &args![this, 3u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_PURECALL);
        assert_eq!(releases(&e), vec![vec![POOL, this]]);
    }
}
