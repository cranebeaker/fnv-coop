//! `fallout shared/packagedataambush.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `00670b30` (`thiscall`, `RET 4`; engine map `PackageLocation::~PackageLocation`): run
/// with 1 on the location at +4.
const LOCATION_DELETE: u32 = 0x0067_0b30;
/// `0067ad70` (`thiscall`, no stack word): the base destructor.
const PACKAGE_DATA_BASE_DESTRUCTOR: u32 = 0x0067_ad70;

// Translated from 0067ad90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAmbushPackageData::scalar deleting destructor` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`): the destructor [`fn_0067adc0`] and, for bit 0 of `flags`,
/// `operator delete` on `this`. Returns `this`.
pub fn tes_ambush_package_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0067adc0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0067adc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067adc0 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f00ba8, state variable) is not translated
/// Unnamed in the engine map (unit `packagedataambush.cpp`, `thiscall`, no stack word): the
/// `TESAmbushPackageData` destructor. Sets the vtable `01068860`, deletes the location at +4
/// when set (`00670b30(1)`), then the base destructor `0067ad70`.
pub fn fn_0067adc0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0106_8860);
    let location = e.mem.u32(this.addr() + 4);
    if location != 0 {
        e.call(LOCATION_DELETE, &args![location, 1u32]);
    }
    e.call(PACKAGE_DATA_BASE_DESTRUCTOR, &args![this]);
}

/// `__RTDynamicCast(object, vfDelta, sourceType, targetType, isReference)`
/// (`cdecl`, five words) and the RTTI type descriptors of `TESPackageData`
/// and `TESAmbushPackageData` (`.?AV<Class>@@` at +8, read from the exe).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TES_PACKAGE_DATA_TYPE: u32 = 0x0119_bdec;
const TES_AMBUSH_PACKAGE_DATA_TYPE: u32 = 0x0119_bdc8;
/// `operator new` (`00401000`, `cdecl`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `PackageLocation::PackageLocation` (engine map, `0067f030`, `thiscall`),
/// `PackageLocation::Copy` (`0067f4d0`, `RET 8`: the other, a word),
/// `PackageLocation::Compare` (`0067f600`, `RET 4`) and
/// `PackageLocation::Save` (`0067f6c0`, `RET 4`: the chunk type).
const LOCATION_CONSTRUCT: u32 = 0x0067_f030;
const LOCATION_COPY: u32 = 0x0067_f4d0;
const LOCATION_COMPARE: u32 = 0x0067_f600;
const LOCATION_SAVE: u32 = 0x0067_f6c0;
/// `TESForm::AddChunk` (engine map, `004856d0`, `cdecl`: the type).
const ADD_CHUNK: u32 = 0x0048_56d0;

// Translated from 0067ae50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067ae50 - the exception-unwinding frame (FS:[0]) around the location's allocation is not translated.
/// `TESAmbushPackageData::Copy` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`): nothing unless `other` is a
/// `TESAmbushPackageData`. The location at +4 is deleted (`00670b30` with
/// 1) and cleared; when the other has one, a new 0xc-byte location
/// (`0067f030`; null when the allocation fails) takes a copy of it
/// (`0067f4d0` with 0).
pub fn tes_ambush_package_data_copy(e: &mut Engine, this: Ptr, other: Ptr) {
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TES_PACKAGE_DATA_TYPE,
                TES_AMBUSH_PACKAGE_DATA_TYPE,
                0u32
            ],
        )
        .u32();
    if cast == 0 {
        return;
    }
    let base = this.addr();
    if e.mem.u32(base + 4) != 0 {
        let location = e.mem.u32(base + 4);
        if location != 0 {
            e.call(LOCATION_DELETE, &args![location, 1u32]);
        }
        e.mem.set_u32(base + 4, 0);
    }
    if e.mem.u32(cast + 4) != 0 {
        let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
        let location = if block != 0 {
            e.call(LOCATION_CONSTRUCT, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(base + 4, location);
        let theirs = e.mem.u32(cast + 4);
        e.call(LOCATION_COPY, &args![location, theirs, 0u32]);
    }
}

// Translated from 0067af50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAmbushPackageData::Compare` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`): true (they differ) when `other` is not a
/// `TESAmbushPackageData`, when this location (+4) compares different
/// (`0067f600`) or, without one, when the other has one.
pub fn tes_ambush_package_data_compare(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TES_PACKAGE_DATA_TYPE,
                TES_AMBUSH_PACKAGE_DATA_TYPE,
                0u32
            ],
        )
        .u32();
    if cast == 0 {
        return true;
    }
    let mine = e.mem.u32(this.addr() + 4);
    if mine != 0 {
        let theirs = e.mem.u32(cast + 4);
        if e.call(LOCATION_COMPARE, &args![mine, theirs]).bool() {
            return true;
        }
    } else if e.mem.u32(cast + 4) != 0 {
        return true;
    }
    false
}

