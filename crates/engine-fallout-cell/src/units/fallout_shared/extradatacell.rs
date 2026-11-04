//! `fallout shared/extradatacell.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0040ed00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCell3D` scalar deleting destructor (engine map name): runs the destructor body `0040ed30` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn extra_cell3d_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0040_ed30, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0040ee40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraHavok` scalar deleting destructor (engine map name): runs the destructor body `0040ee70` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn extra_havok_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0040_ee70, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0040f1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCellImpactSwap` scalar deleting destructor (engine map name): runs the destructor body `0040f210` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn extra_cell_impact_swap_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0040_f210, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0040f4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCellCanopyShadowMask` scalar deleting destructor (engine map name): runs the destructor body `0040f500` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn extra_cell_canopy_shadow_mask_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x0040_f500, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0040f5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraSeenData` scalar deleting destructor (engine map name): runs the destructor body `0040f5f0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn extra_seen_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0040_f5f0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `0040ec80` (`ECX` = extra data, one word: the type; `RET 4`): the
/// `BSExtraData` base constructor.
const EXTRA_DATA_CONSTRUCT: u32 = 0x0040_ec80;
/// `0040ecb0` (`ECX` = extra data): the `BSExtraData` base destructor.
const EXTRA_DATA_DESTRUCT: u32 = 0x0040_ecb0;
/// `00633c90` (`ECX` = pointer slot, one word): constructs a smart pointer
/// slot with the value given.
const SMART_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `0066b0d0` (`ECX` = pointer slot, one word): assigns the slot.
const SMART_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `0045cec0` (`ECX` = pointer slot): the slot's destructor.
const SMART_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// The vtables these functions store; each one's class is the RTTI name read
/// through its locator (the word before the vtable).
const EXTRA_HAVOK_VTABLE: u32 = 0x0101_421c;
const EXTRA_REGION_LIST_VTABLE: u32 = 0x0101_4228;
const EXTRA_CELL_MUSIC_TYPE_VTABLE: u32 = 0x0101_4234;
const EXTRA_CELL_ACOUSTIC_SPACE_VTABLE: u32 = 0x0101_4240;
const EXTRA_CELL_CLIMATE_VTABLE: u32 = 0x0101_424c;
const EXTRA_CELL_IMAGE_SPACE_VTABLE: u32 = 0x0101_4258;
const EXTRA_CELL_WATER_TYPE_VTABLE: u32 = 0x0101_4270;
const EXTRA_PROCESS_MIDDLE_LOW_VTABLE: u32 = 0x0101_427c;
const EXTRA_SEEN_DATA_VTABLE: u32 = 0x0101_4294;

/// The shape every one-field extra data constructor here has: the base
/// constructor with the type, the class vtable, then the field at +0xc
/// (`ExtraRegionList::pList`, `ExtraCellMusicType::pType`, ... in the Xbox
/// PDB). Returns `this`.
fn construct_one_field(e: &mut Engine, this: Ptr, kind: u32, vtable: u32, field: u32) -> Ptr {
    e.call(EXTRA_DATA_CONSTRUCT, &args![this, kind]);
    e.mem.set_u32(this.addr(), vtable);
    e.mem.set_u32(this.addr() + 0xc, field);
    this
}

// Translated from 0040eda0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040eda0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs an `ExtraHavok` (vtable `0101421c`, RTTI; type 1; `RET 4`, one
/// word: the world): the base constructor, the vtable, the two smart pointer
/// slots `spWorld` (+0xc) and `spCellMopp` (+0x10, Xbox PDB) constructed
/// empty (`00633c90` with 0), then `spWorld` assigned the world and
/// `spCellMopp` assigned 0 (`0066b0d0`). Returns `this`. The engine map
/// gives it no name.
pub fn fn_0040eda0(e: &mut Engine, this: Ptr, world: u32) -> Ptr {
    let base = this.addr();
    e.call(EXTRA_DATA_CONSTRUCT, &args![this, 1u32]);
    e.mem.set_u32(base, EXTRA_HAVOK_VTABLE);
    e.call(SMART_POINTER_CONSTRUCT, &args![base + 0xc, 0u32]);
    e.call(SMART_POINTER_CONSTRUCT, &args![base + 0x10, 0u32]);
    e.call(SMART_POINTER_ASSIGN, &args![base + 0xc, world]);
    e.call(SMART_POINTER_ASSIGN, &args![base + 0x10, 0u32]);
    this
}

