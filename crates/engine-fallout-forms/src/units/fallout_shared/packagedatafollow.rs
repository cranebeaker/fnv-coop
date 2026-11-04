//! `fallout shared/packagedatafollow.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TESFollowPackageData` (Xbox PDB, 0x0C bytes) is the follow variant of a
//! package's data block: the vtable word, `pStartLocation` (a
//! `PackageLocation *`) at +4 and `fTriggerRadius` at +8. Its vtable is at
//! `01068988` (RTTI type `.?AVTESFollowPackageData@@` at `0119be88`); the base
//! class `TESPackageData` only has a vtable (`01068884`, type at `0119bdec`).
//! Exception-unwinding frames (`FS:[0]` chains) are not translated; each
//! function that has one carries a `Translation incomplete` marker.
//!
//! The float field is only ever moved with `FLD` / `FSTP`, which turns a
//! signalling NaN into a quiet one (`x87_quiet`).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (cdecl, size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (cdecl, block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `__RTDynamicCast(object, 0, source type, target type, 0)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `TESPackageData` base-class constructor (stores the base vtable).
const PACKAGE_DATA_CONSTRUCT: u32 = 0x0067_ad20;
/// `TESPackageData` base-class destructor body (stores the base vtable).
const PACKAGE_DATA_DESTRUCT: u32 = 0x0067_ad70;
/// `PackageLocation::PackageLocation` (Xbox PDB), thiscall.
const PACKAGE_LOCATION_CONSTRUCT: u32 = 0x0067_f030;
/// `PackageLocation::~PackageLocation` (Xbox PDB), scalar deleting form
/// (one stack argument: the delete flag).
const PACKAGE_LOCATION_DESTRUCT: u32 = 0x0067_0b30;
/// `PackageLocation::Copy` (Xbox PDB): `this`, source, 0.
const PACKAGE_LOCATION_COPY: u32 = 0x0067_f4d0;
/// `PackageLocation::Compare` (Xbox PDB): `this`, other; nonzero = differ.
const PACKAGE_LOCATION_COMPARE: u32 = 0x0067_f600;
/// `PackageLocation::Save` (Xbox PDB): `this`, chunk type.
const PACKAGE_LOCATION_SAVE: u32 = 0x0067_f6c0;
/// `PackageLocation::SaveGame` (Xbox PDB): `this`, save buffer.
const PACKAGE_LOCATION_SAVE_GAME: u32 = 0x0067_fb60;
/// `BGSSaveGameBuffer` raw write: `this`, data, size, 0.
const SAVE_RAW: u32 = 0x0086_5e50;
/// `TESForm::__AddChunkData` (Xbox PDB), cdecl: chunk type, data, size.
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// Returns a byte flag from a global (`011c54ba`); when it is set the
/// chunk is byte-swapped before and after it is written.
const CHUNK_SWAP_FLAG: u32 = 0x0040_1500;
/// Byte-swaps the 4-byte chunk whose address is in `ECX`.
const SWAP_WORD: u32 = 0x0050_3210;
/// `memset` wrapper (cdecl): destination, value, size.
const MEMSET: u32 = 0x0040_3d30;

/// RTTI type descriptors of the `dynamic_cast` from a `TESPackageData` to a
/// `TESFollowPackageData`.
const TYPE_PACKAGE_DATA: u32 = 0x0119_bdec;
const TYPE_FOLLOW_PACKAGE_DATA: u32 = 0x0119_be88;

/// The vtable of `TESFollowPackageData`.
const FOLLOW_PACKAGE_DATA_VTABLE: u32 = 0x0106_8988;
/// Chunk type of the saved trigger radius (the bytes "PKFD") and of the
/// start location (the bytes "PLD2").
const CHUNK_FOLLOW_DATA: u32 = 0x4446_4b50;
const CHUNK_START_LOCATION: u32 = 0x3244_4c50;
/// Size of a `PackageLocation`.
const PACKAGE_LOCATION_SIZE: u32 = 0x0c;

layout! {
    /// `TESFollowPackageData` (Xbox PDB), 0x0C bytes; the PC offsets are the
    /// same.
    pub struct TESFollowPackageData: 0x0c {
        /// `pStartLocation` (Xbox PDB): `PackageLocation *`.
        0x04 pStartLocation: Ptr,
        /// `fTriggerRadius` (Xbox PDB).
        0x08 fTriggerRadius: f32,
    }
}

