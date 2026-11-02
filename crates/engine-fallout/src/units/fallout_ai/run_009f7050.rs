//! `fallout/ai/run_009f7050` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `SpectatorThreatInfo::LastKnownPosSource` (Xbox PDB) +0x14 and
/// `LastKnownPosTarget` +0x20 (three words each).
const SOURCE_POSITION: u32 = 0x14;
const TARGET_POSITION: u32 = 0x20;

fn copy_point(e: &mut Engine, to: u32, from: u32) -> Ptr {
    for i in 0..3 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
    Ptr::new(to)
}

// Translated from 009f7050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`SpectatorThreatInfo`; `RET 4`): copies the
/// last known source position (+0x14) into `out` and returns `out`.
pub fn fn_009f7050(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    copy_point(e, out.addr(), this.addr() + SOURCE_POSITION)
}

// Translated from 009f7080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`SpectatorThreatInfo`; `RET 4`): copies the
/// last known target position (+0x20) into `out` and returns `out`.
pub fn fn_009f7080(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    copy_point(e, out.addr(), this.addr() + TARGET_POSITION)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009f_7050, fn_009f7050(Ptr, Ptr) -> Ptr),
        entry!(0x009f_7080, fn_009f7080(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_009f7050_and_009f7080_copy_the_positions() {
        let mut e = Engine::new();
        let threat = e.mem.alloc(0x34);
        for i in 0..6 {
            e.mem.set_u32(threat + 0x14 + 4 * i, 0x100 + i);
        }
        let out = e.mem.alloc(12);
        assert_eq!(e.call(0x009f_7050, &args![threat, out]).u32(), out);
        assert_eq!(
            (e.mem.u32(out), e.mem.u32(out + 4), e.mem.u32(out + 8)),
            (0x100, 0x101, 0x102)
        );
        assert_eq!(e.call(0x009f_7080, &args![threat, out]).u32(), out);
        assert_eq!(
            (e.mem.u32(out), e.mem.u32(out + 4), e.mem.u32(out + 8)),
            (0x103, 0x104, 0x105)
        );
    }
}
