//! `fallout shared/extradataobjects.cpp` (Xbox PDB source unit), part 3: its functions from `004382a0` up to
//! (not including) `ffffffff` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::extradataobjects`]; anything public there may be used here.
//!
//! Translated: the whole range, `004382a0` (`ExtraSpecialRenderFlags::Compare`)
//! to `00438a60` (29 functions, the last ones of the unit). Nothing is left
//! for a later session.
//!
//! Notes:
//! - After the two `ExtraSpecialRenderFlags` methods and `BSStringT<char>::Set`,
//!   the range is template code the compiler emitted into this unit: the
//!   constructors, destructors and scalar deleting destructors of two hash
//!   maps (`NiTMap<WaterZone *,int>`, `BSMap<TESFaction *,bool>`, and the
//!   `NiTMapBase` and `BSMapBase` bodies under them) and of three
//!   `BSSimpleArray<T,1024>` instances. Their names come from the engine map
//!   and from the scalar deleting destructors' names; where the map has no
//!   name the function is `fn_<addr>` with a description of the body.
//! - The two destructors with a compiler exception-unwinding frame
//!   (`00438720`, `004388a0`) are translated without the frame.
//! - A hash map is `vtable, bucket count (+4), bucket array (+8), item count
//!   (+0xc)`; the bucket release (`00438af0`, another unit's helper, called
//!   by address) walks every chain and calls vtable slots `0x10` and `0x18`
//!   on each node.

#[allow(unused_imports)]
use super::extradataobjects::*;
#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, BSStringT, NiTPointerMap};

/// `RTTI Type Descriptor` of `ExtraSpecialRenderFlags`, the target of the
/// `dynamic_cast` in its `Compare` and `Copy`.
const EXTRA_SPECIAL_RENDER_FLAGS_TYPE: u32 = 0x0118_4078;
/// The accessor of the word at +0xc (`m_flags`); `this` is the object.
const GET_FLAGS: u32 = 0x0084_e3a0;
/// The setter of the word at +0xc (`m_flags`; `this`, the new value). The
/// engine map names the folded body `NonActorMagicCaster::SetCurrentSpell`.
const SET_FLAGS: u32 = 0x0041_fd00;
/// A string-like constructor (`this`, a pointer, 0); `00438390` forwards to it.
const STRING_INIT: u32 = 0x0040_37f0;
/// The length of the `BSStringT` that is `this` (`sLen`, or the C string
/// length when `sLen` is `0xffff`).
const STRING_LENGTH: u32 = 0x0040_48e0;
/// `sMaxLen` of the `BSStringT` that is `this`.
const STRING_CAPACITY: u32 = 0x0040_39a0;
/// Stores `sMaxLen` (`this`, the value; clamped to `0xffff`).
const STRING_SET_CAPACITY: u32 = 0x0040_3a00;
/// Stores `sLen` (`this`, the value; clamped to `0xffff`).
const STRING_SET_LENGTH: u32 = 0x0040_39c0;
/// The `pString` word of the `BSStringT` that is `this`.
const STRING_C_STR: u32 = 0x0055_9450;
/// `memcpy(destination, source, size)` (a cdecl wrapper).
const MEM_COPY: u32 = 0x0040_1460;
/// `memset(destination, value, size)` (a cdecl wrapper).
const MEM_SET: u32 = 0x0040_3d30;
/// The C runtime `memset(destination, value, size)`.
const CRT_MEMSET: u32 = 0x00ec_61c0;
/// The bucket array allocator of `NiTMapBase` (size in bytes; never null).
const BUCKETS_ALLOCATE: u32 = 0x00aa_1070;
/// The bucket array release of `NiTMapBase` (pointer, may be null).
const BUCKETS_FREE: u32 = 0x00aa_10f0;
/// The node release shared by the map destructors (`this` is the map).
const BUCKETS_RELEASE: u32 = 0x0043_8af0;
/// Constructs the `count` elements that start at the pointer (`this`,
/// pointer, count); the `CombatTimeStamp` and `unsigned int` arrays use it.
const ARRAY_CONSTRUCT_ELEMENTS: u32 = 0x0043_0000;
/// The array constructor of the `DismemberedLimb *` and `unsigned int`
/// arrays (`this`, reservation, size).
const ARRAY_CONSTRUCT: u32 = 0x006b_3eb0;
/// Makes room for one more element at the end of an array and returns its
/// index (`this`).
const ARRAY_ADD_SLOT: u32 = 0x0076_1540;

/// Vtables of the template classes of this range.
const WATER_ZONE_MAP_VTABLE: u32 = 0x0101_5fbc;
const FACTION_MAP_VTABLE: u32 = 0x0101_5fdc;
const DISMEMBERED_LIMB_ARRAY_VTABLE: u32 = 0x0101_5ffc;
const COMBAT_TIME_STAMP_ARRAY_VTABLE: u32 = 0x0101_6010;
const UINT_ARRAY_VTABLE: u32 = 0x0101_6024;
const NI_T_MAP_BASE_VTABLE: u32 = 0x0101_6038;
const BS_MAP_BASE_VTABLE: u32 = 0x0101_6058;

layout! {
    /// `ExtraSpecialRenderFlags` (Xbox PDB), 0x14 bytes.
    pub struct ExtraSpecialRenderFlags: 0x14 {
        /// `m_flags` (Xbox PDB).
        0x0C m_flags: i32,
        /// `m_fNodeMaxFadeDistance` (Xbox PDB).
        0x10 m_fNodeMaxFadeDistance: f32,
    }
}

