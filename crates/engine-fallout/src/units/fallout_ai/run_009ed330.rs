//! `fallout/ai/run_009ed330` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009ed330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `01092d84` (RTTI `.?AVBackUpPackage@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `006708b0`. One word, plain `RET`, returns nothing.
pub fn fn_009ed330(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0109_2d84);
    e.call(0x0067_08b0, &args![this]);
}

// Translated from 009ed350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): copies the `BackUpPackage`'s point at
/// `+0x80` (three words) to `out` and returns `out`.
pub fn fn_009ed350(e: &mut Engine, this: Ptr, out: u32) -> u32 {
    let words = e.mem.bytes(this.addr() + 0x80, 0xc);
    e.mem.write(out, &words);
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009e_d330, fn_009ed330(Ptr)),
        entry!(0x009e_d350, fn_009ed350(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_009ed330_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x0067_08b0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x009e_d330, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x0067_08b0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0109_2d84);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0109_2d84,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod n029_tests {
    use super::*;

    #[test]
    fn fn_009ed350_copies_the_point() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x8c);
        for i in 0..3 {
            e.mem.set_u32(this + 0x80 + 4 * i, 0x10 + i);
        }
        let out = e.mem.alloc(0x10);
        e.mem.set_u32(out + 0xc, 0x99);
        assert_eq!(e.call(0x009e_d350, &args![this, out]).u32(), out);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(words, vec![0x10, 0x11, 0x12, 0x99]);
    }
}
