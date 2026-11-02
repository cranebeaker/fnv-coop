//! `Havok SDK/hkwin32thread.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The thread wrapper's handle closer (`00d87920`, `__fastcall` on the wrapper: when its word
/// at `+0` is a handle it calls `CloseHandle` and clears the word). The engine map names
/// it `CloseHandle`; the destructor below is a jump to it.
const CLOSE_HANDLE_WRAPPER: u32 = 0x00d8_7920;

// Translated from 00d87a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkThread::~hkThread` (engine map name), `__fastcall`: a jump to the handle closer
/// (`00d87920`) with the same `this`; whatever it leaves in EAX is not meaningful.
pub fn hk_thread_dtor(e: &mut Engine, this: Ptr) {
    e.call(CLOSE_HANDLE_WRAPPER, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d8_7a70, hk_thread_dtor(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_destructor_runs_the_handle_closer_on_the_same_object() {
        let mut e = Engine::new();
        e.register(CLOSE_HANDLE_WRAPPER, |e, a| {
            // The closer clears the wrapper's handle word.
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let thread = e.mem.alloc(8);
        e.mem.set_u32(thread, 0x1234);
        e.call(0x00d8_7a70, &args![thread]);
        assert_eq!(e.mem.u32(thread), 0);
        let log = e.call_log.as_ref().unwrap();
        let closes: Vec<_> = log.iter().filter(|c| c.0 == CLOSE_HANDLE_WRAPPER).collect();
        assert_eq!(closes.len(), 1);
        assert_eq!(closes[0].1, vec![thread]);
    }
}
