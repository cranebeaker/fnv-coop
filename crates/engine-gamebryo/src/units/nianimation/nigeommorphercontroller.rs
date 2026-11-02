//! `NiAnimation/nigeommorphercontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiGeomMorpherController::ms_RTTI` (the class's `NiRTTI` record).
const NI_GEOM_MORPHER_CONTROLLER_RTTI: u32 = 0x011f_3728;

// Translated from 00a3afd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_geom_morpher_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_GEOM_MORPHER_CONTROLLER_RTTI)
}

// Translated from 00a3b530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a3af10`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x44 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_geom_morpher_controller_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a3_af10;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x44u32]);
    }
    this
}

/// The vtable of `NiGeomMorpherController` (RTTI
/// `.?AVNiGeomMorpherController@@`).
const NI_GEOM_MORPHER_CONTROLLER_VTABLE: u32 = 0x0109_6cec;
/// `NiGeomMorpherController` size (Xbox PDB and the allocations).
const NI_GEOM_MORPHER_CONTROLLER_SIZE: u32 = 0x44;

/// Fields as the PC code uses them (Xbox PDB names): `NiTimeController`'s
/// flags byte +0x08 (bit 5 forces the update), `m_fLoKeyTime` +0x14,
/// `m_fHiKeyTime` +0x18, the last scaled time +0x28 and `m_pkTarget`
/// +0x2c; then `m_uFlags` (16 bits) +0x34, `m_spMorphData` +0x38,
/// `m_pkInterpItems` +0x3c (8 bytes each: the interpolator pointer, then
/// the weight float), `m_bNeedPreDisplay` +0x40, `m_bIgnoreBaseTarget`
/// +0x41, `m_bAlwaysUpdate` +0x42 and `m_bBoundCalculated` +0x43.
const FLAGS: u32 = 0x08;
const LO_KEY_TIME: u32 = 0x14;
const HI_KEY_TIME: u32 = 0x18;
const SCALED_TIME: u32 = 0x28;
const TARGET: u32 = 0x2c;
const MORPHER_FLAGS: u32 = 0x34;
const MORPH_DATA: u32 = 0x38;
const INTERP_ITEMS: u32 = 0x3c;
const NEED_PRE_DISPLAY: u32 = 0x40;
const IGNORE_BASE_TARGET: u32 = 0x41;
const ALWAYS_UPDATE: u32 = 0x42;
const BOUND_CALCULATED: u32 = 0x43;
/// An `InterpItem`: 8 bytes, the weight at +4.
const INTERP_ITEM_SIZE: u32 = 8;
const ITEM_WEIGHT: u32 = 4;

/// `NiMorphData` fields: `m_uiNumTargets` +0x08, `m_uiNumVertsPerTarget`
/// +0x0c, `m_aMorphTargets` +0x10 (16-byte targets: vertices +0, name
/// +4, weight +8) and `m_bRelativeTargets` +0x14.
const DATA_NUM_TARGETS: u32 = 0x08;
const DATA_NUM_VERTS: u32 = 0x0c;
const DATA_TARGETS: u32 = 0x10;
const DATA_RELATIVE: u32 = 0x14;
const MORPH_TARGET_SIZE: u32 = 0x10;
const TARGET_WEIGHT: u32 = 0x08;
/// The geometry's data pointer (`NiGeometry::m_spModelData`, Xbox PDB,
/// +0xb8) and the data's fields the controller uses: the vertex count
/// (16 bits) +0x08, a 16-bit flags word +0x0c, the bound +0x10 (centre,
/// then radius at +0x1c), the vertex array +0x20 and the normal, colour
/// and texture arrays +0x24, +0x28, +0x2c; and the word at +0x04 that
/// `Update` compares with 1.
const GEOMETRY_DATA: u32 = 0xb8;
const GEOMETRY_VERTEX_COUNT: u32 = 0x08;
const GEOMETRY_DATA_FLAGS: u32 = 0x0c;
const GEOMETRY_BOUND: u32 = 0x10;
const GEOMETRY_VERTICES: u32 = 0x20;

/// Constants and globals read from the exe: the scaled time of a forced
/// update, the 16-bit "no index" value, the weight thresholds, the zero
/// centre of an empty bound, and two flag bytes and the pointer `Update`
/// and `CopyMembers` read.
const FORCED_UPDATE_TIME: u32 = 0x0109_6324;
const NO_INDEX: u32 = 0x0109_6320;
const WEIGHT_ABOVE: u32 = 0x0101_7d00;
const WEIGHT_BELOW: u32 = 0x0104_ffb8;
const EMPTY_BOUND_CENTER: u32 = 0x011f_426c;
const ALWAYS_UPDATE_CLONES: u32 = 0x011f_3724;
const UPDATE_TASKS_ENABLED: u32 = 0x011f_3725;
const UPDATE_TASK_OWNER: u32 = 0x011f_5b04;

/// Virtual slots: on the controller `0xc4` (interpolator at an index),
/// `0xc8` (set an interpolator, `(interpolator, index)`), `0xe0` (two
/// floats), `0xe8` (the vertex buffer), `0xec` (no arguments); on an
/// interpolator `0x68` (process clone) and `0x9c` (float update `(time,
/// target, &value)`); on an update task `0x94`; on the target `0x18`.
const SLOT_GET_INTERPOLATOR: u32 = 0xc4;
const SLOT_SET_INTERPOLATOR: u32 = 0xc8;
const SLOT_SET_KEY_TIMES: u32 = 0xe0;
const SLOT_VERTEX_BUFFER: u32 = 0xe8;
const SLOT_AFTER_DATA: u32 = 0xec;
const SLOT_PROCESS_CLONE: u32 = 0x68;
const SLOT_TASK_RUN: u32 = 0x94;
const SLOT_INTERPOLATOR_UPDATE_FLOAT: u32 = 0x9c;
const SLOT_TARGET_CHECK: u32 = 0x18;
const SLOT_DELETE_THIS: u32 = 0x04;

/// Callees by address (engine map names where it has them).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const MEMSET: u32 = 0x00ec_61c0;
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `00a7ec90` (unnamed; `cdecl`, `(buffer, weight, vertices, count)`):
/// adds a weighted target into the vertex buffer.
const ADD_WEIGHTED_TARGET: u32 = 0x00a7_ec90;
const MARK_AS_CHANGED: u32 = 0x00a6_7090;
const BOUND_COMPUTE_FROM_DATA: u32 = 0x00a7_ee30;
const BOUND_MERGE: u32 = 0x00a7_f3f0;
const TIME_CONTROLLER_PROCESS_CLONE: u32 = 0x00a6_d0c0;
/// `008d3ed0` (unnamed; `thiscall` on the controller): the interpolator
/// count the loops run to.
const INTERPOLATOR_COUNT: u32 = 0x008d_3ed0;
/// `00a41590` (`thiscall`, `LEA EAX,[ECX+4]`): a morph target's name.
const MORPH_TARGET_NAME: u32 = 0x00a4_1590;
/// `00cc0760` (`thiscall`, `MOV EAX,[ECX+0xc]`): a morph target's
/// interpolator.
const MORPH_TARGET_INTERPOLATOR: u32 = 0x00cc_0760;
const NI_STRICMP: u32 = 0x00aa_1360;
const TIME_CONTROLLER_DESTRUCT: u32 = 0x00a6_d290;
const GEOMETRY_DATA_REPLACE_DATA: u32 = 0x00a6_6bb0;
const DONT_DO_UPDATE: u32 = 0x00a3_6250;
/// `00714a00` (unit `interfacemanager.cpp`, no arguments; a bool),
/// `00a56f90` (no arguments; an update task or null), `00a56c60`
/// (`thiscall` on the task, `(controller, target)`) and `00a3a730`
/// (`thiscall` on the pointer at `011f5b04`, `(task, 2)`; a bool).
const UPDATE_TASKS_ALLOWED: u32 = 0x0071_4a00;
const NEW_UPDATE_TASK: u32 = 0x00a5_6f90;
const UPDATE_TASK_SET: u32 = 0x00a5_6c60;
const UPDATE_TASK_QUEUE: u32 = 0x00a3_a730;
const INTERP_CONTROLLER_LOAD_BINARY: u32 = 0x00a2_f910;
const INTERP_CONTROLLER_LINK_OBJECT: u32 = 0x00c9_a6b0;
const INTERP_CONTROLLER_COPY_MEMBERS: u32 = 0x00a2_f8a0;
const INTERP_CONTROLLER_CONSTRUCT: u32 = 0x00a2_f8c0;
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
const STREAM_READ_MULTIPLE_LINK_IDS: u32 = 0x00a6_4670;
const RESOLVE_LINK_ID: u32 = 0x00a6_4620;
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
const OBJECT_CLONE: u32 = 0x00a5_d3f0;
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// The `InterpItem` constructor (`00a3a940`, in this file), the address
/// handed to the vector constructor iterator.
const INTERP_ITEM_CONSTRUCT: u32 = 0x00a3_a940;

/// `NiStream::m_uiNifFileVersion` +0xd8 and `m_pkIstr` +0x24c (Xbox PDB
/// names; PC offsets), the stream's 16-bit word at +0x286 that old files
/// take the flags from, and the input stream's position (+4) and read
/// function (+8).
const STREAM_VERSION: u32 = 0xd8;
const STREAM_OLD_FLAGS: u32 = 0x286;
const STREAM_INPUT: u32 = 0x24c;
const BINARY_STREAM_POSITION: u32 = 0x4;
const BINARY_STREAM_READ: u32 = 0x8;

/// One read through the stream's read function into `buffer`: the word at
/// `slot` is set to `component` and passed as the component size; the
/// position advances by the bytes read.
fn stream_read(e: &mut Engine, stream: u32, buffer: u32, size: u32, component: u32, slot: u32) {
    let input = e.mem.u32(stream.wrapping_add(STREAM_INPUT));
    let read = e.mem.u32(input.wrapping_add(BINARY_STREAM_READ));
    e.mem.set_u32(slot, component);
    let bytes = e.call(read, &args![input, buffer, size, slot, 1u32]).u32();
    let position = e.mem.u32(input.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        input.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

/// The inline `NiPointer` release: `InterlockedDecrement` of the count at
/// `+4` and, when it reaches zero, the object's `DeleteThis`.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

/// The inline `NiPointer::operator=` for the pointer at `slot`.
fn assign_pointer(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old != new {
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(slot, new);
        if new != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![new.wrapping_add(4)]);
        }
    }
}

/// A new interpolator item array of `count` items behind a count word
/// (memory manager; the size all ones on overflow or carry), each item
/// constructed through the vector constructor iterator; null when the
/// allocation fails.
fn new_interp_items(e: &mut Engine, count: u32) -> u32 {
    let bytes = count.checked_mul(INTERP_ITEM_SIZE).unwrap_or(u32::MAX);
    let bytes = bytes.checked_add(4).unwrap_or(u32::MAX);
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, bytes])
        .u32();
    if block == 0 {
        return 0;
    }
    let items = block.wrapping_add(4);
    e.mem.set_u32(block, count);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![items, INTERP_ITEM_SIZE, count, INTERP_ITEM_CONSTRUCT],
    );
    items
}

/// Releases each item's interpolator, from the last, and deallocates the
/// array (from its count word).
fn delete_interp_items(e: &mut Engine, items: u32) {
    let block = items.wrapping_sub(4);
    let count = e.mem.u32(block);
    let mut item = items.wrapping_add(count.wrapping_mul(INTERP_ITEM_SIZE));
    let mut remaining = count as i32;
    loop {
        remaining = remaining.wrapping_sub(1);
        if remaining < 0 {
            break;
        }
        item = item.wrapping_sub(INTERP_ITEM_SIZE);
        let interpolator = e.mem.u32(item);
        if interpolator != 0 {
            release(e, interpolator);
        }
    }
    e.call(MEMORY_MANAGER_DEALLOCATE, &args![MEMORY_MANAGER, block]);
}

/// The morph target at `index` of the morph data, or 0 when `index` is not
/// below the target count.
fn morph_target(e: &Engine, data: u32, index: u32) -> u32 {
    if index < e.mem.u32(data.wrapping_add(DATA_NUM_TARGETS)) {
        e.mem
            .u32(data.wrapping_add(DATA_TARGETS))
            .wrapping_add(index.wrapping_mul(MORPH_TARGET_SIZE))
    } else {
        0
    }
}

/// The interpolator count (`008d3ed0`).
fn interpolator_count(e: &mut Engine, this: u32) -> u32 {
    e.call(INTERPOLATOR_COUNT, &args![this]).u32()
}

// Translated from 00a3a8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::GetInterpolator` (Xbox PDB): the interpolator
/// of item `index`.
pub fn ni_geom_morpher_controller_get_interpolator(e: &mut Engine, this: Ptr, index: u16) -> u32 {
    let items = e.mem.u32(this.addr().wrapping_add(INTERP_ITEMS));
    e.mem
        .u32(items.wrapping_add(u32::from(index) * INTERP_ITEM_SIZE))
}

// Translated from 00a3a8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::SetInterpolator` (Xbox PDB): stores
/// `interpolator` (counted; the old one released) as item `index`'s.
pub fn ni_geom_morpher_controller_set_interpolator(
    e: &mut Engine,
    this: Ptr,
    interpolator: u32,
    index: u16,
) {
    let items = e.mem.u32(this.addr().wrapping_add(INTERP_ITEMS));
    let slot = items.wrapping_add(u32::from(index) * INTERP_ITEM_SIZE);
    assign_pointer(e, slot, interpolator);
}

// Translated from 00a3a940 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The `InterpItem` constructor (Ghidra has no function here; the address
/// `LoadBinary` and `00a3b460` hand the vector constructor iterator): no
/// interpolator, weight 0.0. Returns `this`.
pub fn ni_geom_morpher_controller_interp_item_construct(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_f32(this.addr().wrapping_add(ITEM_WEIGHT), 0.0);
    e.mem.set_u32(this.addr(), 0);
    this
}

// Translated from 00a3aa10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::GenMorphInterp` (Xbox PDB): updates each item's
/// weight, then blends the morph targets into the vertex buffer (slot
/// `0xe8`). For each target: the first one of relative morph data gets
/// 0.0 when the base target is ignored and 1.0 otherwise; the others ask
/// their interpolator (slot `0xc4` gives it; its slot `0x9c` with the
/// scaled time, the target and the item's current weight in a stack word)
/// and keep the old weight when it fails or there is none. Then the buffer
/// is cleared (unless the base target is ignored) and every target whose
/// weight is `>=` the float at `01017d00` or `<=` the float at `0104ffb8`
/// is added (`00a7ec90`); if any was and the target's slot `0x18` answers
/// non-zero, its geometry data is marked changed (`MarkAsChanged(1)`). The
/// argument word is not read.
pub fn ni_geom_morpher_controller_gen_morph_interp(e: &mut Engine, this: Ptr, _unused_1: u32) {
    let this = this.addr();
    let buffer = e.vcall(this, SLOT_VERTEX_BUFFER, &args![]).u32();
    let target = e.mem.u32(this.wrapping_add(TARGET));
    let geometry = e.mem.u32(target.wrapping_add(GEOMETRY_DATA));
    let vertex_count = u32::from(e.mem.u16(geometry.wrapping_add(GEOMETRY_VERTEX_COUNT)));
    let data = e.mem.u32(this.wrapping_add(MORPH_DATA));
    let num_targets = e.mem.u32(data.wrapping_add(DATA_NUM_TARGETS));
    e.with_stack(4, |e, value| {
        let value = value.addr();
        for i in 0..num_targets {
            let interpolator = e.vcall(this, SLOT_GET_INTERPOLATOR, &args![i]).u32();
            let items = e.mem.u32(this.wrapping_add(INTERP_ITEMS));
            let mut weight = 0u32;
            if items != 0 {
                let at = items.wrapping_add(i * INTERP_ITEM_SIZE + ITEM_WEIGHT);
                weight = x87_quiet_bits(e.mem.u32(at));
            }
            e.mem.set_u32(value, weight);
            let data = e.mem.u32(this.wrapping_add(MORPH_DATA));
            let store = if i == 0 && e.mem.u8(data.wrapping_add(DATA_RELATIVE)) != 0 {
                if e.mem.u8(this.wrapping_add(IGNORE_BASE_TARGET)) != 0 {
                    e.mem.set_f32(value, 0.0);
                } else {
                    e.mem.set_f32(value, 1.0);
                }
                true
            } else if interpolator != 0 {
                let time = x87_quiet_bits(e.mem.u32(this.wrapping_add(SCALED_TIME)));
                let target = e.mem.u32(this.wrapping_add(TARGET));
                e.vcall(
                    interpolator,
                    SLOT_INTERPOLATOR_UPDATE_FLOAT,
                    &args![time, target, value],
                )
                .u8()
                    != 0
            } else {
                false
            };
            if store {
                let items = e.mem.u32(this.wrapping_add(INTERP_ITEMS));
                if items != 0 {
                    let at = items.wrapping_add(i * INTERP_ITEM_SIZE + ITEM_WEIGHT);
                    e.mem.set_u32(at, x87_quiet_bits(e.mem.u32(value)));
                }
            }
        }
    });
    if e.mem.u8(this.wrapping_add(IGNORE_BASE_TARGET)) == 0 {
        e.call(MEMSET, &args![buffer, 0u32, vertex_count.wrapping_mul(12)]);
    }
    let mut changed = false;
    for i in 0..num_targets {
        let items = e.mem.u32(this.wrapping_add(INTERP_ITEMS));
        if items == 0 {
            continue;
        }
        let weight_bits = x87_quiet_bits(
            e.mem
                .u32(items.wrapping_add(i * INTERP_ITEM_SIZE + ITEM_WEIGHT)),
        );
        let weight = x87_load32(weight_bits);
        let above = x87_load32(e.mem.u32(WEIGHT_ABOVE));
        let apply = weight >= above || {
            let below = x87_load32(e.mem.u32(WEIGHT_BELOW));
            weight <= below
        };
        if apply {
            let data = e.mem.u32(this.wrapping_add(MORPH_DATA));
            let targets = e.mem.u32(data.wrapping_add(DATA_TARGETS));
            let vertices = e.mem.u32(targets.wrapping_add(i * MORPH_TARGET_SIZE));
            e.call(
                ADD_WEIGHTED_TARGET,
                &args![buffer, weight_bits, vertices, vertex_count],
            );
            changed = true;
        }
    }
    if changed {
        let target = e.mem.u32(this.wrapping_add(TARGET));
        if e.vcall(target, SLOT_TARGET_CHECK, &args![]).u32() != 0 {
            let target = e.mem.u32(this.wrapping_add(TARGET));
            let geometry = e.mem.u32(target.wrapping_add(GEOMETRY_DATA));
            e.call(MARK_AS_CHANGED, &args![geometry, 1u32]);
        }
    }
}

