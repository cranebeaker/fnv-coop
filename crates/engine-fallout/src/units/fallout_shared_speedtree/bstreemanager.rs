//! `fallout shared/speedtree/bstreemanager.cpp` (Xbox PDB source unit), subsystem `fallout shared/speedtree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The `BSTreeManager` class itself is declared in
//! `units/unplaced/run_00664440.rs` (its constructor, destructor and
//! singleton functions sit before this unit's first function in the exe).
//! This file holds the rest of the unit: the model lookup, the tree cache
//! clean-up, the per-frame update, the canopy shadow texture and the two
//! container classes the manager uses (the pointer map from tree form to
//! model array, instantiated here, and its base).
//!
//! Notes on the code the exe was compiled from:
//! - Every function that takes the manager from `00664840(1)` calls the
//!   accessor again at every use (the singleton is re-read each time).
//! - Functions that open a `BSCriticalSection` scope guard
//!   (`00404eb0`/`00404ee0`) are translated with it as a block: the guard is
//!   destroyed on every exit, after the `NiPointer` locals.
//! - x87: float arithmetic is computed in `f64` and rounded to `f32` at each
//!   `float` store; copies through `FLD`/`FSTP` quieten signalling NaNs.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTPointerMap;
use crate::units::unplaced::run_00664440::BSTreeManager;

/// The `BSTreeManager` singleton accessor (`00664840`, cdecl: create flag).
const SINGLETON: u32 = 0x0066_4840;
/// `NiPointer<T>::operator=(T *)` (`0066b0d0`, thiscall).
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// `NiPointer<T>::operator=(const NiPointer<T> &)` (`006e5cc0`, thiscall:
/// the other pointer is passed by address).
const NI_POINTER_COPY: u32 = 0x006e_5cc0;
/// `NiPointer` constructor from a pointer (`00633c90`, thiscall).
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
/// `NiPointer` destructor (`0045cec0`).
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `*this` of a `NiPointer` (`00559450`, thiscall: the stored word).
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `operator new` (`00401000`), `operator delete` (`00401030`) and the
/// allocator used by `new NiObject` (`00aa13e0`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// Allocator (`00aa1070`, one word: bytes) and `memset` (`00403d30`, cdecl).
const ALLOCATE_BYTES: u32 = 0x00aa_1070;
const MEMSET: u32 = 0x0040_3d30;
/// Critical-section scope guard: constructor (`00404eb0`: kind, flag, source
/// path, line) and destructor (`00404ee0`); the source path
/// `BSTreeManager.cpp` (`01066398`).
const GUARD_CONSTRUCT: u32 = 0x0040_4eb0;
const GUARD_DESTRUCT: u32 = 0x0040_4ee0;
const SOURCE_PATH_STRING: u32 = 0x0106_6398;
/// The `BSCriticalSection` at `011d5cc0`: enter (`004538a0`, one word) and
/// leave (`004538c0`).
const LOCK: u32 = 0x011d_5cc0;
const LOCK_ENTER: u32 = 0x0045_38a0;
const LOCK_LEAVE: u32 = 0x0045_38c0;
/// The `TES` singleton pointer (`011dea10`).
const TES_SINGLETON: u32 = 0x011d_ea10;
/// `Data\Textures\Trees\CanopyShadow.dds` (`010663fc`), the name
/// `CanopyShadowTexture` (`010663e8`).
const CANOPY_SHADOW_PATH: u32 = 0x0106_63fc;
const CANOPY_TEXTURE_NAME: u32 = 0x0106_63e8;
/// Vtable of the model-array map class (`NiTPointerMap<TESObjectTREE *,
/// NiPointer<BSTreeModel> *>`, `01066434`) and of its base `NiTMapBase<..>`
/// (`010664c4`), both confirmed by their RTTI.
const MAP_VTABLE: u32 = 0x0106_6434;
const MAP_BASE_VTABLE: u32 = 0x0106_64c4;
/// The map's base class destructor (`00666020`) and the `00438af0` that both
/// map destructors call first (`ECX` = the map).
const MAP_BASE_DESTRUCT: u32 = 0x0066_6020;
const MAP_CLEAR: u32 = 0x0043_8af0;
/// Map operations of `pModelMap` used by the clean-up: first item cursor
/// (`004b9ba0`), next item (`006b7f20`: cursor, key, value by address),
/// remove by key (`00405430`) and the emptiness test (`0069b080`).
const MAP_FIRST: u32 = 0x004b_9ba0;
const MAP_NEXT: u32 = 0x006b_7f20;
const MAP_REMOVE_AT: u32 = 0x0040_5430;
const MAP_IS_EMPTY: u32 = 0x0069_b080;
/// Find (`00853130`: key, out value) and set (`00844700`: key, value) of
/// the same map class.
const MAP_FIND: u32 = 0x0085_3130;
const MAP_SET_AT: u32 = 0x0084_4700;
/// `_eh_vector_constructor_iterator_` (`00ec782f`, stdcall: first element,
/// element size, count, constructor, destructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// The per-element constructor handed to it (`006694e0`).
const MODEL_SLOT_CONSTRUCT: u32 = 0x0066_94e0;
/// Vector deleting destructor of a model array (`00667120`, flags).
const MODEL_ARRAY_DELETE: u32 = 0x0066_7120;
/// Reference count word of a counted object (`00726070`, reads `+4`) and the
/// seed of a model (`008d6f30`, reads `+0x40`).
const REFERENCE_COUNT: u32 = 0x0072_6070;
const MODEL_SEED: u32 = 0x008d_6f30;
/// `BSTreeModel::InitAsInstance` (`0066a650`, Xbox PDB name) and
/// `BSTreeModel::InitFromBase` (`0066ac40`, Xbox PDB name).
const MODEL_INIT_AS_INSTANCE: u32 = 0x0066_a650;
const MODEL_INIT_FROM_BASE: u32 = 0x0066_ac40;
/// The global `NiPointer` assignment (`00664820`, cdecl) that mirrors the
/// canopy shadow texture.
const CANOPY_SHADOW_GLOBAL_ASSIGN: u32 = 0x0066_4820;
/// `TES::CreateTextureImage` (`004568c0`) and `TES::RemoveTextureImage`
/// (`00456800`) (Xbox PDB names, from the decompilation of the exe).
const CREATE_TEXTURE_IMAGE: u32 = 0x0045_68c0;
const REMOVE_TEXTURE_IMAGE: u32 = 0x0045_6800;
/// `NiPixelData` constructor (`00a7c190`: width, height, pixel format
/// address, mip levels, faces), the pixel format at `011aa2e8`, the pixel
/// buffer accessor (`0064dac0`: two zero words) and `004f31b0`.
const PIXEL_DATA_CONSTRUCT: u32 = 0x00a7_c190;
const PIXEL_FORMAT: u32 = 0x011a_a2e8;
const PIXEL_DATA_PIXELS: u32 = 0x0064_dac0;
const PIXEL_DATA_FINISH: u32 = 0x004f_31b0;
/// `NiFixedString` constructor from a C string (`00438170`), its C string
/// accessor (`0043b1b0`) and its destructor (`004381b0`).
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
const FIXED_STRING_C_STR: u32 = 0x0043_b1b0;
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
/// `NiSourceTexture::Create` from pixel data (`00a5ff10`: pixel data, name,
/// the format preferences at `011a9598`).
const SOURCE_TEXTURE_CREATE: u32 = 0x00a5_ff10;
const TEXTURE_PREFERENCES: u32 = 0x011a_9598;
/// `NiObjectNET::SetName`-like setter used on the created tree node
/// (`00a5b950`, one `NiFixedString` pointer).
const NODE_SET_NAME: u32 = 0x00a5_b950;
/// Statics the wind update keeps: the elapsed-time accumulator
/// (`011d5dd4`), four phase accumulators (`011d5dd8`), the previous wind
/// speed (`011d5de8`) and the bit that says it has been initialised
/// (`011d5dec`, bit 0).
const TIME_ACCUMULATOR: u32 = 0x011d_5dd4;
const PHASES: u32 = 0x011d_5dd8;
const LAST_WIND_SPEED: u32 = 0x011d_5de8;
const LAST_WIND_SPEED_READY: u32 = 0x011d_5dec;
/// The float static `00665520` fills (`011adce0`).
const CLAMPED_STATIC: u32 = 0x011a_dce0;
/// The `float` table of the four sway directions (`0119b8cc`, 8 bytes per
/// direction: two floats).
const SWAY_TABLE: u32 = 0x0119_b8cc;
/// The object `0084d030` reads (`011f6394`, called with it in `ECX`) and
/// `0.0`/`1.0`/`2.0`/`20.0`/`0.61` as `double` constants, `-1.0` as `float`.
const FRAME_TIME_SOURCE: u32 = 0x011f_6394;
const DOUBLE_ZERO: u32 = 0x0101_2060;
const DOUBLE_ONE: u32 = 0x0101_2070;
const DOUBLE_TWO: u32 = 0x0101_1590;
const DOUBLE_TWENTY: u32 = 0x0102_fc70;
const DOUBLE_WIND_SCALE: u32 = 0x0106_6428;
const FLOAT_MINUS_ONE: u32 = 0x0101_2054;
/// Settings read through `00403e20(setting)` (pointer to the value at
/// `+4`; a static zero for a null setting) by the functions below.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The `float` setting objects: those `006655a0`/`006655f0` read, the
/// shared multiplier of `00665570`/`006655c0` and those of the wind update.
const SETTING_A: u32 = 0x011d_5d64;
const SETTING_B: u32 = 0x011d_5d74;
const SETTING_MULTIPLIER: u32 = 0x011f_9158;
const SETTING_TO_CLAMP: u32 = 0x011d_5d4c;
const TREE_DISTANCE_SETTING: u32 = 0x011d_5d98;
/// Accessor of an integer setting's value pointer (`0043d4d0`).
const SETTING_VALUE_WORD: u32 = 0x0043_d4d0;
/// The setter of the tree distance float static (`00664720`).
const SET_TREE_DISTANCE: u32 = 0x0066_4720;
/// `0084d030`: a float read from the object in `ECX` (returned in `ST0`).
const FRAME_TIME: u32 = 0x0084_d030;
/// Wind update helpers: `CSpeedTreeRT::SetCamera` (`00b06c10`, Xbox PDB name:
/// position, direction), `00b07060` (one float: the time accumulator),
/// `00bb2fc0` (index, two floats), `00bb1390` (four floats), the two
/// float functions `0063c600`/`0057e960` of the sway and `00408840`.
const SPEEDTREE_SET_CAMERA: u32 = 0x00b0_6c10;
const SPEEDTREE_SET_TIME: u32 = 0x00b0_7060;
const SPEEDTREE_SET_WIND_DIRECTION: u32 = 0x00bb_2fc0;
const SPEEDTREE_SET_LIGHTING: u32 = 0x00bb_1390;
const SWAY_FIRST: u32 = 0x0063_c600;
const SWAY_SECOND: u32 = 0x0057_e960;
const SWAY_TERM: u32 = 0x0040_8840;
/// Constructors/accessors of the camera vectors: `006815c0` (constructs the
/// vector in `ECX`), `0045bb80` (pointer to the position at `+0x8c`),
/// `0045bba0` (direction, out pointer), `004a3760` (scalar times vector,
/// out pointer first).
const VECTOR_CONSTRUCT: u32 = 0x0068_15c0;
const OBJECT_POSITION: u32 = 0x0045_bb80;
const OBJECT_DIRECTION: u32 = 0x0045_bba0;
const SCALED_VECTOR: u32 = 0x004a_3760;
/// Reads of the `TES` object the update makes: `008d8520` (the word at
/// `+0x68`), `005f36f0` (the word at `+0x34`; ENGINE_CRATE.md notes the map
/// name of this address is wrong and that it returns `TES::pInteriorCell`),
/// `004536e0` (a float of the object in `ECX`, returned in `ST0`),
/// `00450570`, `00891170` (pointer to three floats) and the three material
/// colour setters.
const TES_FIELD_68: u32 = 0x008d_8520;
const TES_INTERIOR_CELL: u32 = 0x005f_36f0;
const FLOAT_OF_OBJECT: u32 = 0x0045_36e0;
const OBJECT_OF_OBJECT: u32 = 0x0045_0570;
const COLOR_POINTER: u32 = 0x0089_1170;
const COLOR_SETTER_NOOP: u32 = 0x0045_34f0;
const COLOR_SETTER: u32 = 0x004b_c450;
/// Test of the global tree setting byte (`0051d740`) and the form/reference
/// functions `00664f50` calls: `0048cee0` (on the form's model sub-object
/// at `+0x30`), `007af430` (on the reference), `005692d0` (seed of the
/// reference), `00567400` (scale of the reference, returned in `ST0`).
const TREES_ENABLED: u32 = 0x0051_d740;
const FORM_MODEL_CHECK: u32 = 0x0048_cee0;
const REFERENCE_BASE_FORM: u32 = 0x007a_f430;
const REFERENCE_SEED: u32 = 0x0056_92d0;
const REFERENCE_SCALE: u32 = 0x0056_7400;
/// Release of a background-loaded tree (`00c45830`, cdecl).
const RELEASE_TREE: u32 = 0x00c4_5830;
/// Vtable slots of the background-loaded tree map: find (`+8`), insert
/// (`+0x10`), remove (`+0x14`); of the model: create art (`+0xc`).
const SLOT_FIND: u32 = 0x08;
const SLOT_INSERT: u32 = 0x10;
const SLOT_REMOVE: u32 = 0x14;
const SLOT_CREATE_ART: u32 = 0x0c;
/// `BSTreeModel` constructor's base-class initialiser (`004968b0`: stores the
/// base vtable and a zero count at `+4`, and bumps a static object counter;
/// the Xbox PDB base class is `NiRefObject`, the name of this address is
/// unconfirmed) and the `BSTreeModel` vtable (`01066768`, confirmed by RTTI).
const NI_REF_OBJECT_CONSTRUCT: u32 = 0x0049_68b0;
const MODEL_VTABLE: u32 = 0x0106_6768;

