//! `(unplaced)/run_00624210` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `FOIslandActivationListener` (RTTI of the vtables `0104de00` and
//! `0104dde4`): a Havok island-activation and entity listener the game adds
//! to its `hkpWorld`, and the callbacks that wake or settle the references
//! whose collision objects belong to an island's entities. None of these
//! functions has an Xbox PDB name; they are named by address and described
//! by their bodies.

#[allow(unused_imports)]
use crate::prelude::*;

/// The `hkpIslandActivationListener` vtable (RTTI
/// `.?AVhkpIslandActivationListener@@`).
const ISLAND_ACTIVATION_LISTENER_VTABLE: u32 = 0x0104_de10;
/// `FOIslandActivationListener`'s primary vtable (RTTI
/// `.?AVFOIslandActivationListener@@`).
const FO_ISLAND_LISTENER_VTABLE: u32 = 0x0104_de00;
/// `FOIslandActivationListener`'s second vtable, at +4 (its
/// `hkpEntityListener` part; the same RTTI name).
const FO_ISLAND_LISTENER_ENTITY_VTABLE: u32 = 0x0104_dde4;
/// The `hkpEntityListener` part's destructor body (`00554650`, thiscall).
const ENTITY_LISTENER_DESTRUCTOR: u32 = 0x0055_4650;

/// `hkpWorld` (Havok SDK `hkpworld.cpp`) listener calls, thiscall on the
/// world with one word; the bodies are not translated here.
const WORLD_CALL_C938A0: u32 = 0x00c9_38a0;
const WORLD_CALL_C93820: u32 = 0x00c9_3820;
const WORLD_CALL_C91840: u32 = 0x00c9_1840;
const WORLD_CALL_C917A0: u32 = 0x00c9_17a0;

/// `hkpEntity::isActive` (engine map): thiscall(out byte), returns the out
/// pointer.
const ENTITY_IS_ACTIVE: u32 = 0x00c9_bff0;
/// Reads the byte at `this` as a bool (`005609f0`).
const BYTE_AS_BOOL: u32 = 0x0056_09f0;
/// `globalfunc.cpp` helper (`004b5a50`, cdecl(entity)): the entity's
/// collision object, null when none.
const ENTITY_OWNER_4B5A50: u32 = 0x004b_5a50;
/// `globalfunc.cpp` helper (`004b5a80`, cdecl(entity)).
const ENTITY_OBJECT_4B5A80: u32 = 0x004b_5a80;
/// Thiscall getter on the object `004b5a50` gave (`0044ddc0`).
const OWNER_GET_44DDC0: u32 = 0x0044_ddc0;
/// `TESObjectREFR::FindReferenceFor3D` (engine map, cdecl(node)).
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
/// Thiscall getter on a reference (`008d6f30`).
const REFERENCE_GET_8D6F30: u32 = 0x008d_6f30;
/// Thiscall getter on the object `008d6f30` gave (`005497a0`).
const CELL_GET_5497A0: u32 = 0x0054_97a0;
/// A no-argument test (`008c7aa0`).
const TEST_8C7AA0: u32 = 0x008c_7aa0;
/// cdecl(reference, object) (`0054abd0`, `tesobjectcell.cpp`).
const CELL_CALL_54ABD0: u32 = 0x0054_abd0;
/// Increments the counter at `011f942c` (`00624560`).
const COUNTER_INCREMENT: u32 = 0x0062_4560;
/// Clears the counter at `011f942c` (`00624bd0`).
const COUNTER_CLEAR: u32 = 0x0062_4bd0;
/// Thiscall getter on the entity (`00460140`).
const ENTITY_GET_460140: u32 = 0x0046_0140;
/// Thiscall getter (`0043b540`: `0044ddc0` on `this + 0x14`).
const GET_43B540: u32 = 0x0043_b540;
/// Stores its word at `this` and returns `this` (`008c71b0`, thiscall).
const STORE_8C71B0: u32 = 0x008c_71b0;
/// `*this & 0x7F` (`0043b4d0`, thiscall).
const LOW_SEVEN_BITS: u32 = 0x0043_b4d0;
/// The value `0043b4d0` is compared against.
const KIND_0X14: u32 = 0x14;
/// cdecl(table, object) test (`0043b300`, `modelloader.cpp`).
const TABLE_TEST_43B300: u32 = 0x0043_b300;
/// The table passed to `0043b300`.
const TABLE_12681C0: u32 = 0x0126_81c0;
/// The island's entity array (`00624700`: `this + 0x48`).
const ISLAND_ENTITIES: u32 = 0x48;
/// The array's size (`00726070`, thiscall).
const ARRAY_SIZE: u32 = 0x0072_6070;
/// The array's element address (`00629ab0`, thiscall(index)).
const ARRAY_ELEMENT: u32 = 0x0062_9ab0;
/// `BSXFlags::Find` (engine map, cdecl(node, flag)).
const BSX_FLAGS_FIND: u32 = 0x0062_4b70;
/// Thiscall getter on a reference (`005d43c0`).
const REFERENCE_GET_5D43C0: u32 = 0x005d_43c0;
/// `ExtraDataList::GetModelSwap` (engine map, thiscall).
const EXTRA_GET_MODEL_SWAP: u32 = 0x0042_e250;
/// `TESObjectCELL::AttachReference3D` (engine map, thiscall(reference, 0)).
const CELL_ATTACH_REFERENCE_3D: u32 = 0x0054_8880;
/// cdecl(0) (`00450b80`, `tes.cpp`): gives the `this` of the next call.
const TES_GET_450B80: u32 = 0x0045_0b80;
/// `ShadowSceneNode::UpdateObjectLighting` (engine map, thiscall(node, 0)).
const UPDATE_OBJECT_LIGHTING: u32 = 0x00b5_d9f0;
/// Thiscall getter on a reference (`007af430`, the engine map's
/// `BGSSaveFormBuffer::GetForm` on folded code).
const REFERENCE_GET_FORM: u32 = 0x007a_f430;
/// The two forms `007af430`'s result is compared against.
const FORM_011CA27C: u32 = 0x011c_a27c;
const FORM_011CA280: u32 = 0x011c_a280;
/// `bhkWorld::SetMotion` (engine map, cdecl(object, 5, 1, 0, 1)).
const BHK_WORLD_SET_MOTION: u32 = 0x00c6_a350;
/// Thiscall test on the collision object (`00624bb0`).
const OBJECT_TEST_624BB0: u32 = 0x0062_4bb0;
/// Thiscall(bool) on the collision object (`00573f00`).
const OBJECT_SET_573F00: u32 = 0x0057_3f00;
/// The constructor of the 0x40-byte stack transform (`0056ded0`).
const STACK_TRANSFORM_CONSTRUCT: u32 = 0x0056_ded0;
/// The constructor of the 0x34-byte stack transform (`00476a80`).
const STACK_NI_TRANSFORM_CONSTRUCT: u32 = 0x0047_6a80;
/// Returns `this` (`006815c0`).
const IDENTITY_6815C0: u32 = 0x0068_15c0;
/// Thiscall getter (`00436aa0`: `this + 0x30`).
const GET_436AA0: u32 = 0x0043_6aa0;
/// cdecl(destination, source) (`00458620`, `tes.cpp`).
const COPY_458620: u32 = 0x0045_8620;
/// Thiscall(index) (`004b4cf0`: `this + index * 16`).
const ROW_4B4CF0: u32 = 0x004b_4cf0;
/// cdecl(point, row) (`004a3970`).
const POINT_FROM_ROW: u32 = 0x004a_3970;
/// Thiscall(index, point) (`004769c0`).
const SET_COLUMN_4769C0: u32 = 0x0047_69c0;