// Translated from 00a3abe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::CalculateMorphBound` (Xbox PDB): the bound of
/// all morph targets (`NiBound::ComputeFromData` on the first, then on
/// each other merged in with `NiBound::Merge`; a target index past the
/// count gives a null target), or the zero centre (`011f426c`) and radius
/// 0.0 without targets, stored as the geometry data's bound (the radius
/// through the x87 stack).
pub fn ni_geom_morpher_controller_calculate_morph_bound(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let data = e.mem.u32(this.wrapping_add(MORPH_DATA));
    let count = e.mem.u32(data.wrapping_add(DATA_NUM_TARGETS));
    // Frame: the bound being built +0 (centre, radius), the next target's
    // bound +0x10.
    e.with_stack(0x20, |e, frame| {
        let bound = frame.addr();
        let other = bound + 0x10;
        if count == 0 {
            for k in 0..3 {
                let word = e.mem.u32(EMPTY_BOUND_CENTER + 4 * k);
                e.mem.set_u32(bound + 4 * k, word);
            }
            e.mem.set_f32(bound + 0xc, 0.0);
        } else {
            // The compiler kept the "index past the count" test for the
            // first target; with a non-zero count it always passes.
            let first = if count > 0 {
                e.mem.u32(data.wrapping_add(DATA_TARGETS))
            } else {
                0
            };
            let vertices = e.mem.u32(first);
            let vertex_count = e.mem.u32(data.wrapping_add(DATA_NUM_VERTS));
            e.call(
                BOUND_COMPUTE_FROM_DATA,
                &args![bound, vertex_count, vertices],
            );
            let mut i = 1u32;
            while i < count {
                let data = e.mem.u32(this.wrapping_add(MORPH_DATA));
                let target = morph_target(e, data, i);
                let vertices = e.mem.u32(target);
                let vertex_count = e.mem.u32(data.wrapping_add(DATA_NUM_VERTS));
                e.call(
                    BOUND_COMPUTE_FROM_DATA,
                    &args![other, vertex_count, vertices],
                );
                e.call(BOUND_MERGE, &args![bound, other]);
                i += 1;
            }
        }
        let target = e.mem.u32(this.wrapping_add(TARGET));
        let geometry = e
            .mem
            .u32(target.wrapping_add(GEOMETRY_DATA))
            .wrapping_add(GEOMETRY_BOUND);
        let x = e.mem.u32(bound);
        e.mem.set_u32(geometry, x);
        let y = e.mem.u32(bound + 4);
        e.mem.set_u32(geometry.wrapping_add(4), y);
        let radius = x87_quiet_bits(e.mem.u32(bound + 0xc));
        e.mem.set_u32(geometry.wrapping_add(0xc), radius);
        let z = e.mem.u32(bound + 8);
        e.mem.set_u32(geometry.wrapping_add(8), z);
    });
}

// Translated from 00a3acb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::ProcessClone` (Xbox PDB):
/// `NiTimeController::ProcessClone`, then each item's interpolator's
/// process clone (slot `0x68`) for the items up to the interpolator count
/// (a 16-bit index).
pub fn ni_geom_morpher_controller_process_clone(e: &mut Engine, this: Ptr, cloning: u32) {
    e.call(TIME_CONTROLLER_PROCESS_CLONE, &args![this, cloning]);
    let this = this.addr();
    let mut counter = 0u32;
    if interpolator_count(e, this) == 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let items = e.mem.u32(this.wrapping_add(INTERP_ITEMS));
        let interpolator = e.mem.u32(items.wrapping_add(index * INTERP_ITEM_SIZE));
        if interpolator != 0 {
            e.vcall(interpolator, SLOT_PROCESS_CLONE, &args![cloning]);
        }
        counter = counter.wrapping_add(1);
        index = counter & 0xffff;
        if index >= interpolator_count(e, this) {
            break;
        }
    }
}

// Translated from 00a3add0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::GetInterpolatorIndex` (Xbox PDB): the index of
/// the morph target named `name` (`NiStricmp`), or the "no index" word at
/// `01096320` when `name` is null or no target matches.
pub fn ni_geom_morpher_controller_get_interpolator_index(
    e: &mut Engine,
    this: Ptr,
    name: u32,
) -> u16 {
    let this = this.addr();
    if name != 0 {
        let mut counter = 0u32;
        if interpolator_count(e, this) != 0 {
            let mut index = 0u32;
            loop {
                let data = e.mem.u32(this.wrapping_add(MORPH_DATA));
                let target = morph_target(e, data, index);
                let target_name = e.call(MORPH_TARGET_NAME, &args![target]).u32();
                let text = e.mem.u32(target_name);
                if e.call(NI_STRICMP, &args![text, name]).u32() == 0 {
                    return counter as u16;
                }
                counter = counter.wrapping_add(1);
                index = counter & 0xffff;
                if index >= interpolator_count(e, this) {
                    break;
                }
            }
        }
    }
    e.mem.u16(NO_INDEX)
}

// Translated from 00a3aeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::GetInterpolatorIndexFx` (Xbox PDB): the index
/// of the morph target whose name handle equals the fixed string at
/// `name`, or the "no index" word at `01096320`.
pub fn ni_geom_morpher_controller_get_interpolator_index_fx(
    e: &mut Engine,
    this: Ptr,
    name: Ptr,
) -> u16 {
    let this = this.addr();
    let mut counter = 0u32;
    if interpolator_count(e, this) != 0 {
        let mut index = 0u32;
        loop {
            let data = e.mem.u32(this.wrapping_add(MORPH_DATA));
            let target = morph_target(e, data, index);
            let target_name = e.call(MORPH_TARGET_NAME, &args![target]).u32();
            if e.mem.u32(target_name) == e.mem.u32(name.addr()) {
                return counter as u16;
            }
            counter = counter.wrapping_add(1);
            index = counter & 0xffff;
            if index >= interpolator_count(e, this) {
                break;
            }
        }
    }
    e.mem.u16(NO_INDEX)
}

// Translated from 00a3af10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiGeomMorpherController` destructor body
/// (the deleting destructor `00a3b530` runs it). Stores the vtable,
/// releases the morph data and clears it, releases every item's
/// interpolator and frees the item array, releases the (now null) morph
/// data pointer again as the member destructor does, then jumps to the
/// `NiTimeController` destructor body (`00a6d290`).
pub fn fn_00a3af10(e: &mut Engine, this: Ptr) {
    let object = this.addr();
    e.mem.set_u32(object, NI_GEOM_MORPHER_CONTROLLER_VTABLE);
    let data = e.mem.u32(object.wrapping_add(MORPH_DATA));
    if data != 0 {
        release(e, data);
        e.mem.set_u32(object.wrapping_add(MORPH_DATA), 0);
    }
    let items = e.mem.u32(object.wrapping_add(INTERP_ITEMS));
    if items != 0 {
        delete_interp_items(e, items);
    }
    let data = e.mem.u32(object.wrapping_add(MORPH_DATA));
    if data != 0 {
        release(e, data);
    }
    e.call(TIME_CONTROLLER_DESTRUCT, &args![this]);
}

// Translated from 00a3b010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::GetVertexData` (Xbox PDB): the target's
/// geometry data vertex array; when it has none, a new one (12 bytes per
/// vertex, memory manager) is given to the data with
/// `NiGeometryData::ReplaceData` (keeping its normals, colours, texture
/// sets and the flag bits 0x0001 and 0xf000) and returned. 0 without a
/// target.
pub fn ni_geom_morpher_controller_get_vertex_data(e: &mut Engine, this: Ptr) -> u32 {
    let target = e.mem.u32(this.addr().wrapping_add(TARGET));
    if target == 0 {
        return 0;
    }
    let geometry = e.mem.u32(target.wrapping_add(GEOMETRY_DATA));
    let vertices = e.mem.u32(geometry.wrapping_add(GEOMETRY_VERTICES));
    if vertices != 0 {
        return vertices;
    }
    let count = e.mem.u16(geometry.wrapping_add(GEOMETRY_VERTEX_COUNT));
    let bytes = u32::from(count).checked_mul(12).unwrap_or(u32::MAX);
    let vertices = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER, bytes])
        .u32();
    let geometry = e.mem.u32(target.wrapping_add(GEOMETRY_DATA));
    let flags = u32::from(e.mem.u16(geometry.wrapping_add(GEOMETRY_DATA_FLAGS)));
    let normals = e.mem.u32(geometry.wrapping_add(0x24));
    let colors = e.mem.u32(geometry.wrapping_add(0x28));
    let textures = e.mem.u32(geometry.wrapping_add(0x2c));
    e.call(
        GEOMETRY_DATA_REPLACE_DATA,
        &args![
            geometry,
            u32::from(count),
            vertices,
            normals,
            colors,
            textures,
            flags & 1,
            flags & 0xf000
        ],
    );
    vertices
}

// Translated from 00a3b080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::Update` (Xbox PDB). Nothing without morph
/// data. When the byte at `update + 5` is set, the byte at `update + 7` is
/// set and nothing else happens. A forced update (flags bit 5) marks the
/// pre-display flag and sets the scaled time to the float at `01096324`;
/// otherwise the flag is marked when `DontDoUpdate` (on the first float of
/// `update`) says to run or the controller always updates. With a target
/// and the flag: `GenMorphInterp`, then, when the byte at `011f3725` is
/// set, `00714a00` agrees and the geometry data's word at +4 is 1, a new
/// update task (`00a56f90`) is set up for the controller and target
/// (`00a56c60`) and queued (`00a3a730` on the pointer at `011f5b04`, with
/// 2); when queueing fails the task runs its slot `0x94`.
pub fn ni_geom_morpher_controller_update(e: &mut Engine, this: Ptr, update: Ptr) {
    let this = this.addr();
    if e.mem.u32(this.wrapping_add(MORPH_DATA)) == 0 {
        return;
    }
    let update = update.addr();
    if e.mem.u8(update.wrapping_add(5)) != 0 {
        e.mem.set_u8(update.wrapping_add(7), 1);
        return;
    }
    if (e.mem.u8(this.wrapping_add(FLAGS)) >> 5) & 1 != 0 {
        e.mem.set_u8(this.wrapping_add(NEED_PRE_DISPLAY), 1);
        let forced = e.mem.u32(FORCED_UPDATE_TIME);
        e.mem
            .set_u32(this.wrapping_add(SCALED_TIME), x87_quiet_bits(forced));
    } else {
        let time = x87_quiet_bits(e.mem.u32(update));
        if e.call(DONT_DO_UPDATE, &args![this, time]).u8() == 0
            || e.mem.u8(this.wrapping_add(ALWAYS_UPDATE)) != 0
        {
            e.mem.set_u8(this.wrapping_add(NEED_PRE_DISPLAY), 1);
        }
    }
    if e.mem.u32(this.wrapping_add(TARGET)) == 0 {
        return;
    }
    if e.mem.u8(this.wrapping_add(NEED_PRE_DISPLAY)) == 0 {
        return;
    }
    let time = x87_quiet_bits(e.mem.u32(this.wrapping_add(SCALED_TIME)));
    ni_geom_morpher_controller_gen_morph_interp(e, Ptr::new(this), time);
    if e.mem.u8(UPDATE_TASKS_ENABLED) == 0 {
        return;
    }
    if e.call(UPDATE_TASKS_ALLOWED, &args![]).u8() == 0 {
        return;
    }
    let target = e.mem.u32(this.wrapping_add(TARGET));
    let geometry = e.mem.u32(target.wrapping_add(GEOMETRY_DATA));
    if e.mem.u32(geometry.wrapping_add(4)) != 1 {
        return;
    }
    let task = e.call(NEW_UPDATE_TASK, &args![]).u32();
    if task == 0 {
        return;
    }
    let target = e.mem.u32(this.wrapping_add(TARGET));
    e.call(UPDATE_TASK_SET, &args![task, this, target]);
    let owner = e.mem.u32(UPDATE_TASK_OWNER);
    if e.call(UPDATE_TASK_QUEUE, &args![owner, task, 2u32]).u8() == 0 {
        e.vcall(task, SLOT_TASK_RUN, &args![]);
    }
}

// Translated from 00a3b150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::LoadBinary` (Xbox PDB):
/// `NiInterpController::LoadBinary`; the flags word (from the stream's
/// word at +0x286, bits 5-10, before file version `0x0a000102`; read, 2
/// bytes, after); the morph data's link ID (`00a645b0`); from `0x04000002`
/// the always-update byte. From `0x0a010068`: before `0x14010003` the
/// interpolators' link IDs (`ReadMultipleLinkIDs`); from then on the item
/// count (4 bytes), a new item array, and per item (a 16-bit index) its
/// interpolator (`ResolveLinkID`, counted) and weight (4 bytes).
///
/// Stack words follow the exe: two locals (`L1` at -8 and `L2` at -4 from
/// the return address) and the argument's own word.
pub fn ni_geom_morpher_controller_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(INTERP_CONTROLLER_LOAD_BINARY, &args![this, stream]);
    let this = this.addr();
    let stream = stream.addr();
    // Frame: L1 +0, L2 +4, the argument word +8.
    e.with_stack(12, |e, frame| {
        let l1 = frame.addr();
        let l2 = l1 + 4;
        let argument = l1 + 8;
        e.mem.set_u32(argument, stream);
        if e.mem.u32(stream.wrapping_add(STREAM_VERSION)) < 0x0a00_0102 {
            let old = e.mem.u16(stream.wrapping_add(STREAM_OLD_FLAGS));
            e.mem
                .set_u16(this.wrapping_add(MORPHER_FLAGS), (old >> 5) & 0x3f);
        } else {
            stream_read(e, stream, this.wrapping_add(MORPHER_FLAGS), 2, 2, argument);
        }
        e.call(STREAM_READ_LINK_ID, &args![stream]);
        if e.mem.u32(stream.wrapping_add(STREAM_VERSION)) >= 0x0400_0002 {
            stream_read(e, stream, argument, 1, 1, l1);
            let always = u8::from(e.mem.u8(argument) != 0);
            e.mem.set_u8(this.wrapping_add(ALWAYS_UPDATE), always);
        }
        let version = e.mem.u32(stream.wrapping_add(STREAM_VERSION));
        if version < 0x0a01_0068 {
            return;
        }
        if version < 0x1401_0003 {
            e.call(STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]);
            return;
        }
        stream_read(e, stream, argument, 4, 4, l1);
        let count = e.mem.u32(argument);
        let items = new_interp_items(e, count);
        e.mem.set_u32(this.wrapping_add(INTERP_ITEMS), items);
        if e.mem.u32(argument) == 0 {
            return;
        }
        let mut counter = 0u32;
        let mut index = 0u32;
        loop {
            let interpolator = e.call(RESOLVE_LINK_ID, &args![stream]).u32();
            let offset = index * INTERP_ITEM_SIZE;
            e.mem.set_u32(l2, offset);
            let items = e.mem.u32(this.wrapping_add(INTERP_ITEMS));
            assign_pointer(e, items.wrapping_add(offset), interpolator);
            let items = e.mem.u32(this.wrapping_add(INTERP_ITEMS));
            let weight = items.wrapping_add(offset).wrapping_add(ITEM_WEIGHT);
            stream_read(e, stream, weight, 4, 4, l2);
            counter = counter.wrapping_add(1);
            index = counter & 0xffff;
            if index >= e.mem.u32(argument) {
                break;
            }
        }
    });
}

