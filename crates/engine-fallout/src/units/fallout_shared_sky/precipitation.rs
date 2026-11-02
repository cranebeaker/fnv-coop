//! `fallout shared/sky/precipitation.cpp` (Xbox PDB source unit), subsystem `fallout shared/sky`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_00639b00` calls on the member (`009938b0`).
const CALLEE_009938B0: u32 = 0x0099_38b0;

// Translated from 00639b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `009938b0` with `ECX` = `this + 0x54` and passes on its 16-bit result.
pub fn fn_00639b00(e: &mut Engine, this: Ptr) -> u16 {
    e.call(CALLEE_009938B0, &args![this.byte_add(0x54)]).u16()
}

layout! {
    /// `Precipitation` (Xbox PDB), 0x18 bytes; the PC code uses the Xbox PDB's offsets.
    pub struct Precipitation: 0x18 {
        /// `spCurrentPrecipRoot` (Xbox PDB): `NiPointer<NiNode>`.
        0x04 spCurrentPrecipRoot: u32,
        /// `spLastPrecipRoot` (Xbox PDB): `NiPointer<NiNode>`.
        0x08 spLastPrecipRoot: u32,
        /// `pParent` (Xbox PDB): `NiNode*`.
        0x0C pParent: u32,
        /// `fBirthRate` (Xbox PDB).
        0x10 fBirthRate: f32,
        /// `pQueuedModel` (Xbox PDB): `const TESModel*`.
        0x14 pQueuedModel: u32,
    }
}

