//! `(unplaced)/run_00a99c40` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a7b330`: `NiExtraData`'s member copy (`thiscall` on the source: clone, cloning
/// process).
const EXTRA_DATA_COPY_MEMBERS: u32 = 0x00a7_b330;

// Translated from 00a99c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the member copy of an extra data class with one word of its
/// own at `+0xC` (`thiscall` on the source, `RET 8`: clone, cloning process):
/// `NiExtraData`'s copy (`00a7b330`), then the word.
pub fn fn_00a99c40(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    e.call(EXTRA_DATA_COPY_MEMBERS, &args![this, clone, cloning]);
    let value = e.mem.u32(this.addr() + 0xc);
    e.mem.set_u32(clone.addr() + 0xc, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a9_9c40, fn_00a99c40(Ptr, Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_the_base_then_the_word() {
        let mut e = Engine::new();
        e.register(EXTRA_DATA_COPY_MEMBERS, |e, a| {
            assert_eq!(e.mem.u32(a[1] + 0xc), 0);
            Ret::default()
        });
        let source = e.mem.alloc(0x10);
        let clone = e.mem.alloc(0x10);
        e.mem.set_u32(source + 0xc, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x00a9_9c40, &args![source, clone, 0x900u32]);
        assert_eq!(e.mem.u32(clone + 0xc), 0x77);
        let log = e.call_log.take().unwrap();
        let copies: Vec<_> = log
            .iter()
            .filter(|c| c.0 == EXTRA_DATA_COPY_MEMBERS)
            .collect();
        assert_eq!(copies[0].1, vec![source, clone, 0x900]);
    }
}