/// `operator delete(this)` when bit 0 of `flags` is set (the tail of every
/// scalar deleting destructor).
fn delete_when_asked(e: &mut Engine, this: Ptr, flags: u32) {
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
}

/// `__RTDynamicCast(other, 0, BSExtraData, ExtraSpecialRenderFlags, 0)`.
fn cast_to_special_render_flags(e: &mut Engine, other: Ptr) -> Ptr<ExtraSpecialRenderFlags> {
    e.call(
        DYNAMIC_CAST,
        &args![
            other,
            0u32,
            BS_EXTRA_DATA_TYPE,
            EXTRA_SPECIAL_RENDER_FLAGS_TYPE,
            0u32
        ],
    )
    .ptr()
}

// Translated from 004382a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSpecialRenderFlags::Compare` (Xbox PDB): true when `other` is not an
/// `ExtraSpecialRenderFlags`, when `BSExtraData::Compare` says so, when the
/// flags differ, or when `m_fNodeMaxFadeDistance` differs (a NaN counts as
/// different).
pub fn extra_special_render_flags_compare(
    e: &mut Engine,
    this: Ptr<ExtraSpecialRenderFlags>,
    other: Ptr,
) -> bool {
    let cast = cast_to_special_render_flags(e, other);
    if cast.is_null() {
        return true;
    }
    if e.call(BS_EXTRA_DATA_COMPARE, &args![this, other]).bool() {
        return true;
    }
    let ours = e.call(GET_FLAGS, &args![this]).u32();
    let theirs = e.call(GET_FLAGS, &args![cast]).u32();
    if ours != theirs {
        return true;
    }
    // `FUCOMPP`: unordered (NaN) floats count as different.
    e.get(this, ExtraSpecialRenderFlags::m_fNodeMaxFadeDistance)
        != e.get(cast, ExtraSpecialRenderFlags::m_fNodeMaxFadeDistance)
}

// Translated from 00438330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSpecialRenderFlags::Copy` (Xbox PDB): when `other` is an
/// `ExtraSpecialRenderFlags`, copies its flags (through the setter) and its
/// `m_fNodeMaxFadeDistance`.
pub fn extra_special_render_flags_copy(
    e: &mut Engine,
    this: Ptr<ExtraSpecialRenderFlags>,
    other: Ptr,
) {
    let cast = cast_to_special_render_flags(e, other);
    if cast.is_null() {
        return;
    }
    let flags = e.call(GET_FLAGS, &args![cast]).u32();
    e.call(SET_FLAGS, &args![this, flags]);
    let distance = e.get(cast, ExtraSpecialRenderFlags::m_fNodeMaxFadeDistance);
    e.set(
        this,
        ExtraSpecialRenderFlags::m_fNodeMaxFadeDistance,
        distance,
    );
}

// Translated from 00438390 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarding constructor (the engine map has no name for it): runs
/// `004037f0(this, arg, 0)` and returns `this`.
pub fn fn_00438390(e: &mut Engine, this: Ptr, arg: u32) -> Ptr {
    e.call(STRING_INIT, &args![this, arg, 0u32]);
    this
}

// Translated from 004383b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the map whose scalar deleting destructor the engine
/// map calls `NiTMap<WaterZone *,int>`: builds the hash map base
/// (`004386b0`, with the bucket count) and then stores the derived vtable.
pub fn fn_004383b0(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    fn_004386b0(e, this, hash_size);
    e.mem.set_u32(this.addr(), WATER_ZONE_MAP_VTABLE);
    this
}

// Translated from 004383e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the map whose scalar deleting destructor the engine
/// map calls `BSMap<TESFaction *,bool>`: builds the base (`00438820`, with
/// the bucket count) and then stores the derived vtable.
pub fn fn_004383e0(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    fn_00438820(e, this, hash_size);
    e.mem.set_u32(this.addr(), FACTION_MAP_VTABLE);
    this
}

// Translated from 00438410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<WaterZone *,int>::scalar deleting destructor` (Xbox PDB name of the
/// folded body): runs the destructor (`00438720`) and frees the object when
/// bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00438720(e, this.cast());
    delete_when_asked(e, this, flags);
    this
}

// Translated from 00438440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMap<TESFaction *,bool>::scalar deleting destructor` (Xbox PDB name of the
/// folded body): runs the destructor (`004388a0`) and frees the object when
/// bit 0 of `flags` is set. Returns `this`.
pub fn bs_map_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_004388a0(e, this.cast());
    delete_when_asked(e, this, flags);
    this
}

// Translated from 00438470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStringT<char>::Set` (Xbox PDB): copies the string `other` into `this`.
/// When it is longer than `sMaxLen` a new buffer of `len + 1` bytes replaces
/// the old one (which is freed); an empty source frees the buffer and nulls
/// it; otherwise the text is copied over the existing buffer. `sLen` is set
/// last. Returns whether the new string is non-empty.
pub fn bs_string_t_set(e: &mut Engine, this: Ptr<BSStringT>, other: Ptr) -> bool {
    let mut length = e.call(STRING_LENGTH, &args![other]).u32();
    let capacity = e.call(STRING_CAPACITY, &args![this]).u32();
    if length > capacity {
        let old = e.get(this, BSStringT::pString);
        let buffer = e.call(OPERATOR_NEW, &args![length + 1]).u32();
        e.set(this, BSStringT::pString, buffer);
        let text = e.call(STRING_C_STR, &args![other]).u32();
        e.call(MEM_COPY, &args![buffer, text, length + 1]);
        if old != 0 {
            e.call(OPERATOR_DELETE, &args![old]);
        }
        e.call(STRING_SET_CAPACITY, &args![this, length]);
    } else if length == 0 {
        let buffer = e.get(this, BSStringT::pString);
        if buffer != 0 {
            e.call(OPERATOR_DELETE, &args![buffer]);
        }
        e.set(this, BSStringT::pString, 0);
        length = 0;
        e.call(STRING_SET_CAPACITY, &args![this, 0u32]);
    } else {
        let text = e.call(STRING_C_STR, &args![other]).u32();
        let buffer = e.get(this, BSStringT::pString);
        e.call(MEM_COPY, &args![buffer, text, length + 1]);
    }
    e.call(STRING_SET_LENGTH, &args![this, length]);
    length != 0
}