/// Size of the `NiPoint3` local of `00624b10`.
const POINT_SIZE: u32 = 0x0c;
/// Size of the stack transform built by `0056ded0` (the code aligns the
/// frame to 16 bytes for it).
const STACK_TRANSFORM_SIZE: u32 = 0x40;
/// Size of the stack transform built by `00476a80`.
const STACK_NI_TRANSFORM_SIZE: u32 = 0x34;

/// Reference virtual slots, all thiscall: `+0x100` (no words, bool),
/// `+0x150` (no words, bool), `+0x1D0` (`Get3D`, the loaded 3D), `+0x224`
/// (no words, bool), `+0x31C` (no words), `+0x48` (one word).
const REFR_SLOT_100: u32 = 0x100;
const REFR_SLOT_150: u32 = 0x150;
const REFR_GET_3D: u32 = 0x1d0;
const REFR_SLOT_224: u32 = 0x224;
const REFR_SLOT_31C: u32 = 0x31c;
const REFR_SLOT_48: u32 = 0x48;

/// The object an entity belongs to and its reference: `004b5a50(entity)`,
/// then `0044ddc0` on it when it is not null, then `FindReferenceFor3D` on
/// that when it is not null. Returns `(object, reference)`; each is null
/// when the step before was.
fn entity_reference(e: &mut Engine, entity: u32) -> (u32, u32) {
    let owner = e.call(ENTITY_OWNER_4B5A50, &args![entity]).u32();
    let object = if owner == 0 {
        0
    } else {
        e.call(OWNER_GET_44DDC0, &args![owner]).u32()
    };
    let reference = if object == 0 {
        0
    } else {
        e.call(FIND_REFERENCE_FOR_3D, &args![object]).u32()
    };
    (object, reference)
}