/// `FLD float [address]`: the value as the x87 stack holds it.
fn load_float(e: &Engine, address: u32) -> f32 {
    f32::from_bits(x87_quiet_bits(e.mem.u32(address)))
}

/// Value pointer of a setting object, as `00403e20` returns it.
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let pointer = e.call(SETTING_VALUE, &args![setting]).u32();
    load_float(e, pointer)
}

/// The singleton, created on demand (`00664840(1)`).
fn manager(e: &mut Engine) -> Ptr<BSTreeManager> {
    e.call(SINGLETON, &args![1u32]).ptr()
}

/// The word a `NiPointer` at `address` holds (`00559450`).
fn pointer_get(e: &mut Engine, address: u32) -> u32 {
    e.call(NI_POINTER_GET, &args![address]).u32()
}

/// The manager's model map, read through a fresh singleton access.
fn model_map(e: &mut Engine) -> u32 {
    let manager = manager(e);
    e.get(manager, BSTreeManager::pModelMap).addr()
}

/// Runs `body` inside the `BSCriticalSection` scope guard the unit's
/// functions open at their top (`00404eb0(0x1e, 1, path, line)`), destroying
/// it on every exit.
fn with_guard<R>(e: &mut Engine, line: u32, body: impl FnOnce(&mut Engine) -> R) -> R {
    e.with_stack(8, |e, guard| {
        e.call(
            GUARD_CONSTRUCT,
            &args![guard, 0x1eu32, 1u32, SOURCE_PATH_STRING, line],
        );
        let result = body(e);
        e.call(GUARD_DESTRUCT, &args![guard]);
        result
    })
}

// Translation incomplete: 006649c0 - C++ exception unwinding frame (FS:[0]) not translated
// Translated from 006649c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeManager::GetModelArray` (Xbox PDB): the array of model slots (one
/// `NiPointer<BSTreeModel>`) kept for the tree `form` in the model map. The
/// map is created (`00665c50(0x25)`) on first use; an unknown form gets a new
/// counted array (count word, then one slot built by the vector constructor
/// iterator) stored with `SetAt`. A null `form` gives null.
pub fn bs_tree_manager_get_model_array(e: &mut Engine, this: Ptr<BSTreeManager>, form: u32) -> u32 {
    with_guard(e, 0xd8, |e| {
        if form == 0 {
            return 0;
        }
        if e.get(this, BSTreeManager::pModelMap).is_null() {
            let memory = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
            let map = if memory != 0 {
                fn_00665c50(e, Ptr::new(memory), 0x25).addr()
            } else {
                0
            };
            e.set(this, BSTreeManager::pModelMap, Ptr::new(map));
        }
        e.with_stack(4, |e, found| {
            let map = e.get(this, BSTreeManager::pModelMap).addr();
            if e.call(MAP_FIND, &args![map, form, found]).bool() {
                return e.mem.u32(found.addr());
            }
            let memory = e.call(OPERATOR_NEW, &args![8u32]).u32();
            let array = if memory != 0 {
                e.mem.set_u32(memory, 1);
                e.call(
                    VECTOR_CONSTRUCTOR_ITERATOR,
                    &args![
                        memory + 4,
                        4u32,
                        1u32,
                        MODEL_SLOT_CONSTRUCT,
                        NI_POINTER_DESTRUCT
                    ],
                );
                memory + 4
            } else {
                0
            };
            e.mem.set_u32(found.addr(), array);
            let map = e.get(this, BSTreeManager::pModelMap).addr();
            e.call(MAP_SET_AT, &args![map, form, array]);
            array
        })
    })
}

// Translation incomplete: 00664b50 - C++ exception unwinding frame (FS:[0]) not translated
// Translated from 00664b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeManager::GetModel` (Xbox PDB): under the `BSCriticalSection` at
/// `011d5cc0`, finds in the form's model array (`GetModelArray`) the model
/// whose seed (`008d6f30`) equals `seed` and stores it in the `NiPointer` at
/// `out`; when none matches, a new `BSTreeModel` (`0x50` bytes, `00666650`)
/// goes into the first empty slot, or, with no empty slot, the first slot's
/// model is returned. A null `form` stores null.
pub fn bs_tree_manager_get_model(
    e: &mut Engine,
    this: Ptr<BSTreeManager>,
    form: u32,
    seed: u32,
    out: u32,
) {
    e.call(LOCK_ENTER, &args![LOCK, 0u32]);
    e.with_stack(4, |e, found| {
        e.call(NI_POINTER_CONSTRUCT, &args![found, 0u32]);
        if form != 0 {
            let array = bs_tree_manager_get_model_array(e, this, form);
            let mut empty_slot: i32 = -1;
            let mut matched = false;
            for slot in 0..1i32 {
                let model = pointer_get(e, array + slot as u32 * 4);
                if model != 0 {
                    if e.call(MODEL_SEED, &args![model]).u32() == seed {
                        e.call(NI_POINTER_ASSIGN, &args![found, model]);
                        matched = true;
                        break;
                    }
                } else if empty_slot == -1 {
                    empty_slot = slot;
                }
            }
            let done = matched && pointer_get(e, found.addr()) != 0;
            if !done {
                if empty_slot < 0 {
                    e.call(NI_POINTER_COPY, &args![found, array]);
                } else {
                    let memory = e.call(NI_ALLOCATE, &args![0x50u32]).u32();
                    let model = if memory != 0 {
                        fn_00666650(e, Ptr::new(memory)).addr()
                    } else {
                        0
                    };
                    e.call(NI_POINTER_ASSIGN, &args![found, model]);
                    e.call(
                        NI_POINTER_COPY,
                        &args![array + empty_slot as u32 * 4, found],
                    );
                }
            }
        }
        e.call(LOCK_LEAVE, &args![LOCK]);
        e.call(NI_POINTER_COPY, &args![out, found]);
        e.call(NI_POINTER_DESTRUCT, &args![found]);
    })
}

// Translated from 00664cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the tree cache (cdecl, one byte). Without a model map it does
/// nothing. Under the `BSCriticalSection` at `011d5cc0` it walks the model
/// map with its cursor. With `only_unreferenced` set it empties each model
/// slot whose model has at most one reference, deletes (flags 3) and removes
/// the arrays whose slots are all free (restarting the walk after each
/// removal), and destroys the map itself, clearing `pModelMap`, only when it
/// ended up empty. Without it every array is deleted, then the map is
/// cleared, destroyed and `pModelMap` cleared.
pub fn fn_00664cd0(e: &mut Engine, only_unreferenced: u8) {
    let this = manager(e);
    if e.get(this, BSTreeManager::pModelMap).is_null() {
        return;
    }
    e.call(LOCK_ENTER, &args![LOCK, 0u32]);
    e.with_stack(0x0c, |e, scratch| {
        let cursor = scratch.addr();
        let key = cursor + 4;
        let value = cursor + 8;
        if only_unreferenced != 0 {
            let map = model_map(e);
            let first = e.call(MAP_FIRST, &args![map]).u32();
            e.mem.set_u32(cursor, first);
            while e.mem.u32(cursor) != 0 {
                let map = model_map(e);
                e.call(MAP_NEXT, &args![map, cursor, key, value]);
                let array = e.mem.u32(value);
                if array != 0 {
                    let mut all_free = true;
                    for slot in 0..1u32 {
                        let slot_address = array + slot * 4;
                        if pointer_get(e, slot_address) != 0 {
                            let model = pointer_get(e, slot_address);
                            if e.call(REFERENCE_COUNT, &args![model]).u32() <= 1 {
                                e.call(NI_POINTER_ASSIGN, &args![slot_address, 0u32]);
                            } else {
                                all_free = false;
                            }
                        }
                    }
                    if all_free {
                        e.call(MODEL_ARRAY_DELETE, &args![array, 3u32]);
                        e.mem.set_u32(value, 0);
                        let removed_key = e.mem.u32(key);
                        let map = model_map(e);
                        e.call(MAP_REMOVE_AT, &args![map, removed_key]);
                        let map = model_map(e);
                        let first = e.call(MAP_FIRST, &args![map]).u32();
                        e.mem.set_u32(cursor, first);
                    }
                }
            }
            let map = model_map(e);
            if e.call(MAP_IS_EMPTY, &args![map]).bool() {
                let map = model_map(e);
                e.call(MAP_CLEAR, &args![map]);
                let map = model_map(e);
                if map != 0 {
                    e.vcall(map, 0, &args![1u32]);
                }
                let this = manager(e);
                e.set(this, BSTreeManager::pModelMap, Ptr::NULL);
            }
        } else {
            let map = model_map(e);
            let first = e.call(MAP_FIRST, &args![map]).u32();
            e.mem.set_u32(cursor, first);
            while e.mem.u32(cursor) != 0 {
                let map = model_map(e);
                e.call(MAP_NEXT, &args![map, cursor, key, value]);
                let array = e.mem.u32(value);
                if array != 0 {
                    e.call(MODEL_ARRAY_DELETE, &args![array, 3u32]);
                }
            }
            let map = model_map(e);
            e.call(MAP_CLEAR, &args![map]);
            let map = model_map(e);
            if map != 0 {
                e.vcall(map, 0, &args![1u32]);
            }
            let this = manager(e);
            e.set(this, BSTreeManager::pModelMap, Ptr::NULL);
        }
    });
    e.call(LOCK_LEAVE, &args![LOCK]);
}

// Translation incomplete: 00664f50 - C++ exception unwinding frame (FS:[0]) not translated
// Translated from 00664f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the tree node for the base `form` of `reference` (null for a
/// tree without a reference), under the `BSCriticalSection` guard (line
/// `0x1ce`). Fails with null when trees are disabled (`0051d740`), the form
/// is null, its model sub-object (`+0x30`) has no model path (virtual `+0x14`)
/// or fails `0048cee0`, or `reference`'s base form (`007af430`) is not
/// `form`. A tree already loaded in the background for `reference`
/// (`pBackgroundLoadedTreeMap` find, slot `+8`) is removed from that map
/// (slot `+0x14`) and returned. Otherwise the seed is `1`, or the
/// reference's (`005692d0`), the model comes from `GetModel`; a model with
/// more than two references gets a fresh instance model (`00666650`,
/// `InitAsInstance`), otherwise the shared one is initialised from the base
/// (`InitFromBase`); either failing gives null. The model's virtual `+0xc`
/// (art, given the reference's scale or `1.0`) makes the node, which is
/// named after the model path, and the model is marked initialised
/// (`006652c0(1)`); the node is returned.
pub fn fn_00664f50(e: &mut Engine, this: Ptr<BSTreeManager>, reference: u32, form: u32) -> u32 {
    with_guard(e, 0x1ce, |e| {
        e.with_stack(4, |e, model_pointer| {
            e.call(NI_POINTER_CONSTRUCT, &args![model_pointer, 0u32]);
            let node = build_tree_node(e, this, reference, form, model_pointer.addr());
            e.call(NI_POINTER_DESTRUCT, &args![model_pointer]);
            node
        })
    })
}

/// The body of `00664f50` between the guard and the `NiPointer` local.
fn build_tree_node(
    e: &mut Engine,
    this: Ptr<BSTreeManager>,
    reference: u32,
    form: u32,
    model_pointer: u32,
) -> u32 {
    if !e.call(TREES_ENABLED, &args![]).bool() || form == 0 {
        return 0;
    }
    if e.vcall(form + 0x30, 0x14, &args![]).u32() == 0 {
        return 0;
    }
    if e.call(FORM_MODEL_CHECK, &args![form + 0x30]).u32() == 0 {
        return 0;
    }
    if reference != 0 && e.call(REFERENCE_BASE_FORM, &args![reference]).u32() != form {
        return 0;
    }
    let loaded = e.with_stack(4, |e, loaded| {
        let tree_map = e.get(this, BSTreeManager::pBackgroundLoadedTreeMap).addr();
        if e.vcall(tree_map, SLOT_FIND, &args![reference, loaded])
            .bool()
        {
            let tree_map = e.get(this, BSTreeManager::pBackgroundLoadedTreeMap).addr();
            e.vcall(tree_map, SLOT_REMOVE, &args![reference]);
            Some(e.mem.u32(loaded.addr()))
        } else {
            None
        }
    });
    if let Some(node) = loaded {
        return node;
    }
    let mut seed = 1u32;
    if reference != 0 {
        seed = e.call(REFERENCE_SEED, &args![reference]).u32();
    }
    let singleton = manager(e);
    bs_tree_manager_get_model(e, singleton, form, seed, model_pointer);
    if pointer_get(e, model_pointer) == 0 {
        return 0;
    }
    let shared = pointer_get(e, model_pointer);
    let model = if e.call(REFERENCE_COUNT, &args![shared]).u32() > 2 {
        let memory = e.call(NI_ALLOCATE, &args![0x50u32]).u32();
        let instance = if memory != 0 {
            fn_00666650(e, Ptr::new(memory)).addr()
        } else {
            0
        };
        let base = pointer_get(e, model_pointer);
        if !e
            .call(MODEL_INIT_AS_INSTANCE, &args![instance, base])
            .bool()
        {
            if instance != 0 {
                e.vcall(instance, 0, &args![1u32]);
            }
            return 0;
        }
        instance
    } else {
        let base = pointer_get(e, model_pointer);
        if !e
            .call(MODEL_INIT_FROM_BASE, &args![base, form, seed])
            .bool()
        {
            return 0;
        }
        pointer_get(e, model_pointer)
    };
    let mut scale = 1.0f32;
    if reference != 0 {
        scale = e.call(REFERENCE_SCALE, &args![reference]).f32();
    }
    let node = e.vcall(model, SLOT_CREATE_ART, &args![scale]).u32();
    if node != 0 {
        let path = e.vcall(form + 0x30, 0x14, &args![]).u32();
        e.with_stack(4, |e, name| {
            let name_pointer = e.call(FIXED_STRING_CONSTRUCT, &args![name, path]).u32();
            e.call(NODE_SET_NAME, &args![node, name_pointer]);
            e.call(FIXED_STRING_DESTRUCT, &args![name]);
        });
    }
    fn_006652c0(e, Ptr::new(model), 1);
    node
}

