//! `NiXenonRenderer/run_00e95b60` (Xbox PDB source unit), subsystem `NiXenonRenderer`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e95b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// A constructor (unnamed in the engine map; its class is not confirmed),
/// thiscall without stack words: sets the word at `+0` to -1, the word at
/// `+4` to 0xffff and clears the words at `+8` and `+0x10` (the word at
/// `+0xc` is left alone). Answers `this`.
pub fn fn_00e95b60(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0xffff_ffff);
    e.mem.set_u32(base + 4, 0xffff);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u32(base + 0x10, 0);
    this
}

// Translated from 00e95b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00e95b60`): the clean-up of the record `00e95b60` builds
/// (`thiscall`): the object at +0x10 is released (slot 8) and cleared when set.
pub fn fn_00e95b80(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let held = e.mem.u32(base + 0x10);
    if held != 0 {
        e.vcall(held, 8, &args![]);
        e.mem.set_u32(base + 0x10, 0);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e95b60, fn_00e95b60(Ptr) -> Ptr),
        entry!(0x00e9_5b80, fn_00e95b80(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_the_initial_words_and_keeps_0xc() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x14);
        for word in 0..5 {
            e.mem.set_u32(this + 4 * word, 0x1234_5678);
        }
        assert_eq!(e.call(0x00e9_5b60, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 4), 0xffff);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0x1234_5678);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }
}

#[cfg(test)]
mod n011_tests {
    use super::*;

    #[test]
    fn fn_00e95b80_releases_the_held_object_once() {
        let mut e = Engine::new();
        e.put_vtable(0x6000, &[0, 0, 0x6008]);
        e.register(0x6008, |_, _| Ret::default());
        let held = e.mem.alloc(4);
        e.mem.set_u32(held, 0x6000);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x10, held);
        e.call_log = Some(vec![]);
        e.call(0x00e9_5b80, &args![this]);
        e.call(0x00e9_5b80, &args![this]);
        let log = e.call_log.take().unwrap();
        let released: Vec<_> = log.iter().filter(|c| c.0 == 0x6008).collect();
        assert_eq!(released.len(), 1);
        assert_eq!(released[0].1, vec![held]);
        assert_eq!(e.mem.u32(this + 0x10), 0);
    }
}