/// The part `00624340` and `00624580` share for a reference whose `+0x100`
/// says no: the cell's object (`008d6f30`, then `005497a0`) gets the
/// reference through `0054abd0` when `008c7aa0` says so, else its virtual
/// `+0xDC` gets `(Get3D(), 1)`; then the counter at `011f942c` is
/// incremented when the reference's `+0x150` says so.
fn wake_reference(e: &mut Engine, reference: u32) {
    let cell = e.call(REFERENCE_GET_8D6F30, &args![reference]).u32();
    let object = if cell == 0 {
        0
    } else {
        e.call(CELL_GET_5497A0, &args![cell]).u32()
    };
    if object != 0 {
        if e.call(TEST_8C7AA0, &args![]).bool() {
            e.call(CELL_CALL_54ABD0, &args![reference, object]);
        } else {
            let node = e.vcall(reference, REFR_GET_3D, &args![]).u32();
            e.vcall(object, 0xdc, &args![node, 1u32]);
        }
    }
    if e.vcall(reference, REFR_SLOT_150, &args![]).bool() {
        e.call(COUNTER_INCREMENT, &args![]);
    }
}

/// `0043b4d0(008c71b0(local, 0043b540(00460140(entity))))`: the low seven
/// bits of the word the entity's chain gives, through a stack word.
fn entity_kind(e: &mut Engine, entity: u32) -> u32 {
    let holder = e.call(ENTITY_GET_460140, &args![entity]).u32();
    let word = e.call(GET_43B540, &args![holder]).u32();
    e.with_stack(4, |e, local| {
        let stored = e.call(STORE_8C71B0, &args![local, word]).u32();
        e.call(LOW_SEVEN_BITS, &args![stored]).u32()
    })
}

/// The object `004b5a80` gives for the entity when `reference` is not
/// null, its `+0x100` says no, and `0043b300` finds the object in the table
/// at `012681c0`; null otherwise (each test only when the ones before
/// passed, in the code's order).
fn table_object(e: &mut Engine, entity: u32, reference: u32) -> u32 {
    if reference == 0 || e.vcall(reference, REFR_SLOT_100, &args![]).bool() {
        return 0;
    }
    let object = e.call(ENTITY_OBJECT_4B5A80, &args![entity]).u32();
    if object == 0
        || !e
            .call(TABLE_TEST_43B300, &args![TABLE_12681C0, object])
            .bool()
    {
        return 0;
    }
    object
}

// Translated from 00624210 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `hkpIslandActivationListener` base destructor body: stores that
/// class's vtable.
pub fn fn_00624210(e: &mut Engine, this: Ptr) {
    e.mem
        .set_u32(this.addr(), ISLAND_ACTIVATION_LISTENER_VTABLE);
}

// Translated from 00624230 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00624230 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `FOIslandActivationListener`'s destructor body: stores its two vtables
/// (+0 and +4), runs the `hkpEntityListener` part's destructor (`00554650`
/// on `this + 4`) and then the `hkpIslandActivationListener` one
/// (`00624210`).
pub fn fn_00624230(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), FO_ISLAND_LISTENER_VTABLE);
    e.mem
        .set_u32(this.addr() + 4, FO_ISLAND_LISTENER_ENTITY_VTABLE);
    e.call(ENTITY_LISTENER_DESTRUCTOR, &args![this.addr() + 4]);
    fn_00624210(e, this);
}

// Translated from 006242a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When `world` is not null: `00c938a0(world, this)` and
/// `00c93820(world, this + 4)` (the entity-listener part, null for a null
/// `this`), the two `hkpWorld` calls that take this listener.
pub fn fn_006242a0(e: &mut Engine, this: Ptr, world: Ptr) {
    if world.is_null() {
        return;
    }
    e.call(WORLD_CALL_C938A0, &args![world, this]);
    let entity_part = if this.is_null() { 0 } else { this.addr() + 4 };
    e.call(WORLD_CALL_C93820, &args![world, entity_part]);
}

// Translated from 006242f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When `world` is not null: `00c91840(world, this)` and
/// `00c917a0(world, this + 4)` (null for a null `this`), the other two
/// `hkpWorld` listener calls.
pub fn fn_006242f0(e: &mut Engine, this: Ptr, world: Ptr) {
    if world.is_null() {
        return;
    }
    e.call(WORLD_CALL_C91840, &args![world, this]);
    let entity_part = if this.is_null() { 0 } else { this.addr() + 4 };
    e.call(WORLD_CALL_C917A0, &args![world, entity_part]);
}

// Translated from 00624340 (decompiled, FalloutNV.exe 1.4.0.525)
/// The listener's callback for one entity (in `FOIslandActivationListener`'s
/// vtables at `0104dde8` and `0104de08`). For a null entity, nothing.
///
/// When `hkpEntity::isActive` says the entity is active: its reference
/// (`004b5a50`, `0044ddc0`, `FindReferenceFor3D`), when there is one and
/// its `+0x100` says no, is woken: the cell object (`008d6f30`,
/// `005497a0`) gets it through `0054abd0` or its `+0xDC(Get3D(), 1)`, and
/// the counter at `011f942c` is incremented when its `+0x150` says so.
///
/// Otherwise, when the entity's kind is 0x14 and its reference (looked up
/// after the kind) passes the `+0x100`, `004b5a80` and `0043b300` tests,
/// that object's virtual `+0xE4` gets 4.
pub fn fn_00624340(e: &mut Engine, _this: Ptr, entity: Ptr) {
    if entity.is_null() {
        return;
    }
    let active = e.with_stack(4, |e, flag| {
        let flag = e.call(ENTITY_IS_ACTIVE, &args![entity, flag]).u32();
        e.call(BYTE_AS_BOOL, &args![flag]).bool()
    });
    if active {
        let (_, reference) = entity_reference(e, entity.addr());
        if reference != 0 && !e.vcall(reference, REFR_SLOT_100, &args![]).bool() {
            wake_reference(e, reference);
        }
    } else if entity_kind(e, entity.addr()) == KIND_0X14 {
        let (_, reference) = entity_reference(e, entity.addr());
        let object = table_object(e, entity.addr(), reference);
        if object != 0 {
            e.vcall(object, 0xe4, &args![4u32]);
        }
    }
}

