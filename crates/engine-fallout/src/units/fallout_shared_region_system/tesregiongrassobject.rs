//! `fallout shared/region system/tesregiongrassobject.cpp` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f55d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionGrassObject` scalar deleting destructor (engine map name): runs the destructor body `004f5920` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn tes_region_grass_object_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(0x004f_5920, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `TESRegionGrassObject`'s vtable (RTTI `.?AVTESRegionGrassObject@@`).
const TES_REGION_GRASS_OBJECT_VTABLE: u32 = 0x0102_3ec4;
/// `TESRegionObjectBase`'s vtable (RTTI `.?AVTESRegionObjectBase@@`).
const TES_REGION_OBJECT_BASE_VTABLE: u32 = 0x0102_3ee4;
/// The `TESDataHandler` singleton pointer.
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// `00740940` (unnamed in the engine map; no stack word): the word at
/// +0x624 of the data handler (an object whose vtable slot 0 gives the form
/// being loaded, the region the messages name).
const LOADING_HOLDER_00740940: u32 = 0x0074_0940;
/// `00474cb0` (unnamed in the engine map; no stack word): asked of that
/// form whether its name (slot +0x130) may be used.
const NAME_GUARD_00474CB0: u32 = 0x0047_4cb0;
/// The form's text slot (byte offset +0x130) the messages print.
const SLOT_FORM_TEXT: u32 = 0x130;
/// `TESForm::GetFile` (`00484e60`, Xbox PDB; `thiscall`, one word).
const TES_FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (`00485d50`, Xbox PDB; cdecl, the address of
/// a form id and a file).
const TES_FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (unnamed in the engine map; cdecl, the id): the form of an id.
const LOOKUP_FORM_004839C0: u32 = 0x0048_39c0;
/// `00401170` (unnamed in the engine map; no stack word): a form's type byte (+4).
const FORM_TYPE_00401170: u32 = 0x0040_1170;
/// `0084e3a0` (unnamed in the engine map; no stack word): the word at +0xC
/// of a form (`TESForm`'s form id on PC).
const FORM_ID_0084E3A0: u32 = 0x0084_e3a0;
/// `005b5e40` (unnamed in the engine map; cdecl, format and arguments): prints a warning.
const PRINT_WARNING_005B5E40: u32 = 0x005b_5e40;
/// `""`.
const EMPTY_TEXT: u32 = 0x0101_1584;
/// `"Instantiating a Grass region object for region %s (%d) with a non-Grass object."`.
const NON_GRASS_IN_REGION: u32 = 0x0102_4008;
/// `"Instantiating a Grass region object with a non-Grass object."`.
const NON_GRASS: u32 = 0x0102_3fc4;
/// `"Instantiating a Grass region object for region %s (%d) without an object."`.
const NO_OBJECT_IN_REGION: u32 = 0x0102_3f78;
/// `"Instantiating a Grass region object without an object."`.
const NO_OBJECT: u32 = 0x0102_3f40;
/// `"Instantiating a Grass region object with a parent that isn't a land texture."`.
const PARENT_NOT_LAND_TEXTURE: u32 = 0x0102_3ef0;
/// The type byte the constructor accepts for the object (the messages call
/// any other a non-Grass object).
const GRASS_FORM_TYPE: u32 = 0x24;
/// The type byte the constructor accepts for the parent (the message calls
/// any other not a land texture).
const LAND_TEXTURE_FORM_TYPE: u32 = 0x12;
/// `MemoryManager` allocation (`00401000`, cdecl, the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `TESRegionObjectBase::GetObject` (Xbox PDB slot +0x04).
const SLOT_GET_OBJECT: u32 = 0x4;
/// `TESRegionGrassObject::GetParentTexture` (Xbox PDB slot +0x0C).
const SLOT_GET_PARENT_TEXTURE: u32 = 0xc;
/// `TESRegionPoint::TESRegionPoint` (`004f6fc0`, engine map; no stack word).
const REGION_POINT_CTOR: u32 = 0x004f_6fc0;
/// `TESDataHandler::GetCellFromWorldCoord` (`00461bc0`, engine map;
/// `thiscall`, `RET 0x10`: x, y, worldspace, a word).
const GET_CELL_FROM_WORLD_COORD: u32 = 0x0046_1bc0;
/// `00425fd0` (unnamed in the engine map; no stack word): a byte test of
/// the cell (non-zero gives chance 0).
const CELL_TEST_00425FD0: u32 = 0x0042_5fd0;
/// `0052ce20` (unnamed in the engine map; `RET 4`, a `float`): sets the
/// region point's first coordinate.
const REGION_POINT_SET_X_0052CE20: u32 = 0x0052_ce20;
/// `004f5d90` (unnamed in the engine map; `RET 4`, a `float`): sets the
/// region point's second coordinate.
const REGION_POINT_SET_Y_004F5D90: u32 = 0x004f_5d90;
/// `TESObjectCELL::GetLand` (`00546fb0`, engine map; no stack word).
const CELL_GET_LAND: u32 = 0x0054_6fb0;
/// `0053a5e0` (unnamed in the engine map; `RET 4`, the point): the land's
/// texture at the point.
const LAND_TEXTURE_AT_0053A5E0: u32 = 0x0053_a5e0;
/// `TESObjectCELL::GetWaterHeight` (`005471e0`, engine map; ST0 result).
const CELL_GET_WATER_HEIGHT: u32 = 0x0054_71e0;
/// `TESObjectLAND::GetLandHeight` (`0053f180`, engine map; `RET 8`: the
/// point and the address of the `float` result).
const LAND_GET_HEIGHT: u32 = 0x0053_f180;
/// `00408860` (unnamed in the engine map; cdecl, a `float`; ST0 result).
const FLOAT_HELPER_00408860: u32 = 0x0040_8860;
/// `006815c0` (unnamed in the engine map; no stack word): run on each of
/// the two 12-byte locals before the land fills them.
const LOCAL_INIT_006815C0: u32 = 0x0068_15c0;
/// `0053d2e0` (unnamed in the engine map; `RET 0xC`: the point and two
/// 12-byte results): byte answer.
const LAND_VECTORS_AT_0053D2E0: u32 = 0x0053_d2e0;
/// `004f6e00` (unnamed in the engine map; cdecl, a 12-byte value; ST0 result).
const VECTOR_VALUE_004F6E00: u32 = 0x004f_6e00;
/// `TESObjectCELL::GetScaleBasedOnLocation` (`0054fd70`, engine map; `RET
/// 0xC`: the region point, 6, the region object; ST0 result).
const CELL_GET_SCALE_BASED_ON_LOCATION: u32 = 0x0054_fd70;
/// `100.0` (`double`).
const HUNDRED: u32 = 0x0101_7a40;
/// The grass form's slots the chance reads: the density byte (+0x180),
/// the water distance (`u16`, +0x1A0), the water placement mode (+0x1A8),
/// and two ST0 bounds of the land value (+0x198 the lower, +0x19C the upper).
const SLOT_GRASS_DENSITY: u32 = 0x180;
const SLOT_GRASS_WATER_DISTANCE: u32 = 0x1a0;
const SLOT_GRASS_WATER_MODE: u32 = 0x1a8;
const SLOT_GRASS_LOWER_BOUND: u32 = 0x198;
const SLOT_GRASS_UPPER_BOUND: u32 = 0x19c;

// Translated from 004f5590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionGrassObject` constructor from its parts (unnamed in the engine
/// map; `thiscall`, `RET 8`): the base constructor (`004f5600`), the
/// class's vtable, `pObject` (+4, Xbox PDB) = `object`, `pParent` (+8) =
/// `parent`. Returns `this`.
pub fn fn_004f5590(e: &mut Engine, this: Ptr, object: u32, parent: u32) -> Ptr {
    fn_004f5600(e, this);
    e.mem.set_u32(this.addr(), TES_REGION_GRASS_OBJECT_VTABLE);
    e.mem.set_u32(this.addr() + 4, object);
    e.mem.set_u32(this.addr() + 8, parent);
    this
}

// Translated from 004f5600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionObjectBase` constructor (unnamed in the engine map): stores
/// its vtable. Returns `this`.
pub fn fn_004f5600(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), TES_REGION_OBJECT_BASE_VTABLE);
    this
}

