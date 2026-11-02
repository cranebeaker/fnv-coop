//! `NiSystem/run_00aa2710` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Import slot of `SuspendThread` (`stdcall`, the thread handle).
const API_SUSPEND_THREAD: u32 = 0x00fd_f15c;
/// The thread object's handle (+0x24) and status (+0x1c): `m_hThread` and
/// `m_eStatus` in the Xbox PDB's `NiThread`, whose offsets the code matches.
const THREAD_HANDLE: u32 = 0x24;
const THREAD_STATUS: u32 = 0x1c;

// Translated from 00aa2710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; `00aa3040` jumps
/// here): suspends the thread. Without a handle at +0x24 it answers -1;
/// otherwise it answers `SuspendThread(handle)` and, when that is not -1,
/// sets the status at +0x1c to 1.
pub fn fn_00aa2710(e: &mut Engine, this: Ptr) -> u32 {
    let handle = e.mem.u32(this.addr().wrapping_add(THREAD_HANDLE));
    if handle == 0 {
        return 0xffff_ffff;
    }
    let count = e.call(API_SUSPEND_THREAD, &args![handle]).u32();
    if count != 0xffff_ffff {
        e.mem.set_u32(this.addr().wrapping_add(THREAD_STATUS), 1);
    }
    count
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa_2710, fn_00aa2710(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00aa2710_suspends_and_marks_the_thread() {
        let mut e = Engine::new();
        e.register(API_SUSPEND_THREAD, |_, a| {
            if a[0] == 0x99 {
                0xffff_ffffu32.into_ret()
            } else {
                2u32.into_ret()
            }
        });
        let this = e.mem.alloc(0x2c);
        assert_eq!(e.call(0x00aa_2710, &args![this]).u32(), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + THREAD_STATUS), 0);
        e.mem.set_u32(this + THREAD_HANDLE, 0x99);
        assert_eq!(e.call(0x00aa_2710, &args![this]).u32(), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + THREAD_STATUS), 0);
        e.mem.set_u32(this + THREAD_HANDLE, 0x44);
        assert_eq!(e.call(0x00aa_2710, &args![this]).u32(), 2);
        assert_eq!(e.mem.u32(this + THREAD_STATUS), 1);
    }
}
