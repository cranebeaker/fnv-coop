//! `fallout/ai/run_009d69f0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `009ca680` (unnamed): the base constructor (`ECX` only).
const BASE_CONSTRUCT_009CA680: u32 = 0x009c_a680;
/// The vtable `01091bb4` (RTTI `.?AVCombatProcedureMove@@`).
const MOVE_VTABLE: u32 = 0x0109_1bb4;
/// `0097ede0` (named `BGSWorldLocation::CheckSpace` in the engine map; a folded body): the
/// constructor of the 0x10-byte world locations at `+0x14` and `+0x24` (`ECX` only).
const LOCATION_CONSTRUCT: u32 = 0x0097_ede0;
/// `00906f60` (unnamed): stores 0.0 in the two floats at `this` (`ECX` only; the
/// timers).
const CLEAR_TWO_FLOATS: u32 = 0x0090_6f60;
/// `006815c0` (unnamed): returns `this` (the point's no-op constructor, and the location
/// read).
const SELF_006815C0: u32 = 0x0068_15c0;
/// `00633c90` (unnamed): `thiscall`, one word (`RET 4`); constructs the pointer at `+0x58`.
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `0044edb0` (named `BaseProcess::GetCurrentProcedureIndex` in the engine map; a folded
/// getter): `thiscall`; the location's word.
const LOCATION_WORD: u32 = 0x0044_edb0;
/// `0097be60` (unnamed, `combatactions.cpp`): `thiscall`, one word (`RET 4`); a `bool`.
const LOCATION_TEST: u32 = 0x0097_be60;

// Translated from 009d69f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009d69f0 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map (`thiscall`, `RET 0xc`): the `CombatProcedureMove`
/// constructor (vtable `01091bb4`; the fields named after the Xbox PDB as
/// `combatproceduremove.rs` declares them): the base `009ca680`, `eState` 0, the two
/// locations (`+0x14` `Destination`, `+0x24` `CurrentDestination`, `0097ede0`),
/// `fTargetRadius` = `radius` (`FLD`/`FSTP`), `pTargetRef` 0, `iLocation` = `index`, the
/// timers (`+0x40`, `+0x64`, `+0x6c`, `+0x74`, `00906f60`), the look-at point (`+0x48`),
/// `bReservedSecondaryLocation` false, `spQueuedPathingRequest` (`00633c90` with 0),
/// `bHasQueuedPathingRequest` false and `iCurrentNode` -1. With a `location` both
/// destinations become its four words, `pTargetRef` its `0044edb0` word, and the radius is
/// negated when `0097be60` (with 2) says so. Returns `this`.
pub fn fn_009d69f0(e: &mut Engine, this: Ptr, location: u32, radius: f32, index: u32) -> Ptr {
    let p = this.addr();
    e.call(BASE_CONSTRUCT_009CA680, &args![this]);
    e.mem.set_u32(p, MOVE_VTABLE);
    // eState
    e.mem.set_u32(p + 0x10, 0);
    e.call(LOCATION_CONSTRUCT, &args![p + 0x14]);
    e.call(LOCATION_CONSTRUCT, &args![p + 0x24]);
    // fTargetRadius
    e.mem.set_u32(p + 0x34, x87_quiet_bits(radius.to_bits()));
    // pTargetRef
    e.mem.set_u32(p + 0x38, 0);
    // iLocation
    e.mem.set_u32(p + 0x3c, index);
    e.call(CLEAR_TWO_FLOATS, &args![p + 0x40]);
    e.call(SELF_006815C0, &args![p + 0x48]);
    // bReservedSecondaryLocation
    e.mem.set_u8(p + 0x54, 0);
    e.call(POINTER_CONSTRUCT, &args![p + 0x58, 0u32]);
    // bHasQueuedPathingRequest, iCurrentNode
    e.mem.set_u8(p + 0x5c, 0);
    e.mem.set_u32(p + 0x60, 0xffff_ffff);
    for offset in [0x64u32, 0x6c, 0x74] {
        e.call(CLEAR_TWO_FLOATS, &args![p + offset]);
    }
    if location != 0 {
        let source = e.call(SELF_006815C0, &args![location]).u32();
        let words = e.mem.bytes(source, 0x10);
        e.mem.write(p + 0x14, &words);
        let words = e.mem.bytes(p + 0x14, 0x10);
        e.mem.write(p + 0x24, &words);
        let target = e.call(LOCATION_WORD, &args![location]).u32();
        e.mem.set_u32(p + 0x38, target);
        if e.call(LOCATION_TEST, &args![location, 2u32]).u8() != 0 {
            let radius = x87_load32(e.mem.u32(p + 0x34));
            e.mem.set_u32(p + 0x34, x87_store32(-radius));
        }
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x009d_69f0,
        fn_009d69f0(Ptr, u32, f32, u32) -> Ptr
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doubles(e: &mut Engine) {
        for addr in [
            BASE_CONSTRUCT_009CA680,
            LOCATION_CONSTRUCT,
            CLEAR_TWO_FLOATS,
            POINTER_CONSTRUCT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(SELF_006815C0, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(LOCATION_WORD, |e, a| Ret {
            eax: e.mem.u32(a[0] + 8),
            ..Ret::default()
        });
        e.register(LOCATION_TEST, |e, a| Ret {
            eax: u32::from(e.mem.u8(a[0] + 0x10) == a[1] as u8),
            ..Ret::default()
        });
    }

    #[test]
    fn fn_009d69f0_constructs_without_a_location() {
        let mut e = Engine::new();
        doubles(&mut e);
        let this = e.mem.alloc(0x7c);
        e.mem.write(this, &[0xcd; 0x7c]);
        e.call_log = Some(vec![]);
        let r = e.call(0x009d_69f0, &args![this, 0u32, 2.5f32, 7u32]);
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), MOVE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.f32(this + 0x34), 2.5);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        assert_eq!(e.mem.u32(this + 0x3c), 7);
        assert_eq!(e.mem.u8(this + 0x54), 0);
        assert_eq!(e.mem.u8(this + 0x5c), 0);
        assert_eq!(e.mem.u32(this + 0x60), 0xffff_ffff);
        let log = e.call_log.take().unwrap();
        let timers: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == CLEAR_TWO_FLOATS)
            .map(|c| c.1[0] - this)
            .collect();
        assert_eq!(timers, vec![0x40, 0x64, 0x6c, 0x74]);
        assert!(log.contains(&(POINTER_CONSTRUCT, vec![this + 0x58, 0])));
        assert!(!log.iter().any(|c| c.0 == LOCATION_WORD));
    }

    #[test]
    fn fn_009d69f0_copies_the_location() {
        let mut e = Engine::new();
        doubles(&mut e);
        let location = e.mem.alloc(0x14);
        for i in 0..4 {
            e.mem.set_u32(location + 4 * i, 0x100 + i);
        }
        e.mem.set_u8(location + 0x10, 2);
        let this = e.mem.alloc(0x7c);
        e.call(0x009d_69f0, &args![this, location, 2.5f32, 7u32]);
        for i in 0..4 {
            assert_eq!(e.mem.u32(this + 0x14 + 4 * i), 0x100 + i);
            assert_eq!(e.mem.u32(this + 0x24 + 4 * i), 0x100 + i);
        }
        assert_eq!(e.mem.u32(this + 0x38), 0x102);
        // 0097be60 says so: the radius is negated.
        assert_eq!(e.mem.f32(this + 0x34), -2.5);
        e.mem.set_u8(location + 0x10, 1);
        e.call(0x009d_69f0, &args![this, location, 2.5f32, 7u32]);
        assert_eq!(e.mem.f32(this + 0x34), 2.5);
    }
}