// Translated from 006652c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at `+0x44` of `this` (a `BSTreeModel`:
/// `bInitialized`, Xbox PDB `+0x44`; `RET 4`).
pub fn fn_006652c0(e: &mut Engine, this: Ptr, value: u8) {
    // BSTreeModel::bInitialized (Xbox PDB) +0x44
    e.mem.set_u8(this.addr() + 0x44, value);
}

// Translated from 006652e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Per-frame tree update (cdecl: a scene object, a byte). Constructs the
/// local vector (`006815c0`). Without the byte, adds the frame time
/// (`0084d030`) to the time accumulator `011d5dd4` and hands it to
/// `00b07060`. With a non-null `camera`, passes its position (the three
/// floats at the pointer `0045bb80` gives) and the direction `-1 *
/// 0045bba0(camera)` to `CSpeedTreeRT::SetCamera` (Xbox PDB). Without the
/// byte, when the `TES` singleton exists and its `+0x68` word is set, the
/// wind speed becomes `0` when the interior-cell word (`+0x34`) is set,
/// otherwise the float `004536e0` reads from that `+0x68` object; and when
/// the manager has a billboard material (`spBillboardMatProp`) and the
/// object's `00450570` is set, the three colour setters are called with the
/// colour `00891170` gives. Then the manager's `006658b0` runs. In both
/// cases the tree distance static (`00664720`, from the setting at
/// `011d5d98`) and the clamped static (`00665520`, from `011d5d4c`) are set.
pub fn fn_006652e0(e: &mut Engine, camera: u32, skip_update: u8) {
    e.with_stack(0x48, |e, block| {
        let scratch_direction = block.addr();
        let scratch_scaled = block.addr() + 0x0c;
        let vector = block.addr() + 0x18;
        let direction = block.addr() + 0x24;
        let position = block.addr() + 0x30;
        let colour = block.addr() + 0x3c;
        e.call(VECTOR_CONSTRUCT, &args![vector]);
        if skip_update == 0 {
            let frame_time = e.call(FRAME_TIME, &args![FRAME_TIME_SOURCE]).f32();
            let total = (frame_time as f64 + e.global::<f32>(TIME_ACCUMULATOR) as f64) as f32;
            e.set_global(TIME_ACCUMULATOR, total);
            let accumulated = load_float(e, TIME_ACCUMULATOR);
            e.call(SPEEDTREE_SET_TIME, &args![accumulated]);
        }
        if camera != 0 {
            for lane in 0..3u32 {
                let at = e.call(OBJECT_POSITION, &args![camera]).u32();
                let bits = x87_quiet_bits(e.mem.u32(at + lane * 4));
                e.mem.set_u32(position + lane * 4, bits);
            }
            let facing = e
                .call(OBJECT_DIRECTION, &args![camera, scratch_direction])
                .u32();
            let minus_one = x87_quiet_bits(e.mem.u32(FLOAT_MINUS_ONE));
            let scaled = e
                .call(SCALED_VECTOR, &args![scratch_scaled, minus_one, facing])
                .u32();
            for lane in 0..3u32 {
                let bits = e.mem.u32(scaled + lane * 4);
                e.mem.set_u32(vector + lane * 4, bits);
            }
            for lane in 0..3u32 {
                let bits = x87_quiet_bits(e.mem.u32(vector + lane * 4));
                e.mem.set_u32(direction + lane * 4, bits);
            }
            e.call(SPEEDTREE_SET_CAMERA, &args![position, direction]);
        }
        if skip_update == 0 {
            update_from_tes(e, colour);
            let singleton = manager(e);
            fn_006658b0(e, singleton);
        }
        let distance_pointer = e
            .call(SETTING_VALUE_WORD, &args![TREE_DISTANCE_SETTING])
            .u32();
        let distance = e.mem.u32(distance_pointer);
        e.call(SET_TREE_DISTANCE, &args![distance]);
        let pointer = e.call(SETTING_VALUE, &args![SETTING_TO_CLAMP]).u32();
        let value = load_float(e, pointer);
        fn_00665520(e, value);
    });
}

/// The `TES`-dependent part of `006652e0`: the wind speed and the billboard
/// material colours. `colour` is the local the colour is copied to.
fn update_from_tes(e: &mut Engine, colour: u32) {
    let tes = e.global::<u32>(TES_SINGLETON);
    if tes == 0 {
        return;
    }
    if e.call(TES_FIELD_68, &args![tes]).u32() == 0 {
        return;
    }
    if e.call(TES_INTERIOR_CELL, &args![tes]).u32() != 0 {
        let this = manager(e);
        e.set(this, BSTreeManager::fWindSpeed, 0.0);
    } else {
        let object = e.call(TES_FIELD_68, &args![tes]).u32();
        let speed = e.call(FLOAT_OF_OBJECT, &args![object]).f32();
        let this = manager(e);
        e.set(this, BSTreeManager::fWindSpeed, speed);
    }
    let this = manager(e);
    let material_slot = this.addr() + BSTreeManager::spBillboardMatProp.off;
    if pointer_get(e, material_slot) == 0 {
        return;
    }
    let object = e.call(TES_FIELD_68, &args![tes]).u32();
    if e.call(OBJECT_OF_OBJECT, &args![object]).u32() == 0 {
        return;
    }
    let object = e.call(TES_FIELD_68, &args![tes]).u32();
    let inner = e.call(OBJECT_OF_OBJECT, &args![object]).u32();
    let source = e.call(COLOR_POINTER, &args![inner]).u32();
    for lane in 0..3u32 {
        let bits = e.mem.u32(source + lane * 4);
        e.mem.set_u32(colour + lane * 4, bits);
    }
    for setter in [COLOR_SETTER_NOOP, COLOR_SETTER_NOOP, COLOR_SETTER] {
        let this = manager(e);
        let material = pointer_get(e, this.addr() + BSTreeManager::spBillboardMatProp.off);
        e.call(setter, &args![material, colour]);
    }
}

// Translated from 00665520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` clamped to `[0, 1]` into the float static `011adce0`
/// (cdecl, one float): below `0.0` or NaN gives `0.0`, above `1.0` gives
/// `1.0`, anything else is kept (the `0.0` and `1.0` come from the
/// `double` constants at `01012060` and `01012070`).
pub fn fn_00665520(e: &mut Engine, value: f32) {
    let zero = e.global::<f64>(DOUBLE_ZERO);
    let one = e.global::<f64>(DOUBLE_ONE);
    let wide = value as f64;
    let clamped = if wide >= zero {
        if wide > one {
            1.0
        } else {
            value
        }
    } else {
        0.0
    };
    e.set_global(CLAMPED_STATIC, clamped);
}

// Translated from 00665570 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float setting `006655a0` returns times the float setting at
/// `011f9158` (result in `ST0`, rounded to `float`).
pub fn fn_00665570(e: &mut Engine) -> f32 {
    let first = fn_006655a0(e);
    let second = setting_float(e, SETTING_MULTIPLIER);
    (second as f64 * first as f64) as f32
}

// Translated from 006655a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float setting object at `011d5d64`'s value (result in `ST0`).
pub fn fn_006655a0(e: &mut Engine) -> f32 {
    setting_float(e, SETTING_A)
}

// Translated from 006655c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float setting `006655f0` returns times the float setting at
/// `011f9158` (result in `ST0`, rounded to `float`).
pub fn fn_006655c0(e: &mut Engine) -> f32 {
    let first = fn_006655f0(e);
    let second = setting_float(e, SETTING_MULTIPLIER);
    (second as f64 * first as f64) as f32
}

// Translated from 006655f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float setting object at `011d5d74`'s value (result in `ST0`).
pub fn fn_006655f0(e: &mut Engine) -> f32 {
    setting_float(e, SETTING_B)
}

// Translation incomplete: 00665610 - C++ exception unwinding frame (FS:[0]) not translated
// Translated from 00665610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeManager::GetCanopyShadow` (Xbox PDB): the canopy shadow texture
/// (`spCanopyShadow`), under the `BSCriticalSection` guard (line `0x275`).
/// When it is not loaded: with a `TES` singleton the texture image
/// `Data\Textures\Trees\CanopyShadow.dds` is created into it
/// (`TES::CreateTextureImage`, `1`, `0`), without one it is set null, and
/// the global mirror (`00664820`) is updated. When it is still null a 16 by
/// 16 white `NiPixelData` (3 bytes per pixel, `0x74`-byte object) becomes a
/// `NiSourceTexture` named `CanopyShadowTexture`, and the global mirror is
/// updated again. Returns the texture.
pub fn bs_tree_manager_get_canopy_shadow(e: &mut Engine) -> u32 {
    with_guard(e, 0x275, |e| {
        let this = manager(e);
        if pointer_get(e, this.addr() + BSTreeManager::spCanopyShadow.off) == 0 {
            let tes = e.global::<u32>(TES_SINGLETON);
            if tes != 0 {
                let this = manager(e);
                e.call(
                    CREATE_TEXTURE_IMAGE,
                    &args![
                        tes,
                        CANOPY_SHADOW_PATH,
                        this.addr() + BSTreeManager::spCanopyShadow.off,
                        1u32,
                        0u32
                    ],
                );
            } else {
                let this = manager(e);
                e.call(
                    NI_POINTER_ASSIGN,
                    &args![this.addr() + BSTreeManager::spCanopyShadow.off, 0u32],
                );
            }
            let this = manager(e);
            let texture = pointer_get(e, this.addr() + BSTreeManager::spCanopyShadow.off);
            e.call(CANOPY_SHADOW_GLOBAL_ASSIGN, &args![texture]);
        }
        let this = manager(e);
        if pointer_get(e, this.addr() + BSTreeManager::spCanopyShadow.off) == 0 {
            let memory = e.call(NI_ALLOCATE, &args![0x74u32]).u32();
            let pixels = if memory != 0 {
                e.call(
                    PIXEL_DATA_CONSTRUCT,
                    &args![memory, 0x10u32, 0x10u32, PIXEL_FORMAT, 1u32, 1u32],
                )
                .u32()
            } else {
                0
            };
            let buffer = e.call(PIXEL_DATA_PIXELS, &args![pixels, 0u32, 0u32]).u32();
            for row in 0..0x10u32 {
                for column in 0..0x10u32 {
                    let at = buffer + (row * 0x10 + column) * 3;
                    e.mem.set_u8(at, 0xff);
                    e.mem.set_u8(at + 1, 0xff);
                    e.mem.set_u8(at + 2, 0xff);
                }
            }
            e.call(PIXEL_DATA_FINISH, &args![pixels]);
            e.with_stack(4, |e, name| {
                let name_pointer = e
                    .call(FIXED_STRING_CONSTRUCT, &args![name, CANOPY_TEXTURE_NAME])
                    .u32();
                let c_string = e.call(FIXED_STRING_C_STR, &args![name_pointer]).u32();
                let texture = e
                    .call(
                        SOURCE_TEXTURE_CREATE,
                        &args![pixels, c_string, TEXTURE_PREFERENCES],
                    )
                    .u32();
                let this = manager(e);
                e.call(
                    NI_POINTER_ASSIGN,
                    &args![this.addr() + BSTreeManager::spCanopyShadow.off, texture],
                );
                e.call(FIXED_STRING_DESTRUCT, &args![name]);
            });
            let this = manager(e);
            let texture = pointer_get(e, this.addr() + BSTreeManager::spCanopyShadow.off);
            e.call(CANOPY_SHADOW_GLOBAL_ASSIGN, &args![texture]);
        }
        let this = manager(e);
        pointer_get(e, this.addr() + BSTreeManager::spCanopyShadow.off)
    })
}

// Translated from 00665860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeManager::ClearCanopyShadow` (Xbox PDB): when the canopy shadow
/// texture is set, clears it (and the global mirror, `00664820(0)`) and
/// removes the texture image `Data\Textures\Trees\CanopyShadow.dds` from
/// the `TES` singleton (`TES::RemoveTextureImage`, `0`).
pub fn bs_tree_manager_clear_canopy_shadow(e: &mut Engine) {
    let this = manager(e);
    if pointer_get(e, this.addr() + BSTreeManager::spCanopyShadow.off) != 0 {
        let this = manager(e);
        e.call(
            NI_POINTER_ASSIGN,
            &args![this.addr() + BSTreeManager::spCanopyShadow.off, 0u32],
        );
        e.call(CANOPY_SHADOW_GLOBAL_ASSIGN, &args![0u32]);
        let tes = e.global::<u32>(TES_SINGLETON);
        e.call(REMOVE_TEXTURE_IMAGE, &args![tes, CANOPY_SHADOW_PATH, 0u32]);
    }
}