// Translated from 0040ee70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040ee70 - the exception-unwinding frame (FS:[0]) is not translated.
/// `ExtraHavok`'s destructor body: puts back the vtable, assigns 0 to
/// `spCellMopp` (+0x10) and then `spWorld` (+0xc), destroys the two slots
/// (+0x10 first, `0045cec0`), then the `BSExtraData` base (`0040ecb0`). The
/// engine map gives it no name.
pub fn fn_0040ee70(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, EXTRA_HAVOK_VTABLE);
    e.call(SMART_POINTER_ASSIGN, &args![base + 0x10, 0u32]);
    e.call(SMART_POINTER_ASSIGN, &args![base + 0xc, 0u32]);
    e.call(SMART_POINTER_DESTRUCT, &args![base + 0x10]);
    e.call(SMART_POINTER_DESTRUCT, &args![base + 0xc]);
    e.call(EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 0040ef00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs an `ExtraRegionList` (vtable `01014228`, RTTI; type 4) holding
/// the list given (`pList`, +0xc). Returns `this`. The engine map gives it
/// no name.
pub fn fn_0040ef00(e: &mut Engine, this: Ptr, list: u32) -> Ptr {
    construct_one_field(e, this, 4, EXTRA_REGION_LIST_VTABLE, list)
}

// Translated from 0040ef30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs an `ExtraCellMusicType` (vtable `01014234`, RTTI; type 7)
/// holding the music type given (`pType`, +0xc). Returns `this`. The engine
/// map gives it no name.
pub fn fn_0040ef30(e: &mut Engine, this: Ptr, music_type: u32) -> Ptr {
    construct_one_field(e, this, 7, EXTRA_CELL_MUSIC_TYPE_VTABLE, music_type)
}

// Translated from 0040efd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs an `ExtraCellAcousticSpace` (vtable `01014240`, RTTI; type
/// 0x81) holding the acoustic space given (`pSpace`, +0xc). Returns `this`.
/// The engine map gives it no name.
pub fn fn_0040efd0(e: &mut Engine, this: Ptr, space: u32) -> Ptr {
    construct_one_field(e, this, 0x81, EXTRA_CELL_ACOUSTIC_SPACE_VTABLE, space)
}

// Translated from 0040f070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs an `ExtraCellClimate` (vtable `0101424c`, RTTI; type 8)
/// holding the climate given (`pClimate`, +0xc). Returns `this`. The engine
/// map gives it no name.
pub fn fn_0040f070(e: &mut Engine, this: Ptr, climate: u32) -> Ptr {
    construct_one_field(e, this, 8, EXTRA_CELL_CLIMATE_VTABLE, climate)
}

// Translated from 0040f110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs an `ExtraCellImageSpace` (vtable `01014258`, RTTI; type 0x59)
/// holding the image space given (`pImageSpace`, +0xc). Returns `this`. The
/// engine map gives it no name.
pub fn fn_0040f110(e: &mut Engine, this: Ptr, image_space: u32) -> Ptr {
    construct_one_field(e, this, 0x59, EXTRA_CELL_IMAGE_SPACE_VTABLE, image_space)
}

// Translated from 0040f370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs an `ExtraCellWaterType` (vtable `01014270`, RTTI; type 3)
/// with no water (`pWater`, +0xc, zero). Returns `this`. The engine map
/// gives it no name.
pub fn fn_0040f370(e: &mut Engine, this: Ptr) -> Ptr {
    construct_one_field(e, this, 3, EXTRA_CELL_WATER_TYPE_VTABLE, 0)
}

// Translated from 0040f410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs an `ExtraProcessMiddleLow` (vtable `0101427c`, RTTI; type 9)
/// with `iRefCount` (+0xc) zero. Returns `this`. The engine map gives it no
/// name.
pub fn fn_0040f410(e: &mut Engine, this: Ptr) -> Ptr {
    construct_one_field(e, this, 9, EXTRA_PROCESS_MIDDLE_LOW_VTABLE, 0)
}

// Translated from 0040f590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs an `ExtraSeenData` (vtable `01014294`, RTTI; type 5) with no
/// seen data (`pSeenData`, +0xc, zero). Returns `this`. The engine map gives
/// it no name.
pub fn fn_0040f590(e: &mut Engine, this: Ptr) -> Ptr {
    construct_one_field(e, this, 5, EXTRA_SEEN_DATA_VTABLE, 0)
}

// Translated from 0040f5f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040f5f0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `ExtraSeenData`'s destructor body: puts back the vtable, deletes the
/// seen data (`pSeenData`, +0xc) when there is one through its virtual slot
/// 0 with 1 (its deleting destructor), then the `BSExtraData` base
/// (`0040ecb0`). The engine map gives it no name.
pub fn fn_0040f5f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, EXTRA_SEEN_DATA_VTABLE);
    let seen = e.mem.u32(base + 0xc);
    if seen != 0 {
        e.vcall(seen, 0, &args![1u32]);
    }
    e.call(EXTRA_DATA_DESTRUCT, &args![this]);
}