// Translated from 00a3b460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigeommorphercontroller.cpp`): rebuilds
/// the item array. The old items' interpolators are released and the array
/// freed; the pointer is cleared; with morph data that has targets a new
/// array of that many items (memory manager, items constructed by
/// `00a3a940`) takes its place.
pub fn fn_00a3b460(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let items = e.mem.u32(this.wrapping_add(INTERP_ITEMS));
    if items != 0 {
        delete_interp_items(e, items);
    }
    let data = e.mem.u32(this.wrapping_add(MORPH_DATA));
    e.mem.set_u32(this.wrapping_add(INTERP_ITEMS), 0);
    if data == 0 {
        return;
    }
    let count = e.mem.u32(data.wrapping_add(DATA_NUM_TARGETS));
    if count == 0 {
        return;
    }
    let items = new_interp_items(e, count);
    e.mem.set_u32(this.wrapping_add(INTERP_ITEMS), items);
}

/// The constructor body `CreateObject` and `CreateClone` run on a new
/// block: the `NiInterpController` constructor, the vtable, no morph data
/// and no items, flags 0 and the four bools false.
fn construct_in(e: &mut Engine, object: u32) {
    e.call(INTERP_CONTROLLER_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, NI_GEOM_MORPHER_CONTROLLER_VTABLE);
    e.mem.set_u32(object.wrapping_add(MORPH_DATA), 0);
    e.mem.set_u16(object.wrapping_add(MORPHER_FLAGS), 0);
    e.mem.set_u32(object.wrapping_add(INTERP_ITEMS), 0);
    for flag in [
        NEED_PRE_DISPLAY,
        IGNORE_BASE_TARGET,
        ALWAYS_UPDATE,
        BOUND_CALCULATED,
    ] {
        e.mem.set_u8(object.wrapping_add(flag), 0);
    }
}

// Translated from 00a3b550 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::CreateObject` (registered through
/// `NiStream::RegisterLoader` for `"NiGeomMorpherController"` at
/// `00a2daaf`; Ghidra has no function here): allocates 0x44 bytes and,
/// when that succeeds, constructs an empty controller in it. Returns it, or
/// 0.
pub fn ni_geom_morpher_controller_create_object(e: &mut Engine) -> Ptr {
    let object = e
        .call(NI_ALLOCATE, &args![NI_GEOM_MORPHER_CONTROLLER_SIZE])
        .u32();
    if object == 0 {
        return Ptr::new(0);
    }
    construct_in(e, object);
    Ptr::new(object)
}

// Translated from 00a3b5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::LinkObject` (Xbox PDB):
/// `NiInterpController::LinkObject`, the morph data from its link ID
/// (counted). Before file version `0x14010003` the items are rebuilt
/// (`00a3b460`) and take the morph targets' weights. Then the
/// interpolators: before `0x0a010068`, with morph data, each target's
/// (`00cc0760`) is set through slot `0xc8`; otherwise, before
/// `0x14010003`, each linked object (`GetNumberOfLinkIDs`,
/// `GetObjectFromLinkID`) is. Finally, with morph data, files up to
/// `0x0401000c` pass the key times to slot `0xe0`.
pub fn ni_geom_morpher_controller_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(INTERP_CONTROLLER_LINK_OBJECT, &args![this, stream]);
    let object = this.addr();
    let stream = stream.addr();
    let data = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    assign_pointer(e, object.wrapping_add(MORPH_DATA), data);
    if e.mem.u32(stream.wrapping_add(STREAM_VERSION)) < 0x1401_0003 {
        fn_00a3b460(e, this);
        let mut i = 0u32;
        while i < interpolator_count(e, object) {
            let data = e.mem.u32(object.wrapping_add(MORPH_DATA));
            let target = morph_target(e, data, i);
            let weight = x87_quiet_bits(e.mem.u32(target.wrapping_add(TARGET_WEIGHT)));
            let items = e.mem.u32(object.wrapping_add(INTERP_ITEMS));
            e.mem.set_u32(
                items.wrapping_add(i * INTERP_ITEM_SIZE + ITEM_WEIGHT),
                weight,
            );
            i += 1;
        }
    }
    let version = e.mem.u32(stream.wrapping_add(STREAM_VERSION));
    let data = e.mem.u32(object.wrapping_add(MORPH_DATA));
    if version < 0x0a01_0068 && data != 0 {
        let count = e.mem.u32(data.wrapping_add(DATA_NUM_TARGETS));
        for i in 0..count {
            let data = e.mem.u32(object.wrapping_add(MORPH_DATA));
            let target = morph_target(e, data, i);
            let interpolator = e.call(MORPH_TARGET_INTERPOLATOR, &args![target]).u32();
            e.vcall(object, SLOT_SET_INTERPOLATOR, &args![interpolator, i]);
        }
    } else if version < 0x1401_0003 {
        let count = e.call(GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
        for i in 0..count {
            let interpolator = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
            e.vcall(object, SLOT_SET_INTERPOLATOR, &args![interpolator, i]);
        }
    }
    if e.mem.u32(object.wrapping_add(MORPH_DATA)) != 0
        && e.mem.u32(stream.wrapping_add(STREAM_VERSION)) <= 0x0401_000c
    {
        let lo = x87_quiet_bits(e.mem.u32(object.wrapping_add(LO_KEY_TIME)));
        let hi = x87_quiet_bits(e.mem.u32(object.wrapping_add(HI_KEY_TIME)));
        e.vcall(object, SLOT_SET_KEY_TIMES, &args![lo, hi]);
    }
}

// Translated from 00a3ba10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigeommorphercontroller.cpp`): sets the
/// morph data (counted; the old one released), rebuilds the items
/// (`00a3b460`) and, when the low byte of `notify` is set, runs slot
/// `0xec`.
pub fn fn_00a3ba10(e: &mut Engine, this: Ptr, data: u32, notify: u32) {
    assign_pointer(e, this.addr().wrapping_add(MORPH_DATA), data);
    fn_00a3b460(e, this);
    if notify & 0xff != 0 {
        e.vcall(this.addr(), SLOT_AFTER_DATA, &args![]);
    }
}

// Translated from 00a3ba70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `NiGeomMorpherController`'s member copy into
/// `destination` for a clone made by `cloning` (the function `CreateClone`
/// runs). The `NiInterpController` copy (`00a2f8a0`), the flags word, the
/// morph data (`00a3ba10`, notifying when the destination has a target),
/// the always-update bytes of both when the byte at `011f3724` is set, and
/// per item (a 16-bit index, up to the interpolator count) a clone of the
/// interpolator (`00a5d3f0`, counted into the destination's item) and the
/// weight.
pub fn ni_geom_morpher_controller_copy_members(
    e: &mut Engine,
    this: Ptr,
    destination: Ptr,
    cloning: u32,
) {
    e.call(
        INTERP_CONTROLLER_COPY_MEMBERS,
        &args![this, destination, cloning],
    );
    let source = this.addr();
    let target_controller = destination.addr();
    let flags = e.mem.u16(source.wrapping_add(MORPHER_FLAGS));
    e.mem
        .set_u16(target_controller.wrapping_add(MORPHER_FLAGS), flags);
    let data = e.mem.u32(source.wrapping_add(MORPH_DATA));
    if data != 0 {
        let notify = u32::from(e.mem.u32(target_controller.wrapping_add(TARGET)) != 0);
        fn_00a3ba10(e, destination, data, notify);
    }
    if e.mem.u8(ALWAYS_UPDATE_CLONES) != 0 {
        e.mem.set_u8(source.wrapping_add(ALWAYS_UPDATE), 1);
        e.mem
            .set_u8(target_controller.wrapping_add(ALWAYS_UPDATE), 1);
    }
    let mut counter = 0u32;
    if interpolator_count(e, source) == 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let offset = index * INTERP_ITEM_SIZE;
        let item = e
            .mem
            .u32(source.wrapping_add(INTERP_ITEMS))
            .wrapping_add(offset);
        let interpolator = e.mem.u32(item);
        if interpolator != 0 {
            let clone = e.call(OBJECT_CLONE, &args![interpolator, cloning]).u32();
            let items = e.mem.u32(target_controller.wrapping_add(INTERP_ITEMS));
            assign_pointer(e, items.wrapping_add(offset), clone);
        }
        let weight = x87_quiet_bits(e.mem.u32(item.wrapping_add(ITEM_WEIGHT)));
        let items = e.mem.u32(target_controller.wrapping_add(INTERP_ITEMS));
        e.mem
            .set_u32(items.wrapping_add(offset + ITEM_WEIGHT), weight);
        counter = counter.wrapping_add(1);
        index = counter & 0xffff;
        if index >= interpolator_count(e, source) {
            break;
        }
    }
}

// Translated from 00a3bb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::CreateClone` (Xbox PDB): allocates 0x44 bytes
/// and, when that succeeds, constructs an empty controller in it; then the
/// member copy into it, and returns it. When the allocation fails the
/// member copy still runs, into address 0, and 0 is returned.
pub fn ni_geom_morpher_controller_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let object = e
        .call(NI_ALLOCATE, &args![NI_GEOM_MORPHER_CONTROLLER_SIZE])
        .u32();
    if object == 0 {
        ni_geom_morpher_controller_copy_members(e, this, Ptr::new(0), cloning);
        return Ptr::new(0);
    }
    construct_in(e, object);
    ni_geom_morpher_controller_copy_members(e, this, Ptr::new(object), cloning);
    Ptr::new(object)
}

/// The interpolator count virtual slot (`GetInterpolatorCount`, `0xb4`) and the slots run on
/// other objects: an interpolator's `GetActiveTimeRange` (`0xc0`, two float pointers) and
/// `GuaranteeTimeRange` (`0xc4`, two floats), `RegisterStreamables` (`0x54`) and `IsEqual`
/// (`0x5c`) of any object, the geometry data's `0xa0` (no arguments), the stream's
/// `SaveLinkID` (`0x30`) and the update-task owner's `0xa0` (four words).
const SLOT_INTERPOLATOR_COUNT: u32 = 0xb4;
const SLOT_GET_ACTIVE_TIME_RANGE: u32 = 0xc0;
const SLOT_GUARANTEE_TIME_RANGE: u32 = 0xc4;
const SLOT_REGISTER_STREAMABLES: u32 = 0x54;
const SLOT_IS_EQUAL: u32 = 0x5c;
const SLOT_GEOMETRY_DATA_A0: u32 = 0xa0;
const SLOT_SAVE_LINK_ID: u32 = 0x30;
const SLOT_OWNER_QUEUE: u32 = 0xa0;
/// `GetRTTI` (virtual slot `0x08`); an `NiRTTI` record's base-class pointer is at +4.
const SLOT_GET_RTTI: u32 = 0x08;
/// `NiRTTI` records: `NiGeometry` (`011f4acc`), `NiBlendFloatInterpolator` (`011f3e58`) and
/// `NiFloatInterpolator` (`011f3738`), as their static initializers name them.
const NI_GEOMETRY_RTTI: u32 = 0x011f_4acc;
const NI_BLEND_FLOAT_INTERPOLATOR_RTTI: u32 = 0x011f_3e58;
const NI_FLOAT_INTERPOLATOR_RTTI: u32 = 0x011f_3738;
/// `NiTimeController::SetTarget` (`00a6d450`, thiscall, the target).
const TIME_CONTROLLER_SET_TARGET: u32 = 0x00a6_d450;
/// The base class's `RegisterStreamables`, `IsEqual` and `SaveBinary` (`00a2f8b0`,
/// `00c283f0`, `00c27120`; thiscall, one word; the engine map names them after
/// `NiPSysResetOnLoopCtlr`, folded code).
const BASE_REGISTER_STREAMABLES: u32 = 0x00a2_f8b0;
const BASE_IS_EQUAL: u32 = 0x00c2_83f0;
const BASE_SAVE_BINARY: u32 = 0x00c2_7120;
/// `NiFloatInterpolator::NiFloatInterpolator(float)` (`00a3c390`) and
/// `NiFloatInterpolator::SetPoseValue` (`00a3c500`), thiscall with one float.
const FLOAT_INTERPOLATOR_CONSTRUCT_VALUE: u32 = 0x00a3_c390;
const FLOAT_INTERPOLATOR_SET_POSE_VALUE: u32 = 0x00a3_c500;
/// `00653290` (thiscall on an object, an `NiRTTI` record): the object when it is of that
/// class (`006532c0`), else 0.
const DYNAMIC_CAST: u32 = 0x0065_3290;
/// The floats `ResetTimeExtrema` starts the key times from (`01096ce0` for the low one,
/// `01015f5c` for the high one).
const EXTREMA_START_LO: u32 = 0x0109_6ce0;
const EXTREMA_START_HI: u32 = 0x0101_5f5c;
/// `NiStream::m_pkOstr` (Xbox PDB name, PC offset +0x250) and the output stream's write
/// function (+0xc, `cdecl` like the read function).
const STREAM_OUTPUT: u32 = 0x250;
const BINARY_STREAM_WRITE: u32 = 0xc;
/// `GetViewerStrings`: `NiGetViewerString` (`00a7d330`, one word), its unsigned form
/// (`00a7d510`, label and value), the array's resize and `SetAt` (thiscall), `sprintf`
/// (`00ec623a`), the allocation (`00aa1070`, cdecl, size) and `strcpy_s` (`00ec65a6`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
const VIEWER_STRINGS_GROW: u32 = 0x0096_ad30;
const VIEWER_STRINGS_SET_AT: u32 = 0x0096_ae90;
const SPRINTF: u32 = 0x00ec_623a;
const NI_ALLOC: u32 = 0x00aa_1070;
const STRCPY_S: u32 = 0x00ec_65a6;
/// The labels `"NumTargets"` and `"NumVertsPerTarget"` and the formats `"%d %s"` and
/// `"%d %s = %0.2f"`.
const TEXT_NUM_TARGETS: u32 = 0x0109_6e00;
const TEXT_NUM_VERTS_PER_TARGET: u32 = 0x0109_6dec;
const FORMAT_INDEX_NAME: u32 = 0x0107_29ac;
const FORMAT_INDEX_NAME_WEIGHT: u32 = 0x0109_6ddc;
/// The value fields the viewer strings show: `NiBlendFloatInterpolator`'s float +0x30 and
/// `NiFloatInterpolator`'s float +0xc.
const BLEND_FLOAT_VALUE: u32 = 0x30;
const FLOAT_VALUE: u32 = 0xc;

/// One write through the stream's output stream's write function from `buffer`: the word at
/// `slot` is set to `component` and passed as the component size; the position advances by
/// the bytes written.
fn stream_write(e: &mut Engine, stream: u32, buffer: u32, size: u32, component: u32, slot: u32) {
    let output = e.mem.u32(stream.wrapping_add(STREAM_OUTPUT));
    let write = e.mem.u32(output.wrapping_add(BINARY_STREAM_WRITE));
    e.mem.set_u32(slot, component);
    let bytes = e
        .call(write, &args![output, buffer, size, slot, 1u32])
        .u32();
    let position = e.mem.u32(output.wrapping_add(BINARY_STREAM_POSITION));
    e.mem.set_u32(
        output.wrapping_add(BINARY_STREAM_POSITION),
        position.wrapping_add(bytes),
    );
}

