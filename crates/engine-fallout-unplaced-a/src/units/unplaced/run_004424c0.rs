//! `(unplaced)/run_004424c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004424e0` (the engine map's `WaitForSingleObject`, `XAPILIB`):
/// thiscall, no stack words.
const WAIT_FOR_SINGLE_OBJECT: u32 = 0x0044_24e0;

// Translated from 004424c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Waits on the object embedded at `this + 0xC`: `004424e0` with that
/// address as `this`. Returns nothing.
pub fn fn_004424c0(e: &mut Engine, this: Ptr) {
    e.call(
        WAIT_FOR_SINGLE_OBJECT,
        &args![this.addr().wrapping_add(0xc)],
    );
}

/// The counted semaphore's release (`00442550`, the platform's; `thiscall`, no stack words).
const SEMAPHORE_RELEASE: u32 = 0x0044_2550;

// Translated from 00442510 (disassembled, FalloutNV.exe 1.4.0.525)
/// `004424e0` (the counted semaphore's wait) on the object at +0x18 (`thiscall`, no stack words; its `EAX` answers).
pub fn fn_00442510(e: &mut Engine, this: Ptr) -> u32 {
    e.call(WAIT_FOR_SINGLE_OBJECT, &args![this.addr() + 0x18])
        .u32()
}

// Translated from 00442530 (disassembled, FalloutNV.exe 1.4.0.525)
/// `00442550` (its release) on the object at +0x18 (`thiscall`, no stack words; its `EAX` answers).
pub fn fn_00442530(e: &mut Engine, this: Ptr) -> u32 {
    e.call(SEMAPHORE_RELEASE, &args![this.addr() + 0x18]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0044_24c0, fn_004424c0(Ptr)),
        entry!(0x0044_2510, fn_00442510(Ptr) -> u32),
        entry!(0x0044_2530, fn_00442530(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn waits_on_the_object_at_0xc() {
        let mut e = Engine::new();
        e.register(WAIT_FOR_SINGLE_OBJECT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0044_24c0, &args![0x1000u32]);
        let log = e.call_log.take().unwrap();
        let waits: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == WAIT_FOR_SINGLE_OBJECT)
            .collect();
        assert_eq!(waits.len(), 1);
        assert_eq!(waits[0].1, vec![0x100c]);
    }
}

#[cfg(test)]
mod xapilib_tests {
    use super::*;

    #[test]
    fn the_wait_and_the_release_go_to_the_object_at_0x18() {
        let mut e = Engine::new();
        e.register(WAIT_FOR_SINGLE_OBJECT, |_, _| 7u32.into_ret());
        e.register(SEMAPHORE_RELEASE, |_, _| 8u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_2510, &args![0x1000u32]).u32(), 7);
        assert_eq!(e.call(0x0044_2530, &args![0x1000u32]).u32(), 8);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|c| c.0 == WAIT_FOR_SINGLE_OBJECT && c.1 == vec![0x1018]));
        assert!(log
            .iter()
            .any(|c| c.0 == SEMAPHORE_RELEASE && c.1 == vec![0x1018]));
    }
}
