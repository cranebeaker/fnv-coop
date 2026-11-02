//! `BSMain/run_00c54dc0` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c54dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a virtual function (virtual table word `010c2bac`) that
/// answers `this`; `thiscall`, `RET 4` (the stack word is not read).
pub fn fn_00c54dc0(_e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    this
}

/// `NiExtraData`'s member copy (`00a7b330`; `thiscall` on the source, `RET 8`: clone,
/// cloning process).
const EXTRA_DATA_COPY_MEMBERS: u32 = 0x00a7_b330;

// Translated from 00c54dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An extra data's member copy (unnamed in the engine map; `thiscall` on the source,
/// `RET 8`: clone, cloning process): `NiExtraData`'s copy (`00a7b330`), then the two
/// 3-word values at +0xC and +0x18 copied word by word.
pub fn fn_00c54dd0(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    e.call(EXTRA_DATA_COPY_MEMBERS, &args![this, clone, cloning]);
    for offset in [0xcu32, 0x10, 0x14, 0x18, 0x1c, 0x20] {
        let word = e.mem.u32(this.addr().wrapping_add(offset));
        e.mem.set_u32(clone.addr().wrapping_add(offset), word);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c5_4dc0, fn_00c54dc0(Ptr, u32) -> Ptr),
        entry!(0x00c5_4dd0, fn_00c54dd0(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_this() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c5_4dc0, &args![0x5678u32, 9u32]).u32(), 0x5678);
    }
}

#[cfg(test)]
mod n069_tests {
    use super::*;

    #[test]
    fn both_values_are_copied_after_the_base() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x24);
        let clone = e.mem.alloc(0x24);
        for k in 0..6u32 {
            e.mem.set_u32(this + 0xc + 4 * k, 0x100 + k);
        }
        e.mem.set_u32(this + 8, 0x99);
        e.call_log = Some(vec![]);
        e.call(0x00c5_4dd0, &args![this, clone, 0x700u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(EXTRA_DATA_COPY_MEMBERS, vec![this, clone, 0x700])));
        for k in 0..6u32 {
            assert_eq!(e.mem.u32(clone + 0xc + 4 * k), 0x100 + k);
        }
        assert_eq!(e.mem.u32(clone + 8), 0);
    }
}