/// Whether the object's class (`GetRTTI`, slot 8) or one of its bases is `rtti`.
fn is_kind_of(e: &mut Engine, object: u32, rtti: u32) -> bool {
    let mut record = e.vcall(object, SLOT_GET_RTTI, &args![]).u32();
    while record != 0 {
        if record == rtti {
            return true;
        }
        record = e.mem.u32(record.wrapping_add(4));
    }
    false
}

/// The viewer-string array append the compiler inlined with the string in the stack word
/// `slot`: grows the array (16-bit size +0xA, capacity +8, grow-by +0xE) when full, then
/// `SetAt` at the old size with the slot's address.
fn add_viewer_string(e: &mut Engine, strings: u32, slot: u32) {
    let size = u32::from(e.mem.u16(strings.wrapping_add(0xa)));
    if size >= u32::from(e.mem.u16(strings.wrapping_add(8))) {
        let step = u32::from(e.mem.u16(strings.wrapping_add(0xe)));
        e.call(VIEWER_STRINGS_GROW, &args![strings, step + size]);
    }
    e.call(VIEWER_STRINGS_SET_AT, &args![strings, size, slot]);
}

/// The inline `NiTPrimitiveArray::SetAtGrow` of `string` at the array's size (data +4, 16-bit
/// size +0xA, capacity +8, used count +0xC, grow-by +0xE): grows when full; a slot at or past
/// the (new) size makes the size one more, the used count counting a non-null string; inside
/// it the used count follows the change between null and non-null; then the slot is set.
fn set_at_grow(e: &mut Engine, strings: u32, string: u32) {
    let index = u32::from(e.mem.u16(strings.wrapping_add(0xa)));
    if index >= u32::from(e.mem.u16(strings.wrapping_add(8))) {
        let step = u32::from(e.mem.u16(strings.wrapping_add(0xe)));
        e.call(VIEWER_STRINGS_GROW, &args![strings, step + index]);
    }
    let used = strings.wrapping_add(0xc);
    if index >= u32::from(e.mem.u16(strings.wrapping_add(0xa))) {
        e.mem.set_u16(strings.wrapping_add(0xa), (index + 1) as u16);
        if string != 0 {
            e.mem.set_u16(used, e.mem.u16(used).wrapping_add(1));
        }
    } else if string != 0 {
        let data = e.mem.u32(strings.wrapping_add(4));
        if e.mem.u32(data.wrapping_add(index * 4)) == 0 {
            e.mem.set_u16(used, e.mem.u16(used).wrapping_add(1));
        }
    } else {
        let data = e.mem.u32(strings.wrapping_add(4));
        if e.mem.u32(data.wrapping_add(index * 4)) != 0 {
            e.mem.set_u16(used, e.mem.u16(used).wrapping_add(0xffff));
        }
    }
    let data = e.mem.u32(strings.wrapping_add(4));
    e.mem.set_u32(data.wrapping_add(index * 4), string);
}

// Translated from 00a3a730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigeommorphercontroller.cpp`; thiscall, two words): the
/// object's virtual slot `0xa0` with `(0, task, priority, 1)`; its result is returned.
/// `Update` queues its update task through it on the object at `011f5b04`.
pub fn fn_00a3a730(e: &mut Engine, this: Ptr, task: u32, priority: u32) -> u32 {
    e.vcall(
        this.addr(),
        SLOT_OWNER_QUEUE,
        &args![0u32, task, priority, 1u32],
    )
    .u32()
}

// Translated from 00a3a750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::SetTarget` (Xbox PDB): when the target's slot `0x18` answers 0
/// the controller's target is cleared (`NiTimeController::SetTarget(0)`); otherwise it is set
/// and, with morph data, slot `0xec` runs and the bound-calculated flag (+0x43) is set.
pub fn ni_geom_morpher_controller_set_target(e: &mut Engine, this: Ptr, target: Ptr) {
    if e.vcall(target.addr(), SLOT_TARGET_CHECK, &args![]).u32() == 0 {
        e.call(TIME_CONTROLLER_SET_TARGET, &args![this, 0u32]);
        return;
    }
    e.call(TIME_CONTROLLER_SET_TARGET, &args![this, target]);
    if e.mem.u32(this.addr().wrapping_add(MORPH_DATA)) != 0 {
        e.vcall(this.addr(), SLOT_AFTER_DATA, &args![]);
        e.mem.set_u8(this.addr().wrapping_add(BOUND_CALCULATED), 1);
    }
}

// Translated from 00a3a7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::GuaranteeTimeRange` (Xbox PDB): for each morph target index
/// (re-reading the target count each time), the interpolator at that index (slot `0xc4`)
/// gets `GuaranteeTimeRange(start, end)` (its slot `0xc4`).
pub fn ni_geom_morpher_controller_guarantee_time_range(
    e: &mut Engine,
    this: Ptr,
    start: f32,
    end: f32,
) {
    let this = this.addr();
    let data = e.mem.u32(this.wrapping_add(MORPH_DATA));
    if e.mem.u32(data.wrapping_add(DATA_NUM_TARGETS)) == 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let interpolator = e.vcall(this, SLOT_GET_INTERPOLATOR, &args![index]).u32();
        if interpolator != 0 {
            let start = x87_quiet(start).to_bits();
            let end = x87_quiet(end).to_bits();
            e.vcall(interpolator, SLOT_GUARANTEE_TIME_RANGE, &args![start, end]);
        }
        let data = e.mem.u32(this.wrapping_add(MORPH_DATA));
        index = index.wrapping_add(1);
        if index >= e.mem.u32(data.wrapping_add(DATA_NUM_TARGETS)) {
            break;
        }
    }
}

// Translated from 00a3a7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::RegisterStreamables` (Xbox PDB): false when the base's
/// `RegisterStreamables` (`00a2f8b0`) is; otherwise the morph data (when there is one) and
/// each interpolator (slot `0xc4`, for 16-bit indices below the interpolator count, slot
/// `0xb4`, asked again after each) register with the stream (their slot `0x54`). True.
pub fn ni_geom_morpher_controller_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    if !e
        .call(BASE_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let this = this.addr();
    let data = e.mem.u32(this.wrapping_add(MORPH_DATA));
    if data != 0 {
        e.vcall(data, SLOT_REGISTER_STREAMABLES, &args![stream]);
    }
    let mut index = 0u32;
    let mut count = e.vcall(this, SLOT_INTERPOLATOR_COUNT, &args![]).u16();
    while (index as u16) < count {
        let interpolator = e.vcall(this, SLOT_GET_INTERPOLATOR, &args![index]).u32();
        if interpolator != 0 {
            e.vcall(interpolator, SLOT_REGISTER_STREAMABLES, &args![stream]);
        }
        index = index.wrapping_add(1);
        count = e.vcall(this, SLOT_INTERPOLATOR_COUNT, &args![]).u16();
    }
    true
}

// Translated from 00a3a870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::GetInterpolatorID` (Xbox PDB): the name handle of the morph
/// target at `index` (`00a41590` gives the name's address); an index past the target count
/// reads it through a null target, as the exe does.
pub fn ni_geom_morpher_controller_get_interpolator_id(
    e: &mut Engine,
    this: Ptr,
    index: u16,
) -> u32 {
    let data = e.mem.u32(this.addr().wrapping_add(MORPH_DATA));
    let target = morph_target(e, data, u32::from(index));
    let name = e.call(MORPH_TARGET_NAME, &args![target]).u32();
    e.mem.u32(name)
}

/// Item `index`'s weight as its bits (an `FLD`/`FSTP` copy), or 0.0 without items.
fn item_weight(e: &Engine, this: u32, index: u16) -> u32 {
    let items = e.mem.u32(this.wrapping_add(INTERP_ITEMS));
    if items == 0 {
        return 0;
    }
    let at = items.wrapping_add(u32::from(index) * INTERP_ITEM_SIZE + ITEM_WEIGHT);
    x87_quiet_bits(e.mem.u32(at))
}

// Translated from 00a3a900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::CreatePoseInterpolator` (Xbox PDB): a new `NiFloatInterpolator`
/// (0x18 bytes, `00a3c390`) posed at item `index`'s weight (0.0 without items); 0 when the
/// allocation fails.
pub fn ni_geom_morpher_controller_create_pose_interpolator(
    e: &mut Engine,
    this: Ptr,
    index: u16,
) -> u32 {
    let weight = item_weight(e, this.addr(), index);
    let block = e.call(NI_ALLOCATE, &args![0x18u32]).u32();
    if block == 0 {
        return 0;
    }
    e.call(
        FLOAT_INTERPOLATOR_CONSTRUCT_VALUE,
        &args![block, x87_quiet_bits(weight)],
    )
    .u32()
}

// Translated from 00a3a950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::ResetTimeExtrema` (Xbox PDB): unless the controller is managed
/// (flag bit 5 of the byte at +8), the key times start from the floats at `01096ce0` (low)
/// and `01015f5c` (high) and widen to every interpolator's active time range (slot `0xc0`
/// into two stack floats that start from the same two values): a range start below the low
/// key time or a range end above the high one replaces it (NaN replaces nothing). The target
/// count is read once (0 without morph data).
pub fn ni_geom_morpher_controller_reset_time_extrema(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    if (e.mem.u8(this.wrapping_add(FLAGS)) >> 5) & 1 != 0 {
        return;
    }
    let data = e.mem.u32(this.wrapping_add(MORPH_DATA));
    let count = if data != 0 {
        e.mem.u32(data.wrapping_add(DATA_NUM_TARGETS))
    } else {
        0
    };
    let lo = x87_quiet_bits(e.mem.u32(EXTREMA_START_LO));
    e.mem.set_u32(this.wrapping_add(LO_KEY_TIME), lo);
    let hi = x87_quiet_bits(e.mem.u32(EXTREMA_START_HI));
    e.mem.set_u32(this.wrapping_add(HI_KEY_TIME), hi);
    // Frame: the range start +0, the range end +4.
    e.with_stack(8, |e, frame| {
        let start = frame.addr();
        let end = start + 4;
        for index in 0..count {
            let interpolator = e.vcall(this, SLOT_GET_INTERPOLATOR, &args![index]).u32();
            if interpolator == 0 {
                continue;
            }
            let hi = x87_quiet_bits(e.mem.u32(EXTREMA_START_HI));
            e.mem.set_u32(end, hi);
            let lo = x87_quiet_bits(e.mem.u32(EXTREMA_START_LO));
            e.mem.set_u32(start, lo);
            e.vcall(interpolator, SLOT_GET_ACTIVE_TIME_RANGE, &args![start, end]);
            let range_start = x87_load32(e.mem.u32(start));
            let lo = x87_load32(e.mem.u32(this.wrapping_add(LO_KEY_TIME)));
            if range_start < lo {
                e.mem.set_u32(
                    this.wrapping_add(LO_KEY_TIME),
                    x87_quiet_bits(e.mem.u32(start)),
                );
            }
            let range_end = x87_load32(e.mem.u32(end));
            let hi = x87_load32(e.mem.u32(this.wrapping_add(HI_KEY_TIME)));
            if hi < range_end {
                e.mem.set_u32(
                    this.wrapping_add(HI_KEY_TIME),
                    x87_quiet_bits(e.mem.u32(end)),
                );
            }
        }
    });
}

// Translated from 00a3aba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::OnPreDisplay` (Xbox PDB): with a target and the pre-display flag
/// (+0x40) set, the target's geometry data is marked changed (`MarkAsChanged(1)`), its slot
/// `0xa0` runs when bit 0 of the morpher flags (+0x34) is set and the data's word at +0x24 is
/// non-zero, and the flag is cleared.
pub fn ni_geom_morpher_controller_on_pre_display(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let target = e.mem.u32(this.wrapping_add(TARGET));
    if target == 0 || e.mem.u8(this.wrapping_add(NEED_PRE_DISPLAY)) == 0 {
        return;
    }
    let geometry = e.mem.u32(target.wrapping_add(GEOMETRY_DATA));
    e.call(MARK_AS_CHANGED, &args![geometry, 1u32]);
    if e.mem.u8(this.wrapping_add(MORPHER_FLAGS)) & 1 != 0
        && e.mem.u32(geometry.wrapping_add(0x24)) != 0
    {
        e.vcall(geometry, SLOT_GEOMETRY_DATA_A0, &args![]);
    }
    e.mem.set_u8(this.wrapping_add(NEED_PRE_DISPLAY), 0);
}

// Translated from 00a3ad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::IsEqual` (Xbox PDB): the base's `IsEqual` (`00c283f0`), the
/// morph data's `IsEqual` with the other's data (slot `0x5c`), then per item up to the
/// interpolator count (`008d3ed0`, asked again after each): both or neither interpolator, the
/// other's interpolator `IsEqual` this one's, and equal weights (unordered is unequal).
pub fn ni_geom_morpher_controller_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(BASE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let this = this.addr();
    let other = other.addr();
    let data = e.mem.u32(this.wrapping_add(MORPH_DATA));
    let other_data = e.mem.u32(other.wrapping_add(MORPH_DATA));
    if !e.vcall(data, SLOT_IS_EQUAL, &args![other_data]).bool() {
        return false;
    }
    let mut index = 0u32;
    if interpolator_count(e, this) == 0 {
        return true;
    }
    loop {
        let offset = index.wrapping_mul(INTERP_ITEM_SIZE);
        let other_item = e
            .mem
            .u32(other.wrapping_add(INTERP_ITEMS))
            .wrapping_add(offset);
        let theirs = e.mem.u32(other_item);
        let items = e.mem.u32(this.wrapping_add(INTERP_ITEMS));
        let ours = e.mem.u32(items.wrapping_add(offset));
        if theirs != 0 {
            if ours == 0 {
                return false;
            }
            if !e.vcall(theirs, SLOT_IS_EQUAL, &args![ours]).bool() {
                return false;
            }
        } else if ours != 0 {
            return false;
        }
        let other_weight = x87_load32(e.mem.u32(other_item.wrapping_add(ITEM_WEIGHT)));
        let items = e.mem.u32(this.wrapping_add(INTERP_ITEMS));
        let weight = x87_load32(e.mem.u32(items.wrapping_add(offset + ITEM_WEIGHT)));
        if weight != other_weight {
            return false;
        }
        index = index.wrapping_add(1);
        if index >= interpolator_count(e, this) {
            return true;
        }
    }
}

// Translated from 00a3adc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::GetInterpolatorCount` (Xbox PDB): the morph data's target count
/// in AX (the upper half of EAX keeps the data pointer's), or 0 without morph data.
pub fn ni_geom_morpher_controller_get_interpolator_count(e: &mut Engine, this: Ptr) -> u32 {
    let data = e.mem.u32(this.addr().wrapping_add(MORPH_DATA));
    if data == 0 {
        return 0;
    }
    (data & 0xffff_0000) | u32::from(e.mem.u16(data.wrapping_add(DATA_NUM_TARGETS)))
}

// Translated from 00a3ae50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::SynchronizePoseInterpolator` (Xbox PDB):
/// `NiFloatInterpolator::SetPoseValue` with item `index`'s weight (0.0 without items) on the
/// interpolator as an `NiFloatInterpolator` (`00653290`), or on a null one when
/// `interpolator` is null.
pub fn ni_geom_morpher_controller_synchronize_pose_interpolator(
    e: &mut Engine,
    this: Ptr,
    interpolator: u32,
    index: u16,
) {
    let weight = item_weight(e, this.addr(), index);
    let float_interpolator = if interpolator == 0 {
        0
    } else {
        e.call(
            DYNAMIC_CAST,
            &args![interpolator, NI_FLOAT_INTERPOLATOR_RTTI],
        )
        .u32()
    };
    e.call(
        FLOAT_INTERPOLATOR_SET_POSE_VALUE,
        &args![float_interpolator, x87_quiet_bits(weight)],
    );
}

// Translated from 00a3afe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::TargetIsRequiredType` (Xbox PDB): whether there is a target and
/// it is an `NiGeometry` (its `GetRTTI` or a base is `011f4acc`).
pub fn ni_geom_morpher_controller_target_is_required_type(e: &mut Engine, this: Ptr) -> bool {
    let target = e.mem.u32(this.addr().wrapping_add(TARGET));
    target != 0 && is_kind_of(e, target, NI_GEOMETRY_RTTI)
}

