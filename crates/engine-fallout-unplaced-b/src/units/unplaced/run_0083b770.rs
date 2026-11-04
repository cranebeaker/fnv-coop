//! `(unplaced)/run_0083b770` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The constructor and destructor of a static object of two 0xc-byte
//! members (at +0 and +0xc), both unoptimized code with an exception frame
//! (they are the object and atexit entries of the initializer `00f9ec20`).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0041a250` (`thiscall`, plain `RET`, `extradatalist.cpp`): the member's
/// constructor (its first word -1, the byte at +4 and the word at +8 cleared).
const MEMBER_CONSTRUCT: u32 = 0x0041_a250;
/// `00483710` (`thiscall`, plain `RET`, `tesform.cpp`): an empty function,
/// the member's destructor here.
const MEMBER_DESTRUCT: u32 = 0x0048_3710;
/// The second member's offset.
const SECOND_MEMBER: u32 = 0xc;

// Translated from 0083b770 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083b770 - the compiler's exception frame (FS:[0] chain, handler 00f005f8) is not translated
/// Unnamed (`thiscall`): constructs both members (`0041a250` on `this` and
/// on `this + 0xc`); returns `this`.
pub fn fn_0083b770(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MEMBER_CONSTRUCT, &args![this]);
    e.call(
        MEMBER_CONSTRUCT,
        &args![this.addr().wrapping_add(SECOND_MEMBER)],
    );
    this
}

// Translated from 0083b7d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0083b7d0 - the compiler's exception frame (FS:[0] chain, handler 00f005f8) is not translated
/// Unnamed (`thiscall`): destroys the members in reverse order (`00483710`
/// on `this + 0xc`, then on `this`).
pub fn fn_0083b7d0(e: &mut Engine, this: Ptr) {
    e.call(
        MEMBER_DESTRUCT,
        &args![this.addr().wrapping_add(SECOND_MEMBER)],
    );
    e.call(MEMBER_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0083_b770, fn_0083b770(Ptr) -> Ptr),
        entry!(0x0083_b7d0, fn_0083b7d0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_and_destroys_both_members_in_order() {
        let mut e = Engine::new();
        e.register(MEMBER_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(MEMBER_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0083_b770, &args![0x1000u32]).u32(), 0x1000);
        e.call(0x0083_b7d0, &args![0x1000u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .into_iter()
            .filter(|(a, _)| *a == MEMBER_CONSTRUCT || *a == MEMBER_DESTRUCT)
            .collect();
        assert_eq!(
            calls,
            [
                (MEMBER_CONSTRUCT, vec![0x1000]),
                (MEMBER_CONSTRUCT, vec![0x100c]),
                (MEMBER_DESTRUCT, vec![0x100c]),
                (MEMBER_DESTRUCT, vec![0x1000]),
            ]
        );
    }
}
