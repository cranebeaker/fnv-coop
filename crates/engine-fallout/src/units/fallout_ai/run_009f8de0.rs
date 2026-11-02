//! `fallout/ai/run_009f8de0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The size of a `SpectatorThreatInfo` (Xbox PDB, 0x34 bytes).
const THREAT_SIZE: u32 = 0x34;
/// `006e6da0` (cdecl: a size and an address): placement `new`, returns the address.
const PLACEMENT_NEW_006E6DA0: u32 = 0x006e_6da0;
/// The point at `011f426c` (three `float`s) the positions start as.
const POINT_011F426C: u32 = 0x011f_426c;
/// `00633c90` (`thiscall`, `RET 4`): constructs an `NiPointer` with a value.
const NI_POINTER_CONSTRUCT_00633C90: u32 = 0x0063_3c90;
/// `0066b0d0` (`thiscall`, `RET 4`): assigns an `NiPointer`.
const NI_POINTER_ASSIGN_0066B0D0: u32 = 0x0066_b0d0;

// Translated from 009f8de0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f8de0 - its FS:[0] exception frame (the unwind state around each constructor) is not translated
/// Unnamed in the engine map (the `BSSimpleArray<SpectatorThreatInfo,1024>`
/// construct step; `thiscall`, `RET 8`): for each of the `count` threats
/// from `first`, placement `new` (`006e6da0(0x34, address)`) and, when that
/// gives an address, the threat constructor `009f8e80`.
pub fn fn_009f8de0(e: &mut Engine, _this: Ptr, first: Ptr, count: u32) {
    let mut index = 0u32;
    while index < count {
        let address = index.wrapping_mul(THREAT_SIZE).wrapping_add(first.addr());
        let block = e
            .call(PLACEMENT_NEW_006E6DA0, &args![THREAT_SIZE, address])
            .u32();
        if block != 0 {
            fn_009f8e80(e, Ptr::new(block));
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 009f8e80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f8e80 - its FS:[0] exception frame (the unwind state that destroys the NiPointer) is not translated
/// The `SpectatorThreatInfo` constructor (unnamed in the engine map; `ECX`
/// only): no source, target, level or time, `fRadius` 0.0, both last known
/// positions the point at `011f426c`, both flags clear, and the debug
/// geometry pointer at +0x30 constructed null (`00633c90(0)`) and assigned
/// null (`0066b0d0(0)`). Returns `this`.
pub fn fn_009f8e80(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    for offset in [0u32, 4, 8, 0xc] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x10, x87_store32(0.0));
    for position in [0x14u32, 0x20] {
        for i in 0..3 {
            let word = e.mem.u32(POINT_011F426C + 4 * i);
            e.mem.set_u32(base + position + 4 * i, word);
        }
    }
    e.mem.set_u8(base + 0x2c, 0);
    e.mem.set_u8(base + 0x2d, 0);
    e.call(NI_POINTER_CONSTRUCT_00633C90, &args![base + 0x30, 0u32]);
    e.call(NI_POINTER_ASSIGN_0066B0D0, &args![base + 0x30, 0u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009f_8de0, fn_009f8de0(Ptr, Ptr, u32)),
        entry!(0x009f_8e80, fn_009f8e80(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn world() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(POINT_011F426C + 4 * i as u32, *v);
        }
        e.register(PLACEMENT_NEW_006E6DA0, |_, a| a[1].into_ret());
        e.register(NI_POINTER_CONSTRUCT_00633C90, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(NI_POINTER_ASSIGN_0066B0D0, |_, a| a[0].into_ret());
        e
    }

    #[test]
    fn at_009f8e80_clears_the_threat() {
        let mut e = world();
        let threat = e.mem.alloc(0x34);
        for i in 0..0xd {
            e.mem.set_u32(threat + 4 * i, 0xffff_ffff);
        }
        assert_eq!(e.call(0x009f_8e80, &args![threat]).u32(), threat);
        for offset in [0u32, 4, 8, 0xc, 0x10, 0x30] {
            assert_eq!(e.mem.u32(threat + offset), 0);
        }
        assert_eq!(e.mem.f32(threat + 0x14), 1.0);
        assert_eq!(e.mem.f32(threat + 0x28), 3.0);
        assert_eq!(e.mem.u8(threat + 0x2c), 0);
        assert_eq!(e.mem.u8(threat + 0x2d), 0);
    }

    #[test]
    fn at_009f8de0_constructs_each_threat_in_place() {
        let mut e = world();
        let first = e.mem.alloc(0x34 * 3);
        for i in 0..(0xd * 3) {
            e.mem.set_u32(first + 4 * i, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        e.call(0x009f_8de0, &args![0u32, first, 2u32]);
        let log = e.call_log.take().unwrap();
        let placed: Vec<Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == PLACEMENT_NEW_006E6DA0)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(placed, vec![vec![0x34, first], vec![0x34, first + 0x34]]);
        assert_eq!(e.mem.u32(first + 0x34), 0);
        assert_eq!(e.mem.u32(first + 0x68), 0xffff_ffff);
        // A null placement skips the constructor.
        e.register(PLACEMENT_NEW_006E6DA0, |_, _| Ret::default());
        e.call(0x009f_8de0, &args![0u32, first + 0x68, 1u32]);
        assert_eq!(e.mem.u32(first + 0x68), 0xffff_ffff);
    }
}