// Translated from 00a3b350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeomMorpherController::SaveBinary` (Xbox PDB): the base's `SaveBinary` (`00c27120`),
/// the flags word (2 bytes), the morph data's link ID (the stream's slot `0x30`), the
/// always-update byte (written from the argument's own stack word, its low byte), the target
/// count (4 bytes; 0 without morph data), then per item its interpolator's link ID and its
/// weight (4 bytes).
pub fn ni_geom_morpher_controller_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    let this = this.addr();
    let stream = stream.addr();
    // Frame: the count +0, the component size word +4, the argument word +8.
    e.with_stack(12, |e, frame| {
        let count_word = frame.addr();
        let component = count_word + 4;
        let argument = count_word + 8;
        e.mem.set_u32(argument, stream);
        let flags = this.wrapping_add(MORPHER_FLAGS);
        stream_write(e, stream, flags, 2, 2, component);
        let data = e.mem.u32(this.wrapping_add(MORPH_DATA));
        e.vcall(stream, SLOT_SAVE_LINK_ID, &args![data]);
        let always = e.mem.u8(this.wrapping_add(ALWAYS_UPDATE));
        e.mem.set_u8(argument, always);
        stream_write(e, stream, argument, 1, 1, component);
        let data = e.mem.u32(this.wrapping_add(MORPH_DATA));
        let count = if data != 0 {
            e.mem.u32(data.wrapping_add(DATA_NUM_TARGETS))
        } else {
            0
        };
        e.mem.set_u32(count_word, count);
        stream_write(e, stream, count_word, 4, 4, component);
        let mut index = 0u32;
        while index < e.mem.u32(count_word) {
            let items = e.mem.u32(this.wrapping_add(INTERP_ITEMS));
            let interpolator = e.mem.u32(items.wrapping_add(index * INTERP_ITEM_SIZE));
            e.vcall(stream, SLOT_SAVE_LINK_ID, &args![interpolator]);
            let items = e.mem.u32(this.wrapping_add(INTERP_ITEMS));
            let weight = items.wrapping_add(index * INTERP_ITEM_SIZE + ITEM_WEIGHT);
            stream_write(e, stream, weight, 4, 4, component);
            index += 1;
        }
    });
}