/// The `Precipitation` vtable the destructor sets (`0104ed98`).
const PRECIPITATION_VTABLE: u32 = 0x0104_ed98;
/// `NiPointer` helpers (`thiscall` on the pointer's address): the pointer's value (`00559450`,
/// no stack words), assignment of a raw pointer (`0066b0d0`, `RET 4`), assignment of another
/// `NiPointer` (`006e5cc0`, `RET 4`: its address) and the destructor (`0045cec0`).
const POINTER_GET: u32 = 0x0055_9450;
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
const POINTER_ASSIGN_POINTER: u32 = 0x006e_5cc0;
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The function `fn_00636bb0` stores the result of (`0054f4f0`: no arguments, the value of the
/// `NiPointer` global at `011deda4`).
const POINTER_GLOBAL_011DEDA4_GET: u32 = 0x0054_f4f0;
/// `0048cee0` (`thiscall`, no stack words), asked of a model before it is queued.
const MODEL_TEST_0048CEE0: u32 = 0x0048_cee0;
/// The model loader singleton (a pointer global), `ModelLoader::QueueModel` (`00444040`,
/// `RET 0x1c`), `ModelLoader::LoadFile` (`00447080`, `RET 0x18`), `ModelLoader::FindModel`
/// (`004472a0`, `RET 8`) and `0045a5e0` (`RET 4`, the path) called after a found model is loaded.
const MODEL_LOADER: u32 = 0x011c_3b3c;
const MODEL_LOADER_QUEUE_MODEL: u32 = 0x0044_4040;
const MODEL_LOADER_LOAD_FILE: u32 = 0x0044_7080;
const MODEL_LOADER_FIND_MODEL: u32 = 0x0044_72a0;
const MODEL_LOADER_AFTER_LOAD_0045A5E0: u32 = 0x0045_a5e0;
/// `_stricmp`-shaped compare (`00404dc0`, `cdecl`, two strings; 0 when equal) and the text
/// `"Sky\Snow.NIF"` (`0104ed9c`).
const STRING_COMPARE: u32 = 0x0040_4dc0;
const SNOW_MODEL_PATH: u32 = 0x0104_ed9c;
/// `0043d410` (`thiscall` on a 12-byte local, `RET 0xc`: a `float` and two words).
const LOCAL_INIT_0043D410: u32 = 0x0043_d410;
/// The allocation `00aa13e0` (`cdecl`, size) and `NiNode::NiNode` (`00a5ecb0`, `RET 4`).
const NEW_OBJECT: u32 = 0x00aa_13e0;
const NI_NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
/// The loaded model's child count (`00453470`, no stack words) and child (`0043b4a0`, `RET 4`:
/// the index); both also used on the precipitation roots.
const CHILD_COUNT: u32 = 0x0045_3470;
const CHILD_AT: u32 = 0x0043_b4a0;
/// `0045bc00` (`thiscall`, `RET 4`) on what a child's virtual slot +0xc answers, then the
/// kind-of cast `00653270` (`cdecl`: the `NiRTTI` record, the object) to the record at
/// `012024e0`.
const CHILD_OBJECT_0045BC00: u32 = 0x0045_bc00;
const KIND_OF_CAST: u32 = 0x0065_3270;
const CAST_RTTI_012024E0: u32 = 0x0120_24e0;
/// `00461130` (`thiscall` on the parent node, no stack words) and `00637890` (this unit,
/// `thiscall`, `RET 0xc`), which builds the precipitation for one cast child.
const PARENT_0046130: u32 = 0x0046_1130;
const BUILD_CHILD_00637890: u32 = 0x0063_7890;
/// `NiAVObject::UpdateProperties` (`00a5a040`, no stack words).
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// The parent node's virtual slots: detach a child (+0xe8, one word) and attach one (+0xdc, two
/// words: the child and 1).
const SLOT_DETACH_CHILD: u32 = 0xe8;
const SLOT_ATTACH_CHILD: u32 = 0xdc;
/// A model's virtual slot +0x14 (no stack words): its path.
const SLOT_MODEL_PATH: u32 = 0x14;

/// `_ftol2_sse` (`00ec62c0`): truncates the value in `ST0`, passed as an `f64` (two words).
const FTOL: u32 = 0x00ec_62c0;
/// `fn_00639730`'s data: the `float` divisor (`011ccb34`), the `double` numerator (`01028338`)
/// and the two tables of 512 `float`s (`011f52e0`, `011f4ae0`).
const TABLE_DIVISOR_011CCB34: u32 = 0x011c_cb34;
const TABLE_NUMERATOR_01028338: u32 = 0x0102_8338;
const TABLE_011F52E0: u32 = 0x011f_52e0;
const TABLE_011F4AE0: u32 = 0x011f_4ae0;

/// `NiBlendInterpolator::GetHighestWeightedIndex` (`00a368f0`, `u8` result),
/// `NiBlendInterpolator::GetInterpolator` (`00495d50`, `RET 4`), the invalid index byte
/// (`01096968`) and the two `NiRTTI` records `fn_00639b60` casts to (`011f3738`, `011f3e58`).
const BLEND_HIGHEST_WEIGHTED_INDEX: u32 = 0x00a3_68f0;
const BLEND_GET_INTERPOLATOR: u32 = 0x0049_5d50;
const INVALID_INDEX_01096968: u32 = 0x0109_6968;
const INTERPOLATOR_RTTI_011F3738: u32 = 0x011f_3738;
const BLEND_RTTI_011F3E58: u32 = 0x011f_3e58;

/// The value of the `NiPointer` at `slot`.
fn pointer_get(e: &mut Engine, slot: u32) -> u32 {
    e.call(POINTER_GET, &args![slot]).u32()
}

/// The parent's detach of the `NiPointer` at `this + offset` when it holds a node, then the
/// pointer cleared (`00637800`'s and `00636ed0`'s inlined sequence).
fn detach_root(e: &mut Engine, this: Ptr<Precipitation>, offset: u32) {
    let slot = this.addr().wrapping_add(offset);
    if pointer_get(e, slot) != 0 {
        let node = pointer_get(e, slot);
        let parent = e.get(this, Precipitation::pParent);
        e.vcall(parent, SLOT_DETACH_CHILD, &args![node]);
        e.call(POINTER_ASSIGN, &args![slot, 0u32]);
    }
}

// Translated from 00636af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Precipitation::~Precipitation`'s scalar deleting destructor (Xbox PDB name
/// `Precipitation::_scalar_deleting_destructor_`). `thiscall`, `RET 4` (the flags): runs the
/// destructor (`00636b20`), frees the object (`00401030`) when bit 0 of the flags is set and
/// answers `this`.
pub fn precipitation_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<Precipitation>,
    flags: u32,
) -> Ptr<Precipitation> {
    fn_00636b20(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00636b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `Precipitation` destructor (by its body). `thiscall`, no
/// stack words. Sets the vtable (`0104ed98`), assigns null to both precipitation roots
/// (`0066b0d0` on +4, then +8), clears `pQueuedModel`, then destroys the roots' `NiPointer`s
/// (`0045cec0` on +8, then +4).
// Translation incomplete: 00636b20 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the two NiPointer members) is not translated.
pub fn fn_00636b20(e: &mut Engine, this: Ptr<Precipitation>) {
    let base = this.addr();
    e.mem.set_u32(base, PRECIPITATION_VTABLE);
    e.call(POINTER_ASSIGN, &args![base.wrapping_add(4), 0u32]);
    e.call(POINTER_ASSIGN, &args![base.wrapping_add(8), 0u32]);
    e.set(this, Precipitation::pQueuedModel, 0);
    e.call(POINTER_DESTRUCT, &args![base.wrapping_add(8)]);
    e.call(POINTER_DESTRUCT, &args![base.wrapping_add(4)]);
}

// Translated from 00636bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets `pParent` to what `0054f4f0` answers (the value of the
/// `NiPointer` global at `011deda4`; it takes no arguments). `thiscall`, no stack words.
pub fn fn_00636bb0(e: &mut Engine, this: Ptr<Precipitation>) {
    let parent = e.call(POINTER_GLOBAL_011DEDA4_GET, &args![]).u32();
    e.set(this, Precipitation::pParent, parent);
}

// Translated from 00636bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Precipitation::Load` (Xbox PDB), the model overload. `thiscall`, `RET 4` (the model).
/// Nothing while the last precipitation root still holds a node. Otherwise, a model that
/// `0048cee0` accepts is queued with the model loader (`ModelLoader::QueueModel` on its path,
/// six zero words) and becomes `pQueuedModel`; any other model (or none) loads no file
/// (`Load_ov2` with a null path).
pub fn precipitation_load(e: &mut Engine, this: Ptr<Precipitation>, model: Ptr) {
    let last = this.addr().wrapping_add(8);
    if pointer_get(e, last) != 0 {
        return;
    }
    if model.addr() != 0 && e.call(MODEL_TEST_0048CEE0, &args![model]).u32() != 0 {
        let path = e.vcall(model.addr(), SLOT_MODEL_PATH, &args![]).u32();
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(
            MODEL_LOADER_QUEUE_MODEL,
            &args![loader, path, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
        );
        e.set(this, Precipitation::pQueuedModel, model.addr());
    } else {
        precipitation_load_ov2(e, this, Ptr::new(0));
    }
}

// Translated from 00636c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Precipitation::Load` (Xbox PDB, overload 2), the path overload. `thiscall`, `RET 4` (the
/// path). When the current root holds a node it is detached from the parent (slot +0xe8),
/// becomes the last root (`006e5cc0`), the last root is attached again (slot +0xdc with 1) and
/// the current root is cleared. With a non-empty path whose file the model loader loads
/// (`ModelLoader::LoadFile`: path, 0, 1, 0, 0, 0): whether the path is `"Sky\Snow.NIF"`
/// (case-insensitive compare `00404dc0`) is kept, a 12-byte local is set up by `0043d410`
/// (0.0, 0, 0) and passed to the model's virtual slot +0xa4 (with 0), a new `NiNode` (0xac
/// bytes; null when the allocation fails) becomes the current root and `fBirthRate` 0.0; then
/// for each child of the model (`00453470` children, `0043b4a0`) whose virtual slot +0xc
/// answers an object, `0045bc00` (with 0) on that object is cast to the record at `012024e0`
/// (`00653270`); a cast child is built by `00637890` (the child, what `00461130` answers for
/// the parent, the snow flag) and a built one attached to the current root (slot +0xdc with
/// 1). Finally the current root is attached to the parent (slot +0xdc with 1) and its
/// properties updated (`NiAVObject::UpdateProperties`).
// Translation incomplete: 00636c40 - the C++ exception frame (FS:[0] chain, unwind state 0 for the new NiNode) is not translated.
pub fn precipitation_load_ov2(e: &mut Engine, this: Ptr<Precipitation>, path: Ptr) {
    let base = this.addr();
    let current = base.wrapping_add(4);
    let last = base.wrapping_add(8);
    if pointer_get(e, current) != 0 {
        let node = pointer_get(e, current);
        let parent = e.get(this, Precipitation::pParent);
        e.vcall(parent, SLOT_DETACH_CHILD, &args![node]);
        e.call(POINTER_ASSIGN_POINTER, &args![last, current]);
        let node = pointer_get(e, last);
        let parent = e.get(this, Precipitation::pParent);
        e.vcall(parent, SLOT_ATTACH_CHILD, &args![node, 1u32]);
        e.call(POINTER_ASSIGN, &args![current, 0u32]);
    }
    let path = path.addr();
    if path == 0 || e.mem.u8(path) == 0 {
        return;
    }
    let loader = e.global::<u32>(MODEL_LOADER);
    let model = e
        .call(
            MODEL_LOADER_LOAD_FILE,
            &args![loader, path, 0u32, 1u32, 0u32, 0u32, 0u32],
        )
        .u32();
    if model == 0 {
        return;
    }
    let snow = e.call(STRING_COMPARE, &args![path, SNOW_MODEL_PATH]).u32() == 0;
    e.with_stack(0xc, |e, local| {
        e.call(LOCAL_INIT_0043D410, &args![local, 0.0f32, 0u32, 0u32]);
        e.vcall(model, 0xa4, &args![local, 0u32]);
    });
    let memory = e.call(NEW_OBJECT, &args![0xacu32]).u32();
    let node = if memory != 0 {
        e.call(NI_NODE_CONSTRUCT, &args![memory, 0u32]).u32()
    } else {
        0
    };
    e.call(POINTER_ASSIGN, &args![current, node]);
    e.set(
        this,
        Precipitation::fBirthRate,
        f32::from_bits(x87_store32(0.0)),
    );
    let mut index = 0u32;
    while index < e.call(CHILD_COUNT, &args![model]).u32() {
        let child = e.call(CHILD_AT, &args![model, index]).u32();
        let object = e.vcall(child, 0xc, &args![]).u32();
        let cast = if object != 0 {
            let inner = e.call(CHILD_OBJECT_0045BC00, &args![object, 0u32]).u32();
            e.call(KIND_OF_CAST, &args![CAST_RTTI_012024E0, inner])
                .u32()
        } else {
            0
        };
        if cast != 0 {
            let parent = e.get(this, Precipitation::pParent);
            let from_parent = e.call(PARENT_0046130, &args![parent]).u32();
            let built = e
                .call(
                    BUILD_CHILD_00637890,
                    &args![this, cast, from_parent, u32::from(snow)],
                )
                .u32();
            if built != 0 {
                let root = pointer_get(e, current);
                e.vcall(root, SLOT_ATTACH_CHILD, &args![built, 1u32]);
            }
        }
        index = index.wrapping_add(1);
    }
    let root = pointer_get(e, current);
    let parent = e.get(this, Precipitation::pParent);
    e.vcall(parent, SLOT_ATTACH_CHILD, &args![root, 1u32]);
    let root = pointer_get(e, current);
    e.call(UPDATE_PROPERTIES, &args![root]);
}

/// `Sky::GetInstance` (`0046dd00`, no arguments) and the sky's accessors `fn_00636ed0` reads
/// (`thiscall`, no stack words): `00476c90` (compared with 1 and 0), `0044edb0` (the engine map
/// names the folded body `BaseProcess::GetCurrentProcedureIndex`; here a weather), `00825c00`
/// (another weather) and `006447d0` (a `float` in `ST0`), and `00634870` (`u8` result).
const SKY_GET_INSTANCE: u32 = 0x0046_dd00;
const SKY_MODE_00476C90: u32 = 0x0047_6c90;
const SKY_WEATHER_0044EDB0: u32 = 0x0044_edb0;
const SKY_WEATHER_00825C00: u32 = 0x0082_5c00;
const SKY_FRACTION_006447D0: u32 = 0x0064_47d0;
const SKY_TEST_00634870: u32 = 0x0063_4870;
/// The weather's `float` read (`004ed230`, `thiscall`, `RET 0xc`: an index and two `float`s;
/// `float` in `ST0`) and the `float`s `fn_00636ed0` passes (`010231e0`) and falls back to
/// (`01017d00`).
const WEATHER_VALUE_004ED230: u32 = 0x004e_d230;
const WEATHER_ARGUMENT_010231E0: u32 = 0x0102_31e0;
const WEATHER_FALLBACK_01017D00: u32 = 0x0101_7d00;
/// The singleton pointer global `011de45c` and its test `0047c850` (`u8` result).
const SINGLETON_011DE45C: u32 = 0x011d_e45c;
const SINGLETON_TEST_0047C850: u32 = 0x0047_c850;
/// The 4-byte local `00633c90` constructs (`RET 4`, with 0) and `0040c110` destroys.
const LOCAL_CONSTRUCT_00633C90: u32 = 0x0063_3c90;
const LOCAL_DESTRUCT_0040C110: u32 = 0x0040_c110;
/// `1.0` (`double`, `01012070`).
const ONE_01012070: u32 = 0x0101_2070;
/// `006377c0` (this unit, `cdecl`, one `float`: the elapsed time).
const ELAPSED_006377C0: u32 = 0x0063_77c0;
/// The player (a pointer global), its parent cell (`008d6f30`), the cell test `004518e0`
/// (`u8` result), the camera chain (`0045c670` with no arguments, `00558310`, `0045bb80`: a
/// position whose `z` is at +8) and `TESObjectREFR::GetRelevantWaterHeight` (`0057b0a0`, `float`
/// in `ST0`).
const PLAYER: u32 = 0x011d_ea3c;
const PLAYER_PARENT_CELL: u32 = 0x008d_6f30;
const CELL_TEST_004518E0: u32 = 0x0045_18e0;
const CAMERA_HOLDER_GET: u32 = 0x0045_c670;
const CAMERA_NODE_00558310: u32 = 0x0055_8310;
const NODE_POSITION_0045BB80: u32 = 0x0045_bb80;
const RELEVANT_WATER_HEIGHT: u32 = 0x0057_b0a0;
/// The child's property by type (`NiAVObject::GetProperty`, `00a59d30`, `RET 4`, type 3), the
/// property's kind (`00441110`, compared with 0x12), the child's particle data (`005495f0`),
/// its 16-bit count (`004567e0`), the property's word (`005f5f80`) and the data's 16-bit value
/// `004a8ae0` (engine map: `TESActorBaseData::GetFatigue`, folded code).
const GET_PROPERTY: u32 = 0x00a5_9d30;
const PROPERTY_KIND_00441110: u32 = 0x0044_1110;
const CHILD_DATA_005495F0: u32 = 0x0054_95f0;
const DATA_COUNT_004567E0: u32 = 0x0045_67e0;
const PROPERTY_COUNT_005F5F80: u32 = 0x005f_5f80;
const DATA_VALUE_004A8AE0: u32 = 0x004a_8ae0;
/// The root's `00450f90` (`RET 4`: the underwater flag).
const ROOT_SET_FLAG_00450F90: u32 = 0x0045_0f90;

/// `fn_00636ed0`'s property lookup: type 3 when its kind (`00441110`) is 0x12, else 0.
fn kind_0x12_property(e: &mut Engine, child: u32) -> u32 {
    if e.call(GET_PROPERTY, &args![child, 3u32]).u32() == 0 {
        return 0;
    }
    let property = e.call(GET_PROPERTY, &args![child, 3u32]).u32();
    if e.call(PROPERTY_KIND_00441110, &args![property]).u32() == 0x12 {
        e.call(GET_PROPERTY, &args![child, 3u32]).u32()
    } else {
        0
    }
}

/// `fn_00636ed0`'s per-child step: scales the child's active count by `factor` (as bits) and
/// adds its share to `fBirthRate`.
fn scale_child(e: &mut Engine, this: Ptr<Precipitation>, child: u32, property: u32, factor: u32) {
    let data = e.call(CHILD_DATA_005495F0, &args![child]).u32();
    let count = i32::from(e.call(DATA_COUNT_004567E0, &args![data]).u16());
    let divisor = e.call(PROPERTY_COUNT_005F5F80, &args![property]).i32();
    // `CDQ; IDIV`: a zero divisor faults in the exe as here.
    let per_unit = (count / divisor) as u16;
    let word = e.call(PROPERTY_COUNT_005F5F80, &args![property]).i32();
    let scaled = crate::x87::fistp_chop(x87_mul(f64::from(word), x87_m32(factor))) as u16;
    let product = per_unit.wrapping_mul(scaled);
    let current = e.vcall(data, 0xa8, &args![]).u16();
    if product != current {
        let full = u32::from(per_unit) * u32::from(scaled);
        e.vcall(data, 0xa4, &args![full]);
        if scaled != 0 {
            let value = e.call(DATA_VALUE_004A8AE0, &args![data]).u16();
            e.vcall(data, 0x8c, &args![u32::from(value)]);
        } else {
            e.vcall(data, 0x8c, &args![0u32]);
        }
    }
    let word = e.call(PROPERTY_COUNT_005F5F80, &args![property]).i32();
    let rate = e.get(this, Precipitation::fBirthRate).to_bits();
    let sum = x87_add(x87_mul(f64::from(word), x87_m32(factor)), x87_m32(rate));
    e.set(
        this,
        Precipitation::fBirthRate,
        f32::from_bits(x87_store32(sum)),
    );
}

/// `fn_00636ed0`'s loop over the children of the root at `this + offset`.
fn scale_children(e: &mut Engine, this: Ptr<Precipitation>, offset: u32, factor: u32) {
    let slot = this.addr().wrapping_add(offset);
    let mut index = 0u32;
    loop {
        let root = pointer_get(e, slot);
        let child = e.call(CHILD_AT, &args![root, index]).u32();
        index = index.wrapping_add(1);
        if child == 0 {
            break;
        }
        let property = kind_0x12_property(e, child);
        if property != 0 {
            scale_child(e, this, child, property, factor);
        }
    }
}

/// `fn_00636ed0`'s load of a weather's precipitation model (`fn_006377e0`, the model's path
/// from slot +0x14, `Load_ov2`).
fn load_weather_model(e: &mut Engine, this: Ptr<Precipitation>, weather: u32) {
    let model = fn_006377e0(e, Ptr::new(weather));
    let path = e.vcall(model.addr(), SLOT_MODEL_PATH, &args![]).u32();
    precipitation_load_ov2(e, this, Ptr::new(path));
}

// Translated from 00636ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the precipitation's per-frame update. `thiscall`, `RET 4` (a
/// `float`, passed on to `006377c0`). From the sky (`Sky::GetInstance`): an underwater flag
/// (set when `00476c90` answers 1, or 0 on its second call), the current (`0044edb0`) and
/// other (`00825c00`) weathers and a fraction (`006447d0`); a low bound (the current weather's
/// `004ed230` with 6, the `float` at `010231e0` and 0.0; 0.0 without one) and a high bound (the
/// other weather's `004ed230` with 7, 1.0 and the `float` at `01017d00`; that `float` without
/// one). When the singleton test `0047c850` passes, both roots are detached and cleared and
/// the models of the other and the current weather are loaded. Otherwise a queued model whose
/// `0048cee0` and path match the current weather's model is loaded once the loader finds it
/// (`ModelLoader::FindModel`, then `0045a5e0`), and a queued model that does not match is
/// replaced by `Load` with the weather's model; without a queued model, `Load` runs when the
/// sky's `00634870` passes and there is a current weather. Then: the last root is detached
/// when the fraction is at least 1.0; `fBirthRate` is reset; with a root, `006377c0` runs, the
/// underwater flag is also set when the camera is below the player's relevant water height
/// (player in a cell that passes `004518e0`), each child of the current root is scaled by
/// `(fraction - low) / (1 - low)` (0 unless `low < fraction`) and each of the last root by
/// `1 - fraction / high` (0 unless `fraction < high`), adding to `fBirthRate`. Finally each
/// root gets the underwater flag (`00450f90`).
// Translation incomplete: 00636ed0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the 00633c90 local) is not translated.
pub fn fn_00636ed0(e: &mut Engine, this: Ptr<Precipitation>, elapsed: f32) {
    let current_slot = this.addr().wrapping_add(4);
    let last_slot = this.addr().wrapping_add(8);
    let sky = e.call(SKY_GET_INSTANCE, &args![]).u32();
    let mut underwater = e.call(SKY_MODE_00476C90, &args![sky]).u32() == 1
        || e.call(SKY_MODE_00476C90, &args![sky]).u32() == 0;
    let current = e.call(SKY_WEATHER_0044EDB0, &args![sky]).u32();
    let other = e.call(SKY_WEATHER_00825C00, &args![sky]).u32();
    let fraction = x87_store32(e.call(SKY_FRACTION_006447D0, &args![sky]).f64());
    let low = if current != 0 {
        let argument = x87_quiet_bits(e.global::<u32>(WEATHER_ARGUMENT_010231E0));
        let value = e
            .call(
                WEATHER_VALUE_004ED230,
                &args![current, 6u32, argument, 0.0f32],
            )
            .f64();
        x87_store32(value)
    } else {
        x87_store32(0.0)
    };
    let low = x87_quiet_bits(low);
    let high = if other != 0 {
        let fallback = x87_quiet_bits(e.global::<u32>(WEATHER_FALLBACK_01017D00));
        let value = e
            .call(
                WEATHER_VALUE_004ED230,
                &args![other, 7u32, 1.0f32, fallback],
            )
            .f64();
        x87_store32(value)
    } else {
        x87_quiet_bits(e.global::<u32>(WEATHER_FALLBACK_01017D00))
    };
    let high = x87_quiet_bits(high);

    let singleton = e.global::<u32>(SINGLETON_011DE45C);
    if e.call(SINGLETON_TEST_0047C850, &args![singleton]).u8() != 0 {
        detach_root(e, this, 8);
        detach_root(e, this, 4);
        if e.call(SKY_WEATHER_00825C00, &args![sky]).u32() != 0 {
            let weather = e.call(SKY_WEATHER_00825C00, &args![sky]).u32();
            load_weather_model(e, this, weather);
        }
        if e.call(SKY_WEATHER_0044EDB0, &args![sky]).u32() != 0 {
            let weather = e.call(SKY_WEATHER_0044EDB0, &args![sky]).u32();
            load_weather_model(e, this, weather);
        }
    } else if e.get(this, Precipitation::pQueuedModel) != 0 {
        let model = fn_006377e0(e, Ptr::new(current)).addr();
        let mut done = false;
        if model != 0 {
            let queued = e.get(this, Precipitation::pQueuedModel);
            let queued_test = e.call(MODEL_TEST_0048CEE0, &args![queued]).u32();
            if queued_test == e.call(MODEL_TEST_0048CEE0, &args![model]).u32() {
                let model_path = e.vcall(model, SLOT_MODEL_PATH, &args![]).u32();
                let queued = e.get(this, Precipitation::pQueuedModel);
                let queued_path = e.vcall(queued, SLOT_MODEL_PATH, &args![]).u32();
                if e.call(STRING_COMPARE, &args![queued_path, model_path])
                    .u32()
                    == 0
                {
                    let model = fn_006377e0(e, Ptr::new(current)).addr();
                    let path = e.vcall(model, SLOT_MODEL_PATH, &args![]).u32();
                    e.with_stack(4, |e, local| {
                        e.call(LOCAL_CONSTRUCT_00633C90, &args![local, 0u32]);
                        let loader = e.global::<u32>(MODEL_LOADER);
                        if e.call(MODEL_LOADER_FIND_MODEL, &args![loader, path, local])
                            .u8()
                            != 0
                        {
                            e.set(this, Precipitation::pQueuedModel, 0);
                            precipitation_load_ov2(e, this, Ptr::new(path));
                            let loader = e.global::<u32>(MODEL_LOADER);
                            e.call(MODEL_LOADER_AFTER_LOAD_0045A5E0, &args![loader, path]);
                        }
                        e.call(LOCAL_DESTRUCT_0040C110, &args![local]);
                    });
                    done = true;
                }
            }
        }
        if !done {
            e.set(this, Precipitation::pQueuedModel, 0);
            precipitation_load(e, this, Ptr::new(model));
        }
    } else if e.call(SKY_TEST_00634870, &args![sky]).u8() != 0 && current != 0 {
        let model = fn_006377e0(e, Ptr::new(current));
        precipitation_load(e, this, model);
    }

    // `FCOMP double 1.0; TEST AH,1`: detached unless below 1.0 or unordered.
    let one: f64 = e.global(ONE_01012070);
    let fraction_value = x87_load32(fraction);
    if fraction_value >= one {
        detach_root(e, this, 8);
    }
    e.set(
        this,
        Precipitation::fBirthRate,
        f32::from_bits(x87_store32(0.0)),
    );
    if pointer_get(e, current_slot) != 0 || pointer_get(e, last_slot) != 0 {
        e.call(ELAPSED_006377C0, &args![x87_quiet(elapsed)]);
        let player = e.global::<u32>(PLAYER);
        if player != 0 && e.call(PLAYER_PARENT_CELL, &args![player]).u32() != 0 {
            let cell = e.call(PLAYER_PARENT_CELL, &args![player]).u32();
            if e.call(CELL_TEST_004518E0, &args![cell]).u8() != 0 {
                let holder = e.call(CAMERA_HOLDER_GET, &args![]).u32();
                let node = e.call(CAMERA_NODE_00558310, &args![holder]).u32();
                let position = e.call(NODE_POSITION_0045BB80, &args![node]).u32();
                let water = e.call(RELEVANT_WATER_HEIGHT, &args![player]).f64();
                let z = x87_load32(e.mem.u32(position.wrapping_add(8)));
                if z < water {
                    underwater = true;
                }
            }
        }
        if pointer_get(e, current_slot) != 0 {
            let low_value = x87_load32(low);
            let mut factor = x87_store32(0.0);
            if low_value < fraction_value {
                let numerator = x87_sub(fraction_value, x87_m32(low));
                let denominator = x87_sub(1.0, low_value);
                factor = x87_store32(x87_div(numerator, denominator));
            }
            scale_children(e, this, 4, factor);
        }
        if pointer_get(e, last_slot) != 0 {
            let high_value = x87_load32(high);
            let mut factor = x87_store32(0.0);
            if high_value > fraction_value {
                let ratio = x87_div(fraction_value, x87_m32(high));
                factor = x87_store32(x87_sub(1.0, ratio));
            }
            scale_children(e, this, 8, factor);
        }
    }
    for slot in [current_slot, last_slot] {
        if pointer_get(e, slot) != 0 {
            let root = pointer_get(e, slot);
            e.call(ROOT_SET_FLAG_00450F90, &args![root, u32::from(underwater)]);
        }
    }
}

// Translated from 006377e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: answers `this + 0xc8` (a member's address; by its callers the
/// weather's precipitation model). `thiscall`, no stack words.
pub fn fn_006377e0(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0xc8)
}

// Translated from 00637800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Precipitation::FlushAll` (Xbox PDB). `thiscall`, no stack words. Detaches the last and then
/// the current precipitation root from the parent (slot +0xe8) when they hold a node, and
/// clears each detached one (`0066b0d0` with null).
pub fn precipitation_flush_all(e: &mut Engine, this: Ptr<Precipitation>) {
    detach_root(e, this, 8);
    detach_root(e, this, 4);
}

// Translated from 00639730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a table lookup of an angle. `cdecl`, three words (a `float` and
/// two `float` pointers). The index is `_ftol2(([01028338] / [011ccb34]) * angle) & 0x1ff`
/// (x87, in that order); the first out pointer gets the index's entry of the table at
/// `011f52e0`, the second that of the table at `011f4ae0` (`FLD`/`FSTP` copies).
pub fn fn_00639730(e: &mut Engine, angle: f32, first: Ptr, second: Ptr) {
    let divisor = x87_load32(e.global::<u32>(TABLE_DIVISOR_011CCB34));
    let numerator: f64 = e.global(TABLE_NUMERATOR_01028338);
    let scaled = x87_mul(x87_div(numerator, divisor), x87_m32(angle.to_bits()));
    let index = e.call(FTOL, &args![scaled]).u32() & 0x1ff;
    let a = x87_quiet_bits(e.global::<u32>(TABLE_011F52E0 + 4 * index));
    e.mem.set_u32(first.addr(), a);
    let b = x87_quiet_bits(e.global::<u32>(TABLE_011F4AE0 + 4 * index));
    e.mem.set_u32(second.addr(), b);
}

// Translated from 00639940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the engine map places it here; its references are vtable
/// entries): clears the six words at +4 to +0x18 and the byte at +0, sets the mode bits with
/// `fn_006399a0(0)` and answers `this`. `thiscall`, no stack words.
pub fn fn_00639940(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    for offset in [4u32, 8, 0xc, 0x10, 0x14, 0x18] {
        e.mem.set_u32(base.wrapping_add(offset), 0);
    }
    e.mem.set_u8(base, 0);
    fn_006399a0(e, this, 0);
    this
}

// Translated from 006399a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the byte at +0 becomes `(byte & 0xf9) | fn_006399e0(value)`.
/// `thiscall`, `RET 4`.
pub fn fn_006399a0(e: &mut Engine, this: Ptr, value: u32) {
    let bits = fn_006399e0(e, this, value);
    let byte = e.mem.u8(this.addr());
    e.mem.set_u8(this.addr(), (byte & 0xf9) | bits);
}

// Translated from 006399e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: 2 for 0, 3 for 0x4000 and 3 for anything else (`AL`).
/// `thiscall` (`this` unused), `RET 4`.
pub fn fn_006399e0(_e: &mut Engine, _this: Ptr, value: u32) -> u8 {
    match value {
        0 => 2,
        0x4000 => 3,
        _ => 3,
    }
}

// Translated from 00639aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `float` at +0x48 (`FLD`, in `ST0`). `thiscall`.
pub fn fn_00639aa0(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x48)))
}

// Translated from 00639b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `float` at +0x5c (`FLD`, in `ST0`). `thiscall`.
pub fn fn_00639b20(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr().wrapping_add(0x5c)))
}