// Translated from 00624580 (decompiled, FalloutNV.exe 1.4.0.525)
/// The callback for an island (null: nothing): for each entity of the
/// island's array (`this + 0x48`), its reference, when there is one and its
/// `+0x100` says no, gets its virtual `+0x48(4)` and is then woken as in
/// `00624340`.
pub fn fn_00624580(e: &mut Engine, _this: Ptr, island: Ptr) {
    if island.is_null() {
        return;
    }
    let entities = fn_00624700(e, island);
    let count = e.call(ARRAY_SIZE, &args![entities]).i32();
    let mut index = 0i32;
    while index < count {
        let slot = e.call(ARRAY_ELEMENT, &args![entities, index]).u32();
        let entity = e.mem.u32(slot);
        let (_, reference) = entity_reference(e, entity);
        if reference != 0 && !e.vcall(reference, REFR_SLOT_100, &args![]).bool() {
            e.vcall(reference, REFR_SLOT_48, &args![4u32]);
            wake_reference(e, reference);
        }
        index += 1;
    }
}

// Translated from 00624700 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the island's entity array, `this + 0x48`.
pub fn fn_00624700(_e: &mut Engine, this: Ptr) -> u32 {
    this.addr().wrapping_add(ISLAND_ENTITIES)
}

// Translated from 00624720 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00624720 - the /GS stack cookie check (00ec408c) is not translated.
/// The settling callback for an island (null: nothing). For each entity of
/// the island's array, with its object and reference (`004b5a50`,
/// `0044ddc0`, `FindReferenceFor3D`):
///
/// - with a reference: the model swap (`005d43c0`, then
///   `ExtraDataList::GetModelSwap`) and the two `BSXFlags::Find` tests of
///   its 3D for flags 8 and 0x80 are read; `+0x31C` runs when `+0x224` says
///   so; when `+0x100` says no, there is no model swap and flag 0x80 is set
///   or flag 8 is not, the cell (`008d6f30`) attaches the reference's 3D
///   (`TESObjectCELL::AttachReference3D(reference, 0)`) and the counter at
///   `011f942c` is cleared when `+0x150` says so; otherwise, when `+0x100`
///   says no, `ShadowSceneNode::UpdateObjectLighting(Get3D(), 0)` runs on
///   what `00450b80(0)` gives; then, when `007af430` gives the form at
///   `011ca27c` or the one at `011ca280`, `bhkWorld::SetMotion(object, 5, 1,
///   0, 1)`;
/// - then, when the entity's kind is 0x14 and its reference passes the
///   `+0x100`, `004b5a80` and `0043b300` tests, that object's `+0xE4` gets
///   4; and when the collision object (`004b5a50`) is not null and
///   `00624bb0` says no, the object's transform (its `+0xF4` into a
///   `0056ded0` stack transform) is converted (`00624ad0`) into a
///   `00476a80` stack transform that goes to the collision object's
///   `+0xB8`, between `00573f00(1)` and `00573f00(0)` on it.
pub fn fn_00624720(e: &mut Engine, _this: Ptr, island: Ptr) {
    if island.is_null() {
        return;
    }
    let entities = fn_00624700(e, island);
    let count = e.call(ARRAY_SIZE, &args![entities]).i32();
    let mut index = 0i32;
    while index < count {
        let slot = e.call(ARRAY_ELEMENT, &args![entities, index]).u32();
        let entity = e.mem.u32(slot);
        let (object, reference) = entity_reference(e, entity);
        let model_swap = if reference == 0 {
            0
        } else {
            let extra = e.call(REFERENCE_GET_5D43C0, &args![reference]).u32();
            e.call(EXTRA_GET_MODEL_SWAP, &args![extra]).u32()
        };
        let flag_8 = reference != 0 && {
            let node = e.vcall(reference, REFR_GET_3D, &args![]).u32();
            e.call(BSX_FLAGS_FIND, &args![node, 8u32]).u32() != 0
        };
        let flag_80 = reference != 0 && {
            let node = e.vcall(reference, REFR_GET_3D, &args![]).u32();
            e.call(BSX_FLAGS_FIND, &args![node, 0x80u32]).u32() != 0
        };
        if reference != 0 {
            if e.vcall(reference, REFR_SLOT_224, &args![]).bool() {
                e.vcall(reference, REFR_SLOT_31C, &args![]);
            }
            let attach = !e.vcall(reference, REFR_SLOT_100, &args![]).bool()
                && model_swap == 0
                && (!flag_8 || flag_80);
            if attach {
                let cell = e.call(REFERENCE_GET_8D6F30, &args![reference]).u32();
                if cell != 0 {
                    e.call(CELL_ATTACH_REFERENCE_3D, &args![cell, reference, 0u32]);
                }
                if e.vcall(reference, REFR_SLOT_150, &args![]).bool() {
                    e.call(COUNTER_CLEAR, &args![]);
                }
            } else if !e.vcall(reference, REFR_SLOT_100, &args![]).bool() {
                let node = e.vcall(reference, REFR_GET_3D, &args![]).u32();
                let scene = e.call(TES_GET_450B80, &args![0u32]).u32();
                e.call(UPDATE_OBJECT_LIGHTING, &args![scene, node, 0u32]);
            }
            let form = e.call(REFERENCE_GET_FORM, &args![reference]).u32();
            let matches = form == e.global::<u32>(FORM_011CA27C) || {
                let form = e.call(REFERENCE_GET_FORM, &args![reference]).u32();
                form == e.global::<u32>(FORM_011CA280)
            };
            if matches {
                e.call(BHK_WORLD_SET_MOTION, &args![object, 5u32, 1u32, 0u32, 1u32]);
            }
        }
        let body = if entity_kind(e, entity) == KIND_0X14 {
            table_object(e, entity, reference)
        } else {
            0
        };
        if body != 0 {
            e.vcall(body, 0xe4, &args![4u32]);
            let owner = e.call(ENTITY_OWNER_4B5A50, &args![entity]).u32();
            if owner != 0 && !e.call(OBJECT_TEST_624BB0, &args![owner]).bool() {
                e.call(OBJECT_SET_573F00, &args![owner, 1u32]);
                e.with_stack(STACK_TRANSFORM_SIZE, |e, transform| {
                    e.with_stack(STACK_NI_TRANSFORM_SIZE, |e, ni_transform| {
                        e.call(STACK_TRANSFORM_CONSTRUCT, &args![transform]);
                        e.call(STACK_NI_TRANSFORM_CONSTRUCT, &args![ni_transform]);
                        e.vcall(body, 0xf4, &args![transform]);
                        fn_00624ad0(e, ni_transform, transform);
                        e.vcall(owner, 0xb8, &args![ni_transform]);
                    });
                });
                e.call(OBJECT_SET_573F00, &args![owner, 0u32]);
            }
        }
        index += 1;
    }
}