/// `dynamic_cast<TESFollowPackageData *>` of a `TESPackageData *`.
fn cast_to_follow_data(e: &mut Engine, data: Ptr) -> Ptr<TESFollowPackageData> {
    e.call(
        RT_DYNAMIC_CAST,
        &args![data, 0, TYPE_PACKAGE_DATA, TYPE_FOLLOW_PACKAGE_DATA, 0],
    )
    .ptr()
}

/// A new `PackageLocation` (allocation, then the constructor when the
/// allocation succeeded).
fn new_package_location(e: &mut Engine) -> Ptr {
    let block: Ptr = e.call(OPERATOR_NEW, &args![PACKAGE_LOCATION_SIZE]).ptr();
    if block.is_null() {
        Ptr::NULL
    } else {
        e.call(PACKAGE_LOCATION_CONSTRUCT, &args![block]).ptr()
    }
}

// Translated from 0067bf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFollowPackageData::SaveGame` (Xbox PDB): writes the trigger radius
/// (4 raw bytes at +8), a byte saying whether there is a start location, and
/// that location's save game data.
pub fn tes_follow_package_data_save_game(
    e: &mut Engine,
    this: Ptr<TESFollowPackageData>,
    buffer: Ptr,
) {
    e.call(SAVE_RAW, &args![buffer, this.addr() + 8, 4, 0]);
    let has_location = !e.get(this, TESFollowPackageData::pStartLocation).is_null();
    let flag = e.with_stack(1, |e, flag| {
        e.mem.set_u8(flag.addr(), has_location as u8);
        e.call(SAVE_RAW, &args![buffer, flag, 1, 0]);
        e.mem.u8(flag.addr())
    });
    if flag != 0 {
        let location = e.get(this, TESFollowPackageData::pStartLocation);
        e.call(PACKAGE_LOCATION_SAVE_GAME, &args![location, buffer]);
    }
}