// Translated from 00438570 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the array whose scalar deleting destructor the engine
/// map calls `BSSimpleArray<DismemberedLimb *,1024>`: stores the vtable and
/// runs the array constructor `006b3eb0(this, 0, 0)` (an empty array).
pub fn fn_00438570(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), DISMEMBERED_LIMB_ARRAY_VTABLE);
    e.call(ARRAY_CONSTRUCT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 004385a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the `BSSimpleArray<DismemberedLimb *,1024>`: stores the
/// vtable and clears the array, freeing its buffer (`008454f0(this, 1)`).
pub fn fn_004385a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), DISMEMBERED_LIMB_ARRAY_VTABLE);
    e.call(SIMPLE_ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 004385c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the `BSSimpleArray<CombatTimeStamp,1024>`: stores the
/// vtable and builds an empty array (`00438a60(this, 0, 0)`).
pub fn fn_004385c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), COMBAT_TIME_STAMP_ARRAY_VTABLE);
    fn_00438a60(e, this, 0, 0);
    this
}

// Translated from 004385f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the `BSSimpleArray<CombatTimeStamp,1024>`: stores the
/// vtable and clears the array, freeing its buffer.
pub fn fn_004385f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), COMBAT_TIME_STAMP_ARRAY_VTABLE);
    e.call(SIMPLE_ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 00438610 (decompiled, FalloutNV.exe 1.4.0.525)
/// The append of the `BSSimpleArray<CombatTimeStamp,1024>` (the engine map
/// has no name for it): takes the next slot (`00761540`), constructs it
/// (`00430000(this, slot, 1)`), copies in the word `*value` and returns the
/// new index.
pub fn fn_00438610(e: &mut Engine, this: Ptr<BSSimpleArray>, value: Ptr) -> u32 {
    let index = e.call(ARRAY_ADD_SLOT, &args![this]).u32();
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    e.call(
        ARRAY_CONSTRUCT_ELEMENTS,
        &args![this, buffer + index * 4, 1u32],
    );
    let word = e.mem.u32(value.addr());
    // The buffer is read again: the slot request can have moved it.
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    e.mem.set_u32(buffer + index * 4, word);
    index
}

// Translated from 00438660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the `BSSimpleArray<unsigned int,1024>`: stores the
/// vtable and runs the array constructor `006b3eb0(this, 0, 0)`.
pub fn fn_00438660(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), UINT_ARRAY_VTABLE);
    e.call(ARRAY_CONSTRUCT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00438690 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the `BSSimpleArray<unsigned int,1024>`: stores the
/// vtable and clears the array, freeing its buffer.
pub fn fn_00438690(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), UINT_ARRAY_VTABLE);
    e.call(SIMPLE_ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 004386b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase` constructor (the engine map has no name for it): stores
/// the base vtable and the bucket count, zeroes the item count, allocates
/// the bucket array (`hash_size * 4` bytes, `00aa1070`) and zeroes it.
pub fn fn_004386b0(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), NI_T_MAP_BASE_VTABLE);
    e.set(this, NiTPointerMap::m_uiHashSize, hash_size);
    e.set(this, NiTPointerMap::m_uiCount, 0);
    let buckets = e.call(BUCKETS_ALLOCATE, &args![hash_size << 2]).u32();
    e.set(this, NiTPointerMap::m_ppkHashTable, buckets);
    let size = e.get(this, NiTPointerMap::m_uiHashSize) << 2;
    e.call(MEM_SET, &args![buckets, 0u32, size]);
    this
}

// Translated from 00438720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the `NiTMap<WaterZone *,int>` (the engine map names the
/// body after a library function, `CMenu::~CMenu`): stores the derived
/// vtable, releases every node (`00438af0`) and runs the base destructor
/// (`00438780`). The exception-unwinding frame is not translated.
pub fn fn_00438720(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), WATER_ZONE_MAP_VTABLE);
    e.call(BUCKETS_RELEASE, &args![this]);
    fn_00438780(e, this);
}

// Translated from 00438780 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiTMapBase` destructor body: stores the base vtable, releases every
/// node (`00438af0`) and frees the bucket array (`00aa10f0`).
pub fn fn_00438780(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), NI_T_MAP_BASE_VTABLE);
    e.call(BUCKETS_RELEASE, &args![this]);
    let buckets = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(BUCKETS_FREE, &args![buckets]);
}

// Translated from 004387b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<TESBoundObject *,1024>::CompareBuffer<1024>` (Xbox PDB):
/// true when `other` has the same size as `this` and every element (one
/// word) is equal.
pub fn bs_simple_array_compare_buffer(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    other: Ptr<BSSimpleArray>,
) -> bool {
    let other_size = e.call(SIMPLE_ARRAY_SIZE, &args![other]).u32();
    let our_size = e.call(SIMPLE_ARRAY_SIZE, &args![this]).u32();
    if other_size != our_size {
        return false;
    }
    let mut index = 0u32;
    while index < e.get(this, BSSimpleArray::iSize) {
        let theirs = e.call(SIMPLE_ARRAY_AT, &args![other, index]).u32();
        let ours = e.call(SIMPLE_ARRAY_AT, &args![this, index]).u32();
        if e.mem.u32(theirs) != e.mem.u32(ours) {
            return false;
        }
        index += 1;
    }
    true
}