// Translated from 00639b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the word at +0x3c. `thiscall`.
pub fn fn_00639b40(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x3c))
}

// Translated from 00639b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysEmitterCtlr::GetBirthRateInterpolator` (Xbox PDB). `thiscall`, no stack words. The
/// interpolator held at +0x34 (`NiPointer` value) cast to the record at `011f3738`
/// (`00653270`) when that succeeds; otherwise, cast to the record at `011f3e58`, the blend
/// interpolator's interpolator at its highest weighted index (`00a368f0`, `00495d50`) unless
/// that index is the invalid byte at `01096968`; otherwise 0.
pub fn ni_p_sys_emitter_ctlr_get_birth_rate_interpolator(e: &mut Engine, this: Ptr) -> u32 {
    let slot = this.addr().wrapping_add(0x34);
    let interpolator = pointer_get(e, slot);
    let cast = e
        .call(
            KIND_OF_CAST,
            &args![INTERPOLATOR_RTTI_011F3738, interpolator],
        )
        .u32();
    if cast != 0 {
        return cast;
    }
    let interpolator = pointer_get(e, slot);
    let blend = e
        .call(KIND_OF_CAST, &args![BLEND_RTTI_011F3E58, interpolator])
        .u32();
    if blend != 0 {
        let index = e.call(BLEND_HIGHEST_WEIGHTED_INDEX, &args![blend]).u8();
        if index != e.global::<u8>(INVALID_INDEX_01096968) {
            return e
                .call(BLEND_GET_INTERPOLATOR, &args![blend, u32::from(index)])
                .u32();
        }
    }
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0063_9b00, fn_00639b00(Ptr) -> u16),
        entry!(
            0x0063_6af0,
            precipitation_scalar_deleting_destructor(Ptr<Precipitation>, u32) -> Ptr<Precipitation>
        ),
        entry!(0x0063_6b20, fn_00636b20(Ptr<Precipitation>)),
        entry!(0x0063_6bb0, fn_00636bb0(Ptr<Precipitation>)),
        entry!(0x0063_6bd0, precipitation_load(Ptr<Precipitation>, Ptr)),
        entry!(0x0063_6c40, precipitation_load_ov2(Ptr<Precipitation>, Ptr)),
        entry!(0x0063_6ed0, fn_00636ed0(Ptr<Precipitation>, f32)),
        entry!(0x0063_77e0, fn_006377e0(Ptr) -> Ptr),
        entry!(0x0063_7800, precipitation_flush_all(Ptr<Precipitation>)),
        entry!(0x0063_9730, fn_00639730(f32, Ptr, Ptr)),
        entry!(0x0063_9940, fn_00639940(Ptr) -> Ptr),
        entry!(0x0063_99a0, fn_006399a0(Ptr, u32)),
        entry!(0x0063_99e0, fn_006399e0(Ptr, u32) -> u8),
        entry!(0x0063_9aa0, fn_00639aa0(Ptr) -> f32),
        entry!(0x0063_9b20, fn_00639b20(Ptr) -> f32),
        entry!(0x0063_9b40, fn_00639b40(Ptr) -> u32),
        entry!(
            0x0063_9b60,
            ni_p_sys_emitter_ctlr_get_birth_rate_interpolator(Ptr) -> u32
        ),
    ]
    .into_iter()
    .chain(funcs_n086())
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_00639b00_passes_this_plus_54_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_009938B0, |_, a| Ret {
            eax: a[0] | 0x5_0000,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0063_9b00, &args![0x4100u32]).u16(), 0x4154);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_009938B0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x54]);
    }
}

#[cfg(test)]
mod b064_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    const DETACH: u32 = 0x0f00_1000;
    const ATTACH: u32 = 0x0f00_1004;
    const MODEL_PATH: u32 = 0x0f00_1008;

    /// The `NiPointer` helpers as plain word reads and writes, a parent whose slots +0xe8
    /// and +0xdc are doubles, and a precipitation object with that parent.
    fn rig() -> (Engine, Ptr<Precipitation>, u32) {
        let mut e = Engine::new();
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINTER_ASSIGN_POINTER, |e, a| {
            let v = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], v);
            Ret::default()
        });
        e.register(POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(DETACH, |_, _| Ret::default());
        e.register(ATTACH, |_, _| Ret::default());
        // A model's path slot answers the word at the model's +4.
        e.register(MODEL_PATH, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let parent_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(parent_vtable + SLOT_DETACH_CHILD, DETACH);
        e.mem.set_u32(parent_vtable + SLOT_ATTACH_CHILD, ATTACH);
        let parent = e.mem.alloc(0x10);
        e.mem.set_u32(parent, parent_vtable);
        let this: Ptr<Precipitation> = e.new_object();
        e.set(this, Precipitation::pParent, parent);
        (e, this, parent)
    }

    /// A model object whose slot +0x14 answers `path`.
    fn model(e: &mut Engine, path: u32) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + SLOT_MODEL_PATH, MODEL_PATH);
        let model = e.mem.alloc(0x10);
        e.mem.set_u32(model, vtable);
        e.mem.set_u32(model + 4, path);
        model
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_bit_0() {
        for (flags, freed) in [(1u32, true), (2, false)] {
            let (mut e, this, _) = rig();
            e.register(OPERATOR_DELETE, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            let r = e.call(0x0063_6af0, &args![this, flags]).u32();
            assert_eq!(r, this.addr());
            assert_eq!(e.mem.u32(this.addr()), PRECIPITATION_VTABLE);
            let want: Vec<Vec<u32>> = if freed {
                vec![vec![this.addr()]]
            } else {
                vec![]
            };
            assert_eq!(calls(&e, OPERATOR_DELETE), want);
        }
    }

    #[test]
    fn destructor_clears_both_roots_then_destroys_them() {
        let (mut e, this, _) = rig();
        let base = this.addr();
        e.mem.set_u32(base + 4, 0x11);
        e.mem.set_u32(base + 8, 0x22);
        e.set(this, Precipitation::pQueuedModel, 0x33);
        e.call_log = Some(vec![]);
        e.call(0x0063_6b20, &args![this]);
        assert_eq!(e.mem.u32(base), PRECIPITATION_VTABLE);
        assert_eq!(
            calls(&e, POINTER_ASSIGN),
            vec![vec![base + 4, 0], vec![base + 8, 0]]
        );
        assert_eq!(
            calls(&e, POINTER_DESTRUCT),
            vec![vec![base + 8], vec![base + 4]]
        );
        assert_eq!(e.get(this, Precipitation::pQueuedModel), 0);
    }

    #[test]
    fn fn_00636bb0_sets_the_parent_from_the_global_pointer() {
        let (mut e, this, _) = rig();
        e.register(POINTER_GLOBAL_011DEDA4_GET, |_, a| {
            assert!(a.is_empty());
            0x4444u32.into_ret()
        });
        e.call(0x0063_6bb0, &args![this]);
        assert_eq!(e.get(this, Precipitation::pParent), 0x4444);
    }

    #[test]
    fn load_queues_an_accepted_model_and_skips_while_the_last_root_is_held() {
        let (mut e, this, _) = rig();
        e.map(0x011c_3000, 0x1000);
        e.set_global(MODEL_LOADER, 0x7777u32);
        e.register(MODEL_TEST_0048CEE0, |_, _| 1u32.into_ret());
        e.register(MODEL_LOADER_QUEUE_MODEL, |_, _| Ret::default());
        e.register(MODEL_LOADER_LOAD_FILE, |_, _| Ret::default());
        let m = model(&mut e, 0x5150);
        e.call_log = Some(vec![]);
        e.call(0x0063_6bd0, &args![this, m]);
        assert_eq!(
            calls(&e, MODEL_LOADER_QUEUE_MODEL),
            vec![vec![0x7777, 0x5150, 0, 0, 0, 0, 0, 0]]
        );
        assert_eq!(e.get(this, Precipitation::pQueuedModel), m);

        // A held last root: nothing at all.
        e.set(this, Precipitation::pQueuedModel, 0);
        e.mem.set_u32(this.addr() + 8, 0x99);
        e.call_log = Some(vec![]);
        e.call(0x0063_6bd0, &args![this, m]);
        assert!(calls(&e, MODEL_TEST_0048CEE0).is_empty());
        assert_eq!(e.get(this, Precipitation::pQueuedModel), 0);
    }

    #[test]
    fn load_of_a_rejected_model_loads_no_file() {
        let (mut e, this, _) = rig();
        e.register(MODEL_TEST_0048CEE0, |_, _| 0u32.into_ret());
        let m = model(&mut e, 0x5150);
        e.call_log = Some(vec![]);
        e.call(0x0063_6bd0, &args![this, m]);
        assert_eq!(calls(&e, MODEL_TEST_0048CEE0), vec![vec![m]]);
        assert_eq!(e.get(this, Precipitation::pQueuedModel), 0);
        // `Load_ov2(null)` with an empty current root only reads it.
        assert_eq!(
            calls(&e, POINTER_GET),
            vec![vec![this.addr() + 8], vec![this.addr() + 4]]
        );
    }

    #[test]
    fn load_ov2_rotates_the_roots_and_builds_the_cast_children() {
        let (mut e, this, parent) = rig();
        let base = this.addr();
        e.map(0x011c_3000, 0x1000);
        e.set_global(MODEL_LOADER, 0x7777u32);
        e.mem.set_u32(base + 4, 0xaaaa);
        let path = e.mem.alloc(16);
        e.mem.set_cstr(path, b"Sky\\Rain.NIF");
        // The loaded model: two children; the first answers an object, the second none.
        let loaded_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(loaded_vtable + 0xa4, 0x0f00_2000);
        e.register(0x0f00_2000, |_, _| Ret::default());
        let loaded = e.mem.alloc(8);
        e.mem.set_u32(loaded, loaded_vtable);
        let child_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(child_vtable + 0xc, 0x0f00_2004);
        e.register(0x0f00_2004, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let first = e.mem.alloc(8);
        e.mem.set_u32(first, child_vtable);
        e.mem.set_u32(first + 4, 0x6000);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, child_vtable);
        e.register_double(MODEL_LOADER_LOAD_FILE, move |_, _| loaded.into_ret());
        e.register(STRING_COMPARE, |_, _| 1u32.into_ret());
        e.register(LOCAL_INIT_0043D410, |_, _| Ret::default());
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        // The new node shares the parent's vtable (its attach slot).
        let parent_vtable = e.mem.u32(parent);
        e.register_double(NI_NODE_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0], parent_vtable);
            a[0].into_ret()
        });
        e.register(CHILD_COUNT, |_, _| 2u32.into_ret());
        e.register_double(CHILD_AT, move |_, a| {
            [first, second][a[1] as usize].into_ret()
        });
        e.register(CHILD_OBJECT_0045BC00, |_, a| (a[0] + 1).into_ret());
        e.register(KIND_OF_CAST, |_, a| a[1].into_ret());
        e.register(PARENT_0046130, |_, _| 0x8888u32.into_ret());
        e.register(BUILD_CHILD_00637890, |_, _| 0x9999u32.into_ret());
        e.register(UPDATE_PROPERTIES, |_, _| Ret::default());
        e.set(this, Precipitation::fBirthRate, 5.0);
        e.call_log = Some(vec![]);
        e.call(0x0063_6c40, &args![this, path]);

        assert_eq!(calls(&e, DETACH), vec![vec![parent, 0xaaaa]]);
        assert_eq!(e.mem.u32(base + 8), 0xaaaa);
        assert_eq!(
            calls(&e, MODEL_LOADER_LOAD_FILE),
            vec![vec![0x7777, path, 0, 1, 0, 0, 0]]
        );
        assert_eq!(calls(&e, STRING_COMPARE), vec![vec![path, SNOW_MODEL_PATH]]);
        let local = calls(&e, LOCAL_INIT_0043D410)[0][0];
        assert_eq!(calls(&e, LOCAL_INIT_0043D410), vec![vec![local, 0, 0, 0]]);
        assert_eq!(calls(&e, 0x0f00_2000), vec![vec![loaded, local, 0]]);
        let node = e.mem.u32(base + 4);
        assert_eq!(calls(&e, NI_NODE_CONSTRUCT), vec![vec![node, 0]]);
        assert_eq!(e.get(this, Precipitation::fBirthRate), 0.0);
        assert_eq!(
            calls(&e, KIND_OF_CAST),
            vec![vec![CAST_RTTI_012024E0, 0x6001]]
        );
        assert_eq!(
            calls(&e, BUILD_CHILD_00637890),
            vec![vec![base, 0x6001, 0x8888, 0]]
        );
        assert_eq!(
            calls(&e, ATTACH),
            vec![
                vec![parent, 0xaaaa, 1],
                vec![node, 0x9999, 1],
                vec![parent, node, 1]
            ]
        );
        assert_eq!(calls(&e, UPDATE_PROPERTIES), vec![vec![node]]);
    }

    #[test]
    fn load_ov2_with_an_empty_path_or_no_file_stops() {
        let (mut e, this, _) = rig();
        e.map(0x011c_3000, 0x1000);
        e.register(MODEL_LOADER_LOAD_FILE, |_, _| 0u32.into_ret());
        let path = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x0063_6c40, &args![this, path]);
        assert!(calls(&e, MODEL_LOADER_LOAD_FILE).is_empty());
        e.mem.set_u8(path, 0x61);
        e.call(0x0063_6c40, &args![this, path]);
        assert_eq!(calls(&e, MODEL_LOADER_LOAD_FILE).len(), 1);
        assert!(calls(&e, POINTER_ASSIGN).is_empty());
    }

    /// Doubles for the update: the sky's accessors answer the values given.
    fn update_rig(test: u8, fraction: f32, current: u32) -> (Engine, Ptr<Precipitation>, u32) {
        let (mut e, this, parent) = rig();
        for page in [
            0x0101_7000,
            0x0101_2000,
            0x0102_3000,
            0x011d_e000,
            0x011c_3000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ONE_01012070, 1.0f64);
        e.set_global(WEATHER_ARGUMENT_010231E0, 0.25f32);
        e.set_global(WEATHER_FALLBACK_01017D00, 0.001f32);
        e.register(SKY_GET_INSTANCE, |_, _| 0x5000u32.into_ret());
        e.register(SKY_MODE_00476C90, |_, _| 2u32.into_ret());
        e.register_double(SKY_WEATHER_0044EDB0, move |_, _| current.into_ret());
        e.register(SKY_WEATHER_00825C00, |_, _| 0u32.into_ret());
        e.register_double(SKY_FRACTION_006447D0, move |_, _| Ret {
            st0: f64::from(fraction),
            ..Ret::default()
        });
        e.register(WEATHER_VALUE_004ED230, |_, a| Ret {
            st0: if a[1] == 6 { 0.5 } else { 2.0 },
            ..Ret::default()
        });
        e.register_double(SINGLETON_TEST_0047C850, move |_, _| {
            u32::from(test).into_ret()
        });
        e.register(SKY_TEST_00634870, |_, _| 0u32.into_ret());
        e.register(ROOT_SET_FLAG_00450F90, |_, _| Ret::default());
        e.register(ELAPSED_006377C0, |_, _| Ret::default());
        (e, this, parent)
    }

    #[test]
    fn update_with_the_singleton_test_reloads_the_weathers_models() {
        let (mut e, this, parent) = update_rig(1, 0.5, 0);
        let base = this.addr();
        e.mem.set_u32(base + 8, 0xbbbb);
        // The current weather's model (at weather + 0xc8) has an empty path.
        let weather_block = e.mem.alloc(0x100);
        let m = model(&mut e, 0);
        let vtable = e.mem.u32(m);
        e.mem.set_u32(weather_block + 0xc8, vtable);
        e.register_double(SKY_WEATHER_0044EDB0, move |_, _| weather_block.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0063_6ed0, &args![this, 0.1f32]);
        assert_eq!(calls(&e, DETACH), vec![vec![parent, 0xbbbb]]);
        assert_eq!(e.mem.u32(base + 8), 0);
        // The low bound came from the weather with 6, 0.25, 0.0; the high one is the fallback.
        assert_eq!(
            calls(&e, WEATHER_VALUE_004ED230),
            vec![vec![weather_block, 6, 0.25f32.to_bits(), 0]]
        );
        // Only the current weather's model path was asked for (the other weather is null).
        assert_eq!(calls(&e, MODEL_PATH), vec![vec![weather_block + 0xc8]]);
        // No root: no elapsed-time call and no flags.
        assert!(calls(&e, ELAPSED_006377C0).is_empty());
        assert!(calls(&e, ROOT_SET_FLAG_00450F90).is_empty());
        assert_eq!(e.get(this, Precipitation::fBirthRate), 0.0);
    }

    #[test]
    fn update_scales_the_current_roots_children_and_sets_the_underwater_flag() {
        let (mut e, this, _) = update_rig(0, 0.75, 0x100);
        let base = this.addr();
        e.mem.set_u32(base + 4, 0xc000);
        e.register(SKY_MODE_00476C90, |_, _| 1u32.into_ret());
        // The root's children: one with a kind-0x12 property, then none.
        e.register(CHILD_AT, |_, a| {
            if a[1] == 0 { 0xd000u32 } else { 0 }.into_ret()
        });
        e.register(GET_PROPERTY, |_, _| 0xe000u32.into_ret());
        e.register(PROPERTY_KIND_00441110, |_, _| 0x12u32.into_ret());
        let data_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(data_vtable + 0x8c, 0x0f00_3000);
        e.mem.set_u32(data_vtable + 0xa4, 0x0f00_3004);
        e.mem.set_u32(data_vtable + 0xa8, 0x0f00_3008);
        e.register(0x0f00_3000, |_, _| Ret::default());
        e.register(0x0f00_3004, |_, _| Ret::default());
        e.register(0x0f00_3008, |_, _| 7u32.into_ret());
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, data_vtable);
        e.register_double(CHILD_DATA_005495F0, move |_, _| data.into_ret());
        e.register(DATA_COUNT_004567E0, |_, _| 100u32.into_ret());
        e.register(PROPERTY_COUNT_005F5F80, |_, _| 10u32.into_ret());
        e.register(DATA_VALUE_004A8AE0, |_, _| 0x1_0042u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0063_6ed0, &args![this, 0.1f32]);
        // A queued model is absent and 00634870 fails: nothing loads.
        assert!(calls(&e, MODEL_LOADER_LOAD_FILE).is_empty());
        // low = 0.5, fraction 0.75: factor (0.75 - 0.5) / 0.5 = 0.5; 10 * 0.5 = 5 per child.
        assert_eq!(calls(&e, ELAPSED_006377C0), vec![vec![0.1f32.to_bits()]]);
        // 100 / 10 = 10 per unit, times 5 = 50 differs from 7.
        assert_eq!(calls(&e, 0x0f00_3004), vec![vec![data, 50]]);
        assert_eq!(calls(&e, 0x0f00_3000), vec![vec![data, 0x42]]);
        assert_eq!(e.get(this, Precipitation::fBirthRate), 5.0);
        assert_eq!(calls(&e, ROOT_SET_FLAG_00450F90), vec![vec![0xc000, 1]]);
    }

    #[test]
    fn fn_006377e0_answers_the_member_at_0xc8() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0063_77e0, &args![0x1000u32]).u32(), 0x10c8);
    }

    #[test]
    fn flush_all_detaches_the_last_then_the_current_root() {
        let (mut e, this, parent) = rig();
        let base = this.addr();
        e.mem.set_u32(base + 4, 0x44);
        e.mem.set_u32(base + 8, 0x88);
        e.call_log = Some(vec![]);
        e.call(0x0063_7800, &args![this]);
        assert_eq!(
            calls(&e, DETACH),
            vec![vec![parent, 0x88], vec![parent, 0x44]]
        );
        assert_eq!(
            calls(&e, POINTER_ASSIGN),
            vec![vec![base + 8, 0], vec![base + 4, 0]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0063_7800, &args![this]);
        assert!(calls(&e, DETACH).is_empty());
    }

    #[test]
    fn fn_00639730_reads_both_tables_at_the_masked_index() {
        let mut e = Engine::new();
        for page in [0x011c_c000, 0x0102_8000, 0x011f_4000, 0x011f_5000] {
            e.map(page, 0x1000);
        }
        e.set_global(TABLE_DIVISOR_011CCB34, 2.0f32);
        e.set_global(TABLE_NUMERATOR_01028338, 512.0f64);
        e.set_global(TABLE_011F52E0 + 4 * 0x10, 0x7f80_0001u32);
        e.set_global(TABLE_011F4AE0 + 4 * 0x10, 0.5f32);
        e.register(FTOL, |_, a| {
            (f64::take(a, &mut 0).trunc() as i32 as u32).into_ret()
        });
        let out = e.mem.alloc(8);
        // 512 / 2 * 2.0625 = 528 = 0x210, masked to 0x10.
        e.call(0x0063_9730, &args![2.0625f32, out, out + 4]);
        assert_eq!(e.mem.u32(out), 0x7fc0_0001);
        assert_eq!(e.mem.f32(out + 4), 0.5);
    }

    #[test]
    fn fn_00639940_clears_the_words_and_sets_mode_2() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c);
        for offset in (0..0x1c).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        assert_eq!(e.call(0x0063_9940, &args![this]).u32(), this);
        assert_eq!(e.mem.u8(this), 2);
        assert_eq!(e.mem.bytes(this + 4, 0x18), vec![0; 0x18]);
        assert_eq!(e.mem.bytes(this + 1, 3), vec![0xff; 3]);
    }

    #[test]
    fn fn_006399a0_keeps_the_other_bits_and_fn_006399e0_maps_values() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        e.mem.set_u8(this, 0xff);
        e.call(0x0063_99a0, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this), 0xfb);
        e.mem.set_u8(this, 0xf0);
        e.call(0x0063_99a0, &args![this, 0x4000u32]);
        assert_eq!(e.mem.u8(this), 0xf3);
        for (value, want) in [(0u32, 2u8), (0x4000, 3), (7, 3)] {
            assert_eq!(e.call(0x0063_99e0, &args![this, value]).u8(), want);
        }
    }

    #[test]
    fn the_three_getters_read_their_members() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x48, 0x7f80_0002);
        e.mem.set_f32(this + 0x5c, -3.5);
        e.mem.set_u32(this + 0x3c, 0xabcd);
        assert!(e.call(0x0063_9aa0, &args![this]).f64().is_nan());
        assert_eq!(e.call(0x0063_9b20, &args![this]).f64(), -3.5);
        assert_eq!(e.call(0x0063_9b40, &args![this]).u32(), 0xabcd);
    }

    fn interpolator_case(direct: bool, blend: bool, index: u8) -> u32 {
        let mut e = Engine::new();
        e.map(0x0109_6000, 0x1000);
        e.set_global(INVALID_INDEX_01096968, 0xffu8);
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register_double(KIND_OF_CAST, move |_, a| {
            let ok = if a[0] == INTERPOLATOR_RTTI_011F3738 {
                direct
            } else {
                blend
            };
            if ok { a[1] + (a[0] & 0xf) } else { 0 }.into_ret()
        });
        e.register_double(BLEND_HIGHEST_WEIGHTED_INDEX, move |_, _| {
            (0xab00 | u32::from(index)).into_ret()
        });
        e.register(BLEND_GET_INTERPOLATOR, |_, a| (0x7000 + a[1]).into_ret());
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x34, 0x3000);
        e.call(0x0063_9b60, &args![this]).u32()
    }

    #[test]
    fn birth_rate_interpolator_casts_then_falls_back_to_the_blend() {
        assert_eq!(interpolator_case(true, true, 1), 0x3008);
        assert_eq!(interpolator_case(false, true, 2), 0x7002);
        assert_eq!(interpolator_case(false, true, 0xff), 0);
        assert_eq!(interpolator_case(false, false, 1), 0);
    }
}

