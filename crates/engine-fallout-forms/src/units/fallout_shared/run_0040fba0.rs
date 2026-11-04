//! `fallout shared/run_0040fba0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// An empty function (`0040fbe0`) that the lock calls when it releases and
/// after it acquires (`0040fbf0`).
const BEFORE_RELEASE: u32 = 0x0040_fbe0;

// Translated from 0040fba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the recursive lock taken by `0040fbf0`: decrements the count at
/// `+4` and, when it reaches zero, clears the owner word at `+0`.
pub fn fn_0040fba0(e: &mut Engine, this: Ptr) {
    e.call(BEFORE_RELEASE, &args![]);
    let count = e.mem.u32(this.addr() + 4).wrapping_sub(1);
    e.mem.set_u32(this.addr() + 4, count);
    if count == 0 {
        e.mem.set_u32(this.addr(), 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0040fba0, fn_0040fba0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lock(e: &mut Engine, owner: u32, count: u32) -> Ptr {
        e.register(BEFORE_RELEASE, |_, _| Ret::default());
        let lock: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(lock.addr(), owner);
        e.mem.set_u32(lock.addr() + 4, count);
        lock
    }

    #[test]
    fn a_nested_release_keeps_the_owner() {
        let mut e = Engine::new();
        let lock = lock(&mut e, 7, 2);
        e.call(0x0040_fba0, &args![lock]);
        assert_eq!(e.mem.u32(lock.addr() + 4), 1);
        assert_eq!(e.mem.u32(lock.addr()), 7);
    }

    #[test]
    fn the_last_release_frees_the_lock() {
        let mut e = Engine::new();
        let lock = lock(&mut e, 7, 1);
        e.call_log = Some(vec![]);
        e.call(0x0040_fba0, &args![lock]);
        assert_eq!(e.mem.u32(lock.addr() + 4), 0);
        assert_eq!(e.mem.u32(lock.addr()), 0);
        assert_eq!(e.call_log.take().unwrap()[1].0, BEFORE_RELEASE);
    }
}
