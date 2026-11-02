//! `SpeedTree/run_00b37590` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Member constructors (no stack words): `00b31270` (+0x0c, +0x24, +0x54,
/// +0x6c) and `00449fd0` (+0x3c).
const MEMBER_CONSTRUCT_00B31270: u32 = 0x00b3_1270;
const MEMBER_CONSTRUCT_00449FD0: u32 = 0x0044_9fd0;

// Translated from 00b37590 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b37590 - the function's C++ exception frame (FS:[0], unwind states of the members) is not translated
/// A constructor (unnamed in the engine map): the `float`s 0, 1, 0 at +0,
/// +4, +8 (`FLDZ`/`FLD1`), then the members at +0x0c, +0x24, +0x3c, +0x54
/// and +0x6c. Returns `this`.
pub fn fn_00b37590(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, x87_store32(0.0));
    e.mem.set_u32(base + 4, x87_store32(1.0));
    e.mem.set_u32(base + 8, x87_store32(0.0));
    e.call(MEMBER_CONSTRUCT_00B31270, &args![base + 0x0c]);
    e.call(MEMBER_CONSTRUCT_00B31270, &args![base + 0x24]);
    e.call(MEMBER_CONSTRUCT_00449FD0, &args![base + 0x3c]);
    e.call(MEMBER_CONSTRUCT_00B31270, &args![base + 0x54]);
    e.call(MEMBER_CONSTRUCT_00B31270, &args![base + 0x6c]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b3_7590, fn_00b37590(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_the_floats_and_constructs_the_members() {
        let mut e = Engine::new();
        e.register(MEMBER_CONSTRUCT_00B31270, |_, _| Ret::default());
        e.register(MEMBER_CONSTRUCT_00449FD0, |_, _| Ret::default());
        let this = e.mem.alloc(0x84);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b3_7590, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0x3f80_0000);
        assert_eq!(e.mem.u32(this + 8), 0);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.into_iter().filter(|(a, _)| *a != 0x00b3_7590).collect();
        assert_eq!(
            calls,
            vec![
                (MEMBER_CONSTRUCT_00B31270, vec![this + 0x0c]),
                (MEMBER_CONSTRUCT_00B31270, vec![this + 0x24]),
                (MEMBER_CONSTRUCT_00449FD0, vec![this + 0x3c]),
                (MEMBER_CONSTRUCT_00B31270, vec![this + 0x54]),
                (MEMBER_CONSTRUCT_00B31270, vec![this + 0x6c]),
            ]
        );
    }
}