/// The region form being loaded: slot 0 of the object `00740940` returns
/// for the data handler.
fn loading_region(e: &mut Engine) -> u32 {
    let handler = e.global::<u32>(DATA_HANDLER);
    let holder = e.call(LOADING_HOLDER_00740940, &args![handler]).u32();
    e.vcall(holder, 0, &args![]).u32()
}

/// The two messages of a refused object: with the loading region, its
/// text (or `""` when `00474cb0` refuses) and form id; without, the plain
/// message.
fn warn_object(e: &mut Engine, in_region: u32, plain: u32) {
    if loading_region(e) == 0 {
        e.call(PRINT_WARNING_005B5E40, &args![plain]);
        return;
    }
    let region = loading_region(e);
    let text = if e.call(NAME_GUARD_00474CB0, &args![region]).u32() == 0 {
        EMPTY_TEXT
    } else {
        let region = loading_region(e);
        e.vcall(region, SLOT_FORM_TEXT, &args![]).u32()
    };
    let region = loading_region(e);
    let id = e.call(FORM_ID_0084E3A0, &args![region]).u32();
    e.call(PRINT_WARNING_005B5E40, &args![in_region, text, id]);
}

// Translated from 004f5620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionGrassObject::TESRegionGrassObject` (Xbox PDB; `thiscall`,
/// `RET 4`): builds the object from an 8-byte record (object id, parent
/// id). With a data handler whose holder (`00740940`) has a region being
/// loaded, both ids get that region's file's compile index
/// (`TESForm::GetFile(-1)`, `AddCompileIndex`) before the lookup
/// (`004839c0`); otherwise they are looked up as they are. `pObject` (+4)
/// is the object when its type byte is 0x24; else a warning names the
/// object as non-grass (or missing), with the region's text and id when a
/// region is being loaded. `pParent` (+8) is the parent when there is an
/// object and a parent of type byte 0x12; a parent of another type warns.
/// Returns `this`.
pub fn tes_region_grass_object_tes_region_grass_object(
    e: &mut Engine,
    this: Ptr,
    record: Ptr,
) -> Ptr {
    fn_004f5600(e, this);
    e.mem.set_u32(this.addr(), TES_REGION_GRASS_OBJECT_VTABLE);
    let handler = e.global::<u32>(DATA_HANDLER);
    let mut resolved = None;
    if handler != 0 && e.call(LOADING_HOLDER_00740940, &args![handler]).u32() != 0 {
        let holder = e.call(LOADING_HOLDER_00740940, &args![handler]).u32();
        if e.vcall(holder, 0, &args![]).u32() != 0 {
            let region = loading_region(e);
            resolved = Some(e.with_stack(4, |e, id_slot| {
                let object_id = e.mem.u32(record.addr());
                e.mem.set_u32(id_slot.addr(), object_id);
                let file = e
                    .call(TES_FORM_GET_FILE, &args![region, 0xffff_ffffu32])
                    .u32();
                e.call(TES_FORM_ADD_COMPILE_INDEX, &args![id_slot, file]);
                let id = e.mem.u32(id_slot.addr());
                let object = e.call(LOOKUP_FORM_004839C0, &args![id]).u32();
                let parent_id = e.mem.u32(record.addr() + 4);
                e.mem.set_u32(id_slot.addr(), parent_id);
                e.call(TES_FORM_ADD_COMPILE_INDEX, &args![id_slot, file]);
                let id = e.mem.u32(id_slot.addr());
                let parent = e.call(LOOKUP_FORM_004839C0, &args![id]).u32();
                (object, parent)
            }));
        }
    }
    let (object, parent) = match resolved {
        Some(pair) => pair,
        None => {
            let object_id = e.mem.u32(record.addr());
            let object = e.call(LOOKUP_FORM_004839C0, &args![object_id]).u32();
            let parent_id = e.mem.u32(record.addr() + 4);
            let parent = e.call(LOOKUP_FORM_004839C0, &args![parent_id]).u32();
            (object, parent)
        }
    };
    e.mem.set_u32(this.addr() + 4, 0);
    if object == 0 {
        warn_object(e, NO_OBJECT_IN_REGION, NO_OBJECT);
    } else if e.call(FORM_TYPE_00401170, &args![object]).u32() == GRASS_FORM_TYPE {
        e.mem.set_u32(this.addr() + 4, object);
    } else {
        warn_object(e, NON_GRASS_IN_REGION, NON_GRASS);
    }
    e.mem.set_u32(this.addr() + 8, 0);
    if e.mem.u32(this.addr() + 4) != 0 && parent != 0 {
        if e.call(FORM_TYPE_00401170, &args![parent]).u32() == LAND_TEXTURE_FORM_TYPE {
            e.mem.set_u32(this.addr() + 8, parent);
        } else {
            e.call(PRINT_WARNING_005B5E40, &args![PARENT_NOT_LAND_TEXTURE]);
        }
    }
    this
}

