//! `NiMain/run_00a93c00` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a5dc80` (unnamed in the engine map, `thiscall`, clone and cloning process): the base
/// member copy this one runs first.
const BASE_COPY_MEMBERS: u32 = 0x00a5_dc80;

// Translated from 00a93c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, the clone and the cloning process; `RET 8`): a
/// member copy: the base copy (`00a5dc80`), then the 16-bit word at +0xac and the word at
/// +0xb0 copied into the clone (fields unnamed here).
pub fn fn_00a93c00(e: &mut Engine, this: Ptr, clone: Ptr, cloning_process: Ptr) {
    e.call(BASE_COPY_MEMBERS, &args![this, clone, cloning_process]);
    let half = e.mem.u16(this.addr() + 0xac);
    e.mem.set_u16(clone.addr() + 0xac, half);
    let word = e.mem.u32(this.addr() + 0xb0);
    e.mem.set_u32(clone.addr() + 0xb0, word);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a9_3c00, fn_00a93c00(Ptr, Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_the_base_then_two_fields() {
        let mut e = Engine::new();
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0xb4);
        let clone = e.mem.alloc(0xb4);
        e.mem.set_u32(this + 0xac, 0x1234_5678);
        e.mem.set_u32(this + 0xb0, 0x9abc);
        e.mem.set_u16(clone + 0xae, 0xeeee);
        e.call_log = Some(vec![]);
        e.call(0x00a9_3c00, &args![this, clone, 0x33u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(BASE_COPY_MEMBERS, vec![this, clone, 0x33])));
        assert_eq!(e.mem.u32(clone + 0xac), 0xeeee_5678);
        assert_eq!(e.mem.u32(clone + 0xb0), 0x9abc);
    }
}