// Translated from 006658b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Wind update of the manager (`this` = the manager; `fWindSpeed` at
/// `+0x14`). On its first run the static `011d5de8` takes the wind speed.
/// For each of four phases it advances the accumulator `011d5dd8[i]` by the
/// frame time (`0084d030`), rescales it by the previous wind speed over the
/// current one when the current one is not `0.0` (NaN included), and gives
/// `00bb2fc0(i, ..)` the sway terms `0063c600`/`0057e960` of `20 * wind *
/// table * phase`, scaled by `0.61 * wind`; phase `2` also sums the
/// `00408840` of both terms. Four float settings then become lighting
/// values (`1 - s + s * sum / 2`) handed with the wind speed and the
/// frame-time products of two more settings to `00bb1390`; the previous
/// wind speed is then stored.
pub fn fn_006658b0(e: &mut Engine, this: Ptr<BSTreeManager>) {
    let wind_bits = e.mem.u32(this.addr() + BSTreeManager::fWindSpeed.off);
    if e.global::<u32>(LAST_WIND_SPEED_READY) & 1 == 0 {
        let flags = e.global::<u32>(LAST_WIND_SPEED_READY);
        e.set_global(LAST_WIND_SPEED_READY, flags | 1);
        e.set_global(LAST_WIND_SPEED, x87_quiet_bits(wind_bits));
    }
    let wind = load_float(e, this.addr() + BSTreeManager::fWindSpeed.off);
    let zero = e.global::<f64>(DOUBLE_ZERO);
    let scale = (wind as f64 * e.global::<f64>(DOUBLE_WIND_SCALE)) as f32;
    let mut sum = 0.0f32;
    for phase in 0..4u32 {
        let frame_time = e.call(FRAME_TIME, &args![FRAME_TIME_SOURCE]).f32();
        let phase_address = PHASES + phase * 4;
        let advanced = (frame_time as f64 + e.global::<f32>(phase_address) as f64) as f32;
        e.set_global(phase_address, advanced);
        if wind as f64 != zero {
            let current = e.global::<f32>(phase_address) as f64;
            let last = e.global::<f32>(LAST_WIND_SPEED) as f64;
            e.set_global(phase_address, (current * last / wind as f64) as f32);
        }
        let amplitude = (wind as f64 * e.global::<f64>(DOUBLE_TWENTY)) as f32;
        let first_product = ((amplitude as f64 * e.global::<f32>(SWAY_TABLE + phase * 8) as f64)
            * e.global::<f32>(phase_address) as f64) as f32;
        let first = e.call(SWAY_FIRST, &args![first_product]).f32();
        let second_product = ((amplitude as f64
            * e.global::<f32>(SWAY_TABLE + phase * 8 + 4) as f64)
            * e.global::<f32>(phase_address) as f64) as f32;
        let second = e.call(SWAY_SECOND, &args![second_product]).f32();
        if phase == 2 {
            let term = e.call(SWAY_TERM, &args![first]).f32();
            sum = (term as f64 + sum as f64) as f32;
            let term = e.call(SWAY_TERM, &args![second]).f32();
            sum = (term as f64 + sum as f64) as f32;
        }
        let scaled_second = (scale as f64 * second as f64) as f32;
        let scaled_first = (scale as f64 * first as f64) as f32;
        e.call(
            SPEEDTREE_SET_WIND_DIRECTION,
            &args![phase, scaled_first, scaled_second],
        );
    }
    let two = e.global::<f64>(DOUBLE_TWO);
    let mut lighting = [0.0f32; 4];
    for (value, setting) in
        lighting
            .iter_mut()
            .zip([0x011d_5dc4u32, 0x011d_5d3c, 0x011d_5c68, 0x011d_5c74])
    {
        let first_read = setting_float(e, setting);
        let one_minus = 1.0f64 - first_read as f64;
        let second_read = setting_float(e, setting);
        *value = ((second_read as f64 * sum as f64) / two + one_minus) as f32;
    }
    let frame_time = e.call(FRAME_TIME, &args![FRAME_TIME_SOURCE]).f32() as f64;
    let factor = setting_float(e, 0x011d_5c5c);
    let first_light = ((factor as f64 * frame_time) * lighting[3] as f64) as f32;
    let frame_time = e.call(FRAME_TIME, &args![FRAME_TIME_SOURCE]).f32() as f64;
    let factor = setting_float(e, 0x011d_5cdc);
    let second_light = ((factor as f64 * frame_time) * lighting[2] as f64) as f32;
    let third = (wind as f64 * lighting[1] as f64) as f32;
    let fourth = (wind as f64 * lighting[0] as f64) as f32;
    e.call(
        SPEEDTREE_SET_LIGHTING,
        &args![fourth, third, second_light, first_light],
    );
    let bits = x87_quiet_bits(e.mem.u32(this.addr() + BSTreeManager::fWindSpeed.off));
    e.set_global(LAST_WIND_SPEED, bits);
}

// Translated from 00665b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Offers `tree` to the background-loaded tree map for `key` (virtual
/// `+0x10` of `pBackgroundLoadedTreeMap`: key, the address of the tree word,
/// `0`; `RET 8`). When the map does not take it, the tree is destroyed
/// through its virtual slot `0` with `1`.
pub fn fn_00665b80(e: &mut Engine, this: Ptr<BSTreeManager>, key: u32, tree: u32) {
    e.with_stack(4, |e, tree_slot| {
        e.mem.set_u32(tree_slot.addr(), tree);
        let tree_map = e.get(this, BSTreeManager::pBackgroundLoadedTreeMap).addr();
        let taken = e
            .vcall(tree_map, SLOT_INSERT, &args![key, tree_slot, 0u32])
            .bool();
        if !taken {
            let tree = e.mem.u32(tree_slot.addr());
            if tree != 0 {
                e.vcall(tree, 0, &args![1u32]);
            }
        }
    });
}

// Translated from 00665be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeManager::RemoveBackgroundLoadedTree` (Xbox PDB): finds `key` in
/// `pBackgroundLoadedTreeMap` (virtual `+8`), removes it (virtual `+0x14`)
/// and releases the tree found (`00c45830`, cdecl) when it is not null.
pub fn bs_tree_manager_remove_background_loaded_tree(
    e: &mut Engine,
    this: Ptr<BSTreeManager>,
    key: u32,
) {
    e.with_stack(4, |e, found| {
        let tree_map = e.get(this, BSTreeManager::pBackgroundLoadedTreeMap).addr();
        if e.vcall(tree_map, SLOT_FIND, &args![key, found]).bool() {
            let tree_map = e.get(this, BSTreeManager::pBackgroundLoadedTreeMap).addr();
            e.vcall(tree_map, SLOT_REMOVE, &args![key]);
            let tree = e.mem.u32(found.addr());
            if tree != 0 {
                e.call(RELEASE_TREE, &args![tree]);
            }
        }
    });
}

// Translated from 00665c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the model-array map (`NiTPointerMap<TESObjectTREE *,
/// NiPointer<BSTreeModel> *>`, RTTI of vtable `01066434`): builds the base
/// with `hash_size` buckets (`00665f50`), then sets the map's vtable.
pub fn fn_00665c50(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    fn_00665f50(e, this, hash_size);
    e.mem.set_u32(this.addr(), MAP_VTABLE);
    this
}

// Translated from 00665c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<TESObjectTREE *, NiPointer<BSTreeModel> *>::
/// _scalar_deleting_destructor_` (Xbox PDB): runs the destructor
/// (`00665fc0`) and, with bit 0 of `flags`, frees the object. Returns `this`.
pub fn ni_t_pointer_map_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    fn_00665fc0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00665f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the map base (`NiTMapBase<NiTPointerAllocator<unsigned
/// int>, ..>`, RTTI of vtable `010664c4`): stores the vtable, the bucket
/// count `hash_size`, a zero item count, and a zeroed table of `hash_size`
/// pointers (`00aa1070` then `memset`).
pub fn fn_00665f50(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), MAP_BASE_VTABLE);
    e.set(this, NiTPointerMap::m_uiHashSize, hash_size);
    e.set(this, NiTPointerMap::m_uiCount, 0);
    let table = e.call(ALLOCATE_BYTES, &args![hash_size << 2]).u32();
    e.set(this, NiTPointerMap::m_ppkHashTable, table);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let size = e.get(this, NiTPointerMap::m_uiHashSize) << 2;
    e.call(MEMSET, &args![table, 0u32, size]);
    this
}

// Translation incomplete: 00665fc0 - C++ exception unwinding frame (FS:[0]) not translated
// Translated from 00665fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the model-array map: sets the map vtable, clears the map
/// (`00438af0`), then runs the base destructor (`00666020`).
pub fn fn_00665fc0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), MAP_VTABLE);
    e.call(MAP_CLEAR, &args![this]);
    e.call(MAP_BASE_DESTRUCT, &args![this]);
}