// Translated from 00624ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Converts the transform at `source` into the one at `destination`
/// (cdecl): the rotation through `00624b10(destination, 006815c0(source))`,
/// then `00458620(destination + 0x24, 00436aa0(source))` (the part at
/// `source + 0x30`). Returns `destination`.
pub fn fn_00624ad0(e: &mut Engine, destination: Ptr, source: Ptr) -> Ptr {
    let rotation = e.call(IDENTITY_6815C0, &args![source]).ptr::<()>();
    fn_00624b10(e, destination, rotation);
    let translation = e.call(GET_436AA0, &args![source]).u32();
    e.call(
        COPY_458620,
        &args![destination.addr().wrapping_add(0x24), translation],
    );
    destination
}

// Translated from 00624b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies three rows of the matrix at `source` (16 bytes apart, `004b4cf0`)
/// into `destination` (cdecl): each row is turned into a stack point
/// (`004a3970`) and stored at index `index` (`004769c0`). The point is
/// first passed through `006815c0` (which returns it). Returns
/// `destination`.
pub fn fn_00624b10(e: &mut Engine, destination: Ptr, source: Ptr) -> Ptr {
    e.with_stack(POINT_SIZE, |e, point| {
        e.call(IDENTITY_6815C0, &args![point]);
        let mut index = 0u32;
        while index < 3 {
            let row = e.call(ROW_4B4CF0, &args![source, index]).u32();
            e.call(POINT_FROM_ROW, &args![point, row]);
            e.call(SET_COLUMN_4769C0, &args![destination, index, point]);
            index += 1;
        }
    });
    destination
}

/// The counter [`fn_00624560`] increments.
const COUNTER: u32 = 0x011f_942c;

