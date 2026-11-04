//! `fallout shared/bgsloadedreferencecollection.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The collection (`BGSLoadedReferenceCollection`, 0xC0 bytes) holds eight
//! `LoadedReferenceMap`s (hash maps from a key word to a `TESObjectREFR`
//! pointer, 0x18 bytes each, with a count and a time stamp) and is created
//! once, at `011c95c8`. References are sorted into the maps by the type of
//! their base form; an iterator (0x38 bytes, unnamed in the Xbox PDB; named
//! `LoadedReferenceIterator` here) walks them.
//!
//! The maps are built on `NiTMap<unsigned int, TESObjectREFR *>`
//! (`00527770`) and `NiTMapBase` (`005277d0`), whose constructors and
//! destructors are also in this unit.

#[allow(unused_imports)]
use crate::prelude::*;

/// Vtable of `LoadedReferenceMap` (slot 0 is its scalar deleting destructor).
const LOADED_REFERENCE_MAP_VTABLE: u32 = 0x0102_cf0c;
/// Vtable of `NiTMap<unsigned int, TESObjectREFR *>`.
const NI_T_MAP_VTABLE: u32 = 0x0102_cf80;
/// Vtable of `NiTMapBase<..., unsigned int, TESObjectREFR *>`.
const NI_T_MAP_BASE_VTABLE: u32 = 0x0102_cfa0;
/// The source file name the scope guards are given
/// (`...\Fallout Shared\BGSLoadedReferenceCollection.cpp`).
const SOURCE_FILE_NAME: u32 = 0x0102_cf28;
/// The global holding the one collection.
const COLLECTION_INSTANCE: u32 = 0x011c_95c8;
/// A global form pointer; a reference whose base form is this one goes into
/// the map at `+0x90` (named `RadiationMap` in the Xbox PDB).
const FORM_FOR_RADIATION_MAP: u32 = 0x011c_a264;
/// The object `00825c00` is called on to read the time stamp.
const TIME_STAMP_SOURCE: u32 = 0x011f_6394;

/// Hash table size of a `LoadedReferenceMap` (`0x25` = 37 buckets).
const LOADED_REFERENCE_MAP_BUCKETS: u32 = 0x25;
/// Size of the `BGSLoadedReferenceCollection` allocation.
const COLLECTION_SIZE: u32 = 0xc0;
/// Size of one `LoadedReferenceMap`.
const MAP_SIZE: u32 = 0x18;

/// `operator new` (`u32` size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete`.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Allocator used for the hash table (cdecl, size).
const TABLE_ALLOCATE: u32 = 0x00aa_1070;
/// Allocator used to free the hash table (cdecl, block).
const TABLE_FREE: u32 = 0x00aa_10f0;
/// `memset(block, value, size)` wrapper.
const MEMSET: u32 = 0x0040_3d30;
/// Scope guard constructor (`this`, new value, mode, file, line).
const GUARD_CONSTRUCT: u32 = 0x0040_4eb0;
/// Scope guard destructor.
const GUARD_DESTRUCT: u32 = 0x0040_4ee0;
/// Hash-map clear (called by every map destructor body).
const MAP_CLEAR: u32 = 0x0043_8af0;
/// The key of a reference (the word at `+0xC`).
const REFERENCE_KEY: u32 = 0x0084_e3a0;
/// `NiTMapBase::Lookup`-style find: (map, key, out value pointer) -> found.
const MAP_LOOKUP: u32 = 0x0085_3130;
/// `NiTMapBase::SetAt` (map, key, value).
const MAP_SET_AT: u32 = 0x0084_4700;
/// `NiTMapBase::RemoveAt` (map, key) -> removed.
const MAP_REMOVE_AT: u32 = 0x0040_5430;
/// The time stamp getter.
const GET_TIME_STAMP: u32 = 0x0082_5c00;
/// The word at `+0x20` of a reference, its form (`007af430`, named
/// `BGSSaveFormBuffer::GetForm` in the engine map).
const GET_BASE_FORM: u32 = 0x007a_f430;
/// The type byte of a form (`+4`).
const GET_FORM_TYPE: u32 = 0x0040_1170;
/// `TESContainer::ContainerCanHoldType` (Xbox PDB), cdecl, type byte.
const CONTAINER_CAN_HOLD_TYPE: u32 = 0x0048_1f30;
/// The extra data list of a reference.
const GET_EXTRA_DATA_LIST: u32 = 0x005d_43c0;
/// `BaseExtraList::GetExtraData` (Xbox PDB): (list, extra type byte).
const GET_EXTRA_DATA: u32 = 0x0041_0220;
/// `ExtraDataList::GetContainerChanges` (Xbox PDB).
const GET_CONTAINER_CHANGES: u32 = 0x0041_8520;
/// A predicate called with the reference only (unnamed in the engine map).
const REFERENCE_PREDICATE_00568E50: u32 = 0x0056_8e50;
/// `ItemChange::ItemChange_ov3` (Xbox PDB).
const ITEM_CHANGE_CONSTRUCT: u32 = 0x0076_b630;
/// The inventory change cleanup `005272b0` runs after clearing the first word.
const ITEM_CHANGE_RELEASE: u32 = 0x004b_c5f0;
/// Steps an inventory-changes walk: (changes, item change, out value, 1).
const NEXT_INVENTORY_ITEM: u32 = 0x004d_4830;
/// The word at `+8` of an inventory entry (compared with the base form).
const ENTRY_WORD_8: u32 = 0x0044_ddc0;
/// The word at `+4` of an inventory entry (must be non-zero).
const ENTRY_WORD_4: u32 = 0x0072_6070;
/// `+0x18` of the object it is called on.
const OFFSET_0X18: u32 = 0x0050_0940;
/// First position of a hash map.
const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
/// Next map item (map, position pointer, out key, out value).
const MAP_NEXT_ITEM: u32 = 0x006b_7f20;
/// Reference flag tests (`+8` bit 0x800, 0x20 and 0x2000000).
const REFERENCE_FLAG_800: u32 = 0x0044_0da0;
const REFERENCE_FLAG_20: u32 = 0x0044_0d80;
const REFERENCE_FLAG_2000000: u32 = 0x0040_1210;
/// `TESObjectCONT::CanActivate` (Xbox PDB).
const CONTAINER_CAN_ACTIVATE: u32 = 0x0051_6cd0;

/// Slot `+0x160` of a reference's vtable: a predicate without arguments.
const REFERENCE_SLOT_160: u32 = 0x160;
/// Slot `+0x94` of a form's vtable: a predicate without arguments.
const FORM_SLOT_94: u32 = 0x94;
/// Slot `+0x22C` of a reference's vtable: a predicate with one argument.
const REFERENCE_SLOT_22C: u32 = 0x22c;

/// The extra data type `00527080` is asked for when the form type is `0x51`.
const EXTRA_TYPE_FOR_FORM_51: u8 = 0x5d;

layout! {
    /// `LoadedReferenceMap` (Xbox PDB), 0x18 bytes: an
    /// `NiTMap<unsigned int, TESObjectREFR *>` (vtable, hash size, table,
    /// allocator count) and two words.
    pub struct LoadedReferenceMap: 0x18 {
        /// `m_uiHashSize` (Xbox PDB): number of buckets.
        0x04 m_uiHashSize: u32,
        /// `m_ppkHashTable` (Xbox PDB).
        0x08 m_ppkHashTable: u32,
        /// `m_kAllocator` (Xbox PDB): the allocator's word.
        0x0C m_kAllocator: u32,
        /// `iCount` (Xbox PDB).
        0x10 iCount: u32,
        /// `iLastChangedTimeStamp` (Xbox PDB).
        0x14 iLastChangedTimeStamp: u32,
    }

    /// `BGSLoadedReferenceCollection` (Xbox PDB), 0xC0 bytes: eight maps.
    pub struct BGSLoadedReferenceCollection: 0xC0 {
        /// `DynamicReferenceMap` (Xbox PDB).
        0x00 DynamicReferenceMap: Inline<LoadedReferenceMap>,
        /// `ItemsMap` (Xbox PDB).
        0x18 ItemsMap: Inline<LoadedReferenceMap>,
        /// `WeaponsMap` (Xbox PDB).
        0x30 WeaponsMap: Inline<LoadedReferenceMap>,
        /// `AmmoMap` (Xbox PDB).
        0x48 AmmoMap: Inline<LoadedReferenceMap>,
        /// `ContainersMap` (Xbox PDB).
        0x60 ContainersMap: Inline<LoadedReferenceMap>,
        /// `ActorsMap` (Xbox PDB).
        0x78 ActorsMap: Inline<LoadedReferenceMap>,
        /// `RadiationMap` (Xbox PDB).
        0x90 RadiationMap: Inline<LoadedReferenceMap>,
        /// `TeleportDoorMap` (Xbox PDB).
        0xA8 TeleportDoorMap: Inline<LoadedReferenceMap>,
    }

    /// The collection's iterator (0x38 bytes; unnamed in the Xbox PDB, the
    /// name and the field names are ours, taken from how the code uses them).
    pub struct LoadedReferenceIterator: 0x38 {
        /// The form type searched for.
        0x00 form_type: u32,
        /// The base form searched for (zero to search by type).
        0x04 base_form: Ptr,
        /// The third argument handed to `TESObjectCONT::CanActivate`.
        0x08 activate_argument: u32,
        /// The map being walked.
        0x0C current_map: Ptr,
        /// The reference whose inventory is being walked (the container).
        0x10 current_container: Ptr,
        /// The container changes of `current_container`.
        0x14 container_changes: Ptr,
        /// The position in `current_map`.
        0x18 position: Ptr,
        /// The embedded `ItemChange` (0x18 bytes) used to walk an inventory.
        0x1C item_change: Inline<()>,
        /// Which of the collection's maps are being walked (0 = the
        /// containers map, 1 = the actors map, 2 = done).
        0x34 map_stage: u8,
    }
}