// Translated from 00a3b700 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00a3b700 - the /GS cookie check (00ec408c) guarding the 260-byte text buffer is not modelled
/// `NiGeomMorpherController::GetViewerStrings` (Xbox PDB; it does not run the base's): the
/// class name (`011f3728`), `"NumTargets"` and `"NumVertsPerTarget"` (0 without morph data),
/// then one string per morph target: `"%d %s = %0.2f"` with the index, the target's name and
/// the interpolator's value when the interpolator (slot `0xc4`) is an
/// `NiBlendFloatInterpolator` (+0x30) or else an `NiFloatInterpolator` (+0xc), otherwise
/// `"%d %s"`; each formatted into the stack buffer (`sprintf`), copied into a new block of
/// length + 3 bytes (`00aa1070`, `strcpy_s`) and stored with the inline `SetAtGrow`. A target
/// index past the count gives a null target, as the exe does.
pub fn ni_geom_morpher_controller_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let this = this.addr();
    let strings = strings.addr();
    // Frame (offsets from the stack pointer after the three saved registers): the string
    // word +0x20, the value float +0x2c, then the text buffer +0x38.
    e.with_stack(0x140, |e, frame| {
        let frame = frame.addr();
        let slot = frame + 0x20;
        let value_word = frame + 0x2c;
        let buffer = frame + 0x38;
        let class_name = e.mem.u32(NI_GEOM_MORPHER_CONTROLLER_RTTI);
        let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let data = e.mem.u32(this.wrapping_add(MORPH_DATA));
        let count = if data != 0 {
            e.mem.u32(data.wrapping_add(DATA_NUM_TARGETS))
        } else {
            0
        };
        let text = e
            .call(
                NI_GET_VIEWER_STRING_UNSIGNED,
                &args![TEXT_NUM_TARGETS, count],
            )
            .u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let data = e.mem.u32(this.wrapping_add(MORPH_DATA));
        let verts = if data != 0 {
            e.mem.u32(data.wrapping_add(DATA_NUM_VERTS))
        } else {
            0
        };
        let text = e
            .call(
                NI_GET_VIEWER_STRING_UNSIGNED,
                &args![TEXT_NUM_VERTS_PER_TARGET, verts],
            )
            .u32();
        e.mem.set_u32(slot, text);
        add_viewer_string(e, strings, slot);
        let data = e.mem.u32(this.wrapping_add(MORPH_DATA));
        let count = if data != 0 {
            e.mem.u32(data.wrapping_add(DATA_NUM_TARGETS))
        } else {
            0
        };
        let mut index = 0u32;
        let mut offset = 0u32;
        while index < count {
            let data = e.mem.u32(this.wrapping_add(MORPH_DATA));
            let target = if index < e.mem.u32(data.wrapping_add(DATA_NUM_TARGETS)) {
                e.mem
                    .u32(data.wrapping_add(DATA_TARGETS))
                    .wrapping_add(offset)
            } else {
                0
            };
            let interpolator = e.vcall(this, SLOT_GET_INTERPOLATOR, &args![index]).u32();
            let mut value_at = None;
            if interpolator != 0 {
                let blend = is_kind_of(e, interpolator, NI_BLEND_FLOAT_INTERPOLATOR_RTTI);
                let plain = is_kind_of(e, interpolator, NI_FLOAT_INTERPOLATOR_RTTI);
                if blend {
                    value_at = Some(interpolator.wrapping_add(BLEND_FLOAT_VALUE));
                } else if plain {
                    value_at = Some(interpolator.wrapping_add(FLOAT_VALUE));
                }
            }
            if let Some(value_at) = value_at {
                let value = x87_quiet_bits(e.mem.u32(value_at));
                e.mem.set_u32(value_word, value);
                let name = e.call(MORPH_TARGET_NAME, &args![target]).u32();
                let name = e.mem.u32(name);
                let double = x87_load32(e.mem.u32(value_word)).to_bits();
                e.call(
                    SPRINTF,
                    &args![
                        buffer,
                        FORMAT_INDEX_NAME_WEIGHT,
                        index,
                        name,
                        double as u32,
                        (double >> 32) as u32
                    ],
                );
            } else {
                let name = e.call(MORPH_TARGET_NAME, &args![target]).u32();
                let name = e.mem.u32(name);
                e.call(SPRINTF, &args![buffer, FORMAT_INDEX_NAME, index, name]);
            }
            let length = e.mem.cstr(buffer).len() as u32;
            let size = length.wrapping_add(3);
            let copy = e.call(NI_ALLOC, &args![size]).u32();
            e.call(STRCPY_S, &args![copy, size, buffer]);
            set_at_grow(e, strings, copy);
            offset = offset.wrapping_add(MORPH_TARGET_SIZE);
            index += 1;
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a3afd0, ni_geom_morpher_controller_get_rtti() -> Ptr),
        entry!(0x00a3b530, ni_geom_morpher_controller_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00a3_a8a0, ni_geom_morpher_controller_get_interpolator(Ptr, u16) -> u32),
        entry!(
            0x00a3_a8b0,
            ni_geom_morpher_controller_set_interpolator(Ptr, u32, u16)
        ),
        entry!(0x00a3_a940, ni_geom_morpher_controller_interp_item_construct(Ptr) -> Ptr),
        entry!(
            0x00a3_aa10,
            ni_geom_morpher_controller_gen_morph_interp(Ptr, u32)
        ),
        entry!(
            0x00a3_abe0,
            ni_geom_morpher_controller_calculate_morph_bound(Ptr)
        ),
        entry!(
            0x00a3_acb0,
            ni_geom_morpher_controller_process_clone(Ptr, u32)
        ),
        entry!(0x00a3_add0, ni_geom_morpher_controller_get_interpolator_index(Ptr, u32) -> u16),
        entry!(0x00a3_aeb0, ni_geom_morpher_controller_get_interpolator_index_fx(Ptr, Ptr) -> u16),
        entry!(0x00a3_af10, fn_00a3af10(Ptr)),
        entry!(0x00a3_b010, ni_geom_morpher_controller_get_vertex_data(Ptr) -> u32),
        entry!(0x00a3_b080, ni_geom_morpher_controller_update(Ptr, Ptr)),
        entry!(
            0x00a3_b150,
            ni_geom_morpher_controller_load_binary(Ptr, Ptr)
        ),
        entry!(0x00a3_b460, fn_00a3b460(Ptr)),
        entry!(0x00a3_b550, ni_geom_morpher_controller_create_object() -> Ptr),
        entry!(
            0x00a3_b5a0,
            ni_geom_morpher_controller_link_object(Ptr, Ptr)
        ),
        entry!(0x00a3_ba10, fn_00a3ba10(Ptr, u32, u32)),
        entry!(
            0x00a3_ba70,
            ni_geom_morpher_controller_copy_members(Ptr, Ptr, u32)
        ),
        entry!(0x00a3_bb80, ni_geom_morpher_controller_create_clone(Ptr, u32) -> Ptr),
        entry!(0x00a3_a730, fn_00a3a730(Ptr, u32, u32) -> u32),
        entry!(0x00a3_a750, ni_geom_morpher_controller_set_target(Ptr, Ptr)),
        entry!(
            0x00a3_a7a0,
            ni_geom_morpher_controller_guarantee_time_range(Ptr, f32, f32)
        ),
        entry!(
            0x00a3_a7f0,
            ni_geom_morpher_controller_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a3_a870,
            ni_geom_morpher_controller_get_interpolator_id(Ptr, u16) -> u32
        ),
        entry!(
            0x00a3_a900,
            ni_geom_morpher_controller_create_pose_interpolator(Ptr, u16) -> u32
        ),
        entry!(
            0x00a3_a950,
            ni_geom_morpher_controller_reset_time_extrema(Ptr)
        ),
        entry!(0x00a3_aba0, ni_geom_morpher_controller_on_pre_display(Ptr)),
        entry!(
            0x00a3_ad00,
            ni_geom_morpher_controller_is_equal(Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a3_adc0,
            ni_geom_morpher_controller_get_interpolator_count(Ptr) -> u32
        ),
        entry!(
            0x00a3_ae50,
            ni_geom_morpher_controller_synchronize_pose_interpolator(Ptr, u32, u16)
        ),
        entry!(
            0x00a3_afe0,
            ni_geom_morpher_controller_target_is_required_type(Ptr) -> bool
        ),
        entry!(
            0x00a3_b350,
            ni_geom_morpher_controller_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a3_b700,
            ni_geom_morpher_controller_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_geom_morpher_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a3afd0, &args![]).u32(),
            NI_GEOM_MORPHER_CONTROLLER_RTTI
        );
    }

    #[test]
    fn ni_geom_morpher_controller_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a3_af10, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a3b530, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a3b530, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x44]]);
        assert_eq!(calls_to(&e, 0x00a3_af10), vec![vec![this]; 2]);
    }

    type Log = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const DELETE_THIS: u32 = 0x00ee_0100;
    const GET_INTERP: u32 = 0x00ee_0200;
    const SET_INTERP: u32 = 0x00ee_0300;
    const KEY_TIMES: u32 = 0x00ee_0400;
    const VERTEX_BUFFER: u32 = 0x00ee_0500;
    const AFTER_DATA: u32 = 0x00ee_0600;
    const PROCESS_CLONE: u32 = 0x00ee_0700;
    const UPDATE_FLOAT: u32 = 0x00ee_0800;
    const TARGET_CHECK: u32 = 0x00ee_0a00;
    const TASK_RUN: u32 = 0x00ee_0b00;
    const READ: u32 = 0x00ee_0c00;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            ret(eax)
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The interlocked import slots, counting in memory (not logged).
    fn interlocked(e: &mut Engine) {
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            ret(count)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            ret(count)
        });
    }

    /// A vtable with every slot these tests use set to its double.
    fn vtable(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for (slot, target) in [
            (SLOT_DELETE_THIS, DELETE_THIS),
            (SLOT_GET_INTERPOLATOR, GET_INTERP),
            (SLOT_SET_INTERPOLATOR, SET_INTERP),
            (SLOT_SET_KEY_TIMES, KEY_TIMES),
            (SLOT_VERTEX_BUFFER, VERTEX_BUFFER),
            (SLOT_AFTER_DATA, AFTER_DATA),
            (SLOT_PROCESS_CLONE, PROCESS_CLONE),
            (SLOT_INTERPOLATOR_UPDATE_FLOAT, UPDATE_FLOAT),
            (SLOT_TARGET_CHECK, TARGET_CHECK),
            (SLOT_TASK_RUN, TASK_RUN),
        ] {
            e.mem.set_u32(vtable + slot, target);
        }
        vtable
    }

    /// A counted object (count `count`) with the test vtable.
    fn counted(e: &mut Engine, count: u32) -> u32 {
        let vtable = vtable(e);
        let object = e.mem.alloc(0xc0);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    /// An item array of `count` items (behind its count word).
    fn items(e: &mut Engine, count: u32) -> u32 {
        let block = e.mem.alloc(4 + 8 * count);
        e.mem.set_u32(block, count);
        block + 4
    }

    /// Morph data of `count` targets of `vertices` vertices.
    fn morph_data(e: &mut Engine, count: u32, vertices: u32) -> u32 {
        let data = counted(e, 1);
        let targets = e.mem.alloc(0x10 * count.max(1));
        for i in 0..count {
            e.mem.set_u32(targets + 0x10 * i, 0x7000 + 0x100 * i);
            e.mem
                .set_f32(targets + 0x10 * i + TARGET_WEIGHT, i as f32 + 0.5);
        }
        e.mem.set_u32(data + DATA_NUM_TARGETS, count);
        e.mem.set_u32(data + DATA_NUM_VERTS, vertices);
        e.mem.set_u32(data + DATA_TARGETS, targets);
        data
    }

    #[test]
    fn get_and_set_interpolator_use_the_item_array() {
        let mut e = Engine::new();
        interlocked(&mut e);
        let log = Log::default();
        logged(&mut e, &log, DELETE_THIS, 0);
        let this = e.mem.alloc(0x44);
        let items = items(&mut e, 3);
        e.mem.set_u32(this + INTERP_ITEMS, items);
        let old = counted(&mut e, 1);
        let new = counted(&mut e, 0);
        e.mem.set_u32(items + 16, old);
        assert_eq!(e.call(0x00a3_a8a0, &args![this, 2u32]).u32(), old);
        e.call(0x00a3_a8b0, &args![this, new, 2u32]);
        assert_eq!(e.mem.u32(items + 16), new);
        assert_eq!(e.mem.u32(new + 4), 1);
        assert_eq!(calls(&log, DELETE_THIS), [vec![old]]);
        // The same pointer again changes nothing.
        e.call(0x00a3_a8b0, &args![this, new, 2u32]);
        assert_eq!(e.mem.u32(new + 4), 1);
    }

    #[test]
    fn interp_item_construct_clears_the_item() {
        let mut e = Engine::new();
        let item = e.mem.alloc(8);
        e.mem.write(item, &[0xaa; 8]);
        assert_eq!(e.call(0x00a3_a940, &args![item]).u32(), item);
        assert_eq!(e.mem.bytes(item, 8), vec![0; 8]);
    }

    /// A controller over a target with geometry data of 2 vertices, morph
    /// data of `targets` targets and an item array.
    fn controller(e: &mut Engine, targets: u32) -> (u32, u32, u32, u32) {
        let this = counted(e, 1);
        let target = counted(e, 1);
        let geometry = e.mem.alloc(0x40);
        e.mem.set_u16(geometry + GEOMETRY_VERTEX_COUNT, 2);
        e.mem.set_u32(target + GEOMETRY_DATA, geometry);
        e.mem.set_u32(this + TARGET, target);
        let data = morph_data(e, targets, 2);
        e.mem.set_u32(this + MORPH_DATA, data);
        let items = items(e, targets);
        e.mem.set_u32(this + INTERP_ITEMS, items);
        e.mem.set_f32(this + SCALED_TIME, 1.5);
        (this, target, data, items)
    }

    #[test]
    fn gen_morph_interp_updates_weights_and_blends_targets() {
        let mut e = Engine::new();
        let log = Log::default();
        for (address, value) in [(WEIGHT_ABOVE, 0.001f32), (WEIGHT_BELOW, -0.001)] {
            e.map(address, 4);
            e.mem.set_f32(address, value);
        }
        let (this, target, data, items) = controller(&mut e, 3);
        e.mem.set_u8(data + DATA_RELATIVE, 1);
        let interpolator = counted(&mut e, 1);
        // Target 1 has an interpolator answering 0.25; target 2 none.
        let l = log.clone();
        e.register_double(GET_INTERP, move |_, a| {
            l.borrow_mut().push((GET_INTERP, a.to_vec()));
            ret(if a[1] == 1 { interpolator } else { 0 })
        });
        let l = log.clone();
        e.register_double(UPDATE_FLOAT, move |e, a| {
            l.borrow_mut().push((UPDATE_FLOAT, a.to_vec()));
            e.mem.set_f32(a[3], 0.25);
            ret(1)
        });
        e.mem.set_f32(items + 16 + ITEM_WEIGHT, 0.0005);
        logged(&mut e, &log, VERTEX_BUFFER, 0x9000);
        logged(&mut e, &log, MEMSET, 0);
        logged(&mut e, &log, ADD_WEIGHTED_TARGET, 0);
        logged(&mut e, &log, TARGET_CHECK, 1);
        logged(&mut e, &log, MARK_AS_CHANGED, 0);
        e.call(0x00a3_aa10, &args![this, 0u32]);
        assert_eq!(e.mem.f32(items + ITEM_WEIGHT), 1.0);
        assert_eq!(e.mem.f32(items + 8 + ITEM_WEIGHT), 0.25);
        assert_eq!(e.mem.f32(items + 16 + ITEM_WEIGHT), 0.0005);
        let update = &calls(&log, UPDATE_FLOAT)[0];
        assert_eq!(update[..3], [interpolator, 1.5f32.to_bits(), target]);
        assert_eq!(calls(&log, MEMSET), [vec![0x9000, 0, 24]]);
        // Target 2's weight is between the thresholds: not added.
        assert_eq!(
            calls(&log, ADD_WEIGHTED_TARGET),
            [
                vec![0x9000, 1.0f32.to_bits(), 0x7000, 2],
                vec![0x9000, 0.25f32.to_bits(), 0x7100, 2]
            ]
        );
        let geometry = e.mem.u32(target + GEOMETRY_DATA);
        assert_eq!(calls(&log, MARK_AS_CHANGED), [vec![geometry, 1]]);
        // Ignoring the base target: weight 0.0, no clearing, nothing added
        // when no weight passes and the interpolator fails.
        e.mem.set_u8(this + IGNORE_BASE_TARGET, 1);
        logged(&mut e, &log, UPDATE_FLOAT, 0);
        e.mem.set_f32(items + 8 + ITEM_WEIGHT, -0.0005);
        log.borrow_mut().clear();
        e.call(0x00a3_aa10, &args![this, 0u32]);
        assert_eq!(e.mem.f32(items + ITEM_WEIGHT), 0.0);
        assert_eq!(e.mem.f32(items + 8 + ITEM_WEIGHT), -0.0005);
        assert!(calls(&log, MEMSET).is_empty());
        assert!(calls(&log, ADD_WEIGHTED_TARGET).is_empty());
        assert!(calls(&log, TARGET_CHECK).is_empty());
    }

    #[test]
    fn calculate_morph_bound_merges_the_target_bounds() {
        let mut e = Engine::new();
        let log = Log::default();
        let (this, target, data, _) = controller(&mut e, 3);
        let l = log.clone();
        e.register_double(BOUND_COMPUTE_FROM_DATA, move |e, a| {
            l.borrow_mut().push((BOUND_COMPUTE_FROM_DATA, a.to_vec()));
            for k in 0..4 {
                e.mem.set_f32(a[0] + 4 * k, a[2] as f32 + k as f32);
            }
            ret(0)
        });
        logged(&mut e, &log, BOUND_MERGE, 0);
        e.call(0x00a3_abe0, &args![this]);
        let bounds = calls(&log, BOUND_COMPUTE_FROM_DATA);
        assert_eq!(bounds.len(), 3);
        assert_eq!(bounds[0][1..], [2, 0x7000]);
        assert_eq!(bounds[2][1..], [2, 0x7200]);
        assert_eq!(calls(&log, BOUND_MERGE)[0], [bounds[0][0], bounds[1][0]]);
        let geometry = e.mem.u32(target + GEOMETRY_DATA);
        assert_eq!(e.mem.f32(geometry + GEOMETRY_BOUND), 0x7000 as f32);
        assert_eq!(
            e.mem.f32(geometry + GEOMETRY_BOUND + 0xc),
            0x7000 as f32 + 3.0
        );
        // No targets: the zero centre and radius 0.
        e.mem.set_u32(data + DATA_NUM_TARGETS, 0);
        e.map(EMPTY_BOUND_CENTER, 12);
        e.mem.set_f32(EMPTY_BOUND_CENTER + 4, 9.0);
        e.call(0x00a3_abe0, &args![this]);
        assert_eq!(e.mem.f32(geometry + GEOMETRY_BOUND), 0.0);
        assert_eq!(e.mem.f32(geometry + GEOMETRY_BOUND + 4), 9.0);
        assert_eq!(e.mem.f32(geometry + GEOMETRY_BOUND + 0xc), 0.0);
    }

    #[test]
    fn process_clone_passes_the_clone_to_each_interpolator() {
        let mut e = Engine::new();
        let log = Log::default();
        let (this, _, _, items) = controller(&mut e, 3);
        let interpolator = counted(&mut e, 1);
        e.mem.set_u32(items + 8, interpolator);
        logged(&mut e, &log, TIME_CONTROLLER_PROCESS_CLONE, 0);
        logged(&mut e, &log, INTERPOLATOR_COUNT, 3);
        logged(&mut e, &log, PROCESS_CLONE, 0);
        e.call(0x00a3_acb0, &args![this, 0x9000u32]);
        assert_eq!(
            calls(&log, TIME_CONTROLLER_PROCESS_CLONE),
            [vec![this, 0x9000]]
        );
        assert_eq!(calls(&log, PROCESS_CLONE), [vec![interpolator, 0x9000]]);
        assert_eq!(calls(&log, INTERPOLATOR_COUNT).len(), 4);
        logged(&mut e, &log, INTERPOLATOR_COUNT, 0);
        log.borrow_mut().clear();
        e.call(0x00a3_acb0, &args![this, 0x9000u32]);
        assert!(calls(&log, PROCESS_CLONE).is_empty());
    }

    #[test]
    fn get_interpolator_index_finds_the_named_target() {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(NO_INDEX, 2);
        e.mem.set_u16(NO_INDEX, 0xffff);
        let (this, _, data, _) = controller(&mut e, 3);
        let targets = e.mem.u32(data + DATA_TARGETS);
        for i in 0..3 {
            e.mem.set_u32(targets + 0x10 * i + 4, 0x6000 + i);
        }
        logged(&mut e, &log, INTERPOLATOR_COUNT, 3);
        e.register(MORPH_TARGET_NAME, |_, a| ret(a[0] + 4));
        e.register(NI_STRICMP, |_, a| ret(u32::from(a[0] != 0x6002)));
        assert_eq!(e.call(0x00a3_add0, &args![this, 0x1234u32]).u16(), 2);
        assert_eq!(e.call(0x00a3_add0, &args![this, 0u32]).u16(), 0xffff);
        e.register(NI_STRICMP, |_, _| ret(1));
        assert_eq!(e.call(0x00a3_add0, &args![this, 0x1234u32]).u16(), 0xffff);
        // The handle comparison.
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x6001);
        assert_eq!(e.call(0x00a3_aeb0, &args![this, name]).u16(), 1);
        e.mem.set_u32(name, 0x6009);
        assert_eq!(e.call(0x00a3_aeb0, &args![this, name]).u16(), 0xffff);
        logged(&mut e, &log, INTERPOLATOR_COUNT, 0);
        e.mem.set_u32(name, 0x6001);
        assert_eq!(e.call(0x00a3_aeb0, &args![this, name]).u16(), 0xffff);
    }

    #[test]
    fn destructor_releases_the_data_and_the_items() {
        let mut e = Engine::new();
        interlocked(&mut e);
        let log = Log::default();
        logged(&mut e, &log, DELETE_THIS, 0);
        logged(&mut e, &log, MEMORY_MANAGER_DEALLOCATE, 0);
        logged(&mut e, &log, TIME_CONTROLLER_DESTRUCT, 0);
        let (this, _, data, items) = controller(&mut e, 2);
        let interpolator = counted(&mut e, 1);
        e.mem.set_u32(items, interpolator);
        e.call(0x00a3_af10, &args![this]);
        assert_eq!(e.mem.u32(this), NI_GEOM_MORPHER_CONTROLLER_VTABLE);
        assert_eq!(e.mem.u32(this + MORPH_DATA), 0);
        assert_eq!(
            *log.borrow(),
            [
                (DELETE_THIS, vec![data]),
                (DELETE_THIS, vec![interpolator]),
                (MEMORY_MANAGER_DEALLOCATE, vec![MEMORY_MANAGER, items - 4]),
                (TIME_CONTROLLER_DESTRUCT, vec![this]),
            ]
        );
    }

    #[test]
    fn get_vertex_data_makes_a_vertex_array_when_missing() {
        let mut e = Engine::new();
        let log = Log::default();
        let (this, target, _, _) = controller(&mut e, 1);
        let geometry = e.mem.u32(target + GEOMETRY_DATA);
        e.mem.set_u16(geometry + GEOMETRY_DATA_FLAGS, 0x5003);
        e.mem.set_u32(geometry + 0x24, 0x24);
        e.mem.set_u32(geometry + 0x28, 0x28);
        e.mem.set_u32(geometry + 0x2c, 0x2c);
        logged(&mut e, &log, MEMORY_MANAGER_ALLOCATE, 0x8800);
        logged(&mut e, &log, GEOMETRY_DATA_REPLACE_DATA, 0);
        assert_eq!(e.call(0x00a3_b010, &args![this]).u32(), 0x8800);
        assert_eq!(
            *log.borrow(),
            [
                (MEMORY_MANAGER_ALLOCATE, vec![MEMORY_MANAGER, 24]),
                (
                    GEOMETRY_DATA_REPLACE_DATA,
                    vec![geometry, 2, 0x8800, 0x24, 0x28, 0x2c, 1, 0x5000]
                ),
            ]
        );
        e.mem.set_u32(geometry + GEOMETRY_VERTICES, 0x4400);
        assert_eq!(e.call(0x00a3_b010, &args![this]).u32(), 0x4400);
        e.mem.set_u32(this + TARGET, 0);
        assert_eq!(e.call(0x00a3_b010, &args![this]).u32(), 0);
    }

    #[test]
    fn update_runs_the_morph_and_queues_a_task() {
        let mut e = Engine::new();
        let log = Log::default();
        let (this, target, _, _) = controller(&mut e, 0);
        e.map(FORCED_UPDATE_TIME, 4);
        e.mem.set_f32(FORCED_UPDATE_TIME, -f32::MAX);
        e.map(UPDATE_TASKS_ENABLED, 4);
        e.mem.set_u8(UPDATE_TASKS_ENABLED, 1);
        e.map(UPDATE_TASK_OWNER, 4);
        e.mem.set_u32(UPDATE_TASK_OWNER, 0x5b00);
        let geometry = e.mem.u32(target + GEOMETRY_DATA);
        e.mem.set_u32(geometry + 4, 1);
        let task = counted(&mut e, 1);
        for (address, eax) in [
            (VERTEX_BUFFER, 0x9000),
            (MEMSET, 0),
            (UPDATE_TASKS_ALLOWED, 1),
            (NEW_UPDATE_TASK, task),
            (UPDATE_TASK_SET, 0),
            (UPDATE_TASK_QUEUE, 0),
            (TASK_RUN, 0),
            (DONT_DO_UPDATE, 1),
        ] {
            logged(&mut e, &log, address, eax);
        }
        let update = e.mem.alloc(8);
        e.mem.set_u8(this + FLAGS, 0x20);
        e.call(0x00a3_b080, &args![this, update]);
        assert_eq!(e.mem.u8(this + NEED_PRE_DISPLAY), 1);
        assert_eq!(e.mem.f32(this + SCALED_TIME), -f32::MAX);
        assert_eq!(calls(&log, VERTEX_BUFFER), [vec![this]]);
        assert_eq!(calls(&log, UPDATE_TASK_SET), [vec![task, this, target]]);
        assert_eq!(calls(&log, UPDATE_TASK_QUEUE), [vec![0x5b00, task, 2]]);
        assert_eq!(calls(&log, TASK_RUN), [vec![task]]);
        // Not forced: DontDoUpdate says skip, the flag stays clear.
        e.mem.set_u8(this + FLAGS, 0);
        e.mem.set_u8(this + NEED_PRE_DISPLAY, 0);
        e.mem.set_f32(update, 2.0);
        log.borrow_mut().clear();
        e.call(0x00a3_b080, &args![this, update]);
        assert_eq!(calls(&log, DONT_DO_UPDATE), [vec![this, 2.0f32.to_bits()]]);
        assert!(calls(&log, VERTEX_BUFFER).is_empty());
        // Always-update marks it anyway.
        e.mem.set_u8(this + ALWAYS_UPDATE, 1);
        e.call(0x00a3_b080, &args![this, update]);
        assert_eq!(calls(&log, VERTEX_BUFFER).len(), 1);
        // The skip byte at +5.
        e.mem.set_u8(update + 5, 1);
        log.borrow_mut().clear();
        e.call(0x00a3_b080, &args![this, update]);
        assert_eq!(e.mem.u8(update + 7), 1);
        assert!(log.borrow().is_empty());
    }

    /// A stream of file `version` whose reads fill their buffer with the
    /// next word's bytes.
    fn stream(e: &mut Engine, log: &Log, version: u32, words: &'static [u32]) -> u32 {
        let stream = e.mem.alloc(0x5c4);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + STREAM_INPUT, input);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        e.mem.set_u32(input + BINARY_STREAM_READ, READ);
        let l = log.clone();
        let mut next = words.iter();
        e.register_double(READ, move |e, a| {
            l.borrow_mut()
                .push((READ, vec![a[1], a[2], e.mem.u32(a[3]), a[4]]));
            let word = next.next().unwrap().to_le_bytes();
            for b in 0..a[2].min(4) {
                e.mem.set_u8(a[1] + b, word[b as usize]);
            }
            ret(a[2])
        });
        stream
    }

    #[test]
    fn load_binary_reads_the_items_of_new_files() {
        let mut e = Engine::new();
        interlocked(&mut e);
        let log = Log::default();
        let stream = stream(
            &mut e,
            &log,
            0x1402_0007,
            &[0x0102, 1, 2, 0x3f00_0000, 0x3e80_0000],
        );
        let interpolator = counted(&mut e, 0);
        let block = e.mem.alloc(0x14);
        for (address, eax) in [
            (INTERP_CONTROLLER_LOAD_BINARY, 0),
            (STREAM_READ_LINK_ID, 0),
            (MEMORY_MANAGER_ALLOCATE, block),
            (VECTOR_CONSTRUCTOR_ITERATOR, 0),
            (RESOLVE_LINK_ID, interpolator),
        ] {
            logged(&mut e, &log, address, eax);
        }
        let this = e.mem.alloc(0x44);
        e.call(0x00a3_b150, &args![this, stream]);
        assert_eq!(e.mem.u16(this + MORPHER_FLAGS), 0x0102);
        assert_eq!(e.mem.u8(this + ALWAYS_UPDATE), 1);
        assert_eq!(
            calls(&log, MEMORY_MANAGER_ALLOCATE),
            [vec![MEMORY_MANAGER, 0x14]]
        );
        assert_eq!(
            calls(&log, VECTOR_CONSTRUCTOR_ITERATOR),
            [vec![block + 4, 8, 2, INTERP_ITEM_CONSTRUCT]]
        );
        let items = e.mem.u32(this + INTERP_ITEMS);
        assert_eq!(items, block + 4);
        assert_eq!(e.mem.u32(items), interpolator);
        assert_eq!(e.mem.u32(items + 8), interpolator);
        assert_eq!(e.mem.u32(interpolator + 4), 2);
        assert_eq!(e.mem.f32(items + 4), 0.5);
        assert_eq!(e.mem.f32(items + 12), 0.25);
        let reads = calls(&log, READ);
        assert_eq!(reads.len(), 5);
        assert_eq!(reads[3], [items + 4, 4, 4, 1]);
    }

    #[test]
    fn load_binary_of_older_files() {
        let mut e = Engine::new();
        let log = Log::default();
        for (address, eax) in [
            (INTERP_CONTROLLER_LOAD_BINARY, 0),
            (STREAM_READ_LINK_ID, 0),
            (STREAM_READ_MULTIPLE_LINK_IDS, 0),
        ] {
            logged(&mut e, &log, address, eax);
        }
        // Before 0x0a000102: the flags from the stream's word.
        let old = stream(&mut e, &log, 0x0400_0002, &[1]);
        e.mem.set_u16(old + STREAM_OLD_FLAGS, 0x0fe0);
        let this = e.mem.alloc(0x44);
        e.call(0x00a3_b150, &args![this, old]);
        assert_eq!(e.mem.u16(this + MORPHER_FLAGS), 0x3f);
        assert_eq!(calls(&log, READ).len(), 1);
        assert!(calls(&log, STREAM_READ_MULTIPLE_LINK_IDS).is_empty());
        // Between 0x0a010068 and 0x14010003: the link IDs.
        let mid = stream(&mut e, &log, 0x0a01_0068, &[3, 0]);
        e.call(0x00a3_b150, &args![this, mid]);
        assert_eq!(calls(&log, STREAM_READ_MULTIPLE_LINK_IDS), [vec![mid]]);
        assert_eq!(e.mem.u8(this + ALWAYS_UPDATE), 0);
    }

    #[test]
    fn fn_00a3b460_rebuilds_the_items() {
        let mut e = Engine::new();
        interlocked(&mut e);
        let log = Log::default();
        let (this, _, data, items) = controller(&mut e, 2);
        let interpolator = counted(&mut e, 1);
        e.mem.set_u32(items + 8, interpolator);
        let block = e.mem.alloc(0x14);
        for (address, eax) in [
            (DELETE_THIS, 0),
            (MEMORY_MANAGER_DEALLOCATE, 0),
            (MEMORY_MANAGER_ALLOCATE, block),
            (VECTOR_CONSTRUCTOR_ITERATOR, 0),
        ] {
            logged(&mut e, &log, address, eax);
        }
        e.call(0x00a3_b460, &args![this]);
        assert_eq!(calls(&log, DELETE_THIS), [vec![interpolator]]);
        assert_eq!(
            calls(&log, MEMORY_MANAGER_DEALLOCATE),
            [vec![MEMORY_MANAGER, items - 4]]
        );
        assert_eq!(e.mem.u32(this + INTERP_ITEMS), block + 4);
        assert_eq!(e.mem.u32(block), 2);
        // No targets: just cleared; a failed allocation also clears.
        e.mem.set_u32(data + DATA_NUM_TARGETS, 0);
        e.mem.set_u32(block, 0);
        e.call(0x00a3_b460, &args![this]);
        assert_eq!(e.mem.u32(this + INTERP_ITEMS), 0);
        e.mem.set_u32(data + DATA_NUM_TARGETS, 1);
        logged(&mut e, &log, MEMORY_MANAGER_ALLOCATE, 0);
        e.call(0x00a3_b460, &args![this]);
        assert_eq!(e.mem.u32(this + INTERP_ITEMS), 0);
    }

    #[test]
    fn link_object_sets_the_data_weights_and_interpolators() {
        let mut e = Engine::new();
        interlocked(&mut e);
        let log = Log::default();
        let this = counted(&mut e, 1);
        let data = morph_data(&mut e, 2, 3);
        let block = e.mem.alloc(0x14);
        for (address, eax) in [
            (INTERP_CONTROLLER_LINK_OBJECT, 0),
            (MEMORY_MANAGER_DEALLOCATE, 0),
            (GET_OBJECT_FROM_LINK_ID, data),
            (MEMORY_MANAGER_ALLOCATE, block),
            (VECTOR_CONSTRUCTOR_ITERATOR, 0),
            (INTERPOLATOR_COUNT, 2),
            (MORPH_TARGET_INTERPOLATOR, 0x4400),
            (SET_INTERP, 0),
            (KEY_TIMES, 0),
        ] {
            logged(&mut e, &log, address, eax);
        }
        e.mem.set_f32(this + LO_KEY_TIME, 1.0);
        e.mem.set_f32(this + HI_KEY_TIME, 2.0);
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + STREAM_VERSION, 0x0401_000c);
        e.call(0x00a3_b5a0, &args![this, stream]);
        assert_eq!(e.mem.u32(this + MORPH_DATA), data);
        assert_eq!(e.mem.f32(block + 4 + ITEM_WEIGHT), 0.5);
        assert_eq!(e.mem.f32(block + 12 + ITEM_WEIGHT), 1.5);
        let targets = e.mem.u32(data + DATA_TARGETS);
        assert_eq!(
            calls(&log, MORPH_TARGET_INTERPOLATOR),
            [vec![targets], vec![targets + 0x10]]
        );
        assert_eq!(
            calls(&log, SET_INTERP),
            [vec![this, 0x4400, 0], vec![this, 0x4400, 1]]
        );
        assert_eq!(
            calls(&log, KEY_TIMES),
            [vec![this, 1.0f32.to_bits(), 2.0f32.to_bits()]]
        );
        // Between 0x0a010068 and 0x14010003: the linked objects; no key
        // times.
        logged(&mut e, &log, GET_NUMBER_OF_LINK_IDS, 1);
        e.mem.set_u32(stream + STREAM_VERSION, 0x0a01_0068);
        log.borrow_mut().clear();
        e.call(0x00a3_b5a0, &args![this, stream]);
        assert_eq!(calls(&log, SET_INTERP), [vec![this, data, 0]]);
        assert!(calls(&log, KEY_TIMES).is_empty());
        // New files: no rebuild, no interpolators.
        e.mem.set_u32(stream + STREAM_VERSION, 0x1401_0003);
        log.borrow_mut().clear();
        e.call(0x00a3_b5a0, &args![this, stream]);
        assert!(calls(&log, VECTOR_CONSTRUCTOR_ITERATOR).is_empty());
        assert!(calls(&log, SET_INTERP).is_empty());
    }

    #[test]
    fn fn_00a3ba10_sets_the_data_and_notifies_on_request() {
        let mut e = Engine::new();
        interlocked(&mut e);
        let log = Log::default();
        let this = counted(&mut e, 1);
        let data = morph_data(&mut e, 0, 0);
        logged(&mut e, &log, AFTER_DATA, 0);
        e.call(0x00a3_ba10, &args![this, data, 0x100u32]);
        assert_eq!(e.mem.u32(this + MORPH_DATA), data);
        assert_eq!(e.mem.u32(data + 4), 2);
        assert!(log.borrow().is_empty());
        e.call(0x00a3_ba10, &args![this, data, 1u32]);
        assert_eq!(calls(&log, AFTER_DATA), [vec![this]]);
    }

    #[test]
    fn create_clone_copies_flags_data_and_cloned_interpolators() {
        let mut e = Engine::new();
        interlocked(&mut e);
        let log = Log::default();
        e.map(ALWAYS_UPDATE_CLONES, 4);
        e.mem.set_u8(ALWAYS_UPDATE_CLONES, 1);
        let (this, _, _, items) = controller(&mut e, 2);
        let interpolator = counted(&mut e, 1);
        let clone = counted(&mut e, 0);
        e.mem.set_u32(items + 8, interpolator);
        e.mem.set_f32(items + 12, 0.75);
        e.mem.set_u16(this + MORPHER_FLAGS, 0x0c);
        let block = e.mem.alloc(0x44);
        let destination_items = e.mem.alloc(0x14) + 4;
        let l = log.clone();
        e.register_double(INTERP_CONTROLLER_CONSTRUCT, move |e, a| {
            l.borrow_mut()
                .push((INTERP_CONTROLLER_CONSTRUCT, a.to_vec()));
            e.mem.set_u32(a[0] + TARGET, 0);
            ret(a[0])
        });
        // The rebuild in 00a3ba10 hands the destination its items.
        let l = log.clone();
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, a| {
            l.borrow_mut().push((MEMORY_MANAGER_ALLOCATE, a.to_vec()));
            ret(destination_items - 4)
        });
        for (address, eax) in [
            (NI_ALLOCATE, block),
            (INTERP_CONTROLLER_COPY_MEMBERS, 0),
            (VECTOR_CONSTRUCTOR_ITERATOR, 0),
            (INTERPOLATOR_COUNT, 2),
            (OBJECT_CLONE, clone),
        ] {
            logged(&mut e, &log, address, eax);
        }
        assert_eq!(e.call(0x00a3_bb80, &args![this, 0x9000u32]).u32(), block);
        assert_eq!(e.mem.u32(block), NI_GEOM_MORPHER_CONTROLLER_VTABLE);
        assert_eq!(e.mem.u16(block + MORPHER_FLAGS), 0x0c);
        assert_eq!(
            calls(&log, INTERP_CONTROLLER_COPY_MEMBERS),
            [vec![this, block, 0x9000]]
        );
        assert_eq!(e.mem.u8(this + ALWAYS_UPDATE), 1);
        assert_eq!(e.mem.u8(block + ALWAYS_UPDATE), 1);
        assert_eq!(calls(&log, OBJECT_CLONE), [vec![interpolator, 0x9000]]);
        assert_eq!(e.mem.u32(destination_items + 8), clone);
        assert_eq!(e.mem.u32(clone + 4), 1);
        assert_eq!(e.mem.f32(destination_items + 12), 0.75);
        // CreateObject: an empty controller.
        let fresh = e.mem.alloc(0x44);
        e.mem.write(fresh, &[0xaa; 0x44]);
        logged(&mut e, &log, NI_ALLOCATE, fresh);
        assert_eq!(e.call(0x00a3_b550, &args![]).u32(), fresh);
        assert_eq!(
            e.mem.bytes(fresh + 0x34, 0x10),
            [0, 0, 0xaa, 0xaa, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        logged(&mut e, &log, NI_ALLOCATE, 0);
        assert_eq!(e.call(0x00a3_b550, &args![]).u32(), 0);
    }
}