/// `ExtraCell3D`'s vtable (`01014204`; RTTI `ExtraCell3D`, read through its
/// locator).
const EXTRA_CELL_3D_VTABLE: u32 = 0x0101_4204;
/// `ExtraCellImpactSwap`'s vtable (`01014264`; RTTI `ExtraCellImpactSwap`).
const EXTRA_CELL_IMPACT_SWAP_VTABLE: u32 = 0x0101_4264;
/// `ExtraCellCanopyShadowMask`'s vtable (`01014288`; RTTI
/// `ExtraCellCanopyShadowMask`).
const EXTRA_CELL_CANOPY_SHADOW_MASK_VTABLE: u32 = 0x0101_4288;
/// `__RTDynamicCast(object, vfDelta, sourceType, targetType, isReference)`
/// (`cdecl`, five words).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `BSExtraData::Compare` (`0040f700`, `thiscall`, `RET 4`): true when the
/// two base parts differ (`AL`).
const BS_EXTRA_DATA_COMPARE: u32 = 0x0040_f700;
/// `ImpactSwap::Compare` (engine map, `0058f5c0`, `thiscall`, `RET 4`).
const IMPACT_SWAP_COMPARE: u32 = 0x0058_f5c0;
/// `0058f010` (`thiscall`, no stack word; unit `bgsimpactdataset.cpp` by the
/// engine map): the impact swap's destructor body.
const IMPACT_SWAP_DESTRUCT: u32 = 0x0058_f010;
/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// RTTI type descriptors (`.?AV<Class>@@` at +8, read from the exe): the
/// source type `BSExtraData` and the target types of the `Compare`s.
const BS_EXTRA_DATA_TYPE: u32 = 0x0118_3b2c;
const EXTRA_CELL_MUSIC_TYPE_TYPE: u32 = 0x0118_3b84;
const EXTRA_CELL_ACOUSTIC_SPACE_TYPE: u32 = 0x0118_3ba8;
const EXTRA_CELL_CLIMATE_TYPE: u32 = 0x0118_3bd0;
const EXTRA_CELL_IMAGE_SPACE_TYPE: u32 = 0x0118_3bf0;
const EXTRA_CELL_IMPACT_SWAP_TYPE: u32 = 0x0118_3c14;
const EXTRA_CELL_WATER_TYPE_TYPE: u32 = 0x0118_3c38;

/// `dynamic_cast` of `other` from `BSExtraData` to the type at `target`.
fn cast_extra_data(e: &mut Engine, other: Ptr, target: u32) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![other, 0u32, BS_EXTRA_DATA_TYPE, target, 0u32],
    )
    .u32()
}

/// The `Compare` shape of the one-field cell extra data: true (they differ)
/// when `other` is not of the class, when the base parts differ
/// (`0040f700`), or when the words at +0xc differ.
fn compare_one_field(e: &mut Engine, this: Ptr, other: Ptr, target: u32) -> bool {
    let cast = cast_extra_data(e, other, target);
    if cast == 0 {
        return true;
    }
    if e.call(BS_EXTRA_DATA_COMPARE, &args![this, other]).bool() {
        return true;
    }
    e.mem.u32(cast + 0xc) != e.mem.u32(this.addr() + 0xc)
}

// Translated from 0040ed30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040ed30 - the exception-unwinding frame (FS:[0]) is not translated.
/// `ExtraCell3D`'s destructor body (the engine map gives it no name): puts
/// back the vtable `01014204`, assigns 0 to the smart pointer slot at +0xc
/// (`0066b0d0`), destroys the slot (`0045cec0`), then the `BSExtraData`
/// base (`0040ecb0`).
pub fn fn_0040ed30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, EXTRA_CELL_3D_VTABLE);
    e.call(SMART_POINTER_ASSIGN, &args![base + 0xc, 0u32]);
    e.call(SMART_POINTER_DESTRUCT, &args![base + 0xc]);
    e.call(EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 0040ef60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCellMusicType::Compare` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`): true when `other` is not an `ExtraCellMusicType`,
