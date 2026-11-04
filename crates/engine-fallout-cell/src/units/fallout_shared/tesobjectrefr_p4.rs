//! `fallout shared/tesobjectrefr.cpp` (Xbox PDB source unit), part 4: its functions from `0056ea40` up to
//! (not including) `00576870` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::tesobjectrefr`]; anything public there may be used here.
//!
//! # Where this file stops
//!
//! The first batch holds the 40 open functions from `0056ea40` to
//! `00572230` (the Havok collision builders of a reference, its orientation
//! and bound getters, `Set3D` and its helpers, the model getters and
//! `RemoveWeapon`, `MarkAsPickedUp`). The second batch holds the 40 open
//! functions from `00572270` (`MarkAsDeleted`) to `00573ed0` (lights, the
//! extra-data accessors of the action and dismemberment state, `Activate`
//! and `MoveRefToNewSpace`). The third batch holds the last 40 open
//! functions of the range, from `00573f00` to `00576830` (`Enable`,
//! `Disable`, the location, angle and cell setters, `GetSpace`,
//! `GetWorldSpace`, the magic caster and target accessors, the container
//! helpers and `CleanUpTraps`). The range of this part is finished: the
//! function after `00576830` is `00576870`, the first one of the next part.
//!
//! # Conventions of this part
//!
//! The private constants of the main file are not visible here, so the few
//! small accessors the functions lean on are declared below and called by
//! address exactly as the game calls them (`005d43c0` is `this + 0x44`, the
//! embedded `ExtraDataList`; `007af430` reads the base object at `+0x20`;
//! `0043fcd0` is the reference's 3D, the node held at `+0x14` of its loaded
//! data, or the thread's override). Several callees take a pointer to an
//! x86 stack local; here such locals live in one zeroed block ([`Frame`])
//! whose `ebp`-relative offsets are the disassembly's, so each address in
//! a comment can be checked against the code.
//!
//! A pushed argument the callee never reads (`PUSH 0` before a callee that
//! takes no argument) is still passed, as the callers do.

#[allow(unused_imports)]
use super::tesobjectrefr::*;
#[allow(unused_imports)]
use crate::prelude::*;

/// `ExtraDataList` of a reference: `this + 0x44` (`005d43c0`).
const GET_EXTRA_LIST: u32 = 0x005d_43c0;
/// Base object of a reference, the word at `+0x20` (`007af430`).
const GET_BASE_FORM: u32 = 0x007a_f430;
/// `cFormType`, the byte at `+4` of a form (`00401170`).
const FORM_TYPE: u32 = 0x0040_1170;
/// Parent cell of a reference, the word at `+0x40` (`008d6f30`).
const GET_PARENT_CELL: u32 = 0x008d_6f30;
/// The reference's 3D (`0043fcd0`).
const GET_3D: u32 = 0x0043_fcd0;
/// `IsDeleted` test on a form (flag `0x20`, `00440d80`).
const FORM_IS_DELETED: u32 = 0x0044_0d80;
/// `IsDisabled` test on a form (flag `0x800`, `00440da0`).
const FORM_IS_DISABLED: u32 = 0x0044_0da0;
/// The first dword of its object (`00559450`, used on `NiPointer` slots and
/// list nodes).
const READ_FIRST_DWORD: u32 = 0x0055_9450;
/// The word at `+4` of a list node (`00726070`), also read from the
/// primitive extra for its shape type.
const LIST_NEXT: u32 = 0x0072_6070;
/// Constructor of a `NiPoint3`/`NiMatrix3` local that does nothing and
/// returns `this` (`006815c0`).
const NOTHING_CONSTRUCTOR: u32 = 0x0068_15c0;
/// The word at `+0x18` of a node, its parent (`009611e0`).
const NODE_PARENT: u32 = 0x0096_11e0;
/// The word at `+0x68` of an actor (its process, `008d8520`).
const GET_PROCESS: u32 = 0x008d_8520;
/// `operator new` of the `Ni` allocator (`00aa13e0`).
const NI_OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `operator delete` of the `Ni` allocator (`00aa1460`, pointer and size).
const NI_OPERATOR_DELETE: u32 = 0x00aa_1460;
/// Aligned allocation of the Havok objects (`0056d280`, cdecl).
const HAVOK_ALLOCATE: u32 = 0x0056_d280;
/// `ExtraDataList::GetPrimitive` (`0041fbe0`).
const GET_PRIMITIVE: u32 = 0x0041_fbe0;
/// `TESObjectREFR::GetScale` (`00567400`), the scale in `ST0`.
const GET_SCALE: u32 = 0x0056_7400;
/// `ModelLoader::CancelReference` (`00445570`), on the model loader.
const CANCEL_REFERENCE: u32 = 0x0044_5570;
/// `NiObjectNET::GetExtraData` (`00a5bdd0`).
const NODE_GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
/// `NiObjectNET::AddExtraData_ov2` (`00a5bca0`).
const NODE_ADD_EXTRA_DATA: u32 = 0x00a5_bca0;
/// Update of a node with an `NiUpdateData` (`00a59c60`): the node's virtual
/// at `+0xa4` with the data and 0, then the virtual at `+0xfc` of its parent
/// (`+0x18`) when it has one.
const NODE_UPDATE: u32 = 0x00a5_9c60;
/// `NiMatrix3::FromEulerAnglesXYZ` (`00a59540`).
const MATRIX_FROM_EULER: u32 = 0x00a5_9540;
/// `NiMatrix3::ToEulerAnglesXYZ` (`00a592c0`).
const MATRIX_TO_EULER: u32 = 0x00a5_92c0;
/// `NiUpdateData` constructor (`0043d410`; 0xC bytes in the Xbox PDB):
/// `fTime`, then `bUpdateControllers` and `bParallelUpdate`; the other flags
/// are cleared.
const UPDATE_DATA_CONSTRUCTOR: u32 = 0x0043_d410;
/// Constructor of the local pair (a `0x24`-byte matrix at +0 and the vector
/// after it) the Havok builders keep at `ebp-0xc0` (`00476a80`; it runs the
/// empty constructor `006815c0` on both parts).
const MATRIX_CONSTRUCTOR: u32 = 0x0047_6a80;
/// Constructor of the 16-byte local `00561500` fills (`006240d0`; it runs
/// the empty constructor `006815c0`).
const QUATERNION_CONSTRUCTOR: u32 = 0x0062_40d0;
/// Converts the `NiPoint3` at the source into the four floats at the
/// destination (`004a3e00`, cdecl: destination, source): each component is
/// scaled by `004a3e90` (the factor at `011c582c`), the fourth is 0.0.
const CONVERT_VECTOR: u32 = 0x004a_3e00;
/// Copies the four components of the source into the destination through
/// their accessors (`00561500`, cdecl: destination, source).
const CONVERT_ROTATION: u32 = 0x0056_1500;
/// The model loader pointer (`ModelLoader::` methods take it as `this`).
const GLOBAL_MODEL_LOADER: u32 = 0x011c_3b3c;
/// `PlayerCharacter *`.
const GLOBAL_PLAYER: u32 = 0x011d_ea3c;
/// The object `TES::GetCellPriority` and the nav-mesh helpers use as `this`.
const GLOBAL_TES: u32 = 0x011d_ea10;
/// The object whose `0042ce10` flag says "loading".
const GLOBAL_LOADING_OBJECT: u32 = 0x011d_df38;
/// `ProcessLists`.
const OBJECT_PROCESS_LISTS: u32 = 0x011e_0e80;
/// `TESDataHandler *`.
const GLOBAL_DATA_HANDLER: u32 = 0x011c_3f2c;
/// `TESSaveLoadGame *`.
const GLOBAL_SAVE_LOAD: u32 = 0x011d_e45c;
/// The zero `NiPoint3` the game keeps for "no bound".
const ZERO_VECTOR: u32 = 0x011f_426c;
/// Scratch `NiPoint3` the min-corner getter fills (`011ca3b8`).
const SCRATCH_BOUND_MIN: u32 = 0x011c_a3b8;
/// Scratch `NiPoint3` the max-corner getters fill (`011ca3d4`).
const SCRATCH_BOUND_MAX: u32 = 0x011c_a3d4;

/// One zeroed block standing for the stack frame of a function that keeps
/// many locals and passes some of them by address. Offsets are `ebp`
/// relative, as the disassembly writes them.
struct Frame(u32);

/// Bytes of the block, and where `ebp` sits in it.
const FRAME_SIZE: u32 = 0x300;
const FRAME_EBP: i32 = 0x2c0;

impl Frame {
    /// The address of the local at `ebp + offset`.
    fn at(&self, offset: i32) -> u32 {
        (self.0 as i32 + FRAME_EBP + offset) as u32
    }
}

/// Runs `f` with a zeroed [`Frame`].
fn with_frame<R>(e: &mut Engine, f: impl FnOnce(&mut Engine, &Frame) -> R) -> R {
    e.with_stack(FRAME_SIZE, |e, block| f(e, &Frame(block.addr())))
}

fn extra_list(e: &mut Engine, refr: u32) -> u32 {
    e.call(GET_EXTRA_LIST, &args![refr]).u32()
}

fn base_form(e: &mut Engine, refr: u32) -> u32 {
    e.call(GET_BASE_FORM, &args![refr]).u32()
}

fn form_type(e: &mut Engine, form: u32) -> u32 {
    e.call(FORM_TYPE, &args![form]).u32()
}

fn parent_cell(e: &mut Engine, refr: u32) -> u32 {
    e.call(GET_PARENT_CELL, &args![refr]).u32()
}

fn get_3d(e: &mut Engine, refr: u32) -> u32 {
    e.call(GET_3D, &args![refr]).u32()
}

/// `IsActor` (virtual at +0x100).
fn is_actor(e: &mut Engine, refr: u32) -> bool {
    e.vcall(refr, 0x100, &args![]).bool()
}

/// The reference's node (virtual at +0x1d0).
fn node_of(e: &mut Engine, refr: u32) -> u32 {
    e.vcall(refr, 0x1d0, &args![]).u32()
}

/// Copies `words` dwords from `from` to `to`.
fn copy_words(e: &mut Engine, to: u32, from: u32, words: u32) {
    let bytes = e.mem.bytes(from, words * 4);
    e.mem.write(to, &bytes);
}

/// The model loader (`this` of the `ModelLoader::` methods).
fn model_loader(e: &Engine) -> u32 {
    e.global::<u32>(GLOBAL_MODEL_LOADER)
}

// Translated from 0056ea40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor wrapper of a Havok glue object (the linker placed it here):
/// runs the base constructor `0056e8d0` on `this` and returns `this`.
pub fn fn_0056ea40(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0056_e8d0, &args![this]);
    this
}

// Translated from 0056ea60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSphereShape::_scalar_deleting_destructor_` (Xbox PDB): runs
/// `~bhkSphereShape` (`00ca0660`) and, when bit 0 of `flags` is set, frees
/// the 0x14-byte object with `operator delete` (`00aa1460`). Returns `this`.
pub fn bhk_sphere_shape_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x00ca_0660, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_OPERATOR_DELETE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 0056ea90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor wrapper of a Havok glue object: runs `0056d730` on `this`.
pub fn fn_0056ea90(e: &mut Engine, this: Ptr) {
    e.call(0x0056_d730, &args![this]);
}

// Translated from 0056eab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the Havok collision of a reference from its primitive extra
/// (`ExtraPrimitive`): a box (primitive type 1; type 3 is a box whose second
/// half extent is replaced by the constant at `01013ea4`) or a sphere (type 2)
/// shape, a `bhkRigidBody` positioned on the reference node, a `BSXFlags`
/// extra on the node and the `bhkNiCollisionObject` that ties the body to
/// the node, then adds the body (virtual `+0x9c`) to the world of the parent
/// cell: what `004543c0` gives for it (the object held in the extra data of
/// an interior cell, else the shared one at `011ca0d8`). Does
/// nothing without a primitive or a world.
///
/// Before building anything it calls `00567490(this, 1.0)`, sets the scale
/// of its node (`00440490`, the float at `+0x64`) to 1.0 and updates the node (update data at
/// time 0.0, `00a59c60`).
///
/// The `0xE0`-byte `bhkRigidBodyCinfo` lives at `ebp-0x210`. The body gets
/// its position and rotation from the node and the reference (the body's
/// virtuals `+0xdc` and `+0xe0`, fed by `004a3e00` and `00561500`). The
/// pair the function assembles at `ebp-0xc0` (orientation, position) is
/// never read again; the calls that build it are kept.
///
/// The compiler's exception-unwinding frame is not translated.
pub fn fn_0056eab0(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    let extra = extra_list(e, me);
    let primitive = e.call(GET_PRIMITIVE, &args![extra]).u32();
    let cell = parent_cell(e, me);
    let world = if cell != 0 {
        let cell = parent_cell(e, me);
        e.call(0x0045_43c0, &args![cell]).u32()
    } else {
        0
    };
    if world == 0 || primitive == 0 {
        return;
    }
    with_frame(e, |e, f| {
        let mut shape = 0u32;
        // 0056ded0: constructor of the unused local at ebp-0x70
        e.call(0x0056_ded0, &args![f.at(-0x70)]);
        e.call(NOTHING_CONSTRUCTOR, &args![f.at(-0x80)]);
        // 00567490(this, 1.0f) and node->00440490(1.0f)
        e.call(0x0056_7490, &args![me, 1.0f32]);
        let node = node_of(e, me);
        e.call(0x0044_0490, &args![node, 1.0f32]);
        e.call(
            UPDATE_DATA_CONSTRUCTOR,
            &args![f.at(-0x8c), 0.0f32, 0u32, 0u32],
        );
        let node = node_of(e, me);
        e.call(NODE_UPDATE, &args![node, f.at(-0x8c)]);
        e.call(MATRIX_CONSTRUCTOR, &args![f.at(-0xc0)]);
        let orientation = tes_object_refr_get_orientation(e, this, Ptr::new(f.at(-0xe4))).addr();
        copy_words(e, f.at(-0xc0), orientation, 9);
        let position = e.vcall(me, 0x1f4, &args![]).u32();
        copy_words(e, f.at(-0x9c), position, 3);
        e.mem.set_f32(f.at(-0x90), 1.0);
        match e.call(LIST_NEXT, &args![primitive]).u32() {
            1 => {
                // ExtraPrimitive bounds (00413fc0 copies the three floats at +0x18)
                let half = e.call(0x0041_3fc0, &args![primitive, f.at(-0x10c)]).u32();
                e.call(CONVERT_VECTOR, &args![f.at(-0x80), half]);
                let object = e.call(NI_OPERATOR_NEW, &args![0x14u32]).u32();
                shape = if object != 0 {
                    e.call(0x0056_e610, &args![object, f.at(-0x80)]).u32()
                } else {
                    0
                };
            }
            2 => {
                let object = e.call(NI_OPERATOR_NEW, &args![0x14u32]).u32();
                shape = if object != 0 {
                    let half = e.call(0x0041_3fc0, &args![primitive, f.at(-0x128)]).u32();
                    let x = e.mem.f32(half);
                    // 004a3e90 scales a length into Havok units (cdecl, float in ST0)
                    let radius = e.call(0x004a_3e90, &args![x]).f32();
                    e.call(0x0056_e950, &args![object, radius, 0u32]).u32()
                } else {
                    0
                };
            }
            3 => {
                e.call(0x0041_3fc0, &args![primitive, f.at(-0xf4)]);
                let fourth: f32 = e.global(0x0101_3ea4);
                e.mem.set_f32(f.at(-0xf0), fourth);
                e.call(CONVERT_VECTOR, &args![f.at(-0x80), f.at(-0xf4)]);
                let object = e.call(NI_OPERATOR_NEW, &args![0x14u32]).u32();
                shape = if object != 0 {
                    e.call(0x0056_e610, &args![object, f.at(-0x80)]).u32()
                } else {
                    0
                };
            }
            _ => {}
        }
        // the bhkRigidBodyCinfo
        let cinfo = f.at(-0x210);
        e.call(0x00c8_f510, &args![cinfo]);
        fn_0056f110(e, Ptr::new(cinfo), shape);
        let extra = extra_list(e, me);
        let collision_data = e.call(0x0042_11a0, &args![extra]).u32();
        let layer = if collision_data != 0 {
            e.call(READ_FIRST_DWORD, &args![collision_data]).u32()
        } else {
            3
        };
        fn_0056f0f0(e, Ptr::new(cinfo), 0x3_0000 | layer);
        e.call(0x0056_d360, &args![cinfo, 5u32]);
        // BSXFlags extra on the node
        let object = e.call(NI_OPERATOR_NEW, &args![0x10u32]).u32();
        let flags = if object != 0 {
            e.call(0x00c4_2f70, &args![object]).u32()
        } else {
            0
        };
        e.call(0x0051_9230, &args![flags, 2u32, 1u32]);
        let node = node_of(e, me);
        e.call(NODE_ADD_EXTRA_DATA, &args![node, flags]);
        // the body
        let object = e.call(NI_OPERATOR_NEW, &args![0x1cu32]).u32();
        let body = if object != 0 {
            e.call(0x0056_d380, &args![object, cinfo]).u32()
        } else {
            0
        };
        e.call(NOTHING_CONSTRUCTOR, &args![f.at(-0x250)]);
        e.call(QUATERNION_CONSTRUCTOR, &args![f.at(-0x260)]);
        e.call(NOTHING_CONSTRUCTOR, &args![f.at(-0x270)]);
        let node = node_of(e, me);
        e.call(0x0056_d230, &args![node, f.at(-0x270)]);
        let position = e.vcall(me, 0x1f4, &args![]).u32();
        e.call(CONVERT_VECTOR, &args![f.at(-0x250), position]);
        e.call(CONVERT_ROTATION, &args![f.at(-0x260), f.at(-0x270)]);
        e.vcall(body, 0xdc, &args![f.at(-0x250)]);
        e.vcall(body, 0xe0, &args![f.at(-0x260)]);
        let node = node_of(e, me);
        e.call(0x00c8_5c10, &args![body, node, 0u32]);
        e.vcall(body, 0x9c, &args![world]);
        // the collision object tying the body to the node
        let object = e.call(NI_OPERATOR_NEW, &args![0x14u32]).u32();
        let collision_object = if object != 0 {
            let node = node_of(e, me);
            e.call(0x0056_d580, &args![object, node]).u32()
        } else {
            0
        };
        e.call(0x00c6_b980, &args![collision_object, body]);
        e.call(0x0056_d730, &args![cinfo]);
    });
}

// Translated from 0056f0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the collision filter info of a `bhkRigidBodyCinfo` (`this`): the
/// word at `+0` and its `hk` copy at `+0x20`.
pub fn fn_0056f0f0(e: &mut Engine, this: Ptr, filter_info: u32) {
    e.mem.set_u32(this.addr(), filter_info);
    e.mem.set_u32(this.addr() + 0x20, filter_info);
}

// Translated from 0056f110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the shape of a `bhkRigidBodyCinfo` (`this`): the word at `+4`
/// gets what `004ae750` returns for `shape` (0 for a null shape, else what
/// `00620b80` finds in it), and the `hk` copy at `+0x24` the same.
pub fn fn_0056f110(e: &mut Engine, this: Ptr, shape: u32) {
    let havok_shape = e.call(0x004a_e750, &args![shape]).u32();
    e.mem.set_u32(this.addr() + 4, havok_shape);
    let copy = e.mem.u32(this.addr() + 4);
    e.mem.set_u32(this.addr() + 0x24, copy);
}

// Translated from 0056f140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the Havok collision of a reference whose primitive extra describes
/// a box and registers it (the bounding-volume variant of `0056eab0`): the
/// box shape wrapped in a `0x1c`-byte bounding-volume shape (`00c9d990`
/// over the box and a `0x10`-byte companion built by `0056f690`), a body
/// with the fixed filter `0x50015`, `BSXFlags` on the node and the
/// collision object; the body is added to the world of the parent cell
/// through `0056d2c0`. Does nothing without a world. It starts as `0056eab0`
/// does (`00567490(this, 1.0)`, node scale 1.0, node update).
///
/// The `bhkRigidBodyCinfo` lives at `ebp-0x200`; its position (`+0x30`) and
/// rotation (`+0x40`) are set through `0056d300` and `0056d320`.
///
/// The compiler's exception-unwinding frame is not translated.
pub fn fn_0056f140(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    let extra = extra_list(e, me);
    let primitive = e.call(GET_PRIMITIVE, &args![extra]).u32();
    let cell = parent_cell(e, me);
    let world = if cell != 0 {
        let cell = parent_cell(e, me);
        e.call(0x0045_43c0, &args![cell]).u32()
    } else {
        0
    };
    if world == 0 {
        return;
    }
    with_frame(e, |e, f| {
        e.call(0x0056_ded0, &args![f.at(-0x70)]);
        e.call(NOTHING_CONSTRUCTOR, &args![f.at(-0x80)]);
        e.call(0x0056_7490, &args![me, 1.0f32]);
        let node = node_of(e, me);
        e.call(0x0044_0490, &args![node, 1.0f32]);
        e.call(
            UPDATE_DATA_CONSTRUCTOR,
            &args![f.at(-0x8c), 0.0f32, 0u32, 0u32],
        );
        let node = node_of(e, me);
        e.call(NODE_UPDATE, &args![node, f.at(-0x8c)]);
        e.call(MATRIX_CONSTRUCTOR, &args![f.at(-0xc0)]);
        let orientation = tes_object_refr_get_orientation(e, this, Ptr::new(f.at(-0xe4))).addr();
        copy_words(e, f.at(-0xc0), orientation, 9);
        let position = e.vcall(me, 0x1f4, &args![]).u32();
        copy_words(e, f.at(-0x9c), position, 3);
        e.mem.set_f32(f.at(-0x90), 1.0);
        // the dead store of 0x15 at ebp-0xe8 is not kept
        let half = e.call(0x0041_3fc0, &args![primitive, f.at(-0xf4)]).u32();
        e.call(CONVERT_VECTOR, &args![f.at(-0x80), half]);
        // the box
        let object = e.call(HAVOK_ALLOCATE, &args![0x30u32]).u32();
        let box_shape = if object != 0 {
            let radius: f32 = e.global(0x011b_153c);
            e.call(0x00c9_ddb0, &args![object, f.at(-0x80), radius])
                .u32()
        } else {
            0
        };
        // its 0x10-byte companion
        let object = e.call(HAVOK_ALLOCATE, &args![0x10u32]).u32();
        let companion = if object != 0 {
            fn_0056f690(e, Ptr::new(object)).addr()
        } else {
            0
        };
        // the wrapping shape
        let object = e.call(HAVOK_ALLOCATE, &args![0x1cu32]).u32();
        let wrapped = if object != 0 {
            e.call(0x00c9_d990, &args![object, box_shape, companion])
                .u32()
        } else {
            0
        };
        let cinfo = f.at(-0x200);
        e.call(0x00c8_f510, &args![cinfo]);
        e.mem.set_u32(cinfo + 4, wrapped);
        e.call(0x0056_d360, &args![cinfo, 5u32]);
        e.call(NOTHING_CONSTRUCTOR, &args![f.at(-0x210)]);
        e.call(NOTHING_CONSTRUCTOR, &args![f.at(-0x220)]);
        e.call(QUATERNION_CONSTRUCTOR, &args![f.at(-0x230)]);
        let node = node_of(e, me);
        e.call(0x0056_d230, &args![node, f.at(-0x220)]);
        let position = e.vcall(me, 0x1f4, &args![]).u32();
        e.call(CONVERT_VECTOR, &args![f.at(-0x210), position]);
        e.call(CONVERT_ROTATION, &args![f.at(-0x230), f.at(-0x220)]);
        e.call(0x0056_d300, &args![cinfo, f.at(-0x210)]);
        e.call(0x0056_d320, &args![cinfo, f.at(-0x230)]);
        // the collision filter info 0x50000 | 0x15
        e.mem.set_u32(cinfo, 0x5_0000 | 0x15);
        // the body
        let object = e.call(NI_OPERATOR_NEW, &args![0x1cu32]).u32();
        let body = if object != 0 {
            e.call(0x0056_d380, &args![object, cinfo]).u32()
        } else {
            0
        };
        // the collision object
        let object = e.call(NI_OPERATOR_NEW, &args![0x14u32]).u32();
        let collision_object = if object != 0 {
            let node = node_of(e, me);
            e.call(0x0056_d580, &args![object, node]).u32()
        } else {
            0
        };
        // BSXFlags on the node
        let object = e.call(NI_OPERATOR_NEW, &args![0x10u32]).u32();
        let flags = if object != 0 {
            e.call(0x00c4_2f70, &args![object]).u32()
        } else {
            0
        };
        e.call(0x0051_9230, &args![flags, 2u32, 1u32]);
        let node = node_of(e, me);
        e.call(NODE_ADD_EXTRA_DATA, &args![node, flags]);
        e.call(0x00c6_b980, &args![collision_object, body]);
        let node = node_of(e, me);
        e.call(0x00c8_5c10, &args![body, node, 0u32]);
        e.call(0x0056_d2c0, &args![world, body]);
        e.call(0x0056_d730, &args![cinfo]);
    });
}

// Translated from 0056f690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 0x10-byte Havok companion object of `0056f140`: runs
/// the base constructor `0056d610` and sets its own vtable (`01030dc0`).
/// Returns `this`.
pub fn fn_0056f690(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0056_d610, &args![this]);
    e.mem.set_u32(this.addr(), 0x0103_0dc0);
    this
}

// Translated from 0056f6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Asks the base object of the reference, when it has one, its virtual at
/// `+0xa0` and returns the answer as a bool (false without a base object).
pub fn fn_0056f6b0(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    if base_form(e, this.addr()) == 0 {
        return false;
    }
    let base = base_form(e, this.addr());
    e.vcall(base, 0xa0, &args![]).bool()
}

// Translated from 0056f700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Brings the 3D of a reference up to date after a load or enable: a
/// reference that has no 3D (or `005651e0` says is excluded), is not
/// already known to the model loader (`00445750`), is not deleted and not
/// held back by its base object (an `0x1e`-type base the loader flag
/// `00444ed0` says is being saved is exempt) is queued with the model loader
/// (`00444850`) at the priority its cell gets from `TES::GetCellPriority`
/// (`00458be0`) when it is enabled; a disabled one gets its script action
/// list initialised and flag `0x1000` set unless the loading object says
/// loading. Then, with a 3D, the node is updated with update data at time 0.0 (`00a59c60`), an actor's
/// dismembered limbs are rebuilt, the 3D virtual at `+0x1c4` runs and the
/// saved Havok data is restored; without a 3D, a primitive extra makes it
/// run the virtual at `+0x1c4`.
pub fn fn_0056f700(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    let mut saving_other = false;
    if e.call(0x0044_4ed0, &args![me]).bool() && base_form(e, me) != 0 {
        let base = base_form(e, me);
        if form_type(e, base) != 0x1e {
            saving_other = true;
        }
    }
    let node = get_3d(e, me);
    if (node == 0 || e.call(0x0056_51e0, &args![me]).bool())
        && !e.call(0x0044_5750, &args![model_loader(e), me]).bool()
        && !e.call(FORM_IS_DELETED, &args![me]).bool()
        && !saving_other
    {
        if !e.call(FORM_IS_DISABLED, &args![me]).bool() {
            if is_actor(e, me) {
                let actor = me;
                if actor != 0 {
                    e.vcall(actor, 0x240, &args![]);
                    e.call(0x0096_f400, &args![OBJECT_PROCESS_LISTS, actor]);
                }
            }
            let cell = parent_cell(e, me);
            let tes = e.global::<u32>(GLOBAL_TES);
            // 0056f7fb: `00458be0` takes two words (RET 8); the pushed 1 is the last word
            // of `00444850` (RET 0xc).
            let priority = e.call(0x0045_8be0, &args![tes, cell, 0u32]).u32();
            e.call(0x0044_4850, &args![model_loader(e), me, priority, 1u32]);
        } else {
            let loading = e.global::<u32>(GLOBAL_LOADING_OBJECT);
            if !e.call(0x0042_ce10, &args![loading]).bool() {
                let extra = extra_list(e, me);
                e.call(0x005a_c190, &args![me, extra]);
                let extra = extra_list(e, me);
                e.call(0x005a_c750, &args![me, extra, 0x1000u32]);
            }
        }
    }
    if get_3d(e, me) != 0 {
        // the NiUpdateData local at ebp-0x18 (time 0.0)
        e.with_stack(0xc, |e, point| {
            e.call(UPDATE_DATA_CONSTRUCTOR, &args![point, 0.0f32, 0u32, 0u32]);
            let node = get_3d(e, me);
            e.call(NODE_UPDATE, &args![node, point]);
            let actor = is_actor(e, me);
            if actor {
                e.call(0x008b_6ae0, &args![me]);
                let extra = extra_list(e, me);
                e.call(0x0042_2bc0, &args![extra, me]);
            }
            e.vcall(me, 0x1c4, &args![]);
            if actor {
                e.call(0x008a_1a70, &args![me]);
                e.call(0x008b_65f0, &args![me]);
            } else {
                let extra = extra_list(e, me);
                e.call(0x0042_2bc0, &args![extra, me]);
            }
        });
    } else {
        let extra = extra_list(e, me);
        if e.call(GET_PRIMITIVE, &args![extra]).u32() != 0 {
            e.vcall(me, 0x1c4, &args![]);
        }
    }
}

// Translated from 0056f930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::FindReferenceFor3D` (Xbox PDB), cdecl: the reference a
/// node (or one of its ancestors) belongs to. Walks up the parents
/// (`009611e0`): the player's first-person root (`00950bb0(player, 1)`)
/// answers with the player; a node whose owner (virtual at `+0x10`) has a
/// reference (`009ad610`) answers with it, or, when that reference's base
/// object is of form type `0x29`, with what the recursion on the node's
/// parent finds if it finds one. Returns 0 when nothing owns the node.
pub fn tes_object_refr_find_reference_for_3d(e: &mut Engine, node: u32) -> u32 {
    let mut found = 0u32;
    let mut first_person_root = 0u32;
    let player = e.global::<u32>(GLOBAL_PLAYER);
    if player != 0 {
        first_person_root = e.call(0x0095_0bb0, &args![player, 1u32]).u32();
    }
    let mut current = node;
    while current != 0 {
        if current == first_person_root {
            return e.global::<u32>(GLOBAL_PLAYER);
        }
        let owner = e.vcall(current, 0x10, &args![]).u32();
        if owner != 0 {
            found = e.call(0x009a_d610, &args![owner]).u32();
            if found != 0 {
                if base_form(e, found) != 0 {
                    let base = base_form(e, found);
                    if form_type(e, base) == 0x29 {
                        let parent = e.call(NODE_PARENT, &args![current]).u32();
                        let deeper = tes_object_refr_find_reference_for_3d(e, parent);
                        if deeper != 0 {
                            found = deeper;
                        }
                    }
                }
                return found;
            }
        }
        current = e.call(NODE_PARENT, &args![current]).u32();
    }
    found
}

// Translated from 0056fa00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetOrientation` (Xbox PDB): writes the 3x3 rotation
/// matrix (nine floats) of the reference's angles (`+0x24`, `+0x28`,
/// `+0x2c`) to `out` through `NiMatrix3::FromEulerAnglesXYZ`; an actor uses
/// only its heading (`x = y = 0`). Returns `out`.
pub fn tes_object_refr_get_orientation(e: &mut Engine, this: Ptr<TESObjectREFR>, out: Ptr) -> Ptr {
    let me = this.addr();
    e.with_stack(0x24, |e, matrix| {
        e.call(NOTHING_CONSTRUCTOR, &args![matrix]);
        // OBJ_REFR::Angle (Xbox PDB) +0x24 / +0x28 / +0x2c
        let angle_z = e.mem.f32(me + 0x2c);
        if is_actor(e, me) {
            e.call(MATRIX_FROM_EULER, &args![matrix, 0.0f32, 0.0f32, angle_z]);
        } else {
            let angle_x = e.mem.f32(me + 0x24);
            let angle_y = e.mem.f32(me + 0x28);
            e.call(MATRIX_FROM_EULER, &args![matrix, angle_x, angle_y, angle_z]);
        }
        copy_words(e, out.addr(), matrix.addr(), 9);
    });
    out
}

// Translated from 0056fa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetOrientation` (Xbox PDB): the rotation matrix, passed
/// by value (nine words), is converted back into the reference's three
/// angles (`+0x24`, `+0x28`, `+0x2c`) by `NiMatrix3::ToEulerAnglesXYZ`.
#[allow(clippy::too_many_arguments)]
pub fn tes_object_refr_set_orientation(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    m0: u32,
    m1: u32,
    m2: u32,
    m3: u32,
    m4: u32,
    m5: u32,
    m6: u32,
    m7: u32,
    m8: u32,
) {
    let me = this.addr();
    e.with_stack(0x24, |e, matrix| {
        for (i, word) in [m0, m1, m2, m3, m4, m5, m6, m7, m8].into_iter().enumerate() {
            e.mem.set_u32(matrix.addr() + 4 * i as u32, word);
        }
        e.call(
            MATRIX_TO_EULER,
            &args![matrix, me + 0x24, me + 0x28, me + 0x2c],
        );
    });
}

// Translated from 0056fac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::MultipleMatrixByRace` (Xbox PDB): copies the matrix
/// `matrix` (nine words) to `out` and returns `out`. The race-dependent
/// part is compiled out; the only thing left of it is the read of the form
/// type of the base object (`00401170`, `+0x20`).
pub fn tes_object_refr_multiple_matrix_by_race(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    out: Ptr,
    matrix: Ptr,
) -> Ptr {
    // OBJ_REFR::pObjectReference (Xbox PDB) +0x20
    let base = e.mem.u32(this.addr() + 0x20);
    form_type(e, base);
    copy_words(e, out.addr(), matrix.addr(), 9);
    out
}

/// The bound corner of a 3D node: if `node` has a bound extra
/// (`NiObjectNET::GetExtraData` with the name at `012031e4`, read through
/// `0050ea90`) the corner `corner` computes into the scratch vector
/// `scratch`, which is copied to `out`; otherwise the zero vector.
fn bound_corner(e: &mut Engine, out: u32, node: u32, corner: u32, scratch: u32) -> u32 {
    if node != 0 {
        let name = e.call(0x0050_ea90, &args![]).u32();
        let bound = e.call(NODE_GET_EXTRA_DATA, &args![node, name]).u32();
        if bound != 0 {
            e.call(corner, &args![bound, scratch]);
            copy_words(e, out, scratch, 3);
            return out;
        }
    }
    copy_words(e, out, ZERO_VECTOR, 3);
    out
}

// Translated from 0056fb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the minimum corner of the reference's 3D bound to `out` (center
/// minus extents, `0050ea10`), the zero vector when it has no 3D or no
/// bound extra. Returns `out`.
pub fn fn_0056fb10(e: &mut Engine, this: Ptr<TESObjectREFR>, out: Ptr) -> Ptr {
    let node = get_3d(e, this.addr());
    Ptr::new(bound_corner(
        e,
        out.addr(),
        node,
        0x0050_ea10,
        SCRATCH_BOUND_MIN,
    ))
}

// Translated from 0056fba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the maximum corner of the reference's 3D bound to `out` (center
/// plus extents, `0050ea50`), the zero vector when it has no 3D or no bound
/// extra. Returns `out`.
pub fn fn_0056fba0(e: &mut Engine, this: Ptr<TESObjectREFR>, out: Ptr) -> Ptr {
    let node = get_3d(e, this.addr());
    Ptr::new(bound_corner(
        e,
        out.addr(),
        node,
        0x0050_ea50,
        SCRATCH_BOUND_MAX,
    ))
}

// Translated from 0056fc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetBoundMax` (Xbox PDB), cdecl: the maximum corner of
/// the bound of `node` written to `out` (as `0056fba0` does for the
/// reference's 3D); the zero vector for a null node or one without a bound
/// extra. Returns `out`.
pub fn tes_object_refr_get_bound_max(e: &mut Engine, out: Ptr, node: u32) -> Ptr {
    Ptr::new(bound_corner(
        e,
        out.addr(),
        node,
        0x0050_ea50,
        SCRATCH_BOUND_MAX,
    ))
}

/// The item of a list node (`006815c0` returns the address of the node's
/// item slot).
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(NOTHING_CONSTRUCTOR, &args![node]).u32();
    e.mem.u32(slot)
}

/// Base object forms that mark rooms and portals: the base object of a
/// reference is compared with the words at these globals.
const GLOBAL_ROOM_MARKER_BASE: u32 = 0x011c_a238;
const GLOBAL_PORTAL_MARKER_BASE: u32 = 0x011c_a23c;
const GLOBAL_MULTIBOUND_BASE: u32 = 0x011c_a234;

// Translated from 0056fcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes a reference out of everything that tracks its 3D, before the 3D is
/// dropped (`Set3D` with a null node calls it). Does nothing when the
/// reference has no node (virtual at `+0x1d0`). Otherwise:
///
/// * a base object of type `0x23` decrements the counter at `+0xb8` of the
///   object at `011dea10` (`0045ce60`);
/// * `00565670(this)`, `RemoveFromAllWater(this, 0)` (`0057b520`), and for
///   each item of the extra list `00420770` returns, when both this
///   reference and the item have a multibound room (`005699b0`), the room's
///   animation data is added to the item room's list (`0057c5d0`);
/// * a portal marker (base object equal to the word at `011ca23c`) is taken
///   out of the two rooms it joins (`00420bc0`, `00420da0`);
/// * a type `0x1e` base object clears the water-light reference of every lit
///   water reference (`0041f810`, `0041f840`) and, for the ones with a
///   process of type 0xd, updates the property list on its node;
/// * a type `0xe` base object in a cell tells the acoustic space listener
///   (`0061f890`);
/// * with a parent cell the reference leaves the cell's emittance and
///   lighting lists (`00545360`, `005454f0`, `005456e0`) and the room,
///   multibound or portal data of the cell is updated.
///
/// The compiler's exception-unwinding frame is not translated.
pub fn fn_0056fcb0(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    if node_of(e, me) == 0 {
        return;
    }
    let tes = e.global::<u32>(GLOBAL_TES);
    if base_form(e, me) != 0 {
        let base = base_form(e, me);
        if form_type(e, base) == 0x23 {
            e.call(0x0045_ce60, &args![tes]);
        }
    }
    e.call(0x0056_5670, &args![me]);
    e.call(0x0057_b520, &args![me, 0u32]);
    let extra = extra_list(e, me);
    let mut rooms = e.call(0x0042_0770, &args![extra]).u32();
    while rooms != 0 && node_item(e, rooms) != 0 {
        let other = node_item(e, rooms);
        if e.call(0x0056_99b0, &args![me]).u32() != 0
            && e.call(0x0056_99b0, &args![other]).u32() != 0
        {
            with_frame(e, |e, f| {
                let own_room = e.call(0x0056_99b0, &args![me]).u32();
                let data = e.call(0x0066_29f0, &args![own_room]).u32();
                e.call(0x0063_3c90, &args![f.at(-0x60), data]);
                let other_room = e.call(0x0056_99b0, &args![other]).u32();
                let list = e.call(0x0045_c650, &args![other_room]).u32();
                e.call(0x0057_c5d0, &args![list, f.at(-0x64), f.at(-0x60)]);
                e.call(0x0045_cec0, &args![f.at(-0x64)]);
                e.call(0x0045_cec0, &args![f.at(-0x60)]);
            });
        }
        rooms = e.call(LIST_NEXT, &args![rooms]).u32();
    }
    // the portal marker
    if base_form(e, me) == e.global::<u32>(GLOBAL_PORTAL_MARKER_BASE) {
        let extra = extra_list(e, me);
        let _portal = e.call(0x0042_0dd0, &args![extra]).u32();
        if base_form(e, me) == e.global::<u32>(GLOBAL_PORTAL_MARKER_BASE) {
            for side in 0..2u32 {
                let extra = extra_list(e, me);
                let room = e.call(0x0042_0bc0, &args![extra, side]).u32();
                if room != 0 {
                    let room_extra = extra_list(e, room);
                    e.call(0x0042_0da0, &args![room_extra, me]);
                }
            }
        }
    }
    // a type 0x1e base object: the water lights
    if base_form(e, me) != 0 {
        let base = base_form(e, me);
        if form_type(e, base) == 0x1e {
            let extra = extra_list(e, me);
            let mut lit = e.call(0x0041_f810, &args![extra]).u32();
            let stored = e.call(0x0057_25f0, &args![me]).u32();
            while lit != 0 && node_item(e, lit) != 0 {
                let item = node_item(e, lit);
                let item_extra = extra_list(e, item);
                e.call(0x0041_f840, &args![item_extra, me, 0u32]);
                if stored != 0 && e.call(READ_FIRST_DWORD, &args![stored]).u32() != 0 {
                    let object = node_item(e, lit);
                    let node = node_of(e, object);
                    if node != 0 && e.call(0x0070_ec90, &args![tes]).u32() != 0 {
                        let item = node_item(e, lit);
                        let manager = e.call(0x0070_ec90, &args![tes]).u32();
                        let water_node = e.call(0x004e_8030, &args![manager, item]).u32();
                        let scheduler = e.call(0x0040_30b0, &args![water_node]).u32();
                        if scheduler != 0 && e.call(GET_PROCESS, &args![scheduler]).u32() == 0xd {
                            let property = e.call(0x00a5_9d30, &args![water_node, 3u32]).u32();
                            let value = e.call(READ_FIRST_DWORD, &args![stored]).u32();
                            with_frame(e, |e, f| {
                                e.mem.set_u32(f.at(-0x68), value);
                                e.call(0x0057_c730, &args![property + 0x128, f.at(-0x68)]);
                            });
                        }
                    }
                }
                lit = e.call(LIST_NEXT, &args![lit]).u32();
            }
        }
    }
    // a type 0xe base object in a cell
    if base_form(e, me) != 0 {
        let base = base_form(e, me);
        if form_type(e, base) == 0xe && parent_cell(e, me) != 0 {
            let cell = parent_cell(e, me);
            let world = e.call(0x0045_43c0, &args![cell]).u32();
            let listener = e.call(0x0045_cd60, &args![world]).u32();
            e.call(0x0061_f890, &args![listener, me, 1u32]);
        }
    }
    // TESObjectREFR::pParentCell (Xbox PDB) +0x40
    let cell = e.mem.u32(me + 0x40);
    if cell == 0 {
        return;
    }
    if base_form(e, me) != 0 {
        let base = base_form(e, me);
        if form_type(e, base) == 0x23 {
            let manager = e.call(0x0070_ec90, &args![tes]).u32();
            e.call(0x004e_52f0, &args![manager, me]);
        }
    }
    e.call(0x0054_5360, &args![cell, me]);
    let cell = e.mem.u32(me + 0x40);
    e.call(0x0054_54f0, &args![cell, me]);
    let cell = e.mem.u32(me + 0x40);
    let lighting = if cell != 0 && e.call(0x0045_43c0, &args![cell]).u32() != 0 {
        let world = e.call(0x0045_43c0, &args![cell]).u32();
        e.call(0x0059_bb30, &args![world]).u32()
    } else {
        0
    };
    if lighting != 0 {
        e.call(0x0063_1370, &args![lighting, me, 0u32, 1u32]);
    }
    if base_form(e, me) != 0 {
        let base = base_form(e, me);
        if form_type(e, base) == 0x23 {
            let cell = e.mem.u32(me + 0x40);
            e.call(0x0054_56e0, &args![cell, me]);
            e.call(0x0063_12d0, &args![lighting, me]);
        }
    }
    let cell = e.mem.u32(me + 0x40);
    let registry = e.call(0x009d_9f20, &args![cell]).u32();
    if registry == 0 || base_form(e, me) == 0 {
        return;
    }
    let base = base_form(e, me);
    if base == e.global::<u32>(GLOBAL_ROOM_MARKER_BASE) {
        let extra = extra_list(e, me);
        let room = e.call(0x0042_0ed0, &args![extra]).u32();
        if room != 0 {
            e.call(0x00c5_b060, &args![registry, room]);
        }
    } else if base_form(e, me) == e.global::<u32>(GLOBAL_MULTIBOUND_BASE) {
        let extra = extra_list(e, me);
        let multibound = e.call(0x0042_2120, &args![extra]).u32();
        if multibound != 0 {
            let extra = extra_list(e, me);
            let linked = e.call(0x0042_1e10, &args![extra]).u32();
            if linked != 0 {
                let extra = extra_list(e, me);
                let linked = e.call(0x0042_1e10, &args![extra]).u32();
                let linked_extra = extra_list(e, linked);
                let room = e.call(0x0042_0ed0, &args![linked_extra]).u32();
                if room != 0 {
                    e.call(0x00c3_9ad0, &args![room, multibound]);
                } else {
                    e.call(0x00c5_aec0, &args![registry, multibound]);
                }
            } else {
                e.call(0x00c5_aec0, &args![registry, multibound]);
            }
        }
    } else if base_form(e, me) == e.global::<u32>(GLOBAL_PORTAL_MARKER_BASE) {
        let extra = extra_list(e, me);
        let portal = e.call(0x0042_0dd0, &args![extra]).u32();
        if portal != 0 {
            for side in 0..2u32 {
                let extra = extra_list(e, me);
                let joined = e.call(0x0042_0410, &args![extra]).u32();
                let joined_item = if joined != 0 {
                    e.mem.u32(joined + 4 * side)
                } else {
                    0
                };
                if joined_item != 0 {
                    if e.call(0x0056_99b0, &args![joined_item]).u32() != 0 {
                        let item = e.mem.u32(joined + 4 * side);
                        let room = e.call(0x0056_99b0, &args![item]).u32();
                        let list = e.call(0x006a_b360, &args![room]).u32();
                        with_frame(e, |e, f| {
                            e.mem.set_u32(f.at(-0x54), portal);
                            e.call(0x0057_c730, &args![list, f.at(-0x54)]);
                        });
                        if side == 0 {
                            e.call(0x0097_07f0, &args![portal, 0u32]);
                        } else {
                            e.call(0x0042_0ba0, &args![portal, 0u32]);
                        }
                    }
                } else {
                    e.call(0x00c5_b190, &args![registry, portal]);
                }
            }
        }
    }
}

// Translated from 005702e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::Set3D` (Xbox PDB, virtual at `+0x1cc`): replaces the 3D
/// of the reference with `new_node` (null to unload it). `log_unload` makes
/// the unload of an actor whose process type is at most 1 print the "AI:
/// Unloading art" message.
///
/// * The same node as the current one: nothing, except that "replacing" no
///   3D with none cancels the model-loader request and runs
///   `00579ac0(this, 0)`.
/// * The model-loader request is cancelled when the current 3D has no
///   parent. When unloading an actor's 3D whose process type is at most 1,
///   the process is told to drop its art (`0092b7f0` and the virtuals
///   `+0x79c`, `+0x794`, `+0x7a4`). The loaded data's word at `+4` is
///   cleared; when unloading, the primitive extra's slots are cleared
///   (`004a4c50`) and the reference is detached from everything that tracks
///   its 3D (`fn_0056fcb0`).
/// * A reference with a non-zero object health (`0041b6b0`, when `00452370`
///   holds) asks its base object for the destruction form
///   (`GetDestructionForm`, `00475400`): unloading runs `004778a0` on it,
///   getting a 3D preloads its replacement models (`PreloadReplacementModels`,
///   `00477780`).
/// * The old node (held in a `NiPointer` local) leaves its parent (virtual
///   `+0xe8`, or `0087ac60` on the task queue) and the shadow scene node;
///   its local translation (`+0x58`), rotation (`+0x34`) and scale (`+0x64`)
///   are remembered. Unloading also saves Havok data and animation for the
///   changes the loading object reports (`0084a6d0`), kills the light and
///   the animation, removes the weapon of an actor
///   ([`tes_object_refr_remove_weapon`]), clears the ragdoll data of an
///   actor, stops the moving sounds, queues the node with the garbage
///   collector (`00868660`, or `00c45830` for the player or a non-actor) and
///   releases it.
/// * The new node gets the remembered transform (`00440460`, `0043fa80`,
///   `00440490`), is attached to the old parent (virtual `+0xdc`) and
///   updated (`00a5a040`, `00a59c60` with update data at time 0.0).
/// * The nav-mesh obstacle manager is told of the change (virtual `+0x15c`,
///   `006c0c30`, `006c0c80`).
///
/// A reference whose base object has form type `0x25` is left alone when
/// it already has a 3D and gets one. The compiler's exception-unwinding
/// frame is not translated.
pub fn tes_object_refr_set_3d(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    new_node: u32,
    log_unload: bool,
) {
    let me = this.addr();
    if get_3d(e, me) == new_node {
        if new_node == 0 {
            let loader = model_loader(e);
            e.call(CANCEL_REFERENCE, &args![loader, me]);
            e.call(0x0057_9ac0, &args![me, 0u32]);
        }
        return;
    }
    let current = get_3d(e, me);
    if current != 0 {
        let current = get_3d(e, me);
        if e.call(NODE_PARENT, &args![current]).u32() == 0 {
            let loader = model_loader(e);
            e.call(CANCEL_REFERENCE, &args![loader, me]);
        }
    }
    // TESObjectREFR::pLoadedData (Xbox PDB) +0x64, LOADED_REF_DATA's 3D at +0x14
    let loaded = e.mem.u32(me + 0x64);
    if loaded != 0
        && e.call(READ_FIRST_DWORD, &args![loaded + 0x14]).u32() != 0
        && new_node == 0
        && is_actor(e, me)
        && e.call(GET_PROCESS, &args![me]).u32() != 0
        && (e.call(0x0093_1850, &args![me]).u32() as i32) <= 1
    {
        if log_unload {
            let node = e.call(READ_FIRST_DWORD, &args![loaded + 0x14]).u32();
            let name = e.call(0x0041_3f40, &args![node]).u32();
            let name = e.call(0x0043_b1b0, &args![name]).u32();
            e.call(0x005b_5e40, &args![0x0103_0df8u32, name]);
        }
        let process = e.call(GET_PROCESS, &args![me]).u32();
        e.vcall(process, 0x79c, &args![0u32]);
        e.vcall(process, 0x794, &args![0u32]);
        e.vcall(process, 0x7a4, &args![0u32]);
        e.call(0x0092_b7f0, &args![process]);
    }
    let loaded = e.mem.u32(me + 0x64);
    if loaded != 0 {
        e.mem.set_u32(loaded + 4, 0);
    }
    let extra = extra_list(e, me);
    let primitive = e.call(GET_PRIMITIVE, &args![extra]).u32();
    if new_node == 0 && primitive != 0 {
        e.call(0x004a_4c50, &args![primitive]);
    }
    if new_node == 0 {
        fn_0056fcb0(e, this);
    }
    with_frame(e, |e, f| {
        let data = e.call(0x0089_1170, &args![me]).u32();
        // OBJ_REFR::Location (Xbox PDB) +0x10 of the data
        copy_words(e, f.at(-0x4c), data + 0x10, 3);
        e.call(NOTHING_CONSTRUCTOR, &args![f.at(-0x3c)]);
        let mut scale = e.call(GET_SCALE, &args![me]).f32();
        let mut parent = 0u32;
        let mut actor = 0u32;
        if is_actor(e, me) {
            actor = me;
        }
        if get_3d(e, me) != 0 && new_node != 0 {
            let base = e.mem.u32(me + 0x20);
            if base != 0 && form_type(e, base) == 0x25 {
                return;
            }
        }
        if e.call(0x0045_2370, &args![me]).bool() {
            let extra = extra_list(e, me);
            let health = e.call(0x0041_b6b0, &args![extra]).f64();
            if health != e.global::<f64>(0x0101_2060) {
                if get_3d(e, me) != 0 && new_node == 0 {
                    let base = base_form(e, me);
                    let destruction = e.call(0x0047_5400, &args![base]).u32();
                    e.call(0x0047_78a0, &args![destruction]);
                } else if new_node != 0 && get_3d(e, me) == 0 {
                    let first = base_form(e, me);
                    let second = base_form(e, me);
                    let destruction = e.call(0x0047_5400, &args![second]).u32();
                    e.call(0x0047_7780, &args![destruction, first]);
                }
            }
        }
        let old = f.at(-0x18);
        let current = get_3d(e, me);
        e.call(0x0063_3c90, &args![old, current]);
        let old_node = e.call(READ_FIRST_DWORD, &args![old]).u32();
        if old_node == 0 {
            if new_node == 0 {
                let loader = model_loader(e);
                e.call(CANCEL_REFERENCE, &args![loader, me]);
            }
        } else {
            let old_node = e.call(READ_FIRST_DWORD, &args![old]).u32();
            parent = e.call(NODE_PARENT, &args![old_node]).u32();
            if parent != 0 {
                let base = e.mem.u32(me + 0x20);
                let leave_attached = e.call(0x0040_77c0, &args![me]).bool()
                    && base != 0
                    && form_type(e, base) == 0x25;
                if !leave_attached {
                    if e.call(0x008c_7aa0, &args![]).bool() {
                        let node = e.call(READ_FIRST_DWORD, &args![old]).u32();
                        let manager = e.call(0x0045_37b0, &args![]).u32();
                        e.call(0x0087_ac60, &args![manager, node]);
                    } else {
                        let node = e.call(READ_FIRST_DWORD, &args![old]).u32();
                        e.vcall(parent, 0xe8, &args![node]);
                    }
                }
                // the transform of the node being replaced
                let node = e.call(READ_FIRST_DWORD, &args![old]).u32();
                let translate = e.call(0x0043_c490, &args![node]).u32();
                copy_words(e, f.at(-0x4c), translate, 3);
                let node = e.call(READ_FIRST_DWORD, &args![old]).u32();
                let rotate = e.call(0x006a_9540, &args![node]).u32();
                copy_words(e, f.at(-0x3c), rotate, 9);
                let node = e.call(READ_FIRST_DWORD, &args![old]).u32();
                scale = e.call(0x009c_dae0, &args![node]).f32();
            }
            let node = e.call(READ_FIRST_DWORD, &args![old]).u32();
            let scene = e.call(0x0045_0b80, &args![0u32]).u32();
            e.call(0x00b5_b1c0, &args![scene, node]);
            if e.vcall(me, 0xfc, &args![]).bool() {
                let node = e.call(READ_FIRST_DWORD, &args![old]).u32();
                let scene = e.call(0x0045_0b80, &args![0u32]).u32();
                e.call(0x00b5_edf0, &args![scene, node]);
            }
            if new_node == 0 {
                unload_old_3d(e, f, me, actor, old);
            }
            let cell = e.mem.u32(me + 0x40);
            if cell == 0 || !fn_00570ec0(e, Ptr::new(cell)) {
                e.call(0x0097_49b0, &args![OBJECT_PROCESS_LISTS, me]);
                if (fn_00570f60(e) as i32) > 0 {
                    e.call(0x008c_ddd0, &args![me, 1u32]);
                }
            }
            if actor == 0 || actor == e.global::<u32>(GLOBAL_PLAYER) {
                let node = e.call(READ_FIRST_DWORD, &args![old]).u32();
                e.call(0x00c4_5830, &args![node]);
            } else {
                let process = e.call(GET_PROCESS, &args![actor]).u32();
                if process != 0 && (e.call(0x0045_cd60, &args![process]).u32() as i32) <= 1 {
                    e.vcall(process, 0x4fc, &args![0u32]);
                }
                let node = e.call(READ_FIRST_DWORD, &args![old]).u32();
                e.call(0x0086_8660, &args![node]);
            }
            let held_by_loader = e.call(0x0045_23e0, &args![me, 4u32]).bool();
            e.call(0x0066_b0d0, &args![old, 0u32]);
            fn_00570f70(e, this, 0);
            let base = e.mem.u32(me + 0x20);
            if new_node == 0 && base != 0 {
                if e.call(0x0056_51e0, &args![me]).bool() {
                    let loader = model_loader(e);
                    e.call(CANCEL_REFERENCE, &args![loader, me]);
                } else if !held_by_loader {
                    let base = e.mem.u32(me + 0x20);
                    e.vcall(base, 0x150, &args![me]);
                }
            }
        }
        if new_node == 0 {
            e.call(0x0057_9ac0, &args![me, 0u32]);
        }
        fn_00570f70(e, this, new_node);
        if new_node != 0 && parent != 0 {
            e.call(0x0044_0460, &args![new_node, f.at(-0x4c)]);
            e.call(0x0043_fa80, &args![new_node, f.at(-0x3c)]);
            e.call(0x0044_0490, &args![new_node, scale]);
            e.vcall(parent, 0xdc, &args![new_node, 1u32]);
            e.call(0x00c6_bd00, &args![new_node, 1u32]);
            e.call(0x00a5_a040, &args![new_node]);
            e.call(
                UPDATE_DATA_CONSTRUCTOR,
                &args![f.at(-0x88), 0.0f32, 0u32, 0u32],
            );
            e.call(NODE_UPDATE, &args![new_node, f.at(-0x88)]);
        }
        let base = e.mem.u32(me + 0x20);
        if new_node != 0 && base != 0 && form_type(e, base) != 0x25 {
            e.call(0x0056_c7d0, &args![new_node, me]);
        }
        let save_load = e.global::<u32>(GLOBAL_SAVE_LOAD);
        if !e.call(0x0047_c850, &args![save_load]).bool()
            && new_node == 0
            && e.call(0x0056_8e50, &args![me]).u32() != 0
        {
            // the teleport data of the reference and the cell it leads to
            let teleport = e.call(0x0056_8e50, &args![me]).u32();
            if e.call(0x0043_a2b0, &args![teleport]).u32() != 0 {
                let teleport = e.call(0x0056_8e50, &args![me]).u32();
                let cell = e.call(0x0043_a2b0, &args![teleport]).u32();
                e.call(0x0055_1480, &args![cell, 0u32]);
            }
        }
        if e.vcall(me, 0x15c, &args![]).bool() {
            let manager = e.call(0x006c_0720, &args![]).u32();
            if new_node != 0 {
                e.call(0x006c_0c30, &args![manager, me]);
            } else {
                e.call(0x006c_0c80, &args![manager, me]);
            }
        }
        if new_node == 0 && base_form(e, me) != 0 {
            let base = base_form(e, me);
            if form_type(e, base) == 0x1c {
                let manager = e.call(0x006c_0720, &args![]).u32();
                e.call(0x006c_1060, &args![manager, me]);
            }
        }
        e.call(0x0045_cec0, &args![old]);
    });
}

/// The part of `Set3D` that runs when the 3D is being unloaded and the
/// reference had a 3D (`old` is the `NiPointer` local at `ebp-0x18`).
fn unload_old_3d(e: &mut Engine, f: &Frame, me: u32, actor: u32, old: u32) {
    let mut owner_has_flag = false;
    let node = e.call(READ_FIRST_DWORD, &args![old]).u32();
    let owner = e.vcall(node, 0x10, &args![]).u32();
    if owner != 0 {
        e.call(0x0056_c7d0, &args![owner, 0u32]);
        owner_has_flag = fn_00570ee0(e, Ptr::new(owner));
    }
    let node = e.call(READ_FIRST_DWORD, &args![old]).u32();
    if fn_00570ea0(e, Ptr::new(node)) {
        let node = e.call(READ_FIRST_DWORD, &args![old]).u32();
        let name = e.vcall(node, 0xc, &args![]).u32();
        e.call(0x0057_8250, &args![name]);
    }
    e.call(0x0056_8480, &args![me]);
    e.call(0x009c_9e90, &args![0x011f_2250u32, me]);
    if !e.call(FORM_IS_DELETED, &args![me]).bool() {
        let loading = e.global::<u32>(GLOBAL_LOADING_OBJECT);
        if fn_00570f00(e, Ptr::new(loading)) {
            let change = change_flags(e, 4);
            if e.call(0x0084_a6d0, &args![loading, me, change]).bool() {
                e.call(0x0044_dee0, &args![f.at(-0x64)]);
                e.call(0x0086_5020, &args![f.at(-0x64), me]);
                let buffer = e.call(READ_FIRST_DWORD, &args![f.at(-0x64)]).u32();
                let extra = extra_list(e, me);
                e.call(0x0042_2ac0, &args![extra, buffer]);
            }
            let change = change_flags(e, 0x1000_0000);
            if e.call(0x0084_a6d0, &args![loading, me, change]).bool() {
                let mut has_animation = false;
                if !is_actor(e, me) {
                    has_animation = e.call(0x0056_3530, &args![me]).bool();
                }
                if has_animation {
                    e.call(0x0044_dee0, &args![f.at(-0x6c)]);
                    e.call(0x0086_4f20, &args![f.at(-0x6c), me]);
                    let buffer = e.call(READ_FIRST_DWORD, &args![f.at(-0x6c)]).u32();
                    let extra = extra_list(e, me);
                    e.call(0x0042_2940, &args![extra, buffer]);
                } else {
                    e.vcall(me, 0x4c, &args![0x1000_0000u32]);
                }
            }
        }
    }
    // KillLight and SetAnimation (the map names 00572a50 and 00572e50)
    e.call(0x0057_2a50, &args![me, 0u32]);
    e.call(0x0057_2e50, &args![me, 0u32]);
    if actor != 0 {
        tes_object_refr_remove_weapon(e, Ptr::new(me));
        e.call(0x0045_34f0, &args![me, 0u32]);
        e.call(0x0048_3710, &args![me]);
        e.call(0x0048_3710, &args![me]);
    }
    e.vcall(me, 0x1f0, &args![0u32]);
    let node = e.call(READ_FIRST_DWORD, &args![old]).u32();
    let name = e.vcall(node, 0xc, &args![]).u32();
    let has_addon_flags = e.call(0x0057_84b0, &args![name]).bool();
    e.call(0x0056_c880, &args![me, 2u32, u32::from(has_addon_flags)]);
    if has_addon_flags {
        let node = e.call(READ_FIRST_DWORD, &args![old]).u32();
        e.call(0x0057_8300, &args![node]);
    }
    if actor != 0 {
        let controller = e.call(0x0093_06d0, &args![actor]).u32();
        if controller != 0 {
            e.call(0x00c6_d7a0, &args![controller, 0u32]);
        }
        if e.call(GET_PROCESS, &args![actor]).u32() != 0 {
            let process = e.call(GET_PROCESS, &args![actor]).u32();
            e.vcall(process, 0x5f4, &args![0u32]);
        }
        // Actor +0xac: an owned object deleted with its deleting destructor
        let owned = e.mem.u32(actor + 0xac);
        if owned != 0 {
            e.vcall(owned, 0, &args![1u32]);
            e.mem.set_u32(actor + 0xac, 0);
        }
        // Actor +0xb0: bhkRagdollPenetrationUtil::Clear's owner
        let ragdoll = e.mem.u32(actor + 0xb0);
        if ragdoll != 0 {
            e.call(0x00ca_20e0, &args![ragdoll]);
        }
    }
    if owner_has_flag || e.vcall(me, 0x224, &args![]).bool() || e.vcall(me, 0xfc, &args![]).bool() {
        let node = node_of(e, me);
        let audio = e.call(0x0045_3a70, &args![]).u32();
        e.call(0x00ad_8570, &args![audio, node, 0.0f32, 0u32]);
    }
    e.call(0x0057_a3c0, &args![me, 1u32]);
}

/// A `BGSChangeFlags`-style by-value change mask (`008c71b0` stores the
/// value into the stack slot the game passes as the argument).
fn change_flags(e: &mut Engine, value: u32) -> u32 {
    e.with_stack(4, |e, slot| {
        e.call(0x008c_71b0, &args![slot, value]);
        e.mem.u32(slot.addr())
    })
}

// Translated from 00570ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests flag `0x20000` of the node `this` (`00456630`, which tests its
/// flags word at `+0x30`).
pub fn fn_00570ea0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0045_6630, &args![this, 0x2_0000u32]).bool()
}

// Translated from 00570ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when `00450fd0` (the byte at `+0x26`) of the cell `this` is 1.
pub fn fn_00570ec0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0045_0fd0, &args![this]).u32() == 1
}

// Translated from 00570ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Wrapper of `00570ea0` (flag `0x20000` of the node `this`).
pub fn fn_00570ee0(e: &mut Engine, this: Ptr) -> bool {
    fn_00570ea0(e, this)
}

// Translated from 00570f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when `00462480` holds and `00570f40` holds for `this` (the object
/// at `011ddf38`): the thread's state says it is loading and the object has
/// bit 0 of its word at `+0x244` clear.
pub fn fn_00570f00(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0046_2480, &args![this]).bool() && fn_00570f40(e, this)
}

// Translated from 00570f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit 0 of the word at `+0x244` of `this` is clear.
pub fn fn_00570f40(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x244) & 1 == 0
}

// Translated from 00570f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The global at `011dfc98` (a count the callers compare with 0).
pub fn fn_00570f60(e: &mut Engine) -> u32 {
    e.global::<u32>(0x011d_fc98)
}

// Translated from 00570f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tail of `Set3D`: tells the nav-mesh obstacle manager (`006c0720`) about
/// the reference when its virtual at `+0x15c` says it matters (added with a
/// 3D, removed without), forgets a base object of form type `0x1c` when the
/// 3D is gone (`006c1060`), clears the room extra of a room marker or the
/// portal extra of a portal marker (`00420f00`, `00420e00`), runs the
/// virtual at `+0x1c0` (or, when `008c7aa0` holds, the task queue's
/// `0087b2b0` on the singleton `004537b0`) and finally `Set3DVerySimple`.
pub fn fn_00570f70(e: &mut Engine, this: Ptr<TESObjectREFR>, node: u32) {
    let me = this.addr();
    if e.vcall(me, 0x15c, &args![]).bool() {
        let manager = e.call(0x006c_0720, &args![]).u32();
        if node != 0 {
            e.call(0x006c_0c30, &args![manager, me]);
        } else {
            e.call(0x006c_0c80, &args![manager, me]);
        }
    }
    if node == 0 && base_form(e, me) != 0 {
        let base = base_form(e, me);
        if form_type(e, base) == 0x1c {
            let manager = e.call(0x006c_0720, &args![]).u32();
            e.call(0x006c_1060, &args![manager, me]);
        }
    }
    if node == 0 && base_form(e, me) == e.global::<u32>(GLOBAL_ROOM_MARKER_BASE) {
        let extra = extra_list(e, me);
        e.call(0x0042_0f00, &args![extra, 0u32]);
    } else if node == 0 && base_form(e, me) == e.global::<u32>(GLOBAL_PORTAL_MARKER_BASE) {
        let extra = extra_list(e, me);
        e.call(0x0042_0e00, &args![extra, 0u32]);
    }
    if e.call(0x008c_7aa0, &args![]).bool() {
        let queue = e.call(0x0045_37b0, &args![]).u32();
        e.call(0x0087_b2b0, &args![queue, me]);
    } else {
        e.vcall(me, 0x1c0, &args![]);
    }
    tes_object_refr_set_3d_very_simple(e, this, node);
}

// Translated from 00571080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::Set3DVerySimple` (Xbox PDB): with a node, makes sure the
/// loaded data exists (`CreateLoadedData`, `0057c180`) and stores the node in
/// its 3D slot at `+0x14` (`0066b0d0`); without one, clears the loaded data
/// (`0057c300`).
pub fn tes_object_refr_set_3d_very_simple(e: &mut Engine, this: Ptr<TESObjectREFR>, node: u32) {
    if node != 0 {
        e.call(0x0057_c180, &args![this]);
        // TESObjectREFR::pLoadedData (Xbox PDB) +0x64
        let loaded = e.get(this, TESObjectREFR::pLoadedData).addr();
        e.call(0x0066_b0d0, &args![loaded + 0x14, node]);
    } else {
        e.call(0x0057_c300, &args![this]);
    }
}

// Translated from 005710c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Attaches the worn models of the base object of a reference to its 3D
/// (`this` is the reference). Returns what the base object accepted:
///
/// * an NPC base (form type `0x2a`): when the reference has a biped
///   (virtual `+0x1e8`) the base replaces the model (`TESNPC::ReplaceRefModel`,
///   `00605d70`); true;
/// * a creature base (`0x2b`) with a 3D: for each package of the base the
///   process runs (`00717e50`, `008256d0`) the model file named by the
///   reference's model with the file name replaced by the package's is
///   loaded (`ModelLoader::LoadFile`, `00447080`), cloned (deep copy when it
///   has a morpher controller) and attached to the 3D, to the skeleton under
///   a "SkinAttachment" node when it is skinned and the 3D has the named
///   node; true;
/// * any other base: its virtual at `+0x160`.
///
/// The animation (virtual `+0x1e4`) is first reset through `00496280`. The
/// compiler's exception-unwinding frame is not translated.
pub fn fn_005710c0(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    let me = this.addr();
    let base = e.mem.u32(me + 0x20);
    let biped = e.vcall(me, 0x1e8, &args![]).u32();
    let mut done = false;
    let animation = e.vcall(me, 0x1e4, &args![]).u32();
    if animation != 0 {
        e.call(0x0049_6280, &args![animation]);
    }
    let node = get_3d(e, me);
    match form_type(e, base) {
        0x2a => {
            if biped != 0 {
                e.call(0x0060_5d70, &args![base, me]);
            }
            done = true;
        }
        0x2b => {
            if node != 0 {
                attach_creature_models(e, me, base, node);
            }
            done = true;
        }
        _ => {}
    }
    if !done {
        let base = e.mem.u32(me + 0x20);
        done = e.vcall(base, 0x160, &args![]).bool();
    }
    done
}

/// The creature branch of `fn_005710c0`: attaches the model of each package
/// to `node`.
fn attach_creature_models(e: &mut Engine, me: u32, base: u32, node: u32) {
    if is_actor(e, me) {
        let package = e.call(0x0093_44a0, &args![me]).u32();
        if package != 0 {
            e.call(0x0044_1b00, &args![package]);
        }
    }
    e.call(0x005f_a0a0, &args![base, me]);
    let mut list = e.call(0x0071_7e50, &args![base + 0x114]).u32();
    while list != 0 && !e.call(0x0082_56d0, &args![list]).bool() {
        with_frame(e, |e, f| {
            let path = tes_object_refr_get_model(e, Ptr::new(me));
            let buffer = f.at(-0x14c);
            e.call(0x0040_6d30, &args![buffer, 0x104u32, path]);
            let slash = e.call(0x0040_ab30, &args![buffer, 0x5cu32]).u32();
            let file = node_item(e, list);
            let room = 0x104u32
                .wrapping_sub(slash.wrapping_sub(buffer))
                .wrapping_sub(1);
            e.call(0x0040_6d30, &args![slash + 1, room, file]);
            let loader = model_loader(e);
            let model = e
                .call(
                    0x0044_7080,
                    &args![loader, buffer, 3u32, 1u32, 0u32, 0u32, 0u32],
                )
                .u32();
            if model != 0 {
                let cloning = f.at(-0x180);
                let scale = e.call(GET_SCALE, &args![me]).f32();
                e.call(0x004a_d050, &args![cloning, scale]);
                let holder = f.at(-0x15c);
                e.call(0x0063_3c90, &args![holder, 0u32]);
                let cloned = if e.call(0x004b_5bf0, &args![model]).bool() {
                    let tes = e.global::<u32>(GLOBAL_TES);
                    let copy = e.call(0x0045_7ba0, &args![tes, model, cloning]).u32();
                    e.call(0x0066_b0d0, &args![holder, copy]);
                    e.call(READ_FIRST_DWORD, &args![holder]).u32()
                } else {
                    e.call(0x00a5_d2c0, &args![model, cloning]).u32()
                };
                let skinned = e.call(0x0049_10d0, &args![cloned]).bool();
                let name = e.call(0x0057_1550, &args![]).u32();
                let found = fn_00571530(e, Ptr::new(me), node, name);
                if skinned && found != 0 {
                    let fixed = f.at(-0x188);
                    let key = e.call(0x0043_8170, &args![fixed, 0x0101_f724u32]).u32();
                    let child = e.vcall(found, 0x9c, &args![key]).u32();
                    e.call(0x0043_81b0, &args![fixed]);
                    if child != 0 {
                        e.vcall(child, 0xdc, &args![cloned, 1u32]);
                    } else {
                        let parent = e.call(NODE_PARENT, &args![found]).u32();
                        e.vcall(parent, 0xdc, &args![cloned, 1u32]);
                    }
                    e.call(0x004a_de40, &args![node, cloned, 0u32, 1u32]);
                } else {
                    e.call(
                        0x004a_e250,
                        &args![node, cloned, 0u32, 0u32, 0xffff_ffffu32, 0u32],
                    );
                }
                e.call(0x0045_cec0, &args![holder]);
                e.call(0x004a_d270, &args![cloning]);
            }
        });
        list = e.call(LIST_NEXT, &args![list]).u32();
    }
}

// Translated from 00571530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the child of `root` named `name` (`BSUtilities::GetObjectByName`,
/// `00c4b470`, with its flag 1). `this` is not used (the map names the body
/// `std::allocator<char>::deallocate`, which it is not).
pub fn fn_00571530(e: &mut Engine, _this: Ptr, root: u32, name: u32) -> u32 {
    e.call(0x00c4_b470, &args![root, name, 1u32]).u32()
}

// Translated from 00571550 (decompiled, FalloutNV.exe 1.4.0.525)
/// The global at `011c61a4` (the name of the node the skinned models hang
/// under).
pub fn fn_00571550(e: &mut Engine) -> u32 {
    e.global::<u32>(0x011c_61a4)
}

// Translated from 00571560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::ClearInterpolators` (Xbox PDB): with an animation
/// (virtual `+0x1e4`), frees the special idle (`00498910`), clears the
/// animation group `0x14` (`00496080`), deactivates all the controllers of
/// its controller manager (`00496940`, `0048fef0`), clears the
/// controllers' interpolators (`00499160`) and the blend interpolators
/// (`00499080`).
pub fn tes_object_refr_clear_interpolators(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    let animation = e.vcall(me, 0x1e4, &args![]).u32();
    if animation == 0 {
        return;
    }
    e.call(0x0049_8910, &args![animation, 1u32, 0u32]);
    e.call(0x0049_6080, &args![animation, 0x14u32, 0.0f32]);
    let manager = e.call(0x0049_6940, &args![animation]).u32();
    e.call(0x0048_fef0, &args![manager, 0.0f32]);
    let controllers = e.call(0x0055_85e0, &args![animation]).u32();
    e.call(0x0049_9160, &args![controllers]);
    e.call(0x0049_9080, &args![animation]);
}

// Translated from 005715d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetModel` (Xbox PDB): the path of the reference's
/// `TESModel` (its virtual at `+0x14`), 0 without a model.
pub fn tes_object_refr_get_model(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let model = tes_object_refr_get_tes_model(e, this);
    if model == 0 {
        return 0;
    }
    e.vcall(model, 0x14, &args![]).u32()
}

// Translated from 00571600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetModelBoundSize` (Xbox PDB): `GetBoundSize` of the
/// base object (`0050ebf0`), 0.0 without one.
pub fn tes_object_refr_get_model_bound_size(e: &mut Engine, this: Ptr<TESObjectREFR>) -> f32 {
    if base_form(e, this.addr()) == 0 {
        return 0.0;
    }
    let base = base_form(e, this.addr());
    e.call(0x0050_ebf0, &args![base]).f32()
}

// Translated from 00571630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetTESModel` (Xbox PDB): the model of the reference.
/// First the model swap of its extra list (`0042e250`). Without one: a
/// weapon base (form type `0x28`) gives its model, the one of the weapon
/// mods when the extra list has weapon-mod flags (`00522df0` with those
/// flags and 0, else `00522d80`); any other base gives the `TESBoundObject`
/// dynamic cast of itself to the type at `011831e8`. Still none: a base
/// with a biped model (`00480db0`) gives the world model for the sex of
/// the reference's owner when that is an NPC (`00481110`).
pub fn tes_object_refr_get_tes_model(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let me = this.addr();
    let extra = extra_list(e, me);
    let mut model = e.call(0x0042_e250, &args![extra]).u32();
    if model == 0 && base_form(e, me) != 0 {
        let base = base_form(e, me);
        if form_type(e, base) == 0x28 {
            let weapon = base_form(e, me);
            let extra = extra_list(e, me);
            if e.call(0x0042_e560, &args![extra]).bool() {
                let extra = extra_list(e, me);
                let flags = e.call(0x0042_e560, &args![extra]).u8();
                model = e
                    .call(0x0052_2df0, &args![weapon, u32::from(flags), 0u32])
                    .u32();
            } else {
                model = e.call(0x0052_2d80, &args![weapon]).u32();
            }
        } else {
            let base = base_form(e, me);
            model = e
                .call(
                    0x00ec_43fb,
                    &args![base, 0u32, 0x0118_3108u32, 0x0118_31e8u32, 0u32],
                )
                .u32();
        }
    }
    if model == 0 {
        let base = base_form(e, me);
        let biped_form = e.call(0x0048_0db0, &args![base]).u32();
        if biped_form != 0 {
            let mut sex = 0u32;
            let owner = e.call(0x0056_7790, &args![me]).u32();
            if owner != 0 && form_type(e, owner) == 0x2a {
                sex = e.call(0x005f_0cc0, &args![owner]).u32();
            }
            model = e.call(0x0048_1110, &args![biped_form, sex]).u32();
        }
    }
    model
}

/// Index into the table of weapon-animation group numbers at `0118a838`
/// for the item `item`: the signed byte at `+0xf4` (`00446390`) selects the
/// entry.
fn animation_type_entry(e: &mut Engine, item: u32) -> u32 {
    let index = e.call(0x0044_6390, &args![item]).u32() as i32;
    e.mem
        .u32(0x0118_a838u32.wrapping_add((index as u32).wrapping_mul(4)))
}

// Translated from 00571760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Equips the biped model of `item` (an armor or weapon form; the biped
/// part is at `item + 0x3c`) on the reference. Does nothing without a 3D.
///
/// * An actor whose process allows it (virtual `+0x454` on the process)
///   is told virtual `+0x458` (`actor, 0`).
/// * The player's first-person biped gets `004ab750(biped, item, slots)`
///   with the mod slots (`004bd820`) of the equipped item and, when the
///   item has mod effect `0xe` active (`004bd8d0`), the user-interface
///   hook `00709c20` runs for the weapon's `00504e60`.
/// * For another reference the weapon animation group of the item is loaded
///   when its animation lacks it (`005f2370`, `00494710`, the KF file list
///   `00447330` and `00490400`).
/// * Without a biped the model is loaded and attached (`004aeed0`) and the
///   process is told virtual `+0x1cc` (drawn state, biped, animation,
///   actor).
/// * An actor then reloads its targets (without a biped), refreshes its
///   lighting property, alpha and weapon condition effects (`008b0bd0`,
///   `008c4640`, `00891190`), and, for an item in a biped model list
///   (`00475020`) that has a list node, tells its process virtual `+0x468`.
///
/// The string "Lily" in the reference's name (`0055d520`) prints a debug
/// message.
pub fn fn_00571760(e: &mut Engine, this: Ptr<TESObjectREFR>, item: u32) {
    let me = this.addr();
    if get_3d(e, me) == 0 {
        return;
    }
    let name = e.call(0x0055_d520, &args![me]).u32();
    if e.call(0x00ec_7750, &args![name, 0x0102_fdc8u32]).u32() != 0 {
        e.call(0x005b_5e40, &args![0x0103_0e38u32]);
    }
    let mut has_effect = false;
    let mut slots = 0u32;
    let mut actor = 0u32;
    if is_actor(e, me) {
        actor = me;
    }
    let loading = e.global::<u32>(GLOBAL_LOADING_OBJECT);
    if !e.call(0x0042_ce10, &args![loading]).bool() {
        let process = e.call(GET_PROCESS, &args![actor]).u32();
        if e.vcall(process, 0x454, &args![]).bool() {
            let process = e.call(GET_PROCESS, &args![actor]).u32();
            e.vcall(process, 0x458, &args![actor, 0u32]);
        }
    }
    let player = e.global::<u32>(GLOBAL_PLAYER);
    let mut biped = if me == player {
        e.call(0x0095_0b00, &args![player, 1u32]).u32()
    } else {
        e.vcall(me, 0x1e8, &args![]).u32()
    };
    if me == e.global::<u32>(GLOBAL_PLAYER) {
        let process = e.call(GET_PROCESS, &args![player]).u32();
        let worn = e.vcall(process, 0x148, &args![]).u32();
        if worn != 0 {
            slots = u32::from(e.call(0x004b_d820, &args![worn]).u8());
            // the float the effect check fills (ebp-0x10), starting at 0.0
            has_effect = e.with_stack(4, |e, value| {
                e.call(0x004b_d8d0, &args![worn, 0xeu32, value]).bool()
            });
        }
        if biped != 0 {
            e.call(0x004a_b750, &args![biped, item, slots]);
        }
        biped = e.call(0x0095_0b00, &args![player, 0u32]).u32();
        if e.call(0x004a_d010, &args![item]).bool()
            && me == e.global::<u32>(GLOBAL_PLAYER)
            && (!e.call(0x004a_d030, &args![item]).bool() || has_effect)
        {
            let hook = e.call(0x0050_4e60, &args![item]).u32();
            e.call(0x0070_9c20, &args![hook]);
        }
    } else {
        let animation = e.vcall(me, 0x1e4, &args![]).u32();
        if animation != 0 {
            let animation = e.vcall(me, 0x1e4, &args![]).u32();
            if e.call(0x0049_6940, &args![animation]).u32() != 0 {
                let entry = animation_type_entry(e, item);
                let group = e
                    .call(0x005f_2370, &args![0u32, entry, 0x18u32, 0u32])
                    .u16();
                let animation = e.vcall(me, 0x1e4, &args![]).u32();
                if !e
                    .call(0x0049_4710, &args![animation, u32::from(group)])
                    .bool()
                    && !e.vcall(me, 0x22c, &args![1u32]).bool()
                {
                    let entry = animation_type_entry(e, item);
                    let path = tes_object_refr_get_model(e, this);
                    let loader = model_loader(e);
                    let list = e
                        .call(0x0044_7330, &args![loader, path, 1u32, 0u32, entry])
                        .u32();
                    let animation = e.vcall(me, 0x1e4, &args![]).u32();
                    e.call(0x0049_0400, &args![animation, list]);
                }
            }
        }
    }
    if biped != 0 {
        e.call(0x004a_b750, &args![biped, item, slots]);
    } else {
        // the biped part of the item is embedded at +0x3c
        let part = if item != 0 { item + 0x3c } else { 0 };
        if part != 0 {
            e.call(0x004a_eed0, &args![item, part, 5u32, me, 0u32]);
            let loading = e.global::<u32>(GLOBAL_LOADING_OBJECT);
            if !e.call(0x0042_ce10, &args![loading]).bool() {
                let mut drawn = e.call(0x008a_16d0, &args![actor]).u8();
                if drawn == 0 && e.call(0x0046_e8c0, &args![item]).bool() {
                    drawn = 1;
                }
                let process = e.call(GET_PROCESS, &args![actor]).u32();
                let animation = e.vcall(actor, 0x1e4, &args![]).u32();
                e.vcall(
                    process,
                    0x1cc,
                    &args![u32::from(drawn), biped, animation, actor],
                );
            }
        }
    }
    if actor != 0 {
        if biped == 0 {
            e.call(0x008b_0b00, &args![actor, 1u32]);
        }
        let node = get_3d(e, me);
        e.call(0x008b_0bd0, &args![actor, node]);
        e.call(0x008c_4640, &args![actor]);
        e.call(0x0089_1190, &args![actor, 1u32]);
        let list = e.call(0x0047_5020, &args![item]).u32();
        if list != 0 && e.call(LIST_NEXT, &args![list]).u32() != 0 {
            let process = e.call(GET_PROCESS, &args![actor]).u32();
            e.vcall(process, 0x468, &args![1u32]);
        }
    }
}

/// Whether the node list `list` holds an entry whose name equals the string
/// at `01030e44` (the loops of `RemoveWeapon`).
fn list_has_named_entry(e: &mut Engine, list: u32) -> bool {
    let mut index = 0u32;
    while index < e.call(0x0043_b480, &args![list]).u32() {
        let item = e.call(0x0043_b4a0, &args![list, index]).u32();
        if item != 0 {
            let text = e.call(0x0041_3f40, &args![item]).u32();
            if e.call(0x0043_b1b0, &args![text]).u32() != 0 {
                let text = e.call(0x0041_3f40, &args![item]).u32();
                let text = e.call(0x0043_b1b0, &args![text]).u32();
                if e.call(0x0040_8b20, &args![text, 0x0103_0e44u32]).u32() == 0 {
                    return true;
                }
            }
        }
        index += 1;
    }
    false
}

/// Takes `object` out of the shadow scene node and clears its attached
/// effects (`00b5b1c0`, `00572160`).
fn remove_from_scene(e: &mut Engine, object: u32) {
    let scene = e.call(0x0045_0b80, &args![0u32]).u32();
    e.call(0x00b5_b1c0, &args![scene, object]);
    fn_00572160(e, Ptr::new(object));
}

// Translated from 00571b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::RemoveWeapon` (Xbox PDB): takes the weapon of an actor
/// out of its 3D. Does nothing without a 3D.
///
/// * The worn weapon's inventory entry (slot 5 of the container changes,
///   `004c8c10`) is looked up; the player first leaves iron sights
///   (`008bb650`) and VATS (`009c8950`), drops the biped weapon
///   (`004ab5b0`, with `00572220` around it for a non-first-person biped),
///   and hides the gun scope (`00709c40`, `00709ca0`).
/// * Without a biped the "Backpack" node is detached, the weapon node of the
///   process (virtual `+0x1a0`, or its named children `+0x1a4`, or the
///   node's "Weapon" child) is found and, when the worn form has a model,
///   `0045a5e0` is told its path.
/// * The weapon node, the "Weapon" child when the actor virtual `+0x21c`
///   says so, and the named list are taken out of the shadow scene node
///   (`remove_from_scene`).
/// * An actor reloads its targets, refreshes its lighting property and,
///   for a biped model list with a node, tells its process virtual `+0x468`.
pub fn tes_object_refr_remove_weapon(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    if get_3d(e, me) == 0 {
        return;
    }
    let mut worn_form = 0u32;
    let extra = extra_list(e, me);
    let changes = e.call(0x0041_8520, &args![extra]).u32();
    if changes != 0 {
        let entry = e.call(0x004c_8c10, &args![changes, 5u32, 0u32]).u32();
        if entry != 0 {
            worn_form = e.call(0x0044_ddc0, &args![entry]).u32();
            e.call(0x0044_59e0, &args![entry, 1u32]);
        }
    }
    let player = e.global::<u32>(GLOBAL_PLAYER);
    let mut biped;
    if me == player {
        if e.call(0x008b_bc10, &args![player]).bool() && fn_005721e0(e, Ptr::new(player)) {
            e.call(0x008b_b650, &args![player, 0u32, 0u32, 0u32]);
        }
        biped = e.call(0x0095_0b00, &args![player, 1u32]).u32();
    } else {
        biped = e.vcall(me, 0x1e8, &args![]).u32();
    }
    let mut actor = 0u32;
    if is_actor(e, me) {
        actor = me;
    }
    if me == e.global::<u32>(GLOBAL_PLAYER) {
        biped = e.call(0x0095_0b00, &args![player, 1u32]).u32();
        e.call(0x009c_8950, &args![0x011f_2250u32, 0u32, 1u32]);
        if biped != 0 {
            e.call(0x004a_b5b0, &args![biped]);
        }
        biped = e.call(0x0095_0b00, &args![player, 0u32]).u32();
        if e.call(0x008b_bc10, &args![player]).bool() {
            e.call(0x0070_9c40, &args![0u32]);
        }
        e.call(0x0070_9ca0, &args![]);
    }
    let process = e.call(GET_PROCESS, &args![actor]).u32();
    if biped != 0 {
        fn_00572220(e, 1);
        e.call(0x004a_b5b0, &args![biped]);
        fn_00572220(e, 0);
    } else {
        let mut weapon_found = false;
        let node = node_of(e, me);
        let backpack = e.call(0x004a_ae30, &args![node, 0x0101_f5ccu32]).u32();
        if backpack != 0 {
            let parent = e.call(NODE_PARENT, &args![backpack]).u32();
            e.vcall(parent, 0xe8, &args![backpack]);
        }
        if process != 0 {
            let held = e.vcall(process, 0x1a0, &args![0u32]).u32();
            weapon_found = held != 0 && e.call(0x0045_3470, &args![held]).u32() != 0;
            if !weapon_found {
                let list = e.vcall(process, 0x1a4, &args![0u32]).u32();
                let mut found = false;
                if list != 0 {
                    found = list_has_named_entry(e, list);
                }
                if found {
                    weapon_found = list != 0 && e.call(0x0045_3470, &args![list]).u32() != 0;
                }
                if !weapon_found {
                    let node = node_of(e, me);
                    if node != 0 {
                        let weapon = e.call(0x004a_ae30, &args![node, 0x0101_3be8u32]).u32();
                        if weapon != 0 {
                            weapon_found = true;
                        }
                    }
                }
            }
        }
        if worn_form != 0 {
            // the TESModel part of the worn form is embedded at +0x3c
            let model = worn_form + 0x3c;
            if weapon_found {
                let path = e.vcall(model, 0x14, &args![]).u32();
                let loader = model_loader(e);
                e.call(0x0045_a5e0, &args![loader, path]);
            }
        }
    }
    if process != 0 {
        let held = e.vcall(process, 0x1a0, &args![0u32]).u32();
        if held != 0 {
            remove_from_scene(e, held);
        }
        if e.vcall(me, 0x21c, &args![]).bool() {
            let node = node_of(e, me);
            if node != 0 {
                let weapon = e.call(0x004a_ae30, &args![node, 0x0101_3be8u32]).u32();
                if weapon != 0 {
                    remove_from_scene(e, weapon);
                }
            }
        }
        let list = e.vcall(process, 0x1a4, &args![0u32]).u32();
        if list != 0 && list_has_named_entry(e, list) {
            remove_from_scene(e, list);
        }
    }
    if actor != 0 {
        let process = e.call(GET_PROCESS, &args![actor]).u32();
        let data_handler = e.global::<u32>(GLOBAL_DATA_HANDLER);
        let save_load = e.global::<u32>(GLOBAL_SAVE_LOAD);
        if biped == 0
            && process != 0
            && !e.call(0x0042_26e0, &args![data_handler]).bool()
            && !e.call(0x0047_c850, &args![save_load]).bool()
        {
            e.call(0x008b_0b00, &args![actor, 0u32]);
        }
        let node = get_3d(e, me);
        e.call(0x008b_0bd0, &args![actor, node]);
        if e.call(GET_PROCESS, &args![actor]).u32() != 0 {
            let list = e.call(0x0047_5020, &args![worn_form]).u32();
            if list != 0 && e.call(LIST_NEXT, &args![list]).u32() != 0 {
                let process = e.call(GET_PROCESS, &args![actor]).u32();
                e.vcall(process, 0x468, &args![1u32]);
            }
        }
    }
}

// Translated from 00572160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases the attached effects of the node `this`: for each element of
/// the array at `+0x9c` (`00658930` counts them, `00877a30` indexes) whose
/// pointer is set, `0050f5a0` runs on it; then the array is emptied
/// (`NiTArray<NiPointer<BSTempEffect>>::RemoveAll`, `004dffa0`).
pub fn fn_00572160(e: &mut Engine, this: Ptr) {
    let array = this.addr() + 0x9c;
    let count = e.call(0x0065_8930, &args![array]).u32();
    let mut index = 0u32;
    while index < count {
        let slot = e.call(0x0087_7a30, &args![array, index]).u32();
        let effect = e.call(READ_FIRST_DWORD, &args![slot]).u32();
        if effect != 0 {
            e.call(0x0050_f5a0, &args![effect]);
        }
        index += 1;
    }
    e.call(0x004d_ffa0, &args![array]);
}

// Translated from 005721e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when either of the bytes at `+0x64f` and `+0x64d` of the player
/// (`this`) is set (the iron-sights state flags).
pub fn fn_005721e0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x64f) != 0 || e.mem.u8(this.addr() + 0x64d) != 0
}

// Translated from 00572220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at `011dcfa7` (cdecl). `RemoveWeapon` sets it
/// to 1 around the removal of the biped weapon.
pub fn fn_00572220(e: &mut Engine, value: u8) {
    e.set_global(0x011d_cfa7, value);
}

// Translated from 00572230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::MarkAsPickedUp` (Xbox PDB): when the reference was
/// dropped by someone (`ExtraDataList::GetItemDropper`, `0041de00`), takes
/// it out of that dropper's list of dropped items (`RemoveDroppedItem`,
/// `0041e0d0`); then `MarkAsDeleted` (`00572270`).
pub fn tes_object_refr_mark_as_picked_up(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    let extra = extra_list(e, me);
    let dropper = e.call(0x0041_de00, &args![extra]).u32();
    if dropper != 0 {
        let dropper_extra = extra_list(e, dropper);
        e.call(0x0041_e0d0, &args![dropper_extra, me]);
    }
    tes_object_refr_mark_as_deleted(e, this);
}

// Translated from 00572270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::MarkAsDeleted` (Xbox PDB): three virtual calls on the
/// reference: the one at `+0xc4` (`SetDelete` in the Xbox PDB) with 1, the
/// one at `+0xc8` (`SetAltered`) with 1, and the one at `+0x1cc` with
/// `(0, 0)`.
pub fn tes_object_refr_mark_as_deleted(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    e.vcall(me, 0xc4, &args![1u32]);
    e.vcall(me, 0xc8, &args![1u32]);
    e.vcall(me, 0x1cc, &args![0u32, 0u32]);
}

// Translated from 005722c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::IsDead` (Xbox PDB): an actor whose form id is not 7
/// (`0084e3a0`) is dead when the health of its base form
/// (`TESHealthForm::GetFormHealth`, `004873d0`, an unsigned number converted
/// to `float`) is not above the double at `01012060`; any other reference
/// answers with `00477ba0`. The word it pops is never read.
pub fn tes_object_refr_is_dead(e: &mut Engine, this: Ptr<TESObjectREFR>, _unused_1: u32) -> bool {
    let me = this.addr();
    if is_actor(e, me) && e.call(FORM_ID, &args![me]).u32() != 7 {
        let base = e.call(GET_BASE_FORM, &args![me]).u32();
        let health = e.call(0x0048_73d0, &args![base]).u32();
        // FILD of the (zero extended) number, FSTP to a float, FCOMP with the
        // double: true for "less" and "equal", false for "greater" and for
        // an unordered result
        let health = f64::from(health as f32);
        health <= e.global::<f64>(0x0101_2060)
    } else {
        e.call(0x0047_7ba0, &args![me]).bool()
    }
}

// Translated from 00572350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::Is3DCritical` (Xbox PDB): true for a reference that is
/// not an actor (`IsActor`, virtual at `+0x100`).
pub fn tes_object_refr_is_3d_critical(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    !is_actor(e, this.addr())
}

// Translated from 00572380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Distance from the reference `this` to the position `position` (a
/// `NiPoint3`): the difference `position - this->Location` (`+0x30`) is built
/// by `00439ef0` in a local and its length is `00457990` (returned in
/// `ST0`).
pub fn fn_00572380(e: &mut Engine, this: Ptr<TESObjectREFR>, position: u32) -> f32 {
    let me = this.addr();
    e.with_stack(0xc, |e, difference| {
        // OBJ_REFR::Location (Xbox PDB) +0x30
        e.call(0x0043_9ef0, &args![position, difference, me + 0x30]);
        e.call(0x0045_7990, &args![difference]).f32()
    })
}

// Translated from 005723b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetDistanceFromReference` (Xbox PDB): the distance from
/// `this` to `other` ([`fn_00572380`] on the position the virtual at
/// `+0x1f4` of `other` gives), or the float at `01016970` (the largest
/// float) when there is no `other`, when it is disabled (unless
/// `include_disabled` is set), when it is deleted, or when the two are not
/// in the same space. Unless `ignore_cells` is set, the two parent cells
/// (`008d6f30`) must be the same cell when either is an interior (`00425fd0`)
/// and otherwise the two world spaces (`TESObjectCELL::GetWorldSpace`,
/// `0054ddd0`; `TESObjectREFR::GetWorldSpace`, `00575d70` for a reference
/// without a cell) must be the same non-null one.
pub fn tes_object_refr_get_distance_from_reference(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    other: Ptr<TESObjectREFR>,
    include_disabled: u8,
    ignore_cells: u8,
) -> f32 {
    let me = this.addr();
    let other = other.addr();
    let far_away = e.global::<f32>(0x0101_6970);
    if other == 0 {
        return far_away;
    }
    if e.call(FORM_IS_DISABLED, &args![other]).bool() && include_disabled == 0 {
        return far_away;
    }
    if e.call(FORM_IS_DELETED, &args![other]).bool() {
        return far_away;
    }
    let mut same_space = true;
    if ignore_cells == 0 {
        let mut interior_checked = false;
        let other_cell = e.call(GET_PARENT_CELL, &args![other]).u32();
        let this_cell = e.call(GET_PARENT_CELL, &args![me]).u32();
        if other_cell != 0
            && this_cell != 0
            && (e.call(0x0042_5fd0, &args![other_cell]).bool()
                || e.call(0x0042_5fd0, &args![this_cell]).bool())
        {
            if other_cell != this_cell {
                same_space = false;
            }
            interior_checked = true;
        }
        if same_space && !interior_checked {
            let other_space = if other_cell != 0 {
                e.call(0x0054_ddd0, &args![other_cell]).u32()
            } else {
                e.call(0x0057_5d70, &args![me]).u32()
            };
            let this_space = if this_cell != 0 {
                e.call(0x0054_ddd0, &args![this_cell]).u32()
            } else {
                e.call(0x0057_5d70, &args![other]).u32()
            };
            if other_space != this_space || other_space == 0 {
                same_space = false;
            }
        }
    }
    if same_space || ignore_cells != 0 {
        let position = e.vcall(other, 0x1f4, &args![]).u32();
        fn_00572380(e, this, position)
    } else {
        far_away
    }
}

// Translated from 00572500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetDeltaForAngle` (Xbox PDB): writes to `out` the vector
/// `(0, 1, 0)` rotated by the angle `rotation.z + angle`, where `rotation`
/// is the `NiPoint3` at `+0x24` (`00430830`) and the sum is clamped by
/// `ClampAngle` (`004b1480`): a matrix is built from the angle (`004a0c90`),
/// applied to the vector (`004b4500`) and the result is adjusted by
/// `004a0c10`. Returns `out`.
pub fn tes_object_refr_get_delta_for_angle(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    out: Ptr,
    angle: f32,
) -> Ptr {
    let me = this.addr();
    let rotation = e.call(0x0043_0830, &args![me]).u32();
    let sum = (f64::from(e.mem.f32(rotation + 8)) + f64::from(angle)) as f32;
    let clamped = e.call(0x004b_1480, &args![sum]).f32();
    // the locals: the vector (`ebp-0x34`), the rotated copy (`ebp-0x40`) and
    // the 3x3 matrix (`ebp-0x28`)
    e.with_stack(0x3c, |e, frame| {
        let vector = frame.addr();
        let rotated_copy = vector + 0xc;
        let matrix = vector + 0x18;
        e.call(0x0041_6870, &args![vector, 0.0f32, 1.0f32, 0.0f32]);
        e.call(NOTHING_CONSTRUCTOR, &args![matrix]);
        e.call(0x004a_0c90, &args![matrix, clamped]);
        let rotated = e
            .call(0x004b_4500, &args![matrix, rotated_copy, vector])
            .u32();
        copy_words(e, vector, rotated, 3);
        e.call(0x004a_0c10, &args![vector]);
        copy_words(e, out.addr(), vector, 3);
    });
    out
}

// Translated from 005725b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the form type of the reference (`cFormType`, `00401170`) is in
/// `0x3d..=0x40` or is `0x69`.
pub fn fn_005725b0(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    let kind = e.call(FORM_TYPE, &args![this]).i32();
    (0x3d..=0x40).contains(&kind) || kind == 0x69
}

// Translated from 005725f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList` accessor `00418250` on the reference's extra list; its
/// result (the pointer the other light functions read) is returned.
pub fn fn_005725f0(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let extra = extra_list(e, this.addr());
    e.call(0x0041_8250, &args![extra]).u32()
}

// Translated from 005726c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 8-byte holder `fn_00572610` and `fn_00572770` create:
/// `00633c90(this, 0)`; returns `this`.
pub fn fn_005726c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0063_3c90, &args![this, 0u32]);
    this
}

/// What `fn_00572610` and `fn_00572770` share: allocates the 8-byte holder
/// (`00401000`, cdecl), constructs it ([`fn_005726c0`]), stores `value` in
/// it (`0066b0d0`, the `NiPointer` assignment), sets the float at `+4` to
/// 1.0 and passes it to `setter` on the extra list of the reference. The C++
/// exception frame of the original is not translated; a failed allocation is
/// followed by the stores at `+4` of a null pointer, as in the game.
fn store_light_holder(e: &mut Engine, me: u32, value: u32, setter: u32) {
    let holder = e.call(0x0040_1000, &args![8u32]).u32();
    let holder = if holder == 0 {
        0
    } else {
        fn_005726c0(e, Ptr::new(holder)).addr()
    };
    e.call(0x0066_b0d0, &args![holder, value]);
    e.mem.set_f32(holder + 4, 1.0);
    let extra = extra_list(e, me);
    e.call(setter, &args![extra, holder]);
}

// Translated from 00572610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the 8-byte holder around `value` (see [`store_light_holder`]) and
/// hands it to `00418e30` on the extra list of the reference.
pub fn fn_00572610(e: &mut Engine, this: Ptr<TESObjectREFR>, value: u32) {
    store_light_holder(e, this.addr(), value, 0x0041_8e30);
}

// Translated from 005726e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetLight` (Xbox PDB): for a node, looks up the child
/// named `"##LightOff"` (`01030e54`) and `"##LightOn"` (`01030e48`) with
/// `004aae30` and, on the one it finds, calls `00450f90` with `flag` (for the
/// first) or with `flag == 0` (for the second). `this` is not read.
pub fn tes_object_refr_set_light(e: &mut Engine, _this: Ptr<TESObjectREFR>, node: u32, flag: u8) {
    if node == 0 {
        return;
    }
    let off = e.call(0x004a_ae30, &args![node, 0x0103_0e54u32]).u32();
    if off != 0 {
        e.call(0x0045_0f90, &args![off, u32::from(flag)]);
    }
    let on = e.call(0x004a_ae30, &args![node, 0x0103_0e48u32]).u32();
    if on != 0 {
        e.call(0x0045_0f90, &args![on, u32::from(flag == 0)]);
    }
}

// Translated from 00572750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetSpellEffectLight` (Xbox PDB): `00418280` on the
/// extra list of the reference.
pub fn tes_object_refr_get_spell_effect_light(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let extra = extra_list(e, this.addr());
    e.call(0x0041_8280, &args![extra]).u32()
}

// Translated from 00572770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Same as [`fn_00572610`] with the setter `00418f60`.
pub fn fn_00572770(e: &mut Engine, this: Ptr<TESObjectREFR>, value: u32) {
    store_light_holder(e, this.addr(), value, 0x0041_8f60);
}

/// The light holder the extra list keeps for the reference: `00418280`
/// (spell effect light) when `spell_effect` is set, else `00418250`.
fn light_holder(e: &mut Engine, me: u32, spell_effect: bool) -> u32 {
    let extra = extra_list(e, me);
    if spell_effect {
        e.call(0x0041_8280, &args![extra]).u32()
    } else {
        e.call(0x0041_8250, &args![extra]).u32()
    }
}

// Translated from 00572820 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a reference whose base object has form type `0x1e`, calls `0050de20`
/// on that base with the holder from `00418250` and again with the one from
/// `00418280` (each only when the holder is set), and 0.
pub fn fn_00572820(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    let extra = extra_list(e, me);
    let light = e.call(0x0041_8250, &args![extra]).u32();
    let mut base_light = 0u32;
    if e.call(GET_BASE_FORM, &args![me]).u32() != 0 {
        let base = e.call(GET_BASE_FORM, &args![me]).u32();
        if e.call(FORM_TYPE, &args![base]).i32() == 0x1e {
            base_light = e.call(GET_BASE_FORM, &args![me]).u32();
        }
    }
    if light != 0 && base_light != 0 {
        e.call(0x0050_de20, &args![base_light, light, 0u32]);
    }
    let extra = extra_list(e, me);
    let spell_light = e.call(0x0041_8280, &args![extra]).u32();
    if spell_light != 0 && base_light != 0 {
        e.call(0x0050_de20, &args![base_light, spell_light, 0u32]);
    }
}

// Translated from 005728c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the light of the reference to the shadow scene node
/// (`ShadowSceneNode::AddLight`, `00b5c940`, on the node
/// `00450b80(0)` gives): nothing happens for a deleted or disabled
/// reference, when the holder (`00418250`, or `00418280` when `spell_effect`
/// is set) is missing or empty, or when the reference has lit-water
/// references (`0041f810`) that are not empty (`008256d0`). The shadow flag
/// is 1, or `0050dd90` on the base object when its form type is `0x1e`.
pub fn fn_005728c0(e: &mut Engine, this: Ptr<TESObjectREFR>, spell_effect: u8) {
    let me = this.addr();
    if e.call(FORM_IS_DELETED, &args![me]).bool() || e.call(FORM_IS_DISABLED, &args![me]).bool() {
        return;
    }
    let light = light_holder(e, me, spell_effect != 0);
    if light == 0 || e.call(READ_FIRST_DWORD, &args![light]).u32() == 0 {
        return;
    }
    let extra = extra_list(e, me);
    if e.call(0x0041_f810, &args![extra]).u32() != 0 {
        let extra = extra_list(e, me);
        if e.call(0x0041_f810, &args![extra]).u32() == 0 {
            return;
        }
        let extra = extra_list(e, me);
        let lit_water = e.call(0x0041_f810, &args![extra]).u32();
        if !e.call(0x0082_56d0, &args![lit_water]).bool() {
            return;
        }
    }
    let mut shadow = 1u8;
    if e.call(GET_BASE_FORM, &args![me]).u32() != 0 {
        let base = e.call(GET_BASE_FORM, &args![me]).u32();
        if e.call(FORM_TYPE, &args![base]).i32() == 0x1e {
            let base = e.call(GET_BASE_FORM, &args![me]).u32();
            shadow = e.call(0x0050_dd90, &args![base]).u8();
        }
    }
    let node = e.call(READ_FIRST_DWORD, &args![light]).u32();
    let scene = e.call(0x0045_0b80, &args![0u32]).u32();
    e.call(0x00b5_c940, &args![scene, node, u32::from(shadow)]);
}

// Translated from 005729e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::RemoveLight` (Xbox PDB): takes the light of the holder
/// (`00418250`, or `00418280` when `spell_effect` is set) out of the shadow
/// scene node (`ShadowSceneNode::RemoveLight`, `00b5eed0`, on the node
/// `00450b80(0)` gives).
pub fn tes_object_refr_remove_light(e: &mut Engine, this: Ptr<TESObjectREFR>, spell_effect: u8) {
    let light = light_holder(e, this.addr(), spell_effect != 0);
    if light != 0 && e.call(READ_FIRST_DWORD, &args![light]).u32() != 0 {
        let node = e.call(READ_FIRST_DWORD, &args![light]).u32();
        let scene = e.call(0x0045_0b80, &args![0u32]).u32();
        e.call(0x00b5_eed0, &args![scene, node]);
    }
}

// Translated from 00572a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::KillLight` (Xbox PDB): like [`tes_object_refr_remove_light`]
/// and then clears the holder (`0066b0d0(holder, 0)`); whenever there is a
/// holder, `0041ae50` (`spell_effect` set) or `0041ae30` runs on the extra
/// list afterwards.
pub fn tes_object_refr_kill_light(e: &mut Engine, this: Ptr<TESObjectREFR>, spell_effect: u8) {
    let me = this.addr();
    let light = light_holder(e, me, spell_effect != 0);
    if light != 0 {
        if e.call(READ_FIRST_DWORD, &args![light]).u32() != 0 {
            let node = e.call(READ_FIRST_DWORD, &args![light]).u32();
            let scene = e.call(0x0045_0b80, &args![0u32]).u32();
            e.call(0x00b5_eed0, &args![scene, node]);
            e.call(0x0066_b0d0, &args![light, 0u32]);
        }
        let extra = extra_list(e, me);
        if spell_effect != 0 {
            e.call(0x0041_ae50, &args![extra]);
        } else {
            e.call(0x0041_ae30, &args![extra]);
        }
    }
}

// Translated from 00572b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the `NiPoint3` at `+0x18` of the extra data of type `0xf` of the
/// reference (`0041b080`; it is created when missing) into `out` and
/// returns `out`.
pub fn fn_00572b00(e: &mut Engine, this: Ptr<TESObjectREFR>, out: Ptr) -> Ptr {
    let me = this.addr();
    let extra = extra_list(e, me);
    e.call(0x0041_b080, &args![extra, out, me]);
    out
}

// Translated from 00572b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the `NiPoint3` at `+0xc` of the extra data of type `0xf` of the
/// reference (`0041b0d0`; it is created when missing) into `out` and
/// returns `out`.
pub fn fn_00572b30(e: &mut Engine, this: Ptr<TESObjectREFR>, out: Ptr) -> Ptr {
    let me = this.addr();
    let extra = extra_list(e, me);
    e.call(0x0041_b0d0, &args![extra, out, me]);
    out
}

// Translated from 00572b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the starting position and rotation in the extra data of type
/// `0xf` of the reference, under a scope lock (`00404eb0`: `(0x31, 1,
/// "...TESObjectREFR.cpp", 0x3b01)`, released by `00404ee0`). The
/// `NiPoint3` passed by value (`x`, `y`, `z`) is stored with `0041b180`
/// unless it equals the vector at `011f426c` (`00439090` says whether it
/// differs), in which case the position of the reference itself (virtual at
/// `+0x1f4`) is stored; the rotation (`00430830`, the `NiPoint3` at `+0x24`)
/// is stored with `0041b120`.
pub fn fn_00572b60(e: &mut Engine, this: Ptr<TESObjectREFR>, x: u32, y: u32, z: u32) {
    let me = this.addr();
    e.with_stack(0x10, |e, guard| {
        e.call(
            0x0040_4eb0,
            &args![guard, 0x31u32, 1u32, 0x0102_fc20u32, 0x3b01u32],
        );
        // the vector argument lives in the caller's stack words
        let given = e.with_stack(0xc, |e, vector| {
            e.mem.set_u32(vector.addr(), x);
            e.mem.set_u32(vector.addr() + 4, y);
            e.mem.set_u32(vector.addr() + 8, z);
            e.call(0x0043_9090, &args![vector, 0x011f_426cu32]).bool()
        });
        let (px, py, pz) = if given {
            (x, y, z)
        } else {
            let position = e.vcall(me, 0x1f4, &args![]).u32();
            (
                e.mem.u32(position),
                e.mem.u32(position + 4),
                e.mem.u32(position + 8),
            )
        };
        e.with_stack(0xc, |e, result| {
            let extra = extra_list(e, me);
            e.call(0x0041_b180, &args![extra, result, me, px, py, pz]);
        });
        let rotation = e.call(0x0043_0830, &args![me]).u32();
        let (rx, ry, rz) = (
            e.mem.u32(rotation),
            e.mem.u32(rotation + 4),
            e.mem.u32(rotation + 8),
        );
        e.with_stack(0xc, |e, result| {
            let extra = extra_list(e, me);
            e.call(0x0041_b120, &args![extra, result, me, rx, ry, rz]);
        });
        e.call(0x0040_4ee0, &args![guard]);
    });
}

// Translated from 00572c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the form id (`0084e3a0`) is above `0xfeffffff` as
/// `TESDataHandler` (`011c3f2c`, `00469860`) says; otherwise by the form type
/// of the base object: `0x20` asks `00444ed0`, `0x21` and `0x25..=0x27` are
/// false, all others true.
pub fn fn_00572c80(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    let me = this.addr();
    let id = e.call(FORM_ID, &args![me]).u32();
    let handler = e.global::<u32>(GLOBAL_DATA_HANDLER);
    if e.call(0x0046_9860, &args![handler, id]).bool() {
        return true;
    }
    let base = e.call(GET_BASE_FORM, &args![me]).u32();
    match e.call(FORM_TYPE, &args![base]).i32() {
        0x20 => e.call(0x0044_4ed0, &args![me]).bool(),
        0x21 | 0x25..=0x27 => false,
        _ => true,
    }
}

// Translated from 00572d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte `0041b370` reads from the extra data of type `0xe` of the
/// reference (1 when it has none).
pub fn fn_00572d10(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u8 {
    let extra = extra_list(e, this.addr());
    e.call(0x0041_b370, &args![extra]).u8()
}

// Translated from 00572d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether any bit of `mask` is set in the byte of [`fn_00572d10`]
/// (`0041b3a0` on the extra list).
pub fn fn_00572d30(e: &mut Engine, this: Ptr<TESObjectREFR>, mask: u32) -> bool {
    let extra = extra_list(e, this.addr());
    e.call(0x0041_b3a0, &args![extra, mask]).bool()
}

// Translated from 00572d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the bits `mask` in the same byte (`0041b440`). For the mask 4 the
/// reference first changes its form flag `0x800000`: removal
/// (`RemoveChange`, virtual at `+0x4c`) when its original open-by-default
/// state (`00561d90`) is set, addition (`AddChange`, `+0x48`) otherwise.
pub fn fn_00572d50(e: &mut Engine, this: Ptr<TESObjectREFR>, mask: u32) {
    let me = this.addr();
    if mask == 4 {
        let slot = if e.call(0x0056_1d90, &args![me]).bool() {
            0x4c
        } else {
            0x48
        };
        e.vcall(me, slot, &args![0x0080_0000u32]);
    }
    let extra = extra_list(e, me);
    e.call(0x0041_b440, &args![extra, mask]);
}

// Translated from 00572db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::ClearAction` (Xbox PDB): clears the bits `mask` of the
/// byte (`0041b470`). For the mask 4 the reference first changes its form
/// flag `0x800000` the other way round from [`fn_00572d50`]: `AddChange`
/// (`+0x48`) when the original open-by-default state (`00561d90`) is set,
/// `RemoveChange` (`+0x4c`) otherwise.
pub fn tes_object_refr_clear_action(e: &mut Engine, this: Ptr<TESObjectREFR>, mask: u32) {
    let me = this.addr();
    if mask == 4 {
        let slot = if e.call(0x0056_1d90, &args![me]).bool() {
            0x48
        } else {
            0x4c
        };
        e.vcall(me, slot, &args![0x0080_0000u32]);
    }
    let extra = extra_list(e, me);
    e.call(0x0041_b470, &args![extra, mask]);
}

// Translated from 00572e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `reference` in the extra data of type `0xe` (`0041b4b0`): it is
/// created when missing and `reference` is not null, and removed when it has
/// the default byte and `reference` is null.
pub fn fn_00572e10(e: &mut Engine, this: Ptr<TESObjectREFR>, reference: u32) {
    let extra = extra_list(e, this.addr());
    e.call(0x0041_b4b0, &args![extra, reference]);
}

// Translated from 00572e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetActionRef` (Xbox PDB): `ExtraDataList::GetActionRef`
/// (`0041b520`) on the extra list.
pub fn tes_object_refr_get_action_ref(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let extra = extra_list(e, this.addr());
    e.call(0x0041_b520, &args![extra]).u32()
}

// Translated from 00572e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetAnimation` (Xbox PDB): for a mobile object
/// (`IsMobileObject`, virtual at `+0xfc`) first changes the form flag
/// `0x10000000` (`AddChange`, `00484b90`, when `animation` is not null;
/// `RemoveChange`, virtual at `+0x4c`, otherwise), then, when the process
/// (`008d8520`) has a level (`0045cd60`) of 0 or 1, hands the animation to
/// the virtual at `+0x820` of that process (after `00931920(actor, 0)` for
/// any other level when `animation` is set) and returns. For all other
/// references the animation of the extra list (`00418220`) is replaced:
/// by `00418c40` when `animation` is set, removed by `0041adf0` when not.
/// Returns `animation`.
pub fn tes_object_refr_set_animation(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    animation: u32,
) -> u32 {
    let me = this.addr();
    let mut actor = 0u32;
    if e.vcall(me, 0xfc, &args![]).bool() {
        actor = me;
    }
    if actor != 0 {
        if animation != 0 {
            e.call(0x0048_4b90, &args![me, 0x1000_0000u32]);
        } else {
            e.vcall(me, 0x4c, &args![0x1000_0000u32]);
        }
    }
    if actor != 0 {
        let mut process = 0u32;
        if e.call(GET_PROCESS, &args![actor]).u32() != 0 {
            let current = e.call(GET_PROCESS, &args![actor]).u32();
            let level = e.call(0x0045_cd60, &args![current]).i32();
            if level != 1 && level != 0 && animation != 0 {
                e.call(0x0093_1920, &args![actor, 0u32]);
            }
            let current = e.call(GET_PROCESS, &args![actor]).u32();
            let level = e.call(0x0045_cd60, &args![current]).i32();
            if (0..=1).contains(&level) {
                process = e.call(GET_PROCESS, &args![actor]).u32();
            }
        }
        if process != 0 {
            e.vcall(process, 0x820, &args![animation]);
            return animation;
        }
    }
    let handler = e.global::<u32>(GLOBAL_DATA_HANDLER);
    fn_00572fa0(e, Ptr::new(handler));
    let extra = extra_list(e, me);
    let current = e.call(0x0041_8220, &args![extra]).u32();
    if current == animation {
        return animation;
    }
    let extra = extra_list(e, me);
    if animation != 0 {
        e.call(0x0041_8c40, &args![extra, animation]);
    } else {
        e.call(0x0041_adf0, &args![extra]);
    }
    animation
}

// Translated from 00572fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x61f` of the object (the `TESDataHandler` in the only
/// call).
pub fn fn_00572fa0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x61f)
}

// Translated from 00572fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetDismembered` (Xbox PDB): on the dismemberment extra of
/// the reference (`ExtraDataList::AddDismembermentExtra`, `0042e820`):
/// `ExtraDismemberedLimbs::Dismember(this, limb, flag)` (`00430410`) unless
/// `limb` is -1, then `00437730(arg_10)`, `00600ab0(arg_0c)` and
/// `00984f60` with `limb`, or with `arg_14` when `limb` is -1; finally
/// `AddChange` (virtual at `+0x48`) with the flag `0x20000`. The argument
/// names are the stack offsets of the words.
pub fn tes_object_refr_set_dismembered(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    limb: i32,
    arg_0c: u32,
    arg_10: u32,
    arg_14: u32,
    flag: u8,
) {
    let me = this.addr();
    let extra = extra_list(e, me);
    let dismembered = e.call(0x0042_e820, &args![extra]).u32();
    if limb != -1 {
        e.call(
            0x0043_0410,
            &args![dismembered, me, limb as u32, u32::from(flag)],
        );
    }
    e.call(0x0043_7730, &args![dismembered, arg_10]);
    e.call(0x0060_0ab0, &args![dismembered, arg_0c]);
    let chosen = if limb == -1 { arg_14 } else { limb as u32 };
    e.call(0x0098_4f60, &args![dismembered, chosen]);
    e.vcall(me, 0x48, &args![0x0002_0000u32]);
}

// Translated from 00573050 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the reference has a dismemberment extra (`0042e8c0`), calls
/// `004305f0(extra, limb, flag)` on it.
pub fn fn_00573050(e: &mut Engine, this: Ptr<TESObjectREFR>, limb: u32, flag: u8) {
    let extra = extra_list(e, this.addr());
    let dismembered = e.call(0x0042_e8c0, &args![extra]).u32();
    if dismembered != 0 {
        e.call(0x0043_05f0, &args![dismembered, limb, u32::from(flag)]);
    }
}

// Translated from 00573090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetDismembered` (Xbox PDB): false without a
/// dismemberment extra (`0042e8c0`), else `ExtraDismemberedLimbs::Dismembered`
/// (`004303e0`) for the limb.
pub fn tes_object_refr_get_dismembered(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    limb: u32,
) -> bool {
    let extra = extra_list(e, this.addr());
    let dismembered = e.call(0x0042_e8c0, &args![extra]).u32();
    if dismembered == 0 {
        false
    } else {
        e.call(0x0043_03e0, &args![dismembered, limb]).bool()
    }
}

// Translated from 005730d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `+0x10` (`0044edb0`) of the dismemberment extra of the
/// reference (`0042e8c0`), or -1 without one.
pub fn fn_005730d0(e: &mut Engine, this: Ptr<TESObjectREFR>) -> i32 {
    let extra = extra_list(e, this.addr());
    let dismembered = e.call(0x0042_e8c0, &args![extra]).u32();
    if dismembered != 0 {
        e.call(0x0044_edb0, &args![dismembered]).i32()
    } else {
        -1
    }
}

// Translated from 00573110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetLastHitLimb` (Xbox PDB): the word at `+0x18`
/// (`009611e0`) of the dismemberment extra of the reference (`0042e8c0`), or
/// -1 without one.
pub fn tes_object_refr_get_last_hit_limb(e: &mut Engine, this: Ptr<TESObjectREFR>) -> i32 {
    let extra = extra_list(e, this.addr());
    let dismembered = e.call(0x0042_e8c0, &args![extra]).u32();
    if dismembered != 0 {
        e.call(0x0096_11e0, &args![dismembered]).i32()
    } else {
        -1
    }
}

// Translated from 00573150 (decompiled, FalloutNV.exe 1.4.0.525)
/// The virtual at `+0x1e8` of the reference; its result is returned.
pub fn fn_00573150(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    e.vcall(this.addr(), 0x1e8, &args![]).u32()
}

/// The player (`PlayerCharacter *`, `011dea3c`), read each time as the
/// game does.
fn player(e: &Engine) -> u32 {
    e.global::<u32>(GLOBAL_PLAYER)
}

/// The text `00403df0` reads from the setting `holder`, shown by the cdecl `007052f0`
/// with `(0, text, 0, <float at 010162c0>, 0)`.
fn show_message(e: &mut Engine, holder: u32, text: u32) {
    let seconds = e.global::<f32>(0x0101_62c0);
    // 005732f6 / 00573460: `00403df0` takes only the setting (ECX); the five pushed
    // words are the rest of the cdecl `007052f0` (ADD ESP,0x18).
    let message = e.call(0x0040_3df0, &args![holder]).u32();
    e.call(
        0x0070_52f0,
        &args![message, 0u32, text, 0u32, seconds, 0u32],
    );
}

// Translated from 00573170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::Activate` (Xbox PDB virtual `+0x128` is the activation of
/// the base object that this calls): `activator` activates the reference.
///
/// 1. An ash pile or similar (base object equal to the word at `011ca27c` or
///    `011ca280`) with an ash-pile reference in its extra list
///    (`0041e310`) hands the whole activation to that reference.
/// 2. The byte at `011ca428` ("activated") is cleared. The activator is
///    *busy* when it is an actor whose process (`008d8520`) has level
///    `0045cd60 == 0` and answers 4 to its virtual at `+0x610`.
/// 3. The player as activator is turned away (true, nothing happens) while
///    the pipboy is opening (`00709bc0`), false when `004eaf60` and
///    [`fn_005737e0`] differ, and with a message when its virtual `+0x1a0`
///    holds and `008859e0` does not.
/// 4. Several refusals follow (`00477ba0` on a non-actor reference gives
///    true; base form type `0x1c` activated by an actor with a zero byte at
///    `0055d520`, the player activating with that byte zero unless the base
///    form type is `0x23` or `0x16`, and `0057b460` give false; a locked
///    reference (`0041eb60`) that `0041e7b0` does not open shows a message to
///    the player and gives false).
/// 5. An activator that is not busy logs "'%s' activated %s '%s'" when the
///    reference is the one under the cursor (`00703180`); a reference other
///    than the player that is not yet flagged (`fn_00572d30(1)`) is then
///    flagged (`fn_00572d50(2)`, `fn_00572e10(activator)`), runs its scripts
///    (`00565870`, at most 5 deep through the counter at `011ca424`) and
///    returns the byte at `011ca428`.
/// 6. Otherwise `004213c0(this, activator)` and the removal of the extra of
///    type `0x7c` (`00410140`) precede the switch on the base form type: no
///    base is false; `0x20`, `0x21`, `0x22`, `0x25` are false; `0x30` is
///    true; `0x1c` (doors) first sends a non-sleeping player through the
///    teleport cell of the door (`00568e50`, `0043a2b0`, `0054af80`); all
///    other types and the doors then run `0056a290` and the virtual `+0x124`
///    of the base object, which on success sets the byte at `011ca428` and
///    gives true.
///
/// The `flag`, `arg_10` and `arg_14` arguments are only forwarded. The stack
/// cookie check of the original is not translated.
pub fn tes_object_refr_activate(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    activator: u32,
    flag: u8,
    arg_10: u32,
    arg_14: u32,
) -> bool {
    let me = this.addr();
    let mut busy = false;
    if e.call(GET_BASE_FORM, &args![me]).u32() == e.global::<u32>(0x011c_a27c)
        || e.call(GET_BASE_FORM, &args![me]).u32() == e.global::<u32>(0x011c_a280)
    {
        let extra = extra_list(e, me);
        let ash_pile = e.call(0x0041_e310, &args![extra]).u32();
        if ash_pile != 0 {
            return tes_object_refr_activate(
                e,
                Ptr::new(ash_pile),
                activator,
                flag,
                arg_10,
                arg_14,
            );
        }
    }
    e.set_global::<u8>(0x011c_a428, 0);
    if activator != 0 && is_actor(e, activator) && e.call(GET_PROCESS, &args![activator]).u32() != 0
    {
        let process = e.call(GET_PROCESS, &args![activator]).u32();
        if e.call(0x0045_cd60, &args![process]).u32() == 0 {
            let process = e.call(GET_PROCESS, &args![activator]).u32();
            if e.vcall(process, 0x610, &args![]).u32() == 4 {
                busy = true;
            }
        }
    }
    if activator == player(e) {
        if e.call(0x0070_9bc0, &args![]).bool() {
            return true;
        }
        let current = player(e);
        let first = e.call(0x004e_af60, &args![current]).u8();
        let current = player(e);
        let second = fn_005737e0(e, Ptr::new(current));
        if first != second {
            return false;
        }
        let current = player(e);
        if e.vcall(current, 0x1a0, &args![0u32]).bool() {
            let current = player(e);
            if !e.call(0x0088_59e0, &args![current, me]).bool() {
                show_message(e, 0x011d_4f84, 0);
                return true;
            }
        }
    }
    if e.call(0x0047_7ba0, &args![me]).bool() && !is_actor(e, me) {
        return true;
    }
    if e.call(GET_BASE_FORM, &args![me]).u32() != 0 {
        let base = e.call(GET_BASE_FORM, &args![me]).u32();
        if e.call(FORM_TYPE, &args![base]).i32() == 0x1c && activator != 0 && is_actor(e, activator)
        {
            let state = e.call(0x0055_d520, &args![me]).u32();
            if e.mem.u8(state) == 0 {
                return false;
            }
        }
    }
    if activator == player(e) {
        let state = e.call(0x0055_d520, &args![me]).u32();
        if e.mem.u8(state) == 0 {
            let base = e.call(GET_BASE_FORM, &args![me]).u32();
            if e.call(FORM_TYPE, &args![base]).i32() != 0x23 {
                let base = e.call(GET_BASE_FORM, &args![me]).u32();
                if e.call(FORM_TYPE, &args![base]).i32() != 0x16 {
                    return false;
                }
            }
        }
    }
    if e.call(0x0057_b460, &args![me]).bool() {
        return false;
    }
    let extra = extra_list(e, me);
    if e.call(0x0041_eb60, &args![extra]).bool() {
        let extra = extra_list(e, me);
        if e.call(0x0041_e7b0, &args![extra, activator]).u32() == 0 {
            if activator == player(e) {
                show_message(e, 0x011d_2bf8, 0x0103_0e78);
            }
            return false;
        }
    }
    if activator != 0 && !busy {
        if e.call(0x0070_3180, &args![]).u32() == me {
            let base = e.call(GET_BASE_FORM, &args![me]).u32();
            let this_name = e.call(0x0048_2720, &args![base]).u32();
            let base = e.call(GET_BASE_FORM, &args![me]).u32();
            let verb_source = e.call(0x0044_0e30, &args![base]).u32();
            let verb = e.call(0x0046_4f30, &args![verb_source, 0u32]).u32();
            let activator_base = e.call(GET_BASE_FORM, &args![activator]).u32();
            let activator_name = e.call(0x0048_2720, &args![activator_base]).u32();
            e.with_stack(0x10c, |e, text| {
                e.call(
                    SPRINTF,
                    &args![text, 0x0103_0e60u32, activator_name, verb, this_name],
                );
                e.call(0x0070_3c00, &args![text]);
            });
        }
        if me != player(e) && !fn_00572d30(e, this, 1) {
            fn_00572d50(e, this, 2);
            fn_00572e10(e, this, activator);
            if e.global::<i32>(0x011c_a424) < 5 {
                let depth = e.global::<i32>(0x011c_a424);
                e.set_global::<i32>(0x011c_a424, depth + 1);
                e.call(0x0056_5870, &args![me]);
                let depth = e.global::<i32>(0x011c_a424);
                e.set_global::<i32>(0x011c_a424, depth - 1);
            }
            if e.global::<u8>(0x011c_a428) == 0 && activator != 0 && is_actor(e, activator) {
                e.call(0x008b_c980, &args![activator, me, arg_10, arg_14, 1u32]);
            }
            return e.global::<u8>(0x011c_a428) != 0;
        }
    }
    let extra = extra_list(e, me);
    e.call(0x0042_13c0, &args![extra, me, activator]);
    let extra = extra_list(e, me);
    e.call(0x0041_0140, &args![extra, 0x7cu32]);
    if e.call(GET_BASE_FORM, &args![me]).u32() == 0 {
        return false;
    }
    let base = e.call(GET_BASE_FORM, &args![me]).u32();
    let kind = e.call(FORM_TYPE, &args![base]).i32();
    match kind {
        0x20 | 0x21 | 0x22 | 0x25 => return false,
        0x30 => return true,
        0x1c => {
            let door = e.call(0x0056_8e50, &args![me]).u32();
            if door != 0 && activator == player(e) {
                let current = player(e);
                let process = e.call(GET_PROCESS, &args![current]).u32();
                if e.vcall(process, 0x610, &args![]).u32() != 4 {
                    let cell = e.call(0x0043_a2b0, &args![door]).u32();
                    if cell != 0 && e.call(0x0042_5fd0, &args![cell]).bool() {
                        e.call(0x0054_af80, &args![cell]);
                    }
                }
            }
        }
        _ => {}
    }
    e.call(0x0056_a290, &args![me, activator]);
    // TESObjectREFR::data.pObjectReference (Xbox PDB) +0x20: the base object
    let base_object = e.mem.u32(me + 0x20);
    let activated = e
        .vcall(
            base_object,
            0x124,
            &args![me, activator, u32::from(flag), arg_10, arg_14],
        )
        .bool();
    if !activated {
        return false;
    }
    if activator != 0 && is_actor(e, activator) {
        e.call(0x008b_c980, &args![activator, me, arg_10, arg_14, 0u32]);
        if activator != player(e) {
            let current = player(e);
            e.call(0x0095_2c30, &args![current, activator]);
        }
    }
    e.set_global::<u8>(0x011c_a428, 1);
    true
}

// Translated from 005737e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x64c` of the player (`this`).
pub fn fn_005737e0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x64c)
}

// Translated from 00573800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::MoveRefToNewSpace` (Xbox PDB, cdecl): moves `refr` into
/// the cell `cell` (ignored when it is not a valid cell for `00425fd0`) or,
/// without a cell, into the world space `world` at the cell its position
/// falls in (`TESWorldSpace::GetCellFromCellCoord`, `005875a0`, on the
/// position divided by 4096 through `00406d90`). Nothing moves when `refr`
/// is null or both are null. In outline:
///
/// * a mobile object with a character controller (`009306d0`) first gives
///   the controller the height `Location.z` (`00573f20`); a node with a
///   flag set (`006838b0`, `00573eb0`) has it cleared for the duration of
///   the move (`00573ed0`) and restored at the end;
/// * the extra data of saved Havok data is dropped (`00422c20`) when the
///   virtual `+0x22c` says so; the reference is taken out of its old cell
///   (`0054ca90`) or persistent list (`00588030`) and put in the new one
///   (`00548230`, `00587ff0`), recording the starting cell or world
///   (`0041b320`, `0041b2a0`) of a non-actor that has none;
/// * the reference is updated in place, or loaded or unloaded, depending on
///   whether the old and new cells are loaded (`004511e0` on the object
///   `011dea10`), through the virtuals `+0x1c0`, `+0x1c4`, `+0x1fc` and
///   `+0x1cc` or, when `008c7aa0` says so, through the `0087b2xx` helpers;
/// * an actor moved out of every loaded cell is added to the temporary
///   change list (`0096e870` on `011e0e80`) and, at process level 3
///   (`009334b0`), unloaded (`00849730`);
/// * a reference that ends in another cell, is not persistent and is not
///   the player gets the changed flag 8 (virtual `+0x48`).
///
/// The C++ exception frames of the original are not translated.
pub fn tes_object_refr_move_ref_to_new_space(
    e: &mut Engine,
    refr: Ptr<TESObjectREFR>,
    cell: u32,
    world: u32,
) {
    let me = refr.addr();
    let mut cell = cell;
    let mut restore = false;
    let node = e.vcall(me, 0x1d0, &args![]).u32();
    let mut controller = 0u32;
    let mut actor = 0u32;
    let mut flagged = 0u32;
    if e.vcall(me, 0xfc, &args![]).bool() {
        actor = me;
        controller = e.call(0x0093_06d0, &args![actor]).u32();
    }
    if controller != 0 {
        let position = e.vcall(actor, 0x1f4, &args![]).u32();
        let height = e.mem.f32(position + 8);
        e.call(0x0057_3f20, &args![controller, height]);
    }
    if node != 0 {
        flagged = e.call(0x0068_38b0, &args![node]).u32();
        if flagged != 0 && fn_00573eb0(e, Ptr::new(flagged)) {
            fn_00573ed0(e, Ptr::new(flagged), 0);
            restore = true;
        }
    }
    if cell != 0 && !e.call(0x0042_5fd0, &args![cell]).bool() {
        cell = 0;
    }
    if me != 0 && (cell != 0 || world != 0) {
        let mut loaded = false;
        let mut moved = false;
        let old_space = e.call(0x0057_5d70, &args![me]).u32();
        let old_cell = e.call(GET_PARENT_CELL, &args![me]).u32();
        let new_cell;
        if e.vcall(me, 0x22c, &args![0u32]).bool() {
            let extra = extra_list(e, me);
            e.call(0x0042_2c20, &args![extra]);
            e.vcall(me, 0xd4, &args![0u32]);
        }
        if cell != 0 {
            if old_cell != cell {
                moved = true;
                if !is_actor(e, me) {
                    let extra = extra_list(e, me);
                    if e.call(0x0041_b320, &args![extra]).u32() == 0 {
                        let extra = extra_list(e, me);
                        e.call(0x0041_b2a0, &args![extra, me]);
                    }
                }
                if old_space != 0 && e.call(GET_REF_PERSISTS, &args![me]).bool() {
                    e.call(0x0058_8030, &args![old_space, me]);
                }
                e.call(0x0054_8230, &args![cell, me, 0u32]);
            } else {
                update_in_place(e, me);
                if actor != 0 && controller != 0 {
                    let position = e.vcall(actor, 0x1f4, &args![]).u32();
                    e.call(0x0056_20e0, &args![controller, position]);
                }
            }
            let tes = e.global::<u32>(GLOBAL_TES);
            if e.call(0x0045_11e0, &args![tes, cell, 1u32]).bool() {
                loaded = true;
            }
            new_cell = cell;
        } else {
            if old_cell != 0 && e.call(0x0042_5fd0, &args![old_cell]).bool() {
                moved = true;
            }
            if old_space != world && !is_actor(e, me) {
                let extra = extra_list(e, me);
                if e.call(0x0041_b320, &args![extra]).u32() == 0 {
                    let extra = extra_list(e, me);
                    e.call(0x0041_b2a0, &args![extra, me]);
                }
            }
            if e.call(GET_REF_PERSISTS, &args![me]).bool() && old_space != world {
                if old_space != 0 {
                    e.call(0x0058_8030, &args![old_space, me]);
                }
                e.call(0x0058_7ff0, &args![world, me]);
            }
            let position = e.vcall(me, 0x1f4, &args![]).u32();
            let x = e.mem.u32(position);
            let cell_x = e.call(0x0040_6d90, &args![x]).i32() >> 12;
            let position = e.vcall(me, 0x1f4, &args![]).u32();
            let y = e.mem.u32(position + 4);
            let cell_y = e.call(0x0040_6d90, &args![y]).i32() >> 12;
            new_cell = e
                .call(0x0058_75a0, &args![world, cell_x as u32, cell_y as u32])
                .u32();
            if new_cell != 0 {
                e.call(0x0054_8230, &args![new_cell, me, 0u32]);
                let tes = e.global::<u32>(GLOBAL_TES);
                if e.call(0x0045_11e0, &args![tes, new_cell, 1u32]).bool() {
                    if old_space != world {
                        loaded = true;
                        moved = true;
                        if is_actor(e, me) {
                            let current = e.call(GET_PROCESS, &args![me]).u32();
                            if current != 0 {
                                let current = e.call(GET_PROCESS, &args![me]).u32();
                                e.vcall(current, 0x28, &args![]);
                            }
                        }
                    } else if moved && old_cell != 0 {
                        let tes = e.global::<u32>(GLOBAL_TES);
                        if e.call(0x0045_11e0, &args![tes, old_cell, 1u32]).bool() {
                            update_in_place(e, me);
                        }
                    }
                }
            } else {
                if old_cell != 0 {
                    e.call(0x0054_ca90, &args![old_cell, me]);
                }
                let sleeping = is_actor(e, me) && {
                    let current = player(e);
                    e.call(0x0094_df60, &args![current]).bool()
                };
                if !sleeping {
                    let save_load = e.global::<u32>(GLOBAL_SAVE_LOAD);
                    if !e.call(0x0047_c850, &args![save_load]).bool() {
                        e.vcall(me, 0x1cc, &args![0u32, 0u32]);
                    }
                }
            }
        }
        if actor != 0 && !loaded {
            e.call(0x0096_e870, &args![OBJECT_PROCESS_LISTS, actor]);
            if old_cell != 0
                && new_cell == 0
                && e.call(0x0093_34b0, &args![actor]).u32() == 3
                && !e.call(GET_REF_PERSISTS, &args![actor]).bool()
            {
                e.vcall(actor, 0x228, &args![old_cell]);
                e.vcall(actor, 0x48, &args![8u32]);
                let save_load = e.global::<u32>(GLOBAL_SAVE_LOAD);
                e.call(0x008d_0370, &args![save_load, actor]);
                let unloader = e.global::<u32>(GLOBAL_LOADING_OBJECT);
                e.call(0x0084_9730, &args![unloader, actor, 0u32]);
                e.vcall(actor, 0x228, &args![0u32]);
            }
        }
        if moved {
            if e.call(0x008c_7aa0, &args![]).bool() {
                // 00573d30: `004537b0` takes no word (plain RET, a global); the pushed
                // reference is the word of `0087b2b0` (RET 4).
                let handle = e.call(0x0045_37b0, &args![]).u32();
                e.call(0x0087_b2b0, &args![handle, me]);
            } else {
                e.vcall(me, 0x1c0, &args![]);
            }
        }
        if loaded {
            let mut resume = false;
            if e.vcall(me, 0xfc, &args![]).bool()
                && e.vcall(me, 0x1d0, &args![]).u32() != 0
                && e.call(0x0045_0ff0, &args![new_cell]).bool()
            {
                let process = e.call(GET_PROCESS, &args![actor]).u32();
                if process != 0
                    && (e.call(0x0045_cd60, &args![process]).u32() == 1
                        || e.call(0x0045_cd60, &args![process]).u32() == 0)
                    && e.vcall(process, 0x28c, &args![]).u32() == 0
                {
                    resume = true;
                }
            }
            if resume {
                if e.call(0x008c_7aa0, &args![]).bool() {
                    // 00573df8: as above, the reference is the word of `0087b230`.
                    let handle = e.call(0x0045_37b0, &args![]).u32();
                    e.call(0x0087_b230, &args![handle, me]);
                } else {
                    e.vcall(me, 0x1c4, &args![]);
                }
            } else {
                update_in_place(e, me);
            }
        }
        if e.call(GET_PARENT_CELL, &args![me]).u32() != old_cell
            && !e.call(GET_REF_PERSISTS, &args![me]).bool()
            && me != player(e)
        {
            e.vcall(me, 0x48, &args![8u32]);
        }
    }
    if restore && e.vcall(me, 0x1d0, &args![]).u32() != 0 {
        fn_00573ed0(e, Ptr::new(flagged), 1);
    }
}

/// What `MoveRefToNewSpace` does in four places: when `008c7aa0` says so,
/// `0087b270(004537b0(refr))`, else the virtual at `+0x1fc` with 0.
fn update_in_place(e: &mut Engine, me: u32) {
    if e.call(0x008c_7aa0, &args![]).bool() {
        // 005739dc / 00573bf4: as above, the reference is the word of `0087b270`.
        let handle = e.call(0x0045_37b0, &args![]).u32();
        e.call(0x0087_b270, &args![handle, me]);
    } else {
        e.vcall(me, 0x1fc, &args![0u32]);
    }
}

// Translated from 00573eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the bit 4 of the word at `+0xc` is set (`0047b4d0(this, 4)`).
pub fn fn_00573eb0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0047_b4d0, &args![this, 4u32]).bool()
}

// Translated from 00573ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` not 0) or clears the bit 4 of the same word
/// (`004d9fc0(this, flag, 4)`), and when it was set also `00573f00(this,
/// 1)`.
pub fn fn_00573ed0(e: &mut Engine, this: Ptr, flag: u8) {
    e.call(0x004d_9fc0, &args![this, u32::from(flag), 4u32]);
    if flag != 0 {
        e.call(0x0057_3f00, &args![this, 1u32]);
    }
}

/// Base object (`007af430`) compared by [`is_skipped_base`]: the form the
/// exe keeps in the global at `011ca27c` (the exe does not name it).
const SKIPPED_BASE_FORM_A: u32 = 0x011c_a27c;
/// The second base object compared by [`is_skipped_base`] (`011ca280`).
const SKIPPED_BASE_FORM_B: u32 = 0x011c_a280;

// Translated from 00573f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` not 0) or clears the bit 8 of the same flag word that
/// `00573ed0` handles with bit 4 (`004d9fc0(this, flag, 8)`).
pub fn fn_00573f00(e: &mut Engine, this: Ptr, flag: u8) {
    e.call(0x004d_9fc0, &args![this, u32::from(flag), 8u32]);
}

// Translated from 00573f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a float at `+0x544` of `this` (the exe does not name the class
/// or the field).
pub fn fn_00573f20(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x544, x87_quiet(value));
}

// Translated from 00573f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Enables a disabled reference (the counterpart of
/// [`tes_object_refr_disable`]). Nothing happens unless the reference is
/// disabled (`00440da0`). Otherwise: the virtual at `+0xc8` with 1 and
/// `TESForm::SetDisabled` (`00484af0`) with 0; an actor in a cell is put
/// back in the cell's list (`00545590`); the ash pile reference of its extra
/// data (`ExtraDataList::GetAshPileRef`, `0041e310`) is enabled the same
/// way unless the base object is one of the two forms in
/// [`SKIPPED_BASE_FORM_A`] / [`SKIPPED_BASE_FORM_B`]; an actor is
/// reinitialised (virtual `+0x208` with 1); a mobile object without a
/// process gets a new one (`00906dc0` on a `0xb4`-byte block, stored with
/// `00407800`) and is added to the process lists (`0096d450` with level 3).
/// A reference whose parent cell is loaded (`TES::IsCellLoaded`, `004511e0`)
/// and which has no 3D and is not known to the model loader (`00445750`) is
/// queued for loading (`ModelLoader::QueueReference`, `00444850`, with the
/// cell priority of `00458be0`) unless `00444ed0` excepts it, a base form of
/// type `0x1e` or `0x15` always being queued. A mobile object that was not
/// queued runs the virtual `+0x260`; an actor with a process is told to
/// evaluate its package again. Finally the player refreshes its quest
/// targets (`00952c30`) and every child linked to the reference
/// (`0056ac90`) is enabled or disabled depending on `0056aa70`.
/// C++ exception unwinding is not translated.
pub fn fn_00573f40(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    if !e.call(FORM_IS_DISABLED, &args![me]).bool() {
        return;
    }
    e.vcall(me, 0xc8, &args![1u32]);
    e.call(0x0048_4af0, &args![me, 0u32]);
    let cell = parent_cell(e, me);
    if cell != 0 && is_actor(e, me) {
        let cell = parent_cell(e, me);
        e.call(0x0054_5590, &args![cell, me]);
    }
    let list = extra_list(e, me);
    let ash_pile = e.call(0x0041_e310, &args![list]).u32();
    if ash_pile != 0 && !is_skipped_base(e, me) {
        fn_00573f40(e, Ptr::new(ash_pile));
    }
    if is_actor(e, me) {
        e.vcall(me, 0x208, &args![1u32]);
    }
    let mut mobile = 0;
    if e.vcall(me, 0xfc, &args![]).bool() {
        mobile = me;
    }
    if mobile != 0 && e.call(GET_PROCESS, &args![mobile]).u32() == 0 {
        let memory = e.call(0x0040_1000, &args![0xb4u32]).u32();
        let process = if memory != 0 {
            e.call(0x0090_6dc0, &args![memory]).u32()
        } else {
            0
        };
        e.call(0x0040_7800, &args![mobile, process]);
        e.call(
            0x0096_d450,
            &args![OBJECT_PROCESS_LISTS, mobile, 3u32, 0u32, 0u32, 0u32],
        );
    }
    let mut queued = false;
    if parent_cell(e, me) != 0 {
        let tes = e.global::<u32>(GLOBAL_TES);
        let cell = parent_cell(e, me);
        if e.call(0x0045_11e0, &args![tes, cell, 0u32]).bool() && node_of(e, me) == 0 {
            let loader = model_loader(e);
            if !e.call(0x0044_5750, &args![loader, me]).bool() {
                queued = true;
                if mobile != 0 && is_actor(e, mobile) {
                    e.call(0x0096_e870, &args![OBJECT_PROCESS_LISTS, mobile]);
                }
                let cell = parent_cell(e, me);
                if e.call(0x0045_0ff0, &args![cell]).bool() {
                    e.call(0x0057_9ac0, &args![me, 1u32]);
                }
                if !e.call(FORM_IS_DELETED, &args![me]).bool()
                    && base_form(e, me) != 0
                    && (base_form_type(e, me) == 0x1e
                        || base_form_type(e, me) == 0x15
                        || !e.call(0x0044_4ed0, &args![me]).bool())
                {
                    let cell = parent_cell(e, me);
                    let priority = e.call(0x0045_8be0, &args![tes, cell, 0u32]).u32();
                    e.call(0x0044_4850, &args![loader, me, priority, 0u32]);
                }
                let cell = parent_cell(e, me);
                if e.call(0x0045_0ff0, &args![cell]).bool()
                    && node_of(e, me) != 0
                    && base_form(e, me) != 0
                    && base_form_type(e, me) == 0x1e
                {
                    let package = fn_005725f0(e, this);
                    if package != 0 && e.call(READ_FIRST_DWORD, &args![package]).u32() != 0 {
                        let first = e.call(READ_FIRST_DWORD, &args![package]).u32();
                        let table_entry = e.call(0x0045_0b80, &args![0u32]).u32();
                        e.call(0x00b5_f080, &args![table_entry, first]);
                    }
                }
            }
        }
    }
    if mobile != 0 {
        if !queued {
            e.vcall(mobile, 0x260, &args![]);
        }
        let mut actor = 0;
        if is_actor(e, mobile) {
            actor = mobile;
        }
        if actor != 0 && e.call(GET_PROCESS, &args![actor]).u32() != 0 {
            let process = e.call(GET_PROCESS, &args![actor]).u32();
            e.vcall(process, 0x14, &args![actor, 1u32]);
            e.call(0x008a_6ce0, &args![actor, 0u32, 1u32]);
            let process = e.call(GET_PROCESS, &args![actor]).u32();
            if e.call(0x0045_cd60, &args![process]).u32() != 0 {
                e.vcall(actor, 0x25c, &args![0.0f32]);
            }
        }
    }
    let current_player = player(e);
    e.call(0x0095_2c30, &args![current_player, me]);
    if e.call(0x0056_b190, &args![me]).f64() > e.global::<f64>(0x0101_2060) {
        fn_005743f0(e);
    }
    let mut node = e.call(0x0056_ac90, &args![me]).u32();
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let child = e.mem.u32(slot);
        if e.call(0x0056_aa70, &args![child]).bool() {
            let list = extra_list(e, child);
            e.call(0x0041_dc20, &args![list]);
            tes_object_refr_disable(e, Ptr::new(child));
        } else {
            let list = extra_list(e, child);
            if e.call(0x0041_dbd0, &args![list]).bool() {
                e.call(0x0056_c780, &args![child, 1u32]);
            }
            fn_00573f40(e, Ptr::new(child));
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
}

/// True when the base object of `refr` is one of the two forms in
/// [`SKIPPED_BASE_FORM_A`] and [`SKIPPED_BASE_FORM_B`] (the ash pile of such
/// a reference is left alone).
fn is_skipped_base(e: &mut Engine, refr: u32) -> bool {
    let first = base_form(e, refr);
    if first == e.global::<u32>(SKIPPED_BASE_FORM_A) {
        return true;
    }
    let second = base_form(e, refr);
    second == e.global::<u32>(SKIPPED_BASE_FORM_B)
}

/// The form type (`cFormType`) of the base object of `refr`.
fn base_form_type(e: &mut Engine, refr: u32) -> u32 {
    let base = base_form(e, refr);
    form_type(e, base)
}

// Translated from 005743f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the byte at `011dd435` (a flag the exe does not name);
/// [`fn_00573f40`] calls it when `0056b190` returns a positive value.
pub fn fn_005743f0(e: &mut Engine) {
    e.mem.set_u8(0x011d_d435, 0);
}

// Translated from 00574400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::Disable` (Xbox PDB): does nothing when the reference is
/// already disabled (`00440da0`) or is the player. Otherwise it runs the
/// virtual at `+0x1cc` with `(0, 0)`, disables the ash pile reference of its
/// extra data (`0041e310`, unless the base object is one of
/// [`SKIPPED_BASE_FORM_A`] / [`SKIPPED_BASE_FORM_B`]), cleans up its traps
/// ([`tes_object_refr_clean_up_traps`]), runs the virtual at `+0xc8` with 1
/// and `TESForm::SetDisabled` (`00484af0`) with 1. A mobile object with a
/// process leaves the player's lists, the process lists, the cell (an actor
/// also gets `DoDeathStuff` when its virtual `+0x2e8` says so) and the
/// package or furniture it was using; its process is then deleted (virtual 0
/// with 1) and cleared (`00407800`). The terrain tree of its base form is
/// hidden (`BGSTerrainManager::HideTree`), the children linked to it are
/// enabled or disabled depending on `0056aa70`, the dropped items of its
/// extra data are removed and marked deleted, and an actor's radio is
/// switched off (`00835980`).
/// C++ exception unwinding is not translated.
pub fn tes_object_refr_disable(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    if e.call(FORM_IS_DISABLED, &args![me]).bool() || me == player(e) {
        return;
    }
    e.vcall(me, 0x1cc, &args![0u32, 0u32]);
    let list = extra_list(e, me);
    let ash_pile = e.call(0x0041_e310, &args![list]).u32();
    if ash_pile != 0 && !is_skipped_base(e, me) {
        tes_object_refr_disable(e, Ptr::new(ash_pile));
    }
    tes_object_refr_clean_up_traps(e, this);
    e.vcall(me, 0xc8, &args![1u32]);
    e.call(0x0048_4af0, &args![me, 1u32]);
    let mut mobile = 0;
    if e.vcall(me, 0xfc, &args![]).bool() {
        mobile = me;
    }
    if mobile != 0 && e.call(GET_PROCESS, &args![mobile]).u32() != 0 {
        if is_actor(e, mobile) {
            let actor = mobile;
            e.call(0x008c_2b60, &args![actor]);
            e.call(0x0082_4970, &args![actor + 0x94]);
            if fn_00570f60(e) as i32 > 0 {
                e.call(0x008c_ddd0, &args![me, 1u32]);
            }
            let current_player = player(e);
            e.call(0x0093_a660, &args![current_player, actor]);
            e.call(0x0096_7350, &args![current_player, actor]);
            e.call(0x0096_e6f0, &args![OBJECT_PROCESS_LISTS, actor]);
            if parent_cell(e, me) != 0 && e.call(0x0056_56d0, &args![me]).bool() {
                let cell = parent_cell(e, me);
                e.call(0x0054_5560, &args![cell, me]);
            }
            if e.vcall(actor, 0x2e8, &args![]).bool() {
                e.call(0x008b_01c0, &args![actor]);
            }
        }
        e.vcall(mobile, 0x2fc, &args![]);
        let level = e.call(0x0093_1850, &args![mobile]).u32();
        e.call(0x0096_d470, &args![OBJECT_PROCESS_LISTS, mobile, level]);
        if is_actor(e, me) || e.call(0x0056_4d80, &args![me]).bool() {
            e.call(0x0096_f600, &args![OBJECT_PROCESS_LISTS, me, 0u32]);
        }
        if e.call(GET_PROCESS, &args![mobile]).u32() != 0 {
            let process = e.call(GET_PROCESS, &args![mobile]).u32();
            if e.vcall(process, 0x22c, &args![]).u32() != 0 {
                let mut actor = 0;
                if mobile != 0 && is_actor(e, mobile) {
                    actor = mobile;
                }
                if actor != 0 {
                    e.vcall(actor, 0x288, &args![]);
                    e.call(0x0088_1680, &args![actor, 0u32]);
                } else if e.call(0x0057_4900, &args![mobile]).bool() {
                    e.vcall(mobile, 0x288, &args![]);
                }
            }
        }
        if is_actor(e, me) {
            e.vcall(me, 0x4c, &args![0x0010_0000u32]);
            e.vcall(me, 0x4c, &args![0x0020_0000u32]);
        }
        if e.call(GET_PROCESS, &args![mobile]).u32() != 0 {
            let process = e.call(GET_PROCESS, &args![mobile]).u32();
            if e.vcall(process, 0x4c8, &args![]).u32() != 0 {
                if is_actor(e, mobile) {
                    e.call(0x0088_d640, &args![mobile]);
                } else {
                    let first = e.call(GET_PROCESS, &args![mobile]).u32();
                    let second = e.call(GET_PROCESS, &args![mobile]).u32();
                    let marker = e.vcall(first, 0x4d0, &args![]).u32();
                    let owner = e.vcall(second, 0x4c8, &args![]).u32();
                    e.call(0x0056_8020, &args![owner, marker, 0u32]);
                }
            }
            let process = e.call(GET_PROCESS, &args![mobile]).u32();
            if process != 0 {
                e.vcall(process, 0, &args![1u32]);
            }
            e.call(0x0040_7800, &args![mobile, 0u32]);
        }
    }
    let base = base_form(e, me);
    if e.call(0x0054_9580, &args![base]).bool() {
        let tes = e.global::<u32>(GLOBAL_TES);
        let world = e.call(0x004f_d3e0, &args![tes]).u32();
        let manager = e.call(0x0058_6170, &args![world]).u32();
        e.call(0x006f_cfa0, &args![manager, me, 1u32]);
    }
    e.call(0x0056_c880, &args![me, 1u32, 0u32]);
    let current_player = player(e);
    e.call(0x0095_2c30, &args![current_player, me]);
    let mut node = e.call(0x0056_ac90, &args![me]).u32();
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
        let child = e.mem.u32(slot);
        if e.call(0x0056_aa70, &args![child]).bool() {
            let list = extra_list(e, child);
            if e.call(0x0041_dbd0, &args![list]).bool() {
                e.call(0x0056_c780, &args![child, 1u32]);
            }
            fn_00573f40(e, Ptr::new(child));
        } else {
            tes_object_refr_disable(e, Ptr::new(child));
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    let list = extra_list(e, me);
    let dropped = e.call(0x0041_df90, &args![list]).u32();
    while dropped != 0 && !e.call(0x0082_56d0, &args![dropped]).bool() {
        let slot = e.call(LIST_ITEM_SLOT, &args![dropped]).u32();
        let item = e.mem.u32(slot);
        let list = extra_list(e, item);
        e.call(0x0041_de40, &args![list, 0u32]);
        tes_object_refr_mark_as_deleted(e, Ptr::new(item));
        e.call(0x0063_f7b0, &args![dropped]);
    }
    e.call(0x0057_9ac0, &args![me, 0u32]);
    if is_actor(e, me) {
        e.call(0x0083_5980, &args![me]);
    }
}
// Translated from 00575650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetWorldLocation` (Xbox PDB): builds in `out` the world
/// location of the reference. The location object is copied
/// (`BGSWorldLocation::BGSWorldLocation`, `0043a3c0`, on `out`) from what the
/// virtual at `+0x1f4` returns, together with the space of
/// [`tes_object_refr_get_space`]. Returns `out`.
pub fn tes_object_refr_get_world_location(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    out: Ptr,
) -> Ptr {
    let me = this.addr();
    let space = tes_object_refr_get_space(e, this);
    let location = e.vcall(me, 0x1f4, &args![]).u32();
    e.call(0x0043_a3c0, &args![out, location, space]);
    out
}

// Translated from 00575690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetObjectReference` (Xbox PDB): stores the base object
/// (`OBJ_REFR::pObjectReference`, `+0x20`), tells the form whether the
/// object is destructible (`BGSDestructibleObjectForm::IsDestructible`,
/// `004753d0`, then `TESForm::SetDestructible`, `004846a0`) and, for a mobile
/// object (virtual `+0xfc`), passes the result of [`fn_0055e200`] on the
/// object's `+0x30` to the virtual at `+0x1f8`.
pub fn tes_object_refr_set_object_reference(e: &mut Engine, this: Ptr<TESObjectREFR>, object: u32) {
    let me = this.addr();
    // OBJ_REFR::pObjectReference (Xbox PDB) +0x20
    e.mem.set_u32(me + 0x20, object);
    let destructible = e.call(0x0047_53d0, &args![object]).bool();
    e.call(0x0048_46a0, &args![me, u32::from(destructible)]);
    if e.vcall(me, 0xfc, &args![]).bool() {
        let flag = fn_0055e200(e, Ptr::new(object + 0x30));
        e.vcall(me, 0x1f8, &args![u32::from(flag)]);
    }
}

// Translated from 00575700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the three words of a rotation at `+0x24`, `+0x28` and `+0x2c`
/// (`OBJ_REFR::Angle`); an actor's `Z` is clamped with `ClampAngle`
/// (`004b1480`). Then the virtual at `+0x48` with 2.
pub fn fn_00575700(e: &mut Engine, this: Ptr<TESObjectREFR>, x: u32, y: u32, z: u32) {
    let me = this.addr();
    // OBJ_REFR::Angle (Xbox PDB) +0x24 / +0x28 / +0x2c
    e.mem.set_u32(me + 0x24, x);
    e.mem.set_u32(me + 0x28, y);
    e.mem.set_u32(me + 0x2c, z);
    if is_actor(e, me) {
        let angle = e.mem.f32(me + 0x2c);
        let clamped = e.call(0x004b_1480, &args![angle]).f32();
        e.mem.set_f32(me + 0x2c, clamped);
    }
    e.vcall(me, 0x48, &args![2u32]);
}

// Translated from 00575770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the `X` angle (`OBJ_REFR::Angle`, `+0x24`), then the virtual at
/// `+0x48` with 2.
pub fn fn_00575770(e: &mut Engine, this: Ptr<TESObjectREFR>, angle: f32) {
    let me = this.addr();
    e.mem.set_f32(me + 0x24, angle);
    e.vcall(me, 0x48, &args![2u32]);
}

// Translated from 005757a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetAngleOnReferenceY` (Xbox PDB): stores the `Y` angle
/// (`OBJ_REFR::Angle`, `+0x28`), then the virtual at `+0x48` with 2.
pub fn tes_object_refr_set_angle_on_reference_y(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    angle: f32,
) {
    let me = this.addr();
    e.mem.set_f32(me + 0x28, angle);
    e.vcall(me, 0x48, &args![2u32]);
}

// Translated from 005757d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the `Z` angle (`OBJ_REFR::Angle`, `+0x2c`), clamped with
/// `ClampAngle` (`004b1480`) for an actor, then the virtual at `+0x48`
/// with 2.
pub fn fn_005757d0(e: &mut Engine, this: Ptr<TESObjectREFR>, angle: f32) {
    let me = this.addr();
    e.mem.set_f32(me + 0x2c, angle);
    if is_actor(e, me) {
        let stored = e.mem.f32(me + 0x2c);
        let clamped = e.call(0x004b_1480, &args![stored]).f32();
        e.mem.set_f32(me + 0x2c, clamped);
    }
    e.vcall(me, 0x48, &args![2u32]);
}

/// Bytes of the locals of [`tes_object_refr_set_location_on_reference`] and
/// the offset of its `ebp` in them.
const LOCATION_FRAME: u32 = 0x50;
const LOCATION_EBP: i32 = 0x44;

// Translated from 00575830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetLocationOnReference` (Xbox PDB): moves the reference
/// to the `NiPoint3` at `location`. A position outside the range the exe
/// accepts is replaced: while the object `011ddf38` is loading
/// (`0042ce10`) a component equal to `+-FLT_MAX` or outside
/// `+-4096000` (limits at `01030ed0` / `01030ec8`), otherwise `X` or `Y`
/// outside `+-4096000`, makes the virtual at `+0x138` write a valid position
/// to a local (starting from the zero vector `011f426c`) and `Z` is raised
/// by 10.0 (`01020758`). A persistent reference of a fixed-reference world
/// space is removed from that space's persistent data during the move
/// (`00587e40`) and added again afterwards (`00587d10`). The position is
/// stored at `+0x30` (`OBJ_REFR::Location`); when it was replaced and the
/// reference has a 3D, the node's translation is set (`00440460`), its
/// collision simulation reset (`00c6bd00`), its properties updated
/// (`00a5a040`) and the node updated with a fresh `NiUpdateData`
/// (`0043d410`, `00a59c60`). Ends with the virtual at `+0x48` with 2.
pub fn tes_object_refr_set_location_on_reference(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    location: Ptr,
) {
    let me = this.addr();
    let loc = location.addr();
    e.with_stack(LOCATION_FRAME, |e, block| {
        let at = |offset: i32| (block.addr() as i32 + LOCATION_EBP + offset) as u32;
        let corrected = at(-0x10);
        let mut replaced = false;
        copy_words(e, corrected, ZERO_VECTOR, 3);
        copy_words(e, at(-0x1c), ZERO_VECTOR, 3);
        let x = f64::from(e.mem.f32(loc));
        let y = f64::from(e.mem.f32(loc + 4));
        let z = f64::from(e.mem.f32(loc + 8));
        let limit = e.global::<f64>(0x0103_0ed0);
        let negative_limit = e.global::<f64>(0x0103_0ec8);
        let loading_object = e.global::<u32>(GLOBAL_LOADING_OBJECT);
        if loading_object != 0 && e.call(0x0042_ce10, &args![loading_object]).bool() {
            let largest = e.global::<f64>(0x0102_31b0);
            let smallest = e.global::<f64>(0x0102_41b0);
            if x == largest
                || x == smallest
                || y == largest
                || y == smallest
                || z == largest
                || z == smallest
                || x.is_nan()
                || y.is_nan()
                || x >= limit
                || x <= negative_limit
                || y >= limit
                || y <= negative_limit
            {
                e.mem.set_u32(at(-0x24), 0);
                e.vcall(me, 0x138, &args![corrected, at(-0x1c), at(-0x24), 0u32]);
                raise_height(e, corrected);
                replaced = true;
            }
        } else if x > limit || x < negative_limit || y > limit || y < negative_limit {
            copy_words(e, at(-0x30), ZERO_VECTOR, 3);
            e.mem.set_u32(at(-0x34), 0);
            e.vcall(me, 0x138, &args![corrected, at(-0x30), at(-0x34), 0u32]);
            raise_height(e, corrected);
            replaced = true;
        }
        let mut world = 0;
        if e.call(GET_REF_PERSISTS, &args![me]).bool() && e.call(0x0058_7c80, &args![me]).bool() {
            world = tes_object_refr_get_world_space(e, this);
            if world != 0 {
                e.call(0x0058_7e40, &args![world, me]);
            }
        }
        // OBJ_REFR::Location (Xbox PDB) +0x30
        if !replaced {
            copy_words(e, me + 0x30, loc, 3);
        } else {
            copy_words(e, me + 0x30, corrected, 3);
            let node = node_of(e, me);
            if node != 0 {
                let node = node_of(e, me);
                e.call(0x0044_0460, &args![node, corrected]);
                // TESObjectREFR::pLoadedData (Xbox PDB) +0x64, 3D slot at +0x14
                let slot = e.mem.u32(me + 0x64) + 0x14;
                let collision = e.call(READ_FIRST_DWORD, &args![slot]).u32();
                e.call(0x00c6_bd00, &args![collision, 1u32]);
                let slot = e.mem.u32(me + 0x64) + 0x14;
                let object = e.call(READ_FIRST_DWORD, &args![slot]).u32();
                e.call(0x00a5_a040, &args![object]);
                let update = at(-0x40);
                e.call(UPDATE_DATA_CONSTRUCTOR, &args![update, 0.0f32, 0u32, 0u32]);
                let slot = e.mem.u32(me + 0x64) + 0x14;
                let object = e.call(READ_FIRST_DWORD, &args![slot]).u32();
                e.call(NODE_UPDATE, &args![object, update]);
            }
        }
        if world != 0 {
            e.call(0x0058_7d10, &args![world, me]);
        }
        e.vcall(me, 0x48, &args![2u32]);
    });
}

/// Raises the `Z` of the `NiPoint3` at `vector` by 10.0 (`01020758`),
/// rounding to `float`.
fn raise_height(e: &mut Engine, vector: u32) {
    let height = f64::from(e.mem.f32(vector + 8)) + e.global::<f64>(0x0102_0758);
    e.mem.set_f32(vector + 8, height as f32);
}

// Translated from 00575b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetLocationOnReferenceZ` (Xbox PDB): calls
/// [`tes_object_refr_set_location_on_reference`] with the current position
/// whose `Z` is replaced by `z`.
pub fn tes_object_refr_set_location_on_reference_z(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    z: f32,
) {
    let me = this.addr();
    e.with_stack(0xc, |e, local| {
        // OBJ_REFR::Location (Xbox PDB) +0x30
        copy_words(e, local.addr(), me + 0x30, 3);
        e.mem.set_f32(local.addr() + 8, z);
        tes_object_refr_set_location_on_reference(e, this, local);
    });
}

// Translated from 00575bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::SetParentCell` (Xbox PDB): stores the cell
/// (`pParentCell`, `+0x40`). Unless the extra data has a water zone map
/// (`ExtraDataList::QWaterZoneMap`, `0042f1d0`), the loaded data's relevant
/// water height (`+0x8` of `pLoadedData`) becomes the cell's water height
/// (`005471e0`) when the cell has one (`004518e0`) and the default
/// (`01015f5c`) otherwise, and the player resets the byte at `011c7a58`
/// ([`fn_00575c90`]). A mobile actor with a process is told about the
/// reference (process virtual `+0x6ac`) and, when
/// `MobileObject::GetCurrentProcessType` (`00931850`) is 0, runs the virtual
/// at `+0x240`.
pub fn tes_object_refr_set_parent_cell(e: &mut Engine, this: Ptr<TESObjectREFR>, cell: u32) {
    let me = this.addr();
    // TESObjectREFR::pParentCell (Xbox PDB) +0x40
    e.mem.set_u32(me + 0x40, cell);
    let list = extra_list(e, me);
    if e.call(0x0042_f1d0, &args![list]).u32() == 0 {
        // TESObjectREFR::pLoadedData (Xbox PDB) +0x64
        let loaded = e.mem.u32(me + 0x64);
        if loaded != 0 {
            let height = if cell != 0 && e.call(0x0045_18e0, &args![cell]).bool() {
                e.call(0x0054_71e0, &args![cell]).f32()
            } else {
                e.global::<f32>(LOADED_DATA_DEFAULT_HEIGHT)
            };
            // LOADED_REF_DATA::fRelevantWaterHeight (Xbox PDB) +0x08
            let loaded = e.mem.u32(me + 0x64);
            e.mem.set_f32(loaded + 8, height);
        }
        if me == player(e) {
            fn_00575c90(e, 0);
        }
    }
    if is_actor(e, me) && e.call(GET_PROCESS, &args![me]).u32() != 0 {
        let process = e.call(GET_PROCESS, &args![me]).u32();
        e.vcall(process, 0x6ac, &args![me]);
        if e.call(0x0093_1850, &args![me]).u32() == 0 {
            e.vcall(me, 0x240, &args![]);
        }
    }
}

// Translated from 00575c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at `011c7a58` (a global flag the exe does not
/// name).
pub fn fn_00575c90(e: &mut Engine, value: u8) {
    e.mem.set_u8(0x011c_7a58, value);
}

// Translated from 00575ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetSpace` (Xbox PDB): the world space of the parent cell
/// (`TESObjectCELL::GetWorldSpace`, `0054ddd0`), or the cell itself when it
/// has none (an interior). Without a parent cell it is the world space of
/// the persistent cell in the extra data (`0041d460`), 0 when there is
/// none.
pub fn tes_object_refr_get_space(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let me = this.addr();
    // TESObjectREFR::pParentCell (Xbox PDB) +0x40
    let cell = e.mem.u32(me + 0x40);
    if cell != 0 {
        let world = e.call(0x0054_ddd0, &args![cell]).u32();
        if world != 0 {
            world
        } else {
            e.mem.u32(me + 0x40)
        }
    } else {
        let list = extra_list(e, me);
        let persistent = e.call(0x0041_d460, &args![list]).u32();
        if persistent != 0 {
            e.call(0x0054_ddd0, &args![persistent]).u32()
        } else {
            0
        }
    }
}

// Translated from 00575d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetInterior` (Xbox PDB): with a parent cell
/// (`008d6f30`), what `00425fd0` says of it; without one, the cell the
/// `TESChildCell` base reports (its virtual at slot 0, on `this + 0x18`):
/// false when that cell has a world space (`0054ddd0`), true otherwise.
pub fn tes_object_refr_get_interior(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u8 {
    let me = this.addr();
    let mut interior = 1u8;
    let cell = parent_cell(e, me);
    if cell != 0 {
        interior = e.call(0x0042_5fd0, &args![cell]).u32() as u8;
    } else {
        let child_cell = e.vcall(me + CHILD_CELL_OFFSET, 0, &args![]).u32();
        if child_cell != 0 && e.call(0x0054_ddd0, &args![child_cell]).u32() != 0 {
            interior = 0;
        }
    }
    interior
}

// Translated from 00575d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetWorldSpace` (Xbox PDB): the world space
/// (`0054ddd0`) of the parent cell (`008d6f30`), or of the cell the
/// `TESChildCell` base reports (virtual slot 0 on `this + 0x18`) when there
/// is none; 0 without a cell.
pub fn tes_object_refr_get_world_space(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let me = this.addr();
    let mut cell = parent_cell(e, me);
    if cell == 0 {
        cell = e.vcall(me + CHILD_CELL_OFFSET, 0, &args![]).u32();
    }
    if cell != 0 {
        e.call(0x0054_ddd0, &args![cell]).u32()
    } else {
        0
    }
}

// Translated from 00575dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Method of the `TESChildCell` base (`this` is the reference `+ 0x18`): the
/// parent cell of the reference (`008d6f30`), except that a persistent
/// reference (`GetRefPersists`, `005653d0`, or `004077c0`) whose cell is
/// missing or fails `00425fd0` gets the persistent cell of its extra data
/// (`0041d460`) instead.
pub fn fn_00575dc0(e: &mut Engine, child_cell: Ptr) -> u32 {
    let refr = child_cell.addr() - CHILD_CELL_OFFSET;
    let cell = parent_cell(e, refr);
    if !e.call(GET_REF_PERSISTS, &args![refr]).bool() && !e.call(0x0040_77c0, &args![refr]).bool() {
        return cell;
    }
    if cell != 0 && e.call(0x0042_5fd0, &args![cell]).bool() {
        return cell;
    }
    let list = extra_list(e, refr);
    e.call(0x0041_d460, &args![list]).u32()
}

/// `__RTDynamicCast(object, 0, from, to, 0)` of an extra data object
/// (`from` is the type descriptor of `BSExtraData`).
fn cast_extra(e: &mut Engine, object: u32, to: u32) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![object, 0u32, TYPE_BS_EXTRA_DATA, to, 0u32],
    )
    .u32()
}

/// `NonActorMagicCaster`.
const TYPE_NON_ACTOR_MAGIC_CASTER: u32 = 0x0118_4638;
/// `NonActorMagicTarget`.
const TYPE_NON_ACTOR_MAGIC_TARGET: u32 = 0x0118_444c;
/// `TESMagicTargetForm`.
const TYPE_TES_MAGIC_TARGET_FORM: u32 = 0x0118_9e4c;

// Translated from 00575e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NonActorMagicCaster` of the reference, created on demand: looks up
/// the extra data of type `0x32` in the reference's extra data list
/// (`00410220`) and casts it to `NonActorMagicCaster`. When there is none a
/// `0x24`-byte one is built for the reference (`00825ad0`), added to the list
/// (`0040ff60`) and the virtual at `+0x48` is run with `0x80000000`. Returns
/// the caster's `MagicCaster` part (its address `+ 0xc`), 0 without one.
/// C++ exception unwinding is not translated.
pub fn fn_00575e30(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let me = this.addr();
    let list = extra_list(e, me);
    let extra = e.call(0x0041_0220, &args![list, 0x32u32]).u32();
    let mut caster = cast_extra(e, extra, TYPE_NON_ACTOR_MAGIC_CASTER);
    if caster == 0 {
        let memory = e.call(0x0040_1000, &args![0x24u32]).u32();
        caster = if memory != 0 {
            e.call(0x0082_5ad0, &args![memory, me]).u32()
        } else {
            0
        };
        let list = extra_list(e, me);
        e.call(0x0040_ff60, &args![list, caster]);
        e.vcall(me, 0x48, &args![0x8000_0000u32]);
    }
    if caster != 0 {
        caster + 0xc
    } else {
        0
    }
}

// Translated from 00575f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NonActorMagicTarget` of the reference, created on demand, only for
/// a base object that is a `TESMagicTargetForm` ([`fn_00576040`]): looks up
/// the extra data of type `0x33` (`00410220`) and casts it to
/// `NonActorMagicTarget`; when there is none a `0x28`-byte one is built for
/// the reference (`00825fd0`), added to the list (`0040ff60`) and the
/// virtual at `+0x48` is run with `0x80000000`. Returns the target's
/// `MagicTarget` part (its address `+ 0xc`), 0 without one.
/// C++ exception unwinding is not translated.
pub fn fn_00575f30(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u32 {
    let me = this.addr();
    let mut target = 0;
    let base = base_form(e, me);
    if fn_00576040(e, base) {
        let list = extra_list(e, me);
        let extra = e.call(0x0041_0220, &args![list, 0x33u32]).u32();
        target = cast_extra(e, extra, TYPE_NON_ACTOR_MAGIC_TARGET);
        if target == 0 {
            let memory = e.call(0x0040_1000, &args![0x28u32]).u32();
            target = if memory != 0 {
                e.call(0x0082_5fd0, &args![memory, me]).u32()
            } else {
                0
            };
            let list = extra_list(e, me);
            e.call(0x0040_ff60, &args![list, target]);
            e.vcall(me, 0x48, &args![0x8000_0000u32]);
        }
    }
    if target != 0 {
        target + 0xc
    } else {
        0
    }
}

// Translated from 00576040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `form` can be cast to `TESMagicTargetForm`
/// (`__RTDynamicCast(form, 0, TESForm, TESMagicTargetForm, 0)`, a cdecl
/// function).
pub fn fn_00576040(e: &mut Engine, form: u32) -> bool {
    e.call(
        DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_MAGIC_TARGET_FORM, 0u32],
    )
    .u32()
        != 0
}

// Translated from 00576070 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a reference passing `00452370`: the answer of [`fn_00576100`] on the
/// destruction form of its base object (`BGSDestructibleObjectForm::
/// GetDestructionForm`, `00475400`), 0 without one; inverted when
/// [`fn_005760e0`] says the reference has flag `0x4000000`. 0 for a
/// reference `00452370` rejects.
pub fn fn_00576070(e: &mut Engine, this: Ptr<TESObjectREFR>) -> u8 {
    let me = this.addr();
    let mut result = 0u8;
    if e.call(0x0045_2370, &args![me]).bool() {
        let base = base_form(e, me);
        let destruction = e.call(0x0047_5400, &args![base]).u32();
        if destruction != 0 {
            result = u8::from(fn_00576100(e, Ptr::new(destruction)));
        }
        if fn_005760e0(e, this) {
            result = u8::from(result == 0);
        }
    }
    result
}

// Translated from 005760e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `0x4000000` is set in the form flags (the word at `+8`).
pub fn fn_005760e0(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    e.mem.u32(this.addr() + 8) & 0x0400_0000 != 0
}

// Translated from 00576100 (decompiled, FalloutNV.exe 1.4.0.525)
/// False when the word at `+4` is null, otherwise whether bit 0 of the byte
/// at offset 5 of what it points to is set.
pub fn fn_00576100(e: &mut Engine, this: Ptr) -> bool {
    let data = e.mem.u32(this.addr() + 4);
    data != 0 && e.mem.u8(data + 5) & 1 != 0
}

// Translated from 00576130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::AddObjecttoContainer` (Xbox PDB). With an extra data list
/// (`extra_data`): an ownership that is the reference's own owner
/// (`ExtraDataList::GetOwner` `00418660` against `TESObjectREFR::GetOwner`
/// `00567790`) is removed (`0041aed0`); for the reference the list points to
/// (`GetReferencePointer`, `0041c8d0`) the process lists give the actors
/// that have it as target (`0096f450` on its form id and this reference):
/// each one is given a new current target (`Actor::SetCurrentTarget`,
/// `00881620`) of this reference when `Actor::GetCurrentPackageTarget`
/// (`00881650`) says it has one, 0 otherwise, and the list is released
/// (`00470470`, `004702f0` with 1). In every case it ends with
/// `00574fa0(first, extra_data, third)` on this reference.
pub fn tes_object_refr_add_objectto_container(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    first: u32,
    extra_data: u32,
    third: u32,
) {
    let me = this.addr();
    if extra_data != 0 {
        if e.call(0x0041_8660, &args![extra_data]).u32() != 0 {
            let owner = e.call(0x0041_8660, &args![extra_data]).u32();
            let own = e.call(0x0056_7790, &args![me]).u32();
            if owner == own {
                e.call(0x0041_aed0, &args![extra_data]);
            }
        }
        let target = e.call(0x0041_c8d0, &args![extra_data]).u32();
        if target != 0 {
            let form_id = e.call(FORM_ID, &args![target]).u32();
            let head = e
                .call(0x0096_f450, &args![OBJECT_PROCESS_LISTS, form_id, me])
                .u32();
            let mut node = head;
            while node != 0 {
                let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
                if e.mem.u32(slot) == 0 {
                    break;
                }
                let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
                let actor = e.mem.u32(slot);
                if actor != 0 && e.call(0x0088_1650, &args![actor]).u32() != 0 {
                    e.call(0x0088_1620, &args![actor, me]);
                } else {
                    e.call(0x0088_1620, &args![actor, 0u32]);
                }
                node = e.call(LIST_NEXT, &args![node]).u32();
            }
            if head != 0 {
                e.call(0x0047_0470, &args![head]);
                e.call(0x0047_02f0, &args![head, 1u32]);
            }
        }
    }
    e.call(0x0057_4fa0, &args![me, first, extra_data, third]);
}

// Translated from 00576260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetInventoryItem` (Xbox PDB): asks the container changes
/// of the extra data (`ExtraDataList::GetContainerChanges`, `00418520`) for
/// the item (`InventoryChanges::GetInventoryItem`, `004d0650`, with the two
/// arguments); 0 without container changes.
pub fn tes_object_refr_get_inventory_item(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    first: u32,
    second: u32,
) -> u32 {
    let list = extra_list(e, this.addr());
    let changes = e.call(0x0041_8520, &args![list]).u32();
    if changes != 0 {
        e.call(0x004d_0650, &args![changes, first, second]).u32()
    } else {
        0
    }
}

// Translated from 005762b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes to `out` the world position of the center of the reference's
/// bound box: the maximum corner (virtual `+0x1dc`) and the minimum corner
/// (virtual `+0x1d8`) are added (`00439e90`), divided by the float at
/// `010162c0` (`0053d280`) and added to the reference's position (virtual
/// `+0x1f4`, `00439e90`). Returns `out`.
pub fn fn_005762b0(e: &mut Engine, this: Ptr<TESObjectREFR>, out: Ptr) -> Ptr {
    let me = this.addr();
    e.with_stack(0x30, |e, block| {
        let base = block.addr();
        let (quotient, sum, minimum, maximum) = (base, base + 0xc, base + 0x18, base + 0x24);
        let maximum = e.vcall(me, 0x1dc, &args![maximum]).u32();
        let minimum = e.vcall(me, 0x1d8, &args![minimum]).u32();
        let divisor = e.global::<f32>(0x0101_62c0);
        let total = e.call(0x0043_9e90, &args![minimum, sum, maximum]).u32();
        let half = e.call(0x0053_d280, &args![total, quotient, divisor]).u32();
        let position = e.vcall(me, 0x1f4, &args![]).u32();
        e.call(0x0043_9e90, &args![position, out, half]);
    });
    out
}

// Translated from 00576330 (decompiled, FalloutNV.exe 1.4.0.525)
/// For an actor (virtual `+0x100`) whose virtual at `+0x390` is not 0: the
/// object named `BSFaceGenNiNodeBiped` (string at `01020408`) found in the
/// 3D of the reference (`0043fcd0`) by `004aae30` (cdecl, node and name);
/// 0 otherwise. The word the caller pushes is not read.
pub fn fn_00576330(e: &mut Engine, this: Ptr<TESObjectREFR>, _unused_1: u32) -> u32 {
    face_gen_node(e, this.addr(), 0x0102_0408)
}

// Translated from 00576390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`fn_00576330`] for the object named `BSFaceGenNiNodeSkinned`
/// (string at `010203f0`).
pub fn fn_00576390(e: &mut Engine, this: Ptr<TESObjectREFR>, _unused_1: u32) -> u32 {
    face_gen_node(e, this.addr(), 0x0102_03f0)
}

/// What [`fn_00576330`] and [`fn_00576390`] share, for the name at `name`.
fn face_gen_node(e: &mut Engine, me: u32, name: u32) -> u32 {
    if !is_actor(e, me) {
        return 0;
    }
    if e.vcall(me, 0x390, &args![]).u32() == 0 {
        return 0;
    }
    let node = get_3d(e, me);
    e.call(0x004a_ae30, &args![node, name]).u32()
}

// Translated from 005763f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the virtual at `+0x1b0` with `argument` and returns its result.
pub fn fn_005763f0(e: &mut Engine, this: Ptr<TESObjectREFR>, argument: u32) -> u32 {
    e.vcall(this.addr(), 0x1b0, &args![argument]).u32()
}

// Translated from 00576420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::GetFaceAnimationData` (Xbox PDB): the result of the
/// virtual at `+0x100` of the object the virtual at `+0x1b4` returns for
/// `argument`; 0 when that is null.
pub fn tes_object_refr_get_face_animation_data(
    e: &mut Engine,
    this: Ptr<TESObjectREFR>,
    argument: u32,
) -> u32 {
    let face = e.vcall(this.addr(), 0x1b4, &args![argument]).u32();
    if face != 0 {
        e.vcall(face, 0x100, &args![]).u32()
    } else {
        0
    }
}

// Translated from 00576470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::ClampToGround` (Xbox PDB): with a parent cell, takes the
/// position (virtual `+0x1f4`), asks the cell for the land height at it
/// (`TESObjectCELL::GetLandHeight`, `005547c0`) and, when there is one,
/// moves the reference there with
/// [`tes_object_refr_set_location_on_reference`]. Returns whether it did.
pub fn tes_object_refr_clamp_to_ground(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    let me = this.addr();
    let mut clamped = false;
    let cell = parent_cell(e, me);
    if cell != 0 {
        let position = e.vcall(me, 0x1f4, &args![]).u32();
        e.with_stack(0x10, |e, block| {
            let (point, height) = (block.addr(), block.addr() + 0xc);
            copy_words(e, point, position, 3);
            if e.call(0x0055_47c0, &args![cell, point, height]).bool() {
                let ground = e.mem.f32(height);
                e.mem.set_f32(point + 8, ground);
                tes_object_refr_set_location_on_reference(e, this, Ptr::new(point));
                clamped = true;
            }
        });
    }
    clamped
}

// Translated from 005764f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl function of two pointers, `first` and `second` (the byte at `second + 4`
/// is cleared). When the setting object `011ca394` has its byte set
/// the 3D owner of `first` (`0044ddc0`) gets `004902f0(0)`,
/// [`fn_00576640`] with 0 and [`fn_00576660`] with 1, and `second[4] = 0`.
/// Otherwise, with the setting `011ca3ec` set and a `first`, the reference
/// found for the owner's 3D (`0056f930`) decides: the owner is changed that
/// way only when the base object is of a type `TESContainer::
/// ContainerCanHoldType` (`00481f30`) accepts, not a form of type `0x1e`
/// that `0046f070` rejects; in every other case `00c66ff0(first, second)`
/// runs.
pub fn fn_005764f0(e: &mut Engine, first: u32, second: u32) {
    let setting = e.call(0x0040_8d60, &args![0x011c_a394u32]).u32();
    if e.mem.u8(setting) != 0 {
        let owner = e.call(0x0044_ddc0, &args![first]).u32();
        change_owner(e, owner, second);
        return;
    }
    let mut fallback = true;
    let setting = e.call(0x0040_8d60, &args![0x011c_a3ecu32]).u32();
    if e.mem.u8(setting) != 0 && first != 0 {
        let owner = e.call(0x0044_ddc0, &args![first]).u32();
        let found = e.call(0x0056_f930, &args![owner]).u32();
        if found != 0 {
            if base_form(e, found) != 0 {
                let kind = base_form_type(e, found);
                if e.call(0x0048_1f30, &args![kind]).bool() {
                    fallback = false;
                }
            }
            if !fallback && base_form_type(e, found) == 0x1e {
                let base = base_form(e, found);
                if base != 0 && !e.call(0x0046_f070, &args![base]).bool() {
                    fallback = true;
                }
            }
            if !fallback {
                change_owner(e, owner, second);
            }
        }
    }
    if fallback {
        e.call(0x00c6_6ff0, &args![first, second]);
    }
}

/// The body [`fn_005764f0`] runs in two places: `004902f0(owner, 0)`,
/// [`fn_00576640`] and [`fn_00576660`] on it, and `second[4] = 0`.
fn change_owner(e: &mut Engine, owner: u32, second: u32) {
    e.call(0x0049_02f0, &args![owner, 0u32]);
    fn_00576640(e, Ptr::new(owner), 0);
    fn_00576660(e, Ptr::new(owner), 1);
    e.mem.set_u8(second + 4, 0);
}

// Translated from 00576640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0043b370(this, flag, 4)`: sets or clears the flag word's bit 4.
pub fn fn_00576640(e: &mut Engine, this: Ptr, flag: u8) {
    e.call(0x0043_b370, &args![this, u32::from(flag), 4u32]);
}

// Translated from 00576660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0043b370(this, flag, 0x10)`: sets or clears the flag word's bit 0x10.
pub fn fn_00576660(e: &mut Engine, this: Ptr, flag: u8) {
    e.call(0x0043_b370, &args![this, u32::from(flag), 0x10u32]);
}

// Translated from 00576680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the node of the reference from the physics world, with the object
/// `011dea10` told 1 (`00453860`) and 0 at the end: if the
/// reference has a node (virtual `+0x1d0`) it cleans up its traps
/// ([`tes_object_refr_clean_up_traps`]) and removes the node's objects from
/// the world (`bhkWorld::RemoveObjects`, `00c69ee0`, with `(node, 1, 0)`);
/// the phantom of its loaded data (`+0x18`) is released (`0066b0d0`
/// with 0); a base form of type `0xe` with a parent cell is passed to
/// `0061f890` (through `004543c0` and `0045cd60` of the cell); finally
/// `0056c880(this, 1, 0)`. Returns whether there was a node.
pub fn fn_00576680(e: &mut Engine, this: Ptr<TESObjectREFR>) -> bool {
    let me = this.addr();
    let mut removed = false;
    let manager = e.global::<u32>(GLOBAL_TES);
    e.call(0x0045_3860, &args![manager, 1u32]);
    let node = node_of(e, me);
    if node != 0 {
        tes_object_refr_clean_up_traps(e, this);
        e.call(0x00c6_9ee0, &args![node, 1u32, 0u32]);
        removed = true;
    }
    // TESObjectREFR::pLoadedData (Xbox PDB) +0x64, spPhantom +0x18
    let loaded = e.mem.u32(me + 0x64);
    if loaded != 0 {
        e.call(0x0066_b0d0, &args![loaded + 0x18, 0u32]);
    }
    if base_form(e, me) != 0 && base_form_type(e, me) == 0xe && parent_cell(e, me) != 0 {
        let cell = parent_cell(e, me);
        let first = e.call(0x0045_43c0, &args![cell]).u32();
        let second = e.call(0x0045_cd60, &args![first]).u32();
        e.call(0x0061_f890, &args![second, me, 1u32]);
    }
    e.call(0x0056_c880, &args![me, 1u32, 0u32]);
    e.call(0x0045_3860, &args![manager, 0u32]);
    removed
}

// Translated from 00576760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR::CleanUpTraps` (Xbox PDB): for the object `004543c0`
/// gives for the parent cell, its base-form-like part (`007af430`, minus 8:
/// the trap listener) runs [`fn_00576800`], `BGSZoneTargetListener::
/// RemoveTarget` (`00620130`) and `TESTrapListener::ClearCurrentRefs`
/// (`0062de90`) on this reference. Nothing without a parent cell or such an
/// object.
pub fn tes_object_refr_clean_up_traps(e: &mut Engine, this: Ptr<TESObjectREFR>) {
    let me = this.addr();
    let cell = parent_cell(e, me);
    let trap_source = if cell != 0 {
        e.call(0x0045_43c0, &args![cell]).u32()
    } else {
        0
    };
    if trap_source != 0 {
        let part = base_form(e, trap_source);
        let listener = if part != 0 { part.wrapping_sub(8) } else { 0 };
        if listener != 0 {
            fn_00576800(e, Ptr::new(listener), this.addr());
            e.call(0x0062_0130, &args![listener, me]);
            e.call(0x0062_de90, &args![listener, me]);
        }
    }
}

// Translated from 00576800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `006205e0(this, fn_00576830(this, argument))`.
pub fn fn_00576800(e: &mut Engine, this: Ptr, argument: u32) {
    let found = fn_00576830(e, this, argument);
    e.call(0x0062_05e0, &args![this, found]);
}

// Translated from 00576830 (decompiled, FalloutNV.exe 1.4.0.525)
/// 0 when the word at `+4` is null, otherwise `00576870(that, argument)`.
pub fn fn_00576830(e: &mut Engine, this: Ptr, argument: u32) -> u32 {
    let inner = e.mem.u32(this.addr() + 4);
    if inner != 0 {
        e.call(0x0057_6870, &args![inner, argument]).u32()
    } else {
        0
    }
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0056ea40, fn_0056ea40(Ptr) -> Ptr),
        entry!(
            0x0056ea60,
            bhk_sphere_shape_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0056ea90, fn_0056ea90(Ptr)),
        entry!(0x0056eab0, fn_0056eab0(Ptr<TESObjectREFR>)),
        entry!(0x0056f0f0, fn_0056f0f0(Ptr, u32)),
        entry!(0x0056f110, fn_0056f110(Ptr, u32)),
        entry!(0x0056f140, fn_0056f140(Ptr<TESObjectREFR>)),
        entry!(0x0056f690, fn_0056f690(Ptr) -> Ptr),
        entry!(0x0056f6b0, fn_0056f6b0(Ptr<TESObjectREFR>) -> bool),
        entry!(0x0056f700, fn_0056f700(Ptr<TESObjectREFR>)),
        entry!(
            0x0056f930,
            tes_object_refr_find_reference_for_3d(u32) -> u32
        ),
        entry!(
            0x0056fa00,
            tes_object_refr_get_orientation(Ptr<TESObjectREFR>, Ptr) -> Ptr
        ),
        entry!(
            0x0056fa90,
            tes_object_refr_set_orientation(
                Ptr<TESObjectREFR>,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            )
        ),
        entry!(
            0x0056fac0,
            tes_object_refr_multiple_matrix_by_race(Ptr<TESObjectREFR>, Ptr, Ptr) -> Ptr
        ),
        entry!(0x0056fb10, fn_0056fb10(Ptr<TESObjectREFR>, Ptr) -> Ptr),
        entry!(0x0056fba0, fn_0056fba0(Ptr<TESObjectREFR>, Ptr) -> Ptr),
        entry!(
            0x0056fc30,
            tes_object_refr_get_bound_max(Ptr, u32) -> Ptr
        ),
        entry!(0x0056fcb0, fn_0056fcb0(Ptr<TESObjectREFR>)),
        entry!(
            0x005702e0,
            tes_object_refr_set_3d(Ptr<TESObjectREFR>, u32, bool)
        ),
        entry!(0x00570ea0, fn_00570ea0(Ptr) -> bool),
        entry!(0x00570ec0, fn_00570ec0(Ptr) -> bool),
        entry!(0x00570ee0, fn_00570ee0(Ptr) -> bool),
        entry!(0x00570f00, fn_00570f00(Ptr) -> bool),
        entry!(0x00570f40, fn_00570f40(Ptr) -> bool),
        entry!(0x00570f60, fn_00570f60() -> u32),
        entry!(0x00570f70, fn_00570f70(Ptr<TESObjectREFR>, u32)),
        entry!(
            0x00571080,
            tes_object_refr_set_3d_very_simple(Ptr<TESObjectREFR>, u32)
        ),
        entry!(0x005710c0, fn_005710c0(Ptr<TESObjectREFR>) -> bool),
        entry!(0x00571530, fn_00571530(Ptr, u32, u32) -> u32),
        entry!(0x00571550, fn_00571550() -> u32),
        entry!(
            0x00571560,
            tes_object_refr_clear_interpolators(Ptr<TESObjectREFR>)
        ),
        entry!(
            0x005715d0,
            tes_object_refr_get_model(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(
            0x00571600,
            tes_object_refr_get_model_bound_size(Ptr<TESObjectREFR>) -> f32
        ),
        entry!(
            0x00571630,
            tes_object_refr_get_tes_model(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(0x00571760, fn_00571760(Ptr<TESObjectREFR>, u32)),
        entry!(
            0x00571b50,
            tes_object_refr_remove_weapon(Ptr<TESObjectREFR>)
        ),
        entry!(0x00572160, fn_00572160(Ptr)),
        entry!(0x005721e0, fn_005721e0(Ptr) -> bool),
        entry!(0x00572220, fn_00572220(u8)),
        entry!(
            0x00572230,
            tes_object_refr_mark_as_picked_up(Ptr<TESObjectREFR>)
        ),
        entry!(
            0x00572270,
            tes_object_refr_mark_as_deleted(Ptr<TESObjectREFR>)
        ),
        entry!(
            0x005722c0,
            tes_object_refr_is_dead(Ptr<TESObjectREFR>, u32) -> bool
        ),
        entry!(
            0x00572350,
            tes_object_refr_is_3d_critical(Ptr<TESObjectREFR>) -> bool
        ),
        entry!(0x00572380, fn_00572380(Ptr<TESObjectREFR>, u32) -> f32),
        entry!(
            0x005723b0,
            tes_object_refr_get_distance_from_reference(
                Ptr<TESObjectREFR>,
                Ptr<TESObjectREFR>,
                u8,
                u8,
            ) -> f32
        ),
        entry!(
            0x00572500,
            tes_object_refr_get_delta_for_angle(Ptr<TESObjectREFR>, Ptr, f32) -> Ptr
        ),
        entry!(0x005725b0, fn_005725b0(Ptr<TESObjectREFR>) -> bool),
        entry!(0x005725f0, fn_005725f0(Ptr<TESObjectREFR>) -> u32),
        entry!(0x00572610, fn_00572610(Ptr<TESObjectREFR>, u32)),
        entry!(0x005726c0, fn_005726c0(Ptr) -> Ptr),
        entry!(
            0x005726e0,
            tes_object_refr_set_light(Ptr<TESObjectREFR>, u32, u8)
        ),
        entry!(
            0x00572750,
            tes_object_refr_get_spell_effect_light(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(0x00572770, fn_00572770(Ptr<TESObjectREFR>, u32)),
        entry!(0x00572820, fn_00572820(Ptr<TESObjectREFR>)),
        entry!(0x005728c0, fn_005728c0(Ptr<TESObjectREFR>, u8)),
        entry!(
            0x005729e0,
            tes_object_refr_remove_light(Ptr<TESObjectREFR>, u8)
        ),
        entry!(
            0x00572a50,
            tes_object_refr_kill_light(Ptr<TESObjectREFR>, u8)
        ),
        entry!(0x00572b00, fn_00572b00(Ptr<TESObjectREFR>, Ptr) -> Ptr),
        entry!(0x00572b30, fn_00572b30(Ptr<TESObjectREFR>, Ptr) -> Ptr),
        entry!(0x00572b60, fn_00572b60(Ptr<TESObjectREFR>, u32, u32, u32)),
        entry!(0x00572c80, fn_00572c80(Ptr<TESObjectREFR>) -> bool),
        entry!(0x00572d10, fn_00572d10(Ptr<TESObjectREFR>) -> u8),
        entry!(0x00572d30, fn_00572d30(Ptr<TESObjectREFR>, u32) -> bool),
        entry!(0x00572d50, fn_00572d50(Ptr<TESObjectREFR>, u32)),
        entry!(
            0x00572db0,
            tes_object_refr_clear_action(Ptr<TESObjectREFR>, u32)
        ),
        entry!(0x00572e10, fn_00572e10(Ptr<TESObjectREFR>, u32)),
        entry!(
            0x00572e30,
            tes_object_refr_get_action_ref(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(
            0x00572e50,
            tes_object_refr_set_animation(Ptr<TESObjectREFR>, u32) -> u32
        ),
        entry!(0x00572fa0, fn_00572fa0(Ptr) -> u8),
        entry!(
            0x00572fc0,
            tes_object_refr_set_dismembered(Ptr<TESObjectREFR>, i32, u32, u32, u32, u8)
        ),
        entry!(0x00573050, fn_00573050(Ptr<TESObjectREFR>, u32, u8)),
        entry!(
            0x00573090,
            tes_object_refr_get_dismembered(Ptr<TESObjectREFR>, u32) -> bool
        ),
        entry!(0x005730d0, fn_005730d0(Ptr<TESObjectREFR>) -> i32),
        entry!(
            0x00573110,
            tes_object_refr_get_last_hit_limb(Ptr<TESObjectREFR>) -> i32
        ),
        entry!(0x00573150, fn_00573150(Ptr<TESObjectREFR>) -> u32),
        entry!(
            0x00573170,
            tes_object_refr_activate(Ptr<TESObjectREFR>, u32, u8, u32, u32) -> bool
        ),
        entry!(0x005737e0, fn_005737e0(Ptr) -> u8),
        entry!(
            0x00573800,
            tes_object_refr_move_ref_to_new_space(Ptr<TESObjectREFR>, u32, u32)
        ),
        entry!(0x00573eb0, fn_00573eb0(Ptr) -> bool),
        entry!(0x00573ed0, fn_00573ed0(Ptr, u8)),
        entry!(0x00573f00, fn_00573f00(Ptr, u8)),
        entry!(0x00573f20, fn_00573f20(Ptr, f32)),
        entry!(0x00573f40, fn_00573f40(Ptr<TESObjectREFR>)),
        entry!(0x005743f0, fn_005743f0()),
        entry!(0x00574400, tes_object_refr_disable(Ptr<TESObjectREFR>)),
        entry!(
            0x00575650,
            tes_object_refr_get_world_location(Ptr<TESObjectREFR>, Ptr) -> Ptr
        ),
        entry!(
            0x00575690,
            tes_object_refr_set_object_reference(Ptr<TESObjectREFR>, u32)
        ),
        entry!(0x00575700, fn_00575700(Ptr<TESObjectREFR>, u32, u32, u32)),
        entry!(0x00575770, fn_00575770(Ptr<TESObjectREFR>, f32)),
        entry!(
            0x005757a0,
            tes_object_refr_set_angle_on_reference_y(Ptr<TESObjectREFR>, f32)
        ),
        entry!(0x005757d0, fn_005757d0(Ptr<TESObjectREFR>, f32)),
        entry!(
            0x00575830,
            tes_object_refr_set_location_on_reference(Ptr<TESObjectREFR>, Ptr)
        ),
        entry!(
            0x00575b70,
            tes_object_refr_set_location_on_reference_z(Ptr<TESObjectREFR>, f32)
        ),
        entry!(
            0x00575bb0,
            tes_object_refr_set_parent_cell(Ptr<TESObjectREFR>, u32)
        ),
        entry!(0x00575c90, fn_00575c90(u8)),
        entry!(
            0x00575ca0,
            tes_object_refr_get_space(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(
            0x00575d10,
            tes_object_refr_get_interior(Ptr<TESObjectREFR>) -> u8
        ),
        entry!(
            0x00575d70,
            tes_object_refr_get_world_space(Ptr<TESObjectREFR>) -> u32
        ),
        entry!(0x00575dc0, fn_00575dc0(Ptr) -> u32),
        entry!(0x00575e30, fn_00575e30(Ptr<TESObjectREFR>) -> u32),
        entry!(0x00575f30, fn_00575f30(Ptr<TESObjectREFR>) -> u32),
        entry!(0x00576040, fn_00576040(u32) -> bool),
        entry!(0x00576070, fn_00576070(Ptr<TESObjectREFR>) -> u8),
        entry!(0x005760e0, fn_005760e0(Ptr<TESObjectREFR>) -> bool),
        entry!(0x00576100, fn_00576100(Ptr) -> bool),
        entry!(
            0x00576130,
            tes_object_refr_add_objectto_container(Ptr<TESObjectREFR>, u32, u32, u32)
        ),
        entry!(
            0x00576260,
            tes_object_refr_get_inventory_item(Ptr<TESObjectREFR>, u32, u32) -> u32
        ),
        entry!(0x005762b0, fn_005762b0(Ptr<TESObjectREFR>, Ptr) -> Ptr),
        entry!(0x00576330, fn_00576330(Ptr<TESObjectREFR>, u32) -> u32),
        entry!(0x00576390, fn_00576390(Ptr<TESObjectREFR>, u32) -> u32),
        entry!(0x005763f0, fn_005763f0(Ptr<TESObjectREFR>, u32) -> u32),
        entry!(
            0x00576420,
            tes_object_refr_get_face_animation_data(Ptr<TESObjectREFR>, u32) -> u32
        ),
        entry!(
            0x00576470,
            tes_object_refr_clamp_to_ground(Ptr<TESObjectREFR>) -> bool
        ),
        entry!(0x005764f0, fn_005764f0(u32, u32)),
        entry!(0x00576640, fn_00576640(Ptr, u8)),
        entry!(0x00576660, fn_00576660(Ptr, u8)),
        entry!(0x00576680, fn_00576680(Ptr<TESObjectREFR>) -> bool),
        entry!(
            0x00576760,
            tes_object_refr_clean_up_traps(Ptr<TESObjectREFR>)
        ),
        entry!(0x00576800, fn_00576800(Ptr, u32)),
        entry!(0x00576830, fn_00576830(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// A double that returns `value` in `eax`.
    fn returns(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| ret(value));
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn addresses(log: &[(u32, Vec<u32>)]) -> Vec<u32> {
        log.iter().map(|(a, _)| *a).collect()
    }

    /// Every function address this file calls (the `0x0123_4567` literals of
    /// its own source), so a test only has to say what matters.
    fn called_addresses() -> Vec<u32> {
        let source = include_str!("tesobjectrefr_p4.rs");
        let bytes = source.as_bytes();
        let mut found = Vec::new();
        let mut i = 0;
        while i + 11 <= bytes.len() {
            if &bytes[i..i + 2] == b"0x" && bytes[i + 6] == b'_' {
                let (Some(high), Some(low)) = (source.get(i + 2..i + 6), source.get(i + 7..i + 11))
                else {
                    i += 1;
                    continue;
                };
                let text = [high, low].concat();
                if let Ok(value) = u32::from_str_radix(&text, 16) {
                    if (0x0040_0000..0x0100_0000).contains(&value) {
                        found.push(value);
                    }
                }
            }
            i += 1;
        }
        found.sort_unstable();
        found.dedup();
        found
    }

    /// An engine with the small accessors registered with the bodies the exe
    /// gives them, every other callee of this file stubbed out (returns 0)
    /// and the pages of the globals mapped.
    fn engine() -> Engine {
        let mut e = Engine::new();
        // every other callee of this file does nothing and returns 0 (the real
        // translations of other units must not run), except the functions of
        // this file itself
        let mine: Vec<u32> = funcs().into_iter().map(|(addr, _)| addr).collect();
        for addr in called_addresses() {
            if !mine.contains(&addr) {
                e.register(addr, |_, _| Ret::default());
            }
        }
        // the named constants of the main file this part calls
        for addr in [FORM_ID, GET_REF_PERSISTS, SPRINTF] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(GET_EXTRA_LIST, |_, a| ret(a[0] + 0x44));
        e.register(GET_BASE_FORM, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(FORM_TYPE, |e, a| ret(u32::from(e.mem.u8(a[0] + 4))));
        e.register(GET_PARENT_CELL, |e, a| ret(e.mem.u32(a[0] + 0x40)));
        e.register(GET_3D, |e, a| {
            let loaded = e.mem.u32(a[0] + 0x64);
            ret(if loaded != 0 {
                e.mem.u32(loaded + 0x14)
            } else {
                0
            })
        });
        e.register(READ_FIRST_DWORD, |e, a| ret(e.mem.u32(a[0])));
        e.register(LIST_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(NOTHING_CONSTRUCTOR, |_, a| ret(a[0]));
        e.register(NODE_PARENT, |e, a| ret(e.mem.u32(a[0] + 0x18)));
        e.register(GET_PROCESS, |e, a| ret(e.mem.u32(a[0] + 0x68)));
        e.register(FORM_IS_DELETED, |e, a| {
            ret(u32::from(e.mem.u32(a[0] + 8) & 0x20 != 0))
        });
        e.register(FORM_IS_DISABLED, |e, a| {
            ret(u32::from(e.mem.u32(a[0] + 8) & 0x800 != 0))
        });
        // NiPointer constructor from a pointer and assignment: both store it
        e.register(0x0063_3c90, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(0x0066_b0d0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(NI_OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(HAVOK_ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        for page in [
            0x0101_2000,
            0x0101_3000,
            0x0101_6000,
            0x011b_1000,
            0x0118_a000,
            0x011c_3000,
            0x011c_6000,
            0x011c_a000,
            0x011d_c000,
            0x011d_d000,
            0x011d_e000,
            0x011d_f000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        e
    }

    /// A zeroed object of `size` bytes whose vtable has the given
    /// `(byte offset, value)` slots: each slot is a double returning
    /// `value`, registered at the address of the slot itself (see
    /// [`slot`]).
    fn object(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(size);
        let vtable = e.mem.alloc(0x800);
        for (offset, value) in slots {
            let value = *value;
            e.register_double(vtable + offset, move |_, _| ret(value));
            e.mem.set_u32(vtable + offset, vtable + offset);
        }
        e.mem.set_u32(object, vtable);
        object
    }

    /// The address a virtual call of `slot` on `object` lands on (what the
    /// call log names).
    fn slot(e: &Engine, object: u32, slot: u32) -> u32 {
        e.mem.u32(object) + slot
    }

    /// A reference (zeroed `0x700` bytes, so actor fields exist) whose
    /// `IsActor` answer is `actor` and whose other virtual slots are the
    /// given ones.
    fn reference(e: &mut Engine, actor: bool, slots: &[(u32, u32)]) -> u32 {
        let mut all = vec![(0x100, u32::from(actor))];
        all.extend_from_slice(slots);
        object(e, 0x700, &all)
    }

    /// A form object with a type byte and flags.
    fn form(e: &mut Engine, kind: u8, flags: u32) -> u32 {
        let form = e.mem.alloc(0x100);
        e.mem.set_u8(form + 4, kind);
        e.mem.set_u32(form + 8, flags);
        form
    }

    /// Gives the reference a loaded-data block whose 3D slot holds `node`.
    fn set_node(e: &mut Engine, refr: u32, node: u32) {
        let loaded = e.mem.alloc(0x40);
        e.mem.set_u32(loaded + 0x14, node);
        e.mem.set_u32(refr + 0x64, loaded);
    }

    /// Runs `body` with the call log on and returns the log.
    fn logged(e: &mut Engine, body: impl FnOnce(&mut Engine)) -> Vec<(u32, Vec<u32>)> {
        e.call_log = Some(vec![]);
        body(e);
        e.call_log.take().unwrap()
    }

    fn float_bits(value: f32) -> u32 {
        value.to_bits()
    }

    // ---- 0056ea40, 0056ea60, 0056ea90 ------------------------------------

    #[test]
    fn constructor_wrapper_0056ea40_runs_the_base_constructor_and_returns_this() {
        let mut e = engine();
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_ea40, &args![0x1234u32]).u32(), 0x1234);
        });
        assert_eq!(calls_to(&log, 0x0056_e8d0), vec![vec![0x1234]]);
    }

    #[test]
    fn sphere_shape_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = engine();
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_ea60, &args![0x4000u32, 1u32]).u32(), 0x4000);
        });
        assert_eq!(calls_to(&log, 0x00ca_0660), vec![vec![0x4000]]);
        assert_eq!(calls_to(&log, NI_OPERATOR_DELETE), vec![vec![0x4000, 0x14]]);

        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_ea60, &args![0x4000u32, 2u32]).u32(), 0x4000);
        });
        assert_eq!(calls_to(&log, 0x00ca_0660).len(), 1);
        assert!(calls_to(&log, NI_OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn destructor_wrapper_0056ea90_runs_the_base_destructor() {
        let mut e = engine();
        let log = logged(&mut e, |e| {
            e.call(0x0056_ea90, &args![0x1234u32]);
        });
        assert_eq!(calls_to(&log, 0x0056_d730), vec![vec![0x1234]]);
    }

    // ---- 0056eab0 ----------------------------------------------------------

    /// A reference in a cell with a world and a primitive extra of type
    /// `kind` (half extents 1.5, 2.5, 3.5), with the Havok callees standing in.
    /// Returns the engine, the reference, the world, the body, the node, and
    /// the cinfo words (filter, shape) seen when the body is built.
    #[allow(clippy::type_complexity)]
    fn collision_setup(kind: u32) -> (Engine, u32, u32, u32, u32, Rc<RefCell<Vec<u32>>>) {
        let mut e = engine();
        let node = object(&mut e, 0x100, &[]);
        let position = e.mem.alloc(0x10);
        let refr = reference(&mut e, false, &[(0x1d0, node), (0x1f4, position)]);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        let world = e.mem.alloc(0x40);
        returns(&mut e, 0x0045_43c0, world);
        let primitive = e.mem.alloc(0x40);
        e.mem.set_u32(primitive + 4, kind);
        e.mem.set_f32(primitive + 0x18, 1.5);
        e.mem.set_f32(primitive + 0x1c, 2.5);
        e.mem.set_f32(primitive + 0x20, 3.5);
        returns(&mut e, GET_PRIMITIVE, primitive);
        // 00413fc0: copies the three floats at +0x18 to its argument
        e.register(0x0041_3fc0, |e, a| {
            copy_words(e, a[1], a[0] + 0x18, 3);
            ret(a[1])
        });
        // 004a3e90: a length scaled by 2
        e.register(0x004a_3e90, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) * 2.0,
            ..Ret::default()
        });
        e.register(0x0056_e610, |_, a| ret(a[0]));
        e.register(0x0056_e950, |_, a| ret(a[0]));
        e.register(0x004a_e750, |_, a| ret(a[0] + 8));
        e.register(0x00c4_2f70, |_, a| ret(a[0]));
        e.register(0x0056_d580, |_, a| ret(a[0]));
        let body = object(&mut e, 0x40, &[(0xdc, 0), (0xe0, 0), (0x9c, 0)]);
        // 0056d380: the body; the cinfo words at that moment are recorded
        let seen = Rc::new(RefCell::new(Vec::new()));
        let record = seen.clone();
        e.register_double(0x0056_d380, move |e, a| {
            let mut seen = record.borrow_mut();
            for offset in [0u32, 4, 0x20, 0x24] {
                seen.push(e.mem.u32(a[1] + offset));
            }
            ret(body)
        });
        (e, refr, world, body, node, seen)
    }

    #[test]
    fn collision_of_a_sphere_primitive_builds_a_sphere_body_in_the_world() {
        let (mut e, refr, world, body, node, seen) = collision_setup(2);
        let log = logged(&mut e, |e| {
            e.call(0x0056_eab0, &args![refr]);
        });
        let spheres = calls_to(&log, 0x0056_e950);
        assert_eq!(spheres.len(), 1);
        // the radius is the scaled first half extent, the second argument 0
        assert_eq!(spheres[0][1], float_bits(3.0));
        assert_eq!(spheres[0][2], 0);
        assert!(calls_to(&log, 0x0056_e610).is_empty());
        let sphere = spheres[0][0];
        assert_eq!(calls_to(&log, NI_OPERATOR_NEW)[0], vec![0x14]);
        // the cinfo holds the shape (through 004ae750) and the default layer 3
        assert_eq!(
            *seen.borrow(),
            vec![0x3_0003, sphere + 8, 0x3_0003, sphere + 8]
        );
        let flags = calls_to(&log, 0x0051_9230);
        assert_eq!(flags.len(), 1);
        assert_eq!(flags[0][1..], [2, 1]);
        assert_eq!(
            calls_to(&log, NODE_ADD_EXTRA_DATA),
            vec![vec![node, flags[0][0]]]
        );
        // the body is told the world (virtual +0x9c) and tied to the node
        assert_eq!(
            calls_to(&log, slot(&e, body, 0x9c)),
            vec![vec![body, world]]
        );
        assert_eq!(calls_to(&log, 0x00c8_5c10), vec![vec![body, node, 0]]);
        assert_eq!(calls_to(&log, 0x00c6_b980).len(), 1);
        assert_eq!(calls_to(&log, 0x00c6_b980)[0][1], body);
        // the cinfo is destroyed last
        assert_eq!(log.last().unwrap().0, 0x0056_d730);
    }

    #[test]
    fn collision_of_box_primitives_converts_the_half_extents() {
        // type 3 (a plane) replaces the second half extent with the global at 01013ea4
        for (kind, expected) in [(1u32, [1.5f32, 2.5, 3.5]), (3u32, [1.5, 0.04, 3.5])] {
            let (mut e, refr, ..) = collision_setup(kind);
            e.set_global(0x0101_3ea4, 0.04f32);
            let converted = Rc::new(RefCell::new(Vec::new()));
            let record = converted.clone();
            e.register_double(CONVERT_VECTOR, move |e, a| {
                // the first conversion is the half extents (source = a[1])
                if record.borrow().is_empty() {
                    for offset in [0u32, 4, 8, 12] {
                        record.borrow_mut().push(e.mem.f32(a[1] + offset));
                    }
                }
                Ret::default()
            });
            let log = logged(&mut e, |e| {
                e.call(0x0056_eab0, &args![refr]);
            });
            assert_eq!(calls_to(&log, 0x0056_e610).len(), 1, "kind {kind}");
            assert!(calls_to(&log, 0x0056_e950).is_empty());
            let seen = converted.borrow();
            assert_eq!(seen[..3], expected, "kind {kind}");
        }
    }

    #[test]
    fn collision_uses_the_collision_data_layer_when_the_extra_has_one() {
        let (mut e, refr, _, _, _, seen) = collision_setup(2);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 7);
        returns(&mut e, 0x0042_11a0, data);
        e.call(0x0056_eab0, &args![refr]);
        assert_eq!(seen.borrow()[0], 0x3_0007);
        assert_eq!(seen.borrow()[2], 0x3_0007);
    }

    #[test]
    fn collision_needs_a_primitive_and_a_world() {
        let (mut e, refr, ..) = collision_setup(2);
        returns(&mut e, GET_PRIMITIVE, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_eab0, &args![refr]);
        });
        assert!(calls_to(&log, NI_OPERATOR_NEW).is_empty());

        let (mut e, refr, ..) = collision_setup(2);
        returns(&mut e, 0x0045_43c0, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_eab0, &args![refr]);
        });
        assert!(calls_to(&log, NI_OPERATOR_NEW).is_empty());

        // no parent cell at all
        let (mut e, refr, ..) = collision_setup(2);
        e.mem.set_u32(refr + 0x40, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_eab0, &args![refr]);
        });
        assert!(calls_to(&log, 0x0045_43c0).is_empty());
        assert!(calls_to(&log, NI_OPERATOR_NEW).is_empty());
    }

    // ---- 0056f0f0, 0056f110 ----------------------------------------------

    #[test]
    fn cinfo_filter_info_is_stored_twice() {
        let mut e = engine();
        let cinfo = e.mem.alloc(0xe0);
        e.call(0x0056_f0f0, &args![cinfo, 0x5_0015u32]);
        assert_eq!(e.mem.u32(cinfo), 0x5_0015);
        assert_eq!(e.mem.u32(cinfo + 0x20), 0x5_0015);
    }

    #[test]
    fn cinfo_shape_is_stored_twice_from_the_accessor() {
        let mut e = engine();
        e.register(0x004a_e750, |_, a| ret(a[0] + 0x10));
        let cinfo = e.mem.alloc(0xe0);
        e.call(0x0056_f110, &args![cinfo, 0x2000u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0x2010);
        assert_eq!(e.mem.u32(cinfo + 0x24), 0x2010);
        // a null shape
        e.register(0x004a_e750, |_, _| Ret::default());
        e.call(0x0056_f110, &args![cinfo, 0u32]);
        assert_eq!(e.mem.u32(cinfo + 4), 0);
        assert_eq!(e.mem.u32(cinfo + 0x24), 0);
    }

    // ---- 0056f140 ----------------------------------------------------------

    #[test]
    fn collision_wrapper_builds_the_bounding_volume_shape_and_adds_the_body() {
        let (mut e, refr, world, body, node, seen) = collision_setup(1);
        // allocation sizes of the Havok objects are logged by the engine's double
        e.register(0x00c9_ddb0, |_, a| ret(a[0]));
        e.register(0x00c9_d990, |_, a| ret(a[0]));
        e.set_global(0x011b_153c, 0.5f32);
        let converted = Rc::new(RefCell::new(Vec::new()));
        let record = converted.clone();
        e.register_double(CONVERT_VECTOR, move |e, a| {
            if record.borrow().is_empty() {
                for offset in [0u32, 4, 8] {
                    record.borrow_mut().push(e.mem.f32(a[1] + offset));
                }
            }
            Ret::default()
        });
        let log = logged(&mut e, |e| {
            e.call(0x0056_f140, &args![refr]);
        });
        assert_eq!(*converted.borrow(), vec![1.5, 2.5, 3.5]);
        let sizes: Vec<u32> = calls_to(&log, HAVOK_ALLOCATE)
            .iter()
            .map(|a| a[0])
            .collect();
        assert_eq!(sizes, vec![0x30, 0x10, 0x1c]);
        let boxes = calls_to(&log, 0x00c9_ddb0);
        assert_eq!(boxes.len(), 1);
        assert_eq!(boxes[0][2], float_bits(0.5));
        // the companion got its own vtable
        let companions = calls_to(&log, 0x0056_d610);
        assert_eq!(companions.len(), 1);
        assert_eq!(e.mem.u32(companions[0][0]), 0x0103_0dc0);
        // the wrapping shape is built from the box and the companion
        let wrappers = calls_to(&log, 0x00c9_d990);
        assert_eq!(wrappers.len(), 1);
        assert_eq!(wrappers[0][1], boxes[0][0]);
        assert_eq!(wrappers[0][2], companions[0][0]);
        // the cinfo: wrapping shape at +4, fixed filter 0x50015 (words 0 and 0x20 are
        // only set by 0056f0f0, so the hk copy stays 0)
        assert_eq!(seen.borrow()[0], 0x5_0015);
        assert_eq!(seen.borrow()[1], wrappers[0][0]);
        // position and rotation are set on the cinfo, and the body added to the world
        assert_eq!(calls_to(&log, 0x0056_d300).len(), 1);
        assert_eq!(calls_to(&log, 0x0056_d320).len(), 1);
        assert_eq!(calls_to(&log, 0x0056_d2c0), vec![vec![world, body]]);
        assert_eq!(calls_to(&log, 0x00c8_5c10), vec![vec![body, node, 0]]);
        assert_eq!(log.last().unwrap().0, 0x0056_d730);
    }

    #[test]
    fn collision_wrapper_needs_a_world() {
        let (mut e, refr, ..) = collision_setup(1);
        returns(&mut e, 0x0045_43c0, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_f140, &args![refr]);
        });
        assert!(calls_to(&log, HAVOK_ALLOCATE).is_empty());
    }

    // ---- 0056f690, 0056f6b0 ----------------------------------------------

    #[test]
    fn companion_constructor_sets_its_vtable_after_the_base_constructor() {
        let mut e = engine();
        let object = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_f690, &args![object]).u32(), object);
        });
        assert_eq!(calls_to(&log, 0x0056_d610), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), 0x0103_0dc0);
    }

    #[test]
    fn base_form_query_0056f6b0_asks_the_base_object() {
        let mut e = engine();
        let refr = e.mem.alloc(0x100);
        assert!(!e.call(0x0056_f6b0, &args![refr]).bool());

        let yes = object(&mut e, 0x40, &[(0xa0, 1)]);
        e.mem.set_u32(refr + 0x20, yes);
        assert!(e.call(0x0056_f6b0, &args![refr]).bool());

        let no = object(&mut e, 0x40, &[(0xa0, 0)]);
        e.mem.set_u32(refr + 0x20, no);
        assert!(!e.call(0x0056_f6b0, &args![refr]).bool());
    }

    // ---- 0056f700 ----------------------------------------------------------

    /// A reference in a cell for the 3D refresh: model loader at the global,
    /// base object of type `base_kind`.
    fn refresh_setup(actor: bool, base_kind: u8, flags: u32) -> (Engine, u32, u32) {
        let mut e = engine();
        let loader = e.mem.alloc(0x20);
        e.set_global(GLOBAL_MODEL_LOADER, loader);
        let tes = e.mem.alloc(0x20);
        e.set_global(GLOBAL_TES, tes);
        let loading = e.mem.alloc(0x20);
        e.set_global(GLOBAL_LOADING_OBJECT, loading);
        let refr = reference(&mut e, actor, &[(0x240, 0), (0x1c4, 0)]);
        let base = form(&mut e, base_kind, 0);
        e.mem.set_u32(refr + 0x20, base);
        e.mem.set_u32(refr + 8, flags);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        returns(&mut e, 0x0045_8be0, 0x77);
        (e, refr, loader)
    }

    #[test]
    fn refresh_queues_an_enabled_reference_without_3d_with_the_model_loader() {
        let (mut e, refr, loader) = refresh_setup(false, 0x1, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_f700, &args![refr]);
        });
        let cell = e.mem.u32(refr + 0x40);
        let tes = e.global::<u32>(GLOBAL_TES);
        assert_eq!(calls_to(&log, 0x0045_8be0), vec![vec![tes, cell, 0]]);
        assert_eq!(
            calls_to(&log, 0x0044_4850),
            vec![vec![loader, refr, 0x77, 1]]
        );
        assert!(calls_to(&log, 0x0096_f400).is_empty());
        // no 3D: no node work
        assert!(calls_to(&log, NODE_UPDATE).is_empty());
    }

    #[test]
    fn refresh_removes_an_actor_from_the_temp_change_list_before_queueing() {
        let (mut e, refr, loader) = refresh_setup(true, 0x2a, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_f700, &args![refr]);
        });
        let order = addresses(&log);
        let change = order.iter().position(|a| *a == 0x0096_f400).unwrap();
        let queue = order.iter().position(|a| *a == 0x0044_4850).unwrap();
        assert!(change < queue);
        assert_eq!(
            calls_to(&log, 0x0096_f400),
            vec![vec![OBJECT_PROCESS_LISTS, refr]]
        );
        assert_eq!(calls_to(&log, slot(&e, refr, 0x240)), vec![vec![refr]]);
        assert_eq!(calls_to(&log, 0x0044_4850)[0][0], loader);
    }

    #[test]
    fn refresh_skips_references_the_loader_knows_or_that_are_deleted_or_saved() {
        // already known to the model loader
        let (mut e, refr, _) = refresh_setup(false, 0x1, 0);
        returns(&mut e, 0x0044_5750, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0056_f700, &args![refr]);
        });
        assert!(calls_to(&log, 0x0044_4850).is_empty());
        // deleted
        let (mut e, refr, _) = refresh_setup(false, 0x1, 0x20);
        let log = logged(&mut e, |e| {
            e.call(0x0056_f700, &args![refr]);
        });
        assert!(calls_to(&log, 0x0044_4850).is_empty());
        // being saved with a base object that is not of type 0x1e
        let (mut e, refr, _) = refresh_setup(false, 0x1, 0);
        returns(&mut e, 0x0044_4ed0, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0056_f700, &args![refr]);
        });
        assert!(calls_to(&log, 0x0044_4850).is_empty());
        // ... but a base object of type 0x1e is exempt
        let (mut e, refr, _) = refresh_setup(false, 0x1e, 0);
        returns(&mut e, 0x0044_4ed0, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0056_f700, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x0044_4850).len(), 1);
    }

    #[test]
    fn refresh_of_a_disabled_reference_sets_the_script_action_flag_unless_loading() {
        let (mut e, refr, _) = refresh_setup(false, 0x1, 0x800);
        let log = logged(&mut e, |e| {
            e.call(0x0056_f700, &args![refr]);
        });
        assert!(calls_to(&log, 0x0044_4850).is_empty());
        let extra = refr + 0x44;
        assert_eq!(calls_to(&log, 0x005a_c190), vec![vec![refr, extra]]);
        assert_eq!(calls_to(&log, 0x005a_c750), vec![vec![refr, extra, 0x1000]]);

        let (mut e, refr, _) = refresh_setup(false, 0x1, 0x800);
        returns(&mut e, 0x0042_ce10, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0056_f700, &args![refr]);
        });
        assert!(calls_to(&log, 0x005a_c190).is_empty());
        assert!(calls_to(&log, 0x005a_c750).is_empty());
    }

    #[test]
    fn refresh_with_a_3d_resets_the_node_and_restores_havok_data() {
        // a non-actor: the saved Havok data is restored after the virtual
        let (mut e, refr, _) = refresh_setup(false, 0x1, 0);
        let node = object(&mut e, 0x100, &[]);
        set_node(&mut e, refr, node);
        returns(&mut e, 0x0044_5750, 1); // not queued again
        let log = logged(&mut e, |e| {
            e.call(0x0056_f700, &args![refr]);
        });
        let translate = calls_to(&log, NODE_UPDATE);
        assert_eq!(translate.len(), 1);
        assert_eq!(translate[0][0], node);
        let order = addresses(&log);
        let virtual_call = order
            .iter()
            .position(|a| *a == slot(&e, refr, 0x1c4))
            .unwrap();
        let restore = order.iter().position(|a| *a == 0x0042_2bc0).unwrap();
        assert!(virtual_call < restore);
        assert_eq!(calls_to(&log, 0x0042_2bc0), vec![vec![refr + 0x44, refr]]);
        assert!(calls_to(&log, 0x008b_6ae0).is_empty());

        // an actor: limbs are rebuilt first, then the critical stage and velocity
        let (mut e, refr, _) = refresh_setup(true, 0x2a, 0);
        let node = object(&mut e, 0x100, &[]);
        set_node(&mut e, refr, node);
        returns(&mut e, 0x0044_5750, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0056_f700, &args![refr]);
        });
        let order = addresses(&log);
        let limbs = order.iter().position(|a| *a == 0x008b_6ae0).unwrap();
        let virtual_call = order
            .iter()
            .position(|a| *a == slot(&e, refr, 0x1c4))
            .unwrap();
        let critical = order.iter().position(|a| *a == 0x008a_1a70).unwrap();
        let velocity = order.iter().position(|a| *a == 0x008b_65f0).unwrap();
        assert!(limbs < virtual_call && virtual_call < critical && critical < velocity);
        assert_eq!(calls_to(&log, 0x0042_2bc0), vec![vec![refr + 0x44, refr]]);
    }

    #[test]
    fn refresh_without_3d_runs_the_virtual_only_for_a_primitive() {
        let (mut e, refr, _) = refresh_setup(false, 0x1, 0x20);
        let log = logged(&mut e, |e| {
            e.call(0x0056_f700, &args![refr]);
        });
        assert!(calls_to(&log, slot(&e, refr, 0x1c4)).is_empty());

        returns(&mut e, GET_PRIMITIVE, 0x5000);
        let log = logged(&mut e, |e| {
            e.call(0x0056_f700, &args![refr]);
        });
        assert_eq!(calls_to(&log, slot(&e, refr, 0x1c4)).len(), 1);
    }

    // ---- 0056f930 ----------------------------------------------------------

    /// A node with an owner (virtual +0x10) and a parent (+0x18).
    fn node_with_owner(e: &mut Engine, owner: u32, parent: u32) -> u32 {
        let node = object(e, 0x40, &[(0x10, owner)]);
        e.mem.set_u32(node + 0x18, parent);
        node
    }

    #[test]
    fn find_reference_for_3d_answers_the_player_for_the_first_person_root() {
        let mut e = engine();
        let player = e.mem.alloc(0x40);
        e.set_global(GLOBAL_PLAYER, player);
        let root = node_with_owner(&mut e, 0, 0);
        let leaf = node_with_owner(&mut e, 0, root);
        returns(&mut e, 0x0095_0bb0, root);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_f930, &args![leaf]).u32(), player);
        });
        assert_eq!(calls_to(&log, 0x0095_0bb0), vec![vec![player, 1]]);
    }

    #[test]
    fn find_reference_for_3d_answers_the_reference_of_the_first_owned_ancestor() {
        let mut e = engine();
        let found = e.mem.alloc(0x40);
        let owner = e.mem.alloc(0x40);
        returns(&mut e, 0x009a_d610, found);
        let top = node_with_owner(&mut e, owner, 0);
        let leaf = node_with_owner(&mut e, 0, top);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_f930, &args![leaf]).u32(), found);
        });
        // no player: the first-person root is not asked for
        assert!(calls_to(&log, 0x0095_0bb0).is_empty());
        assert_eq!(calls_to(&log, 0x009a_d610), vec![vec![owner]]);
        // nothing owns anything: 0
        let mut e = engine();
        let top = node_with_owner(&mut e, 0, 0);
        let leaf = node_with_owner(&mut e, 0, top);
        assert_eq!(e.call(0x0056_f930, &args![leaf]).u32(), 0);
        assert_eq!(e.call(0x0056_f930, &args![0u32]).u32(), 0);
    }

    #[test]
    fn find_reference_for_3d_looks_further_up_for_a_base_object_of_type_0x29() {
        let mut e = engine();
        let owner = e.mem.alloc(0x40);
        let outer_owner = e.mem.alloc(0x40);
        let inner = e.mem.alloc(0x40);
        let outer = e.mem.alloc(0x40);
        let base = form(&mut e, 0x29, 0);
        e.mem.set_u32(inner + 0x20, base);
        let plain_base = form(&mut e, 0x1, 0);
        e.mem.set_u32(outer + 0x20, plain_base);
        e.register_double(0x009a_d610, move |_, a| {
            ret(if a[0] == owner {
                inner
            } else if a[0] == outer_owner {
                outer
            } else {
                0
            })
        });
        let outer_node = node_with_owner(&mut e, outer_owner, 0);
        let leaf = node_with_owner(&mut e, owner, outer_node);
        assert_eq!(e.call(0x0056_f930, &args![leaf]).u32(), outer);
        // the parent has no owner: the inner reference stays
        let bare = node_with_owner(&mut e, 0, 0);
        let leaf = node_with_owner(&mut e, owner, bare);
        assert_eq!(e.call(0x0056_f930, &args![leaf]).u32(), inner);
    }

    // ---- 0056fa00 .. 0056fc30 ---------------------------------------------

    #[test]
    fn get_orientation_uses_the_three_angles_of_a_non_actor() {
        let mut e = engine();
        // FromEulerAnglesXYZ writes a recognisable matrix
        e.register(MATRIX_FROM_EULER, |e, a| {
            for i in 0..9 {
                e.mem.set_u32(a[0] + 4 * i, 0x100 + i);
            }
            ret(a[0])
        });
        let refr = reference(&mut e, false, &[]);
        e.mem.set_f32(refr + 0x24, 0.5);
        e.mem.set_f32(refr + 0x28, 1.5);
        e.mem.set_f32(refr + 0x2c, 2.5);
        let out = e.mem.alloc(0x30);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_fa00, &args![refr, out]).u32(), out);
        });
        let call = &calls_to(&log, MATRIX_FROM_EULER)[0];
        assert_eq!(
            call[1..],
            [float_bits(0.5), float_bits(1.5), float_bits(2.5)]
        );
        for i in 0..9 {
            assert_eq!(e.mem.u32(out + 4 * i), 0x100 + i);
        }
    }

    #[test]
    fn get_orientation_of_an_actor_keeps_only_the_heading() {
        let mut e = engine();
        let refr = reference(&mut e, true, &[]);
        e.mem.set_f32(refr + 0x24, 0.5);
        e.mem.set_f32(refr + 0x28, 1.5);
        e.mem.set_f32(refr + 0x2c, 2.5);
        let out = e.mem.alloc(0x30);
        let log = logged(&mut e, |e| {
            e.call(0x0056_fa00, &args![refr, out]);
        });
        let call = &calls_to(&log, MATRIX_FROM_EULER)[0];
        assert_eq!(call[1..], [0, 0, float_bits(2.5)]);
    }

    #[test]
    fn set_orientation_converts_the_matrix_into_the_three_angles() {
        let mut e = engine();
        let refr = e.mem.alloc(0x80);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let record = seen.clone();
        e.register_double(MATRIX_TO_EULER, move |e, a| {
            for i in 0..9 {
                record.borrow_mut().push(e.mem.u32(a[0] + 4 * i));
            }
            Ret::default()
        });
        let log = logged(&mut e, |e| {
            e.call(
                0x0056_fa90,
                &args![refr, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32],
            );
        });
        assert_eq!(*seen.borrow(), (1..=9).collect::<Vec<u32>>());
        let call = &calls_to(&log, MATRIX_TO_EULER)[0];
        assert_eq!(call[1..], [refr + 0x24, refr + 0x28, refr + 0x2c]);
    }

    #[test]
    fn multiple_matrix_by_race_copies_the_matrix_and_reads_the_base_form_type() {
        let mut e = engine();
        let refr = e.mem.alloc(0x80);
        let base = form(&mut e, 0x2a, 0);
        e.mem.set_u32(refr + 0x20, base);
        let input = e.mem.alloc(0x30);
        for i in 0..9 {
            e.mem.set_u32(input + 4 * i, 10 + i);
        }
        let out = e.mem.alloc(0x30);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_fac0, &args![refr, out, input]).u32(), out);
        });
        assert_eq!(calls_to(&log, FORM_TYPE), vec![vec![base]]);
        for i in 0..9 {
            assert_eq!(e.mem.u32(out + 4 * i), 10 + i);
        }
    }

    /// A reference whose 3D node has a bound extra; the corner function
    /// writes (1, 2, 3) to its scratch argument.
    fn bound_setup(corner: u32) -> (Engine, u32, u32, u32) {
        let mut e = engine();
        returns(&mut e, 0x0050_ea90, 0x6000);
        let refr = e.mem.alloc(0x80);
        let node = e.mem.alloc(0x40);
        set_node(&mut e, refr, node);
        let bound = e.mem.alloc(0x40);
        e.register_double(NODE_GET_EXTRA_DATA, move |_, a| {
            ret(if a[1] == 0x6000 { bound } else { 0 })
        });
        e.register(corner, |e, a| {
            e.mem.set_f32(a[1], 1.0);
            e.mem.set_f32(a[1] + 4, 2.0);
            e.mem.set_f32(a[1] + 8, 3.0);
            Ret::default()
        });
        for i in 0..3 {
            e.mem.set_u32(ZERO_VECTOR + 4 * i, 0);
        }
        (e, refr, node, bound)
    }

    #[test]
    fn bound_corners_come_from_the_bound_extra_or_are_zero() {
        for (address, corner, scratch) in [
            (0x0056_fb10u32, 0x0050_ea10u32, SCRATCH_BOUND_MIN),
            (0x0056_fba0, 0x0050_ea50, SCRATCH_BOUND_MAX),
        ] {
            let (mut e, refr, _, bound) = bound_setup(corner);
            let out = e.mem.alloc(0x10);
            let log = logged(&mut e, |e| {
                assert_eq!(e.call(address, &args![refr, out]).u32(), out);
            });
            assert_eq!(calls_to(&log, corner), vec![vec![bound, scratch]]);
            assert_eq!(e.mem.f32(out), 1.0);
            assert_eq!(e.mem.f32(out + 4), 2.0);
            assert_eq!(e.mem.f32(out + 8), 3.0);

            // no bound extra: the zero vector
            e.mem.set_f32(ZERO_VECTOR, 9.0);
            e.register(NODE_GET_EXTRA_DATA, |_, _| Ret::default());
            let out = e.mem.alloc(0x10);
            e.call(address, &args![refr, out]);
            assert_eq!(e.mem.f32(out), 9.0);
            assert_eq!(e.mem.f32(out + 4), 0.0);

            // no 3D: the zero vector as well, without asking for the extra
            let bare = e.mem.alloc(0x80);
            let log = logged(&mut e, |e| {
                e.call(address, &args![bare, out]);
            });
            assert!(calls_to(&log, NODE_GET_EXTRA_DATA).is_empty());
        }
    }

    #[test]
    fn get_bound_max_takes_the_node_directly() {
        let (mut e, _, node, bound) = bound_setup(0x0050_ea50);
        let out = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0056_fc30, &args![out, node]).u32(), out);
        });
        assert_eq!(
            calls_to(&log, 0x0050_ea50),
            vec![vec![bound, SCRATCH_BOUND_MAX]]
        );
        assert_eq!(e.mem.f32(out + 8), 3.0);
        // a null node gives the zero vector
        e.mem.set_f32(ZERO_VECTOR, 7.0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_fc30, &args![out, 0u32]);
        });
        assert_eq!(e.mem.f32(out), 7.0);
        assert!(calls_to(&log, NODE_GET_EXTRA_DATA).is_empty());
    }

    // ---- 0056fcb0 ----------------------------------------------------------

    /// A reference with a node and a base object of type `base_kind`; the
    /// global at 011dea10 is an object too.
    fn unload_setup(base_kind: u8) -> (Engine, u32, u32) {
        let mut e = engine();
        let tes = e.mem.alloc(0x100);
        e.set_global(GLOBAL_TES, tes);
        let node = object(&mut e, 0x40, &[]);
        let refr = reference(&mut e, false, &[(0x1d0, node)]);
        let base = form(&mut e, base_kind, 0);
        e.mem.set_u32(refr + 0x20, base);
        (e, refr, tes)
    }

    #[test]
    fn unload_cleanup_does_nothing_without_a_node() {
        let mut e = engine();
        let refr = reference(&mut e, false, &[(0x1d0, 0)]);
        let log = logged(&mut e, |e| {
            e.call(0x0056_fcb0, &args![refr]);
        });
        assert_eq!(log.len(), 2); // the call and the virtual
    }

    #[test]
    fn unload_cleanup_leaves_water_and_counts_type_0x23_down() {
        let (mut e, refr, tes) = unload_setup(0x23);
        let log = logged(&mut e, |e| {
            e.call(0x0056_fcb0, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x0045_ce60), vec![vec![tes]]);
        assert_eq!(calls_to(&log, 0x0056_5670), vec![vec![refr]]);
        assert_eq!(calls_to(&log, 0x0057_b520), vec![vec![refr, 0]]);
        // no parent cell: nothing about cells
        assert!(calls_to(&log, 0x0054_5360).is_empty());

        // another type: the counter is left alone
        let (mut e, refr, _) = unload_setup(0x1);
        let log = logged(&mut e, |e| {
            e.call(0x0056_fcb0, &args![refr]);
        });
        assert!(calls_to(&log, 0x0045_ce60).is_empty());
        assert_eq!(calls_to(&log, 0x0056_5670).len(), 1);
    }

    #[test]
    fn unload_cleanup_adds_the_room_data_to_each_other_room_item() {
        let (mut e, refr, _) = unload_setup(0x1);
        let other = e.mem.alloc(0x100);
        let lonely = e.mem.alloc(0x100);
        // the list: node 1 holds `other`, node 2 holds `lonely`
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second, lonely);
        let first = e.mem.alloc(0x10);
        e.mem.set_u32(first, other);
        e.mem.set_u32(first + 4, second);
        returns(&mut e, 0x0042_0770, first);
        let own_room = e.mem.alloc(0x20);
        let other_room = e.mem.alloc(0x20);
        e.register_double(0x0056_99b0, move |_, a| {
            ret(if a[0] == refr {
                own_room
            } else if a[0] == other {
                other_room
            } else {
                0
            })
        });
        returns(&mut e, 0x0066_29f0, 0x7777);
        returns(&mut e, 0x0045_c650, 0x8888);
        let log = logged(&mut e, |e| {
            e.call(0x0056_fcb0, &args![refr]);
        });
        let holders = calls_to(&log, 0x0063_3c90);
        assert_eq!(holders.len(), 1);
        assert_eq!(holders[0][1], 0x7777);
        let adds = calls_to(&log, 0x0057_c5d0);
        assert_eq!(adds.len(), 1);
        assert_eq!(adds[0][0], 0x8888);
        // the second local is the NiPointer built from the room data
        assert_eq!(adds[0][2], holders[0][0]);
        assert_eq!(
            calls_to(&log, 0x0045_cec0),
            vec![vec![adds[0][1]], vec![holders[0][0]]]
        );
        assert_eq!(calls_to(&log, 0x0066_29f0), vec![vec![own_room]]);
        assert_eq!(calls_to(&log, 0x0045_c650), vec![vec![other_room]]);
    }

    #[test]
    fn unload_cleanup_takes_a_portal_marker_out_of_its_two_rooms() {
        let (mut e, refr, _) = unload_setup(0x1);
        let portal_base = e.mem.u32(refr + 0x20);
        e.set_global(GLOBAL_PORTAL_MARKER_BASE, portal_base);
        let room = e.mem.alloc(0x100);
        e.register_double(0x0042_0bc0, move |_, a| {
            ret(if a[1] == 0 { room } else { 0 })
        });
        let log = logged(&mut e, |e| {
            e.call(0x0056_fcb0, &args![refr]);
        });
        assert_eq!(
            calls_to(&log, 0x0042_0bc0),
            vec![vec![refr + 0x44, 0], vec![refr + 0x44, 1]]
        );
        assert_eq!(calls_to(&log, 0x0042_0da0), vec![vec![room + 0x44, refr]]);
    }

    #[test]
    fn unload_cleanup_clears_the_water_lights_of_a_type_0x1e_base() {
        // the water node's scheduler runs a process of type 0xd (or not)
        for (process_type, updates_property) in [(0xdu32, true), (0x2, false)] {
            let (mut e, refr, tes) = unload_setup(0x1e);
            let water_node = object(&mut e, 0x40, &[]);
            let water = reference(&mut e, false, &[(0x1d0, water_node)]);
            let item = e.mem.alloc(0x10);
            e.mem.set_u32(item, water);
            returns(&mut e, 0x0041_f810, item);
            let stored = e.mem.alloc(0x10);
            e.mem.set_u32(stored, 0x5151);
            returns(&mut e, 0x0057_25f0, stored);
            let manager = e.mem.alloc(0x20);
            returns(&mut e, 0x0070_ec90, manager);
            let found = e.mem.alloc(0x100);
            returns(&mut e, 0x004e_8030, found);
            let scheduler = e.mem.alloc(0x100);
            e.mem.set_u32(scheduler + 0x68, process_type);
            returns(&mut e, 0x0040_30b0, scheduler);
            let property = e.mem.alloc(0x200);
            returns(&mut e, 0x00a5_9d30, property);
            let seen = Rc::new(RefCell::new(Vec::new()));
            let record = seen.clone();
            e.register_double(0x0057_c730, move |e, a| {
                record.borrow_mut().push((a[0], e.mem.u32(a[1])));
                Ret::default()
            });
            let log = logged(&mut e, |e| {
                e.call(0x0056_fcb0, &args![refr]);
            });
            assert_eq!(
                calls_to(&log, 0x0041_f840),
                vec![vec![water + 0x44, refr, 0]]
            );
            assert_eq!(calls_to(&log, 0x004e_8030), vec![vec![manager, water]]);
            assert_eq!(calls_to(&log, 0x0070_ec90)[0], vec![tes]);
            if updates_property {
                assert_eq!(calls_to(&log, 0x00a5_9d30), vec![vec![found, 3]]);
                assert_eq!(*seen.borrow(), vec![(property + 0x128, 0x5151)]);
            } else {
                assert!(calls_to(&log, 0x00a5_9d30).is_empty());
                assert!(seen.borrow().is_empty());
            }
        }
    }

    #[test]
    fn unload_cleanup_tells_the_acoustic_listener_about_a_type_0xe_base_in_a_cell() {
        let (mut e, refr, _) = unload_setup(0xe);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        returns(&mut e, 0x0045_43c0, 0x1111);
        returns(&mut e, 0x0045_cd60, 0x2222);
        let log = logged(&mut e, |e| {
            e.call(0x0056_fcb0, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x0045_43c0)[0], vec![cell]);
        assert_eq!(calls_to(&log, 0x0045_cd60)[0], vec![0x1111]);
        assert_eq!(calls_to(&log, 0x0061_f890), vec![vec![0x2222, refr, 1]]);
    }

    #[test]
    fn unload_cleanup_leaves_the_cell_lists_of_the_reference() {
        let (mut e, refr, tes) = unload_setup(0x23);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        returns(&mut e, 0x0045_43c0, 0x1111);
        returns(&mut e, 0x0059_bb30, 0x3333);
        returns(&mut e, 0x0070_ec90, 0x4444);
        let log = logged(&mut e, |e| {
            e.call(0x0056_fcb0, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x004e_52f0), vec![vec![0x4444, refr]]);
        assert_eq!(calls_to(&log, 0x0054_5360), vec![vec![cell, refr]]);
        assert_eq!(calls_to(&log, 0x0054_54f0), vec![vec![cell, refr]]);
        assert_eq!(calls_to(&log, 0x0059_bb30), vec![vec![0x1111]]);
        assert_eq!(calls_to(&log, 0x0063_1370), vec![vec![0x3333, refr, 0, 1]]);
        assert_eq!(calls_to(&log, 0x0054_56e0), vec![vec![cell, refr]]);
        assert_eq!(calls_to(&log, 0x0063_12d0), vec![vec![0x3333, refr]]);
        assert_eq!(calls_to(&log, 0x0045_ce60), vec![vec![tes]]);

        // no lighting object: 00631370 is skipped
        returns(&mut e, 0x0059_bb30, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_fcb0, &args![refr]);
        });
        assert!(calls_to(&log, 0x0063_1370).is_empty());
    }

    #[test]
    fn unload_cleanup_updates_the_registry_for_room_multibound_and_portal_markers() {
        // a room marker: the room extra goes to the registry
        let (mut e, refr, _) = unload_setup(0x1);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        let base = e.mem.u32(refr + 0x20);
        e.set_global(GLOBAL_ROOM_MARKER_BASE, base);
        returns(&mut e, 0x009d_9f20, 0x5555);
        returns(&mut e, 0x0042_0ed0, 0x6666);
        let log = logged(&mut e, |e| {
            e.call(0x0056_fcb0, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x009d_9f20), vec![vec![cell]]);
        assert_eq!(calls_to(&log, 0x00c5_b060), vec![vec![0x5555, 0x6666]]);

        // a multibound marker without a linked reference
        let (mut e, refr, _) = unload_setup(0x1);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        let base = e.mem.u32(refr + 0x20);
        e.set_global(GLOBAL_MULTIBOUND_BASE, base);
        returns(&mut e, 0x009d_9f20, 0x5555);
        returns(&mut e, 0x0042_2120, 0x7070);
        let log = logged(&mut e, |e| {
            e.call(0x0056_fcb0, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x00c5_aec0), vec![vec![0x5555, 0x7070]]);
        assert!(calls_to(&log, 0x00c3_9ad0).is_empty());

        // ... with a linked reference that has a room
        let linked = e.mem.alloc(0x100);
        returns(&mut e, 0x0042_1e10, linked);
        returns(&mut e, 0x0042_0ed0, 0x6666);
        let log = logged(&mut e, |e| {
            e.call(0x0056_fcb0, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x00c3_9ad0), vec![vec![0x6666, 0x7070]]);
        assert_eq!(calls_to(&log, 0x0042_0ed0), vec![vec![linked + 0x44]]);
        assert!(calls_to(&log, 0x00c5_aec0).is_empty());

        // ... linked but without a room: the registry again
        returns(&mut e, 0x0042_0ed0, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0056_fcb0, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x00c5_aec0), vec![vec![0x5555, 0x7070]]);
    }

    #[test]
    fn unload_cleanup_updates_both_rooms_of_a_portal_marker_in_a_cell() {
        let (mut e, refr, _) = unload_setup(0x1);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        let base = e.mem.u32(refr + 0x20);
        e.set_global(GLOBAL_PORTAL_MARKER_BASE, base);
        returns(&mut e, 0x009d_9f20, 0x5555);
        returns(&mut e, 0x0042_0dd0, 0x9999);
        // the portal joins one room (slot 0) and nothing (slot 1)
        let rooms = e.mem.alloc(0x10);
        let room_ref = e.mem.alloc(0x100);
        e.mem.set_u32(rooms, room_ref);
        returns(&mut e, 0x0042_0410, rooms);
        returns(&mut e, 0x0056_99b0, 0x1212);
        returns(&mut e, 0x006a_b360, 0x3434);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let record = seen.clone();
        e.register_double(0x0057_c730, move |e, a| {
            record.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let log = logged(&mut e, |e| {
            e.call(0x0056_fcb0, &args![refr]);
        });
        assert_eq!(*seen.borrow(), vec![(0x3434, 0x9999)]);
        assert_eq!(calls_to(&log, 0x0097_07f0), vec![vec![0x9999, 0]]);
        // slot 1 is empty: the registry hears about the portal
        assert_eq!(calls_to(&log, 0x00c5_b190), vec![vec![0x5555, 0x9999]]);
        assert!(calls_to(&log, 0x0042_0ba0).is_empty());
    }

    // ---- 005702e0 and its tail 00570f70 -----------------------------------

    /// Everything `Set3D` reads: a reference (an actor when asked) with the
    /// 3D `old` under `parent`, a base object, the process of an actor.
    struct Scene {
        e: Engine,
        refr: u32,
        old: u32,
        parent: u32,
        base: u32,
        process: u32,
    }

    fn scene(actor: bool) -> Scene {
        let mut e = engine();
        for global in [
            GLOBAL_MODEL_LOADER,
            GLOBAL_TES,
            GLOBAL_LOADING_OBJECT,
            GLOBAL_SAVE_LOAD,
            GLOBAL_DATA_HANDLER,
            GLOBAL_PLAYER,
        ] {
            let object = e.mem.alloc(0x300);
            e.set_global(global, object);
        }
        let parent = object(&mut e, 0x100, &[(0xe8, 0), (0xdc, 0)]);
        let old = object(&mut e, 0x100, &[(0x10, 0), (0xc, 0x4242)]);
        e.mem.set_u32(old + 0x18, parent);
        let base = object(&mut e, 0x100, &[(0x150, 0)]);
        e.mem.set_u8(base + 4, 1);
        let process = object(
            &mut e,
            0x700,
            &[
                (0x79c, 0),
                (0x794, 0),
                (0x7a4, 0),
                (0x1a0, 0),
                (0x1a4, 0),
                (0x5f4, 0),
                (0x4fc, 0),
                (0x468, 0),
                (0x454, 0),
                (0x458, 0),
                (0x148, 0),
            ],
        );
        let refr = reference(
            &mut e,
            actor,
            &[
                (0x1d0, old),
                (0x15c, 1),
                (0x1f0, 0),
                (0x224, 0),
                (0xfc, 0),
                (0x4c, 0),
                (0x1c0, 0),
                (0x21c, 0),
                (0x1e8, 0),
            ],
        );
        e.mem.set_u32(refr + 0x20, base);
        if actor {
            e.mem.set_u32(refr + 0x68, process);
        }
        set_node(&mut e, refr, old);
        // the node accessors
        // 00891170: the OBJ_REFR data embedded at +0x20
        e.register(0x0089_1170, |_, a| ret(a[0] + 0x20));
        e.register(0x0043_c490, |_, a| ret(a[0] + 0x58));
        e.register(0x006a_9540, |_, a| ret(a[0] + 0x34));
        e.register(0x009c_dae0, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        e.register(0x0045_0b80, |_, _| ret(0x00d0_0d00));
        e.register(0x008c_71b0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        Scene {
            e,
            refr,
            old,
            parent,
            base,
            process,
        }
    }

    #[test]
    fn set_3d_with_the_current_node_changes_nothing() {
        let mut s = scene(false);
        let (refr, old) = (s.refr, s.old);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_02e0, &args![refr, old, 0u32]);
        });
        assert!(calls_to(&log, CANCEL_REFERENCE).is_empty());
        assert!(calls_to(&log, 0x0057_9ac0).is_empty());
        assert_eq!(log.len(), 2); // the call and the 3D accessor
    }

    #[test]
    fn set_3d_of_a_reference_without_3d_to_null_cancels_the_loader_request() {
        let mut s = scene(false);
        let refr = s.refr;
        set_node(&mut s.e, refr, 0);
        let loader = model_loader(&s.e);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_02e0, &args![refr, 0u32, 0u32]);
        });
        assert_eq!(calls_to(&log, CANCEL_REFERENCE), vec![vec![loader, refr]]);
        assert_eq!(calls_to(&log, 0x0057_9ac0), vec![vec![refr, 0]]);
    }

    #[test]
    fn set_3d_on_a_bare_reference_stores_the_node_and_registers_the_obstacle() {
        let mut s = scene(false);
        let refr = s.refr;
        set_node(&mut s.e, refr, 0);
        let loaded = s.e.mem.alloc(0x40);
        s.e.register_double(0x0057_c180, move |e, a| {
            e.mem.set_u32(a[0] + 0x64, loaded);
            Ret::default()
        });
        let node = object(&mut s.e, 0x100, &[]);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_02e0, &args![refr, node, 0u32]);
        });
        // Set3DVerySimple stores the node in the loaded data
        assert_eq!(calls_to(&log, 0x0066_b0d0), vec![vec![loaded + 0x14, node]]);
        // the obstacle is added (by 00570f70 and by the tail of Set3D)
        assert_eq!(calls_to(&log, 0x006c_0c30).len(), 2);
        assert!(calls_to(&log, 0x006c_0c80).is_empty());
        // no old node: nothing is detached or transformed
        assert!(calls_to(&log, 0x0044_0460).is_empty());
        assert!(calls_to(&log, 0x00b5_b1c0).is_empty());
        assert!(calls_to(&log, CANCEL_REFERENCE).is_empty());
    }

    #[test]
    fn set_3d_replaces_the_node_under_the_same_parent_with_its_transform() {
        let mut s = scene(false);
        let (refr, old, parent) = (s.refr, s.old, s.parent);
        s.e.mem.set_f32(old + 0x58, 1.0);
        s.e.mem.set_f32(old + 0x5c, 2.0);
        s.e.mem.set_f32(old + 0x60, 3.0);
        for i in 0..9 {
            s.e.mem.set_u32(old + 0x34 + 4 * i, 101 + i);
        }
        let node = object(&mut s.e, 0x100, &[]);
        let translation = Rc::new(RefCell::new(Vec::new()));
        let rotation = Rc::new(RefCell::new(Vec::new()));
        let (t, r) = (translation.clone(), rotation.clone());
        s.e.register_double(0x0044_0460, move |e, a| {
            for i in 0..3 {
                t.borrow_mut().push(e.mem.f32(a[1] + 4 * i));
            }
            Ret::default()
        });
        s.e.register_double(0x0043_fa80, move |e, a| {
            for i in 0..9 {
                r.borrow_mut().push(e.mem.u32(a[1] + 4 * i));
            }
            Ret::default()
        });
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_02e0, &args![refr, node, 0u32]);
        });
        // the old node leaves its parent (virtual +0xe8) and the shadow scene node
        assert_eq!(
            calls_to(&log, slot(&s.e, parent, 0xe8)),
            vec![vec![parent, old]]
        );
        assert_eq!(calls_to(&log, 0x00b5_b1c0), vec![vec![0x00d0_0d00, old]]);
        // the new node gets the remembered transform and the old parent
        assert_eq!(*translation.borrow(), vec![1.0, 2.0, 3.0]);
        assert_eq!(*rotation.borrow(), (101..110).collect::<Vec<u32>>());
        assert_eq!(
            calls_to(&log, 0x0044_0490),
            vec![vec![node, float_bits(2.0)]]
        );
        assert_eq!(
            calls_to(&log, slot(&s.e, parent, 0xdc)),
            vec![vec![parent, node, 1]]
        );
        assert_eq!(calls_to(&log, 0x00c6_bd00), vec![vec![node, 1]]);
        assert_eq!(calls_to(&log, 0x00a5_a040), vec![vec![node]]);
        assert_eq!(calls_to(&log, NODE_UPDATE)[0][0], node);
        // the old node is released, the obstacle removed then added
        assert_eq!(calls_to(&log, 0x0066_b0d0)[0][1], 0);
        assert_eq!(calls_to(&log, 0x006c_0c80).len(), 1);
        assert_eq!(calls_to(&log, 0x006c_0c30).len(), 2);
    }

    #[test]
    fn set_3d_leaves_a_3d_of_a_type_0x25_base_alone() {
        let mut s = scene(false);
        let (refr, base) = (s.refr, s.base);
        s.e.mem.set_u8(base + 4, 0x25);
        let node = object(&mut s.e, 0x100, &[]);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_02e0, &args![refr, node, 0u32]);
        });
        assert!(calls_to(&log, 0x0063_3c90).is_empty());
        assert!(calls_to(&log, 0x006c_0c30).is_empty());
    }

    #[test]
    fn set_3d_to_null_cancels_the_loader_request_of_a_parentless_3d() {
        let mut s = scene(false);
        let (refr, old) = (s.refr, s.old);
        s.e.mem.set_u32(old + 0x18, 0);
        let loader = model_loader(&s.e);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_02e0, &args![refr, 0u32, 0u32]);
        });
        assert_eq!(calls_to(&log, CANCEL_REFERENCE)[0], vec![loader, refr]);
        // no parent: nothing is detached
        assert!(calls_to(&log, slot(&s.e, s.parent, 0xe8)).is_empty());
    }

    /// Unloads the 3D of an actor reference; returns the log.
    fn unload_actor(
        configure: impl FnOnce(&mut Scene),
        log_unload: bool,
    ) -> (Scene, Vec<(u32, Vec<u32>)>) {
        let mut s = scene(true);
        configure(&mut s);
        let refr = s.refr;
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_02e0, &args![refr, 0u32, u32::from(log_unload)]);
        });
        (s, log)
    }

    #[test]
    fn set_3d_to_null_on_an_actor_drops_the_art_of_its_process() {
        let (s, log) = unload_actor(
            |s| {
                s.e.register(0x0093_1850, |_, _| ret(1));
                let loaded = s.e.mem.u32(s.refr + 0x64);
                s.e.mem.set_u32(loaded + 4, 0x1234);
                s.e.register(0x0041_3f40, |_, _| ret(0x1111));
                s.e.register(0x0043_b1b0, |_, _| ret(0x2222));
                returns(&mut s.e, GET_PRIMITIVE, 0x5000);
            },
            true,
        );
        let process = s.process;
        assert_eq!(
            calls_to(&log, slot(&s.e, process, 0x79c)),
            vec![vec![process, 0]]
        );
        assert_eq!(
            calls_to(&log, slot(&s.e, process, 0x794)),
            vec![vec![process, 0]]
        );
        assert_eq!(
            calls_to(&log, slot(&s.e, process, 0x7a4)),
            vec![vec![process, 0]]
        );
        assert_eq!(calls_to(&log, 0x0092_b7f0), vec![vec![process]]);
        assert_eq!(calls_to(&log, 0x005b_5e40), vec![vec![0x0103_0df8, 0x2222]]);
        // the loaded data forgets the 3D's owner word, the primitive is reset
        let loaded = s.e.mem.u32(s.refr + 0x64);
        assert_eq!(s.e.mem.u32(loaded + 4), 0);
        assert_eq!(calls_to(&log, 0x004a_4c50), vec![vec![0x5000]]);
        // the cleanup of everything that tracks the 3D ran
        assert_eq!(calls_to(&log, 0x0056_5670), vec![vec![s.refr]]);

        // without the log flag no message is printed, and a process type above 1 is left alone
        let (s, log) = unload_actor(
            |s| {
                s.e.register(0x0093_1850, |_, _| ret(2));
            },
            false,
        );
        assert!(calls_to(&log, 0x005b_5e40).is_empty());
        assert!(calls_to(&log, slot(&s.e, s.process, 0x79c)).is_empty());
    }

    #[test]
    fn set_3d_to_null_on_an_actor_saves_havok_data_and_animation_for_a_saved_change() {
        let (s, log) = unload_actor(
            |s| {
                s.e.register(0x0046_2480, |_, _| ret(1));
                s.e.register(0x0084_a6d0, |_, _| ret(1));
                s.e.register(0x0086_5020, |e, a| {
                    e.mem.set_u32(a[0], 0x6161);
                    ret(a[0])
                });
            },
            false,
        );
        // the Havok data goes to the extra list, the animation to the virtual +0x4c
        assert_eq!(
            calls_to(&log, 0x0042_2ac0),
            vec![vec![s.refr + 0x44, 0x6161]]
        );
        assert_eq!(
            calls_to(&log, slot(&s.e, s.refr, 0x4c)),
            vec![vec![s.refr, 0x1000_0000]]
        );
        // the two change masks were built with 008c71b0
        let masks = calls_to(&log, 0x008c_71b0);
        assert_eq!(masks.len(), 2);
        assert_eq!(masks[0][1], 4);
        assert_eq!(masks[1][1], 0x1000_0000);
        let queries = calls_to(&log, 0x0084_a6d0);
        assert_eq!(queries[0][1..], [s.refr, 4]);
        assert_eq!(queries[1][1..], [s.refr, 0x1000_0000]);
    }

    #[test]
    fn set_3d_to_null_does_not_save_when_the_form_is_deleted_or_nothing_changed() {
        let (_, log) = unload_actor(
            |s| {
                s.e.register(0x0046_2480, |_, _| ret(1));
                s.e.register(0x0084_a6d0, |_, _| ret(1));
                s.e.mem.set_u32(s.refr + 8, 0x20);
            },
            false,
        );
        assert!(calls_to(&log, 0x0084_a6d0).is_empty());
        let (_, log) = unload_actor(
            |s| {
                s.e.register(0x0046_2480, |_, _| ret(1));
                s.e.register(0x0084_a6d0, |_, _| ret(0));
            },
            false,
        );
        assert!(calls_to(&log, 0x0042_2ac0).is_empty());
        assert!(calls_to(&log, 0x0042_2940).is_empty());
        // both queries were made, and neither answered yes
        assert_eq!(calls_to(&log, 0x0084_a6d0).len(), 2);
    }

    #[test]
    fn set_3d_to_null_on_an_actor_removes_the_weapon_and_clears_its_ragdoll_data() {
        let (s, log) = unload_actor(
            |s| {
                let owned = object(&mut s.e, 0x20, &[(0, 0)]);
                s.e.mem.set_u32(s.refr + 0xac, owned);
                s.e.mem.set_u32(s.refr + 0xb0, 0x7a7a);
                returns(&mut s.e, 0x0093_06d0, 0x8b8b);
            },
            false,
        );
        let refr = s.refr;
        let owned = s.e.mem.u32(refr + 0xac);
        assert_eq!(owned, 0);
        assert_eq!(calls_to(&log, 0x0045_34f0), vec![vec![refr, 0]]);
        assert_eq!(calls_to(&log, 0x0048_3710), vec![vec![refr], vec![refr]]);
        assert_eq!(calls_to(&log, 0x00c6_d7a0), vec![vec![0x8b8b, 0]]);
        assert_eq!(calls_to(&log, 0x00ca_20e0), vec![vec![0x7a7a]]);
        assert_eq!(calls_to(&log, slot(&s.e, refr, 0x1f0)), vec![vec![refr, 0]]);
        assert_eq!(calls_to(&log, 0x0057_2a50), vec![vec![refr, 0]]);
        assert_eq!(calls_to(&log, 0x0057_2e50), vec![vec![refr, 0]]);
        // the process leaves the simulation (virtual +0x5f4), its ragdoll holder is gone
        assert_eq!(
            calls_to(&log, slot(&s.e, s.process, 0x5f4)),
            vec![vec![s.process, 0]]
        );
        // RemoveWeapon ran: it asked the process for the held weapon node
        assert!(!calls_to(&log, slot(&s.e, s.process, 0x1a0)).is_empty());
    }

    #[test]
    fn set_3d_to_null_stops_the_moving_sounds_unless_the_reference_keeps_them() {
        let (s, log) = unload_actor(
            |s| {
                returns(&mut s.e, 0x0045_3a70, 0xa0d1);
                // the virtual at +0x224 asks for the sounds to be stopped
                let refr = s.refr;
                let vtable = s.e.mem.u32(refr);
                s.e.register(vtable + 0x224, |_, _| ret(1));
            },
            false,
        );
        assert_eq!(calls_to(&log, 0x00ad_8570), vec![vec![0xa0d1, s.old, 0, 0]]);
        assert_eq!(calls_to(&log, 0x0057_a3c0), vec![vec![s.refr, 1]]);

        let (s, log) = unload_actor(|_| {}, false);
        assert!(calls_to(&log, 0x00ad_8570).is_empty());
        assert_eq!(calls_to(&log, 0x0057_a3c0), vec![vec![s.refr, 1]]);
    }

    #[test]
    fn set_3d_to_null_releases_the_old_node_through_the_garbage_collector() {
        let (s, log) = unload_actor(
            |s| {
                s.e.register(0x0045_cd60, |_, _| ret(1));
            },
            false,
        );
        // an actor that is not the player: its process stops using the art, the node is queued
        assert_eq!(
            calls_to(&log, slot(&s.e, s.process, 0x4fc)),
            vec![vec![s.process, 0]]
        );
        assert_eq!(calls_to(&log, 0x0086_8660), vec![vec![s.old]]);
        assert!(calls_to(&log, 0x00c4_5830).is_empty());
        // the base object hears about it (virtual +0x150) once the loader is done
        assert_eq!(
            calls_to(&log, slot(&s.e, s.base, 0x150)),
            vec![vec![s.base, s.refr]]
        );
        // the old node was released through the NiPointer assignment
        assert!(calls_to(&log, 0x0066_b0d0).iter().any(|a| a[1] == 0));
        // a loader that holds the reference: the base object is not told
        let (s, log) = unload_actor(
            |s| {
                s.e.register(0x0045_23e0, |_, _| ret(1));
            },
            false,
        );
        assert!(calls_to(&log, slot(&s.e, s.base, 0x150)).is_empty());
        // a reference the loader wants cancelled
        let (s, log) = unload_actor(
            |s| {
                s.e.register(0x0056_51e0, |_, _| ret(1));
            },
            false,
        );
        assert!(calls_to(&log, slot(&s.e, s.base, 0x150)).is_empty());
        assert!(!calls_to(&log, CANCEL_REFERENCE).is_empty());
    }

    #[test]
    fn set_3d_to_null_hands_the_players_node_to_its_own_release() {
        let mut s = scene(true);
        let (refr, old) = (s.refr, s.old);
        s.e.set_global(GLOBAL_PLAYER, refr);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_02e0, &args![refr, 0u32, 0u32]);
        });
        assert_eq!(calls_to(&log, 0x00c4_5830), vec![vec![old]]);
        assert!(calls_to(&log, 0x0086_8660).is_empty());
    }

    #[test]
    fn set_3d_to_null_with_a_destructible_updates_the_replacement_models() {
        let mut s = scene(false);
        let (refr, base) = (s.refr, s.base);
        // the object health is set, the reference has a 3D: the destruction form is released
        s.e.register(0x0045_2370, |_, _| ret(1));
        s.e.register(0x0041_b6b0, |_, _| Ret {
            st0: 50.0,
            ..Ret::default()
        });
        returns(&mut s.e, 0x0047_5400, 0x3030);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_02e0, &args![refr, 0u32, 0u32]);
        });
        assert_eq!(calls_to(&log, 0x0047_5400), vec![vec![base]]);
        assert_eq!(calls_to(&log, 0x0047_78a0), vec![vec![0x3030]]);

        // a node being attached to a reference without 3D preloads the replacements
        let mut s = scene(false);
        let (refr, base) = (s.refr, s.base);
        set_node(&mut s.e, refr, 0);
        s.e.register(0x0045_2370, |_, _| ret(1));
        s.e.register(0x0041_b6b0, |_, _| Ret {
            st0: 50.0,
            ..Ret::default()
        });
        returns(&mut s.e, 0x0047_5400, 0x3030);
        let node = object(&mut s.e, 0x100, &[]);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_02e0, &args![refr, node, 0u32]);
        });
        assert_eq!(calls_to(&log, 0x0047_7780), vec![vec![0x3030, base]]);

        // a zero health leaves the destruction form alone
        let mut s = scene(false);
        let refr = s.refr;
        s.e.register(0x0045_2370, |_, _| ret(1));
        s.e.register(0x0041_b6b0, |_, _| Ret::default());
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_02e0, &args![refr, 0u32, 0u32]);
        });
        assert!(calls_to(&log, 0x0047_5400).is_empty());
    }

    #[test]
    fn set_3d_tail_0x00570f70_updates_markers_and_runs_the_virtual_without_the_task_queue() {
        let mut s = scene(false);
        let (refr, base) = (s.refr, s.base);
        s.e.mem.set_u8(base + 4, 0x1c);
        returns(&mut s.e, 0x006c_0720, 0x9090);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_0f70, &args![refr, 0u32]);
        });
        assert_eq!(calls_to(&log, 0x006c_0c80), vec![vec![0x9090, refr]]);
        assert_eq!(calls_to(&log, 0x006c_1060), vec![vec![0x9090, refr]]);
        assert_eq!(calls_to(&log, slot(&s.e, refr, 0x1c0)), vec![vec![refr]]);
        assert!(calls_to(&log, 0x0087_b2b0).is_empty());
        assert_eq!(calls_to(&log, 0x0057_c300), vec![vec![refr]]);

        // the task queue replaces the virtual when 008c7aa0 holds
        s.e.register(0x008c_7aa0, |_, _| ret(1));
        returns(&mut s.e, 0x0045_37b0, 0xabab);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_0f70, &args![refr, 0u32]);
        });
        assert_eq!(calls_to(&log, 0x0087_b2b0), vec![vec![0xabab, refr]]);
        assert!(calls_to(&log, slot(&s.e, refr, 0x1c0)).is_empty());
    }

    #[test]
    fn set_3d_tail_0x00570f70_clears_the_room_or_portal_extra_of_a_marker() {
        let mut s = scene(false);
        let (refr, base) = (s.refr, s.base);
        s.e.set_global(GLOBAL_ROOM_MARKER_BASE, base);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_0f70, &args![refr, 0u32]);
        });
        assert_eq!(calls_to(&log, 0x0042_0f00), vec![vec![refr + 0x44, 0]]);
        assert!(calls_to(&log, 0x0042_0e00).is_empty());

        let mut s = scene(false);
        let (refr, base) = (s.refr, s.base);
        s.e.set_global(GLOBAL_PORTAL_MARKER_BASE, base);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_0f70, &args![refr, 0u32]);
        });
        assert_eq!(calls_to(&log, 0x0042_0e00), vec![vec![refr + 0x44, 0]]);
        assert!(calls_to(&log, 0x0042_0f00).is_empty());

        // with a node nothing is cleared and the obstacle is added
        let mut s = scene(false);
        let (refr, base) = (s.refr, s.base);
        s.e.set_global(GLOBAL_ROOM_MARKER_BASE, base);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_0f70, &args![refr, 0x4444u32]);
        });
        assert!(calls_to(&log, 0x0042_0f00).is_empty());
        assert_eq!(calls_to(&log, 0x006c_0c30).len(), 1);
    }

    // ---- 00570ea0 .. 00570f60 ---------------------------------------------

    #[test]
    fn node_flag_tests_ask_for_flag_0x20000() {
        let mut e = engine();
        returns(&mut e, 0x0045_6630, 1);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0057_0ea0, &args![0x4000u32]).bool());
            assert!(e.call(0x0057_0ee0, &args![0x4000u32]).bool());
        });
        assert_eq!(
            calls_to(&log, 0x0045_6630),
            vec![vec![0x4000, 0x2_0000], vec![0x4000, 0x2_0000]]
        );
        returns(&mut e, 0x0045_6630, 0);
        assert!(!e.call(0x0057_0ea0, &args![0x4000u32]).bool());
        assert!(!e.call(0x0057_0ee0, &args![0x4000u32]).bool());
    }

    #[test]
    fn cell_test_00570ec0_is_true_only_for_exactly_one() {
        let mut e = engine();
        for (answer, expected) in [(1u32, true), (0, false), (2, false)] {
            returns(&mut e, 0x0045_0fd0, answer);
            assert_eq!(e.call(0x0057_0ec0, &args![0x4000u32]).bool(), expected);
        }
    }

    #[test]
    fn loading_tests_00570f00_and_00570f40() {
        let mut e = engine();
        let object = e.mem.alloc(0x300);
        assert!(e.call(0x0057_0f40, &args![object]).bool());
        e.mem.set_u32(object + 0x244, 1);
        assert!(!e.call(0x0057_0f40, &args![object]).bool());
        e.mem.set_u32(object + 0x244, 2);
        assert!(e.call(0x0057_0f40, &args![object]).bool());

        returns(&mut e, 0x0046_2480, 1);
        assert!(e.call(0x0057_0f00, &args![object]).bool());
        e.mem.set_u32(object + 0x244, 3);
        assert!(!e.call(0x0057_0f00, &args![object]).bool());
        e.mem.set_u32(object + 0x244, 0);
        returns(&mut e, 0x0046_2480, 0);
        assert!(!e.call(0x0057_0f00, &args![object]).bool());
    }

    #[test]
    fn globals_00570f60_and_00571550_are_read_from_memory() {
        let mut e = engine();
        e.set_global(0x011d_fc98, 7u32);
        e.set_global(0x011c_61a4, 0x1357u32);
        assert_eq!(e.call(0x0057_0f60, &args![]).u32(), 7);
        assert_eq!(e.call(0x0057_1550, &args![]).u32(), 0x1357);
    }

    #[test]
    fn name_lookup_00571530_passes_root_and_name_with_the_flag() {
        let mut e = engine();
        returns(&mut e, 0x00c4_b470, 0x6677);
        let log = logged(&mut e, |e| {
            assert_eq!(
                e.call(0x0057_1530, &args![0x1u32, 0x2000u32, 0x3000u32])
                    .u32(),
                0x6677
            );
        });
        assert_eq!(calls_to(&log, 0x00c4_b470), vec![vec![0x2000, 0x3000, 1]]);
    }

    // ---- 005710c0 ----------------------------------------------------------

    #[test]
    fn worn_models_of_an_npc_base_replace_the_model_when_there_is_a_biped() {
        let mut e = engine();
        let refr = reference(&mut e, false, &[(0x1e8, 0x9000), (0x1e4, 0)]);
        let base = form(&mut e, 0x2a, 0);
        e.mem.set_u32(refr + 0x20, base);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0057_10c0, &args![refr]).bool());
        });
        assert_eq!(calls_to(&log, 0x0060_5d70), vec![vec![base, refr]]);
        assert!(calls_to(&log, 0x0049_6280).is_empty());

        // without a biped the base is left alone; an animation is reset first
        let refr = reference(&mut e, false, &[(0x1e8, 0), (0x1e4, 0x9100)]);
        e.mem.set_u32(refr + 0x20, base);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0057_10c0, &args![refr]).bool());
        });
        assert!(calls_to(&log, 0x0060_5d70).is_empty());
        assert_eq!(calls_to(&log, 0x0049_6280), vec![vec![0x9100]]);
    }

    #[test]
    fn worn_models_of_another_base_come_from_its_virtual() {
        let mut e = engine();
        let base = object(&mut e, 0x100, &[(0x160, 1)]);
        e.mem.set_u8(base + 4, 0x28);
        let refr = reference(&mut e, false, &[(0x1e8, 0), (0x1e4, 0)]);
        e.mem.set_u32(refr + 0x20, base);
        assert!(e.call(0x0057_10c0, &args![refr]).bool());
        let base = object(&mut e, 0x100, &[(0x160, 0)]);
        e.mem.set_u8(base + 4, 0x28);
        e.mem.set_u32(refr + 0x20, base);
        assert!(!e.call(0x0057_10c0, &args![refr]).bool());
    }

    /// A creature reference with a 3D and one package model to attach.
    /// Returns the engine, the reference, the node, the clone and the file
    /// name the model loader was asked for.
    fn creature_setup() -> (Engine, u32, u32, u32, Rc<RefCell<Vec<u8>>>) {
        let mut e = engine();
        let loader = e.mem.alloc(0x20);
        e.set_global(GLOBAL_MODEL_LOADER, loader);
        let tes = e.mem.alloc(0x20);
        e.set_global(GLOBAL_TES, tes);
        let refr = reference(&mut e, false, &[(0x1e8, 0), (0x1e4, 0)]);
        let base = form(&mut e, 0x2b, 0);
        e.mem.set_u32(refr + 0x20, base);
        let node = object(&mut e, 0x100, &[]);
        set_node(&mut e, refr, node);
        // the model path of the reference
        let path = e.mem.alloc(0x200);
        e.mem.set_cstr(path, b"meshes\\creatures\\body.nif");
        let model = object(&mut e, 0x40, &[(0x14, path)]);
        returns(&mut e, 0x0042_e250, model);
        // the package list: one entry naming another file
        let file = e.mem.alloc(0x40);
        e.mem.set_cstr(file, b"skeleton.nif");
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry, file);
        returns(&mut e, 0x0071_7e50, entry);
        // string helpers: copy, last backslash
        e.register(0x0040_6d30, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            ret(a[0])
        });
        e.register(0x0040_ab30, |e, a| {
            let text = e.mem.cstr(a[0]);
            match text.iter().rposition(|c| *c == a[1] as u8) {
                Some(index) => ret(a[0] + index as u32),
                None => ret(0),
            }
        });
        let loaded_file = Rc::new(RefCell::new(Vec::new()));
        let record = loaded_file.clone();
        let loaded_model = e.mem.alloc(0x40);
        e.register_double(0x0044_7080, move |e, a| {
            *record.borrow_mut() = e.mem.cstr(a[1]);
            ret(loaded_model)
        });
        e.register(GET_SCALE, |_, _| Ret {
            st0: 1.5,
            ..Ret::default()
        });
        let clone = e.mem.alloc(0x40);
        e.register_double(0x00a5_d2c0, move |_, _| ret(clone));
        (e, refr, node, clone, loaded_file)
    }

    #[test]
    fn creature_package_models_are_cloned_and_attached_to_the_3d() {
        let (mut e, refr, node, clone, loaded_file) = creature_setup();
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0057_10c0, &args![refr]).bool());
        });
        // the file name of the model is replaced by the package's
        assert_eq!(
            loaded_file.borrow().as_slice(),
            b"meshes\\creatures\\skeleton.nif"
        );
        let loads = calls_to(&log, 0x0044_7080);
        assert_eq!(loads[0][2..], [3, 1, 0, 0, 0]);
        // the clone's scale is the reference's scale
        assert_eq!(calls_to(&log, 0x004a_d050)[0][1], float_bits(1.5));
        // not skinned: attached to the node as a plain child
        assert_eq!(
            calls_to(&log, 0x004a_e250),
            vec![vec![node, clone, 0, 0, 0xffff_ffff, 0]]
        );
        assert!(calls_to(&log, 0x004a_de40).is_empty());
        // the NiPointer and the cloning process are destroyed
        assert_eq!(calls_to(&log, 0x0045_cec0).len(), 1);
        assert_eq!(calls_to(&log, 0x004a_d270).len(), 1);
        // the package list was walked from the base object
        let base = e.mem.u32(refr + 0x20);
        assert_eq!(calls_to(&log, 0x0071_7e50), vec![vec![base + 0x114]]);
        assert_eq!(calls_to(&log, 0x005f_a0a0), vec![vec![base, refr]]);
    }

    #[test]
    fn creature_models_with_a_morpher_use_the_deep_copy() {
        let (mut e, refr, node, _, _) = creature_setup();
        returns(&mut e, 0x004b_5bf0, 1);
        let copy = e.mem.alloc(0x40);
        returns(&mut e, 0x0045_7ba0, copy);
        let log = logged(&mut e, |e| {
            e.call(0x0057_10c0, &args![refr]);
        });
        let tes = e.global::<u32>(GLOBAL_TES);
        let copies = calls_to(&log, 0x0045_7ba0);
        assert_eq!(copies.len(), 1);
        assert_eq!(copies[0][0], tes);
        assert!(calls_to(&log, 0x00a5_d2c0).is_empty());
        assert_eq!(calls_to(&log, 0x004a_e250)[0][..2], [node, copy]);
    }

    #[test]
    fn skinned_creature_models_hang_under_the_skin_attachment_child() {
        let (mut e, refr, node, clone, _) = creature_setup();
        returns(&mut e, 0x0049_10d0, 1);
        e.set_global(0x011c_61a4, 0x1234u32);
        let child = object(&mut e, 0x40, &[(0xdc, 0)]);
        let found = object(&mut e, 0x40, &[(0x9c, child)]);
        returns(&mut e, 0x00c4_b470, found);
        returns(&mut e, 0x0043_8170, 0x4040);
        let log = logged(&mut e, |e| {
            e.call(0x0057_10c0, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x00c4_b470), vec![vec![node, 0x1234, 1]]);
        assert_eq!(
            calls_to(&log, slot(&e, found, 0x9c)),
            vec![vec![found, 0x4040]]
        );
        assert_eq!(
            calls_to(&log, slot(&e, child, 0xdc)),
            vec![vec![child, clone, 1]]
        );
        assert_eq!(calls_to(&log, 0x004a_de40), vec![vec![node, clone, 0, 1]]);
        assert!(calls_to(&log, 0x004a_e250).is_empty());
        // the fixed string "SkinAttachment" (01 01f724) is built and destroyed
        assert_eq!(calls_to(&log, 0x0043_8170)[0][1], 0x0101_f724);
        assert_eq!(calls_to(&log, 0x0043_81b0).len(), 1);

        // without that child the found node's parent takes the clone
        let (mut e, refr, node, clone, _) = creature_setup();
        returns(&mut e, 0x0049_10d0, 1);
        let parent = object(&mut e, 0x40, &[(0xdc, 0)]);
        let found = object(&mut e, 0x40, &[(0x9c, 0)]);
        e.mem.set_u32(found + 0x18, parent);
        returns(&mut e, 0x00c4_b470, found);
        let log = logged(&mut e, |e| {
            e.call(0x0057_10c0, &args![refr]);
        });
        assert_eq!(
            calls_to(&log, slot(&e, parent, 0xdc)),
            vec![vec![parent, clone, 1]]
        );
        assert_eq!(calls_to(&log, 0x004a_de40), vec![vec![node, clone, 0, 1]]);
    }

    #[test]
    fn creature_models_are_skipped_without_a_3d_or_for_a_running_package() {
        let (mut e, refr, ..) = creature_setup();
        set_node(&mut e, refr, 0);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0057_10c0, &args![refr]).bool());
        });
        assert!(calls_to(&log, 0x0071_7e50).is_empty());

        let (mut e, refr, ..) = creature_setup();
        returns(&mut e, 0x0082_56d0, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0057_10c0, &args![refr]);
        });
        assert!(calls_to(&log, 0x0044_7080).is_empty());
    }

    // ---- 00571560 .. 00571630 ---------------------------------------------

    #[test]
    fn clear_interpolators_resets_the_animation_in_order() {
        let mut e = engine();
        let refr = reference(&mut e, false, &[(0x1e4, 0xa11a)]);
        returns(&mut e, 0x0049_6940, 0xc7c7);
        returns(&mut e, 0x0055_85e0, 0xd8d8);
        let log = logged(&mut e, |e| {
            e.call(0x0057_1560, &args![refr]);
        });
        let order: Vec<(u32, Vec<u32>)> = log.into_iter().skip(2).collect();
        assert_eq!(
            order,
            vec![
                (0x0049_8910, vec![0xa11a, 1, 0]),
                (0x0049_6080, vec![0xa11a, 0x14, 0]),
                (0x0049_6940, vec![0xa11a]),
                (0x0048_fef0, vec![0xc7c7, 0]),
                (0x0055_85e0, vec![0xa11a]),
                (0x0049_9160, vec![0xd8d8]),
                (0x0049_9080, vec![0xa11a]),
            ]
        );

        // no animation: nothing
        let refr = reference(&mut e, false, &[(0x1e4, 0)]);
        let log = logged(&mut e, |e| {
            e.call(0x0057_1560, &args![refr]);
        });
        assert_eq!(log.len(), 2);
    }

    fn model_with_path(e: &mut Engine, path: u32) -> u32 {
        object(e, 0x40, &[(0x14, path)])
    }

    #[test]
    fn model_of_a_reference_prefers_the_model_swap() {
        let mut e = engine();
        let refr = e.mem.alloc(0x100);
        let swap = e.mem.alloc(0x40);
        returns(&mut e, 0x0042_e250, swap);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_1630, &args![refr]).u32(), swap);
        });
        assert_eq!(calls_to(&log, 0x0042_e250), vec![vec![refr + 0x44]]);
        assert!(calls_to(&log, 0x00ec_43fb).is_empty());
    }

    #[test]
    fn model_of_a_weapon_depends_on_its_mod_flags() {
        let mut e = engine();
        let refr = e.mem.alloc(0x100);
        let weapon = form(&mut e, 0x28, 0);
        e.mem.set_u32(refr + 0x20, weapon);
        returns(&mut e, 0x0052_2d80, 0x1010);
        returns(&mut e, 0x0052_2df0, 0x2020);
        // no mod flags: the plain model
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_1630, &args![refr]).u32(), 0x1010);
        });
        assert_eq!(calls_to(&log, 0x0052_2d80), vec![vec![weapon]]);
        // mod flags: the modded model for those flags (second word 0)
        returns(&mut e, 0x0042_e560, 5);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_1630, &args![refr]).u32(), 0x2020);
        });
        assert_eq!(calls_to(&log, 0x0052_2df0), vec![vec![weapon, 5, 0]]);
    }

    #[test]
    fn model_of_another_base_is_its_bound_object_cast() {
        let mut e = engine();
        let refr = e.mem.alloc(0x100);
        let base = form(&mut e, 0x1, 0);
        e.mem.set_u32(refr + 0x20, base);
        returns(&mut e, 0x00ec_43fb, 0x3030);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_1630, &args![refr]).u32(), 0x3030);
        });
        assert_eq!(
            calls_to(&log, 0x00ec_43fb),
            vec![vec![base, 0, 0x0118_3108, 0x0118_31e8, 0]]
        );
    }

    #[test]
    fn model_falls_back_to_the_world_model_of_a_biped_form_for_the_owners_sex() {
        let mut e = engine();
        let refr = e.mem.alloc(0x100);
        let base = form(&mut e, 0x1, 0);
        e.mem.set_u32(refr + 0x20, base);
        returns(&mut e, 0x0048_0db0, 0x5050);
        returns(&mut e, 0x0048_1110, 0x6060);
        // no owner: sex 0
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_1630, &args![refr]).u32(), 0x6060);
        });
        assert_eq!(calls_to(&log, 0x0048_0db0), vec![vec![base]]);
        assert_eq!(calls_to(&log, 0x0048_1110), vec![vec![0x5050, 0]]);
        // an NPC owner decides
        let owner = form(&mut e, 0x2a, 0);
        returns(&mut e, 0x0056_7790, owner);
        returns(&mut e, 0x005f_0cc0, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0057_1630, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x0048_1110), vec![vec![0x5050, 1]]);
        // an owner that is not an NPC does not
        let faction = form(&mut e, 0x8, 0);
        returns(&mut e, 0x0056_7790, faction);
        let log = logged(&mut e, |e| {
            e.call(0x0057_1630, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x0048_1110), vec![vec![0x5050, 0]]);
        // no biped form: no model
        returns(&mut e, 0x0048_0db0, 0);
        assert_eq!(e.call(0x0057_1630, &args![refr]).u32(), 0);
    }

    #[test]
    fn model_path_is_the_virtual_of_the_model_or_zero() {
        let mut e = engine();
        let refr = e.mem.alloc(0x100);
        let base = form(&mut e, 0x1, 0);
        e.mem.set_u32(refr + 0x20, base);
        returns(&mut e, 0x0048_0db0, 0);
        assert_eq!(e.call(0x0057_15d0, &args![refr]).u32(), 0);
        let model = model_with_path(&mut e, 0x7777);
        returns(&mut e, 0x0042_e250, model);
        assert_eq!(e.call(0x0057_15d0, &args![refr]).u32(), 0x7777);
    }

    #[test]
    fn model_bound_size_comes_from_the_base_object() {
        let mut e = engine();
        let refr = e.mem.alloc(0x100);
        assert_eq!(e.call(0x0057_1600, &args![refr]).f32(), 0.0);
        let base = form(&mut e, 0x1, 0);
        e.mem.set_u32(refr + 0x20, base);
        e.register(0x0050_ebf0, |_, _| Ret {
            st0: 3.5,
            ..Ret::default()
        });
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_1600, &args![refr]).f32(), 3.5);
        });
        assert_eq!(calls_to(&log, 0x0050_ebf0), vec![vec![base]]);
    }

    // ---- 00571760 ----------------------------------------------------------

    struct Equip {
        e: Engine,
        actor: u32,
        process: u32,
        item: u32,
        animation: u32,
        node: u32,
    }

    fn equip_setup() -> Equip {
        let mut e = engine();
        for global in [GLOBAL_MODEL_LOADER, GLOBAL_LOADING_OBJECT, GLOBAL_PLAYER] {
            let object = e.mem.alloc(0x300);
            e.set_global(global, object);
        }
        let animation = object(&mut e, 0x100, &[]);
        let process = object(
            &mut e,
            0x700,
            &[(0x454, 1), (0x458, 0), (0x1cc, 0), (0x468, 0), (0x148, 0)],
        );
        let actor = reference(&mut e, true, &[(0x1e8, 0), (0x1e4, animation), (0x22c, 0)]);
        e.mem.set_u32(actor + 0x68, process);
        let node = object(&mut e, 0x100, &[]);
        set_node(&mut e, actor, node);
        let item = e.mem.alloc(0x100);
        Equip {
            e,
            actor,
            process,
            item,
            animation,
            node,
        }
    }

    #[test]
    fn equip_does_nothing_without_a_3d() {
        let mut s = equip_setup();
        let (actor, item) = (s.actor, s.item);
        set_node(&mut s.e, actor, 0);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_1760, &args![actor, item]);
        });
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn equip_prints_the_debug_message_for_names_containing_lily() {
        let mut s = equip_setup();
        let (actor, item) = (s.actor, s.item);
        returns(&mut s.e, 0x0055_d520, 0x4a4a);
        returns(&mut s.e, 0x00ec_7750, 0x4a50);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_1760, &args![actor, item]);
        });
        assert_eq!(calls_to(&log, 0x00ec_7750), vec![vec![0x4a4a, 0x0102_fdc8]]);
        assert_eq!(calls_to(&log, 0x005b_5e40), vec![vec![0x0103_0e38]]);
        returns(&mut s.e, 0x00ec_7750, 0);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_1760, &args![actor, item]);
        });
        assert!(calls_to(&log, 0x005b_5e40).is_empty());
    }

    #[test]
    fn equip_on_a_non_player_actor_loads_the_animation_and_attaches_the_item() {
        let mut s = equip_setup();
        let (actor, item, process, animation, node) =
            (s.actor, s.item, s.process, s.animation, s.node);
        let loader = model_loader(&s.e);
        s.e.mem.set_u32(0x0118_a838 + 2 * 4, 0x66);
        returns(&mut s.e, 0x0044_6390, 2);
        returns(&mut s.e, 0x0049_6940, 1);
        returns(&mut s.e, 0x005f_2370, 0x1_1234);
        let path = s.e.mem.alloc(0x40);
        let model = object(&mut s.e, 0x40, &[(0x14, path)]);
        returns(&mut s.e, 0x0042_e250, model);
        returns(&mut s.e, 0x0044_7330, 0x7a7a);
        returns(&mut s.e, 0x0046_e8c0, 1);
        let list = s.e.mem.alloc(0x10);
        s.e.mem.set_u32(list + 4, 1);
        returns(&mut s.e, 0x0047_5020, list);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_1760, &args![actor, item]);
        });
        // a non-player process gets virtual +0x458 (actor, 0)
        assert_eq!(
            calls_to(&log, slot(&s.e, process, 0x458)),
            vec![vec![process, actor, 0]]
        );
        // the weapon animation group is looked up and its animations loaded
        assert_eq!(calls_to(&log, 0x005f_2370), vec![vec![0, 0x66, 0x18, 0]]);
        assert_eq!(calls_to(&log, 0x0049_4710), vec![vec![animation, 0x1234]]);
        assert_eq!(
            calls_to(&log, slot(&s.e, actor, 0x22c)),
            vec![vec![actor, 1]]
        );
        assert_eq!(
            calls_to(&log, 0x0044_7330),
            vec![vec![loader, path, 1, 0, 0x66]]
        );
        assert_eq!(calls_to(&log, 0x0049_0400), vec![vec![animation, 0x7a7a]]);
        // the item is attached with the drawn state decided by the item
        assert_eq!(
            calls_to(&log, 0x004a_eed0),
            vec![vec![item, item + 0x3c, 5, actor, 0]]
        );
        assert_eq!(
            calls_to(&log, slot(&s.e, process, 0x1cc)),
            vec![vec![process, 1, 0, animation, actor]]
        );
        // the actor refreshes itself
        assert_eq!(calls_to(&log, 0x008b_0b00), vec![vec![actor, 1]]);
        assert_eq!(calls_to(&log, 0x008b_0bd0), vec![vec![actor, node]]);
        assert_eq!(calls_to(&log, 0x008c_4640), vec![vec![actor]]);
        assert_eq!(calls_to(&log, 0x0089_1190), vec![vec![actor, 1]]);
        assert_eq!(
            calls_to(&log, slot(&s.e, process, 0x468)),
            vec![vec![process, 1]]
        );
        // the player hooks are not touched
        assert!(calls_to(&log, 0x004a_b750).is_empty());
    }

    #[test]
    fn equip_skips_the_animation_load_when_the_group_is_already_loaded() {
        let mut s = equip_setup();
        let (actor, item) = (s.actor, s.item);
        returns(&mut s.e, 0x0049_6940, 1);
        returns(&mut s.e, 0x0049_4710, 1);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_1760, &args![actor, item]);
        });
        assert!(calls_to(&log, 0x0044_7330).is_empty());
        // the drawn state is the actor's own when the item does not force it
        returns(&mut s.e, 0x008a_16d0, 1);
        let process = s.process;
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_1760, &args![actor, item]);
        });
        assert_eq!(calls_to(&log, slot(&s.e, process, 0x1cc))[0][1], 1);
    }

    #[test]
    fn equip_on_the_player_uses_the_first_person_biped_and_the_mod_effect() {
        let mut s = equip_setup();
        let (actor, item, process) = (s.actor, s.item, s.process);
        s.e.set_global(GLOBAL_PLAYER, actor);
        let vtable = s.e.mem.u32(process);
        s.e.register(vtable + 0x148, |_, _| ret(0x6600));
        s.e.register_double(0x0095_0b00, |_, a| {
            ret(if a[1] == 1 { 0xb1b1 } else { 0xb2b2 })
        });
        returns(&mut s.e, 0x004b_d820, 3);
        returns(&mut s.e, 0x004b_d8d0, 1);
        returns(&mut s.e, 0x004a_d010, 1);
        returns(&mut s.e, 0x0050_4e60, 0x4e4e);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_1760, &args![actor, item]);
        });
        // the mod slots and the mod effect 0xe of the worn item
        assert_eq!(calls_to(&log, 0x004b_d820), vec![vec![0x6600]]);
        let effects = calls_to(&log, 0x004b_d8d0);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0][..2], [0x6600, 0xe]);
        // both bipeds get the item with the slots
        assert_eq!(
            calls_to(&log, 0x004a_b750),
            vec![vec![0xb1b1, item, 3], vec![0xb2b2, item, 3]]
        );
        // the user-interface hook runs for the weapon when the mod effect is active
        assert_eq!(calls_to(&log, 0x0070_9c20), vec![vec![0x4e4e]]);
        // no animation loading and no attachment: the biped exists
        assert!(calls_to(&log, 0x0044_7330).is_empty());
        assert!(calls_to(&log, 0x004a_eed0).is_empty());

        // without the mod effect, a weapon that is not exempt gets no hook
        returns(&mut s.e, 0x004b_d8d0, 0);
        returns(&mut s.e, 0x004a_d030, 1);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_1760, &args![actor, item]);
        });
        assert!(calls_to(&log, 0x0070_9c20).is_empty());
    }

    // ---- 00571b50 ----------------------------------------------------------

    /// An actor with a process whose held-weapon virtual returns `held`,
    /// whose named list is `list`, and a worn weapon form with a model.
    struct Weapon {
        e: Engine,
        actor: u32,
        process: u32,
        node: u32,
        path: u32,
    }

    fn weapon_setup(held: u32, list: u32) -> Weapon {
        let mut e = engine();
        for global in [
            GLOBAL_MODEL_LOADER,
            GLOBAL_PLAYER,
            GLOBAL_DATA_HANDLER,
            GLOBAL_SAVE_LOAD,
        ] {
            let object = e.mem.alloc(0x300);
            e.set_global(global, object);
        }
        let process = object(&mut e, 0x700, &[(0x1a0, held), (0x1a4, list), (0x468, 0)]);
        let node = object(&mut e, 0x100, &[]);
        let actor = reference(&mut e, true, &[(0x1e8, 0), (0x1d0, node), (0x21c, 0)]);
        e.mem.set_u32(actor + 0x68, process);
        set_node(&mut e, actor, node);
        // the worn weapon form, with a TESModel at +0x3c
        let worn = e.mem.alloc(0x100);
        let path = e.mem.alloc(0x40);
        let model_vtable = e.mem.alloc(0x40);
        e.register_double(model_vtable + 0x14, move |_, _| ret(path));
        e.mem.set_u32(model_vtable + 0x14, model_vtable + 0x14);
        e.mem.set_u32(worn + 0x3c, model_vtable);
        let changes = e.mem.alloc(0x40);
        returns(&mut e, 0x0041_8520, changes);
        let entry = e.mem.alloc(0x40);
        returns(&mut e, 0x004c_8c10, entry);
        returns(&mut e, 0x0044_ddc0, worn);
        returns(&mut e, 0x0045_0b80, 0x5c5c);
        Weapon {
            e,
            actor,
            process,
            node,
            path,
        }
    }

    #[test]
    fn remove_weapon_does_nothing_without_a_3d() {
        let mut s = weapon_setup(0, 0);
        let actor = s.actor;
        set_node(&mut s.e, actor, 0);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_1b50, &args![actor]);
        });
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn remove_weapon_detaches_the_held_weapon_node_and_tells_the_loader_its_model() {
        let mut s = weapon_setup(0, 0);
        let (actor, node, path) = (s.actor, s.node, s.path);
        let held = s.e.mem.alloc(0x100);
        let vtable = s.e.mem.u32(s.process);
        s.e.register_double(vtable + 0x1a0, move |_, _| ret(held));
        returns(&mut s.e, 0x0045_3470, 1);
        let backpack = s.e.mem.alloc(0x40);
        let parent = object(&mut s.e, 0x40, &[(0xe8, 0)]);
        s.e.mem.set_u32(backpack + 0x18, parent);
        s.e.register_double(0x004a_ae30, move |_, a| {
            ret(if a[1] == 0x0101_f5cc { backpack } else { 0 })
        });
        let loader = model_loader(&s.e);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_1b50, &args![actor]);
        });
        // the worn weapon is looked up in slot 5 and marked
        assert_eq!(calls_to(&log, 0x004c_8c10)[0][1..], [5, 0]);
        // the backpack leaves its parent
        assert_eq!(
            calls_to(&log, slot(&s.e, parent, 0xe8)),
            vec![vec![parent, backpack]]
        );
        // the loader hears the model of the worn weapon
        assert_eq!(calls_to(&log, 0x0045_a5e0), vec![vec![loader, path]]);
        // the held node leaves the shadow scene node and its effects are cleared
        assert_eq!(calls_to(&log, 0x00b5_b1c0), vec![vec![0x5c5c, held]]);
        assert_eq!(calls_to(&log, 0x004d_ffa0), vec![vec![held + 0x9c]]);
        // the actor refreshes its lighting
        assert_eq!(calls_to(&log, 0x008b_0bd0), vec![vec![actor, node]]);
    }

    #[test]
    fn remove_weapon_finds_the_weapon_child_of_the_3d_when_the_process_has_none() {
        let mut s = weapon_setup(0, 0);
        let (actor, node, path) = (s.actor, s.node, s.path);
        let child = s.e.mem.alloc(0x100);
        s.e.register_double(0x004a_ae30, move |_, a| {
            ret(if a[0] == node && a[1] == 0x0101_3be8 {
                child
            } else {
                0
            })
        });
        let loader = model_loader(&s.e);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_1b50, &args![actor]);
        });
        assert_eq!(calls_to(&log, 0x0045_a5e0), vec![vec![loader, path]]);
        // the child is only removed from the scene when the actor's virtual asks
        assert!(calls_to(&log, 0x00b5_b1c0).is_empty());

        let vtable = s.e.mem.u32(actor);
        s.e.register(vtable + 0x21c, |_, _| ret(1));
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_1b50, &args![actor]);
        });
        assert_eq!(calls_to(&log, 0x00b5_b1c0), vec![vec![0x5c5c, child]]);
        assert_eq!(calls_to(&log, 0x004d_ffa0), vec![vec![child + 0x9c]]);
    }

    #[test]
    fn remove_weapon_recognises_the_named_entry_of_the_process_list() {
        let mut s = weapon_setup(0, 0);
        let (actor, process, path) = (s.actor, s.process, s.path);
        let list = s.e.mem.alloc(0x100);
        let vtable = s.e.mem.u32(process);
        s.e.register_double(vtable + 0x1a4, move |_, _| ret(list));
        returns(&mut s.e, 0x0043_b480, 2);
        let first = s.e.mem.alloc(0x10);
        let second = s.e.mem.alloc(0x10);
        s.e.register_double(0x0043_b4a0, move |_, a| {
            ret(if a[1] == 0 { first } else { second })
        });
        // only the second entry has the wanted name
        s.e.register_double(0x0041_3f40, |_, a| ret(a[0]));
        s.e.register_double(0x0043_b1b0, |_, a| ret(a[0] + 1));
        s.e.register_double(0x0040_8b20, move |_, a| {
            ret(if a[0] == second + 1 { 0 } else { 1 })
        });
        returns(&mut s.e, 0x0045_3470, 1);
        let loader = model_loader(&s.e);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_1b50, &args![actor]);
        });
        let compared = calls_to(&log, 0x0040_8b20);
        assert_eq!(compared[0], vec![first + 1, 0x0103_0e44]);
        assert_eq!(compared[1], vec![second + 1, 0x0103_0e44]);
        assert_eq!(calls_to(&log, 0x0045_a5e0), vec![vec![loader, path]]);
        // the list is cleared at the end
        assert_eq!(calls_to(&log, 0x00b5_b1c0), vec![vec![0x5c5c, list]]);

        // no entry matches: the list is left alone
        returns(&mut s.e, 0x0040_8b20, 1);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_1b50, &args![actor]);
        });
        assert!(calls_to(&log, 0x00b5_b1c0).is_empty());
        assert!(calls_to(&log, 0x0045_a5e0).is_empty());
    }

    #[test]
    fn remove_weapon_of_the_player_leaves_iron_sights_and_vats() {
        let mut s = weapon_setup(0, 0);
        let actor = s.actor;
        s.e.set_global(GLOBAL_PLAYER, actor);
        // iron sights: the player flag bytes
        s.e.mem.set_u8(actor + 0x64f, 1);
        returns(&mut s.e, 0x008b_bc10, 1);
        s.e.register_double(0x0095_0b00, |_, a| {
            ret(if a[1] == 1 { 0xb1b1 } else { 0xb2b2 })
        });
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_1b50, &args![actor]);
        });
        assert_eq!(calls_to(&log, 0x008b_b650), vec![vec![actor, 0, 0, 0]]);
        assert_eq!(calls_to(&log, 0x009c_8950), vec![vec![0x011f_2250, 0, 1]]);
        // the first-person and the third-person bipeds both drop the weapon
        assert_eq!(
            calls_to(&log, 0x004a_b5b0),
            vec![vec![0xb1b1], vec![0xb2b2]]
        );
        assert_eq!(calls_to(&log, 0x0070_9c40), vec![vec![0]]);
        assert_eq!(calls_to(&log, 0x0070_9ca0).len(), 1);
        // the flag byte that the biped removal needs is back to 0
        assert_eq!(s.e.global::<u8>(0x011d_cfa7), 0);

        // not in iron sights: no iron-sights exit
        s.e.mem.set_u8(actor + 0x64f, 0);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_1b50, &args![actor]);
        });
        assert!(calls_to(&log, 0x008b_b650).is_empty());
    }

    // ---- 00572160, 005721e0, 00572220, 00572230 ----------------------------

    #[test]
    fn effects_of_a_node_are_released_one_by_one_and_the_array_emptied() {
        let mut e = engine();
        returns(&mut e, 0x0065_8930, 2);
        let effect = e.mem.alloc(0x10);
        let first = e.mem.alloc(0x10);
        e.mem.set_u32(first, effect);
        let second = e.mem.alloc(0x10);
        e.register_double(0x0087_7a30, move |_, a| {
            ret(if a[1] == 0 { first } else { second })
        });
        let node = e.mem.alloc(0x100);
        let log = logged(&mut e, |e| {
            e.call(0x0057_2160, &args![node]);
        });
        assert_eq!(calls_to(&log, 0x0065_8930), vec![vec![node + 0x9c]]);
        assert_eq!(
            calls_to(&log, 0x0087_7a30),
            vec![vec![node + 0x9c, 0], vec![node + 0x9c, 1]]
        );
        // only the element with a pointer is released
        assert_eq!(calls_to(&log, 0x0050_f5a0), vec![vec![effect]]);
        assert_eq!(log.last().unwrap(), &(0x004d_ffa0, vec![node + 0x9c]));
    }

    #[test]
    fn iron_sights_test_reads_the_two_flag_bytes_of_the_player() {
        let mut e = engine();
        let player = e.mem.alloc(0x700);
        assert!(!e.call(0x0057_21e0, &args![player]).bool());
        e.mem.set_u8(player + 0x64d, 1);
        assert!(e.call(0x0057_21e0, &args![player]).bool());
        e.mem.set_u8(player + 0x64d, 0);
        e.mem.set_u8(player + 0x64f, 5);
        assert!(e.call(0x0057_21e0, &args![player]).bool());
    }

    #[test]
    fn flag_setter_00572220_stores_a_byte_in_the_global() {
        let mut e = engine();
        e.call(0x0057_2220, &args![1u8]);
        assert_eq!(e.global::<u8>(0x011d_cfa7), 1);
        e.call(0x0057_2220, &args![0u8]);
        assert_eq!(e.global::<u8>(0x011d_cfa7), 0);
    }

    #[test]
    fn picking_up_removes_the_dropped_item_from_its_dropper_and_marks_it_deleted() {
        let mut e = engine();
        let refr = reference(&mut e, false, &[(0xc4, 0), (0xc8, 0), (0x1cc, 0)]);
        let dropper = e.mem.alloc(0x100);
        returns(&mut e, 0x0041_de00, dropper);
        let log = logged(&mut e, |e| {
            e.call(0x0057_2230, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x0041_de00), vec![vec![refr + 0x44]]);
        assert_eq!(
            calls_to(&log, 0x0041_e0d0),
            vec![vec![dropper + 0x44, refr]]
        );
        // then the three virtual calls of MarkAsDeleted
        assert_eq!(log[log.len() - 3].0, slot(&e, refr, 0xc4));
        assert_eq!(
            log.last().unwrap(),
            &(slot(&e, refr, 0x1cc), vec![refr, 0, 0])
        );

        // nobody dropped it: only the deletion mark
        returns(&mut e, 0x0041_de00, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0057_2230, &args![refr]);
        });
        assert!(calls_to(&log, 0x0041_e0d0).is_empty());
        assert_eq!(calls_to(&log, slot(&e, refr, 0xc4)), vec![vec![refr, 1]]);
    }

    // ---- 00572270 .. 005723b0 -----------------------------------------------

    fn float_ret(value: f32) -> Ret {
        Ret {
            st0: f64::from(value),
            ..Ret::default()
        }
    }

    /// Makes the virtual at byte offset `offset` of `object` return `value`.
    fn set_slot(e: &mut Engine, object: u32, offset: u32, value: u32) {
        let vtable = e.mem.u32(object);
        e.register_double(vtable + offset, move |_, _| ret(value));
        e.mem.set_u32(vtable + offset, vtable + offset);
    }

    #[test]
    fn mark_as_deleted_makes_the_three_virtual_calls_in_order() {
        let mut e = engine();
        let refr = reference(&mut e, false, &[(0xc4, 0), (0xc8, 0), (0x1cc, 0)]);
        let log = logged(&mut e, |e| {
            e.call(0x0057_2270, &args![refr]);
        });
        assert_eq!(
            addresses(&log),
            vec![
                0x0057_2270,
                slot(&e, refr, 0xc4),
                slot(&e, refr, 0xc8),
                slot(&e, refr, 0x1cc)
            ]
        );
        assert_eq!(log[1].1, vec![refr, 1]);
        assert_eq!(log[2].1, vec![refr, 1]);
        assert_eq!(log[3].1, vec![refr, 0, 0]);
    }

    /// A reference (an actor or not) whose form id is `id`, with a base form.
    fn dead_setup(actor: bool, id: u32) -> (Engine, u32) {
        let mut e = engine();
        let refr = reference(&mut e, actor, &[]);
        e.register_double(FORM_ID, move |_, _| ret(id));
        let base = form(&mut e, 0x2a, 0);
        e.mem.set_u32(refr + 0x20, base);
        e.set_global(0x0101_2060, 0.0f64);
        (e, refr)
    }

    #[test]
    fn an_actor_is_dead_when_its_health_is_not_above_the_threshold() {
        let (mut e, refr) = dead_setup(true, 0x14);
        let base = e.mem.u32(refr + 0x20);
        returns(&mut e, 0x0048_73d0, 0);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0057_22c0, &args![refr, 0u32]).bool());
        });
        // the health is read from the base form
        assert_eq!(calls_to(&log, 0x0048_73d0), vec![vec![base]]);
        returns(&mut e, 0x0048_73d0, 3);
        assert!(!e.call(0x0057_22c0, &args![refr, 0u32]).bool());
        // equal to the threshold counts as dead
        e.set_global(0x0101_2060, 3.0f64);
        assert!(e.call(0x0057_22c0, &args![refr, 0u32]).bool());
        returns(&mut e, 0x0048_73d0, 4);
        assert!(!e.call(0x0057_22c0, &args![refr, 0u32]).bool());
    }

    #[test]
    fn a_non_actor_or_form_id_7_asks_00477ba0_instead() {
        for (actor, id) in [(false, 0x14), (true, 7)] {
            let (mut e, refr) = dead_setup(actor, id);
            returns(&mut e, 0x0047_7ba0, 1);
            let log = logged(&mut e, |e| {
                assert!(e.call(0x0057_22c0, &args![refr, 0u32]).bool());
            });
            assert!(calls_to(&log, 0x0048_73d0).is_empty());
            assert_eq!(calls_to(&log, 0x0047_7ba0), vec![vec![refr]]);
            returns(&mut e, 0x0047_7ba0, 0);
            assert!(!e.call(0x0057_22c0, &args![refr, 0u32]).bool());
        }
    }

    #[test]
    fn is_3d_critical_is_true_for_a_non_actor() {
        let mut e = engine();
        let plain = reference(&mut e, false, &[]);
        let actor = reference(&mut e, true, &[]);
        assert!(e.call(0x0057_2350, &args![plain]).bool());
        assert!(!e.call(0x0057_2350, &args![actor]).bool());
    }

    #[test]
    fn distance_to_a_position_is_the_length_of_the_difference_vector() {
        let mut e = engine();
        let refr = e.mem.alloc(0x100);
        let position = e.mem.alloc(0xc);
        e.register(0x0045_7990, |_, _| float_ret(7.5));
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_2380, &args![refr, position]).f32(), 7.5);
        });
        // position - this->Location (+0x30), into a local that is then measured
        let difference = calls_to(&log, 0x0043_9ef0)[0][1];
        assert_eq!(
            calls_to(&log, 0x0043_9ef0),
            vec![vec![position, difference, refr + 0x30]]
        );
        assert_eq!(calls_to(&log, 0x0045_7990), vec![vec![difference]]);
    }

    /// `this` and `other` references; `other` reports `position`; the length
    /// of every difference vector is 12.5; the cell of a reference holds its
    /// world space at `+0x10` (also what `TESObjectREFR::GetWorldSpace`
    /// reads from the reference itself).
    fn distance_setup() -> (Engine, u32, u32, u32) {
        let mut e = engine();
        e.set_global(0x0101_6970, f32::MAX);
        let position = e.mem.alloc(0xc);
        let this = reference(&mut e, false, &[]);
        let other = reference(&mut e, false, &[(0x1f4, position)]);
        e.register(0x0045_7990, |_, _| float_ret(12.5));
        e.register(0x0054_ddd0, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(0x0057_5d70, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        // `IsInterior` (`00425fd0`): bit 0 of the byte at `+0x24` of the cell
        e.register(0x0042_5fd0, |e, a| {
            ret(u32::from(e.mem.u8(a[0] + 0x24) & 1))
        });
        (e, this, other, position)
    }

    fn distance(e: &mut Engine, this: u32, other: u32, include: u8, ignore: u8) -> f32 {
        e.call(0x0057_23b0, &args![this, other, include, ignore])
            .f32()
    }

    #[test]
    fn distance_without_a_reference_is_the_largest_float() {
        let (mut e, this, _, _) = distance_setup();
        assert_eq!(distance(&mut e, this, 0, 0, 0), f32::MAX);
    }

    #[test]
    fn distance_skips_a_disabled_or_deleted_reference() {
        let (mut e, this, other, _) = distance_setup();
        // no cells: the world spaces of the two references (both 0) differ
        // from "non-null", so give them one
        e.mem.set_u32(this + 0x10, 5);
        e.mem.set_u32(other + 0x10, 5);
        assert_eq!(distance(&mut e, this, other, 0, 0), 12.5);
        e.mem.set_u32(other + 8, 0x800);
        assert_eq!(distance(&mut e, this, other, 0, 0), f32::MAX);
        assert_eq!(distance(&mut e, this, other, 1, 0), 12.5);
        e.mem.set_u32(other + 8, 0x20);
        assert_eq!(distance(&mut e, this, other, 1, 0), f32::MAX);
    }

    #[test]
    fn distance_between_exterior_cells_needs_the_same_non_null_world_space() {
        let (mut e, this, other, position) = distance_setup();
        let this_cell = e.mem.alloc(0x40);
        let other_cell = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x40, this_cell);
        e.mem.set_u32(other + 0x40, other_cell);
        e.mem.set_u32(this_cell + 0x10, 7);
        e.mem.set_u32(other_cell + 0x10, 7);
        let log = logged(&mut e, |e| {
            assert_eq!(distance(e, this, other, 0, 0), 12.5);
        });
        assert_eq!(
            calls_to(&log, 0x0054_ddd0),
            vec![vec![other_cell], vec![this_cell]]
        );
        assert_eq!(calls_to(&log, slot(&e, other, 0x1f4)), vec![vec![other]]);
        assert_eq!(calls_to(&log, 0x0043_9ef0)[0][0], position);
        // different world spaces
        e.mem.set_u32(other_cell + 0x10, 8);
        assert_eq!(distance(&mut e, this, other, 0, 0), f32::MAX);
        // ... unless the cells are to be ignored
        assert_eq!(distance(&mut e, this, other, 0, 1), 12.5);
        // the same world space 0 is not a space
        e.mem.set_u32(other_cell + 0x10, 0);
        e.mem.set_u32(this_cell + 0x10, 0);
        assert_eq!(distance(&mut e, this, other, 0, 0), f32::MAX);
    }

    #[test]
    fn distance_between_interior_cells_needs_the_same_cell() {
        let (mut e, this, other, _) = distance_setup();
        let this_cell = e.mem.alloc(0x40);
        let other_cell = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x40, this_cell);
        e.mem.set_u32(other + 0x40, other_cell);
        e.mem.set_u8(other_cell + 0x24, 1);
        assert_eq!(distance(&mut e, this, other, 0, 0), f32::MAX);
        assert_eq!(distance(&mut e, this, other, 0, 1), 12.5);
        // the same interior cell
        e.mem.set_u32(this + 0x40, other_cell);
        assert_eq!(distance(&mut e, this, other, 0, 0), 12.5);
    }

    #[test]
    fn distance_with_one_reference_outside_a_cell_compares_the_world_spaces() {
        let (mut e, this, other, _) = distance_setup();
        let other_cell = e.mem.alloc(0x40);
        e.mem.set_u32(other + 0x40, other_cell);
        e.mem.set_u32(other_cell + 0x10, 9);
        // `this` has no cell: the world space of `other` itself is asked
        // (`00575d70`) in its place, as the game does
        e.mem.set_u32(other + 0x10, 9);
        let log = logged(&mut e, |e| {
            assert_eq!(distance(e, this, other, 0, 0), 12.5);
        });
        assert_eq!(calls_to(&log, 0x0057_5d70), vec![vec![other]]);
        assert_eq!(calls_to(&log, 0x0054_ddd0), vec![vec![other_cell]]);
        e.mem.set_u32(other + 0x10, 10);
        assert_eq!(distance(&mut e, this, other, 0, 0), f32::MAX);

        // the other way round: `other` has no cell and `this` has one
        let (mut e, this, other, _) = distance_setup();
        let this_cell = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x40, this_cell);
        e.mem.set_u32(this_cell + 0x10, 4);
        e.mem.set_u32(this + 0x10, 4);
        let log = logged(&mut e, |e| {
            assert_eq!(distance(e, this, other, 0, 0), 12.5);
        });
        assert_eq!(calls_to(&log, 0x0057_5d70), vec![vec![this]]);
        assert_eq!(calls_to(&log, 0x0054_ddd0), vec![vec![this_cell]]);
    }

    // ---- 00572500, 005725b0 .. 005726e0 ---------------------------------------

    #[test]
    fn delta_for_angle_rotates_the_unit_vector_by_the_clamped_angle() {
        let mut e = engine();
        let refr = e.mem.alloc(0x100);
        let rotation = e.mem.alloc(0xc);
        e.mem.set_f32(rotation + 8, 1.0);
        returns(&mut e, 0x0043_0830, rotation);
        e.register_double(0x004b_1480, |_, a| {
            // the clamp returns its argument plus one
            float_ret(f32::from_bits(a[0]) + 1.0)
        });
        let rotated = e.mem.alloc(0xc);
        e.mem.set_f32(rotated, 0.25);
        e.mem.set_f32(rotated + 4, 0.5);
        e.mem.set_f32(rotated + 8, 0.75);
        returns(&mut e, 0x004b_4500, rotated);
        let out = e.mem.alloc(0xc);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_2500, &args![refr, out, 0.5f32]).u32(), out);
        });
        // angle z + the argument, clamped, then the rotation matrix
        assert_eq!(calls_to(&log, 0x004b_1480), vec![vec![float_bits(1.5)]]);
        let unit = calls_to(&log, 0x0041_6870)[0].clone();
        assert_eq!(
            &unit[1..],
            &[float_bits(0.0), float_bits(1.0), float_bits(0.0)]
        );
        let matrix = calls_to(&log, 0x004a_0c90)[0].clone();
        assert_eq!(matrix[1], float_bits(2.5));
        let rotation_call = calls_to(&log, 0x004b_4500)[0].clone();
        assert_eq!(rotation_call[0], matrix[0]);
        assert_eq!(rotation_call[2], unit[0]);
        assert_ne!(rotation_call[1], unit[0]);
        // the rotated vector is copied back, adjusted, and copied to `out`
        assert_eq!(calls_to(&log, 0x004a_0c10), vec![vec![unit[0]]]);
        assert_eq!(e.mem.f32(out), 0.25);
        assert_eq!(e.mem.f32(out + 4), 0.5);
        assert_eq!(e.mem.f32(out + 8), 0.75);
    }

    #[test]
    fn form_type_ranges_of_005725b0() {
        let mut e = engine();
        for (kind, expected) in [
            (0x3c, false),
            (0x3d, true),
            (0x40, true),
            (0x41, false),
            (0x68, false),
            (0x69, true),
            (0x6a, false),
        ] {
            let refr = form(&mut e, kind, 0);
            assert_eq!(e.call(0x0057_25b0, &args![refr]).bool(), expected);
        }
    }

    #[test]
    fn light_accessors_read_the_extra_list() {
        let mut e = engine();
        let refr = e.mem.alloc(0x100);
        returns(&mut e, 0x0041_8250, 0x1111);
        returns(&mut e, 0x0041_8280, 0x2222);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_25f0, &args![refr]).u32(), 0x1111);
            assert_eq!(e.call(0x0057_2750, &args![refr]).u32(), 0x2222);
        });
        assert_eq!(calls_to(&log, 0x0041_8250), vec![vec![refr + 0x44]]);
        assert_eq!(calls_to(&log, 0x0041_8280), vec![vec![refr + 0x44]]);
    }

    #[test]
    fn holder_constructor_initialises_through_00633c90_and_returns_this() {
        let mut e = engine();
        let holder = e.mem.alloc(8);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_26c0, &args![holder]).u32(), holder);
        });
        assert_eq!(calls_to(&log, 0x0063_3c90), vec![vec![holder, 0]]);
    }

    #[test]
    fn light_holders_are_created_filled_and_stored_on_the_extra_list() {
        for (entry, setter) in [(0x0057_2610, 0x0041_8e30), (0x0057_2770, 0x0041_8f60)] {
            let mut e = engine();
            let refr = e.mem.alloc(0x100);
            let holder = e.mem.alloc(8);
            returns(&mut e, 0x0040_1000, holder);
            let log = logged(&mut e, |e| {
                e.call(entry, &args![refr, 0x7777u32]);
            });
            assert_eq!(calls_to(&log, 0x0040_1000), vec![vec![8]]);
            assert_eq!(calls_to(&log, 0x0063_3c90), vec![vec![holder, 0]]);
            assert_eq!(calls_to(&log, 0x0066_b0d0), vec![vec![holder, 0x7777]]);
            assert_eq!(e.mem.f32(holder + 4), 1.0);
            assert_eq!(calls_to(&log, setter), vec![vec![refr + 0x44, holder]]);
        }
    }

    #[test]
    fn set_light_switches_the_off_and_on_children_oppositely() {
        let mut e = engine();
        let refr = e.mem.alloc(0x100);
        let node = e.mem.alloc(0x10);
        e.register(0x004a_ae30, |_, a| {
            ret(match a[1] {
                0x0103_0e54 => 0xa1,
                0x0103_0e48 => 0xb2,
                _ => 0,
            })
        });
        let log = logged(&mut e, |e| {
            e.call(0x0057_26e0, &args![refr, node, 1u8]);
        });
        assert_eq!(
            calls_to(&log, 0x004a_ae30),
            vec![vec![node, 0x0103_0e54], vec![node, 0x0103_0e48]]
        );
        assert_eq!(
            calls_to(&log, 0x0045_0f90),
            vec![vec![0xa1, 1], vec![0xb2, 0]]
        );
        let log = logged(&mut e, |e| {
            e.call(0x0057_26e0, &args![refr, node, 0u8]);
        });
        assert_eq!(
            calls_to(&log, 0x0045_0f90),
            vec![vec![0xa1, 0], vec![0xb2, 1]]
        );
        // a node without those children, and no node at all
        e.register(0x004a_ae30, |_, _| Ret::default());
        let log = logged(&mut e, |e| {
            e.call(0x0057_26e0, &args![refr, node, 1u8]);
            e.call(0x0057_26e0, &args![refr, 0u32, 1u8]);
        });
        assert!(calls_to(&log, 0x0045_0f90).is_empty());
        assert_eq!(calls_to(&log, 0x004a_ae30).len(), 2);
    }

    // ---- 00572820 .. 00572b60 ---------------------------------------------------

    #[test]
    fn base_lights_are_set_for_a_reference_of_a_form_type_1e_base() {
        let mut e = engine();
        let refr = reference(&mut e, false, &[]);
        let base = form(&mut e, 0x1e, 0);
        e.mem.set_u32(refr + 0x20, base);
        returns(&mut e, 0x0041_8250, 0x1111);
        returns(&mut e, 0x0041_8280, 0x2222);
        let log = logged(&mut e, |e| {
            e.call(0x0057_2820, &args![refr]);
        });
        assert_eq!(
            calls_to(&log, 0x0050_de20),
            vec![vec![base, 0x1111, 0], vec![base, 0x2222, 0]]
        );
        // a holder that is missing is skipped
        returns(&mut e, 0x0041_8250, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0057_2820, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x0050_de20), vec![vec![base, 0x2222, 0]]);
        // another form type of the base: nothing
        e.mem.set_u8(base + 4, 0x1f);
        let log = logged(&mut e, |e| {
            e.call(0x0057_2820, &args![refr]);
        });
        assert!(calls_to(&log, 0x0050_de20).is_empty());
    }

    /// A reference with a holder (`00418250`) around a light node, and the
    /// shadow scene node `00450b80` gives.
    fn light_setup() -> (Engine, u32, u32, u32) {
        let mut e = engine();
        let refr = reference(&mut e, false, &[]);
        let holder = e.mem.alloc(0x10);
        let node = e.mem.alloc(0x10);
        e.mem.set_u32(holder, node);
        returns(&mut e, 0x0041_8250, holder);
        returns(&mut e, 0x0045_0b80, 0x5c5c);
        (e, refr, holder, node)
    }

    #[test]
    fn a_light_is_added_to_the_shadow_scene_node() {
        let (mut e, refr, _, node) = light_setup();
        let log = logged(&mut e, |e| {
            e.call(0x0057_28c0, &args![refr, 0u8]);
        });
        assert_eq!(calls_to(&log, 0x0045_0b80), vec![vec![0]]);
        assert_eq!(calls_to(&log, 0x00b5_c940), vec![vec![0x5c5c, node, 1]]);
    }

    #[test]
    fn a_spell_effect_light_comes_from_the_other_holder() {
        let (mut e, refr, _, _) = light_setup();
        let holder = e.mem.alloc(0x10);
        let node = e.mem.alloc(0x10);
        e.mem.set_u32(holder, node);
        returns(&mut e, 0x0041_8280, holder);
        let log = logged(&mut e, |e| {
            e.call(0x0057_28c0, &args![refr, 1u8]);
        });
        assert_eq!(calls_to(&log, 0x00b5_c940), vec![vec![0x5c5c, node, 1]]);
    }

    #[test]
    fn a_light_is_not_added_for_a_deleted_or_disabled_reference_or_without_a_node() {
        let (mut e, refr, holder, node) = light_setup();
        for flags in [0x20, 0x800] {
            e.mem.set_u32(refr + 8, flags);
            let log = logged(&mut e, |e| {
                e.call(0x0057_28c0, &args![refr, 0u8]);
            });
            assert!(calls_to(&log, 0x00b5_c940).is_empty());
        }
        e.mem.set_u32(refr + 8, 0);
        // the holder holds no light
        e.mem.set_u32(holder, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0057_28c0, &args![refr, 0u8]);
        });
        assert!(calls_to(&log, 0x00b5_c940).is_empty());
        e.mem.set_u32(holder, node);
        // no holder
        returns(&mut e, 0x0041_8250, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0057_28c0, &args![refr, 0u8]);
        });
        assert!(calls_to(&log, 0x00b5_c940).is_empty());
    }

    #[test]
    fn a_light_with_lit_water_references_needs_them_empty() {
        let (mut e, refr, _, node) = light_setup();
        returns(&mut e, 0x0041_f810, 0x3333);
        // `0082 56d0` says "not empty": no light
        returns(&mut e, 0x0082_56d0, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0057_28c0, &args![refr, 0u8]);
        });
        assert_eq!(calls_to(&log, 0x0082_56d0), vec![vec![0x3333]]);
        assert!(calls_to(&log, 0x00b5_c940).is_empty());
        returns(&mut e, 0x0082_56d0, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0057_28c0, &args![refr, 0u8]);
        });
        assert_eq!(calls_to(&log, 0x00b5_c940), vec![vec![0x5c5c, node, 1]]);
    }

    #[test]
    fn the_shadow_flag_of_a_form_type_1e_base_comes_from_0050dd90() {
        let (mut e, refr, _, node) = light_setup();
        let base = form(&mut e, 0x1e, 0);
        e.mem.set_u32(refr + 0x20, base);
        // only the low byte counts
        returns(&mut e, 0x0050_dd90, 0x100);
        let log = logged(&mut e, |e| {
            e.call(0x0057_28c0, &args![refr, 0u8]);
        });
        assert_eq!(calls_to(&log, 0x0050_dd90), vec![vec![base]]);
        assert_eq!(calls_to(&log, 0x00b5_c940), vec![vec![0x5c5c, node, 0]]);
    }

    #[test]
    fn remove_light_takes_the_node_out_of_the_shadow_scene_node() {
        let (mut e, refr, holder, node) = light_setup();
        let log = logged(&mut e, |e| {
            e.call(0x0057_29e0, &args![refr, 0u8]);
        });
        assert_eq!(calls_to(&log, 0x00b5_eed0), vec![vec![0x5c5c, node]]);
        // the spell effect holder is asked for when the flag is set
        let log = logged(&mut e, |e| {
            e.call(0x0057_29e0, &args![refr, 1u8]);
        });
        assert!(calls_to(&log, 0x00b5_eed0).is_empty());
        assert_eq!(calls_to(&log, 0x0041_8280).len(), 1);
        // an empty holder
        e.mem.set_u32(holder, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0057_29e0, &args![refr, 0u8]);
        });
        assert!(calls_to(&log, 0x00b5_eed0).is_empty());
    }

    #[test]
    fn kill_light_also_clears_the_holder_and_the_extra() {
        let (mut e, refr, holder, node) = light_setup();
        let log = logged(&mut e, |e| {
            e.call(0x0057_2a50, &args![refr, 0u8]);
        });
        assert_eq!(calls_to(&log, 0x00b5_eed0), vec![vec![0x5c5c, node]]);
        assert_eq!(calls_to(&log, 0x0066_b0d0), vec![vec![holder, 0]]);
        assert_eq!(calls_to(&log, 0x0041_ae30), vec![vec![refr + 0x44]]);
        assert!(calls_to(&log, 0x0041_ae50).is_empty());
        // the empty holder is not cleared, the extra still is
        e.mem.set_u32(holder, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0057_2a50, &args![refr, 0u8]);
        });
        assert!(calls_to(&log, 0x00b5_eed0).is_empty());
        assert!(calls_to(&log, 0x0066_b0d0).is_empty());
        assert_eq!(calls_to(&log, 0x0041_ae30).len(), 1);
        // the spell effect variant uses the other holder and clearer
        returns(&mut e, 0x0041_8280, holder);
        let log = logged(&mut e, |e| {
            e.call(0x0057_2a50, &args![refr, 1u8]);
        });
        assert_eq!(calls_to(&log, 0x0041_ae50), vec![vec![refr + 0x44]]);
        assert!(calls_to(&log, 0x0041_ae30).is_empty());
        // no holder at all: nothing
        returns(&mut e, 0x0041_8250, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0057_2a50, &args![refr, 0u8]);
        });
        assert!(calls_to(&log, 0x0041_ae30).is_empty());
    }

    #[test]
    fn the_extra_vector_getters_return_the_out_pointer() {
        let mut e = engine();
        let refr = e.mem.alloc(0x100);
        let out = e.mem.alloc(0xc);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_2b00, &args![refr, out]).u32(), out);
            assert_eq!(e.call(0x0057_2b30, &args![refr, out]).u32(), out);
        });
        assert_eq!(
            calls_to(&log, 0x0041_b080),
            vec![vec![refr + 0x44, out, refr]]
        );
        assert_eq!(
            calls_to(&log, 0x0041_b0d0),
            vec![vec![refr + 0x44, out, refr]]
        );
    }

    #[test]
    fn starting_position_is_stored_under_the_scope_lock() {
        let mut e = engine();
        let position = e.mem.alloc(0xc);
        e.mem.set_u32(position, 0xa1);
        e.mem.set_u32(position + 4, 0xa2);
        e.mem.set_u32(position + 8, 0xa3);
        let refr = reference(&mut e, false, &[(0x1f4, position)]);
        let rotation = e.mem.alloc(0xc);
        e.mem.set_u32(rotation, 0xb1);
        e.mem.set_u32(rotation + 4, 0xb2);
        e.mem.set_u32(rotation + 8, 0xb3);
        returns(&mut e, 0x0043_0830, rotation);
        // the given vector differs from the zero vector at 011f426c
        let seen = Rc::new(RefCell::new(vec![]));
        let seen_in_double = seen.clone();
        e.register_double(0x0043_9090, move |e, a| {
            for word in 0..3 {
                seen_in_double.borrow_mut().push(e.mem.u32(a[0] + 4 * word));
            }
            ret(1)
        });
        let log = logged(&mut e, |e| {
            e.call(0x0057_2b60, &args![refr, 1u32, 2u32, 3u32]);
        });
        assert_eq!(*seen.borrow(), vec![1, 2, 3]);
        assert_eq!(calls_to(&log, 0x0043_9090)[0][1], 0x011f_426c);
        let guard = calls_to(&log, 0x0040_4eb0)[0][0];
        assert_eq!(
            calls_to(&log, 0x0040_4eb0),
            vec![vec![guard, 0x31, 1, 0x0102_fc20, 0x3b01]]
        );
        let position_call = calls_to(&log, 0x0041_b180)[0].clone();
        assert_eq!(
            &position_call[2..],
            &[refr, 1, 2, 3],
            "the given vector is stored as the starting position"
        );
        let rotation_call = calls_to(&log, 0x0041_b120)[0].clone();
        assert_eq!(&rotation_call[2..], &[refr, 0xb1, 0xb2, 0xb3]);
        assert_eq!(position_call[0], refr + 0x44);
        assert_eq!(rotation_call[0], refr + 0x44);
        // the lock is taken first and released last
        assert_eq!(
            addresses(&log)[1],
            0x0040_4eb0,
            "the scope lock comes first"
        );
        assert_eq!(*log.last().unwrap(), (0x0040_4ee0, vec![guard]));

        // the zero vector: the reference's own position is stored instead
        returns(&mut e, 0x0043_9090, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0057_2b60, &args![refr, 0u32, 0u32, 0u32]);
        });
        let position_call = calls_to(&log, 0x0041_b180)[0].clone();
        assert_eq!(&position_call[2..], &[refr, 0xa1, 0xa2, 0xa3]);
    }

    // ---- 00572c80 .. 00572e30 -------------------------------------------------

    #[test]
    fn data_handler_ids_and_form_types_decide_00572c80() {
        let mut e = engine();
        let refr = e.mem.alloc(0x100);
        let base = form(&mut e, 0x10, 0);
        e.mem.set_u32(refr + 0x20, base);
        let handler = e.mem.alloc(0x100);
        e.set_global(GLOBAL_DATA_HANDLER, handler);
        e.register_double(FORM_ID, |_, _| ret(0x1234));
        // the id is above 0xfeffffff for the handler
        returns(&mut e, 0x0046_9860, 1);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0057_2c80, &args![refr]).bool());
        });
        assert_eq!(calls_to(&log, 0x0046_9860), vec![vec![handler, 0x1234]]);
        assert!(calls_to(&log, GET_BASE_FORM).is_empty());
        returns(&mut e, 0x0046_9860, 0);
        for (kind, expected) in [
            (0x10, true),
            (0x1f, true),
            (0x21, false),
            (0x22, true),
            (0x24, true),
            (0x25, false),
            (0x26, false),
            (0x27, false),
            (0x28, true),
        ] {
            e.mem.set_u8(base + 4, kind);
            assert_eq!(
                e.call(0x0057_2c80, &args![refr]).bool(),
                expected,
                "{kind:#x}"
            );
        }
        // form type 0x20 asks 00444ed0
        e.mem.set_u8(base + 4, 0x20);
        returns(&mut e, 0x0044_4ed0, 0);
        assert!(!e.call(0x0057_2c80, &args![refr]).bool());
        returns(&mut e, 0x0044_4ed0, 1);
        assert!(e.call(0x0057_2c80, &args![refr]).bool());
    }

    #[test]
    fn extra_flag_accessors_forward_to_the_extra_list() {
        let mut e = engine();
        let refr = e.mem.alloc(0x100);
        returns(&mut e, 0x0041_b370, 0x0107);
        returns(&mut e, 0x0041_b3a0, 1);
        returns(&mut e, 0x0041_b520, 0x9999);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_2d10, &args![refr]).u8(), 7);
            assert!(e.call(0x0057_2d30, &args![refr, 4u32]).bool());
            assert_eq!(e.call(0x0057_2e30, &args![refr]).u32(), 0x9999);
            e.call(0x0057_2e10, &args![refr, 0x4242u32]);
        });
        assert_eq!(calls_to(&log, 0x0041_b370), vec![vec![refr + 0x44]]);
        assert_eq!(calls_to(&log, 0x0041_b3a0), vec![vec![refr + 0x44, 4]]);
        assert_eq!(calls_to(&log, 0x0041_b520), vec![vec![refr + 0x44]]);
        assert_eq!(calls_to(&log, 0x0041_b4b0), vec![vec![refr + 0x44, 0x4242]]);
    }

    #[test]
    fn setting_or_clearing_bits_of_the_extra_byte_changes_the_form_flag_for_4() {
        // (entry, extra function, slot when the original state is set, slot otherwise)
        for (entry, extra_fn, set_slot, unset_slot) in [
            (0x0057_2d50, 0x0041_b440, 0x4c, 0x48),
            (0x0057_2db0, 0x0041_b470, 0x48, 0x4c),
        ] {
            let mut e = engine();
            let refr = reference(&mut e, false, &[(0x48, 0), (0x4c, 0)]);
            returns(&mut e, 0x0056_1d90, 1);
            let log = logged(&mut e, |e| {
                e.call(entry, &args![refr, 4u32]);
            });
            assert_eq!(
                calls_to(&log, slot(&e, refr, set_slot)),
                vec![vec![refr, 0x0080_0000]]
            );
            assert!(calls_to(&log, slot(&e, refr, unset_slot)).is_empty());
            assert_eq!(calls_to(&log, extra_fn), vec![vec![refr + 0x44, 4]]);

            returns(&mut e, 0x0056_1d90, 0);
            let log = logged(&mut e, |e| {
                e.call(entry, &args![refr, 4u32]);
            });
            assert_eq!(
                calls_to(&log, slot(&e, refr, unset_slot)),
                vec![vec![refr, 0x0080_0000]]
            );
            assert!(calls_to(&log, slot(&e, refr, set_slot)).is_empty());

            // any other mask only reaches the extra list
            let log = logged(&mut e, |e| {
                e.call(entry, &args![refr, 2u32]);
            });
            assert_eq!(addresses(&log), vec![entry, GET_EXTRA_LIST, extra_fn]);
            assert_eq!(calls_to(&log, extra_fn), vec![vec![refr + 0x44, 2]]);
        }
    }

    // ---- 00572e50 .. 00573150 -------------------------------------------------

    /// A reference whose mobile-object test answers `mobile`, with the data
    /// handler global in place and the animation in its extra list being
    /// `current` (`00418220`).
    fn animation_setup(mobile: bool, current: u32) -> (Engine, u32) {
        let mut e = engine();
        let refr = reference(&mut e, mobile, &[(0xfc, u32::from(mobile)), (0x4c, 0)]);
        let handler = e.mem.alloc(0x700);
        e.set_global(GLOBAL_DATA_HANDLER, handler);
        returns(&mut e, 0x0041_8220, current);
        (e, refr)
    }

    #[test]
    fn set_animation_on_a_plain_reference_replaces_the_animation_of_the_extra_list() {
        let (mut e, refr) = animation_setup(false, 0x10);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_2e50, &args![refr, 0x20u32]).u32(), 0x20);
        });
        assert_eq!(calls_to(&log, 0x0041_8c40), vec![vec![refr + 0x44, 0x20]]);
        assert!(calls_to(&log, 0x0041_adf0).is_empty());
        assert!(calls_to(&log, 0x0048_4b90).is_empty());
        // removing it
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_2e50, &args![refr, 0u32]).u32(), 0);
        });
        assert_eq!(calls_to(&log, 0x0041_adf0), vec![vec![refr + 0x44]]);
        // the same animation: nothing is stored
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_2e50, &args![refr, 0x10u32]).u32(), 0x10);
        });
        assert!(calls_to(&log, 0x0041_8c40).is_empty());
        assert!(calls_to(&log, 0x0041_adf0).is_empty());
    }

    #[test]
    fn set_animation_of_a_mobile_object_hands_it_to_the_process() {
        let (mut e, refr) = animation_setup(true, 0);
        let process = object(&mut e, 0x100, &[(0x820, 0)]);
        e.mem.set_u32(refr + 0x68, process);
        // process level 0
        returns(&mut e, 0x0045_cd60, 0);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_2e50, &args![refr, 0x20u32]).u32(), 0x20);
        });
        assert_eq!(calls_to(&log, 0x0048_4b90), vec![vec![refr, 0x1000_0000]]);
        assert_eq!(
            calls_to(&log, slot(&e, process, 0x820)),
            vec![vec![process, 0x20]]
        );
        assert!(calls_to(&log, 0x0041_8c40).is_empty());
        assert!(calls_to(&log, 0x0093_1920).is_empty());
        // removing the animation clears the change flag through the virtual
        let log = logged(&mut e, |e| {
            e.call(0x0057_2e50, &args![refr, 0u32]);
        });
        assert_eq!(
            calls_to(&log, slot(&e, refr, 0x4c)),
            vec![vec![refr, 0x1000_0000]]
        );
        assert!(calls_to(&log, 0x0048_4b90).is_empty());
        assert_eq!(
            calls_to(&log, slot(&e, process, 0x820)),
            vec![vec![process, 0]]
        );
    }

    #[test]
    fn set_animation_changes_the_process_level_first_for_a_high_level() {
        let (mut e, refr) = animation_setup(true, 0);
        let process = object(&mut e, 0x100, &[(0x820, 0)]);
        e.mem.set_u32(refr + 0x68, process);
        // level 2 before the change, level 1 after it
        let levels = Rc::new(RefCell::new(vec![2u32, 1u32].into_iter()));
        let levels_in_double = levels.clone();
        e.register_double(0x0045_cd60, move |_, _| {
            ret(levels_in_double.borrow_mut().next().unwrap())
        });
        let log = logged(&mut e, |e| {
            e.call(0x0057_2e50, &args![refr, 0x20u32]);
        });
        assert_eq!(calls_to(&log, 0x0093_1920), vec![vec![refr, 0]]);
        assert_eq!(
            calls_to(&log, slot(&e, process, 0x820)),
            vec![vec![process, 0x20]]
        );

        // a level that stays high: the process does not take the animation
        let (mut e, refr) = animation_setup(true, 0);
        let process = object(&mut e, 0x100, &[(0x820, 0)]);
        e.mem.set_u32(refr + 0x68, process);
        returns(&mut e, 0x0045_cd60, 3);
        let log = logged(&mut e, |e| {
            e.call(0x0057_2e50, &args![refr, 0x20u32]);
        });
        assert_eq!(calls_to(&log, 0x0093_1920), vec![vec![refr, 0]]);
        assert!(calls_to(&log, slot(&e, process, 0x820)).is_empty());
        assert_eq!(calls_to(&log, 0x0041_8c40), vec![vec![refr + 0x44, 0x20]]);
        // without the animation the level is not changed
        let log = logged(&mut e, |e| {
            e.call(0x0057_2e50, &args![refr, 0u32]);
        });
        assert!(calls_to(&log, 0x0093_1920).is_empty());
    }

    #[test]
    fn the_data_handler_byte_is_read() {
        let mut e = engine();
        let handler = e.mem.alloc(0x700);
        e.mem.set_u8(handler + 0x61f, 9);
        assert_eq!(e.call(0x0057_2fa0, &args![handler]).u8(), 9);
    }

    #[test]
    fn set_dismembered_with_a_limb_dismembers_it_and_notifies() {
        let mut e = engine();
        let refr = reference(&mut e, false, &[(0x48, 0)]);
        returns(&mut e, 0x0042_e820, 0x6000);
        let log = logged(&mut e, |e| {
            e.call(
                0x0057_2fc0,
                &args![refr, 3u32, 0x11u32, 0x22u32, 0x33u32, 1u8],
            );
        });
        assert_eq!(calls_to(&log, 0x0042_e820), vec![vec![refr + 0x44]]);
        assert_eq!(calls_to(&log, 0x0043_0410), vec![vec![0x6000, refr, 3, 1]]);
        assert_eq!(calls_to(&log, 0x0043_7730), vec![vec![0x6000, 0x22]]);
        assert_eq!(calls_to(&log, 0x0060_0ab0), vec![vec![0x6000, 0x11]]);
        assert_eq!(calls_to(&log, 0x0098_4f60), vec![vec![0x6000, 3]]);
        assert_eq!(
            log.last().unwrap(),
            &(slot(&e, refr, 0x48), vec![refr, 0x0002_0000])
        );
    }

    #[test]
    fn set_dismembered_without_a_limb_uses_the_fourth_argument() {
        let mut e = engine();
        let refr = reference(&mut e, false, &[(0x48, 0)]);
        returns(&mut e, 0x0042_e820, 0x6000);
        let log = logged(&mut e, |e| {
            e.call(
                0x0057_2fc0,
                &args![refr, 0xffff_ffffu32, 0x11u32, 0x22u32, 0x33u32, 1u8],
            );
        });
        assert!(calls_to(&log, 0x0043_0410).is_empty());
        assert_eq!(calls_to(&log, 0x0098_4f60), vec![vec![0x6000, 0x33]]);
    }

    #[test]
    fn dismemberment_accessors_need_the_extra_and_forward_to_it() {
        let mut e = engine();
        let refr = e.mem.alloc(0x100);
        // no dismemberment extra
        assert!(!e.call(0x0057_3090, &args![refr, 2u32]).bool());
        assert_eq!(e.call(0x0057_30d0, &args![refr]).i32(), -1);
        assert_eq!(e.call(0x0057_3110, &args![refr]).i32(), -1);
        let log = logged(&mut e, |e| {
            e.call(0x0057_3050, &args![refr, 2u32, 1u8]);
        });
        assert!(calls_to(&log, 0x0043_05f0).is_empty());

        returns(&mut e, 0x0042_e8c0, 0x7000);
        returns(&mut e, 0x0043_03e0, 1);
        returns(&mut e, 0x0044_edb0, 0xfffe);
        returns(&mut e, 0x0096_11e0, 5);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0057_3090, &args![refr, 2u32]).bool());
            assert_eq!(e.call(0x0057_30d0, &args![refr]).i32(), 0xfffe);
            assert_eq!(e.call(0x0057_3110, &args![refr]).i32(), 5);
            e.call(0x0057_3050, &args![refr, 2u32, 1u8]);
        });
        assert_eq!(calls_to(&log, 0x0043_03e0), vec![vec![0x7000, 2]]);
        assert_eq!(calls_to(&log, 0x0044_edb0), vec![vec![0x7000]]);
        assert_eq!(calls_to(&log, 0x0096_11e0), vec![vec![0x7000]]);
        assert_eq!(calls_to(&log, 0x0043_05f0), vec![vec![0x7000, 2, 1]]);
    }

    #[test]
    fn the_virtual_at_1e8_is_called_and_its_result_returned() {
        let mut e = engine();
        let refr = reference(&mut e, false, &[(0x1e8, 0x77)]);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_3150, &args![refr]).u32(), 0x77);
        });
        assert_eq!(calls_to(&log, slot(&e, refr, 0x1e8)), vec![vec![refr]]);
    }

    // ---- 00573170 ------------------------------------------------------------

    struct ActivateSetup {
        e: Engine,
        refr: u32,
        base: u32,
        activator: u32,
        player: u32,
    }

    /// A reference with a base object (form type 0x10, whose activation
    /// virtual at `+0x124` succeeds), activated by an actor that is not the
    /// player; the reference counts as already flagged (`0041b3a0`), so the
    /// plain case reaches the switch on the form type.
    fn activate_setup() -> ActivateSetup {
        let mut e = engine();
        let base = object(&mut e, 0x100, &[(0x124, 1)]);
        e.mem.set_u8(base + 4, 0x10);
        let refr = reference(&mut e, false, &[]);
        e.mem.set_u32(refr + 0x20, base);
        let activator_base = form(&mut e, 0x2a, 0);
        let activator = reference(&mut e, true, &[(0x610, 0)]);
        e.mem.set_u32(activator + 0x20, activator_base);
        let player = reference(&mut e, true, &[(0x1a0, 0), (0x610, 0)]);
        e.set_global(GLOBAL_PLAYER, player);
        let state = e.mem.alloc(4);
        e.mem.set_u8(state, 1);
        returns(&mut e, 0x0055_d520, state);
        returns(&mut e, 0x0041_b3a0, 1);
        ActivateSetup {
            e,
            refr,
            base,
            activator,
            player,
        }
    }

    fn activate(s: &mut ActivateSetup, activator: u32) -> bool {
        let refr = s.refr;
        s.e.call(0x0057_3170, &args![refr, activator, 3u8, 0x44u32, 0x55u32])
            .bool()
    }

    #[test]
    fn activating_runs_the_base_object_and_records_the_activation() {
        let mut s = activate_setup();
        let (refr, base, activator, player) = (s.refr, s.base, s.activator, s.player);
        let log = logged(&mut s.e, |e| {
            assert!(e
                .call(0x0057_3170, &args![refr, activator, 3u8, 0x44u32, 0x55u32])
                .bool());
        });
        assert_eq!(calls_to(&log, 0x0056_a290), vec![vec![refr, activator]]);
        assert_eq!(
            calls_to(&log, slot(&s.e, base, 0x124)),
            vec![vec![base, refr, activator, 3, 0x44, 0x55]]
        );
        // an actor activator is told, and the player checks its quest targets
        assert_eq!(
            calls_to(&log, 0x008b_c980),
            vec![vec![activator, refr, 0x44, 0x55, 0]]
        );
        assert_eq!(calls_to(&log, 0x0095_2c30), vec![vec![player, activator]]);
        assert_eq!(s.e.global::<u8>(0x011c_a428), 1);
        // the extra bookkeeping before the switch
        assert_eq!(
            calls_to(&log, 0x0042_13c0),
            vec![vec![refr + 0x44, refr, activator]]
        );
        assert_eq!(calls_to(&log, 0x0041_0140), vec![vec![refr + 0x44, 0x7c]]);
    }

    #[test]
    fn a_base_object_that_refuses_gives_false_and_no_activation_record() {
        let mut s = activate_setup();
        s.e.set_global::<u8>(0x011c_a428, 1);
        let base = s.base;
        set_slot(&mut s.e, base, 0x124, 0);
        let activator = s.activator;
        assert!(!activate(&mut s, activator));
        // the byte was cleared at the start and nothing set it again
        assert_eq!(s.e.global::<u8>(0x011c_a428), 0);
    }

    #[test]
    fn the_player_activating_is_told_and_not_activated_when_the_other_object_is_not_set() {
        let mut s = activate_setup();
        let (player, refr) = (s.player, s.refr);
        // the pipboy is opening
        returns(&mut s.e, 0x0070_9bc0, 1);
        let log = logged(&mut s.e, |e| {
            assert!(e
                .call(0x0057_3170, &args![refr, player, 0u8, 0u32, 0u32])
                .bool());
        });
        assert!(calls_to(&log, 0x0056_a290).is_empty());
        returns(&mut s.e, 0x0070_9bc0, 0);

        // the two player bytes differ
        returns(&mut s.e, 0x004e_af60, 1);
        s.e.mem.set_u8(player + 0x64c, 0);
        let log = logged(&mut s.e, |e| {
            assert!(!e
                .call(0x0057_3170, &args![refr, player, 0u8, 0u32, 0u32])
                .bool());
        });
        assert!(calls_to(&log, 0x0056_a290).is_empty());
        s.e.mem.set_u8(player + 0x64c, 1);

        // the player's virtual +0x1a0 holds and 008859e0 does not: a message
        set_slot(&mut s.e, player, 0x1a0, 1);
        s.e.set_global(0x0101_62c0, 2.0f32);
        let log = logged(&mut s.e, |e| {
            assert!(e
                .call(0x0057_3170, &args![refr, player, 0u8, 0u32, 0u32])
                .bool());
        });
        assert_eq!(calls_to(&log, 0x0040_3df0), vec![vec![0x011d_4f84]]);
        let shown = calls_to(&log, 0x0070_52f0);
        assert_eq!(shown.len(), 1);
        assert_eq!(shown[0][1..], [0, 0, 0, float_bits(2.0), 0]);
        assert!(calls_to(&log, 0x0056_a290).is_empty());
        // 008859e0 holds: the activation goes on
        returns(&mut s.e, 0x0088_59e0, 1);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_3170, &args![refr, player, 0u8, 0u32, 0u32]);
        });
        assert!(calls_to(&log, 0x0040_3df0).is_empty());
        assert_eq!(calls_to(&log, 0x0056_a290), vec![vec![refr, player]]);
    }

    #[test]
    fn a_non_actor_reference_answering_00477ba0_is_activated_by_doing_nothing() {
        let mut s = activate_setup();
        returns(&mut s.e, 0x0047_7ba0, 1);
        let (activator, refr) = (s.activator, s.refr);
        let log = logged(&mut s.e, |e| {
            assert!(e
                .call(0x0057_3170, &args![refr, activator, 0u8, 0u32, 0u32])
                .bool());
        });
        assert!(calls_to(&log, 0x0056_a290).is_empty());
        // an actor reference does not take that way
        set_slot(&mut s.e, refr, 0x100, 1);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_3170, &args![refr, activator, 0u8, 0u32, 0u32]);
        });
        assert_eq!(calls_to(&log, 0x0056_a290).len(), 1);
    }

    #[test]
    fn a_form_type_1c_base_activated_by_an_actor_needs_its_state_byte() {
        let mut s = activate_setup();
        s.e.mem.set_u8(s.base + 4, 0x1c);
        let (activator, refr) = (s.activator, s.refr);
        let zero_state = s.e.mem.alloc(4);
        returns(&mut s.e, 0x0055_d520, zero_state);
        assert!(!activate(&mut s, activator));
        assert_eq!(s.e.global::<u8>(0x011c_a428), 0);
        // with the byte set the door code runs
        s.e.mem.set_u8(zero_state, 1);
        let log = logged(&mut s.e, |e| {
            assert!(e
                .call(0x0057_3170, &args![refr, activator, 0u8, 0u32, 0u32])
                .bool());
        });
        assert_eq!(calls_to(&log, 0x0056_8e50), vec![vec![refr]]);
    }

    #[test]
    fn the_player_activating_with_a_zero_state_byte_needs_form_type_23_or_16() {
        let mut s = activate_setup();
        let (player, refr) = (s.player, s.refr);
        returns(&mut s.e, 0x004e_af60, 0);
        let zero_state = s.e.mem.alloc(4);
        returns(&mut s.e, 0x0055_d520, zero_state);
        let go = |s: &mut ActivateSetup| {
            s.e.call(0x0057_3170, &args![refr, player, 0u8, 0u32, 0u32])
                .bool()
        };
        assert!(!go(&mut s));
        for kind in [0x23, 0x16] {
            s.e.mem.set_u8(s.base + 4, kind);
            let log = logged(&mut s.e, |e| {
                e.call(0x0057_3170, &args![refr, player, 0u8, 0u32, 0u32]);
            });
            assert_eq!(calls_to(&log, 0x0056_a290).len(), 1, "{kind:#x}");
        }
    }

    #[test]
    fn a_reference_refused_by_0057b460_is_not_activated() {
        let mut s = activate_setup();
        returns(&mut s.e, 0x0057_b460, 1);
        let activator = s.activator;
        assert!(!activate(&mut s, activator));
    }

    #[test]
    fn a_locked_reference_that_is_not_opened_tells_the_player() {
        let mut s = activate_setup();
        returns(&mut s.e, 0x0041_eb60, 1);
        returns(&mut s.e, 0x0041_e7b0, 0);
        let (activator, player, refr) = (s.activator, s.player, s.refr);
        let log = logged(&mut s.e, |e| {
            assert!(!e
                .call(0x0057_3170, &args![refr, activator, 0u8, 0u32, 0u32])
                .bool());
        });
        assert_eq!(
            calls_to(&log, 0x0041_e7b0),
            vec![vec![refr + 0x44, activator]]
        );
        assert!(calls_to(&log, 0x0040_3df0).is_empty());
        // for the player the message names the text at 01030e78
        returns(&mut s.e, 0x004e_af60, 0);
        s.e.mem.set_u8(player + 0x64c, 0);
        s.e.set_global(0x0101_62c0, 2.0f32);
        let log = logged(&mut s.e, |e| {
            assert!(!e
                .call(0x0057_3170, &args![refr, player, 0u8, 0u32, 0u32])
                .bool());
        });
        assert_eq!(calls_to(&log, 0x0040_3df0), vec![vec![0x011d_2bf8]]);
        let shown = calls_to(&log, 0x0070_52f0);
        assert_eq!(shown.len(), 1);
        assert_eq!(shown[0][1..], [0, 0x0103_0e78, 0, float_bits(2.0), 0]);
        // a lock that opens does not stop the activation
        returns(&mut s.e, 0x0041_e7b0, 1);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_3170, &args![refr, activator, 0u8, 0u32, 0u32]);
        });
        assert_eq!(calls_to(&log, 0x0056_a290).len(), 1);
    }

    #[test]
    fn form_types_20_21_22_25_refuse_and_30_accepts_without_running_the_base_object() {
        let mut s = activate_setup();
        let (activator, base) = (s.activator, s.base);
        for (kind, expected) in [
            (0x20, false),
            (0x21, false),
            (0x22, false),
            (0x25, false),
            (0x30, true),
        ] {
            s.e.mem.set_u8(base + 4, kind);
            assert_eq!(activate(&mut s, activator), expected, "{kind:#x}");
            assert_eq!(s.e.global::<u8>(0x011c_a428), 0, "{kind:#x}");
        }
        // other types, including those just below 0x1c, run the base object
        for kind in [0x0b, 0x1b, 0x1d, 0x23, 0x2f, 0x31] {
            s.e.mem.set_u8(base + 4, kind);
            assert!(activate(&mut s, activator), "{kind:#x}");
        }
        // no base object: false
        let refr = s.refr;
        s.e.mem.set_u32(refr + 0x20, 0);
        assert!(!activate(&mut s, activator));
    }

    #[test]
    fn a_door_sends_the_player_through_its_teleport_cell_unless_the_state_is_4() {
        let mut s = activate_setup();
        let (player, base, refr) = (s.player, s.base, s.refr);
        s.e.mem.set_u8(base + 4, 0x1c);
        returns(&mut s.e, 0x004e_af60, 0);
        s.e.mem.set_u8(player + 0x64c, 0);
        returns(&mut s.e, 0x0056_8e50, 0x8000);
        returns(&mut s.e, 0x0043_a2b0, 0x9000);
        returns(&mut s.e, 0x0042_5fd0, 1);
        let process = object(&mut s.e, 0x100, &[(0x610, 0)]);
        s.e.mem.set_u32(player + 0x68, process);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_3170, &args![refr, player, 0u8, 0u32, 0u32]);
        });
        assert_eq!(calls_to(&log, 0x0043_a2b0), vec![vec![0x8000]]);
        assert_eq!(calls_to(&log, 0x0054_af80), vec![vec![0x9000]]);
        assert_eq!(calls_to(&log, 0x0056_a290).len(), 1);
        // the player is in state 4: no teleport
        set_slot(&mut s.e, process, 0x610, 4);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_3170, &args![refr, player, 0u8, 0u32, 0u32]);
        });
        assert!(calls_to(&log, 0x0054_af80).is_empty());
        assert_eq!(calls_to(&log, 0x0056_a290).len(), 1);
    }

    #[test]
    fn an_unflagged_reference_runs_its_scripts_and_reports_the_activation() {
        let mut s = activate_setup();
        returns(&mut s.e, 0x0041_b3a0, 0);
        let (refr, activator) = (s.refr, s.activator);
        // the script run sees the counter one higher and sets the byte
        let seen = Rc::new(RefCell::new(vec![]));
        let seen_in_double = seen.clone();
        s.e.register_double(0x0056_5870, move |e, _| {
            seen_in_double
                .borrow_mut()
                .push(e.global::<i32>(0x011c_a424));
            ret(0)
        });
        let log = logged(&mut s.e, |e| {
            assert!(!e
                .call(0x0057_3170, &args![refr, activator, 3u8, 0x44u32, 0x55u32])
                .bool());
        });
        assert_eq!(*seen.borrow(), vec![1]);
        assert_eq!(s.e.global::<i32>(0x011c_a424), 0);
        assert_eq!(calls_to(&log, 0x0041_b3a0), vec![vec![refr + 0x44, 1]]);
        assert_eq!(calls_to(&log, 0x0041_b440), vec![vec![refr + 0x44, 2]]);
        assert_eq!(
            calls_to(&log, 0x0041_b4b0),
            vec![vec![refr + 0x44, activator]]
        );
        assert_eq!(
            calls_to(&log, 0x008b_c980),
            vec![vec![activator, refr, 0x44, 0x55, 1]]
        );
        // the base object did not run
        assert!(calls_to(&log, 0x0056_a290).is_empty());

        // the scripts are nested at most 5 deep
        s.e.set_global::<i32>(0x011c_a424, 5);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_3170, &args![refr, activator, 3u8, 0u32, 0u32]);
        });
        assert!(calls_to(&log, 0x0056_5870).is_empty());
        assert_eq!(s.e.global::<i32>(0x011c_a424), 5);
        s.e.set_global::<i32>(0x011c_a424, 0);

        // a script that activated the reference already: its byte is the result
        s.e.register_double(0x0056_5870, |e, _| {
            e.set_global::<u8>(0x011c_a428, 1);
            ret(0)
        });
        let log = logged(&mut s.e, |e| {
            assert!(e
                .call(0x0057_3170, &args![refr, activator, 3u8, 0u32, 0u32])
                .bool());
        });
        assert!(calls_to(&log, 0x008b_c980).is_empty());
    }

    #[test]
    fn the_player_and_a_flagged_reference_skip_the_script_path() {
        let mut s = activate_setup();
        let (refr, activator) = (s.refr, s.activator);
        // flagged: skipped
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_3170, &args![refr, activator, 0u8, 0u32, 0u32]);
        });
        assert!(calls_to(&log, 0x0056_5870).is_empty());
        // the reference is the player itself: skipped too
        returns(&mut s.e, 0x0041_b3a0, 0);
        let player = s.player;
        s.e.mem.set_u32(player + 0x20, s.base);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_3170, &args![player, activator, 0u8, 0u32, 0u32]);
        });
        assert!(calls_to(&log, 0x0056_5870).is_empty());
        // an activator that is busy: skipped, and no log line is written
        let process = object(&mut s.e, 0x100, &[(0x610, 4)]);
        s.e.mem.set_u32(activator + 0x68, process);
        returns(&mut s.e, 0x0045_cd60, 0);
        returns(&mut s.e, 0x0070_3180, refr);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_3170, &args![refr, activator, 0u8, 0u32, 0u32]);
        });
        assert!(calls_to(&log, 0x0056_5870).is_empty());
        assert!(calls_to(&log, 0x0070_3c00).is_empty());
    }

    #[test]
    fn the_targeted_reference_activation_is_logged_with_both_names() {
        let mut s = activate_setup();
        let (refr, activator, base) = (s.refr, s.activator, s.base);
        returns(&mut s.e, 0x0070_3180, refr);
        let activator_base = s.e.mem.u32(activator + 0x20);
        s.e.register_double(0x0048_2720, move |_, a| {
            ret(if a[0] == base { 0xaa } else { 0xbb })
        });
        returns(&mut s.e, 0x0044_0e30, 0xcc);
        returns(&mut s.e, 0x0046_4f30, 0xdd);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_3170, &args![refr, activator, 0u8, 0u32, 0u32]);
        });
        let line = calls_to(&log, SPRINTF)[0].clone();
        // buffer, "'%s' activated %s '%s'", activator name, verb, reference name
        assert_eq!(&line[1..], &[0x0103_0e60, 0xbb, 0xdd, 0xaa]);
        assert_eq!(calls_to(&log, 0x0070_3c00), vec![vec![line[0]]]);
        assert_eq!(
            calls_to(&log, 0x0048_2720),
            vec![vec![base], vec![activator_base]]
        );
        assert_eq!(calls_to(&log, 0x0044_0e30), vec![vec![base]]);
        assert_eq!(calls_to(&log, 0x0046_4f30), vec![vec![0xcc, 0]]);
    }

    #[test]
    fn an_ash_pile_hands_the_activation_to_its_reference() {
        let mut s = activate_setup();
        let (refr, activator, base) = (s.refr, s.activator, s.base);
        s.e.set_global(0x011c_a280, base);
        let ash_base = object(&mut s.e, 0x100, &[(0x124, 1)]);
        s.e.mem.set_u8(ash_base + 4, 0x10);
        let ash = reference(&mut s.e, false, &[]);
        s.e.mem.set_u32(ash + 0x20, ash_base);
        returns(&mut s.e, 0x0041_e310, ash);
        let log = logged(&mut s.e, |e| {
            assert!(e
                .call(0x0057_3170, &args![refr, activator, 0u8, 1u32, 2u32])
                .bool());
        });
        assert_eq!(calls_to(&log, 0x0041_e310), vec![vec![refr + 0x44]]);
        // the base object of the ash pile reference ran, not the pile's own
        assert_eq!(calls_to(&log, 0x0056_a290), vec![vec![ash, activator]]);
        assert_eq!(
            calls_to(&log, slot(&s.e, ash_base, 0x124)),
            vec![vec![ash_base, ash, activator, 0, 1, 2]]
        );
        assert!(calls_to(&log, slot(&s.e, base, 0x124)).is_empty());
        // no ash pile reference: the reference itself is activated
        returns(&mut s.e, 0x0041_e310, 0);
        let log = logged(&mut s.e, |e| {
            e.call(0x0057_3170, &args![refr, activator, 0u8, 1u32, 2u32]);
        });
        assert_eq!(calls_to(&log, 0x0056_a290), vec![vec![refr, activator]]);
    }

    // ---- 005737e0, 00573800, 00573eb0, 00573ed0 -------------------------------

    #[test]
    fn the_player_byte_at_64c_is_read() {
        let mut e = engine();
        let player = e.mem.alloc(0x700);
        e.mem.set_u8(player + 0x64c, 3);
        assert_eq!(e.call(0x0057_37e0, &args![player]).u8(), 3);
    }

    #[test]
    fn the_flag_word_helpers_forward_with_the_bit_4() {
        let mut e = engine();
        returns(&mut e, 0x0047_b4d0, 1);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0057_3eb0, &args![0x5000u32]).bool());
        });
        assert_eq!(calls_to(&log, 0x0047_b4d0), vec![vec![0x5000, 4]]);
        let log = logged(&mut e, |e| {
            e.call(0x0057_3ed0, &args![0x5000u32, 1u8]);
        });
        assert_eq!(
            calls_to(&log, 0x004d_9fc0),
            vec![vec![0x5000, 1, 4], vec![0x5000, 1, 8]]
        );
        assert_eq!(calls_to(&log, 0x0057_3f00), vec![vec![0x5000, 1]]);
        let log = logged(&mut e, |e| {
            e.call(0x0057_3ed0, &args![0x5000u32, 0u8]);
        });
        assert_eq!(calls_to(&log, 0x004d_9fc0), vec![vec![0x5000, 0, 4]]);
        assert!(calls_to(&log, 0x0057_3f00).is_empty());
    }

    struct MoveSetup {
        e: Engine,
        refr: u32,
        old_cell: u32,
        cell: u32,
        world: u32,
    }

    /// A non-actor reference in the exterior cell `old_cell`, an interior
    /// destination `cell` and a world space `world`; `AddReference`
    /// (`00548230`) moves the reference into its cell the way the game does.
    /// The positions are 8192 and 4096, so the cell coordinates are (2, 1).
    fn move_setup() -> MoveSetup {
        let mut e = engine();
        let position = e.mem.alloc(0xc);
        e.mem.set_f32(position, 8192.0);
        e.mem.set_f32(position + 4, 4096.0);
        e.mem.set_f32(position + 8, 33.0);
        let refr = reference(
            &mut e,
            false,
            &[
                (0xfc, 0),
                (0x1d0, 0),
                (0x1f4, position),
                (0x22c, 0),
                (0x1c0, 0),
                (0x1c4, 0),
                (0x1cc, 0),
                (0x1fc, 0),
                (0xd4, 0),
                (0x48, 0),
                (0x228, 0),
            ],
        );
        let old_cell = e.mem.alloc(0x40);
        let cell = e.mem.alloc(0x40);
        let world = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, old_cell);
        // `IsInterior`: bit 0 of the byte at +0x24
        e.mem.set_u8(cell + 0x24, 1);
        e.register(0x0042_5fd0, |e, a| {
            ret(u32::from(e.mem.u8(a[0] + 0x24) & 1))
        });
        e.register(0x0054_8230, |e, a| {
            e.mem.set_u32(a[1] + 0x40, a[0]);
            Ret::default()
        });
        // float to int for the cell coordinates
        e.register(0x0040_6d90, |_, a| ret(f32::from_bits(a[0]) as i32 as u32));
        MoveSetup {
            e,
            refr,
            old_cell,
            cell,
            world,
        }
    }

    fn move_ref(s: &mut MoveSetup, cell: u32, world: u32) -> Vec<(u32, Vec<u32>)> {
        let refr = s.refr;
        logged(&mut s.e, |e| {
            e.call(0x0057_3800, &args![refr, cell, world]);
        })
    }

    #[test]
    fn moving_nowhere_changes_nothing() {
        let mut s = move_setup();
        let log = move_ref(&mut s, 0, 0);
        assert_eq!(addresses(&log)[0], 0x0057_3800);
        assert!(calls_to(&log, 0x0054_8230).is_empty());
        assert!(calls_to(&log, 0x0054_ca90).is_empty());
        // an exterior cell is not used as the destination
        let old_cell = s.old_cell;
        let log = move_ref(&mut s, old_cell, 0);
        assert!(calls_to(&log, 0x0054_8230).is_empty());
        assert!(calls_to(&log, 0x0054_ca90).is_empty());
    }

    #[test]
    fn moving_to_another_cell_records_the_starting_cell_and_the_persistent_list() {
        let mut s = move_setup();
        let (refr, cell) = (s.refr, s.cell);
        // it was in a world space it persists in
        returns(&mut s.e, 0x0057_5d70, 0x7777);
        returns(&mut s.e, GET_REF_PERSISTS, 1);
        let log = move_ref(&mut s, cell, 0);
        // the starting cell is recorded for a non-actor that has none
        assert_eq!(calls_to(&log, 0x0041_b320), vec![vec![refr + 0x44]]);
        assert_eq!(calls_to(&log, 0x0041_b2a0), vec![vec![refr + 0x44, refr]]);
        assert_eq!(calls_to(&log, 0x0058_8030), vec![vec![0x7777, refr]]);
        assert_eq!(calls_to(&log, 0x0054_8230), vec![vec![cell, refr, 0]]);
        // the cell is not loaded: the reference is reinitialised through the
        // virtual at +0x1c0; it persists, so no change flag is added
        assert_eq!(calls_to(&log, slot(&s.e, refr, 0x1c0)), vec![vec![refr]]);
        assert!(calls_to(&log, slot(&s.e, refr, 0x48)).is_empty());
        // the saved Havok data is not dropped for a reference that says no
        assert!(calls_to(&log, 0x0042_2c20).is_empty());
    }

    #[test]
    fn moving_to_another_cell_marks_a_non_persistent_reference_changed() {
        let mut s = move_setup();
        let (refr, cell, old_cell) = (s.refr, s.cell, s.old_cell);
        // it already has a starting cell
        returns(&mut s.e, 0x0041_b320, 1);
        let log = move_ref(&mut s, cell, 0);
        assert!(calls_to(&log, 0x0041_b2a0).is_empty());
        assert_eq!(calls_to(&log, slot(&s.e, refr, 0x48)), vec![vec![refr, 8]]);
        // the same move of the player adds no flag
        s.e.set_global(GLOBAL_PLAYER, refr);
        s.e.mem.set_u32(refr + 0x40, old_cell);
        let log = move_ref(&mut s, cell, 0);
        assert!(calls_to(&log, slot(&s.e, refr, 0x48)).is_empty());
    }

    #[test]
    fn moving_with_saved_havok_data_drops_it_first() {
        let mut s = move_setup();
        let (refr, cell) = (s.refr, s.cell);
        set_slot(&mut s.e, refr, 0x22c, 1);
        let log = move_ref(&mut s, cell, 0);
        let positions = addresses(&log);
        let drop_at = positions.iter().position(|a| *a == 0x0042_2c20).unwrap();
        assert_eq!(log[drop_at].1, vec![refr + 0x44]);
        assert_eq!(log[drop_at + 1], (slot(&s.e, refr, 0xd4), vec![refr, 0]));
    }

    #[test]
    fn moving_within_the_same_cell_updates_in_place() {
        let mut s = move_setup();
        let (refr, old_cell) = (s.refr, s.old_cell);
        s.e.mem.set_u8(old_cell + 0x24, 1);
        let log = move_ref(&mut s, old_cell, 0);
        assert_eq!(calls_to(&log, slot(&s.e, refr, 0x1fc)), vec![vec![refr, 0]]);
        assert!(calls_to(&log, 0x0054_8230).is_empty());
        // `008c7aa0` set: the helper on the handle that 004537b0 gives
        returns(&mut s.e, 0x008c_7aa0, 1);
        returns(&mut s.e, 0x0045_37b0, 0x1234);
        let log = move_ref(&mut s, old_cell, 0);
        assert_eq!(calls_to(&log, 0x0087_b270), vec![vec![0x1234, refr]]);
        assert_eq!(calls_to(&log, 0x0045_37b0), vec![Vec::<u32>::new()]);
        assert!(calls_to(&log, slot(&s.e, refr, 0x1fc)).is_empty());
    }

    #[test]
    fn a_mobile_object_with_a_controller_gets_its_height_and_position() {
        let mut s = move_setup();
        let (refr, old_cell) = (s.refr, s.old_cell);
        s.e.mem.set_u8(old_cell + 0x24, 1);
        set_slot(&mut s.e, refr, 0xfc, 1);
        let controller = s.e.mem.alloc(0x600);
        returns(&mut s.e, 0x0093_06d0, controller);
        let log = move_ref(&mut s, old_cell, 0);
        assert_eq!(calls_to(&log, 0x0093_06d0), vec![vec![refr]]);
        assert_eq!(
            calls_to(&log, 0x0057_3f20),
            vec![vec![controller, float_bits(33.0)]]
        );
        // the same cell: the controller is placed at the position again
        let placed = calls_to(&log, 0x0056_20e0);
        assert_eq!(placed.len(), 1);
        assert_eq!(placed[0][0], controller);
        assert_eq!(s.e.mem.f32(controller + 0x544), 33.0);
    }

    #[test]
    fn moving_into_a_loaded_cell_updates_the_reference_through_the_helpers() {
        let mut s = move_setup();
        let (refr, cell, old_cell) = (s.refr, s.cell, s.old_cell);
        s.e.register_double(0x0045_11e0, move |_, a| ret(u32::from(a[1] == cell)));
        // not a mobile object: updated through the virtual at +0x1fc
        let log = move_ref(&mut s, cell, 0);
        assert_eq!(calls_to(&log, 0x0045_11e0)[0][1..], [cell, 1]);
        assert_eq!(calls_to(&log, slot(&s.e, refr, 0x1fc)), vec![vec![refr, 0]]);
        // `008c7aa0` set: the helpers of the handle
        returns(&mut s.e, 0x008c_7aa0, 1);
        returns(&mut s.e, 0x0045_37b0, 0x1234);
        s.e.mem.set_u32(refr + 0x40, old_cell);
        let log = move_ref(&mut s, cell, 0);
        // the reference moved (1c0 replacement) and was loaded (update)
        assert_eq!(calls_to(&log, 0x0087_b2b0), vec![vec![0x1234, refr]]);
        assert_eq!(calls_to(&log, 0x0087_b270), vec![vec![0x1234, refr]]);
    }

    #[test]
    fn a_mobile_object_moved_into_a_loaded_cell_resumes_when_its_process_allows() {
        let mut s = move_setup();
        let (refr, cell, old_cell) = (s.refr, s.cell, s.old_cell);
        s.e.register_double(0x0045_11e0, move |_, a| ret(u32::from(a[1] == cell)));
        set_slot(&mut s.e, refr, 0xfc, 1);
        set_slot(&mut s.e, refr, 0x1d0, 0x4040);
        returns(&mut s.e, 0x0045_0ff0, 1);
        let process = object(&mut s.e, 0x100, &[(0x28c, 0)]);
        s.e.mem.set_u32(refr + 0x68, process);
        returns(&mut s.e, 0x0045_cd60, 1);
        let log = move_ref(&mut s, cell, 0);
        assert_eq!(calls_to(&log, 0x0045_0ff0), vec![vec![cell]]);
        assert_eq!(calls_to(&log, slot(&s.e, refr, 0x1c4)), vec![vec![refr]]);
        assert!(calls_to(&log, slot(&s.e, refr, 0x1fc)).is_empty());
        // process level 2: not resumed
        returns(&mut s.e, 0x0045_cd60, 2);
        s.e.mem.set_u32(refr + 0x40, old_cell);
        let log = move_ref(&mut s, cell, 0);
        assert!(calls_to(&log, slot(&s.e, refr, 0x1c4)).is_empty());
        assert_eq!(calls_to(&log, slot(&s.e, refr, 0x1fc)), vec![vec![refr, 0]]);
        // the process vetoes (virtual +0x28c nonzero): updated in place
        returns(&mut s.e, 0x0045_cd60, 1);
        set_slot(&mut s.e, process, 0x28c, 1);
        s.e.mem.set_u32(refr + 0x40, old_cell);
        let log = move_ref(&mut s, cell, 0);
        assert!(calls_to(&log, slot(&s.e, refr, 0x1c4)).is_empty());
        assert_eq!(calls_to(&log, slot(&s.e, refr, 0x1fc)), vec![vec![refr, 0]]);
    }

    #[test]
    fn moving_into_a_world_space_finds_the_cell_by_the_position() {
        let mut s = move_setup();
        let (refr, old_cell, world) = (s.refr, s.old_cell, s.world);
        // the old cell is an interior cell, so the move counts as a change
        s.e.mem.set_u8(old_cell + 0x24, 1);
        let found = s.cell;
        returns(&mut s.e, 0x0058_75a0, found);
        let log = move_ref(&mut s, 0, world);
        // the coordinates are the positions divided by 4096
        assert_eq!(calls_to(&log, 0x0058_75a0), vec![vec![world, 2, 1]]);
        assert_eq!(calls_to(&log, 0x0054_8230), vec![vec![found, refr, 0]]);
        // the starting world space is recorded; the old space was 0, so
        // nothing is removed from a persistent list
        assert_eq!(calls_to(&log, 0x0041_b2a0).len(), 1);
        assert!(calls_to(&log, 0x0058_8030).is_empty());
        // the cell is not loaded: reinitialised through the virtual at +0x1c0
        assert_eq!(calls_to(&log, slot(&s.e, refr, 0x1c0)), vec![vec![refr]]);
    }

    #[test]
    fn a_persistent_reference_changes_its_persistent_world_space_list() {
        let mut s = move_setup();
        let (refr, world) = (s.refr, s.world);
        let found = s.cell;
        returns(&mut s.e, 0x0058_75a0, found);
        returns(&mut s.e, GET_REF_PERSISTS, 1);
        returns(&mut s.e, 0x0057_5d70, 0x7777);
        let log = move_ref(&mut s, 0, world);
        assert_eq!(calls_to(&log, 0x0058_8030), vec![vec![0x7777, refr]]);
        assert_eq!(calls_to(&log, 0x0058_7ff0), vec![vec![world, refr]]);
        // the same world space: neither list changes
        returns(&mut s.e, 0x0057_5d70, world);
        let log = move_ref(&mut s, 0, world);
        assert!(calls_to(&log, 0x0058_8030).is_empty());
        assert!(calls_to(&log, 0x0058_7ff0).is_empty());
    }

    #[test]
    fn a_world_space_without_a_cell_there_takes_the_reference_out_of_its_cell() {
        let mut s = move_setup();
        let (refr, old_cell, world) = (s.refr, s.old_cell, s.world);
        returns(&mut s.e, 0x0058_75a0, 0);
        let log = move_ref(&mut s, 0, world);
        assert_eq!(calls_to(&log, 0x0054_ca90), vec![vec![old_cell, refr]]);
        assert_eq!(
            calls_to(&log, slot(&s.e, refr, 0x1cc)),
            vec![vec![refr, 0, 0]]
        );
        // while the save game is being loaded (0047c850) nothing more is done
        returns(&mut s.e, 0x0047_c850, 1);
        let log = move_ref(&mut s, 0, world);
        assert!(calls_to(&log, slot(&s.e, refr, 0x1cc)).is_empty());
    }

    #[test]
    fn an_actor_pushed_out_of_every_loaded_cell_is_unloaded_at_process_level_3() {
        let mut s = move_setup();
        let (refr, old_cell, world) = (s.refr, s.old_cell, s.world);
        set_slot(&mut s.e, refr, 0xfc, 1);
        set_slot(&mut s.e, refr, 0x100, 1);
        returns(&mut s.e, 0x0058_75a0, 0);
        returns(&mut s.e, 0x0093_34b0, 3);
        let save_load = s.e.mem.alloc(0x40);
        s.e.set_global(GLOBAL_SAVE_LOAD, save_load);
        let unloader = s.e.mem.alloc(0x40);
        s.e.set_global(GLOBAL_LOADING_OBJECT, unloader);
        let log = move_ref(&mut s, 0, world);
        assert_eq!(
            calls_to(&log, 0x0096_e870),
            vec![vec![OBJECT_PROCESS_LISTS, refr]]
        );
        assert_eq!(
            calls_to(&log, slot(&s.e, refr, 0x228)),
            vec![vec![refr, old_cell], vec![refr, 0]]
        );
        assert_eq!(calls_to(&log, 0x008d_0370), vec![vec![save_load, refr]]);
        assert_eq!(calls_to(&log, 0x0084_9730), vec![vec![unloader, refr, 0]]);
        assert!(calls_to(&log, slot(&s.e, refr, 0x48)).contains(&vec![refr, 8]));
        // a persistent actor is kept loaded
        returns(&mut s.e, GET_REF_PERSISTS, 1);
        let log = move_ref(&mut s, 0, world);
        assert!(calls_to(&log, 0x0084_9730).is_empty());
        assert_eq!(calls_to(&log, 0x0096_e870).len(), 1);
        // a process level other than 3 is kept as well
        returns(&mut s.e, GET_REF_PERSISTS, 0);
        returns(&mut s.e, 0x0093_34b0, 2);
        let log = move_ref(&mut s, 0, world);
        assert!(calls_to(&log, 0x0084_9730).is_empty());
    }

    #[test]
    fn the_flag_of_the_node_is_cleared_for_the_move_and_restored_afterwards() {
        let mut s = move_setup();
        let (refr, world) = (s.refr, s.world);
        set_slot(&mut s.e, refr, 0x1d0, 0x4040);
        let holder = s.e.mem.alloc(0x40);
        returns(&mut s.e, 0x0068_38b0, holder);
        returns(&mut s.e, 0x0047_b4d0, 1);
        returns(&mut s.e, 0x0058_75a0, 0);
        let log = move_ref(&mut s, 0, world);
        assert_eq!(calls_to(&log, 0x0068_38b0), vec![vec![0x4040]]);
        assert_eq!(
            calls_to(&log, 0x004d_9fc0),
            vec![vec![holder, 0, 4], vec![holder, 1, 4], vec![holder, 1, 8]]
        );
        assert_eq!(calls_to(&log, 0x0057_3f00), vec![vec![holder, 1]]);
        // the flag was not set: untouched
        returns(&mut s.e, 0x0047_b4d0, 0);
        let log = move_ref(&mut s, 0, world);
        assert!(calls_to(&log, 0x004d_9fc0).is_empty());
    }

    #[test]
    fn a_mobile_object_arriving_in_a_loaded_cell_of_another_world_space_is_woken() {
        let mut s = move_setup();
        let (refr, world) = (s.refr, s.world);
        let found = s.cell;
        returns(&mut s.e, 0x0058_75a0, found);
        s.e.register_double(0x0045_11e0, move |_, a| ret(u32::from(a[1] == found)));
        set_slot(&mut s.e, refr, 0xfc, 1);
        set_slot(&mut s.e, refr, 0x100, 1);
        let process = object(&mut s.e, 0x100, &[(0x28, 0)]);
        s.e.mem.set_u32(refr + 0x68, process);
        let log = move_ref(&mut s, 0, world);
        // another world space: the process is asked through its virtual +0x28
        assert_eq!(
            calls_to(&log, slot(&s.e, process, 0x28)),
            vec![vec![process]]
        );
        // the loaded reference does not enter the temporary change list
        assert!(calls_to(&log, 0x0096_e870).is_empty());
        // and it was reinitialised and updated
        assert_eq!(calls_to(&log, slot(&s.e, refr, 0x1c0)), vec![vec![refr]]);
    }

    // ---- 00573f00 .. 00576830 (third batch) ------------------------------

    /// A node of a `BSSimpleList` holding `item`, followed by `next`.
    fn list_node(e: &mut Engine, item: u32, next: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, item);
        e.mem.set_u32(node + 4, next);
        node
    }

    /// Maps and fills the constants of the exe the third batch reads.
    fn constants(e: &mut Engine) {
        for page in [
            0x0101_5000u32,
            0x0102_0000,
            0x0102_3000,
            0x0102_4000,
            0x0103_0000,
            0x011c_7000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_f64(0x0102_31b0, f64::from(f32::MAX));
        e.mem.set_f64(0x0102_41b0, -f64::from(f32::MAX));
        e.mem.set_f64(0x0103_0ed0, 4_096_000.0);
        e.mem.set_f64(0x0103_0ec8, -4_096_000.0);
        e.mem.set_f64(0x0102_0758, 10.0);
        e.mem.set_f32(0x0101_62c0, 2.0);
        e.mem.set_f32(0x0101_5f5c, -2_000_000.0);
    }

    /// A reference that is disabled (flag `0x800`), not an actor, with all
    /// the virtual slots `Enable` and `Disable` use answering 0.
    fn toggled_reference(e: &mut Engine, disabled: bool) -> u32 {
        let slots = [
            (0xc4, 0),
            (0xc8, 0),
            (0xfc, 0),
            (0x1cc, 0),
            (0x1d0, 0),
            (0x208, 0),
            (0x260, 0),
        ];
        let refr = reference(e, false, &slots);
        e.mem.set_u32(refr + 8, if disabled { 0x800 } else { 0 });
        refr
    }

    #[test]
    fn flag_setter_00573f00_passes_the_flag_and_bit_8() {
        let mut e = engine();
        let log = logged(&mut e, |e| {
            e.call(0x0057_3f00, &args![0x5000u32, 1u32]);
            e.call(0x0057_3f00, &args![0x5000u32, 0u32]);
        });
        assert_eq!(
            calls_to(&log, 0x004d_9fc0),
            vec![vec![0x5000, 1, 8], vec![0x5000, 0, 8]]
        );
    }

    #[test]
    fn float_setter_00573f20_stores_at_0x544() {
        let mut e = engine();
        let object = e.mem.alloc(0x600);
        e.call(0x0057_3f20, &args![object, 2.5f32]);
        assert_eq!(e.mem.f32(object + 0x544), 2.5);
    }

    #[test]
    fn enable_does_nothing_for_a_reference_that_is_not_disabled() {
        let mut e = engine();
        let refr = toggled_reference(&mut e, false);
        let log = logged(&mut e, |e| {
            e.call(0x0057_3f40, &args![refr]);
        });
        assert_eq!(addresses(&log), vec![0x0057_3f40, FORM_IS_DISABLED]);
    }

    #[test]
    fn enable_clears_the_disabled_state_and_refreshes_the_player() {
        let mut e = engine();
        constants(&mut e);
        let refr = toggled_reference(&mut e, true);
        let player = e.mem.alloc(0x40);
        e.set_global(GLOBAL_PLAYER, player);
        let log = logged(&mut e, |e| {
            e.call(0x0057_3f40, &args![refr]);
        });
        assert_eq!(calls_to(&log, slot(&e, refr, 0xc8)), vec![vec![refr, 1]]);
        assert_eq!(calls_to(&log, 0x0048_4af0), vec![vec![refr, 0]]);
        assert_eq!(calls_to(&log, 0x0095_2c30), vec![vec![player, refr]]);
        // not an actor, not mobile, no cell: nothing else is touched
        assert!(calls_to(&log, slot(&e, refr, 0x208)).is_empty());
        assert!(calls_to(&log, 0x0044_4850).is_empty());
        assert!(calls_to(&log, 0x0096_d450).is_empty());
    }

    #[test]
    fn enable_of_an_actor_in_a_cell_adds_it_to_the_cell_and_reinitialises_it() {
        let mut e = engine();
        constants(&mut e);
        let refr = reference(
            &mut e,
            true,
            &[(0xc8, 0), (0xfc, 0), (0x1d0, 0), (0x208, 0), (0x260, 0)],
        );
        e.mem.set_u32(refr + 8, 0x800);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        let log = logged(&mut e, |e| {
            e.call(0x0057_3f40, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x0054_5590), vec![vec![cell, refr]]);
        assert_eq!(calls_to(&log, slot(&e, refr, 0x208)), vec![vec![refr, 1]]);
    }

    #[test]
    fn enable_skips_the_ash_pile_of_the_two_excepted_base_forms() {
        let mut e = engine();
        constants(&mut e);
        let refr = toggled_reference(&mut e, true);
        let ash = toggled_reference(&mut e, true);
        returns(&mut e, 0x0041_e310, ash);
        let base = form(&mut e, 0x10, 0);
        e.mem.set_u32(refr + 0x20, base);
        let log = logged(&mut e, |e| {
            e.call(0x0057_3f40, &args![refr]);
        });
        // the ash pile is enabled too
        assert_eq!(calls_to(&log, slot(&e, ash, 0xc8)), vec![vec![ash, 1]]);
        // but not when the base object is one of the two globals
        e.set_global(SKIPPED_BASE_FORM_B, base);
        let log = logged(&mut e, |e| {
            e.call(0x0057_3f40, &args![refr]);
        });
        assert!(calls_to(&log, slot(&e, ash, 0xc8)).is_empty());
        e.set_global(SKIPPED_BASE_FORM_B, 0);
        e.set_global(SKIPPED_BASE_FORM_A, base);
        let log = logged(&mut e, |e| {
            e.call(0x0057_3f40, &args![refr]);
        });
        assert!(calls_to(&log, slot(&e, ash, 0xc8)).is_empty());
    }

    #[test]
    fn enable_gives_a_mobile_object_without_a_process_a_new_one() {
        let mut e = engine();
        constants(&mut e);
        let refr = reference(
            &mut e,
            false,
            &[(0xc8, 0), (0xfc, 1), (0x1d0, 0), (0x208, 0), (0x260, 0)],
        );
        e.mem.set_u32(refr + 8, 0x800);
        e.register_double(0x0040_1000, |e, a| ret(e.mem.alloc(a[0])));
        e.register_double(0x0090_6dc0, |_, a| ret(a[0]));
        let log = logged(&mut e, |e| {
            e.call(0x0057_3f40, &args![refr]);
        });
        let allocation = calls_to(&log, 0x0040_1000);
        assert_eq!(allocation, vec![vec![0xb4]]);
        let process = calls_to(&log, 0x0090_6dc0)[0][0];
        assert_eq!(calls_to(&log, 0x0040_7800), vec![vec![refr, process]]);
        assert_eq!(
            calls_to(&log, 0x0096_d450),
            vec![vec![OBJECT_PROCESS_LISTS, refr, 3, 0, 0, 0]]
        );
        // a mobile object that was not queued runs the virtual at +0x260
        assert_eq!(calls_to(&log, slot(&e, refr, 0x260)), vec![vec![refr]]);
    }

    #[test]
    fn enable_queues_a_reference_in_a_loaded_cell_for_loading() {
        let mut e = engine();
        constants(&mut e);
        let refr = toggled_reference(&mut e, true);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        let base = form(&mut e, 0x1e, 0);
        e.mem.set_u32(refr + 0x20, base);
        let loader = e.mem.alloc(0x40);
        e.set_global(GLOBAL_MODEL_LOADER, loader);
        returns(&mut e, 0x0045_11e0, 1);
        returns(&mut e, 0x0045_8be0, 77);
        let log = logged(&mut e, |e| {
            e.call(0x0057_3f40, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x0044_4850), vec![vec![loader, refr, 77, 0]]);
        // the loaded flag is only set when the cell passes 00450ff0
        assert!(calls_to(&log, 0x0057_9ac0).is_empty());
        returns(&mut e, 0x0045_0ff0, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0057_3f40, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x0057_9ac0), vec![vec![refr, 1]]);
        // a model loader that already has the reference leaves it alone
        returns(&mut e, 0x0044_5750, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0057_3f40, &args![refr]);
        });
        assert!(calls_to(&log, 0x0044_4850).is_empty());
    }

    #[test]
    fn enable_queues_other_base_forms_only_when_00444ed0_does_not_except_them() {
        let mut e = engine();
        constants(&mut e);
        let refr = toggled_reference(&mut e, true);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        let base = form(&mut e, 0x10, 0);
        e.mem.set_u32(refr + 0x20, base);
        returns(&mut e, 0x0045_11e0, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0057_3f40, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x0044_4850).len(), 1);
        returns(&mut e, 0x0044_4ed0, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0057_3f40, &args![refr]);
        });
        assert!(calls_to(&log, 0x0044_4850).is_empty());
        // type 0x15 is queued regardless
        e.mem.set_u8(base + 4, 0x15);
        let log = logged(&mut e, |e| {
            e.call(0x0057_3f40, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x0044_4850).len(), 1);
    }

    #[test]
    fn enable_of_an_actor_with_a_process_evaluates_its_package_again() {
        let mut e = engine();
        constants(&mut e);
        let refr = reference(
            &mut e,
            true,
            &[
                (0xc8, 0),
                (0xfc, 1),
                (0x1d0, 0),
                (0x208, 0),
                (0x260, 0),
                (0x25c, 0),
            ],
        );
        e.mem.set_u32(refr + 8, 0x800);
        let process = object(&mut e, 0x100, &[(0x14, 0)]);
        e.mem.set_u32(refr + 0x68, process);
        returns(&mut e, 0x0045_cd60, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0057_3f40, &args![refr]);
        });
        assert_eq!(
            calls_to(&log, slot(&e, process, 0x14)),
            vec![vec![process, refr, 1]]
        );
        assert_eq!(calls_to(&log, 0x008a_6ce0), vec![vec![refr, 0, 1]]);
        assert_eq!(calls_to(&log, slot(&e, refr, 0x25c)).len(), 1);
        // 0x260 is skipped only when the reference was queued
        assert_eq!(calls_to(&log, slot(&e, refr, 0x260)).len(), 1);
    }

    #[test]
    fn enable_visits_the_linked_children_and_resets_the_flag_for_a_positive_value() {
        let mut e = engine();
        constants(&mut e);
        let refr = toggled_reference(&mut e, true);
        let to_disable = toggled_reference(&mut e, false);
        let to_enable = toggled_reference(&mut e, true);
        let second = list_node(&mut e, to_enable, 0);
        let first = list_node(&mut e, to_disable, second);
        e.register_double(0x0056_ac90, move |_, a| {
            ret(if a[0] == refr { first } else { 0 })
        });
        e.register_double(0x0056_aa70, move |_, a| ret(u32::from(a[0] == to_disable)));
        returns(&mut e, 0x0041_dbd0, 1);
        e.register_double(0x0056_b190, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        e.mem.set_u8(0x011d_d435, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0057_3f40, &args![refr]);
        });
        // the child flagged by 0056aa70 is disabled ...
        assert_eq!(calls_to(&log, 0x0041_dc20).len(), 1);
        assert_eq!(
            calls_to(&log, slot(&e, to_disable, 0x1cc)),
            vec![vec![to_disable, 0, 0]]
        );
        // ... the other one is enabled, with the pop-in call first
        assert_eq!(calls_to(&log, 0x0056_c780), vec![vec![to_enable, 1]]);
        assert_eq!(
            calls_to(&log, slot(&e, to_enable, 0xc8)),
            vec![vec![to_enable, 1]]
        );
        assert_eq!(e.mem.u8(0x011d_d435), 0);
    }

    #[test]
    fn byte_clearer_005743f0_clears_the_flag() {
        let mut e = engine();
        e.mem.set_u8(0x011d_d435, 7);
        e.call(0x0057_43f0, &args![]);
        assert_eq!(e.mem.u8(0x011d_d435), 0);
    }

    #[test]
    fn disable_does_nothing_for_a_disabled_reference_or_the_player() {
        let mut e = engine();
        let refr = toggled_reference(&mut e, true);
        let log = logged(&mut e, |e| {
            e.call(0x0057_4400, &args![refr]);
        });
        assert_eq!(addresses(&log), vec![0x0057_4400, FORM_IS_DISABLED]);
        let player = toggled_reference(&mut e, false);
        e.set_global(GLOBAL_PLAYER, player);
        let log = logged(&mut e, |e| {
            e.call(0x0057_4400, &args![player]);
        });
        assert!(calls_to(&log, slot(&e, player, 0x1cc)).is_empty());
    }

    #[test]
    fn disable_of_a_plain_reference_runs_the_fixed_sequence() {
        let mut e = engine();
        constants(&mut e);
        let refr = toggled_reference(&mut e, false);
        let player = e.mem.alloc(0x40);
        e.set_global(GLOBAL_PLAYER, player);
        let log = logged(&mut e, |e| {
            e.call(0x0057_4400, &args![refr]);
        });
        assert_eq!(
            calls_to(&log, slot(&e, refr, 0x1cc)),
            vec![vec![refr, 0, 0]]
        );
        assert_eq!(calls_to(&log, slot(&e, refr, 0xc8)), vec![vec![refr, 1]]);
        assert_eq!(calls_to(&log, 0x0048_4af0), vec![vec![refr, 1]]);
        assert_eq!(calls_to(&log, 0x0056_c880), vec![vec![refr, 1, 0]]);
        assert_eq!(calls_to(&log, 0x0095_2c30), vec![vec![player, refr]]);
        assert_eq!(calls_to(&log, 0x0057_9ac0), vec![vec![refr, 0]]);
        // no process work, no radio
        assert!(calls_to(&log, 0x0093_a660).is_empty());
        assert!(calls_to(&log, 0x0083_5980).is_empty());
    }

    #[test]
    fn disable_hides_the_terrain_tree_of_a_base_form_that_has_one() {
        let mut e = engine();
        constants(&mut e);
        let refr = toggled_reference(&mut e, false);
        let base = form(&mut e, 0x10, 0);
        e.mem.set_u32(refr + 0x20, base);
        let tes = e.mem.alloc(0x40);
        e.set_global(GLOBAL_TES, tes);
        returns(&mut e, 0x0054_9580, 1);
        returns(&mut e, 0x004f_d3e0, 0x6000);
        returns(&mut e, 0x0058_6170, 0x6100);
        let log = logged(&mut e, |e| {
            e.call(0x0057_4400, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x004f_d3e0), vec![vec![tes]]);
        assert_eq!(calls_to(&log, 0x0058_6170), vec![vec![0x6000]]);
        assert_eq!(calls_to(&log, 0x006f_cfa0), vec![vec![0x6100, refr, 1]]);
    }

    #[test]
    fn disable_disables_the_ash_pile_unless_the_base_form_is_excepted() {
        let mut e = engine();
        constants(&mut e);
        let refr = toggled_reference(&mut e, false);
        let ash = toggled_reference(&mut e, false);
        returns(&mut e, 0x0041_e310, ash);
        let base = form(&mut e, 0x10, 0);
        e.mem.set_u32(refr + 0x20, base);
        let log = logged(&mut e, |e| {
            e.call(0x0057_4400, &args![refr]);
        });
        assert_eq!(calls_to(&log, slot(&e, ash, 0x1cc)).len(), 1);
        e.set_global(SKIPPED_BASE_FORM_A, base);
        let log = logged(&mut e, |e| {
            e.call(0x0057_4400, &args![refr]);
        });
        assert!(calls_to(&log, slot(&e, ash, 0x1cc)).is_empty());
    }

    /// A mobile actor with a process whose virtuals answer the way the
    /// test needs.
    fn disable_actor(e: &mut Engine, slots: &[(u32, u32)]) -> (u32, u32) {
        let mut all = vec![
            (0xc8, 0),
            (0xfc, 1),
            (0x1cc, 0),
            (0x1d0, 0),
            (0x4c, 0),
            (0x2e8, 0),
            (0x2fc, 0),
        ];
        all.extend_from_slice(slots);
        let refr = reference(e, true, &all);
        let process = object(e, 0x100, &[(0, 0), (0x22c, 0), (0x4c8, 0), (0x4d0, 0)]);
        e.mem.set_u32(refr + 0x68, process);
        (refr, process)
    }

    #[test]
    fn disable_of_an_actor_removes_it_from_the_world_lists_and_clears_its_process() {
        let mut e = engine();
        constants(&mut e);
        let (refr, process) = disable_actor(&mut e, &[]);
        let player = e.mem.alloc(0x40);
        e.set_global(GLOBAL_PLAYER, player);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        returns(&mut e, 0x0056_56d0, 1);
        returns(&mut e, 0x0093_1850, 3);
        let log = logged(&mut e, |e| {
            e.call(0x0057_4400, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x008c_2b60), vec![vec![refr]]);
        assert_eq!(calls_to(&log, 0x0082_4970), vec![vec![refr + 0x94]]);
        assert_eq!(calls_to(&log, 0x0093_a660), vec![vec![player, refr]]);
        assert_eq!(calls_to(&log, 0x0096_7350), vec![vec![player, refr]]);
        assert_eq!(
            calls_to(&log, 0x0096_e6f0),
            vec![vec![OBJECT_PROCESS_LISTS, refr]]
        );
        assert_eq!(calls_to(&log, 0x0054_5560), vec![vec![cell, refr]]);
        assert_eq!(
            calls_to(&log, 0x0096_d470),
            vec![vec![OBJECT_PROCESS_LISTS, refr, 3]]
        );
        assert_eq!(
            calls_to(&log, 0x0096_f600),
            vec![vec![OBJECT_PROCESS_LISTS, refr, 0]]
        );
        // the process is deleted with the flag 1 and cleared
        assert_eq!(calls_to(&log, slot(&e, process, 0)), vec![vec![process, 1]]);
        assert_eq!(calls_to(&log, 0x0040_7800), vec![vec![refr, 0]]);
        // the actor's two flag bits are cleared through the virtual at +0x4c
        assert_eq!(
            calls_to(&log, slot(&e, refr, 0x4c)),
            vec![vec![refr, 0x10_0000], vec![refr, 0x20_0000]]
        );
        // an actor's radio is switched off at the end
        assert_eq!(calls_to(&log, 0x0083_5980), vec![vec![refr]]);
        // 0x2e8 said no
        assert!(calls_to(&log, 0x008b_01c0).is_empty());
    }

    #[test]
    fn disable_of_an_actor_adds_death_work_deletes_arrows_and_ends_the_package() {
        let mut e = engine();
        constants(&mut e);
        let (refr, process) = disable_actor(&mut e, &[(0x288, 0)]);
        // the counter 00570f60 reads this global: two attached arrows
        e.set_global(0x011d_fc98u32, 2u32);
        set_slot(&mut e, refr, 0x2e8, 1);
        set_slot(&mut e, process, 0x22c, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0057_4400, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x008c_ddd0), vec![vec![refr, 1]]);
        assert_eq!(calls_to(&log, 0x008b_01c0), vec![vec![refr]]);
        // the process answered +0x22c: the actor's package is interrupted
        assert_eq!(calls_to(&log, slot(&e, refr, 0x288)).len(), 1);
        assert_eq!(calls_to(&log, 0x0088_1680), vec![vec![refr, 0]]);
    }

    #[test]
    fn disable_of_a_marker_user_gives_the_marker_back() {
        let mut e = engine();
        constants(&mut e);
        // not an actor, but mobile with a process using a marker
        let refr = reference(
            &mut e,
            false,
            &[(0xc8, 0), (0xfc, 1), (0x1cc, 0), (0x1d0, 0), (0x2fc, 0)],
        );
        let process = object(
            &mut e,
            0x100,
            &[(0, 0), (0x22c, 0), (0x4c8, 0x9000), (0x4d0, 0x9100)],
        );
        e.mem.set_u32(refr + 0x68, process);
        let log = logged(&mut e, |e| {
            e.call(0x0057_4400, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x0056_8020), vec![vec![0x9000, 0x9100, 0]]);
        assert!(calls_to(&log, 0x0088_d640).is_empty());
        // an actor leaves its furniture instead
        let (actor, _) = disable_actor(&mut e, &[]);
        let process = e.mem.u32(actor + 0x68);
        set_slot(&mut e, process, 0x4c8, 0x9000);
        let log = logged(&mut e, |e| {
            e.call(0x0057_4400, &args![actor]);
        });
        assert_eq!(calls_to(&log, 0x0088_d640), vec![vec![actor]]);
        assert!(calls_to(&log, 0x0056_8020).is_empty());
    }

    #[test]
    fn disable_visits_children_and_deletes_dropped_items() {
        let mut e = engine();
        constants(&mut e);
        let refr = toggled_reference(&mut e, false);
        let child_enabled = toggled_reference(&mut e, true);
        let child_disabled = toggled_reference(&mut e, false);
        let second = list_node(&mut e, child_disabled, 0);
        let first = list_node(&mut e, child_enabled, second);
        e.register_double(0x0056_ac90, move |_, a| {
            ret(if a[0] == refr { first } else { 0 })
        });
        e.register_double(0x0056_aa70, move |_, a| {
            ret(u32::from(a[0] == child_enabled))
        });
        returns(&mut e, 0x0041_dbd0, 1);
        // dropped items: one node; after 0063f7b0 the list is empty
        let item = toggled_reference(&mut e, false);
        let dropped = list_node(&mut e, item, 0);
        e.register_double(0x0041_df90, move |_, a| {
            ret(if a[0] == refr + 0x44 { dropped } else { 0 })
        });
        let emptied = Rc::new(RefCell::new(false));
        let flag = emptied.clone();
        e.register_double(0x0063_f7b0, move |_, _| {
            *flag.borrow_mut() = true;
            Ret::default()
        });
        let flag = emptied.clone();
        e.register_double(0x0082_56d0, move |_, _| ret(u32::from(*flag.borrow())));
        let log = logged(&mut e, |e| {
            e.call(0x0057_4400, &args![refr]);
        });
        // the child 0056aa70 flags is enabled (pop-in first), the other disabled
        assert_eq!(calls_to(&log, 0x0056_c780), vec![vec![child_enabled, 1]]);
        assert_eq!(
            calls_to(&log, slot(&e, child_enabled, 0xc8)),
            vec![vec![child_enabled, 1]]
        );
        assert_eq!(
            calls_to(&log, slot(&e, child_disabled, 0x1cc)),
            vec![vec![child_disabled, 0, 0]]
        );
        // the dropped item is cleared from its extra data and marked deleted
        assert_eq!(calls_to(&log, 0x0041_de40), vec![vec![item + 0x44, 0]]);
        assert_eq!(calls_to(&log, slot(&e, item, 0xc4)).len(), 1);
        assert_eq!(calls_to(&log, 0x0063_f7b0), vec![vec![dropped]]);
    }

    #[test]
    fn world_location_copies_the_position_with_the_space() {
        let mut e = engine();
        let position = e.mem.alloc(0xc);
        let refr = reference(&mut e, false, &[(0x1f4, position)]);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        returns(&mut e, 0x0054_ddd0, 0x7000);
        let out = e.mem.alloc(0x20);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_5650, &args![refr, out]).u32(), out);
        });
        assert_eq!(
            calls_to(&log, 0x0043_a3c0),
            vec![vec![out, position, 0x7000]]
        );
    }

    #[test]
    fn set_object_reference_stores_the_base_and_updates_the_flags() {
        let mut e = engine();
        let refr = reference(&mut e, false, &[(0xfc, 0), (0x1f8, 0)]);
        let object = e.mem.alloc(0x80);
        returns(&mut e, 0x0047_53d0, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0057_5690, &args![refr, object]);
        });
        assert_eq!(e.mem.u32(refr + 0x20), object);
        assert_eq!(calls_to(&log, 0x0048_46a0), vec![vec![refr, 1]]);
        // not mobile: the virtual at +0x1f8 is not run
        assert!(calls_to(&log, slot(&e, refr, 0x1f8)).is_empty());
        // mobile: it gets the opposite of the answer of 00461580 on object + 0x30
        set_slot(&mut e, refr, 0xfc, 1);
        returns(&mut e, 0x0046_1580, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0057_5690, &args![refr, object]);
        });
        assert_eq!(
            calls_to(&log, 0x0046_1580),
            vec![vec![object + 0x30, 0x200]]
        );
        assert_eq!(calls_to(&log, slot(&e, refr, 0x1f8)), vec![vec![refr, 1]]);
    }

    #[test]
    fn angle_setter_00575700_clamps_the_z_angle_of_an_actor() {
        let mut e = engine();
        e.register(0x004b_1480, |_, a| float_ret(f32::from_bits(a[0]) - 6.0));
        let refr = reference(&mut e, false, &[(0x48, 0)]);
        let log = logged(&mut e, |e| {
            e.call(0x0057_5700, &args![refr, 1.0f32, 2.0f32, 7.0f32]);
        });
        assert_eq!(e.mem.f32(refr + 0x24), 1.0);
        assert_eq!(e.mem.f32(refr + 0x28), 2.0);
        assert_eq!(e.mem.f32(refr + 0x2c), 7.0);
        assert_eq!(calls_to(&log, slot(&e, refr, 0x48)), vec![vec![refr, 2]]);
        let actor = reference(&mut e, true, &[(0x48, 0)]);
        e.call(0x0057_5700, &args![actor, 1.0f32, 2.0f32, 7.0f32]);
        assert_eq!(e.mem.f32(actor + 0x2c), 1.0);
        assert_eq!(e.mem.f32(actor + 0x28), 2.0);
    }

    #[test]
    fn single_angle_setters_store_one_component_and_notify() {
        let mut e = engine();
        e.register(0x004b_1480, |_, a| float_ret(f32::from_bits(a[0]) - 6.0));
        let refr = reference(&mut e, false, &[(0x48, 0)]);
        let log = logged(&mut e, |e| {
            e.call(0x0057_5770, &args![refr, 0.5f32]);
            e.call(0x0057_57a0, &args![refr, 1.5f32]);
            e.call(0x0057_57d0, &args![refr, 9.0f32]);
        });
        assert_eq!(e.mem.f32(refr + 0x24), 0.5);
        assert_eq!(e.mem.f32(refr + 0x28), 1.5);
        assert_eq!(e.mem.f32(refr + 0x2c), 9.0);
        assert_eq!(calls_to(&log, slot(&e, refr, 0x48)).len(), 3);
        // the Z angle of an actor is clamped
        let actor = reference(&mut e, true, &[(0x48, 0)]);
        e.call(0x0057_57d0, &args![actor, 9.0f32]);
        assert_eq!(e.mem.f32(actor + 0x2c), 3.0);
    }

    /// A reference ready for `SetLocationOnReference`.
    fn location_setup() -> (Engine, u32, u32) {
        let mut e = engine();
        constants(&mut e);
        let refr = reference(&mut e, false, &[(0x48, 0), (0x138, 0), (0x1d0, 0)]);
        let location = e.mem.alloc(0xc);
        e.mem.set_f32(location, 100.0);
        e.mem.set_f32(location + 4, 200.0);
        e.mem.set_f32(location + 8, 300.0);
        (e, refr, location)
    }

    #[test]
    fn set_location_stores_a_valid_position() {
        let (mut e, refr, location) = location_setup();
        let log = logged(&mut e, |e| {
            e.call(0x0057_5830, &args![refr, location]);
        });
        assert_eq!(e.mem.f32(refr + 0x30), 100.0);
        assert_eq!(e.mem.f32(refr + 0x34), 200.0);
        assert_eq!(e.mem.f32(refr + 0x38), 300.0);
        assert!(calls_to(&log, slot(&e, refr, 0x138)).is_empty());
        assert_eq!(calls_to(&log, slot(&e, refr, 0x48)), vec![vec![refr, 2]]);
    }

    #[test]
    fn set_location_replaces_a_position_out_of_range_and_updates_the_node() {
        let (mut e, refr, location) = location_setup();
        e.mem.set_f32(location, 5_000_000.0);
        let node = e.mem.alloc(0x40);
        let loaded = e.mem.alloc(0x40);
        e.mem.set_u32(loaded + 0x14, node);
        e.mem.set_u32(refr + 0x64, loaded);
        set_slot(&mut e, refr, 0x1d0, node);
        e.register_double(slot(&e, refr, 0x138), |e, a| {
            // the virtual writes a valid position to its first argument
            e.mem.set_f32(a[1], 1.0);
            e.mem.set_f32(a[1] + 4, 2.0);
            e.mem.set_f32(a[1] + 8, 3.0);
            ret(0)
        });
        let log = logged(&mut e, |e| {
            e.call(0x0057_5830, &args![refr, location]);
        });
        let call = &calls_to(&log, slot(&e, refr, 0x138))[0];
        assert_eq!(call[0], refr);
        assert_eq!(call.len(), 5);
        assert_eq!(call[4], 0);
        assert_eq!(e.mem.f32(refr + 0x30), 1.0);
        assert_eq!(e.mem.f32(refr + 0x34), 2.0);
        // Z was raised by 10
        assert_eq!(e.mem.f32(refr + 0x38), 13.0);
        // the node gets the translation, a simulation reset, properties and an update
        assert_eq!(calls_to(&log, 0x0044_0460), vec![vec![node, call[1]]]);
        assert_eq!(calls_to(&log, 0x00c6_bd00), vec![vec![node, 1]]);
        assert_eq!(calls_to(&log, 0x00a5_a040), vec![vec![node]]);
        let update = calls_to(&log, UPDATE_DATA_CONSTRUCTOR);
        assert_eq!(update.len(), 1);
        assert_eq!(update[0][1..], [0.0f32.to_bits(), 0, 0]);
        assert_eq!(calls_to(&log, NODE_UPDATE), vec![vec![node, update[0][0]]]);
    }

    #[test]
    fn set_location_while_loading_rejects_the_infinite_and_the_huge() {
        let (mut e, refr, location) = location_setup();
        let loading = e.mem.alloc(0x40);
        e.set_global(GLOBAL_LOADING_OBJECT, loading);
        returns(&mut e, 0x0042_ce10, 1);
        e.mem.set_f32(location + 8, f32::MAX);
        let log = logged(&mut e, |e| {
            e.call(0x0057_5830, &args![refr, location]);
        });
        assert_eq!(calls_to(&log, slot(&e, refr, 0x138)).len(), 1);
        // a plain position passes
        e.mem.set_f32(location + 8, 5.0);
        let log = logged(&mut e, |e| {
            e.call(0x0057_5830, &args![refr, location]);
        });
        assert!(calls_to(&log, slot(&e, refr, 0x138)).is_empty());
        // a NaN in X is not a valid position either
        e.mem.set_f32(location, f32::NAN);
        let log = logged(&mut e, |e| {
            e.call(0x0057_5830, &args![refr, location]);
        });
        assert_eq!(calls_to(&log, slot(&e, refr, 0x138)).len(), 1);
        // while not loading, a NaN passes the plain range check
        returns(&mut e, 0x0042_ce10, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0057_5830, &args![refr, location]);
        });
        assert!(calls_to(&log, slot(&e, refr, 0x138)).is_empty());
    }

    #[test]
    fn set_location_moves_a_fixed_persistent_reference_out_of_its_world_space() {
        let (mut e, refr, location) = location_setup();
        returns(&mut e, GET_REF_PERSISTS, 1);
        returns(&mut e, 0x0058_7c80, 1);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        returns(&mut e, 0x0054_ddd0, 0x7000);
        let log = logged(&mut e, |e| {
            e.call(0x0057_5830, &args![refr, location]);
        });
        assert_eq!(calls_to(&log, 0x0058_7e40), vec![vec![0x7000, refr]]);
        assert_eq!(calls_to(&log, 0x0058_7d10), vec![vec![0x7000, refr]]);
        let order = addresses(&log);
        let removed = order.iter().position(|a| *a == 0x0058_7e40).unwrap();
        let added = order.iter().position(|a| *a == 0x0058_7d10).unwrap();
        assert!(removed < added);
    }

    #[test]
    fn set_location_z_keeps_x_and_y() {
        let (mut e, refr, _) = location_setup();
        e.mem.set_f32(refr + 0x30, 11.0);
        e.mem.set_f32(refr + 0x34, 22.0);
        e.mem.set_f32(refr + 0x38, 33.0);
        e.call(0x0057_5b70, &args![refr, 99.0f32]);
        assert_eq!(e.mem.f32(refr + 0x30), 11.0);
        assert_eq!(e.mem.f32(refr + 0x34), 22.0);
        assert_eq!(e.mem.f32(refr + 0x38), 99.0);
    }

    #[test]
    fn set_parent_cell_updates_the_water_height_of_the_loaded_data() {
        let mut e = engine();
        constants(&mut e);
        let refr = reference(&mut e, false, &[]);
        let loaded = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x64, loaded);
        let cell = e.mem.alloc(0x40);
        returns(&mut e, 0x0045_18e0, 1);
        e.register_double(0x0054_71e0, |_, _| float_ret(123.5));
        e.call(0x0057_5bb0, &args![refr, cell]);
        assert_eq!(e.mem.u32(refr + 0x40), cell);
        assert_eq!(e.mem.f32(loaded + 8), 123.5);
        // a cell without water, or no cell: the default
        returns(&mut e, 0x0045_18e0, 0);
        e.call(0x0057_5bb0, &args![refr, cell]);
        assert_eq!(e.mem.f32(loaded + 8), -2_000_000.0);
        e.mem.set_f32(loaded + 8, 5.0);
        e.call(0x0057_5bb0, &args![refr, 0u32]);
        assert_eq!(e.mem.f32(loaded + 8), -2_000_000.0);
        // a water zone map in the extra data leaves the height alone
        e.mem.set_f32(loaded + 8, 5.0);
        returns(&mut e, 0x0042_f1d0, 1);
        e.call(0x0057_5bb0, &args![refr, cell]);
        assert_eq!(e.mem.f32(loaded + 8), 5.0);
    }

    #[test]
    fn set_parent_cell_of_the_player_clears_the_global_flag() {
        let mut e = engine();
        constants(&mut e);
        let refr = reference(&mut e, false, &[]);
        e.set_global(GLOBAL_PLAYER, refr);
        e.mem.set_u8(0x011c_7a58, 1);
        e.call(0x0057_5bb0, &args![refr, 0u32]);
        assert_eq!(e.mem.u8(0x011c_7a58), 0);
        // another reference leaves it alone
        let other = reference(&mut e, false, &[]);
        e.mem.set_u8(0x011c_7a58, 1);
        e.call(0x0057_5bb0, &args![other, 0u32]);
        assert_eq!(e.mem.u8(0x011c_7a58), 1);
    }

    #[test]
    fn set_parent_cell_tells_the_process_of_an_actor() {
        let mut e = engine();
        constants(&mut e);
        let refr = reference(&mut e, true, &[(0x240, 0)]);
        let process = object(&mut e, 0x100, &[(0x6ac, 0)]);
        e.mem.set_u32(refr + 0x68, process);
        returns(&mut e, 0x0093_1850, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0057_5bb0, &args![refr, 0x4000u32]);
        });
        assert_eq!(
            calls_to(&log, slot(&e, process, 0x6ac)),
            vec![vec![process, refr]]
        );
        assert_eq!(calls_to(&log, slot(&e, refr, 0x240)).len(), 1);
        // another process type does not run +0x240
        returns(&mut e, 0x0093_1850, 2);
        let log = logged(&mut e, |e| {
            e.call(0x0057_5bb0, &args![refr, 0x4000u32]);
        });
        assert!(calls_to(&log, slot(&e, refr, 0x240)).is_empty());
    }

    #[test]
    fn global_flag_setter_00575c90_stores_the_byte() {
        let mut e = engine();
        constants(&mut e);
        e.call(0x0057_5c90, &args![5u32]);
        assert_eq!(e.mem.u8(0x011c_7a58), 5);
    }

    #[test]
    fn get_space_prefers_the_world_space_of_the_cell() {
        let mut e = engine();
        let refr = reference(&mut e, false, &[]);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        returns(&mut e, 0x0054_ddd0, 0x7000);
        assert_eq!(e.call(0x0057_5ca0, &args![refr]).u32(), 0x7000);
        // an interior has no world space: the cell itself
        returns(&mut e, 0x0054_ddd0, 0);
        assert_eq!(e.call(0x0057_5ca0, &args![refr]).u32(), cell);
        // without a cell: the world space of the persistent cell
        e.mem.set_u32(refr + 0x40, 0);
        assert_eq!(e.call(0x0057_5ca0, &args![refr]).u32(), 0);
        returns(&mut e, 0x0041_d460, 0x5000);
        returns(&mut e, 0x0054_ddd0, 0x7100);
        assert_eq!(e.call(0x0057_5ca0, &args![refr]).u32(), 0x7100);
    }

    #[test]
    fn get_interior_follows_the_cell_or_the_child_cell_base() {
        let mut e = engine();
        let refr = reference(&mut e, false, &[]);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        returns(&mut e, 0x0042_5fd0, 0);
        assert_eq!(e.call(0x0057_5d10, &args![refr]).u32() & 0xff, 0);
        returns(&mut e, 0x0042_5fd0, 1);
        assert_eq!(e.call(0x0057_5d10, &args![refr]).u32() & 0xff, 1);
        // no cell: the child cell base (vtable slot 0 of refr + 0x18) decides
        e.mem.set_u32(refr + 0x40, 0);
        let child_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x18, child_vtable);
        e.register_double(child_vtable, move |_, _| ret(0));
        e.mem.set_u32(child_vtable, child_vtable);
        assert_eq!(e.call(0x0057_5d10, &args![refr]).u32() & 0xff, 1);
        e.register_double(child_vtable, move |_, _| ret(0x8000));
        returns(&mut e, 0x0054_ddd0, 0x7000);
        assert_eq!(e.call(0x0057_5d10, &args![refr]).u32() & 0xff, 0);
        // a base cell without a world space is an interior
        returns(&mut e, 0x0054_ddd0, 0);
        assert_eq!(e.call(0x0057_5d10, &args![refr]).u32() & 0xff, 1);
    }

    #[test]
    fn get_world_space_uses_the_cell_or_the_child_cell_base() {
        let mut e = engine();
        let refr = reference(&mut e, false, &[]);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        returns(&mut e, 0x0054_ddd0, 0x7000);
        assert_eq!(e.call(0x0057_5d70, &args![refr]).u32(), 0x7000);
        e.mem.set_u32(refr + 0x40, 0);
        // no cell and no child cell: zero
        let child_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x18, child_vtable);
        e.mem.set_u32(child_vtable, child_vtable);
        e.register_double(child_vtable, move |_, _| ret(0));
        assert_eq!(e.call(0x0057_5d70, &args![refr]).u32(), 0);
        e.register_double(child_vtable, move |_, _| ret(0x8000));
        assert_eq!(e.call(0x0057_5d70, &args![refr]).u32(), 0x7000);
    }

    #[test]
    fn child_cell_cell_getter_replaces_unusable_cells_of_persistent_references() {
        let mut e = engine();
        let refr = reference(&mut e, false, &[]);
        let child = refr + 0x18;
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        returns(&mut e, 0x0041_d460, 0x5000);
        // not persistent: the parent cell
        assert_eq!(e.call(0x0057_5dc0, &args![child]).u32(), cell);
        // persistent, and the cell passes 00425fd0: still the parent cell
        returns(&mut e, GET_REF_PERSISTS, 1);
        returns(&mut e, 0x0042_5fd0, 1);
        assert_eq!(e.call(0x0057_5dc0, &args![child]).u32(), cell);
        // persistent and the cell fails it: the persistent cell
        returns(&mut e, 0x0042_5fd0, 0);
        assert_eq!(e.call(0x0057_5dc0, &args![child]).u32(), 0x5000);
        // the other persistence test (004077c0) counts as well
        returns(&mut e, GET_REF_PERSISTS, 0);
        returns(&mut e, 0x0040_77c0, 1);
        assert_eq!(e.call(0x0057_5dc0, &args![child]).u32(), 0x5000);
        // and a persistent reference without a cell
        e.mem.set_u32(refr + 0x40, 0);
        assert_eq!(e.call(0x0057_5dc0, &args![child]).u32(), 0x5000);
    }

    /// Doubles for the extra data accessors: the cast answers `cast`, the
    /// construction allocates the block and returns it.
    fn magic_setup(cast: u32) -> (Engine, u32) {
        let mut e = engine();
        let refr = reference(&mut e, false, &[(0x48, 0)]);
        e.register_double(DYNAMIC_CAST, move |_, a| {
            ret(if a[3] == TYPE_TES_MAGIC_TARGET_FORM {
                1
            } else {
                cast
            })
        });
        e.register_double(0x0040_1000, |e, a| ret(e.mem.alloc(a[0])));
        e.register_double(0x0082_5ad0, |_, a| ret(a[0]));
        e.register_double(0x0082_5fd0, |_, a| ret(a[0]));
        returns(&mut e, 0x0041_0220, 0x4400);
        (e, refr)
    }

    #[test]
    fn magic_caster_accessor_returns_the_existing_extra() {
        let (mut e, refr) = magic_setup(0x4000);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_5e30, &args![refr]).u32(), 0x400c);
        });
        assert_eq!(calls_to(&log, 0x0041_0220), vec![vec![refr + 0x44, 0x32]]);
        assert_eq!(
            calls_to(&log, DYNAMIC_CAST),
            vec![vec![
                0x4400,
                0,
                TYPE_BS_EXTRA_DATA,
                TYPE_NON_ACTOR_MAGIC_CASTER,
                0
            ]]
        );
        assert!(calls_to(&log, 0x0040_1000).is_empty());
    }

    #[test]
    fn magic_caster_accessor_creates_the_missing_extra() {
        let (mut e, refr) = magic_setup(0);
        let log = logged(&mut e, |e| {
            let caster = e.call(0x0057_5e30, &args![refr]).u32();
            let created = calls_to(&e.call_log.clone().unwrap(), 0x0082_5ad0)[0][0];
            assert_eq!(caster, created + 0xc);
        });
        assert_eq!(calls_to(&log, 0x0040_1000), vec![vec![0x24]]);
        let created = calls_to(&log, 0x0082_5ad0);
        assert_eq!(created[0][1], refr);
        assert_eq!(
            calls_to(&log, 0x0040_ff60),
            vec![vec![refr + 0x44, created[0][0]]]
        );
        assert_eq!(
            calls_to(&log, slot(&e, refr, 0x48)),
            vec![vec![refr, 0x8000_0000]]
        );
    }

    #[test]
    fn magic_caster_accessor_returns_zero_when_the_allocation_fails() {
        let (mut e, refr) = magic_setup(0);
        returns(&mut e, 0x0040_1000, 0);
        assert_eq!(e.call(0x0057_5e30, &args![refr]).u32(), 0);
    }

    #[test]
    fn magic_target_accessor_needs_a_magic_target_base_form() {
        let (mut e, refr) = magic_setup(0x4000);
        let base = form(&mut e, 0x10, 0);
        e.mem.set_u32(refr + 0x20, base);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_5f30, &args![refr]).u32(), 0x400c);
        });
        assert_eq!(calls_to(&log, 0x0041_0220), vec![vec![refr + 0x44, 0x33]]);
        // a base form that is not a TESMagicTargetForm: nothing
        e.register_double(DYNAMIC_CAST, |_, _| ret(0));
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_5f30, &args![refr]).u32(), 0);
        });
        assert!(calls_to(&log, 0x0041_0220).is_empty());
    }

    #[test]
    fn magic_target_accessor_creates_the_missing_extra() {
        let (mut e, refr) = magic_setup(0);
        let log = logged(&mut e, |e| {
            e.call(0x0057_5f30, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x0040_1000), vec![vec![0x28]]);
        let created = calls_to(&log, 0x0082_5fd0);
        assert_eq!(created[0][1], refr);
        assert_eq!(
            calls_to(&log, 0x0040_ff60),
            vec![vec![refr + 0x44, created[0][0]]]
        );
        assert_eq!(
            calls_to(&log, slot(&e, refr, 0x48)),
            vec![vec![refr, 0x8000_0000]]
        );
    }

    #[test]
    fn magic_target_form_test_00576040_casts_from_tes_form() {
        let mut e = engine();
        e.register_double(DYNAMIC_CAST, |_, _| ret(0x55));
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_6040, &args![0x1234u32]).u32() & 1, 1);
        });
        assert_eq!(
            calls_to(&log, DYNAMIC_CAST),
            vec![vec![
                0x1234,
                0,
                TYPE_TES_FORM,
                TYPE_TES_MAGIC_TARGET_FORM,
                0
            ]]
        );
        e.register_double(DYNAMIC_CAST, |_, _| ret(0));
        assert_eq!(e.call(0x0057_6040, &args![0x1234u32]).u32() & 1, 0);
    }

    #[test]
    fn destruction_state_00576070_follows_the_destruction_form_and_the_flag() {
        let mut e = engine();
        let refr = e.mem.alloc(0x100);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x20, base);
        // 00452370 rejects: zero
        assert_eq!(e.call(0x0057_6070, &args![refr]).u32() & 0xff, 0);
        returns(&mut e, 0x0045_2370, 1);
        // no destruction form: zero
        assert_eq!(e.call(0x0057_6070, &args![refr]).u32() & 0xff, 0);
        // a destruction form whose data has bit 0 of the byte at +5 set
        let destruction = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(destruction + 4, data);
        returns(&mut e, 0x0047_5400, destruction);
        assert_eq!(e.call(0x0057_6070, &args![refr]).u32() & 0xff, 0);
        e.mem.set_u8(data + 5, 1);
        assert_eq!(e.call(0x0057_6070, &args![refr]).u32() & 0xff, 1);
        // flag 0x4000000 inverts the answer
        e.mem.set_u32(refr + 8, 0x0400_0000);
        assert_eq!(e.call(0x0057_6070, &args![refr]).u32() & 0xff, 0);
        e.mem.set_u8(data + 5, 0);
        assert_eq!(e.call(0x0057_6070, &args![refr]).u32() & 0xff, 1);
        // also without a destruction form
        returns(&mut e, 0x0047_5400, 0);
        assert_eq!(e.call(0x0057_6070, &args![refr]).u32() & 0xff, 1);
    }

    #[test]
    fn flag_test_005760e0_reads_bit_0x4000000() {
        let mut e = engine();
        let form = e.mem.alloc(0x20);
        assert!(!e.call(0x0057_60e0, &args![form]).bool());
        e.mem.set_u32(form + 8, 0x0400_0000);
        assert!(e.call(0x0057_60e0, &args![form]).bool());
        e.mem.set_u32(form + 8, 0xfbff_ffff);
        assert!(!e.call(0x0057_60e0, &args![form]).bool());
    }

    #[test]
    fn data_flag_test_00576100_reads_bit_0_of_byte_5() {
        let mut e = engine();
        let object = e.mem.alloc(0x20);
        assert!(!e.call(0x0057_6100, &args![object]).bool());
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(object + 4, data);
        assert!(!e.call(0x0057_6100, &args![object]).bool());
        e.mem.set_u8(data + 5, 0x03);
        assert!(e.call(0x0057_6100, &args![object]).bool());
        e.mem.set_u8(data + 5, 0xfe);
        assert!(!e.call(0x0057_6100, &args![object]).bool());
    }

    /// Two actors on a process-list chain for the container tests.
    fn container_setup() -> (Engine, u32, u32, u32, u32, u32) {
        let mut e = engine();
        let refr = reference(&mut e, false, &[]);
        let extra = e.mem.alloc(0x40);
        let target = e.mem.alloc(0x40);
        let actor_a = e.mem.alloc(0x40);
        let actor_b = e.mem.alloc(0x40);
        let second = list_node(&mut e, actor_b, 0);
        let head = list_node(&mut e, actor_a, second);
        returns(&mut e, 0x0041_c8d0, target);
        e.register_double(FORM_ID, |_, _| ret(0x77));
        returns(&mut e, 0x0096_f450, head);
        e.register_double(0x0088_1650, move |_, a| ret(u32::from(a[0] == actor_a)));
        (e, refr, extra, actor_a, actor_b, head)
    }

    #[test]
    fn add_object_to_container_forwards_without_extra_data() {
        let mut e = engine();
        let refr = reference(&mut e, false, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0057_6130, &args![refr, 0x11u32, 0u32, 0x33u32]);
        });
        assert_eq!(addresses(&log), vec![0x0057_6130, 0x0057_4fa0]);
        assert_eq!(calls_to(&log, 0x0057_4fa0), vec![vec![refr, 0x11, 0, 0x33]]);
    }

    #[test]
    fn add_object_to_container_retargets_the_actors_that_had_the_item_as_target() {
        let (mut e, refr, extra, actor_a, actor_b, head) = container_setup();
        returns(&mut e, 0x0041_8660, 5);
        returns(&mut e, 0x0056_7790, 5);
        let log = logged(&mut e, |e| {
            e.call(0x0057_6130, &args![refr, 0x11u32, extra, 0x33u32]);
        });
        // the ownership is the reference's own: removed
        assert_eq!(calls_to(&log, 0x0041_aed0), vec![vec![extra]]);
        assert_eq!(
            calls_to(&log, 0x0096_f450),
            vec![vec![OBJECT_PROCESS_LISTS, 0x77, refr]]
        );
        // the actor with a package target gets this reference, the other 0
        assert_eq!(
            calls_to(&log, 0x0088_1620),
            vec![vec![actor_a, refr], vec![actor_b, 0]]
        );
        assert_eq!(calls_to(&log, 0x0047_0470), vec![vec![head]]);
        assert_eq!(calls_to(&log, 0x0047_02f0), vec![vec![head, 1]]);
        assert_eq!(
            calls_to(&log, 0x0057_4fa0),
            vec![vec![refr, 0x11, extra, 0x33]]
        );
    }

    #[test]
    fn add_object_to_container_keeps_ownership_of_another_owner() {
        let (mut e, refr, extra, _, _, _) = container_setup();
        returns(&mut e, 0x0041_8660, 5);
        returns(&mut e, 0x0056_7790, 6);
        let log = logged(&mut e, |e| {
            e.call(0x0057_6130, &args![refr, 0x11u32, extra, 0x33u32]);
        });
        assert!(calls_to(&log, 0x0041_aed0).is_empty());
        // no referenced object: no process list work either
        returns(&mut e, 0x0041_c8d0, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0057_6130, &args![refr, 0x11u32, extra, 0x33u32]);
        });
        assert!(calls_to(&log, 0x0096_f450).is_empty());
        assert_eq!(calls_to(&log, 0x0057_4fa0).len(), 1);
    }

    #[test]
    fn get_inventory_item_asks_the_container_changes() {
        let mut e = engine();
        let refr = reference(&mut e, false, &[]);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_6260, &args![refr, 1u32, 2u32]).u32(), 0);
        });
        assert!(calls_to(&log, 0x004d_0650).is_empty());
        returns(&mut e, 0x0041_8520, 0x9000);
        returns(&mut e, 0x004d_0650, 0x9100);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_6260, &args![refr, 1u32, 2u32]).u32(), 0x9100);
        });
        assert_eq!(calls_to(&log, 0x0041_8520), vec![vec![refr + 0x44]]);
        assert_eq!(calls_to(&log, 0x004d_0650), vec![vec![0x9000, 1, 2]]);
    }

    #[test]
    fn bound_center_adds_the_corners_halves_them_and_adds_the_position() {
        let mut e = engine();
        constants(&mut e);
        let position = e.mem.alloc(0xc);
        let refr = reference(&mut e, false, &[(0x1d8, 0), (0x1dc, 0), (0x1f4, position)]);
        // the corner getters return the buffer they were given
        for offset in [0x1d8u32, 0x1dc] {
            let at = slot(&e, refr, offset);
            e.register_double(at, |_, a| ret(a[1]));
        }
        e.register_double(0x0043_9e90, |_, a| ret(a[1]));
        e.register_double(0x0053_d280, |_, a| ret(a[1]));
        let out = e.mem.alloc(0xc);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_62b0, &args![refr, out]).u32(), out);
        });
        let maximum = calls_to(&log, slot(&e, refr, 0x1dc))[0][1];
        let minimum = calls_to(&log, slot(&e, refr, 0x1d8))[0][1];
        let additions = calls_to(&log, 0x0043_9e90);
        assert_eq!(additions.len(), 2);
        // the sum of the corners
        assert_eq!(additions[0][0], minimum);
        assert_eq!(additions[0][2], maximum);
        let sum = additions[0][1];
        // divided by the float at 010162c0
        let division = calls_to(&log, 0x0053_d280);
        assert_eq!(division.len(), 1);
        assert_eq!(division[0][0], sum);
        assert_eq!(f32::from_bits(division[0][2]), 2.0);
        // added to the position, into out
        assert_eq!(additions[1], vec![position, out, division[0][1]]);
    }

    #[test]
    fn face_gen_node_getters_need_an_actor_with_a_value_at_0x390() {
        let mut e = engine();
        let node = 0x6000;
        e.register_double(0x004a_ae30, move |_, _| ret(0x6100));
        let player_like = reference(&mut e, false, &[(0x390, 1)]);
        set_node(&mut e, player_like, node);
        // not an actor
        assert_eq!(e.call(0x0057_6330, &args![player_like, 0u32]).u32(), 0);
        assert_eq!(e.call(0x0057_6390, &args![player_like, 0u32]).u32(), 0);
        // an actor whose virtual +0x390 says 0
        let actor = reference(&mut e, true, &[(0x390, 0)]);
        set_node(&mut e, actor, node);
        assert_eq!(e.call(0x0057_6330, &args![actor, 0u32]).u32(), 0);
        // an actor that has it
        let actor = reference(&mut e, true, &[(0x390, 1)]);
        set_node(&mut e, actor, node);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_6330, &args![actor, 0u32]).u32(), 0x6100);
            assert_eq!(e.call(0x0057_6390, &args![actor, 0u32]).u32(), 0x6100);
        });
        assert_eq!(
            calls_to(&log, 0x004a_ae30),
            vec![vec![node, 0x0102_0408], vec![node, 0x0102_03f0]]
        );
    }

    #[test]
    fn virtual_forwarders_005763f0_and_00576420() {
        let mut e = engine();
        let face = object(&mut e, 0x40, &[(0x100, 0x1234)]);
        let refr = reference(&mut e, false, &[(0x1b0, 0x55), (0x1b4, face)]);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_63f0, &args![refr, 9u32]).u32(), 0x55);
            assert_eq!(e.call(0x0057_6420, &args![refr, 8u32]).u32(), 0x1234);
        });
        assert_eq!(calls_to(&log, slot(&e, refr, 0x1b0)), vec![vec![refr, 9]]);
        assert_eq!(calls_to(&log, slot(&e, refr, 0x1b4)), vec![vec![refr, 8]]);
        // no face data: zero
        set_slot(&mut e, refr, 0x1b4, 0);
        assert_eq!(e.call(0x0057_6420, &args![refr, 8u32]).u32(), 0);
    }

    #[test]
    fn clamp_to_ground_moves_the_reference_to_the_land_height() {
        let (mut e, refr, _) = location_setup();
        let position = e.mem.alloc(0xc);
        e.mem.set_f32(position, 10.0);
        e.mem.set_f32(position + 4, 20.0);
        e.mem.set_f32(position + 8, 30.0);
        set_slot(&mut e, refr, 0x1f4, position);
        // no cell: nothing
        assert!(!e.call(0x0057_6470, &args![refr]).bool());
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        // a cell without land height
        assert!(!e.call(0x0057_6470, &args![refr]).bool());
        e.register_double(0x0055_47c0, |e, a| {
            e.mem.set_f32(a[2], 4.5);
            ret(1)
        });
        assert!(e.call(0x0057_6470, &args![refr]).bool());
        assert_eq!(e.mem.f32(refr + 0x30), 10.0);
        assert_eq!(e.mem.f32(refr + 0x34), 20.0);
        assert_eq!(e.mem.f32(refr + 0x38), 4.5);
    }

    /// The two settings of `005764f0`: `first` is the byte for `011ca394`,
    /// `second` the one for `011ca3ec`.
    fn settings(e: &mut Engine, first: u8, second: u8) {
        let bytes = e.mem.alloc(8);
        e.mem.set_u8(bytes, first);
        e.mem.set_u8(bytes + 4, second);
        e.register_double(0x0040_8d60, move |_, a| {
            ret(if a[0] == 0x011c_a394 {
                bytes
            } else {
                bytes + 4
            })
        });
    }

    #[test]
    fn owner_change_005764f0_with_the_first_setting_always_changes_the_owner() {
        let mut e = engine();
        settings(&mut e, 1, 0);
        returns(&mut e, 0x0044_ddc0, 0x8000);
        let second = e.mem.alloc(0x10);
        e.mem.set_u8(second + 4, 9);
        let log = logged(&mut e, |e| {
            e.call(0x0057_64f0, &args![0x1111u32, second]);
        });
        assert_eq!(calls_to(&log, 0x0044_ddc0), vec![vec![0x1111]]);
        assert_eq!(calls_to(&log, 0x0049_02f0), vec![vec![0x8000, 0]]);
        assert_eq!(
            calls_to(&log, 0x0043_b370),
            vec![vec![0x8000, 0, 4], vec![0x8000, 1, 0x10]]
        );
        assert_eq!(e.mem.u8(second + 4), 0);
        assert!(calls_to(&log, 0x00c6_6ff0).is_empty());
    }

    #[test]
    fn owner_change_005764f0_without_settings_runs_00c66ff0() {
        let mut e = engine();
        settings(&mut e, 0, 0);
        let second = e.mem.alloc(0x10);
        let log = logged(&mut e, |e| {
            e.call(0x0057_64f0, &args![0x1111u32, second]);
        });
        assert_eq!(calls_to(&log, 0x00c6_6ff0), vec![vec![0x1111, second]]);
        assert!(calls_to(&log, 0x0049_02f0).is_empty());
        // the second setting without a first argument: the same
        settings(&mut e, 0, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0057_64f0, &args![0u32, second]);
        });
        assert_eq!(calls_to(&log, 0x00c6_6ff0), vec![vec![0, second]]);
    }

    #[test]
    fn owner_change_005764f0_follows_the_found_reference_with_the_second_setting() {
        let mut e = engine();
        settings(&mut e, 0, 1);
        returns(&mut e, 0x0044_ddc0, 0x8000);
        let found = e.mem.alloc(0x100);
        returns(&mut e, 0x0056_f930, found);
        let base = form(&mut e, 0x10, 0);
        e.mem.set_u32(found + 0x20, base);
        let second = e.mem.alloc(0x10);
        // a base form type that cannot hold the item: 00c66ff0
        let log = logged(&mut e, |e| {
            e.call(0x0057_64f0, &args![0x1111u32, second]);
        });
        assert_eq!(calls_to(&log, 0x0048_1f30), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, 0x00c6_6ff0).len(), 1);
        // a type it accepts: the owner is changed instead
        returns(&mut e, 0x0048_1f30, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0057_64f0, &args![0x1111u32, second]);
        });
        assert!(calls_to(&log, 0x00c6_6ff0).is_empty());
        assert_eq!(calls_to(&log, 0x0049_02f0), vec![vec![0x8000, 0]]);
        // type 0x1e whose base 0046f070 rejects: back to 00c66ff0
        e.mem.set_u8(base + 4, 0x1e);
        let log = logged(&mut e, |e| {
            e.call(0x0057_64f0, &args![0x1111u32, second]);
        });
        assert_eq!(calls_to(&log, 0x00c6_6ff0).len(), 1);
        assert_eq!(calls_to(&log, 0x0046_f070), vec![vec![base]]);
        // accepted by 0046f070
        returns(&mut e, 0x0046_f070, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0057_64f0, &args![0x1111u32, second]);
        });
        assert!(calls_to(&log, 0x00c6_6ff0).is_empty());
        // no found reference: 00c66ff0
        returns(&mut e, 0x0056_f930, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0057_64f0, &args![0x1111u32, second]);
        });
        assert_eq!(calls_to(&log, 0x00c6_6ff0).len(), 1);
    }

    #[test]
    fn flag_helpers_00576640_and_00576660_pass_their_bits() {
        let mut e = engine();
        let log = logged(&mut e, |e| {
            e.call(0x0057_6640, &args![0x5000u32, 1u32]);
            e.call(0x0057_6660, &args![0x5000u32, 0u32]);
        });
        assert_eq!(
            calls_to(&log, 0x0043_b370),
            vec![vec![0x5000, 1, 4], vec![0x5000, 0, 0x10]]
        );
    }

    #[test]
    fn remove_from_physics_without_a_node_only_resets_the_phantom_and_flags() {
        let mut e = engine();
        let refr = reference(&mut e, false, &[(0x1d0, 0)]);
        let tes = e.mem.alloc(0x40);
        e.set_global(GLOBAL_TES, tes);
        let loaded = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x64, loaded);
        let log = logged(&mut e, |e| {
            assert!(!e.call(0x0057_6680, &args![refr]).bool());
        });
        assert_eq!(
            calls_to(&log, 0x0045_3860),
            vec![vec![tes, 1], vec![tes, 0]]
        );
        assert_eq!(calls_to(&log, 0x0066_b0d0), vec![vec![loaded + 0x18, 0]]);
        assert_eq!(calls_to(&log, 0x0056_c880), vec![vec![refr, 1, 0]]);
        assert!(calls_to(&log, 0x00c6_9ee0).is_empty());
    }

    #[test]
    fn remove_from_physics_removes_the_node_and_runs_the_cell_hook_for_type_0xe() {
        let mut e = engine();
        let refr = reference(&mut e, false, &[(0x1d0, 0x4040)]);
        let tes = e.mem.alloc(0x40);
        e.set_global(GLOBAL_TES, tes);
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        let base = form(&mut e, 0xe, 0);
        e.mem.set_u32(refr + 0x20, base);
        let source = e.mem.alloc(0x100);
        returns(&mut e, 0x0045_43c0, source);
        returns(&mut e, 0x0045_cd60, 0x200);
        let log = logged(&mut e, |e| {
            assert!(e.call(0x0057_6680, &args![refr]).bool());
        });
        assert_eq!(calls_to(&log, 0x00c6_9ee0), vec![vec![0x4040, 1, 0]]);
        assert_eq!(calls_to(&log, 0x0045_43c0), vec![vec![cell], vec![cell]]);
        assert_eq!(calls_to(&log, 0x0045_cd60), vec![vec![source]]);
        assert_eq!(calls_to(&log, 0x0061_f890), vec![vec![0x200, refr, 1]]);
        // the traps were cleaned up before the removal
        let order = addresses(&log);
        let traps = order.iter().position(|a| *a == 0x0045_43c0).unwrap();
        let removal = order.iter().position(|a| *a == 0x00c6_9ee0).unwrap();
        assert!(traps < removal);
    }

    #[test]
    fn clean_up_traps_runs_the_three_listener_calls() {
        let mut e = engine();
        let refr = reference(&mut e, false, &[]);
        // no cell: nothing
        let log = logged(&mut e, |e| {
            e.call(0x0057_6760, &args![refr]);
        });
        assert!(calls_to(&log, 0x0062_0130).is_empty());
        let cell = e.mem.alloc(0x40);
        e.mem.set_u32(refr + 0x40, cell);
        // a trap source whose +0x20 is the listener plus 8
        let source = e.mem.alloc(0x40);
        let listener = e.mem.alloc(0x40);
        e.mem.set_u32(source + 0x20, listener + 8);
        returns(&mut e, 0x0045_43c0, source);
        let log = logged(&mut e, |e| {
            e.call(0x0057_6760, &args![refr]);
        });
        assert_eq!(calls_to(&log, 0x0062_0130), vec![vec![listener, refr]]);
        assert_eq!(calls_to(&log, 0x0062_de90), vec![vec![listener, refr]]);
        assert_eq!(calls_to(&log, 0x0062_05e0), vec![vec![listener, 0]]);
        // the source has no such part: nothing
        e.mem.set_u32(source + 0x20, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0057_6760, &args![refr]);
        });
        assert!(calls_to(&log, 0x0062_0130).is_empty());
    }

    #[test]
    fn trap_helpers_00576800_and_00576830_chain_through_00576870() {
        let mut e = engine();
        let holder = e.mem.alloc(0x10);
        // empty: 0, and 006205e0 gets 0
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_6830, &args![holder, 0x42u32]).u32(), 0);
            e.call(0x0057_6800, &args![holder, 0x42u32]);
        });
        assert!(calls_to(&log, 0x0057_6870).is_empty());
        assert_eq!(calls_to(&log, 0x0062_05e0), vec![vec![holder, 0]]);
        // with an inner object
        e.mem.set_u32(holder + 4, 0x7000);
        returns(&mut e, 0x0057_6870, 0x7100);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0057_6830, &args![holder, 0x42u32]).u32(), 0x7100);
            e.call(0x0057_6800, &args![holder, 0x42u32]);
        });
        assert_eq!(
            calls_to(&log, 0x0057_6870),
            vec![vec![0x7000, 0x42], vec![0x7000, 0x42]]
        );
        assert_eq!(calls_to(&log, 0x0062_05e0), vec![vec![holder, 0x7100]]);
    }
}