/// The float `006377c0` adds to (`011ae5d0`).
const ACCUMULATOR_011AE5D0: u32 = 0x011a_e5d0;
/// `_rand` (`00ecadb8`, `cdecl`).
const RAND: u32 = 0x00ec_adb8;
/// The doubles of the random helpers: the divisor (`01029790`), the factor
/// (`01011590`) and the addend (`0101a6b0`) (32767, 2 and -1 in the
/// decompiler's reading).
const RANDOM_DIVISOR_01029790: u32 = 0x0102_9790;
const RANDOM_FACTOR_01011590: u32 = 0x0101_1590;
const RANDOM_ADDEND_0101A6B0: u32 = 0x0101_a6b0;
/// `00416870` (`thiscall`, `RET 0xc`: three floats): sets a point and
/// answers it.
const POINT_SET_00416870: u32 = 0x0041_6870;
/// `00658930` (`thiscall`, plain `RET`): the 16-bit count at `+0xa` of the
/// array at `+0x18`, zero-extended.
const ARRAY_COUNT_00658930: u32 = 0x0065_8930;
/// `00877a30` (`thiscall`, `RET 4`: an index): the element's slot.
const ARRAY_AT_00877A30: u32 = 0x0087_7a30;
/// `00401000` (`cdecl`, one word: bytes): allocates a block.
const ALLOCATE_00401000: u32 = 0x0040_1000;
/// `00a5d3a0` (`thiscall`): the `NiObject` base constructor.
const NI_OBJECT_CONSTRUCTOR: u32 = 0x00a5_d3a0;
/// The `NiAdditionalGeometryData` vtable (RTTI
/// `.?AVNiAdditionalGeometryData@@`).
const ADDITIONAL_GEOMETRY_DATA_VTABLE: u32 = 0x0102_e274;
/// `00540830` (`thiscall`, `RET 8`) and `005e03d0` (`thiscall`): run on the
/// member at `+0x1c`.
const MEMBER_INIT_00540830: u32 = 0x0054_0830;
const MEMBER_INIT_005E03D0: u32 = 0x005e_03d0;
/// `00401050` (`stdcall`, four words: block, element size, count,
/// constructor): runs the constructor on each element.
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;

// Translated from 006377c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one float): adds `value` to the
/// float at `011ae5d0` (`FLD; FADD dword; FSTP`).
pub fn fn_006377c0(e: &mut Engine, value: f32) {
    let sum = x87_add(
        x87_load32(e.mem.u32(ACCUMULATOR_011AE5D0)),
        x87_m32(value.to_bits()),
    );
    e.mem.set_u32(ACCUMULATOR_011AE5D0, x87_store32(sum));
}

// Translated from 00639780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no words): the random number
/// (`006397b0`) divided by the double at `01029790`, stored as a float and
/// answered in `ST0`.
pub fn fn_00639780(e: &mut Engine) -> f32 {
    let random = fn_006397b0(e) as i32;
    let divisor = f64::from_bits(e.mem.u64(RANDOM_DIVISOR_01029790));
    f32::from_bits(x87_store32(x87_div(f64::from(random), divisor)))
}

// Translated from 006397b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no words): `_rand` (`00ecadb8`),
/// its answer passed on.
pub fn fn_006397b0(e: &mut Engine) -> u32 {
    e.call(RAND, &[]).u32()
}

// Translated from 006397c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, no words): the random number
/// (`006397b0`) times the double at `01011590`, divided by the one at
/// `01029790`, plus the one at `0101a6b0`, stored as a float and answered in
/// `ST0`.
pub fn fn_006397c0(e: &mut Engine) -> f32 {
    let random = fn_006397b0(e) as i32;
    let factor = f64::from_bits(e.mem.u64(RANDOM_FACTOR_01011590));
    let divisor = f64::from_bits(e.mem.u64(RANDOM_DIVISOR_01029790));
    let addend = f64::from_bits(e.mem.u64(RANDOM_ADDEND_0101A6B0));
    let value = x87_add(x87_div(x87_mul(f64::from(random), factor), divisor), addend);
    f32::from_bits(x87_store32(value))
}

// Translated from 006397f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`: three floats): a point
/// made by `00416870` in a stack temporary from the three floats (each
/// copied through `FLD`/`FSTP`), its three words copied to `this`.
pub fn fn_006397f0(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32) {
    e.with_stack(12, |e, temporary| {
        let point = e
            .call(
                POINT_SET_00416870,
                &args![
                    temporary,
                    x87_quiet_bits(x.to_bits()),
                    x87_quiet_bits(y.to_bits()),
                    x87_quiet_bits(z.to_bits())
                ],
            )
            .u32();
        for word in 0..3 {
            let value = e.mem.u32(point.wrapping_add(word * 4));
            e.mem.set_u32(this.addr().wrapping_add(word * 4), value);
        }
    });
}

// Translated from 00639830 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00639830 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// A constructor of a class derived from `NiAdditionalGeometryData`
/// (unnamed in the engine map; `thiscall`, `RET 0xc`): the `NiObject` base
/// constructor (`00a5d3a0`), the vtable `0102e274`, `00540830` on `+0x1c`
/// with 0 and 1, `+8` cleared, the low 16 bits of `kind` at `+0xc`, `count`
/// at `+0x10`, a block of `count` 0x1c-byte elements (`00401000`; all ones
/// as the size when `count * 0x1c` overflows) each built by `00639940`
/// (`00401050`) at `+0x14` (null when the allocation fails), `005e03d0` on
/// `+0x1c`, and `last` at `+0x18`. Returns `this`.
pub fn fn_00639830(e: &mut Engine, this: Ptr, kind: u32, count: u32, last: u32) -> Ptr {
    let base = this.addr();
    e.call(NI_OBJECT_CONSTRUCTOR, &args![base]);
    e.mem.set_u32(base, ADDITIONAL_GEOMETRY_DATA_VTABLE);
    e.call(MEMBER_INIT_00540830, &args![base + 0x1c, 0u32, 1u32]);
    e.mem.set_u32(base + 8, 0);
    e.mem.set_u16(base + 0xc, kind as u16);
    e.mem.set_u32(base + 0x10, count);
    let size = count.checked_mul(0x1c).unwrap_or(u32::MAX);
    let block = e.call(ALLOCATE_00401000, &args![size]).u32();
    let elements = if block != 0 {
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![block, 0x1cu32, count, 0x0063_9940u32],
        );
        block
    } else {
        0
    };
    e.mem.set_u32(base + 0x14, elements);
    e.call(MEMBER_INIT_005E03D0, &args![base + 0x1c]);
    e.mem.set_u32(base + 0x18, last);
    this
}

// Translated from 00639a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): the low 3 bits of
/// the 16-bit word at `+0xac` (`00639a30` with 7 and 0), zero-extended
/// from 16 bits.
pub fn fn_00639a10(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(fn_00639a30(e, this, 7, 0) as u16)
}

// Translated from 00639a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): the 16-bit word at
/// `+0xac` masked with the low 16 bits of `mask`, shifted right
/// (arithmetic) by the low 16 bits of `shift` (the CPU uses its low 5).
pub fn fn_00639a30(e: &mut Engine, this: Ptr, mask: u32, shift: u32) -> i32 {
    let bits = u32::from(e.mem.u16(this.addr() + 0xac)) & u32::from(mask as u16);
    (bits as i32) >> (u32::from(shift as u16) & 0x1f)
}

// Translated from 00639a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): below the count of the
/// array at `+0x18` (`00658930`), the pointer in element `index`'s slot
/// (`00877a30`, then `00559450`); otherwise null.
pub fn fn_00639a60(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let array = this.addr() + 0x18;
    if index < e.call(ARRAY_COUNT_00658930, &args![array]).u32() {
        let slot = e.call(ARRAY_AT_00877A30, &args![array, index]).u32();
        e.call(POINTER_GET, &args![slot]).u32()
    } else {
        0
    }
}

// Translated from 00639ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): below the 16-bit count
/// of the array at `+0x54` (`00639b00`), the pointer in element `index`'s
/// slot (`00877a30`, then `00559450`); otherwise null.
pub fn fn_00639ac0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    if index < u32::from(fn_00639b00(e, this)) {
        let slot = e
            .call(ARRAY_AT_00877A30, &args![this.addr() + 0x54, index])
            .u32();
        e.call(POINTER_GET, &args![slot]).u32()
    } else {
        0
    }
}

