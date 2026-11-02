//! `fallout shared/packagedataescort.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESEscortPackageData`'s vtable (RTTI `.?AVTESEscortPackageData@@`).
const TES_ESCORT_PACKAGE_DATA_VTABLE: u32 = 0x0106_8960;
/// `TESPackageData`'s constructor (`0067ad20`).
const TES_PACKAGE_DATA_CONSTRUCT: u32 = 0x0067_ad20;
/// `TESPackageData`'s destructor body (`0067ad70`).
const TES_PACKAGE_DATA_DESTRUCT: u32 = 0x0067_ad70;

// Translated from 0067bc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEscortPackageData::`scalar deleting destructor'` (Xbox PDB): the
/// destructor body [`fn_0067bca0`], then the release `00401030` when bit 0
/// of `flags` is set. Returns `this`.
pub fn tes_escort_package_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0067bca0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0067bc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEscortPackageData` constructor from a value: the `TESPackageData`
/// constructor, the vtable `01068960`, +0x4 = 0 and +0x8 = the word at
/// `source` (one stack word). Returns `this`.
pub fn fn_0067bc60(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(TES_PACKAGE_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), TES_ESCORT_PACKAGE_DATA_VTABLE);
    e.mem.set_u32(this.addr() + 4, 0);
    let value = e.mem.u32(source.addr());
    e.mem.set_u32(this.addr() + 8, value);
    this
}

// Translated from 0067bca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067bca0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `TESEscortPackageData` destructor body: the vtable `01068960`, then the
/// `PackageLocation` at +0x4, when there is one, deleted through
/// `00670b30` (`PackageLocation::~PackageLocation` in the engine map, called
/// with 1: the deleting form), then the `TESPackageData` destructor body.
/// The pointer at +0x4 is left as it was.
pub fn fn_0067bca0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TES_ESCORT_PACKAGE_DATA_VTABLE);
    let location = e.mem.u32(this.addr() + 4);
    if location != 0 {
        e.call(0x0067_0b30, &args![location, 1u32]);
    }
    e.call(TES_PACKAGE_DATA_DESTRUCT, &args![this]);
}

/// `__RTDynamicCast(object, vfDelta, sourceType, targetType, isReference)`
/// (`cdecl`, five words) and the RTTI type descriptors of `TESPackageData`
/// and `TESEscortPackageData` (`.?AV<Class>@@` at +8, read from the exe).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TES_PACKAGE_DATA_TYPE: u32 = 0x0119_bdec;
const TES_ESCORT_PACKAGE_DATA_TYPE: u32 = 0x0119_be60;
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
/// `00670b30` (`thiscall`, `RET 4`; engine map `PackageLocation::~PackageLocation`):
/// run with 1 on a location to delete it.
const LOCATION_DELETE: u32 = 0x0067_0b30;
/// `00403d30` (`cdecl`: destination, value, count): fills bytes.
const FILL_BYTES: u32 = 0x0040_3d30;
/// `00401500` (no parameters): whether saved data is byte-swapped (`AL`).
const NEEDS_SWAP: u32 = 0x0040_1500;
/// `00503210` (`thiscall`, no stack word): byte-swaps the word at `this`.
const SWAP_WORD: u32 = 0x0050_3210;
/// `TESForm::__AddChunkData` (engine map, `00485990`, `cdecl`: type, data,
/// size).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;

// Translated from 0067bd30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067bd30 - the exception-unwinding frame (FS:[0]) around the location's allocation is not translated.
/// `TESEscortPackageData::Copy` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`): nothing unless `other` is a
/// `TESEscortPackageData`. The location at +4 is deleted (`00670b30` with
/// 1) and cleared; when the other has one, a new 0xc-byte location
/// (`0067f030`; null when the allocation fails) takes a copy of it
/// (`0067f4d0` with 0). Then the word at +8 is copied.
pub fn tes_escort_package_data_copy(e: &mut Engine, this: Ptr, other: Ptr) {
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TES_PACKAGE_DATA_TYPE,
                TES_ESCORT_PACKAGE_DATA_TYPE,
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
    let value = e.mem.u32(cast + 8);
    e.mem.set_u32(base + 8, value);
}

