//! `Havok SDK/run_00e3b630` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e3b630 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: sets the 3x4 float block at `this + 0x20` (three vectors) to the
/// identity (zero, then 1.0 at `+0x20`, `+0x34` and `+0x48`) and runs `00cb2d90` (`this` =
/// `this + 0x50`, one stack word `extra`, popped by the callee). The function takes one stack
/// word and pops it (`RET 4`) and returns nothing. The class is unconfirmed.
pub fn fn_00e3b630(e: &mut Engine, this: Ptr, extra: u32) {
    let base = this.addr();
    for word in 0..12 {
        e.mem.set_u32(base + 0x20 + 4 * word, 0);
    }
    e.mem.set_u32(base + 0x20, 0x3f80_0000);
    e.mem.set_u32(base + 0x34, 0x3f80_0000);
    e.mem.set_u32(base + 0x48, 0x3f80_0000);
    e.call(0x00cb_2d90, &args![base + 0x50, extra]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e3_b630, fn_00e3b630(Ptr, u32))]
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;

    /// The identity block is in place before the call, which gets `this + 0x50` and the word.
    #[test]
    fn fn_00e3b630_sets_the_identity_then_calls() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x60);
        e.mem.write(this, &[0x66; 0x60]);
        e.register_double(0x00cb_2d90, move |e, a| {
            assert_eq!(a, [this + 0x50, 7]);
            assert_eq!(e.mem.f32(this + 0x34), 1.0);
            Ret::default()
        });
        e.call(0x00e3_b630, &args![this, 7u32]);
        let words: Vec<f32> = (0..12).map(|i| e.mem.f32(this + 0x20 + 4 * i)).collect();
        assert_eq!(
            words,
            vec![1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0]
        );
        assert_eq!(e.mem.u32(this + 0x1c), 0x6666_6666);
        assert_eq!(e.mem.u32(this + 0x50), 0x6666_6666);
    }
}