/// This unit's n086 translations, by exe address.
fn funcs_n086() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0063_77c0, fn_006377c0(f32)),
        entry!(
            0x0063_7890,
            fn_00637890(Ptr<Precipitation>, u32, u32, u32) -> u32
        ),
        entry!(0x0063_9780, fn_00639780() -> f32),
        entry!(0x0063_97b0, fn_006397b0() -> u32),
        entry!(0x0063_97c0, fn_006397c0() -> f32),
        entry!(0x0063_97f0, fn_006397f0(Ptr, f32, f32, f32)),
        entry!(0x0063_9830, fn_00639830(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0063_9a10, fn_00639a10(Ptr) -> u32),
        entry!(0x0063_9a30, fn_00639a30(Ptr, u32, u32) -> i32),
        entry!(0x0063_9a60, fn_00639a60(Ptr, u32) -> u32),
        entry!(0x0063_9ac0, fn_00639ac0(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod n086_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn random_doubles(e: &mut Engine) {
        for (addr, value) in [
            (RANDOM_DIVISOR_01029790, 32767.0f64),
            (RANDOM_FACTOR_01011590, 2.0),
            (RANDOM_ADDEND_0101A6B0, -1.0),
        ] {
            e.map(addr, 8);
            e.mem.set_u64(addr, value.to_bits());
        }
    }

    #[test]
    fn fn_006377c0_adds_to_the_accumulator() {
        let mut e = Engine::new();
        e.map(ACCUMULATOR_011AE5D0, 4);
        e.mem.set_f32(ACCUMULATOR_011AE5D0, 1.5);
        e.call(0x0063_77c0, &args![2.25f32.to_bits()]);
        assert_eq!(e.mem.f32(ACCUMULATOR_011AE5D0), 3.75);
    }

    #[test]
    fn random_helpers_scale_rand() {
        let mut e = Engine::new();
        random_doubles(&mut e);
        e.register(RAND, |_, _| 32767u32.into_ret());
        assert_eq!(e.call(0x0063_97b0, &[]).u32(), 32767);
        assert_eq!(e.call(0x0063_9780, &[]).f32(), 1.0);
        assert_eq!(e.call(0x0063_97c0, &[]).f32(), 1.0);
        e.register(RAND, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x0063_9780, &[]).f32(), 0.0);
        assert_eq!(e.call(0x0063_97c0, &[]).f32(), -1.0);
        e.register(RAND, |_, _| 16384u32.into_ret());
        let expected = (16384.0f64 / 32767.0) as f32;
        assert_eq!(e.call(0x0063_9780, &[]).f32(), expected);
        let expected = (16384.0f64 * 2.0 / 32767.0 - 1.0) as f32;
        assert_eq!(e.call(0x0063_97c0, &[]).f32(), expected);
    }

    #[test]
    fn fn_006397f0_copies_the_made_point() {
        let mut e = Engine::new();
        e.register(POINT_SET_00416870, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            e.mem.set_u32(a[0] + 8, a[3]);
            a[0].into_ret()
        });
        let this = e.mem.alloc(12);
        // A signalling NaN goes through FLD/FSTP quieted.
        e.call(
            0x0063_97f0,
            &args![this, 1.0f32.to_bits(), 0x7f80_0001u32, 3.0f32.to_bits()],
        );
        assert_eq!(e.mem.f32(this), 1.0);
        assert_eq!(e.mem.u32(this + 4), 0x7fc0_0001);
        assert_eq!(e.mem.f32(this + 8), 3.0);
    }

    #[test]
    fn fn_00639830_builds_the_geometry_data() {
        let mut e = Engine::new();
        for addr in [
            NI_OBJECT_CONSTRUCTOR,
            MEMBER_INIT_00540830,
            MEMBER_INIT_005E03D0,
            VECTOR_CONSTRUCTOR_ITERATOR,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(ALLOCATE_00401000, |_, _| 0x5000u32.into_ret());
        let this = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        let result = e.call(0x0063_9830, &args![this, 0x1_0003u32, 4u32, 9u32]);
        assert_eq!(result.u32(), this);
        assert_eq!(e.mem.u32(this), ADDITIONAL_GEOMETRY_DATA_VTABLE);
        assert_eq!(e.mem.u16(this + 0xc), 3);
        assert_eq!(e.mem.u32(this + 0x10), 4);
        assert_eq!(e.mem.u32(this + 0x14), 0x5000);
        assert_eq!(e.mem.u32(this + 0x18), 9);
        assert_eq!(calls(&e, ALLOCATE_00401000), vec![vec![0x70]]);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![0x5000, 0x1c, 4, 0x0063_9940]]
        );
        assert_eq!(
            calls(&e, MEMBER_INIT_00540830),
            vec![vec![this + 0x1c, 0, 1]]
        );
        assert_eq!(calls(&e, MEMBER_INIT_005E03D0), vec![vec![this + 0x1c]]);

        // Overflowing size and a failed allocation.
        e.register(ALLOCATE_00401000, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0063_9830, &args![this, 0u32, 0x1000_0000u32, 0u32]);
        assert_eq!(calls(&e, ALLOCATE_00401000), vec![vec![u32::MAX]]);
        assert!(calls(&e, VECTOR_CONSTRUCTOR_ITERATOR).is_empty());
        assert_eq!(e.mem.u32(this + 0x14), 0);
    }

    #[test]
    fn fn_00639a30_masks_and_shifts() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xb0);
        e.mem.set_u16(this + 0xac, 0xf0f5);
        assert_eq!(e.call(0x0063_9a10, &args![this]).u32(), 5);
        assert_eq!(
            e.call(0x0063_9a30, &args![this, 0xffff_ff00u32, 4u32])
                .i32(),
            0xf00
        );
        // Only the low 16 bits of the shift word, then its low 5 bits.
        assert_eq!(
            e.call(0x0063_9a30, &args![this, 0xffffu32, 0x1_0024u32])
                .i32(),
            0xf0f
        );
    }

    #[test]
    fn indexed_pointers_check_the_count() {
        let mut e = Engine::new();
        e.register(ARRAY_COUNT_00658930, |_, _| 2u32.into_ret());
        e.register(CALLEE_009938B0, |_, _| 2u32.into_ret());
        e.register(ARRAY_AT_00877A30, |_, a| (a[0] + a[1] * 4).into_ret());
        e.register(POINTER_GET, |_, a| (a[0] | 0x8000_0000).into_ret());
        let this = 0x1000u32;
        assert_eq!(e.call(0x0063_9a60, &args![this, 1u32]).u32(), 0x8000_101c);
        assert_eq!(e.call(0x0063_9a60, &args![this, 2u32]).u32(), 0);
        assert_eq!(e.call(0x0063_9ac0, &args![this, 1u32]).u32(), 0x8000_1058);
        assert_eq!(e.call(0x0063_9ac0, &args![this, 2u32]).u32(), 0);
    }
}

/// The calls `00637890` makes (`thiscall` unless said): the root's first
/// time controller (`0043b230`) and a controller's next (`004a8a90`); the
/// kind-of test (`0043b300`, `cdecl`: the `NiRTTI` record, the object; a
/// byte) with the records `012024e8` (the emitter controller), `01202538`
/// (a node with a precipitation model), `01202530` and `01202528` (two
/// child kinds), `01202518`, `01202510`, `01202508` and `01202500` (the
/// box, sphere, cylinder and mesh emitters) and `011f4aa0`; the casts
/// (`00653270`) to `01202540` (the emitter), `011f4acc`, `011f4a20` and
/// `011f4a88`; the root's child (`004d7d70`, one word: the index).
const FIRST_CONTROLLER_0043B230: u32 = 0x0043_b230;
const NEXT_CONTROLLER_004A8A90: u32 = 0x004a_8a90;
const IS_KIND_OF_0043B300: u32 = 0x0043_b300;
const RTTI_EMITTER_CONTROLLER_012024E8: u32 = 0x0120_24e8;
const RTTI_MODEL_NODE_01202538: u32 = 0x0120_2538;
const RTTI_CHILD_01202530: u32 = 0x0120_2530;
const RTTI_CHILD_01202528: u32 = 0x0120_2528;
const RTTI_BOX_EMITTER_01202518: u32 = 0x0120_2518;
const RTTI_SPHERE_EMITTER_01202510: u32 = 0x0120_2510;
const RTTI_CYLINDER_EMITTER_01202508: u32 = 0x0120_2508;
const RTTI_MESH_EMITTER_01202500: u32 = 0x0120_2500;
const RTTI_011F4AA0: u32 = 0x011f_4aa0;
const RTTI_EMITTER_01202540: u32 = 0x0120_2540;
const RTTI_011F4ACC: u32 = 0x011f_4acc;
const RTTI_011F4A20: u32 = 0x011f_4a20;
const RTTI_011F4A88: u32 = 0x011f_4a88;
const CHILD_AT_004D7D70: u32 = 0x004d_7d70;
/// The kind-of test of `0045bad0` (`cdecl`: record, object; a byte) on
/// the root's `009611e0` object.
const IS_KIND_OF_0045BAD0: u32 = 0x0045_bad0;
const ROOT_OBJECT_009611E0: u32 = 0x0096_11e0;
/// The float getters (in `ST0`) on the emitter and the second child:
/// `004a7bd0`, `00495460`, `007df1f0`, `00526ac0`, `00526ae0`, `00759450`,
/// `00792760`; and `005081b0` on the emitter's shape.
const GETTER_004A7BD0: u32 = 0x004a_7bd0;
const GETTER_00495460: u32 = 0x0049_5460;
const GETTER_007DF1F0: u32 = 0x007d_f1f0;
const GETTER_00526AC0: u32 = 0x0052_6ac0;
const GETTER_00526AE0: u32 = 0x0052_6ae0;
const GETTER_00759450: u32 = 0x0075_9450;
const GETTER_00792760: u32 = 0x0079_2760;
const GETTER_005081B0: u32 = 0x0050_81b0;
/// The model's geometry calls: `005495f0` (the model's geometry), the
/// vertex count (`004a8ae0`, 16 bits), triangle count (`008041a0`, 16
/// bits), vertices (`007af430`), texture coordinates (`0055b980`) and
/// triangle list (`00a7b850`).
const MODEL_GEOMETRY_005495F0: u32 = 0x0054_95f0;
const VERTEX_COUNT_004A8AE0: u32 = 0x004a_8ae0;
const TRIANGLE_COUNT_008041A0: u32 = 0x0080_41a0;
const VERTICES_007AF430: u32 = 0x007a_f430;
const TEXTURE_COORDINATES_0055B980: u32 = 0x0055_b980;
const TRIANGLES_00A7B850: u32 = 0x00a7_b850;
/// `004865f0` (`cdecl`, two words): the smaller of the two.
const MIN_004865F0: u32 = 0x0048_65f0;
/// `00aa1070` (`cdecl`, one word: bytes): the index block.
const ALLOCATE_00AA1070: u32 = 0x00aa_1070;
/// The geometry: `NiTriShapeDynamicData` (0x5c bytes, `00a816a0`, eleven
/// words) and `NiTriShape` (0xc4 bytes, `00a74480`, one word).
const TRI_SHAPE_DYNAMIC_DATA_00A816A0: u32 = 0x00a8_16a0;
const TRI_SHAPE_00A74480: u32 = 0x00a7_4480;
/// The point helpers (with `00416870` above): `004a3760` (`cdecl`: out,
/// a float, a point: the scaled point), `00452dc0` (two floats: a texture
/// coordinate), `0045bb20` (out, a float: the point scaled),
/// `00439e90` (out, a point: the sum), `00524c40` (out, a point: the
/// transform of a point), `004b4500` (out, a point: the transform's
/// rotation of a point) and `004169a0` (`cdecl`: an angle and the two
/// out floats).
const POINT_SCALED_004A3760: u32 = 0x004a_3760;
const TEXTURE_POINT_00452DC0: u32 = 0x0045_2dc0;
const POINT_TIMES_0045BB20: u32 = 0x0045_bb20;
const POINT_PLUS_00439E90: u32 = 0x0043_9e90;
const TRANSFORM_POINT_00524C40: u32 = 0x0052_4c40;
const TRANSFORM_ROTATE_004B4500: u32 = 0x004b_4500;
const SINE_COSINE_004169A0: u32 = 0x0041_69a0;
/// The transforms: the constructor (`00476a80`), the inverse
/// (`NiTransform::InvertNonUniform`, `004b4880`: out) and the product
/// (`0062c250`: out, right-hand side); an emitter's two objects (`006a1b60`
/// and `0044edb0`) and an object's world transform (`00461130`).
const TRANSFORM_CONSTRUCT_00476A80: u32 = 0x0047_6a80;
const TRANSFORM_INVERT_004B4880: u32 = 0x004b_4880;
const TRANSFORM_PRODUCT_0062C250: u32 = 0x0062_c250;
const EMITTER_OBJECT_006A1B60: u32 = 0x006a_1b60;
const EMITTER_OBJECT_0044EDB0: u32 = 0x0044_edb0;
const WORLD_TRANSFORM_00461130: u32 = 0x0046_1130;
/// The bound (`00537b10`, four words) and its center (`006815c0`) and
/// radius (`0084d030`, in `ST0`); `0063f790` (a float) sets its radius;
/// `004a5200` gives the shape the bound.
const BOUND_00537B10: u32 = 0x0053_7b10;
const POINT_CONSTRUCT_006815C0: u32 = 0x0068_15c0;
const BOUND_RADIUS_0084D030: u32 = 0x0084_d030;
const BOUND_SET_RADIUS_0063F790: u32 = 0x0063_f790;
const SHAPE_SET_BOUND_004A5200: u32 = 0x004a_5200;
/// The clamps (`cdecl`, two floats, in `ST0`): `0040ebd0` and `00404010`.
const CLAMP_0040EBD0: u32 = 0x0040_ebd0;
const CLAMP_00404010: u32 = 0x0040_4010;
/// The doubles `0.5` (`01011588`) and `2.0` (`01011590`) in the
/// decompiler's reading, and the float the corners use (`01012054`).
const HALF_01011588: u32 = 0x0101_1588;
const TWO_01011590: u32 = 0x0101_1590;
const CORNER_01012054: u32 = 0x0101_2054;
/// The additional geometry data's calls (`NiAdditionalGeometryData`):
/// `SetDataBlockCount` (`00a72ca0`), `SetDataBlock` (`00a72bb0`, four
/// words), `SetDataStream` (`00a726a0`, seven words); `00539ef0` gives it
/// to the shape.
const SET_DATA_BLOCK_COUNT: u32 = 0x00a7_2ca0;
const SET_DATA_BLOCK: u32 = 0x00a7_2bb0;
const SET_DATA_STREAM: u32 = 0x00a7_26a0;
const SHAPE_SET_DATA_00539EF0: u32 = 0x0053_9ef0;
/// The property calls: `NiAVObject::GetProperty` (`00a59d30`),
/// `NiAVObject::AttachProperty` (`00439410`), the removal by type
/// (`00a5b230`), the precipitation shader property (0xa0 bytes,
/// `00bb0d40`, seven words) and its `005a8060` (one word), and
/// `BSShaderManager::PrepareObject` (`00b57e30`, `cdecl`, three words).
const GET_PROPERTY_00A59D30: u32 = 0x00a5_9d30;
const ATTACH_PROPERTY_00439410: u32 = 0x0043_9410;
const REMOVE_PROPERTY_00A5B230: u32 = 0x00a5_b230;
const PRECIPITATION_PROPERTY_00BB0D40: u32 = 0x00bb_0d40;
const PROPERTY_SET_005A8060: u32 = 0x005a_8060;
const PREPARE_OBJECT_00B57E30: u32 = 0x00b5_7e30;
/// `00401030` (`cdecl`): frees a block.
const FREE_00401030: u32 = 0x0040_1030;

/// The kind-of test (`0043b300`) of `object` against `record`.
fn is_kind_of(e: &mut Engine, record: u32, object: u32) -> bool {
    e.call(IS_KIND_OF_0043B300, &args![record, object]).u32() & 0xff != 0
}

/// A block of `count` elements of `size` bytes (`00401000`; all ones as the
/// size when it overflows), each built by `006815c0` (`00401050`); null
/// when the allocation fails.
fn new_elements(e: &mut Engine, count: u32, size: u32) -> u32 {
    let bytes = count.checked_mul(size).unwrap_or(u32::MAX);
    let block = e.call(ALLOCATE_00401000, &args![bytes]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![block, size, count, POINT_CONSTRUCT_006815C0],
    );
    block
}

/// Copies `words` words from `from` to `to`.
fn copy_words(e: &mut Engine, from: u32, to: u32, words: u32) {
    for i in 0..words {
        let value = e.mem.u32(from.wrapping_add(i * 4));
        e.mem.set_u32(to.wrapping_add(i * 4), value);
    }
}

/// A new `NiTriShape` (0xc4 bytes) of a new `NiTriShapeDynamicData` (0x5c
/// bytes; vertices, positions, texture coordinates, triangles, indices);
/// null parts when an allocation fails.
fn new_tri_shape(
    e: &mut Engine,
    vertices: u32,
    positions: u32,
    coordinates: u32,
    triangles: u32,
    indices: u32,
) -> u32 {
    let block = e.call(NEW_OBJECT, &args![0xc4u32]).u32();
    if block == 0 {
        return 0;
    }
    let data_block = e.call(NEW_OBJECT, &args![0x5cu32]).u32();
    let data = if data_block != 0 {
        e.call(
            TRI_SHAPE_DYNAMIC_DATA_00A816A0,
            &args![
                data_block,
                vertices,
                positions,
                0u32,
                0u32,
                coordinates,
                1u32,
                0u32,
                triangles,
                indices,
                0u32,
                0u32
            ],
        )
        .u32()
    } else {
        0
    };
    e.call(TRI_SHAPE_00A74480, &args![block, data]).u32()
}

/// `FLD dword` of the float at `addr`.
fn fld(e: &Engine, addr: u32) -> f64 {
    x87_load32(e.mem.u32(addr))
}

/// A `float` memory operand at `addr`.
fn fmem(e: &Engine, addr: u32) -> f64 {
    x87_m32(e.mem.u32(addr))
}

/// A `double` memory operand at `addr`.
fn dmem(e: &Engine, addr: u32) -> f64 {
    f64::from_bits(e.mem.u64(addr))
}

/// A float pushed through `FLD dword`/`FSTP dword`.
fn float_arg(e: &Engine, addr: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(addr))
}

/// The emitter's transform into `transform` (13 words): the product of the
/// inverse of `second`'s world transform and `first`'s (temporaries at the
/// frame offsets the branch uses), then the velocity's rotation by it
/// (`004b4500`) into `rotated` (3 words).
#[allow(clippy::too_many_arguments)]
fn emitter_transform(
    e: &mut Engine,
    first: u32,
    second: u32,
    slots: [u32; 5],
    transform: u32,
    velocity: u32,
    rotated: u32,
) {
    let [first_copy, second_copy, inverse, product, rotation] = slots;
    let world = e.call(WORLD_TRANSFORM_00461130, &args![first]).u32();
    copy_words(e, world, first_copy, 13);
    let world = e.call(WORLD_TRANSFORM_00461130, &args![second]).u32();
    copy_words(e, world, second_copy, 13);
    e.call(TRANSFORM_CONSTRUCT_00476A80, &args![inverse]);
    e.call(TRANSFORM_INVERT_004B4880, &args![second_copy, inverse]);
    let result = e
        .call(
            TRANSFORM_PRODUCT_0062C250,
            &args![inverse, product, first_copy],
        )
        .u32();
    copy_words(e, result, transform, 13);
    let point = e
        .call(
            TRANSFORM_ROTATE_004B4500,
            &args![transform, rotation, velocity],
        )
        .u32();
    copy_words(e, point, rotated, 3);
}