/// its base part differs, or its music type (+0xc) differs.
pub fn extra_cell_music_type_compare(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    compare_one_field(e, this, other, EXTRA_CELL_MUSIC_TYPE_TYPE)
}

// Translated from 0040f000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCellAcousticSpace::Compare` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`): true when `other` is not an
/// `ExtraCellAcousticSpace`, its base part differs, or its space (+0xc)
/// differs.
pub fn extra_cell_acoustic_space_compare(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    compare_one_field(e, this, other, EXTRA_CELL_ACOUSTIC_SPACE_TYPE)
}

// Translated from 0040f0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCellClimate::Compare` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`): true when `other` is not an `ExtraCellClimate`, its
/// base part differs, or its climate (+0xc) differs.
pub fn extra_cell_climate_compare(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    compare_one_field(e, this, other, EXTRA_CELL_CLIMATE_TYPE)
}

// Translated from 0040f140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCellImageSpace::Compare` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`): true when `other` is not an `ExtraCellImageSpace`,
/// its base part differs, or its image space (+0xc) differs.
pub fn extra_cell_image_space_compare(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    compare_one_field(e, this, other, EXTRA_CELL_IMAGE_SPACE_TYPE)
}

// Translated from 0040f1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs an `ExtraCellImpactSwap` (vtable `01014264`, RTTI; type 0x8c)
/// with no swap (+0xc zero). Returns `this`. The engine map gives it no
/// name.
pub fn fn_0040f1b0(e: &mut Engine, this: Ptr) -> Ptr {
    construct_one_field(e, this, 0x8c, EXTRA_CELL_IMPACT_SWAP_VTABLE, 0)
}

// Translated from 0040f210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040f210 - the exception-unwinding frame (FS:[0]) is not translated.
/// `ExtraCellImpactSwap`'s destructor body (the engine map gives it no
/// name): puts back the vtable `01014264`, deletes the impact swap at +0xc
/// when there is one (`0040f2a0` with 1), then the `BSExtraData` base
/// (`0040ecb0`).
pub fn fn_0040f210(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, EXTRA_CELL_IMPACT_SWAP_VTABLE);
    let swap = e.mem.u32(base + 0xc);
    if swap != 0 {
        fn_0040f2a0(e, Ptr::new(swap), 1);
    }
    e.call(EXTRA_DATA_DESTRUCT, &args![this]);
}

// Translated from 0040f2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The impact swap's scalar deleting destructor (the engine map gives it no
/// name; `thiscall`, `RET 4`): its destructor body `0058f010`, then
/// `operator delete` on `this` for bit 0 of `flags`. Returns `this`.
pub fn fn_0040f2a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(IMPACT_SWAP_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0040f2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCellImpactSwap::Compare` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`): true when `other` is not an `ExtraCellImpactSwap`
/// or its base part differs; with both swaps (+0xc) set, whether
/// `ImpactSwap::Compare` (`0058f5c0`, on `other`'s swap with this one's)
/// says they differ; otherwise true when exactly one is set.
pub fn extra_cell_impact_swap_compare(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let cast = cast_extra_data(e, other, EXTRA_CELL_IMPACT_SWAP_TYPE);
    if cast == 0 {
        return true;
    }
    if e.call(BS_EXTRA_DATA_COMPARE, &args![this, other]).bool() {
        return true;
    }
    let theirs = e.mem.u32(cast + 0xc);
    let mine = e.mem.u32(this.addr() + 0xc);
    if theirs != 0 && mine != 0 {
        e.call(IMPACT_SWAP_COMPARE, &args![theirs, mine]).bool()
    } else {
        theirs != 0 || mine != 0
    }
}

// Translated from 0040f3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraCellWaterType::Compare` (Xbox PDB name in the engine map;
/// `thiscall`, `RET 4`): true when `other` is not an `ExtraCellWaterType`,
/// its base part differs, or its water (+0xc) differs.
pub fn extra_cell_water_type_compare(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    compare_one_field(e, this, other, EXTRA_CELL_WATER_TYPE_TYPE)
}