// Translated from 00438820 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSMapBase` constructor (the engine map has no name for it): stores
/// the base vtable and the bucket count, zeroes the item count and, for a
/// non-zero count, allocates the bucket array (`operator new`) and zeroes it
/// (`memset`). With a zero count the bucket pointer is left as it was.
pub fn fn_00438820(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), BS_MAP_BASE_VTABLE);
    e.set(this, NiTPointerMap::m_uiHashSize, hash_size);
    e.set(this, NiTPointerMap::m_uiCount, 0);
    if hash_size != 0 {
        let bytes = e.get(this, NiTPointerMap::m_uiHashSize) << 2;
        let buckets = e.call(OPERATOR_NEW, &args![bytes]).u32();
        e.set(this, NiTPointerMap::m_ppkHashTable, buckets);
        let bytes = e.get(this, NiTPointerMap::m_uiHashSize) << 2;
        e.call(CRT_MEMSET, &args![buckets, 0u32, bytes]);
    }
    this
}

// Translated from 004388a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the `BSMap<TESFaction *,bool>` (the engine map names the
/// body after a folded library function, `ctype<char>::~ctype<char>`):
/// stores the derived vtable, releases every node (`00438af0`) and runs the
/// base destructor (`00438900`). The exception-unwinding frame is not
/// translated.
pub fn fn_004388a0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), FACTION_MAP_VTABLE);
    e.call(BUCKETS_RELEASE, &args![this]);
    fn_00438900(e, this);
}

// Translated from 00438900 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSMapBase` destructor body: stores the base vtable and frees the
/// bucket array when there is one.
pub fn fn_00438900(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), BS_MAP_BASE_VTABLE);
    let buckets = e.get(this, NiTPointerMap::m_ppkHashTable);
    if buckets != 0 {
        e.call(OPERATOR_DELETE, &args![buckets]);
    }
}

// Translated from 00438940 (decompiled, FalloutNV.exe 1.4.0.525)
/// A map node release (the engine map has no name for it; `this` is not
/// used): clears the byte at +8 of `node` and frees it.
pub fn fn_00438940(e: &mut Engine, _this: Ptr, node: Ptr) {
    e.mem.set_u8(node.addr() + 8, 0);
    e.call(OPERATOR_DELETE, &args![node]);
}

// Translated from 00438970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<DismemberedLimb *,1024>::scalar deleting destructor` (Xbox
/// PDB name of the folded body): runs the destructor (`004385a0`) and frees
/// the object when bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_dismembered_limb_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_004385a0(e, this);
    delete_when_asked(e, this, flags);
    this
}

// Translated from 004389a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<CombatTimeStamp,1024>::scalar deleting destructor` (Xbox PDB
/// name of the folded body): runs the destructor (`004385f0`) and frees the
/// object when bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_combat_time_stamp_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_004385f0(e, this);
    delete_when_asked(e, this, flags);
    this
}

// Translated from 004389d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<unsigned int,1024>::scalar deleting destructor` (Xbox PDB
/// name of the folded body): runs the destructor (`00438690`) and frees the
/// object when bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_uint_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00438690(e, this);
    delete_when_asked(e, this, flags);
    this
}

// Translated from 00438a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<...>::scalar deleting destructor` (Xbox PDB name of the folded
/// body): runs the base destructor (`00438780`) and frees the object when
/// bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_base_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00438780(e, this.cast());
    delete_when_asked(e, this, flags);
    this
}

// Translated from 00438a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMapBase<TESFaction *,bool>::scalar deleting destructor` (Xbox PDB name of
/// the folded body): runs the base destructor (`00438900`) and frees the
/// object when bit 0 of `flags` is set. Returns `this`.
pub fn bs_map_base_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00438900(e, this.cast());
    delete_when_asked(e, this, flags);
    this
}