// Translated from 00526b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadedReferenceMap` constructor (unnamed in the engine map): the
/// `NiTMap` constructor with 37 buckets (`00527770`), the vtable `0102cf0c`,
/// count and time stamp zero. Returns `this`.
pub fn fn_00526b00(e: &mut Engine, this: Ptr<LoadedReferenceMap>) -> Ptr<LoadedReferenceMap> {
    fn_00527770(e, this.cast(), LOADED_REFERENCE_MAP_BUCKETS);
    e.mem.set_u32(this.addr(), LOADED_REFERENCE_MAP_VTABLE);
    e.set(this, LoadedReferenceMap::iCount, 0);
    e.set(this, LoadedReferenceMap::iLastChangedTimeStamp, 0);
    this
}

// Translated from 00526b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadedReferenceMap::_scalar_deleting_destructor_` (Xbox PDB name): the
/// destructor body (`00526b70`), then the free (`00401030`) when bit 0 of
/// `flags` is set. Returns `this`.
pub fn loaded_reference_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<LoadedReferenceMap>,
    flags: u32,
) -> Ptr<LoadedReferenceMap> {
    fn_00526b70(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00526b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LoadedReferenceMap` destructor body (unnamed in the engine map): resets
/// the vtable to `0102cf0c`, clears the map (`00438af0`) and runs the
/// `NiTMap` destructor body (`00527840`).
// Translation incomplete: 00526b70 - the exception-unwinding frame (FS:[0] handler) is not modelled; the ordinary behaviour is complete
pub fn fn_00526b70(e: &mut Engine, this: Ptr<LoadedReferenceMap>) {
    e.mem.set_u32(this.addr(), LOADED_REFERENCE_MAP_VTABLE);
    e.call(MAP_CLEAR, &args![this]);
    fn_00527840(e, this.cast());
}

// Translated from 00526bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a reference to a `LoadedReferenceMap` (unnamed in the engine map):
/// looks its key (`0084e3a0`) up and, when it is not there yet, stores it
/// (`SetAt`, `00844700`), counts it and takes a new time stamp. Returns
/// whether it was added.
pub fn fn_00526bd0(e: &mut Engine, this: Ptr<LoadedReferenceMap>, reference: Ptr) -> bool {
    e.with_stack(4, |e, found| {
        e.mem.set_u32(found.addr(), 0);
        let key = e.call(REFERENCE_KEY, &args![reference]).u32();
        if e.call(MAP_LOOKUP, &args![this, key, found]).bool() {
            return false;
        }
        let key = e.call(REFERENCE_KEY, &args![reference]).u32();
        e.call(MAP_SET_AT, &args![this, key, reference]);
        let count = e.get(this, LoadedReferenceMap::iCount);
        e.set(this, LoadedReferenceMap::iCount, count.wrapping_add(1));
        let stamp = e.call(GET_TIME_STAMP, &args![TIME_STAMP_SOURCE]).u32();
        e.set(this, LoadedReferenceMap::iLastChangedTimeStamp, stamp);
        true
    })
}

// Translated from 00526c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes a reference from a `LoadedReferenceMap` (unnamed in the engine
/// map): `RemoveAt` (`00405430`) by its key; when something was removed the
/// count goes down and the time stamp is renewed. Returns whether it was
/// removed.
pub fn fn_00526c40(e: &mut Engine, this: Ptr<LoadedReferenceMap>, reference: Ptr) -> bool {
    let key = e.call(REFERENCE_KEY, &args![reference]).u32();
    if !e.call(MAP_REMOVE_AT, &args![this, key]).bool() {
        return false;
    }
    let count = e.get(this, LoadedReferenceMap::iCount);
    e.set(this, LoadedReferenceMap::iCount, count.wrapping_sub(1));
    let stamp = e.call(GET_TIME_STAMP, &args![TIME_STAMP_SOURCE]).u32();
    e.set(this, LoadedReferenceMap::iLastChangedTimeStamp, stamp);
    true
}

// Translated from 00526c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSLoadedReferenceCollection` constructor (unnamed in the engine map):
/// constructs its eight maps in order. Returns `this`.
// Translation incomplete: 00526c90 - the exception-unwinding frame (FS:[0] handler) is not modelled; the ordinary behaviour is complete
pub fn fn_00526c90(
    e: &mut Engine,
    this: Ptr<BGSLoadedReferenceCollection>,
) -> Ptr<BGSLoadedReferenceCollection> {
    for index in 0..8 {
        let map = this.cast::<LoadedReferenceMap>().index(index, MAP_SIZE);
        fn_00526b00(e, map);
    }
    this
}

// Translated from 00526d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSLoadedReferenceCollection` destructor body (unnamed in the engine
/// map): destroys the eight maps from the last to the first.
// Translation incomplete: 00526d50 - the exception-unwinding frame (FS:[0] handler) is not modelled; the ordinary behaviour is complete
pub fn fn_00526d50(e: &mut Engine, this: Ptr<BGSLoadedReferenceCollection>) {
    for index in (0..8).rev() {
        let map = this.cast::<LoadedReferenceMap>().index(index, MAP_SIZE);
        fn_00526b70(e, map);
    }
}

// Translated from 00526e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the one `BGSLoadedReferenceCollection` (unnamed in the engine
/// map): under a scope guard (`00404eb0`: 0x2c, 1, the source file name,
/// line 0x58) allocates 0xC0 bytes, constructs the collection when the
/// allocation succeeded, and stores the result at `011c95c8`.
// Translation incomplete: 00526e10 - the exception-unwinding frame (FS:[0] handler) is not modelled; the ordinary behaviour is complete
pub fn fn_00526e10(e: &mut Engine) {
    e.with_stack(4, |e, guard| {
        e.call(
            GUARD_CONSTRUCT,
            &args![guard, 0x2cu32, true, SOURCE_FILE_NAME, 0x58u32],
        );
        let block = e.call(OPERATOR_NEW, &args![COLLECTION_SIZE]).ptr::<()>();
        let collection = if block.is_null() {
            Ptr::NULL
        } else {
            fn_00526c90(e, block.cast())
        };
        e.mem.set_u32(COLLECTION_INSTANCE, collection.addr());
        e.call(GUARD_DESTRUCT, &args![guard]);
    });
}

// Translated from 00526eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the one collection (unnamed in the engine map): its scalar
/// deleting destructor with the free flag when it exists, then the global is
/// cleared.
pub fn fn_00526eb0(e: &mut Engine) {
    let collection = e.mem.u32(COLLECTION_INSTANCE);
    if collection != 0 {
        fn_00526ef0(e, Ptr::new(collection), 1);
    }
    e.mem.set_u32(COLLECTION_INSTANCE, 0);
}

// Translated from 00526ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSLoadedReferenceCollection`'s scalar deleting destructor (unnamed in
/// the engine map): the destructor body (`00526d50`), then the free when bit
/// 0 of `flags` is set. Returns `this`.
pub fn fn_00526ef0(
    e: &mut Engine,
    this: Ptr<BGSLoadedReferenceCollection>,
    flags: u32,
) -> Ptr<BGSLoadedReferenceCollection> {
    fn_00526d50(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00526f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sorts a reference into the collection's maps by the type of its base
/// form (unnamed in the engine map), under a scope guard (`00404eb0`: 0x2c,
/// 1, the source file name, line 0x68):
/// - when `ContainerCanHoldType` accepts the type: the items map (`+0x18`)
///   and the map `005271b0` picks for the type;
/// - type `0x1b`: the containers map (`+0x60`);
/// - type `0x2a` or `0x2b`: the actors map (`+0x78`);
/// - the base form being the global form at `011ca264`: the map at `+0x90`;
/// - type `0x51` with extra data `0x5d` present: the map at `+0x90`;
/// - type `0x1c` with `00568e50` non-zero: the map at `+0xa8`.
// Translation incomplete: 00526f20 - the exception-unwinding frame (FS:[0] handler) is not modelled; the ordinary behaviour is complete
pub fn fn_00526f20(e: &mut Engine, this: Ptr<BGSLoadedReferenceCollection>, reference: Ptr) {
    e.with_stack(4, |e, guard| {
        e.call(
            GUARD_CONSTRUCT,
            &args![guard, 0x2cu32, true, SOURCE_FILE_NAME, 0x68u32],
        );
        let form = e.call(GET_BASE_FORM, &args![reference]).u32();
        let form_type = e.call(GET_FORM_TYPE, &args![form]).u32();
        if e.call(CONTAINER_CAN_HOLD_TYPE, &args![form_type & 0xff])
            .bool()
        {
            fn_00526bd0(
                e,
                this.at(BGSLoadedReferenceCollection::ItemsMap),
                reference,
            );
            let map = fn_005271b0(e, this, form_type);
            if !map.is_null() {
                fn_00526bd0(e, map, reference);
            }
        }
        if form_type == 0x1b {
            fn_00526bd0(
                e,
                this.at(BGSLoadedReferenceCollection::ContainersMap),
                reference,
            );
        }
        if form_type == 0x2a || form_type == 0x2b {
            fn_00526bd0(
                e,
                this.at(BGSLoadedReferenceCollection::ActorsMap),
                reference,
            );
        }
        let form = e.call(GET_BASE_FORM, &args![reference]).u32();
        if form == e.mem.u32(FORM_FOR_RADIATION_MAP) {
            fn_00526bd0(
                e,
                this.at(BGSLoadedReferenceCollection::RadiationMap),
                reference,
            );
        }
        if form_type == 0x51 && fn_00527080(e, reference, EXTRA_TYPE_FOR_FORM_51) != 0 {
            fn_00526bd0(
                e,
                this.at(BGSLoadedReferenceCollection::RadiationMap),
                reference,
            );
        }
        if form_type == 0x1c
            && e.call(REFERENCE_PREDICATE_00568E50, &args![reference])
                .u32()
                != 0
        {
            fn_00526bd0(
                e,
                this.at(BGSLoadedReferenceCollection::TeleportDoorMap),
                reference,
            );
        }
        e.call(GUARD_DESTRUCT, &args![guard]);
    });
}

// Translated from 00527080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks up the extra data of the given type on a reference (unnamed in the
/// engine map): `005d43c0` gives its extra data list, then
/// `BaseExtraList::GetExtraData` (`00410220`) finds the entry. Returns what
/// that returns.
pub fn fn_00527080(e: &mut Engine, this: Ptr, extra_type: u8) -> u32 {
    let list = e.call(GET_EXTRA_DATA_LIST, &args![this]).u32();
    e.call(GET_EXTRA_DATA, &args![list, extra_type]).u32()
}

// Translated from 005270b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reverse of `00526f20` (unnamed in the engine map): removes a
/// reference from the same maps (`00526c40`). Unlike the add, the type `0x51`
/// case removes from the map at `+0x90` without asking for the extra data.
pub fn fn_005270b0(e: &mut Engine, this: Ptr<BGSLoadedReferenceCollection>, reference: Ptr) {
    let form = e.call(GET_BASE_FORM, &args![reference]).u32();
    let form_type = e.call(GET_FORM_TYPE, &args![form]).u32();
    if e.call(CONTAINER_CAN_HOLD_TYPE, &args![form_type & 0xff])
        .bool()
    {
        fn_00526c40(
            e,
            this.at(BGSLoadedReferenceCollection::ItemsMap),
            reference,
        );
        let map = fn_005271b0(e, this, form_type);
        if !map.is_null() {
            fn_00526c40(e, map, reference);
        }
    }
    if form_type == 0x1b {
        fn_00526c40(
            e,
            this.at(BGSLoadedReferenceCollection::ContainersMap),
            reference,
        );
    }
    if form_type == 0x2a || form_type == 0x2b {
        fn_00526c40(
            e,
            this.at(BGSLoadedReferenceCollection::ActorsMap),
            reference,
        );
    }
    let form = e.call(GET_BASE_FORM, &args![reference]).u32();
    if form == e.mem.u32(FORM_FOR_RADIATION_MAP) {
        fn_00526c40(
            e,
            this.at(BGSLoadedReferenceCollection::RadiationMap),
            reference,
        );
    }
    if form_type == 0x51 {
        fn_00526c40(
            e,
            this.at(BGSLoadedReferenceCollection::RadiationMap),
            reference,
        );
    }
    if form_type == 0x1c
        && e.call(REFERENCE_PREDICATE_00568E50, &args![reference])
            .u32()
            != 0
    {
        fn_00526c40(
            e,
            this.at(BGSLoadedReferenceCollection::TeleportDoorMap),
            reference,
        );
    }
}

// Translated from 005271b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The collection's map for a form type (unnamed in the engine map): the
/// weapons map (`+0x30`) for type `0x28`, the ammo map (`+0x48`) for `0x29`,
/// else null.
pub fn fn_005271b0(
    _e: &mut Engine,
    this: Ptr<BGSLoadedReferenceCollection>,
    form_type: u32,
) -> Ptr<LoadedReferenceMap> {
    match form_type {
        0x28 => this.at(BGSLoadedReferenceCollection::WeaponsMap),
        0x29 => this.at(BGSLoadedReferenceCollection::AmmoMap),
        _ => Ptr::NULL,
    }
}

// Translated from 005271f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Iterator constructor searching by form type (unnamed in the engine map):
/// stores the type, a null base form and the activate argument, constructs
/// the embedded `ItemChange` (`00527270`) and resets the walk (`00527350`).
/// Returns `this`.
// Translation incomplete: 005271f0 - the exception-unwinding frame (FS:[0] handler) is not modelled; the ordinary behaviour is complete
pub fn fn_005271f0(
    e: &mut Engine,
    this: Ptr<LoadedReferenceIterator>,
    form_type: u32,
    activate_argument: u32,
) -> Ptr<LoadedReferenceIterator> {
    e.set(this, LoadedReferenceIterator::form_type, form_type);
    e.set(this, LoadedReferenceIterator::base_form, Ptr::NULL);
    e.set(
        this,
        LoadedReferenceIterator::activate_argument,
        activate_argument,
    );
    fn_00527270(e, this.at(LoadedReferenceIterator::item_change));
    fn_00527350(e, this);
    this
}

// Translated from 00527270 (decompiled, FalloutNV.exe 1.4.0.525)
/// The embedded `ItemChange` initializer (unnamed in the engine map):
/// `ItemChange::ItemChange_ov3` (`0076b630`), then the words at `+0xC` and
/// `+0x10` and the byte at `+0x14` zero. Returns `this`.
pub fn fn_00527270(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(ITEM_CHANGE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    e.mem.set_u8(this.addr() + 0x14, 0);
    this
}

// Translated from 005272b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The embedded `ItemChange` cleanup (unnamed in the engine map): clears the
/// first word, then runs `004bc5f0`.
pub fn fn_005272b0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0);
    e.call(ITEM_CHANGE_RELEASE, &args![this]);
}

// Translated from 005272d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Iterator constructor searching by base form (unnamed in the engine map):
/// the type is the type of `base_form` (`00401170`), the base form and the
/// activate argument are stored, the embedded `ItemChange` is constructed
/// and the walk reset. Returns `this`.
// Translation incomplete: 005272d0 - the exception-unwinding frame (FS:[0] handler) is not modelled; the ordinary behaviour is complete
pub fn fn_005272d0(
    e: &mut Engine,
    this: Ptr<LoadedReferenceIterator>,
    base_form: Ptr,
    activate_argument: u32,
) -> Ptr<LoadedReferenceIterator> {
    let form_type = e.call(GET_FORM_TYPE, &args![base_form]).u32();
    e.set(this, LoadedReferenceIterator::form_type, form_type);
    e.set(this, LoadedReferenceIterator::base_form, base_form);
    e.set(
        this,
        LoadedReferenceIterator::activate_argument,
        activate_argument,
    );
    fn_00527270(e, this.at(LoadedReferenceIterator::item_change));
    fn_00527350(e, this);
    this
}

// Translated from 00527350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets an iterator's walk (unnamed in the engine map): no container, no
/// changes, stage zero; the map is the collection's map for the type
/// (`005271b0` on the global collection) or, when there is none, the object
/// `00500940` returns for the collection; the position is the map's first.
pub fn fn_00527350(e: &mut Engine, this: Ptr<LoadedReferenceIterator>) {
    e.set(this, LoadedReferenceIterator::current_container, Ptr::NULL);
    e.set(this, LoadedReferenceIterator::container_changes, Ptr::NULL);
    e.set(this, LoadedReferenceIterator::map_stage, 0);
    let collection = Ptr::<BGSLoadedReferenceCollection>::new(e.mem.u32(COLLECTION_INSTANCE));
    let form_type = e.get(this, LoadedReferenceIterator::form_type);
    let mut map = fn_005271b0(e, collection, form_type).cast::<()>();
    e.set(this, LoadedReferenceIterator::current_map, map);
    if map.is_null() {
        map = e.call(OFFSET_0X18, &args![collection]).ptr();
        e.set(this, LoadedReferenceIterator::current_map, map);
    }
    let position = e.call(MAP_FIRST_POSITION, &args![map]).ptr();
    e.set(this, LoadedReferenceIterator::position, position);
}

// Translated from 005273c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Steps a loaded-reference iterator (unnamed in the engine map): finds the
/// next matching reference and stores it in `out_reference` (and, for an
/// item found in a container's inventory, the inventory entry in `out_item`;
/// otherwise `out_item` is zero). Returns whether one was found.
///
/// First the container being walked (`current_container`) is finished: its
/// inventory entries (`004d4830`) are compared with the base form searched
/// for, or the form type. Then the maps are walked: stage 0 moves to the
/// containers map when `flags` has bit 0, stage 1 to the actors map when
/// `flags & 6` is set, stage 2 ends the walk. Each reference is skipped when
/// flag tests (`00440da0`, `00440d80`, vtable slot `+0x160` of the reference,
/// slot `+0x94` of its base form, `00401210` unless `flags & 8`) say so.
/// Before the first stage change the base form or type is matched directly;
/// afterwards a reference found in the actors map is accepted by
/// `flags & 2` against `flags & 4` and its slot `+0x22c`, and a container
/// (type `0x1b`) by `TESObjectCONT::CanActivate`, and then its inventory
/// becomes the container being walked.
pub fn fn_005273c0(
    e: &mut Engine,
    this: Ptr<BGSLoadedReferenceCollection>,
    iterator: Ptr<LoadedReferenceIterator>,
    out_reference: Ptr,
    out_item: Ptr,
    flags: u32,
) -> bool {
    e.with_stack(12, |e, scratch| {
        next_loaded_reference(e, this, iterator, out_reference, out_item, flags, scratch)
    })
}

/// The body of `005273c0`; `scratch` holds its three stack locals: the
/// inventory entry (`+0`), the map key (`+4`) and the map value (`+8`).
fn next_loaded_reference(
    e: &mut Engine,
    this: Ptr<BGSLoadedReferenceCollection>,
    iter: Ptr<LoadedReferenceIterator>,
    out_reference: Ptr,
    out_item: Ptr,
    flags: u32,
    scratch: Ptr,
) -> bool {
    use LoadedReferenceIterator as It;
    let entry_slot = scratch;
    let key_slot = scratch.byte_add(4);
    let value_slot = scratch.byte_add(8);
    let item_change = iter.at(It::item_change);
    let containers_map = this
        .at(BGSLoadedReferenceCollection::ContainersMap)
        .cast::<()>();
    let actors_map = this
        .at(BGSLoadedReferenceCollection::ActorsMap)
        .cast::<()>();
    let items_map = this.at(BGSLoadedReferenceCollection::ItemsMap).cast::<()>();

    'restart: loop {
        if e.get(iter, It::current_map).is_null() {
            return false;
        }
        if !e.get(iter, It::current_container).is_null() {
            e.mem.set_u32(entry_slot.addr(), 0);
            loop {
                let changes = e.get(iter, It::container_changes);
                let more = e
                    .call(
                        NEXT_INVENTORY_ITEM,
                        &args![changes, item_change, entry_slot, 1u32],
                    )
                    .bool();
                if !more {
                    fn_00527740(e, item_change);
                    e.set(iter, It::container_changes, Ptr::NULL);
                    e.set(iter, It::current_container, Ptr::NULL);
                    break;
                }
                let entry = e.mem.u32(entry_slot.addr());
                let base_form = e.get(iter, It::base_form);
                let matches = if !base_form.is_null() {
                    e.call(ENTRY_WORD_8, &args![entry]).u32() == base_form.addr()
                } else {
                    let form = e.call(ENTRY_WORD_8, &args![entry]).u32();
                    e.call(GET_FORM_TYPE, &args![form]).u32() == e.get(iter, It::form_type)
                };
                if matches && e.call(ENTRY_WORD_4, &args![entry]).u32() != 0 {
                    let container = e.get(iter, It::current_container);
                    e.mem.set_u32(out_reference.addr(), container.addr());
                    e.mem.set_u32(out_item.addr(), entry);
                    return true;
                }
            }
        }
        while e.get(iter, It::position).is_null() {
            match e.get(iter, It::map_stage) {
                0 => {
                    if flags & 1 != 0 {
                        e.set(iter, It::current_map, containers_map);
                    }
                }
                1 => {
                    if flags & 6 != 0 {
                        e.set(iter, It::current_map, actors_map);
                    }
                }
                2 => {
                    e.set(iter, It::current_map, Ptr::NULL);
                    return false;
                }
                _ => {}
            }
            let stage = e.get(iter, It::map_stage);
            e.set(iter, It::map_stage, stage.wrapping_add(1));
            let map = e.get(iter, It::current_map);
            if !map.is_null() {
                let position = e.call(MAP_FIRST_POSITION, &args![map]).ptr();
                e.set(iter, It::position, position);
            }
        }
        let past_first_stage = e.get(iter, It::map_stage) != 0;
        let not_items_map = e.get(iter, It::current_map) != items_map;

        loop {
            if e.get(iter, It::position).is_null() {
                continue 'restart;
            }
            e.mem.set_u32(key_slot.addr(), 0);
            e.mem.set_u32(value_slot.addr(), 0);
            let map = e.get(iter, It::current_map);
            e.call(
                MAP_NEXT_ITEM,
                &args![map, iter.byte_add(0x18), key_slot, value_slot],
            );
            let reference = e.mem.u32(value_slot.addr());
            if e.call(REFERENCE_FLAG_800, &args![reference]).bool()
                || e.call(REFERENCE_FLAG_20, &args![reference]).bool()
                || e.vcall(reference, REFERENCE_SLOT_160, &[]).bool()
            {
                continue;
            }
            let form = e.call(GET_BASE_FORM, &args![reference]).u32();
            if e.vcall(form, FORM_SLOT_94, &[]).bool() {
                continue;
            }
            if e.call(REFERENCE_FLAG_2000000, &args![reference]).bool() && flags & 8 == 0 {
                continue;
            }
            if !past_first_stage {
                let base_form = e.get(iter, It::base_form);
                if !base_form.is_null() {
                    let form = e.call(GET_BASE_FORM, &args![reference]).u32();
                    if base_form.addr() != form {
                        continue;
                    }
                } else if !not_items_map {
                    let form = e.call(GET_BASE_FORM, &args![reference]).u32();
                    let form_type = e.call(GET_FORM_TYPE, &args![form]).u32();
                    if e.get(iter, It::form_type) != form_type {
                        continue;
                    }
                }
                e.mem.set_u32(out_reference.addr(), reference);
                e.mem.set_u32(out_item.addr(), 0);
                return true;
            }
            if e.get(iter, It::current_map) == actors_map {
                let wants_a = flags & 2 != 0;
                let wants_b = flags & 4 != 0;
                if wants_a != wants_b {
                    let answer = e.vcall(reference, REFERENCE_SLOT_22C, &args![0u32]).u8();
                    if u32::from(answer) == u32::from(!wants_a) {
                        continue;
                    }
                }
            } else {
                let form = e.call(GET_BASE_FORM, &args![reference]).u32();
                let form_type = e.call(GET_FORM_TYPE, &args![form]).u32();
                if form_type == 0x1b {
                    let form = e.call(GET_BASE_FORM, &args![reference]).u32();
                    let activate_argument = e.get(iter, It::activate_argument);
                    if !e
                        .call(
                            CONTAINER_CAN_ACTIVATE,
                            &args![form, reference, activate_argument, 0u32],
                        )
                        .bool()
                    {
                        continue;
                    }
                }
            }
            let list = e.call(GET_EXTRA_DATA_LIST, &args![reference]).u32();
            let changes = e.call(GET_CONTAINER_CHANGES, &args![list]).ptr();
            e.set(iter, It::container_changes, changes);
            if !changes.is_null() {
                e.set(iter, It::current_container, Ptr::new(reference));
            }
            continue 'restart;
        }
    }
}

// Translated from 00527740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the words at `+0xC` and `+0x10` and the byte at `+0x14` of the
/// embedded `ItemChange` (unnamed in the engine map).
pub fn fn_00527740(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0xc, 0);
    e.mem.set_u32(this.addr() + 0x10, 0);
    e.mem.set_u8(this.addr() + 0x14, 0);
}

// Translated from 00527770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned int, TESObjectREFR *>` constructor (unnamed in the
/// engine map): the `NiTMapBase` constructor (`005277d0`) with `hash_size`
/// buckets, then the vtable `0102cf80`. Returns `this`.
pub fn fn_00527770(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    fn_005277d0(e, this, hash_size);
    e.mem.set_u32(this.addr(), NI_T_MAP_VTABLE);
    this
}

// Translated from 005277a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned int, TESObjectREFR *>::_scalar_deleting_destructor_`
/// (Xbox PDB name): the destructor body (`00527840`), then the free when bit
/// 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_reference_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00527840(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 005277d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., unsigned int, TESObjectREFR *>` constructor (unnamed in
/// the engine map): the vtable `0102cfa0`, the bucket count, a zero count,
/// and a hash table of `hash_size` words, allocated (`00aa1070`) and
/// cleared (`00403d30`). Returns `this`.
pub fn fn_005277d0(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NI_T_MAP_BASE_VTABLE);
    e.mem.set_u32(this.addr() + 4, hash_size);
    e.mem.set_u32(this.addr() + 0xc, 0);
    let table = e
        .call(TABLE_ALLOCATE, &args![hash_size.wrapping_shl(2)])
        .u32();
    e.mem.set_u32(this.addr() + 8, table);
    let buckets = e.mem.u32(this.addr() + 4);
    e.call(MEMSET, &args![table, 0u32, buckets.wrapping_shl(2)]);
    this
}

