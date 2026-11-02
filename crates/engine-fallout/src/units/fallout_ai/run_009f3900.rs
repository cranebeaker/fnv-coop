//! `fallout/ai/run_009f3900` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `ActorPackageData`'s constructor (`0067d920`: stores the base vtable).
const ACTOR_PACKAGE_DATA_CONSTRUCT: u32 = 0x0067_d920;
/// The `PotentialTargets` array's constructor (`009f61b0`).
const POTENTIAL_TARGETS_CONSTRUCT: u32 = 0x009f_61b0;
/// The `SandBoxActorPackageData` vtable (its RTTI locator names
/// `.?AVSandBoxActorPackageData@@`).
const SANDBOX_ACTOR_PACKAGE_DATA_VTABLE: u32 = 0x0109_35f8;

// Translated from 009f3900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009f3900 - C++ exception unwinding (FS:[0] frame) not translated
/// The `SandBoxActorPackageData` constructor (Xbox PDB field names): the
/// `ActorPackageData` base, the class's vtable, the `PotentialTargets`
/// array (+0x20), then `eProcedure` -1, `fProcedureDuration`,
/// `fLastTimeChanged` and `fTotalTime` 0.0, `iNextTimeToScanWorld` 0,
/// `iNextTimeToChangeLocations` -1, `bShouldPickNewProcedure` 1, `bFood`,
/// `bChair`, `bBed` 0, `pCurrentTargetObject` and `pPrevTargetObject` null,
/// `cCurrentTargetMarker` 0x7F, `pLocation` null, `eState` 0,
/// `iActivityStartDay` 0, `fActivityStartHour` and `fActivityDuration` 0.0,
/// `eActivity` -1 and the six `cProcedureWeight` bytes 0x80, in the exe's
/// order. `iIdleMarkers` and `iPrevTargetChangedTimeStamp` are left as they
/// are. Returns `this`.
pub fn fn_009f3900(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(ACTOR_PACKAGE_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, SANDBOX_ACTOR_PACKAGE_DATA_VTABLE);
    e.call(POTENTIAL_TARGETS_CONSTRUCT, &args![base + 0x20]);
    e.mem.set_u32(base + 8, 0xffff_ffff);
    e.mem.set_f32(base + 0xc, 0.0);
    e.mem.set_f32(base + 0x14, 0.0);
    e.mem.set_f32(base + 0x18, 0.0);
    e.mem.set_u32(base + 0x30, 0);
    e.mem.set_u32(base + 0x1c, 0xffff_ffff);
    e.mem.set_u8(base + 0x34, 1);
    e.mem.set_u8(base + 0x35, 0);
    e.mem.set_u8(base + 0x36, 0);
    e.mem.set_u8(base + 0x37, 0);
    e.mem.set_u32(base + 0x3c, 0);
    e.mem.set_u32(base + 0x44, 0);
    e.mem.set_u8(base + 0x40, 0x7f);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 4, 0);
    e.mem.set_u32(base + 0x4c, 0);
    e.mem.set_f32(base + 0x50, 0.0);
    e.mem.set_f32(base + 0x54, 0.0);
    e.mem.set_u32(base + 0x58, 0xffff_ffff);
    for i in 0..6 {
        e.mem.set_u8(base + 0x5c + i, 0x80);
    }
    this
}

// Translated from 009f3a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SandBoxActorPackageData::GetPackageType` (vtable slot +8 by the Xbox
/// PDB's class): 12.
pub fn fn_009f3a30(_e: &mut Engine, _this: Ptr) -> u32 {
    0xc
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x009f3900, fn_009f3900(Ptr) -> Ptr),
        entry!(0x009f3a30, fn_009f3a30(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sandbox_data_constructor_sets_every_start_value() {
        let mut e = Engine::new();
        e.register(ACTOR_PACKAGE_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(POTENTIAL_TARGETS_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x64);
        e.mem.write(this, &[0xaa; 0x64]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009f_3900, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ACTOR_PACKAGE_DATA_CONSTRUCT, vec![this]));
        assert_eq!(log[2], (POTENTIAL_TARGETS_CONSTRUCT, vec![this + 0x20]));
        assert_eq!(e.mem.u32(this), SANDBOX_ACTOR_PACKAGE_DATA_VTABLE);
        for offset in [8, 0x1c, 0x58] {
            assert_eq!(e.mem.u32(this + offset), 0xffff_ffff);
        }
        for offset in [4, 0xc, 0x10, 0x14, 0x18, 0x30, 0x3c, 0x44, 0x4c, 0x50, 0x54] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.bytes(this + 0x34, 4), vec![1, 0, 0, 0]);
        assert_eq!(e.mem.u8(this + 0x40), 0x7f);
        assert_eq!(e.mem.bytes(this + 0x5c, 6), vec![0x80; 6]);
        assert_eq!(e.mem.u32(this + 0x38), 0xaaaa_aaaa);
        assert_eq!(e.mem.u32(this + 0x48), 0xaaaa_aaaa);
    }

    #[test]
    fn sandbox_data_package_type_is_12() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x009f_3a30, &args![0x1000u32]).u32(), 12);
    }
}