// Translated from 00438a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the `BSSimpleArray<CombatTimeStamp,1024>` body (the
/// engine map has no name for it; the vtable is stored by the caller):
/// zeroes the buffer, size and reserved words, takes the larger of the two
/// counts as the reservation, allocates it through vtable slot 4 and
/// constructs `size` elements (`00430000`).
pub fn fn_00438a60(e: &mut Engine, this: Ptr, reserve: u32, size: u32) {
    let array: Ptr<BSSimpleArray> = this.cast();
    e.set(array, BSSimpleArray::pBuffer, 0);
    e.set(array, BSSimpleArray::iSize, 0);
    e.set(array, BSSimpleArray::iReservedSize, 0);
    let reserve = reserve.max(size);
    if reserve != 0 {
        let buffer = e.vcall(this.addr(), 4, &args![reserve]).u32();
        e.set(array, BSSimpleArray::pBuffer, buffer);
        e.set(array, BSSimpleArray::iReservedSize, reserve);
    }
    if size != 0 {
        let buffer = e.get(array, BSSimpleArray::pBuffer);
        e.call(ARRAY_CONSTRUCT_ELEMENTS, &args![this, buffer, size]);
        e.set(array, BSSimpleArray::iSize, size);
    }
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004382a0,
            extra_special_render_flags_compare(Ptr<ExtraSpecialRenderFlags>, Ptr) -> bool
        ),
        entry!(
            0x00438330,
            extra_special_render_flags_copy(Ptr<ExtraSpecialRenderFlags>, Ptr)
        ),
        entry!(0x00438390, fn_00438390(Ptr, u32) -> Ptr),
        entry!(
            0x004383b0,
            fn_004383b0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x004383e0,
            fn_004383e0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00438410,
            ni_t_map_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00438440,
            bs_map_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00438470, bs_string_t_set(Ptr<BSStringT>, Ptr) -> bool),
        entry!(0x00438570, fn_00438570(Ptr) -> Ptr),
        entry!(0x004385a0, fn_004385a0(Ptr)),
        entry!(0x004385c0, fn_004385c0(Ptr) -> Ptr),
        entry!(0x004385f0, fn_004385f0(Ptr)),
        entry!(0x00438610, fn_00438610(Ptr<BSSimpleArray>, Ptr) -> u32),
        entry!(0x00438660, fn_00438660(Ptr) -> Ptr),
        entry!(0x00438690, fn_00438690(Ptr)),
        entry!(
            0x004386b0,
            fn_004386b0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00438720, fn_00438720(Ptr<NiTPointerMap>)),
        entry!(0x00438780, fn_00438780(Ptr<NiTPointerMap>)),
        entry!(
            0x004387b0,
            bs_simple_array_compare_buffer(Ptr<BSSimpleArray>, Ptr<BSSimpleArray>) -> bool
        ),
        entry!(
            0x00438820,
            fn_00438820(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x004388a0, fn_004388a0(Ptr<NiTPointerMap>)),
        entry!(0x00438900, fn_00438900(Ptr<NiTPointerMap>)),
        entry!(0x00438940, fn_00438940(Ptr, Ptr)),
        entry!(
            0x00438970,
            bs_simple_array_dismembered_limb_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x004389a0,
            bs_simple_array_combat_time_stamp_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x004389d0,
            bs_simple_array_uint_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00438a00,
            ni_t_map_base_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00438a30,
            bs_map_base_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00438a60, fn_00438a60(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An engine with the allocator's `operator new` and `operator delete`
    /// and the dynamic cast (the object itself, so a null `other` stays null).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_DELETE, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A render-flags object with the given flags word and fade distance.
    fn render_flags(e: &mut Engine, flags: u32, distance: f32) -> u32 {
        let object = e.mem.alloc(0x14);
        e.mem.set_u32(object + 0xc, flags);
        e.mem.set_f32(object + 0x10, distance);
        object
    }

    fn register_flag_accessors(e: &mut Engine) {
        e.register(GET_FLAGS, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(SET_FLAGS, |e, a| {
            e.mem.set_u32(a[0] + 0xc, a[1]);
            Ret::default()
        });
    }

    #[test]
    fn special_render_flags_compare_checks_each_part() {
        let mut e = engine();
        register_flag_accessors(&mut e);
        // The base comparison answers "different" when the word at +8 of
        // `this` is not zero.
        e.register(BS_EXTRA_DATA_COMPARE, |e, a| ret(e.mem.u32(a[0] + 8)));
        let this = render_flags(&mut e, 5, 2.0);
        let same = render_flags(&mut e, 5, 2.0);
        let other_flags = render_flags(&mut e, 6, 2.0);
        let other_distance = render_flags(&mut e, 5, 3.0);
        let nan = render_flags(&mut e, 5, f32::NAN);
        e.call_log = Some(vec![]);
        // A null `other` is not an `ExtraSpecialRenderFlags`.
        assert!(e.call(0x004382a0, &args![this, 0u32]).bool());
        assert!(!e.call(0x004382a0, &args![this, same]).bool());
        assert!(e.call(0x004382a0, &args![this, other_flags]).bool());
        assert!(e.call(0x004382a0, &args![this, other_distance]).bool());
        assert!(e.call(0x004382a0, &args![this, nan]).bool());
        assert_eq!(
            calls(&e, DYNAMIC_CAST)[1],
            vec![same, 0, BS_EXTRA_DATA_TYPE, 0x0118_4078, 0]
        );
        // When the base comparison says "different", the fields are not read.
        e.mem.set_u32(this + 8, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004382a0, &args![this, same]).bool());
        assert!(calls(&e, GET_FLAGS).is_empty());
    }

    #[test]
    fn special_render_flags_copy_copies_flags_and_distance() {
        let mut e = engine();
        register_flag_accessors(&mut e);
        let this = render_flags(&mut e, 1, 1.0);
        let source = render_flags(&mut e, 0x77, 9.5);
        e.call_log = Some(vec![]);
        e.call(0x00438330, &args![this, source]);
        assert_eq!(e.mem.u32(this + 0xc), 0x77);
        assert_eq!(e.mem.f32(this + 0x10), 9.5);
        assert_eq!(calls(&e, SET_FLAGS), vec![vec![this, 0x77]]);
        // A null source changes nothing.
        e.call(0x00438330, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0xc), 0x77);
        assert_eq!(calls(&e, SET_FLAGS).len(), 1);
    }

    #[test]
    fn fn_00438390_forwards_with_zero() {
        let mut e = engine();
        e.register(STRING_INIT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00438390, &args![0x1000u32, 0x2000u32]).u32(),
            0x1000
        );
        assert_eq!(calls(&e, STRING_INIT), vec![vec![0x1000, 0x2000, 0]]);
    }

    /// Doubles for the hash map base constructors and destructors.
    fn register_bucket_functions(e: &mut Engine) {
        e.register(BUCKETS_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        e.register(BUCKETS_FREE, |e, a| {
            if a[0] != 0 {
                e.mem.free(a[0]);
            }
            Ret::default()
        });
        e.register(MEM_SET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            Ret::default()
        });
        e.register(CRT_MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            ret(a[0])
        });
        e.register(BUCKETS_RELEASE, |_, _| Ret::default());
    }

    fn dirty_map(e: &mut Engine) -> u32 {
        let map = e.mem.alloc(0x10);
        for word in 0..4 {
            e.mem.set_u32(map + word * 4, 0xdead_0000 + word);
        }
        map
    }

    #[test]
    fn nitmap_base_constructor_allocates_zeroed_buckets() {
        let mut e = engine();
        register_bucket_functions(&mut e);
        let map = dirty_map(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004386b0, &args![map, 5u32]).u32(), map);
        assert_eq!(e.mem.u32(map), NI_T_MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 5);
        assert_eq!(e.mem.u32(map + 0xc), 0);
        let buckets = e.mem.u32(map + 8);
        assert_eq!(calls(&e, BUCKETS_ALLOCATE), vec![vec![20]]);
        assert_eq!(calls(&e, MEM_SET), vec![vec![buckets, 0, 20]]);
    }

    #[test]
    fn water_zone_map_constructor_stores_its_vtable_last() {
        let mut e = engine();
        register_bucket_functions(&mut e);
        let map = dirty_map(&mut e);
        assert_eq!(e.call(0x004383b0, &args![map, 3u32]).u32(), map);
        assert_eq!(e.mem.u32(map), WATER_ZONE_MAP_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 3);
        assert_ne!(e.mem.u32(map + 8), 0);
    }

    #[test]
    fn bs_map_base_constructor_allocates_only_for_a_non_zero_count() {
        let mut e = engine();
        register_bucket_functions(&mut e);
        let map = dirty_map(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00438820, &args![map, 4u32]).u32(), map);
        assert_eq!(e.mem.u32(map), BS_MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 4);
        assert_eq!(e.mem.u32(map + 0xc), 0);
        let buckets = e.mem.u32(map + 8);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![16]]);
        assert_eq!(calls(&e, CRT_MEMSET), vec![vec![buckets, 0, 16]]);
        // A zero count leaves the bucket word alone.
        let empty = dirty_map(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00438820, &args![empty, 0u32]);
        assert_eq!(e.mem.u32(empty + 8), 0xdead_0002);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn faction_map_constructor_stores_its_vtable_last() {
        let mut e = engine();
        register_bucket_functions(&mut e);
        let map = dirty_map(&mut e);
        assert_eq!(e.call(0x004383e0, &args![map, 2u32]).u32(), map);
        assert_eq!(e.mem.u32(map), FACTION_MAP_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 2);
    }

    #[test]
    fn nitmap_base_destructor_releases_nodes_then_buckets() {
        let mut e = engine();
        register_bucket_functions(&mut e);
        let map = dirty_map(&mut e);
        let buckets = e.mem.alloc(8);
        e.mem.set_u32(map + 8, buckets);
        e.call_log = Some(vec![]);
        e.call(0x00438780, &args![map]);
        assert_eq!(e.mem.u32(map), NI_T_MAP_BASE_VTABLE);
        assert_eq!(calls(&e, BUCKETS_RELEASE), vec![vec![map]]);
        assert_eq!(calls(&e, BUCKETS_FREE), vec![vec![buckets]]);
        assert_eq!(e.mem.block_size(buckets), None);
    }

    #[test]
    fn water_zone_map_destructor_runs_the_base_destructor() {
        let mut e = engine();
        register_bucket_functions(&mut e);
        let map = dirty_map(&mut e);
        e.mem.set_u32(map + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00438720, &args![map]);
        // Both the derived and the base destructor release the nodes.
        assert_eq!(calls(&e, BUCKETS_RELEASE), vec![vec![map], vec![map]]);
        assert_eq!(calls(&e, BUCKETS_FREE), vec![vec![0]]);
        assert_eq!(e.mem.u32(map), NI_T_MAP_BASE_VTABLE);
    }

    #[test]
    fn bs_map_base_destructor_frees_buckets_when_present() {
        let mut e = engine();
        let map = dirty_map(&mut e);
        let buckets = e.mem.alloc(8);
        e.mem.set_u32(map + 8, buckets);
        e.call_log = Some(vec![]);
        e.call(0x00438900, &args![map]);
        assert_eq!(e.mem.u32(map), BS_MAP_BASE_VTABLE);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![buckets]]);
        e.mem.set_u32(map + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00438900, &args![map]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn faction_map_destructor_runs_the_base_destructor() {
        let mut e = engine();
        register_bucket_functions(&mut e);
        let map = dirty_map(&mut e);
        e.mem.set_u32(map + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x004388a0, &args![map]);
        assert_eq!(calls(&e, BUCKETS_RELEASE), vec![vec![map]]);
        assert_eq!(e.mem.u32(map), BS_MAP_BASE_VTABLE);
    }

    /// The map scalar deleting destructors: a map with no buckets, so only
    /// the object itself reaches `operator delete`.
    fn check_scalar_deleting_map(addr: u32, releases_nodes: bool) {
        let mut e = engine();
        register_bucket_functions(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(addr, &args![this, 0u32]).u32(), this);
        assert!(e.mem.block_size(this).is_some());
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert_eq!(!calls(&e, BUCKETS_RELEASE).is_empty(), releases_nodes);
        assert_eq!(e.call(addr, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.block_size(this), None);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    #[test]
    fn water_zone_map_scalar_deleting_destructor() {
        check_scalar_deleting_map(0x00438410, true);
    }

    #[test]
    fn faction_map_scalar_deleting_destructor() {
        check_scalar_deleting_map(0x00438440, true);
    }

    #[test]
    fn nitmap_base_scalar_deleting_destructor() {
        check_scalar_deleting_map(0x00438a00, true);
    }

    #[test]
    fn bs_map_base_scalar_deleting_destructor_frees_on_request() {
        check_scalar_deleting_map(0x00438a30, false);
    }

    /// An array scalar deleting destructor: stores the vtable, clears the
    /// array and frees the object only when bit 0 of the flags is set.
    fn check_scalar_deleting_array(addr: u32, vtable: u32) {
        let mut e = engine();
        e.register(SIMPLE_ARRAY_CLEAR, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(addr, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), vtable);
        assert_eq!(calls(&e, SIMPLE_ARRAY_CLEAR), vec![vec![this, 1]]);
        assert!(e.mem.block_size(this).is_some());
        assert_eq!(e.call(addr, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.block_size(this), None);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    #[test]
    fn dismembered_limb_array_scalar_deleting_destructor() {
        check_scalar_deleting_array(0x00438970, DISMEMBERED_LIMB_ARRAY_VTABLE);
    }

    #[test]
    fn combat_time_stamp_array_scalar_deleting_destructor() {
        check_scalar_deleting_array(0x004389a0, COMBAT_TIME_STAMP_ARRAY_VTABLE);
    }

    #[test]
    fn uint_array_scalar_deleting_destructor() {
        check_scalar_deleting_array(0x004389d0, UINT_ARRAY_VTABLE);
    }

    fn check_array_destructor(addr: u32, vtable: u32) {
        let mut e = engine();
        e.register(SIMPLE_ARRAY_CLEAR, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(addr, &args![this]);
        assert_eq!(e.mem.u32(this), vtable);
        assert_eq!(calls(&e, SIMPLE_ARRAY_CLEAR), vec![vec![this, 1]]);
    }

    #[test]
    fn dismembered_limb_array_destructor() {
        check_array_destructor(0x004385a0, DISMEMBERED_LIMB_ARRAY_VTABLE);
    }

    #[test]
    fn combat_time_stamp_array_destructor() {
        check_array_destructor(0x004385f0, COMBAT_TIME_STAMP_ARRAY_VTABLE);
    }

    #[test]
    fn uint_array_destructor() {
        check_array_destructor(0x00438690, UINT_ARRAY_VTABLE);
    }

    fn check_array_constructor(addr: u32, vtable: u32) {
        let mut e = engine();
        e.register(ARRAY_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(addr, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), vtable);
        assert_eq!(calls(&e, ARRAY_CONSTRUCT), vec![vec![this, 0, 0]]);
    }

    #[test]
    fn dismembered_limb_array_constructor() {
        check_array_constructor(0x00438570, DISMEMBERED_LIMB_ARRAY_VTABLE);
    }

    #[test]
    fn uint_array_constructor() {
        check_array_constructor(0x00438660, UINT_ARRAY_VTABLE);
    }

    /// An array object whose vtable slot 4 allocates `count * 4` bytes.
    fn array_with_allocator(e: &mut Engine) -> u32 {
        let allocator = 0x0300_0000;
        e.register(allocator, |e, a| ret(e.mem.alloc(a[1] * 4)));
        e.put_vtable(0x0300_1000, &[0, allocator]);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x0300_1000);
        for word in 1..4 {
            e.mem.set_u32(this + word * 4, 0xdead_0000);
        }
        this
    }

    #[test]
    fn array_body_constructor_reserves_the_larger_count() {
        let mut e = engine();
        e.register(ARRAY_CONSTRUCT_ELEMENTS, |_, _| Ret::default());
        let this = array_with_allocator(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00438a60, &args![this, 2u32, 5u32]);
        let buffer = e.mem.u32(this + 4);
        assert_ne!(buffer, 0);
        assert_eq!(e.mem.u32(this + 8), 5);
        assert_eq!(e.mem.u32(this + 0xc), 5);
        assert_eq!(
            calls(&e, ARRAY_CONSTRUCT_ELEMENTS),
            vec![vec![this, buffer, 5]]
        );
        // A larger reservation than the size.
        let this = array_with_allocator(&mut e);
        e.call(0x00438a60, &args![this, 8u32, 3u32]);
        assert_eq!(e.mem.u32(this + 8), 3);
        assert_eq!(e.mem.u32(this + 0xc), 8);
    }

    #[test]
    fn array_body_constructor_with_nothing_allocates_nothing() {
        let mut e = engine();
        let this = array_with_allocator(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00438a60, &args![this, 0u32, 0u32]);
        assert_eq!(
            (
                e.mem.u32(this + 4),
                e.mem.u32(this + 8),
                e.mem.u32(this + 0xc)
            ),
            (0, 0, 0)
        );
        // The vtable word is not touched.
        assert_eq!(e.mem.u32(this), 0x0300_1000);
    }

    #[test]
    fn combat_time_stamp_array_constructor_stores_vtable_then_builds_empty() {
        let mut e = engine();
        let this = array_with_allocator(&mut e);
        e.call(0x004385c0, &args![this]);
        assert_eq!(e.mem.u32(this), COMBAT_TIME_STAMP_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
    }

    #[test]
    fn array_append_copies_the_word_into_the_new_slot() {
        let mut e = engine();
        e.register(ARRAY_ADD_SLOT, |e, a| {
            // Reallocates on every call so the buffer must be read after it.
            let size = e.mem.u32(a[0] + 8);
            let buffer = e.mem.alloc(16);
            e.mem.set_u32(a[0] + 4, buffer);
            e.mem.set_u32(a[0] + 8, size + 1);
            ret(size)
        });
        e.register(ARRAY_CONSTRUCT_ELEMENTS, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, 2);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0x4242);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00438610, &args![this, value]).u32(), 2);
        let buffer = e.mem.u32(this + 4);
        assert_eq!(e.mem.u32(buffer + 8), 0x4242);
        assert_eq!(calls(&e, ARRAY_CONSTRUCT_ELEMENTS)[0][2], 1);
    }

    fn array_of(e: &mut Engine, words: &[u32]) -> u32 {
        let array = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(4 * words.len().max(1) as u32);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *word);
        }
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 8, words.len() as u32);
        array
    }

    #[test]
    fn compare_buffer_wants_same_size_and_elements() {
        let mut e = engine();
        e.register(SIMPLE_ARRAY_SIZE, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(SIMPLE_ARRAY_AT, |e, a| ret(e.mem.u32(a[0] + 4) + 4 * a[1]));
        let this = array_of(&mut e, &[1, 2, 3]);
        let same = array_of(&mut e, &[1, 2, 3]);
        let shorter = array_of(&mut e, &[1, 2]);
        let different = array_of(&mut e, &[1, 9, 3]);
        let empty = array_of(&mut e, &[]);
        let also_empty = array_of(&mut e, &[]);
        assert!(e.call(0x004387b0, &args![this, same]).bool());
        assert!(!e.call(0x004387b0, &args![this, shorter]).bool());
        assert!(!e.call(0x004387b0, &args![this, different]).bool());
        assert!(e.call(0x004387b0, &args![empty, also_empty]).bool());
    }

    #[test]
    fn map_node_release_clears_the_byte_and_frees() {
        let mut e = engine();
        let node = e.mem.alloc(0x10);
        e.mem.set_u8(node + 8, 1);
        e.call_log = Some(vec![]);
        e.call(0x00438940, &args![0u32, node]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![node]]);
        assert_eq!(e.mem.block_size(node), None);
    }

    /// A `BSStringT`-shaped object: text pointer, length, capacity.
    fn string_object(e: &mut Engine, text: &[u8], length: u16, capacity: u16) -> u32 {
        let object = e.mem.alloc(8);
        let buffer = if capacity == 0 {
            0
        } else {
            let buffer = e.mem.alloc(capacity as u32 + 1);
            e.mem.set_cstr(buffer, text);
            buffer
        };
        e.mem.set_u32(object, buffer);
        e.mem.set_u16(object + 4, length);
        e.mem.set_u16(object + 6, capacity);
        object
    }

    fn register_string_functions(e: &mut Engine) {
        e.register(STRING_LENGTH, |e, a| ret(e.mem.u16(a[0] + 4) as u32));
        e.register(STRING_CAPACITY, |e, a| ret(e.mem.u16(a[0] + 6) as u32));
        e.register(STRING_SET_CAPACITY, |e, a| {
            e.mem.set_u16(a[0] + 6, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_SET_LENGTH, |e, a| {
            e.mem.set_u16(a[0] + 4, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_C_STR, |e, a| ret(e.mem.u32(a[0])));
        e.register(MEM_COPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            ret(a[0])
        });
    }

    #[test]
    fn string_set_grows_the_buffer_when_the_source_is_longer() {
        let mut e = engine();
        register_string_functions(&mut e);
        let this = string_object(&mut e, b"ab", 2, 2);
        let old = e.mem.u32(this);
        let other = string_object(&mut e, b"hello", 5, 5);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00438470, &args![this, other]).bool());
        let buffer = e.mem.u32(this);
        assert_ne!(buffer, old);
        assert_eq!(e.mem.cstr(buffer), b"hello");
        assert_eq!(e.mem.u16(this + 4), 5);
        assert_eq!(e.mem.u16(this + 6), 5);
        assert_eq!(e.mem.block_size(old), None);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![6]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![old]]);
    }

    #[test]
    fn string_set_grows_without_an_old_buffer() {
        let mut e = engine();
        register_string_functions(&mut e);
        let this = string_object(&mut e, b"", 0, 0);
        let other = string_object(&mut e, b"xyz", 3, 3);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00438470, &args![this, other]).bool());
        assert_eq!(e.mem.cstr(e.mem.u32(this)), b"xyz");
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn string_set_copies_over_the_buffer_when_it_fits() {
        let mut e = engine();
        register_string_functions(&mut e);
        let this = string_object(&mut e, b"abcdefgh", 8, 8);
        let buffer = e.mem.u32(this);
        let other = string_object(&mut e, b"hi", 2, 2);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00438470, &args![this, other]).bool());
        assert_eq!(e.mem.u32(this), buffer);
        assert_eq!(e.mem.cstr(buffer), b"hi");
        assert_eq!(e.mem.u16(this + 4), 2);
        // The capacity is left alone.
        assert_eq!(e.mem.u16(this + 6), 8);
        assert!(calls(&e, OPERATOR_NEW).is_empty());
        assert!(calls(&e, STRING_SET_CAPACITY).is_empty());
    }

    #[test]
    fn string_set_with_an_empty_source_frees_and_nulls() {
        let mut e = engine();
        register_string_functions(&mut e);
        let this = string_object(&mut e, b"abc", 3, 3);
        let buffer = e.mem.u32(this);
        let other = string_object(&mut e, b"", 0, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00438470, &args![this, other]).bool());
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u16(this + 4), 0);
        assert_eq!(e.mem.u16(this + 6), 0);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![buffer]]);
        // No buffer to free the second time.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00438470, &args![this, other]).bool());
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
    }
}
