//! `(unplaced)/run_00ec1a00` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `ReleaseSemaphore` (kernel32 import slot `00fdf1b4`).
const RELEASE_SEMAPHORE: u32 = 0x00fd_f1b4;
/// `InterlockedIncrement` (kernel32 import slot `00fdf1c0`).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;

// Translated from 00ec1a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): releases the
/// semaphore whose handle is at `this + 0x34` by one
/// (`ReleaseSemaphore(handle, 1, NULL)`), increments the counter at
/// `this + 0x30` (`InterlockedIncrement`) and sets the byte at `this + 0x3c`
/// to 1.
pub fn fn_00ec1a00(e: &mut Engine, this: Ptr) {
    let counter = this.addr().wrapping_add(0x30);
    let semaphore = e.mem.u32(counter.wrapping_add(4));
    e.call(RELEASE_SEMAPHORE, &args![semaphore, 1u32, 0u32]);
    e.call(INTERLOCKED_INCREMENT, &args![counter]);
    e.mem.set_u8(this.addr().wrapping_add(0x3c), 1);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ec_1a00, fn_00ec1a00(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signals_the_semaphore_counts_and_sets_the_flag() {
        let mut e = Engine::new();
        e.register(RELEASE_SEMAPHORE, |_, _| 1u32.into_ret());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x30, 4);
        e.mem.set_u32(this + 0x34, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x00ec_1a00, &args![this]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 != 0x00ec_1a00).cloned().collect();
        assert_eq!(
            calls,
            vec![
                (RELEASE_SEMAPHORE, vec![0x77, 1, 0]),
                (INTERLOCKED_INCREMENT, vec![this + 0x30]),
            ]
        );
        assert_eq!(e.mem.u32(this + 0x30), 5);
        assert_eq!(e.mem.u8(this + 0x3c), 1);
    }
}