// Translated from 00666270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, TESObjectTREE *,
/// NiPointer<BSTreeModel> *>::_scalar_deleting_destructor_` (Xbox PDB): runs
/// the base destructor (`00666020`) and, with bit 0 of `flags`, frees the
/// object. Returns `this`.
pub fn ni_t_map_base_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    flags: u32,
) -> Ptr<NiTPointerMap> {
    e.call(MAP_BASE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translation incomplete: 00666650 - C++ exception unwinding frame (FS:[0]) not translated
// Translated from 00666650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreeModel` constructor (vtable `01066768`, RTTI `BSTreeModel`; 0x50
/// bytes): the reference-counted base (`004968b0`), the vtable, the
/// `NiPointer` members built empty (`00633c90(0)`) at `+0x10`, `+0x1c`,
/// `+0x20`, `+0x2c`, `+0x30`, `+0x34`, `+0x38`, then every field set to its
/// empty value: `eType` (`+8`) and `pSpeedTree` (`+0xc`) `0`, the branch and
/// leaf data pointers (`+0x14`, `+0x18`) `0`, the `NiPointer`s at `+0x1c`,
/// `+0x20`, `+0x2c`, `+0x30` and `+0x34` assigned null, the shader property
/// arrays (`+0x24`, `+0x28`) `0`, `fCurveScalar` (`+0x3c`) `0.0`, `iSeed`
/// (`+0x40`) `1`, `bInitialized` (`+0x44`) false, `fTrunkHeight` and
/// `fTrunkWidth` (`+0x48`, `+0x4c`) `0.0`. Field names are the Xbox PDB's.
pub fn fn_00666650(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_REF_OBJECT_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, MODEL_VTABLE);
    for field in [0x10u32, 0x1c, 0x20, 0x2c, 0x30, 0x34, 0x38] {
        e.call(NI_POINTER_CONSTRUCT, &args![base + field, 0u32]);
    }
    e.mem.set_u32(base + 0x0c, 0);
    e.mem.set_u32(base + 0x08, 0);
    e.mem.set_u32(base + 0x14, 0);
    e.mem.set_u32(base + 0x18, 0);
    e.call(NI_POINTER_ASSIGN, &args![base + 0x1c, 0u32]);
    e.call(NI_POINTER_ASSIGN, &args![base + 0x20, 0u32]);
    e.mem.set_u32(base + 0x24, 0);
    e.mem.set_u32(base + 0x28, 0);
    e.call(NI_POINTER_ASSIGN, &args![base + 0x2c, 0u32]);
    e.call(NI_POINTER_ASSIGN, &args![base + 0x30, 0u32]);
    e.call(NI_POINTER_ASSIGN, &args![base + 0x34, 0u32]);
    e.mem.set_f32(base + 0x3c, 0.0);
    e.mem.set_u32(base + 0x40, 1);
    e.mem.set_u8(base + 0x44, 0);
    e.mem.set_f32(base + 0x48, 0.0);
    e.mem.set_f32(base + 0x4c, 0.0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006649c0,
            bs_tree_manager_get_model_array(Ptr<BSTreeManager>, u32) -> u32
        ),
        entry!(
            0x00664b50,
            bs_tree_manager_get_model(Ptr<BSTreeManager>, u32, u32, u32)
        ),
        entry!(0x00664cd0, fn_00664cd0(u8)),
        entry!(0x00664f50, fn_00664f50(Ptr<BSTreeManager>, u32, u32) -> u32),
        entry!(0x006652c0, fn_006652c0(Ptr, u8)),
        entry!(0x006652e0, fn_006652e0(u32, u8)),
        entry!(0x00665520, fn_00665520(f32)),
        entry!(0x00665570, fn_00665570() -> f32),
        entry!(0x006655a0, fn_006655a0() -> f32),
        entry!(0x006655c0, fn_006655c0() -> f32),
        entry!(0x006655f0, fn_006655f0() -> f32),
        entry!(0x00665610, bs_tree_manager_get_canopy_shadow() -> u32),
        entry!(0x00665860, bs_tree_manager_clear_canopy_shadow()),
        entry!(0x006658b0, fn_006658b0(Ptr<BSTreeManager>)),
        entry!(0x00665b80, fn_00665b80(Ptr<BSTreeManager>, u32, u32)),
        entry!(
            0x00665be0,
            bs_tree_manager_remove_background_loaded_tree(Ptr<BSTreeManager>, u32)
        ),
        entry!(
            0x00665c50,
            fn_00665c50(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00665c80,
            ni_t_pointer_map_scalar_deleting_destructor(
                Ptr<NiTPointerMap>,
                u32,
            ) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x00665f50,
            fn_00665f50(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00665fc0, fn_00665fc0(Ptr<NiTPointerMap>)),
        entry!(
            0x00666270,
            ni_t_map_base_scalar_deleting_destructor(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x00666650, fn_00666650(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn float_ret(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// Registers a double that records its call and then runs `behaviour`.
    fn logged(e: &mut Engine, log: &Log, addr: u32, behaviour: fn(&mut Engine, &[u32]) -> Ret) {
        let log = log.clone();
        e.register_double(addr, move |e, words| {
            log.borrow_mut().push((addr, words.to_vec()));
            behaviour(e, words)
        });
    }
    /// Like `logged`, with a behaviour that keeps captured state.
    fn logged_with(
        e: &mut Engine,
        log: &Log,
        addr: u32,
        mut behaviour: impl FnMut(&mut Engine, &[u32]) -> Ret + 'static,
    ) {
        let log = log.clone();
        e.register_double(addr, move |e, words| {
            log.borrow_mut().push((addr, words.to_vec()));
            behaviour(e, words)
        });
    }

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An engine whose `NiPointer` helpers, guard, lock and allocators have
    /// their real behaviour (and are logged).
    fn base_engine() -> (Engine, Log) {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for addr in [
            GUARD_CONSTRUCT,
            GUARD_DESTRUCT,
            LOCK_ENTER,
            LOCK_LEAVE,
            NI_POINTER_DESTRUCT,
        ] {
            logged(&mut e, &log, addr, |_, _| Ret::default());
        }
        let store: fn(&mut Engine, &[u32]) -> Ret = |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        };
        logged(&mut e, &log, NI_POINTER_CONSTRUCT, store);
        logged(&mut e, &log, NI_POINTER_ASSIGN, store);
        logged(&mut e, &log, NI_POINTER_COPY, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(NI_POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        for addr in [OPERATOR_NEW, NI_ALLOCATE, ALLOCATE_BYTES] {
            logged(&mut e, &log, addr, |e, a| ret(e.mem.alloc(a[0])));
        }
        logged(&mut e, &log, NI_REF_OBJECT_CONSTRUCT, |_, _| Ret::default());
        (e, log)
    }

    /// A manager object that the singleton accessor returns.
    fn manager_in(e: &mut Engine) -> Ptr<BSTreeManager> {
        let manager = e.new_object::<BSTreeManager>();
        let address = manager.addr();
        e.register_double(SINGLETON, move |_, _| ret(address));
        manager
    }

    /// Maps the constants and statics the float code reads.
    fn map_floats(e: &mut Engine) {
        e.map(0x0101_2000, 0x100);
        e.set_global(DOUBLE_ONE, 1.0f64);
        e.set_global(FLOAT_MINUS_ONE, -1.0f32);
        e.map(0x0101_1500, 0x100);
        e.set_global(DOUBLE_TWO, 2.0f64);
        e.map(0x0102_fc00, 0x100);
        e.set_global(DOUBLE_TWENTY, 20.0f64);
        e.map(0x0106_6400, 0x40);
        e.set_global(DOUBLE_WIND_SCALE, 0.61f32 as f64);
        e.map(0x011d_5c00, 0x400);
        e.map(0x011a_dce0, 8);
        e.map(0x0119_b8c0, 0x40);
        e.map(0x011f_9100, 0x100);
        e.register(SETTING_VALUE, |_, a| ret(a[0] + 4));
    }

    #[test]
    fn model_array_for_a_null_form_is_null() {
        let (mut e, log) = base_engine();
        let this = manager_in(&mut e);
        assert_eq!(e.call(0x0066_49c0, &args![this, 0u32]).u32(), 0);
        let construct = calls_to(&log, GUARD_CONSTRUCT);
        assert_eq!(construct.len(), 1);
        assert_eq!(construct[0][1..], [0x1e, 1, SOURCE_PATH_STRING, 0xd8]);
        assert_eq!(calls_to(&log, GUARD_DESTRUCT), vec![vec![construct[0][0]]]);
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn model_array_found_in_the_map_is_returned() {
        let (mut e, log) = base_engine();
        let this = manager_in(&mut e);
        let map = e.mem.alloc(0x10);
        e.set(this, BSTreeManager::pModelMap, Ptr::new(map));
        logged(&mut e, &log, MAP_FIND, |e, a| {
            e.mem.set_u32(a[2], 0x7777);
            ret(1)
        });
        logged(&mut e, &log, MAP_SET_AT, |_, _| Ret::default());
        assert_eq!(e.call(0x0066_49c0, &args![this, 0x4444u32]).u32(), 0x7777);
        let find = calls_to(&log, MAP_FIND);
        assert_eq!(find.len(), 1);
        assert_eq!(find[0][..2], [map, 0x4444]);
        assert!(calls_to(&log, MAP_SET_AT).is_empty());
    }

    #[test]
    fn model_array_is_created_with_the_map_when_the_form_is_new() {
        let (mut e, log) = base_engine();
        let this = manager_in(&mut e);
        logged(&mut e, &log, MAP_FIND, |_, _| ret(0));
        logged(&mut e, &log, MAP_SET_AT, |_, _| Ret::default());
        logged(&mut e, &log, VECTOR_CONSTRUCTOR_ITERATOR, |_, _| {
            Ret::default()
        });
        logged(&mut e, &log, MEMSET, |_, _| Ret::default());
        let array = e.call(0x0066_49c0, &args![this, 0x4444u32]).u32();
        // The map was built with 0x25 buckets and stored.
        let map = e.get(this, BSTreeManager::pModelMap).addr();
        assert_ne!(map, 0);
        assert_eq!(e.mem.u32(map), MAP_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 0x25);
        assert_eq!(calls_to(&log, OPERATOR_NEW)[0], vec![0x10]);
        assert_eq!(calls_to(&log, ALLOCATE_BYTES), vec![vec![0x94]]);
        // The array is the counted block's element area.
        assert_eq!(calls_to(&log, OPERATOR_NEW)[1], vec![8]);
        assert_eq!(e.mem.u32(array - 4), 1);
        assert_eq!(
            calls_to(&log, VECTOR_CONSTRUCTOR_ITERATOR),
            vec![vec![array, 4, 1, MODEL_SLOT_CONSTRUCT, NI_POINTER_DESTRUCT]]
        );
        assert_eq!(calls_to(&log, MAP_SET_AT), vec![vec![map, 0x4444, array]]);
    }

    /// A model array with one slot holding a model of the given seed.
    fn array_with_model(e: &mut Engine, seed: u32) -> (u32, u32) {
        let model = e.mem.alloc(0x50);
        e.mem.set_u32(model + 0x40, seed);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, model);
        (array, model)
    }

    /// Doubles for `GetModel`: the map finds `array`, models report their seed.
    fn model_lookup(e: &mut Engine, this: Ptr<BSTreeManager>, array: u32) {
        let map = e.mem.alloc(0x10);
        e.set(this, BSTreeManager::pModelMap, Ptr::new(map));
        e.register_double(MAP_FIND, move |e, a| {
            e.mem.set_u32(a[2], array);
            ret(1)
        });
        e.register(MODEL_SEED, |e, a| ret(e.mem.u32(a[0] + 0x40)));
    }

    #[test]
    fn get_model_returns_the_model_with_the_seed() {
        let (mut e, log) = base_engine();
        let this = manager_in(&mut e);
        let (array, model) = array_with_model(&mut e, 5);
        model_lookup(&mut e, this, array);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0x1234);
        e.call(0x0066_4b50, &args![this, 0x4444u32, 5u32, out]);
        assert_eq!(e.mem.u32(out), model);
        assert_eq!(calls_to(&log, LOCK_ENTER), vec![vec![LOCK, 0]]);
        assert_eq!(calls_to(&log, LOCK_LEAVE), vec![vec![LOCK]]);
        assert!(calls_to(&log, NI_ALLOCATE).is_empty());
    }

    #[test]
    fn get_model_builds_a_model_in_an_empty_slot() {
        let (mut e, log) = base_engine();
        let this = manager_in(&mut e);
        let array = e.mem.alloc(8);
        model_lookup(&mut e, this, array);
        let out = e.mem.alloc(4);
        e.call(0x0066_4b50, &args![this, 0x4444u32, 5u32, out]);
        let model = e.mem.u32(array);
        assert_ne!(model, 0);
        assert_eq!(e.mem.u32(out), model);
        assert_eq!(calls_to(&log, NI_ALLOCATE), vec![vec![0x50]]);
        assert_eq!(e.mem.u32(model), MODEL_VTABLE);
        assert_eq!(e.mem.u32(model + 0x40), 1);
    }

    #[test]
    fn get_model_with_another_seed_and_no_empty_slot_gives_the_first_model() {
        let (mut e, log) = base_engine();
        let this = manager_in(&mut e);
        let (array, model) = array_with_model(&mut e, 9);
        model_lookup(&mut e, this, array);
        let out = e.mem.alloc(4);
        e.call(0x0066_4b50, &args![this, 0x4444u32, 5u32, out]);
        assert_eq!(e.mem.u32(out), model);
        assert_eq!(e.mem.u32(array), model);
        assert!(calls_to(&log, NI_ALLOCATE).is_empty());
    }

    #[test]
    fn get_model_for_a_null_form_stores_null() {
        let (mut e, log) = base_engine();
        let this = manager_in(&mut e);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0x1234);
        e.call(0x0066_4b50, &args![this, 0u32, 5u32, out]);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(calls_to(&log, LOCK_ENTER).len(), 1);
        assert_eq!(calls_to(&log, LOCK_LEAVE).len(), 1);
    }

    /// Doubles for the model map walk: `items` are the (key, array) pairs.
    fn walk_doubles(e: &mut Engine, items: Vec<(u32, u32)>) -> Items {
        let items = Rc::new(RefCell::new(items));
        let position = Rc::new(Cell::new(0usize));
        let (first_items, first_position) = (items.clone(), position.clone());
        e.register_double(MAP_FIRST, move |_, _| {
            first_position.set(0);
            ret(!first_items.borrow().is_empty() as u32)
        });
        let (next_items, next_position) = (items.clone(), position);
        e.register_double(MAP_NEXT, move |e, a| {
            let list = next_items.borrow();
            let (key, value) = list[next_position.get()];
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], value);
            next_position.set(next_position.get() + 1);
            e.mem
                .set_u32(a[1], (next_position.get() < list.len()) as u32);
            Ret::default()
        });
        let remove_items = items.clone();
        e.register_double(MAP_REMOVE_AT, move |_, a| {
            remove_items.borrow_mut().retain(|(key, _)| *key != a[1]);
            Ret::default()
        });
        let empty_items = items.clone();
        e.register_double(MAP_IS_EMPTY, move |_, _| {
            ret(empty_items.borrow().is_empty() as u32)
        });
        items
    }

    type Items = Rc<RefCell<Vec<(u32, u32)>>>;
    type CacheEnv = (Engine, Log, Ptr<BSTreeManager>, u32, Items);

    const MAP_TEST_VTABLE: u32 = 0x7400_0000;
    const MAP_DESTROY_DOUBLE: u32 = 0x7000_0700;

    /// A manager with a model map (a vtable whose slot 0 is a double) and the
    /// walk doubles over `items`.
    fn cache_env(items: Vec<(u32, u32)>) -> CacheEnv {
        let (mut e, log) = base_engine();
        let this = manager_in(&mut e);
        let map = e.mem.alloc(0x10);
        e.put_vtable(MAP_TEST_VTABLE, &[MAP_DESTROY_DOUBLE]);
        e.mem.set_u32(map, MAP_TEST_VTABLE);
        e.set(this, BSTreeManager::pModelMap, Ptr::new(map));
        logged(&mut e, &log, MAP_DESTROY_DOUBLE, |_, _| Ret::default());
        logged(&mut e, &log, MODEL_ARRAY_DELETE, |_, _| Ret::default());
        logged(&mut e, &log, MAP_CLEAR, |_, _| Ret::default());
        e.register(REFERENCE_COUNT, |e, a| ret(e.mem.u32(a[0] + 4)));
        let items = walk_doubles(&mut e, items);
        (e, log, this, map, items)
    }

    /// An array with one slot holding a model with `references` references.
    fn array_with_references(e: &mut Engine, references: u32) -> (u32, u32) {
        let (array, model) = array_with_model(e, 0);
        e.mem.set_u32(model + 4, references);
        (array, model)
    }

    #[test]
    fn clearing_without_a_model_map_does_nothing() {
        let (mut e, log, this, _, _) = cache_env(vec![]);
        e.set(this, BSTreeManager::pModelMap, Ptr::NULL);
        e.call(0x0066_4cd0, &args![1u32]);
        assert!(calls_to(&log, LOCK_ENTER).is_empty());
        assert!(calls_to(&log, MODEL_ARRAY_DELETE).is_empty());
    }

    #[test]
    fn clearing_everything_deletes_the_arrays_and_the_map() {
        let (mut e, log, this, map, _) = cache_env(vec![]);
        let (first, _) = array_with_references(&mut e, 7);
        let (second, _) = array_with_references(&mut e, 7);
        walk_doubles(&mut e, vec![(1, first), (2, second)]);
        e.call(0x0066_4cd0, &args![0u32]);
        assert_eq!(
            calls_to(&log, MODEL_ARRAY_DELETE),
            vec![vec![first, 3], vec![second, 3]]
        );
        assert_eq!(calls_to(&log, MAP_CLEAR), vec![vec![map]]);
        assert_eq!(calls_to(&log, MAP_DESTROY_DOUBLE), vec![vec![map, 1]]);
        assert_eq!(e.get(this, BSTreeManager::pModelMap), Ptr::NULL);
        assert_eq!(calls_to(&log, LOCK_ENTER), vec![vec![LOCK, 0]]);
        assert_eq!(calls_to(&log, LOCK_LEAVE), vec![vec![LOCK]]);
    }

    #[test]
    fn clearing_unreferenced_frees_models_and_removes_empty_arrays() {
        let (mut e, log, this, map, _) = cache_env(vec![]);
        let (free_array, _) = array_with_references(&mut e, 1);
        let (busy_array, busy_model) = array_with_references(&mut e, 3);
        let items = walk_doubles(&mut e, vec![(1, free_array), (2, busy_array)]);
        e.call(0x0066_4cd0, &args![1u32]);
        // The free array lost its model, was deleted and removed.
        assert_eq!(calls_to(&log, NI_POINTER_ASSIGN), vec![vec![free_array, 0]]);
        assert_eq!(
            calls_to(&log, MODEL_ARRAY_DELETE),
            vec![vec![free_array, 3]]
        );
        assert_eq!(*items.borrow(), vec![(2, busy_array)]);
        // The busy array keeps its model and the map stays.
        assert_eq!(e.mem.u32(busy_array), busy_model);
        assert_eq!(e.get(this, BSTreeManager::pModelMap).addr(), map);
        assert!(calls_to(&log, MAP_DESTROY_DOUBLE).is_empty());
    }

    #[test]
    fn clearing_unreferenced_destroys_a_map_that_ends_empty() {
        let (mut e, log, this, map, _) = cache_env(vec![]);
        let (array, _) = array_with_references(&mut e, 0);
        walk_doubles(&mut e, vec![(1, array)]);
        e.call(0x0066_4cd0, &args![1u32]);
        assert_eq!(calls_to(&log, MAP_CLEAR), vec![vec![map]]);
        assert_eq!(calls_to(&log, MAP_DESTROY_DOUBLE), vec![vec![map, 1]]);
        assert_eq!(e.get(this, BSTreeManager::pModelMap), Ptr::NULL);
    }

    const FORM_VTABLE: u32 = 0x7200_0000;
    const TREE_MAP_VTABLE: u32 = 0x7300_0000;
    const PATH_DOUBLE: u32 = 0x7000_0100;
    const FIND_DOUBLE: u32 = 0x7000_0200;
    const REMOVE_DOUBLE: u32 = 0x7000_0300;
    const CREATE_ART_DOUBLE: u32 = 0x7000_0400;
    const DESTROY_DOUBLE: u32 = 0x7000_0500;
    const INSERT_DOUBLE: u32 = 0x7000_0600;
    const NODE: u32 = 0x6000_0001;
    const PATH: u32 = 0x6100_0000;

    struct Tree {
        e: Engine,
        log: Log,
        this: Ptr<BSTreeManager>,
        form: u32,
        reference: u32,
        array: u32,
        model: u32,
        tree_map: u32,
        background: Rc<Cell<u32>>,
        init_from_base: Rc<Cell<u32>>,
        init_as_instance: Rc<Cell<u32>>,
    }

    /// Everything `00664f50` needs, with a model of `references` references
    /// in the form's model array (seed 5, the reference's seed).
    fn tree_fixture(references: u32) -> Tree {
        let (mut e, log) = base_engine();
        let this = manager_in(&mut e);
        let form = e.mem.alloc(0x40);
        e.mem.set_u32(form + 0x30, FORM_VTABLE);
        e.put_vtable(FORM_VTABLE, &[0, 0, 0, 0, 0, PATH_DOUBLE]);
        e.register(PATH_DOUBLE, |_, _| ret(PATH));
        let reference = e.mem.alloc(8);
        e.register(TREES_ENABLED, |_, _| ret(1));
        logged(&mut e, &log, FORM_MODEL_CHECK, |_, _| ret(1));
        e.register_double(REFERENCE_BASE_FORM, move |_, _| ret(form));
        e.register(REFERENCE_SEED, |_, _| ret(5));
        e.register(REFERENCE_SCALE, |_, _| float_ret(2.0));
        let tree_map = e.mem.alloc(8);
        e.mem.set_u32(tree_map, TREE_MAP_VTABLE);
        e.put_vtable(
            TREE_MAP_VTABLE,
            &[0, 0, FIND_DOUBLE, 0, INSERT_DOUBLE, REMOVE_DOUBLE],
        );
        e.set(
            this,
            BSTreeManager::pBackgroundLoadedTreeMap,
            Ptr::new(tree_map),
        );
        let background = Rc::new(Cell::new(0u32));
        let found = background.clone();
        let find_log = log.clone();
        e.register_double(FIND_DOUBLE, move |e, a| {
            find_log.borrow_mut().push((FIND_DOUBLE, a.to_vec()));
            if found.get() != 0 {
                e.mem.set_u32(a[2], found.get());
                ret(1)
            } else {
                ret(0)
            }
        });
        logged(&mut e, &log, REMOVE_DOUBLE, |_, _| Ret::default());
        let (array, model) = array_with_model(&mut e, 5);
        e.mem.set_u32(model, MODEL_VTABLE);
        e.mem.set_u32(model + 4, references);
        e.put_vtable(MODEL_VTABLE, &[DESTROY_DOUBLE, 0, 0, CREATE_ART_DOUBLE]);
        model_lookup(&mut e, this, array);
        e.register(REFERENCE_COUNT, |e, a| ret(e.mem.u32(a[0] + 4)));
        logged(&mut e, &log, CREATE_ART_DOUBLE, |_, _| ret(NODE));
        logged(&mut e, &log, DESTROY_DOUBLE, |_, _| Ret::default());
        let init_from_base = Rc::new(Cell::new(1u32));
        let result = init_from_base.clone();
        let base_log = log.clone();
        e.register_double(MODEL_INIT_FROM_BASE, move |_, a| {
            base_log
                .borrow_mut()
                .push((MODEL_INIT_FROM_BASE, a.to_vec()));
            ret(result.get())
        });
        let init_as_instance = Rc::new(Cell::new(1u32));
        let result = init_as_instance.clone();
        let init_log = log.clone();
        e.register_double(MODEL_INIT_AS_INSTANCE, move |_, a| {
            init_log
                .borrow_mut()
                .push((MODEL_INIT_AS_INSTANCE, a.to_vec()));
            ret(result.get())
        });
        logged(&mut e, &log, FIXED_STRING_CONSTRUCT, |_, a| ret(a[0]));
        logged(&mut e, &log, NODE_SET_NAME, |_, _| Ret::default());
        logged(&mut e, &log, FIXED_STRING_DESTRUCT, |_, _| Ret::default());
        Tree {
            e,
            log,
            this,
            form,
            reference,
            array,
            model,
            tree_map,
            background,
            init_from_base,
            init_as_instance,
        }
    }

    #[test]
    fn a_tree_is_not_built_when_trees_are_disabled() {
        let mut t = tree_fixture(2);
        t.e.register(TREES_ENABLED, |_, _| ret(0));
        let node =
            t.e.call(0x0066_4f50, &args![t.this, t.reference, t.form])
                .u32();
        assert_eq!(node, 0);
        assert!(calls_to(&t.log, LOCK_ENTER).is_empty());
        // The pointer local dies before the guard.
        let order: Vec<u32> = t.log.borrow().iter().map(|(a, _)| *a).collect();
        let pointer = order
            .iter()
            .position(|a| *a == NI_POINTER_DESTRUCT)
            .unwrap();
        let guard = order.iter().position(|a| *a == GUARD_DESTRUCT).unwrap();
        assert!(pointer < guard);
        let construct = calls_to(&t.log, GUARD_CONSTRUCT);
        assert_eq!(construct[0][1..], [0x1e, 1, SOURCE_PATH_STRING, 0x1ce]);
    }

    #[test]
    fn a_null_form_or_one_without_a_model_path_gives_null() {
        let mut t = tree_fixture(2);
        assert_eq!(
            t.e.call(0x0066_4f50, &args![t.this, t.reference, 0u32])
                .u32(),
            0
        );
        t.e.register(PATH_DOUBLE, |_, _| ret(0));
        let node =
            t.e.call(0x0066_4f50, &args![t.this, t.reference, t.form])
                .u32();
        assert_eq!(node, 0);
        assert!(calls_to(&t.log, FORM_MODEL_CHECK).is_empty());
    }

    #[test]
    fn a_form_failing_its_check_or_a_foreign_reference_gives_null() {
        let mut t = tree_fixture(2);
        t.e.register(FORM_MODEL_CHECK, |_, _| ret(0));
        assert_eq!(
            t.e.call(0x0066_4f50, &args![t.this, t.reference, t.form])
                .u32(),
            0
        );
        let mut t = tree_fixture(2);
        t.e.register(REFERENCE_BASE_FORM, |_, _| ret(0x1111));
        assert_eq!(
            t.e.call(0x0066_4f50, &args![t.this, t.reference, t.form])
                .u32(),
            0
        );
        assert!(calls_to(&t.log, FIND_DOUBLE).is_empty());
    }

    #[test]
    fn a_background_loaded_tree_is_taken_out_of_the_map() {
        let mut t = tree_fixture(2);
        t.background.set(0x4444);
        let node =
            t.e.call(0x0066_4f50, &args![t.this, t.reference, t.form])
                .u32();
        assert_eq!(node, 0x4444);
        assert_eq!(
            calls_to(&t.log, REMOVE_DOUBLE),
            vec![vec![t.tree_map, t.reference]]
        );
        assert!(calls_to(&t.log, LOCK_ENTER).is_empty());
    }

    #[test]
    fn a_shared_model_makes_the_named_node_at_the_reference_scale() {
        let mut t = tree_fixture(2);
        let node =
            t.e.call(0x0066_4f50, &args![t.this, t.reference, t.form])
                .u32();
        assert_eq!(node, NODE);
        assert_eq!(
            calls_to(&t.log, MODEL_INIT_FROM_BASE),
            vec![vec![t.model, t.form, 5]]
        );
        assert_eq!(
            calls_to(&t.log, CREATE_ART_DOUBLE),
            vec![vec![t.model, 2.0f32.to_bits()]]
        );
        let name = calls_to(&t.log, FIXED_STRING_CONSTRUCT);
        assert_eq!(name.len(), 1);
        assert_eq!(name[0][1], PATH);
        assert_eq!(
            calls_to(&t.log, NODE_SET_NAME),
            vec![vec![NODE, name[0][0]]]
        );
        assert_eq!(
            calls_to(&t.log, FIXED_STRING_DESTRUCT),
            vec![vec![name[0][0]]]
        );
        assert_eq!(t.e.mem.u8(t.model + 0x44), 1);
        assert_eq!(t.e.mem.u32(t.array), t.model);
    }

    #[test]
    fn a_model_with_many_references_gets_an_instance_model() {
        let mut t = tree_fixture(3);
        // No reference: seed 1, scale 1.0, no base form check.
        let node = t.e.call(0x0066_4f50, &args![t.this, 0u32, t.form]).u32();
        assert_eq!(node, NODE);
        let created = calls_to(&t.log, NI_ALLOCATE);
        assert_eq!(created, vec![vec![0x50]]);
        let init = calls_to(&t.log, MODEL_INIT_AS_INSTANCE);
        assert_eq!(init.len(), 1);
        let instance = init[0][0];
        assert_eq!(t.e.mem.u32(instance), MODEL_VTABLE);
        assert_eq!(
            calls_to(&t.log, CREATE_ART_DOUBLE),
            vec![vec![instance, 1.0f32.to_bits()]]
        );
        assert_eq!(t.e.mem.u8(instance + 0x44), 1);
    }

    #[test]
    fn a_failed_instance_is_destroyed_and_nothing_is_built() {
        let mut t = tree_fixture(3);
        t.init_as_instance.set(0);
        let node =
            t.e.call(0x0066_4f50, &args![t.this, t.reference, t.form])
                .u32();
        assert_eq!(node, 0);
        let init = calls_to(&t.log, MODEL_INIT_AS_INSTANCE);
        assert_eq!(calls_to(&t.log, DESTROY_DOUBLE), vec![vec![init[0][0], 1]]);
        assert!(calls_to(&t.log, CREATE_ART_DOUBLE).is_empty());
    }

    #[test]
    fn a_shared_model_failing_its_base_init_gives_null() {
        let mut t = tree_fixture(1);
        t.init_from_base.set(0);
        let node =
            t.e.call(0x0066_4f50, &args![t.this, t.reference, t.form])
                .u32();
        assert_eq!(node, 0);
        assert!(calls_to(&t.log, CREATE_ART_DOUBLE).is_empty());
        assert_eq!(calls_to(&t.log, NI_POINTER_DESTRUCT).len(), 2);
    }

    #[test]
    fn no_model_gives_null() {
        let mut t = tree_fixture(2);
        t.e.mem.set_u32(t.array, 0);
        t.e.register(NI_ALLOCATE, |_, _| ret(0));
        let node =
            t.e.call(0x0066_4f50, &args![t.this, t.reference, t.form])
                .u32();
        assert_eq!(node, 0);
        assert!(calls_to(&t.log, CREATE_ART_DOUBLE).is_empty());
    }

    #[test]
    fn the_initialised_byte_is_stored() {
        let mut e = Engine::new();
        let model: Ptr = Ptr::new(e.mem.alloc(0x50));
        e.call(0x0066_52c0, &args![model, 1u32]);
        assert_eq!(e.mem.u8(model.addr() + 0x44), 1);
        e.call(0x0066_52c0, &args![model, 0u32]);
        assert_eq!(e.mem.u8(model.addr() + 0x44), 0);
    }

    fn clamped(value: f32) -> u32 {
        let mut e = Engine::new();
        map_floats(&mut e);
        e.call(0x0066_5520, &args![value]);
        e.global::<u32>(CLAMPED_STATIC)
    }

    #[test]
    fn the_clamp_keeps_values_between_zero_and_one() {
        assert_eq!(clamped(0.25), 0.25f32.to_bits());
        assert_eq!(clamped(1.0), 1.0f32.to_bits());
        assert_eq!(clamped(0.0), 0);
        assert_eq!(clamped(-0.0), (-0.0f32).to_bits());
    }

    #[test]
    fn the_clamp_saturates_and_sends_nan_to_zero() {
        assert_eq!(clamped(-1.0), 0);
        assert_eq!(clamped(f32::NEG_INFINITY), 0);
        assert_eq!(clamped(f32::NAN), 0);
        assert_eq!(clamped(f32::from_bits(0x7f80_0001)), 0);
        assert_eq!(clamped(7.5), 1.0f32.to_bits());
        assert_eq!(clamped(f32::INFINITY), 1.0f32.to_bits());
    }

    #[test]
    fn the_setting_getters_and_their_products() {
        let mut e = Engine::new();
        map_floats(&mut e);
        e.set_global(SETTING_A + 4, 1.5f32);
        e.set_global(SETTING_B + 4, 0.5f32);
        e.set_global(SETTING_MULTIPLIER + 4, 4.0f32);
        assert_eq!(e.call(0x0066_55a0, &[]).f32(), 1.5);
        assert_eq!(e.call(0x0066_55f0, &[]).f32(), 0.5);
        assert_eq!(e.call(0x0066_5570, &[]).f32(), 6.0);
        assert_eq!(e.call(0x0066_55c0, &[]).f32(), 2.0);
        e.set_global(SETTING_A + 4, 0x7f80_0001u32);
        assert_eq!(e.call(0x0066_55a0, &[]).f32().to_bits(), 0x7fc0_0001);
    }

    /// An engine with the canopy shadow doubles; `texture_image` is the
    /// texture the `TES` image creation stores (0 for none).
    fn canopy_env(texture_image: u32) -> (Engine, Log, Ptr<BSTreeManager>, u32) {
        let (mut e, log) = base_engine();
        let this = manager_in(&mut e);
        e.map(0x011d_ea00, 0x100);
        let buffer = e.mem.alloc(0x300);
        logged(&mut e, &log, CANOPY_SHADOW_GLOBAL_ASSIGN, |_, _| {
            Ret::default()
        });
        let created = Rc::new(Cell::new(texture_image));
        let image_log = log.clone();
        e.register_double(CREATE_TEXTURE_IMAGE, move |e, a| {
            image_log
                .borrow_mut()
                .push((CREATE_TEXTURE_IMAGE, a.to_vec()));
            e.mem.set_u32(a[2], created.get());
            Ret::default()
        });
        logged(&mut e, &log, REMOVE_TEXTURE_IMAGE, |_, _| Ret::default());
        logged(&mut e, &log, PIXEL_DATA_CONSTRUCT, |_, a| ret(a[0]));
        let pixel_log = log.clone();
        e.register_double(PIXEL_DATA_PIXELS, move |_, a| {
            pixel_log.borrow_mut().push((PIXEL_DATA_PIXELS, a.to_vec()));
            ret(buffer)
        });
        logged(&mut e, &log, PIXEL_DATA_FINISH, |_, _| Ret::default());
        logged(&mut e, &log, FIXED_STRING_CONSTRUCT, |_, a| ret(a[0]));
        logged(&mut e, &log, FIXED_STRING_C_STR, |_, _| ret(0x6200_0000));
        logged(&mut e, &log, SOURCE_TEXTURE_CREATE, |_, _| ret(0x9999));
        logged(&mut e, &log, FIXED_STRING_DESTRUCT, |_, _| Ret::default());
        (e, log, this, buffer)
    }

    #[test]
    fn the_canopy_shadow_is_a_white_texture_without_a_tes() {
        let (mut e, log, this, buffer) = canopy_env(0);
        let texture = e.call(0x0066_5610, &[]).u32();
        assert_eq!(texture, 0x9999);
        assert_eq!(e.get(this, BSTreeManager::spCanopyShadow).addr(), 0x9999);
        assert_eq!(
            calls_to(&log, GUARD_CONSTRUCT)[0][1..],
            [0x1e, 1, SOURCE_PATH_STRING, 0x275]
        );
        assert_eq!(calls_to(&log, NI_ALLOCATE), vec![vec![0x74]]);
        let pixels = calls_to(&log, PIXEL_DATA_CONSTRUCT);
        assert_eq!(pixels.len(), 1);
        assert_eq!(pixels[0][1..], [0x10, 0x10, PIXEL_FORMAT, 1, 1]);
        assert_eq!(
            calls_to(&log, PIXEL_DATA_PIXELS),
            vec![vec![pixels[0][0], 0, 0]]
        );
        assert!(e.mem.bytes(buffer, 0x300).iter().all(|byte| *byte == 0xff));
        assert_eq!(calls_to(&log, PIXEL_DATA_FINISH), vec![vec![pixels[0][0]]]);
        let name = calls_to(&log, FIXED_STRING_CONSTRUCT);
        assert_eq!(name[0][1], CANOPY_TEXTURE_NAME);
        assert_eq!(calls_to(&log, FIXED_STRING_C_STR), vec![vec![name[0][0]]]);
        assert_eq!(
            calls_to(&log, SOURCE_TEXTURE_CREATE),
            vec![vec![pixels[0][0], 0x6200_0000, TEXTURE_PREFERENCES]]
        );
        assert_eq!(
            calls_to(&log, FIXED_STRING_DESTRUCT),
            vec![vec![name[0][0]]]
        );
        assert_eq!(
            calls_to(&log, CANOPY_SHADOW_GLOBAL_ASSIGN),
            vec![vec![0], vec![0x9999]]
        );
        assert!(calls_to(&log, CREATE_TEXTURE_IMAGE).is_empty());
    }

    #[test]
    fn the_canopy_shadow_image_is_loaded_through_the_tes_when_there_is_one() {
        let (mut e, log, this, _) = canopy_env(0x5555);
        let tes = e.mem.alloc(8);
        e.set_global(TES_SINGLETON, tes);
        let texture = e.call(0x0066_5610, &[]).u32();
        assert_eq!(texture, 0x5555);
        let slot = this.addr() + BSTreeManager::spCanopyShadow.off;
        assert_eq!(
            calls_to(&log, CREATE_TEXTURE_IMAGE),
            vec![vec![tes, CANOPY_SHADOW_PATH, slot, 1, 0]]
        );
        assert!(calls_to(&log, NI_ALLOCATE).is_empty());
        assert_eq!(
            calls_to(&log, CANOPY_SHADOW_GLOBAL_ASSIGN),
            vec![vec![0x5555]]
        );
    }

    #[test]
    fn a_loaded_canopy_shadow_is_returned_as_it_is() {
        let (mut e, log, this, _) = canopy_env(0);
        e.set(this, BSTreeManager::spCanopyShadow, Ptr::new(0x7777));
        assert_eq!(e.call(0x0066_5610, &[]).u32(), 0x7777);
        assert!(calls_to(&log, CANOPY_SHADOW_GLOBAL_ASSIGN).is_empty());
        assert_eq!(calls_to(&log, GUARD_DESTRUCT).len(), 1);
    }

    #[test]
    fn clearing_the_canopy_shadow_removes_the_texture_image() {
        let (mut e, log, this, _) = canopy_env(0);
        let tes = e.mem.alloc(8);
        e.set_global(TES_SINGLETON, tes);
        e.call(0x0066_5860, &[]);
        assert!(calls_to(&log, REMOVE_TEXTURE_IMAGE).is_empty());
        e.set(this, BSTreeManager::spCanopyShadow, Ptr::new(0x7777));
        e.call(0x0066_5860, &[]);
        assert_eq!(e.get(this, BSTreeManager::spCanopyShadow), Ptr::NULL);
        assert_eq!(calls_to(&log, CANOPY_SHADOW_GLOBAL_ASSIGN), vec![vec![0]]);
        assert_eq!(
            calls_to(&log, REMOVE_TEXTURE_IMAGE),
            vec![vec![tes, CANOPY_SHADOW_PATH, 0]]
        );
    }

    /// Doubles for the wind update, recording what they are given.
    fn wind_env() -> (Engine, Log) {
        let mut e = Engine::new();
        map_floats(&mut e);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        logged(&mut e, &log, FRAME_TIME, |_, _| float_ret(0.25));
        logged(&mut e, &log, SWAY_FIRST, |_, _| float_ret(0.5));
        logged(&mut e, &log, SWAY_SECOND, |_, _| float_ret(0.25));
        logged(&mut e, &log, SWAY_TERM, |_, a| {
            float_ret(f32::from_bits(a[0]) as f64 * 10.0)
        });
        logged(&mut e, &log, SPEEDTREE_SET_WIND_DIRECTION, |_, _| {
            Ret::default()
        });
        logged(&mut e, &log, SPEEDTREE_SET_LIGHTING, |_, _| Ret::default());
        for phase in 0..8u32 {
            let value = if phase % 2 == 0 { 1.0f32 } else { 2.0 };
            e.set_global(SWAY_TABLE + 4 * phase, value);
        }
        for (setting, value) in [
            (0x011d_5dc4u32, 0.5f32),
            (0x011d_5d3c, 0.25),
            (0x011d_5c68, 0.125),
            (0x011d_5c74, 0.75),
            (0x011d_5c5c, 3.0),
            (0x011d_5cdc, 5.0),
        ] {
            e.set_global(setting + 4, value);
        }
        (e, log)
    }

    fn wind_manager(e: &mut Engine, wind: f32) -> Ptr<BSTreeManager> {
        let this = e.new_object::<BSTreeManager>();
        e.set(this, BSTreeManager::fWindSpeed, wind);
        this
    }

    #[test]
    fn the_wind_update_rescales_the_phases_and_hands_over_the_sway() {
        let (mut e, log) = wind_env();
        let this = wind_manager(&mut e, 2.0);
        e.set_global(LAST_WIND_SPEED_READY, 1u32);
        e.set_global(LAST_WIND_SPEED, 4.0f32);
        e.call(0x0066_58b0, &args![this]);
        // Each phase: 0.25 advanced, then times last / current wind.
        for phase in 0..4u32 {
            assert_eq!(e.global::<f32>(PHASES + 4 * phase), 0.5);
        }
        let scale = (2.0f64 * 0.61f32 as f64) as f32;
        let first = (scale as f64 * 0.5) as f32;
        let second = (scale as f64 * 0.25) as f32;
        // The sway terms of `20 * wind * table * phase`.
        assert_eq!(
            calls_to(&log, SWAY_FIRST)[0],
            vec![(40.0f32 * 1.0 * 0.5).to_bits()]
        );
        assert_eq!(
            calls_to(&log, SWAY_SECOND)[0],
            vec![(40.0f32 * 2.0 * 0.5).to_bits()]
        );
        let directions = calls_to(&log, SPEEDTREE_SET_WIND_DIRECTION);
        assert_eq!(directions.len(), 4);
        for (phase, call) in directions.iter().enumerate() {
            assert_eq!(*call, vec![phase as u32, first.to_bits(), second.to_bits()]);
        }
        // Only phase 2 sums the terms: 0.5 * 10 + 0.25 * 10.
        assert_eq!(calls_to(&log, SWAY_TERM).len(), 2);
        let lighting = |setting: f32| setting * 7.5 / 2.0 + (1.0 - setting);
        assert_eq!(
            calls_to(&log, SPEEDTREE_SET_LIGHTING),
            vec![vec![
                (2.0 * lighting(0.5)).to_bits(),
                (2.0 * lighting(0.25)).to_bits(),
                (5.0 * 0.25 * lighting(0.125)).to_bits(),
                (3.0 * 0.25 * lighting(0.75)).to_bits(),
            ]]
        );
        assert_eq!(e.global::<f32>(LAST_WIND_SPEED), 2.0);
    }

    #[test]
    fn the_first_wind_update_takes_the_wind_speed_and_zero_wind_skips_the_rescale() {
        let (mut e, _log) = wind_env();
        let this = wind_manager(&mut e, 0.0);
        e.set_global(LAST_WIND_SPEED, 9.0f32);
        e.call(0x0066_58b0, &args![this]);
        assert_eq!(e.global::<u32>(LAST_WIND_SPEED_READY) & 1, 1);
        assert_eq!(e.global::<f32>(LAST_WIND_SPEED), 0.0);
        for phase in 0..4u32 {
            assert_eq!(e.global::<f32>(PHASES + 4 * phase), 0.25);
        }
    }

    #[test]
    fn a_nan_wind_speed_still_rescales_the_phases() {
        let (mut e, _log) = wind_env();
        let this = wind_manager(&mut e, f32::NAN);
        e.set_global(LAST_WIND_SPEED_READY, 1u32);
        e.set_global(LAST_WIND_SPEED, 1.0f32);
        e.call(0x0066_58b0, &args![this]);
        assert!(e.global::<f32>(PHASES).is_nan());
    }

    /// Everything `006652e0` calls, recording; the camera position is
    /// `(1, 2, signalling NaN)`.
    struct Update {
        e: Engine,
        log: Log,
        this: Ptr<BSTreeManager>,
        camera: u32,
        camera_call: Rc<RefCell<Vec<u32>>>,
    }

    fn update_env() -> Update {
        let (mut e, log) = base_engine();
        let this = manager_in(&mut e);
        map_floats(&mut e);
        e.map(0x011d_ea00, 0x100);
        logged(&mut e, &log, VECTOR_CONSTRUCT, |_, _| Ret::default());
        logged(&mut e, &log, FRAME_TIME, |_, _| float_ret(0.5));
        logged(&mut e, &log, SPEEDTREE_SET_TIME, |_, _| Ret::default());
        logged(&mut e, &log, SET_TREE_DISTANCE, |_, _| Ret::default());
        e.register(SETTING_VALUE_WORD, |_, a| ret(a[0] + 4));
        for addr in [
            SPEEDTREE_SET_WIND_DIRECTION,
            SPEEDTREE_SET_LIGHTING,
            SWAY_FIRST,
            SWAY_SECOND,
            SWAY_TERM,
        ] {
            logged(&mut e, &log, addr, |_, _| Ret::default());
        }
        let camera = e.mem.alloc(0x10);
        let position = e.mem.alloc(12);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_u32(position + 8, 0x7f80_0001);
        let direction = e.mem.alloc(12);
        logged_with(&mut e, &log, OBJECT_POSITION, move |_, _| ret(position));
        logged_with(&mut e, &log, OBJECT_DIRECTION, move |_, _| ret(direction));
        logged(&mut e, &log, SCALED_VECTOR, |e, a| {
            let factor = f32::from_bits(a[1]);
            let source = a[2];
            let lane = e.mem.f32(source) * factor;
            e.mem.set_f32(a[0], lane);
            e.mem.set_u32(a[0] + 4, 0x7f80_0002);
            let lane = e.mem.f32(source + 8) * factor;
            e.mem.set_f32(a[0] + 8, lane);
            ret(a[0])
        });
        e.mem.set_f32(direction, 0.5);
        e.mem.set_f32(direction + 8, 0.125);
        let camera_call = Rc::new(RefCell::new(Vec::new()));
        let seen = camera_call.clone();
        e.register_double(SPEEDTREE_SET_CAMERA, move |e, a| {
            let mut words = Vec::new();
            for lane in 0..3u32 {
                words.push(e.mem.u32(a[0] + 4 * lane));
            }
            for lane in 0..3u32 {
                words.push(e.mem.u32(a[1] + 4 * lane));
            }
            *seen.borrow_mut() = words;
            Ret::default()
        });
        // The tail: the tree distance setting word and the clamped setting.
        e.set_global(TREE_DISTANCE_SETTING + 4, 100u32);
        e.set_global(SETTING_TO_CLAMP + 4, 0.5f32);
        Update {
            e,
            log,
            this,
            camera,
            camera_call,
        }
    }

    #[test]
    fn the_update_without_the_byte_work_only_sets_the_statics() {
        let mut u = update_env();
        u.e.call(0x0066_52e0, &args![0u32, 1u32]);
        assert_eq!(calls_to(&u.log, SET_TREE_DISTANCE), vec![vec![100]]);
        assert_eq!(u.e.global::<f32>(CLAMPED_STATIC), 0.5);
        assert!(calls_to(&u.log, SPEEDTREE_SET_TIME).is_empty());
        assert!(u.camera_call.borrow().is_empty());
    }

    #[test]
    fn the_update_gives_the_speedtree_the_camera_and_its_negated_direction() {
        let mut u = update_env();
        u.e.call(0x0066_52e0, &args![u.camera, 1u32]);
        // The signalling NaN of the position comes out quiet; the lane the
        // scaled-vector double fills with one comes out quiet too.
        assert_eq!(
            *u.camera_call.borrow(),
            vec![
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                0x7fc0_0001,
                (-0.5f32).to_bits(),
                0x7fc0_0002,
                (-0.125f32).to_bits(),
            ]
        );
        let scaled = calls_to(&u.log, SCALED_VECTOR);
        assert_eq!(scaled[0][1], (-1.0f32).to_bits());
        assert_eq!(calls_to(&u.log, OBJECT_POSITION).len(), 3);
    }

    #[test]
    fn the_update_advances_the_time_and_the_manager() {
        let mut u = update_env();
        u.e.set_global(TIME_ACCUMULATOR, 0.25f32);
        u.e.call(0x0066_52e0, &args![0u32, 0u32]);
        assert_eq!(u.e.global::<f32>(TIME_ACCUMULATOR), 0.75);
        assert_eq!(
            calls_to(&u.log, SPEEDTREE_SET_TIME),
            vec![vec![0.75f32.to_bits()]]
        );
        // The manager's wind update ran.
        assert_eq!(calls_to(&u.log, SPEEDTREE_SET_LIGHTING).len(), 1);
        assert_eq!(calls_to(&u.log, SET_TREE_DISTANCE), vec![vec![100]]);
    }

    #[test]
    fn an_interior_cell_stops_the_wind() {
        let mut u = update_env();
        let tes = u.e.mem.alloc(8);
        u.e.set_global(TES_SINGLETON, tes);
        u.e.register(TES_FIELD_68, |_, _| ret(0x4000));
        u.e.register(TES_INTERIOR_CELL, |_, _| ret(0x4100));
        u.e.set(u.this, BSTreeManager::fWindSpeed, 9.0);
        u.e.call(0x0066_52e0, &args![0u32, 0u32]);
        assert_eq!(u.e.get(u.this, BSTreeManager::fWindSpeed), 0.0);
        assert!(calls_to(&u.log, FLOAT_OF_OBJECT).is_empty());
    }

    #[test]
    fn outdoors_the_wind_comes_from_the_tes_and_the_material_gets_the_colour() {
        let mut u = update_env();
        let tes = u.e.mem.alloc(8);
        u.e.set_global(TES_SINGLETON, tes);
        u.e.register(TES_FIELD_68, |_, _| ret(0x4000));
        u.e.register(TES_INTERIOR_CELL, |_, _| ret(0));
        logged(&mut u.e, &u.log, FLOAT_OF_OBJECT, |_, _| float_ret(3.5));
        u.e.register(OBJECT_OF_OBJECT, |_, _| ret(0x4200));
        let colour = u.e.mem.alloc(12);
        u.e.mem.set_f32(colour, 0.1);
        u.e.mem.set_f32(colour + 4, 0.2);
        u.e.mem.set_f32(colour + 8, 0.3);
        u.e.register_double(COLOR_POINTER, move |_, _| ret(colour));
        let material = u.e.mem.alloc(8);
        u.e.set(
            u.this,
            BSTreeManager::spBillboardMatProp,
            Ptr::new(material),
        );
        let seen = Rc::new(RefCell::new(Vec::new()));
        for setter in [COLOR_SETTER_NOOP, COLOR_SETTER] {
            let seen = seen.clone();
            u.e.register_double(setter, move |e, a| {
                let lanes: Vec<u32> = (0..3).map(|lane| e.mem.u32(a[1] + 4 * lane)).collect();
                seen.borrow_mut().push((setter, a[0], lanes));
                Ret::default()
            });
        }
        u.e.call(0x0066_52e0, &args![0u32, 0u32]);
        assert_eq!(u.e.get(u.this, BSTreeManager::fWindSpeed), 3.5);
        let lanes = vec![0.1f32.to_bits(), 0.2f32.to_bits(), 0.3f32.to_bits()];
        assert_eq!(
            *seen.borrow(),
            vec![
                (COLOR_SETTER_NOOP, material, lanes.clone()),
                (COLOR_SETTER_NOOP, material, lanes.clone()),
                (COLOR_SETTER, material, lanes),
            ]
        );
    }

    /// The background tree map of `00665b80`/`00665be0`: its insert and
    /// remove slots are doubles; the tree objects destroy through
    /// `DESTROY_DOUBLE`.
    const TREE_OBJECT_VTABLE: u32 = 0x7500_0000;

    fn background_env(taken: u32, found: u32) -> (Engine, Log, Ptr<BSTreeManager>, u32) {
        let (mut e, log) = base_engine();
        let this = manager_in(&mut e);
        let tree_map = e.mem.alloc(8);
        e.mem.set_u32(tree_map, TREE_MAP_VTABLE);
        e.put_vtable(
            TREE_MAP_VTABLE,
            &[0, 0, FIND_DOUBLE, 0, INSERT_DOUBLE, REMOVE_DOUBLE],
        );
        e.set(
            this,
            BSTreeManager::pBackgroundLoadedTreeMap,
            Ptr::new(tree_map),
        );
        let insert_log = log.clone();
        e.register_double(INSERT_DOUBLE, move |e, a| {
            // The tree word the exe passes by address.
            insert_log
                .borrow_mut()
                .push((INSERT_DOUBLE, vec![a[0], a[1], e.mem.u32(a[2]), a[3]]));
            ret(taken)
        });
        let find_log = log.clone();
        e.register_double(FIND_DOUBLE, move |e, a| {
            find_log.borrow_mut().push((FIND_DOUBLE, a.to_vec()));
            if found != 0 {
                e.mem.set_u32(a[2], found);
                ret(1)
            } else {
                ret(0)
            }
        });
        logged(&mut e, &log, REMOVE_DOUBLE, |_, _| Ret::default());
        logged(&mut e, &log, DESTROY_DOUBLE, |_, _| Ret::default());
        logged(&mut e, &log, RELEASE_TREE, |_, _| Ret::default());
        e.put_vtable(TREE_OBJECT_VTABLE, &[DESTROY_DOUBLE]);
        (e, log, this, tree_map)
    }

    #[test]
    fn a_tree_the_map_does_not_take_is_destroyed() {
        let (mut e, log, this, tree_map) = background_env(0, 0);
        let tree = e.mem.alloc(8);
        e.mem.set_u32(tree, TREE_OBJECT_VTABLE);
        e.call(0x0066_5b80, &args![this, 0x1234u32, tree]);
        assert_eq!(
            calls_to(&log, INSERT_DOUBLE),
            vec![vec![tree_map, 0x1234, tree, 0]]
        );
        assert_eq!(calls_to(&log, DESTROY_DOUBLE), vec![vec![tree, 1]]);
    }

    #[test]
    fn a_tree_the_map_takes_is_kept_and_a_null_tree_is_not_destroyed() {
        let (mut e, log, this, _) = background_env(1, 0);
        let tree = e.mem.alloc(8);
        e.mem.set_u32(tree, TREE_OBJECT_VTABLE);
        e.call(0x0066_5b80, &args![this, 0x1234u32, tree]);
        assert!(calls_to(&log, DESTROY_DOUBLE).is_empty());
        let (mut e, log, this, _) = background_env(0, 0);
        e.call(0x0066_5b80, &args![this, 0x1234u32, 0u32]);
        assert!(calls_to(&log, DESTROY_DOUBLE).is_empty());
    }

    #[test]
    fn removing_a_background_tree_releases_what_was_found() {
        let (mut e, log, this, tree_map) = background_env(0, 0x4444);
        e.call(0x0066_5be0, &args![this, 0x1234u32]);
        assert_eq!(calls_to(&log, REMOVE_DOUBLE), vec![vec![tree_map, 0x1234]]);
        assert_eq!(calls_to(&log, RELEASE_TREE), vec![vec![0x4444]]);
        let (mut e, log, this, _) = background_env(0, 0);
        e.call(0x0066_5be0, &args![this, 0x1234u32]);
        assert!(calls_to(&log, REMOVE_DOUBLE).is_empty());
        assert!(calls_to(&log, RELEASE_TREE).is_empty());
    }

    #[test]
    fn the_map_base_constructor_builds_a_zeroed_bucket_table() {
        let (mut e, log) = base_engine();
        logged(&mut e, &log, MEMSET, |_, _| Ret::default());
        let map = e.mem.alloc(0x10);
        let back = e.call(0x0066_5f50, &args![map, 7u32]);
        assert_eq!(back.u32(), map);
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 7);
        assert_eq!(e.mem.u32(map + 0xc), 0);
        let table = e.mem.u32(map + 8);
        assert_eq!(calls_to(&log, ALLOCATE_BYTES), vec![vec![28]]);
        assert_eq!(calls_to(&log, MEMSET), vec![vec![table, 0, 28]]);
    }

    #[test]
    fn the_map_constructor_sets_its_own_vtable() {
        let (mut e, log) = base_engine();
        logged(&mut e, &log, MEMSET, |_, _| Ret::default());
        let map = e.mem.alloc(0x10);
        let back = e.call(0x0066_5c50, &args![map, 0x25u32]);
        assert_eq!(back.u32(), map);
        assert_eq!(e.mem.u32(map), MAP_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 0x25);
    }

    fn destructor_env() -> (Engine, Log, u32) {
        let (mut e, log) = base_engine();
        logged(&mut e, &log, MAP_CLEAR, |_, _| Ret::default());
        logged(&mut e, &log, MAP_BASE_DESTRUCT, |_, _| Ret::default());
        logged(&mut e, &log, OPERATOR_DELETE, |_, _| Ret::default());
        let map = e.mem.alloc(0x10);
        (e, log, map)
    }

    #[test]
    fn the_map_destructor_clears_then_runs_the_base() {
        let (mut e, log, map) = destructor_env();
        e.call(0x0066_5fc0, &args![map]);
        assert_eq!(e.mem.u32(map), MAP_VTABLE);
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(order, vec![MAP_CLEAR, MAP_BASE_DESTRUCT]);
        assert_eq!(calls_to(&log, MAP_CLEAR), vec![vec![map]]);
    }

    #[test]
    fn the_map_scalar_deleting_destructors_free_only_with_bit_zero() {
        let (mut e, log, map) = destructor_env();
        assert_eq!(e.call(0x0066_5c80, &args![map, 0u32]).u32(), map);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        assert_eq!(e.call(0x0066_5c80, &args![map, 1u32]).u32(), map);
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![map]]);
        let (mut e, log, map) = destructor_env();
        assert_eq!(e.call(0x0066_6270, &args![map, 0u32]).u32(), map);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        assert_eq!(e.call(0x0066_6270, &args![map, 3u32]).u32(), map);
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![MAP_BASE_DESTRUCT, MAP_BASE_DESTRUCT, OPERATOR_DELETE]
        );
    }

    #[test]
    fn the_model_constructor_empties_every_field() {
        let (mut e, log) = base_engine();
        let model = e.mem.alloc(0x50);
        e.mem.write(model, &[0xaa; 0x50]);
        let back = e.call(0x0066_6650, &args![model]);
        assert_eq!(back.u32(), model);
        assert_eq!(calls_to(&log, NI_REF_OBJECT_CONSTRUCT), vec![vec![model]]);
        assert_eq!(e.mem.u32(model), MODEL_VTABLE);
        let constructed: Vec<u32> = calls_to(&log, NI_POINTER_CONSTRUCT)
            .iter()
            .map(|call| call[0] - model)
            .collect();
        assert_eq!(constructed, vec![0x10, 0x1c, 0x20, 0x2c, 0x30, 0x34, 0x38]);
        let assigned: Vec<u32> = calls_to(&log, NI_POINTER_ASSIGN)
            .iter()
            .map(|call| call[0] - model)
            .collect();
        assert_eq!(assigned, vec![0x1c, 0x20, 0x2c, 0x30, 0x34]);
        for offset in [
            0x08u32, 0x0c, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c,
            0x44, 0x48, 0x4c,
        ] {
            let width = if offset == 0x44 { 1 } else { 4 };
            let bytes = e.mem.bytes(model + offset, width);
            assert!(bytes.iter().all(|byte| *byte == 0), "field +{offset:#x}");
        }
        assert_eq!(e.mem.u32(model + 0x40), 1);
    }
}