// Translated from 004f5920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionGrassObject` destructor body (unnamed in the engine map):
/// stores the class's vtable; nothing else.
pub fn fn_004f5920(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TES_REGION_GRASS_OBJECT_VTABLE);
}

// Translated from 004f5940 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f5940 - its FS:[0] exception frame (the unwind state that frees the block when the constructor throws) is not translated
/// `TESRegionGrassObject::Copy` (Xbox PDB; `thiscall`, `RET 4`): a non-null
/// `target` is returned as it is (nothing is copied into it). Otherwise a
/// new 0xC-byte object is made (`004f5590`) from this object's
/// `GetObject` and `GetParentTexture` (the parent asked first); returns it,
/// or null when the allocation fails.
pub fn tes_region_grass_object_copy(e: &mut Engine, this: Ptr, target: Ptr) -> Ptr {
    if !target.is_null() {
        return target;
    }
    let block = e.call(ALLOCATE, &args![0xcu32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    let parent = e
        .vcall(this.addr(), SLOT_GET_PARENT_TEXTURE, &args![])
        .u32();
    let object = e.vcall(this.addr(), SLOT_GET_OBJECT, &args![]).u32();
    fn_004f5590(e, Ptr::new(block), object, parent)
}

// Translated from 004f59f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionGrassObject::GetGenerateChance` (Xbox PDB; `thiscall`, `RET
/// 0xC`): the chance (ST0) that grass grows at `point` (two `float`s) of
/// `worldspace`. 0 without a `pObject` (+4), without a cell at the point
/// (`GetCellFromWorldCoord` with 0) or when the cell's byte test
/// `00425fd0` is set. The chance starts as the grass's density byte (slot
/// +0x180) / 100.0; a local `TESRegionPoint` is set to the point. With a
/// parent texture (slot +0xC) the land's texture at the point (`0053a5e0`)
/// must exist and have the parent's form id. Then the grass's water mode
/// (slot +0x1A8) checks the land height `L` (`GetLandHeight`) against the
/// water height `W` and the distance `D` (slot +0x1A0): 0 needs `L >= W+D`,
/// 1 `W <= L <= W+D`, 2 `L <= W-D`, 3 `W-D <= L <= W`, 4 `|W-L| >= D` and 5
/// `|W-L| <= D` (through `00408860`), 6 `L <= W+D`, 7 `L >= W-D`; another
/// mode gives 0 (each failed test, ordered, gives 0; NaN passes). When
/// `0053d2e0` fills the two vectors, the value `004f6e00` makes of the
/// first must lie within the grass's bounds (slots +0x198, +0x19C). With
/// `scale` (its low byte) set, the chance is multiplied by
/// `GetScaleBasedOnLocation(point, 6, this)`.
pub fn tes_region_grass_object_get_generate_chance(
    e: &mut Engine,
    this: Ptr,
    point: Ptr,
    worldspace: u32,
    scale: u32,
) -> f64 {
    // The frame from EBP-0x48: the two vectors (-0x48, -0x3C), the land
    // height (-0x20), the region point (-8).
    e.with_stack(0x48, |e, frame| {
        let vector_a = frame.addr();
        let vector_b = frame.addr() + 0xc;
        let land_height = frame.addr() + 0x28;
        let region_point = frame.addr() + 0x40;
        e.call(REGION_POINT_CTOR, &args![region_point]);
        // The exe first stores 0.0 in the chance; every path that returns
        // before the density is read returns its own 0.0 (FLDZ).
        let grass = e.mem.u32(this.addr() + 4);
        if grass == 0 {
            return 0.0;
        }
        let handler = e.global::<u32>(DATA_HANDLER);
        let y = x87_quiet_bits(e.mem.u32(point.addr() + 4));
        let x = x87_quiet_bits(e.mem.u32(point.addr()));
        let cell = e
            .call(
                GET_CELL_FROM_WORLD_COORD,
                &args![handler, x, y, worldspace, 0u32],
            )
            .u32();
        if cell == 0 || e.call(CELL_TEST_00425FD0, &args![cell]).u8() != 0 {
            return 0.0;
        }
        let density = e.vcall(grass, SLOT_GRASS_DENSITY, &args![]).u8();
        let hundred: f64 = e.global(HUNDRED);
        let mut chance = x87_store32(x87_div(f64::from(density), hundred));
        let x = x87_quiet_bits(e.mem.u32(point.addr()));
        e.call(REGION_POINT_SET_X_0052CE20, &args![region_point, x]);
        let y = x87_quiet_bits(e.mem.u32(point.addr() + 4));
        e.call(REGION_POINT_SET_Y_004F5D90, &args![region_point, y]);
        if e.vcall(this.addr(), SLOT_GET_PARENT_TEXTURE, &args![])
            .u32()
            != 0
        {
            let land = e.call(CELL_GET_LAND, &args![cell]).u32();
            let texture = e.call(LAND_TEXTURE_AT_0053A5E0, &args![land, point]).u32();
            if texture == 0 {
                return 0.0;
            }
            let texture_id = e.call(FORM_ID_0084E3A0, &args![texture]).u32();
            let parent = e
                .vcall(this.addr(), SLOT_GET_PARENT_TEXTURE, &args![])
                .u32();
            if texture_id != e.call(FORM_ID_0084E3A0, &args![parent]).u32() {
                return 0.0;
            }
        }
        let grass = e.mem.u32(this.addr() + 4);
        let distance = e.vcall(grass, SLOT_GRASS_WATER_DISTANCE, &args![]).u16();
        let distance = x87_store32(f64::from(distance));
        let water = e.call(CELL_GET_WATER_HEIGHT, &args![cell]).f64();
        let water = x87_store32(water);
        let land = e.call(CELL_GET_LAND, &args![cell]).u32();
        e.call(LAND_GET_HEIGHT, &args![land, point, land_height]);
        let height = x87_m32(e.mem.u32(land_height));
        let w = x87_m32(water);
        let d = x87_m32(distance);
        let grass = e.mem.u32(this.addr() + 4);
        let mode = e.vcall(grass, SLOT_GRASS_WATER_MODE, &args![]).u32();
        let passes = match mode {
            0 => !(height < x87_add(w, d)),
            1 => !(w > height) && !(height > x87_add(w, d)),
            2 => !(height > x87_sub(w, d)),
            3 => !(w < height) && !(height < x87_sub(w, d)),
            4 | 5 => {
                let difference = x87_store32(x87_sub(w, height));
                let magnitude = e.call(FLOAT_HELPER_00408860, &args![difference]).f64();
                let magnitude = x87_m32(x87_store32(magnitude));
                if mode == 4 {
                    !(d > magnitude)
                } else {
                    !(d < magnitude)
                }
            }
            6 => !(height > x87_add(w, d)),
            7 => !(height < x87_sub(w, d)),
            _ => false,
        };
        if !passes {
            return 0.0;
        }
        e.call(LOCAL_INIT_006815C0, &args![vector_a]);
        e.call(LOCAL_INIT_006815C0, &args![vector_b]);
        let land = e.call(CELL_GET_LAND, &args![cell]).u32();
        if e.call(
            LAND_VECTORS_AT_0053D2E0,
            &args![land, point, vector_a, vector_b],
        )
        .u8()
            != 0
        {
            let words = [
                e.mem.u32(vector_a),
                e.mem.u32(vector_a + 4),
                e.mem.u32(vector_a + 8),
            ];
            let value = e.call(VECTOR_VALUE_004F6E00, &words).f64();
            let value = x87_m32(x87_store32(value));
            let grass = e.mem.u32(this.addr() + 4);
            let upper = e.vcall(grass, SLOT_GRASS_UPPER_BOUND, &args![]).f64();
            if value > upper {
                return 0.0;
            }
            let grass = e.mem.u32(this.addr() + 4);
            let lower = e.vcall(grass, SLOT_GRASS_LOWER_BOUND, &args![]).f64();
            if value < lower {
                return 0.0;
            }
        }
        if scale as u8 != 0 {
            let factor = e
                .call(
                    CELL_GET_SCALE_BASED_ON_LOCATION,
                    &args![cell, region_point, 6u32, this],
                )
                .f64();
            chance = x87_store32(x87_mul(factor, x87_m32(chance)));
        }
        x87_m32(chance)
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_55d0, tes_region_grass_object_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x004f_5590, fn_004f5590(Ptr, u32, u32) -> Ptr),
        entry!(0x004f_5600, fn_004f5600(Ptr) -> Ptr),
        entry!(
            0x004f_5620,
            tes_region_grass_object_tes_region_grass_object(Ptr, Ptr) -> Ptr
        ),
        entry!(0x004f_5920, fn_004f5920(Ptr)),
        entry!(0x004f_5940, tes_region_grass_object_copy(Ptr, Ptr) -> Ptr),
        entry!(
            0x004f_59f0,
            tes_region_grass_object_get_generate_chance(Ptr, Ptr, u32, u32) -> f64
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004f55d0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_5920, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_55d0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_55d0)
                .collect();
            let mut expected = vec![(0x004f_5920, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod tests_n073 {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Default::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Default::default()
        }
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn small_constructors_and_destructor_store_vtables_and_parts() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc);
        assert_eq!(e.call(0x004f_5600, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), TES_REGION_OBJECT_BASE_VTABLE);
        assert_eq!(
            e.call(0x004f_5590, &args![this, 0x11u32, 0x22u32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), TES_REGION_GRASS_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x11);
        assert_eq!(e.mem.u32(this + 8), 0x22);
        e.mem.set_u32(this, 0);
        e.call(0x004f_5920, &args![this]);
        assert_eq!(e.mem.u32(this), TES_REGION_GRASS_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x11);
    }

    /// Forms: a word at +4 holds the type byte, +0xC the id. The lookup
    /// double maps an id to the form whose id it is (none for others).
    fn form(e: &mut Engine, kind: u8, id: u32) -> u32 {
        let f = e.mem.alloc(0x10);
        e.mem.set_u8(f + 4, kind);
        e.mem.set_u32(f + 0xc, id);
        f
    }

    fn constructor_doubles(e: &mut Engine, forms: Vec<u32>) {
        e.map(DATA_HANDLER & !0xfff, 0x1000);
        e.register(FORM_TYPE_00401170, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        e.register(FORM_ID_0084E3A0, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(PRINT_WARNING_005B5E40, |_, _| Ret::default());
        e.register_double(LOOKUP_FORM_004839C0, move |e, a| {
            ret(forms
                .iter()
                .copied()
                .find(|f| e.mem.u32(f + 0xc) == a[0])
                .unwrap_or(0))
        });
    }

    #[test]
    fn constructor_without_a_loading_region_looks_ids_up_as_they_are() {
        let mut e = Engine::new();
        let grass = form(&mut e, 0x24, 0x100);
        let texture = form(&mut e, 0x12, 0x200);
        let other = form(&mut e, 0x30, 0x300);
        constructor_doubles(&mut e, vec![grass, texture, other]);
        let this = e.mem.alloc(0xc);
        let record = e.mem.alloc(8);
        e.mem.set_u32(record, 0x100);
        e.mem.set_u32(record + 4, 0x200);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_5620, &args![this, record]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), TES_REGION_GRASS_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + 4), grass);
        assert_eq!(e.mem.u32(this + 8), texture);
        assert!(calls_to(&log, PRINT_WARNING_005B5E40).is_empty());
        // A parent of another type warns and stays null.
        e.mem.set_u32(record + 4, 0x300);
        e.call_log = Some(vec![]);
        e.call(0x004f_5620, &args![this, record]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(
            calls_to(&log, PRINT_WARNING_005B5E40),
            vec![vec![PARENT_NOT_LAND_TEXTURE]]
        );
    }

    #[test]
    fn constructor_warns_about_a_missing_or_non_grass_object() {
        let mut e = Engine::new();
        let other = form(&mut e, 0x30, 0x300);
        let texture = form(&mut e, 0x12, 0x200);
        constructor_doubles(&mut e, vec![other, texture]);
        // The holder answers no region: the plain messages.
        let holder = e.mem.alloc(4);
        e.put_vtable(0x0150_0000, &[0x0150_1000]);
        e.mem.set_u32(holder, 0x0150_0000);
        e.register(0x0150_1000, |_, _| ret(0));
        e.register_double(LOADING_HOLDER_00740940, move |_, _| ret(holder));
        let this = e.mem.alloc(0xc);
        let record = e.mem.alloc(8);
        e.mem.set_u32(record, 0x999);
        e.mem.set_u32(record + 4, 0x200);
        e.call_log = Some(vec![]);
        e.call(0x004f_5620, &args![this, record]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(
            calls_to(&log, PRINT_WARNING_005B5E40),
            vec![vec![NO_OBJECT]]
        );
        e.mem.set_u32(record, 0x300);
        e.call_log = Some(vec![]);
        e.call(0x004f_5620, &args![this, record]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, PRINT_WARNING_005B5E40),
            vec![vec![NON_GRASS]]
        );
    }

    #[test]
    fn constructor_with_a_loading_region_adds_its_compile_index_and_names_it() {
        let mut e = Engine::new();
        let grass = form(&mut e, 0x24, 0x0500_0100);
        let texture = form(&mut e, 0x12, 0x0500_0200);
        let other = form(&mut e, 0x30, 0x0500_0300);
        constructor_doubles(&mut e, vec![grass, texture, other]);
        e.set_global(DATA_HANDLER, 0x1234u32);
        let region = form(&mut e, 0x4a, 0x777);
        e.put_vtable(0x0150_0100, &{
            let mut slots = vec![0u32; 0x4d];
            slots[0x130 / 4] = 0x0150_1130;
            slots
        });
        e.mem.set_u32(region, 0x0150_0100);
        e.register(0x0150_1130, |_, _| ret(0x5151));
        let holder = e.mem.alloc(4);
        e.put_vtable(0x0150_0000, &[0x0150_1000]);
        e.mem.set_u32(holder, 0x0150_0000);
        e.register_double(0x0150_1000, move |_, _| ret(region));
        e.register_double(LOADING_HOLDER_00740940, move |_, _| ret(holder));
        e.register(TES_FORM_GET_FILE, |_, _| ret(0xf11e));
        e.register(TES_FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0500_0000);
            Ret::default()
        });
        e.register(NAME_GUARD_00474CB0, |_, _| ret(1));
        let this = e.mem.alloc(0xc);
        let record = e.mem.alloc(8);
        e.mem.set_u32(record, 0x100);
        e.mem.set_u32(record + 4, 0x200);
        e.call_log = Some(vec![]);
        e.call(0x004f_5620, &args![this, record]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 4), grass);
        assert_eq!(e.mem.u32(this + 8), texture);
        assert_eq!(
            calls_to(&log, TES_FORM_GET_FILE),
            vec![vec![region, 0xffff_ffff]]
        );
        // A non-grass object names the region (its text and id).
        e.mem.set_u32(record, 0x300);
        e.call_log = Some(vec![]);
        e.call(0x004f_5620, &args![this, record]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, PRINT_WARNING_005B5E40),
            vec![vec![NON_GRASS_IN_REGION, 0x5151, 0x777]]
        );
        // Without the name guard, the text is empty; a missing object.
        e.register(NAME_GUARD_00474CB0, |_, _| ret(0));
        e.mem.set_u32(record, 0x999);
        e.call_log = Some(vec![]);
        e.call(0x004f_5620, &args![this, record]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, PRINT_WARNING_005B5E40),
            vec![vec![NO_OBJECT_IN_REGION, EMPTY_TEXT, 0x777]]
        );
    }

    #[test]
    fn copy_returns_a_given_target_or_makes_a_new_object() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_5940, &args![this, 0x4444u32]).u32(), 0x4444);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.register(ALLOCATE, |_, _| ret(0));
        assert_eq!(e.call(0x004f_5940, &args![this, 0u32]).u32(), 0);
        let block = e.mem.alloc(0xc);
        e.register_double(ALLOCATE, move |_, _| ret(block));
        e.put_vtable(0x0150_0000, &[0, 0x0150_1004, 0, 0x0150_100c]);
        e.mem.set_u32(this, 0x0150_0000);
        e.register(0x0150_1004, |_, _| ret(0x11));
        e.register(0x0150_100c, |_, _| ret(0x22));
        assert_eq!(e.call(0x004f_5940, &args![this, 0u32]).u32(), block);
        assert_eq!(e.mem.u32(block), TES_REGION_GRASS_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(block + 4), 0x11);
        assert_eq!(e.mem.u32(block + 8), 0x22);
    }

    /// The setting for the chance: a grass form with the slots it reads, a
    /// cell, the land height (+0 of `heights`), water height and distance.
    struct Chance {
        this: u32,
        point: u32,
        grass: u32,
    }

    fn chance_setup(e: &mut Engine, mode: u32, land: f32, water: f64, distance: u16) -> Chance {
        e.map(DATA_HANDLER & !0xfff, 0x1000);
        e.set_global(DATA_HANDLER, 0x1234u32);
        e.map(HUNDRED & !0xfff, 0x1000);
        e.set_global(HUNDRED, 100.0f64);
        let mut slots = vec![0u32; 0x6c];
        slots[0x180 / 4] = 0x0150_1180;
        slots[0x198 / 4] = 0x0150_1198;
        slots[0x19c / 4] = 0x0150_119c;
        slots[0x1a0 / 4] = 0x0150_11a0;
        slots[0x1a8 / 4] = 0x0150_11a8;
        e.put_vtable(0x0150_0000, &slots);
        let grass = e.mem.alloc(0x10);
        e.mem.set_u32(grass, 0x0150_0000);
        e.register(0x0150_1180, |_, _| ret(50));
        e.register(0x0150_1198, |_, _| st0(0.25));
        e.register(0x0150_119c, |_, _| st0(0.75));
        e.register_double(0x0150_11a0, move |_, _| ret(distance as u32 | 0xabcd_0000));
        e.register_double(0x0150_11a8, move |_, _| ret(mode));
        // This object: slot 0xC (no parent texture).
        e.put_vtable(0x0150_0200, &[0, 0, 0, 0x0150_120c]);
        e.register(0x0150_120c, |_, _| ret(0));
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this, 0x0150_0200);
        e.mem.set_u32(this + 4, grass);
        let point = e.mem.alloc(8);
        e.mem.set_f32(point, 1.5);
        e.mem.set_f32(point + 4, -2.5);
        e.register(REGION_POINT_CTOR, |_, _| Ret::default());
        e.register(GET_CELL_FROM_WORLD_COORD, |_, _| ret(0xce11));
        e.register(CELL_TEST_00425FD0, |_, _| ret(0));
        e.register(REGION_POINT_SET_X_0052CE20, |_, _| Ret::default());
        e.register(REGION_POINT_SET_Y_004F5D90, |_, _| Ret::default());
        e.register(CELL_GET_LAND, |_, _| ret(0x1a4d));
        e.register_double(CELL_GET_WATER_HEIGHT, move |_, _| st0(water));
        e.register_double(LAND_GET_HEIGHT, move |e, a| {
            e.mem.set_f32(a[2], land);
            Ret::default()
        });
        e.register(FLOAT_HELPER_00408860, |_, a| {
            st0(f64::from(f32::from_bits(a[0]).abs()))
        });
        e.register(LOCAL_INIT_006815C0, |_, a| ret(a[0]));
        e.register(LAND_VECTORS_AT_0053D2E0, |_, _| ret(0));
        Chance { this, point, grass }
    }

    fn chance(e: &mut Engine, c: &Chance, scale: u32) -> f64 {
        e.call(0x004f_59f0, &args![c.this, c.point, 0x3000u32, scale])
            .f64()
    }

    #[test]
    fn chance_is_zero_without_object_cell_or_with_the_cell_test() {
        let mut e = Engine::new();
        let c = chance_setup(&mut e, 0, 10.0, 0.0, 5);
        e.call_log = Some(vec![]);
        assert_eq!(chance(&mut e, &c, 0), 0.5);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, GET_CELL_FROM_WORLD_COORD),
            vec![vec![
                0x1234,
                1.5f32.to_bits(),
                (-2.5f32).to_bits(),
                0x3000,
                0
            ]]
        );
        e.register(CELL_TEST_00425FD0, |_, _| ret(1));
        assert_eq!(chance(&mut e, &c, 0), 0.0);
        e.register(GET_CELL_FROM_WORLD_COORD, |_, _| ret(0));
        assert_eq!(chance(&mut e, &c, 0), 0.0);
        e.mem.set_u32(c.this + 4, 0);
        assert_eq!(chance(&mut e, &c, 0), 0.0);
    }

    #[test]
    fn chance_follows_the_water_modes() {
        // (mode, land height, water height, distance, grows)
        let cases: [(u32, f32, f64, u16, bool); 20] = [
            (0, 10.0, 0.0, 5, true),
            (0, 4.0, 0.0, 5, false),
            (1, 3.0, 0.0, 5, true),
            (1, -1.0, 0.0, 5, false),
            (1, 6.0, 0.0, 5, false),
            (2, -5.0, 0.0, 5, true),
            (2, -4.0, 0.0, 5, false),
            (3, -3.0, 0.0, 5, true),
            (3, 1.0, 0.0, 5, false),
            (3, -6.0, 0.0, 5, false),
            (4, -8.0, 0.0, 5, true),
            (4, 4.0, 0.0, 5, false),
            (5, 4.0, 0.0, 5, true),
            (5, -8.0, 0.0, 5, false),
            (6, 5.0, 0.0, 5, true),
            (6, 6.0, 0.0, 5, false),
            (7, -5.0, 0.0, 5, true),
            (7, -6.0, 0.0, 5, false),
            (8, 0.0, 0.0, 5, false),
            (0, f32::NAN, 0.0, 5, true),
        ];
        for (mode, land, water, distance, grows) in cases {
            let mut e = Engine::new();
            let c = chance_setup(&mut e, mode, land, water, distance);
            let expected = if grows { 0.5 } else { 0.0 };
            assert_eq!(chance(&mut e, &c, 0), expected, "mode {mode} land {land}");
        }
    }

    #[test]
    fn chance_checks_the_parent_texture() {
        let mut e = Engine::new();
        let c = chance_setup(&mut e, 0, 10.0, 0.0, 5);
        let parent = e.mem.alloc(0x10);
        e.mem.set_u32(parent + 0xc, 0x42);
        e.register_double(0x0150_120c, move |_, _| ret(parent));
        e.register(FORM_ID_0084E3A0, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(LAND_TEXTURE_AT_0053A5E0, |_, _| ret(0));
        assert_eq!(chance(&mut e, &c, 0), 0.0);
        let other = e.mem.alloc(0x10);
        e.mem.set_u32(other + 0xc, 0x43);
        e.register_double(LAND_TEXTURE_AT_0053A5E0, move |_, _| ret(other));
        assert_eq!(chance(&mut e, &c, 0), 0.0);
        e.register_double(LAND_TEXTURE_AT_0053A5E0, move |_, _| ret(parent));
        e.call_log = Some(vec![]);
        assert_eq!(chance(&mut e, &c, 0), 0.5);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, LAND_TEXTURE_AT_0053A5E0),
            vec![vec![0x1a4d, c.point]]
        );
    }

    #[test]
    fn chance_checks_the_land_value_bounds_and_scales() {
        let mut e = Engine::new();
        let c = chance_setup(&mut e, 0, 10.0, 0.0, 5);
        e.register(LAND_VECTORS_AT_0053D2E0, |e, a| {
            e.mem.set_f32(a[2], 7.0);
            ret(1)
        });
        for (value, grows) in [(0.5f64, true), (0.8, false), (0.2, false), (0.75, true)] {
            e.register_double(VECTOR_VALUE_004F6E00, move |_, _| st0(value));
            let expected = if grows { 0.5 } else { 0.0 };
            assert_eq!(chance(&mut e, &c, 0), expected, "value {value}");
        }
        e.register(VECTOR_VALUE_004F6E00, |_, a| {
            assert_eq!(a[0], 7.0f32.to_bits());
            st0(0.5)
        });
        e.register(CELL_GET_SCALE_BASED_ON_LOCATION, |_, _| st0(0.5));
        e.call_log = Some(vec![]);
        assert_eq!(chance(&mut e, &c, 0x100), 0.5);
        assert!(calls_to(
            &e.call_log.take().unwrap(),
            CELL_GET_SCALE_BASED_ON_LOCATION
        )
        .is_empty());
        e.call_log = Some(vec![]);
        assert_eq!(chance(&mut e, &c, 1), 0.25);
        let log = e.call_log.take().unwrap();
        let scale = calls_to(&log, CELL_GET_SCALE_BASED_ON_LOCATION);
        assert_eq!(scale.len(), 1);
        assert_eq!((scale[0][0], scale[0][2], scale[0][3]), (0xce11, 6, c.this));
        assert!(c.grass != 0);
    }
}
