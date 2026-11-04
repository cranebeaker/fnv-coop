//! `fallout shared/packagedatauseitem.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TESUseItemPackageData` (Xbox PDB, 8 bytes) is the use-item variant of a
//! package's data block: the vtable word and `pSearchLocation` (a
//! `PackageLocation *`) at +4. Its vtable is at `01068a0c` (RTTI type
//! `.?AVTESUseItemPackageData@@` at `0119bed8`); the base class
//! `TESPackageData` only has a vtable (`01068884`, type at `0119bdec`).
//! Exception-unwinding frames (`FS:[0]` chains) are not translated; each
//! function that has one carries a `Translation incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (cdecl, size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (cdecl, block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `__RTDynamicCast(object, 0, source type, target type, 0)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
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
/// `PackageLocation::InitLoadGame` (Xbox PDB): `this`, argument.
const PACKAGE_LOCATION_INIT_LOAD_GAME: u32 = 0x0067_fc50;
/// `BGSSaveGameBuffer` raw write: `this`, data, size, 0.
const SAVE_RAW: u32 = 0x0086_5e50;
/// `TESForm::AddChunk` (Xbox PDB), cdecl: chunk type.
const ADD_CHUNK: u32 = 0x0048_56d0;

/// RTTI type descriptors of the `dynamic_cast` from a `TESPackageData` to a
/// `TESUseItemPackageData`.
const TYPE_PACKAGE_DATA: u32 = 0x0119_bdec;
const TYPE_USE_ITEM_PACKAGE_DATA: u32 = 0x0119_bed8;

/// The vtable of `TESUseItemPackageData`.
const USE_ITEM_PACKAGE_DATA_VTABLE: u32 = 0x0106_8a0c;
/// Chunk type of the use-item data (the bytes "PUID") and of the search
/// location (the bytes "PLD2").
const CHUNK_USE_ITEM_DATA: u32 = 0x4449_5550;
const CHUNK_SEARCH_LOCATION: u32 = 0x3244_4c50;
/// Size of a `PackageLocation`.
const PACKAGE_LOCATION_SIZE: u32 = 0x0c;

layout! {
    /// `TESUseItemPackageData` (Xbox PDB), 8 bytes; the PC offsets are the
    /// same.
    pub struct TESUseItemPackageData: 0x08 {
        /// `pSearchLocation` (Xbox PDB): `PackageLocation *`.
        0x04 pSearchLocation: Ptr,
    }
}