// Translated from 0067be40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEscortPackageData::Compare` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`): true (they differ) when `other` is not a
/// `TESEscortPackageData`, when this location (+4) compares different
/// (`0067f600`) or, without one, when the other has one, or when
/// the words at +8 differ.
pub fn tes_escort_package_data_compare(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TES_PACKAGE_DATA_TYPE,
                TES_ESCORT_PACKAGE_DATA_TYPE,
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
    e.mem.u32(this.addr() + 8) != e.mem.u32(cast + 8)
}

// Translated from 0067bec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESEscortPackageData::Save` (Xbox PDB name in the engine map; no stack
/// word): the word at +8 as chunk `PKE2` (`00485990`, through a stack word
/// cleared by `00403d30`, byte-swapped before and after by `00503210` when
/// `00401500` says so), then the location, when there is one, as `PLD2`
/// (`0067f6c0`).
pub fn tes_escort_package_data_save(e: &mut Engine, this: Ptr) {
    e.with_stack(4, |e, buffer| {
        e.call(FILL_BYTES, &args![buffer, 0u32, 4u32]);
        let value = e.mem.u32(this.addr() + 8);
        e.mem.set_u32(buffer.addr(), value);
        if e.call(NEEDS_SWAP, &args![]).bool() {
            e.call(SWAP_WORD, &args![buffer]);
        }
        e.call(ADD_CHUNK_DATA, &args![0x3245_4b50u32, buffer, 4u32]);
        if e.call(NEEDS_SWAP, &args![]).bool() {
            e.call(SWAP_WORD, &args![buffer]);
        }
    });
    let location = e.mem.u32(this.addr() + 4);
    if location != 0 {
        e.call(LOCATION_SAVE, &args![location, 0x3244_4c50u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067_bd30, tes_escort_package_data_copy(Ptr, Ptr)),
        entry!(0x0067_be40, tes_escort_package_data_compare(Ptr, Ptr) -> bool),
        entry!(0x0067_bec0, tes_escort_package_data_save(Ptr)),
        entry!(
            0x0067_bc30,
            tes_escort_package_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0067_bc60, fn_0067bc60(Ptr, Ptr) -> Ptr),
        entry!(0x0067_bca0, fn_0067bca0(Ptr)),
    ]
}

#[cfg(test)]
mod b028_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for callee in [
            TES_PACKAGE_DATA_CONSTRUCT,
            TES_PACKAGE_DATA_DESTRUCT,
            0x0067_0b30,
            0x0040_1030,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn fn_0067bc60_copies_the_value() {
        let mut e = engine();
        let this = e.mem.alloc(0xc);
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, 345);
        e.mem.set_u32(this + 4, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0067_bc60, &args![this, source]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(TES_PACKAGE_DATA_CONSTRUCT, vec![this])));
        assert_eq!(e.mem.u32(this), TES_ESCORT_PACKAGE_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 345);
    }

    #[test]
    fn fn_0067bca0_deletes_the_location_when_present() {
        let mut e = engine();
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 4, 0x7000);
        e.call_log = Some(vec![]);
        e.call(0x0067_bca0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0067_0b30, vec![0x7000, 1]),
                (TES_PACKAGE_DATA_DESTRUCT, vec![this])
            ]
        );
        assert_eq!(e.mem.u32(this), TES_ESCORT_PACKAGE_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x7000);

        e.mem.set_u32(this + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0067_bca0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(TES_PACKAGE_DATA_DESTRUCT, vec![this])]);
    }

    #[test]
    fn the_deleting_destructor_releases_only_with_bit_0() {
        let mut e = engine();
        let this = e.mem.alloc(0xc);
        for flags in [0u32, 1, 2, 3] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0067_bc30, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            let releases: Vec<_> = log.iter().filter(|c| c.0 == 0x0040_1030).collect();
            assert_eq!(releases.len(), (flags & 1) as usize);
            assert!(log.contains(&(TES_PACKAGE_DATA_DESTRUCT, vec![this])));
        }
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
        e.mem.map(CAST, 0xc);
        e.register(DYNAMIC_CAST, |e, _| word(e.mem.u32(CAST)));
        e.register(LOCATION_COMPARE, |e, _| word(e.mem.u32(CAST + 4)));
        e.register(NEEDS_SWAP, |e, _| word(e.mem.u32(CAST + 8)));
        e.register(OPERATOR_NEW, |_, _| word(0x7000));
        e.register(LOCATION_CONSTRUCT, |_, a| word(a[0] + 1));
        e.register(ADD_CHUNK_DATA, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x1234);
            Ret::default()
        });
        for addr in [
            LOCATION_DELETE,
            LOCATION_COPY,
            LOCATION_SAVE,
            FILL_BYTES,
            SWAP_WORD,
        ] {
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
    fn copy_replaces_the_location_and_the_word() {
        let mut e = engine();
        let this = e.mem.alloc(0xc);
        let other = e.mem.alloc(0xc);
        e.mem.set_u32(this + 4, 0x5000);
        e.mem.set_u32(other + 8, 0x99);
        let (_, log) = run(&mut e, 0x0067_bd30, &[this, other]);
        assert_eq!(
            log,
            vec![(
                DYNAMIC_CAST,
                vec![
                    other,
                    0,
                    TES_PACKAGE_DATA_TYPE,
                    TES_ESCORT_PACKAGE_DATA_TYPE,
                    0
                ]
            )]
        );
        assert_eq!(e.mem.u32(this + 8), 0);
        e.mem.set_u32(CAST, other);
        let (_, log) = run(&mut e, 0x0067_bd30, &[this, other]);
        assert_eq!(log[1], (LOCATION_DELETE, vec![0x5000, 1]));
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0x99);
        e.mem.set_u32(other + 4, 0x6000);
        let (_, log) = run(&mut e, 0x0067_bd30, &[this, other]);
        assert_eq!(e.mem.u32(this + 4), 0x7001);
        assert_eq!(
            log.last().unwrap(),
            &(LOCATION_COPY, vec![0x7001, 0x6000, 0])
        );
    }

    #[test]
    fn compare_checks_the_location_and_the_word() {
        let mut e = engine();
        let this = e.mem.alloc(0xc);
        let other = e.mem.alloc(0xc);
        let compare = |e: &mut Engine| e.call(0x0067_be40, &args![this, other]).bool();
        assert!(compare(&mut e));
        e.mem.set_u32(CAST, other);
        assert!(!compare(&mut e));
        e.mem.set_u32(other + 8, 1);
        assert!(compare(&mut e));
        e.mem.set_u32(other + 8, 0);
        e.mem.set_u32(other + 4, 0x6000);
        assert!(compare(&mut e));
        e.mem.set_u32(this + 4, 0x5000);
        assert!(!compare(&mut e));
        e.mem.set_u32(CAST + 4, 1);
        assert!(compare(&mut e));
    }

    #[test]
    fn save_writes_the_word_and_the_location() {
        let mut e = engine();
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 0x1234);
        let (_, log) = run(&mut e, 0x0067_bec0, &[this]);
        let buffer = log[0].1[0];
        assert_eq!(log[0], (FILL_BYTES, vec![buffer, 0, 4]));
        assert!(log.contains(&(ADD_CHUNK_DATA, vec![0x3245_4b50, buffer, 4])));
        assert!(!log.iter().any(|c| c.0 == SWAP_WORD || c.0 == LOCATION_SAVE));
        e.mem.set_u32(CAST + 8, 1);
        e.mem.set_u32(this + 4, 0x5000);
        let (_, log) = run(&mut e, 0x0067_bec0, &[this]);
        assert_eq!(log.iter().filter(|c| c.0 == SWAP_WORD).count(), 2);
        assert_eq!(
            log.last().unwrap(),
            &(LOCATION_SAVE, vec![0x5000, 0x3244_4c50])
        );
    }
}