#[cfg(test)]
mod n004_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// A double at `address` that logs its words and answers `eax`.
    fn logged(e: &mut Engine, log: &Log, address: u32, eax: u32) {
        let log = log.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            ret(eax)
        });
    }

    /// A double at `address` that answers the cell's current value.
    fn answers(e: &mut Engine, address: u32, value: &Rc<Cell<u32>>) {
        let value = value.clone();
        e.register_double(address, move |_, _| ret(value.get()));
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An object of `size` bytes whose vtable has `slots` set to the given targets.
    fn object(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for (slot, target) in slots {
            e.mem.set_u32(vtable + slot, *target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// Morph data with `count` targets of 16 bytes and 5 vertices per target.
    fn morph_data(e: &mut Engine, count: u32) -> u32 {
        let data = e.mem.alloc(0x18);
        let targets = e.mem.alloc(0x10 * count.max(1));
        e.mem.set_u32(data + DATA_NUM_TARGETS, count);
        e.mem.set_u32(data + DATA_NUM_VERTS, 5);
        e.mem.set_u32(data + DATA_TARGETS, targets);
        data
    }

    #[test]
    fn fn_00a3a730_queues_through_slot_a0() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, 0x00ee_1000, 0x0101);
        let owner = object(&mut e, 8, &[(SLOT_OWNER_QUEUE, 0x00ee_1000)]);
        let result = e.call(0x00a3_a730, &args![owner, 0x5000u32, 2u32]).u32();
        assert_eq!(result, 0x0101);
        assert_eq!(calls(&log, 0x00ee_1000), [vec![owner, 0, 0x5000, 2, 1]]);
    }

    #[test]
    fn set_target_clears_rejected_targets_and_notifies_with_data() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, TIME_CONTROLLER_SET_TARGET, 0);
        logged(&mut e, &log, 0x00ee_1100, 0);
        let check = Rc::new(Cell::new(0u32));
        answers(&mut e, 0x00ee_1200, &check);
        let this = object(&mut e, 0x44, &[(SLOT_AFTER_DATA, 0x00ee_1100)]);
        let target = object(&mut e, 0xc0, &[(SLOT_TARGET_CHECK, 0x00ee_1200)]);
        e.call(0x00a3_a750, &args![this, target]);
        assert_eq!(calls(&log, TIME_CONTROLLER_SET_TARGET), [vec![this, 0]]);
        check.set(0x100);
        e.call(0x00a3_a750, &args![this, target]);
        assert!(calls(&log, 0x00ee_1100).is_empty());
        e.mem.set_u32(this + MORPH_DATA, 0x7000);
        e.call(0x00a3_a750, &args![this, target]);
        assert_eq!(
            calls(&log, TIME_CONTROLLER_SET_TARGET),
            [vec![this, 0], vec![this, target], vec![this, target]]
        );
        assert_eq!(calls(&log, 0x00ee_1100), [vec![this]]);
        assert_eq!(e.mem.u8(this + BOUND_CALCULATED), 1);
    }

    #[test]
    fn guarantee_time_range_passes_quiet_copies_to_each_interpolator() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, 0x00ee_1300, 0);
        let interpolator = object(&mut e, 8, &[(SLOT_GUARANTEE_TIME_RANGE, 0x00ee_1300)]);
        e.register_double(0x00ee_1400, move |_, a| {
            ret(if a[1] == 0 { interpolator } else { 0 })
        });
        let this = object(&mut e, 0x44, &[(SLOT_GET_INTERPOLATOR, 0x00ee_1400)]);
        let data = morph_data(&mut e, 2);
        e.mem.set_u32(this + MORPH_DATA, data);
        e.call(0x00a3_a7a0, &args![this, 0x7f80_0001u32, 2.5f32]);
        assert_eq!(
            calls(&log, 0x00ee_1300),
            [vec![interpolator, 0x7fc0_0001, 2.5f32.to_bits()]]
        );
        // No targets: no interpolator is asked for.
        e.mem.set_u32(data + DATA_NUM_TARGETS, 0);
        e.call(0x00a3_a7a0, &args![this, 0u32, 0u32]);
        assert_eq!(calls(&log, 0x00ee_1300).len(), 1);
    }

    #[test]
    fn register_streamables_registers_the_data_and_the_interpolators() {
        let mut e = Engine::new();
        let log = Log::default();
        let base = Rc::new(Cell::new(0x100u32));
        answers(&mut e, BASE_REGISTER_STREAMABLES, &base);
        logged(&mut e, &log, 0x00ee_1500, 1);
        let data = object(&mut e, 0x18, &[(SLOT_REGISTER_STREAMABLES, 0x00ee_1500)]);
        let interpolator = object(&mut e, 8, &[(SLOT_REGISTER_STREAMABLES, 0x00ee_1500)]);
        e.register_double(0x00ee_1600, move |_, a| {
            ret(if a[1] == 1 { interpolator } else { 0 })
        });
        e.register(0x00ee_1700, |_, _| ret(0xffff_0002));
        let this = object(
            &mut e,
            0x44,
            &[
                (SLOT_GET_INTERPOLATOR, 0x00ee_1600),
                (SLOT_INTERPOLATOR_COUNT, 0x00ee_1700),
            ],
        );
        e.mem.set_u32(this + MORPH_DATA, data);
        let stream = 0x6000u32;
        assert_eq!(e.call(0x00a3_a7f0, &args![this, stream]).u8(), 0);
        assert!(calls(&log, 0x00ee_1500).is_empty());
        base.set(1);
        assert_eq!(e.call(0x00a3_a7f0, &args![this, stream]).u8(), 1);
        assert_eq!(
            calls(&log, 0x00ee_1500),
            [vec![data, stream], vec![interpolator, stream]]
        );
    }

    #[test]
    fn get_interpolator_id_reads_the_target_name() {
        let mut e = Engine::new();
        let log = Log::default();
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x0abc_0000);
        logged(&mut e, &log, MORPH_TARGET_NAME, name);
        let this = e.mem.alloc(0x44);
        let data = morph_data(&mut e, 2);
        e.mem.set_u32(this + MORPH_DATA, data);
        let targets = e.mem.u32(data + DATA_TARGETS);
        assert_eq!(e.call(0x00a3_a870, &args![this, 1u32]).u32(), 0x0abc_0000);
        assert_eq!(e.call(0x00a3_a870, &args![this, 2u32]).u32(), 0x0abc_0000);
        assert_eq!(
            calls(&log, MORPH_TARGET_NAME),
            [vec![targets + 0x10], vec![0]]
        );
    }

    #[test]
    fn create_pose_interpolator_poses_at_the_item_weight() {
        let mut e = Engine::new();
        let log = Log::default();
        let block = Rc::new(Cell::new(0x9000u32));
        answers(&mut e, NI_ALLOCATE, &block);
        logged(&mut e, &log, FLOAT_INTERPOLATOR_CONSTRUCT_VALUE, 0x9000);
        let this = e.mem.alloc(0x44);
        assert_eq!(e.call(0x00a3_a900, &args![this, 3u32]).u32(), 0x9000);
        let items = e.mem.alloc(0x20);
        e.mem.set_u32(this + INTERP_ITEMS, items);
        e.mem.set_u32(items + 0xc, 0x7f80_0005);
        assert_eq!(e.call(0x00a3_a900, &args![this, 1u32]).u32(), 0x9000);
        assert_eq!(
            calls(&log, FLOAT_INTERPOLATOR_CONSTRUCT_VALUE),
            [vec![0x9000, 0], vec![0x9000, 0x7fc0_0005]]
        );
        block.set(0);
        assert_eq!(e.call(0x00a3_a900, &args![this, 1u32]).u32(), 0);
        assert_eq!(calls(&log, FLOAT_INTERPOLATOR_CONSTRUCT_VALUE).len(), 2);
    }

    #[test]
    fn reset_time_extrema_widens_to_the_interpolator_ranges() {
        let mut e = Engine::new();
        e.map(EXTREMA_START_LO, 4);
        e.map(EXTREMA_START_HI, 4);
        e.mem.set_f32(EXTREMA_START_LO, f32::MAX);
        e.mem.set_f32(EXTREMA_START_HI, -f32::MAX);
        let ranges = [(1.0f32, 4.0f32), (0.5, f32::NAN), (2.0, 6.0)];
        let mut interpolators = vec![];
        for (k, (start, end)) in ranges.iter().enumerate() {
            let target = 0x00ee_1800 + 0x10 * k as u32;
            let (start, end) = (*start, *end);
            e.register_double(target, move |e, a| {
                assert_eq!(e.mem.f32(a[1]), f32::MAX);
                assert_eq!(e.mem.f32(a[2]), -f32::MAX);
                e.mem.set_f32(a[1], start);
                e.mem.set_f32(a[2], end);
                Ret::default()
            });
            let interpolator = object(&mut e, 8, &[(SLOT_GET_ACTIVE_TIME_RANGE, target)]);
            interpolators.push(interpolator);
        }
        e.register_double(0x00ee_1900, move |_, a| {
            ret(interpolators.get(a[1] as usize).copied().unwrap_or(0))
        });
        let this = object(&mut e, 0x44, &[(SLOT_GET_INTERPOLATOR, 0x00ee_1900)]);
        let data = morph_data(&mut e, 4);
        e.mem.set_u32(this + MORPH_DATA, data);
        e.mem.set_u8(this + FLAGS, 0x20);
        e.call(0x00a3_a950, &args![this]);
        assert_eq!(e.mem.u32(this + LO_KEY_TIME), 0);
        e.mem.set_u8(this + FLAGS, 0);
        e.call(0x00a3_a950, &args![this]);
        assert_eq!(e.mem.f32(this + LO_KEY_TIME), 0.5);
        assert_eq!(e.mem.f32(this + HI_KEY_TIME), 6.0);
        // Without morph data nothing is asked and the start values stay.
        e.mem.set_u32(this + MORPH_DATA, 0);
        e.call(0x00a3_a950, &args![this]);
        assert_eq!(e.mem.f32(this + LO_KEY_TIME), f32::MAX);
        assert_eq!(e.mem.f32(this + HI_KEY_TIME), -f32::MAX);
    }

    #[test]
    fn on_pre_display_marks_the_data_and_clears_the_flag() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, MARK_AS_CHANGED, 0);
        logged(&mut e, &log, 0x00ee_1a00, 0);
        let geometry = object(&mut e, 0x40, &[(SLOT_GEOMETRY_DATA_A0, 0x00ee_1a00)]);
        let target = e.mem.alloc(0xc0);
        e.mem.set_u32(target + GEOMETRY_DATA, geometry);
        let this = e.mem.alloc(0x44);
        // Without a target, or without the flag, nothing happens.
        e.mem.set_u8(this + NEED_PRE_DISPLAY, 1);
        e.call(0x00a3_aba0, &args![this]);
        e.mem.set_u32(this + TARGET, target);
        e.mem.set_u8(this + NEED_PRE_DISPLAY, 0);
        e.call(0x00a3_aba0, &args![this]);
        assert!(calls(&log, MARK_AS_CHANGED).is_empty());
        e.mem.set_u8(this + NEED_PRE_DISPLAY, 1);
        e.mem.set_u16(this + MORPHER_FLAGS, 1);
        e.call(0x00a3_aba0, &args![this]);
        assert!(calls(&log, 0x00ee_1a00).is_empty());
        assert_eq!(e.mem.u8(this + NEED_PRE_DISPLAY), 0);
        e.mem.set_u8(this + NEED_PRE_DISPLAY, 1);
        e.mem.set_u32(geometry + 0x24, 0x1234);
        e.call(0x00a3_aba0, &args![this]);
        assert_eq!(
            calls(&log, MARK_AS_CHANGED),
            [vec![geometry, 1], vec![geometry, 1]]
        );
        assert_eq!(calls(&log, 0x00ee_1a00), [vec![geometry]]);
    }

    #[test]
    fn is_equal_compares_data_interpolators_and_weights() {
        let mut e = Engine::new();
        let base = Rc::new(Cell::new(1u32));
        answers(&mut e, BASE_IS_EQUAL, &base);
        let equal = Rc::new(Cell::new(1u32));
        answers(&mut e, 0x00ee_1b00, &equal);
        let count = Rc::new(Cell::new(2u32));
        answers(&mut e, INTERPOLATOR_COUNT, &count);
        let data = object(&mut e, 0x18, &[(SLOT_IS_EQUAL, 0x00ee_1b00)]);
        let theirs = object(&mut e, 8, &[(SLOT_IS_EQUAL, 0x00ee_1b00)]);
        let ours = object(&mut e, 8, &[(SLOT_IS_EQUAL, 0x00ee_1b00)]);
        let this = e.mem.alloc(0x44);
        let other = e.mem.alloc(0x44);
        e.mem.set_u32(this + MORPH_DATA, data);
        let items = e.mem.alloc(0x10);
        let other_items = e.mem.alloc(0x10);
        e.mem.set_u32(this + INTERP_ITEMS, items);
        e.mem.set_u32(other + INTERP_ITEMS, other_items);
        e.mem.set_u32(items, ours);
        e.mem.set_u32(other_items, theirs);
        e.mem.set_f32(items + 4, 0.5);
        e.mem.set_f32(other_items + 4, 0.5);
        assert!(e.call(0x00a3_ad00, &args![this, other]).bool());
        // A NaN weight never compares equal.
        e.mem.set_f32(items + 0xc, f32::NAN);
        e.mem.set_f32(other_items + 0xc, f32::NAN);
        assert!(!e.call(0x00a3_ad00, &args![this, other]).bool());
        count.set(1);
        assert!(e.call(0x00a3_ad00, &args![this, other]).bool());
        e.mem.set_u32(items, 0);
        assert!(!e.call(0x00a3_ad00, &args![this, other]).bool());
        e.mem.set_u32(other_items, 0);
        assert!(e.call(0x00a3_ad00, &args![this, other]).bool());
        e.mem.set_u32(items, ours);
        assert!(!e.call(0x00a3_ad00, &args![this, other]).bool());
        count.set(0);
        assert!(e.call(0x00a3_ad00, &args![this, other]).bool());
        equal.set(0);
        assert!(!e.call(0x00a3_ad00, &args![this, other]).bool());
        equal.set(1);
        base.set(0);
        assert!(!e.call(0x00a3_ad00, &args![this, other]).bool());
    }

    #[test]
    fn get_interpolator_count_keeps_the_pointer_high_half() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x44);
        assert_eq!(e.call(0x00a3_adc0, &args![this]).u32(), 0);
        let data = morph_data(&mut e, 7);
        e.mem.set_u32(this + MORPH_DATA, data);
        assert_eq!(
            e.call(0x00a3_adc0, &args![this]).u32(),
            (data & 0xffff_0000) | 7
        );
    }

    #[test]
    fn synchronize_pose_interpolator_sets_the_cast_interpolator_pose() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, DYNAMIC_CAST, 0x8800);
        logged(&mut e, &log, FLOAT_INTERPOLATOR_SET_POSE_VALUE, 0);
        let this = e.mem.alloc(0x44);
        let items = e.mem.alloc(0x10);
        e.mem.set_u32(this + INTERP_ITEMS, items);
        e.mem.set_f32(items + 0xc, 0.75);
        e.call(0x00a3_ae50, &args![this, 0u32, 1u32]);
        e.call(0x00a3_ae50, &args![this, 0x4400u32, 1u32]);
        assert_eq!(
            calls(&log, DYNAMIC_CAST),
            [vec![0x4400, NI_FLOAT_INTERPOLATOR_RTTI]]
        );
        assert_eq!(
            calls(&log, FLOAT_INTERPOLATOR_SET_POSE_VALUE),
            [vec![0, 0.75f32.to_bits()], vec![0x8800, 0.75f32.to_bits()]]
        );
    }

    #[test]
    fn target_is_required_type_walks_the_rtti_chain() {
        let mut e = Engine::new();
        let base = e.mem.alloc(8);
        e.mem.set_u32(base + 4, NI_GEOMETRY_RTTI);
        let other = e.mem.alloc(8);
        let record = Rc::new(Cell::new(base));
        answers(&mut e, 0x00ee_1c00, &record);
        let target = object(&mut e, 8, &[(SLOT_GET_RTTI, 0x00ee_1c00)]);
        e.map(NI_GEOMETRY_RTTI, 8);
        let this = e.mem.alloc(0x44);
        assert!(!e.call(0x00a3_afe0, &args![this]).bool());
        e.mem.set_u32(this + TARGET, target);
        assert!(e.call(0x00a3_afe0, &args![this]).bool());
        record.set(other);
        assert!(!e.call(0x00a3_afe0, &args![this]).bool());
    }

    type Writes = Rc<RefCell<Vec<(Vec<u8>, u32, u32)>>>;

    #[test]
    fn save_binary_writes_flags_data_and_items() {
        let mut e = Engine::new();
        let log = Log::default();
        logged(&mut e, &log, BASE_SAVE_BINARY, 0);
        logged(&mut e, &log, 0x00ee_1d00, 0);
        let written = Writes::default();
        let record = written.clone();
        e.register_double(0x00ee_1e00, move |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            record.borrow_mut().push((bytes, e.mem.u32(a[3]), a[4]));
            ret(a[2])
        });
        let stream = object(&mut e, 0x260, &[(SLOT_SAVE_LINK_ID, 0x00ee_1d00)]);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(output + BINARY_STREAM_WRITE, 0x00ee_1e00);
        e.mem.set_u32(stream + STREAM_OUTPUT, output);
        let this = e.mem.alloc(0x44);
        e.mem.set_u16(this + MORPHER_FLAGS, 0x0102);
        e.mem.set_u8(this + ALWAYS_UPDATE, 1);
        let data = morph_data(&mut e, 2);
        e.mem.set_u32(this + MORPH_DATA, data);
        let items = e.mem.alloc(0x10);
        e.mem.set_u32(this + INTERP_ITEMS, items);
        e.mem.set_u32(items, 0x5100);
        e.mem.set_f32(items + 4, 1.5);
        e.mem.set_u32(items + 8, 0x5200);
        e.mem.set_f32(items + 0xc, 2.5);
        e.call(0x00a3_b350, &args![this, stream]);
        assert_eq!(calls(&log, BASE_SAVE_BINARY), [vec![this, stream]]);
        assert_eq!(
            calls(&log, 0x00ee_1d00),
            [
                vec![stream, data],
                vec![stream, 0x5100],
                vec![stream, 0x5200]
            ]
        );
        assert_eq!(
            *written.borrow(),
            [
                (vec![2, 1], 2, 1),
                (vec![1], 1, 1),
                (vec![2, 0, 0, 0], 4, 1),
                (1.5f32.to_le_bytes().to_vec(), 4, 1),
                (2.5f32.to_le_bytes().to_vec(), 4, 1),
            ]
        );
        assert_eq!(e.mem.u32(output + BINARY_STREAM_POSITION), 15);
    }

    #[test]
    fn get_viewer_strings_lists_each_target_with_its_value() {
        let mut e = Engine::new();
        let log = Log::default();
        e.map(NI_GEOM_MORPHER_CONTROLLER_RTTI, 4);
        e.mem.set_u32(NI_GEOM_MORPHER_CONTROLLER_RTTI, 0x0155_0000);
        logged(&mut e, &log, NI_GET_VIEWER_STRING, 0x0166_0000);
        e.register(NI_GET_VIEWER_STRING_UNSIGNED, |_, a| ret(a[1]));
        e.register(VIEWER_STRINGS_SET_AT, |_, _| Ret::default());
        e.register(VIEWER_STRINGS_GROW, |e, a| {
            // A bigger data block holding the old entries, and the new capacity.
            let data = e.mem.alloc(4 * a[1]);
            let old = e.mem.u32(a[0] + 4);
            let size = u32::from(e.mem.u16(a[0] + 0xa));
            for i in 0..size {
                let word = e.mem.u32(old + 4 * i);
                e.mem.set_u32(data + 4 * i, word);
            }
            e.mem.set_u32(a[0] + 4, data);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        let formats = Log::default();
        let record = formats.clone();
        e.register_double(SPRINTF, move |e, a| {
            record.borrow_mut().push((a[1], a[2..].to_vec()));
            e.mem.set_cstr(a[0], b"ab");
            Ret::default()
        });
        let next = Rc::new(Cell::new(0x0177_0000u32));
        let blocks = next.clone();
        e.register_double(NI_ALLOC, move |_, a| {
            assert_eq!(a[0], 5);
            blocks.set(blocks.get() + 0x10);
            ret(blocks.get())
        });
        logged(&mut e, &log, STRCPY_S, 0);
        let name = e.mem.alloc(4);
        e.mem.set_u32(name, 0x0188_0000);
        e.register_double(MORPH_TARGET_NAME, move |_, _| ret(name));
        // Interpolators: a blend float one (value 0.5), a float one (value 2.0), none.
        let blend_rtti = e.mem.alloc(8);
        e.mem
            .set_u32(blend_rtti + 4, NI_BLEND_FLOAT_INTERPOLATOR_RTTI);
        e.map(NI_BLEND_FLOAT_INTERPOLATOR_RTTI, 8);
        e.map(NI_FLOAT_INTERPOLATOR_RTTI, 8);
        e.register_double(0x00ee_1f00, move |_, _| ret(blend_rtti));
        e.register(0x00ee_1f10, |_, _| ret(NI_FLOAT_INTERPOLATOR_RTTI));
        let blend = object(&mut e, 0x40, &[(SLOT_GET_RTTI, 0x00ee_1f00)]);
        e.mem.set_f32(blend + BLEND_FLOAT_VALUE, 0.5);
        let plain = object(&mut e, 0x40, &[(SLOT_GET_RTTI, 0x00ee_1f10)]);
        e.mem.set_f32(plain + FLOAT_VALUE, 2.0);
        let list = [blend, plain, 0];
        e.register_double(0x00ee_1f20, move |_, a| ret(list[a[1] as usize]));
        let this = object(&mut e, 0x44, &[(SLOT_GET_INTERPOLATOR, 0x00ee_1f20)]);
        let data = morph_data(&mut e, 3);
        e.mem.set_u32(this + MORPH_DATA, data);
        let strings = e.mem.alloc(0x10);
        let first = e.mem.alloc(8);
        e.mem.set_u32(strings + 4, first);
        e.mem.set_u16(strings + 8, 2);
        e.mem.set_u16(strings + 0xe, 1);
        e.call(0x00a3_b700, &args![this, strings]);
        let half = 0.5f64.to_bits();
        let two = 2.0f64.to_bits();
        assert_eq!(
            *formats.borrow(),
            [
                (
                    FORMAT_INDEX_NAME_WEIGHT,
                    vec![0, 0x0188_0000, half as u32, (half >> 32) as u32]
                ),
                (
                    FORMAT_INDEX_NAME_WEIGHT,
                    vec![1, 0x0188_0000, two as u32, (two >> 32) as u32]
                ),
                (FORMAT_INDEX_NAME, vec![2, 0x0188_0000]),
            ]
        );
        assert_eq!(calls(&log, NI_GET_VIEWER_STRING), [vec![0x0155_0000]]);
        assert_eq!(calls(&log, STRCPY_S).len(), 3);
        // The doubled SetAt stored nothing, so the copies went into slots 0..2.
        assert_eq!(e.mem.u16(strings + 0xa), 3);
        assert_eq!(e.mem.u16(strings + 0xc), 3);
        let stored = e.mem.u32(strings + 4);
        assert_eq!(
            [
                e.mem.u32(stored),
                e.mem.u32(stored + 4),
                e.mem.u32(stored + 8)
            ],
            [0x0177_0010, 0x0177_0020, 0x0177_0030]
        );
    }
}