/// `dynamic_cast<TESUseItemPackageData *>` of a `TESPackageData *`.
fn cast_to_use_item_data(e: &mut Engine, data: Ptr) -> Ptr<TESUseItemPackageData> {
    e.call(
        RT_DYNAMIC_CAST,
        &args![data, 0, TYPE_PACKAGE_DATA, TYPE_USE_ITEM_PACKAGE_DATA, 0],
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

// Translated from 0067b000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESUseItemPackageData::SaveGame` (Xbox PDB): writes a byte saying
/// whether there is a search location, and that location's save game data.
pub fn tes_use_item_package_data_save_game(
    e: &mut Engine,
    this: Ptr<TESUseItemPackageData>,
    buffer: Ptr,
) {
    let has_location = !e
        .get(this, TESUseItemPackageData::pSearchLocation)
        .is_null();
    let flag = e.with_stack(1, |e, flag| {
        e.mem.set_u8(flag.addr(), has_location as u8);
        e.call(SAVE_RAW, &args![buffer, flag, 1, 0]);
        e.mem.u8(flag.addr())
    });
    if flag != 0 {
        let location = e.get(this, TESUseItemPackageData::pSearchLocation);
        e.call(PACKAGE_LOCATION_SAVE_GAME, &args![location, buffer]);
    }
}

// Translated from 0067bfa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESUseItemPackageData::InitLoadGame` (Xbox PDB): forwards its argument to
/// the search location's `InitLoadGame` when there is one.
pub fn tes_use_item_package_data_init_load_game(
    e: &mut Engine,
    this: Ptr<TESUseItemPackageData>,
    argument: u32,
) {
    let location = e.get(this, TESUseItemPackageData::pSearchLocation);
    if !location.is_null() {
        e.call(PACKAGE_LOCATION_INIT_LOAD_GAME, &args![location, argument]);
    }
}

// Translated from 0067c7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESUseItemPackageData::_scalar_deleting_destructor_` (Xbox PDB): runs
/// the destructor body and frees the object when bit 0 of `flags` is set.
pub fn tes_use_item_package_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<TESUseItemPackageData>,
    flags: u32,
) -> Ptr<TESUseItemPackageData> {
    fn_0067c7d0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0067c7d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067c7d0 - C++ exception-unwinding frame (FS:[0] handler chain) not translated
/// The destructor body (`fastcall`; the Xbox PDB map gives no name): stores
/// this class's vtable, deletes the search location (through its scalar
/// deleting destructor) when there is one, and runs the base destructor. The
/// field is not cleared.
pub fn fn_0067c7d0(e: &mut Engine, this: Ptr<TESUseItemPackageData>) {
    e.mem.set_u32(this.addr(), USE_ITEM_PACKAGE_DATA_VTABLE);
    let location = e.get(this, TESUseItemPackageData::pSearchLocation);
    if !location.is_null() {
        e.call(PACKAGE_LOCATION_DESTRUCT, &args![location, 1]);
    }
    e.call(PACKAGE_DATA_DESTRUCT, &args![this]);
}

// Translated from 0067c860 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0067c860 - C++ exception-unwinding frame (FS:[0] handler chain) not translated
/// `TESUseItemPackageData::Copy` (Xbox PDB): copies the search location of
/// another package data (cast to this class; nothing happens if it is not
/// one): the old location is deleted, then a copy of the source's is made
/// when it has one.
pub fn tes_use_item_package_data_copy(
    e: &mut Engine,
    this: Ptr<TESUseItemPackageData>,
    source: Ptr,
) {
    let other = cast_to_use_item_data(e, source);
    if other.is_null() {
        return;
    }
    let old_location = e.get(this, TESUseItemPackageData::pSearchLocation);
    if !old_location.is_null() {
        e.call(PACKAGE_LOCATION_DESTRUCT, &args![old_location, 1]);
        e.set(this, TESUseItemPackageData::pSearchLocation, Ptr::NULL);
    }
    let source_location = e.get(other, TESUseItemPackageData::pSearchLocation);
    if !source_location.is_null() {
        let location = new_package_location(e);
        e.set(this, TESUseItemPackageData::pSearchLocation, location);
        e.call(PACKAGE_LOCATION_COPY, &args![location, source_location, 0]);
    }
}

// Translated from 0067c960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESUseItemPackageData::Compare` (Xbox PDB): true when the other package
/// data differs from this one (or is not a use-item package data): the
/// search locations must both be absent or compare equal through
/// `PackageLocation::Compare`.
pub fn tes_use_item_package_data_compare(
    e: &mut Engine,
    this: Ptr<TESUseItemPackageData>,
    other: Ptr,
) -> bool {
    let other = cast_to_use_item_data(e, other);
    if other.is_null() {
        return true;
    }
    let location = e.get(this, TESUseItemPackageData::pSearchLocation);
    let other_location = e.get(other, TESUseItemPackageData::pSearchLocation);
    if !location.is_null() {
        e.call(PACKAGE_LOCATION_COMPARE, &args![location, other_location])
            .bool()
    } else {
        !other_location.is_null()
    }
}

// Translated from 0067c9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESUseItemPackageData::Save` (Xbox PDB): adds the empty "PUID" chunk
/// (`TESForm::AddChunk`), then the search location as a "PLD2" chunk when
/// there is one.
pub fn tes_use_item_package_data_save(e: &mut Engine, this: Ptr<TESUseItemPackageData>) {
    e.call(ADD_CHUNK, &args![CHUNK_USE_ITEM_DATA]);
    let location = e.get(this, TESUseItemPackageData::pSearchLocation);
    if !location.is_null() {
        e.call(
            PACKAGE_LOCATION_SAVE,
            &args![location, CHUNK_SEARCH_LOCATION],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0067b000,
            tes_use_item_package_data_save_game(Ptr<TESUseItemPackageData>, Ptr)
        ),
        entry!(
            0x0067bfa0,
            tes_use_item_package_data_init_load_game(Ptr<TESUseItemPackageData>, u32)
        ),
        entry!(
            0x0067c7a0,
            tes_use_item_package_data_scalar_deleting_destructor(
                Ptr<TESUseItemPackageData>,
                u32,
            )
                -> Ptr<TESUseItemPackageData>
        ),
        entry!(0x0067c7d0, fn_0067c7d0(Ptr<TESUseItemPackageData>)),
        entry!(
            0x0067c860,
            tes_use_item_package_data_copy(Ptr<TESUseItemPackageData>, Ptr)
        ),
        entry!(
            0x0067c960,
            tes_use_item_package_data_compare(Ptr<TESUseItemPackageData>, Ptr) -> bool
        ),
        entry!(
            0x0067c9d0,
            tes_use_item_package_data_save(Ptr<TESUseItemPackageData>)
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
        e.register(PACKAGE_DATA_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e
    }

    fn use_item_data(e: &mut Engine) -> Ptr<TESUseItemPackageData> {
        e.new_object()
    }

    #[test]
    fn save_game_writes_the_flag_and_the_location() {
        let mut e = engine_with_casts();
        let data = use_item_data(&mut e);
        let buffer = Ptr::<()>::new(e.mem.alloc(8));
        let location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(data, TESUseItemPackageData::pSearchLocation, location);
        let flag = std::rc::Rc::new(std::cell::Cell::new(9u8));
        let sink = flag.clone();
        e.register_double(SAVE_RAW, move |e, a| {
            sink.set(e.mem.u8(a[1]));
            Ret::default()
        });
        e.register(PACKAGE_LOCATION_SAVE_GAME, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0067_b000, &args![data, buffer]);
        let log = e.call_log.take().unwrap();
        let raw = log.iter().find(|(addr, _)| *addr == SAVE_RAW).unwrap();
        assert_eq!(raw.1[0], buffer.addr());
        assert_eq!(raw.1[2..], [1, 0]);
        assert_eq!(flag.get(), 1);
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == PACKAGE_LOCATION_SAVE_GAME
                && a[..] == [location.addr(), buffer.addr()]));
    }

    #[test]
    fn save_game_without_a_location_writes_a_zero_flag_only() {
        let mut e = engine_with_casts();
        let data = use_item_data(&mut e);
        let buffer = Ptr::<()>::new(e.mem.alloc(8));
        let flag = std::rc::Rc::new(std::cell::Cell::new(9u8));
        let sink = flag.clone();
        e.register_double(SAVE_RAW, move |e, a| {
            sink.set(e.mem.u8(a[1]));
            Ret::default()
        });
        e.register(PACKAGE_LOCATION_SAVE_GAME, |_, _| panic!("no location"));
        e.call(0x0067_b000, &args![data, buffer]);
        assert_eq!(flag.get(), 0);
    }

    #[test]
    fn init_load_game_forwards_only_with_a_location() {
        let mut e = engine_with_casts();
        let data = use_item_data(&mut e);
        e.register(PACKAGE_LOCATION_INIT_LOAD_GAME, |_, _| {
            panic!("no location")
        });
        e.call(0x0067_bfa0, &args![data, 7u32]);
        let location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(data, TESUseItemPackageData::pSearchLocation, location);
        e.register(PACKAGE_LOCATION_INIT_LOAD_GAME, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0067_bfa0, &args![data, 7u32]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == PACKAGE_LOCATION_INIT_LOAD_GAME
                && a[..] == [location.addr(), 7]));
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_with_the_flag() {
        let mut e = engine_with_casts();
        let data = use_item_data(&mut e);
        let location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(data, TESUseItemPackageData::pSearchLocation, location);
        e.register(PACKAGE_LOCATION_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let back = e.call(0x0067_c7a0, &args![data, 0]).ptr::<()>();
        assert_eq!(back.addr(), data.addr());
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(addr, _)| *addr != OPERATOR_DELETE));
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == PACKAGE_LOCATION_DESTRUCT && a[..] == [location.addr(), 1]));
        assert_eq!(e.mem.u32(data.addr()), USE_ITEM_PACKAGE_DATA_VTABLE);

        e.call_log = Some(vec![]);
        e.call(0x0067_c7a0, &args![data, 1]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == OPERATOR_DELETE && a[..] == [data.addr()]));
    }

    #[test]
    fn destructor_body_without_a_location_only_runs_the_base() {
        let mut e = engine_with_casts();
        let data = use_item_data(&mut e);
        e.register(PACKAGE_LOCATION_DESTRUCT, |_, _| panic!("no location"));
        e.call_log = Some(vec![]);
        e.call(0x0067_c7d0, &args![data]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == PACKAGE_DATA_DESTRUCT && a[..] == [data.addr()]));
        assert_eq!(e.mem.u32(data.addr()), USE_ITEM_PACKAGE_DATA_VTABLE);
    }

    #[test]
    fn copy_replaces_the_location() {
        let mut e = engine_with_casts();
        let this = use_item_data(&mut e);
        let source = use_item_data(&mut e);
        let source_location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(
            source,
            TESUseItemPackageData::pSearchLocation,
            source_location,
        );
        let old_location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(this, TESUseItemPackageData::pSearchLocation, old_location);
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
        e.call(0x0067_c860, &args![this, source]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(
            |(addr, a)| *addr == PACKAGE_LOCATION_DESTRUCT && a[..] == [old_location.addr(), 1]
        ));
        assert!(log.iter().any(|(addr, a)| *addr == PACKAGE_LOCATION_COPY
            && a[..] == [new_block, source_location.addr(), 0]));
        assert_eq!(
            e.get(this, TESUseItemPackageData::pSearchLocation).addr(),
            new_block
        );
    }

    #[test]
    fn copy_without_a_source_location_leaves_none() {
        let mut e = engine_with_casts();
        let this = use_item_data(&mut e);
        let source = use_item_data(&mut e);
        let old_location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(this, TESUseItemPackageData::pSearchLocation, old_location);
        e.register(PACKAGE_LOCATION_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |_, _| panic!("source has no location"));
        e.call(0x0067_c860, &args![this, source]);
        assert!(e
            .get(this, TESUseItemPackageData::pSearchLocation)
            .is_null());
    }

    #[test]
    fn copy_of_something_that_is_not_use_item_data_does_nothing() {
        let mut e = engine_with_casts();
        let this = use_item_data(&mut e);
        let old_location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(this, TESUseItemPackageData::pSearchLocation, old_location);
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        e.register(PACKAGE_LOCATION_DESTRUCT, |_, _| panic!("untouched"));
        e.call(0x0067_c860, &args![this, 0x1000u32]);
        assert_eq!(
            e.get(this, TESUseItemPackageData::pSearchLocation).addr(),
            old_location.addr()
        );
    }

    #[test]
    fn compare_locations_by_presence_and_by_their_compare() {
        let mut e = engine_with_casts();
        let a = use_item_data(&mut e);
        let b = use_item_data(&mut e);
        assert!(!e.call(0x0067_c960, &args![a, b]).bool());
        let location_a = Ptr::<()>::new(e.mem.alloc(0x0c));
        let location_b = Ptr::<()>::new(e.mem.alloc(0x0c));
        // Only the other has a location.
        e.set(b, TESUseItemPackageData::pSearchLocation, location_b);
        assert!(e.call(0x0067_c960, &args![a, b]).bool());
        // Both have one: PackageLocation::Compare decides.
        e.set(a, TESUseItemPackageData::pSearchLocation, location_a);
        e.register(PACKAGE_LOCATION_COMPARE, |_, _| Ret::default());
        assert!(!e.call(0x0067_c960, &args![a, b]).bool());
        e.register(PACKAGE_LOCATION_COMPARE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        assert!(e.call(0x0067_c960, &args![a, b]).bool());
        // Only this one has a location: Compare is asked with a null other.
        e.set(b, TESUseItemPackageData::pSearchLocation, Ptr::NULL);
        e.register(PACKAGE_LOCATION_COMPARE, |_, a| {
            assert_eq!(a[1], 0);
            Ret::default()
        });
        assert!(!e.call(0x0067_c960, &args![a, b]).bool());
        // Not a use-item package data.
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        assert!(e.call(0x0067_c960, &args![a, b]).bool());
    }

    #[test]
    fn save_adds_the_chunk_and_the_location() {
        let mut e = engine_with_casts();
        let data = use_item_data(&mut e);
        e.register(ADD_CHUNK, |_, a| {
            assert_eq!(a[0], CHUNK_USE_ITEM_DATA);
            Ret::default()
        });
        e.register(PACKAGE_LOCATION_SAVE, |_, _| panic!("no location"));
        e.call(0x0067_c9d0, &args![data]);
        let location = Ptr::<()>::new(e.mem.alloc(0x0c));
        e.set(data, TESUseItemPackageData::pSearchLocation, location);
        e.register(PACKAGE_LOCATION_SAVE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0067_c9d0, &args![data]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(|(addr, a)| *addr == PACKAGE_LOCATION_SAVE
            && a[..] == [location.addr(), CHUNK_SEARCH_LOCATION]));
    }
}
