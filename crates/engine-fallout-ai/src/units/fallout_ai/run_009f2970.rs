//! `fallout/ai/run_009f2970` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `ActorPackageData`'s constructor (`0067d920`: stores the base vtable).
const ACTOR_PACKAGE_DATA_CONSTRUCT: u32 = 0x0067_d920;
/// The `References` array's constructor (`005e04c0`; a `BSSimpleArray`
/// constructor the engine map places in `tesscriptfunctions.cpp`).
const REFERENCES_CONSTRUCT: u32 = 0x005e_04c0;
/// The `PatrolActorPackageData` vtable (its RTTI locator names
/// `.?AVPatrolActorPackageData@@`).
const PATROL_ACTOR_PACKAGE_DATA_VTABLE: u32 = 0x0109_3418;

// Translated from 009f29a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f29a0 - C++ exception unwinding (FS:[0] frame) not translated
/// The `PatrolActorPackageData` constructor: the `ActorPackageData` base,
/// the class's vtable, the `References` array (+0x1C), then `eState` (+4),
/// `pActor` (+0x10), `pPackage` (+0x14) and `pPackageData` (+0x18) cleared
/// (Xbox PDB field names). Returns `this`.
pub fn fn_009f29a0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(ACTOR_PACKAGE_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, PATROL_ACTOR_PACKAGE_DATA_VTABLE);
    e.call(REFERENCES_CONSTRUCT, &args![base + 0x1c]);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 0x14, 0);
    e.mem.set_u32(base + 0x18, 0);
    this
}

// Translated from 009f2a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PatrolActorPackageData::GetPackageType` (vtable slot +8 by the Xbox PDB's
/// class): 13.
pub fn fn_009f2a30(_e: &mut Engine, _this: Ptr) -> u32 {
    0xd
}

/// `TESObjectREFR::SetTargeted` (`00564db0`, engine map): `thiscall`, one word (`RET 4`).
const SET_TARGETED: u32 = 0x0056_4db0;

// Translated from 009f2970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): stores `reference` at `+8` and marks it
/// targeted (`TESObjectREFR::SetTargeted` with 1, on the stored word).
pub fn fn_009f2970(e: &mut Engine, this: Ptr, reference: u32) {
    e.mem.set_u32(this.addr() + 0x8, reference);
    let stored = e.mem.u32(this.addr() + 0x8);
    e.call(SET_TARGETED, &args![stored, 1u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009f_2970, fn_009f2970(Ptr, u32)),
        entry!(0x009f29a0, fn_009f29a0(Ptr) -> Ptr),
        entry!(0x009f2a30, fn_009f2a30(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patrol_data_constructor_builds_the_array_and_clears_its_fields() {
        let mut e = Engine::new();
        e.register(ACTOR_PACKAGE_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(REFERENCES_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x38);
        e.mem.write(this, &[0xaa; 0x38]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009f_29a0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ACTOR_PACKAGE_DATA_CONSTRUCT, vec![this]));
        assert_eq!(log[2], (REFERENCES_CONSTRUCT, vec![this + 0x1c]));
        assert_eq!(e.mem.u32(this), PATROL_ACTOR_PACKAGE_DATA_VTABLE);
        for offset in [4, 0x10, 0x14, 0x18] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u32(this + 8), 0xaaaa_aaaa);
    }

    #[test]
    fn patrol_data_package_type_is_13() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x009f_2a30, &args![0x1000u32]).u32(), 13);
    }
}

#[cfg(test)]
mod n029_tests {
    use super::*;

    #[test]
    fn fn_009f2970_stores_and_targets_the_reference() {
        let mut e = Engine::new();
        e.register(SET_TARGETED, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x009f_2970, &args![this, 0x5000u32]);
        assert_eq!(e.mem.u32(this + 8), 0x5000);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(SET_TARGETED, vec![0x5000, 1])));
    }
}