// Translated from 00527840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned int, TESObjectREFR *>` destructor body (unnamed in the
/// engine map; the decompiler's library name is wrong): sets the vtable
/// `0102cf80`, clears the map (`00438af0`) and runs the `NiTMapBase`
/// destructor body (`005278a0`).
// Translation incomplete: 00527840 - the exception-unwinding frame (FS:[0] handler) is not modelled; the ordinary behaviour is complete
pub fn fn_00527840(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_T_MAP_VTABLE);
    e.call(MAP_CLEAR, &args![this]);
    fn_005278a0(e, this);
}

// Translated from 005278a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., unsigned int, TESObjectREFR *>` destructor body (unnamed
/// in the engine map): sets the vtable `0102cfa0`, clears the map
/// (`00438af0`) and frees the hash table (`00aa10f0`).
pub fn fn_005278a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), NI_T_MAP_BASE_VTABLE);
    e.call(MAP_CLEAR, &args![this]);
    let table = e.mem.u32(this.addr() + 8);
    e.call(TABLE_FREE, &args![table]);
}

// Translated from 005278d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<unsigned int, TESObjectREFR *>>, unsigned
/// int, TESObjectREFR *>::_scalar_deleting_destructor_` (Xbox PDB name): the
/// destructor body (`005278a0`), then the free when bit 0 of `flags` is set.
/// Returns `this`.
pub fn ni_t_map_base_reference_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_005278a0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00526b00,
            fn_00526b00(Ptr<LoadedReferenceMap>) -> Ptr<LoadedReferenceMap>
        ),
        entry!(
            0x00526b40,
            loaded_reference_map_scalar_deleting_destructor(
                Ptr<LoadedReferenceMap>,
                u32,
            ) -> Ptr<LoadedReferenceMap>
        ),
        entry!(0x00526b70, fn_00526b70(Ptr<LoadedReferenceMap>)),
        entry!(
            0x00526bd0,
            fn_00526bd0(Ptr<LoadedReferenceMap>, Ptr) -> bool
        ),
        entry!(
            0x00526c40,
            fn_00526c40(Ptr<LoadedReferenceMap>, Ptr) -> bool
        ),
        entry!(
            0x00526c90,
            fn_00526c90(Ptr<BGSLoadedReferenceCollection>) -> Ptr<BGSLoadedReferenceCollection>
        ),
        entry!(0x00526d50, fn_00526d50(Ptr<BGSLoadedReferenceCollection>)),
        entry!(0x00526e10, fn_00526e10()),
        entry!(0x00526eb0, fn_00526eb0()),
        entry!(
            0x00526ef0,
            fn_00526ef0(
                Ptr<BGSLoadedReferenceCollection>,
                u32,
            ) -> Ptr<BGSLoadedReferenceCollection>
        ),
        entry!(
            0x00526f20,
            fn_00526f20(Ptr<BGSLoadedReferenceCollection>, Ptr)
        ),
        entry!(0x00527080, fn_00527080(Ptr, u8) -> u32),
        entry!(
            0x005270b0,
            fn_005270b0(Ptr<BGSLoadedReferenceCollection>, Ptr)
        ),
        entry!(
            0x005271b0,
            fn_005271b0(Ptr<BGSLoadedReferenceCollection>, u32) -> Ptr<LoadedReferenceMap>
        ),
        entry!(
            0x005271f0,
            fn_005271f0(Ptr<LoadedReferenceIterator>, u32, u32) -> Ptr<LoadedReferenceIterator>
        ),
        entry!(0x00527270, fn_00527270(Ptr) -> Ptr),
        entry!(0x005272b0, fn_005272b0(Ptr)),
        entry!(
            0x005272d0,
            fn_005272d0(Ptr<LoadedReferenceIterator>, Ptr, u32) -> Ptr<LoadedReferenceIterator>
        ),
        entry!(0x00527350, fn_00527350(Ptr<LoadedReferenceIterator>)),
        entry!(
            0x005273c0,
            fn_005273c0(
                Ptr<BGSLoadedReferenceCollection>,
                Ptr<LoadedReferenceIterator>,
                Ptr,
                Ptr,
                u32,
            ) -> bool
        ),
        entry!(0x00527740, fn_00527740(Ptr)),
        entry!(0x00527770, fn_00527770(Ptr, u32) -> Ptr),
        entry!(0x005277a0, ni_t_map_reference_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x005277d0, fn_005277d0(Ptr, u32) -> Ptr),
        entry!(0x00527840, fn_00527840(Ptr)),
        entry!(0x005278a0, fn_005278a0(Ptr)),
        entry!(0x005278d0, ni_t_map_base_reference_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    /// The vtable shared by the test references and forms.
    const TEST_VTABLE: u32 = 0x0800_0000;
    const SLOT_160_TARGET: u32 = 0x0900_0160;
    const SLOT_94_TARGET: u32 = 0x0900_0094;
    const SLOT_22C_TARGET: u32 = 0x0900_022c;

    type CallLog = Vec<(u32, Vec<u32>)>;
    type Queue = Rc<RefCell<VecDeque<u32>>>;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An engine whose callees outside this file are doubles that read the
    /// test objects' fields (see [`new_reference`]).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_9000, 0x1000);
        e.map(0x011c_a000, 0x1000);
        let mut slots = vec![0u32; 0x230 / 4];
        slots[0x94 / 4] = SLOT_94_TARGET;
        slots[0x160 / 4] = SLOT_160_TARGET;
        slots[0x22c / 4] = SLOT_22C_TARGET;
        e.put_vtable(TEST_VTABLE, &slots);
        e.register(SLOT_160_TARGET, |e, a| {
            ret(u32::from(e.mem.u8(a[0] + 0x40)))
        });
        e.register(SLOT_94_TARGET, |e, a| ret(u32::from(e.mem.u8(a[0] + 0x40))));
        e.register(SLOT_22C_TARGET, |e, a| {
            ret(u32::from(e.mem.u8(a[0] + 0x41)))
        });
        e.register(GET_BASE_FORM, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(GET_FORM_TYPE, |e, a| ret(u32::from(e.mem.u8(a[0] + 4))));
        e.register(REFERENCE_KEY, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(ENTRY_WORD_8, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(ENTRY_WORD_4, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(REFERENCE_FLAG_800, |e, a| {
            ret(u32::from(e.mem.u8(a[0] + 0x42)))
        });
        e.register(REFERENCE_FLAG_20, |e, a| {
            ret(u32::from(e.mem.u8(a[0] + 0x43)))
        });
        e.register(REFERENCE_FLAG_2000000, |e, a| {
            ret(u32::from(e.mem.u8(a[0] + 0x44)))
        });
        e.register(GET_EXTRA_DATA_LIST, |_, a| ret(a[0]));
        e.register(GET_CONTAINER_CHANGES, |e, a| ret(e.mem.u32(a[0] + 0x48)));
        e.register(GET_EXTRA_DATA, |e, a| {
            ret(if a[1] == 0x5d {
                e.mem.u32(a[0] + 0x4c)
            } else {
                0
            })
        });
        e.register(REFERENCE_PREDICATE_00568E50, |e, a| {
            ret(e.mem.u32(a[0] + 0x50))
        });
        e.register(CONTAINER_CAN_HOLD_TYPE, |_, a| {
            ret(u32::from([0x28, 0x29, 0x30].contains(&a[0])))
        });
        e.register(CONTAINER_CAN_ACTIVATE, |e, a| {
            ret(u32::from(e.mem.u8(a[1] + 0x45)))
        });
        e.register(MAP_CLEAR, |_, _| Ret::default());
        e.register(TABLE_FREE, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(TABLE_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(MEMSET, |_, _| Ret::default());
        e.register(GUARD_CONSTRUCT, |_, a| ret(a[0]));
        e.register(GUARD_DESTRUCT, |_, _| Ret::default());
        e.register(GET_TIME_STAMP, |_, _| ret(0x1234));
        e.register(MAP_SET_AT, |_, _| Ret::default());
        e.register(MAP_REMOVE_AT, |_, _| ret(1));
        e.register(MAP_LOOKUP, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e
    }

    fn log(e: &Engine) -> CallLog {
        e.call_log.clone().unwrap()
    }

    /// A reference (0x60 bytes) with its form: the form at `+0x20`, the key
    /// at `+0xC`; the form's type byte at `+4`. Test flags: `+0x40` the
    /// answer of slot `0x160` (and of the form's slot `0x94`), `+0x41` slot
    /// `0x22c`, `+0x42`/`+0x43`/`+0x44` the three flag tests, `+0x45`
    /// CanActivate, `+0x48` container changes, `+0x4C` extra data, `+0x50`
    /// the `00568e50` answer.
    fn new_reference(e: &mut Engine, form_type: u8) -> Ptr {
        let reference = e.mem.alloc(0x60);
        let form = e.mem.alloc(0x60);
        e.mem.set_u32(reference, TEST_VTABLE);
        e.mem.set_u32(form, TEST_VTABLE);
        e.mem.set_u32(reference + 0x20, form);
        e.mem.set_u32(reference + 0xc, reference ^ 0x55);
        e.mem.set_u8(form + 4, form_type);
        Ptr::new(reference)
    }

    fn new_collection(e: &mut Engine) -> Ptr<BGSLoadedReferenceCollection> {
        e.new_object()
    }

    /// The maps (as offsets in the collection) the recorded calls to `call`
    /// (`SetAt` / `RemoveAt`) touched.
    fn touched_maps(e: &Engine, collection: Ptr, call: u32) -> Vec<u32> {
        log(e)
            .iter()
            .filter(|(addr, _)| *addr == call)
            .map(|(_, args)| args[0] - collection.addr())
            .collect()
    }

    /// Makes the global radiation-map form the reference's own form.
    fn make_radiation_form(e: &mut Engine, reference: Ptr) {
        let form = e.mem.u32(reference.addr() + 0x20);
        e.mem.set_u32(FORM_FOR_RADIATION_MAP, form);
    }

    #[test]
    fn every_function_of_the_unit_is_registered_once() {
        let mut addrs: Vec<u32> = funcs().iter().map(|f| f.0).collect();
        assert_eq!(addrs.len(), 27);
        addrs.sort_unstable();
        addrs.dedup();
        assert_eq!(addrs.len(), 27);
    }

    #[test]
    fn map_constructor_builds_the_table_and_zeroes_the_counts() {
        let mut e = engine();
        let map: Ptr<LoadedReferenceMap> = e.new_object();
        e.set(map, LoadedReferenceMap::iCount, 9);
        e.set(map, LoadedReferenceMap::iLastChangedTimeStamp, 9);
        assert_eq!(fn_00526b00(&mut e, map), map);
        assert_eq!(e.mem.u32(map.addr()), LOADED_REFERENCE_MAP_VTABLE);
        assert_eq!(e.get(map, LoadedReferenceMap::m_uiHashSize), 0x25);
        assert_eq!(e.get(map, LoadedReferenceMap::m_kAllocator), 0);
        assert_eq!(e.get(map, LoadedReferenceMap::iCount), 0);
        assert_eq!(e.get(map, LoadedReferenceMap::iLastChangedTimeStamp), 0);
        let table = e.get(map, LoadedReferenceMap::m_ppkHashTable);
        assert_ne!(table, 0);
        assert_eq!(
            log(&e),
            vec![(TABLE_ALLOCATE, vec![0x94]), (MEMSET, vec![table, 0, 0x94])]
        );
    }

    #[test]
    fn map_scalar_deleting_destructor_clears_frees_the_table_and_the_object() {
        let mut e = engine();
        let map: Ptr<LoadedReferenceMap> = e.new_object();
        fn_00526b00(&mut e, map);
        let table = e.get(map, LoadedReferenceMap::m_ppkHashTable);
        e.call_log = Some(vec![]);
        loaded_reference_map_scalar_deleting_destructor(&mut e, map, 1);
        let this = map.addr();
        assert_eq!(
            log(&e),
            vec![
                (MAP_CLEAR, vec![this]),
                (MAP_CLEAR, vec![this]),
                (MAP_CLEAR, vec![this]),
                (TABLE_FREE, vec![table]),
                (OPERATOR_DELETE, vec![this]),
            ]
        );
        assert_eq!(e.mem.u32(this), NI_T_MAP_BASE_VTABLE);
        e.call_log = Some(vec![]);
        loaded_reference_map_scalar_deleting_destructor(&mut e, map, 0);
        assert!(log(&e).iter().all(|(addr, _)| *addr != OPERATOR_DELETE));
    }

    #[test]
    fn ni_t_map_constructor_and_destructors() {
        let mut e = engine();
        let map = Ptr::new(e.mem.alloc(0x18));
        assert_eq!(fn_00527770(&mut e, map, 7), map);
        assert_eq!(e.mem.u32(map.addr()), NI_T_MAP_VTABLE);
        assert_eq!(e.mem.u32(map.addr() + 4), 7);
        let table = e.mem.u32(map.addr() + 8);
        assert_eq!(log(&e)[1], (MEMSET, vec![table, 0, 28]));
        e.call_log = Some(vec![]);
        assert_eq!(
            ni_t_map_reference_scalar_deleting_destructor(&mut e, map, 1),
            map
        );
        assert_eq!(e.mem.u32(map.addr()), NI_T_MAP_BASE_VTABLE);
        assert_eq!(
            log(&e).last().unwrap(),
            &(OPERATOR_DELETE, vec![map.addr()])
        );
        e.call_log = Some(vec![]);
        assert_eq!(
            ni_t_map_base_reference_scalar_deleting_destructor(&mut e, map, 0),
            map
        );
        assert_eq!(
            log(&e),
            vec![(MAP_CLEAR, vec![map.addr()]), (TABLE_FREE, vec![table])]
        );
        ni_t_map_base_reference_scalar_deleting_destructor(&mut e, map, 1);
        assert_eq!(
            log(&e).last().unwrap(),
            &(OPERATOR_DELETE, vec![map.addr()])
        );
    }

    #[test]
    fn adding_a_reference_stores_counts_and_stamps() {
        let mut e = engine();
        let map: Ptr<LoadedReferenceMap> = e.new_object();
        e.set(map, LoadedReferenceMap::iCount, 6);
        let reference = new_reference(&mut e, 5);
        let key = e.mem.u32(reference.addr() + 0xc);
        assert!(fn_00526bd0(&mut e, map, reference));
        assert_eq!(e.get(map, LoadedReferenceMap::iCount), 7);
        assert_eq!(
            e.get(map, LoadedReferenceMap::iLastChangedTimeStamp),
            0x1234
        );
        let calls = log(&e);
        let addrs: Vec<u32> = calls.iter().map(|c| c.0).collect();
        assert_eq!(
            addrs,
            vec![
                REFERENCE_KEY,
                MAP_LOOKUP,
                REFERENCE_KEY,
                MAP_SET_AT,
                GET_TIME_STAMP
            ]
        );
        assert_eq!(calls[3].1, vec![map.addr(), key, reference.addr()]);
        assert_eq!(calls[4].1, vec![TIME_STAMP_SOURCE]);
    }

    #[test]
    fn adding_a_known_reference_changes_nothing() {
        let mut e = engine();
        e.register(MAP_LOOKUP, |_, _| ret(1));
        let map: Ptr<LoadedReferenceMap> = e.new_object();
        e.set(map, LoadedReferenceMap::iCount, 6);
        let reference = new_reference(&mut e, 5);
        assert!(!fn_00526bd0(&mut e, map, reference));
        assert_eq!(e.get(map, LoadedReferenceMap::iCount), 6);
        assert_eq!(e.get(map, LoadedReferenceMap::iLastChangedTimeStamp), 0);
        assert!(log(&e)
            .iter()
            .all(|c| c.0 != MAP_SET_AT && c.0 != GET_TIME_STAMP));
    }

    #[test]
    fn removing_a_reference_counts_down_and_stamps() {
        let mut e = engine();
        let map: Ptr<LoadedReferenceMap> = e.new_object();
        e.set(map, LoadedReferenceMap::iCount, 6);
        let reference = new_reference(&mut e, 5);
        let key = e.mem.u32(reference.addr() + 0xc);
        assert!(fn_00526c40(&mut e, map, reference));
        assert_eq!(e.get(map, LoadedReferenceMap::iCount), 5);
        assert_eq!(
            e.get(map, LoadedReferenceMap::iLastChangedTimeStamp),
            0x1234
        );
        assert_eq!(log(&e)[1], (MAP_REMOVE_AT, vec![map.addr(), key]));
    }

    #[test]
    fn removing_an_unknown_reference_changes_nothing() {
        let mut e = engine();
        e.register(MAP_REMOVE_AT, |_, _| ret(0));
        let map: Ptr<LoadedReferenceMap> = e.new_object();
        e.set(map, LoadedReferenceMap::iCount, 6);
        let reference = new_reference(&mut e, 5);
        assert!(!fn_00526c40(&mut e, map, reference));
        assert_eq!(e.get(map, LoadedReferenceMap::iCount), 6);
        assert!(log(&e).iter().all(|c| c.0 != GET_TIME_STAMP));
    }

    #[test]
    fn collection_constructor_makes_eight_maps() {
        let mut e = engine();
        let collection = new_collection(&mut e);
        assert_eq!(fn_00526c90(&mut e, collection), collection);
        for index in 0..8 {
            let map = collection.addr() + 0x18 * index;
            assert_eq!(e.mem.u32(map), LOADED_REFERENCE_MAP_VTABLE);
            assert_eq!(e.mem.u32(map + 4), 0x25);
        }
    }

    #[test]
    fn collection_destructor_destroys_the_maps_backwards() {
        let mut e = engine();
        let collection = new_collection(&mut e);
        fn_00526c90(&mut e, collection);
        e.call_log = Some(vec![]);
        fn_00526d50(&mut e, collection);
        let clears: Vec<u32> = log(&e)
            .iter()
            .filter(|c| c.0 == MAP_CLEAR)
            .map(|c| c.1[0] - collection.addr())
            .collect();
        assert_eq!(clears.len(), 24);
        let firsts: Vec<u32> = clears.iter().step_by(3).copied().collect();
        assert_eq!(firsts, vec![0xa8, 0x90, 0x78, 0x60, 0x48, 0x30, 0x18, 0]);
    }

    #[test]
    fn creating_the_collection_stores_it_in_the_global() {
        let mut e = engine();
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        fn_00526e10(&mut e);
        let collection = e.mem.u32(COLLECTION_INSTANCE);
        assert_ne!(collection, 0);
        assert_eq!(e.mem.u32(collection + 0xa8), LOADED_REFERENCE_MAP_VTABLE);
        let calls = log(&e);
        let guard = calls[0].1[0];
        assert_eq!(
            calls[0],
            (
                GUARD_CONSTRUCT,
                vec![guard, 0x2c, 1, SOURCE_FILE_NAME, 0x58]
            )
        );
        assert_eq!(calls[1], (OPERATOR_NEW, vec![0xc0]));
        assert_eq!(calls.last().unwrap(), &(GUARD_DESTRUCT, vec![guard]));
    }

    #[test]
    fn creating_the_collection_with_a_failed_allocation_stores_null() {
        let mut e = engine();
        e.mem.set_u32(COLLECTION_INSTANCE, 0x1111);
        e.register(OPERATOR_NEW, |_, _| ret(0));
        fn_00526e10(&mut e);
        assert_eq!(e.mem.u32(COLLECTION_INSTANCE), 0);
        let addrs: Vec<u32> = log(&e).iter().map(|c| c.0).collect();
        assert_eq!(addrs, vec![GUARD_CONSTRUCT, OPERATOR_NEW, GUARD_DESTRUCT]);
    }

    #[test]
    fn destroying_the_collection_frees_it_and_clears_the_global() {
        let mut e = engine();
        let collection = new_collection(&mut e);
        fn_00526c90(&mut e, collection);
        e.mem.set_u32(COLLECTION_INSTANCE, collection.addr());
        e.call_log = Some(vec![]);
        fn_00526eb0(&mut e);
        assert_eq!(e.mem.u32(COLLECTION_INSTANCE), 0);
        assert_eq!(
            log(&e).last().unwrap(),
            &(OPERATOR_DELETE, vec![collection.addr()])
        );
        e.call_log = Some(vec![]);
        fn_00526eb0(&mut e);
        assert!(log(&e).is_empty());
    }

    #[test]
    fn collection_scalar_deleting_destructor_frees_only_with_the_flag() {
        let mut e = engine();
        let collection = new_collection(&mut e);
        fn_00526c90(&mut e, collection);
        e.call_log = Some(vec![]);
        assert_eq!(fn_00526ef0(&mut e, collection, 0), collection);
        assert!(log(&e).iter().all(|c| c.0 != OPERATOR_DELETE));
        assert_eq!(fn_00526ef0(&mut e, collection, 3), collection);
        assert_eq!(log(&e).last().unwrap().0, OPERATOR_DELETE);
    }

    /// Adds a reference of the given form type to a fresh collection and
    /// returns the maps (offsets) it was added to.
    fn add_maps(e: &mut Engine, form_type: u8, setup: fn(&mut Engine, Ptr)) -> Vec<u32> {
        let collection = new_collection(e);
        let reference = new_reference(e, form_type);
        setup(e, reference);
        e.call_log = Some(vec![]);
        fn_00526f20(e, collection, reference);
        touched_maps(e, collection.cast(), MAP_SET_AT)
    }

    #[test]
    fn adding_by_type_picks_the_maps() {
        let mut e = engine();
        e.mem.set_u32(FORM_FOR_RADIATION_MAP, 0x7777);
        assert_eq!(add_maps(&mut e, 0x28, |_, _| {}), vec![0x18, 0x30]);
        assert_eq!(add_maps(&mut e, 0x29, |_, _| {}), vec![0x18, 0x48]);
        assert_eq!(add_maps(&mut e, 0x30, |_, _| {}), vec![0x18]);
        assert_eq!(add_maps(&mut e, 0x1b, |_, _| {}), vec![0x60]);
        assert_eq!(add_maps(&mut e, 0x2a, |_, _| {}), vec![0x78]);
        assert_eq!(add_maps(&mut e, 0x2b, |_, _| {}), vec![0x78]);
        assert_eq!(add_maps(&mut e, 0x10, |_, _| {}), Vec::<u32>::new());
    }

    #[test]
    fn adding_by_special_cases() {
        let mut e = engine();
        e.mem.set_u32(FORM_FOR_RADIATION_MAP, 0x7777);
        // The base form is the global form.
        assert_eq!(add_maps(&mut e, 0x10, make_radiation_form), vec![0x90]);
        e.mem.set_u32(FORM_FOR_RADIATION_MAP, 0x7777);
        // Type 0x51 needs the extra data 0x5d.
        assert_eq!(add_maps(&mut e, 0x51, |_, _| {}), Vec::<u32>::new());
        assert_eq!(
            add_maps(&mut e, 0x51, |e, r| e.mem.set_u32(r.addr() + 0x4c, 1)),
            vec![0x90]
        );
        assert!(log(&e)
            .iter()
            .any(|c| c.0 == GET_EXTRA_DATA && c.1[1] == 0x5d));
        // Type 0x1c needs the predicate.
        assert_eq!(add_maps(&mut e, 0x1c, |_, _| {}), Vec::<u32>::new());
        assert_eq!(
            add_maps(&mut e, 0x1c, |e, r| e.mem.set_u32(r.addr() + 0x50, 1)),
            vec![0xa8]
        );
    }

    #[test]
    fn adding_runs_under_the_scope_guard() {
        let mut e = engine();
        e.mem.set_u32(FORM_FOR_RADIATION_MAP, 0x7777);
        add_maps(&mut e, 0x10, |_, _| {});
        let calls = log(&e);
        assert_eq!(calls[0].0, GUARD_CONSTRUCT);
        assert_eq!(calls[0].1[1..], [0x2c, 1, SOURCE_FILE_NAME, 0x68]);
        assert_eq!(calls.last().unwrap().0, GUARD_DESTRUCT);
    }

    fn removed_maps(e: &mut Engine, form_type: u8, setup: fn(&mut Engine, Ptr)) -> Vec<u32> {
        let collection = new_collection(e);
        let reference = new_reference(e, form_type);
        setup(e, reference);
        e.call_log = Some(vec![]);
        fn_005270b0(e, collection, reference);
        touched_maps(e, collection.cast(), MAP_REMOVE_AT)
    }

    #[test]
    fn removing_by_type_mirrors_adding() {
        let mut e = engine();
        e.mem.set_u32(FORM_FOR_RADIATION_MAP, 0x7777);
        assert_eq!(removed_maps(&mut e, 0x28, |_, _| {}), vec![0x18, 0x30]);
        assert_eq!(removed_maps(&mut e, 0x29, |_, _| {}), vec![0x18, 0x48]);
        assert_eq!(removed_maps(&mut e, 0x1b, |_, _| {}), vec![0x60]);
        assert_eq!(removed_maps(&mut e, 0x2b, |_, _| {}), vec![0x78]);
        // No extra data is asked for when removing type 0x51.
        assert_eq!(removed_maps(&mut e, 0x51, |_, _| {}), vec![0x90]);
        assert_eq!(removed_maps(&mut e, 0x1c, |_, _| {}), Vec::<u32>::new());
        assert_eq!(
            removed_maps(&mut e, 0x1c, |e, r| e.mem.set_u32(r.addr() + 0x50, 1)),
            vec![0xa8]
        );
        assert_eq!(removed_maps(&mut e, 0x10, make_radiation_form), vec![0x90]);
    }

    #[test]
    fn extra_data_lookup_goes_through_the_list() {
        let mut e = engine();
        let reference = new_reference(&mut e, 1);
        e.mem.set_u32(reference.addr() + 0x4c, 0xabcd);
        assert_eq!(fn_00527080(&mut e, reference, 0x5d), 0xabcd);
        assert_eq!(
            log(&e),
            vec![
                (GET_EXTRA_DATA_LIST, vec![reference.addr()]),
                (GET_EXTRA_DATA, vec![reference.addr(), 0x5d]),
            ]
        );
    }

    #[test]
    fn map_for_type() {
        let mut e = engine();
        let collection = new_collection(&mut e);
        let base = collection.addr();
        assert_eq!(fn_005271b0(&mut e, collection, 0x28).addr(), base + 0x30);
        assert_eq!(fn_005271b0(&mut e, collection, 0x29).addr(), base + 0x48);
        assert!(fn_005271b0(&mut e, collection, 0x2a).is_null());
    }

    #[test]
    fn item_change_helpers() {
        let mut e = engine();
        e.register(ITEM_CHANGE_CONSTRUCT, |_, _| Ret::default());
        e.register(ITEM_CHANGE_RELEASE, |_, _| Ret::default());
        let item = Ptr::new(e.mem.alloc(0x18));
        for offset in [0xc, 0x10] {
            e.mem.set_u32(item.addr() + offset, 5);
        }
        e.mem.set_u8(item.addr() + 0x14, 5);
        assert_eq!(fn_00527270(&mut e, item), item);
        assert_eq!(log(&e), vec![(ITEM_CHANGE_CONSTRUCT, vec![item.addr()])]);
        assert_eq!(e.mem.u32(item.addr() + 0xc), 0);
        assert_eq!(e.mem.u32(item.addr() + 0x10), 0);
        assert_eq!(e.mem.u8(item.addr() + 0x14), 0);
        e.mem.set_u32(item.addr(), 9);
        e.mem.set_u32(item.addr() + 0xc, 5);
        e.mem.set_u8(item.addr() + 0x14, 5);
        fn_00527740(&mut e, item);
        assert_eq!(e.mem.u32(item.addr() + 0xc), 0);
        assert_eq!(e.mem.u8(item.addr() + 0x14), 0);
        assert_eq!(e.mem.u32(item.addr()), 9);
        fn_005272b0(&mut e, item);
        assert_eq!(e.mem.u32(item.addr()), 0);
        assert_eq!(
            log(&e).last().unwrap(),
            &(ITEM_CHANGE_RELEASE, vec![item.addr()])
        );
    }

    /// An engine for the iterator tests: the collection exists at the
    /// global; the first-position and next-item walkers are doubles fed from
    /// the two queues (positions handed out by `MAP_FIRST_POSITION`;
    /// references handed out by `MAP_NEXT_ITEM`, which leaves a non-zero
    /// position while more remain).
    fn iterator_engine() -> (Engine, Ptr<BGSLoadedReferenceCollection>, Queue, Queue) {
        let mut e = engine();
        e.register(ITEM_CHANGE_CONSTRUCT, |_, _| Ret::default());
        e.register(OFFSET_0X18, |_, a| ret(a[0] + 0x18));
        let collection = new_collection(&mut e);
        e.mem.set_u32(COLLECTION_INSTANCE, collection.addr());
        let positions: Queue = Rc::new(RefCell::new(VecDeque::new()));
        let positions_seen = positions.clone();
        e.register_double(MAP_FIRST_POSITION, move |_, _| {
            ret(positions_seen.borrow_mut().pop_front().unwrap_or(0))
        });
        let entries: Queue = Rc::new(RefCell::new(VecDeque::new()));
        let entries_seen = entries.clone();
        e.register_double(MAP_NEXT_ITEM, move |e, a| {
            let mut queue = entries_seen.borrow_mut();
            let reference = queue.pop_front().unwrap();
            e.mem.set_u32(a[2], 0xdead);
            e.mem.set_u32(a[3], reference);
            e.mem.set_u32(a[1], u32::from(!queue.is_empty()));
            Ret::default()
        });
        (e, collection, positions, entries)
    }

    /// Feeds `NEXT_INVENTORY_ITEM` from a queue of entries.
    fn feed_inventory(e: &mut Engine, entries: &[u32]) -> Queue {
        let queue: Queue = Rc::new(RefCell::new(entries.iter().copied().collect()));
        let queue_seen = queue.clone();
        e.register_double(NEXT_INVENTORY_ITEM, move |e, a| {
            match queue_seen.borrow_mut().pop_front() {
                Some(entry) => {
                    e.mem.set_u32(a[2], entry);
                    ret(1)
                }
                None => ret(0),
            }
        });
        queue
    }

    fn new_iterator(e: &mut Engine) -> Ptr<LoadedReferenceIterator> {
        e.new_object()
    }

    fn started_iterator(e: &mut Engine, form_type: u32) -> Ptr<LoadedReferenceIterator> {
        let iterator = new_iterator(e);
        fn_005271f0(e, iterator, form_type, 0);
        iterator
    }

    fn run_next(
        e: &mut Engine,
        collection: Ptr<BGSLoadedReferenceCollection>,
        iterator: Ptr<LoadedReferenceIterator>,
        flags: u32,
    ) -> (bool, u32, u32) {
        let out = e.mem.alloc(8);
        let found = fn_005273c0(
            e,
            collection,
            iterator,
            Ptr::new(out),
            Ptr::new(out + 4),
            flags,
        );
        (found, e.mem.u32(out), e.mem.u32(out + 4))
    }

    #[test]
    fn iterator_constructors_store_the_search_and_reset() {
        let (mut e, collection, positions, _) = iterator_engine();
        positions.borrow_mut().extend([0x40, 0x41]);
        let iterator = new_iterator(&mut e);
        assert_eq!(fn_005271f0(&mut e, iterator, 0x28, 77), iterator);
        assert_eq!(e.get(iterator, LoadedReferenceIterator::form_type), 0x28);
        assert!(e
            .get(iterator, LoadedReferenceIterator::base_form)
            .is_null());
        assert_eq!(
            e.get(iterator, LoadedReferenceIterator::activate_argument),
            77
        );
        assert_eq!(
            e.get(iterator, LoadedReferenceIterator::current_map).addr(),
            collection.addr() + 0x30
        );
        assert_eq!(
            e.get(iterator, LoadedReferenceIterator::position).addr(),
            0x40
        );

        // By base form: the type comes from the form; a type without a map
        // falls back to the object 00500940 returns.
        let form = e.mem.alloc(0x20);
        e.mem.set_u8(form + 4, 0x05);
        let other = new_iterator(&mut e);
        e.set(
            other,
            LoadedReferenceIterator::current_container,
            Ptr::new(1),
        );
        assert_eq!(fn_005272d0(&mut e, other, Ptr::new(form), 3), other);
        assert_eq!(e.get(other, LoadedReferenceIterator::form_type), 5);
        assert_eq!(
            e.get(other, LoadedReferenceIterator::base_form).addr(),
            form
        );
        assert_eq!(e.get(other, LoadedReferenceIterator::activate_argument), 3);
        assert!(e
            .get(other, LoadedReferenceIterator::current_container)
            .is_null());
        assert_eq!(
            e.get(other, LoadedReferenceIterator::current_map).addr(),
            collection.addr() + 0x18
        );
        assert_eq!(e.get(other, LoadedReferenceIterator::position).addr(), 0x41);
    }

    #[test]
    fn next_without_a_map_is_false() {
        let (mut e, collection, _, _) = iterator_engine();
        let iterator = new_iterator(&mut e);
        assert!(!run_next(&mut e, collection, iterator, 0).0);
    }

    #[test]
    fn next_in_a_typed_map_returns_the_first_unfiltered_reference() {
        let (mut e, collection, positions, entries) = iterator_engine();
        positions.borrow_mut().push_back(1);
        let hidden = new_reference(&mut e, 0x28);
        e.mem.set_u8(hidden.addr() + 0x42, 1);
        let wanted = new_reference(&mut e, 0x28);
        entries.borrow_mut().extend([hidden.addr(), wanted.addr()]);
        let iterator = started_iterator(&mut e, 0x28);
        let (found, reference, item) = run_next(&mut e, collection, iterator, 0);
        assert!(found);
        assert_eq!(reference, wanted.addr());
        assert_eq!(item, 0);
    }

    #[test]
    fn next_skips_by_every_flag_test() {
        // The test flag each skip reads; the 0x2000000 test is overridden
        // by bit 3 of the flags.
        for (offset, flags, skipped) in [
            (0x42, 0, true),
            (0x43, 0, true),
            (0x40, 0, true),
            (0x44, 0, true),
            (0x44, 8, false),
        ] {
            let (mut e, collection, positions, entries) = iterator_engine();
            positions.borrow_mut().push_back(1);
            let candidate = new_reference(&mut e, 0x28);
            e.mem.set_u8(candidate.addr() + offset, 1);
            let fallback = new_reference(&mut e, 0x28);
            entries
                .borrow_mut()
                .extend([candidate.addr(), fallback.addr()]);
            let iterator = started_iterator(&mut e, 0x28);
            let (found, reference, _) = run_next(&mut e, collection, iterator, flags);
            assert!(found);
            let expected = if skipped { fallback } else { candidate };
            assert_eq!(reference, expected.addr(), "offset {offset:#x}");
        }
    }

    #[test]
    fn next_skips_references_whose_form_slot_says_so() {
        let (mut e, collection, positions, entries) = iterator_engine();
        positions.borrow_mut().push_back(1);
        let hidden = new_reference(&mut e, 0x28);
        let form = e.mem.u32(hidden.addr() + 0x20);
        e.mem.set_u8(form + 0x40, 1);
        let wanted = new_reference(&mut e, 0x28);
        entries.borrow_mut().extend([hidden.addr(), wanted.addr()]);
        let iterator = started_iterator(&mut e, 0x28);
        assert_eq!(run_next(&mut e, collection, iterator, 0).1, wanted.addr());
    }

    #[test]
    fn next_in_the_items_map_matches_the_type_or_the_base_form() {
        // By type: the items map is the only one checked.
        let (mut e, collection, positions, entries) = iterator_engine();
        positions.borrow_mut().push_back(1);
        let wrong = new_reference(&mut e, 3);
        let right = new_reference(&mut e, 5);
        entries.borrow_mut().extend([wrong.addr(), right.addr()]);
        let iterator = started_iterator(&mut e, 5);
        assert_eq!(run_next(&mut e, collection, iterator, 0).1, right.addr());

        // By base form.
        let (mut e, collection, positions, entries) = iterator_engine();
        positions.borrow_mut().push_back(1);
        let wrong = new_reference(&mut e, 5);
        let right = new_reference(&mut e, 5);
        entries.borrow_mut().extend([wrong.addr(), right.addr()]);
        let form = Ptr::new(e.mem.u32(right.addr() + 0x20));
        let iterator = new_iterator(&mut e);
        fn_005272d0(&mut e, iterator, form, 0);
        assert_eq!(run_next(&mut e, collection, iterator, 0).1, right.addr());
    }

    #[test]
    fn next_ends_after_the_last_stage() {
        let (mut e, collection, positions, _) = iterator_engine();
        positions.borrow_mut().extend([0, 0, 0]);
        let iterator = started_iterator(&mut e, 5);
        // Stage 0 with flags bit 0 moves to the containers map, the walk
        // runs out, and stage 2 ends it.
        assert!(!run_next(&mut e, collection, iterator, 1).0);
        assert!(e
            .get(iterator, LoadedReferenceIterator::current_map)
            .is_null());
        assert_eq!(e.get(iterator, LoadedReferenceIterator::map_stage), 2);
    }

    #[test]
    fn next_moves_to_the_actors_map_when_flags_ask_for_it() {
        let (mut e, collection, positions, entries) = iterator_engine();
        // Items map empty, stage 1 finds the actors map's first position.
        positions.borrow_mut().extend([0, 0, 1]);
        let actor = new_reference(&mut e, 0x2a);
        entries.borrow_mut().push_back(actor.addr());
        let iterator = started_iterator(&mut e, 5);
        // Flags 2: wants one side only; the answer 0 skips the actor.
        assert!(!run_next(&mut e, collection, iterator, 2).0);
        let walked = log(&e)
            .iter()
            .find(|c| c.0 == MAP_NEXT_ITEM)
            .map(|c| c.1[0])
            .unwrap();
        assert_eq!(walked, collection.addr() + 0x78);
        assert_eq!(e.get(iterator, LoadedReferenceIterator::map_stage), 2);
    }

    #[test]
    fn next_walks_a_containers_inventory() {
        let (mut e, collection, positions, entries) = iterator_engine();
        // The items map is empty; the containers map has one container.
        positions.borrow_mut().extend([0, 1]);
        let container = new_reference(&mut e, 0x1b);
        e.mem.set_u8(container.addr() + 0x45, 1);
        e.mem.set_u32(container.addr() + 0x48, 0x5000);
        entries.borrow_mut().push_back(container.addr());
        let iterator = started_iterator(&mut e, 5);

        // The inventory has one entry of the searched type.
        let wanted_form = e.mem.alloc(0x20);
        e.mem.set_u8(wanted_form + 4, 5);
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(entry + 8, wanted_form);
        e.mem.set_u32(entry + 4, 3);
        feed_inventory(&mut e, &[entry]);

        let (found, reference, item) = run_next(&mut e, collection, iterator, 1);
        assert!(found);
        assert_eq!(reference, container.addr());
        assert_eq!(item, entry);
        assert_eq!(
            e.get(iterator, LoadedReferenceIterator::container_changes)
                .addr(),
            0x5000
        );

        // The next step finds the inventory exhausted: the embedded
        // ItemChange is cleared, the container dropped, and the maps run out.
        e.mem.set_u32(iterator.addr() + 0x28, 7);
        let (found, _, _) = run_next(&mut e, collection, iterator, 1);
        assert!(!found);
        assert!(e
            .get(iterator, LoadedReferenceIterator::current_container)
            .is_null());
        assert!(e
            .get(iterator, LoadedReferenceIterator::container_changes)
            .is_null());
        assert_eq!(e.mem.u32(iterator.addr() + 0x28), 0);
        let walks = log(&e)
            .iter()
            .filter(|c| c.0 == NEXT_INVENTORY_ITEM)
            .count();
        assert_eq!(walks, 2);
    }

    #[test]
    fn next_inventory_entries_need_the_form_and_the_word_at_4() {
        let (mut e, collection, positions, _) = iterator_engine();
        positions.borrow_mut().push_back(1);
        let iterator = started_iterator(&mut e, 5);
        let container = new_reference(&mut e, 0x1b);
        e.set(
            iterator,
            LoadedReferenceIterator::current_container,
            container.cast(),
        );
        e.set(
            iterator,
            LoadedReferenceIterator::container_changes,
            Ptr::new(0x6000),
        );
        let form_a = e.mem.alloc(0x20);
        e.mem.set_u8(form_a + 4, 9);
        let form_b = e.mem.alloc(0x20);
        e.mem.set_u8(form_b + 4, 5);
        let make_entry = |e: &mut Engine, form: u32, word_4: u32| {
            let entry = e.mem.alloc(0x20);
            e.mem.set_u32(entry + 8, form);
            e.mem.set_u32(entry + 4, word_4);
            entry
        };
        let wrong_type = make_entry(&mut e, form_a, 1);
        let empty = make_entry(&mut e, form_b, 0);
        let good = make_entry(&mut e, form_b, 1);
        let queue = feed_inventory(&mut e, &[wrong_type, empty, good]);
        let (found, reference, item) = run_next(&mut e, collection, iterator, 0);
        assert!(found);
        assert_eq!(reference, container.addr());
        assert_eq!(item, good);

        // With a base form, the entry's form must be exactly it.
        e.set(
            iterator,
            LoadedReferenceIterator::base_form,
            Ptr::new(form_b),
        );
        queue.borrow_mut().extend([wrong_type, good]);
        let (found, _, item) = run_next(&mut e, collection, iterator, 0);
        assert!(found);
        assert_eq!(item, good);
    }

    #[test]
    fn next_in_the_actors_map_uses_the_flags_and_slot_22c() {
        for (flags, answer, skipped) in [
            (2, 0, true),
            (2, 1, false),
            (4, 1, true),
            (4, 0, false),
            (6, 1, false),
            (6, 0, false),
        ] {
            let (mut e, collection, positions, entries) = iterator_engine();
            positions.borrow_mut().push_back(1);
            let iterator = started_iterator(&mut e, 5);
            let actors = collection.addr() + 0x78;
            e.set(
                iterator,
                LoadedReferenceIterator::current_map,
                Ptr::new(actors),
            );
            e.set(iterator, LoadedReferenceIterator::position, Ptr::new(1));
            e.mem.set_u8(iterator.addr() + 0x34, 1);
            let actor = new_reference(&mut e, 0x2a);
            e.mem.set_u8(actor.addr() + 0x41, answer);
            e.mem.set_u32(actor.addr() + 0x48, 0x7000);
            entries.borrow_mut().push_back(actor.addr());
            // The actor's inventory is empty, so a taken actor is dropped
            // again at once.
            feed_inventory(&mut e, &[]);
            let (found, _, _) = run_next(&mut e, collection, iterator, flags);
            assert!(!found);
            let asked = log(&e).iter().any(|c| c.0 == GET_CONTAINER_CHANGES);
            assert_eq!(asked, !skipped, "flags {flags} answer {answer}");
            assert!(e
                .get(iterator, LoadedReferenceIterator::current_container)
                .is_null());
        }
    }

    #[test]
    fn next_in_the_containers_map_asks_can_activate() {
        for (can_activate, taken) in [(0u8, false), (1, true)] {
            let (mut e, collection, positions, entries) = iterator_engine();
            positions.borrow_mut().push_back(1);
            let iterator = started_iterator(&mut e, 5);
            let containers = collection.addr() + 0x60;
            e.set(
                iterator,
                LoadedReferenceIterator::current_map,
                Ptr::new(containers),
            );
            e.set(iterator, LoadedReferenceIterator::position, Ptr::new(1));
            e.mem.set_u8(iterator.addr() + 0x34, 1);
            e.set(iterator, LoadedReferenceIterator::activate_argument, 0x99);
            let container = new_reference(&mut e, 0x1b);
            e.mem.set_u8(container.addr() + 0x45, can_activate);
            entries.borrow_mut().push_back(container.addr());
            feed_inventory(&mut e, &[]);
            run_next(&mut e, collection, iterator, 1);
            let calls = log(&e);
            let activation = calls
                .iter()
                .find(|c| c.0 == CONTAINER_CAN_ACTIVATE)
                .unwrap();
            let form = e.mem.u32(container.addr() + 0x20);
            assert_eq!(activation.1, vec![form, container.addr(), 0x99, 0]);
            assert_eq!(
                calls.iter().any(|c| c.0 == GET_CONTAINER_CHANGES),
                taken,
                "can activate {can_activate}"
            );
        }
    }
}