// Translated from 00624560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Increments the counter at `011f942c` (unnamed in the engine map; no arguments, plain
/// `RET`; called by `00624340` and `00624580`) and answers its new value (`EAX`).
pub fn fn_00624560(e: &mut Engine) -> u32 {
    let value = e.global::<u32>(COUNTER).wrapping_add(1);
    e.set_global(COUNTER, value);
    value
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_4560, fn_00624560() -> u32),
        entry!(0x0062_4210, fn_00624210(Ptr)),
        entry!(0x0062_4230, fn_00624230(Ptr)),
        entry!(0x0062_42a0, fn_006242a0(Ptr, Ptr)),
        entry!(0x0062_42f0, fn_006242f0(Ptr, Ptr)),
        entry!(0x0062_4340, fn_00624340(Ptr, Ptr)),
        entry!(0x0062_4580, fn_00624580(Ptr, Ptr)),
        entry!(0x0062_4700, fn_00624700(Ptr) -> u32),
        entry!(0x0062_4720, fn_00624720(Ptr, Ptr)),
        entry!(0x0062_4ad0, fn_00624ad0(Ptr, Ptr) -> Ptr),
        entry!(0x0062_4b10, fn_00624b10(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type CallLog = [(u32, Vec<u32>)];

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Doubles every callee with a result of 0.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for addr in [
            ENTITY_LISTENER_DESTRUCTOR,
            WORLD_CALL_C938A0,
            WORLD_CALL_C93820,
            WORLD_CALL_C91840,
            WORLD_CALL_C917A0,
            ENTITY_IS_ACTIVE,
            BYTE_AS_BOOL,
            ENTITY_OWNER_4B5A50,
            ENTITY_OBJECT_4B5A80,
            OWNER_GET_44DDC0,
            FIND_REFERENCE_FOR_3D,
            REFERENCE_GET_8D6F30,
            CELL_GET_5497A0,
            TEST_8C7AA0,
            CELL_CALL_54ABD0,
            COUNTER_INCREMENT,
            COUNTER_CLEAR,
            ENTITY_GET_460140,
            GET_43B540,
            LOW_SEVEN_BITS,
            TABLE_TEST_43B300,
            ARRAY_SIZE,
            ARRAY_ELEMENT,
            BSX_FLAGS_FIND,
            REFERENCE_GET_5D43C0,
            EXTRA_GET_MODEL_SWAP,
            CELL_ATTACH_REFERENCE_3D,
            TES_GET_450B80,
            UPDATE_OBJECT_LIGHTING,
            REFERENCE_GET_FORM,
            BHK_WORLD_SET_MOTION,
            OBJECT_TEST_624BB0,
            OBJECT_SET_573F00,
            STACK_TRANSFORM_CONSTRUCT,
            STACK_NI_TRANSFORM_CONSTRUCT,
            GET_436AA0,
            COPY_458620,
            ROW_4B4CF0,
            POINT_FROM_ROW,
            SET_COLUMN_4769C0,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(STORE_8C71B0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(IDENTITY_6815C0, |_, a| a[0].into_ret());
        e
    }

    /// An object whose vtable sends slot `s` to `base + s`, each a double
    /// answering `result(s)`.
    fn object(e: &mut Engine, base: u32, result: fn(u32) -> u32) -> u32 {
        let vtable = e.mem.alloc(0x400);
        for slot in (0..0x400u32).step_by(4) {
            e.mem.set_u32(vtable + slot, base + slot);
            e.register_double(base + slot, move |_, _| result(slot).into_ret());
        }
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, vtable);
        this
    }

    /// The entity -> owner -> object -> reference chain.
    fn chain(e: &mut Engine, reference: u32) {
        e.register(ENTITY_OWNER_4B5A50, |_, _| 0x0bbb_0000u32.into_ret());
        e.register(OWNER_GET_44DDC0, |_, _| 0x0ccc_0000u32.into_ret());
        e.register_double(FIND_REFERENCE_FOR_3D, move |_, _| reference.into_ret());
    }

    const REF_SLOTS: u32 = 0x7100_0000;
    const BODY_SLOTS: u32 = 0x7200_0000;
    const CELL_SLOTS: u32 = 0x7300_0000;
    const OWNER_SLOTS: u32 = 0x7400_0000;

    #[test]
    fn base_destructor_stores_the_base_vtable() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        e.call(0x0062_4210, &args![this]);
        assert_eq!(e.mem.u32(this), ISLAND_ACTIVATION_LISTENER_VTABLE);
    }

    #[test]
    fn destructor_runs_both_base_parts() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0062_4230, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, ENTITY_LISTENER_DESTRUCTOR),
            vec![vec![this + 4]]
        );
        assert_eq!(e.mem.u32(this), ISLAND_ACTIVATION_LISTENER_VTABLE);
        assert_eq!(e.mem.u32(this + 4), FO_ISLAND_LISTENER_ENTITY_VTABLE);
    }

    #[test]
    fn world_calls_pass_both_parts() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x0062_42a0, &args![0x1000u32, 0u32]);
        e.call(0x0062_42a0, &args![0x1000u32, 0x2000u32]);
        e.call(0x0062_42a0, &args![0u32, 0x2000u32]);
        e.call(0x0062_42f0, &args![0x1000u32, 0x2000u32]);
        e.call(0x0062_42f0, &args![0x1000u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, WORLD_CALL_C938A0),
            vec![vec![0x2000, 0x1000], vec![0x2000, 0]]
        );
        assert_eq!(
            calls(&log, WORLD_CALL_C93820),
            vec![vec![0x2000, 0x1004], vec![0x2000, 0]]
        );
        assert_eq!(calls(&log, WORLD_CALL_C91840), vec![vec![0x2000, 0x1000]]);
        assert_eq!(calls(&log, WORLD_CALL_C917A0), vec![vec![0x2000, 0x1004]]);
    }

    #[test]
    fn entity_callback_wakes_an_active_entitys_reference() {
        let mut e = engine();
        // +0x150 true, everything else false.
        let reference = object(&mut e, REF_SLOTS, |slot| (slot == REFR_SLOT_150) as u32);
        let cell_object = object(&mut e, CELL_SLOTS, |_| 0);
        chain(&mut e, reference);
        e.register(BYTE_AS_BOOL, |_, _| 1u32.into_ret());
        e.register(REFERENCE_GET_8D6F30, |_, _| 0x0ddd_0000u32.into_ret());
        e.register_double(CELL_GET_5497A0, move |_, _| cell_object.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_4340, &args![0u32, 0x0aaa_0000u32]);
        let log = e.call_log.take().unwrap();
        // 008c7aa0 says no: the object's +0xDC gets (Get3D(), 1).
        assert_eq!(
            calls(&log, CELL_SLOTS + 0xdc),
            vec![vec![cell_object, 0, 1]]
        );
        assert!(calls(&log, CELL_CALL_54ABD0).is_empty());
        assert_eq!(calls(&log, COUNTER_INCREMENT).len(), 1);
        assert!(calls(&log, LOW_SEVEN_BITS).is_empty());

        // 008c7aa0 says yes: 0054abd0(reference, object).
        e.register(TEST_8C7AA0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_4340, &args![0u32, 0x0aaa_0000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, CELL_CALL_54ABD0),
            vec![vec![reference, cell_object]]
        );
        // A null entity does nothing.
        e.call_log = Some(vec![]);
        e.call(0x0062_4340, &args![0u32, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn entity_callback_on_an_inactive_kind_0x14_entity() {
        let mut e = engine();
        let reference = object(&mut e, REF_SLOTS, |_| 0);
        let body = object(&mut e, BODY_SLOTS, |_| 0);
        chain(&mut e, reference);
        e.register(LOW_SEVEN_BITS, |_, _| KIND_0X14.into_ret());
        e.register_double(ENTITY_OBJECT_4B5A80, move |_, _| body.into_ret());
        e.register(TABLE_TEST_43B300, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_4340, &args![0u32, 0x0aaa_0000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, BODY_SLOTS + 0xe4), vec![vec![body, 4]]);
        assert_eq!(
            calls(&log, TABLE_TEST_43B300),
            vec![vec![TABLE_12681C0, body]]
        );
        // Another kind: no lookup at all.
        e.register(LOW_SEVEN_BITS, |_, _| 0x13u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_4340, &args![0u32, 0x0aaa_0000u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, ENTITY_OWNER_4B5A50).is_empty());
    }

    #[test]
    fn island_callback_wakes_each_entitys_reference() {
        let mut e = engine();
        let reference = object(&mut e, REF_SLOTS, |_| 0);
        chain(&mut e, reference);
        let entities = e.mem.alloc(8);
        e.mem.set_u32(entities, 0x0aaa_0000);
        e.mem.set_u32(entities + 4, 0x0aaa_1000);
        e.register(ARRAY_SIZE, |_, _| 2u32.into_ret());
        e.register_double(ARRAY_ELEMENT, move |_, a| (entities + 4 * a[1]).into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_4580, &args![0u32, 0x5000u32]);
        e.call(0x0062_4580, &args![0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, ARRAY_SIZE), vec![vec![0x5048]]);
        assert_eq!(
            calls(&log, ENTITY_OWNER_4B5A50),
            vec![vec![0x0aaa_0000], vec![0x0aaa_1000]]
        );
        assert_eq!(
            calls(&log, REF_SLOTS + 0x48),
            vec![vec![reference, 4], vec![reference, 4]]
        );
        assert_eq!(calls(&log, REFERENCE_GET_8D6F30).len(), 2);
    }

    #[test]
    fn island_array_is_at_0x48() {
        let mut e = engine();
        assert_eq!(e.call(0x0062_4700, &args![0x1000u32]).u32(), 0x1048);
    }

    #[test]
    fn settling_callback_attaches_or_relights() {
        let mut e = engine();
        // +0x224 true, +0x150 true, +0x100 false.
        let reference = object(&mut e, REF_SLOTS, |slot| {
            (slot == REFR_SLOT_224 || slot == REFR_SLOT_150) as u32
        });
        chain(&mut e, reference);
        let entities = e.mem.alloc(4);
        e.mem.set_u32(entities, 0x0aaa_0000);
        e.register(ARRAY_SIZE, |_, _| 1u32.into_ret());
        e.register_double(ARRAY_ELEMENT, move |_, _| entities.into_ret());
        e.register(REFERENCE_GET_8D6F30, |_, _| 0x0ddd_0000u32.into_ret());
        e.map(0x011c_a000, 0x1000);
        e.set_global(FORM_011CA280, 0x0eee_0000u32);
        e.register(REFERENCE_GET_FORM, |_, _| 0x0eee_0000u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_4720, &args![0u32, 0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, REF_SLOTS + 0x31c), vec![vec![reference]]);
        assert_eq!(
            calls(&log, CELL_ATTACH_REFERENCE_3D),
            vec![vec![0x0ddd_0000, reference, 0]]
        );
        assert_eq!(calls(&log, COUNTER_CLEAR).len(), 1);
        assert!(calls(&log, UPDATE_OBJECT_LIGHTING).is_empty());
        assert_eq!(calls(&log, REFERENCE_GET_FORM).len(), 2);
        assert_eq!(
            calls(&log, BHK_WORLD_SET_MOTION),
            vec![vec![0x0ccc_0000, 5, 1, 0, 1]]
        );
        assert_eq!(calls(&log, BSX_FLAGS_FIND), vec![vec![0, 8], vec![0, 0x80]]);

        // Flag 8 set without flag 0x80: the lighting update instead.
        e.register(BSX_FLAGS_FIND, |_, a| ((a[1] == 8) as u32).into_ret());
        e.register(TES_GET_450B80, |_, _| 0x0fff_0000u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_4720, &args![0u32, 0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, CELL_ATTACH_REFERENCE_3D).is_empty());
        assert_eq!(calls(&log, TES_GET_450B80), vec![vec![0]]);
        assert_eq!(
            calls(&log, UPDATE_OBJECT_LIGHTING),
            vec![vec![0x0fff_0000, 0, 0]]
        );
    }

    #[test]
    fn settling_callback_moves_a_kind_0x14_body_transform() {
        let mut e = engine();
        let reference = object(&mut e, REF_SLOTS, |_| 0);
        let body = object(&mut e, BODY_SLOTS, |_| 0);
        let owner = object(&mut e, OWNER_SLOTS, |_| 0);
        chain(&mut e, reference);
        // 004b5a50 gives the owner on every call.
        e.register_double(ENTITY_OWNER_4B5A50, move |_, _| owner.into_ret());
        let entities = e.mem.alloc(4);
        e.register(ARRAY_SIZE, |_, _| 1u32.into_ret());
        e.register_double(ARRAY_ELEMENT, move |_, _| entities.into_ret());
        e.map(0x011c_a000, 0x1000);
        e.set_global(FORM_011CA27C, 0x0123_0000u32);
        e.set_global(FORM_011CA280, 0x0124_0000u32);
        e.register(LOW_SEVEN_BITS, |_, _| KIND_0X14.into_ret());
        e.register_double(ENTITY_OBJECT_4B5A80, move |_, _| body.into_ret());
        e.register(TABLE_TEST_43B300, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0062_4720, &args![0u32, 0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, BHK_WORLD_SET_MOTION).is_empty());
        assert_eq!(calls(&log, BODY_SLOTS + 0xe4), vec![vec![body, 4]]);
        assert_eq!(
            calls(&log, OBJECT_SET_573F00),
            vec![vec![owner, 1], vec![owner, 0]]
        );
        let transform = calls(&log, STACK_TRANSFORM_CONSTRUCT)[0][0];
        let ni_transform = calls(&log, STACK_NI_TRANSFORM_CONSTRUCT)[0][0];
        assert_eq!(calls(&log, BODY_SLOTS + 0xf4), vec![vec![body, transform]]);
        assert_eq!(
            calls(&log, OWNER_SLOTS + 0xb8),
            vec![vec![owner, ni_transform]]
        );
        assert_eq!(calls(&log, COPY_458620), vec![vec![ni_transform + 0x24, 0]]);
    }

    #[test]
    fn transform_conversion_copies_rotation_and_translation() {
        let mut e = engine();
        e.register(GET_436AA0, |_, a| (a[0] + 0x30).into_ret());
        e.call_log = Some(vec![]);
        let back = e.call(0x0062_4ad0, &args![0x1000u32, 0x2000u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, 0x1000);
        assert_eq!(calls(&log, COPY_458620), vec![vec![0x1024, 0x2030]]);
        assert_eq!(calls(&log, ROW_4B4CF0).len(), 3);
    }

    #[test]
    fn rotation_copy_takes_three_rows() {
        let mut e = engine();
        e.register(ROW_4B4CF0, |_, a| (a[0] + 16 * a[1]).into_ret());
        e.call_log = Some(vec![]);
        let back = e.call(0x0062_4b10, &args![0x1000u32, 0x2000u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, 0x1000);
        let point = calls(&log, IDENTITY_6815C0)[0][0];
        assert_eq!(
            calls(&log, POINT_FROM_ROW),
            vec![
                vec![point, 0x2000],
                vec![point, 0x2010],
                vec![point, 0x2020]
            ]
        );
        assert_eq!(
            calls(&log, SET_COLUMN_4769C0),
            vec![
                vec![0x1000, 0, point],
                vec![0x1000, 1, point],
                vec![0x1000, 2, point]
            ]
        );
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    #[test]
    fn fn_00624560_counts_up() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.set_global(COUNTER, 41u32);
        assert_eq!(e.call(0x0062_4560, &args![]).u32(), 42);
        assert_eq!(e.global::<u32>(COUNTER), 42);
    }
}
