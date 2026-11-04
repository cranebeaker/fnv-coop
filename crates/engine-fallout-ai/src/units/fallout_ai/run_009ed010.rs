//! `fallout/ai/run_009ed010` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00679a90` (named `TESPackage::InitLoadGame` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const INIT_LOAD_GAME_TARGET_00679A90: u32 = 0x0067_9a90;

// Translated from 009ed010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable words 01092c40 and 01092de8): forwards `this` and one
/// pointer to `00679a90` (named `TESPackage::InitLoadGame`; Xbox PDB
/// `?InitLoadGame@TESPackage@@UAAXPAVBGSLoadFormBuffer@@@Z`, void, the overload with one
/// parameter matches its `RET 4`). Returns nothing.
pub fn fn_009ed010(e: &mut Engine, this: Ptr, buffer: Ptr) {
    e.call(INIT_LOAD_GAME_TARGET_00679A90, &args![this, buffer]);
}

/// `TESPackage::TESPackage` (`006707c0`, Xbox PDB): `ECX` only.
const PACKAGE_CONSTRUCT: u32 = 0x0067_07c0;
/// The `BackUpPackage` vtable (`01092d84`, RTTI `.?AVBackUpPackage@@`).
const BACK_UP_PACKAGE_VTABLE: u32 = 0x0109_2d84;
/// `006815c0` (unnamed): returns `this` (the point's no-op constructor).
const POINT_CONSTRUCT: u32 = 0x0068_15c0;
/// `009ed330` (unnamed, `run_009ed330`): the `BackUpPackage` destructor (`ECX` only).
const BACK_UP_PACKAGE_DESTRUCT: u32 = 0x009e_d330;
/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;

// Translated from 009ed030 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009ed030 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map (`ECX` only): the `BackUpPackage` constructor (vtable
/// `01092d84`, RTTI `.?AVBackUpPackage@@`): `TESPackage::TESPackage`, the vtable and the
/// point at `+0x80` (`006815c0`). Returns `this`.
pub fn fn_009ed030(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(PACKAGE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), BACK_UP_PACKAGE_VTABLE);
    e.call(POINT_CONSTRUCT, &args![this.addr() + 0x80]);
    this
}

// Translated from 009ed0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the `BackUpPackage` scalar deleting
/// destructor: `009ed330`, then `operator delete` on `this` for bit 0 of `flags`. Returns
/// `this`.
pub fn fn_009ed0a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(BACK_UP_PACKAGE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009e_d010, fn_009ed010(Ptr, Ptr)),
        entry!(0x009e_d030, fn_009ed030(Ptr) -> Ptr),
        entry!(0x009e_d0a0, fn_009ed0a0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_009ed010_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(INIT_LOAD_GAME_TARGET_00679A90, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x009e_d010, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == INIT_LOAD_GAME_TARGET_00679A90)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod n029_tests {
    use super::*;

    #[test]
    fn fn_009ed030_constructs_the_package() {
        let mut e = Engine::new();
        e.register(PACKAGE_CONSTRUCT, |_, _| Ret::default());
        e.register(POINT_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let this = e.mem.alloc(0x8c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009e_d030, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let order: Vec<_> = log.into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (PACKAGE_CONSTRUCT, vec![this]),
                (POINT_CONSTRUCT, vec![this + 0x80])
            ]
        );
        assert_eq!(e.mem.u32(this), BACK_UP_PACKAGE_VTABLE);
    }

    #[test]
    fn fn_009ed0a0_destroys_and_frees_for_bit_0() {
        let mut e = Engine::new();
        e.register(BACK_UP_PACKAGE_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        for flags in [0u32, 1] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x009e_d0a0, &args![0x5000u32, flags]).u32(), 0x5000);
            let log = e.call_log.take().unwrap();
            assert!(log.contains(&(BACK_UP_PACKAGE_DESTRUCT, vec![0x5000])));
            let frees = log.iter().filter(|c| c.0 == OPERATOR_DELETE).count();
            assert_eq!(frees, flags as usize);
        }
    }
}