// Translated from 0067c010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFollowPackageData::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor body and frees the object when bit 0 of `flags` is set.
pub fn tes_follow_package_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<TESFollowPackageData>,
    flags: u32,
) -> Ptr<TESFollowPackageData> {
    fn_0067c080(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0067c040 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `TESFollowPackageData` (the Xbox PDB map gives no name
/// for it): the base constructor, then the class's vtable `01068988`, the
/// trigger radius (its one stack argument, moved with `FLD` / `FSTP`) and a
/// null start location. Returns `this`.
pub fn fn_0067c040(
    e: &mut Engine,
    this: Ptr<TESFollowPackageData>,
    trigger_radius: f32,
) -> Ptr<TESFollowPackageData> {
    e.call(PACKAGE_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), FOLLOW_PACKAGE_DATA_VTABLE);
    e.set(
        this,
        TESFollowPackageData::fTriggerRadius,
        x87_quiet(trigger_radius),
    );
    e.set(this, TESFollowPackageData::pStartLocation, Ptr::NULL);
    this
}

// Translated from 0067c080 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067c080 - C++ exception-unwinding frame (FS:[0] handler chain) not translated
/// The destructor body (`fastcall`; the Xbox PDB map gives no name): stores
/// this class's vtable, deletes the start location (through its scalar
/// deleting destructor) when there is one, and runs the base destructor. The
/// field is not cleared.
pub fn fn_0067c080(e: &mut Engine, this: Ptr<TESFollowPackageData>) {
    e.mem.set_u32(this.addr(), FOLLOW_PACKAGE_DATA_VTABLE);
    let location = e.get(this, TESFollowPackageData::pStartLocation);
    if !location.is_null() {
        e.call(PACKAGE_LOCATION_DESTRUCT, &args![location, 1]);
    }
    e.call(PACKAGE_DATA_DESTRUCT, &args![this]);
}

// Translated from 0067c110 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067c110 - C++ exception-unwinding frame (FS:[0] handler chain) not translated
/// `TESFollowPackageData::Copy` (Xbox PDB): copies the fields of another
/// package data (cast to this class; nothing happens if it is not one). The
/// trigger radius is copied with `FLD` / `FSTP`; the start location is
/// deleted, then copied into a new one when the source has one (the compiler
/// repeated the null test of the old location; both tests are the same).
pub fn tes_follow_package_data_copy(e: &mut Engine, this: Ptr<TESFollowPackageData>, source: Ptr) {
    let other = cast_to_follow_data(e, source);
    if other.is_null() {
        return;
    }
    let radius = e.get(other, TESFollowPackageData::fTriggerRadius);
    e.set(
        this,
        TESFollowPackageData::fTriggerRadius,
        x87_quiet(radius),
    );
    let old_location = e.get(this, TESFollowPackageData::pStartLocation);
    if !old_location.is_null() {
        e.call(PACKAGE_LOCATION_DESTRUCT, &args![old_location, 1]);
        e.set(this, TESFollowPackageData::pStartLocation, Ptr::NULL);
    }
    let source_location = e.get(other, TESFollowPackageData::pStartLocation);
    if !source_location.is_null() {
        let location = new_package_location(e);
        e.set(this, TESFollowPackageData::pStartLocation, location);
        e.call(PACKAGE_LOCATION_COPY, &args![location, source_location, 0]);
    }
}

// Translated from 0067c220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFollowPackageData::Compare` (Xbox PDB): true when the other package
/// data differs from this one (or is not a follow package data). The trigger
/// radii are equal only when `FUCOMPP` finds them equal (a NaN never is);
/// the start locations must both be absent or compare equal through
/// `PackageLocation::Compare`.
pub fn tes_follow_package_data_compare(
    e: &mut Engine,
    this: Ptr<TESFollowPackageData>,
    other: Ptr,
) -> bool {
    let other = cast_to_follow_data(e, other);
    if other.is_null() {
        return true;
    }
    let radius = e.get(this, TESFollowPackageData::fTriggerRadius);
    let other_radius = e.get(other, TESFollowPackageData::fTriggerRadius);
    if radius != other_radius {
        return true;
    }
    let location = e.get(this, TESFollowPackageData::pStartLocation);
    let other_location = e.get(other, TESFollowPackageData::pStartLocation);
    if !location.is_null() {
        e.call(PACKAGE_LOCATION_COMPARE, &args![location, other_location])
            .bool()
    } else {
        !other_location.is_null()
    }
}

// Translated from 0067c2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFollowPackageData::Save` (Xbox PDB): writes the 4-byte "PKFD" chunk
/// holding the trigger radius (moved with `FLD` / `FSTP`), byte swapping it
/// before and after the write when the global flag says so, then the start
/// location as a "PLD2" chunk when there is one.
pub fn tes_follow_package_data_save(e: &mut Engine, this: Ptr<TESFollowPackageData>) {
    e.with_stack(4, |e, chunk| {
        e.call(MEMSET, &args![chunk, 0, 4]);
        let radius = e.get(this, TESFollowPackageData::fTriggerRadius);
        e.mem.set_f32(chunk.addr(), x87_quiet(radius));
        if e.call(CHUNK_SWAP_FLAG, &args![]).bool() {
            e.call(SWAP_WORD, &args![chunk]);
        }
        e.call(ADD_CHUNK_DATA, &args![CHUNK_FOLLOW_DATA, chunk, 4]);
        if e.call(CHUNK_SWAP_FLAG, &args![]).bool() {
            e.call(SWAP_WORD, &args![chunk]);
        }
    });
    let location = e.get(this, TESFollowPackageData::pStartLocation);
    if !location.is_null() {
        e.call(
            PACKAGE_LOCATION_SAVE,
            &args![location, CHUNK_START_LOCATION],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0067bf40,
            tes_follow_package_data_save_game(Ptr<TESFollowPackageData>, Ptr)
        ),
        entry!(
            0x0067c010,
            tes_follow_package_data_scalar_deleting_destructor(
                Ptr<TESFollowPackageData>,
                u32,
            )
                -> Ptr<TESFollowPackageData>
        ),
        entry!(
            0x0067c040,
            fn_0067c040(Ptr<TESFollowPackageData>, f32) -> Ptr<TESFollowPackageData>
        ),
        entry!(0x0067c080, fn_0067c080(Ptr<TESFollowPackageData>)),
        entry!(
            0x0067c110,
            tes_follow_package_data_copy(Ptr<TESFollowPackageData>, Ptr)
        ),
        entry!(
            0x0067c220,
            tes_follow_package_data_compare(Ptr<TESFollowPackageData>, Ptr) -> bool
        ),
        entry!(
            0x0067c2b0,
            tes_follow_package_data_save(Ptr<TESFollowPackageData>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Stand-ins for the casts and the base class: the object passes straight
    /// through the `dynamic_cast`; the base class and `operator delete` are
    /// plain.
    fn engine_with_casts() -> Engine {
        let mut e = Engine::new();
        e.register(RT_DYNAMIC_CAST, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(PACKAGE_DATA_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(PACKAGE_DATA_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e
    }

    fn follow_data(e: &mut Engine) -> Ptr<TESFollowPackageData> {
        e.new_object()
    }

    #[test]
    fn x87_copy_quiets_only_signalling_nans() {
        assert_eq!(
            x87_quiet(f32::from_bits(0x7f80_0001)).to_bits(),
            0x7fc0_0001
        );
        assert_eq!(
            x87_quiet(f32::from_bits(0xff80_0001)).to_bits(),
            0xffc0_0001
        );
        assert_eq!(x87_quiet(f32::INFINITY).to_bits(), 0x7f80_0000);
        assert_eq!(
            x87_quiet(f32::from_bits(0x7fc0_0000)).to_bits(),
            0x7fc0_0000
        );
        assert_eq!(x87_quiet(1.5).to_bits(), 1.5f32.to_bits());
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_with_the_flag() {
        let mut e = engine_with_casts();
        let data = follow_data(&mut e);
        let location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(data, TESFollowPackageData::pStartLocation, location);
        e.register(PACKAGE_LOCATION_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let back = e.call(0x0067_c010, &args![data, 0]).ptr::<()>();
        assert_eq!(back.addr(), data.addr());
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(addr, _)| *addr != OPERATOR_DELETE));
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == PACKAGE_LOCATION_DESTRUCT && a[..] == [location.addr(), 1]));
        assert_eq!(e.mem.u32(data.addr()), FOLLOW_PACKAGE_DATA_VTABLE);

        e.call_log = Some(vec![]);
        e.call(0x0067_c010, &args![data, 1]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == OPERATOR_DELETE && a[..] == [data.addr()]));
    }

    #[test]
    fn constructor_stores_the_vtable_the_radius_and_no_location() {
        let mut e = engine_with_casts();
        let data = follow_data(&mut e);
        e.set(data, TESFollowPackageData::pStartLocation, Ptr::new(0x44));
        let back = e.call(0x0067_c040, &args![data, 128.5f32]).ptr::<()>();
        assert_eq!(back.addr(), data.addr());
        assert_eq!(e.mem.u32(data.addr()), FOLLOW_PACKAGE_DATA_VTABLE);
        assert_eq!(e.get(data, TESFollowPackageData::fTriggerRadius), 128.5);
        assert!(e.get(data, TESFollowPackageData::pStartLocation).is_null());
    }

    #[test]
    fn constructor_turns_a_signalling_nan_radius_quiet() {
        let mut e = engine_with_casts();
        let data = follow_data(&mut e);
        e.call(0x0067_c040, &args![data, f32::from_bits(0x7f80_0001)]);
        assert_eq!(
            e.get(data, TESFollowPackageData::fTriggerRadius).to_bits(),
            0x7fc0_0001
        );
    }

    #[test]
    fn destructor_body_without_a_location_only_runs_the_base() {
        let mut e = engine_with_casts();
        let data = follow_data(&mut e);
        e.register(PACKAGE_LOCATION_DESTRUCT, |_, _| panic!("no location"));
        e.call(0x0067_c080, &args![data]);
        assert_eq!(e.mem.u32(data.addr()), FOLLOW_PACKAGE_DATA_VTABLE);
    }

    #[test]
    fn destructor_body_deletes_the_location() {
        let mut e = engine_with_casts();
        let data = follow_data(&mut e);
        let location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(data, TESFollowPackageData::pStartLocation, location);
        e.register(PACKAGE_LOCATION_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0067_c080, &args![data]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == PACKAGE_LOCATION_DESTRUCT && a[..] == [location.addr(), 1]));
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == PACKAGE_DATA_DESTRUCT && a[..] == [data.addr()]));
        // The field is not cleared.
        assert_eq!(
            e.get(data, TESFollowPackageData::pStartLocation).addr(),
            location.addr()
        );
    }

    #[test]
    fn copy_replaces_the_location_and_copies_the_radius() {
        let mut e = engine_with_casts();
        let this = follow_data(&mut e);
        let source = follow_data(&mut e);
        e.set(
            source,
            TESFollowPackageData::fTriggerRadius,
            f32::from_bits(0x7f80_0001),
        );
        let source_location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(
            source,
            TESFollowPackageData::pStartLocation,
            source_location,
        );
        let old_location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(this, TESFollowPackageData::pStartLocation, old_location);
        let new_block = e.mem.alloc(0x0c);
        e.register_double(OPERATOR_NEW, move |_, a| {
            assert_eq!(a[0], 0x0c);
            Ret {
                eax: new_block,
                ..Ret::default()
            }
        });
        e.register(PACKAGE_LOCATION_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(PACKAGE_LOCATION_DESTRUCT, |_, _| Ret::default());
        e.register(PACKAGE_LOCATION_COPY, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0067_c110, &args![this, source]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(
            |(addr, a)| *addr == PACKAGE_LOCATION_DESTRUCT && a[..] == [old_location.addr(), 1]
        ));
        assert!(log.iter().any(|(addr, a)| *addr == PACKAGE_LOCATION_COPY
            && a[..] == [new_block, source_location.addr(), 0]));
        assert_eq!(
            e.get(this, TESFollowPackageData::pStartLocation).addr(),
            new_block
        );
        // The copy goes through FLD / FSTP: the signalling NaN comes back quiet.
        assert_eq!(
            e.get(this, TESFollowPackageData::fTriggerRadius).to_bits(),
            0x7fc0_0001
        );
    }

    #[test]
    fn copy_without_a_source_location_leaves_none() {
        let mut e = engine_with_casts();
        let this = follow_data(&mut e);
        let source = follow_data(&mut e);
        e.set(source, TESFollowPackageData::fTriggerRadius, 3.0);
        let old_location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(this, TESFollowPackageData::pStartLocation, old_location);
        e.register(PACKAGE_LOCATION_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |_, _| panic!("source has no location"));
        e.call(0x0067_c110, &args![this, source]);
        assert!(e.get(this, TESFollowPackageData::pStartLocation).is_null());
        assert_eq!(e.get(this, TESFollowPackageData::fTriggerRadius), 3.0);
    }

    #[test]
    fn copy_of_something_that_is_not_follow_data_does_nothing() {
        let mut e = engine_with_casts();
        let this = follow_data(&mut e);
        e.set(this, TESFollowPackageData::fTriggerRadius, 9.0);
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        e.call(0x0067_c110, &args![this, 0x1000u32]);
        assert_eq!(e.get(this, TESFollowPackageData::fTriggerRadius), 9.0);
    }

    #[test]
    fn compare_finds_a_different_radius_and_never_equates_a_nan() {
        let mut e = engine_with_casts();
        let a = follow_data(&mut e);
        let b = follow_data(&mut e);
        assert!(!e.call(0x0067_c220, &args![a, b]).bool());
        e.set(b, TESFollowPackageData::fTriggerRadius, 1.0);
        assert!(e.call(0x0067_c220, &args![a, b]).bool());
        // Equal values, signed zeros included, compare equal.
        e.set(a, TESFollowPackageData::fTriggerRadius, 0.0);
        e.set(b, TESFollowPackageData::fTriggerRadius, -0.0);
        assert!(!e.call(0x0067_c220, &args![a, b]).bool());
        // A NaN differs from everything, itself included.
        e.set(a, TESFollowPackageData::fTriggerRadius, f32::NAN);
        e.set(b, TESFollowPackageData::fTriggerRadius, f32::NAN);
        assert!(e.call(0x0067_c220, &args![a, b]).bool());
        // Not a follow package data.
        e.set(b, TESFollowPackageData::fTriggerRadius, 0.0);
        e.set(a, TESFollowPackageData::fTriggerRadius, 0.0);
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        assert!(e.call(0x0067_c220, &args![a, b]).bool());
    }

    #[test]
    fn compare_locations_by_presence_and_by_their_compare() {
        let mut e = engine_with_casts();
        let a = follow_data(&mut e);
        let b = follow_data(&mut e);
        let location_a = Ptr::<()>::new(e.mem.alloc(0x0c));
        let location_b = Ptr::<()>::new(e.mem.alloc(0x0c));
        // Only the other has a location.
        e.set(b, TESFollowPackageData::pStartLocation, location_b);
        assert!(e.call(0x0067_c220, &args![a, b]).bool());
        // Both have one: PackageLocation::Compare decides.
        e.set(a, TESFollowPackageData::pStartLocation, location_a);
        e.register(PACKAGE_LOCATION_COMPARE, |_, _| Ret::default());
        assert!(!e.call(0x0067_c220, &args![a, b]).bool());
        e.register(PACKAGE_LOCATION_COMPARE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        assert!(e.call(0x0067_c220, &args![a, b]).bool());
        // Only this one has a location: Compare is asked with a null other.
        e.set(b, TESFollowPackageData::pStartLocation, Ptr::NULL);
        e.register(PACKAGE_LOCATION_COMPARE, |_, a| {
            assert_eq!(a[1], 0);
            Ret::default()
        });
        assert!(!e.call(0x0067_c220, &args![a, b]).bool());
    }

    #[test]
    fn save_writes_the_radius_chunk_and_swaps_around_it_when_flagged() {
        let mut e = engine_with_casts();
        let data = follow_data(&mut e);
        e.set(data, TESFollowPackageData::fTriggerRadius, 250.0);
        let location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(data, TESFollowPackageData::pStartLocation, location);
        e.register(MEMSET, |e, a| {
            assert_eq!((a[1], a[2]), (0, 4));
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(CHUNK_SWAP_FLAG, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(SWAP_WORD, |_, _| Ret::default());
        let written = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let sink = written.clone();
        e.register_double(ADD_CHUNK_DATA, move |e, a| {
            *sink.borrow_mut() = e.mem.bytes(a[1], a[2]);
            assert_eq!(a[0], CHUNK_FOLLOW_DATA);
            Ret::default()
        });
        e.register(PACKAGE_LOCATION_SAVE, |_, a| {
            assert_eq!(a[1], CHUNK_START_LOCATION);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0067_c2b0, &args![data]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|(addr, _)| *addr == SWAP_WORD).count(), 2);
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == PACKAGE_LOCATION_SAVE && a[0] == location.addr()));
        assert_eq!(*written.borrow(), 250.0f32.to_le_bytes().to_vec());
    }

    #[test]
    fn save_without_a_location_or_swap_writes_only_the_chunk() {
        let mut e = engine_with_casts();
        let data = follow_data(&mut e);
        e.set(
            data,
            TESFollowPackageData::fTriggerRadius,
            f32::from_bits(0x7f80_0001),
        );
        e.register(MEMSET, |_, _| Ret::default());
        e.register(CHUNK_SWAP_FLAG, |_, _| Ret::default());
        e.register(ADD_CHUNK_DATA, |e, a| {
            // The radius went through FLD / FSTP.
            assert_eq!(e.mem.u32(a[1]), 0x7fc0_0001);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0067_c2b0, &args![data]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .all(|(addr, _)| *addr != SWAP_WORD && *addr != PACKAGE_LOCATION_SAVE));
    }

    #[test]
    fn save_game_writes_the_radius_the_flag_and_the_location() {
        let mut e = engine_with_casts();
        let data = follow_data(&mut e);
        let buffer = Ptr::<()>::new(e.mem.alloc(8));
        let location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(data, TESFollowPackageData::pStartLocation, location);
        let flag = std::rc::Rc::new(std::cell::Cell::new(9u8));
        let sink = flag.clone();
        e.register_double(SAVE_RAW, move |e, a| {
            if a[2] == 1 {
                sink.set(e.mem.u8(a[1]));
            }
            Ret::default()
        });
        e.register(PACKAGE_LOCATION_SAVE_GAME, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0067_bf40, &args![data, buffer]);
        let log = e.call_log.take().unwrap();
        let raw: Vec<_> = log.iter().filter(|(addr, _)| *addr == SAVE_RAW).collect();
        assert_eq!(raw.len(), 2);
        assert_eq!(raw[0].1[..], [buffer.addr(), data.addr() + 8, 4, 0]);
        assert_eq!(raw[1].1[0], buffer.addr());
        assert_eq!(raw[1].1[2..], [1, 0]);
        assert_eq!(flag.get(), 1);
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == PACKAGE_LOCATION_SAVE_GAME
                && a[..] == [location.addr(), buffer.addr()]));
    }

    #[test]
    fn save_game_without_a_location_skips_the_location() {
        let mut e = engine_with_casts();
        let data = follow_data(&mut e);
        let buffer = Ptr::<()>::new(e.mem.alloc(8));
        let flag = std::rc::Rc::new(std::cell::Cell::new(9u8));
        let sink = flag.clone();
        e.register_double(SAVE_RAW, move |e, a| {
            if a[2] == 1 {
                sink.set(e.mem.u8(a[1]));
            }
            Ret::default()
        });
        e.register(PACKAGE_LOCATION_SAVE_GAME, |_, _| panic!("no location"));
        e.call(0x0067_bf40, &args![data, buffer]);
        assert_eq!(flag.get(), 0);
    }
}
