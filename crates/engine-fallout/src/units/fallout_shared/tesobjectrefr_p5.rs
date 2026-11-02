//! `fallout shared/tesobjectrefr.cpp` (Xbox PDB source unit), part 5: its functions from `00576870` up to
//! (not including) `0057cac0` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::tesobjectrefr`]; anything public there may be used here.
//!
//! # What is in this part
//!
//! This part holds the batch `00576870` to `00578970` (the first 40 open
//! functions of the range). It is mostly the glue between a reference and
//! its scene-graph and Havok objects: the transform-change callback
//! (`TransChangeCallback`, `00577450`), the recursive addon-node walkers
//! (`AddAddonNodes`, `RemoveAddonNodes`, ...), the owner tests
//! (`IsAnOwner`, `IsOwnerEvil`) and the small accessors the linker placed
//! between them. The next session continues with the next open function
//! after `00578970` in address order (`005789b0`).
//!
//! The second batch, `005789b0` to `0057b410` (the next 40 open functions),
//! holds the speech and sound glue (`00578a80` makes a reference speak a
//! topic and play its line; `00579ac0` starts and stops its looping sound;
//! `0057a410` walks the addon nodes' sounds), the target-type and
//! stealing tests (`00579280`, `00579690`), the small constructors and
//! destructors of two extra records, the multi-bound task and the
//! water-state accessors of the loaded data. The next session continues
//! after `0057b410` (`0057b460`).
//!
//! The third batch, `0057b460` to `0057ca90` (the last 40 open functions of
//! the range), holds the water removal of a reference (`0057b520`), the
//! speech start (`0057b7c0`), the bound rebuild over a scene graph
//! (`0057bd80`), the creation and destruction of the loaded data
//! (`0057c180`, `0057c300`) and the constructors, destructors and lookups
//! of the small hash tables and lists the linker placed after them. The
//! range is finished; the next function in the file is `0057cac0`, which
//! belongs to the next part.
//!
//! # Conventions of the code read here
//!
//! * The compiler's pushes in front of a call often belong to a later call:
//!   `PUSH x; CALL getter; MOV ECX,EAX; CALL method` hands `x` to `method`.
//!   Every call's argument words below were checked against the callee's
//!   `RET n` (or the caller's stack cleanup).
//! * Callees that take no stack word and ignore `ECX` are called without
//!   arguments (`00453a70`, `00448a80`, `0045a190`, `00458b20`).
//! * Locals the game keeps on its stack and passes by address (vectors,
//!   transforms) are scratch blocks of the engine's heap, freed before the
//!   function returns.
//! * C++ exception-unwinding frames (`00577ca0`) and stack-cookie checks are
//!   not translated.

#[allow(unused_imports)]
use super::tesobjectrefr::*;
#[allow(unused_imports)]
use crate::prelude::*;

/// `005d43c0`: the reference's embedded `ExtraDataList` (`this + 0x44`).
const GET_EXTRA_LIST: u32 = 0x005d_43c0;
/// `BGSSaveFormBuffer::GetForm` (engine map name; the body returns the
/// reference's base object, `pObjectReference`).
const GET_BASE_FORM: u32 = 0x007a_f430;
/// The form type byte (`cFormType`, at +4) of a form.
const FORM_TYPE: u32 = 0x0040_1170;
/// `__RTDynamicCast(object, 0, from, to, 0)`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `_finite(double)` and `_isnan(double)` of the C runtime.
const CRT_FINITE: u32 = 0x00ec_7595;
const CRT_ISNAN: u32 = 0x00ec_75b1;
/// `strncmp`.
const STRNCMP: u32 = 0x00ec_8a19;
/// The reference's parent cell (`pParentCell`, or the child-cell fallback).
const GET_PARENT_CELL: u32 = 0x008d_6f30;
/// Prints a message to the game's log (`format, args...`, cdecl).
const MESSAGE: u32 = 0x005b_5e40;

/// `0043b300(type, object)`: true when `object` is an instance of the
/// run-time type described by the descriptor at `type` (cdecl).
const IS_KIND_OF: u32 = 0x0043_b300;
/// Number of children of a scene-graph node.
const CHILD_COUNT: u32 = 0x0043_b480;
/// Child `index` of a scene-graph node.
const CHILD_AT: u32 = 0x0043_b4a0;
/// The run-time type descriptor of the scene-graph node class the addon
/// walkers accept (passed to [`IS_KIND_OF`]).
const NODE_TYPE: u32 = 0x0120_2de8;
/// `AsNode`: the node a scene-graph object is (virtual slot at +0x0c).
const SLOT_AS_NODE: u32 = 0x0c;
/// A reference's 3D (the virtual at +0x1d0, no stack word; the Actor
/// override of the slot is called the same way).
const SLOT_GET_3D: u32 = 0x1d0;
/// `IsActor` (virtual at +0x100).
const SLOT_IS_ACTOR: u32 = 0x100;
/// Writes the reference's position into the buffer it is given and returns a
/// pointer to the position (virtual at +0x170).
const SLOT_GET_POSITION: u32 = 0x170;
/// Returns a pointer to the reference's position (virtual at +0x1f4).
const SLOT_POSITION_PTR: u32 = 0x1f4;

/// `TESDataHandler` singleton (global `011c3f2c`).
const GLOBAL_DATA_HANDLER: u32 = 0x011c_3f2c;
/// `TES` singleton (global `011dea10`).
const GLOBAL_TES: u32 = 0x011d_ea10;
/// The object whose byte flag [`SWAP_FLAG`] and `0042ce10` read (global
/// `011ddf38`; the main file calls it the loading object).
const GLOBAL_FLAG_OBJECT: u32 = 0x011d_df38;
/// The player character (global `011dea3c`).
const GLOBAL_PLAYER: u32 = 0x011d_ea3c;
/// Base-form pointers compared with a reference's base object.
const GLOBAL_BASE_FORM_A: u32 = 0x011c_a240;
const GLOBAL_BASE_FORM_B: u32 = 0x011c_a27c;
const GLOBAL_BASE_FORM_C: u32 = 0x011c_a280;

/// `0x011f426c`: a zero vector (three floats).
const ZERO_VECTOR: u32 = 0x011f_426c;
/// Sets the byte flag of the object it is called on and returns the old one.
const SWAP_FLAG: u32 = 0x0046_23f0;
/// Moves a reference's position to the given point (pointer to three floats).
const SET_POSITION: u32 = 0x0057_5830;
/// Sets a reference's rotation (three euler angles by value).
const SET_ROTATION: u32 = 0x0057_5700;

/// Form type numbers compared with `cFormType` here; only the numbers are
/// known from the code.
const FORM_TYPE_NPC_LIKE: u32 = 0x08;
const FORM_TYPE_FACTION_LIKE: u32 = 0x2a;

/// `double` 128.0: how far below the terrain minimum a reference counts as
/// fallen out of the world.
const GROUND_MARGIN: u32 = 0x0102_e430;
/// `double` constants of `TransChangeCallback`.
const VELOCITY_SCALE: u32 = 0x0102_90b0;
const VELOCITY_LIMIT: u32 = 0x0102_0998;
/// `float` -1.0 used as the velocity of a pushed body.
const PUSH_VELOCITY: u32 = 0x0101_2054;
/// `float` threshold of the stillness test in `005768b0`.
const STILL_THRESHOLD: u32 = 0x0101_7d00;
/// Object whose address is `this` of the Havok vector helper (`0045bb20`).
const HAVOK_VECTOR_HELPER: u32 = 0x011a_9484;
/// Table of `float`s indexed by a `hkUFloat8` byte (`00577bf0`).
const UFLOAT8_TABLE: u32 = 0x010c_7948;
/// The message "HAVOK: Disabling collision on ref '%s' (%08X)."
const DISABLE_COLLISION_MESSAGE: u32 = 0x0103_0ed8;
/// "BASE", the name prefix of the addon-node markers.
const BASE_PREFIX: u32 = 0x0103_0f08;
/// The empty string literal.
const EMPTY_STRING: u32 = 0x0101_1584;
/// Divisor of the addon randomizer's seed (`double`).
const RANDOM_DIVISOR: u32 = 0x0101_7a40;
/// `double` compared with a global's value in `IsAnOwner`.
const OWNERSHIP_VALUE_LIMIT: u32 = 0x0101_2060;

/// `_finite` on a `float`.
fn crt_finite(e: &mut Engine, value: f32) -> bool {
    e.call(CRT_FINITE, &args![value as f64]).i32() != 0
}

/// `_isnan` on a `float`.
fn crt_isnan(e: &mut Engine, value: f32) -> bool {
    e.call(CRT_ISNAN, &args![value as f64]).i32() != 0
}

fn is_node(e: &mut Engine, object: u32) -> bool {
    e.call(IS_KIND_OF, &args![NODE_TYPE, object]).bool()
}

fn child_count(e: &mut Engine, node: u32) -> u32 {
    e.call(CHILD_COUNT, &args![node]).u32()
}

fn child_at(e: &mut Engine, node: u32, index: u32) -> u32 {
    e.call(CHILD_AT, &args![node, index]).u32()
}

/// The child's `AsNode` (virtual at +0x0c), 0 for a null child.
fn child_node(e: &mut Engine, child: u32) -> u32 {
    if child == 0 {
        0
    } else {
        e.vcall(child, SLOT_AS_NODE, &args![]).u32()
    }
}

/// The addon record the data handler holds for `node` (`009ee040(node)` is
/// the key handed to `004617e0`).
fn addon_for_node(e: &mut Engine, node: u32) -> u32 {
    let key = e.call(0x009e_e040, &args![node]).u32();
    let handler = global_word(e, GLOBAL_DATA_HANDLER);
    e.call(0x0046_17e0, &args![handler, key]).u32()
}

fn global_word(e: &Engine, addr: u32) -> u32 {
    e.global::<u32>(addr)
}

/// `body` of a collision object, or 0 (`006fa820` guarded by a null test).
fn body_of(e: &mut Engine, collision: u32) -> u32 {
    if collision == 0 {
        0
    } else {
        e.call(0x006f_a820, &args![collision]).u32()
    }
}

// Translated from 00576870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks up the element with form ID `form_id` in the list at `this + 4`
/// (`0057c400`) and returns the address 4 bytes before it, or 0.
pub fn fn_00576870(e: &mut Engine, this: Ptr, form_id: u32) -> u32 {
    let found = e.call(0x0057_c400, &args![this.addr() + 4, form_id]).u32();
    if found == 0 {
        0
    } else {
        found - 4
    }
}

// Translated from 005768b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Called when a reference's 3D has just been loaded: initializes the
/// Havok objects its base form needs (placeable water, primitive trigger,
/// ...), hands the loaded 3D's collision object to the cell's Havok world,
/// sets the motion type of dynamic bodies, and finishes with `0056c880`.
///
/// The world calls (virtual slots `0xd0`, `0xd4` of the cell's world) are
/// the ones the code makes; the names of the callees are not known from the
/// exe.
pub fn fn_005768b0(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    let cell = e.get(this, TESObjectREFR::pParentCell);
    if !cell.is_null()
        && !e.call(0x0045_0ff0, &args![cell]).bool()
        && !e.call(0x0045_23c0, &args![cell]).bool()
    {
        return;
    }
    let model = e.vcall(me, SLOT_GET_3D, &args![]).u32();
    if model != 0 {
        let tes = global_word(e, GLOBAL_TES);
        e.call(0x0045_3860, &args![tes, 1u32]);
        init_havok_for_base_form(e, this);
        handle_loaded_model(e, this, model);
        let tes = global_word(e, GLOBAL_TES);
        e.call(0x0045_3860, &args![tes, 0u32]);
    }
    e.call(0x0056_c880, &args![this, 1u32, 1u32]);
}

/// The first half of `005768b0`: unless `004523e0(this, 1)`, picks the Havok
/// initializer by the base form.
fn init_havok_for_base_form(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    if e.call(0x0045_23e0, &args![this, 1u32]).bool() {
        return;
    }
    let base = e.call(GET_BASE_FORM, &args![this]).u32();
    if e.call(FORM_TYPE, &args![base]).u32() == 0x23 {
        let base = e.call(GET_BASE_FORM, &args![this]).u32();
        if !e.call(0x0045_2440, &args![base]).bool() {
            // the placeable-water initializer
            e.call(0x0056_c8f0, &args![this]);
        }
        return;
    }
    if e.call(GET_BASE_FORM, &args![this]).u32() == global_word(e, GLOBAL_BASE_FORM_A) {
        e.call(0x0056_eab0, &args![this]);
        return;
    }
    let base = e.call(GET_BASE_FORM, &args![this]).u32();
    if e.call(FORM_TYPE, &args![base]).u32() == 0x0e {
        e.call(0x0056_f140, &args![this]);
        return;
    }
    let extra = e.call(GET_EXTRA_LIST, &args![this]).u32();
    if e.call(0x0041_fbe0, &args![extra]).u32() != 0 {
        let base = e.call(GET_BASE_FORM, &args![this]).u32();
        if e.call(FORM_TYPE, &args![base]).u32() == 0x15 {
            // the primitive-trigger initializer
            e.call(0x0056_d7e0, &args![this]);
        }
    }
}

/// The second half of `005768b0`, for a reference that has a 3D `model`.
fn handle_loaded_model(e: &mut Engine, this: Ptr<TESObjectREFR>, model: u32) {
    let me = this.addr();
    let model_node = e.vcall(model, SLOT_AS_NODE, &args![]).u32();
    let mut target = model;
    if !e.vcall(me, SLOT_IS_ACTOR, &args![]).bool() {
        let found = e.call(0x00c8_12d0, &args![model_node, 1u32]).u32();
        if found != 0 {
            target = e.call(0x0044_ddc0, &args![found]).u32();
        }
    }
    if target == 0 {
        // no target: a model of the `011d5e70` type is offered to the
        // cell's world through its slot 0xd0
        if e.call(IS_KIND_OF, &args![0x011d_5e70u32, model]).bool() {
            let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
            if cell != 0 {
                let world = e.call(0x0045_43c0, &args![cell]).u32();
                if world != 0 {
                    e.vcall(world, 0xd0, &args![model, 0u32, 0u32, 0u32, 0u32]);
                }
            }
        }
        return;
    }
    let cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    if cell == 0 {
        return;
    }
    let world = e.call(0x0045_43c0, &args![cell]).u32();
    let collision = e.call(0x004b_5260, &args![target]).u32();
    let body = body_of(e, collision);
    if world == 0 {
        return;
    }
    if body != 0 {
        let body_world = e.vcall(body, 0x94, &args![]).u32();
        let this_world = e.vcall(world, 0x94, &args![]).u32();
        if body_world == this_world {
            return;
        }
    }
    e.vcall(world, 0xd4, &args![target, model]);

    let base = e.call(GET_BASE_FORM, &args![this]).u32();
    let set_dynamic = base == global_word(e, GLOBAL_BASE_FORM_B)
        || e.call(GET_BASE_FORM, &args![this]).u32() == global_word(e, GLOBAL_BASE_FORM_C);
    if set_dynamic {
        let body = body_of(e, collision);
        let shape = e.call(0x0056_0d80, &args![body]).u32();
        let probe = e.mem.alloc(16);
        e.call(0x004a_3c90, &args![probe, shape]);
        let flag_object = global_word(e, GLOBAL_FLAG_OBJECT);
        let mut motion = false;
        let mut test_moving = true;
        if (e.call(0x0042_ce10, &args![flag_object]).bool() || fn_00576d30(e, this.cast()))
            && body != 0
        {
            let still_object = e.call(0x0045_8b20, &args![]).u32();
            let threshold: f32 = e.global(STILL_THRESHOLD);
            if e.call(0x0045_8900, &args![probe, still_object, threshold])
                .u32()
                != 0
            {
                motion = true;
                test_moving = false;
            }
        }
        if test_moving && !e.call(0x0056_4c10, &args![this]).bool() {
            motion = true;
        }
        if motion {
            e.call(0x00c6_a350, &args![model, 5u32, 1u32, 0u32, 1u32]);
        }
        e.mem.free(probe);
    }
    let base = e.call(GET_BASE_FORM, &args![this]).u32();
    let base_type = e.call(FORM_TYPE, &args![base]).i32();
    if (0x20..=0x21).contains(&base_type) {
        e.call(0x00c6_a350, &args![model, 5u32, 1u32, 0u32, 1u32]);
    } else if e.call(0x0056_4c10, &args![this]).bool() {
        let body = body_of(e, collision);
        let rigid = if body != 0 {
            e.call(0x0062_0b80, &args![body]).u32()
        } else {
            0
        };
        if rigid != 0 {
            let flags = e.call(0x009d_9f40, &args![rigid]).u32();
            if e.call(0x0051_7690, &args![flags]).i32() != 5 {
                let flags = e.call(0x009d_9f40, &args![rigid]).u32();
                if e.call(0x0051_7690, &args![flags]).i32() != 4 {
                    let velocity = e.mem.alloc(16);
                    let down: f32 = e.global(PUSH_VELOCITY);
                    e.call(0x0045_bb20, &args![HAVOK_VECTOR_HELPER, velocity, down]);
                    e.call(0x0062_b8d0, &args![model, velocity, 0u32]);
                    e.mem.free(velocity);
                }
            }
        }
        e.call(0x0056_4c60, &args![this, 0u32]);
    }
    e.call(0x0086_a830, &args![this, model]);
}

// Translated from 00576d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the form flag `0x200000` (`TESForm::iFormFlags`, +0x08) is set.
pub fn fn_00576d30(e: &mut Engine, this: Ptr) -> bool {
    // TESForm::iFormFlags +0x08
    e.mem.u32(this.addr() + 8) & 0x0020_0000 != 0
}

// Translated from 00576d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks a scene-graph object and its children (`this` is only handed on to
/// the recursion): converts each node's world
/// transform into a Havok transform and passes it to the collision shape's
/// two setters (`00ca4120` / `00ca36a0`, by shape type).
///
/// For a node with a collision object the rigid body's shape is looked up;
/// a shape of the first two types (descriptors `012682e8` / `012682dc`) is
/// used directly, one of the third type (`012682d0`) through the shape it
/// wraps (`00577190`).
pub fn fn_00576d50(e: &mut Engine, _unused_0: Ptr, node: Ptr) {
    if node.is_null() {
        return;
    }
    let n = node.addr();
    // a NiPoint3 local, built as (0, 1, 0) and then filled by the virtual
    // at +0xb8 (it writes the vector it is given)
    let direction = e.mem.alloc(16);
    e.call(0x0043_d410, &args![direction, 0.0f32, 1u32, 0u32]);
    e.vcall(n, 0xb8, &args![direction]);
    let transform = e.call(0x0046_1130, &args![node]).u32();
    let copy = e.mem.alloc(52);
    for word in 0..13 {
        let value = e.mem.u32(transform + word * 4);
        e.mem.set_u32(copy + word * 4, value);
    }
    let havok_transform = e.mem.alloc(72);
    e.call(0x0056_ded0, &args![havok_transform]);
    fn_00577070(e, Ptr::new(havok_transform));
    e.call(0x0056_df80, &args![havok_transform, copy]);

    let collision = e.call(0x0043_b610, &args![node]).u32();
    let body = body_of(e, collision);
    let shape = if body != 0 {
        e.call(0x004a_e6a0, &args![body]).u32()
    } else {
        0
    };
    if shape != 0 {
        let mut first_kind = 0u32;
        let mut second_kind = 0u32;
        if e.call(IS_KIND_OF, &args![0x0126_82e8u32, shape]).bool() {
            first_kind = e.call(0x0062_0b80, &args![shape]).u32();
        } else if e.call(IS_KIND_OF, &args![0x0126_82dcu32, shape]).bool() {
            second_kind = e.call(0x0062_0b80, &args![shape]).u32();
        } else if e.call(IS_KIND_OF, &args![0x0126_82d0u32, shape]).bool() {
            let inner = fn_00577190(e, Ptr::new(shape));
            if inner != 0 && e.call(IS_KIND_OF, &args![0x0126_82e8u32, inner]).bool() {
                first_kind = e.call(0x0062_0b80, &args![inner]).u32();
            } else if inner != 0 && e.call(IS_KIND_OF, &args![0x0126_82dcu32, inner]).bool() {
                second_kind = e.call(0x0062_0b80, &args![inner]).u32();
            }
        }
        if first_kind != 0 {
            e.call(0x00ca_4120, &args![first_kind, havok_transform]);
        }
        if second_kind != 0 {
            e.call(0x00ca_36a0, &args![second_kind, havok_transform]);
        }
    }
    e.mem.free(direction);
    e.mem.free(copy);
    e.mem.free(havok_transform);

    let children = e.vcall(n, SLOT_AS_NODE, &args![]).u32();
    if children != 0 {
        let count = child_count(e, children);
        for index in 0..count {
            let child = child_at(e, children, index);
            fn_00576d50(e, _unused_0, Ptr::new(child));
        }
    }
}

// Translated from 00577070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets a 72-byte Havok transform: identity rotation (`00577090`) and a
/// zero translation vector at +0x30.
pub fn fn_00577070(e: &mut Engine, this: Ptr) {
    fn_00577090(e, this);
    e.call(0x0053_8f40, &args![this.addr() + 0x30]);
}

// Translated from 00577090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the three rows of a 3x4 matrix at `this` to the identity: zeroes
/// them (`00577110`) and writes 1.0 on the diagonal.
pub fn fn_00577090(e: &mut Engine, this: Ptr) {
    fn_00577110(e, this);
    for k in 0..3u32 {
        let cell = fn_005770e0(e, this, k, k);
        e.mem.set_f32(cell, 1.0);
    }
}

// Translated from 005770e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Address of element (`column`, `row`) of a matrix of 16-byte rows:
/// `this + row * 16 + column * 4`.
pub fn fn_005770e0(e: &mut Engine, this: Ptr, column: u32, row: u32) -> u32 {
    let row_address = e.call(0x004b_4cf0, &args![this, row]).u32();
    e.call(0x0056_0d30, &args![row_address, column]).u32()
}

// Translated from 00577110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Zeroes the three rows (16 bytes each) of the matrix at `this`.
pub fn fn_00577110(e: &mut Engine, this: Ptr) {
    let zero = e.mem.alloc(16);
    e.call(0x0068_15c0, &args![zero]);
    e.call(0x0053_8f40, &args![zero]);
    for row in 0..3u32 {
        let row_address = e.call(0x004b_4cf0, &args![this, row]).u32();
        e.call(0x004a_3f10, &args![row_address, zero]);
    }
    e.mem.free(zero);
}

// Translated from 00577190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The shape wrapped by a collision shape: `005771f0` of it, converted by
/// `005771d0`, or 0.
pub fn fn_00577190(e: &mut Engine, this: Ptr) -> u32 {
    let inner = fn_005771f0(e, this);
    if inner == 0 {
        0
    } else {
        fn_005771d0(e, inner)
    }
}

// Translated from 005771d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cdecl` wrapper of `004a3a40(value)`.
pub fn fn_005771d0(e: &mut Engine, value: u32) -> u32 {
    e.call(0x004a_3a40, &args![value]).u32()
}

// Translated from 005771f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The object `00577190` works on: `004ae750(this)`, then its virtual at
/// +0x10, 0x10 bytes before the pointer it returns (0 when either is null).
pub fn fn_005771f0(e: &mut Engine, this: Ptr) -> u32 {
    let first = e.call(0x004a_e750, &args![this]).u32();
    if first == 0 {
        return 0;
    }
    let second = e.vcall(first, 0x10, &args![]).u32();
    if second == 0 {
        0
    } else {
        second - 0x10
    }
}

// Translated from 00577250 (decompiled, FalloutNV.exe 1.4.0.525)
/// A float derived from the reference's container (the extra data's
/// `00418520` list value, else `00481e10` of the container) and limited for
/// actors: a non-player actor is capped at `008a0c20`'s value, the player
/// gets `00577310` added.
pub fn fn_00577250(e: &mut Engine, this: Ptr, flag: u8) -> f32 {
    let mut value = 0.0f32;
    if e.call(0x0055_d310, &args![this]).u32() != 0 {
        let extra = e.call(GET_EXTRA_LIST, &args![this]).u32();
        let list = e.call(0x0041_8520, &args![extra]).u32();
        if list != 0 {
            value = e.call(0x004d_0900, &args![list, flag]).f32();
        } else {
            let container = e.call(0x0055_d310, &args![this]).u32();
            value = e.call(0x0048_1e10, &args![container, flag]).f32();
        }
    }
    if e.vcall(this.addr(), SLOT_IS_ACTOR, &args![]).bool() {
        let player = global_word(e, GLOBAL_PLAYER);
        if this.addr() != player {
            let cap = e.call(0x008a_0c20, &args![this]).f32();
            if value as f64 > cap as f64 {
                value = e.call(0x008a_0c20, &args![this]).f32();
            }
        } else {
            let bonus = fn_00577310(e, Ptr::new(player));
            value = (bonus as f64 + value as f64) as f32;
        }
    }
    value
}

// Translated from 00577310 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at +0xe04 of the object (the player character when called
/// from `00577250`).
pub fn fn_00577310(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0xe04)
}

// Translated from 00577330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::RestoreRagDollData` (Xbox PDB): when `0043fcd0(this)`
/// (the reference's 3D) is non-null, applies the ragdoll data `data` (taken
/// from the reference's extra data list when `data` is null) to the
/// reference (`004d9910`); without a 3D, stores `data` in the extra data list
/// (`0041d5b0`).
pub fn tes_object_refr_restore_rag_doll_data(e: &mut Engine, this: Ptr, data: u32) {
    if e.call(0x0043_fcd0, &args![this]).u32() != 0 {
        let mut data = data;
        if data == 0 {
            let extra = e.call(GET_EXTRA_LIST, &args![this]).u32();
            data = e.call(0x0041_d6d0, &args![extra]).u32();
        }
        if data != 0 {
            e.call(0x004d_9910, &args![data, this]);
        }
    } else if data != 0 {
        let extra = e.call(GET_EXTRA_LIST, &args![this]).u32();
        e.call(0x0041_d5b0, &args![extra, data]);
    }
}

// Translated from 00577390 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a reference whose extra data holds a primitive that `0056c7f0`
/// accepts: reads the primitive's 16-byte value (`00413f90`), sets its last
/// float to the `float` at `0101df44` (`enable`) or to 0.0, hands the value
/// back through the primitive's virtual at +8, and, unless the base form is
/// the one in `011ca240` and when the reference has a 3D, calls
/// `00450f90(3D, enable == 0)` on the 3D.
pub fn fn_00577390(e: &mut Engine, this: Ptr, enable: u8) {
    let extra = e.call(GET_EXTRA_LIST, &args![this]).u32();
    let primitive = e.call(0x0041_fbe0, &args![extra]).u32();
    if primitive == 0 || e.call(0x0056_c7f0, &args![primitive]).u32() == 0 {
        return;
    }
    let class_id = e.mem.alloc(16);
    e.call(0x0041_3f90, &args![primitive, class_id]);
    let value: f32 = if enable != 0 {
        e.global(0x0101_df44)
    } else {
        0.0
    };
    // the last four bytes of the 16-byte ID
    e.mem.set_f32(class_id + 12, value);
    e.vcall(primitive, 8, &args![class_id]);
    e.mem.free(class_id);
    if e.call(GET_BASE_FORM, &args![this]).u32() != global_word(e, GLOBAL_BASE_FORM_A)
        && e.vcall(this.addr(), SLOT_GET_3D, &args![]).u32() != 0
    {
        let model = e.vcall(this.addr(), SLOT_GET_3D, &args![]).u32();
        e.call(0x0045_0f90, &args![model, (enable == 0) as u32]);
    }
}

// Translated from 00577450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::TransChangeCallback` (Xbox PDB): the scene-graph node
/// `node` of a reference has moved (`flags & 1`) or rotated (`flags & 2`);
/// brings the reference in line.
///
/// With `flags & 1` it checks the new position: not finite, or below the
/// terrain minimum (exterior) or the cell's bound (interior), means the
/// reference fell out of the world, and it is put back at the position
/// the cell's placement info (`0054cfd0`) gives (or its Havok body is frozen
/// when it moves too fast). It then writes the position back to the
/// reference (`00575830`), sinks water-bound references' addon nodes, and,
/// outdoors, moves the reference to the cell of its new coordinates.
/// With `flags & 2` it converts the node's rotation matrix to euler angles
/// and writes them to the reference (`00575700`). It always finishes by
/// refreshing the 3D's lighting.
pub fn tes_object_refr_trans_change_callback(e: &mut Engine, node: Ptr, flags: u32) {
    let node_owner = e.call(0x0044_ddc0, &args![node]).u32();
    let refr = e.call(0x0056_f930, &args![node_owner]).u32();
    if refr == 0 {
        return;
    }
    let model = e.vcall(refr, SLOT_GET_3D, &args![]).u32();
    // the result (a ragdoll test) is not used
    e.call(0x0062_c3c0, &args![refr, model]);
    let scratch = e.mem.alloc(0x100);
    if flags & 1 != 0 {
        position_changed(e, node, node_owner, refr, scratch);
    }
    if flags & 2 != 0 {
        rotation_changed(e, node_owner, refr, scratch);
    }
    let model = e.vcall(refr, SLOT_GET_3D, &args![]).u32();
    let lighting = e.call(0x0045_0b80, &args![0u32]).u32();
    e.call(0x00b5_d9f0, &args![lighting, model, 1u32]);
    e.mem.free(scratch);
}

/// `flags & 1` half of [`tes_object_refr_trans_change_callback`]. `scratch`
/// is a block of 0x100 bytes for the function's stack locals.
fn position_changed(e: &mut Engine, node: Ptr, node_owner: u32, refr: u32, scratch: u32) {
    let cell = e.call(GET_PARENT_CELL, &args![refr]).u32();
    if cell != 0 {
        let (fell_out, below_both) = check_fell_out(e, node_owner, refr, cell, scratch);
        if fell_out {
            if below_both {
                // put the reference back at the cell's placement point
                let point = scratch + 0x30;
                for word in 0..3u32 {
                    let value = e.mem.u32(ZERO_VECTOR + word * 4);
                    e.mem.set_u32(point + word * 4, value);
                }
                let info = scratch + 0x40;
                e.call(0x0068_15c0, &args![info]);
                e.call(0x0054_cfd0, &args![cell, point, info]);
                let [x, y, z] = [e.mem.u32(point), e.mem.u32(point + 4), e.mem.u32(point + 8)];
                e.vcall(refr, 0x174, &args![x, y, z]);
            }
            let model = e.vcall(refr, SLOT_GET_3D, &args![]).u32();
            if model != 0 {
                let position = e
                    .vcall(refr, SLOT_GET_POSITION, &args![scratch + 0x50])
                    .u32();
                e.call(0x0044_0460, &args![model, position]);
                e.call(0x0043_fa80, &args![model, 0x011a_9448u32]);
            }
            e.call(0x00c6_bd00, &args![model, 1u32]);
            let direction = scratch + 0x60;
            e.call(0x0043_d410, &args![direction, 0.0f32, 1u32, 0u32]);
            e.call(0x00a5_9c60, &args![model, direction]);
            let body = e.call(0x006f_a820, &args![node]).u32();
            if body != 0 {
                let velocity = bhk_rigid_body_gethk_max_linear_velocity(e, Ptr::new(body));
                let scaled = (velocity as f64 * e.global::<f64>(VELOCITY_SCALE)) as f32;
                if scaled as f64 > e.global::<f64>(VELOCITY_LIMIT) {
                    fn_00577c40(e, Ptr::new(body), scaled);
                } else {
                    e.vcall(node.addr(), 0xb0, &args![4u32, 0u32, 0u32]);
                    e.call(0x0057_3ed0, &args![node, 0u32]);
                    let form_id = e.call(0x0084_e3a0, &args![refr]).u32();
                    let name = e.vcall(refr, 0x130, &args![]).u32();
                    e.call(MESSAGE, &args![DISABLE_COLLISION_MESSAGE, name, form_id]);
                }
            }
        }
    }

    // the reference follows the node: put the position back through the
    // reference's setter with the flag object's flag cleared
    let position = e.vcall(refr, SLOT_POSITION_PTR, &args![]).u32();
    let saved = scratch + 0x70;
    for word in 0..3u32 {
        let value = e.mem.u32(position + word * 4);
        e.mem.set_u32(saved + word * 4, value);
    }
    let flag_object = global_word(e, GLOBAL_FLAG_OBJECT);
    let old_flag = e.call(SWAP_FLAG, &args![flag_object, 0u32]).u8();
    let node_position = e.call(0x0045_bb80, &args![node_owner]).u32();
    e.call(SET_POSITION, &args![refr, node_position]);
    e.call(SWAP_FLAG, &args![flag_object, old_flag]);
    e.vcall(refr, 0x48, &args![4u32]);

    let base = e.call(GET_BASE_FORM, &args![refr]).u32();
    if base != 0 {
        let base = e.call(GET_BASE_FORM, &args![refr]).u32();
        if e.call(FORM_TYPE, &args![base]).u32() == 0x1e
            && !e.call(0x0044_8a20, &args![refr]).bool()
        {
            let position = e.vcall(refr, SLOT_POSITION_PTR, &args![]).u32();
            let water = e.call(0x0057_b0a0, &args![refr]).f32();
            if e.mem.f32(position + 8) < water {
                e.call(0x0048_4a70, &args![refr, 1u32]);
                let model = e.vcall(refr, SLOT_GET_3D, &args![]).u32();
                tes_object_refr_remove_addon_nodes(e, model);
                e.call(0x0057_29e0, &args![refr, 0u32]);
            }
        }
    }

    let tes = global_word(e, GLOBAL_TES);
    if e.call(0x005f_36f0, &args![tes]).u32() != 0 {
        return;
    }
    // outdoors: the reference may have crossed into another cell
    let world_space = e.call(0x004f_d3e0, &args![tes]).u32();
    let mut actor = 0;
    if refr != 0 && e.vcall(refr, SLOT_IS_ACTOR, &args![]).bool() {
        actor = refr;
    }
    if actor != 0 {
        e.call(0x008a_e640, &args![actor, 0u32]);
    }
    let at = e.vcall(refr, SLOT_POSITION_PTR, &args![]).u32();
    let y = e.mem.f32(at + 4);
    let at = e.vcall(refr, SLOT_POSITION_PTR, &args![]).u32();
    let x = e.mem.f32(at);
    let handler = global_word(e, GLOBAL_DATA_HANDLER);
    let new_cell = e
        .call(0x0046_1bc0, &args![handler, x, y, world_space, 0u32])
        .u32();
    if new_cell == cell {
        return;
    }
    if new_cell == 0 {
        // the new coordinates are in no cell: restore the saved position
        let old_flag = e.call(SWAP_FLAG, &args![flag_object, 0u32]).u8();
        e.call(SET_POSITION, &args![refr, saved]);
        e.call(SWAP_FLAG, &args![flag_object, old_flag]);
        e.call(0x0044_0460, &args![node_owner, saved]);
        e.vcall(node.addr(), 0xb0, &args![4u32, 0u32, 0u32]);
        return;
    }
    e.call(0x0057_3800, &args![refr, 0u32, world_space]);
    if !e.vcall(refr, SLOT_IS_ACTOR, &args![]).bool()
        && e.vcall(refr, SLOT_GET_3D, &args![]).u32() != 0
    {
        let attach = e.call(0x0054_97a0, &args![new_cell]).u32();
        if attach != 0 {
            if !e.call(0x008c_7aa0, &args![]).bool() {
                e.call(0x0054_96b0, &args![new_cell, refr, attach]);
            } else {
                e.call(0x0054_abd0, &args![refr, attach]);
            }
        }
    }
}

/// The position-validity tests of `TransChangeCallback`: returns (the
/// reference left the world, it is also below the world at its saved
/// position). `scratch` holds the stack locals.
fn check_fell_out(
    e: &mut Engine,
    node_owner: u32,
    refr: u32,
    cell: u32,
    scratch: u32,
) -> (bool, bool) {
    let position = e.call(0x0045_bb80, &args![node_owner]).u32();
    let [x, y, z] = [
        e.mem.f32(position),
        e.mem.f32(position + 4),
        e.mem.f32(position + 8),
    ];
    if !crt_finite(e, x)
        || !crt_finite(e, y)
        || !crt_finite(e, z)
        || crt_isnan(e, x)
        || crt_isnan(e, y)
        || crt_isnan(e, z)
    {
        return (true, false);
    }
    let mut fell_out = false;
    let mut below_both = false;
    if e.call(0x0042_5fd0, &args![cell]).bool() {
        // interior cell: compare with the cell's bounding sphere
        let bounds_node = e.call(0x0045_6fc0, &args![cell, 3u32]).u32();
        if bounds_node != 0 {
            let source = e.call(0x0043_d450, &args![bounds_node]).u32();
            let bound = scratch;
            for word in 0..4u32 {
                let value = e.mem.u32(source + word * 4);
                e.mem.set_u32(bound + word * 4, value);
            }
            let me = e.call(0x0068_15c0, &args![bound]).u32();
            if e.call(0x0043_9090, &args![me, ZERO_VECTOR]).bool() {
                let me = e.call(0x0068_15c0, &args![bound]).u32();
                let current = e.call(0x0045_bb80, &args![node_owner]).u32();
                let drop = e.mem.f32(me + 8) as f64 - e.mem.f32(current + 8) as f64;
                let radius = e.call(0x0084_d030, &args![bound]).f32();
                if (radius as f64) < drop {
                    fell_out = true;
                    let me = e.call(0x0068_15c0, &args![bound]).u32();
                    let saved = e
                        .vcall(refr, SLOT_GET_POSITION, &args![scratch + 0x10])
                        .u32();
                    let drop = e.mem.f32(me + 8) as f64 - e.mem.f32(saved + 8) as f64;
                    let radius = e.call(0x0084_d030, &args![bound]).f32();
                    if (radius as f64) < drop {
                        below_both = true;
                    }
                }
            }
        }
    } else if e.call(0x0054_6fb0, &args![cell]).u32() != 0 {
        // exterior: compare with the land's lowest height minus 128
        let land = e.call(0x0054_6fb0, &args![cell]).u32();
        let lowest = e.call(0x0053_f440, &args![land, scratch + 0x20]).u32();
        let limit = (e.mem.f32(lowest) as f64 - e.global::<f64>(GROUND_MARGIN)) as f32;
        let current = e.call(0x0045_bb80, &args![node_owner]).u32();
        if e.mem.f32(current + 8) < limit {
            fell_out = true;
            let saved = e
                .vcall(refr, SLOT_GET_POSITION, &args![scratch + 0x10])
                .u32();
            if e.mem.f32(saved + 8) < limit {
                below_both = true;
            }
        }
    }
    (fell_out, below_both)
}

/// `flags & 2` half of [`tes_object_refr_trans_change_callback`].
fn rotation_changed(e: &mut Engine, node_owner: u32, refr: u32, scratch: u32) {
    let angles = scratch + 0x80;
    e.call(0x0068_15c0, &args![angles]);
    let matrix = e.call(0x0046_1130, &args![node_owner]).u32();
    e.call(0x00a5_92c0, &args![matrix, angles, angles + 4, angles + 8]);
    let flag_object = global_word(e, GLOBAL_FLAG_OBJECT);
    let old_flag = e.call(SWAP_FLAG, &args![flag_object, 0u32]).u8();
    let [x, y, z] = [
        e.mem.u32(angles),
        e.mem.u32(angles + 4),
        e.mem.u32(angles + 8),
    ];
    e.call(SET_ROTATION, &args![refr, x, y, z]);
    e.call(SWAP_FLAG, &args![flag_object, old_flag]);
}

// Translated from 00577b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRigidBody::GethkMaxLinearVelocity` (Xbox PDB): the maximum linear
/// velocity of the body's `004ae750` object, a `float` looked up in the
/// table at `010c7948` by the `hkUFloat8` byte stored at +0xac of its
/// rigid-body info; 0.0 when there is no such object.
///
/// When the object has a body-info object (`00ca86c0`), the byte is read
/// in place; otherwise the byte is built from `00577c10`'s velocity with
/// `00ca9360` in a temporary.
pub fn bhk_rigid_body_gethk_max_linear_velocity(e: &mut Engine, this: Ptr) -> f32 {
    let object = e.call(0x004a_e750, &args![this]).u32();
    if object == 0 {
        return 0.0;
    }
    let info = e.call(0x00ca_86c0, &args![object]).u32();
    if info != 0 {
        let rigid = e.call(0x0046_0140, &args![info]).u32();
        fn_00577bf0(e, Ptr::new(rigid + 0xac))
    } else {
        let velocity = fn_00577c10(e, Ptr::new(object));
        let temporary = e.mem.alloc(4);
        let byte_cell = fn_00577bd0(e, Ptr::new(temporary), velocity);
        let result = fn_00577bf0(e, Ptr::new(byte_cell));
        e.mem.free(temporary);
        result
    }
}

// Translated from 00577bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the `hkUFloat8` at `this` (`00ca9360`, which takes the
/// address of the float) and returns `this`.
pub fn fn_00577bd0(e: &mut Engine, this: Ptr, value: f32) -> u32 {
    let argument = e.mem.alloc(4);
    e.mem.set_f32(argument, value);
    e.call(0x00ca_9360, &args![this, argument]);
    e.mem.free(argument);
    this.addr()
}

// Translated from 00577bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` of the `hkUFloat8` at `this`: the table entry for its byte.
pub fn fn_00577bf0(e: &mut Engine, this: Ptr) -> f32 {
    let index = e.mem.u8(this.addr()) as u32;
    e.global(UFLOAT8_TABLE + index * 4)
}

// Translated from 00577c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The velocity table entry of the `hkUFloat8` at +0xac of the info object
/// `00460140(009d9f40(this))` gives.
pub fn fn_00577c10(e: &mut Engine, this: Ptr) -> f32 {
    let body = e.call(0x009d_9f40, &args![this]).u32();
    let info = e.call(0x0046_0140, &args![body]).u32();
    fn_00577bf0(e, Ptr::new(info + 0xac))
}

// Translated from 00577c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `hkUFloat8` of `004ae750(this)` from `value` (`00577c70`), if it
/// has such an object.
pub fn fn_00577c40(e: &mut Engine, this: Ptr, value: f32) {
    let object = e.call(0x004a_e750, &args![this]).u32();
    if object != 0 {
        fn_00577c70(e, Ptr::new(object), value);
    }
}

// Translated from 00577c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` into the `hkUFloat8` at +0xac of the info object
/// `00460140(009d9f40(this))` gives (`00ca9360`, which takes the float's
/// address).
pub fn fn_00577c70(e: &mut Engine, this: Ptr, value: f32) {
    let body = e.call(0x009d_9f40, &args![this]).u32();
    let info = e.call(0x0046_0140, &args![body]).u32();
    let argument = e.mem.alloc(4);
    e.mem.set_f32(argument, value);
    e.call(0x00ca_9360, &args![info + 0xac, argument]);
    e.mem.free(argument);
}

// Translated from 00577ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::DebugDisplayCallback` (Xbox PDB): for a non-null node,
/// allocates a 0x80-byte debug-display object (`00aa13e0`, built by
/// `00b6fc90`), sizes it for 0x30 entries (`00441130`), attaches it to the
/// node (`00439410`) and finishes with `00b57e30(node, 0, 0)`. The
/// exception-unwinding frame of the allocation is not translated.
pub fn tes_object_refr_debug_display_callback(e: &mut Engine, node: Ptr) {
    if node.is_null() {
        return;
    }
    let memory = e.call(0x00aa_13e0, &args![0x80u32]).u32();
    let object = if memory != 0 {
        e.call(0x00b6_fc90, &args![memory]).u32()
    } else {
        0
    };
    e.call(0x0044_1130, &args![object, 0x30u32, 1u32]);
    e.call(0x0043_9410, &args![node, object]);
    e.call(0x00b5_7e30, &args![node, 0u32, 0u32]);
}

// Translated from 00577d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a reference with a container (`0055d310`): takes the object
/// `004bf220(this)` gives, passes it and the result of `004839c0(0xf)` to
/// `004c8f30` when that result is non-null, and, when `0042cde0` of the
/// object is true, calls `0041aeb0` on the reference's extra data list.
/// Returns what `004c8f30` returned (0 without a container or block).
pub fn fn_00577d50(e: &mut Engine, this: Ptr) -> u32 {
    let mut result = 0;
    if e.call(0x0055_d310, &args![this]).u32() != 0 {
        let owner = e.call(0x004b_f220, &args![this]).u32();
        let block = e.call(0x0048_39c0, &args![0x0fu32]).u32();
        if block != 0 {
            result = e.call(0x004c_8f30, &args![owner, block]).u32();
        }
        if e.call(0x0042_cde0, &args![owner]).bool() {
            let extra = e.call(GET_EXTRA_LIST, &args![this]).u32();
            e.call(0x0041_aeb0, &args![extra]);
        }
    }
    result
}

// Translated from 00577de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a reference with a container (`0055d310`): `004d0360` of the
/// container's owner object (`004bf220`); false without a container.
pub fn fn_00577de0(e: &mut Engine, this: Ptr) -> bool {
    if e.call(0x0055_d310, &args![this]).u32() == 0 {
        return false;
    }
    let owner = e.call(0x004b_f220, &args![this]).u32();
    e.call(0x004d_0360, &args![owner]).u8() != 0
}

// Translated from 00577e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::AddAddonNodes` (Xbox PDB): walks the scene graph under
/// `node` and attaches the addon node the data handler holds for it.
///
/// For a node: every child whose name starts with `"BASE"` (and whose
/// `0048af70` test passes) is detached (virtual at +0xf0 with its index).
/// Then the node's addon record (`004617e0`) is attached (virtual at +0xdc)
/// when it has a model and `00448bf0` is false, after being randomized with
/// `RandomizeAddons` and a seed taken from `00487f50() % 1000 / 100.0`.
/// The children are then processed recursively; when anything was added,
/// `004902f0(node, 1)` and `00576660(node, 0)` are called. Returns whether
/// anything was added.
pub fn tes_object_refr_add_addon_nodes(e: &mut Engine, node: u32) -> bool {
    if node == 0 {
        return false;
    }
    let mut added = false;
    if is_node(e, node) {
        let mut index = 0;
        while index < child_count(e, node) {
            let child = child_at(e, node, index);
            let child_as_node = child_node(e, child);
            if child_as_node != 0 {
                let name_holder = e.call(0x0041_3f40, &args![child_as_node]).u32();
                if e.call(0x0048_af70, &args![name_holder]).bool() {
                    let name_holder = e.call(0x0041_3f40, &args![child_as_node]).u32();
                    let name = e.call(0x0043_b1b0, &args![name_holder]).u32();
                    if e.call(STRNCMP, &args![name, BASE_PREFIX, 4u32]).i32() == 0 {
                        e.vcall(child_as_node, 0xf0, &args![index]);
                    }
                }
            }
            index += 1;
        }
        let addon = addon_for_node(e, node);
        if addon != 0
            && e.call(0x0048_cee0, &args![addon + 0x30]).u32() != 0
            && !e.call(0x0044_8bf0, &args![addon]).bool()
        {
            let created = e.vcall(addon, 0x178, &args![0u32]).u32();
            if created != 0 {
                let created_node = e.vcall(created, 0x10, &args![]).u32();
                if created_node != 0 {
                    e.call(0x0056_c7d0, &args![created_node, 0u32]);
                }
                let tick = e.call(0x0048_7f50, &args![]).u32();
                let seed = ((tick % 1000) as f64 / e.global::<f64>(RANDOM_DIVISOR)) as f32;
                fn_005784f0(e, created, seed);
                e.vcall(node, 0xdc, &args![created, 1u32]);
                added = true;
            }
        }
    }
    let mut index = 0;
    while index < child_count(e, node) {
        let child = child_at(e, node, index);
        let child_as_node = child_node(e, child);
        if child_as_node != 0 {
            added |= tes_object_refr_add_addon_nodes(e, child_as_node);
        }
        index += 1;
    }
    if added {
        e.call(0x0049_02f0, &args![node, 1u32]);
        e.call(0x0057_6660, &args![node, 0u32]);
    }
    added
}

// Translated from 00578060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::AddMasterParticleAddonNodes` (Xbox PDB): for a node whose
/// addon record (`004617e0`) satisfies `00448bf0`, calls `00450f90(node, 0)`
/// and `00c505c0` with the record's `+0x5c` value (`0059e300`) and the node,
/// on the object `0045a190` returns. Then, unless `00456610(node)`, it does
/// the same for every child. Returns whether anything was done.
pub fn tes_object_refr_add_master_particle_addon_nodes(e: &mut Engine, node: u32) -> bool {
    if node == 0 {
        return false;
    }
    let mut done = false;
    if is_node(e, node) {
        let addon = addon_for_node(e, node);
        if addon != 0 && e.call(0x0044_8bf0, &args![addon]).bool() {
            e.call(0x0045_0f90, &args![node, 0u32]);
            let value = e.call(0x0059_e300, &args![addon]).u32();
            let target = e.call(0x0045_a190, &args![]).u32();
            e.call(0x00c5_05c0, &args![target, value, node]);
            done = true;
        }
    }
    if e.call(0x0045_6610, &args![node]).bool() {
        return done;
    }
    let mut index = 0;
    while index < child_count(e, node) {
        let child = child_at(e, node, index);
        let child_as_node = child_node(e, child);
        if child_as_node != 0 {
            done |= tes_object_refr_add_master_particle_addon_nodes(e, child_as_node);
        }
        index += 1;
    }
    done
}

// Translated from 00578170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::RemoveMasterParticleAddonNodes` (Xbox PDB): for a node
/// whose addon record satisfies `00448bf0`, calls `00450f90(node, 1)`, and
/// recurses over the children. Returns whether anything was done.
pub fn tes_object_refr_remove_master_particle_addon_nodes(e: &mut Engine, node: u32) -> bool {
    if node == 0 {
        return false;
    }
    let mut done = false;
    if is_node(e, node) {
        let addon = addon_for_node(e, node);
        if addon != 0 && e.call(0x0044_8bf0, &args![addon]).bool() {
            e.call(0x0045_0f90, &args![node, 1u32]);
            done = true;
        }
    }
    let mut index = 0;
    while index < child_count(e, node) {
        let child = child_at(e, node, index);
        let child_as_node = child_node(e, child);
        if child_as_node != 0 {
            done |= tes_object_refr_remove_master_particle_addon_nodes(e, child_as_node);
        }
        index += 1;
    }
    done
}

// Translated from 00578250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks `node` and its children: for each node `00570ea0` accepts, calls
/// `00ad8570(manager, node, 0.0, 0)` with the object `00453a70` returns.
/// Returns whether the root node itself was accepted.
pub fn fn_00578250(e: &mut Engine, node: u32) -> bool {
    if node == 0 {
        return false;
    }
    let mut accepted = false;
    if e.call(0x0057_0ea0, &args![node]).bool() {
        let manager = e.call(0x0045_3a70, &args![]).u32();
        e.call(0x00ad_8570, &args![manager, node, 0.0f32, 0u32]);
        accepted = true;
    }
    let mut index = 0;
    while index < child_count(e, node) {
        let child = child_at(e, node, index);
        let child_as_node = child_node(e, child);
        if child_as_node != 0 {
            fn_00578250(e, child_as_node);
        }
        index += 1;
    }
    accepted
}

// Translated from 00578300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::RemoveAddonNodes` (Xbox PDB): for a node, calls
/// `00c69ee0(node, 1, 1)`; if its addon record has a model (`0048cee0` of
/// `record + 0x30`) and `00453470(node)` is non-zero, tells the record
/// (virtual at +0x150) and calls `00572160(node)`. Recurses over the
/// children. Returns whether the record was told.
pub fn tes_object_refr_remove_addon_nodes(e: &mut Engine, node: u32) -> bool {
    if node == 0 {
        return false;
    }
    let mut removed = false;
    if is_node(e, node) {
        e.call(0x00c6_9ee0, &args![node, 1u32, 1u32]);
        let addon = addon_for_node(e, node);
        if addon != 0
            && e.call(0x0048_cee0, &args![addon + 0x30]).u32() != 0
            && e.call(0x0045_3470, &args![node]).u32() != 0
        {
            e.vcall(addon, 0x150, &args![0u32]);
            e.call(0x0057_2160, &args![node]);
            removed = true;
        }
    }
    let mut index = 0;
    while index < child_count(e, node) {
        let child = child_at(e, node, index);
        let child_as_node = child_node(e, child);
        if child_as_node != 0 {
            tes_object_refr_remove_addon_nodes(e, child_as_node);
        }
        index += 1;
    }
    removed
}

// Translated from 00578400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::HasAddonNodes` (Xbox PDB): whether the node, or any node
/// below it, passes `00453470`.
pub fn tes_object_refr_has_addon_nodes(e: &mut Engine, node: u32) -> bool {
    if node == 0 {
        return false;
    }
    if is_node(e, node) && e.call(0x0045_3470, &args![node]).u32() != 0 {
        return true;
    }
    let mut index = 0;
    while index < child_count(e, node) {
        let child = child_at(e, node, index);
        let child_as_node = child_node(e, child);
        if child_as_node != 0 && tes_object_refr_has_addon_nodes(e, child_as_node) {
            return true;
        }
        index += 1;
    }
    false
}

// Translated from 005784b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::HasAddonFlags` (Xbox PDB): whether the object has an
/// extra-data entry (`00a5bdd0`) of the kind `00448a80` returns and
/// `00448a40` accepts it.
pub fn tes_object_refr_has_addon_flags(e: &mut Engine, object: u32) -> bool {
    let kind = e.call(0x0044_8a80, &args![]).u32();
    let entry = e.call(0x00a5_bdd0, &args![object, kind]).u32();
    entry != 0 && e.call(0x0044_8a40, &args![entry]).bool()
}

// Translated from 005784f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::RandomizeAddons` (Xbox PDB): applies `value` to every
/// entry of the chain of `node`'s first list (`0043b230`, stepping with
/// `004a8a90`, applying `00621b20`), then to the chain of the list its
/// `00a59d30(node, 5)` entry has, then recurses over the children.
pub fn fn_005784f0(e: &mut Engine, node: u32, value: f32) {
    if node == 0 {
        return;
    }
    let mut entry = e.call(0x0043_b230, &args![node]).u32();
    while entry != 0 {
        e.call(0x0062_1b20, &args![entry, value]);
        entry = e.call(0x004a_8a90, &args![entry]).u32();
    }
    let second = e.call(0x00a5_9d30, &args![node, 5u32]).u32();
    entry = if second != 0 {
        e.call(0x0043_b230, &args![second]).u32()
    } else {
        0
    };
    while entry != 0 {
        e.call(0x0062_1b20, &args![entry, value]);
        entry = e.call(0x004a_8a90, &args![entry]).u32();
    }
    let children = e.vcall(node, SLOT_AS_NODE, &args![]).u32();
    if children != 0 {
        let mut index = 0;
        while index < child_count(e, children) {
            let child = child_at(e, children, index);
            fn_005784f0(e, child, value);
            index += 1;
        }
    }
}

// Translated from 005785e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::IsAnOwner` (Xbox PDB): whether `actor` counts as an owner
/// of this reference.
///
/// The reference's owner (`00567790`) is a form; without one everybody
/// owns it. An `actor` that is the owner itself (compared with its
/// `+0x1a4` form, else its base form) owns it. An owner of form type 8
/// (an NPC) that `00578770` marks (flag `0x200` at +0x34) makes the
/// `check_base` parameter true; with an NPC owner and `check_base`, the
/// actor owns the reference when `008b8e90(actor, owner)` and the actor's
/// base is of form type 0x2a/0x2b. A non-NPC owner (or one without the
/// flag) never matches here (the code only compares its global's value
/// with `01012060`, which cannot change the result).
pub fn tes_object_refr_is_an_owner(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    mut check_base: bool,
) -> bool {
    let mut result = false;
    let mut owner = 0;
    let mut owner_global = 0;
    if e.call(0x0056_7790, &args![this]).u32() != 0 {
        owner = e.call(0x0056_7790, &args![this]).u32();
        // the rank is read (and not used further)
        e.call(0x0056_79f0, &args![this]);
        owner_global = e.call(0x0056_7960, &args![this]).u32();
    }
    if owner != 0 && actor != 0 {
        let mut actor_form = e.vcall(actor, 0x1a4, &args![]).u32();
        if actor_form == 0 {
            actor_form = e.call(GET_BASE_FORM, &args![actor]).u32();
        }
        let mut as_bound_object = 0;
        let form_type = e.call(FORM_TYPE, &args![actor_form]).i32();
        if (0x2a..=0x2b).contains(&form_type) {
            as_bound_object = e
                .call(
                    DYNAMIC_CAST,
                    &args![actor_form, 0u32, 0x0118_3108u32, 0x0118_46e8u32, 0u32],
                )
                .u32();
        }
        if owner == actor_form {
            result = true;
        } else {
            let mut npc_owner = 0;
            if owner != 0 && e.call(FORM_TYPE, &args![owner]).u32() == FORM_TYPE_NPC_LIKE {
                npc_owner = owner;
            }
            if npc_owner != 0 && fn_00578770(e, Ptr::new(npc_owner)) {
                check_base = true;
            }
            if npc_owner == 0 || !check_base {
                if owner_global != 0 {
                    // the global's value is compared with a constant and 0
                    // is stored into the result when they are equal (it is
                    // already false)
                    let value = e.call(0x0052_6ac0, &args![owner_global]).f64();
                    if value == e.global::<f64>(OWNERSHIP_VALUE_LIMIT) {
                        result = false;
                    }
                } else {
                    result = false;
                }
            } else if as_bound_object != 0 && e.call(0x008b_8e90, &args![actor, npc_owner]).bool() {
                result = true;
            }
        }
    } else if owner == 0 {
        result = true;
    }
    result
}

// Translated from 00578770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag `0x200` of the dword at +0x34 is set.
pub fn fn_00578770(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x34) & 0x200 != 0
}

// Translated from 00578790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::IsOwnerEvil` (Xbox PDB): whether the owner of this
/// reference (`owner`, or `00567790` of the reference when null) is evil:
/// an NPC owner (form type 8) is evil when `0047d7c0` says so; an owner of
/// form type 0x2a is evil when it has no actor in the process lists
/// (`00970a20`) or that actor's value 0x17 (virtual at +0xc of the object at
/// +0x100 of its base) maps to 2 or 4 through `0047e040`. Other owners are
/// not evil.
pub fn tes_object_refr_is_owner_evil(e: &mut Engine, this: Ptr, owner: u32) -> bool {
    let mut evil = false;
    let owner = if owner == 0 {
        e.call(0x0056_7790, &args![this]).u32()
    } else {
        owner
    };
    if owner == 0 {
        return false;
    }
    let owner_type = e.call(FORM_TYPE, &args![owner]).u32();
    if owner_type == FORM_TYPE_NPC_LIKE {
        if e.call(0x0047_d7c0, &args![owner]).bool() {
            evil = true;
        }
    } else if e.call(FORM_TYPE, &args![owner]).u32() == FORM_TYPE_FACTION_LIKE {
        let process_lists = 0x011e_0e80u32;
        let actor_ref = e
            .call(0x0097_0a20, &args![process_lists, owner, 1u32])
            .u32();
        evil = true;
        if actor_ref != 0 {
            let base = e.call(0x0041_81e0, &args![actor_ref]).u32();
            let values = base + 0x100;
            let value = e.vcall(values, 0xc, &args![0x17u32]).f32();
            let class = e.call(0x0047_e040, &args![value]).i32();
            if class != 2 && class != 4 {
                evil = false;
            }
        }
    }
    evil
}

// Translated from 00578870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the name of the reference's location into the string `result`: the
/// parent cell's name (`00408da0(cell + 0x18)`) when the reference is in an
/// interior cell (`00425fd0`), else what the worldspace-side object
/// (`00575d70`) formats for the position (virtual at +0x138), else the empty
/// string. The string is set with `004037f0(result, text, 0)`.
pub fn fn_00578870(e: &mut Engine, this: Ptr, result: Ptr) {
    let mut cell = e.call(GET_PARENT_CELL, &args![this]).u32();
    if cell != 0 && !e.call(0x0042_5fd0, &args![cell]).bool() {
        cell = 0;
    }
    if cell != 0 {
        let name = e.call(0x0040_8da0, &args![cell + 0x18]).u32();
        e.call(0x0040_37f0, &args![result, name, 0u32]);
        return;
    }
    let place = e.call(0x0057_5d70, &args![this]).u32();
    if place != 0 {
        let [x, y, z] = {
            let position = e.vcall(this.addr(), SLOT_POSITION_PTR, &args![]).u32();
            [
                e.mem.u32(position),
                e.mem.u32(position + 4),
                e.mem.u32(position + 8),
            ]
        };
        e.vcall(place, 0x138, &args![result, x, y, z]);
    } else {
        e.call(0x0040_37f0, &args![result, EMPTY_STRING, 0u32]);
    }
}

// Translated from 00578920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00c6bd00(3D, flag)` on the reference's 3D (virtual at +0x1d0).
pub fn fn_00578920(e: &mut Engine, this: Ptr, flag: u8) {
    let model = e.vcall(this.addr(), SLOT_GET_3D, &args![]).u32();
    e.call(0x00c6_bd00, &args![model, flag]);
}

// Translated from 00578950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::AddTalkingActivatorMobileObjectREFR` (Xbox PDB):
/// `004ff070` (`BGSTalkingActivator::AddMobileObjectExtra`) on the base form
/// with this reference; returns its result.
pub fn tes_object_refr_add_talking_activator_mobile_object_refr(e: &mut Engine, this: Ptr) -> u32 {
    let base = e.call(GET_BASE_FORM, &args![this]).u32();
    e.call(0x004f_f070, &args![base, this]).u32()
}

// Translated from 00578970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetMobileObjectExtra` (Xbox PDB): the dword at +0xc of
/// the talking-actor extra (`0042e110`) of the reference's extra data list,
/// or 0.
pub fn tes_object_refr_get_mobile_object_extra(e: &mut Engine, this: Ptr) -> u32 {
    let mut result = 0;
    let extra = e.call(GET_EXTRA_LIST, &args![this]).u32();
    if e.call(0x0042_e110, &args![extra]).u32() != 0 {
        let extra = e.call(GET_EXTRA_LIST, &args![this]).u32();
        let talking = e.call(0x0042_e110, &args![extra]).u32();
        result = e.mem.u32(talking + 0xc);
    }
    result
}

// ---- batch 005789b0 .. 0057b410 -------------------------------------------
//
// Constants and helpers of the second batch (the first batch's are above).

/// Virtual at +0x48 of a form; called with a change-flags word.
const SLOT_MARK_CHANGED: u32 = 0x48;
/// Change-flag word the lock change passes.
const CHANGE_FLAG_LOCK: u32 = 0x1000;
/// Change-flag word the say-to setters pass.
const CHANGE_FLAG_SAY_TO: u32 = 0x8000_0000;
/// `ExtraDataList::GetLock` (Xbox PDB).
const EXTRA_GET_LOCK: u32 = 0x0056_9140;
/// `0x00568e50(this)`: the reference's teleport data (null when none).
const GET_TELEPORT_DATA: u32 = 0x0056_8e50;
/// `ExtraDataList::GetLastFinishedSequence` (Xbox PDB).
const EXTRA_GET_LAST_FINISHED_SEQUENCE: u32 = 0x0042_28f0;
/// Stores the sequence in the extra data list.
const EXTRA_SET_LAST_FINISHED_SEQUENCE: u32 = 0x0042_2850;
/// `ExtraDataList::GetSayToExtra` (Xbox PDB).
const EXTRA_GET_SAY_TO: u32 = 0x0042_ede0;
/// `ExtraDataList::RemoveSayToInfoExtra` (Xbox PDB).
const EXTRA_REMOVE_SAY_TO_INFO: u32 = 0x0042_edb0;
/// Stores the say-to topic info in the extra data list.
const EXTRA_SET_SAY_TO_INFO: u32 = 0x0042_ec70;
/// Stores the say-to topic in the extra data list.
const EXTRA_SET_SAY_TO: u32 = 0x0042_ee00;
/// Third say-to setter of the extra data list (the unnamed one at 0057ad60).
const EXTRA_SET_SAY_TO_OTHER: u32 = 0x0042_eec0;
/// `ExtraDataList::GetPrimitive` (Xbox PDB).
const EXTRA_GET_PRIMITIVE: u32 = 0x0041_fbe0;
/// `ExtraDataList::GetSound` (Xbox PDB): fills the handle it is given.
const EXTRA_GET_SOUND: u32 = 0x0041_8890;
/// `ExtraDataList::SetSound` (Xbox PDB).
const EXTRA_SET_SOUND: u32 = 0x0041_a800;
/// Extra-data type of the sound extra.
const EXTRA_TYPE_SOUND: u32 = 0x4f;
/// `0x00527080(this, type)`: finds a record on the reference by type.
const FIND_RECORD_BY_TYPE: u32 = 0x0052_7080;
/// The record type the 0x0c-field accessors use.
const RECORD_TYPE_0X6D: u32 = 0x6d;

/// `BSAudio::QInstance` (Xbox PDB): the audio manager.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
/// `BSAudio::GetSoundHandleByFilename` (Xbox PDB).
const AUDIO_HANDLE_BY_FILENAME: u32 = 0x00ad_7480;
/// `BSAudio::GetSoundHandleByNumericID` (Xbox PDB).
const AUDIO_HANDLE_BY_ID: u32 = 0x00ad_73b0;
/// `BSAudio::SpawnSoundReference` (Xbox PDB).
const AUDIO_SPAWN_AT: u32 = 0x00ad_7620;
/// The `BSSoundHandle` (12 bytes) default constructor, destructor and copy.
const HANDLE_CONSTRUCT: u32 = 0x0041_a250;
const HANDLE_DESTRUCT: u32 = 0x0048_3710;
const HANDLE_ASSIGN: u32 = 0x0041_8900;
/// `BSSoundHandle` methods (Xbox PDB).
const HANDLE_SET_POSITION: u32 = 0x00ad_8b60;
const HANDLE_SET_MIN_MAX: u32 = 0x00ad_8be0;
const HANDLE_SET_FOLLOW: u32 = 0x00ad_8f20;
const HANDLE_PLAY: u32 = 0x00ad_8830;
const HANDLE_SET_COMPLETION_CALLBACK: u32 = 0x00ad_8e60;
const HANDLE_IS_VALID: u32 = 0x00ad_8ce0;
const HANDLE_STOP: u32 = 0x00ad_88f0;
const HANDLE_RELEASE: u32 = 0x00ad_8d10;
const HANDLE_FADE_OUT_AND_RELEASE: u32 = 0x00ad_8da0;
/// Takes a handle and a vector by value-words (the unnamed method at
/// `00ad8ba0`).
const HANDLE_SET_VECTOR: u32 = 0x00ad_8ba0;

/// `float` -1.0.
const MINUS_ONE: u32 = 0x0101_2054;
/// `double` 30.0 (the frame rate lip-sync lengths are divided by).
const FRAMES_PER_SECOND: u32 = 0x0101_db88;

/// Base-form pointers (globals) compared with a reference's base object.
const GLOBAL_BASE_FORM_234: u32 = 0x011c_a234;
const GLOBAL_BASE_FORM_238: u32 = 0x011c_a238;
const GLOBAL_BASE_FORM_23C: u32 = 0x011c_a23c;
const GLOBAL_BASE_FORM_230: u32 = 0x011c_a230;
/// Two global tables (objects, passed as `this`).
const OBJECT_REFERENCE_MAP: u32 = 0x011c_a304;
const OBJECT_REFERENCE_LOOKUP: u32 = 0x011c_a0e0;
/// The subtitles setting (an object whose accessor returns a byte pointer).
const OBJECT_SUBTITLE_SETTING: u32 = 0x011d_8928;
/// Objects whose accessor `00403e20` returns a `float` pointer (the sound
/// range settings).
const OBJECT_SOUND_RANGE_MIN: u32 = 0x011f_6da8;
const OBJECT_SOUND_RANGE_MAX: u32 = 0x011f_6db4;
/// Vtable of the 0x10-byte record (`0057a870`) and of the 0x18-byte one
/// (`0057a960`) and the base of the latter (`0057a9d0`).
const VTABLE_SAY_TO_RECORD: u32 = 0x0103_0f14;
const VTABLE_ANIM_NOTE_RECEIVER: u32 = 0x0101_fc74;
const VTABLE_ANIM_NOTE_RECEIVER_BASE: u32 = 0x0101_fc80;
/// Type descriptors the cast in `00579690` and `0057b240` use.
const TYPE_BOUND_OBJECT_TARGET_A: u32 = 0x0118_3a00;
const TYPE_BOUND_OBJECT_TARGET_B: u32 = 0x0118_9d70;
/// Type descriptors of the cast in `00579620`.
const TYPE_CAST_SOURCE: u32 = 0x0118_3028;
const TYPE_CAST_TARGET: u32 = 0x0118_3158;

fn first_dword(e: &mut Engine, object: u32) -> u32 {
    e.call(READ_FIRST_DWORD, &args![object]).u32()
}

fn base_form_of(e: &mut Engine, refr: u32) -> u32 {
    e.call(GET_BASE_FORM, &args![refr]).u32()
}

fn base_form_type(e: &mut Engine, refr: u32) -> u32 {
    let base = base_form_of(e, refr);
    e.call(FORM_TYPE, &args![base]).u32()
}

fn extra_list_of(e: &mut Engine, refr: u32) -> u32 {
    e.call(GET_EXTRA_LIST, &args![refr]).u32()
}

fn is_actor_ref(e: &mut Engine, refr: u32) -> bool {
    e.vcall(refr, SLOT_IS_ACTOR, &args![]).bool()
}

fn model_of(e: &mut Engine, refr: u32) -> u32 {
    e.vcall(refr, SLOT_GET_3D, &args![]).u32()
}

fn position_words(e: &mut Engine, position: u32) -> [u32; 3] {
    [
        e.mem.u32(position),
        e.mem.u32(position + 4),
        e.mem.u32(position + 8),
    ]
}

fn put_words(e: &mut Engine, at: u32, words: [u32; 3]) {
    for (i, word) in words.iter().enumerate() {
        e.mem.set_u32(at + 4 * i as u32, *word);
    }
}

fn parent_cell_of(e: &mut Engine, refr: u32) -> u32 {
    e.call(GET_PARENT_CELL, &args![refr]).u32()
}

// Translated from 005789b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::AddLockChange` (Xbox PDB): marks the reference's lock as
/// changed (virtual `+0x48` with `0x1000`). A reference without a lock
/// extra is marked through the teleport data's linked reference instead, when
/// that one has a lock.
pub fn tes_object_refr_add_lock_change(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    if e.call(EXTRA_GET_LOCK, &args![me]).u32() != 0 {
        e.vcall(me, SLOT_MARK_CHANGED, &args![CHANGE_FLAG_LOCK]);
        return;
    }
    let teleport = e.call(GET_TELEPORT_DATA, &args![me]).u32();
    if teleport == 0 {
        return;
    }
    if first_dword(e, teleport) == 0 {
        return;
    }
    let linked = first_dword(e, teleport);
    if e.call(EXTRA_GET_LOCK, &args![linked]).u32() == 0 {
        return;
    }
    let target = first_dword(e, teleport);
    e.vcall(target, SLOT_MARK_CHANGED, &args![CHANGE_FLAG_LOCK]);
}

// Translated from 00578a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetLastFinishedSequence` (Xbox PDB): stores `sequence` in
/// the extra data list unless the save/load object reports something and the
/// list already has a last finished sequence.
pub fn tes_object_refr_set_last_finished_sequence(e: &mut Engine, this: Ptr, sequence: u32) {
    let me = this.addr();
    let save_load = e.global::<u32>(GLOBAL_SAVE_LOAD);
    if e.call(0x0047_c850, &args![save_load]).bool() {
        let list = extra_list_of(e, me);
        if e.call(EXTRA_GET_LAST_FINISHED_SEQUENCE, &args![list]).u32() != 0 {
            return;
        }
    }
    let list = extra_list_of(e, me);
    e.call(EXTRA_SET_LAST_FINISHED_SEQUENCE, &args![list, sequence]);
}

// Translated from 005790b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` the sound-range setting object at `011f6da8` holds (the
/// minimum distance `00578a80` hands to `SetMinMax`).
pub fn fn_005790b0(e: &mut Engine) -> f32 {
    let value = e.call(0x0040_3e20, &args![OBJECT_SOUND_RANGE_MIN]).u32();
    e.mem.f32(value)
}

// Translated from 005790d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` the sound-range setting object at `011f6db4` holds (the
/// maximum distance `00578a80` hands to `SetMinMax`).
pub fn fn_005790d0(e: &mut Engine) -> f32 {
    let value = e.call(0x0040_3e20, &args![OBJECT_SOUND_RANGE_MAX]).u32();
    e.mem.f32(value)
}

// Translated from 005790f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The completion callback of a spoken line: looks the speaker up by form ID
/// (`004839c0`) and, when it is an actor (virtual `+0xf0`) with a say-to
/// extra, sets that extra's byte at `+0x18`. The second word is passed by the
/// callers and never read.
pub fn fn_005790f0(e: &mut Engine, form_id: u32, _unused_1: u32) {
    let speaker = e.call(0x0048_39c0, &args![form_id]).u32();
    if speaker == 0 {
        return;
    }
    if !e.vcall(speaker, 0xf0, &args![]).bool() {
        return;
    }
    let list = extra_list_of(e, speaker);
    let say_to = e.call(EXTRA_GET_SAY_TO, &args![list]).u32();
    if say_to != 0 {
        e.mem.set_u8(say_to + 0x18, 1);
    }
}

// Translated from 00579160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::UpdateSoundCallBack` (Xbox PDB): when the say-to extra's
/// byte `+0x18` is set, runs the topic info's result script (unless its flag
/// `+0x25 & 8` says it already ran), sets the action flag `0x40000` on the
/// script at `+0x10`, and removes the say-to info extra.
pub fn tes_object_refr_update_sound_call_back(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let list = extra_list_of(e, me);
    let say_to = e.call(EXTRA_GET_SAY_TO, &args![list]).u32();
    if say_to == 0 || e.mem.u8(say_to + 0x18) == 0 {
        return;
    }
    let info = e.mem.u32(say_to + 0xc);
    if info != 0 && !fn_00579200(e, Ptr::new(info)) {
        e.call(0x0061_f170, &args![info, 1u32, me]);
    }
    let script = e.mem.u32(say_to + 0x10);
    if script != 0 {
        let list = extra_list_of(e, me);
        e.call(0x005a_c750, &args![script, list, 0x0004_0000u32]);
    }
    let list = extra_list_of(e, me);
    e.call(EXTRA_REMOVE_SAY_TO_INFO, &args![list]);
}

// Translated from 00579200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x08` of the byte at `this + 0x25` is set.
pub fn fn_00579200(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x25) & 0x08 != 0
}

// Translated from 00579220 (decompiled, FalloutNV.exe 1.4.0.525)
/// When `00452370(this)` holds, and either `00477ba0(this)` does not or `flag`
/// is set: asks the base form's destruction form (`00475400`) to run its
/// handler (`00475b20(destruction, this, amount, flag)`).
pub fn fn_00579220(e: &mut Engine, this: Ptr, amount: f32, flag: u8) {
    let me = this.addr();
    if !e.call(0x0045_2370, &args![me]).bool() {
        return;
    }
    if e.call(0x0047_7ba0, &args![me]).bool() && flag == 0 {
        return;
    }
    let base = base_form_of(e, me);
    let destruction = e.call(0x0047_5400, &args![base]).u32();
    e.call(0x0047_5b20, &args![destruction, me, amount, flag]);
}

// Translated from 00579280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetTargetType` (Xbox PDB): the crosshair target category
/// of the reference by its base form's type (0 for none). The numbers are
/// the ones the code returns; their meaning is not named here.
///
/// Actors (`0x16`, `0x2a`) and creatures (`0x2b`) answer by state:
/// 2 when the virtual at `+0x22c` says so, `0xb`/`0xa`/`1`/`7` otherwise.
pub fn tes_object_refr_get_target_type(e: &mut Engine, this: Ptr) -> u32 {
    let me = this.addr();
    if !is_actor_ref(e, me) && e.call(0x0047_7ba0, &args![me]).bool() {
        return 0;
    }
    let player = e.global::<u32>(GLOBAL_PLAYER);
    let base_type = base_form_type(e, me);
    // OBJ_REFR::pObjectReference (this + 0x20)
    let object = e.mem.u32(me + 0x20);
    match base_type {
        0x15 => {
            if fn_00579620(e, object) != 0 {
                4
            } else {
                0
            }
        }
        0x16 | 0x2a => {
            let actor = if is_actor_ref(e, me) { me } else { 0 };
            if actor == player {
                return 0;
            }
            if e.vcall(me, 0x22c, &args![0u32]).bool()
                && e.call(0x004f_8960, &args![actor]).u32() != 6
            {
                return 2;
            }
            if actor != 0
                && fn_00579670(e, Ptr::new(actor))
                && e.call(0x0047_c850, &args![player]).bool()
            {
                return 0xb;
            }
            if actor != 0 && e.call(0x0087_f3d0, &args![actor]).bool() {
                return 0xa;
            }
            if e.call(0x0049_97b0, &args![player]).bool() && base_form_type(e, me) != 0x16 {
                return 1;
            }
            7
        }
        0x18 | 0x1a | 0x1d | 0x1f | 0x28 | 0x29 | 0x2e | 0x2f | 0x31 | 0x32 | 0x34 | 0x3e
        | 0x67 | 0x6c | 0x73 | 0x74 | 0x26 => 1,
        0x19 => 6,
        0x1b => 2,
        0x1c => {
            let name = e.call(0x0055_d520, &args![me]).u32();
            if e.call(0x0044_a670, &args![name]).u32() != 0 {
                8
            } else {
                0
            }
        }
        0x1e => {
            if e.call(0x0046_f070, &args![object]).bool() {
                1
            } else {
                4
            }
        }
        0x17 => 4,
        0x23 => 0xe,
        0x27 => {
            if e.call(0x0050_93f0, &args![object]).bool() {
                3
            } else if e.call(0x0050_9420, &args![object]).bool() {
                5
            } else {
                0
            }
        }
        0x2b => {
            let actor = if is_actor_ref(e, me) { me } else { 0 };
            if actor == 0 {
                return 0;
            }
            if e.vcall(me, 0x22c, &args![0u32]).bool()
                && (!e.call(0x0056_6950, &args![actor]).bool()
                    || e.call(0x004f_8960, &args![actor]).u32() != 6)
            {
                return 2;
            }
            let process = e.call(0x0041_81e0, &args![actor]).u32();
            if !e.vcall(process + 0x30, 0x18, &args![]).bool() {
                return 0;
            }
            if e.call(0x0087_f3d0, &args![actor]).bool() {
                return 0xa;
            }
            if e.call(0x0049_97b0, &args![player]).bool() {
                1
            } else {
                7
            }
        }
        _ => 0,
    }
}

// Translated from 00579620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Casts `form` between two run-time types (`01183028` to `01183158`); when
/// the cast succeeds, returns what `0048cee0` says about the result,
/// otherwise 0.
pub fn fn_00579620(e: &mut Engine, form: u32) -> u32 {
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, TYPE_CAST_SOURCE, TYPE_CAST_TARGET, 0u32],
        )
        .u32();
    if cast == 0 {
        0
    } else {
        e.call(0x0048_cee0, &args![cast]).u32()
    }
}

// Translated from 00579670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the word at `this + 0x1ac` is 9.
pub fn fn_00579670(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x1ac) == 9
}

/// The owner test block `00579690` runs twice: the teleport data's cell
/// owner (an evil-faction check on a faction or a form of type 8) decides
/// whether the target counts. `check_player` adds the final
/// `00546ca0(cell, player)` test of the second copy.
fn teleport_owner_test(e: &mut Engine, this: u32, check_player: bool) -> bool {
    let teleport = e.call(GET_TELEPORT_DATA, &args![this]).u32();
    if teleport == 0 {
        return false;
    }
    let cell = e.call(0x0043_a2b0, &args![teleport]).u32();
    let mut owner = 0;
    let mut evil = false;
    if cell != 0 {
        owner = e.call(0x0054_6a40, &args![cell]).u32();
        if owner != 0 && e.call(FORM_TYPE, &args![owner]).u32() == 8 {
            evil = e.call(0x0047_d7c0, &args![owner]).bool();
        } else if owner != 0 && e.call(FORM_TYPE, &args![owner]).u32() == 0x2a {
            evil = e.call(0x0047_d740, &args![owner + 0x30]).bool();
        }
    }
    if cell == 0
        || owner == 0
        || !e.call(0x0042_5fd0, &args![cell]).bool()
        || e.call(0x0054_43e0, &args![cell]).bool()
        || evil
    {
        return false;
    }
    if check_player {
        let player = e.global::<u32>(GLOBAL_PLAYER);
        if e.call(0x0054_6ca0, &args![cell, player]).bool() {
            return false;
        }
    }
    true
}

// Translated from 00579690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the player would be stealing by activating this reference:
/// doors (form type `0x1c`) and beds (furniture `0x27`, sleepable) with an
/// owner the player does not belong to, actors (`0x2a`, `0x2b`) the player
/// may not use, and any other owned object (not a creature) the player is
/// not an owner of. Returns 0 when the player's virtual at `+0x22c` says no.
pub fn fn_00579690(e: &mut Engine, this: Ptr) -> bool {
    let me = this.addr();
    let player = e.global::<u32>(GLOBAL_PLAYER);
    let mut result = false;
    let lock = e.call(0x0056_9160, &args![me]).u32();
    if e.vcall(player, 0x22c, &args![0u32]).bool() {
        return false;
    }
    if base_form_type(e, me) == 0x1c {
        let door = base_form_of(e, me);
        let player = e.global::<u32>(GLOBAL_PLAYER);
        let lock_applies = door != 0
            && !tes_object_refr_is_an_owner(e, Ptr::new(me), player, true)
            && e.call(0x0056_7790, &args![me]).u32() != 0;
        if lock_applies {
            if e.call(0x0051_8f00, &args![me, player, 0u32, 0u32]).bool() {
                return false;
            }
            if lock != 0 && e.call(0x0050_21a0, &args![lock]).bool() {
                result = true;
            } else {
                result = teleport_owner_test(e, me, false);
            }
        } else {
            result = teleport_owner_test(e, me, true);
        }
    } else if base_form_type(e, me) == 0x27 {
        let furniture = base_form_of(e, me);
        let player = e.global::<u32>(GLOBAL_PLAYER);
        result = e.call(0x0050_9420, &args![furniture]).bool()
            && e.call(0x0056_7790, &args![me]).u32() != 0
            && !tes_object_refr_is_an_owner(e, Ptr::new(me), player, true);
    } else {
        let player = e.global::<u32>(GLOBAL_PLAYER);
        if base_form_type(e, me) == 0x2a
            && e.call(0x0049_97b0, &args![player]).bool()
            && !e.vcall(me, 0x22c, &args![0u32]).bool()
        {
            result = true;
        } else if base_form_type(e, me) == 0x2b && e.call(0x0049_97b0, &args![player]).bool() {
            let base = base_form_of(e, me);
            let cast = e
                .call(
                    DYNAMIC_CAST,
                    &args![
                        base,
                        0u32,
                        TYPE_TES_BOUND_OBJECT,
                        TYPE_BOUND_OBJECT_TARGET_A,
                        0u32
                    ],
                )
                .u32();
            if cast != 0
                && e.vcall(cast + 0x30, 0x18, &args![]).bool()
                && !e.vcall(me, 0x22c, &args![0u32]).bool()
            {
                result = true;
            }
        }
    }
    if !result && base_form_type(e, me) != 0x2b && e.call(0x0056_7790, &args![me]).u32() != 0 {
        let player = e.global::<u32>(GLOBAL_PLAYER);
        if !tes_object_refr_is_an_owner(e, Ptr::new(me), player, true) {
            result = true;
        }
    }
    result
}

/// The `BSSoundHandle` the audio manager returns for `name`/`flags`
/// (`BSAudio::GetSoundHandleByFilename`), assigned into `handle`; the
/// temporary is destroyed.
fn assign_handle_by_filename(e: &mut Engine, handle: u32, name: u32, flags: u32) {
    let temporary = e.mem.alloc(12);
    let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
    let made = e
        .call(
            AUDIO_HANDLE_BY_FILENAME,
            &args![audio, temporary, name, flags, 0u32],
        )
        .u32();
    e.call(HANDLE_ASSIGN, &args![handle, made]);
    e.call(HANDLE_DESTRUCT, &args![temporary]);
    e.mem.free(temporary);
}

/// The sound-playing half of `00578a80`: loads the line's sound file into
/// `handle` (reading the lip-sync length from the file when requested),
/// positions and plays it, and registers the completion callback.
fn say_play_sound(
    e: &mut Engine,
    this: u32,
    handle: u32,
    response_name: u32,
    ignore_3d: bool,
    variant: bool,
    read_lip_length: bool,
) {
    let buffer = e.mem.alloc(0x200);
    e.call(0x0040_6d30, &args![buffer, 0x200u32, response_name]);
    if read_lip_length {
        let string = e.mem.alloc(8);
        e.call(0x0040_37b0, &args![string]);
        e.call(0x0040_37f0, &args![string, buffer, 0x200u32]);
        if e.call(0x004d_5ad0, &args![string]).bool() {
            let lip = e.call(0x004d_52d0, &args![string]).u32();
            if lip != 0 {
                let frames = first_dword(e, lip);
                let divisor: f64 = e.global(FRAMES_PER_SECOND);
                // the length in seconds; the exe keeps it in a local it
                // never reads again
                let _length_seconds = (f64::from(frames) / divisor) as f32;
                e.call(0x004d_5850, &args![lip, 1u32]);
            }
        }
        e.call(0x0040_37d0, &args![string]);
        e.mem.free(string);
    }
    // the exe's local `[ebp-0x18]` is set to 0 and never changed
    let base_flags = 0u32;
    let has_3d = !ignore_3d && model_of(e, this) != 0;
    let flags = match (has_3d, variant) {
        (false, true) => base_flags | 0x101,
        (false, false) => base_flags | 0x105,
        (true, true) => base_flags | 0x102,
        (true, false) => base_flags | 0x106,
    };
    assign_handle_by_filename(e, handle, buffer, flags);
    if !ignore_3d && model_of(e, this) != 0 {
        let model = model_of(e, this);
        let position = e.call(0x0045_bb80, &args![model]).u32();
        let [x, y, z] = position_words(e, position);
        e.call(HANDLE_SET_POSITION, &args![handle, x, y, z]);
        let maximum = fn_005790d0(e);
        let minimum = fn_005790b0(e);
        e.call(HANDLE_SET_MIN_MAX, &args![handle, minimum, maximum]);
        let model = model_of(e, this);
        e.call(HANDLE_SET_FOLLOW, &args![handle, model]);
    }
    e.call(HANDLE_PLAY, &args![handle, 0u32]);
    let id = e.call(FORM_ID, &args![this]).u32();
    e.call(
        HANDLE_SET_COMPLETION_CALLBACK,
        &args![handle, 0x0057_90f0u32, id],
    );
    e.mem.free(buffer);
}

// Translated from 00578a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes the reference speak `topic`: creates the dialogue item
/// (`TESTopic::CreateDialogueItem`), records the topic and its info on the
/// reference (`SetSayToTopic`, `SetSayToTopicInfo`), runs the info's result
/// and adds it to the topic list, then plays the first response's sound
/// file through a `BSSoundHandle` (positioned at the reference's 3D unless
/// `ignore_3d`) and shows the subtitle (`Interface::ShowSubtitle`) when the
/// subtitle setting or the actor's `AlwaysShowSubtitles` allow it. The sound
/// handle is returned through `out`, which the caller has constructed.
///
/// Parameters, by what the code does with them: `dialogue_arg` goes to the
/// dialogue item and the subtitle; `ignore_3d` selects the sound flags
/// without positioning; `variant` selects between the flag values `0x101` /
/// `0x105` (no 3D) and `0x102` / `0x106` (3D); `read_lip_length` reads the
/// lip-sync data of the file; `subtitle_flag` goes to the subtitle. The
/// word at `+0x20` is never read. C++ exception unwinding is not
/// translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_00578a80(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    topic: u32,
    dialogue_arg: u32,
    ignore_3d: u8,
    variant: u8,
    read_lip_length: u8,
    _unused_7: u32,
    subtitle_flag: u8,
) -> Ptr {
    let me = this.addr();
    let handle = e.mem.alloc(12);
    e.call(HANDLE_CONSTRUCT, &args![handle]);
    let dialogue = e
        .call(
            0x0061_b320,
            &args![topic, me, dialogue_arg, 0u32, 0u32, 0u32],
        )
        .u32();
    fn_0057ad20(e, this, topic);
    // the exe reads the base form and, for type 0x16, the base form again;
    // the result goes to a local that is never read
    if base_form_of(e, me) != 0 && base_form_type(e, me) == 0x16 {
        base_form_of(e, me);
    }
    if dialogue != 0 {
        let info = e.call(FORM_ID, &args![dialogue]).u32();
        fn_0057ace0(e, this, info);
        if info != 0 {
            e.call(0x0061_f170, &args![info, 0u32, me]);
            e.call(0x0061_f150, &args![info]);
        }
        e.call(0x0083_c7b0, &args![dialogue]);
        let response = e.call(0x0083_c820, &args![dialogue]).u32();
        if response != 0 {
            let holder = e.call(0x0046_0140, &args![response]).u32();
            if first_dword(e, holder) == 0 {
                if info != 0 {
                    let id = e.call(FORM_ID, &args![me]).u32();
                    fn_005790f0(e, id, 0);
                }
            } else {
                let holder = e.call(0x0046_0140, &args![response]).u32();
                let name = first_dword(e, holder);
                say_play_sound(
                    e,
                    me,
                    handle,
                    name,
                    ignore_3d != 0,
                    variant != 0,
                    read_lip_length != 0,
                );
            }
            let setting = e.call(0x0040_8d60, &args![OBJECT_SUBTITLE_SETTING]).u32();
            let show = e.mem.u8(setting) != 0
                || (is_actor_ref(e, me) && e.call(0x008c_1bc0, &args![me]).bool());
            if show {
                let position = e.vcall(me, SLOT_POSITION_PTR, &args![]).u32();
                let [x, y, z] = position_words(e, position);
                let copy = e.mem.alloc(12);
                e.call(HANDLE_ASSIGN, &args![copy, handle]);
                let [h0, h1, h2] = position_words(e, copy);
                let holder = e.call(0x0068_15c0, &args![response]).u32();
                let text = first_dword(e, holder);
                e.call(
                    0x0070_5210,
                    &args![
                        text,
                        h0,
                        h1,
                        h2,
                        x,
                        y,
                        z,
                        dialogue_arg,
                        u32::from(subtitle_flag)
                    ],
                );
                e.mem.free(copy);
            }
        }
    }
    e.call(HANDLE_ASSIGN, &args![out, handle]);
    e.call(HANDLE_DESTRUCT, &args![handle]);
    e.mem.free(handle);
    out
}

// Translated from 0057a2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the reference's base form has one of the types that carry a
/// looping sound: `0x0d`, `0x15`, `0x17`, `0x1b`, `0x1c`, `0x1e`, `0x1f`,
/// `0x20`, `0x22`, `0x23`, `0x2b` (the jump table of the exe).
pub fn fn_0057a2f0(e: &mut Engine, this: Ptr) -> bool {
    let me = this.addr();
    if base_form_of(e, me) == 0 {
        return false;
    }
    sound_carrying_type(base_form_type(e, me))
}

/// The types `0057a2f0` accepts and `00579ac0` fades out.
fn sound_carrying_type(form_type: u32) -> bool {
    matches!(
        form_type,
        0x0d | 0x15 | 0x17 | 0x1b | 0x1c | 0x1e | 0x1f | 0x20 | 0x22 | 0x23 | 0x2b
    )
}

// Translated from 0057a370 (decompiled, FalloutNV.exe 1.4.0.525)
/// False when `00437b90(this)` holds, or when the base form is a door
/// (type `0x1c`) and `00477ba0(this)` holds; true otherwise.
pub fn fn_0057a370(e: &mut Engine, this: Ptr) -> bool {
    let me = this.addr();
    if e.call(0x0043_7b90, &args![me]).bool() {
        return false;
    }
    !(base_form_type(e, me) == 0x1c && e.call(0x0047_7ba0, &args![me]).bool())
}

// Translated from 0057a3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::UpdateAddonNodeSounds` (Xbox PDB): when the reference has
/// a 3D (`0043fcd0`) and `004523e0(this, 2)` holds, walks the 3D's addon
/// nodes with `0057a410`.
pub fn tes_object_refr_update_addon_node_sounds(e: &mut Engine, this: Ptr, flag: u8) {
    let me = this.addr();
    let model = e.call(0x0043_fcd0, &args![me]).u32();
    if model != 0 && e.call(0x0045_23e0, &args![me, 2u32]).bool() {
        fn_0057a410(e, this, model, flag);
    }
}

// Translated from 0057a410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the scene-graph object `node` and its children. For a node the
/// data handler has an addon record for (`009ee040` key, `004617e0`), whose
/// record has a sound (`006a1b60`), it keeps a small record holding a
/// sound handle at `+0x0c` (`0x18` bytes made with `00aa13e0`, `004f91b0`):
/// when `flag` is set (or `00456610(node)` holds) it stops and releases an
/// existing handle; otherwise it makes the handle when none exists, makes it
/// follow the node and plays it. The flag is passed on to the children;
/// `this` is only handed on to them. C++ exception unwinding is not
/// translated.
#[allow(clippy::only_used_in_recursion)]
pub fn fn_0057a410(e: &mut Engine, this: Ptr, node: u32, flag: u8) {
    if node == 0 {
        return;
    }
    let mut flag = flag;
    if e.call(0x0045_6610, &args![node]).bool() {
        flag = 1;
    }
    if is_node(e, node) {
        let record = addon_for_node(e, node);
        if record != 0 && e.call(0x006a_1b60, &args![record]).u32() != 0 {
            addon_node_sound(e, node, record, flag);
        }
    }
    let as_node = e.vcall(node, SLOT_AS_NODE, &args![]).u32();
    if as_node != 0 {
        let mut index = 0;
        while index < child_count(e, as_node) {
            let child = child_at(e, as_node, index);
            if child != 0 {
                fn_0057a410(e, this, child, flag);
            }
            index += 1;
        }
    }
}

/// The sound part of `0057a410` for one node with an addon record.
fn addon_node_sound(e: &mut Engine, node: u32, record: u32, flag: u8) {
    let slot = e.mem.alloc(4);
    let global = e.call(0x004f_9230, &args![]).u32();
    let made = e.call(0x00a5_bdd0, &args![node, global]).u32();
    e.call(0x0063_3c90, &args![slot, made]);
    if flag != 0 {
        if first_dword(e, slot) != 0 {
            let global = e.call(0x004f_9230, &args![]).u32();
            e.call(0x00a5_be90, &args![node, global]);
            let holder = first_dword(e, slot);
            e.call(HANDLE_STOP, &args![holder + 0xc]);
            let holder = first_dword(e, slot);
            e.call(HANDLE_RELEASE, &args![holder + 0xc]);
        }
    } else if first_dword(e, slot) == 0 {
        let memory = e.call(0x00aa_13e0, &args![0x18u32]).u32();
        let created = if memory != 0 {
            e.call(0x004f_91b0, &args![memory]).u32()
        } else {
            0
        };
        e.call(0x0066_b0d0, &args![slot, created]);
        let scratch = e.mem.alloc(0x48);
        let sound = e.call(0x006a_1b60, &args![record]).u32();
        let info = e.call(0x004e_75d0, &args![sound, scratch]).u32();
        let kind = e.mem.u32(info + 4);
        let handle_flags = if e.call(0x005e_39b0, &args![kind]).u32() != 0 {
            let sound = e.call(0x006a_1b60, &args![record]).u32();
            let info = e.call(0x004e_75d0, &args![sound, scratch + 0x24]).u32();
            let kind = e.mem.u32(info + 4);
            e.call(0x005e_39b0, &args![kind]).u32() | 2
        } else {
            0x102
        };
        e.mem.free(scratch);
        let sound = e.call(0x006a_1b60, &args![record]).u32();
        let id = e.call(FORM_ID, &args![sound]).u32();
        let temporary = e.mem.alloc(12);
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let made = e
            .call(
                AUDIO_HANDLE_BY_ID,
                &args![audio, temporary, id, handle_flags],
            )
            .u32();
        let holder = first_dword(e, slot);
        e.call(HANDLE_ASSIGN, &args![holder + 0xc, made]);
        e.call(HANDLE_DESTRUCT, &args![temporary]);
        e.mem.free(temporary);
        let holder = first_dword(e, slot);
        e.call(HANDLE_SET_FOLLOW, &args![holder + 0xc, node]);
        let position = e.call(0x0045_bb80, &args![node]).u32();
        let [x, y, z] = position_words(e, position);
        let holder = first_dword(e, slot);
        e.call(HANDLE_SET_POSITION, &args![holder + 0xc, x, y, z]);
        let holder = first_dword(e, slot);
        e.call(HANDLE_PLAY, &args![holder + 0xc, 0u32]);
        let holder = first_dword(e, slot);
        e.call(0x00a5_bca0, &args![node, holder]);
    }
    e.call(0x0045_cec0, &args![slot]);
    e.mem.free(slot);
}

// Translated from 0057a740 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0x0c` of the record of type `0x6d` that `00527080(this)`
/// finds on the reference, 0 when there is none.
pub fn fn_0057a740(e: &mut Engine, this: Ptr) -> u32 {
    let record = e
        .call(FIND_RECORD_BY_TYPE, &args![this, RECORD_TYPE_0X6D])
        .u32();
    if record == 0 {
        0
    } else {
        e.mem.u32(record + 0xc)
    }
}

// Translated from 0057a770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `0057a740`, but makes the record when it is missing: allocates the
/// `0x18`-byte object (`0057a960`), links it to the reference (`006ecd40`),
/// wraps it in a `0x10`-byte record (`0057a870`) and adds that to the
/// reference's extra data list (`0040ff60`). Returns the record's word at
/// `+0x0c`. C++ exception unwinding is not translated.
pub fn fn_0057a770(e: &mut Engine, this: Ptr) -> u32 {
    let me = this.addr();
    let mut record = e
        .call(FIND_RECORD_BY_TYPE, &args![me, RECORD_TYPE_0X6D])
        .u32();
    if record == 0 {
        let memory = e.call(0x0040_1000, &args![0x18u32]).u32();
        let receiver = if memory != 0 {
            fn_0057a960(e, Ptr::new(memory)).addr()
        } else {
            0
        };
        e.call(0x006e_cd40, &args![receiver, me]);
        let memory = e.call(0x0040_1000, &args![0x10u32]).u32();
        record = if memory != 0 {
            fn_0057a870(e, Ptr::new(memory), receiver).addr()
        } else {
            0
        };
        let list = extra_list_of(e, me);
        e.call(EXTRA_ADD, &args![list, record]);
    }
    e.mem.u32(record + 0xc)
}

// Translated from 0057a870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `0x10`-byte extra record (type `0x6d`): the base
/// constructor (`0040ec80`), the record's vtable, and the wrapped object at
/// `+0x0c`.
pub fn fn_0057a870(e: &mut Engine, this: Ptr, wrapped: u32) -> Ptr {
    e.call(0x0040_ec80, &args![this, RECORD_TYPE_0X6D]);
    e.mem.set_u32(this.addr(), VTABLE_SAY_TO_RECORD);
    e.mem.set_u32(this.addr() + 0xc, wrapped);
    this
}

// Translated from 0057a8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraAnimNoteReceiver::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor (`0057a8d0`) and, when bit 0 of `flags` is set, frees the
/// object (`00401030`).
pub fn extra_anim_note_receiver_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0057a8d0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0057a8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `0x10`-byte extra record: sets its vtable, deletes the
/// wrapped object at `+0x0c` through its scalar deleting destructor
/// (virtual 0 with 1), then runs the base destructor (`0040ecb0`). C++
/// exception unwinding is not translated.
pub fn fn_0057a8d0(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    e.mem.set_u32(me, VTABLE_SAY_TO_RECORD);
    let wrapped = e.mem.u32(me + 0xc);
    if wrapped != 0 {
        e.vcall(wrapped, 0, &args![1u32]);
    }
    e.call(0x0040_ecb0, &args![me]);
}

// Translated from 0057a960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `0x18`-byte animation-note receiver: the base
/// constructor (`0057a9d0`), its vtable, and `0057c700(this + 8, 0, 1)`.
/// C++ exception unwinding is not translated.
pub fn fn_0057a960(e: &mut Engine, this: Ptr) -> Ptr {
    fn_0057a9d0(e, this);
    e.mem.set_u32(this.addr(), VTABLE_ANIM_NOTE_RECEIVER);
    e.call(0x0057_c700, &args![this.addr() + 8, 0u32, 1u32]);
    this
}

// Translated from 0057a9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the animation-note receiver's base: stores its vtable.
pub fn fn_0057a9d0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_ANIM_NOTE_RECEIVER_BASE);
    this
}

// Translated from 0057a9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CheckWithinMultiBoundTask::Run` (Xbox PDB): when the task's target
/// (smart pointer at `+0x1c`) has a multi-bound (its virtual `+0x10`,
/// `009ad610`) and the task's reference (`+0x18`) has a 3D, stores the result
/// of `00569670(target, other, &true, 1)` in the byte at `+0x24`.
pub fn check_within_multi_bound_task_run(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    if first_dword(e, me + 0x1c) == 0 {
        return;
    }
    let target = first_dword(e, me + 0x1c);
    let bound = e.vcall(target, 0x10, &args![]).u32();
    if bound == 0 || e.call(0x009a_d610, &args![bound]).u32() == 0 {
        return;
    }
    let reference = e.mem.u32(me + 0x18);
    if model_of(e, reference) == 0 {
        return;
    }
    let flag = e.mem.alloc(4);
    e.mem.set_u8(flag, 1);
    let other = first_dword(e, me + 0x20);
    let target = first_dword(e, me + 0x1c);
    let result = e.call(0x0056_9670, &args![target, other, flag, 1u32]).u8();
    e.mem.set_u8(me + 0x24, result);
    e.mem.free(flag);
}

// Translated from 0057aa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CheckWithinMultiBoundTask::PostProcess` (Xbox PDB): when `Run` found
/// something (byte `+0x24`), the target's multi-bound is looked up again
/// (virtual `+0x10`, `009ad610`); if the reference has a parent cell and
/// `0057ab50(bound)` holds, records the pair with `SetAt` on the global map
/// at `011ca304`.
pub fn check_within_multi_bound_task_post_process(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    if e.mem.u8(me + 0x24) == 0 {
        return;
    }
    let bound_source = if first_dword(e, me + 0x1c) != 0 {
        let target = first_dword(e, me + 0x1c);
        e.vcall(target, 0x10, &args![]).u32()
    } else {
        0
    };
    let bound = if bound_source != 0 {
        e.call(0x009a_d610, &args![bound_source]).u32()
    } else {
        0
    };
    if bound == 0 {
        return;
    }
    let reference = e.mem.u32(me + 0x18);
    if parent_cell_of(e, reference) == 0 {
        return;
    }
    parent_cell_of(e, reference);
    if fn_0057ab50(e, bound) {
        e.call(0x0084_4700, &args![OBJECT_REFERENCE_MAP, bound, reference]);
    }
}

// Translated from 0057ab50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks `key` up in the global map at `011ca0e0` (`00853130`) and returns
/// whether it is there.
pub fn fn_0057ab50(e: &mut Engine, key: u32) -> bool {
    let value = e.mem.alloc(4);
    let found = e
        .call(0x0085_3130, &args![OBJECT_REFERENCE_LOOKUP, key, value])
        .bool();
    e.mem.free(value);
    found
}

// Translated from 0057ab70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drains the global map at `011ca304` (iterating with `004b9ba0` and
/// `006b7f20`, then emptying it with `00438af0`). Each entry is a reference
/// and a value; unless the reference's 3D already belongs to the same cell
/// (`005497a0` of its parent cell against `009611e0` of its 3D), the value's
/// owner is looked up in the parent cell (`00545960`) and, when found,
/// told about the 3D (virtuals `+0xe8` on the 3D's owner and `+0xdc` on the
/// found object), then `00569ae0(reference, value)` runs.
pub fn fn_0057ab70(e: &mut Engine) {
    let cursor = e.mem.alloc(12);
    // The first position once (0057ab7c); `006b7f20` then reads the entry at the cursor and
    // moves the cursor on, and the loop tests only the cursor (0057ab84).
    let first = e.call(0x004b_9ba0, &args![OBJECT_REFERENCE_MAP]).u32();
    e.mem.set_u32(cursor, first);
    while e.mem.u32(cursor) != 0 {
        e.call(
            0x006b_7f20,
            &args![OBJECT_REFERENCE_MAP, cursor, cursor + 4, cursor + 8],
        );
        let reference = e.mem.u32(cursor + 4);
        let value = e.mem.u32(cursor + 8);
        if reference == 0 || value == 0 {
            continue;
        }
        let same_cell = model_of(e, reference) != 0 && {
            let cell = parent_cell_of(e, reference);
            let from_cell = e.call(0x0054_97a0, &args![cell]).u32();
            let model = model_of(e, reference);
            let from_model = e.call(0x0096_11e0, &args![model]).u32();
            from_cell == from_model
        };
        if same_cell {
            continue;
        }
        let cell = parent_cell_of(e, reference);
        let target = e.call(0x0054_5960, &args![cell, value]).u32();
        if target == 0 {
            continue;
        }
        let model = model_of(e, reference);
        let owner = e.call(0x0096_11e0, &args![model]).u32();
        let model = model_of(e, reference);
        e.vcall(owner, 0xe8, &args![model]);
        let model = model_of(e, reference);
        e.vcall(target, 0xdc, &args![model, 1u32]);
        e.call(0x0056_9ae0, &args![reference, value]);
    }
    e.call(0x0043_8af0, &args![OBJECT_REFERENCE_MAP]);
    e.mem.free(cursor);
}

// Translated from 0057acc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `key` from the global map at `011ca304` (`00405430`, the map's
/// `RemoveAt`) and returns its result.
pub fn fn_0057acc0(e: &mut Engine, key: u32) -> u32 {
    e.call(0x0040_5430, &args![OBJECT_REFERENCE_MAP, key]).u32()
}

// Translated from 0057ace0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetSayToTopicInfo` (Xbox PDB): stores `info` in the extra
/// data list and marks the reference changed with `0x80000000`.
pub fn fn_0057ace0(e: &mut Engine, this: Ptr, info: u32) {
    let me = this.addr();
    let list = extra_list_of(e, me);
    e.call(EXTRA_SET_SAY_TO_INFO, &args![list, info]);
    e.vcall(me, SLOT_MARK_CHANGED, &args![CHANGE_FLAG_SAY_TO]);
}

// Translated from 0057ad20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetSayToTopic` (Xbox PDB): stores `topic` in the extra
/// data list and marks the reference changed with `0x80000000`.
pub fn fn_0057ad20(e: &mut Engine, this: Ptr, topic: u32) {
    let me = this.addr();
    let list = extra_list_of(e, me);
    e.call(EXTRA_SET_SAY_TO, &args![list, topic]);
    e.vcall(me, SLOT_MARK_CHANGED, &args![CHANGE_FLAG_SAY_TO]);
}

// Translated from 0057ad60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` through the third say-to setter of the extra data list
/// (`0042eec0`) and marks the reference changed with `0x80000000`.
pub fn fn_0057ad60(e: &mut Engine, this: Ptr, value: u32) {
    let me = this.addr();
    let list = extra_list_of(e, me);
    e.call(EXTRA_SET_SAY_TO_OTHER, &args![list, value]);
    e.vcall(me, SLOT_MARK_CHANGED, &args![CHANGE_FLAG_SAY_TO]);
}

/// The stop path of `00579ac0` for a sound-carrying type: fades the handle
/// of the reference's sound extra out over 1000 ms and removes the extra,
/// when the handle is valid.
fn fade_out_sound_extra(e: &mut Engine, this: u32) {
    let handle = e.mem.alloc(12);
    e.call(HANDLE_CONSTRUCT, &args![handle]);
    let list = extra_list_of(e, this);
    e.call(EXTRA_GET_SOUND, &args![list, handle]);
    if e.call(HANDLE_IS_VALID, &args![handle]).bool() {
        e.call(HANDLE_FADE_OUT_AND_RELEASE, &args![handle, 1000u32]);
        let list = extra_list_of(e, this);
        e.call(EXTRA_REMOVE_TYPE, &args![list, EXTRA_TYPE_SOUND]);
    }
    e.call(HANDLE_DESTRUCT, &args![handle]);
    e.mem.free(handle);
}

/// What `00579ac0` finds to play for a base form of `form_type`.
struct SoundChoice {
    /// The sound form (0 for none).
    sound: u32,
    /// The primitive extra (0 unless the base form type is `0x0d`).
    primitive: u32,
    /// The vector handed to the handle in the primitive case.
    direction: [u32; 3],
    /// The position the sound is spawned at in the primitive case.
    spawn_at: [u32; 3],
}

/// The `switch` of `00579ac0` on the base form's type `form_type`.
fn sound_for_base_form(e: &mut Engine, this: u32, form_type: u32) -> SoundChoice {
    let zero = position_words(e, ZERO_VECTOR);
    let mut choice = SoundChoice {
        sound: 0,
        primitive: 0,
        direction: zero,
        spawn_at: zero,
    };
    match form_type {
        0x0d => {
            choice.sound = base_form_of(e, this);
            let list = extra_list_of(e, this);
            choice.primitive = e.call(EXTRA_GET_PRIMITIVE, &args![list]).u32();
            if choice.primitive != 0 {
                primitive_vectors(e, this, &mut choice);
            }
        }
        0x15 => {
            let base = base_form_of(e, this);
            if e.call(0x004f_d3c0, &args![base]).u32() != 0
                && e.call(0x0047_b250, &args![this]).i32() == 1
            {
                let loading = e.global::<u32>(GLOBAL_FLAG_OBJECT);
                if !e.call(0x0042_ce10, &args![loading]).bool() {
                    e.call(0x0083_25b0, &args![this, 1u32]);
                }
            }
            let base = base_form_of(e, this);
            choice.sound = e.call(0x004f_d3a0, &args![base]).u32();
        }
        0x17 => {
            let base = base_form_of(e, this);
            choice.sound = e.call(0x004f_d3a0, &args![base]).u32();
        }
        0x1b => {
            let base = base_form_of(e, this);
            choice.sound = e.call(0x0091_85e0, &args![base]).u32();
        }
        0x1c => {
            let base = base_form_of(e, this);
            choice.sound = e.call(0x004f_b070, &args![base]).u32();
        }
        0x1e => {
            let base = base_form_of(e, this);
            choice.sound = e.call(0x0040_36b0, &args![base]).u32();
        }
        0x1f => {
            let base = base_form_of(e, this);
            choice.sound = e.call(0x008c_dd90, &args![base]).u32();
        }
        0x20 => {
            let base = base_form_of(e, this);
            choice.sound = e.mem.u32(base + 0x54);
        }
        0x22 => {
            let base = base_form_of(e, this);
            let adjusted = if base != 0 { base - 0x14 } else { 0 };
            choice.sound = e.call(0x005e_3fa0, &args![adjusted]).u32();
        }
        0x23 => {
            let base = base_form_of(e, this);
            if base != 0
                && !e.call(0x0045_2440, &args![base]).bool()
                && !e.call(0x004e_32c0, &args![base]).bool()
            {
                let inner = e.vcall(base, 0x140, &args![]).u32();
                if inner != 0 {
                    choice.sound = e.call(0x0040_7840, &args![inner]).u32();
                }
            }
        }
        0x2b if is_actor_ref(e, this) && e.vcall(this, 0x21c, &args![]).bool() => {
            let base = base_form_of(e, this);
            choice.sound = e.call(0x005f_92d0, &args![base, 0x15u32]).u32();
        }
        _ => {}
    }
    choice
}

/// The primitive case of `sound_for_base_form`: the two vectors derived from
/// the primitive extra's box (`00413fc0`), rotated by the reference's
/// orientation (`0056fa00`) and moved to its position; the middle
/// components of the two vectors are swapped at the end.
fn primitive_vectors(e: &mut Engine, this: u32, choice: &mut SoundChoice) {
    let scratch = e.mem.alloc(0x100);
    let matrix = scratch;
    let position = scratch + 0x30;
    let vector = scratch + 0x40;
    let moved = scratch + 0x50;
    let out = scratch + 0x60;
    e.call(0x0056_fa00, &args![this, matrix]);
    let at = e.vcall(this, SLOT_POSITION_PTR, &args![]).u32();
    let words = position_words(e, at);
    put_words(e, position, words);
    let list = extra_list_of(e, this);
    let extra = e.call(EXTRA_GET_PRIMITIVE, &args![list]).u32();
    let box_vector = e.call(0x0041_3fc0, &args![extra, out]).u32();
    let scale: f32 = e.global(MINUS_ONE);
    let scaled = e
        .call(0x004a_3760, &args![out + 0x10, scale, box_vector])
        .u32();
    let words = position_words(e, scaled);
    put_words(e, vector, words);
    let rotated = e
        .call(0x004b_3ae0, &args![out + 0x20, vector, matrix])
        .u32();
    let words = position_words(e, rotated);
    put_words(e, vector, words);
    let at = e.vcall(this, SLOT_POSITION_PTR, &args![]).u32();
    e.call(0x0063_c8a0, &args![vector, at]);
    choice.direction = position_words(e, vector);
    let list = extra_list_of(e, this);
    let extra = e.call(EXTRA_GET_PRIMITIVE, &args![list]).u32();
    let box_vector = e.call(0x0041_3fc0, &args![extra, out + 0x30]).u32();
    let words = position_words(e, box_vector);
    put_words(e, moved, words);
    let rotated = e.call(0x004b_3ae0, &args![out + 0x40, moved, matrix]).u32();
    let words = position_words(e, rotated);
    put_words(e, moved, words);
    e.call(0x0063_c8a0, &args![moved, position]);
    choice.spawn_at = position_words(e, moved);
    std::mem::swap(&mut choice.spawn_at[1], &mut choice.direction[1]);
    e.mem.free(scratch);
}

// Translated from 00579ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts (`start` set) or stops (`start` clear) the reference's sound by
/// its base form's type, then refreshes the addon-node sounds
/// (`UpdateAddonNodeSounds`) when the reference has a 3D.
///
/// Starting (unless `00440da0` or `00477ba0` hold): the sound form comes
/// from the base form (`sound_for_base_form`: the types `0x0d`, `0x15`,
/// `0x17`, `0x1b`, `0x1c`, `0x1e`, `0x1f`, `0x20`, `0x22`, `0x23`, `0x2b`);
/// a valid old handle in the sound extra is stopped, released and its extra
/// removed; then the new handle is made from the sound's form id
/// (`SpawnSoundReference`, at a position when the sound's data has bit
/// `0x10`, or for a primitive), or the sound is only recorded in
/// `pRandomSound`, and the handle goes into the sound extra
/// (`ExtraDataList::SetSound`). Stopping fades a valid handle out over
/// 1000 ms for the sound-carrying types and, for a type `0x15` base form
/// that passes the checks, calls `008325b0(this, 0)`.
///
/// C++ exception unwinding is not translated.
pub fn fn_00579ac0(e: &mut Engine, this: Ptr, start: u8) {
    let me = this.addr();
    if base_form_of(e, me) == 0 {
        return;
    }
    if start != 0 {
        if !e.call(0x0044_0da0, &args![me]).bool() && !e.call(0x0047_7ba0, &args![me]).bool() {
            let form_type = base_form_type(e, me);
            let choice = sound_for_base_form(e, me, form_type);
            start_sound(e, me, &choice);
        }
    } else {
        if sound_carrying_type(base_form_type(e, me)) {
            fade_out_sound_extra(e, me);
        }
        if base_form_of(e, me) != 0 && base_form_type(e, me) == 0x15 {
            let base = base_form_of(e, me);
            if e.call(0x004f_d3c0, &args![base]).u32() != 0 {
                let loading = e.global::<u32>(GLOBAL_FLAG_OBJECT);
                if !e.call(0x0042_ce10, &args![loading]).bool() {
                    let handler = e.global::<u32>(GLOBAL_DATA_HANDLER);
                    if !e.call(0x0042_26e0, &args![handler]).bool() {
                        e.call(0x0083_25b0, &args![me, 0u32]);
                    }
                }
            }
        }
    }
    if e.call(0x0043_fcd0, &args![me]).u32() != 0 {
        tes_object_refr_update_addon_node_sounds(e, this, start);
    }
}

/// The handle-making half of `00579ac0`.
fn start_sound(e: &mut Engine, this: u32, choice: &SoundChoice) {
    let handle = e.mem.alloc(12);
    e.call(HANDLE_CONSTRUCT, &args![handle]);
    let list = extra_list_of(e, this);
    e.call(EXTRA_GET_SOUND, &args![list, handle]);
    if e.call(HANDLE_IS_VALID, &args![handle]).bool() {
        e.call(HANDLE_STOP, &args![handle]);
        e.call(HANDLE_RELEASE, &args![handle]);
        let list = extra_list_of(e, this);
        e.call(EXTRA_REMOVE_TYPE, &args![list, EXTRA_TYPE_SOUND]);
    }
    let sound = choice.sound;
    if sound != 0 {
        let primitive_flag = if choice.primitive != 0 {
            0x1000_0000
        } else {
            0
        };
        let scratch = e.mem.alloc(0x24);
        let data = e.call(0x004e_75d0, &args![sound, scratch]).u32();
        let positional = if e.mem.u32(data + 4) & 0x10 != 0 {
            0x10
        } else {
            0
        };
        e.mem.free(scratch);
        let temporary = e.mem.alloc(12);
        if choice.primitive != 0 {
            let [x, y, z] = choice.spawn_at;
            let id = e.call(FORM_ID, &args![sound]).u32();
            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
            let made = e
                .call(
                    AUDIO_SPAWN_AT,
                    &args![audio, temporary, id, primitive_flag | 0x2012, x, y, z, 0u32],
                )
                .u32();
            e.call(HANDLE_ASSIGN, &args![handle, made]);
            e.call(HANDLE_DESTRUCT, &args![temporary]);
            let [x, y, z] = choice.direction;
            e.call(HANDLE_SET_VECTOR, &args![handle, x, y, z]);
        } else if positional != 0 {
            let at = e.vcall(this, SLOT_POSITION_PTR, &args![]).u32();
            let [x, y, z] = position_words(e, at);
            let id = e.call(FORM_ID, &args![sound]).u32();
            let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
            let made = e
                .call(
                    AUDIO_SPAWN_AT,
                    &args![audio, temporary, id, positional | 0x2002, x, y, z, 0u32],
                )
                .u32();
            e.call(HANDLE_ASSIGN, &args![handle, made]);
            e.call(HANDLE_DESTRUCT, &args![temporary]);
        } else {
            // TESObjectREFR::pRandomSound (+0x1c)
            e.mem.set_u32(this + 0x1c, sound);
        }
        e.mem.free(temporary);
        let list = extra_list_of(e, this);
        e.call(EXTRA_SET_SOUND, &args![list, handle]);
    }
    e.call(HANDLE_DESTRUCT, &args![handle]);
    e.mem.free(handle);
}

// Translated from 0057ada0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the physics-shaped parts of a reference that carries a
/// primitive extra (`ExtraDataList::GetPrimitive`): for the base forms at
/// `011ca234` / `011ca23c` the box's x and z size and the primitive's
/// position and rotation go to the object `00422120` / `00420dd0` finds
/// (`00416a30`, `004169f0`, `00416a70`); for those at `011ca238` /
/// `011ca230` they go to the node of the shape `00420ed0` / `00422020` finds
/// (virtuals `+0x8c`, `+0xb8`, `+0x14` and `00439680`, `004396b0`,
/// `004addc0`). A final zero vector is handed to `00a59c60`.
pub fn fn_0057ada0(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let list = extra_list_of(e, me);
    let primitive = e.call(EXTRA_GET_PRIMITIVE, &args![list]).u32();
    if primitive == 0 {
        return;
    }
    let base_234 = e.global::<u32>(GLOBAL_BASE_FORM_234);
    let base_23c = e.global::<u32>(GLOBAL_BASE_FORM_23C);
    let base_238 = e.global::<u32>(GLOBAL_BASE_FORM_238);
    let base_230 = e.global::<u32>(GLOBAL_BASE_FORM_230);
    if base_form_of(e, me) == base_234 || base_form_of(e, me) == base_23c {
        let is_234 = base_form_of(e, me) == base_234;
        let list = extra_list_of(e, me);
        let target = if is_234 {
            e.call(0x0042_2120, &args![list]).u32()
        } else {
            e.call(0x0042_0dd0, &args![list]).u32()
        };
        if target != 0 {
            primitive_to_target(e, primitive, target);
        }
    } else if base_form_of(e, me) == base_238 || base_form_of(e, me) == base_230 {
        let is_238 = base_form_of(e, me) == base_238;
        let target = if is_238 {
            let list = extra_list_of(e, me);
            let shape_owner = e.call(0x0042_0ed0, &args![list]).u32();
            if shape_owner == 0 {
                return;
            }
            e.call(0x0066_29f0, &args![shape_owner]).u32()
        } else {
            let list = extra_list_of(e, me);
            e.call(0x0042_2020, &args![list]).u32()
        };
        if target != 0 {
            let node = e.call(0x0043_b230, &args![target]).u32();
            if node != 0 {
                primitive_to_shape_node(e, me, primitive, target, node);
            }
        }
    }
    let zero = e.mem.alloc(12);
    e.call(0x0043_d410, &args![zero, 0.0f32, 0u32, 0u32]);
    let helper = e.call(0x0045_0b80, &args![0u32]).u32();
    e.call(0x00a5_9c60, &args![helper, zero]);
    e.mem.free(zero);
}

/// First branch of `0057ada0`: the primitive's box size (x and z), position
/// and rotation go to `target`.
fn primitive_to_target(e: &mut Engine, primitive: u32, target: u32) {
    let scratch = e.mem.alloc(0x30);
    let first = e.call(0x0041_3fc0, &args![primitive, scratch]).u32();
    let z = e.mem.f32(first + 8);
    let second = e.call(0x0041_3fc0, &args![primitive, scratch + 0x0c]).u32();
    let x = e.mem.f32(second);
    let made = e.call(0x0045_2dc0, &args![scratch + 0x18, x, z]).u32();
    e.call(0x0041_6a30, &args![target, made]);
    let holder = e.call(0x0056_c7f0, &args![primitive]).u32();
    let position = e.call(0x0045_bb80, &args![holder]).u32();
    e.call(0x0041_69f0, &args![target, position]);
    let holder = e.call(0x0056_c7f0, &args![primitive]).u32();
    let rotation = e.call(0x006a_9540, &args![holder]).u32();
    e.call(0x0041_6a70, &args![target, rotation]);
    e.mem.free(scratch);
}

/// Second branch of `0057ada0`: moves the shape node `node` of `target`
/// to the primitive's position, by the virtual `+0x8c` (1 means the plain
/// form).
fn primitive_to_shape_node(e: &mut Engine, me: u32, primitive: u32, target: u32, node: u32) {
    let scratch = e.mem.alloc(0x40);
    let size = scratch;
    let position = scratch + 0x10;
    let reference_position = scratch + 0x20;
    let linked = scratch + 0x30;
    e.call(0x0041_3fc0, &args![primitive, size]);
    let holder = e.call(0x0056_c7f0, &args![primitive]).u32();
    let primitive_position = e.call(0x0045_bb80, &args![holder]).u32();
    let words = position_words(e, primitive_position);
    put_words(e, position, words);
    let at = e.call(0x0043_0830, &args![me]).u32();
    let words = position_words(e, at);
    put_words(e, reference_position, words);
    let zero_matches = e
        .call(0x0043_90c0, &args![reference_position, ZERO_VECTOR])
        .bool();
    if zero_matches {
        if e.vcall(node, 0x8c, &args![]).i32() == 1 {
            e.call(0x0043_9680, &args![node, size]);
            e.vcall(node, 0xb8, &args![position]);
        } else {
            let at = e.call(0x0043_0830, &args![me]).u32();
            let words = position_words(e, at);
            put_words(e, linked, words);
            let result = e.vcall(primitive, 0x14, &args![linked]).u32();
            e.call(0x004a_ddc0, &args![target, result]);
        }
    } else if e.vcall(node, 0x8c, &args![]).i32() == 1 {
        let result = e.vcall(primitive, 0x14, &args![reference_position]).u32();
        e.call(0x004a_ddc0, &args![target, result]);
    } else {
        e.call(0x0043_9680, &args![node, size]);
        e.vcall(node, 0xb8, &args![position]);
        let holder = e.call(0x0056_c7f0, &args![primitive]).u32();
        let rotation = e.call(0x006a_9540, &args![holder]).u32();
        e.call(0x0043_96b0, &args![node, rotation]);
    }
    e.mem.free(scratch);
}

// Translated from 0057b0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetRelevantWaterHeight` (Xbox PDB): the loaded data's
/// `fRelevantWaterHeight`, or the default height when the reference has no
/// loaded data.
pub fn tes_object_refr_get_relevant_water_height(e: &mut Engine, this: Ptr) -> f32 {
    let loaded = e.get(this.cast::<TESObjectREFR>(), TESObjectREFR::pLoadedData);
    if loaded.is_null() {
        e.global::<f32>(LOADED_DATA_DEFAULT_HEIGHT)
    } else {
        e.get(
            loaded.cast::<LOADED_REF_DATA>(),
            LOADED_REF_DATA::fRelevantWaterHeight,
        )
    }
}

// Translated from 0057b0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Refreshes the loaded data's water state: `pCurrentWaterObject` is
/// replaced by what `0042f030(extra list, a, b)` returns; with such an
/// object the height is its position's z (virtual `+0x1f4`), otherwise the
/// parent cell's (`005471e0`, when `004518e0` holds) or the default height.
/// For an actor (virtual `+0x100`) with a `009306d0` object, passes the new
/// height on to it (`0057b1d0`).
pub fn fn_0057b0d0(e: &mut Engine, this: Ptr, water_arg: u32, flag: u8) {
    let me = this.addr();
    let refr = this.cast::<TESObjectREFR>();
    if !e.get(refr, TESObjectREFR::pLoadedData).is_null() {
        let list = extra_list_of(e, me);
        let found = e
            .call(0x0042_f030, &args![list, water_arg, u32::from(flag)])
            .u32();
        let loaded = e.get(refr, TESObjectREFR::pLoadedData);
        e.mem.set_u32(loaded.addr(), found);
        let loaded = e.get(refr, TESObjectREFR::pLoadedData);
        let water = e.mem.u32(loaded.addr());
        if water != 0 {
            let position = e.vcall(water, SLOT_POSITION_PTR, &args![]).u32();
            let z = e.mem.f32(position + 8);
            let loaded = e.get(refr, TESObjectREFR::pLoadedData);
            e.mem.set_f32(loaded.addr() + 8, z);
        } else {
            let height = if parent_cell_of(e, me) != 0 && {
                let cell = parent_cell_of(e, me);
                e.call(0x0045_18e0, &args![cell]).bool()
            } {
                let cell = parent_cell_of(e, me);
                e.call(0x0054_71e0, &args![cell]).f32()
            } else {
                e.global::<f32>(LOADED_DATA_DEFAULT_HEIGHT)
            };
            let loaded = e.get(refr, TESObjectREFR::pLoadedData);
            e.mem.set_f32(loaded.addr() + 8, height);
        }
    }
    if is_actor_ref(e, me) {
        let process = e.call(0x0093_06d0, &args![me]).u32();
        if process != 0 {
            let height = tes_object_refr_get_relevant_water_height(e, this);
            fn_0057b1d0(e, Ptr::new(process), height);
        }
    }
}

// Translated from 0057b1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `004a3e90(height)` (a conversion of the water height) in the
/// `float` at `this + 0x53c`.
pub fn fn_0057b1d0(e: &mut Engine, this: Ptr, height: f32) {
    let converted = e.call(0x004a_3e90, &args![height]).f32();
    e.mem.set_f32(this.addr() + 0x53c, converted);
}

// Translated from 0057b200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the loaded data's `iUnderwaterCount` is above 0 (false without
/// loaded data).
pub fn fn_0057b200(e: &mut Engine, this: Ptr) -> bool {
    let loaded = e.get(this.cast::<TESObjectREFR>(), TESObjectREFR::pLoadedData);
    !loaded.is_null()
        && e.get(
            loaded.cast::<LOADED_REF_DATA>(),
            LOADED_REF_DATA::iUnderwaterCount,
        ) > 0
}

// Translated from 0057b240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Counts the reference into (`entering` set) or out of the water: adds or
/// subtracts one from the loaded data's `iUnderwaterCount` (not below 0),
/// clears `pCurrentWaterObject` and then, for a count of 0, takes the water
/// object from the extra of type `0x7e`, and for a count of 1 the first
/// water object of the parent cell's list that `00452440` accepts. A
/// non-actor that is not a door gets `00484a10(this, entering)` called for
/// the two counts.
pub fn fn_0057b240(e: &mut Engine, this: Ptr, entering: u8) {
    let me = this.addr();
    let refr = this.cast::<TESObjectREFR>();
    let loaded = e.get(refr, TESObjectREFR::pLoadedData);
    if loaded.is_null() {
        return;
    }
    let loaded = loaded.cast::<LOADED_REF_DATA>();
    let delta = if entering != 0 { 1 } else { -1 };
    let mut count = e.get(loaded, LOADED_REF_DATA::iUnderwaterCount) + delta;
    e.set(loaded, LOADED_REF_DATA::iUnderwaterCount, count);
    if count < 0 {
        count = 0;
        e.set(loaded, LOADED_REF_DATA::iUnderwaterCount, 0);
    }
    e.set(loaded, LOADED_REF_DATA::pCurrentWaterObject, Ptr::NULL);
    match count {
        0 => {
            let list = extra_list_of(e, me);
            let extra = e.call(0x0041_0220, &args![list, 0x7eu32]).u32();
            let water = if extra != 0 {
                let inner = e.mem.u32(extra + 0x1c);
                e.call(FORM_ID, &args![inner + 4]).u32()
            } else {
                0
            };
            let loaded = e
                .get(refr, TESObjectREFR::pLoadedData)
                .cast::<LOADED_REF_DATA>();
            e.set(
                loaded,
                LOADED_REF_DATA::pCurrentWaterObject,
                Ptr::new(water),
            );
            if !is_actor_ref(e, me) && base_form_type(e, me) != 0x1c {
                e.call(0x0048_4a10, &args![me, 0u32]);
            }
        }
        1 => {
            let list = if parent_cell_of(e, me) != 0 {
                let cell = parent_cell_of(e, me);
                e.call(0x0054_5710, &args![cell]).u32()
            } else {
                0
            };
            let mut node = list;
            while node != 0 {
                let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
                if e.mem.u32(slot) == 0 {
                    break;
                }
                let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
                let water = e.mem.u32(slot);
                let base = base_form_of(e, water);
                let cast = e
                    .call(
                        DYNAMIC_CAST,
                        &args![
                            base,
                            0u32,
                            TYPE_TES_BOUND_OBJECT,
                            TYPE_BOUND_OBJECT_TARGET_B,
                            0u32
                        ],
                    )
                    .u32();
                if e.call(0x0045_2440, &args![cast]).bool() {
                    let loaded = e
                        .get(refr, TESObjectREFR::pLoadedData)
                        .cast::<LOADED_REF_DATA>();
                    e.set(
                        loaded,
                        LOADED_REF_DATA::pCurrentWaterObject,
                        Ptr::new(water),
                    );
                    break;
                }
                node = e.call(LIST_NEXT, &args![node]).u32();
            }
            if !is_actor_ref(e, me) && base_form_type(e, me) != 0x1c {
                e.call(0x0048_4a10, &args![me, 1u32]);
            }
        }
        _ => {}
    }
}

// Translated from 0057b410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the reference has a base form whose virtual `+0xb8` says yes.
pub fn fn_0057b410(e: &mut Engine, this: Ptr) -> bool {
    let me = this.addr();
    if base_form_of(e, me) == 0 {
        return false;
    }
    let base = base_form_of(e, me);
    e.vcall(base, 0xb8, &args![]).bool()
}

// ======================================================================
// Third batch: `0057b460` to `0057ca90` (the next 40 open functions).
// ======================================================================

/// Type descriptor `0043b300` is asked about in `0057bd80` (a bounded
/// scene-graph node class).
const TYPE_BOUNDED_NODE: u32 = 0x0120_2e74;
/// Type descriptors of the two casts in `0057bd80` (the common source type
/// and the two target types).
const TYPE_BOUND_SOURCE: u32 = 0x0118_85fc;
const TYPE_BOUND_TARGET_A: u32 = 0x0118_85dc;
const TYPE_BOUND_TARGET_B: u32 = 0x0118_861c;
/// Doubles of `0057bd80`: the "is rotated" threshold (about 1e-5), pi / 2,
/// 3 pi / 2, the threshold of the third-quarter test (about 1e-5), the
/// threshold of the full-turn test (1e-4) and 2 pi.
const ROTATION_THRESHOLD: u32 = 0x0103_0f40;
const HALF_PI: u32 = 0x0103_0f38;
const THREE_HALF_PI: u32 = 0x0103_0f30;
const QUARTER_TURN_THRESHOLD: u32 = 0x0101_dae8;
const FULL_TURN_THRESHOLD: u32 = 0x0103_0f28;
const TWO_PI: u32 = 0x0101_ff48;
/// The default `fRelevantWaterHeight` of a reference without water (`float`).
const NO_WATER_HEIGHT: u32 = 0x0101_5f5c;
/// `float` -1.0 (initial `fCachedRadius` of loaded data).
const NO_CACHED_RADIUS: u32 = 0x0101_2054;
/// "'%s' :%s", the subtitle format of `0057b7c0`.
const SUBTITLE_FORMAT: u32 = 0x0103_0f1c;
/// The completion callback `0057b7c0` hands to the sound handle.
const SAY_COMPLETION_CALLBACK: u32 = 0x0057_90f0;

/// The object (global) whose state `0045ce80` sets (and `0043d4d0` reads);
/// `0057b7c0` raises it to its maximum for the duration.
const OBJECT_LOCK_STATE: u32 = 0x011c_d378;
/// The object `ProcessLists::SortActorsCloseToPlayer` (`0096e570`) is
/// called on.
const OBJECT_PROCESS_LISTS: u32 = 0x011e_0e80;
/// Type number of the extra data `0057b520` removes (read from the extra
/// list by `GetExtraData`).
const EXTRA_TYPE_WATER: u32 = 0x7e;
/// `TESWaterSystem::RemoveTESObjectFromWaterGroup` (Xbox PDB).
const WATER_REMOVE_FROM_GROUP: u32 = 0x004e_5df0;
/// The getter `0070ec90(TES)` returns the water system from.
const GET_WATER_SYSTEM: u32 = 0x0070_ec90;

/// Vtables the constructors and destructors below store (their classes are
/// named by the run-time type information):
/// `BSMap<TESObjectREFR*,bool>`, `NiTPointerMap<TESObjectREFR*,bool>`,
/// `NiTPointerMap<unsigned int,bool>`, the `NiTPrimitiveArray` of
/// `BSAnimNoteReceiver` types, and the two `NiTMapBase` instances.
const VTABLE_BS_MAP_REFR_BOOL: u32 = 0x0103_0f4c;
const VTABLE_NI_POINTER_MAP_REFR_BOOL: u32 = 0x0103_0f6c;
const VTABLE_NI_POINTER_MAP_UINT_BOOL: u32 = 0x0103_0f8c;
const VTABLE_NI_PRIMITIVE_ARRAY_NOTE_TYPE: u32 = 0x0103_0fac;
const VTABLE_NI_MAP_BASE_REFR_BOOL: u32 = 0x0103_0fb4;
const VTABLE_NI_MAP_BASE_UINT_BOOL: u32 = 0x0103_0fd4;

/// `operator delete` of the game's allocator (cdecl, one argument).
const FREE_OBJECT: u32 = 0x0040_1030;
/// The allocator the hash tables take their buckets from, and its release.
const ALLOCATE_BUCKETS: u32 = 0x00aa_1070;
const FREE_BUCKETS: u32 = 0x00aa_10f0;

fn free_if_flagged(e: &mut Engine, object: Ptr, flags: u32) {
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![object]);
    }
}

/// Whether the reference's base form has form type `0x1c` and the
/// reference's own form flags (`0044ddc0`, the dword at `this + 8`) contain
/// `mask`.
fn base_type_1c_with_flag(e: &mut Engine, me: u32, mask: u32) -> bool {
    if base_form_of(e, me) == 0 {
        return false;
    }
    if base_form_type(e, me) != 0x1c {
        return false;
    }
    e.call(0x0044_ddc0, &args![me]).u32() & mask != 0
}

// Translated from 0057b460 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the reference's base form has form type `0x1c` and the
/// reference's form flags have bit `0x100`.
pub fn fn_0057b460(e: &mut Engine, this: Ptr) -> bool {
    base_type_1c_with_flag(e, this.addr(), 0x100)
}

// Translated from 0057b4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the reference's base form has form type `0x1c` and the
/// reference's form flags have bit `0x40`.
pub fn fn_0057b4b0(e: &mut Engine, this: Ptr) -> bool {
    base_type_1c_with_flag(e, this.addr(), 0x40)
}

// Translated from 0057b500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `0042f2d0` on the reference's extra data list and answers its value
/// (the exe ends on that call and leaves its `EAX`; the caller tests `AL`).
pub fn fn_0057b500(e: &mut Engine, this: Ptr) -> bool {
    let list = extra_list_of(e, this.addr());
    e.call(0x0042_f2d0, &args![list]).bool()
}

// Translated from 0057b520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::RemoveFromAllWater` (Xbox PDB): takes the reference out
/// of the water groups it is in. With `flag` 0 it walks the water-zone map
/// of the extra of type `0x7e` and removes the reference from the group of
/// every zone; otherwise it removes the parent cell's target from the
/// zone listener. Then it drops the extra, removes the reference from the
/// default group when the cell has water, tells the water system and the
/// character controller of an actor, and clears the loaded data's current
/// water object and underwater count.
pub fn tes_object_refr_remove_from_all_water(e: &mut Engine, this: Ptr, flag: u8) {
    let me = this.addr();
    let tes = global_word(e, GLOBAL_TES);
    let water = e.call(GET_WATER_SYSTEM, &args![tes]).u32();
    let list = extra_list_of(e, me);
    let extra = e.call(EXTRA_GET_DATA, &args![list, EXTRA_TYPE_WATER]).u32();
    if flag == 0 {
        if extra != 0 {
            let list = extra_list_of(e, me);
            let zones = e.call(0x0042_f1d0, &args![list]).u32();
            if zones != 0 {
                let first = e.call(0x004b_9ba0, &args![zones]).u32();
                // iterator, key and value slots
                e.with_stack(12, |e, slots| {
                    let at = slots.addr();
                    e.mem.set_u32(at, first);
                    while e.mem.u32(at) != 0 {
                        e.call(0x006b_7f20, &args![zones, at, at + 4, at + 8]);
                        let key = e.mem.u32(at + 4);
                        fn_0057b700(e, Ptr::new(key), me);
                        let height = e.mem.f32(key + 0x18);
                        let zone_id = e.call(FORM_ID, &args![key + 4]).u32();
                        e.call(WATER_REMOVE_FROM_GROUP, &args![water, zone_id, me, height]);
                    }
                });
            }
        }
    } else {
        let cell = e.call(GET_PARENT_CELL, &args![me]).u32();
        if cell != 0 {
            let cell = e.call(GET_PARENT_CELL, &args![me]).u32();
            let target = e.call(0x0045_43c0, &args![cell]).u32();
            if target != 0 {
                let listener = e.call(0x0059_bb30, &args![target]).u32();
                if listener != 0 {
                    e.call(0x0062_0130, &args![listener + 0x14, me]);
                    e.call(0x0063_1370, &args![listener, me, 0u32, 1u32]);
                }
            }
        }
    }
    if extra != 0 {
        let list = extra_list_of(e, me);
        e.call(EXTRA_REMOVE, &args![list, extra, 1u32]);
    }
    // pParentCell (Xbox PDB) +0x40
    let cell = e.mem.u32(me + 0x40);
    if cell != 0 && e.call(0x0045_18e0, &args![cell]).bool() {
        let height = tes_object_refr_get_relevant_water_height(e, this);
        e.call(WATER_REMOVE_FROM_GROUP, &args![water, 0u32, me, height]);
    }
    let tes = global_word(e, GLOBAL_TES);
    if e.call(GET_WATER_SYSTEM, &args![tes]).u32() != 0 {
        let tes = global_word(e, GLOBAL_TES);
        let system = e.call(GET_WATER_SYSTEM, &args![tes]).u32();
        e.call(0x004e_5fe0, &args![system, me]);
    }
    if e.vcall(me, SLOT_IS_ACTOR, &args![]).bool() {
        let controller = e.call(0x0093_06d0, &args![me]).u32();
        if controller != 0 {
            let height = tes_object_refr_get_relevant_water_height(e, this);
            fn_0057b1d0(e, Ptr::new(controller), height);
        }
    }
    // pLoadedData (Xbox PDB) +0x64: pCurrentWaterObject and iUnderwaterCount
    let loaded = e.mem.u32(me + 0x64);
    if loaded != 0 {
        e.mem.set_u32(loaded + 4, 0);
        e.mem.set_u32(loaded, 0);
    }
}

// Translated from 0057b700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks up `form_id` with `0057b730` and hands the result to `0061ff30`.
pub fn fn_0057b700(e: &mut Engine, this: Ptr, form_id: u32) {
    let found = fn_0057b730(e, this, form_id);
    e.call(0x0061_ff30, &args![this, found]);
}

// Translated from 0057b730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `0057b750` on the object held at `this + 0x14`.
pub fn fn_0057b730(e: &mut Engine, this: Ptr, form_id: u32) -> u32 {
    let held = e.mem.u32(this.addr() + 0x14);
    fn_0057b750(e, Ptr::new(held), form_id)
}

// Translated from 0057b750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks up the element with form ID `form_id` in the list at `this + 0x10`
/// (`0057c400`) and returns the address 0x10 bytes before it, or 0.
pub fn fn_0057b750(e: &mut Engine, this: Ptr, form_id: u32) -> u32 {
    let found = fn_0057c400(e, Ptr::new(this.addr() + 0x10), form_id);
    if found == 0 {
        0
    } else {
        found - 0x10
    }
}

// Translated from 0057b790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the reference's parent cell (`this + 0x40`) is loaded
/// (`TES::IsCellLoaded` of the TES singleton, second argument 0); false
/// without a cell.
pub fn fn_0057b790(e: &mut Engine, this: Ptr) -> bool {
    let cell = e.mem.u32(this.addr() + 0x40);
    if cell == 0 {
        return false;
    }
    let tes = global_word(e, GLOBAL_TES);
    e.call(0x0045_11e0, &args![tes, cell, 0u32]).bool()
}

// Translated from 0057b7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes the reference say `topic`. The game's dialogue lock
/// (`0045ce80` on the global at `011cd378`) is raised to its maximum for
/// the duration.
///
/// An actor with a process creates the dialogue item
/// (`TESTopic::CreateDialogueItem`, `0061b320`), makes the player its
/// speaking target (`0057bd60`), starts the topic on the process (virtual
/// `0x2a4`), runs the first response's results and, when the process says
/// so (virtual `0x30c`), shows the subtitle (`Interface::ShowSubtitle`,
/// `00705210`, with the `'%s' :%s` form when `00703150` says so). Any other
/// reference with a 3D model sets the say-to topic and info (`0057ad20`,
/// `0057ace0`) and starts the line through its virtual `0x13c`, with
/// `005790f0` as the completion callback of the sound handle. The
/// reference's virtual `0xd0` is called with 1 and
/// `ProcessLists::SortActorsCloseToPlayer` runs at the end. The C++
/// exception frame and the stack-cookie check are not translated. The third
/// stack word (0 at every caller: 005faa7c, 00608407, 0060841c, 00756e4c,
/// 00756e5e) is never read; `RET 0xc` (0057bd02) pops it.
pub fn fn_0057b7c0(e: &mut Engine, this: Ptr, topic: u32, flag: u8, _unused_3: u32) {
    let me = this.addr();
    let lock = e.call(0x0043_d4d0, &args![OBJECT_LOCK_STATE]).u32();
    let saved = e.mem.u32(lock);
    e.call(0x0045_ce80, &args![OBJECT_LOCK_STATE, 0x7fff_ffffu32]);
    if e.vcall(me, SLOT_IS_ACTOR, &args![]).bool() {
        say_topic_as_actor(e, me, topic, flag);
    } else {
        say_topic_as_object(e, me, topic);
    }
    e.vcall(me, 0xd0, &args![1u32]);
    e.call(0x0045_ce80, &args![OBJECT_LOCK_STATE, saved]);
    e.call(0x0096_e570, &args![OBJECT_PROCESS_LISTS]);
}

fn process_of(e: &mut Engine, actor: u32) -> u32 {
    e.call(0x008d_8520, &args![actor]).u32()
}

/// The actor half of `0057b7c0`.
fn say_topic_as_actor(e: &mut Engine, actor: u32, topic: u32, flag: u8) {
    if process_of(e, actor) == 0 {
        return;
    }
    if e.vcall(actor, 0x22c, &args![0u32]).bool() {
        return;
    }
    let player = global_word(e, GLOBAL_PLAYER);
    let dialogue = e
        .call(0x0061_b320, &args![topic, actor, player, 0u32, 0u32, 1u32])
        .u32();
    e.mem.set_u8(actor + 0x7f, 1);
    fn_0057bd60(e, Ptr::new(actor), player);
    if e.vcall(actor, 0x214, &args![]).u32() == 0 {
        let process = process_of(e, actor);
        if e.vcall(process, 0x30c, &args![]).bool() {
            e.call(0x0088_1680, &args![actor, 0u32]);
            let process = process_of(e, actor);
            e.vcall(process, 0x1dc, &args![actor, 1u32]);
        }
    }
    let process = process_of(e, actor);
    e.vcall(
        process,
        0x2a4,
        &args![actor, topic, u32::from(flag), 1u32, 1u32, 0u32],
    );
    if dialogue != 0 {
        e.call(0x0083_c7b0, &args![dialogue]);
        e.call(0x0083_c820, &args![dialogue]);
        e.call(0x0083_c850, &args![dialogue, 0u32]);
        e.call(0x0083_c850, &args![dialogue, 1u32]);
        let list = extra_list_of(e, actor);
        e.call(0x005a_c750, &args![topic, list, 0x40000u32]);
    }
    let process = process_of(e, actor);
    if !e.vcall(process, 0x30c, &args![]).bool() {
        return;
    }
    fn_0057bd40(e, Ptr::new(actor));
    let process = process_of(e, actor);
    let response = e.vcall(process, 0x248, &args![]).u32();
    if response == 0 {
        return;
    }
    let current = e.call(0x0083_c820, &args![response]).u32();
    // a 12-byte record the subtitle call takes by value: built by 0057bd10
    let record = e.mem.alloc(12);
    if e.call(0x0070_3150, &args![]).bool() {
        let holder = e.call(0x0068_15c0, &args![current]).u32();
        let text = first_dword(e, holder);
        let name = e.call(0x0055_d520, &args![actor]).u32();
        let buffer = e.mem.alloc(0x110);
        e.call(SPRINTF, &args![buffer, SUBTITLE_FORMAT, name, text]);
        let position = e.vcall(actor, SLOT_POSITION_PTR, &args![]).u32();
        let [x, y, z] = position_words(e, position);
        fn_0057bd10(e, Ptr::new(record), 0);
        let [r0, r1, r2] = position_words(e, record);
        // 0057ba50 / 0057ba57: the 0 and the player pushed before the position's +0x1f4
        // (no stack word) are the last two words of the cdecl `00705210` (ADD ESP,0x24).
        let player = e.global::<u32>(GLOBAL_PLAYER);
        e.call(
            0x0070_5210,
            &args![buffer, r0, r1, r2, x, y, z, player, 0u32],
        );
        e.mem.free(buffer);
    } else {
        let position = e.vcall(actor, SLOT_POSITION_PTR, &args![]).u32();
        let [x, y, z] = position_words(e, position);
        fn_0057bd10(e, Ptr::new(record), 0);
        let [r0, r1, r2] = position_words(e, record);
        let holder = e.call(0x0068_15c0, &args![current]).u32();
        let text = first_dword(e, holder);
        // 0057baac / 0057bab4: as above, the player and 0 are the last two words.
        let player = e.global::<u32>(GLOBAL_PLAYER);
        e.call(0x0070_5210, &args![text, r0, r1, r2, x, y, z, player, 0u32]);
    }
    e.mem.free(record);
    let process = process_of(e, actor);
    e.vcall(process, 0x24c, &args![0u32]);
}

/// The other half of `0057b7c0` (a reference that is not an actor).
fn say_topic_as_object(e: &mut Engine, me: u32, topic: u32) {
    let model = e.vcall(me, SLOT_GET_3D, &args![]).u32();
    // an unnamed predicate of the reference (virtual 0xfc)
    if !e.vcall(me, 0xfc, &args![]).bool() {
        return;
    }
    if model == 0 {
        let owner = e.call(0x005e_3fa0, &args![me]).u32();
        e.vcall(owner, 0x1d0, &args![]);
        return;
    }
    fn_0057ad20(e, Ptr::new(me), topic);
    let dialogue = e
        .call(0x0061_b320, &args![topic, me, 0u32, 0u32, 0u32, 0u32])
        .u32();
    if dialogue != 0 {
        let info = e.call(FORM_ID, &args![dialogue]).u32();
        fn_0057ace0(e, Ptr::new(me), info);
    }
    let player = global_word(e, GLOBAL_PLAYER);
    // the sound handle (12 bytes) the line's virtual returns
    let handle = e.mem.alloc(16);
    e.vcall(
        me,
        0x13c,
        &args![handle, topic, player, 0u32, 0u32, 0u32, 0u32, 0u32],
    );
    let id = e.call(FORM_ID, &args![me]).u32();
    e.call(
        HANDLE_SET_COMPLETION_CALLBACK,
        &args![handle, SAY_COMPLETION_CALLBACK, id],
    );
    e.call(HANDLE_DESTRUCT, &args![handle]);
    e.mem.free(handle);
}

// Translated from 0057bd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a 12-byte record: first word `first`, byte at +4 and word at
/// +8 cleared. Returns `this`.
pub fn fn_0057bd10(e: &mut Engine, this: Ptr, first: u32) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at, first);
    e.mem.set_u8(at + 4, 0);
    e.mem.set_u32(at + 8, 0);
    this
}

// Translated from 0057bd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `0042ed30` on the reference's extra data list and returns its
/// result.
pub fn fn_0057bd40(e: &mut Engine, this: Ptr) -> u32 {
    let list = extra_list_of(e, this.addr());
    e.call(0x0042_ed30, &args![list]).u32()
}

// Translated from 0057bd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the dword at `this + 0x70`.
pub fn fn_0057bd60(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x70, value);
}

// Translated from 0057bd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks a scene-graph tree and rebuilds the bound of every node of the
/// bounded class (`0043b300(01202e74, node)`) from the node's rotation
/// (see [`rebuild_bound`]). `this` is only handed down the recursion.
/// Returns true when a node of the tree below `node` (found by the
/// recursion on its children) was handled, and otherwise whether `node`
/// itself was.
#[allow(clippy::only_used_in_recursion)] // `this` (ECX) is only handed down, as the game does
pub fn fn_0057bd80(e: &mut Engine, this: Ptr, node: u32) -> bool {
    if node == 0 {
        return false;
    }
    let mut handled = false;
    if e.call(IS_KIND_OF, &args![TYPE_BOUNDED_NODE, node]).bool() {
        rebuild_bound(e, node);
        handled = true;
    }
    let as_node = e.vcall(node, SLOT_AS_NODE, &args![]).u32();
    if as_node != 0 {
        let mut index = 0u32;
        while index < e.call(0x0045_3470, &args![as_node]).u32() {
            let child = e.call(CHILD_AT, &args![as_node, index]).u32();
            if fn_0057bd80(e, this, child) {
                return true;
            }
            index += 1;
        }
    }
    handled
}

/// `|value|` through `00408840` (a `float` in, the result in `ST0`).
fn float_abs(e: &mut Engine, value: f32) -> f64 {
    e.call(0x0040_8840, &args![value]).f64()
}

/// The body of `0057bd80` for a node of the bounded class. The node's
/// rotation (`00461130`) is turned into angles by `00a58550`; when the
/// first angle is not (nearly) zero and is a quarter or three quarters of a
/// turn the first two components of the extent vector (taken from the
/// bound the node's owner has, or the zero vector) are swapped; a rotation
/// that is neither makes a new `BSMultiBoundOBB` (`0x48` bytes) the
/// owner's bound. The bound is placed at the node's position (or its single
/// child's world-bound centre), the extent is scaled by `008d01e0` and
/// applied to the bound objects, and the node's virtual `0xbc` runs.
fn rebuild_bound(e: &mut Engine, node: u32) {
    // scratch block: +0x00 the first angle (and three more outputs of
    // 00a58550), +0x10 the extent vector, +0x20 the centre
    let scratch = e.mem.alloc(0x30);
    let angle_at = scratch;
    let extent_at = scratch + 0x10;
    let centre_at = scratch + 0x20;
    let rotation = e.call(0x0046_1130, &args![node]).u32();
    e.call(
        0x00a5_8550,
        &args![rotation, angle_at, scratch + 4, scratch + 8, scratch + 12],
    );
    let angle = e.mem.f32(angle_at);
    let mut rotated = float_abs(e, angle) > e.global::<f64>(ROTATION_THRESHOLD);
    let bound_owner = e.call(0x0066_29f0, &args![node]).u32();
    let mut bound = e.call(0x0043_b230, &args![bound_owner]).u32();
    let mut first = e
        .call(
            DYNAMIC_CAST,
            &args![bound, 0u32, TYPE_BOUND_SOURCE, TYPE_BOUND_TARGET_A, 0u32],
        )
        .u32();
    let mut second = e
        .call(
            DYNAMIC_CAST,
            &args![bound, 0u32, TYPE_BOUND_SOURCE, TYPE_BOUND_TARGET_B, 0u32],
        )
        .u32();
    let source = if first != 0 {
        e.call(0x0050_0940, &args![first]).u32()
    } else if second != 0 {
        e.call(0x0050_0940, &args![second]).u32()
    } else {
        ZERO_VECTOR
    };
    for i in 0..3 {
        let word = e.mem.u32(source + 4 * i);
        e.mem.set_u32(extent_at + 4 * i, word);
    }
    if rotated {
        let mut angle = angle;
        if angle < 0.0 {
            angle = (f64::from(angle) + e.global::<f64>(TWO_PI)) as f32;
            e.mem.set_f32(angle_at, angle);
        }
        let to_half = (f64::from(angle) - e.global::<f64>(HALF_PI)) as f32;
        let mut swap = float_abs(e, to_half) < e.global::<f64>(ROTATION_THRESHOLD);
        if !swap {
            let to_three_half = (f64::from(angle) - e.global::<f64>(THREE_HALF_PI)) as f32;
            swap = float_abs(e, to_three_half) < e.global::<f64>(QUARTER_TURN_THRESHOLD);
        }
        if swap {
            let (a, b) = (e.mem.u32(extent_at), e.mem.u32(extent_at + 4));
            e.mem.set_u32(extent_at, b);
            e.mem.set_u32(extent_at + 4, a);
            rotated = false;
        }
        let to_full = (f64::from(angle) - e.global::<f64>(TWO_PI)) as f32;
        if float_abs(e, to_full) < e.global::<f64>(FULL_TURN_THRESHOLD) {
            rotated = false;
        }
    }
    if rotated {
        first = 0;
        let memory = e.call(0x00aa_13e0, &args![0x48u32]).u32();
        let created = if memory != 0 {
            e.call(0x00c3_6310, &args![memory]).u32()
        } else {
            0
        };
        second = created;
        let rotation = e.call(0x0046_1130, &args![node]).u32();
        e.call(0x0043_96b0, &args![created, rotation]);
        e.call(0x004a_ddc0, &args![bound_owner, created]);
        bound = created;
    }
    let position = e.call(0x0045_bb80, &args![node]).u32();
    for i in 0..3 {
        let word = e.mem.u32(position + 4 * i);
        e.mem.set_u32(centre_at + 4 * i, word);
    }
    let children = e.call(0x0045_3470, &args![node]).u32();
    let only_child = if children == 1 {
        e.call(CHILD_AT, &args![node, 0u32]).u32()
    } else {
        0
    };
    if only_child != 0 {
        let world_bound = e.call(0x0043_d450, &args![only_child]).u32();
        let centre = e.call(0x0068_15c0, &args![world_bound]).u32();
        for i in 0..3 {
            let word = e.mem.u32(centre + 4 * i);
            e.mem.set_u32(centre_at + 4 * i, word);
        }
    }
    e.vcall(bound, 0xb8, &args![centre_at]);
    let scale = e.call(0x008d_01e0, &args![node]).f32();
    e.call(0x0043_9180, &args![extent_at, scale]);
    if first != 0 {
        e.call(0x0043_9680, &args![first, extent_at]);
    }
    if second != 0 {
        e.call(0x0043_9680, &args![second, extent_at]);
    }
    e.vcall(node, 0xbc, &args![]);
    e.mem.free(scratch);
}

// Translated from 0057c180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::CreateLoadedData` (Xbox PDB): when the reference has no
/// loaded data yet, allocates the 0x1c-byte record (`0057c2a0`), clears the
/// water object and underwater count, sets `fRelevantWaterHeight` to the
/// parent cell's water height (`005471e0`) when the cell has water and to
/// the default height otherwise, `fCachedRadius` to -1, the flags to 0 and
/// the two pointers to null (`0066b0d0`). The C++ exception frame is not
/// translated.
pub fn tes_object_refr_create_loaded_data(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    if e.mem.u32(me + 0x64) != 0 {
        return;
    }
    let memory = e.call(0x0040_1000, &args![0x1cu32]).u32();
    let loaded = if memory != 0 {
        fn_0057c2a0(e, Ptr::new(memory)).addr()
    } else {
        0
    };
    e.mem.set_u32(me + 0x64, loaded);
    // LOADED_REF_DATA (Xbox PDB): pCurrentWaterObject, iUnderwaterCount
    e.mem.set_u32(loaded, 0);
    e.mem.set_u32(loaded + 4, 0);
    let cell = e.call(GET_PARENT_CELL, &args![me]).u32();
    let height = if cell != 0 && e.call(0x0045_18e0, &args![cell]).bool() {
        e.call(0x0054_71e0, &args![cell]).f32()
    } else {
        e.global::<f32>(NO_WATER_HEIGHT)
    };
    // fRelevantWaterHeight +8, fCachedRadius +0xc, iFlags +0x10
    e.mem.set_f32(loaded + 8, height);
    let radius = e.global::<f32>(NO_CACHED_RADIUS);
    e.mem.set_f32(loaded + 0xc, radius);
    e.mem.set_u32(loaded + 0x10, 0);
    e.call(0x0066_b0d0, &args![loaded + 0x14, 0u32]);
    e.call(0x0066_b0d0, &args![loaded + 0x18, 0u32]);
}

// Translated from 0057c2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a `LOADED_REF_DATA`: sets its two `NiPointer`s (`m_spData3D`
/// at +0x14 and `spPhantom` at +0x18) to null with `00633c90`. Returns
/// `this`. The C++ exception frame is not translated.
pub fn fn_0057c2a0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0063_3c90, &args![this.addr() + 0x14, 0u32]);
    e.call(0x0063_3c90, &args![this.addr() + 0x18, 0u32]);
    this
}

// Translated from 0057c300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the reference's loaded data: nulls its two `NiPointer`s
/// (`0066b0d0`), deletes it through its scalar deleting destructor
/// (`0057c370`) and clears `pLoadedData`.
pub fn fn_0057c300(e: &mut Engine, this: Ptr) {
    let me = this.addr();
    let loaded = e.mem.u32(me + 0x64);
    if loaded == 0 {
        return;
    }
    e.call(0x0066_b0d0, &args![loaded + 0x14, 0u32]);
    e.call(0x0066_b0d0, &args![loaded + 0x18, 0u32]);
    let loaded = e.mem.u32(me + 0x64);
    if loaded != 0 {
        fn_0057c370(e, Ptr::new(loaded), 1);
    }
    e.mem.set_u32(me + 0x64, 0);
}

// Translated from 0057c370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `LOADED_REF_DATA`: runs its destructor
/// (`0057c3a0`) and, for bit 0 of `flags`, frees the object. Returns `this`.
pub fn fn_0057c370(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0057c3a0(e, this);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 0057c3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `LOADED_REF_DATA`: releases the `NiPointer`s at +0x18 and
/// +0x14 (`0045cec0`). The C++ exception frame is not translated.
pub fn fn_0057c3a0(e: &mut Engine, this: Ptr) {
    e.call(0x0045_cec0, &args![this.addr() + 0x18]);
    e.call(0x0045_cec0, &args![this.addr() + 0x14]);
}

// Translated from 0057c400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the singly linked list starting at node `this` and returns the
/// first node whose form ID (`0084e3a0`) is `form_id`, or 0.
pub fn fn_0057c400(e: &mut Engine, this: Ptr, form_id: u32) -> u32 {
    let mut node = this.addr();
    while node != 0 && e.call(FORM_ID, &args![node]).u32() != form_id {
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    node
}

// Translated from 0057c440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `BSMap<TESObjectREFR*,bool>`: runs the base constructor
/// `00559b40` with `size` and stores the vtable. Returns `this`.
pub fn fn_0057c440(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    e.call(0x0055_9b40, &args![this, size]);
    e.mem.set_u32(this.addr(), VTABLE_BS_MAP_REFR_BOOL);
    this
}

// Translated from 0057c470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `NiTPointerMap<TESObjectREFR*,bool>`: runs the base
/// constructor `0057c7e0` with `size` and stores the vtable. Returns `this`.
pub fn fn_0057c470(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    fn_0057c7e0(e, this, size);
    e.mem.set_u32(this.addr(), VTABLE_NI_POINTER_MAP_REFR_BOOL);
    this
}

// Translated from 0057c4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMap<TESObjectREFR*,bool>::_scalar_deleting_destructor_` (Xbox PDB):
/// runs the destructor (`0057c780`) and frees the object for bit 0 of
/// `flags`. Returns `this`.
pub fn bs_map_refr_bool_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0057c780(e, this);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 0057c4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<TESObjectREFR*,bool>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor (`0057c8d0`) and frees the object for
/// bit 0 of `flags`. Returns `this`.
pub fn ni_t_pointer_map_refr_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0057c8d0(e, this);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 0057c500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned int,bool>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor (`0057c9d0`) and frees the object for
/// bit 0 of `flags`. Returns `this`.
pub fn ni_t_pointer_map_uint_bool_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_0057c9d0(e, this);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 0057c530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `0057caf0` and then `00483710` on `this`. The C++ exception frame
/// is not translated.
pub fn fn_0057c530(e: &mut Engine, this: Ptr) {
    e.call(0x0057_caf0, &args![this]);
    e.call(0x0048_3710, &args![this]);
}

// Translated from 0057c590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerListBase<NiTPointerAllocator<unsigned int>,NiPointer<BSMultiBound>>::AddTail`
/// (Xbox PDB): takes a new node from `0049fa80`, stores `item` in it
/// through the `NiPointer` assignment `006e5cc0` at node + 8 and links it
/// with `00559a70`.
pub fn ni_t_pointer_list_base_add_tail(e: &mut Engine, this: Ptr, item: u32) {
    let node = e.call(0x0049_fa80, &args![this]).u32();
    e.call(0x006e_5cc0, &args![node + 8, item]);
    e.call(0x0055_9a70, &args![this, node]);
}

// Translated from 0057c5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the first entry of the container `this` that the key `source`
/// matches (`0057c6a0`). When there is one, builds a temporary pointer from
/// it (`004a4500`) and hands that to `00559a40` on `dest`; otherwise hands
/// `source` itself. Destroys the temporary (`0045cec0`) and returns `dest`.
/// The C++ exception frame is not translated.
pub fn fn_0057c5d0(e: &mut Engine, this: Ptr, dest: u32, source: u32) -> u32 {
    let found = fn_0057c6a0(e, this, source, 0);
    // the entry found, and the temporary pointer
    e.with_stack(12, |e, slots| {
        let entry_at = slots.addr();
        let temporary = entry_at + 4;
        e.mem.set_u32(entry_at, found);
        let value = if found == 0 {
            source
        } else {
            e.call(0x004a_4500, &args![this, temporary, entry_at]).u32()
        };
        e.call(0x0055_9a40, &args![dest, value]);
        if found != 0 {
            e.call(0x0045_cec0, &args![temporary]);
        }
    });
    dest
}

// Translated from 0057c6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scans the entries of the container `this` from `cursor` (from its first
/// entry (`00559450`) when 0) and returns the first one for which
/// `004b0460(key, entry)` holds, or 0. `0057cbe0` advances the cursor.
pub fn fn_0057c6a0(e: &mut Engine, this: Ptr, key: u32, cursor: u32) -> u32 {
    let mut cursor = cursor;
    if cursor == 0 {
        cursor = first_dword(e, this.addr());
    }
    // the cursor lives in memory: 0057cbe0 advances it through a pointer
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), cursor);
        loop {
            let current = e.mem.u32(slot.addr());
            if current == 0 {
                return 0;
            }
            let entry = e.call(0x0057_cbe0, &args![this, slot.addr()]).u32();
            if e.call(0x004b_0460, &args![key, entry]).bool() {
                return current;
            }
        }
    })
}

// Translated from 0057c700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `NiTPrimitiveArray` of `BSAnimNoteReceiver` types:
/// runs the base constructor `0057cb70` with the two words and stores the
/// vtable. Returns `this`.
pub fn fn_0057c700(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.call(0x0057_cb70, &args![this, first, second]);
    e.mem
        .set_u32(this.addr(), VTABLE_NI_PRIMITIVE_ARRAY_NOTE_TYPE);
    this
}

// Translated from 0057c730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the entry for the pointer `slot` in the container `this`
/// (`0049c680(slot, 0)`); when there is one returns `0049f590` of it
/// (the entry is passed by address), otherwise the dword `slot` points to.
pub fn fn_0057c730(e: &mut Engine, this: Ptr, slot: u32) -> u32 {
    let found = e.call(0x0049_c680, &args![this, slot, 0u32]).u32();
    if found == 0 {
        return e.mem.u32(slot);
    }
    e.with_stack(4, |e, holder| {
        e.mem.set_u32(holder.addr(), found);
        e.call(0x0049_f590, &args![this, holder.addr()]).u32()
    })
}

// Translated from 0057c780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `BSMap<TESObjectREFR*,bool>`: stores the vtable, runs
/// `00438af0` and then the base destructor `005596a0`. The C++ exception
/// frame is not translated.
pub fn fn_0057c780(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_BS_MAP_REFR_BOOL);
    e.call(0x0043_8af0, &args![this]);
    e.call(0x0055_96a0, &args![this]);
}

/// Constructor body of the two hash-table instances: vtable, hash size,
/// count 0 and a zeroed bucket array of `size` pointers.
fn construct_hash_table(e: &mut Engine, this: Ptr, vtable: u32, size: u32) {
    let me = this.addr();
    e.mem.set_u32(me, vtable);
    e.mem.set_u32(me + 4, size);
    e.mem.set_u32(me + 0xc, 0);
    let bytes = e.mem.u32(me + 4) << 2;
    let buckets = e.call(ALLOCATE_BUCKETS, &args![bytes]).u32();
    e.mem.set_u32(me + 8, buckets);
    let bytes = e.mem.u32(me + 4) << 2;
    e.call(MEMSET, &args![buckets, 0u32, bytes]);
}

// Translated from 0057c7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `NiTMapBase<NiTPointerAllocator<unsigned int>,TESObjectREFR*,bool>`:
/// stores the vtable and the hash size, clears the count and allocates a
/// zeroed bucket array of `size` pointers. Returns `this`.
pub fn fn_0057c7e0(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    construct_hash_table(e, this, VTABLE_NI_MAP_BASE_REFR_BOOL, size);
    this
}

// Translated from 0057c850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks `key` up in the hash table `this`: the bucket is the object's
/// virtual at +4 (hash), entries chain through their first dword, the
/// virtual at +8 compares `key` with the entry's key (+4); on a match the
/// entry's byte at +8 is stored in `*out` and the result is true.
pub fn fn_0057c850(e: &mut Engine, this: Ptr, key: u32, out: u32) -> bool {
    let me = this.addr();
    let bucket = e.vcall(me, 4, &args![key]).u32();
    let table = e.mem.u32(me + 8);
    let mut entry = e.mem.u32(table + 4 * bucket);
    while entry != 0 {
        let entry_key = e.mem.u32(entry + 4);
        if e.vcall(me, 8, &args![key, entry_key]).bool() {
            let value = e.mem.u8(entry + 8);
            e.mem.set_u8(out, value);
            return true;
        }
        entry = e.mem.u32(entry);
    }
    false
}

// Translated from 0057c8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `NiTPointerMap<TESObjectREFR*,bool>`: stores the vtable,
/// runs `00438af0` and the base destructor `0057c930`. The C++ exception
/// frame is not translated.
pub fn fn_0057c8d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_NI_POINTER_MAP_REFR_BOOL);
    e.call(0x0043_8af0, &args![this]);
    fn_0057c930(e, this);
}

// Translated from 0057c930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `NiTMapBase<NiTPointerAllocator<unsigned int>,TESObjectREFR*,bool>`:
/// stores the vtable, runs `00438af0` and frees the bucket array
/// (`00aa10f0`).
pub fn fn_0057c930(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_NI_MAP_BASE_REFR_BOOL);
    e.call(0x0043_8af0, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(FREE_BUCKETS, &args![buckets]);
}

// Translated from 0057c960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `NiTMapBase<NiTPointerAllocator<unsigned int>,unsigned int,bool>`:
/// stores the vtable and the hash size, clears the count and allocates a
/// zeroed bucket array of `size` pointers. Returns `this`.
pub fn fn_0057c960(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    construct_hash_table(e, this, VTABLE_NI_MAP_BASE_UINT_BOOL, size);
    this
}

// Translated from 0057c9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `NiTPointerMap<unsigned int,bool>`: stores the vtable,
/// runs `00438af0` and the base destructor `0057ca30`. The C++ exception
/// frame is not translated.
pub fn fn_0057c9d0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_NI_POINTER_MAP_UINT_BOOL);
    e.call(0x0043_8af0, &args![this]);
    fn_0057ca30(e, this);
}

// Translated from 0057ca30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `NiTMapBase<NiTPointerAllocator<unsigned int>,unsigned int,bool>`:
/// stores the vtable, runs `00438af0` and frees the bucket array
/// (`00aa10f0`).
pub fn fn_0057ca30(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_NI_MAP_BASE_UINT_BOOL);
    e.call(0x0043_8af0, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(FREE_BUCKETS, &args![buckets]);
}

// Translated from 0057ca60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `004b05b0` on `this` and frees the object for bit 0 of `flags`
/// (a scalar deleting destructor). Returns `this`.
pub fn fn_0057ca60(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004b_05b0, &args![this]);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 0057ca90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>,TESObjectREFR*,bool>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor (`0057c930`) and frees the object for
/// bit 0 of `flags`. Returns `this`.
pub fn ni_t_map_base_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0057c930(e, this);
    free_if_flagged(e, this, flags);
    this
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00576870, fn_00576870(Ptr, u32) -> u32),
        entry!(0x005768b0, fn_005768b0(Ptr<TESObjectREFR>)),
        entry!(0x00576d30, fn_00576d30(Ptr) -> bool),
        entry!(0x00576d50, fn_00576d50(Ptr, Ptr)),
        entry!(0x00577070, fn_00577070(Ptr)),
        entry!(0x00577090, fn_00577090(Ptr)),
        entry!(0x005770e0, fn_005770e0(Ptr, u32, u32) -> u32),
        entry!(0x00577110, fn_00577110(Ptr)),
        entry!(0x00577190, fn_00577190(Ptr) -> u32),
        entry!(0x005771d0, fn_005771d0(u32) -> u32),
        entry!(0x005771f0, fn_005771f0(Ptr) -> u32),
        entry!(0x00577250, fn_00577250(Ptr, u8) -> f32),
        entry!(0x00577310, fn_00577310(Ptr) -> f32),
        entry!(0x00577330, tes_object_refr_restore_rag_doll_data(Ptr, u32)),
        entry!(0x00577390, fn_00577390(Ptr, u8)),
        entry!(0x00577450, tes_object_refr_trans_change_callback(Ptr, u32)),
        entry!(
            0x00577b50,
            bhk_rigid_body_gethk_max_linear_velocity(Ptr) -> f32
        ),
        entry!(0x00577bd0, fn_00577bd0(Ptr, f32) -> u32),
        entry!(0x00577bf0, fn_00577bf0(Ptr) -> f32),
        entry!(0x00577c10, fn_00577c10(Ptr) -> f32),
        entry!(0x00577c40, fn_00577c40(Ptr, f32)),
        entry!(0x00577c70, fn_00577c70(Ptr, f32)),
        entry!(0x00577ca0, tes_object_refr_debug_display_callback(Ptr)),
        entry!(0x00577d50, fn_00577d50(Ptr) -> u32),
        entry!(0x00577de0, fn_00577de0(Ptr) -> bool),
        entry!(0x00577e20, tes_object_refr_add_addon_nodes(u32) -> bool),
        entry!(
            0x00578060,
            tes_object_refr_add_master_particle_addon_nodes(u32) -> bool
        ),
        entry!(
            0x00578170,
            tes_object_refr_remove_master_particle_addon_nodes(u32) -> bool
        ),
        entry!(0x00578250, fn_00578250(u32) -> bool),
        entry!(0x00578300, tes_object_refr_remove_addon_nodes(u32) -> bool),
        entry!(0x00578400, tes_object_refr_has_addon_nodes(u32) -> bool),
        entry!(0x005784b0, tes_object_refr_has_addon_flags(u32) -> bool),
        entry!(0x005784f0, fn_005784f0(u32, f32)),
        entry!(
            0x005785e0,
            tes_object_refr_is_an_owner(Ptr, u32, bool) -> bool
        ),
        entry!(0x00578770, fn_00578770(Ptr) -> bool),
        entry!(0x00578790, tes_object_refr_is_owner_evil(Ptr, u32) -> bool),
        entry!(0x00578870, fn_00578870(Ptr, Ptr)),
        entry!(0x00578920, fn_00578920(Ptr, u8)),
        entry!(
            0x00578950,
            tes_object_refr_add_talking_activator_mobile_object_refr(Ptr) -> u32
        ),
        entry!(
            0x00578970,
            tes_object_refr_get_mobile_object_extra(Ptr) -> u32
        ),
        entry!(0x005789b0, tes_object_refr_add_lock_change(Ptr)),
        entry!(
            0x00578a30,
            tes_object_refr_set_last_finished_sequence(Ptr, u32)
        ),
        entry!(
            0x00578a80,
            fn_00578a80(Ptr, Ptr, u32, u32, u8, u8, u8, u32, u8) -> Ptr
        ),
        entry!(0x005790b0, fn_005790b0() -> f32),
        entry!(0x005790d0, fn_005790d0() -> f32),
        entry!(0x005790f0, fn_005790f0(u32, u32)),
        entry!(0x00579160, tes_object_refr_update_sound_call_back(Ptr)),
        entry!(0x00579200, fn_00579200(Ptr) -> bool),
        entry!(0x00579220, fn_00579220(Ptr, f32, u8)),
        entry!(0x00579280, tes_object_refr_get_target_type(Ptr) -> u32),
        entry!(0x00579620, fn_00579620(u32) -> u32),
        entry!(0x00579670, fn_00579670(Ptr) -> bool),
        entry!(0x00579690, fn_00579690(Ptr) -> bool),
        entry!(0x00579ac0, fn_00579ac0(Ptr, u8)),
        entry!(0x0057a2f0, fn_0057a2f0(Ptr) -> bool),
        entry!(0x0057a370, fn_0057a370(Ptr) -> bool),
        entry!(
            0x0057a3c0,
            tes_object_refr_update_addon_node_sounds(Ptr, u8)
        ),
        entry!(0x0057a410, fn_0057a410(Ptr, u32, u8)),
        entry!(0x0057a740, fn_0057a740(Ptr) -> u32),
        entry!(0x0057a770, fn_0057a770(Ptr) -> u32),
        entry!(0x0057a870, fn_0057a870(Ptr, u32) -> Ptr),
        entry!(
            0x0057a8a0,
            extra_anim_note_receiver_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0057a8d0, fn_0057a8d0(Ptr)),
        entry!(0x0057a960, fn_0057a960(Ptr) -> Ptr),
        entry!(0x0057a9d0, fn_0057a9d0(Ptr) -> Ptr),
        entry!(0x0057a9f0, check_within_multi_bound_task_run(Ptr)),
        entry!(0x0057aa90, check_within_multi_bound_task_post_process(Ptr)),
        entry!(0x0057ab50, fn_0057ab50(u32) -> bool),
        entry!(0x0057ab70, fn_0057ab70()),
        entry!(0x0057acc0, fn_0057acc0(u32) -> u32),
        entry!(0x0057ace0, fn_0057ace0(Ptr, u32)),
        entry!(0x0057ad20, fn_0057ad20(Ptr, u32)),
        entry!(0x0057ad60, fn_0057ad60(Ptr, u32)),
        entry!(0x0057ada0, fn_0057ada0(Ptr)),
        entry!(
            0x0057b0a0,
            tes_object_refr_get_relevant_water_height(Ptr) -> f32
        ),
        entry!(0x0057b0d0, fn_0057b0d0(Ptr, u32, u8)),
        entry!(0x0057b1d0, fn_0057b1d0(Ptr, f32)),
        entry!(0x0057b200, fn_0057b200(Ptr) -> bool),
        entry!(0x0057b240, fn_0057b240(Ptr, u8)),
        entry!(0x0057b410, fn_0057b410(Ptr) -> bool),
        entry!(0x0057b460, fn_0057b460(Ptr) -> bool),
        entry!(0x0057b4b0, fn_0057b4b0(Ptr) -> bool),
        entry!(0x0057b500, fn_0057b500(Ptr) -> bool),
        entry!(0x0057b520, tes_object_refr_remove_from_all_water(Ptr, u8)),
        entry!(0x0057b700, fn_0057b700(Ptr, u32)),
        entry!(0x0057b730, fn_0057b730(Ptr, u32) -> u32),
        entry!(0x0057b750, fn_0057b750(Ptr, u32) -> u32),
        entry!(0x0057b790, fn_0057b790(Ptr) -> bool),
        entry!(0x0057b7c0, fn_0057b7c0(Ptr, u32, u8, u32)),
        entry!(0x0057bd10, fn_0057bd10(Ptr, u32) -> Ptr),
        entry!(0x0057bd40, fn_0057bd40(Ptr) -> u32),
        entry!(0x0057bd60, fn_0057bd60(Ptr, u32)),
        entry!(0x0057bd80, fn_0057bd80(Ptr, u32) -> bool),
        entry!(0x0057c180, tes_object_refr_create_loaded_data(Ptr)),
        entry!(0x0057c2a0, fn_0057c2a0(Ptr) -> Ptr),
        entry!(0x0057c300, fn_0057c300(Ptr)),
        entry!(0x0057c370, fn_0057c370(Ptr, u32) -> Ptr),
        entry!(0x0057c3a0, fn_0057c3a0(Ptr)),
        entry!(0x0057c400, fn_0057c400(Ptr, u32) -> u32),
        entry!(0x0057c440, fn_0057c440(Ptr, u32) -> Ptr),
        entry!(0x0057c470, fn_0057c470(Ptr, u32) -> Ptr),
        entry!(
            0x0057c4a0,
            bs_map_refr_bool_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0057c4d0,
            ni_t_pointer_map_refr_bool_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0057c500,
            ni_t_pointer_map_uint_bool_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0057c530, fn_0057c530(Ptr)),
        entry!(0x0057c590, ni_t_pointer_list_base_add_tail(Ptr, u32)),
        entry!(0x0057c5d0, fn_0057c5d0(Ptr, u32, u32) -> u32),
        entry!(0x0057c6a0, fn_0057c6a0(Ptr, u32, u32) -> u32),
        entry!(0x0057c700, fn_0057c700(Ptr, u32, u32) -> Ptr),
        entry!(0x0057c730, fn_0057c730(Ptr, u32) -> u32),
        entry!(0x0057c780, fn_0057c780(Ptr)),
        entry!(0x0057c7e0, fn_0057c7e0(Ptr, u32) -> Ptr),
        entry!(0x0057c850, fn_0057c850(Ptr, u32, u32) -> bool),
        entry!(0x0057c8d0, fn_0057c8d0(Ptr)),
        entry!(0x0057c930, fn_0057c930(Ptr)),
        entry!(0x0057c960, fn_0057c960(Ptr, u32) -> Ptr),
        entry!(0x0057c9d0, fn_0057c9d0(Ptr)),
        entry!(0x0057ca30, fn_0057ca30(Ptr)),
        entry!(0x0057ca60, fn_0057ca60(Ptr, u32) -> Ptr),
        entry!(
            0x0057ca90,
            ni_t_map_base_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    /// Targets of the shared doubles: `AsNode` returning its `this`, and
    /// the form-type reader.
    const AS_NODE_SELF: u32 = 0x00ff_0001;
    const RETURNS_TRUE: u32 = 0x00ff_0002;
    const RETURNS_FALSE: u32 = 0x00ff_0003;

    /// An engine with the pages of the globals the translations read mapped
    /// and the scene-graph accessors registered with their real bodies.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x0101_7000,
            0x0101_d000,
            0x0102_0000,
            0x0102_9000,
            0x0102_e000,
            0x0103_0000,
            0x010c_7000,
            0x011c_3000,
            0x011c_a000,
            0x011d_d000,
            0x011d_e000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.register(AS_NODE_SELF, |_, a| a[0].into_ret());
        e.register(RETURNS_TRUE, |_, _| true.into_ret());
        e.register(RETURNS_FALSE, |_, _| false.into_ret());
        // 0043b480 / 0043b4a0: a node's child count at +0x20 and children
        // from +0x24 (the test nodes below are built that way)
        e.register(CHILD_COUNT, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(CHILD_AT, |e, a| {
            e.mem.u32(a[0] + 0x24 + 4 * a[1]).into_ret()
        });
        // 0043b300: every object is of the node type, nothing else
        e.register(IS_KIND_OF, |_, a| (a[0] == NODE_TYPE).into_ret());
        // 005d43c0: the extra data list at this + 0x44
        e.register(GET_EXTRA_LIST, |_, a| (a[0] + 0x44).into_ret());
        // 007af430: pObjectReference at +0x20; 00401170: the form type byte
        e.register(GET_BASE_FORM, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(FORM_TYPE, |e, a| u32::from(e.mem.u8(a[0] + 4)).into_ret());
        // 008d6f30: pParentCell at +0x40
        e.register(GET_PARENT_CELL, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        e
    }

    fn returns(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    fn returns_float(e: &mut Engine, addr: u32, value: f32) {
        e.register_double(addr, move |_, _| Ret {
            st0: f64::from(value),
            ..Ret::default()
        });
    }

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    fn start_log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    fn take_log(e: &mut Engine) -> Log {
        e.call_log.take().expect("log started")
    }

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// An object whose vtable has the given (byte offset, target) slots.
    fn object_with(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(size);
        let vtable = e.mem.alloc(0x400);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        e.mem.set_u32(object, vtable);
        object
    }

    /// A scene-graph node: `AsNode` returns itself, children listed from
    /// +0x24 with their count at +0x20.
    fn node(e: &mut Engine, children: &[u32]) -> u32 {
        let node = object_with(e, 0x80, &[(0x0c, AS_NODE_SELF)]);
        e.mem.set_u32(node + 0x20, children.len() as u32);
        for (i, child) in children.iter().enumerate() {
            e.mem.set_u32(node + 0x24 + 4 * i as u32, *child);
        }
        node
    }

    /// A form object with its type byte at +4 and flags at +8.
    fn form(e: &mut Engine, kind: u8, flags: u32) -> u32 {
        let form = e.mem.alloc(0x100);
        e.mem.set_u8(form + 4, kind);
        e.mem.set_u32(form + 8, flags);
        form
    }

    // ---- 00576870 -------------------------------------------------------

    #[test]
    fn fn_00576870_returns_the_found_element_minus_four() {
        let mut e = engine();
        returns(&mut e, 0x0057_c400, 0x2004);
        start_log(&mut e);
        assert_eq!(
            e.call(0x0057_6870, &args![0x1000u32, 0x77u32]).u32(),
            0x2000
        );
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0057_c400), vec![vec![0x1004, 0x77]]);
        returns(&mut e, 0x0057_c400, 0);
        assert_eq!(e.call(0x0057_6870, &args![0x1000u32, 0x77u32]).u32(), 0);
    }

    // ---- 005768b0 -------------------------------------------------------

    /// Doubles for everything `005768b0` calls, all returning 0/false.
    fn loaded_model_world(e: &mut Engine) {
        stub(
            e,
            &[
                0x0045_3860,
                0x0056_c880,
                0x0056_c8f0,
                0x0056_eab0,
                0x0056_f140,
                0x0056_d7e0,
                0x0086_a830,
                0x0056_4c60,
                0x00c6_a350,
                0x0062_b8d0,
                0x0045_bb20,
                0x004a_3c90,
            ],
        );
        stub(
            e,
            &[
                0x0045_0ff0,
                0x0045_23c0,
                0x0045_23e0,
                0x0045_2440,
                0x0041_fbe0,
                0x0056_4c10,
                0x0042_ce10,
                0x0045_8900,
                0x004b_5260,
                0x0056_0d80,
                0x00c8_12d0,
                0x0044_ddc0,
                0x0062_0b80,
                0x009d_9f40,
                0x0051_7690,
                0x0045_43c0,
            ],
        );
    }

    /// A reference at `this` with the given base form, a 3D and IsActor.
    fn loaded_reference(e: &mut Engine, base: u32, model: u32, actor: bool) -> u32 {
        let is_actor = if actor { RETURNS_TRUE } else { RETURNS_FALSE };
        let base = if base == 0 { form(e, 0x30, 0) } else { base };
        let get_model = 0x00ff_0010;
        e.register_double(get_model, move |_, _| Ret {
            eax: model,
            ..Ret::default()
        });
        let refr = object_with(e, 0x80, &[(0x100, is_actor), (0x1d0, get_model)]);
        e.mem.set_u32(refr + 0x20, base);
        refr
    }

    #[test]
    fn fn_005768b0_stops_for_a_cell_the_two_tests_reject() {
        let mut e = engine();
        loaded_model_world(&mut e);
        let cell = e.mem.alloc(0x40);
        let model = node(&mut e, &[]);
        let refr = loaded_reference(&mut e, 0, model, false);
        e.mem.set_u32(refr + 0x40, cell);
        start_log(&mut e);
        e.call(0x0057_68b0, &args![refr]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0045_0ff0), vec![vec![cell]]);
        assert_eq!(calls_to(&log, 0x0045_23c0), vec![vec![cell]]);
        assert!(calls_to(&log, 0x0056_c880).is_empty());
        // one of the tests accepting lets it go on
        returns(&mut e, 0x0045_23c0, 1);
        start_log(&mut e);
        e.call(0x0057_68b0, &args![refr]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0056_c880), vec![vec![refr, 1, 1]]);
    }

    #[test]
    fn fn_005768b0_without_a_model_only_finishes() {
        let mut e = engine();
        loaded_model_world(&mut e);
        let refr = loaded_reference(&mut e, 0, 0, false);
        start_log(&mut e);
        e.call(0x0057_68b0, &args![refr]);
        let log = take_log(&mut e);
        let addrs: Vec<u32> = log
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| *a != 0x00ff_0010)
            .collect();
        assert_eq!(addrs, vec![0x0057_68b0, 0x0056_c880]);
    }

    #[test]
    fn fn_005768b0_initializes_placeable_water_and_hands_the_model_to_the_world() {
        let mut e = engine();
        loaded_model_world(&mut e);
        let base = form(&mut e, 0x23, 0);
        let model = node(&mut e, &[]);
        let refr = loaded_reference(&mut e, base, model, true);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        returns(&mut e, 0x0045_0ff0, 1);
        // the world of the cell
        e.register_double(0x00ff_0020, |_, _| Ret::default());
        e.register_double(0x00ff_0021, |_, _| Ret::default());
        let world = object_with(&mut e, 0x20, &[(0x94, 0x00ff_0020), (0xd4, 0x00ff_0021)]);
        returns(&mut e, 0x0045_43c0, world);
        start_log(&mut e);
        e.call(0x0057_68b0, &args![refr]);
        let log = take_log(&mut e);
        let tes = e.global::<u32>(GLOBAL_TES);
        assert_eq!(
            calls_to(&log, 0x0045_3860),
            vec![vec![tes, 1], vec![tes, 0]]
        );
        // the placeable-water initializer, then the world takes the model
        assert_eq!(calls_to(&log, 0x0056_c8f0), vec![vec![refr]]);
        assert_eq!(calls_to(&log, 0x00ff_0021), vec![vec![world, model, model]]);
        assert_eq!(calls_to(&log, 0x0086_a830), vec![vec![refr, model]]);
        // no motion change for this base form
        assert!(calls_to(&log, 0x00c6_a350).is_empty());
        // a water reference that 452440 accepts skips the initializer
        returns(&mut e, 0x0045_2440, 1);
        start_log(&mut e);
        e.call(0x0057_68b0, &args![refr]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0056_c8f0).is_empty());
    }

    #[test]
    fn fn_005768b0_picks_the_initializer_by_the_base_form() {
        let mut e = engine();
        loaded_model_world(&mut e);
        let model = node(&mut e, &[]);
        // the base form recorded in the global at 011ca240
        let special = form(&mut e, 1, 0);
        e.set_global(GLOBAL_BASE_FORM_A, special);
        let refr = loaded_reference(&mut e, special, model, false);
        start_log(&mut e);
        e.call(0x0057_68b0, &args![refr]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0056_eab0), vec![vec![refr]]);
        // a form of type 0x0e
        let other = form(&mut e, 0x0e, 0);
        e.mem.set_u32(refr + 0x20, other);
        start_log(&mut e);
        e.call(0x0057_68b0, &args![refr]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0056_f140), vec![vec![refr]]);
        // a form of type 0x15 with a primitive in the extra data
        let trigger = form(&mut e, 0x15, 0);
        e.mem.set_u32(refr + 0x20, trigger);
        returns(&mut e, 0x0041_fbe0, 0x5000);
        start_log(&mut e);
        e.call(0x0057_68b0, &args![refr]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0056_d7e0), vec![vec![refr]]);
        // none of them when 004523e0 already says the work is done
        returns(&mut e, 0x0045_23e0, 1);
        start_log(&mut e);
        e.call(0x0057_68b0, &args![refr]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0056_d7e0).is_empty());
    }

    #[test]
    fn fn_005768b0_sets_the_motion_of_special_bases() {
        let mut e = engine();
        loaded_model_world(&mut e);
        let model = node(&mut e, &[]);
        let special = form(&mut e, 0x30, 0);
        e.set_global(GLOBAL_BASE_FORM_B, special);
        let refr = loaded_reference(&mut e, special, model, true);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        returns(&mut e, 0x0045_0ff0, 1);
        e.register(0x00ff_0021, |_, _| Ret::default());
        let world = object_with(&mut e, 0x20, &[(0xd4, 0x00ff_0021), (0x94, 0x00ff_0021)]);
        returns(&mut e, 0x0045_43c0, world);
        start_log(&mut e);
        e.call(0x0057_68b0, &args![refr]);
        let log = take_log(&mut e);
        // 00564c10 is false and there is no body: the motion is set
        assert_eq!(calls_to(&log, 0x00c6_a350), vec![vec![model, 5, 1, 0, 1]]);
        // the world's body is checked against the body of the target
        // (there is none here), so the work goes on
        assert_eq!(calls_to(&log, 0x0086_a830).len(), 1);
        // 00564c10 true: no motion change
        returns(&mut e, 0x0056_4c10, 1);
        start_log(&mut e);
        e.call(0x0057_68b0, &args![refr]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x00c6_a350).is_empty());
        // ... unless the stillness test 00458900 says so
        let collision = e.mem.alloc(0x40);
        returns(&mut e, 0x004b_5260, collision);
        // a body of another world than the cell's
        returns(&mut e, 0x00ff_0040, 5);
        let body = object_with(&mut e, 0x40, &[(0x94, 0x00ff_0040)]);
        returns(&mut e, 0x006f_a820, body);
        returns(&mut e, 0x0042_ce10, 1);
        returns(&mut e, 0x0045_8900, 1);
        returns(&mut e, 0x0045_8b20, 0x0126_7e30);
        start_log(&mut e);
        e.call(0x0057_68b0, &args![refr]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x00c6_a350).len(), 1);
        let threshold = e.global::<u32>(STILL_THRESHOLD);
        assert_eq!(
            calls_to(&log, 0x0045_8900)[0][1..],
            [0x0126_7e30, threshold]
        );
        // the same body as the cell's world: nothing is done
        returns(&mut e, 0x00ff_0040, 0);
        start_log(&mut e);
        e.call(0x0057_68b0, &args![refr]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0086_a830).is_empty());
    }

    #[test]
    fn fn_005768b0_offers_a_model_without_target_to_the_world() {
        let mut e = engine();
        loaded_model_world(&mut e);
        let model = node(&mut e, &[]);
        let refr = loaded_reference(&mut e, 0, model, false);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        returns(&mut e, 0x0045_0ff0, 1);
        // IsActor is false: a found object (00c812d0) without a target
        // (0044ddc0 gives 0)
        returns(&mut e, 0x00c8_12d0, 0x9000);
        e.register(IS_KIND_OF, |_, a| (a[0] == 0x011d_5e70).into_ret());
        e.register_double(0x00ff_0030, |_, _| Ret::default());
        let world = object_with(&mut e, 0x20, &[(0xd0, 0x00ff_0030)]);
        returns(&mut e, 0x0045_43c0, world);
        start_log(&mut e);
        e.call(0x0057_68b0, &args![refr]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, 0x00ff_0030),
            vec![vec![world, model, 0, 0, 0, 0]]
        );
        assert!(calls_to(&log, 0x0086_a830).is_empty());
    }

    // ---- 00576d30 -------------------------------------------------------

    #[test]
    fn fn_00576d30_tests_flag_200000() {
        let mut e = engine();
        let a = form(&mut e, 0, 0x0020_0000);
        let b = form(&mut e, 0, 0x0010_0000);
        assert!(e.call(0x0057_6d30, &args![a]).bool());
        assert!(!e.call(0x0057_6d30, &args![b]).bool());
    }

    // ---- 00576d50 -------------------------------------------------------

    /// The matrix helpers with the bodies the exe gives them.
    fn matrix_helpers(e: &mut Engine) {
        // 004b4cf0: this + index * 16; 00560d30: this + index * 4
        e.register(0x004b_4cf0, |_, a| (a[0] + a[1] * 16).into_ret());
        e.register(0x0056_0d30, |_, a| (a[0] + a[1] * 4).into_ret());
        // 006815c0: returns this; 00538f40: zeroes 16 bytes; 004a3f10: copies
        // 16 bytes from the argument
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0053_8f40, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i, 0);
            }
            Ret::default()
        });
        e.register(0x004a_3f10, |e, a| {
            for i in 0..4 {
                let v = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, v);
            }
            Ret::default()
        });
    }

    fn filled(e: &mut Engine, words: u32, value: u32) -> u32 {
        let block = e.mem.alloc(words * 4);
        for i in 0..words {
            e.mem.set_u32(block + 4 * i, value);
        }
        block
    }

    #[test]
    fn fn_005770e0_addresses_the_element() {
        let mut e = engine();
        matrix_helpers(&mut e);
        let m = e.mem.alloc(0x40);
        // column 2 of row 1
        assert_eq!(e.call(0x0057_70e0, &args![m, 2u32, 1u32]).u32(), m + 16 + 8);
    }

    #[test]
    fn fn_00577110_zeroes_three_rows() {
        let mut e = engine();
        matrix_helpers(&mut e);
        let m = filled(&mut e, 16, 0xdead_beef);
        e.call(0x0057_7110, &args![m]);
        for i in 0..12 {
            assert_eq!(e.mem.u32(m + 4 * i), 0);
        }
        // the fourth row is not touched
        assert_eq!(e.mem.u32(m + 0x30), 0xdead_beef);
    }

    #[test]
    fn fn_00577090_makes_the_identity() {
        let mut e = engine();
        matrix_helpers(&mut e);
        let m = filled(&mut e, 16, 0xdead_beef);
        e.call(0x0057_7090, &args![m]);
        for row in 0..3u32 {
            for column in 0..4u32 {
                let expected = if row == column { 1.0f32 } else { 0.0 };
                assert_eq!(e.mem.f32(m + row * 16 + column * 4), expected);
            }
        }
    }

    #[test]
    fn fn_00577070_adds_a_zero_translation() {
        let mut e = engine();
        matrix_helpers(&mut e);
        let m = filled(&mut e, 18, 0xdead_beef);
        e.call(0x0057_7070, &args![m]);
        assert_eq!(e.mem.f32(m), 1.0);
        assert_eq!(e.mem.f32(m + 0x14), 1.0);
        assert_eq!(e.mem.f32(m + 0x28), 1.0);
        for i in 0..4 {
            assert_eq!(e.mem.u32(m + 0x30 + 4 * i), 0);
        }
        // beyond the 16-byte translation vector nothing changes
        assert_eq!(e.mem.u32(m + 0x40), 0xdead_beef);
    }

    // ---- 00577190, 005771d0, 005771f0 ----------------------------------

    #[test]
    fn fn_005771f0_steps_back_0x10_from_the_virtual_result() {
        let mut e = engine();
        let get_inner = 0x00ff_0050;
        returns(&mut e, get_inner, 0x4010);
        let object = object_with(&mut e, 0x20, &[(0x10, get_inner)]);
        returns(&mut e, 0x004a_e750, object);
        assert_eq!(e.call(0x0057_71f0, &args![0x77u32]).u32(), 0x4000);
        // null virtual result
        returns(&mut e, get_inner, 0);
        assert_eq!(e.call(0x0057_71f0, &args![0x77u32]).u32(), 0);
        // no object
        returns(&mut e, 0x004a_e750, 0);
        assert_eq!(e.call(0x0057_71f0, &args![0x77u32]).u32(), 0);
    }

    #[test]
    fn fn_005771d0_forwards_to_004a3a40() {
        let mut e = engine();
        e.register(0x004a_3a40, |_, a| (a[0] + 1).into_ret());
        assert_eq!(e.call(0x0057_71d0, &args![0x10u32]).u32(), 0x11);
    }

    #[test]
    fn fn_00577190_converts_the_inner_object() {
        let mut e = engine();
        let get_inner = 0x00ff_0050;
        returns(&mut e, get_inner, 0x4010);
        let object = object_with(&mut e, 0x20, &[(0x10, get_inner)]);
        returns(&mut e, 0x004a_e750, object);
        e.register(0x004a_3a40, |_, a| (a[0] + 1).into_ret());
        assert_eq!(e.call(0x0057_7190, &args![0x77u32]).u32(), 0x4001);
        returns(&mut e, 0x004a_e750, 0);
        assert_eq!(e.call(0x0057_7190, &args![0x77u32]).u32(), 0);
    }

    // ---- 00576d50 -------------------------------------------------------

    /// A scene-graph node for `00576d50`: virtual 0xb8 fills the vector, the
    /// children are its `AsNode` result (a node).
    fn walked_node(e: &mut Engine, children: Option<u32>) -> u32 {
        e.register(0x00ff_0060, |e, a| {
            e.mem.set_f32(a[1], 7.0);
            Ret::default()
        });
        e.register(0x00ff_0061, move |_, a| {
            // AsNode: the children container if given, else 0
            let _ = a;
            Ret::default()
        });
        let node = object_with(e, 0x80, &[(0xb8, 0x00ff_0060), (0x0c, 0x00ff_0061)]);
        if let Some(children) = children {
            let vtable = e.mem.u32(node);
            e.mem.set_u32(vtable + 0x0c, AS_NODE_SELF);
            // the node itself is its own child container
            e.mem.set_u32(node + 0x20, 1);
            e.mem.set_u32(node + 0x24, children);
        }
        node
    }

    fn shape_doubles(e: &mut Engine) {
        stub(
            e,
            &[
                0x0043_d410,
                0x0056_ded0,
                0x0056_df80,
                0x00ca_4120,
                0x00ca_36a0,
            ],
        );
        matrix_helpers(e);
        returns(e, 0x0043_b610, 0);
    }

    #[test]
    fn fn_00576d50_does_nothing_for_a_null_node() {
        let mut e = engine();
        shape_doubles(&mut e);
        start_log(&mut e);
        e.call(0x0057_6d50, &args![0x11u32, 0u32]);
        let log = take_log(&mut e);
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn fn_00576d50_converts_the_transform_and_walks_the_children() {
        let mut e = engine();
        shape_doubles(&mut e);
        let transform = filled(&mut e, 13, 0x3f80_0000);
        returns(&mut e, 0x0046_1130, transform);
        let child = walked_node(&mut e, None);
        let root = walked_node(&mut e, Some(child));
        start_log(&mut e);
        e.call(0x0057_6d50, &args![0x11u32, root]);
        let log = take_log(&mut e);
        // the node, then its child (the root is its own children container
        // here)
        let converts = calls_to(&log, 0x0056_df80);
        assert!(converts.len() >= 2);
        // the NiTransform is copied (52 bytes) before the conversion
        let copy = converts[0][1];
        for i in 0..13 {
            assert_eq!(e.mem.u32(copy + 4 * i), 0x3f80_0000);
        }
        // the direction vector starts as (0, 1, 0)
        let direction = calls_to(&log, 0x0043_d410)[0].clone();
        assert_eq!(direction[2..], [1, 0]);
        assert!(calls_to(&log, 0x00ca_4120).is_empty());
    }

    #[test]
    fn fn_00576d50_passes_the_transform_to_the_shape_setters() {
        let mut e = engine();
        shape_doubles(&mut e);
        let transform = filled(&mut e, 13, 1);
        returns(&mut e, 0x0046_1130, transform);
        let node = walked_node(&mut e, None);
        // the vtable's AsNode answers 0: no children
        returns(&mut e, 0x0043_b610, 0x3000);
        returns(&mut e, 0x006f_a820, 0x3100);
        returns(&mut e, 0x004a_e6a0, 0x3200);
        returns(&mut e, 0x0062_0b80, 0x5555);
        // first kind
        e.register(IS_KIND_OF, |_, a| (a[0] == 0x0126_82e8).into_ret());
        start_log(&mut e);
        e.call(0x0057_6d50, &args![0u32, node]);
        let log = take_log(&mut e);
        let hk = calls_to(&log, 0x0056_df80)[0][0];
        assert_eq!(calls_to(&log, 0x00ca_4120), vec![vec![0x5555, hk]]);
        assert!(calls_to(&log, 0x00ca_36a0).is_empty());
        // second kind
        e.register(IS_KIND_OF, |_, a| (a[0] == 0x0126_82dc).into_ret());
        start_log(&mut e);
        e.call(0x0057_6d50, &args![0u32, node]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x00ca_36a0).len(), 1);
        assert!(calls_to(&log, 0x00ca_4120).is_empty());
        // third kind: the wrapped shape decides
        e.register(IS_KIND_OF, |_, a| {
            (a[0] == 0x0126_82d0 || (a[0] == 0x0126_82dc && a[1] == 0x7001)).into_ret()
        });
        let get_inner = 0x00ff_0050;
        returns(&mut e, get_inner, 0x7011);
        let wrapper = object_with(&mut e, 0x20, &[(0x10, get_inner)]);
        e.register_double(0x004a_e750, move |_, _| Ret {
            eax: wrapper,
            ..Ret::default()
        });
        e.register(0x004a_3a40, |_, a| a[0].into_ret());
        start_log(&mut e);
        e.call(0x0057_6d50, &args![0u32, node]);
        let log = take_log(&mut e);
        // 0x7011 - 0x10 = 0x7001 is the inner shape, a second-kind shape
        assert_eq!(calls_to(&log, 0x00ca_36a0).len(), 1);
    }

    // ---- 00577250, 00577310 --------------------------------------------

    /// A reference-like object for `00577250`: IsActor as given.
    fn container_holder(e: &mut Engine, actor: bool) -> u32 {
        let is_actor = if actor { RETURNS_TRUE } else { RETURNS_FALSE };
        object_with(e, 0xe10, &[(0x100, is_actor)])
    }

    #[test]
    fn fn_00577310_reads_the_float_at_0xe04() {
        let mut e = engine();
        let object = e.mem.alloc(0xe10);
        e.mem.set_f32(object + 0xe04, 2.5);
        assert_eq!(e.call(0x0057_7310, &args![object]).f32(), 2.5);
    }

    #[test]
    fn fn_00577250_takes_the_value_from_the_list_or_the_container() {
        let mut e = engine();
        let this = container_holder(&mut e, false);
        // no container: 0.0
        returns(&mut e, 0x0055_d310, 0);
        assert_eq!(e.call(0x0057_7250, &args![this, 1u32]).f32(), 0.0);
        // a container and a list in the extra data
        returns(&mut e, 0x0055_d310, 0x6000);
        returns(&mut e, 0x0041_8520, 0x6100);
        returns_float(&mut e, 0x004d_0900, 3.5);
        start_log(&mut e);
        assert_eq!(e.call(0x0057_7250, &args![this, 1u32]).f32(), 3.5);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x004d_0900), vec![vec![0x6100, 1]]);
        // no list: the container answers
        returns(&mut e, 0x0041_8520, 0);
        returns_float(&mut e, 0x0048_1e10, 2.0);
        start_log(&mut e);
        assert_eq!(e.call(0x0057_7250, &args![this, 0u32]).f32(), 2.0);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0048_1e10), vec![vec![0x6000, 0]]);
    }

    #[test]
    fn fn_00577250_limits_actors() {
        let mut e = engine();
        let actor = container_holder(&mut e, true);
        let player = container_holder(&mut e, true);
        e.set_global(GLOBAL_PLAYER, player);
        returns(&mut e, 0x0055_d310, 0x6000);
        returns(&mut e, 0x0041_8520, 0x6100);
        returns_float(&mut e, 0x004d_0900, 5.0);
        // a cap below the value lowers it
        returns_float(&mut e, 0x008a_0c20, 3.0);
        assert_eq!(e.call(0x0057_7250, &args![actor, 0u32]).f32(), 3.0);
        // a cap above the value leaves it
        returns_float(&mut e, 0x008a_0c20, 9.0);
        assert_eq!(e.call(0x0057_7250, &args![actor, 0u32]).f32(), 5.0);
        // the player: the +0xe04 float is added
        e.mem.set_f32(player + 0xe04, 0.5);
        assert_eq!(e.call(0x0057_7250, &args![player, 0u32]).f32(), 5.5);
    }

    // ---- 00577330 -------------------------------------------------------

    #[test]
    fn restore_rag_doll_data_applies_the_data_to_a_reference_with_3d() {
        let mut e = engine();
        returns(&mut e, 0x0043_fcd0, 0x9000);
        stub(&mut e, &[0x004d_9910, 0x0041_d5b0]);
        returns(&mut e, 0x0041_d6d0, 0x7700);
        let this = e.mem.alloc(0x80);
        start_log(&mut e);
        e.call(0x0057_7330, &args![this, 0x1234u32]);
        // data of the extra list when none is given
        e.call(0x0057_7330, &args![this, 0u32]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, 0x004d_9910),
            vec![vec![0x1234, this], vec![0x7700, this]]
        );
        assert!(calls_to(&log, 0x0041_d5b0).is_empty());
        // nothing to apply
        returns(&mut e, 0x0041_d6d0, 0);
        start_log(&mut e);
        e.call(0x0057_7330, &args![this, 0u32]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x004d_9910).is_empty());
    }

    #[test]
    fn restore_rag_doll_data_stores_the_data_without_3d() {
        let mut e = engine();
        returns(&mut e, 0x0043_fcd0, 0);
        stub(&mut e, &[0x004d_9910, 0x0041_d5b0]);
        let this = e.mem.alloc(0x80);
        start_log(&mut e);
        e.call(0x0057_7330, &args![this, 0x1234u32]);
        e.call(0x0057_7330, &args![this, 0u32]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0041_d5b0), vec![vec![this + 0x44, 0x1234]]);
        assert!(calls_to(&log, 0x004d_9910).is_empty());
    }

    // ---- 00577390 -------------------------------------------------------

    #[test]
    fn fn_00577390_updates_the_primitive_and_the_3d() {
        let mut e = engine();
        e.mem.set_f32(0x0101_df44, 0.25);
        returns(&mut e, 0x0041_fbe0, 0);
        let this = container_holder(&mut e, false);
        let base = form(&mut e, 1, 0);
        e.mem.set_u32(this + 0x20, base);
        start_log(&mut e);
        e.call(0x0057_7390, &args![this, 1u32]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0041_3f90).is_empty());

        // a primitive that 0056c7f0 accepts: its class ID is read, the
        // last float set, and the primitive's virtual at +8 gets it
        let seen = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let seen_in_double = seen.clone();
        e.register_double(0x00ff_0070, move |e, a| {
            seen_in_double.borrow_mut().push(e.mem.f32(a[1] + 12));
            Ret::default()
        });
        let primitive = object_with(&mut e, 0x40, &[(8, 0x00ff_0070)]);
        returns(&mut e, 0x0041_fbe0, primitive);
        returns(&mut e, 0x0056_c7f0, 1);
        stub(&mut e, &[0x0041_3f90, 0x0045_0f90]);
        let model = node(&mut e, &[]);
        let get_model = 0x00ff_0071;
        returns(&mut e, get_model, model);
        let this = object_with(&mut e, 0x100, &[(0x1d0, get_model)]);
        e.mem.set_u32(this + 0x20, base);
        start_log(&mut e);
        e.call(0x0057_7390, &args![this, 1u32]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0045_0f90), vec![vec![model, 0]]);
        e.call(0x0057_7390, &args![this, 0u32]);
        assert_eq!(*seen.borrow(), vec![0.25, 0.0]);
    }

    #[test]
    fn fn_00577390_leaves_the_special_base_alone() {
        let mut e = engine();
        let primitive = object_with(&mut e, 0x40, &[(8, RETURNS_TRUE)]);
        returns(&mut e, 0x0041_fbe0, primitive);
        returns(&mut e, 0x0056_c7f0, 1);
        stub(&mut e, &[0x0041_3f90, 0x0045_0f90]);
        let special = form(&mut e, 1, 0);
        e.set_global(GLOBAL_BASE_FORM_A, special);
        let get_model = 0x00ff_0071;
        returns(&mut e, get_model, 0x100);
        let this = object_with(&mut e, 0x100, &[(0x1d0, get_model)]);
        e.mem.set_u32(this + 0x20, special);
        start_log(&mut e);
        e.call(0x0057_7390, &args![this, 0u32]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0045_0f90).is_empty());
        // 0056c7f0 refusing stops before the class ID is read
        returns(&mut e, 0x0056_c7f0, 0);
        start_log(&mut e);
        e.call(0x0057_7390, &args![this, 0u32]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0041_3f90).is_empty());
    }

    // ---- hkUFloat8 helpers ----------------------------------------------

    #[test]
    fn fn_00577bf0_reads_the_table_entry_of_the_byte() {
        let mut e = engine();
        e.mem.set_f32(UFLOAT8_TABLE + 4 * 7, 12.5);
        let byte = e.mem.alloc(8);
        e.mem.set_u8(byte, 7);
        assert_eq!(e.call(0x0057_7bf0, &args![byte]).f32(), 12.5);
    }

    #[test]
    fn fn_00577bd0_hands_the_address_of_the_float_to_the_setter() {
        let mut e = engine();
        // 00ca9360 stores the byte 9 when the float is 2.0
        e.register(0x00ca_9360, |e, a| {
            let value = e.mem.f32(a[1]);
            e.mem.set_u8(a[0], if value == 2.0 { 9 } else { 0 });
            Ret::default()
        });
        let byte = e.mem.alloc(8);
        assert_eq!(e.call(0x0057_7bd0, &args![byte, 2.0f32]).u32(), byte);
        assert_eq!(e.mem.u8(byte), 9);
    }

    #[test]
    fn fn_00577c10_reads_the_table_entry_of_the_infos_byte() {
        let mut e = engine();
        e.mem.set_f32(UFLOAT8_TABLE + 4 * 3, 4.5);
        returns(&mut e, 0x009d_9f40, 0x40);
        // the info object 00460140 gives
        e.register(0x0046_0140, |_, a| {
            assert_eq!(a[0], 0x40);
            0x6000_0000u32.into_ret()
        });
        e.map(0x6000_0000, 0x1000);
        e.mem.set_u8(0x6000_0000 + 0xac, 3);
        assert_eq!(e.call(0x0057_7c10, &args![0x11u32]).f32(), 4.5);
    }

    #[test]
    fn fn_00577c70_and_00577c40_store_through_the_info_object() {
        let mut e = engine();
        returns(&mut e, 0x009d_9f40, 0x40);
        returns(&mut e, 0x0046_0140, 0x6000_0000);
        e.map(0x6000_0000, 0x1000);
        let stored = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let stored_in_double = stored.clone();
        e.register_double(0x00ca_9360, move |e, a| {
            stored_in_double.borrow_mut().push((a[0], e.mem.f32(a[1])));
            Ret::default()
        });
        e.call(0x0057_7c70, &args![0x11u32, 6.5f32]);
        assert_eq!(*stored.borrow(), vec![(0x6000_00ac, 6.5)]);
        // 00577c40: through the object 004ae750 gives, if any
        returns(&mut e, 0x004a_e750, 0);
        e.call(0x0057_7c40, &args![0x11u32, 1.5f32]);
        assert_eq!(stored.borrow().len(), 1);
        returns(&mut e, 0x004a_e750, 0x22);
        e.call(0x0057_7c40, &args![0x11u32, 1.5f32]);
        assert_eq!(
            *stored.borrow(),
            vec![(0x6000_00ac, 6.5), (0x6000_00ac, 1.5)]
        );
    }

    #[test]
    fn gethk_max_linear_velocity_has_two_sources() {
        let mut e = engine();
        e.mem.set_f32(UFLOAT8_TABLE + 4 * 5, 8.0);
        e.mem.set_f32(UFLOAT8_TABLE + 4 * 6, 9.0);
        e.map(0x6000_0000, 0x1000);
        e.mem.set_u8(0x6000_0000 + 0xac, 5);
        // no object: 0.0
        returns(&mut e, 0x004a_e750, 0);
        assert_eq!(e.call(0x0057_7b50, &args![0x11u32]).f32(), 0.0);
        // an object with a body-info object: its byte is read in place
        returns(&mut e, 0x004a_e750, 0x33);
        returns(&mut e, 0x00ca_86c0, 0x44);
        returns(&mut e, 0x0046_0140, 0x6000_0000);
        assert_eq!(e.call(0x0057_7b50, &args![0x11u32]).f32(), 8.0);
        // without: a byte is built from 00577c10's value by 00ca9360
        returns(&mut e, 0x00ca_86c0, 0);
        returns(&mut e, 0x009d_9f40, 0x40);
        e.register(0x00ca_9360, |e, a| {
            // the float 8.0 becomes the byte 6
            let byte = if e.mem.f32(a[1]) == 8.0 { 6 } else { 0 };
            e.mem.set_u8(a[0], byte);
            Ret::default()
        });
        assert_eq!(e.call(0x0057_7b50, &args![0x11u32]).f32(), 9.0);
    }

    // ---- 00577ca0, 00577d50, 00577de0 -----------------------------------

    #[test]
    fn debug_display_callback_attaches_a_new_object() {
        let mut e = engine();
        returns(&mut e, 0x00aa_13e0, 0x8000);
        returns(&mut e, 0x00b6_fc90, 0x8100);
        stub(&mut e, &[0x0044_1130, 0x0043_9410, 0x00b5_7e30]);
        start_log(&mut e);
        e.call(0x0057_7ca0, &args![0u32]);
        assert_eq!(take_log(&mut e).len(), 1);
        start_log(&mut e);
        e.call(0x0057_7ca0, &args![0x55u32]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x00aa_13e0), vec![vec![0x80]]);
        assert_eq!(calls_to(&log, 0x00b6_fc90), vec![vec![0x8000]]);
        assert_eq!(calls_to(&log, 0x0044_1130), vec![vec![0x8100, 0x30, 1]]);
        assert_eq!(calls_to(&log, 0x0043_9410), vec![vec![0x55, 0x8100]]);
        assert_eq!(calls_to(&log, 0x00b5_7e30), vec![vec![0x55, 0, 0]]);
    }

    #[test]
    fn fn_00577d50_builds_the_list_for_a_container() {
        let mut e = engine();
        let this = e.mem.alloc(0x80);
        returns(&mut e, 0x0055_d310, 0);
        assert_eq!(e.call(0x0057_7d50, &args![this]).u32(), 0);
        returns(&mut e, 0x0055_d310, 0x9000);
        returns(&mut e, 0x004b_f220, 0x9100);
        returns(&mut e, 0x0048_39c0, 0x9200);
        returns(&mut e, 0x004c_8f30, 0x9300);
        returns(&mut e, 0x0042_cde0, 1);
        stub(&mut e, &[0x0041_aeb0]);
        start_log(&mut e);
        assert_eq!(e.call(0x0057_7d50, &args![this]).u32(), 0x9300);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0048_39c0), vec![vec![0x0f]]);
        assert_eq!(calls_to(&log, 0x004c_8f30), vec![vec![0x9100, 0x9200]]);
        assert_eq!(calls_to(&log, 0x0041_aeb0), vec![vec![this + 0x44]]);
        // no block: the result is 0, the extra call still depends on 0042cde0
        returns(&mut e, 0x0048_39c0, 0);
        returns(&mut e, 0x0042_cde0, 0);
        start_log(&mut e);
        assert_eq!(e.call(0x0057_7d50, &args![this]).u32(), 0);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x004c_8f30).is_empty());
        assert!(calls_to(&log, 0x0041_aeb0).is_empty());
    }

    #[test]
    fn fn_00577de0_asks_the_owner_object() {
        let mut e = engine();
        let this = e.mem.alloc(0x80);
        returns(&mut e, 0x0055_d310, 0);
        assert!(!e.call(0x0057_7de0, &args![this]).bool());
        returns(&mut e, 0x0055_d310, 0x9000);
        returns(&mut e, 0x004b_f220, 0x9100);
        returns(&mut e, 0x004d_0360, 1);
        assert!(e.call(0x0057_7de0, &args![this]).bool());
        returns(&mut e, 0x004d_0360, 0);
        assert!(!e.call(0x0057_7de0, &args![this]).bool());
    }

    // ---- 00577450 -------------------------------------------------------

    struct Scene {
        node: u32,
        owner: u32,
        refr: u32,
        model: u32,
        cell: u32,
    }

    /// The `double` an x87 helper receives in two words.
    fn double_of(a: &[u32]) -> f64 {
        f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32)
    }

    /// A node, its owner (position at +0x8c), the reference (position at
    /// +0x90; IsActor false, in a cell) and its 3D, with the callee doubles
    /// of `TransChangeCallback` that do nothing.
    fn scene(e: &mut Engine) -> Scene {
        e.register(CRT_FINITE, |_, a| double_of(a).is_finite().into_ret());
        e.register(CRT_ISNAN, |_, a| double_of(a).is_nan().into_ret());
        // 0044ddc0: +8; 00461130: +0x68; 0045bb80: +0x8c; 006815c0: this
        e.register(0x0044_ddc0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(0x0046_1130, |_, a| (a[0] + 0x68).into_ret());
        e.register(0x0045_bb80, |_, a| (a[0] + 0x8c).into_ret());
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        stub(
            e,
            &[
                0x0062_c3c0,
                0x0044_0460,
                0x0043_fa80,
                0x00c6_bd00,
                0x0043_d410,
                0x00a5_9c60,
                0x0057_3ed0,
                0x005b_5e40,
                0x0057_5830,
                SWAP_FLAG,
                0x0048_4a70,
                0x0057_29e0,
                0x00b5_d9f0,
                0x0054_cfd0,
                0x0057_3800,
                0x0054_96b0,
                0x0054_abd0,
                0x008a_e640,
                0x00a5_92c0,
                0x0057_5700,
                0x00c6_9ee0,
                0x009e_e040,
                0x0046_17e0,
                0x006f_a820,
                0x008c_7aa0,
                0x0054_97a0,
            ],
        );
        returns(e, 0x0045_0b80, 0x00a0_0000);
        // exterior without a land by default, inside an interior of TES
        returns(e, 0x0042_5fd0, 0);
        returns(e, 0x0054_6fb0, 0);
        returns(e, 0x005f_36f0, 1);
        e.mem.set_f64(GROUND_MARGIN, 128.0);
        e.mem.set_f64(VELOCITY_SCALE, 0.25);
        e.mem.set_f64(VELOCITY_LIMIT, 5.0);

        // the reference: position at +0x90, virtual 0x170 copies it out,
        // 0x1f4 gives its address
        e.register(0x00ff_0080, |e, a| {
            for i in 0..3 {
                let v = e.mem.u32(a[0] + 0x90 + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(0x00ff_0081, |_, a| (a[0] + 0x90).into_ret());
        e.register(0x00ff_0082, |e, a| e.mem.u32(a[0] + 0x98).into_ret());
        let model = node(e, &[]);
        let get_model = 0x00ff_0083;
        returns(e, get_model, model);
        let refr = object_with(
            e,
            0x100,
            &[
                (0x100, RETURNS_FALSE),
                (0x1d0, get_model),
                (0x170, 0x00ff_0080),
                (0x1f4, 0x00ff_0081),
                (0x48, 0x00ff_0090),
                (0x130, 0x00ff_0091),
                (0x174, 0x00ff_0092),
            ],
        );
        stub(e, &[0x00ff_0090, 0x00ff_0091, 0x00ff_0092]);
        let base = form(e, 0x30, 0);
        e.mem.set_u32(refr + 0x20, base);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        let owner = e.mem.alloc(0x100);
        let node_object = object_with(e, 0x40, &[(0xb0, 0x00ff_0093)]);
        stub(e, &[0x00ff_0093]);
        e.mem.set_u32(node_object + 8, owner);
        returns(e, 0x0056_f930, refr);
        // positions: node at z = 50, reference at z = 60
        e.mem.set_f32(owner + 0x8c + 8, 50.0);
        e.mem.set_f32(refr + 0x90 + 8, 60.0);
        Scene {
            node: node_object,
            owner,
            refr,
            model,
            cell,
        }
    }

    #[test]
    fn trans_change_callback_ignores_a_node_without_reference() {
        let mut e = engine();
        let s = scene(&mut e);
        returns(&mut e, 0x0056_f930, 0);
        start_log(&mut e);
        e.call(0x0057_7450, &args![s.node, 3u32]);
        let log = take_log(&mut e);
        assert_eq!(log.len(), 3);
        assert_eq!(calls_to(&log, 0x0044_ddc0), vec![vec![s.node]]);
        assert_eq!(calls_to(&log, 0x0056_f930), vec![vec![s.owner]]);
    }

    #[test]
    fn trans_change_callback_with_no_flags_only_refreshes_the_lighting() {
        let mut e = engine();
        let s = scene(&mut e);
        start_log(&mut e);
        e.call(0x0057_7450, &args![s.node, 0u32]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0062_c3c0), vec![vec![s.refr, s.model]]);
        assert_eq!(calls_to(&log, 0x0045_0b80), vec![vec![0]]);
        assert_eq!(
            calls_to(&log, 0x00b5_d9f0),
            vec![vec![0x00a0_0000, s.model, 1]]
        );
        assert!(calls_to(&log, SET_POSITION).is_empty());
        assert!(calls_to(&log, SET_ROTATION).is_empty());
    }

    #[test]
    fn trans_change_callback_writes_the_position_back_around_a_flag_swap() {
        let mut e = engine();
        let s = scene(&mut e);
        // the previous flag value is handed back to the swap
        returns(&mut e, SWAP_FLAG, 1);
        // land at height 100: the limit is -28, the node (z = 50) is above
        returns(&mut e, 0x0054_6fb0, 0x5000);
        let height = e.mem.alloc(8);
        e.mem.set_f32(height, 100.0);
        returns(&mut e, 0x0053_f440, height);
        start_log(&mut e);
        e.call(0x0057_7450, &args![s.node, 1u32]);
        let log = take_log(&mut e);
        let flag_object = e.global::<u32>(GLOBAL_FLAG_OBJECT);
        assert_eq!(
            calls_to(&log, SWAP_FLAG),
            vec![vec![flag_object, 0], vec![flag_object, 1]]
        );
        assert_eq!(
            calls_to(&log, SET_POSITION),
            vec![vec![s.refr, s.owner + 0x8c]]
        );
        assert_eq!(calls_to(&log, 0x00ff_0090), vec![vec![s.refr, 4]]);
        // nothing fell out of the world
        assert!(calls_to(&log, 0x0044_0460).is_empty());
        assert!(calls_to(&log, 0x0054_cfd0).is_empty());
        // the interior test of TES says interior: no cell lookup
        assert!(calls_to(&log, 0x0046_1bc0).is_empty());
    }

    #[test]
    fn trans_change_callback_disables_the_collision_of_a_slow_fallen_node() {
        let mut e = engine();
        let s = scene(&mut e);
        // a position that is not finite
        e.mem.set_f32(s.owner + 0x8c, f32::NAN);
        returns(&mut e, 0x006f_a820, 0x7000);
        returns(&mut e, 0x004a_e750, 0);
        returns(&mut e, 0x0084_e3a0, 0x0012_34ab);
        returns(&mut e, 0x00ff_0091, 0x6666);
        start_log(&mut e);
        e.call(0x0057_7450, &args![s.node, 1u32]);
        let log = take_log(&mut e);
        // not finite: no placement, but the 3D is moved to the position
        assert!(calls_to(&log, 0x0054_cfd0).is_empty());
        assert_eq!(calls_to(&log, 0x0044_0460)[0][0], s.model);
        assert_eq!(
            calls_to(&log, 0x0043_fa80),
            vec![vec![s.model, 0x011a_9448]]
        );
        assert_eq!(calls_to(&log, 0x00c6_bd00), vec![vec![s.model, 1]]);
        assert_eq!(calls_to(&log, 0x00a5_9c60).len(), 1);
        // the body is too slow: the node's virtual +0xb0 disables it
        assert_eq!(calls_to(&log, 0x00ff_0093), vec![vec![s.node, 4, 0, 0]]);
        assert_eq!(calls_to(&log, 0x0057_3ed0), vec![vec![s.node, 0]]);
        assert_eq!(
            calls_to(&log, MESSAGE),
            vec![vec![DISABLE_COLLISION_MESSAGE, 0x6666, 0x0012_34ab]]
        );
    }

    #[test]
    fn trans_change_callback_limits_a_fast_fallen_body() {
        let mut e = engine();
        let s = scene(&mut e);
        e.mem.set_f32(s.owner + 0x8c, f32::INFINITY);
        returns(&mut e, 0x006f_a820, 0x7000);
        // the body's object has an info object whose byte 7 maps to 40.0
        returns(&mut e, 0x004a_e750, 0x7100);
        returns(&mut e, 0x00ca_86c0, 0x7200);
        returns(&mut e, 0x0046_0140, 0x6000_0000);
        returns(&mut e, 0x009d_9f40, 0x7300);
        e.map(0x6000_0000, 0x1000);
        e.mem.set_u8(0x6000_0000 + 0xac, 7);
        e.mem.set_f32(UFLOAT8_TABLE + 28, 40.0);
        let stored = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let stored_in_double = stored.clone();
        e.register_double(0x00ca_9360, move |e, a| {
            stored_in_double.borrow_mut().push((a[0], e.mem.f32(a[1])));
            Ret::default()
        });
        start_log(&mut e);
        e.call(0x0057_7450, &args![s.node, 1u32]);
        let log = take_log(&mut e);
        // 40.0 * 0.25 = 10.0 is above 5.0: the limit is stored, the
        // collision is not disabled
        assert_eq!(*stored.borrow(), vec![(0x6000_00ac, 10.0)]);
        assert!(calls_to(&log, 0x0057_3ed0).is_empty());
        assert!(calls_to(&log, MESSAGE).is_empty());
    }

    #[test]
    fn trans_change_callback_replaces_a_reference_below_the_land() {
        let mut e = engine();
        let s = scene(&mut e);
        // land height 100: limit -28; node and reference both lower
        e.mem.set_f32(s.owner + 0x8c + 8, -50.0);
        e.mem.set_f32(s.refr + 0x90 + 8, -40.0);
        returns(&mut e, 0x0054_6fb0, 0x5000);
        let height = e.mem.alloc(8);
        e.mem.set_f32(height, 100.0);
        returns(&mut e, 0x0053_f440, height);
        e.mem.set_u32(ZERO_VECTOR, 0x1111);
        e.mem.set_u32(ZERO_VECTOR + 4, 0x2222);
        e.mem.set_u32(ZERO_VECTOR + 8, 0x3333);
        start_log(&mut e);
        e.call(0x0057_7450, &args![s.node, 1u32]);
        let log = take_log(&mut e);
        // the cell's placement info gets a copy of the vector, then the
        // reference is moved to what it wrote (here: unchanged)
        let place = calls_to(&log, 0x0054_cfd0);
        assert_eq!(place.len(), 1);
        assert_eq!(place[0][0], s.cell);
        assert_eq!(
            calls_to(&log, 0x00ff_0092),
            vec![vec![s.refr, 0x1111, 0x2222, 0x3333]]
        );
        // a reference that is only the node below the land is not replaced
        e.mem.set_f32(s.refr + 0x90 + 8, 60.0);
        start_log(&mut e);
        e.call(0x0057_7450, &args![s.node, 1u32]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0054_cfd0).is_empty());
        assert_eq!(calls_to(&log, 0x0044_0460).len(), 1);
    }

    #[test]
    fn trans_change_callback_compares_with_the_bounds_of_an_interior_cell() {
        let mut e = engine();
        let s = scene(&mut e);
        returns(&mut e, 0x0042_5fd0, 1);
        returns(&mut e, 0x0045_6fc0, 0x5000);
        // the bounding sphere: centre z = 100
        let bounds = e.mem.alloc(16);
        e.mem.set_f32(bounds + 8, 100.0);
        returns(&mut e, 0x0043_d450, bounds);
        returns(&mut e, 0x0043_9090, 1);
        returns_float(&mut e, 0x0084_d030, 5.0);
        start_log(&mut e);
        e.call(0x0057_7450, &args![s.node, 1u32]);
        let log = take_log(&mut e);
        // node z = 50 is 50 below the centre (> 5), reference z = 60 is 40
        // below: both count as fallen
        assert_eq!(calls_to(&log, 0x0054_cfd0).len(), 1);
        // a large radius keeps the node inside
        returns_float(&mut e, 0x0084_d030, 80.0);
        start_log(&mut e);
        e.call(0x0057_7450, &args![s.node, 1u32]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0054_cfd0).is_empty());
        // a radius between the two drops: only the node is below
        returns_float(&mut e, 0x0084_d030, 45.0);
        start_log(&mut e);
        e.call(0x0057_7450, &args![s.node, 1u32]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0054_cfd0).is_empty());
        assert_eq!(calls_to(&log, 0x0044_0460).len(), 1);
        // the test of the bounds (0043_9090) refusing: nothing
        returns(&mut e, 0x0043_9090, 0);
        start_log(&mut e);
        e.call(0x0057_7450, &args![s.node, 1u32]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0044_0460).is_empty());
    }

    #[test]
    fn trans_change_callback_submits_the_water_bound_reference_to_the_sinking() {
        let mut e = engine();
        let s = scene(&mut e);
        let water = form(&mut e, 0x1e, 0);
        e.mem.set_u32(s.refr + 0x20, water);
        returns(&mut e, 0x0044_8a20, 0);
        returns_float(&mut e, 0x0057_b0a0, 70.0);
        start_log(&mut e);
        e.call(0x0057_7450, &args![s.node, 1u32]);
        let log = take_log(&mut e);
        // the reference (z = 60) is below the water height 70
        assert_eq!(calls_to(&log, 0x0048_4a70), vec![vec![s.refr, 1]]);
        assert_eq!(calls_to(&log, 0x0057_29e0), vec![vec![s.refr, 0]]);
        // above the water, or flagged: nothing
        returns_float(&mut e, 0x0057_b0a0, 10.0);
        start_log(&mut e);
        e.call(0x0057_7450, &args![s.node, 1u32]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0048_4a70).is_empty());
        returns_float(&mut e, 0x0057_b0a0, 70.0);
        returns(&mut e, 0x0044_8a20, 1);
        start_log(&mut e);
        e.call(0x0057_7450, &args![s.node, 1u32]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0048_4a70).is_empty());
    }

    #[test]
    fn trans_change_callback_writes_the_euler_angles_back() {
        let mut e = engine();
        let s = scene(&mut e);
        e.register(0x00a5_92c0, |e, a| {
            e.mem.set_f32(a[1], 0.5);
            e.mem.set_f32(a[2], 1.5);
            e.mem.set_f32(a[3], 2.5);
            Ret::default()
        });
        start_log(&mut e);
        e.call(0x0057_7450, &args![s.node, 2u32]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, SET_ROTATION),
            vec![vec![
                s.refr,
                0.5f32.to_bits(),
                1.5f32.to_bits(),
                2.5f32.to_bits()
            ]]
        );
        // the matrix is the owner's transform
        assert_eq!(calls_to(&log, 0x00a5_92c0)[0][0], s.owner + 0x68);
        assert!(calls_to(&log, SET_POSITION).is_empty());
    }

    #[test]
    fn trans_change_callback_moves_the_reference_to_the_cell_of_its_coordinates() {
        let mut e = engine();
        let s = scene(&mut e);
        // exterior: TES has no interior cell
        returns(&mut e, 0x005f_36f0, 0);
        returns(&mut e, 0x004f_d3e0, 0x77);
        e.mem.set_f32(s.refr + 0x90, 11.0);
        e.mem.set_f32(s.refr + 0x94, 22.0);
        // the same cell: nothing more happens
        let cell = s.cell;
        returns(&mut e, 0x0046_1bc0, cell);
        start_log(&mut e);
        e.call(0x0057_7450, &args![s.node, 1u32]);
        let log = take_log(&mut e);
        let handler = e.global::<u32>(GLOBAL_DATA_HANDLER);
        assert_eq!(
            calls_to(&log, 0x0046_1bc0),
            vec![vec![handler, 11.0f32.to_bits(), 22.0f32.to_bits(), 0x77, 0]]
        );
        assert!(calls_to(&log, 0x0057_3800).is_empty());
        // no cell: the saved position is restored and the node frozen
        returns(&mut e, 0x0046_1bc0, 0);
        start_log(&mut e);
        e.call(0x0057_7450, &args![s.node, 1u32]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, SET_POSITION).len(), 2);
        assert_eq!(calls_to(&log, 0x00ff_0093), vec![vec![s.node, 4, 0, 0]]);
        assert!(calls_to(&log, 0x0057_3800).is_empty());
        // another cell: the reference moves and its 3D is attached
        returns(&mut e, 0x0046_1bc0, 0x0abc_0000);
        returns(&mut e, 0x0054_97a0, 0x0def_0000);
        start_log(&mut e);
        e.call(0x0057_7450, &args![s.node, 1u32]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0057_3800), vec![vec![s.refr, 0, 0x77]]);
        assert_eq!(
            calls_to(&log, 0x0054_96b0),
            vec![vec![0x0abc_0000, s.refr, 0x0def_0000]]
        );
        // the other attach path
        returns(&mut e, 0x008c_7aa0, 1);
        start_log(&mut e);
        e.call(0x0057_7450, &args![s.node, 1u32]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0054_abd0), vec![vec![s.refr, 0x0def_0000]]);
        // an actor is not attached, and its AI is updated
        let vtable = e.mem.u32(s.refr);
        e.mem.set_u32(vtable + 0x100, RETURNS_TRUE);
        start_log(&mut e);
        e.call(0x0057_7450, &args![s.node, 1u32]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x008a_e640), vec![vec![s.refr, 0]]);
        assert!(calls_to(&log, 0x0054_abd0).is_empty());
    }

    // ---- addon-node walkers ---------------------------------------------

    /// A node with an addon record: the record is what `004617e0` returns
    /// for the key `009ee040(node)` gives (the node's own address).
    fn addon_world(e: &mut Engine, records: &[(u32, u32)]) {
        e.register(0x009e_e040, |_, a| a[0].into_ret());
        let table: Vec<(u32, u32)> = records.to_vec();
        e.register_double(0x0046_17e0, move |_, a| Ret {
            eax: table
                .iter()
                .find(|(key, _)| *key == a[1])
                .map_or(0, |(_, record)| *record),
            ..Ret::default()
        });
    }

    #[test]
    fn has_addon_nodes_searches_the_tree() {
        let mut e = engine();
        let leaf = node(&mut e, &[]);
        let root = node(&mut e, &[leaf]);
        returns(&mut e, 0x0045_3470, 0);
        assert!(!e.call(0x0057_8400, &args![root]).bool());
        assert!(!e.call(0x0057_8400, &args![0u32]).bool());
        // the leaf accepted
        returns(&mut e, 0x0045_3470, 0);
        assert!(!e.call(0x0057_8400, &args![root]).bool());
        e.register_double(0x0045_3470, move |_, a| Ret {
            eax: u32::from(a[0] == leaf),
            ..Ret::default()
        });
        assert!(e.call(0x0057_8400, &args![root]).bool());
        // the root itself
        e.register_double(0x0045_3470, move |_, a| Ret {
            eax: u32::from(a[0] == root),
            ..Ret::default()
        });
        start_log(&mut e);
        assert!(e.call(0x0057_8400, &args![root]).bool());
        let log = take_log(&mut e);
        // the search stops at the root
        assert!(calls_to(&log, CHILD_COUNT).is_empty());
    }

    #[test]
    fn remove_addon_nodes_tells_the_record_and_recurses() {
        let mut e = engine();
        let leaf = node(&mut e, &[]);
        let root = node(&mut e, &[leaf]);
        let tell = 0x00ff_00a0;
        returns(&mut e, tell, 0);
        let record = object_with(&mut e, 0x60, &[(0x150, tell)]);
        addon_world(&mut e, &[(root, record)]);
        stub(&mut e, &[0x00c6_9ee0, 0x0057_2160]);
        returns(&mut e, 0x0048_cee0, 1);
        returns(&mut e, 0x0045_3470, 1);
        start_log(&mut e);
        assert!(e.call(0x0057_8300, &args![root]).bool());
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, 0x00c6_9ee0),
            vec![vec![root, 1, 1], vec![leaf, 1, 1]]
        );
        assert_eq!(calls_to(&log, tell), vec![vec![record, 0]]);
        assert_eq!(calls_to(&log, 0x0057_2160), vec![vec![root]]);
        // the leaf is visited (00c69ee0) even though it has no record
        assert_eq!(calls_to(&log, 0x0046_17e0).len(), 2);
        // a record without model: nothing is told
        returns(&mut e, 0x0048_cee0, 0);
        start_log(&mut e);
        assert!(!e.call(0x0057_8300, &args![root]).bool());
        let log = take_log(&mut e);
        assert!(calls_to(&log, tell).is_empty());
        assert!(!e.call(0x0057_8300, &args![0u32]).bool());
    }

    #[test]
    fn master_particle_addon_nodes_are_switched_on_and_off() {
        let mut e = engine();
        let leaf = node(&mut e, &[]);
        let root = node(&mut e, &[leaf]);
        let record = e.mem.alloc(0x80);
        addon_world(&mut e, &[(leaf, record)]);
        returns(&mut e, 0x0044_8bf0, 1);
        stub(&mut e, &[0x0045_0f90, 0x00c5_05c0]);
        returns(&mut e, 0x0059_e300, 0x6000);
        returns(&mut e, 0x0045_a190, 0x6100);
        returns(&mut e, 0x0045_6610, 0);
        start_log(&mut e);
        assert!(e.call(0x0057_8060, &args![root]).bool());
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0045_0f90), vec![vec![leaf, 0]]);
        assert_eq!(
            calls_to(&log, 0x00c5_05c0),
            vec![vec![0x6100, 0x6000, leaf]]
        );
        // 00456610 stops the descent below the root
        returns(&mut e, 0x0045_6610, 1);
        start_log(&mut e);
        assert!(!e.call(0x0057_8060, &args![root]).bool());
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0045_0f90).is_empty());
        assert!(!e.call(0x0057_8060, &args![0u32]).bool());

        // removing
        start_log(&mut e);
        assert!(e.call(0x0057_8170, &args![root]).bool());
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0045_0f90), vec![vec![leaf, 1]]);
        returns(&mut e, 0x0044_8bf0, 0);
        assert!(!e.call(0x0057_8170, &args![root]).bool());
        assert!(!e.call(0x0057_8170, &args![0u32]).bool());
    }

    #[test]
    fn fn_00578250_visits_the_nodes_it_accepts() {
        let mut e = engine();
        let leaf = node(&mut e, &[]);
        let root = node(&mut e, &[leaf]);
        e.register_double(0x0057_0ea0, move |_, a| Ret {
            eax: u32::from(a[0] == root),
            ..Ret::default()
        });
        returns(&mut e, 0x0045_3a70, 0x6000);
        stub(&mut e, &[0x00ad_8570]);
        start_log(&mut e);
        assert!(e.call(0x0057_8250, &args![root]).bool());
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x00ad_8570), vec![vec![0x6000, root, 0, 0]]);
        // not accepted at the root, but below it
        e.register_double(0x0057_0ea0, move |_, a| Ret {
            eax: u32::from(a[0] == leaf),
            ..Ret::default()
        });
        start_log(&mut e);
        assert!(!e.call(0x0057_8250, &args![root]).bool());
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x00ad_8570).len(), 1);
        assert!(!e.call(0x0057_8250, &args![0u32]).bool());
    }

    #[test]
    fn add_addon_nodes_detaches_base_children_and_attaches_the_record() {
        let mut e = engine();
        // a child named "BASE..." (strncmp of the name with "BASE", 4)
        let detach = 0x00ff_00b0;
        returns(&mut e, detach, 0);
        let child = object_with(&mut e, 0x80, &[(0x0c, AS_NODE_SELF), (0xf0, detach)]);
        let attach = 0x00ff_00b1;
        returns(&mut e, attach, 0);
        let root = object_with(&mut e, 0x80, &[(0x0c, AS_NODE_SELF), (0xdc, attach)]);
        e.mem.set_u32(root + 0x20, 1);
        e.mem.set_u32(root + 0x24, child);
        // child names via 00413f40 / 0043b1b0
        e.register(0x0041_3f40, |_, a| a[0].into_ret());
        returns(&mut e, 0x0048_af70, 1);
        e.register(0x0043_b1b0, |_, a| a[0].into_ret());
        e.register(STRNCMP, |e, a| {
            // equal when the child (name holder) is flagged at +0x10
            assert_eq!(a[1], BASE_PREFIX);
            assert_eq!(a[2], 4);
            i32::from(e.mem.u32(a[0] + 0x10) != 0).into_ret()
        });
        // no addon record for the root: only the detaching and the
        // recursion happen
        addon_world(&mut e, &[]);
        start_log(&mut e);
        assert!(!e.call(0x0057_7e20, &args![root]).bool());
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, detach), vec![vec![child, 0]]);
        assert!(calls_to(&log, 0x0049_02f0).is_empty());
        // a child whose name does not start with BASE stays
        e.mem.set_u32(child + 0x10, 1);
        start_log(&mut e);
        e.call(0x0057_7e20, &args![root]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, detach).is_empty());
        assert!(!e.call(0x0057_7e20, &args![0u32]).bool());
    }

    #[test]
    fn add_addon_nodes_attaches_a_randomized_record() {
        let mut e = engine();
        let attach = 0x00ff_00b1;
        returns(&mut e, attach, 0);
        let root = object_with(&mut e, 0x80, &[(0x0c, AS_NODE_SELF), (0xdc, attach)]);
        // the record: virtual 0x178 creates the node to attach (whose
        // virtual 0x10 gives a second object)
        let created = node(&mut e, &[]);
        let make = 0x00ff_00b2;
        returns(&mut e, make, created);
        let second_getter = 0x00ff_00b3;
        returns(&mut e, second_getter, 0x6000);
        let vtable = e.mem.u32(created);
        e.mem.set_u32(vtable + 0x10, second_getter);
        let record = object_with(&mut e, 0x80, &[(0x178, make)]);
        addon_world(&mut e, &[(root, record)]);
        returns(&mut e, 0x0048_cee0, 1);
        returns(&mut e, 0x0044_8bf0, 0);
        stub(&mut e, &[0x0056_c7d0, 0x0049_02f0, 0x0057_6660]);
        // tick 1234: seed = (1234 % 1000) / 100.0 = 2.34
        returns(&mut e, 0x0048_7f50, 1234);
        e.mem.set_f64(RANDOM_DIVISOR, 100.0);
        // the randomizer finds no entries
        returns(&mut e, 0x0043_b230, 0);
        returns(&mut e, 0x00a5_9d30, 0);
        start_log(&mut e);
        assert!(e.call(0x0057_7e20, &args![root]).bool());
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0056_c7d0), vec![vec![0x6000, 0]]);
        assert_eq!(calls_to(&log, attach), vec![vec![root, created, 1]]);
        assert_eq!(calls_to(&log, 0x0049_02f0), vec![vec![root, 1]]);
        assert_eq!(calls_to(&log, 0x0057_6660), vec![vec![root, 0]]);
        // the same seed reaches the randomizer's entries
        let entry = e.mem.alloc(16);
        e.register_double(0x0043_b230, move |_, a| Ret {
            eax: if a[0] == created { entry } else { 0 },
            ..Ret::default()
        });
        returns(&mut e, 0x004a_8a90, 0);
        stub(&mut e, &[0x0062_1b20]);
        start_log(&mut e);
        e.call(0x0057_7e20, &args![root]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, 0x0062_1b20),
            vec![vec![entry, (2.34f64 as f32).to_bits()]]
        );
        // a record that is excluded by 00448bf0 attaches nothing
        returns(&mut e, 0x0044_8bf0, 1);
        start_log(&mut e);
        assert!(!e.call(0x0057_7e20, &args![root]).bool());
        let log = take_log(&mut e);
        assert!(calls_to(&log, attach).is_empty());
    }

    #[test]
    fn randomize_addons_walks_both_lists_and_the_children() {
        let mut e = engine();
        let leaf = node(&mut e, &[]);
        let root = node(&mut e, &[leaf]);
        let first = e.mem.alloc(16);
        let second_list = e.mem.alloc(16);
        let second = e.mem.alloc(16);
        e.register_double(0x0043_b230, move |_, a| Ret {
            eax: if a[0] == root {
                first
            } else if a[0] == second_list {
                second
            } else {
                0
            },
            ..Ret::default()
        });
        returns(&mut e, 0x004a_8a90, 0);
        e.register_double(0x00a5_9d30, move |_, a| Ret {
            eax: if a[0] == root { second_list } else { 0 },
            ..Ret::default()
        });
        stub(&mut e, &[0x0062_1b20]);
        start_log(&mut e);
        e.call(0x0057_84f0, &args![root, 1.5f32]);
        let log = take_log(&mut e);
        let bits = 1.5f32.to_bits();
        assert_eq!(
            calls_to(&log, 0x0062_1b20),
            vec![vec![first, bits], vec![second, bits]]
        );
        // the children are visited with the same value
        let visited = calls_to(&log, 0x0043_b230);
        assert_eq!(visited, vec![vec![root], vec![second_list], vec![leaf]]);
        assert_eq!(calls_to(&log, 0x00a5_9d30)[0], vec![root, 5]);
        e.call(0x0057_84f0, &args![0u32, 1.5f32]);
    }

    #[test]
    fn has_addon_flags_asks_the_extra_data() {
        let mut e = engine();
        returns(&mut e, 0x0044_8a80, 0x0120_2ddc);
        returns(&mut e, 0x00a5_bdd0, 0);
        returns(&mut e, 0x0044_8a40, 1);
        start_log(&mut e);
        assert!(!e.call(0x0057_84b0, &args![0x77u32]).bool());
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x00a5_bdd0), vec![vec![0x77, 0x0120_2ddc]]);
        assert!(calls_to(&log, 0x0044_8a40).is_empty());
        returns(&mut e, 0x00a5_bdd0, 0x88);
        assert!(e.call(0x0057_84b0, &args![0x77u32]).bool());
        returns(&mut e, 0x0044_8a40, 0);
        assert!(!e.call(0x0057_84b0, &args![0x77u32]).bool());
    }

    // ---- owners ---------------------------------------------------------

    #[test]
    fn fn_00578770_tests_flag_200() {
        let mut e = engine();
        let object = e.mem.alloc(0x40);
        assert!(!e.call(0x0057_8770, &args![object]).bool());
        e.mem.set_u32(object + 0x34, 0x200);
        assert!(e.call(0x0057_8770, &args![object]).bool());
    }

    /// A reference whose owner (`00567790`) is `owner`.
    fn owned(e: &mut Engine, owner: u32) -> u32 {
        returns(e, 0x0056_7790, owner);
        stub(e, &[0x0056_79f0, 0x0056_7960]);
        e.mem.alloc(0x80)
    }

    #[test]
    fn is_an_owner_without_owner_everybody_owns() {
        let mut e = engine();
        let this = owned(&mut e, 0);
        assert!(e.call(0x0057_85e0, &args![this, 0x55u32, false]).bool());
        assert!(e.call(0x0057_85e0, &args![this, 0u32, false]).bool());
    }

    #[test]
    fn is_an_owner_compares_the_actors_form_with_the_owner() {
        let mut e = engine();
        let owner = form(&mut e, 0x30, 0);
        let this = owned(&mut e, owner);
        // no actor: not an owner
        assert!(!e.call(0x0057_85e0, &args![this, 0u32, false]).bool());
        // the actor's own form (virtual 0x1a4) is the owner
        let get_form = 0x00ff_00c0;
        returns(&mut e, get_form, owner);
        let actor = object_with(&mut e, 0x80, &[(0x1a4, get_form)]);
        assert!(e.call(0x0057_85e0, &args![this, actor, false]).bool());
        // without a form of its own the base form is used
        returns(&mut e, get_form, 0);
        e.mem.set_u32(actor + 0x20, owner);
        assert!(e.call(0x0057_85e0, &args![this, actor, false]).bool());
        // another form: not an owner
        let other = form(&mut e, 0x30, 0);
        e.mem.set_u32(actor + 0x20, other);
        assert!(!e.call(0x0057_85e0, &args![this, actor, false]).bool());
    }

    #[test]
    fn is_an_owner_accepts_an_npc_owners_actors_through_008b8e90() {
        let mut e = engine();
        let npc = form(&mut e, 8, 0);
        let this = owned(&mut e, npc);
        let get_form = 0x00ff_00c0;
        // the actor's form is of type 0x2a: it is cast to the bound object
        let actor_form = form(&mut e, 0x2a, 0);
        returns(&mut e, get_form, actor_form);
        let actor = object_with(&mut e, 0x80, &[(0x1a4, get_form)]);
        e.register(DYNAMIC_CAST, |_, a| {
            assert_eq!(a[1..], [0, 0x0118_3108, 0x0118_46e8, 0]);
            a[0].into_ret()
        });
        returns(&mut e, 0x008b_8e90, 1);
        // the flag of the NPC (+0x34 & 0x200) or the argument turns it on
        assert!(!e.call(0x0057_85e0, &args![this, actor, false]).bool());
        e.mem.set_u32(npc + 0x34, 0x200);
        start_log(&mut e);
        assert!(e.call(0x0057_85e0, &args![this, actor, false]).bool());
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x008b_8e90), vec![vec![actor, npc]]);
        e.mem.set_u32(npc + 0x34, 0);
        assert!(e.call(0x0057_85e0, &args![this, actor, true]).bool());
        // 008b8e90 refusing
        returns(&mut e, 0x008b_8e90, 0);
        assert!(!e.call(0x0057_85e0, &args![this, actor, true]).bool());
    }

    #[test]
    fn is_an_owner_never_matches_a_faction_owner_by_value() {
        let mut e = engine();
        let faction = form(&mut e, 0x2a, 0);
        let this = owned(&mut e, faction);
        returns(&mut e, 0x0056_7960, 0x6000);
        returns_float(&mut e, 0x0052_6ac0, 1.0);
        e.mem.set_f64(OWNERSHIP_VALUE_LIMIT, 1.0);
        let get_form = 0x00ff_00c0;
        let actor_form = form(&mut e, 0x30, 0);
        returns(&mut e, get_form, actor_form);
        let actor = object_with(&mut e, 0x80, &[(0x1a4, get_form)]);
        assert!(!e.call(0x0057_85e0, &args![this, actor, false]).bool());
        assert!(!e.call(0x0057_85e0, &args![this, actor, true]).bool());
    }

    #[test]
    fn is_owner_evil_for_an_npc_owner() {
        let mut e = engine();
        let npc = form(&mut e, 8, 0);
        returns(&mut e, 0x0047_d7c0, 1);
        let this = e.mem.alloc(0x80);
        start_log(&mut e);
        assert!(e.call(0x0057_8790, &args![this, npc]).bool());
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0047_d7c0), vec![vec![npc]]);
        returns(&mut e, 0x0047_d7c0, 0);
        assert!(!e.call(0x0057_8790, &args![this, npc]).bool());
        // without an owner argument the reference's owner is used
        returns(&mut e, 0x0047_d7c0, 1);
        returns(&mut e, 0x0056_7790, npc);
        assert!(e.call(0x0057_8790, &args![this, 0u32]).bool());
        returns(&mut e, 0x0056_7790, 0);
        assert!(!e.call(0x0057_8790, &args![this, 0u32]).bool());
    }

    #[test]
    fn is_owner_evil_for_a_faction_owner_depends_on_the_actor_value() {
        let mut e = engine();
        let faction = form(&mut e, 0x2a, 0);
        let this = e.mem.alloc(0x80);
        // no actor reference: evil
        returns(&mut e, 0x0097_0a20, 0);
        assert!(e.call(0x0057_8790, &args![this, faction]).bool());
        // an actor whose value 0x17 maps to 2 or 4 stays evil; others are not
        let get_value = 0x00ff_00d0;
        returns_float(&mut e, get_value, 3.0);
        let base = object_with(&mut e, 0x200, &[]);
        let values = base + 0x100;
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(values, vtable);
        e.mem.set_u32(vtable + 0xc, get_value);
        returns(&mut e, 0x0097_0a20, 0x6000);
        returns(&mut e, 0x0041_81e0, base);
        e.register(0x0047_e040, |_, a| {
            let class = if f32::from_bits(a[0]) == 3.0 {
                2u32
            } else {
                1u32
            };
            class.into_ret()
        });
        start_log(&mut e);
        assert!(e.call(0x0057_8790, &args![this, faction]).bool());
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, get_value), vec![vec![values, 0x17]]);
        assert_eq!(
            calls_to(&log, 0x0097_0a20),
            vec![vec![0x011e_0e80, faction, 1]]
        );
        returns_float(&mut e, get_value, 5.0);
        assert!(!e.call(0x0057_8790, &args![this, faction]).bool());
        // other form types are not evil
        let other = form(&mut e, 0x30, 0);
        assert!(!e.call(0x0057_8790, &args![this, other]).bool());
    }

    // ---- 00578870 .. 00578970 -------------------------------------------

    #[test]
    fn fn_00578870_names_the_location() {
        let mut e = engine();
        let this = scene(&mut e).refr;
        stub(&mut e, &[0x0040_37f0]);
        returns(&mut e, 0x0040_8da0, 0x6000);
        returns(&mut e, 0x0042_5fd0, 1);
        let result = e.mem.alloc(16);
        // an interior cell: its name
        start_log(&mut e);
        e.call(0x0057_8870, &args![this, result]);
        let log = take_log(&mut e);
        let cell = e.mem.u32(this + 0x40);
        assert_eq!(calls_to(&log, 0x0040_8da0), vec![vec![cell + 0x18]]);
        assert_eq!(calls_to(&log, 0x0040_37f0), vec![vec![result, 0x6000, 0]]);
        // an exterior cell: the place object formats the position
        returns(&mut e, 0x0042_5fd0, 0);
        let format = 0x00ff_00e0;
        returns(&mut e, format, 0);
        let place = object_with(&mut e, 0x80, &[(0x138, format)]);
        returns(&mut e, 0x0057_5d70, place);
        e.mem.set_f32(this + 0x90, 1.0);
        start_log(&mut e);
        e.call(0x0057_8870, &args![this, result]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, format),
            vec![vec![place, result, 1.0f32.to_bits(), 0, 60.0f32.to_bits()]]
        );
        // no place: the empty string
        returns(&mut e, 0x0057_5d70, 0);
        start_log(&mut e);
        e.call(0x0057_8870, &args![this, result]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, 0x0040_37f0),
            vec![vec![result, EMPTY_STRING, 0]]
        );
    }

    #[test]
    fn fn_00578920_passes_the_3d_and_the_flag() {
        let mut e = engine();
        stub(&mut e, &[0x00c6_bd00]);
        let get_model = 0x00ff_00f0;
        returns(&mut e, get_model, 0x6000);
        let this = object_with(&mut e, 0x80, &[(0x1d0, get_model)]);
        start_log(&mut e);
        e.call(0x0057_8920, &args![this, 1u32]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x00c6_bd00), vec![vec![0x6000, 1]]);
    }

    #[test]
    fn add_talking_activator_mobile_object_refr_forwards_to_the_base_form() {
        let mut e = engine();
        let base = form(&mut e, 0x30, 0);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x20, base);
        returns(&mut e, 0x004f_f070, 0x4242);
        start_log(&mut e);
        assert_eq!(e.call(0x0057_8950, &args![this]).u32(), 0x4242);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x004f_f070), vec![vec![base, this]]);
    }

    #[test]
    fn get_mobile_object_extra_reads_the_extra_at_0xc() {
        let mut e = engine();
        let this = e.mem.alloc(0x80);
        returns(&mut e, 0x0042_e110, 0);
        assert_eq!(e.call(0x0057_8970, &args![this]).u32(), 0);
        let extra = e.mem.alloc(0x20);
        e.mem.set_u32(extra + 0xc, 0xfeed);
        returns(&mut e, 0x0042_e110, extra);
        assert_eq!(e.call(0x0057_8970, &args![this]).u32(), 0xfeed);
    }

    // ---- second batch: 005789b0 .. 0057b410 -----------------------------

    /// Target of the virtual at +0x48 (mark changed) in the doubles.
    const MARK_CHANGED: u32 = 0x00ff_0100;
    /// A virtual that returns the constant in `Ret::eax` set per test.
    const SPARE_SLOT: u32 = 0x00ff_0101;

    /// Adds the registrations the second batch's tests share.
    fn engine2() -> Engine {
        let mut e = engine();
        e.map(0x0101_5000, 0x1000);
        e.register(MARK_CHANGED, |_, _| Ret::default());
        // 0084e3a0: iFormID at +0x0c; 00559450: the first dword
        e.register(0x0084_e3a0, |e, a| e.mem.u32(a[0] + 0x0c).into_ret());
        e.register(0x0055_9450, |e, a| e.mem.u32(a[0]).into_ret());
        e
    }

    /// A reference whose base form has type `kind`, with IsActor false and
    /// the mark-changed slot, plus the given virtual slots.
    fn refr(e: &mut Engine, kind: u8, slots: &[(u32, u32)]) -> u32 {
        let mut all = vec![(0x100, RETURNS_FALSE), (0x48, MARK_CHANGED)];
        all.extend_from_slice(slots);
        let this = object_with(e, 0x600, &all);
        let base = form(e, kind, 0);
        e.mem.set_u32(this + 0x20, base);
        this
    }

    // ---- 005789b0 -------------------------------------------------------

    #[test]
    fn add_lock_change_marks_the_reference_or_the_linked_one() {
        let mut e = engine2();
        let linked = refr(&mut e, 0x30, &[]);
        let teleport = e.mem.alloc(8);
        e.mem.set_u32(teleport, linked);
        let this = refr(&mut e, 0x30, &[]);
        // the reference has a lock of its own
        returns(&mut e, EXTRA_GET_LOCK, 1);
        start_log(&mut e);
        e.call(0x0057_89b0, &args![this]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, MARK_CHANGED), vec![vec![this, 0x1000]]);
        // no lock: the teleport data's linked reference decides
        e.register_double(EXTRA_GET_LOCK, move |_, a| Ret {
            eax: u32::from(a[0] == linked),
            ..Ret::default()
        });
        returns(&mut e, GET_TELEPORT_DATA, teleport);
        start_log(&mut e);
        e.call(0x0057_89b0, &args![this]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, MARK_CHANGED), vec![vec![linked, 0x1000]]);
        // the linked reference has no lock either
        e.mem.set_u32(teleport, this);
        start_log(&mut e);
        e.call(0x0057_89b0, &args![this]);
        assert!(calls_to(&take_log(&mut e), MARK_CHANGED).is_empty());
        // no teleport data, no first dword
        returns(&mut e, GET_TELEPORT_DATA, 0);
        start_log(&mut e);
        e.call(0x0057_89b0, &args![this]);
        assert!(calls_to(&take_log(&mut e), MARK_CHANGED).is_empty());
    }

    // ---- 00578a30 -------------------------------------------------------

    #[test]
    fn set_last_finished_sequence_keeps_an_existing_one_while_loading() {
        let mut e = engine2();
        let this = refr(&mut e, 0x30, &[]);
        stub(&mut e, &[0x0042_2850]);
        returns(&mut e, 0x0047_c850, 0);
        returns(&mut e, 0x0042_28f0, 5);
        start_log(&mut e);
        e.call(0x0057_8a30, &args![this, 0x77u32]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0042_2850), vec![vec![this + 0x44, 0x77]]);
        // the save/load object reports something and a sequence exists
        returns(&mut e, 0x0047_c850, 1);
        start_log(&mut e);
        e.call(0x0057_8a30, &args![this, 0x77u32]);
        assert!(calls_to(&take_log(&mut e), 0x0042_2850).is_empty());
        // ... and none exists
        returns(&mut e, 0x0042_28f0, 0);
        start_log(&mut e);
        e.call(0x0057_8a30, &args![this, 0x78u32]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0042_2850), vec![vec![this + 0x44, 0x78]]);
    }

    // ---- 005790b0, 005790d0 ---------------------------------------------

    #[test]
    fn the_sound_range_getters_read_the_settings_floats() {
        let mut e = engine2();
        let setting = e.mem.alloc(8);
        e.mem.set_f32(setting, 12.5);
        e.register_double(0x0040_3e20, move |_, a| {
            assert!(a[0] == OBJECT_SOUND_RANGE_MIN || a[0] == OBJECT_SOUND_RANGE_MAX);
            Ret {
                eax: setting,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x0057_90b0, &args![]).f32(), 12.5);
        e.mem.set_f32(setting, 3.0);
        assert_eq!(e.call(0x0057_90d0, &args![]).f32(), 3.0);
    }

    // ---- 005790f0 -------------------------------------------------------

    #[test]
    fn the_completion_callback_flags_the_say_to_extra_of_an_actor() {
        let mut e = engine2();
        let say_to = e.mem.alloc(0x20);
        returns(&mut e, EXTRA_GET_SAY_TO, say_to);
        let speaker = refr(&mut e, 0x2a, &[(0xf0, RETURNS_TRUE)]);
        returns(&mut e, 0x0048_39c0, speaker);
        start_log(&mut e);
        e.call(0x0057_90f0, &args![0x1234u32, 0u32]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0048_39c0), vec![vec![0x1234]]);
        assert_eq!(e.mem.u8(say_to + 0x18), 1);
        // not an actor: untouched
        e.mem.set_u8(say_to + 0x18, 0);
        let other = refr(&mut e, 0x2a, &[(0xf0, RETURNS_FALSE)]);
        returns(&mut e, 0x0048_39c0, other);
        e.call(0x0057_90f0, &args![0x1234u32, 0u32]);
        assert_eq!(e.mem.u8(say_to + 0x18), 0);
        // no reference for the id, no say-to extra
        returns(&mut e, 0x0048_39c0, 0);
        e.call(0x0057_90f0, &args![0x1234u32, 0u32]);
        returns(&mut e, 0x0048_39c0, speaker);
        returns(&mut e, EXTRA_GET_SAY_TO, 0);
        e.call(0x0057_90f0, &args![0x1234u32, 0u32]);
    }

    // ---- 00579160 -------------------------------------------------------

    #[test]
    fn update_sound_call_back_runs_the_result_and_removes_the_extra() {
        let mut e = engine2();
        let this = refr(&mut e, 0x30, &[]);
        let say_to = e.mem.alloc(0x20);
        let info = e.mem.alloc(0x40);
        let script = 0x7000;
        e.mem.set_u32(say_to + 0xc, info);
        e.mem.set_u32(say_to + 0x10, script);
        e.mem.set_u8(say_to + 0x18, 1);
        returns(&mut e, EXTRA_GET_SAY_TO, say_to);
        stub(
            &mut e,
            &[0x0061_f170, 0x005a_c750, EXTRA_REMOVE_SAY_TO_INFO],
        );
        start_log(&mut e);
        e.call(0x0057_9160, &args![this]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0061_f170), vec![vec![info, 1, this]]);
        assert_eq!(
            calls_to(&log, 0x005a_c750),
            vec![vec![script, this + 0x44, 0x0004_0000]]
        );
        assert_eq!(
            calls_to(&log, EXTRA_REMOVE_SAY_TO_INFO),
            vec![vec![this + 0x44]]
        );
        // the info's result already ran (flag 8 at +0x25), no script
        e.mem.set_u8(info + 0x25, 8);
        e.mem.set_u32(say_to + 0x10, 0);
        start_log(&mut e);
        e.call(0x0057_9160, &args![this]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0061_f170).is_empty());
        assert!(calls_to(&log, 0x005a_c750).is_empty());
        assert_eq!(calls_to(&log, EXTRA_REMOVE_SAY_TO_INFO).len(), 1);
        // not flagged: nothing happens
        e.mem.set_u8(say_to + 0x18, 0);
        start_log(&mut e);
        e.call(0x0057_9160, &args![this]);
        assert!(calls_to(&take_log(&mut e), EXTRA_REMOVE_SAY_TO_INFO).is_empty());
    }

    // ---- 00579200, 00579220 ---------------------------------------------

    #[test]
    fn fn_00579200_tests_bit_8_of_the_byte_at_0x25() {
        let mut e = engine2();
        let object = e.mem.alloc(0x40);
        assert!(!e.call(0x0057_9200, &args![object]).bool());
        e.mem.set_u8(object + 0x25, 0x08);
        assert!(e.call(0x0057_9200, &args![object]).bool());
        e.mem.set_u8(object + 0x25, 0xf7);
        assert!(!e.call(0x0057_9200, &args![object]).bool());
    }

    #[test]
    fn fn_00579220_asks_the_destruction_form_unless_blocked() {
        let mut e = engine2();
        let this = refr(&mut e, 0x30, &[]);
        let base = e.mem.u32(this + 0x20);
        returns(&mut e, 0x0045_2370, 1);
        returns(&mut e, 0x0047_7ba0, 0);
        returns(&mut e, 0x0047_5400, 0x5555);
        stub(&mut e, &[0x0047_5b20]);
        start_log(&mut e);
        e.call(0x0057_9220, &args![this, 2.5f32, 0u8]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0047_5400), vec![vec![base]]);
        assert_eq!(
            calls_to(&log, 0x0047_5b20),
            vec![vec![0x5555, this, 2.5f32.to_bits(), 0]]
        );
        // 00477ba0 holds and the flag is clear: nothing
        returns(&mut e, 0x0047_7ba0, 1);
        start_log(&mut e);
        e.call(0x0057_9220, &args![this, 2.5f32, 0u8]);
        assert!(calls_to(&take_log(&mut e), 0x0047_5b20).is_empty());
        // ... with the flag set it runs
        start_log(&mut e);
        e.call(0x0057_9220, &args![this, 1.0f32, 1u8]);
        assert_eq!(calls_to(&take_log(&mut e), 0x0047_5b20).len(), 1);
        // 00452370 false: nothing
        returns(&mut e, 0x0045_2370, 0);
        start_log(&mut e);
        e.call(0x0057_9220, &args![this, 1.0f32, 1u8]);
        assert!(calls_to(&take_log(&mut e), 0x0047_5b20).is_empty());
    }

    // ---- 00579280 -------------------------------------------------------

    fn target_type(e: &mut Engine, this: u32) -> u32 {
        e.call(0x0057_9280, &args![this]).u32()
    }

    #[test]
    fn get_target_type_by_the_base_form_type() {
        let mut e = engine2();
        returns(&mut e, 0x0047_7ba0, 0);
        for (kind, expected) in [
            (0x19u8, 6u32),
            (0x1b, 2),
            (0x23, 0xe),
            (0x17, 4),
            (0x18, 1),
            (0x26, 1),
            (0x74, 1),
            (0x30, 0),
        ] {
            let this = refr(&mut e, kind, &[]);
            assert_eq!(target_type(&mut e, this), expected, "type {kind:#x}");
        }
        // a non-actor for which 00477ba0 holds has no target type
        returns(&mut e, 0x0047_7ba0, 1);
        let this = refr(&mut e, 0x19, &[]);
        assert_eq!(target_type(&mut e, this), 0);
        // ... unless it is an actor
        let actor = refr(&mut e, 0x19, &[(0x100, RETURNS_TRUE)]);
        assert_eq!(target_type(&mut e, actor), 6);
    }

    #[test]
    fn get_target_type_of_a_form_through_its_object_reference() {
        let mut e = engine2();
        returns(&mut e, 0x0047_7ba0, 0);
        // 0x15: the cast result decides
        let this = refr(&mut e, 0x15, &[]);
        let base = e.mem.u32(this + 0x20);
        e.register_double(DYNAMIC_CAST, move |_, a| {
            assert_eq!(a[0], base);
            assert_eq!(&a[2..4], &[TYPE_CAST_SOURCE, TYPE_CAST_TARGET]);
            Ret {
                eax: 0x9990,
                ..Ret::default()
            }
        });
        returns(&mut e, 0x0048_cee0, 1);
        assert_eq!(target_type(&mut e, this), 4);
        returns(&mut e, 0x0048_cee0, 0);
        assert_eq!(target_type(&mut e, this), 0);
        returns(&mut e, DYNAMIC_CAST, 0);
        assert_eq!(e.call(0x0057_9620, &args![base]).u32(), 0);
        // 0x27: sit, sleep or neither
        let this = refr(&mut e, 0x27, &[]);
        returns(&mut e, 0x0050_93f0, 1);
        returns(&mut e, 0x0050_9420, 0);
        assert_eq!(target_type(&mut e, this), 3);
        returns(&mut e, 0x0050_93f0, 0);
        assert_eq!(target_type(&mut e, this), 0);
        returns(&mut e, 0x0050_9420, 1);
        assert_eq!(target_type(&mut e, this), 5);
        // 0x1c: by the length of what 0055d520 returns
        let this = refr(&mut e, 0x1c, &[]);
        returns(&mut e, 0x0055_d520, 0x5000);
        returns(&mut e, 0x0044_a670, 3);
        assert_eq!(target_type(&mut e, this), 8);
        returns(&mut e, 0x0044_a670, 0);
        assert_eq!(target_type(&mut e, this), 0);
        // 0x1e
        let this = refr(&mut e, 0x1e, &[]);
        returns(&mut e, 0x0046_f070, 1);
        assert_eq!(target_type(&mut e, this), 1);
        returns(&mut e, 0x0046_f070, 0);
        assert_eq!(target_type(&mut e, this), 4);
    }

    #[test]
    fn get_target_type_of_an_actor() {
        let mut e = engine2();
        returns(&mut e, 0x0047_7ba0, 0);
        let player = refr(
            &mut e,
            0x2a,
            &[(0x100, RETURNS_TRUE), (0x22c, RETURNS_FALSE)],
        );
        e.set_global(GLOBAL_PLAYER, player);
        let flagged = 0x00ff_0110;
        returns(&mut e, flagged, 0);
        let actor = refr(&mut e, 0x2a, &[(0x100, RETURNS_TRUE), (0x22c, flagged)]);
        returns(&mut e, 0x004f_8960, 6);
        returns(&mut e, 0x0047_c850, 0);
        returns(&mut e, 0x0087_f3d0, 0);
        returns(&mut e, 0x0049_97b0, 0);
        // the player itself
        assert_eq!(target_type(&mut e, player), 0);
        // nothing special: 7
        assert_eq!(target_type(&mut e, actor), 7);
        // the player's flag makes the actor a friend (1) unless it is a
        // 0x16 base form
        returns(&mut e, 0x0049_97b0, 1);
        assert_eq!(target_type(&mut e, actor), 1);
        let creature = refr(&mut e, 0x16, &[(0x100, RETURNS_TRUE), (0x22c, flagged)]);
        assert_eq!(target_type(&mut e, creature), 7);
        // essential (0087f3d0): 0xa
        returns(&mut e, 0x0087_f3d0, 1);
        assert_eq!(target_type(&mut e, actor), 0xa);
        // the word at +0x1ac is 9 and 0047c850 holds: 0xb
        e.mem.set_u32(actor + 0x1ac, 9);
        returns(&mut e, 0x0047_c850, 1);
        assert_eq!(target_type(&mut e, actor), 0xb);
        // flagged and 004f8960 is not 6: 2
        returns(&mut e, flagged, 1);
        returns(&mut e, 0x004f_8960, 5);
        assert_eq!(target_type(&mut e, actor), 2);
        returns(&mut e, 0x004f_8960, 6);
        assert_eq!(target_type(&mut e, actor), 0xb);
        // not an actor at all (type 0x16, IsActor false): 7 or 1
        let thing = refr(&mut e, 0x16, &[(0x22c, RETURNS_FALSE)]);
        returns(&mut e, 0x0047_c850, 0);
        assert_eq!(target_type(&mut e, thing), 7);
    }

    #[test]
    fn get_target_type_of_a_creature() {
        let mut e = engine2();
        returns(&mut e, 0x0047_7ba0, 0);
        let player = refr(
            &mut e,
            0x2a,
            &[(0x100, RETURNS_TRUE), (0x22c, RETURNS_FALSE)],
        );
        e.set_global(GLOBAL_PLAYER, player);
        let flagged = 0x00ff_0111;
        returns(&mut e, flagged, 0);
        let creature = refr(&mut e, 0x2b, &[(0x100, RETURNS_TRUE), (0x22c, flagged)]);
        let slot18 = 0x00ff_0112;
        returns(&mut e, slot18, 1);
        let process = e.mem.alloc(0x40);
        let inner = object_with(&mut e, 0x10, &[(0x18, slot18)]);
        e.mem.set_u32(process + 0x30, e.mem.u32(inner));
        returns(&mut e, 0x0041_81e0, process);
        returns(&mut e, 0x0056_6950, 1);
        returns(&mut e, 0x004f_8960, 6);
        returns(&mut e, 0x0087_f3d0, 0);
        returns(&mut e, 0x0049_97b0, 0);
        assert_eq!(target_type(&mut e, creature), 7);
        returns(&mut e, 0x0049_97b0, 1);
        assert_eq!(target_type(&mut e, creature), 1);
        returns(&mut e, 0x0087_f3d0, 1);
        assert_eq!(target_type(&mut e, creature), 0xa);
        // the process object's virtual says no: 0
        returns(&mut e, slot18, 0);
        assert_eq!(target_type(&mut e, creature), 0);
        returns(&mut e, slot18, 1);
        // flagged: 2 unless 00566950 and 004f8960 == 6
        returns(&mut e, flagged, 1);
        assert_eq!(target_type(&mut e, creature), 0xa);
        returns(&mut e, 0x004f_8960, 4);
        assert_eq!(target_type(&mut e, creature), 2);
        returns(&mut e, 0x004f_8960, 6);
        returns(&mut e, 0x0056_6950, 0);
        assert_eq!(target_type(&mut e, creature), 2);
        // a creature base form on a non-actor reference: 0
        let thing = refr(&mut e, 0x2b, &[]);
        assert_eq!(target_type(&mut e, thing), 0);
    }

    // ---- 00579620, 00579670 ---------------------------------------------

    #[test]
    fn fn_00579620_asks_0048cee0_only_when_the_cast_works() {
        let mut e = engine2();
        e.register_double(DYNAMIC_CAST, |_, a| Ret {
            eax: if a[0] == 0x1000 { 0x2000 } else { 0 },
            ..Ret::default()
        });
        e.register_double(0x0048_cee0, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0057_9620, &args![0x1000u32]).u32(), 0x2001);
        assert_eq!(e.call(0x0057_9620, &args![0x1001u32]).u32(), 0);
    }

    #[test]
    fn fn_00579670_compares_the_word_at_0x1ac_with_nine() {
        let mut e = engine2();
        let object = e.mem.alloc(0x200);
        assert!(!e.call(0x0057_9670, &args![object]).bool());
        e.mem.set_u32(object + 0x1ac, 9);
        assert!(e.call(0x0057_9670, &args![object]).bool());
        e.mem.set_u32(object + 0x1ac, 8);
        assert!(!e.call(0x0057_9670, &args![object]).bool());
    }

    // ---- 00579690 -------------------------------------------------------

    /// The world `00579690` reads: a player whose `+0x22c` is the returned
    /// flag and who is never an owner; `owner` is what `00567790` returns
    /// for the reference.
    fn stealing_world(e: &mut Engine, owner: u32) -> u32 {
        let player_form = form(e, 0x30, 0);
        let get_form = 0x00ff_0120;
        returns(e, get_form, player_form);
        let player = refr(
            e,
            0x2a,
            &[
                (0x100, RETURNS_TRUE),
                (0x22c, RETURNS_FALSE),
                (0x1a4, get_form),
            ],
        );
        e.set_global(GLOBAL_PLAYER, player);
        returns(e, 0x0056_9160, 0);
        returns(e, 0x0056_7790, owner);
        returns(e, 0x0056_79f0, 0);
        returns(e, 0x0056_7960, 0);
        returns(e, 0x0049_97b0, 0);
        player
    }

    #[test]
    fn fn_00579690_an_owned_plain_object_is_stealing_unless_the_player_owns_it() {
        let mut e = engine2();
        let owner = form(&mut e, 0x2a, 0);
        let player = stealing_world(&mut e, owner);
        let this = refr(&mut e, 0x30, &[]);
        assert!(e.call(0x0057_9690, &args![this]).bool());
        // no owner: nobody is stealing
        returns(&mut e, 0x0056_7790, 0);
        assert!(!e.call(0x0057_9690, &args![this]).bool());
        // a creature base form is never "owned"
        returns(&mut e, 0x0056_7790, owner);
        let creature = refr(&mut e, 0x2b, &[]);
        assert!(!e.call(0x0057_9690, &args![creature]).bool());
        // the player's own flag (+0x22c) turns everything off
        let flagged = 0x00ff_0121;
        returns(&mut e, flagged, 1);
        e.mem.set_u32(e.mem.u32(player) + 0x22c, flagged);
        assert!(!e.call(0x0057_9690, &args![this]).bool());
    }

    #[test]
    fn fn_00579690_beds_and_actors() {
        let mut e = engine2();
        let owner = form(&mut e, 0x2a, 0);
        stealing_world(&mut e, owner);
        // a furniture base form: needs 00509420 and an owner
        let bed = refr(&mut e, 0x27, &[]);
        returns(&mut e, 0x0050_9420, 1);
        assert!(e.call(0x0057_9690, &args![bed]).bool());
        returns(&mut e, 0x0050_9420, 0);
        // not sleepable: falls through to the generic owner test (owned)
        assert!(e.call(0x0057_9690, &args![bed]).bool());
        returns(&mut e, 0x0056_7790, 0);
        assert!(!e.call(0x0057_9690, &args![bed]).bool());
        returns(&mut e, 0x0056_7790, owner);
        // an actor base form (0x2a) the player may not use
        let flagged = 0x00ff_0122;
        returns(&mut e, flagged, 0);
        let actor = refr(&mut e, 0x2a, &[(0x22c, flagged)]);
        returns(&mut e, 0x0056_7790, 0);
        returns(&mut e, 0x0049_97b0, 1);
        assert!(e.call(0x0057_9690, &args![actor]).bool());
        returns(&mut e, flagged, 1);
        assert!(!e.call(0x0057_9690, &args![actor]).bool());
        // a creature base form: the cast target's virtual +0x18 decides
        let slot18 = 0x00ff_0123;
        returns(&mut e, slot18, 1);
        let target = e.mem.alloc(0x40);
        let inner = object_with(&mut e, 0x10, &[(0x18, slot18)]);
        e.mem.set_u32(target + 0x30, e.mem.u32(inner));
        returns(&mut e, DYNAMIC_CAST, target);
        let creature = refr(&mut e, 0x2b, &[(0x22c, flagged)]);
        returns(&mut e, flagged, 0);
        assert!(e.call(0x0057_9690, &args![creature]).bool());
        returns(&mut e, slot18, 0);
        assert!(!e.call(0x0057_9690, &args![creature]).bool());
    }

    /// `00567790` returns `owner` for its first `calls` calls and 0 after,
    /// so that the final owned-object test of `00579690` (the fourth call
    /// on) does not mask the result of the door rules.
    fn owner_for_first_calls(e: &mut Engine, owner: u32, calls: u32) {
        let count = std::rc::Rc::new(std::cell::Cell::new(0u32));
        e.register_double(0x0056_7790, move |_, _| {
            let n = count.get();
            count.set(n + 1);
            Ret {
                eax: if n < calls { owner } else { 0 },
                ..Ret::default()
            }
        });
    }

    #[test]
    fn fn_00579690_doors() {
        let mut e = engine2();
        let owner = form(&mut e, 0x2a, 0);
        let player = stealing_world(&mut e, owner);
        let door = refr(&mut e, 0x1c, &[]);
        // the door has an owner the player is not part of: the lock rules
        owner_for_first_calls(&mut e, owner, 3);
        returns(&mut e, 0x0051_8f00, 1);
        assert!(!e.call(0x0057_9690, &args![door]).bool());
        returns(&mut e, 0x0051_8f00, 0);
        returns(&mut e, 0x0056_9160, 0x6000);
        returns(&mut e, 0x0050_21a0, 1);
        owner_for_first_calls(&mut e, owner, 3);
        start_log(&mut e);
        assert!(e.call(0x0057_9690, &args![door]).bool());
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0051_8f00), vec![vec![door, player, 0, 0]]);
        assert_eq!(calls_to(&log, 0x0050_21a0), vec![vec![0x6000]]);
        // an unlocked door is judged by the teleport data's cell owner
        returns(&mut e, 0x0050_21a0, 0);
        returns(&mut e, GET_TELEPORT_DATA, 0);
        owner_for_first_calls(&mut e, owner, 3);
        assert!(!e.call(0x0057_9690, &args![door]).bool());
        let teleport = e.mem.alloc(8);
        let cell = e.mem.alloc(0x40);
        let cell_owner = form(&mut e, 0x2a, 0);
        returns(&mut e, GET_TELEPORT_DATA, teleport);
        returns(&mut e, 0x0043_a2b0, cell);
        returns(&mut e, 0x0054_6a40, cell_owner);
        returns(&mut e, 0x0042_5fd0, 1);
        returns(&mut e, 0x0054_43e0, 0);
        returns(&mut e, 0x0047_d740, 0);
        let judge = |e: &mut Engine| {
            owner_for_first_calls(e, owner, 3);
            e.call(0x0057_9690, &args![door]).bool()
        };
        assert!(judge(&mut e));
        // ... which says no for an evil faction, a flagged cell, a cell
        // without the 00425fd0 property, or without owner
        returns(&mut e, 0x0047_d740, 1);
        assert!(!judge(&mut e));
        returns(&mut e, 0x0047_d740, 0);
        returns(&mut e, 0x0054_43e0, 1);
        assert!(!judge(&mut e));
        returns(&mut e, 0x0054_43e0, 0);
        returns(&mut e, 0x0042_5fd0, 0);
        assert!(!judge(&mut e));
        returns(&mut e, 0x0042_5fd0, 1);
        returns(&mut e, 0x0054_6a40, 0);
        assert!(!judge(&mut e));
        // an owner of form type 8 is judged by 0047d7c0
        let npc = form(&mut e, 8, 0);
        returns(&mut e, 0x0054_6a40, npc);
        returns(&mut e, 0x0047_d7c0, 0);
        assert!(judge(&mut e));
        returns(&mut e, 0x0047_d7c0, 1);
        assert!(!judge(&mut e));
        // a door without owner of its own uses the stricter second copy
        // (00546ca0 against the player)
        returns(&mut e, 0x0056_7790, 0);
        returns(&mut e, 0x0047_d7c0, 0);
        returns(&mut e, 0x0054_6ca0, 0);
        assert!(e.call(0x0057_9690, &args![door]).bool());
        start_log(&mut e);
        returns(&mut e, 0x0054_6ca0, 1);
        assert!(!e.call(0x0057_9690, &args![door]).bool());
        assert_eq!(
            calls_to(&take_log(&mut e), 0x0054_6ca0),
            vec![vec![cell, player]]
        );
    }

    // ---- 00579ac0 -------------------------------------------------------

    /// The doubles `00579ac0` reaches for a reference without 3D whose sound
    /// handle is not valid.
    fn sound_world(e: &mut Engine) {
        stub(
            e,
            &[
                0x0044_0da0,
                0x0047_7ba0,
                HANDLE_CONSTRUCT,
                HANDLE_DESTRUCT,
                EXTRA_GET_SOUND,
                HANDLE_IS_VALID,
                HANDLE_STOP,
                HANDLE_RELEASE,
                HANDLE_FADE_OUT_AND_RELEASE,
                EXTRA_REMOVE_TYPE,
                EXTRA_SET_SOUND,
                HANDLE_ASSIGN,
                0x0043_fcd0,
                0x004f_d3c0,
                0x0042_ce10,
                0x0042_26e0,
                0x0083_25b0,
                AUDIO_INSTANCE,
                0x004e_75d0,
            ],
        );
    }

    #[test]
    fn fn_00579ac0_starts_the_sound_of_a_door_like_form_in_the_extra() {
        let mut e = engine2();
        sound_world(&mut e);
        let this = refr(&mut e, 0x17, &[(0x1f4, SPARE_SLOT)]);
        let position = this + 0x30;
        returns(&mut e, SPARE_SLOT, position);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, 3.0);
        let base = e.mem.u32(this + 0x20);
        returns(&mut e, 0x004f_d3a0, 0x5000); // the sound form
        returns(&mut e, 0x0084_e3a0, 0x1234); // its form id
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(data + 4, 0x10); // positional
        returns(&mut e, 0x004e_75d0, data);
        returns(&mut e, AUDIO_INSTANCE, 0x7000);
        returns(&mut e, AUDIO_SPAWN_AT, 0x7100);
        // a valid old handle is stopped and its extra removed
        returns(&mut e, HANDLE_IS_VALID, 1);
        start_log(&mut e);
        e.call(0x0057_9ac0, &args![this, 1u8]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x004f_d3a0), vec![vec![base]]);
        assert_eq!(calls_to(&log, HANDLE_STOP).len(), 1);
        assert_eq!(
            calls_to(&log, EXTRA_REMOVE_TYPE),
            vec![vec![this + 0x44, 0x4f]]
        );
        let spawn = calls_to(&log, AUDIO_SPAWN_AT);
        assert_eq!(spawn.len(), 1);
        assert_eq!(spawn[0][0], 0x7000);
        assert_eq!(
            spawn[0][2..],
            [
                0x1234,
                0x2012,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                0
            ]
        );
        assert_eq!(calls_to(&log, EXTRA_SET_SOUND).len(), 1);
        // the 3D check is made at the end
        assert_eq!(calls_to(&log, 0x0043_fcd0), vec![vec![this]]);
    }

    #[test]
    fn fn_00579ac0_records_a_sound_without_position_in_prandomsound() {
        let mut e = engine2();
        sound_world(&mut e);
        let this = refr(&mut e, 0x1b, &[]);
        returns(&mut e, 0x0091_85e0, 0x5000);
        let data = e.mem.alloc(0x20);
        returns(&mut e, 0x004e_75d0, data);
        returns(&mut e, 0x0043_fcd0, 0x9000);
        returns(&mut e, 0x0045_23e0, 0);
        start_log(&mut e);
        e.call(0x0057_9ac0, &args![this, 1u8]);
        let log = take_log(&mut e);
        assert_eq!(e.mem.u32(this + 0x1c), 0x5000);
        assert!(calls_to(&log, AUDIO_SPAWN_AT).is_empty());
        assert_eq!(calls_to(&log, EXTRA_SET_SOUND).len(), 1);
        // the addon nodes are asked with the start flag
        assert_eq!(calls_to(&log, 0x0045_23e0), vec![vec![this, 2]]);
        // no sound form found: nothing is stored
        returns(&mut e, 0x0091_85e0, 0);
        e.mem.set_u32(this + 0x1c, 0);
        start_log(&mut e);
        e.call(0x0057_9ac0, &args![this, 1u8]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, EXTRA_SET_SOUND).is_empty());
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        // 00440da0 holds: nothing is started at all
        returns(&mut e, 0x0044_0da0, 1);
        start_log(&mut e);
        e.call(0x0057_9ac0, &args![this, 1u8]);
        assert!(calls_to(&take_log(&mut e), HANDLE_CONSTRUCT).is_empty());
    }

    #[test]
    fn fn_00579ac0_picks_the_sound_by_the_base_form_type() {
        let mut e = engine2();
        sound_world(&mut e);
        let data = e.mem.alloc(0x20);
        returns(&mut e, 0x004e_75d0, data);
        // type 0x20: the word at base + 0x54
        let this = refr(&mut e, 0x20, &[]);
        let base = e.mem.u32(this + 0x20);
        e.mem.set_u32(base + 0x54, 0x5001);
        e.call(0x0057_9ac0, &args![this, 1u8]);
        assert_eq!(e.mem.u32(this + 0x1c), 0x5001);
        // type 0x22: 005e3fa0 of base - 0x14
        let this = refr(&mut e, 0x22, &[]);
        let base = e.mem.u32(this + 0x20);
        e.register_double(0x005e_3fa0, move |_, a| Ret {
            eax: if a[0] == base - 0x14 { 0x5002 } else { 0 },
            ..Ret::default()
        });
        e.call(0x0057_9ac0, &args![this, 1u8]);
        assert_eq!(e.mem.u32(this + 0x1c), 0x5002);
        // types 0x1c, 0x1e, 0x1f call their own getters
        for (kind, getter, sound) in [
            (0x1cu8, 0x004f_b070u32, 0x5003u32),
            (0x1e, 0x0040_36b0, 0x5004),
            (0x1f, 0x008c_dd90, 0x5005),
        ] {
            let this = refr(&mut e, kind, &[]);
            let base = e.mem.u32(this + 0x20);
            e.register_double(getter, move |_, a| Ret {
                eax: if a[0] == base { sound } else { 0 },
                ..Ret::default()
            });
            e.call(0x0057_9ac0, &args![this, 1u8]);
            assert_eq!(e.mem.u32(this + 0x1c), sound, "type {kind:#x}");
        }
        // type 0x23: only when 00452440 and 004e32c0 say no and the
        // virtual +0x140 gives something
        let slot140 = 0x00ff_0130;
        returns(&mut e, slot140, 0x6000);
        let this = refr(&mut e, 0x23, &[]);
        let base = e.mem.u32(this + 0x20);
        let vtable_owner = object_with(&mut e, 0x10, &[(0x140, slot140)]);
        let vtable = e.mem.u32(vtable_owner);
        e.mem.set_u32(base, vtable);
        stub(&mut e, &[0x0045_2440, 0x004e_32c0]);
        returns(&mut e, 0x0040_7840, 0x5006);
        e.call(0x0057_9ac0, &args![this, 1u8]);
        assert_eq!(e.mem.u32(this + 0x1c), 0x5006);
        // type 0x2b: actor and virtual +0x21c, then 005f92d0(base, 0x15)
        let slot21c = 0x00ff_0131;
        returns(&mut e, slot21c, 1);
        let this = refr(&mut e, 0x2b, &[(0x100, RETURNS_TRUE), (0x21c, slot21c)]);
        start_log(&mut e);
        returns(&mut e, 0x005f_92d0, 0x5007);
        e.call(0x0057_9ac0, &args![this, 1u8]);
        let base = e.mem.u32(this + 0x20);
        assert_eq!(
            calls_to(&take_log(&mut e), 0x005f_92d0),
            vec![vec![base, 0x15]]
        );
        assert_eq!(e.mem.u32(this + 0x1c), 0x5007);
    }

    #[test]
    fn fn_00579ac0_primitive_sounds_are_spawned_with_the_computed_vectors() {
        let mut e = engine2();
        sound_world(&mut e);
        let this = refr(&mut e, 0x0d, &[(0x1f4, SPARE_SLOT)]);
        let position = e.mem.alloc(16);
        returns(&mut e, SPARE_SLOT, position);
        for (i, v) in [10.0f32, 20.0, 30.0].iter().enumerate() {
            e.mem.set_f32(position + 4 * i as u32, *v);
        }
        let primitive = 0x5500;
        returns(&mut e, EXTRA_GET_PRIMITIVE, primitive);
        stub(&mut e, &[0x0056_fa00, 0x0063_c8a0]);
        // 00413fc0 hands out a vector; 004a3760 and 004b3ae0 return their
        // second/third argument vectors unchanged
        let box_vector = e.mem.alloc(16);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(box_vector + 4 * i as u32, *v);
        }
        returns(&mut e, 0x0041_3fc0, box_vector);
        e.register(0x004a_3760, |_, a| a[2].into_ret());
        e.register(0x004b_3ae0, |_, a| a[1].into_ret());
        e.set_global(MINUS_ONE, -1.0f32);
        let data = e.mem.alloc(0x20);
        returns(&mut e, 0x004e_75d0, data);
        returns(&mut e, 0x004f_9230, 0);
        returns(&mut e, 0x0084_e3a0, 0x777);
        returns(&mut e, AUDIO_INSTANCE, 0x7000);
        returns(&mut e, AUDIO_SPAWN_AT, 0x7100);
        stub(&mut e, &[HANDLE_SET_VECTOR]);
        start_log(&mut e);
        e.call(0x0057_9ac0, &args![this, 1u8]);
        let log = take_log(&mut e);
        // the box vector (1,2,3) is the direction and spawn point before
        // the middle components are swapped: both start as (1,2,3), the
        // swap changes nothing, 0063c8a0 adds the position (stubbed)
        let spawn = calls_to(&log, AUDIO_SPAWN_AT);
        assert_eq!(spawn.len(), 1);
        assert_eq!(spawn[0][3], 0x1000_0000 | 0x2012);
        assert_eq!(
            spawn[0][4..7],
            [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]
        );
        assert_eq!(calls_to(&log, HANDLE_SET_VECTOR).len(), 1);
        assert_eq!(calls_to(&log, 0x0056_fa00).len(), 1);
    }

    #[test]
    fn fn_00579ac0_stops_by_fading_out() {
        let mut e = engine2();
        sound_world(&mut e);
        let this = refr(&mut e, 0x17, &[]);
        returns(&mut e, HANDLE_IS_VALID, 1);
        start_log(&mut e);
        e.call(0x0057_9ac0, &args![this, 0u8]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, HANDLE_FADE_OUT_AND_RELEASE).len(), 1);
        assert_eq!(calls_to(&log, HANDLE_FADE_OUT_AND_RELEASE)[0][1], 1000);
        assert_eq!(
            calls_to(&log, EXTRA_REMOVE_TYPE),
            vec![vec![this + 0x44, 0x4f]]
        );
        assert!(calls_to(&log, 0x0083_25b0).is_empty());
        // a type 0x15 form with 004fd3c0 and nothing blocking: 008325b0
        let this = refr(&mut e, 0x15, &[]);
        returns(&mut e, 0x004f_d3c0, 1);
        start_log(&mut e);
        e.call(0x0057_9ac0, &args![this, 0u8]);
        assert_eq!(
            calls_to(&take_log(&mut e), 0x0083_25b0),
            vec![vec![this, 0]]
        );
        // a loading object blocks it
        returns(&mut e, 0x0042_ce10, 1);
        start_log(&mut e);
        e.call(0x0057_9ac0, &args![this, 0u8]);
        assert!(calls_to(&take_log(&mut e), 0x0083_25b0).is_empty());
        // a base form of another type: neither
        let this = refr(&mut e, 0x30, &[]);
        start_log(&mut e);
        e.call(0x0057_9ac0, &args![this, 0u8]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, HANDLE_CONSTRUCT).is_empty());
        // no base form: nothing at all
        let nothing = object_with(&mut e, 0x100, &[(0x100, RETURNS_FALSE)]);
        start_log(&mut e);
        e.call(0x0057_9ac0, &args![nothing, 0u8]);
        assert!(calls_to(&take_log(&mut e), 0x0043_fcd0).is_empty());
    }

    // ---- 0057a2f0, 0057a370, 0057a3c0 -----------------------------------

    #[test]
    fn fn_0057a2f0_accepts_the_sound_carrying_types() {
        let mut e = engine2();
        for kind in 0u8..0x80 {
            let this = refr(&mut e, kind, &[]);
            let expected = matches!(
                kind,
                0x0d | 0x15 | 0x17 | 0x1b | 0x1c | 0x1e | 0x1f | 0x20 | 0x22 | 0x23 | 0x2b
            );
            assert_eq!(
                e.call(0x0057_a2f0, &args![this]).bool(),
                expected,
                "type {kind:#x}"
            );
        }
        let without_base = object_with(&mut e, 0x100, &[]);
        assert!(!e.call(0x0057_a2f0, &args![without_base]).bool());
    }

    #[test]
    fn fn_0057a370_excludes_blocked_references_and_door_exceptions() {
        let mut e = engine2();
        returns(&mut e, 0x0043_7b90, 0);
        returns(&mut e, 0x0047_7ba0, 0);
        let door = refr(&mut e, 0x1c, &[]);
        let other = refr(&mut e, 0x30, &[]);
        assert!(e.call(0x0057_a370, &args![door]).bool());
        assert!(e.call(0x0057_a370, &args![other]).bool());
        returns(&mut e, 0x0047_7ba0, 1);
        assert!(!e.call(0x0057_a370, &args![door]).bool());
        assert!(e.call(0x0057_a370, &args![other]).bool());
        returns(&mut e, 0x0043_7b90, 1);
        assert!(!e.call(0x0057_a370, &args![other]).bool());
    }

    #[test]
    fn update_addon_node_sounds_needs_a_3d_and_the_check() {
        let mut e = engine2();
        let this = refr(&mut e, 0x30, &[]);
        returns(&mut e, 0x0043_fcd0, 0x9000);
        returns(&mut e, 0x0045_23e0, 1);
        stub(&mut e, &[0x0045_6610, 0x009e_e040]);
        returns(&mut e, 0x0046_17e0, 0);
        let node = node(&mut e, &[]);
        returns(&mut e, 0x0043_fcd0, node);
        start_log(&mut e);
        e.call(0x0057_a3c0, &args![this, 1u8]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0045_23e0), vec![vec![this, 2]]);
        assert_eq!(calls_to(&log, 0x0045_6610), vec![vec![node]]);
        // the check fails
        returns(&mut e, 0x0045_23e0, 0);
        start_log(&mut e);
        e.call(0x0057_a3c0, &args![this, 1u8]);
        assert!(calls_to(&take_log(&mut e), 0x0045_6610).is_empty());
        // no 3D
        returns(&mut e, 0x0045_23e0, 1);
        returns(&mut e, 0x0043_fcd0, 0);
        start_log(&mut e);
        e.call(0x0057_a3c0, &args![this, 1u8]);
        assert!(calls_to(&take_log(&mut e), 0x0045_6610).is_empty());
    }

    // ---- 0057a410 -------------------------------------------------------

    /// Doubles for the sound-handle calls of `0057a410`; the smart pointer
    /// at `slot` holds what `00a5bdd0` returned.
    fn addon_sound_world(e: &mut Engine, existing: u32) {
        e.register(0x0063_3c90, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(0x0066_b0d0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        returns(e, 0x00a5_bdd0, existing);
        stub(
            e,
            &[
                0x0045_cec0,
                HANDLE_STOP,
                HANDLE_RELEASE,
                0x00a5_be90,
                HANDLE_ASSIGN,
                HANDLE_DESTRUCT,
                HANDLE_SET_FOLLOW,
                HANDLE_SET_POSITION,
                HANDLE_PLAY,
                0x00a5_bca0,
                0x004f_9230,
                0x009e_e040,
            ],
        );
    }

    #[test]
    fn fn_0057a410_makes_and_plays_the_sound_of_an_addon_node() {
        let mut e = engine2();
        let this = refr(&mut e, 0x30, &[]);
        let node = node(&mut e, &[]);
        let record = e.mem.alloc(0x20);
        addon_sound_world(&mut e, 0);
        returns(&mut e, 0x0045_6610, 0);
        returns(&mut e, 0x0046_17e0, record);
        let sound = form(&mut e, 0x30, 0);
        e.mem.set_u32(sound + 0x0c, 0x4321);
        returns(&mut e, 0x006a_1b60, sound);
        let holder = e.mem.alloc(0x20);
        returns(&mut e, 0x00aa_13e0, holder);
        returns(&mut e, 0x004f_91b0, holder);
        let info = e.mem.alloc(0x20);
        returns(&mut e, 0x004e_75d0, info);
        returns(&mut e, 0x005e_39b0, 0);
        returns(&mut e, AUDIO_INSTANCE, 0x7000);
        returns(&mut e, AUDIO_HANDLE_BY_ID, 0x7100);
        let position = e.mem.alloc(16);
        for (i, v) in [4.0f32, 5.0, 6.0].iter().enumerate() {
            e.mem.set_f32(position + 4 * i as u32, *v);
        }
        returns(&mut e, 0x0045_bb80, position);
        start_log(&mut e);
        e.call(0x0057_a410, &args![this, node, 0u8]);
        let log = take_log(&mut e);
        let handle = holder + 0xc;
        let by_id = calls_to(&log, AUDIO_HANDLE_BY_ID);
        assert_eq!(by_id.len(), 1);
        assert_eq!(by_id[0][0], 0x7000);
        assert_eq!(by_id[0][2..], [0x4321, 0x102]);
        assert_eq!(calls_to(&log, HANDLE_ASSIGN), vec![vec![handle, 0x7100]]);
        assert_eq!(calls_to(&log, HANDLE_SET_FOLLOW), vec![vec![handle, node]]);
        assert_eq!(
            calls_to(&log, HANDLE_SET_POSITION),
            vec![vec![
                handle,
                4.0f32.to_bits(),
                5.0f32.to_bits(),
                6.0f32.to_bits()
            ]]
        );
        assert_eq!(calls_to(&log, HANDLE_PLAY), vec![vec![handle, 0]]);
        assert_eq!(calls_to(&log, 0x00a5_bca0), vec![vec![node, holder]]);
        assert!(calls_to(&log, HANDLE_STOP).is_empty());
        // a sound type 5e39b0 knows adds 2 to its result
        returns(&mut e, 0x005e_39b0, 0x20);
        start_log(&mut e);
        e.call(0x0057_a410, &args![this, node, 0u8]);
        let by_id = calls_to(&take_log(&mut e), AUDIO_HANDLE_BY_ID);
        assert_eq!(by_id[0][3], 0x22);
    }

    #[test]
    fn fn_0057a410_stops_an_existing_handle_when_asked_to() {
        let mut e = engine2();
        let this = refr(&mut e, 0x30, &[]);
        let child = node(&mut e, &[]);
        let parent = node(&mut e, &[child]);
        let record = e.mem.alloc(0x20);
        let holder = e.mem.alloc(0x20);
        addon_sound_world(&mut e, holder);
        // only the child has an addon record; 00456610 holds for it, which
        // sets the flag
        e.register_double(0x0046_17e0, move |_, a| Ret {
            eax: if a[1] == child { record } else { 0 },
            ..Ret::default()
        });
        e.register(0x009e_e040, |_, a| a[0].into_ret());
        e.register_double(0x0045_6610, move |_, a| Ret {
            eax: u32::from(a[0] == child),
            ..Ret::default()
        });
        returns(&mut e, 0x006a_1b60, 0x6000);
        start_log(&mut e);
        e.call(0x0057_a410, &args![this, parent, 0u8]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0045_6610), vec![vec![parent], vec![child]]);
        assert_eq!(calls_to(&log, HANDLE_STOP), vec![vec![holder + 0xc]]);
        assert_eq!(calls_to(&log, HANDLE_RELEASE), vec![vec![holder + 0xc]]);
        assert_eq!(calls_to(&log, 0x00a5_be90).len(), 1);
        assert!(calls_to(&log, HANDLE_PLAY).is_empty());
        // a null node does nothing
        start_log(&mut e);
        e.call(0x0057_a410, &args![this, 0u32, 0u8]);
        assert!(calls_to(&take_log(&mut e), 0x0045_6610).is_empty());
    }

    // ---- 0057a740, 0057a770 ---------------------------------------------

    #[test]
    fn fn_0057a740_reads_word_c_of_the_record() {
        let mut e = engine2();
        let this = refr(&mut e, 0x30, &[]);
        returns(&mut e, 0x0052_7080, 0);
        assert_eq!(e.call(0x0057_a740, &args![this]).u32(), 0);
        let record = e.mem.alloc(0x20);
        e.mem.set_u32(record + 0xc, 0xabc);
        e.register_double(0x0052_7080, move |_, a| {
            assert_eq!(a[1], 0x6d);
            Ret {
                eax: record,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x0057_a740, &args![this]).u32(), 0xabc);
    }

    #[test]
    fn fn_0057a770_makes_the_record_when_missing() {
        let mut e = engine2();
        let this = refr(&mut e, 0x30, &[]);
        let existing = e.mem.alloc(0x20);
        e.mem.set_u32(existing + 0xc, 0x777);
        returns(&mut e, 0x0052_7080, existing);
        start_log(&mut e);
        assert_eq!(e.call(0x0057_a770, &args![this]).u32(), 0x777);
        assert!(calls_to(&take_log(&mut e), 0x0040_ff60).is_empty());
        // missing: allocate 0x18 and 0x10 bytes, link, add to the list
        returns(&mut e, 0x0052_7080, 0);
        let receiver = e.mem.alloc(0x18);
        let record = e.mem.alloc(0x10);
        let blocks = std::rc::Rc::new(std::cell::RefCell::new(vec![receiver, record]));
        e.register_double(0x0040_1000, move |_, a| {
            assert!(a[0] == 0x18 || a[0] == 0x10);
            Ret {
                eax: blocks.borrow_mut().remove(0),
                ..Ret::default()
            }
        });
        stub(
            &mut e,
            &[0x006e_cd40, 0x0040_ff60, 0x0040_ec80, 0x0057_c700],
        );
        start_log(&mut e);
        let result = e.call(0x0057_a770, &args![this]).u32();
        let log = take_log(&mut e);
        // the record wraps the receiver at +0x0c
        assert_eq!(result, receiver);
        assert_eq!(e.mem.u32(record + 0xc), receiver);
        assert_eq!(e.mem.u32(record), VTABLE_SAY_TO_RECORD);
        assert_eq!(e.mem.u32(receiver), VTABLE_ANIM_NOTE_RECEIVER);
        assert_eq!(calls_to(&log, 0x006e_cd40), vec![vec![receiver, this]]);
        assert_eq!(calls_to(&log, 0x0040_ff60), vec![vec![this + 0x44, record]]);
        assert_eq!(calls_to(&log, 0x0040_ec80), vec![vec![record, 0x6d]]);
        assert_eq!(calls_to(&log, 0x0057_c700), vec![vec![receiver + 8, 0, 1]]);
    }

    // ---- 0057a870 .. 0057a9d0 -------------------------------------------

    #[test]
    fn the_extra_record_constructor_and_destructor() {
        let mut e = engine2();
        stub(&mut e, &[0x0040_ec80, 0x0040_ecb0, 0x0040_1030]);
        let record = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0057_a870, &args![record, 0x1111u32]).u32(), record);
        assert_eq!(e.mem.u32(record), VTABLE_SAY_TO_RECORD);
        assert_eq!(e.mem.u32(record + 0xc), 0x1111);
        // the destructor deletes the wrapped object through its virtual 0
        let delete = 0x00ff_0140;
        stub(&mut e, &[delete]);
        let wrapped = object_with(&mut e, 0x20, &[(0, delete)]);
        e.mem.set_u32(record + 0xc, wrapped);
        e.mem.set_u32(record, 0);
        start_log(&mut e);
        e.call(0x0057_a8d0, &args![record]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, delete), vec![vec![wrapped, 1]]);
        assert_eq!(calls_to(&log, 0x0040_ecb0), vec![vec![record]]);
        assert_eq!(e.mem.u32(record), VTABLE_SAY_TO_RECORD);
        // nothing wrapped: no deletion
        e.mem.set_u32(record + 0xc, 0);
        start_log(&mut e);
        e.call(0x0057_a8d0, &args![record]);
        assert!(calls_to(&take_log(&mut e), delete).is_empty());
    }

    #[test]
    fn the_scalar_deleting_destructor_frees_on_bit_zero() {
        let mut e = engine2();
        stub(&mut e, &[0x0040_ec80, 0x0040_ecb0, 0x0040_1030]);
        let record = e.mem.alloc(0x10);
        start_log(&mut e);
        assert_eq!(e.call(0x0057_a8a0, &args![record, 1u32]).u32(), record);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0040_1030), vec![vec![record]]);
        assert_eq!(calls_to(&log, 0x0040_ecb0), vec![vec![record]]);
        start_log(&mut e);
        e.call(0x0057_a8a0, &args![record, 0u32]);
        assert!(calls_to(&take_log(&mut e), 0x0040_1030).is_empty());
        start_log(&mut e);
        e.call(0x0057_a8a0, &args![record, 2u32]);
        assert!(calls_to(&take_log(&mut e), 0x0040_1030).is_empty());
    }

    #[test]
    fn the_receiver_constructors_set_their_vtables() {
        let mut e = engine2();
        stub(&mut e, &[0x0057_c700]);
        let receiver = e.mem.alloc(0x18);
        start_log(&mut e);
        assert_eq!(e.call(0x0057_a960, &args![receiver]).u32(), receiver);
        let log = take_log(&mut e);
        assert_eq!(e.mem.u32(receiver), VTABLE_ANIM_NOTE_RECEIVER);
        assert_eq!(calls_to(&log, 0x0057_c700), vec![vec![receiver + 8, 0, 1]]);
        assert_eq!(e.call(0x0057_a9d0, &args![receiver]).u32(), receiver);
        assert_eq!(e.mem.u32(receiver), VTABLE_ANIM_NOTE_RECEIVER_BASE);
    }

    // ---- 0057a9f0, 0057aa90, 0057ab50 -----------------------------------

    /// A multi-bound task: the reference at +0x18 has a 3D, the target
    /// smart pointers at +0x1c and +0x20 hold objects.
    fn bound_task(e: &mut Engine) -> (u32, u32, u32) {
        let slot10 = 0x00ff_0150;
        returns(e, slot10, 0x8000);
        let target = object_with(e, 0x40, &[(0x10, slot10)]);
        let reference = refr(e, 0x30, &[(0x1d0, SPARE_SLOT)]);
        returns(e, SPARE_SLOT, 0x9000);
        e.mem.set_u32(reference + 0x40, 0x7000);
        let task = e.mem.alloc(0x40);
        e.mem.set_u32(task + 0x18, reference);
        e.mem.set_u32(task + 0x1c, target);
        e.mem.set_u32(task + 0x20, 0xaaaa);
        (task, target, reference)
    }

    #[test]
    fn multi_bound_task_run_stores_the_check_result() {
        let mut e = engine2();
        let (task, target, _) = bound_task(&mut e);
        returns(&mut e, 0x009a_d610, 1);
        returns(&mut e, 0x0056_9670, 1);
        start_log(&mut e);
        e.call(0x0057_a9f0, &args![task]);
        let log = take_log(&mut e);
        let check = calls_to(&log, 0x0056_9670);
        assert_eq!(check.len(), 1);
        assert_eq!(check[0][0], target);
        assert_eq!(check[0][1], 0xaaaa);
        assert_eq!(check[0][3], 1);
        assert_eq!(e.mem.u8(task + 0x24), 1);
        // the check says no
        returns(&mut e, 0x0056_9670, 0);
        e.call(0x0057_a9f0, &args![task]);
        assert_eq!(e.mem.u8(task + 0x24), 0);
        // no multi-bound: the byte is left alone
        e.mem.set_u8(task + 0x24, 7);
        returns(&mut e, 0x009a_d610, 0);
        e.call(0x0057_a9f0, &args![task]);
        assert_eq!(e.mem.u8(task + 0x24), 7);
        // no target at all
        e.mem.set_u32(task + 0x1c, 0);
        e.call(0x0057_a9f0, &args![task]);
        assert_eq!(e.mem.u8(task + 0x24), 7);
    }

    #[test]
    fn multi_bound_task_run_needs_a_3d() {
        let mut e = engine2();
        let (task, _, _) = bound_task(&mut e);
        returns(&mut e, 0x009a_d610, 1);
        returns(&mut e, 0x0056_9670, 1);
        returns(&mut e, SPARE_SLOT, 0);
        e.mem.set_u8(task + 0x24, 5);
        e.call(0x0057_a9f0, &args![task]);
        assert_eq!(e.mem.u8(task + 0x24), 5);
    }

    #[test]
    fn multi_bound_task_post_process_records_the_pair() {
        let mut e = engine2();
        let (task, _, reference) = bound_task(&mut e);
        returns(&mut e, 0x009a_d610, 0x8888);
        stub(&mut e, &[0x0084_4700]);
        returns(&mut e, 0x0085_3130, 1);
        // Run found nothing: no work
        start_log(&mut e);
        e.call(0x0057_aa90, &args![task]);
        assert!(calls_to(&take_log(&mut e), 0x0084_4700).is_empty());
        e.mem.set_u8(task + 0x24, 1);
        start_log(&mut e);
        e.call(0x0057_aa90, &args![task]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, 0x0084_4700),
            vec![vec![OBJECT_REFERENCE_MAP, 0x8888, reference]]
        );
        assert_eq!(
            calls_to(&log, 0x0085_3130)[0][..2],
            [OBJECT_REFERENCE_LOOKUP, 0x8888]
        );
        // the lookup fails
        returns(&mut e, 0x0085_3130, 0);
        start_log(&mut e);
        e.call(0x0057_aa90, &args![task]);
        assert!(calls_to(&take_log(&mut e), 0x0084_4700).is_empty());
        // no parent cell
        returns(&mut e, 0x0085_3130, 1);
        e.mem.set_u32(reference + 0x40, 0);
        start_log(&mut e);
        e.call(0x0057_aa90, &args![task]);
        assert!(calls_to(&take_log(&mut e), 0x0084_4700).is_empty());
    }

    #[test]
    fn fn_0057ab50_reports_whether_the_key_is_in_the_map() {
        let mut e = engine2();
        returns(&mut e, 0x0085_3130, 1);
        start_log(&mut e);
        assert!(e.call(0x0057_ab50, &args![0x55u32]).bool());
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, 0x0085_3130)[0][..2],
            [OBJECT_REFERENCE_LOOKUP, 0x55]
        );
        returns(&mut e, 0x0085_3130, 0);
        assert!(!e.call(0x0057_ab50, &args![0x55u32]).bool());
    }

    // ---- 0057ab70, 0057acc0 ---------------------------------------------

    #[test]
    fn fn_0057ab70_drains_the_map_and_tells_the_owners() {
        let mut e = engine2();
        let slot_e8 = 0x00ff_0160;
        let slot_dc = 0x00ff_0161;
        stub(&mut e, &[slot_e8, slot_dc]);
        let owner_object = object_with(&mut e, 0x40, &[(0xe8, slot_e8)]);
        let target = object_with(&mut e, 0x40, &[(0xdc, slot_dc)]);
        let reference = refr(&mut e, 0x30, &[(0x1d0, SPARE_SLOT)]);
        returns(&mut e, SPARE_SLOT, 0x9000);
        // the first position once; reading the entry there moves the cursor past the end
        returns(&mut e, 0x004b_9ba0, 1);
        e.register_double(0x006b_7f20, move |e, a| {
            // key = the reference, value = 0x4444
            e.mem.set_u32(a[1], 0);
            e.mem.set_u32(a[2], reference);
            e.mem.set_u32(a[3], 0x4444);
            Ret::default()
        });
        returns(&mut e, 0x0054_97a0, 1);
        returns(&mut e, 0x0096_11e0, owner_object);
        returns(&mut e, 0x0054_5960, target);
        stub(&mut e, &[0x0056_9ae0, 0x0043_8af0]);
        start_log(&mut e);
        e.call(0x0057_ab70, &args![]);
        let log = take_log(&mut e);
        let cell = e.mem.u32(reference + 0x40);
        assert_eq!(calls_to(&log, 0x0054_5960), vec![vec![cell, 0x4444]]);
        assert_eq!(calls_to(&log, slot_e8), vec![vec![owner_object, 0x9000]]);
        assert_eq!(calls_to(&log, slot_dc), vec![vec![target, 0x9000, 1]]);
        assert_eq!(calls_to(&log, 0x0056_9ae0), vec![vec![reference, 0x4444]]);
        assert_eq!(
            calls_to(&log, 0x0043_8af0),
            vec![vec![OBJECT_REFERENCE_MAP]]
        );
    }

    #[test]
    fn fn_0057ab70_skips_entries_of_the_same_cell_or_without_target() {
        let mut e = engine2();
        let reference = refr(&mut e, 0x30, &[(0x1d0, SPARE_SLOT)]);
        returns(&mut e, SPARE_SLOT, 0x9000);
        // the first position once (counted); three entries, the cursor moving 1 -> 2 -> 3 -> end
        let firsts = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let counted = firsts.clone();
        e.register_double(0x004b_9ba0, move |_, _| {
            counted.set(counted.get() + 1);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        // first entry has no value, the others have one
        let values = std::rc::Rc::new(std::cell::RefCell::new(vec![0u32, 0x4444, 0x5555]));
        e.register_double(0x006b_7f20, move |e, a| {
            let position = e.mem.u32(a[1]);
            e.mem
                .set_u32(a[1], if position < 3 { position + 1 } else { 0 });
            e.mem.set_u32(a[2], reference);
            e.mem.set_u32(a[3], values.borrow_mut().remove(0));
            Ret::default()
        });
        // same cell for 0x4444 (both lookups equal), no target for 0x5555
        let cells = std::rc::Rc::new(std::cell::RefCell::new(vec![7u32, 8]));
        e.register_double(0x0054_97a0, move |_, _| Ret {
            eax: cells.borrow_mut().remove(0),
            ..Ret::default()
        });
        let models = std::rc::Rc::new(std::cell::RefCell::new(vec![7u32, 9]));
        e.register_double(0x0096_11e0, move |_, _| Ret {
            eax: models.borrow_mut().remove(0),
            ..Ret::default()
        });
        returns(&mut e, 0x0054_5960, 0);
        stub(&mut e, &[0x0056_9ae0, 0x0043_8af0]);
        start_log(&mut e);
        e.call(0x0057_ab70, &args![]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0054_5960).len(), 1);
        assert!(calls_to(&log, 0x0056_9ae0).is_empty());
        assert_eq!(calls_to(&log, 0x0043_8af0).len(), 1);
        // 0057ab7c: the first position is asked for once; skipped entries never restart the walk.
        assert_eq!(firsts.get(), 1);
    }

    #[test]
    fn fn_0057acc0_removes_the_key_from_the_map() {
        let mut e = engine2();
        returns(&mut e, 0x0040_5430, 1);
        start_log(&mut e);
        assert_eq!(e.call(0x0057_acc0, &args![0x1234u32]).u32(), 1);
        assert_eq!(
            calls_to(&take_log(&mut e), 0x0040_5430),
            vec![vec![OBJECT_REFERENCE_MAP, 0x1234]]
        );
    }

    // ---- 0057ace0, 0057ad20, 0057ad60 -----------------------------------

    #[test]
    fn the_say_to_setters_store_and_mark_the_reference() {
        let mut e = engine2();
        let this = refr(&mut e, 0x30, &[]);
        stub(
            &mut e,
            &[
                EXTRA_SET_SAY_TO_INFO,
                EXTRA_SET_SAY_TO,
                EXTRA_SET_SAY_TO_OTHER,
            ],
        );
        for (entry, setter) in [
            (0x0057_ace0u32, EXTRA_SET_SAY_TO_INFO),
            (0x0057_ad20, EXTRA_SET_SAY_TO),
            (0x0057_ad60, EXTRA_SET_SAY_TO_OTHER),
        ] {
            start_log(&mut e);
            e.call(entry, &args![this, 0x3030u32]);
            let log = take_log(&mut e);
            assert_eq!(calls_to(&log, setter), vec![vec![this + 0x44, 0x3030]]);
            assert_eq!(calls_to(&log, MARK_CHANGED), vec![vec![this, 0x8000_0000]]);
        }
    }

    // ---- 00578a80 -------------------------------------------------------

    /// Everything `00578a80` reaches, for a reference without 3D whose
    /// topic has a response with a sound file name.
    struct SpeechWorld {
        this: u32,
        out: u32,
        info: u32,
        name_text: u32,
    }

    fn speech_world(e: &mut Engine, name: &[u8]) -> SpeechWorld {
        let this = refr(e, 0x2a, &[(0x1d0, SPARE_SLOT), (0x1f4, 0x00ff_0170)]);
        returns(e, SPARE_SLOT, 0);
        returns(e, 0x00ff_0170, this + 0x30);
        e.mem.set_u32(this + 0x0c, 0xf00d);
        for (i, v) in [7.0f32, 8.0, 9.0].iter().enumerate() {
            e.mem.set_f32(this + 0x30 + 4 * i as u32, *v);
        }
        let out = e.mem.alloc(12);
        let dialogue = e.mem.alloc(0x20);
        let info = e.mem.alloc(0x20);
        e.mem.set_u32(dialogue + 0x0c, info);
        returns(e, 0x0061_b320, dialogue);
        stub(
            e,
            &[
                EXTRA_SET_SAY_TO,
                EXTRA_SET_SAY_TO_INFO,
                0x0061_f170,
                0x0061_f150,
                0x0083_c7b0,
                0x0048_39c0,
                HANDLE_CONSTRUCT,
                HANDLE_DESTRUCT,
                HANDLE_ASSIGN,
                HANDLE_PLAY,
                HANDLE_SET_COMPLETION_CALLBACK,
                AUDIO_INSTANCE,
                0x0040_6d30,
                0x0070_5210,
            ],
        );
        let response = e.mem.alloc(0x20);
        returns(e, 0x0083_c820, response);
        // 00460140 / 006815c0: pointers to the name and the subtitle text
        let name_pointer = e.mem.alloc(8);
        let name_text = e.mem.alloc(0x40);
        e.mem.set_cstr(name_text, name);
        e.mem
            .set_u32(name_pointer, if name.is_empty() { 0 } else { name_text });
        returns(e, 0x0046_0140, name_pointer);
        let text_pointer = e.mem.alloc(8);
        e.mem.set_u32(text_pointer, 0x1c1c);
        returns(e, 0x0068_15c0, text_pointer);
        returns(e, AUDIO_HANDLE_BY_FILENAME, 0x7100);
        // the subtitles setting is off; the actor does not force them
        let setting = e.mem.alloc(4);
        returns(e, 0x0040_8d60, setting);
        returns(e, 0x008c_1bc0, 0);
        SpeechWorld {
            this,
            out,
            info,
            name_text,
        }
    }

    #[test]
    fn fn_00578a80_plays_the_line_and_hands_back_the_handle() {
        let mut e = engine2();
        let world = speech_world(&mut e, b"line.wav");
        let this = world.this;
        start_log(&mut e);
        let result = e
            .call(
                0x0057_8a80,
                &args![this, world.out, 0x44u32, 0x55u32, 0u8, 1u8, 0u8, 0u32, 1u8],
            )
            .u32();
        let log = take_log(&mut e);
        assert_eq!(result, world.out);
        // the dialogue item, the topic and its info are recorded
        assert_eq!(
            calls_to(&log, 0x0061_b320),
            vec![vec![0x44, this, 0x55, 0, 0, 0]]
        );
        assert_eq!(
            calls_to(&log, EXTRA_SET_SAY_TO),
            vec![vec![this + 0x44, 0x44]]
        );
        assert_eq!(
            calls_to(&log, EXTRA_SET_SAY_TO_INFO),
            vec![vec![this + 0x44, world.info]]
        );
        assert_eq!(calls_to(&log, 0x0061_f170), vec![vec![world.info, 0, this]]);
        // no 3D, variant set: flags 0x101; the file name is copied
        let by_name = calls_to(&log, AUDIO_HANDLE_BY_FILENAME);
        assert_eq!(by_name.len(), 1);
        assert_eq!(by_name[0][3], 0x101);
        let copy = calls_to(&log, 0x0040_6d30);
        assert_eq!(copy[0][1..], [0x200, world.name_text]);
        // played, with the completion callback and the speaker's id
        assert_eq!(calls_to(&log, HANDLE_PLAY).len(), 1);
        assert_eq!(
            calls_to(&log, HANDLE_SET_COMPLETION_CALLBACK)[0][1..],
            [0x0057_90f0, 0xf00d]
        );
        // the subtitle is off
        assert!(calls_to(&log, 0x0070_5210).is_empty());
        // the handle is copied to the caller's slot
        assert_eq!(calls_to(&log, HANDLE_ASSIGN).last().unwrap()[0], world.out);
    }

    #[test]
    fn fn_00578a80_flags_by_3d_and_variant() {
        let mut e = engine2();
        let world = speech_world(&mut e, b"line.wav");
        let this = world.this;
        let model = 0x6000;
        returns(&mut e, SPARE_SLOT, model);
        stub(
            &mut e,
            &[HANDLE_SET_POSITION, HANDLE_SET_MIN_MAX, HANDLE_SET_FOLLOW],
        );
        let position = e.mem.alloc(16);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(position + 4 * i as u32, *v);
        }
        returns(&mut e, 0x0045_bb80, position);
        let range = e.mem.alloc(8);
        e.mem.set_f32(range, 5.0);
        e.register(0x0040_3e20, |e, a| {
            // the minimum object points at a float of 5.0, the maximum at 50.0
            let target = e.mem.alloc(4);
            e.mem.set_f32(
                target,
                if a[0] == OBJECT_SOUND_RANGE_MIN {
                    5.0
                } else {
                    50.0
                },
            );
            target.into_ret()
        });
        for (ignore_3d, variant, flags) in [
            (0u8, 1u8, 0x102u32),
            (0, 0, 0x106),
            (1, 1, 0x101),
            (1, 0, 0x105),
        ] {
            start_log(&mut e);
            e.call(
                0x0057_8a80,
                &args![this, world.out, 1u32, 2u32, ignore_3d, variant, 0u8, 0u32, 0u8],
            );
            let log = take_log(&mut e);
            assert_eq!(
                calls_to(&log, AUDIO_HANDLE_BY_FILENAME)[0][3],
                flags,
                "ignore {ignore_3d} variant {variant}"
            );
            if ignore_3d == 0 {
                let handle = calls_to(&log, HANDLE_PLAY)[0][0];
                assert_eq!(
                    calls_to(&log, HANDLE_SET_POSITION),
                    vec![vec![
                        handle,
                        1.0f32.to_bits(),
                        2.0f32.to_bits(),
                        3.0f32.to_bits()
                    ]]
                );
                assert_eq!(
                    calls_to(&log, HANDLE_SET_MIN_MAX),
                    vec![vec![handle, 5.0f32.to_bits(), 50.0f32.to_bits()]]
                );
                assert_eq!(calls_to(&log, HANDLE_SET_FOLLOW), vec![vec![handle, model]]);
            } else {
                assert!(calls_to(&log, HANDLE_SET_POSITION).is_empty());
            }
        }
    }

    #[test]
    fn fn_00578a80_reads_the_lip_sync_length_and_shows_the_subtitle() {
        let mut e = engine2();
        let world = speech_world(&mut e, b"line.wav");
        let this = world.this;
        // the lip data object is deleted after its length is read
        stub(
            &mut e,
            &[0x0040_37b0, 0x0040_37f0, 0x0040_37d0, 0x004d_5850],
        );
        returns(&mut e, 0x004d_5ad0, 1);
        let lip = e.mem.alloc(8);
        e.mem.set_u32(lip, 90);
        returns(&mut e, 0x004d_52d0, lip);
        e.map(0x0101_d000, 0x1000);
        e.mem.set_f64(FRAMES_PER_SECOND, 30.0);
        // the subtitle setting is on
        let setting = e.mem.alloc(4);
        e.mem.set_u8(setting, 1);
        returns(&mut e, 0x0040_8d60, setting);
        start_log(&mut e);
        e.call(
            0x0057_8a80,
            &args![this, world.out, 1u32, 0x55u32, 0u8, 1u8, 1u8, 0u32, 1u8],
        );
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x004d_5850), vec![vec![lip, 1]]);
        assert_eq!(calls_to(&log, 0x0040_37d0).len(), 1);
        assert_eq!(
            calls_to(&log, 0x0070_5210),
            vec![vec![
                0x1c1c,
                0,
                0,
                0,
                7.0f32.to_bits(),
                8.0f32.to_bits(),
                9.0f32.to_bits(),
                0x55,
                1
            ]]
        );
        // no lip data: nothing to delete
        returns(&mut e, 0x004d_5ad0, 0);
        start_log(&mut e);
        e.call(
            0x0057_8a80,
            &args![this, world.out, 1u32, 0x55u32, 0u8, 1u8, 1u8, 0u32, 1u8],
        );
        assert!(calls_to(&take_log(&mut e), 0x004d_5850).is_empty());
        // the subtitle also shows for an actor that always shows them
        e.mem.set_u8(setting, 0);
        let actor = refr(
            &mut e,
            0x2a,
            &[
                (0x100, RETURNS_TRUE),
                (0x1d0, SPARE_SLOT),
                (0x1f4, 0x00ff_0170),
            ],
        );
        returns(&mut e, 0x008c_1bc0, 1);
        start_log(&mut e);
        e.call(
            0x0057_8a80,
            &args![actor, world.out, 1u32, 0x55u32, 0u8, 1u8, 0u8, 0u32, 0u8],
        );
        assert_eq!(calls_to(&take_log(&mut e), 0x0070_5210).len(), 1);
    }

    #[test]
    fn fn_00578a80_without_a_sound_file_calls_the_completion_callback_directly() {
        let mut e = engine2();
        let world = speech_world(&mut e, b"");
        let this = world.this;
        start_log(&mut e);
        e.call(
            0x0057_8a80,
            &args![this, world.out, 1u32, 0x55u32, 0u8, 1u8, 0u8, 0u32, 0u8],
        );
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0048_39c0), vec![vec![0xf00d]]);
        assert!(calls_to(&log, AUDIO_HANDLE_BY_FILENAME).is_empty());
        assert!(calls_to(&log, HANDLE_PLAY).is_empty());
    }

    #[test]
    fn fn_00578a80_without_a_dialogue_item_only_returns_the_handle() {
        let mut e = engine2();
        let world = speech_world(&mut e, b"line.wav");
        returns(&mut e, 0x0061_b320, 0);
        start_log(&mut e);
        let result = e
            .call(
                0x0057_8a80,
                &args![world.this, world.out, 1u32, 2u32, 0u8, 0u8, 0u8, 0u32, 0u8],
            )
            .u32();
        let log = take_log(&mut e);
        assert_eq!(result, world.out);
        assert!(calls_to(&log, EXTRA_SET_SAY_TO_INFO).is_empty());
        assert!(calls_to(&log, 0x0083_c7b0).is_empty());
        assert_eq!(calls_to(&log, EXTRA_SET_SAY_TO).len(), 1);
    }

    // ---- 0057ada0 -------------------------------------------------------

    /// Doubles for both branches of `0057ada0`; `primitive` is what the
    /// extra data list gives.
    fn primitive_world(e: &mut Engine) -> (u32, u32) {
        e.map(0x5000, 0x1000);
        let primitive = object_with(e, 0x40, &[(0x14, SPARE_SLOT)]);
        returns(e, SPARE_SLOT, 0x3333);
        returns(e, EXTRA_GET_PRIMITIVE, primitive);
        let vector = e.mem.alloc(16);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(vector + 4 * i as u32, *v);
        }
        returns(e, 0x0041_3fc0, vector);
        returns(e, 0x0056_c7f0, 0x5100);
        returns(e, 0x0045_bb80, 0x5200);
        returns(e, 0x006a_9540, 0x5300);
        stub(e, &[0x0043_d410, 0x0045_0b80, 0x00a5_9c60]);
        e.register(0x0045_2dc0, |_, a| a[0].into_ret());
        stub(e, &[0x0041_6a30, 0x0041_69f0, 0x0041_6a70]);
        e.set_global(GLOBAL_BASE_FORM_234, 0x1001u32);
        e.set_global(GLOBAL_BASE_FORM_23C, 0x1002u32);
        e.set_global(GLOBAL_BASE_FORM_238, 0x1003u32);
        e.set_global(GLOBAL_BASE_FORM_230, 0x1004u32);
        (primitive, vector)
    }

    fn refr_with_base(e: &mut Engine, base: u32) -> u32 {
        let this = refr(e, 0x30, &[]);
        e.mem.set_u32(this + 0x20, base);
        this
    }

    #[test]
    fn fn_0057ada0_first_branch_copies_the_box_to_the_target() {
        let mut e = engine2();
        primitive_world(&mut e);
        returns(&mut e, 0x0042_2120, 0x4000);
        returns(&mut e, 0x0042_0dd0, 0x4100);
        for (base, target) in [(0x1001u32, 0x4000u32), (0x1002, 0x4100)] {
            let this = refr_with_base(&mut e, base);
            start_log(&mut e);
            e.call(0x0057_ada0, &args![this]);
            let log = take_log(&mut e);
            // 00452dc0 gets the x and z of the box (1.0 and 3.0); the
            // target receives its result, position and rotation
            let made = calls_to(&log, 0x0045_2dc0);
            assert_eq!(made[0][1..], [1.0f32.to_bits(), 3.0f32.to_bits()]);
            assert_eq!(calls_to(&log, 0x0041_6a30)[0][0], target);
            assert_eq!(calls_to(&log, 0x0041_69f0), vec![vec![target, 0x5200]]);
            assert_eq!(calls_to(&log, 0x0041_6a70), vec![vec![target, 0x5300]]);
            // the final zero vector goes to 00a59c60
            assert_eq!(calls_to(&log, 0x0043_d410)[0][1..], [0, 0, 0]);
            assert_eq!(calls_to(&log, 0x00a5_9c60).len(), 1);
        }
        // no target: nothing is set, the tail still runs
        returns(&mut e, 0x0042_2120, 0);
        let this = refr_with_base(&mut e, 0x1001);
        start_log(&mut e);
        e.call(0x0057_ada0, &args![this]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0041_6a30).is_empty());
        assert_eq!(calls_to(&log, 0x00a5_9c60).len(), 1);
        // no primitive: nothing at all
        returns(&mut e, EXTRA_GET_PRIMITIVE, 0);
        start_log(&mut e);
        e.call(0x0057_ada0, &args![this]);
        assert!(calls_to(&take_log(&mut e), 0x00a5_9c60).is_empty());
        // another base form: only the tail
        returns(&mut e, EXTRA_GET_PRIMITIVE, 0x5555);
        let other = refr_with_base(&mut e, 0x1fff);
        start_log(&mut e);
        e.call(0x0057_ada0, &args![other]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x0041_6a30).is_empty());
        assert_eq!(calls_to(&log, 0x00a5_9c60).len(), 1);
    }

    #[test]
    fn fn_0057ada0_second_branch_moves_the_shape_node() {
        let mut e = engine2();
        let (primitive, _) = primitive_world(&mut e);
        let slot8c = 0x00ff_0180;
        let slotb8 = 0x00ff_0181;
        stub(&mut e, &[slotb8]);
        let plain = std::rc::Rc::new(std::cell::Cell::new(1u32));
        let plain_in = plain.clone();
        e.register_double(slot8c, move |_, _| Ret {
            eax: plain_in.get(),
            ..Ret::default()
        });
        let node = object_with(&mut e, 0x40, &[(0x8c, slot8c), (0xb8, slotb8)]);
        returns(&mut e, 0x0042_0ed0, 0x4200);
        returns(&mut e, 0x0066_29f0, 0x4300);
        returns(&mut e, 0x0042_2020, 0x4400);
        returns(&mut e, 0x0043_b230, node);
        stub(&mut e, &[0x0043_9680, 0x0043_96b0, 0x004a_ddc0]);
        let reference_position = e.mem.alloc(16);
        returns(&mut e, 0x0043_0830, reference_position);
        let zero_matches = std::rc::Rc::new(std::cell::Cell::new(1u32));
        let zero_in = zero_matches.clone();
        e.register_double(0x0043_90c0, move |_, _| Ret {
            eax: zero_in.get(),
            ..Ret::default()
        });
        let this = refr_with_base(&mut e, 0x1003);
        // zero position, plain: size and position go to the node
        start_log(&mut e);
        e.call(0x0057_ada0, &args![this]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0042_0ed0), vec![vec![this + 0x44]]);
        assert_eq!(calls_to(&log, 0x0066_29f0), vec![vec![0x4200]]);
        assert_eq!(calls_to(&log, 0x0043_b230), vec![vec![0x4300]]);
        assert_eq!(calls_to(&log, 0x0043_9680).len(), 1);
        assert_eq!(calls_to(&log, slotb8).len(), 1);
        assert!(calls_to(&log, 0x004a_ddc0).is_empty());
        // zero position, not plain: the primitive's virtual +0x14 and 004addc0
        plain.set(0);
        start_log(&mut e);
        e.call(0x0057_ada0, &args![this]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x004a_ddc0), vec![vec![0x4300, 0x3333]]);
        assert_eq!(calls_to(&log, SPARE_SLOT)[0][0], primitive);
        // non-zero position, plain: virtual +0x14 and 004addc0 too
        zero_matches.set(0);
        plain.set(1);
        start_log(&mut e);
        e.call(0x0057_ada0, &args![this]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x004a_ddc0), vec![vec![0x4300, 0x3333]]);
        // non-zero position, not plain: node, position and rotation
        plain.set(0);
        start_log(&mut e);
        e.call(0x0057_ada0, &args![this]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0043_96b0), vec![vec![node, 0x5300]]);
        assert_eq!(calls_to(&log, slotb8).len(), 1);
        // the other base form uses 00422020 directly
        let other = refr_with_base(&mut e, 0x1004);
        start_log(&mut e);
        e.call(0x0057_ada0, &args![other]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0043_b230), vec![vec![0x4400]]);
        assert!(calls_to(&log, 0x0042_0ed0).is_empty());
        // no extra at all for the first one: return before the tail
        returns(&mut e, 0x0042_0ed0, 0);
        start_log(&mut e);
        e.call(0x0057_ada0, &args![this]);
        assert!(calls_to(&take_log(&mut e), 0x00a5_9c60).is_empty());
    }

    // ---- 0057b0a0, 0057b0d0, 0057b1d0, 0057b200 ------------------------

    fn loaded_data(e: &mut Engine, this: u32) -> u32 {
        let loaded = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x64, loaded);
        loaded
    }

    #[test]
    fn get_relevant_water_height_reads_the_loaded_data_or_the_default() {
        let mut e = engine2();
        let this = refr(&mut e, 0x30, &[]);
        e.mem.set_f32(LOADED_DATA_DEFAULT_HEIGHT, -2048.0);
        assert_eq!(e.call(0x0057_b0a0, &args![this]).f32(), -2048.0);
        let loaded = loaded_data(&mut e, this);
        e.mem.set_f32(loaded + 8, 12.5);
        assert_eq!(e.call(0x0057_b0a0, &args![this]).f32(), 12.5);
    }

    #[test]
    fn fn_0057b0d0_takes_the_height_from_the_water_object() {
        let mut e = engine2();
        e.mem.set_f32(LOADED_DATA_DEFAULT_HEIGHT, -2048.0);
        let water = refr(&mut e, 0x30, &[(0x1f4, SPARE_SLOT)]);
        let water_position = e.mem.alloc(16);
        e.mem.set_f32(water_position + 8, 77.0);
        returns(&mut e, SPARE_SLOT, water_position);
        let this = refr(&mut e, 0x30, &[]);
        let loaded = loaded_data(&mut e, this);
        returns(&mut e, 0x0042_f030, water);
        start_log(&mut e);
        e.call(0x0057_b0d0, &args![this, 0x99u32, 1u8]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, 0x0042_f030),
            vec![vec![this + 0x44, 0x99, 1]]
        );
        assert_eq!(e.mem.u32(loaded), water);
        assert_eq!(e.mem.f32(loaded + 8), 77.0);
    }

    #[test]
    fn fn_0057b0d0_without_a_water_object_uses_the_cell_or_the_default() {
        let mut e = engine2();
        e.mem.set_f32(LOADED_DATA_DEFAULT_HEIGHT, -2048.0);
        let this = refr(&mut e, 0x30, &[]);
        let cell = 0x7000;
        e.mem.set_u32(this + 0x40, cell);
        let loaded = loaded_data(&mut e, this);
        returns(&mut e, 0x0042_f030, 0);
        returns(&mut e, 0x0045_18e0, 1);
        returns_float(&mut e, 0x0054_71e0, 33.0);
        e.call(0x0057_b0d0, &args![this, 0u32, 0u8]);
        assert_eq!(e.mem.u32(loaded), 0);
        assert_eq!(e.mem.f32(loaded + 8), 33.0);
        // the cell has no water
        returns(&mut e, 0x0045_18e0, 0);
        e.call(0x0057_b0d0, &args![this, 0u32, 0u8]);
        assert_eq!(e.mem.f32(loaded + 8), -2048.0);
        // no cell
        e.mem.set_u32(this + 0x40, 0);
        e.mem.set_f32(loaded + 8, 1.0);
        e.call(0x0057_b0d0, &args![this, 0u32, 0u8]);
        assert_eq!(e.mem.f32(loaded + 8), -2048.0);
    }

    #[test]
    fn fn_0057b0d0_tells_an_actors_process_the_new_height() {
        let mut e = engine2();
        e.mem.set_f32(LOADED_DATA_DEFAULT_HEIGHT, -2048.0);
        let this = refr(&mut e, 0x2a, &[(0x100, RETURNS_TRUE)]);
        let loaded = loaded_data(&mut e, this);
        e.mem.set_f32(loaded + 8, 4.0);
        e.mem.set_u32(this + 0x40, 0);
        returns(&mut e, 0x0042_f030, 0);
        let process = e.mem.alloc(0x600);
        returns(&mut e, 0x0093_06d0, process);
        returns_float(&mut e, 0x004a_3e90, 8.0);
        start_log(&mut e);
        e.call(0x0057_b0d0, &args![this, 0u32, 0u8]);
        let log = take_log(&mut e);
        // the height is the default (no water, no cell)
        assert_eq!(
            calls_to(&log, 0x004a_3e90),
            vec![vec![(-2048.0f32).to_bits()]]
        );
        assert_eq!(e.mem.f32(process + 0x53c), 8.0);
        // no process: nothing
        returns(&mut e, 0x0093_06d0, 0);
        e.mem.set_f32(process + 0x53c, 1.0);
        e.call(0x0057_b0d0, &args![this, 0u32, 0u8]);
        assert_eq!(e.mem.f32(process + 0x53c), 1.0);
        // no loaded data: the process still gets the default height
        e.mem.set_u32(this + 0x64, 0);
        returns(&mut e, 0x0093_06d0, process);
        e.call(0x0057_b0d0, &args![this, 0u32, 0u8]);
        assert_eq!(e.mem.f32(process + 0x53c), 8.0);
    }

    #[test]
    fn fn_0057b1d0_stores_the_converted_height() {
        let mut e = engine2();
        returns_float(&mut e, 0x004a_3e90, 0.5);
        let object = e.mem.alloc(0x600);
        start_log(&mut e);
        e.call(0x0057_b1d0, &args![object, 10.0f32]);
        assert_eq!(
            calls_to(&take_log(&mut e), 0x004a_3e90),
            vec![vec![10.0f32.to_bits()]]
        );
        assert_eq!(e.mem.f32(object + 0x53c), 0.5);
    }

    #[test]
    fn fn_0057b200_is_true_for_a_positive_underwater_count() {
        let mut e = engine2();
        let this = refr(&mut e, 0x30, &[]);
        assert!(!e.call(0x0057_b200, &args![this]).bool());
        let loaded = loaded_data(&mut e, this);
        assert!(!e.call(0x0057_b200, &args![this]).bool());
        e.mem.set_i32(loaded + 4, 1);
        assert!(e.call(0x0057_b200, &args![this]).bool());
        e.mem.set_i32(loaded + 4, -1);
        assert!(!e.call(0x0057_b200, &args![this]).bool());
    }

    // ---- 0057b240 -------------------------------------------------------

    #[test]
    fn fn_0057b240_counts_and_clamps() {
        let mut e = engine2();
        stub(&mut e, &[0x0048_4a10]);
        returns(&mut e, 0x0054_5710, 0);
        returns(&mut e, 0x0041_0220, 0);
        let this = refr(&mut e, 0x30, &[]);
        // without loaded data nothing happens
        e.call(0x0057_b240, &args![this, 1u8]);
        let loaded = loaded_data(&mut e, this);
        e.mem.set_u32(loaded, 0x1111);
        e.mem.set_i32(loaded + 4, 5);
        e.call(0x0057_b240, &args![this, 1u8]);
        assert_eq!(e.mem.i32(loaded + 4), 6);
        assert_eq!(e.mem.u32(loaded), 0);
        e.call(0x0057_b240, &args![this, 0u8]);
        assert_eq!(e.mem.i32(loaded + 4), 5);
        // leaving at zero clamps to zero
        e.mem.set_i32(loaded + 4, 0);
        e.call(0x0057_b240, &args![this, 0u8]);
        assert_eq!(e.mem.i32(loaded + 4), 0);
    }

    #[test]
    fn fn_0057b240_back_on_land_takes_the_water_of_the_extra() {
        let mut e = engine2();
        stub(&mut e, &[0x0048_4a10]);
        let this = refr(&mut e, 0x30, &[]);
        let loaded = loaded_data(&mut e, this);
        e.mem.set_i32(loaded + 4, 1);
        let inner = e.mem.alloc(0x20);
        let extra = e.mem.alloc(0x40);
        e.mem.set_u32(extra + 0x1c, inner);
        e.mem.set_u32(inner + 4 + 0x0c, 0x8123);
        returns(&mut e, 0x0041_0220, extra);
        start_log(&mut e);
        e.call(0x0057_b240, &args![this, 0u8]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0041_0220), vec![vec![this + 0x44, 0x7e]]);
        assert_eq!(e.mem.u32(loaded), 0x8123);
        assert_eq!(calls_to(&log, 0x0048_4a10), vec![vec![this, 0]]);
        // a door or an actor does not get the call
        let door = refr(&mut e, 0x1c, &[]);
        let loaded = loaded_data(&mut e, door);
        e.mem.set_i32(loaded + 4, 1);
        start_log(&mut e);
        e.call(0x0057_b240, &args![door, 0u8]);
        assert!(calls_to(&take_log(&mut e), 0x0048_4a10).is_empty());
        // no extra: no water
        returns(&mut e, 0x0041_0220, 0);
        e.mem.set_i32(loaded + 4, 1);
        e.mem.set_u32(loaded, 0x55);
        e.call(0x0057_b240, &args![door, 0u8]);
        assert_eq!(e.mem.u32(loaded), 0);
    }

    #[test]
    fn fn_0057b240_entering_picks_the_first_water_of_the_cell() {
        let mut e = engine2();
        stub(&mut e, &[0x0048_4a10]);
        let this = refr(&mut e, 0x30, &[]);
        let cell = 0x7000;
        e.mem.set_u32(this + 0x40, cell);
        let loaded = loaded_data(&mut e, this);
        e.mem.set_i32(loaded + 4, 0);
        // a list of two items: the first is not water, the second is
        let first = refr(&mut e, 0x30, &[]);
        let second = refr(&mut e, 0x30, &[]);
        let slot_first = e.mem.alloc(8);
        e.mem.set_u32(slot_first, first);
        let slot_second = e.mem.alloc(8);
        e.mem.set_u32(slot_second, second);
        returns(&mut e, 0x0054_5710, 0xa1);
        e.register_double(LIST_ITEM_SLOT, move |_, a| Ret {
            eax: if a[0] == 0xa1 {
                slot_first
            } else {
                slot_second
            },
            ..Ret::default()
        });
        e.register(LIST_NEXT, |_, a| {
            (if a[0] == 0xa1 { 0xa2 } else { 0 }).into_ret()
        });
        e.register_double(DYNAMIC_CAST, move |e, a| {
            // the base form of the second item casts, the first does not
            let base_of_second = e.mem.u32(second + 0x20);
            assert_eq!(
                a[2..],
                [TYPE_TES_BOUND_OBJECT, TYPE_BOUND_OBJECT_TARGET_B, 0]
            );
            Ret {
                eax: u32::from(a[0] == base_of_second),
                ..Ret::default()
            }
        });
        e.register(0x0045_2440, |_, a| (a[0] != 0).into_ret());
        start_log(&mut e);
        e.call(0x0057_b240, &args![this, 1u8]);
        let log = take_log(&mut e);
        assert_eq!(e.mem.i32(loaded + 4), 1);
        assert_eq!(e.mem.u32(loaded), second);
        assert_eq!(calls_to(&log, 0x0048_4a10), vec![vec![this, 1]]);
        // an empty slot ends the walk without water
        e.mem.set_u32(slot_first, 0);
        e.mem.set_i32(loaded + 4, 0);
        e.call(0x0057_b240, &args![this, 1u8]);
        assert_eq!(e.mem.u32(loaded), 0);
        // no cell: no walk
        e.mem.set_u32(this + 0x40, 0);
        e.mem.set_i32(loaded + 4, 0);
        e.call(0x0057_b240, &args![this, 1u8]);
        assert_eq!(e.mem.u32(loaded), 0);
        // a count above one does nothing more
        e.mem.set_i32(loaded + 4, 4);
        start_log(&mut e);
        e.call(0x0057_b240, &args![this, 1u8]);
        assert!(calls_to(&take_log(&mut e), 0x0048_4a10).is_empty());
    }

    // ---- 0057b410 -------------------------------------------------------

    #[test]
    fn fn_0057b410_asks_the_base_forms_virtual_b8() {
        let mut e = engine2();
        let slot = 0x00ff_0190;
        returns(&mut e, slot, 1);
        let base = object_with(&mut e, 0x40, &[(0xb8, slot)]);
        let this = object_with(&mut e, 0x100, &[]);
        e.mem.set_u32(this + 0x20, base);
        assert!(e.call(0x0057_b410, &args![this]).bool());
        returns(&mut e, slot, 0);
        assert!(!e.call(0x0057_b410, &args![this]).bool());
        e.mem.set_u32(this + 0x20, 0);
        assert!(!e.call(0x0057_b410, &args![this]).bool());
    }

    // ======================================================================
    // Third batch: 0057b460 to 0057ca90
    // ======================================================================

    type Seen = std::rc::Rc<std::cell::RefCell<Vec<(u32, u32, [u32; 3])>>>;

    /// Adds the registrations the third batch's tests share.
    fn engine3() -> Engine {
        let mut e = engine2();
        e.map(0x0101_f000, 0x1000);
        e.register(0x0044_ddc0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(LIST_NEXT, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e
    }

    /// Allocates `size` bytes the way the game's allocator does.
    fn allocator(e: &mut Engine, addr: u32) {
        e.register_double(addr, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
    }

    // ---- 0057b460 / 0057b4b0 --------------------------------------------

    #[test]
    fn fn_0057b460_needs_form_type_1c_and_flag_100() {
        let mut e = engine3();
        let this = refr(&mut e, 0x1c, &[]);
        assert!(!e.call(0x0057_b460, &args![this]).bool());
        e.mem.set_u32(this + 8, 0x100);
        assert!(e.call(0x0057_b460, &args![this]).bool());
        let other = refr(&mut e, 0x1d, &[]);
        e.mem.set_u32(other + 8, 0x100);
        assert!(!e.call(0x0057_b460, &args![other]).bool());
        e.mem.set_u32(this + 0x20, 0);
        assert!(!e.call(0x0057_b460, &args![this]).bool());
    }

    #[test]
    fn fn_0057b4b0_needs_form_type_1c_and_flag_40() {
        let mut e = engine3();
        let this = refr(&mut e, 0x1c, &[]);
        e.mem.set_u32(this + 8, 0x100);
        assert!(!e.call(0x0057_b4b0, &args![this]).bool());
        e.mem.set_u32(this + 8, 0x40);
        assert!(e.call(0x0057_b4b0, &args![this]).bool());
        let other = refr(&mut e, 0x1b, &[]);
        e.mem.set_u32(other + 8, 0x40);
        assert!(!e.call(0x0057_b4b0, &args![other]).bool());
        e.mem.set_u32(this + 0x20, 0);
        assert!(!e.call(0x0057_b4b0, &args![this]).bool());
    }

    // ---- 0057b500 -------------------------------------------------------

    #[test]
    fn fn_0057b500_runs_0042f2d0_on_the_extra_list() {
        let mut e = engine3();
        stub(&mut e, &[0x0042_f2d0]);
        let this = e.mem.alloc(0x100);
        start_log(&mut e);
        e.call(0x0057_b500, &args![this]);
        assert_eq!(
            calls_to(&take_log(&mut e), 0x0042_f2d0),
            vec![vec![this + 0x44]]
        );
    }

    // ---- 0057b520 -------------------------------------------------------

    #[test]
    fn remove_from_all_water_walks_the_zone_map_without_the_flag() {
        let mut e = engine3();
        let water = 0x7777_0000;
        let tes = e.mem.alloc(8);
        e.set_global(GLOBAL_TES, tes);
        returns(&mut e, GET_WATER_SYSTEM, water);
        returns(&mut e, EXTRA_GET_DATA, 0xe0e0);
        returns(&mut e, 0x0042_f1d0, 0x5000);
        returns(&mut e, 0x004b_9ba0, 1);
        stub(
            &mut e,
            &[
                0x0061_ff30,
                WATER_REMOVE_FROM_GROUP,
                0x004e_5fe0,
                EXTRA_REMOVE,
            ],
        );
        let this = refr(&mut e, 0x30, &[]);
        let loaded = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x64, loaded);
        e.mem.set_u32(loaded, 0x1234);
        e.mem.set_i32(loaded + 4, 3);
        let mut zones = vec![];
        let mut holders = vec![];
        for i in 0..2u32 {
            let zone = e.mem.alloc(0x40);
            let holder = e.mem.alloc(0x40);
            e.mem.set_u32(zone + 0x10, 0x100 + i);
            e.mem.set_u32(zone + 0x14, holder);
            e.mem.set_f32(zone + 0x18, 3.5 + i as f32);
            // the list node 0x10 into the holder carries the form ID `this`
            e.mem.set_u32(holder + 0x10 + 0x0c, this);
            zones.push(zone);
            holders.push(holder);
        }
        let order = zones.clone();
        let mut next = 0usize;
        e.register_double(0x006b_7f20, move |e, a| {
            // a: map, iterator slot, key slot, value slot
            e.mem.set_u32(a[2], order[next]);
            next += 1;
            e.mem.set_u32(a[1], if next < 2 { 2 } else { 0 });
            Ret::default()
        });
        start_log(&mut e);
        e.call(0x0057_b520, &args![this, 0u8]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, WATER_REMOVE_FROM_GROUP),
            vec![
                vec![water, 0x100, this, 3.5f32.to_bits()],
                vec![water, 0x101, this, 4.5f32.to_bits()],
            ]
        );
        assert_eq!(
            calls_to(&log, 0x0061_ff30),
            vec![vec![zones[0], holders[0]], vec![zones[1], holders[1]]]
        );
        assert_eq!(
            calls_to(&log, EXTRA_REMOVE),
            vec![vec![this + 0x44, 0xe0e0, 1]]
        );
        assert_eq!(calls_to(&log, 0x004e_5fe0), vec![vec![water, this]]);
        assert_eq!(e.mem.u32(loaded), 0);
        assert_eq!(e.mem.i32(loaded + 4), 0);
    }

    #[test]
    fn remove_from_all_water_with_the_flag_detaches_the_cell_target() {
        let mut e = engine3();
        let tes = e.mem.alloc(8);
        e.set_global(GLOBAL_TES, tes);
        e.set_global::<f32>(LOADED_DATA_DEFAULT_HEIGHT, 2.0);
        returns(&mut e, GET_WATER_SYSTEM, 0);
        returns(&mut e, EXTRA_GET_DATA, 0);
        returns(&mut e, 0x0045_43c0, 0x4300);
        returns(&mut e, 0x0059_bb30, 0x5900);
        returns(&mut e, 0x0045_18e0, 1);
        returns_float(&mut e, 0x004a_3e90, 0.5);
        let controller = e.mem.alloc(0x600);
        returns(&mut e, 0x0093_06d0, controller);
        stub(&mut e, &[0x0062_0130, 0x0063_1370, WATER_REMOVE_FROM_GROUP]);
        let this = refr(&mut e, 0x30, &[(0x100, RETURNS_TRUE)]);
        e.mem.set_u32(this + 0x40, 0xce11);
        start_log(&mut e);
        e.call(0x0057_b520, &args![this, 1u8]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0045_43c0), vec![vec![0xce11]]);
        assert_eq!(calls_to(&log, 0x0062_0130), vec![vec![0x5900 + 0x14, this]]);
        assert_eq!(calls_to(&log, 0x0063_1370), vec![vec![0x5900, this, 0, 1]]);
        // no extra, no water system: no group removal through the zone map,
        // but the cell has water, so the reference leaves the default group
        assert_eq!(
            calls_to(&log, WATER_REMOVE_FROM_GROUP),
            vec![vec![0, 0, this, 2.0f32.to_bits()]]
        );
        assert!(calls_to(&log, EXTRA_REMOVE).is_empty());
        // an actor: the character controller gets the converted height
        assert_eq!(e.mem.f32(controller + 0x53c), 0.5);
    }

    // ---- 0057b700 / 0057b730 / 0057b750 ----------------------------------

    #[test]
    fn fn_0057b750_returns_the_found_node_minus_0x10() {
        let mut e = engine3();
        let this = e.mem.alloc(0x40);
        // the first node (this + 0x10) has form ID 7, the next one 9
        let second = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x10 + 0x0c, 7);
        e.mem.set_u32(this + 0x10 + 0x10, second);
        e.mem.set_u32(second + 0x0c, 9);
        assert_eq!(e.call(0x0057_b750, &args![this, 7u32]).u32(), this);
        assert_eq!(e.call(0x0057_b750, &args![this, 9u32]).u32(), second - 0x10);
        assert_eq!(e.call(0x0057_b750, &args![this, 5u32]).u32(), 0);
    }

    #[test]
    fn fn_0057b730_looks_in_the_object_at_0x14() {
        let mut e = engine3();
        let this = e.mem.alloc(0x40);
        let held = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x14, held);
        e.mem.set_u32(held + 0x10 + 0x0c, 7);
        assert_eq!(e.call(0x0057_b730, &args![this, 7u32]).u32(), held);
        assert_eq!(e.call(0x0057_b730, &args![this, 8u32]).u32(), 0);
    }

    #[test]
    fn fn_0057b700_hands_the_lookup_result_to_0061ff30() {
        let mut e = engine3();
        stub(&mut e, &[0x0061_ff30]);
        let this = e.mem.alloc(0x40);
        let held = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x14, held);
        e.mem.set_u32(held + 0x10 + 0x0c, 7);
        start_log(&mut e);
        e.call(0x0057_b700, &args![this, 7u32]);
        assert_eq!(
            calls_to(&take_log(&mut e), 0x0061_ff30),
            vec![vec![this, held]]
        );
        start_log(&mut e);
        e.call(0x0057_b700, &args![this, 8u32]);
        assert_eq!(
            calls_to(&take_log(&mut e), 0x0061_ff30),
            vec![vec![this, 0]]
        );
    }

    // ---- 0057b790 -------------------------------------------------------

    #[test]
    fn fn_0057b790_asks_the_tes_about_the_parent_cell() {
        let mut e = engine3();
        e.set_global(GLOBAL_TES, 0x5555u32);
        returns(&mut e, 0x0045_11e0, 1);
        let this = e.mem.alloc(0x100);
        assert!(!e.call(0x0057_b790, &args![this]).bool());
        e.mem.set_u32(this + 0x40, 0xce11);
        start_log(&mut e);
        assert!(e.call(0x0057_b790, &args![this]).bool());
        assert_eq!(
            calls_to(&take_log(&mut e), 0x0045_11e0),
            vec![vec![0x5555, 0xce11, 0]]
        );
        returns(&mut e, 0x0045_11e0, 0);
        assert!(!e.call(0x0057_b790, &args![this]).bool());
    }

    // ---- 0057b7c0 -------------------------------------------------------

    const SAY_SLOT_D0: u32 = 0x00ff_0201;
    const SAY_SLOT_START: u32 = 0x00ff_0203;
    const SAY_SLOT_RESPONSE: u32 = 0x00ff_0204;
    const SAY_SLOT_AFTER: u32 = 0x00ff_0205;
    const SAY_SLOT_POSITION: u32 = 0x00ff_0206;
    const SAY_SLOT_LINE: u32 = 0x00ff_0207;
    const SAY_SLOT_MODEL: u32 = 0x00ff_0208;
    const SAY_SLOT_INTERRUPT: u32 = 0x00ff_0209;
    const SAY_SLOT_PREDICATE: u32 = 0x00ff_020a;
    const SAY_SLOT_OWNER_MODEL: u32 = 0x00ff_020b;

    fn say_engine() -> Engine {
        let mut e = engine3();
        e.map(0x011c_d000, 0x1000);
        let lock = e.mem.alloc(8);
        e.mem.set_u32(lock, 0x1234);
        returns(&mut e, 0x0043_d4d0, lock);
        stub(
            &mut e,
            &[
                0x0045_ce80,
                0x0096_e570,
                SAY_SLOT_D0,
                0x0088_1680,
                0x005a_c750,
                0x0083_c7b0,
                0x0083_c850,
                0x0070_5210,
                SAY_SLOT_START,
                SAY_SLOT_AFTER,
                SAY_SLOT_LINE,
                SAY_SLOT_INTERRUPT,
                HANDLE_SET_COMPLETION_CALLBACK,
                HANDLE_DESTRUCT,
                EXTRA_SET_SAY_TO,
                EXTRA_SET_SAY_TO_INFO,
                0x005e_3fa0,
            ],
        );
        e
    }

    #[test]
    fn fn_0057b7c0_an_actor_without_a_process_only_brackets_the_call() {
        let mut e = say_engine();
        returns(&mut e, 0x008d_8520, 0);
        let this = refr(&mut e, 0x30, &[(0x100, RETURNS_TRUE), (0xd0, SAY_SLOT_D0)]);
        start_log(&mut e);
        e.call(0x0057_b7c0, &args![this, 0x77u32, 1u8, 0u32]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, 0x0045_ce80),
            vec![vec![0x011c_d378, 0x7fff_ffff], vec![0x011c_d378, 0x1234]]
        );
        assert_eq!(calls_to(&log, SAY_SLOT_D0), vec![vec![this, 1]]);
        assert_eq!(calls_to(&log, 0x0096_e570), vec![vec![0x011e_0e80]]);
        assert!(calls_to(&log, 0x0061_b320).is_empty());
    }

    #[test]
    fn fn_0057b7c0_an_actor_runs_the_topic_and_shows_the_subtitle() {
        let mut e = say_engine();
        let player = 0x00a1_a1a1;
        e.set_global(GLOBAL_PLAYER, player);
        let response = 0xc000;
        let process = object_with(
            &mut e,
            0x40,
            &[
                (0x30c, RETURNS_TRUE),
                (0x2a4, SAY_SLOT_START),
                (0x248, SAY_SLOT_RESPONSE),
                (0x24c, SAY_SLOT_AFTER),
                (0x1dc, SAY_SLOT_LINE),
            ],
        );
        returns(&mut e, SAY_SLOT_RESPONSE, response);
        returns(&mut e, 0x008d_8520, process);
        returns(&mut e, 0x0061_b320, 0x6666);
        returns(&mut e, 0x0083_c820, 0xc100);
        let text = e.mem.alloc(8);
        let holder = e.mem.alloc(8);
        e.mem.set_u32(holder, text);
        returns(&mut e, 0x0068_15c0, holder);
        stub(&mut e, &[0x0042_ed30, 0x0055_d520, SPRINTF]);
        returns(&mut e, 0x0055_d520, 0x4e4e);
        returns(&mut e, 0x0070_3150, 0);
        let position = e.mem.alloc(16);
        e.mem.set_f32(position, 1.0);
        e.mem.set_f32(position + 4, 2.0);
        e.mem.set_f32(position + 8, 3.0);
        returns(&mut e, SAY_SLOT_POSITION, position);
        returns(&mut e, SAY_SLOT_INTERRUPT, 0);
        let this = refr(
            &mut e,
            0x30,
            &[
                (0x100, RETURNS_TRUE),
                (0xd0, SAY_SLOT_D0),
                (0x22c, RETURNS_FALSE),
                (0x214, SAY_SLOT_INTERRUPT),
                (0x1f4, SAY_SLOT_POSITION),
            ],
        );
        start_log(&mut e);
        e.call(0x0057_b7c0, &args![this, 0x77u32, 1u8, 0u32]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, 0x0061_b320),
            vec![vec![0x77, this, player, 0, 0, 1]]
        );
        assert_eq!(e.mem.u8(this + 0x7f), 1);
        assert_eq!(e.mem.u32(this + 0x70), player);
        // the interrupt virtual says 0 and the process is busy: it is ended
        assert_eq!(calls_to(&log, 0x0088_1680), vec![vec![this, 0]]);
        assert_eq!(calls_to(&log, SAY_SLOT_LINE), vec![vec![process, this, 1]]);
        assert_eq!(
            calls_to(&log, SAY_SLOT_START),
            vec![vec![process, this, 0x77, 1, 1, 1, 0]]
        );
        assert_eq!(calls_to(&log, 0x0083_c850).len(), 2);
        assert_eq!(
            calls_to(&log, 0x005a_c750),
            vec![vec![0x77, this + 0x44, 0x40000]]
        );
        let player_word = e.global::<u32>(GLOBAL_PLAYER);
        assert_eq!(
            calls_to(&log, 0x0070_5210),
            vec![vec![
                text,
                0,
                0,
                0,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                player_word,
                0
            ]]
        );
        assert_eq!(calls_to(&log, SAY_SLOT_AFTER), vec![vec![process, 0]]);
        assert!(calls_to(&log, SPRINTF).is_empty());
        // the full-help form formats the name and the text into a buffer
        returns(&mut e, 0x0070_3150, 1);
        start_log(&mut e);
        e.call(0x0057_b7c0, &args![this, 0x77u32, 1u8, 0u32]);
        let log = take_log(&mut e);
        let sprintf = calls_to(&log, SPRINTF);
        assert_eq!(sprintf.len(), 1);
        assert_eq!(sprintf[0][1..], [SUBTITLE_FORMAT, 0x4e4e, text]);
        let subtitle = calls_to(&log, 0x0070_5210);
        assert_eq!(subtitle[0][0], sprintf[0][0]);
    }

    #[test]
    fn fn_0057b7c0_other_references_start_the_line_through_virtual_13c() {
        let mut e = say_engine();
        let player = 0x00a1_a1a1;
        e.set_global(GLOBAL_PLAYER, player);
        returns(&mut e, SAY_SLOT_MODEL, 0x3d3d);
        let dialogue = e.mem.alloc(0x20);
        e.mem.set_u32(dialogue + 0x0c, 0xd1d1);
        returns(&mut e, 0x0061_b320, dialogue);
        let this = refr(
            &mut e,
            0x30,
            &[
                (0xd0, SAY_SLOT_D0),
                (0x1d0, SAY_SLOT_MODEL),
                (0xfc, RETURNS_TRUE),
                (0x13c, SAY_SLOT_LINE),
            ],
        );
        e.mem.set_u32(this + 0x0c, 0xf0f0);
        start_log(&mut e);
        e.call(0x0057_b7c0, &args![this, 0x77u32, 0u8, 0u32]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, EXTRA_SET_SAY_TO),
            vec![vec![this + 0x44, 0x77]]
        );
        assert_eq!(
            calls_to(&log, 0x0061_b320),
            vec![vec![0x77, this, 0, 0, 0, 0]]
        );
        assert_eq!(
            calls_to(&log, EXTRA_SET_SAY_TO_INFO),
            vec![vec![this + 0x44, 0xd1d1]]
        );
        let line = calls_to(&log, SAY_SLOT_LINE);
        assert_eq!(line.len(), 1);
        assert_eq!(line[0][0], this);
        assert_eq!(line[0][2..], [0x77, player, 0, 0, 0, 0, 0]);
        let handle = line[0][1];
        assert_eq!(
            calls_to(&log, HANDLE_SET_COMPLETION_CALLBACK),
            vec![vec![handle, 0x0057_90f0, 0xf0f0]]
        );
        assert_eq!(calls_to(&log, HANDLE_DESTRUCT), vec![vec![handle]]);
        assert_eq!(calls_to(&log, SAY_SLOT_D0), vec![vec![this, 1]]);
    }

    #[test]
    fn fn_0057b7c0_a_reference_without_a_model_asks_its_owner() {
        let mut e = say_engine();
        returns(&mut e, SAY_SLOT_MODEL, 0);
        returns(&mut e, 0x005e_3fa0, 0);
        let this = refr(
            &mut e,
            0x30,
            &[
                (0xd0, SAY_SLOT_D0),
                (0x1d0, SAY_SLOT_MODEL),
                (0xfc, RETURNS_TRUE),
            ],
        );
        let owner = object_with(&mut e, 0x40, &[(0x1d0, SAY_SLOT_OWNER_MODEL)]);
        returns(&mut e, SAY_SLOT_OWNER_MODEL, 0);
        returns(&mut e, 0x005e_3fa0, owner);
        start_log(&mut e);
        e.call(0x0057_b7c0, &args![this, 0x77u32, 0u8, 0u32]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, SAY_SLOT_OWNER_MODEL), vec![vec![owner]]);
        assert!(calls_to(&log, 0x0061_b320).is_empty());
        // the predicate (virtual 0xfc) false: nothing but the bracket
        let this = refr(
            &mut e,
            0x30,
            &[
                (0xd0, SAY_SLOT_D0),
                (0x1d0, SAY_SLOT_MODEL),
                (0xfc, SAY_SLOT_PREDICATE),
            ],
        );
        returns(&mut e, SAY_SLOT_PREDICATE, 0);
        start_log(&mut e);
        e.call(0x0057_b7c0, &args![this, 0x77u32, 0u8, 0u32]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x005e_3fa0).is_empty());
        assert_eq!(calls_to(&log, SAY_SLOT_D0), vec![vec![this, 1]]);
    }

    // ---- 0057bd10 / 0057bd40 / 0057bd60 ----------------------------------

    #[test]
    fn fn_0057bd10_builds_the_record() {
        let mut e = engine3();
        let record = e.mem.alloc(16);
        e.mem.set_u32(record + 4, 0xffff_ffff);
        e.mem.set_u32(record + 8, 0xffff_ffff);
        assert_eq!(e.call(0x0057_bd10, &args![record, 0x55u32]).u32(), record);
        assert_eq!(e.mem.u32(record), 0x55);
        assert_eq!(e.mem.u8(record + 4), 0);
        assert_eq!(e.mem.u32(record + 8), 0);
    }

    #[test]
    fn fn_0057bd40_returns_the_result_of_0042ed30() {
        let mut e = engine3();
        returns(&mut e, 0x0042_ed30, 9);
        let this = e.mem.alloc(0x100);
        start_log(&mut e);
        assert_eq!(e.call(0x0057_bd40, &args![this]).u32(), 9);
        assert_eq!(
            calls_to(&take_log(&mut e), 0x0042_ed30),
            vec![vec![this + 0x44]]
        );
    }

    #[test]
    fn fn_0057bd60_stores_the_value_at_0x70() {
        let mut e = engine3();
        let this = e.mem.alloc(0x100);
        e.call(0x0057_bd60, &args![this, 0xabcdu32]);
        assert_eq!(e.mem.u32(this + 0x70), 0xabcd);
    }

    // ---- 0057bd80 -------------------------------------------------------

    const BOUND_B8: u32 = 0x00ff_0301;
    const NODE_DONE: u32 = 0x00ff_0302;

    struct BoundCase {
        e: Engine,
        node: u32,
        bound: u32,
        first: u32,
        owner: u32,
        seen: Seen,
    }

    fn words(e: &mut Engine, at: u32) -> [u32; 3] {
        position_words(e, at)
    }

    /// A bounded node whose rotation angle is `angle`; its owner's bound has
    /// a first cast object (extent 1, 2, 3) when `cast`.
    fn bound_case(angle: f32, cast: bool) -> BoundCase {
        let mut e = engine3();
        for (addr, bits) in [
            (ROTATION_THRESHOLD, 0x3EE4_F8B5_88E3_68F1u64),
            (HALF_PI, 0x3FF9_21FB_6000_0000),
            (THREE_HALF_PI, 0x4012_D97C_8000_0000),
            (QUARTER_TURN_THRESHOLD, 0x3EE4_F8B5_8000_0000),
            (FULL_TURN_THRESHOLD, 0x3F1A_36E2_EB1C_432D),
            (TWO_PI, 0x4019_21FB_6000_0000),
        ] {
            e.set_global::<f64>(addr, f64::from_bits(bits));
        }
        let seen: Seen = Default::default();
        e.register(IS_KIND_OF, |e, a| {
            (a[0] == TYPE_BOUNDED_NODE && e.mem.u32(a[1] + 0x7c) == 1).into_ret()
        });
        e.register(0x0045_3470, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(CHILD_AT, |e, a| {
            e.mem.u32(a[0] + 0x24 + 4 * a[1]).into_ret()
        });
        e.register(0x0046_1130, |_, a| (a[0] + 0x40).into_ret());
        e.register(0x00a5_8550, |e, a| {
            let angle = e.mem.f32(a[0]);
            e.mem.set_f32(a[1], angle);
            Ret::default()
        });
        e.register(0x0040_8840, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0]).abs()),
            ..Ret::default()
        });
        e.register(0x0066_29f0, |e, a| e.mem.u32(a[0] + 0x44).into_ret());
        e.register(0x0043_b230, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(DYNAMIC_CAST, |e, a| {
            let at = match a[3] {
                TYPE_BOUND_TARGET_A => 4,
                TYPE_BOUND_TARGET_B => 8,
                _ => panic!("unexpected cast target"),
            };
            e.mem.u32(a[0] + at).into_ret()
        });
        e.register(0x0050_0940, |_, a| (a[0] + 0x10).into_ret());
        e.register(0x0045_bb80, |_, a| (a[0] + 0x50).into_ret());
        e.register(0x0043_d450, |_, a| (a[0] + 0x30).into_ret());
        e.register(0x0068_15c0, |_, a| (a[0] + 0x10).into_ret());
        returns_float(&mut e, 0x008d_01e0, 2.0);
        stub(&mut e, &[0x0043_96b0, 0x004a_ddc0, NODE_DONE]);
        e.register(0x00c3_6310, |_, a| a[0].into_ret());
        e.register_double(0x00aa_13e0, |e, a| {
            assert_eq!(a[0], 0x48);
            Ret {
                eax: object_with(e, 0x48, &[(0xb8, BOUND_B8)]),
                ..Ret::default()
            }
        });
        let log = seen.clone();
        e.register_double(BOUND_B8, move |e, a| {
            let centre = words(e, a[1]);
            log.borrow_mut().push((BOUND_B8, a[0], centre));
            Ret::default()
        });
        let log = seen.clone();
        e.register_double(0x0043_9180, move |e, a| {
            let extent = words(e, a[0]);
            log.borrow_mut().push((0x0043_9180, a[1], extent));
            Ret::default()
        });
        let log = seen.clone();
        e.register_double(0x0043_9680, move |e, a| {
            let extent = words(e, a[1]);
            log.borrow_mut().push((0x0043_9680, a[0], extent));
            Ret::default()
        });
        let bound = object_with(&mut e, 0x40, &[(0xb8, BOUND_B8)]);
        let first = e.mem.alloc(0x40);
        e.mem.set_f32(first + 0x10, 1.0);
        e.mem.set_f32(first + 0x14, 2.0);
        e.mem.set_f32(first + 0x18, 3.0);
        e.mem.set_u32(bound + 4, if cast { first } else { 0 });
        let owner = e.mem.alloc(0x10);
        e.mem.set_u32(owner, bound);
        let node = object_with(&mut e, 0x80, &[(0x0c, AS_NODE_SELF), (0xbc, NODE_DONE)]);
        e.mem.set_u32(node + 0x7c, 1);
        e.mem.set_u32(node + 0x44, owner);
        e.mem.set_f32(node + 0x40, angle);
        e.mem.set_f32(node + 0x50, 0.5);
        e.mem.set_f32(node + 0x54, 0.25);
        e.mem.set_f32(node + 0x58, 0.125);
        BoundCase {
            e,
            node,
            bound,
            first,
            owner,
            seen,
        }
    }

    const POSITION_WORDS: [u32; 3] = [0x3f00_0000, 0x3e80_0000, 0x3e00_0000];

    #[test]
    fn fn_0057bd80_an_unrotated_node_uses_the_zero_extent() {
        let mut c = bound_case(0.0, false);
        let this = c.e.mem.alloc(8);
        start_log(&mut c.e);
        assert!(c.e.call(0x0057_bd80, &args![this, c.node]).bool());
        let log = take_log(&mut c.e);
        assert_eq!(
            *c.seen.borrow(),
            vec![
                (BOUND_B8, c.bound, POSITION_WORDS),
                (0x0043_9180, 2.0f32.to_bits(), [0, 0, 0]),
            ]
        );
        assert!(calls_to(&log, 0x00aa_13e0).is_empty());
        assert_eq!(calls_to(&log, NODE_DONE), vec![vec![c.node]]);
    }

    #[test]
    fn fn_0057bd80_a_quarter_turn_swaps_the_first_two_extent_components() {
        let mut c = bound_case(1.570_796_4, true);
        let this = c.e.mem.alloc(8);
        assert!(c.e.call(0x0057_bd80, &args![this, c.node]).bool());
        let swapped = [2.0f32.to_bits(), 1.0f32.to_bits(), 3.0f32.to_bits()];
        assert_eq!(
            *c.seen.borrow(),
            vec![
                (BOUND_B8, c.bound, POSITION_WORDS),
                (0x0043_9180, 2.0f32.to_bits(), swapped),
                (0x0043_9680, c.first, swapped),
            ]
        );
    }

    #[test]
    fn fn_0057bd80_a_negative_angle_is_wrapped_before_the_tests() {
        // -pi/2 + 2 pi = 3 pi / 2: also a swap
        let mut c = bound_case(-1.570_796_4, true);
        let this = c.e.mem.alloc(8);
        assert!(c.e.call(0x0057_bd80, &args![this, c.node]).bool());
        let swapped = [2.0f32.to_bits(), 1.0f32.to_bits(), 3.0f32.to_bits()];
        assert_eq!(c.seen.borrow()[2], (0x0043_9680, c.first, swapped));
    }

    #[test]
    fn fn_0057bd80_any_other_rotation_makes_a_new_bound() {
        let mut c = bound_case(1.0, true);
        let this = c.e.mem.alloc(8);
        start_log(&mut c.e);
        assert!(c.e.call(0x0057_bd80, &args![this, c.node]).bool());
        let log = take_log(&mut c.e);
        assert_eq!(calls_to(&log, 0x00aa_13e0), vec![vec![0x48]]);
        let created = calls_to(&log, 0x0043_96b0)[0][0];
        assert_eq!(
            calls_to(&log, 0x0043_96b0),
            vec![vec![created, c.node + 0x40]]
        );
        assert_eq!(calls_to(&log, 0x004a_ddc0), vec![vec![c.owner, created]]);
        // the new bound gets the centre and the unswapped extent (read from
        // the old bound's cast object, which is not touched itself)
        let seen = c.seen.borrow();
        assert_eq!(seen[0], (BOUND_B8, created, POSITION_WORDS));
        let extent = [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()];
        assert_eq!(seen[1], (0x0043_9180, 2.0f32.to_bits(), extent));
        assert_eq!(seen[2], (0x0043_9680, created, extent));
        assert_eq!(seen.len(), 3);
    }

    #[test]
    fn fn_0057bd80_a_single_child_gives_the_centre_and_trees_stop_at_the_first_hit() {
        let mut c = bound_case(0.0, false);
        // the bounded node has one (not bounded) child with a centre of its
        // world bound
        let child = object_with(&mut c.e, 0x80, &[(0x0c, AS_NODE_SELF)]);
        c.e.mem.set_u32(c.node + 0x20, 1);
        c.e.mem.set_u32(c.node + 0x24, child);
        c.e.mem.set_f32(child + 0x40, 9.0);
        c.e.mem.set_f32(child + 0x44, 8.0);
        c.e.mem.set_f32(child + 0x48, 7.0);
        let this = c.e.mem.alloc(8);
        assert!(c.e.call(0x0057_bd80, &args![this, c.node]).bool());
        assert_eq!(
            c.seen.borrow()[0],
            (
                BOUND_B8,
                c.bound,
                [9.0f32.to_bits(), 8.0f32.to_bits(), 7.0f32.to_bits()]
            )
        );
        // a parent that is not bounded: only its first bounded child is built
        let mut d = bound_case(0.0, false);
        let second = object_with(&mut d.e, 0x80, &[(0x0c, AS_NODE_SELF), (0xbc, NODE_DONE)]);
        d.e.mem.set_u32(second + 0x7c, 1);
        d.e.mem.set_u32(second + 0x44, d.owner);
        let plain = object_with(&mut d.e, 0x80, &[(0x0c, AS_NODE_SELF)]);
        d.e.mem.set_u32(plain + 0x20, 3);
        let miss = object_with(&mut d.e, 0x80, &[(0x0c, AS_NODE_SELF)]);
        d.e.mem.set_u32(plain + 0x24, miss);
        d.e.mem.set_u32(plain + 0x28, d.node);
        d.e.mem.set_u32(plain + 0x2c, second);
        let this = d.e.mem.alloc(8);
        start_log(&mut d.e);
        assert!(d.e.call(0x0057_bd80, &args![this, plain]).bool());
        let log = take_log(&mut d.e);
        assert_eq!(calls_to(&log, NODE_DONE), vec![vec![d.node]]);
        // nothing bounded below: false; a null node: false
        d.e.mem.set_u32(plain + 0x20, 1);
        assert!(!d.e.call(0x0057_bd80, &args![this, plain]).bool());
        assert!(!d.e.call(0x0057_bd80, &args![this, 0u32]).bool());
    }

    // ---- 0057c180 / 0057c2a0 / 0057c300 ------------------------------------

    #[test]
    fn create_loaded_data_fills_the_record() {
        let mut e = engine3();
        e.set_global::<f32>(NO_WATER_HEIGHT, -3.0e38);
        e.set_global::<f32>(NO_CACHED_RADIUS, -1.0);
        allocator(&mut e, 0x0040_1000);
        stub(&mut e, &[0x0063_3c90, 0x0066_b0d0]);
        returns_float(&mut e, 0x0054_71e0, 42.5);
        returns(&mut e, 0x0045_18e0, 1);
        let this = refr(&mut e, 0x30, &[]);
        e.mem.set_u32(this + 0x40, 0xce11);
        start_log(&mut e);
        e.call(0x0057_c180, &args![this]);
        let log = take_log(&mut e);
        let loaded = e.mem.u32(this + 0x64);
        assert_ne!(loaded, 0);
        assert_eq!(calls_to(&log, 0x0040_1000), vec![vec![0x1c]]);
        assert_eq!(e.mem.u32(loaded), 0);
        assert_eq!(e.mem.i32(loaded + 4), 0);
        assert_eq!(e.mem.f32(loaded + 8), 42.5);
        assert_eq!(e.mem.f32(loaded + 0xc), -1.0);
        assert_eq!(e.mem.u32(loaded + 0x10), 0);
        // the constructor's two pointer resets, then the two of the caller
        assert_eq!(
            calls_to(&log, 0x0066_b0d0),
            vec![vec![loaded + 0x14, 0], vec![loaded + 0x18, 0]]
        );
        assert_eq!(
            calls_to(&log, 0x0063_3c90),
            vec![vec![loaded + 0x14, 0], vec![loaded + 0x18, 0]]
        );
        // existing data: nothing happens
        start_log(&mut e);
        e.call(0x0057_c180, &args![this]);
        assert!(calls_to(&take_log(&mut e), 0x0040_1000).is_empty());
        // a cell without water or no cell: the default height
        e.mem.set_u32(this + 0x64, 0);
        returns(&mut e, 0x0045_18e0, 0);
        e.call(0x0057_c180, &args![this]);
        let loaded = e.mem.u32(this + 0x64);
        assert_eq!(e.mem.f32(loaded + 8), -3.0e38);
        e.mem.set_u32(this + 0x64, 0);
        e.mem.set_u32(this + 0x40, 0);
        e.call(0x0057_c180, &args![this]);
        let loaded = e.mem.u32(this + 0x64);
        assert_eq!(e.mem.f32(loaded + 8), -3.0e38);
    }

    #[test]
    fn fn_0057c2a0_nulls_the_two_pointers_and_returns_this() {
        let mut e = engine3();
        stub(&mut e, &[0x0063_3c90]);
        let this = e.mem.alloc(0x20);
        start_log(&mut e);
        assert_eq!(e.call(0x0057_c2a0, &args![this]).u32(), this);
        assert_eq!(
            calls_to(&take_log(&mut e), 0x0063_3c90),
            vec![vec![this + 0x14, 0], vec![this + 0x18, 0]]
        );
    }

    #[test]
    fn fn_0057c300_deletes_the_loaded_data() {
        let mut e = engine3();
        stub(&mut e, &[0x0066_b0d0, 0x0045_cec0, FREE_OBJECT]);
        let this = e.mem.alloc(0x100);
        start_log(&mut e);
        e.call(0x0057_c300, &args![this]);
        assert_eq!(take_log(&mut e).len(), 1);
        let loaded = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x64, loaded);
        start_log(&mut e);
        e.call(0x0057_c300, &args![this]);
        let log = take_log(&mut e);
        assert_eq!(
            calls_to(&log, 0x0066_b0d0),
            vec![vec![loaded + 0x14, 0], vec![loaded + 0x18, 0]]
        );
        assert_eq!(
            calls_to(&log, 0x0045_cec0),
            vec![vec![loaded + 0x18], vec![loaded + 0x14]]
        );
        assert_eq!(calls_to(&log, FREE_OBJECT), vec![vec![loaded]]);
        assert_eq!(e.mem.u32(this + 0x64), 0);
    }

    // ---- 0057c370 / 0057c3a0 ----------------------------------------------

    #[test]
    fn fn_0057c370_destroys_and_frees_on_bit_zero() {
        let mut e = engine3();
        stub(&mut e, &[0x0045_cec0, FREE_OBJECT]);
        let this = e.mem.alloc(0x20);
        start_log(&mut e);
        assert_eq!(e.call(0x0057_c370, &args![this, 1u32]).u32(), this);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0045_cec0).len(), 2);
        assert_eq!(calls_to(&log, FREE_OBJECT), vec![vec![this]]);
        start_log(&mut e);
        e.call(0x0057_c370, &args![this, 2u32]);
        assert!(calls_to(&take_log(&mut e), FREE_OBJECT).is_empty());
    }

    #[test]
    fn fn_0057c3a0_releases_plus_0x18_then_plus_0x14() {
        let mut e = engine3();
        stub(&mut e, &[0x0045_cec0]);
        let this = e.mem.alloc(0x20);
        start_log(&mut e);
        e.call(0x0057_c3a0, &args![this]);
        assert_eq!(
            calls_to(&take_log(&mut e), 0x0045_cec0),
            vec![vec![this + 0x18], vec![this + 0x14]]
        );
    }

    // ---- 0057c400 -------------------------------------------------------

    #[test]
    fn fn_0057c400_finds_the_node_by_form_id() {
        let mut e = engine3();
        let nodes: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x20)).collect();
        for (i, node) in nodes.iter().enumerate() {
            e.mem.set_u32(node + 0x0c, 5 + i as u32);
            let next = nodes.get(i + 1).copied().unwrap_or(0);
            e.mem.set_u32(node + 0x10, next);
        }
        assert_eq!(e.call(0x0057_c400, &args![nodes[0], 5u32]).u32(), nodes[0]);
        assert_eq!(e.call(0x0057_c400, &args![nodes[0], 7u32]).u32(), nodes[2]);
        assert_eq!(e.call(0x0057_c400, &args![nodes[0], 9u32]).u32(), 0);
        assert_eq!(e.call(0x0057_c400, &args![0u32, 5u32]).u32(), 0);
    }

    // ---- constructors -----------------------------------------------------

    #[test]
    fn fn_0057c440_runs_the_base_and_stores_the_vtable() {
        let mut e = engine3();
        stub(&mut e, &[0x0055_9b40]);
        let this = e.mem.alloc(0x20);
        start_log(&mut e);
        assert_eq!(e.call(0x0057_c440, &args![this, 37u32]).u32(), this);
        assert_eq!(
            calls_to(&take_log(&mut e), 0x0055_9b40),
            vec![vec![this, 37]]
        );
        assert_eq!(e.mem.u32(this), 0x0103_0f4c);
    }

    #[test]
    fn fn_0057c470_builds_the_table_and_stores_its_own_vtable() {
        let mut e = engine3();
        allocator(&mut e, ALLOCATE_BUCKETS);
        stub(&mut e, &[MEMSET]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 99);
        start_log(&mut e);
        assert_eq!(e.call(0x0057_c470, &args![this, 8u32]).u32(), this);
        let log = take_log(&mut e);
        assert_eq!(e.mem.u32(this), 0x0103_0f6c);
        assert_eq!(e.mem.u32(this + 4), 8);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        let buckets = e.mem.u32(this + 8);
        assert_ne!(buckets, 0);
        assert_eq!(calls_to(&log, ALLOCATE_BUCKETS), vec![vec![32]]);
        assert_eq!(calls_to(&log, MEMSET), vec![vec![buckets, 0, 32]]);
    }

    #[test]
    fn fn_0057c7e0_builds_a_zeroed_bucket_array() {
        let mut e = engine3();
        allocator(&mut e, ALLOCATE_BUCKETS);
        stub(&mut e, &[MEMSET]);
        let this = e.mem.alloc(0x20);
        start_log(&mut e);
        assert_eq!(e.call(0x0057_c7e0, &args![this, 5u32]).u32(), this);
        let log = take_log(&mut e);
        assert_eq!(e.mem.u32(this), 0x0103_0fb4);
        assert_eq!(e.mem.u32(this + 4), 5);
        let buckets = e.mem.u32(this + 8);
        assert_eq!(calls_to(&log, MEMSET), vec![vec![buckets, 0, 20]]);
    }

    #[test]
    fn fn_0057c960_builds_a_zeroed_bucket_array() {
        let mut e = engine3();
        allocator(&mut e, ALLOCATE_BUCKETS);
        stub(&mut e, &[MEMSET]);
        let this = e.mem.alloc(0x20);
        start_log(&mut e);
        assert_eq!(e.call(0x0057_c960, &args![this, 3u32]).u32(), this);
        let log = take_log(&mut e);
        assert_eq!(e.mem.u32(this), 0x0103_0fd4);
        assert_eq!(e.mem.u32(this + 4), 3);
        let buckets = e.mem.u32(this + 8);
        assert_eq!(calls_to(&log, MEMSET), vec![vec![buckets, 0, 12]]);
    }

    #[test]
    fn fn_0057c700_runs_the_base_and_stores_the_vtable() {
        let mut e = engine3();
        stub(&mut e, &[0x0057_cb70]);
        let this = e.mem.alloc(0x20);
        start_log(&mut e);
        assert_eq!(e.call(0x0057_c700, &args![this, 3u32, 4u32]).u32(), this);
        assert_eq!(
            calls_to(&take_log(&mut e), 0x0057_cb70),
            vec![vec![this, 3, 4]]
        );
        assert_eq!(e.mem.u32(this), 0x0103_0fac);
    }

    // ---- destructors ------------------------------------------------------

    #[test]
    fn fn_0057c780_stores_the_vtable_and_runs_the_two_destructors() {
        let mut e = engine3();
        stub(&mut e, &[0x0043_8af0, 0x0055_96a0]);
        let this = e.mem.alloc(0x20);
        start_log(&mut e);
        e.call(0x0057_c780, &args![this]);
        let log = take_log(&mut e);
        assert_eq!(e.mem.u32(this), 0x0103_0f4c);
        let order: Vec<u32> = log.iter().map(|(a, _)| *a).skip(1).collect();
        assert_eq!(order, vec![0x0043_8af0, 0x0055_96a0]);
    }

    #[test]
    fn fn_0057c8d0_and_fn_0057c9d0_destroy_through_their_bases() {
        let mut e = engine3();
        stub(&mut e, &[0x0043_8af0, FREE_BUCKETS]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 0xb0b0);
        start_log(&mut e);
        e.call(0x0057_c8d0, &args![this]);
        let log = take_log(&mut e);
        // the base destructor leaves its own vtable
        assert_eq!(e.mem.u32(this), 0x0103_0fb4);
        assert_eq!(calls_to(&log, 0x0043_8af0).len(), 2);
        assert_eq!(calls_to(&log, FREE_BUCKETS), vec![vec![0xb0b0]]);
        start_log(&mut e);
        e.call(0x0057_c9d0, &args![this]);
        let log = take_log(&mut e);
        assert_eq!(e.mem.u32(this), 0x0103_0fd4);
        assert_eq!(calls_to(&log, 0x0043_8af0).len(), 2);
        assert_eq!(calls_to(&log, FREE_BUCKETS), vec![vec![0xb0b0]]);
    }

    #[test]
    fn fn_0057c930_and_fn_0057ca30_free_the_buckets() {
        let mut e = engine3();
        stub(&mut e, &[0x0043_8af0, FREE_BUCKETS]);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 8, 0xb0b0);
        start_log(&mut e);
        e.call(0x0057_c930, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0103_0fb4);
        e.call(0x0057_ca30, &args![this]);
        let log = take_log(&mut e);
        assert_eq!(e.mem.u32(this), 0x0103_0fd4);
        assert_eq!(
            calls_to(&log, FREE_BUCKETS),
            vec![vec![0xb0b0], vec![0xb0b0]]
        );
    }

    #[test]
    fn the_scalar_deleting_destructors_free_on_bit_zero() {
        let mut e = engine3();
        stub(
            &mut e,
            &[
                0x0043_8af0,
                0x0055_96a0,
                FREE_BUCKETS,
                FREE_OBJECT,
                0x004b_05b0,
            ],
        );
        let this = e.mem.alloc(0x20);
        for addr in [
            0x0057_c4a0u32,
            0x0057_c4d0,
            0x0057_c500,
            0x0057_ca60,
            0x0057_ca90,
        ] {
            start_log(&mut e);
            assert_eq!(e.call(addr, &args![this, 1u32]).u32(), this);
            assert_eq!(calls_to(&take_log(&mut e), FREE_OBJECT), vec![vec![this]]);
            start_log(&mut e);
            assert_eq!(e.call(addr, &args![this, 0u32]).u32(), this);
            assert!(calls_to(&take_log(&mut e), FREE_OBJECT).is_empty());
        }
        // each one runs its own destructor first
        start_log(&mut e);
        e.call(0x0057_c4a0, &args![this, 0u32]);
        assert_eq!(calls_to(&take_log(&mut e), 0x0055_96a0).len(), 1);
        start_log(&mut e);
        e.call(0x0057_ca60, &args![this, 0u32]);
        assert_eq!(calls_to(&take_log(&mut e), 0x004b_05b0), vec![vec![this]]);
    }

    // ---- 0057c530 / 0057c590 ----------------------------------------------

    #[test]
    fn fn_0057c530_runs_0057caf0_then_00483710() {
        let mut e = engine3();
        stub(&mut e, &[0x0057_caf0, 0x0048_3710]);
        let this = e.mem.alloc(0x20);
        start_log(&mut e);
        e.call(0x0057_c530, &args![this]);
        let order: Vec<(u32, Vec<u32>)> = take_log(&mut e).into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![(0x0057_caf0, vec![this]), (0x0048_3710, vec![this])]
        );
    }

    #[test]
    fn add_tail_stores_the_item_in_a_new_node_and_links_it() {
        let mut e = engine3();
        returns(&mut e, 0x0049_fa80, 0x7000);
        stub(&mut e, &[0x006e_5cc0, 0x0055_9a70]);
        let this = e.mem.alloc(0x20);
        start_log(&mut e);
        e.call(0x0057_c590, &args![this, 0xeeeeu32]);
        let log = take_log(&mut e);
        assert_eq!(calls_to(&log, 0x0049_fa80), vec![vec![this]]);
        assert_eq!(calls_to(&log, 0x006e_5cc0), vec![vec![0x7008, 0xeeee]]);
        assert_eq!(calls_to(&log, 0x0055_9a70), vec![vec![this, 0x7000]]);
    }

    // ---- 0057c5d0 / 0057c6a0 ----------------------------------------------

    #[test]
    fn fn_0057c6a0_scans_from_the_cursor_until_the_predicate_holds() {
        let mut e = engine3();
        // 0057cbe0 advances the cursor by 0x10 (0 after 0x30) and returns
        // the entry it left; the predicate accepts the entry 0x20
        e.register(0x0057_cbe0, |e, a| {
            let entry = e.mem.u32(a[1]);
            e.mem
                .set_u32(a[1], if entry >= 0x30 { 0 } else { entry + 0x10 });
            entry.into_ret()
        });
        e.register(0x004b_0460, |_, a| (a[1] == a[0]).into_ret());
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x10);
        // from the first entry
        assert_eq!(e.call(0x0057_c6a0, &args![this, 0x20u32, 0u32]).u32(), 0x20);
        // from a given cursor
        assert_eq!(
            e.call(0x0057_c6a0, &args![this, 0x30u32, 0x20u32]).u32(),
            0x30
        );
        // no entry accepted
        assert_eq!(e.call(0x0057_c6a0, &args![this, 0x99u32, 0u32]).u32(), 0);
        // an empty container
        e.mem.set_u32(this, 0);
        assert_eq!(e.call(0x0057_c6a0, &args![this, 0x20u32, 0u32]).u32(), 0);
    }

    #[test]
    fn fn_0057c5d0_hands_the_found_entry_or_the_source_to_00559a40() {
        let mut e = engine3();
        e.register(0x0057_cbe0, |e, a| {
            let entry = e.mem.u32(a[1]);
            e.mem.set_u32(a[1], 0);
            entry.into_ret()
        });
        e.register(0x004b_0460, |_, a| (a[1] == a[0]).into_ret());
        stub(&mut e, &[0x0055_9a40, 0x0045_cec0]);
        let entry_seen = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let seen = entry_seen.clone();
        e.register_double(0x004a_4500, move |e, a| {
            seen.set(e.mem.u32(a[2]));
            Ret {
                eax: 0xabc,
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x10);
        let dest = 0x9000;
        // found: the temporary pointer is built, used and released
        start_log(&mut e);
        assert_eq!(e.call(0x0057_c5d0, &args![this, dest, 0x10u32]).u32(), dest);
        let log = take_log(&mut e);
        assert_eq!(entry_seen.get(), 0x10);
        let built = calls_to(&log, 0x004a_4500);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][0], this);
        assert_eq!(calls_to(&log, 0x0055_9a40), vec![vec![dest, 0xabc]]);
        assert_eq!(calls_to(&log, 0x0045_cec0), vec![vec![built[0][1]]]);
        // not found: the source goes through
        start_log(&mut e);
        e.call(0x0057_c5d0, &args![this, dest, 0x77u32]);
        let log = take_log(&mut e);
        assert!(calls_to(&log, 0x004a_4500).is_empty());
        assert!(calls_to(&log, 0x0045_cec0).is_empty());
        assert_eq!(calls_to(&log, 0x0055_9a40), vec![vec![dest, 0x77]]);
    }

    // ---- 0057c730 -------------------------------------------------------

    #[test]
    fn fn_0057c730_prefers_the_found_entry_to_the_slot() {
        let mut e = engine3();
        returns(&mut e, 0x0049_c680, 0);
        let this = e.mem.alloc(8);
        let slot = e.mem.alloc(8);
        e.mem.set_u32(slot, 0x1357);
        start_log(&mut e);
        assert_eq!(e.call(0x0057_c730, &args![this, slot]).u32(), 0x1357);
        assert_eq!(
            calls_to(&take_log(&mut e), 0x0049_c680),
            vec![vec![this, slot, 0]]
        );
        let seen = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let found = seen.clone();
        returns(&mut e, 0x0049_c680, 0x2468);
        e.register_double(0x0049_f590, move |e, a| {
            found.set(e.mem.u32(a[1]));
            Ret {
                eax: 0x8888,
                ..Ret::default()
            }
        });
        assert_eq!(e.call(0x0057_c730, &args![this, slot]).u32(), 0x8888);
        assert_eq!(seen.get(), 0x2468);
    }

    // ---- 0057c850 -------------------------------------------------------

    #[test]
    fn fn_0057c850_walks_the_bucket_chain() {
        let mut e = engine3();
        const HASH: u32 = 0x00ff_0401;
        const EQUAL: u32 = 0x00ff_0402;
        // the hash is the key modulo 4; keys are equal when they are
        e.register(HASH, |_, a| (a[1] % 4).into_ret());
        e.register(EQUAL, |_, a| (a[1] == a[2]).into_ret());
        let this = object_with(&mut e, 0x20, &[(4, HASH), (8, EQUAL)]);
        let table = e.mem.alloc(16);
        e.mem.set_u32(this + 8, table);
        // bucket 1: key 5 (value 0x11) then key 9 (value 0x22)
        let second = e.mem.alloc(16);
        e.mem.set_u32(second + 4, 9);
        e.mem.set_u8(second + 8, 0x22);
        let first = e.mem.alloc(16);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 4, 5);
        e.mem.set_u8(first + 8, 0x11);
        e.mem.set_u32(table + 4, first);
        let out = e.mem.alloc(4);
        assert!(e.call(0x0057_c850, &args![this, 5u32, out]).bool());
        assert_eq!(e.mem.u8(out), 0x11);
        assert!(e.call(0x0057_c850, &args![this, 9u32, out]).bool());
        assert_eq!(e.mem.u8(out), 0x22);
        e.mem.set_u8(out, 0x77);
        assert!(!e.call(0x0057_c850, &args![this, 13u32, out]).bool());
        assert!(!e.call(0x0057_c850, &args![this, 2u32, out]).bool());
        assert_eq!(e.mem.u8(out), 0x77);
    }

    #[test]
    fn fn_0057b500_answers_0042f2d0s_value() {
        let mut e = engine3();
        e.register(0x0042_f2d0, |_, _| true.into_ret());
        let this = e.mem.alloc(0x100);
        assert!(e.call(0x0057_b500, &args![this]).bool());
    }
}