/// The height bound `scale * rotated.z + transform.z` (stored) clamped by
/// `clamp` against `transform.z`, as the float it answers.
fn height_bound(e: &mut Engine, clamp: u32, scale: u32, rotated_z: u32, translate_z: u32) -> u32 {
    let height = x87_store32(x87_add(
        x87_mul(fld(e, scale), fmem(e, rotated_z)),
        fmem(e, translate_z),
    ));
    let bound = e
        .call(
            clamp,
            &args![float_arg(e, translate_z), x87_quiet_bits(height)],
        )
        .f64();
    x87_store32(bound)
}

/// The box/sphere/cylinder/mesh corner made by `00416870` into `temporary`
/// and copied to `to`.
fn corner(e: &mut Engine, temporary: u32, to: u32, x: u32, y: u32, z: u32) {
    let point = e.call(POINT_SET_00416870, &args![temporary, x, y, z]).u32();
    copy_words(e, point, to, 3);
}

// Translated from 00637890 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00637890 - the C++ exception-unwinding frame (FS:[0]) is not translated.
/// The precipitation geometry for one cast child (unnamed in the engine map;
/// `thiscall` on the `Precipitation`, `RET 0xc`: the child, a word that is
/// not read, the snow byte); answers the new `NiTriShape` or null.
///
/// The root's first emitter controller (`012024e8`, walking `0043b230`/
/// `004a8a90`) gives the emitter (`01202540` of its `+0x3c` word) and the
/// particle count: the birth rate interpolator's value (slot `+0x9c` with
/// 0.0, 0) times the emitter's `+0x48` float, truncated to 16 bits. With
/// no emitter, null. A model root (`01202538`) takes the geometry of its
/// first `01202530` child (its `00639a60` 0 object's slot `+0xc`, child 0,
/// cast to `011f4acc`): unless that is already `011f4a20`, the model's
/// triangle data (`005495f0` cast to `011f4a88`) is repeated for the count
/// (at most `0xffff / vertices`) into new position, texture coordinate and
/// index blocks; another root makes quads of four corners
/// (`(-1|1, 0, -1|1)` scaled by the emitter's `00792760`) with coordinates
/// (0,1), (1,1), (1,0), (0,0) and two triangles each, the count at most
/// `0x3fff`. The second kind of child (`01202528`) gives the float `+0xe8`
/// (`004a7bd0`).
///
/// With a shape: the speed (`004a7bd0 + (r - 0.5) * 00495460`), the
/// declination (`007df1f0 + r' * 00526ac0`) and the plane angle
/// (`00526ae0 + r'' * 00759450`, `r` from `00639780`, `r'` and `r''` from
/// `006397c0`) through the angle table (`00639730`) make the velocity
/// `speed * (sin d * cos p, sin d * sin p, cos d)`. The emitter's kind
/// (box, sphere, cylinder, mesh) gives the transform from the emitter
/// object to the root (or from the mesh emitter's first object, `00639ac0`
/// 0, to the root), the rotated velocity and the low and high corners of
/// the volume (the half sizes `00639b20 / 2` and `005081b0 / 2`, the
/// radius `005081b0`, or the mesh's bound, the heights clamped by
/// `0040ebd0` and `00404010` against the transform's z); another kind
/// deletes the shape (slot 0 with 1) and answers null. Each particle's
/// position (a random point in the volume, transformed, plus the scaled
/// velocity times `00639780`; the mesh emitter's slot `+0xa8`) is repeated
/// for its vertices into an additional geometry data block (0x2c bytes,
/// `00639830`), given to the shape with the bound's radius set to half the
/// volume's x extent and its center cleared. The shape takes the model's
/// properties 5 and 0, loses property 3, and gets a new precipitation
/// property (0xa0 bytes, `00bb0d40`; with the count, `+0xe8`, the rotated
/// velocity, both corners, the snow byte and the root's `00639a10` value
/// when it is an `011f4aa0`, else 1) with `005a8060(0x1e)`; then
/// `PrepareObject`.
pub fn fn_00637890(
    e: &mut Engine,
    _this: Ptr<Precipitation>,
    root: u32,
    _unused_2: u32,
    snow: u32,
) -> u32 {
    e.with_stack(0x850, |e, frame| {
        // The frame is EBP - 0x850.
        let f = frame.addr();
        let at = |offset: u32| f + 0x850 - offset;
        build_precipitation(e, root, snow, &at)
    })
}

/// The body of [`fn_00637890`]; `at(x)` is the frame address `EBP - x`.
fn build_precipitation(e: &mut Engine, root: u32, snow: u32, at: &dyn Fn(u32) -> u32) -> u32 {
    let birth_rate = at(0x10);
    let extent = at(0xe8);
    let transform = at(0xe4);
    let low = at(0x8c);
    let high = at(0x11c);
    e.mem.set_u32(birth_rate, 0.0f32.to_bits());
    let mut emitter = 0u32;
    let mut first_child = 0u32;
    let mut second_child = 0u32;
    let mut model = 0u32;
    let mut shape = 0u32;
    let mut controller = e.call(FIRST_CONTROLLER_0043B230, &args![root]).u32();
    while controller != 0 {
        if is_kind_of(e, RTTI_EMITTER_CONTROLLER_012024E8, controller) {
            let word = fn_00639b40(e, Ptr::new(controller));
            emitter = e
                .call(KIND_OF_CAST, &args![RTTI_EMITTER_01202540, word])
                .u32();
            let interpolator =
                ni_p_sys_emitter_ctlr_get_birth_rate_interpolator(e, Ptr::new(controller));
            e.vcall(
                interpolator,
                0x9c,
                &args![0.0f32.to_bits(), 0u32, birth_rate],
            );
            let rate = f64::from(fn_00639aa0(e, Ptr::new(emitter)));
            let value = x87_mul(rate, fmem(e, birth_rate));
            e.mem.set_u32(birth_rate, x87_store32(value));
            break;
        }
        controller = e.call(NEXT_CONTROLLER_004A8A90, &args![controller]).u32();
    }
    if emitter == 0 {
        return 0;
    }
    let mut vertices = 0u16;
    let mut count = crate::x87::fistp_chop(fld(e, birth_rate)) as u16;
    e.mem.set_u32(extent, 0.0f32.to_bits());
    e.call(TRANSFORM_CONSTRUCT_00476A80, &args![transform]);
    e.call(POINT_CONSTRUCT_006815C0, &args![low]);
    e.call(POINT_CONSTRUCT_006815C0, &args![high]);
    if is_kind_of(e, RTTI_MODEL_NODE_01202538, root) {
        let mut index = 0u32;
        loop {
            let child = e.call(CHILD_AT_004D7D70, &args![root, index]).u32();
            index = index.wrapping_add(1);
            if child == 0 {
                break;
            }
            if first_child == 0 && is_kind_of(e, RTTI_CHILD_01202530, child) {
                first_child = child;
            }
            if second_child == 0 && is_kind_of(e, RTTI_CHILD_01202528, child) {
                second_child = child;
            }
        }
        if first_child != 0 {
            let holder = fn_00639a60(e, Ptr::new(first_child), 0);
            let object = e.vcall(holder, 0xc, &[]).u32();
            if object != 0 {
                let child = e.call(CHILD_OBJECT_0045BC00, &args![object, 0u32]).u32();
                model = e.call(KIND_OF_CAST, &args![RTTI_011F4ACC, child]).u32();
            }
        }
        if second_child != 0 {
            let value = e.call(GETTER_004A7BD0, &args![second_child]).f64();
            e.mem.set_u32(extent, x87_store32(value));
        }
        if e.call(KIND_OF_CAST, &args![RTTI_011F4A20, model]).u32() == 0 {
            let geometry = e.call(MODEL_GEOMETRY_005495F0, &args![model]).u32();
            let data = e.call(KIND_OF_CAST, &args![RTTI_011F4A88, geometry]).u32();
            vertices = e.call(VERTEX_COUNT_004A8AE0, &args![data]).u32() as u16;
            let triangles = e.call(TRIANGLE_COUNT_008041A0, &args![data]).u32() as u16;
            let source_positions = e.call(VERTICES_007AF430, &args![data]).u32();
            let source_coordinates = e.call(TEXTURE_COORDINATES_0055B980, &args![data]).u32();
            let source_indices = e.call(TRIANGLES_00A7B850, &args![data]).u32();
            let most = 0xffff / u32::from(vertices);
            count = e.call(MIN_004865F0, &args![u32::from(count), most]).u32() as u16;
            let total = u32::from(vertices) * u32::from(count);
            let positions = new_elements(e, total, 0xc);
            let total = u32::from(vertices) * u32::from(count);
            let coordinates = new_elements(e, total, 8);
            let indices = e
                .call(
                    ALLOCATE_00AA1070,
                    &args![(u32::from(triangles) * u32::from(count)) << 1],
                )
                .u32();
            let mut next = 0u32;
            for copy in 0..u32::from(count) {
                let base = u32::from(vertices) * copy;
                for vertex in 0..u32::from(vertices) {
                    let to = base + vertex;
                    copy_words(
                        e,
                        source_positions.wrapping_add(vertex * 0xc),
                        positions.wrapping_add(to.wrapping_mul(0xc)),
                        3,
                    );
                    copy_words(
                        e,
                        source_coordinates.wrapping_add(vertex * 8),
                        coordinates.wrapping_add(to.wrapping_mul(8)),
                        2,
                    );
                }
                for triangle in 0..u32::from(triangles) {
                    let index = u32::from(e.mem.u16(source_indices.wrapping_add(triangle * 2)));
                    e.mem.set_u16(
                        indices.wrapping_add(next.wrapping_mul(2)),
                        index.wrapping_add(base) as u16,
                    );
                    next = next.wrapping_add(1);
                }
            }
            shape = new_tri_shape(
                e,
                u32::from(vertices) * u32::from(count),
                positions,
                coordinates,
                (u32::from(triangles) / 3).wrapping_mul(u32::from(count)),
                indices,
            );
        }
    } else {
        let mut index = 0u32;
        loop {
            let child = e.call(CHILD_AT_004D7D70, &args![root, index]).u32();
            index = index.wrapping_add(1);
            if child == 0 {
                break;
            }
            if is_kind_of(e, RTTI_CHILD_01202528, child) {
                second_child = child;
                break;
            }
        }
        if second_child != 0 {
            let value = e.call(GETTER_004A7BD0, &args![second_child]).f64();
            e.mem.set_u32(extent, x87_store32(value));
        }
        model = root;
        vertices = 4;
        count = e
            .call(MIN_004865F0, &args![u32::from(count), 0xffff / 4u32])
            .u32() as u16;
        let triangles = u32::from(count) << 1;
        let total = u32::from(count) << 2;
        let positions = new_elements(e, total, 0xc);
        let coordinates = new_elements(e, total, 8);
        let indices = e
            .call(ALLOCATE_00AA1070, &args![(triangles * 3) << 1])
            .u32();
        let minus = float_arg(e, CORNER_01012054);
        let (zero, one) = (0.0f32.to_bits(), 1.0f32.to_bits());
        for quad in 0..u32::from(count) {
            let size = x87_store32(e.call(GETTER_00792760, &args![emitter]).f64());
            let base = quad << 2;
            let corners = [
                (at(0x4b4), at(0x4c0), minus, zero, minus),
                (at(0x4cc), at(0x4d8), one, zero, minus),
                (at(0x4e4), at(0x4f0), one, zero, one),
                (at(0x4fc), at(0x508), minus, zero, one),
            ];
            for (k, (temporary, out, x, y, z)) in (0u32..).zip(corners) {
                let point = e.call(POINT_SET_00416870, &args![temporary, x, y, z]).u32();
                let scaled = e
                    .call(
                        POINT_SCALED_004A3760,
                        &args![out, x87_quiet_bits(size), point],
                    )
                    .u32();
                copy_words(
                    e,
                    scaled,
                    positions.wrapping_add((base + k).wrapping_mul(0xc)),
                    3,
                );
            }
            let coordinate_values = [
                (at(0x510), zero, one),
                (at(0x518), one, one),
                (at(0x520), one, zero),
                (at(0x528), zero, zero),
            ];
            for (k, (temporary, u, v)) in (0u32..).zip(coordinate_values) {
                let point = e
                    .call(TEXTURE_POINT_00452DC0, &args![temporary, u, v])
                    .u32();
                copy_words(
                    e,
                    point,
                    coordinates
                        .wrapping_add(base.wrapping_mul(8))
                        .wrapping_add(k * 8),
                    2,
                );
            }
            let first = indices.wrapping_add(quad.wrapping_mul(6).wrapping_mul(2));
            for (k, offset) in (0u32..).zip([0u32, 1, 2, 3, 0, 2]) {
                e.mem
                    .set_u16(first + k * 2, base.wrapping_add(offset) as u16);
            }
        }
        shape = new_tri_shape(
            e,
            total & 0xffff,
            positions,
            coordinates,
            triangles & 0xffff,
            indices,
        );
    }
    if shape == 0 {
        return 0;
    }
    particles(e, root, snow, emitter, model, shape, vertices, count, at)
}

