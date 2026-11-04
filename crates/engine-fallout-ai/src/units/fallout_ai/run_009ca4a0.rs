//! `fallout/ai/run_009ca4a0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The three functions work on a 0x28-byte object whose fields match the Xbox PDB's
//! `VATS_COMMAND` (`eAction`, the bools at `+4` to `+7`, `cFireShots`, `cFakeFireShots`,
//! `pTarget`, `eLimb`, `pHitData`, `fTotalDamage`, `fCumulativeDamage`, `fActionPoints`,
//! `bMissFortune`); the engine map names none of them.

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`00401000`) and `operator delete` (`00401030`), `cdecl`, one word.
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `HitData::HitData` (`009b4d90`, Xbox PDB): `ECX` only.
const HIT_DATA_CONSTRUCT: u32 = 0x009b_4d90;

// Translated from 009ca4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the constructor; initializes the fields
/// (`fn_009ca500`) and returns `this`.
pub fn fn_009ca4a0(e: &mut Engine, this: Ptr) -> Ptr {
    fn_009ca500(e, this);
    this
}

// Translated from 009ca4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` only): the destructor; frees the hit data at `+0x14`
/// (`operator delete`) and clears the pointer when it is set.
pub fn fn_009ca4c0(e: &mut Engine, this: Ptr) {
    let hit_data = e.mem.u32(this.addr() + 0x14);
    if hit_data != 0 {
        e.call(OPERATOR_DELETE, &args![hit_data]);
        e.mem.set_u32(this.addr() + 0x14, 0);
    }
}

// Translated from 009ca500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009ca500 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map (`ECX` only): action 0x16, the six bytes `+4` to `+9`
/// cleared, no target, limb 0x4d, a new hit data (`operator new(0x64)`,
/// `HitData::HitData`; null when the allocation fails), the floats at `+0x18`, `+0x20` and
/// `+0x1c` set to 0.0 and the byte at `+0x24` cleared.
pub fn fn_009ca500(e: &mut Engine, this: Ptr) {
    let p = this.addr();
    e.mem.set_u32(p, 0x16);
    for offset in 4..=9 {
        e.mem.set_u8(p + offset, 0);
    }
    e.mem.set_u32(p + 0xc, 0);
    e.mem.set_u32(p + 0x10, 0x4d);
    let block = e.call(OPERATOR_NEW, &args![0x64u32]).u32();
    let hit_data = if block != 0 {
        e.call(HIT_DATA_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.mem.set_u32(p + 0x14, hit_data);
    e.mem.set_u32(p + 0x18, 0.0f32.to_bits());
    e.mem.set_u32(p + 0x20, 0.0f32.to_bits());
    e.mem.set_u32(p + 0x1c, 0.0f32.to_bits());
    e.mem.set_u8(p + 0x24, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009c_a4a0, fn_009ca4a0(Ptr) -> Ptr),
        entry!(0x009c_a4c0, fn_009ca4c0(Ptr)),
        entry!(0x009c_a500, fn_009ca500(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn fn_009ca4a0_and_fn_009ca500_initialize_the_command() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(HIT_DATA_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let this = e.mem.alloc(0x28);
        e.mem.write(this, &[0xcd; 0x28]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009c_a4a0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x16);
        assert!(e.mem.bytes(this + 4, 6).iter().all(|b| *b == 0));
        assert_eq!(e.mem.u16(this + 0xa), 0xcdcd);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x4d);
        let block = calls(&e, HIT_DATA_CONSTRUCT)[0][0];
        assert_eq!(e.mem.u32(this + 0x14), block);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x64]]);
        assert!(e.mem.bytes(this + 0x18, 0xd).iter().all(|b| *b == 0));
        assert_eq!(e.mem.u8(this + 0x25), 0xcd);
        // A failed allocation leaves no hit data.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call(0x009c_a500, &args![this]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
    }

    #[test]
    fn fn_009ca4c0_frees_the_hit_data() {
        let mut e = Engine::new();
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x28);
        e.call_log = Some(vec![]);
        e.call(0x009c_a4c0, &args![this]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.mem.set_u32(this + 0x14, 0x5000);
        e.call(0x009c_a4c0, &args![this]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x5000]]);
        assert_eq!(e.mem.u32(this + 0x14), 0);
    }
}
