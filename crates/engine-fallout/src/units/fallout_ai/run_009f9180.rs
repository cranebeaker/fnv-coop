//! `fallout/ai/run_009f9180` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_ai::trespasspackage::TrespassPackage;

/// `009f6c90` (`run_009f6c90.rs`, `ECX` only): the `SpectatorThreatInfo`
/// destructor body.
const THREAT_DESTRUCT_009F6C90: u32 = 0x009f_6c90;
/// `009f9450` (`trespasspackage.rs`, `ECX` only): the `TrespassPackage`
/// destructor body.
const TRESPASS_PACKAGE_DESTRUCT_009F9450: u32 = 0x009f_9450;
/// `MemoryManager` deallocation (`00401030`, cdecl, the block).
const DEALLOCATE: u32 = 0x0040_1030;
/// `TESPackage::TESPackage` (`006707c0`, engine map; `ECX` only).
const PACKAGE_CONSTRUCT: u32 = 0x0067_07c0;
/// The `TrespassPackage` vtable (RTTI `.?AVTrespassPackage@@`).
const TRESPASS_PACKAGE_VTABLE: u32 = 0x0109_39ec;

// Translated from 009f9180 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `SpectatorThreatInfo` scalar deleting destructor (unnamed in the
/// engine map; `RET 4`): the destructor body `009f6c90`, then
/// `00401030(this)` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_009f9180(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(THREAT_DESTRUCT_009F6C90, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 009f91b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TrespassPackage` constructor without arguments (unnamed in the
/// engine map; `ECX` only; the vtable's RTTI is `.?AVTrespassPackage@@`):
/// the `TESPackage` constructor, the vtable, `fwarningtimer` 0.0,
/// `iwarning`, `pOwner` and `pObjectOwned` 0, `iCrimeNumber` -1 and
/// `iwarninglevel` 3. Returns `this`.
pub fn fn_009f91b0(e: &mut Engine, this: Ptr<TrespassPackage>) -> Ptr<TrespassPackage> {
    e.call(PACKAGE_CONSTRUCT, &args![this]);
    e.set(this, TrespassPackage::vfptr, TRESPASS_PACKAGE_VTABLE);
    e.mem.set_u32(this.addr() + 0x80, x87_store32(0.0));
    e.set(this, TrespassPackage::iwarning, 0);
    e.set(this, TrespassPackage::pOwner, Ptr::NULL);
    e.set(this, TrespassPackage::pObjectOwned, Ptr::NULL);
    e.set(this, TrespassPackage::iCrimeNumber, -1);
    e.set(this, TrespassPackage::iwarninglevel, 3);
    this
}

// Translated from 009f9220 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TrespassPackage` scalar deleting destructor (unnamed in the engine
/// map; `RET 4`): the destructor body `009f9450`, then `00401030(this)`
/// when bit 0 of `flags` is set. Returns `this`.
pub fn fn_009f9220(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(TRESPASS_PACKAGE_DESTRUCT_009F9450, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009f_9180, fn_009f9180(Ptr, u32) -> Ptr),
        entry!(
            0x009f_91b0,
            fn_009f91b0(Ptr<TrespassPackage>) -> Ptr<TrespassPackage>
        ),
        entry!(0x009f_9220, fn_009f9220(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting(address: u32, body: u32) {
        for (flags, frees) in [(0u32, false), (1, true), (2, false), (3, true)] {
            let mut e = Engine::new();
            e.register(body, |_, _| Ret::default());
            e.register(DEALLOCATE, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![0x5000u32, flags]).u32(), 0x5000);
            let log = e.call_log.take().unwrap();
            let order: Vec<(u32, Vec<u32>)> = log.into_iter().skip(1).collect();
            let mut expected = vec![(body, vec![0x5000])];
            if frees {
                expected.push((DEALLOCATE, vec![0x5000]));
            }
            assert_eq!(order, expected);
        }
    }

    #[test]
    fn at_009f9180_destroys_the_threat_then_frees_on_bit_zero() {
        check_deleting(0x009f_9180, THREAT_DESTRUCT_009F6C90);
    }

    #[test]
    fn at_009f9220_destroys_the_package_then_frees_on_bit_zero() {
        check_deleting(0x009f_9220, TRESPASS_PACKAGE_DESTRUCT_009F9450);
    }

    #[test]
    fn at_009f91b0_constructs_the_trespass_package() {
        let mut e = Engine::new();
        e.register(PACKAGE_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.new_object::<TrespassPackage>();
        for i in 0..(0x9c / 4) {
            e.mem.set_u32(this.addr() + 4 * i, 0x5555_5555);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009f_91b0, &args![this]).u32(), this.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (PACKAGE_CONSTRUCT, vec![this.addr()]));
        assert_eq!(e.get(this, TrespassPackage::vfptr), TRESPASS_PACKAGE_VTABLE);
        assert_eq!(e.get(this, TrespassPackage::fwarningtimer), 0.0);
        assert_eq!(e.get(this, TrespassPackage::iwarning), 0);
        assert!(e.get(this, TrespassPackage::pOwner).is_null());
        assert!(e.get(this, TrespassPackage::pObjectOwned).is_null());
        assert_eq!(e.get(this, TrespassPackage::iCrimeNumber), -1);
        assert_eq!(e.get(this, TrespassPackage::iwarninglevel), 3);
        assert_eq!(e.get(this, TrespassPackage::iNumberActors), 0x5555_5555);
    }
}