// Translated from 0040f440 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040f440 - the exception-unwinding frame (FS:[0]) is not translated.
/// Constructs an `ExtraCellCanopyShadowMask` (vtable `01014288`, RTTI; type
/// 0xa; `RET 8`): the base constructor, the vtable, the smart pointer slot
/// at +0x10 constructed empty (`00633c90` with 0), `first` at +0xc, the slot
/// assigned `second` (`0066b0d0`), and +0x18 cleared. Returns `this`. The
/// engine map gives it no name.
pub fn fn_0040f440(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    let base = this.addr();
    e.call(EXTRA_DATA_CONSTRUCT, &args![this, 0xau32]);
    e.mem.set_u32(base, EXTRA_CELL_CANOPY_SHADOW_MASK_VTABLE);
    e.call(SMART_POINTER_CONSTRUCT, &args![base + 0x10, 0u32]);
    e.mem.set_u32(base + 0xc, first);
    e.call(SMART_POINTER_ASSIGN, &args![base + 0x10, second]);
    e.mem.set_u32(base + 0x18, 0);
    this
}

// Translated from 0040f500 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040f500 - the exception-unwinding frame (FS:[0]) is not translated.
/// `ExtraCellCanopyShadowMask`'s destructor body (the engine map gives it
/// no name): puts back the vtable `01014288`, clears +0xc, assigns 0 to the
/// slot at +0x10 (`0066b0d0`), clears +0x18, destroys the slot (`0045cec0`),
/// then the `BSExtraData` base (`0040ecb0`).
pub fn fn_0040f500(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, EXTRA_CELL_CANOPY_SHADOW_MASK_VTABLE);
    e.mem.set_u32(base + 0xc, 0);
    e.call(SMART_POINTER_ASSIGN, &args![base + 0x10, 0u32]);
    e.mem.set_u32(base + 0x18, 0);
    e.call(SMART_POINTER_DESTRUCT, &args![base + 0x10]);
    e.call(EXTRA_DATA_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_ed30, fn_0040ed30(Ptr)),
        entry!(0x0040_ef60, extra_cell_music_type_compare(Ptr, Ptr) -> bool),
        entry!(0x0040_f000, extra_cell_acoustic_space_compare(Ptr, Ptr) -> bool),
        entry!(0x0040_f0a0, extra_cell_climate_compare(Ptr, Ptr) -> bool),
        entry!(0x0040_f140, extra_cell_image_space_compare(Ptr, Ptr) -> bool),
        entry!(0x0040_f1b0, fn_0040f1b0(Ptr) -> Ptr),
        entry!(0x0040_f210, fn_0040f210(Ptr)),
        entry!(0x0040_f2a0, fn_0040f2a0(Ptr, u32) -> Ptr),
        entry!(0x0040_f2d0, extra_cell_impact_swap_compare(Ptr, Ptr) -> bool),
        entry!(0x0040_f3a0, extra_cell_water_type_compare(Ptr, Ptr) -> bool),
        entry!(0x0040_f440, fn_0040f440(Ptr, u32, u32) -> Ptr),
        entry!(0x0040_f500, fn_0040f500(Ptr)),
        entry!(0x0040_ed00, extra_cell3d_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0040_ee40, extra_havok_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0040_f1e0, extra_cell_impact_swap_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0040_f4d0, extra_cell_canopy_shadow_mask_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0040_f5c0, extra_seen_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0040_eda0, fn_0040eda0(Ptr, u32) -> Ptr),
        entry!(0x0040_ee70, fn_0040ee70(Ptr)),
        entry!(0x0040_ef00, fn_0040ef00(Ptr, u32) -> Ptr),
        entry!(0x0040_ef30, fn_0040ef30(Ptr, u32) -> Ptr),
        entry!(0x0040_efd0, fn_0040efd0(Ptr, u32) -> Ptr),
        entry!(0x0040_f070, fn_0040f070(Ptr, u32) -> Ptr),
        entry!(0x0040_f110, fn_0040f110(Ptr, u32) -> Ptr),
        entry!(0x0040_f370, fn_0040f370(Ptr) -> Ptr),
        entry!(0x0040_f410, fn_0040f410(Ptr) -> Ptr),
        entry!(0x0040_f590, fn_0040f590(Ptr) -> Ptr),
        entry!(0x0040_f5f0, fn_0040f5f0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_0040ed00_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_ed30, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_ed00, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_ed00)
                .collect();
            let mut expected = vec![(0x0040_ed30, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0040ee40_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_ee70, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_ee40, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_ee40)
                .collect();
            let mut expected = vec![(0x0040_ee70, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0040f1e0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_f210, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_f1e0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_f1e0)
                .collect();
            let mut expected = vec![(0x0040_f210, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0040f4d0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_f500, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_f4d0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_f4d0)
                .collect();
            let mut expected = vec![(0x0040_f500, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_0040f5c0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_f5f0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_f5c0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_f5c0)
                .collect();
            let mut expected = vec![(0x0040_f5f0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b027_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// The calls a run made, without the top-level call.
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

    fn engine() -> Engine {
        let mut e = Engine::new();
        for addr in [
            EXTRA_DATA_CONSTRUCT,
            EXTRA_DATA_DESTRUCT,
            SMART_POINTER_CONSTRUCT,
            SMART_POINTER_ASSIGN,
            SMART_POINTER_DESTRUCT,
        ] {
            e.register(addr, |_, _| ret(0));
        }
        e
    }

    #[test]
    fn at_0040eda0_builds_an_extra_havok() {
        let mut e = engine();
        let this = e.mem.alloc(0x14);
        let (result, log) = run(&mut e, 0x0040_eda0, &[this, 0x5000]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), EXTRA_HAVOK_VTABLE);
        assert_eq!(
            log,
            vec![
                (EXTRA_DATA_CONSTRUCT, vec![this, 1]),
                (SMART_POINTER_CONSTRUCT, vec![this + 0xc, 0]),
                (SMART_POINTER_CONSTRUCT, vec![this + 0x10, 0]),
                (SMART_POINTER_ASSIGN, vec![this + 0xc, 0x5000]),
                (SMART_POINTER_ASSIGN, vec![this + 0x10, 0]),
            ]
        );
    }

    #[test]
    fn at_0040ee70_releases_both_pointers_then_the_base() {
        let mut e = engine();
        let this = e.mem.alloc(0x14);
        let (_, log) = run(&mut e, 0x0040_ee70, &[this]);
        assert_eq!(e.mem.u32(this), EXTRA_HAVOK_VTABLE);
        assert_eq!(
            log,
            vec![
                (SMART_POINTER_ASSIGN, vec![this + 0x10, 0]),
                (SMART_POINTER_ASSIGN, vec![this + 0xc, 0]),
                (SMART_POINTER_DESTRUCT, vec![this + 0x10]),
                (SMART_POINTER_DESTRUCT, vec![this + 0xc]),
                (EXTRA_DATA_DESTRUCT, vec![this]),
            ]
        );
    }

    #[test]
    fn one_field_constructors_store_type_vtable_and_field() {
        let cases: [(u32, u32, u32, Option<u32>); 8] = [
            (0x0040_ef00, 4, EXTRA_REGION_LIST_VTABLE, Some(0x5100)),
            (0x0040_ef30, 7, EXTRA_CELL_MUSIC_TYPE_VTABLE, Some(0x5200)),
            (
                0x0040_efd0,
                0x81,
                EXTRA_CELL_ACOUSTIC_SPACE_VTABLE,
                Some(0x5300),
            ),
            (0x0040_f070, 8, EXTRA_CELL_CLIMATE_VTABLE, Some(0x5400)),
            (
                0x0040_f110,
                0x59,
                EXTRA_CELL_IMAGE_SPACE_VTABLE,
                Some(0x5500),
            ),
            (0x0040_f370, 3, EXTRA_CELL_WATER_TYPE_VTABLE, None),
            (0x0040_f410, 9, EXTRA_PROCESS_MIDDLE_LOW_VTABLE, None),
            (0x0040_f590, 5, EXTRA_SEEN_DATA_VTABLE, None),
        ];
        for (addr, kind, vtable, field) in cases {
            let mut e = engine();
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this + 0xc, 0xffff_ffff);
            let mut words = vec![this];
            words.extend(field);
            let (result, log) = run(&mut e, addr, &words);
            assert_eq!(result.u32(), this);
            assert_eq!(log, vec![(EXTRA_DATA_CONSTRUCT, vec![this, kind])]);
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(e.mem.u32(this + 0xc), field.unwrap_or(0));
        }
    }

    #[test]
    fn at_0040f5f0_deletes_the_seen_data_when_there_is_one() {
        let mut e = engine();
        e.register(0x0f00_1000, |_, _| ret(0));
        e.put_vtable(0x0f00_0000, &[0x0f00_1000]);
        let seen = e.mem.alloc(8);
        e.mem.set_u32(seen, 0x0f00_0000);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, seen);
        let (_, log) = run(&mut e, 0x0040_f5f0, &[this]);
        assert_eq!(e.mem.u32(this), EXTRA_SEEN_DATA_VTABLE);
        assert_eq!(
            log,
            vec![
                (0x0f00_1000, vec![seen, 1]),
                (EXTRA_DATA_DESTRUCT, vec![this]),
            ]
        );
        e.mem.set_u32(this + 0xc, 0);
        let (_, log) = run(&mut e, 0x0040_f5f0, &[this]);
        assert_eq!(log, vec![(EXTRA_DATA_DESTRUCT, vec![this])]);
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// The calls a run made, without the top-level call.
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

    /// Doubles for every callee; the cast answers the word at 0x0f00_0000
    /// (set per case), the base compare the word at 0x0f00_0004, the swap
    /// compare the word at 0x0f00_0008.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for addr in [
            EXTRA_DATA_CONSTRUCT,
            EXTRA_DATA_DESTRUCT,
            SMART_POINTER_CONSTRUCT,
            SMART_POINTER_ASSIGN,
            SMART_POINTER_DESTRUCT,
            IMPACT_SWAP_DESTRUCT,
            OPERATOR_DELETE,
        ] {
            e.register(addr, |_, _| ret(0));
        }
        e.mem.map(0x0f00_0000, 0x10);
        e.register(DYNAMIC_CAST, |e, _| ret(e.mem.u32(0x0f00_0000)));
        e.register(BS_EXTRA_DATA_COMPARE, |e, _| ret(e.mem.u32(0x0f00_0004)));
        e.register(IMPACT_SWAP_COMPARE, |e, _| ret(e.mem.u32(0x0f00_0008)));
        e
    }

    #[test]
    fn at_0040ed30_releases_the_pointer_then_the_base() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        let (_, log) = run(&mut e, 0x0040_ed30, &[this]);
        assert_eq!(e.mem.u32(this), EXTRA_CELL_3D_VTABLE);
        assert_eq!(
            log,
            vec![
                (SMART_POINTER_ASSIGN, vec![this + 0xc, 0]),
                (SMART_POINTER_DESTRUCT, vec![this + 0xc]),
                (EXTRA_DATA_DESTRUCT, vec![this]),
            ]
        );
    }

    #[test]
    fn one_field_compares_check_class_base_and_field() {
        let cases = [
            (0x0040_ef60, EXTRA_CELL_MUSIC_TYPE_TYPE),
            (0x0040_f000, EXTRA_CELL_ACOUSTIC_SPACE_TYPE),
            (0x0040_f0a0, EXTRA_CELL_CLIMATE_TYPE),
            (0x0040_f140, EXTRA_CELL_IMAGE_SPACE_TYPE),
            (0x0040_f3a0, EXTRA_CELL_WATER_TYPE_TYPE),
        ];
        for (addr, target) in cases {
            let mut e = engine();
            let this = e.mem.alloc(0x10);
            let other = e.mem.alloc(0x10);
            e.mem.set_u32(this + 0xc, 7);
            e.mem.set_u32(other + 0xc, 7);
            // Not of the class.
            let (result, log) = run(&mut e, addr, &[this, other]);
            assert!(result.bool());
            assert_eq!(
                log,
                vec![(DYNAMIC_CAST, vec![other, 0, BS_EXTRA_DATA_TYPE, target, 0])]
            );
            e.mem.set_u32(0x0f00_0000, other);
            // Same field.
            let (result, log) = run(&mut e, addr, &[this, other]);
            assert!(!result.bool());
            assert_eq!(log[1], (BS_EXTRA_DATA_COMPARE, vec![this, other]));
            // Field differs.
            e.mem.set_u32(other + 0xc, 8);
            assert!(run(&mut e, addr, &[this, other]).0.bool());
            // Base differs.
            e.mem.set_u32(other + 0xc, 7);
            e.mem.set_u32(0x0f00_0004, 1);
            assert!(run(&mut e, addr, &[this, other]).0.bool());
        }
    }

    #[test]
    fn at_0040f1b0_builds_an_impact_swap_holder() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, 0xffff_ffff);
        let (result, log) = run(&mut e, 0x0040_f1b0, &[this]);
        assert_eq!(result.u32(), this);
        assert_eq!(log, vec![(EXTRA_DATA_CONSTRUCT, vec![this, 0x8c])]);
        assert_eq!(e.mem.u32(this), EXTRA_CELL_IMPACT_SWAP_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0);
    }

    #[test]
    fn at_0040f210_deletes_the_swap_when_there_is_one() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        let swap = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xc, swap);
        let (_, log) = run(&mut e, 0x0040_f210, &[this]);
        assert_eq!(e.mem.u32(this), EXTRA_CELL_IMPACT_SWAP_VTABLE);
        assert_eq!(
            log,
            vec![
                (IMPACT_SWAP_DESTRUCT, vec![swap]),
                (OPERATOR_DELETE, vec![swap]),
                (EXTRA_DATA_DESTRUCT, vec![this]),
            ]
        );
        e.mem.set_u32(this + 0xc, 0);
        let (_, log) = run(&mut e, 0x0040_f210, &[this]);
        assert_eq!(log, vec![(EXTRA_DATA_DESTRUCT, vec![this])]);
    }

    #[test]
    fn at_0040f2a0_destroys_and_deletes_on_bit_zero() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (3, true)] {
            let (result, log) = run(&mut e, 0x0040_f2a0, &[this, flags]);
            assert_eq!(result.u32(), this);
            let mut expected = vec![(IMPACT_SWAP_DESTRUCT, vec![this])];
            if deletes {
                expected.push((OPERATOR_DELETE, vec![this]));
            }
            assert_eq!(log, expected);
        }
    }

    #[test]
    fn at_0040f2d0_compares_the_swaps() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        let other = e.mem.alloc(0x10);
        assert!(run(&mut e, 0x0040_f2d0, &[this, other]).0.bool());
        e.mem.set_u32(0x0f00_0000, other);
        // Neither has a swap.
        assert!(!run(&mut e, 0x0040_f2d0, &[this, other]).0.bool());
        // Only one has.
        e.mem.set_u32(this + 0xc, 0x100);
        assert!(run(&mut e, 0x0040_f2d0, &[this, other]).0.bool());
        e.mem.set_u32(this + 0xc, 0);
        e.mem.set_u32(other + 0xc, 0x200);
        assert!(run(&mut e, 0x0040_f2d0, &[this, other]).0.bool());
        // Both: the swap compare decides, on the other swap with this one.
        e.mem.set_u32(this + 0xc, 0x100);
        let (result, log) = run(&mut e, 0x0040_f2d0, &[this, other]);
        assert!(!result.bool());
        assert_eq!(log[2], (IMPACT_SWAP_COMPARE, vec![0x200, 0x100]));
        e.mem.set_u32(0x0f00_0008, 1);
        assert!(run(&mut e, 0x0040_f2d0, &[this, other]).0.bool());
        // Base differs.
        e.mem.set_u32(0x0f00_0004, 1);
        let (result, log) = run(&mut e, 0x0040_f2d0, &[this, other]);
        assert!(result.bool());
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn at_0040f440_builds_a_canopy_shadow_mask() {
        let mut e = engine();
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0x18, 0xffff_ffff);
        let (result, log) = run(&mut e, 0x0040_f440, &[this, 0x1234, 0x5678]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), EXTRA_CELL_CANOPY_SHADOW_MASK_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0x1234);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(
            log,
            vec![
                (EXTRA_DATA_CONSTRUCT, vec![this, 0xa]),
                (SMART_POINTER_CONSTRUCT, vec![this + 0x10, 0]),
                (SMART_POINTER_ASSIGN, vec![this + 0x10, 0x5678]),
            ]
        );
    }

    #[test]
    fn at_0040f500_clears_and_releases() {
        let mut e = engine();
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 0xc, 5);
        e.mem.set_u32(this + 0x18, 6);
        let (_, log) = run(&mut e, 0x0040_f500, &[this]);
        assert_eq!(e.mem.u32(this), EXTRA_CELL_CANOPY_SHADOW_MASK_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(
            log,
            vec![
                (SMART_POINTER_ASSIGN, vec![this + 0x10, 0]),
                (SMART_POINTER_DESTRUCT, vec![this + 0x10]),
                (EXTRA_DATA_DESTRUCT, vec![this]),
            ]
        );
    }
}