// Translated from 0067afc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAmbushPackageData::Save` (Xbox PDB name in the engine map; no stack
/// word): an empty `PKAM` chunk (`004856d0`), then the location, when there
/// is one, as `PLD2` (`0067f6c0`).
pub fn tes_ambush_package_data_save(e: &mut Engine, this: Ptr) {
    e.call(ADD_CHUNK, &args![0x4d41_4b50u32]);
    let location = e.mem.u32(this.addr() + 4);
    if location != 0 {
        e.call(LOCATION_SAVE, &args![location, 0x3244_4c50u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067_ae50, tes_ambush_package_data_copy(Ptr, Ptr)),
        entry!(0x0067_af50, tes_ambush_package_data_compare(Ptr, Ptr) -> bool),
        entry!(0x0067_afc0, tes_ambush_package_data_save(Ptr)),
        entry!(
            0x0067_ad90,
            tes_ambush_package_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0067_adc0, fn_0067adc0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destructor_deletes_the_location_then_runs_the_base() {
        let mut e = Engine::new();
        for callee in [
            OPERATOR_DELETE,
            LOCATION_DELETE,
            PACKAGE_DATA_BASE_DESTRUCTOR,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x6000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0067_ad90, &args![this, 1u32]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (LOCATION_DELETE, vec![0x6000, 1]),
                (PACKAGE_DATA_BASE_DESTRUCTOR, vec![this]),
                (OPERATOR_DELETE, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0106_8860);
        e.mem.set_u32(this + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0067_ad90, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(PACKAGE_DATA_BASE_DESTRUCTOR, vec![this])]);
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    const CAST: u32 = 0x0f00_0000;

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.mem.map(CAST, 8);
        e.register(DYNAMIC_CAST, |e, _| word(e.mem.u32(CAST)));
        e.register(LOCATION_COMPARE, |e, _| word(e.mem.u32(CAST + 4)));
        e.register(OPERATOR_NEW, |_, _| word(0x7000));
        e.register(LOCATION_CONSTRUCT, |_, a| word(a[0] + 1));
        for addr in [LOCATION_DELETE, LOCATION_COPY, LOCATION_SAVE, ADD_CHUNK] {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let result = e.call(addr, words);
        let log = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect();
        (result, log)
    }

    #[test]
    fn copy_replaces_the_location() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x5000);
        let (_, log) = run(&mut e, 0x0067_ae50, &[this, other]);
        assert_eq!(
            log,
            vec![(
                DYNAMIC_CAST,
                vec![
                    other,
                    0,
                    TES_PACKAGE_DATA_TYPE,
                    TES_AMBUSH_PACKAGE_DATA_TYPE,
                    0
                ]
            )]
        );
        assert_eq!(e.mem.u32(this + 4), 0x5000);
        e.mem.set_u32(CAST, other);
        // The other has none: this one's is deleted.
        let (_, log) = run(&mut e, 0x0067_ae50, &[this, other]);
        assert_eq!(log[1], (LOCATION_DELETE, vec![0x5000, 1]));
        assert_eq!(log.len(), 2);
        assert_eq!(e.mem.u32(this + 4), 0);
        // The other has one: copied into a new location.
        e.mem.set_u32(other + 4, 0x6000);
        let (_, log) = run(&mut e, 0x0067_ae50, &[this, other]);
        assert_eq!(e.mem.u32(this + 4), 0x7001);
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (OPERATOR_NEW, vec![0xc]),
                (LOCATION_CONSTRUCT, vec![0x7000]),
                (LOCATION_COPY, vec![0x7001, 0x6000, 0]),
            ]
        );
    }

    #[test]
    fn compare_checks_the_locations() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        let other = e.mem.alloc(8);
        let compare = |e: &mut Engine| e.call(0x0067_af50, &args![this, other]).bool();
        assert!(compare(&mut e));
        e.mem.set_u32(CAST, other);
        assert!(!compare(&mut e));
        e.mem.set_u32(other + 4, 0x6000);
        assert!(compare(&mut e));
        e.mem.set_u32(this + 4, 0x5000);
        assert!(!compare(&mut e));
        e.mem.set_u32(CAST + 4, 1);
        assert!(compare(&mut e));
    }

    #[test]
    fn save_writes_the_chunk_and_the_location() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        let (_, log) = run(&mut e, 0x0067_afc0, &[this]);
        assert_eq!(log, vec![(ADD_CHUNK, vec![0x4d41_4b50])]);
        e.mem.set_u32(this + 4, 0x5000);
        let (_, log) = run(&mut e, 0x0067_afc0, &[this]);
        assert_eq!(log[1], (LOCATION_SAVE, vec![0x5000, 0x3244_4c50]));
    }
}