/// The particles, their geometry data and properties of
/// [`fn_00637890`] (from `0063858c`).
#[allow(clippy::too_many_arguments)]
fn particles(
    e: &mut Engine,
    root: u32,
    snow: u32,
    emitter: u32,
    model: u32,
    shape: u32,
    vertices: u16,
    count: u16,
    at: &dyn Fn(u32) -> u32,
) -> u32 {
    let extent = at(0xe8);
    let transform = at(0xe4);
    let translate_z = transform + 0x2c;
    let low = at(0x8c);
    let high = at(0x11c);
    let direction = at(0x110);
    let velocity = at(0x104);
    let rotated = at(0x60);
    let rotated_z = rotated + 8;
    let scale = at(0xec);
    let base_speed = e.call(GETTER_004A7BD0, &args![emitter]).f64();
    let speed_range = e.call(GETTER_00495460, &args![emitter]).f64();
    let random = f64::from(fn_00639780(e));
    let speed = x87_store32(x87_add(
        x87_mul(x87_sub(random, dmem(e, HALF_01011588)), speed_range),
        base_speed,
    ));
    let base = e.call(GETTER_007DF1F0, &args![emitter]).f64();
    let range = e.call(GETTER_00526AC0, &args![emitter]).f64();
    let random = f64::from(fn_006397c0(e));
    let declination = x87_store32(x87_add(x87_mul(random, range), base));
    let base = e.call(GETTER_00526AE0, &args![emitter]).f64();
    let range = e.call(GETTER_00759450, &args![emitter]).f64();
    let random = f64::from(fn_006397c0(e));
    let plane = x87_store32(x87_add(x87_mul(random, range), base));
    let (plane_sin, plane_cos) = (at(0x34), at(0x80));
    let (sin_declination, cos_declination) = (at(0xf8), at(0x38));
    fn_00639730(
        e,
        f32::from_bits(x87_quiet_bits(plane)),
        Ptr::new(plane_sin),
        Ptr::new(plane_cos),
    );
    fn_00639730(
        e,
        f32::from_bits(x87_quiet_bits(declination)),
        Ptr::new(sin_declination),
        Ptr::new(cos_declination),
    );
    let z = float_arg(e, cos_declination);
    let y = x87_store32(x87_mul(fld(e, sin_declination), fmem(e, plane_sin)));
    let x = x87_store32(x87_mul(fld(e, sin_declination), fmem(e, plane_cos)));
    e.call(
        POINT_SET_00416870,
        &args![direction, x87_quiet_bits(x), x87_quiet_bits(y), z],
    );
    e.call(
        POINT_SCALED_004A3760,
        &args![velocity, x87_quiet_bits(speed), direction],
    );
    let emitter_scale = x87_store32(f64::from(fn_00639aa0(e, Ptr::new(emitter))));
    e.mem.set_u32(scale, emitter_scale);
    e.call(POINT_CONSTRUCT_006815C0, &args![rotated]);
    let mut box_emitter = 0u32;
    let mut cylinder = 0u32;
    let mut sphere = 0u32;
    let mut mesh = 0u32;
    let two = |e: &Engine| dmem(e, TWO_01011590);
    if is_kind_of(e, RTTI_BOX_EMITTER_01202518, emitter) {
        box_emitter = e
            .call(KIND_OF_CAST, &args![RTTI_BOX_EMITTER_01202518, emitter])
            .u32();
        let first = e.call(EMITTER_OBJECT_006A1B60, &args![box_emitter]).u32();
        emitter_transform_split(
            e,
            box_emitter,
            [at(0x230), at(0x1c8), at(0x1fc), at(0x56c), at(0x578)],
            first,
            transform,
            velocity,
            rotated,
        );
        let z = height_bound(e, CLAMP_0040EBD0, scale, rotated_z, translate_z);
        let y = -x87_div(f64::from(fn_00639b20(e, Ptr::new(box_emitter))), two(e));
        let y = x87_store32(y);
        let x = -x87_div(e.call(GETTER_005081B0, &args![box_emitter]).f64(), two(e));
        let x = x87_store32(x);
        corner(e, at(0x584), low, x87_quiet_bits(x), x87_quiet_bits(y), z);
        let z = height_bound(e, CLAMP_00404010, scale, rotated_z, translate_z);
        let y = x87_store32(x87_div(
            f64::from(fn_00639b20(e, Ptr::new(box_emitter))),
            two(e),
        ));
        let x = x87_store32(x87_div(
            e.call(GETTER_005081B0, &args![box_emitter]).f64(),
            two(e),
        ));
        corner(e, at(0x590), high, x87_quiet_bits(x), x87_quiet_bits(y), z);
    } else if is_kind_of(e, RTTI_SPHERE_EMITTER_01202510, emitter) {
        sphere = e
            .call(KIND_OF_CAST, &args![RTTI_SPHERE_EMITTER_01202510, emitter])
            .u32();
        let first = e.call(EMITTER_OBJECT_006A1B60, &args![sphere]).u32();
        emitter_transform_split(
            e,
            sphere,
            [at(0x2cc), at(0x264), at(0x298), at(0x5c4), at(0x5d0)],
            first,
            transform,
            velocity,
            rotated,
        );
        radius_corners(
            e,
            sphere,
            [at(0x5dc), at(0x5e8)],
            scale,
            rotated_z,
            translate_z,
            low,
            high,
        );
    } else if is_kind_of(e, RTTI_CYLINDER_EMITTER_01202508, emitter) {
        cylinder = e
            .call(
                KIND_OF_CAST,
                &args![RTTI_CYLINDER_EMITTER_01202508, emitter],
            )
            .u32();
        let first = e.call(EMITTER_OBJECT_006A1B60, &args![cylinder]).u32();
        emitter_transform_split(
            e,
            cylinder,
            [at(0x368), at(0x300), at(0x334), at(0x61c), at(0x628)],
            first,
            transform,
            velocity,
            rotated,
        );
        radius_corners(
            e,
            cylinder,
            [at(0x634), at(0x640)],
            scale,
            rotated_z,
            translate_z,
            low,
            high,
        );
    } else if is_kind_of(e, RTTI_MESH_EMITTER_01202500, emitter) {
        mesh = e
            .call(KIND_OF_CAST, &args![RTTI_MESH_EMITTER_01202500, emitter])
            .u32();
        let first = fn_00639ac0(e, Ptr::new(mesh), 0);
        emitter_transform(
            e,
            first,
            root,
            [at(0x424), at(0x3ac), at(0x3e0), at(0x674), at(0x680)],
            transform,
            velocity,
            rotated,
        );
        let bound = at(0x378);
        let center = at(0x3f0);
        let radius = at(0x3e4);
        let holder = fn_00639ac0(e, Ptr::new(mesh), 0);
        let source = e.call(BOUND_00537B10, &args![holder]).u32();
        copy_words(e, source, bound, 4);
        let source = e.call(POINT_CONSTRUCT_006815C0, &args![bound]).u32();
        copy_words(e, source, center, 3);
        let value = x87_store32(e.call(BOUND_RADIUS_0084D030, &args![bound]).f64());
        e.mem.set_u32(radius, value);
        let z = height_bound(e, CLAMP_0040EBD0, scale, rotated_z, translate_z);
        let y = x87_store32(x87_sub(fld(e, center + 4), fmem(e, radius)));
        let x = x87_store32(x87_sub(fld(e, center), fmem(e, radius)));
        corner(e, at(0x68c), low, x87_quiet_bits(x), x87_quiet_bits(y), z);
        let z = height_bound(e, CLAMP_00404010, scale, rotated_z, translate_z);
        let y = x87_store32(x87_add(fld(e, center + 4), fmem(e, radius)));
        let x = x87_store32(x87_add(fld(e, center), fmem(e, radius)));
        corner(e, at(0x698), high, x87_quiet_bits(x), x87_quiet_bits(y), z);
    } else {
        if shape != 0 {
            e.vcall(shape, 0, &args![1u32]);
        }
        return 0;
    }
    let total = u32::from(vertices) * u32::from(count);
    let positions = new_elements(e, total, 0xc);
    let mesh_point = at(0x44);
    let scaled = at(0xac);
    e.call(POINT_CONSTRUCT_006815C0, &args![mesh_point]);
    e.call(
        POINT_SCALED_004A3760,
        &args![scaled, float_arg(e, scale), rotated],
    );
    let point = at(0x434);
    for particle in 0..u32::from(count) {
        e.call(POINT_CONSTRUCT_006815C0, &args![point]);
        let temporaries = if box_emitter != 0 {
            let r = f64::from(fn_00639b20(e, Ptr::new(box_emitter)));
            let y = x87_store32(x87_mul(
                x87_sub(f64::from(fn_00639780(e)), dmem(e, HALF_01011588)),
                r,
            ));
            let r = e.call(GETTER_005081B0, &args![box_emitter]).f64();
            let x = x87_store32(x87_mul(
                x87_sub(f64::from(fn_00639780(e)), dmem(e, HALF_01011588)),
                r,
            ));
            corner(
                e,
                at(0x6b8),
                point,
                x87_quiet_bits(x),
                x87_quiet_bits(y),
                0.0f32.to_bits(),
            );
            let moved = e
                .call(
                    TRANSFORM_POINT_00524C40,
                    &args![transform, at(0x6c4), point],
                )
                .u32();
            copy_words(e, moved, point, 3);
            (at(0x6d0), at(0x6dc))
        } else if cylinder != 0 {
            let radius = e.call(GETTER_005081B0, &args![cylinder]).f64();
            let distance = x87_store32(x87_mul(f64::from(fn_00639780(e)), radius));
            let angle = x87_store32(x87_mul(
                f64::from(fn_00639780(e)),
                fmem(e, TABLE_DIVISOR_011CCB34),
            ));
            let (sine, cosine) = (at(0x43c), at(0x444));
            e.call(
                SINE_COSINE_004169A0,
                &args![x87_quiet_bits(angle), sine, cosine],
            );
            let x = x87_store32(x87_mul(x87_load32(distance), fmem(e, cosine)));
            e.mem.set_u32(point, x);
            let y = x87_store32(x87_mul(x87_load32(distance), fmem(e, sine)));
            e.mem.set_u32(point + 4, y);
            let offset = x87_sub(f64::from(fn_00639780(e)), dmem(e, HALF_01011588));
            let height = f64::from(fn_00639b20(e, Ptr::new(cylinder)));
            e.mem
                .set_u32(point + 8, x87_store32(x87_mul(height, offset)));
            let moved = e
                .call(
                    TRANSFORM_POINT_00524C40,
                    &args![transform, at(0x6e8), point],
                )
                .u32();
            copy_words(e, moved, point, 3);
            (at(0x6f4), at(0x700))
        } else if sphere != 0 {
            let radius = e.call(GETTER_005081B0, &args![sphere]).f64();
            let distance = x87_store32(x87_mul(f64::from(fn_00639780(e)), radius));
            let first_angle = x87_store32(x87_mul(
                f64::from(fn_00639780(e)),
                fmem(e, TABLE_DIVISOR_011CCB34),
            ));
            let second_angle = x87_store32(x87_mul(
                f64::from(fn_00639780(e)),
                fmem(e, TABLE_DIVISOR_011CCB34),
            ));
            let (first_sine, first_cosine) = (at(0x44c), at(0x454));
            e.call(
                SINE_COSINE_004169A0,
                &args![x87_quiet_bits(first_angle), first_sine, first_cosine],
            );
            let (second_sine, second_cosine) = (at(0x458), at(0x45c));
            e.call(
                SINE_COSINE_004169A0,
                &args![x87_quiet_bits(second_angle), second_sine, second_cosine],
            );
            let x = x87_mul(
                x87_mul(x87_load32(distance), fmem(e, second_sine)),
                fmem(e, first_cosine),
            );
            e.mem.set_u32(point, x87_store32(x));
            let y = x87_mul(
                x87_mul(x87_load32(distance), fmem(e, second_sine)),
                fmem(e, first_sine),
            );
            e.mem.set_u32(point + 4, x87_store32(y));
            let z = x87_mul(x87_load32(distance), fmem(e, second_cosine));
            e.mem.set_u32(point + 8, x87_store32(z));
            let moved = e
                .call(
                    TRANSFORM_POINT_00524C40,
                    &args![transform, at(0x70c), point],
                )
                .u32();
            copy_words(e, moved, point, 3);
            (at(0x718), at(0x724))
        } else {
            e.vcall(mesh, 0xa8, &args![point, mesh_point]);
            (at(0x730), at(0x73c))
        };
        // `00639780` times the scaled velocity (`0045bb20`), plus the
        // point (`00439e90`).
        let (times_out, plus_out) = temporaries;
        let random = x87_store32(f64::from(fn_00639780(e)));
        let times = e
            .call(
                POINT_TIMES_0045BB20,
                &args![scaled, times_out, x87_quiet_bits(random)],
            )
            .u32();
        let sum = e
            .call(POINT_PLUS_00439E90, &args![times, plus_out, point])
            .u32();
        copy_words(e, sum, point, 3);
        for vertex in 0..u32::from(vertices) {
            let to =
                positions.wrapping_add((u32::from(vertices) * particle + vertex).wrapping_mul(0xc));
            copy_words(e, point, to, 3);
        }
    }
    let block = e.call(NEW_OBJECT, &args![0x2cu32]).u32();
    let additional = if block != 0 {
        fn_00639830(e, Ptr::new(block), total & 0xffff, 1, 1).addr()
    } else {
        0
    };
    e.call(SET_DATA_BLOCK_COUNT, &args![additional, 1u32]);
    e.call(
        SET_DATA_BLOCK,
        &args![additional, 0u32, positions, total.wrapping_mul(0xc), 1u32],
    );
    e.call(FREE_00401030, &args![positions]);
    e.call(
        SET_DATA_STREAM,
        &args![
            additional,
            0u32,
            0u32,
            0u32,
            3u32,
            total & 0xffff,
            0xcu32,
            0xcu32
        ],
    );
    e.call(SHAPE_SET_DATA_00539EF0, &args![shape, additional]);
    let bound = at(0x9c);
    let source = e.call(BOUND_00537B10, &args![shape]).u32();
    copy_words(e, source, bound, 4);
    let width = x87_store32(x87_div(
        x87_sub(fld(e, high), fmem(e, low)),
        dmem(e, TWO_01011590),
    ));
    e.call(
        BOUND_SET_RADIUS_0063F790,
        &args![bound, x87_quiet_bits(width)],
    );
    fn_006397f0(e, Ptr::new(bound), 0.0, 0.0, 0.0);
    e.call(SHAPE_SET_BOUND_004A5200, &args![shape, bound]);
    let mut kind = 1u32;
    let object = e.call(ROOT_OBJECT_009611E0, &args![root]).u32();
    if e.call(IS_KIND_OF_0045BAD0, &args![RTTI_011F4AA0, object])
        .u32()
        & 0xff
        != 0
    {
        let object = e.call(ROOT_OBJECT_009611E0, &args![root]).u32();
        kind = fn_00639a10(e, Ptr::new(object));
    }
    for property in [5u32, 0] {
        if e.call(GET_PROPERTY_00A59D30, &args![model, property]).u32() != 0 {
            let found = e.call(GET_PROPERTY_00A59D30, &args![model, property]).u32();
            e.call(ATTACH_PROPERTY_00439410, &args![shape, found]);
        }
    }
    e.call(REMOVE_PROPERTY_00A5B230, &args![shape, 3u32]);
    let snow = u32::from(snow & 0xff != 0);
    let block = e.call(NEW_OBJECT, &args![0xa0u32]).u32();
    let property = if block != 0 {
        e.call(
            PRECIPITATION_PROPERTY_00BB0D40,
            &args![
                block,
                u32::from(count),
                float_arg(e, extent),
                rotated,
                low,
                high,
                snow,
                kind
            ],
        )
        .u32()
    } else {
        0
    };
    e.call(ATTACH_PROPERTY_00439410, &args![shape, property]);
    e.call(PROPERTY_SET_005A8060, &args![property, 0x1eu32]);
    e.call(PREPARE_OBJECT_00B57E30, &args![shape, 0u32, 0u32]);
    shape
}

/// [`emitter_transform`] for the box, sphere and cylinder emitters: their
/// `006a1b60` object (`first`, already fetched) and `0044edb0` object.
fn emitter_transform_split(
    e: &mut Engine,
    emitter: u32,
    slots: [u32; 5],
    first: u32,
    transform: u32,
    velocity: u32,
    rotated: u32,
) {
    let [first_copy, second_copy, inverse, product, rotation] = slots;
    let world = e.call(WORLD_TRANSFORM_00461130, &args![first]).u32();
    copy_words(e, world, first_copy, 13);
    let second = e.call(EMITTER_OBJECT_0044EDB0, &args![emitter]).u32();
    let world = e.call(WORLD_TRANSFORM_00461130, &args![second]).u32();
    copy_words(e, world, second_copy, 13);
    e.call(TRANSFORM_CONSTRUCT_00476A80, &args![inverse]);
    e.call(TRANSFORM_INVERT_004B4880, &args![second_copy, inverse]);
    let result = e
        .call(
            TRANSFORM_PRODUCT_0062C250,
            &args![inverse, product, first_copy],
        )
        .u32();
    copy_words(e, result, transform, 13);
    let point = e
        .call(
            TRANSFORM_ROTATE_004B4500,
            &args![transform, rotation, velocity],
        )
        .u32();
    copy_words(e, point, rotated, 3);
}

/// The sphere's and cylinder's corners: `(-r, -r, low height)` and
/// `(r, r, high height)` with `r` from `005081b0` (asked for each value).
#[allow(clippy::too_many_arguments)]
fn radius_corners(
    e: &mut Engine,
    emitter: u32,
    temporaries: [u32; 2],
    scale: u32,
    rotated_z: u32,
    translate_z: u32,
    low: u32,
    high: u32,
) {
    let z = height_bound(e, CLAMP_0040EBD0, scale, rotated_z, translate_z);
    let y = x87_store32(-e.call(GETTER_005081B0, &args![emitter]).f64());
    let x = x87_store32(-e.call(GETTER_005081B0, &args![emitter]).f64());
    corner(e, temporaries[0], low, x, y, z);
    let z = height_bound(e, CLAMP_00404010, scale, rotated_z, translate_z);
    let y = x87_store32(e.call(GETTER_005081B0, &args![emitter]).f64());
    let x = x87_store32(e.call(GETTER_005081B0, &args![emitter]).f64());
    corner(e, temporaries[1], high, x, y, z);
}

