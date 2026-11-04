//! `fallout shared/pathfinding/run_006b46a0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006815c0` (unnamed in the engine map): returns its `this`; the
/// constructor runs it on +0x10 (a member with nothing to construct).
const IDENTITY_006815C0: u32 = 0x0068_15c0;
/// `0069aee0` (unnamed in the engine map): the constructor run on +0x24
/// and +0x34.
const MEMBER_CTOR_0069AEE0: u32 = 0x0069_aee0;
/// `005e04c0` (unnamed in the engine map): the constructor run on +0x44.
const MEMBER_CTOR_005E04C0: u32 = 0x005e_04c0;
/// The three words (a zero point in the exe's data) copied to +0x10.
const ZERO_POINT: u32 = 0x011f_426c;

// Translated from 006b46a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006b46a0 - its FS:[0] exception frame (the unwind states that destroy the members at +0x24 and +0x34 when a later constructor throws) is not translated
/// Unnamed in the engine map: a navmesh info constructor. Runs `006815c0`
/// on +0x10, constructs +0x24 and +0x34 (`0069aee0`) and +0x44
/// (`005e04c0`), then clears +8, +0 and +4 and +0x1C, stores `0xDEADDEAD`
/// at +0xC, copies the three words at `011f426c` to +0x10, clears +0x58 and
/// +0x54 and stores 0.0 (`FLDZ`) at +0x20. Returns `this`.
pub fn fn_006b46a0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(IDENTITY_006815C0, &args![base + 0x10]);
    e.call(MEMBER_CTOR_0069AEE0, &args![base + 0x24]);
    e.call(MEMBER_CTOR_0069AEE0, &args![base + 0x34]);
    e.call(MEMBER_CTOR_005E04C0, &args![base + 0x44]);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base, 0);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 0x1c, 0);
    e.mem.set_u32(base + 0xc, 0xdead_dead);
    for i in 0..3 {
        let word = e.global::<u32>(ZERO_POINT + 4 * i);
        e.mem.set_u32(base + 0x10 + 4 * i, word);
    }
    e.mem.set_u32(base + 0x58, 0);
    e.mem.set_u32(base + 0x54, 0);
    e.mem.set_u32(base + 0x20, 0.0f32.to_bits());
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006b_46a0, fn_006b46a0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_006b46a0_constructs_members_and_sets_defaults() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        e.set_global::<u32>(ZERO_POINT, 0x11);
        e.set_global::<u32>(ZERO_POINT + 4, 0x22);
        e.set_global::<u32>(ZERO_POINT + 8, 0x33);
        e.register(IDENTITY_006815C0, |_, a| a[0].into_ret());
        e.register(MEMBER_CTOR_0069AEE0, |_, _| Ret::default());
        e.register(MEMBER_CTOR_005E04C0, |_, _| Ret::default());
        let this = e.mem.alloc(0x5c);
        e.mem.write(this, &[0xcd; 0x5c]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006b_46a0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        for offset in [0u32, 4, 8, 0x1c, 0x20, 0x54, 0x58] {
            assert_eq!(e.mem.u32(this + offset), 0, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0xc), 0xdead_dead);
        assert_eq!(e.mem.u32(this + 0x10), 0x11);
        assert_eq!(e.mem.u32(this + 0x14), 0x22);
        assert_eq!(e.mem.u32(this + 0x18), 0x33);
        assert_eq!(e.mem.u32(this + 0x24), 0xcdcd_cdcd);
        let calls: Vec<_> = log.into_iter().filter(|c| c.0 != 0x006b_46a0).collect();
        assert_eq!(
            calls,
            vec![
                (IDENTITY_006815C0, vec![this + 0x10]),
                (MEMBER_CTOR_0069AEE0, vec![this + 0x24]),
                (MEMBER_CTOR_0069AEE0, vec![this + 0x34]),
                (MEMBER_CTOR_005E04C0, vec![this + 0x44]),
            ]
        );
    }
}