#[cfg(test)]
mod n086b_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    fn st0(value: f32) -> Ret {
        Ret {
            st0: f64::from(value),
            ..Ret::default()
        }
    }

    fn floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in (0u32..).zip(values) {
            e.mem.set_f32(at + i * 4, *v);
        }
    }

    fn read3(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    /// An object with a kind (`+4`, what the kind-of doubles test) and a
    /// vtable whose slots at `offsets` go to `0x7700_0000 + offset`.
    fn object(e: &mut Engine, kind: u32, offsets: &[u32]) -> u32 {
        let object = e.mem.alloc(0x100);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, kind);
        for &offset in offsets {
            e.mem.set_u32(vtable + offset, 0x7700_0000 + offset);
        }
        object
    }

    struct Scene {
        root: u32,
        emitter: u32,
    }

    /// The doubles of every callee, the globals, and a root with an
    /// emitter controller (birth rate 2 times the emitter's 1: two
    /// particles) whose emitter has the kind `kind`.
    fn scene(e: &mut Engine, root_kind: u32, kind: u32) -> Scene {
        for page in [
            0x0101_1000u32,
            0x0101_2000,
            0x0102_8000,
            0x011c_c000,
            0x011f_4000,
            0x011f_5000,
            0x0102_9000,
            0x0101_a000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_u64(HALF_01011588, 0.5f64.to_bits());
        e.mem.set_u64(TWO_01011590, 2.0f64.to_bits());
        e.mem.set_f32(CORNER_01012054, -1.0);
        e.mem.set_u64(RANDOM_DIVISOR_01029790, 32767.0f64.to_bits());
        e.mem.set_u64(RANDOM_ADDEND_0101A6B0, (-1.0f64).to_bits());
        e.mem.set_f32(TABLE_DIVISOR_011CCB34, 1.0);
        e.mem.set_u64(TABLE_NUMERATOR_01028338, 1.0f64.to_bits());
        e.mem.set_f32(TABLE_011F52E0, 0.0);
        e.mem.set_f32(TABLE_011F4AE0, 1.0);
        e.register(RAND, |_, _| 0u32.into_ret());
        e.register(FTOL, |_, a| {
            (f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)) as i32 as u32).into_ret()
        });
        e.register(IS_KIND_OF_0043B300, |e, a| {
            u32::from(a[1] != 0 && e.mem.u32(a[1] + 4) == a[0]).into_ret()
        });
        e.register(KIND_OF_CAST, |e, a| {
            let pass = [
                RTTI_EMITTER_01202540,
                RTTI_011F4ACC,
                RTTI_011F4A88,
                INTERPOLATOR_RTTI_011F3738,
            ];
            let hit = a[1] != 0 && (pass.contains(&a[0]) || e.mem.u32(a[1] + 4) == a[0]);
            (if hit { a[1] } else { 0 }).into_ret()
        });
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(CHILD_AT_004D7D70, |e, a| {
            e.mem.u32(a[0] + 0x80 + a[1] * 4).into_ret()
        });
        e.register(NEXT_CONTROLLER_004A8A90, |_, _| 0u32.into_ret());
        e.register(GETTER_004A7BD0, |e, a| st0(e.mem.f32(a[0] + 0x50)));
        e.register(GETTER_005081B0, |e, a| st0(e.mem.f32(a[0] + 0x58)));
        for addr in [
            GETTER_00495460,
            GETTER_007DF1F0,
            GETTER_00526AC0,
            GETTER_00526AE0,
            GETTER_00759450,
        ] {
            e.register(addr, |_, _| st0(0.0));
        }
        e.register(GETTER_00792760, |_, _| st0(3.0));
        e.register(MIN_004865F0, |_, a| a[0].min(a[1]).into_ret());
        e.register(ALLOCATE_00401000, |e, a| {
            e.mem.alloc(a[0].min(0x1000)).into_ret()
        });
        e.register(ALLOCATE_00AA1070, |e, a| {
            e.mem.alloc(a[0].min(0x1000)).into_ret()
        });
        e.register(NEW_OBJECT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(TRI_SHAPE_DYNAMIC_DATA_00A816A0, |_, a| a[0].into_ret());
        e.register(TRI_SHAPE_00A74480, |_, a| a[0].into_ret());
        e.register(POINT_SET_00416870, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[0] + i * 4, a[1 + i as usize]);
            }
            a[0].into_ret()
        });
        e.register(POINT_SCALED_004A3760, |e, a| {
            let s = f32::from_bits(a[1]);
            for i in 0..3 {
                let v = e.mem.f32(a[2] + i * 4) * s;
                e.mem.set_f32(a[0] + i * 4, v);
            }
            a[0].into_ret()
        });
        e.register(TEXTURE_POINT_00452DC0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            a[0].into_ret()
        });
        e.register(POINT_TIMES_0045BB20, |e, a| {
            let s = f32::from_bits(a[2]);
            for i in 0..3 {
                let v = e.mem.f32(a[0] + i * 4) * s;
                e.mem.set_f32(a[1] + i * 4, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_PLUS_00439E90, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + i * 4) + e.mem.f32(a[2] + i * 4);
                e.mem.set_f32(a[1] + i * 4, v);
            }
            a[1].into_ret()
        });
        // The transform of a point: plus its translation (+0x24).
        e.register(TRANSFORM_POINT_00524C40, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[2] + i * 4) + e.mem.f32(a[0] + 0x24 + i * 4);
                e.mem.set_f32(a[1] + i * 4, v);
            }
            a[1].into_ret()
        });
        e.register(TRANSFORM_ROTATE_004B4500, |e, a| {
            for i in 0..3 {
                let v = e.mem.u32(a[2] + i * 4);
                e.mem.set_u32(a[1] + i * 4, v);
            }
            a[1].into_ret()
        });
        e.register(SINE_COSINE_004169A0, |e, a| {
            e.mem.set_f32(a[1], 0.5);
            e.mem.set_f32(a[2], 0.25);
            Ret::default()
        });
        e.register(WORLD_TRANSFORM_00461130, |_, a| a[0].into_ret());
        e.register(TRANSFORM_PRODUCT_0062C250, |e, a| {
            for i in 0..13 {
                let v = e.mem.u32(a[2] + i * 4);
                e.mem.set_u32(a[1] + i * 4, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_CONSTRUCT_006815C0, |_, a| a[0].into_ret());
        e.register(CLAMP_0040EBD0, |_, a| {
            st0(f32::from_bits(a[0]).min(f32::from_bits(a[1])))
        });
        e.register(CLAMP_00404010, |_, a| {
            st0(f32::from_bits(a[0]).max(f32::from_bits(a[1])))
        });
        e.register(BOUND_00537B10, |_, a| (a[0] + 0x60).into_ret());
        e.register(BOUND_RADIUS_0084D030, |e, a| st0(e.mem.f32(a[0] + 0xc)));
        e.register(ROOT_OBJECT_009611E0, |_, a| a[0].into_ret());
        e.register(IS_KIND_OF_0045BAD0, |_, _| 0u32.into_ret());
        e.register(GET_PROPERTY_00A59D30, |_, _| 0u32.into_ret());
        e.register(PRECIPITATION_PROPERTY_00BB0D40, |_, a| a[0].into_ret());
        quiet(
            e,
            &[
                TRANSFORM_CONSTRUCT_00476A80,
                TRANSFORM_INVERT_004B4880,
                VECTOR_CONSTRUCTOR_ITERATOR,
                SET_DATA_BLOCK_COUNT,
                SET_DATA_BLOCK,
                SET_DATA_STREAM,
                FREE_00401030,
                SHAPE_SET_DATA_00539EF0,
                BOUND_SET_RADIUS_0063F790,
                SHAPE_SET_BOUND_004A5200,
                ATTACH_PROPERTY_00439410,
                REMOVE_PROPERTY_00A5B230,
                PROPERTY_SET_005A8060,
                PREPARE_OBJECT_00B57E30,
                NI_OBJECT_CONSTRUCTOR,
                MEMBER_INIT_00540830,
                MEMBER_INIT_005E03D0,
            ],
        );
        let root = object(e, root_kind, &[]);
        let interpolator = object(e, 0, &[0x9c]);
        e.register(0x7700_009c, |e, a| {
            e.mem.set_f32(a[3], 2.0);
            Ret::default()
        });
        let emitter = object(e, kind, &[0xa8]);
        floats(e, emitter + 0x48, &[1.0]);
        floats(e, emitter + 0x50, &[5.0, 0.0, 6.0, 4.0]);
        let controller = object(e, RTTI_EMITTER_CONTROLLER_012024E8, &[]);
        e.mem.set_u32(controller + 0x34, interpolator);
        e.mem.set_u32(controller + 0x3c, emitter);
        e.register_double(FIRST_CONTROLLER_0043B230, move |_, _| controller.into_ret());
        // The emitter's objects: one translated to (10, 20, 30).
        let emitter_object = e.mem.alloc(0x40);
        floats(e, emitter_object + 0x24, &[10.0, 20.0, 30.0]);
        e.register_double(EMITTER_OBJECT_006A1B60, move |_, _| {
            emitter_object.into_ret()
        });
        let other = e.mem.alloc(0x40);
        e.register_double(EMITTER_OBJECT_0044EDB0, move |_, _| other.into_ret());
        Scene { root, emitter }
    }

    fn build(e: &mut Engine, s: &Scene, snow: u32) -> u32 {
        e.call_log = Some(vec![]);
        e.call(0x0063_7890, &args![0x1234u32, s.root, 0x9999u32, snow])
            .u32()
    }

    #[test]
    fn no_emitter_builds_nothing() {
        let mut e = Engine::new();
        let s = scene(&mut e, 0, RTTI_BOX_EMITTER_01202518);
        e.register(FIRST_CONTROLLER_0043B230, |_, _| 0u32.into_ret());
        assert_eq!(build(&mut e, &s, 0), 0);
        assert!(calls(&e, NEW_OBJECT).is_empty());
    }

    #[test]
    fn quads_with_a_box_emitter() {
        let mut e = Engine::new();
        let s = scene(&mut e, 0, RTTI_BOX_EMITTER_01202518);
        let shape = build(&mut e, &s, 1);
        assert_ne!(shape, 0);
        // Two quads: four corners each, scaled by 3.
        let data = &calls(&e, TRI_SHAPE_DYNAMIC_DATA_00A816A0)[0];
        assert_eq!(data[1], 8);
        assert_eq!(data[8], 4);
        let (positions, coordinates, indices) = (data[2], data[5], data[9]);
        assert_eq!(read3(&e, positions), [-3.0, 0.0, -3.0]);
        assert_eq!(read3(&e, positions + 0xc), [3.0, 0.0, -3.0]);
        assert_eq!(read3(&e, positions + 0x18), [3.0, 0.0, 3.0]);
        assert_eq!(read3(&e, positions + 0x24), [-3.0, 0.0, 3.0]);
        assert_eq!(read3(&e, positions + 0x54), [-3.0, 0.0, 3.0]);
        assert_eq!(
            [e.mem.f32(coordinates), e.mem.f32(coordinates + 4)],
            [0.0, 1.0]
        );
        assert_eq!(
            [e.mem.f32(coordinates + 0x10), e.mem.f32(coordinates + 0x14)],
            [1.0, 0.0]
        );
        let index: Vec<u16> = (0..12).map(|i| e.mem.u16(indices + i * 2)).collect();
        assert_eq!(index, vec![0, 1, 2, 3, 0, 2, 4, 5, 6, 7, 4, 6]);
        // The box: half sizes 3 and 2 around the emitter object; heights
        // from its z (30) to z plus the speed (5).
        let property = &calls(&e, PRECIPITATION_PROPERTY_00BB0D40)[0];
        assert_eq!(property[1], 2);
        assert_eq!(read3(&e, property[3]), [0.0, 0.0, 5.0]);
        assert_eq!(read3(&e, property[4]), [-3.0, -2.0, 30.0]);
        assert_eq!(read3(&e, property[5]), [3.0, 2.0, 35.0]);
        assert_eq!(property[6..], [1, 1]);
        // Each particle at the box's random point (-3, -2, 0) moved by the
        // translation, for all four vertices of both particles.
        let block = &calls(&e, SET_DATA_BLOCK)[0];
        assert_eq!(block[3], 8 * 0xc);
        for i in 0..8 {
            assert_eq!(read3(&e, block[2] + i * 0xc), [7.0, 18.0, 30.0]);
        }
        assert_eq!(
            calls(&e, SET_DATA_STREAM)[0][1..],
            [0, 0, 0, 3, 8, 0xc, 0xc]
        );
        assert_eq!(calls(&e, BOUND_SET_RADIUS_0063F790)[0][1], 3.0f32.to_bits());
        assert_eq!(calls(&e, PREPARE_OBJECT_00B57E30), vec![vec![shape, 0, 0]]);
        assert_eq!(calls(&e, REMOVE_PROPERTY_00A5B230), vec![vec![shape, 3]]);
    }

    #[test]
    fn model_with_a_sphere_emitter() {
        let mut e = Engine::new();
        let s = scene(
            &mut e,
            RTTI_MODEL_NODE_01202538,
            RTTI_SPHERE_EMITTER_01202510,
        );
        // The first child's holder (00639a60 0) answers the model object.
        let first = object(&mut e, RTTI_CHILD_01202530, &[]);
        let second = object(&mut e, RTTI_CHILD_01202528, &[]);
        floats(&mut e, second + 0x50, &[7.5]);
        e.mem.set_u32(s.root + 0x80, first);
        e.mem.set_u32(s.root + 0x84, second);
        e.register(ARRAY_COUNT_00658930, |_, _| 1u32.into_ret());
        e.register(ARRAY_AT_00877A30, |_, a| (a[0] + 8).into_ret());
        let holder = object(&mut e, 0, &[0xc]);
        e.mem.set_u32(first + 0x20, holder);
        let model = object(&mut e, 0, &[]);
        e.register_double(0x7700_000c, move |_, _| 0x4545u32.into_ret());
        e.register_double(CHILD_OBJECT_0045BC00, move |_, _| model.into_ret());
        e.register(MODEL_GEOMETRY_005495F0, |_, a| a[0].into_ret());
        e.register(VERTEX_COUNT_004A8AE0, |_, _| 3u32.into_ret());
        e.register(TRIANGLE_COUNT_008041A0, |_, _| 3u32.into_ret());
        let source = e.mem.alloc(0x40);
        floats(
            &mut e,
            source,
            &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
        );
        e.register_double(VERTICES_007AF430, move |_, _| source.into_ret());
        let uv = e.mem.alloc(0x40);
        e.register_double(TEXTURE_COORDINATES_0055B980, move |_, _| uv.into_ret());
        let triangles = e.mem.alloc(8);
        for (i, v) in (0u32..).zip([2u16, 0, 1]) {
            e.mem.set_u16(triangles + i * 2, v);
        }
        e.register_double(TRIANGLES_00A7B850, move |_, _| triangles.into_ret());
        let shape = build(&mut e, &s, 0);
        assert_ne!(shape, 0);
        let data = &calls(&e, TRI_SHAPE_DYNAMIC_DATA_00A816A0)[0];
        assert_eq!(data[1], 6);
        assert_eq!(data[8], 2);
        assert_eq!(read3(&e, data[2] + 0x24), [1.0, 2.0, 3.0]);
        let index: Vec<u16> = (0..6).map(|i| e.mem.u16(data[9] + i * 2)).collect();
        assert_eq!(index, vec![2, 0, 1, 5, 3, 4]);
        let property = &calls(&e, PRECIPITATION_PROPERTY_00BB0D40)[0];
        assert_eq!(property[2], 7.5f32.to_bits());
        assert_eq!(read3(&e, property[4]), [-6.0, -6.0, 30.0]);
        assert_eq!(read3(&e, property[5]), [6.0, 6.0, 35.0]);
        assert_eq!(property[6], 0);
        // The particles: radius 0 at the random 0, moved to the translation.
        let block = &calls(&e, SET_DATA_BLOCK)[0];
        assert_eq!(read3(&e, block[2]), [10.0, 20.0, 30.0]);
        assert_eq!(calls(&e, ATTACH_PROPERTY_00439410).len(), 1);
    }

    #[test]
    fn cylinder_and_mesh_emitters() {
        let mut e = Engine::new();
        let s = scene(&mut e, 0, RTTI_CYLINDER_EMITTER_01202508);
        build(&mut e, &s, 0);
        let property = &calls(&e, PRECIPITATION_PROPERTY_00BB0D40)[0];
        assert_eq!(read3(&e, property[4]), [-6.0, -6.0, 30.0]);
        // The cylinder's point: distance 0, height (0 - 0.5) * 4, moved.
        let block = &calls(&e, SET_DATA_BLOCK)[0];
        assert_eq!(read3(&e, block[2]), [10.0, 20.0, 28.0]);
        assert_eq!(calls(&e, SINE_COSINE_004169A0).len(), 2);

        let mut e = Engine::new();
        let s = scene(&mut e, 0, RTTI_MESH_EMITTER_01202500);
        // The mesh's first object (00639ac0 0): its bound centered at
        // (1, 2, 3) with radius 0.5.
        e.register(CALLEE_009938B0, |_, _| 1u32.into_ret());
        e.register(ARRAY_AT_00877A30, |_, a| (a[0] + 8).into_ret());
        let holder = e.mem.alloc(0x100);
        floats(&mut e, holder + 0x24, &[10.0, 20.0, 30.0]);
        floats(&mut e, holder + 0x60, &[1.0, 2.0, 3.0, 0.5]);
        e.mem.set_u32(s.emitter + 0x54 + 8, holder);
        e.register(0x7700_00a8, |e, a| {
            for (i, v) in (0u32..).zip([1.0f32, 2.0, 3.0]) {
                e.mem.set_f32(a[1] + i * 4, v);
            }
            Ret::default()
        });
        build(&mut e, &s, 0);
        let property = &calls(&e, PRECIPITATION_PROPERTY_00BB0D40)[0];
        assert_eq!(read3(&e, property[4]), [0.5, 1.5, 30.0]);
        assert_eq!(read3(&e, property[5]), [1.5, 2.5, 35.0]);
        let block = &calls(&e, SET_DATA_BLOCK)[0];
        assert_eq!(read3(&e, block[2]), [1.0, 2.0, 3.0]);
        let mesh_calls = calls(&e, 0x7700_00a8);
        assert_eq!(mesh_calls.len(), 2);
        assert_eq!(mesh_calls[0][0], s.emitter);
    }

    #[test]
    fn another_emitter_kind_deletes_the_shape() {
        let mut e = Engine::new();
        let s = scene(&mut e, 0, 0x0bad_0000);
        let deleting = 0x7700_0000u32;
        e.register(TRI_SHAPE_00A74480, |e, a| {
            let vtable = e.mem.alloc(8);
            e.mem.set_u32(vtable, 0x7700_0000);
            e.mem.set_u32(a[0], vtable);
            a[0].into_ret()
        });
        e.register(deleting, |_, _| Ret::default());
        assert_eq!(build(&mut e, &s, 0), 0);
        let deleted = calls(&e, deleting);
        assert_eq!(deleted.len(), 1);
        assert_eq!(deleted[0][1], 1);
        assert!(calls(&e, PRECIPITATION_PROPERTY_00BB0D40).is_empty());
    }
}
