//! `fallout shared/pathfinding/pathingsolution.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `PathingSolution` (Xbox PDB; 0x44 bytes on PC) is what the pathing task
//! hands back: two arrays of nodes, `VirtualPathingNodes` (0x30-byte
//! [`VirtualPathingNode`]s, +0x08) and `CurrentPathingNodes` (0x3C-byte
//! [`PathingNode`]s, +0x20), the door ids the path crosses (`DoorIDs`, a
//! `BSSimpleArray` of ids at +0x30, which the shared layout in `pathfind.rs`
//! does not declare), the first and last loaded virtual node indices and
//! `bIncompletePath`. The Xbox build's debug data between `NiRefObject` and
//! the node arrays is missing on PC.
//!
//! The file also holds what the game does with the current nodes as a path:
//! the nodes are joined by curves whose parameter runs from node to node
//! (`position` 2.5 is half way from node 2 to node 3). The segment between
//! two nodes is a cubic blend of the two node positions and tangents (the
//! four weights `1 - h01`, `t^3 - 2 t^2 + t`, `-2 t^3 + 3 t^2` and
//! `t^3 - t^2`, which is a cubic Hermite blend). The z tangent is not used:
//! `fn_006e7d60` passes zero for it. The 0x44-byte object that holds the
//! weights and the two end nodes is called [`CurveSegment`] here; the Xbox
//! PDB has no class by that name (unconfirmed which class it is), so the
//! name is a description of what the code does with it.
//!
//! How the translations read the game code:
//! - A node's position is fetched in two steps: `00717e50` (the node's
//!   `Location`, a `PathingLocation`, at +4) and `005c3420` (copies that
//!   location's `Location` point, at +4 of it, to the given buffer). The
//!   decompiler hangs the buffer on the first call; the disassembly shows
//!   it belongs to the second.
//! - Local structures the game keeps on its stack are zeroed blocks from
//!   `Engine::with_stack`, addressed by their `EBP`-relative role.
//! - x87: floats are computed in `f64` and rounded to `f32` at each `float`
//!   store, as `docs/ENGINE_CRATE.md` says. Functions whose `ST0` result the
//!   caller adds to a `float` accumulator before storing are read with
//!   `.f64()`.
//! - The compiler's exception-unwinding frames (`FS:[0]`) are not
//!   translated; every function that has one carries a
//!   `// Translation incomplete:` marker.
//! - Callees outside this file are called by address: the node and
//!   location accessors, the `NiPoint3` arithmetic, the save and load
//!   buffer, the `BSSimpleArray` helpers shared by other element types and
//!   the placement-construction loops (`006e95d0`, `006e9700`) which belong
//!   to other units.

// The x87 comparisons (FCOMPP then a flag test) treat NaN as "less" or "not
// greater"; the negated forms below keep that, so the lint must not rewrite them.
#![allow(clippy::neg_cmp_op_on_partial_ord)]

#[allow(unused_imports)]
use crate::prelude::*;

use super::pathfind::{PathingLocation, PathingSolution};
use crate::types::{BSSimpleArray, NiPoint3};

// ---------------------------------------------------------------------------
// Layouts and constants
// ---------------------------------------------------------------------------

layout! {
    /// `VirtualPathingNode` (Xbox PDB), 0x30 bytes.
    pub struct VirtualPathingNode: 0x30 {
        /// `iFlags` (Xbox PDB).
        0x00 iFlags: u32,
        /// `Location` (Xbox PDB): a `PathingLocation`.
        0x04 Location: Inline<PathingLocation>,
        /// `pDoorRef` (Xbox PDB): `TESObjectREFR*`.
        0x2C pDoorRef: Ptr,
    }

    /// `PathingNode` (Xbox PDB), 0x3C bytes.
    pub struct PathingNode: 0x3C {
        /// `iFlags` (Xbox PDB); bit 2 (`4`) is tested by `006ca0c0`.
        0x00 iFlags: u32,
        /// `Location` (Xbox PDB): a `PathingLocation`.
        0x04 Location: Inline<PathingLocation>,
        /// `Tangent` (Xbox PDB): a `NiPoint3`, at +0x2C..+0x38.
        0x2C Tangent: Inline<NiPoint3>,
        /// `pDoorRef` (Xbox PDB): `TESObjectREFR*`.
        0x38 pDoorRef: Ptr,
    }

    /// The 0x44-byte curve object of the file: the parameter and the four
    /// blend weights, then the start node's position and tangent (0x14) and
    /// the end node's (0x2C), each 24 bytes (see the module comment).
    pub struct CurveSegment: 0x44 {
        /// The parameter the weights were computed for (`0` at the start
        /// node, `1` at the end node).
        0x00 fParameter: f32,
        /// Weight of the start position, `1 - fWeightEndPosition`.
        0x04 fWeightStartPosition: f32,
        /// Weight of the start tangent.
        0x08 fWeightStartTangent: f32,
        /// Weight of the end position.
        0x0C fWeightEndPosition: f32,
        /// Weight of the end tangent.
        0x10 fWeightEndTangent: f32,
    }
}

/// Size of a [`VirtualPathingNode`] and of a [`PathingNode`].
const VIRTUAL_NODE_SIZE: u32 = 0x30;
const CURRENT_NODE_SIZE: u32 = 0x3c;
/// Size of the solution on PC.
const SOLUTION_SIZE: u32 = 0x44;
/// `DoorIDs` (Xbox PDB), a `BSSimpleArray<unsigned int,1024>`, at +0x30.
const DOOR_IDS: u32 = 0x30;
/// Start and end node of a [`CurveSegment`] (position then tangent).
const SEGMENT_START: u32 = 0x14;
const SEGMENT_END: u32 = 0x2c;
/// Size of the position-and-tangent block of a node (`fn_006e7e10`).
const SEGMENT_NODE_SIZE: u32 = 0x18;

/// `PathingSolution` vtable (`~NiRefObject` scalar deleting destructor first).
const SOLUTION_VTABLE: u32 = 0x0106_d4bc;
/// Vtables of `BSSimpleArray<VirtualPathingNode,1024>` and
/// `BSSimpleArray<PathingNode,1024>`: scalar deleting destructor, `_Allocate`
/// (slot +4), another function, `_Reallocate`.
const VIRTUAL_ARRAY_VTABLE: u32 = 0x0106_d4d4;
const CURRENT_ARRAY_VTABLE: u32 = 0x0106_d4e8;
/// The zero point (three `float`s) the game copies as a default position.
const ZERO_POINT: u32 = 0x011f_426c;
/// `-2.0` and `3.0` (`double`): the cubic weights of `fn_006e7c40`.
const MINUS_TWO: u32 = 0x0106_d4c8;
const THREE: u32 = 0x0102_1928;
/// `1.0` and `0.5` (`double`).
const ONE: u32 = 0x0101_2070;
const HALF: u32 = 0x0101_1588;
/// `10.0` (`float`): the tolerance `fn_006e8790` hands to `fn_006e9830`.
const TEN: u32 = 0x0101_7b78;
/// `0.99` and `0.01` (`double`): the limits of the search parameter in
/// `fn_006e9830`.
const UPPER_LIMIT: u32 = 0x0106_d508;
const LOWER_LIMIT: u32 = 0x0101_6408;

/// `_ftol2_sse` (`00ec62c0`): truncates the `ST0` value (a leading `f64`
/// argument in the uniform form) to an integer.
const FTOL: u32 = 0x00ec_62c0;
/// `operator delete(ptr, size)` (`00aa1460`, cdecl).
const OPERATOR_DELETE_SIZED: u32 = 0x00aa_1460;
/// `operator new(bytes)` (`00401000`) and `operator delete(ptr)` (`00401030`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The reallocation primitive `(from, bytes)` (cdecl) other arrays use.
const REALLOCATE_BYTES: u32 = 0x0042_f5d0;

/// `BSSimpleArray::iSize` getter (`0044ddc0`; `this` is the array).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// Size of the array at +0x20 of the solution (`008b6800`) and of the one at
/// +0x08 (`005a4320`); both call `0044ddc0`.
const CURRENT_NODE_COUNT: u32 = 0x008b_6800;
const VIRTUAL_NODE_COUNT: u32 = 0x005a_4320;
/// `iSize == 0` (`0076b610`; `this` is the array).
const ARRAY_IS_EMPTY: u32 = 0x0076_b610;
/// `this + 4` (`00717e50`): a node's `Location`.
const NODE_LOCATION: u32 = 0x0071_7e50;
/// `(location, buffer)` (`005c3420`): copies the location's point (+4) to
/// `buffer` and returns `buffer`.
const LOCATION_POINT: u32 = 0x005c_3420;
/// `(a, out, b)` (`00439ef0`): `out = a - b` for points; returns `out`.
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// Length of the point (`00457990`, 3 components) and of its x and y
/// (`00589850`); square root of the sum of squares.
const POINT_LENGTH: u32 = 0x0045_7990;
const POINT_LENGTH_2D: u32 = 0x0058_9850;
/// Sum of squares of the three components (`004a7290`).
const POINT_LENGTH_SQUARED: u32 = 0x004a_7290;
/// `(node)` (`006ca0c0`): `(iFlags & 4) == 4`.
const NODE_FLAG_4: u32 = 0x006c_a0c0;
/// The no-op constructor (`006815c0`, returns `this`).
const TRIVIAL_CONSTRUCT: u32 = 0x0068_15c0;
/// The destructor of [`CurveSegment`] (`0052d240`; it calls the empty
/// function `00483710`).
const SEGMENT_DESTRUCT: u32 = 0x0052_d240;
/// `(float* value, min, max)` (`005330e0`, cdecl): clamps `*value`.
const CLAMP: u32 = 0x0053_30e0;
/// `(solution)` (`006b05a0`): the last current node, or null.
const LAST_CURRENT_NODE: u32 = 0x006b_05a0;
/// `(location, other)` (`006dcce0`, `PathingLocation::operator=`).
const LOCATION_ASSIGN: u32 = 0x006d_cce0;
/// `(array, free)` (`006cefa0`, `006cf120`): empties a `BSSimpleArray` of
/// `VirtualPathingNode`s / `PathingNode`s, releasing the buffer when `free`.
const VIRTUAL_ARRAY_CLEAR: u32 = 0x006c_efa0;
const CURRENT_ARRAY_CLEAR: u32 = 0x006c_f120;
/// `(array, free)` (`008454f0`): the same for the `DoorIDs` array.
const DOOR_IDS_CLEAR: u32 = 0x0084_54f0;
/// `(array)` (`00438690`): the `DoorIDs` array's destructor.
const DOOR_IDS_DESTRUCT: u32 = 0x0043_8690;
/// `~NiRefObject` (`00496910`).
const REF_OBJECT_DESTRUCT: u32 = 0x0049_6910;
/// Element helpers of the node arrays: `(array, newCapacity, oldSize)`
/// reallocates (`006cfe70` for 0x30-byte elements, `006a21c0` for 0x3C),
/// `(array, pointer, count)` destroys elements (`006cfe30`, `006cff20`) and
/// constructs them (`006e95d0`, `006e9700`).
const VIRTUAL_ARRAY_REALLOCATE: u32 = 0x006c_fe70;
const CURRENT_ARRAY_REALLOCATE: u32 = 0x006a_21c0;
const VIRTUAL_ARRAY_DESTROY: u32 = 0x006c_fe30;
const CURRENT_ARRAY_DESTROY: u32 = 0x006c_ff20;
const VIRTUAL_ARRAY_CONSTRUCT: u32 = 0x006e_95d0;
const CURRENT_ARRAY_CONSTRUCT: u32 = 0x006e_9700;
/// The curve point at a parameter: `(segment, out, t)` (`006e9ab0`, the
/// small function that sets the weights with [`fn_006e7c20`] and blends with
/// [`fn_006e7d60`]); belongs to another unit's range.
const SEGMENT_POINT: u32 = 0x006e_9ab0;

/// Save buffer: `(buffer, data, size, flag)` (`00865e50`), the sized value
/// `(buffer, value)` (`00865f60`), `(buffer, formId, flag)` (`00865db0`).
const SAVE_BYTES: u32 = 0x0086_5e50;
const SAVE_SIZED_VALUE: u32 = 0x0086_5f60;
const SAVE_FORM_ID: u32 = 0x0086_5db0;
/// Load buffer: `(buffer, data, size)` (`00864980`), the sized value
/// `(buffer)` (`00864a60`), `(buffer, formIdOut)` (`008648e0`).
const LOAD_BYTES: u32 = 0x0086_4980;
const LOAD_SIZED_VALUE: u32 = 0x0086_4a60;
const LOAD_FORM_ID: u32 = 0x0086_48e0;
/// Node save / load: `(node, buffer)` for the virtual nodes (`006f50b0`,
/// `006f5100`), for the current nodes (`006e2350`, `006e23b0`).
const VIRTUAL_NODE_SAVE: u32 = 0x006f_50b0;
const VIRTUAL_NODE_LOAD: u32 = 0x006f_5100;
const CURRENT_NODE_SAVE: u32 = 0x006e_2350;
const CURRENT_NODE_LOAD: u32 = 0x006e_23b0;
/// The local `BSSimpleArray` of `NavMeshInfo*` the save and load build:
/// constructor `(array)` (`006e3850`), destructor `(array)` (`006e38b0`),
/// `SetSize(array, size, flag)` (`006e3ad0`), element `(array, index)`
/// (`006a1400`), and the element save / load `(info, buffer)` (`006dc890`,
/// `006dc910`).
const INFO_ARRAY_CONSTRUCT: u32 = 0x006e_3850;
const INFO_ARRAY_DESTRUCT: u32 = 0x006e_38b0;
const INFO_ARRAY_SET_SIZE: u32 = 0x006e_3ad0;
const INFO_ARRAY_AT: u32 = 0x006a_1400;
const INFO_SAVE: u32 = 0x006d_c890;
const INFO_LOAD: u32 = 0x006d_c910;
/// `(array, index)` (`006a7ad0`): address of the element of a pointer
/// array; `(array, size, flag)` (`006f2290`): its `SetSize`.
const POINTER_ARRAY_AT: u32 = 0x006a_7ad0;
const POINTER_ARRAY_SET_SIZE: u32 = 0x006f_2290;
/// The first load buffer virtual function (offset +0): its low byte is
/// compared with this to decide whether the door ids were saved.
const LOAD_BUFFER_VERSION_SLOT: u32 = 0;
const DOOR_IDS_VERSION: u32 = 0x12;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// `_ftol2_sse` of an `ST0` value.
fn float_to_int(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL, &args![value]).i32()
}

/// Copies `words` 32-bit words.
fn copy_words(e: &mut Engine, from: u32, to: u32, words: u32) {
    for i in 0..words {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// `00717e50` then `005c3420`: copies the position of the node into
/// `buffer` and returns `buffer`.
fn node_position(e: &mut Engine, node: u32, buffer: u32) -> u32 {
    let location = e.call(NODE_LOCATION, &args![node]).u32();
    e.call(LOCATION_POINT, &args![location, buffer]).u32()
}

/// The number of current nodes (`008b6800`).
fn current_node_count(e: &mut Engine, this: Ptr<PathingSolution>) -> u32 {
    e.call(CURRENT_NODE_COUNT, &args![this]).u32()
}

// ---------------------------------------------------------------------------
// PathingSolution
// ---------------------------------------------------------------------------

// Translated from 006e76f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingSolution::_scalar_deleting_destructor_` (Xbox PDB): destroys the
/// solution and, when bit 0 of `flags` is set, frees its 0x44 bytes.
pub fn pathing_solution_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<PathingSolution>,
    flags: u32,
) -> Ptr<PathingSolution> {
    fn_006e7720(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE_SIZED, &args![this, SOLUTION_SIZE]);
    }
    this
}

// Translated from 006e7720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of `PathingSolution` (unnamed in the map): sets the
/// vtable, empties the three arrays through [`fn_006e77b0`], runs the member
/// destructors of the `DoorIDs`, current-node and virtual-node arrays in
/// that order, then `~NiRefObject`.
// Translation incomplete: 006e7720 - C++ exception unwinding frame (FS:[0], states 3 to 0) not translated.
pub fn fn_006e7720(e: &mut Engine, this: Ptr<PathingSolution>) {
    e.mem.set_u32(this.addr(), SOLUTION_VTABLE);
    fn_006e77b0(e, this);
    e.call(DOOR_IDS_DESTRUCT, &args![this.addr() + DOOR_IDS]);
    fn_006e9260(e, this.at(PathingSolution::CurrentPathingNodes));
    fn_006e9080(e, this.at(PathingSolution::VirtualPathingNodes));
    e.call(REF_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 006e77b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the three arrays (releasing their buffers) and resets the loaded
/// virtual node indices to -1 and `bIncompletePath` to false.
pub fn fn_006e77b0(e: &mut Engine, this: Ptr<PathingSolution>) {
    let virtual_nodes = this.at(PathingSolution::VirtualPathingNodes);
    e.call(VIRTUAL_ARRAY_CLEAR, &args![virtual_nodes, 1u32]);
    let current_nodes = this.at(PathingSolution::CurrentPathingNodes);
    e.call(CURRENT_ARRAY_CLEAR, &args![current_nodes, 1u32]);
    e.call(DOOR_IDS_CLEAR, &args![this.addr() + DOOR_IDS, 1u32]);
    e.set(this, PathingSolution::iFirstLoadedVirtualNodeIndex, -1);
    e.set(this, PathingSolution::iLastLoadedVirtualNodeIndex, -1);
    e.set(this, PathingSolution::bIncompletePath, false);
}

// Translated from 006e7800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Length of the virtual-node path in the plane: the sum of the distances in
/// x and y between each virtual node and the next, skipping the pairs whose
/// first node has bit 2 of its flags set.
pub fn fn_006e7800(e: &mut Engine, this: Ptr<PathingSolution>) -> f32 {
    let mut length: f32 = 0.0;
    let count = e.call(VIRTUAL_NODE_COUNT, &args![this]).u32();
    e.with_stack(0x30, |e, frame| {
        let previous_position = frame.addr();
        let difference = frame.addr() + 0x0c;
        let position = frame.addr() + 0x18;
        let mut index = 1u32;
        while index < count {
            let previous = fn_006e78b0(e, this, index - 1);
            if e.call(NODE_FLAG_4, &args![previous]).bool() {
                index += 1;
                continue;
            }
            let node = fn_006e78b0(e, this, index);
            let from = node_position(e, previous.addr(), previous_position);
            let to = node_position(e, node.addr(), position);
            let delta = e.call(POINT_SUBTRACT, &args![to, difference, from]).u32();
            let step = e.call(POINT_LENGTH_2D, &args![delta]).f64();
            length = (step + length as f64) as f32;
            index += 1;
        }
    });
    length
}

// Translated from 006e78b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The virtual node `index` (element of the array at +0x08).
pub fn fn_006e78b0(
    e: &mut Engine,
    this: Ptr<PathingSolution>,
    index: u32,
) -> Ptr<VirtualPathingNode> {
    fn_006e8f40(e, this.at(PathingSolution::VirtualPathingNodes), index)
}

// Translated from 006e78d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Length of the current-node path: the sum of the 3D distances between each
/// current node and the next.
pub fn fn_006e78d0(e: &mut Engine, this: Ptr<PathingSolution>) -> f32 {
    let mut length: f32 = 0.0;
    let count = e.call(CURRENT_NODE_COUNT, &args![this]).u32();
    e.with_stack(0x30, |e, frame| {
        let previous_position = frame.addr();
        let difference = frame.addr() + 0x0c;
        let position = frame.addr() + 0x18;
        let mut index = 1u32;
        while index < count {
            let previous = fn_006e7970(e, this, index - 1);
            let node = fn_006e7970(e, this, index);
            let from = node_position(e, previous.addr(), previous_position);
            let to = node_position(e, node.addr(), position);
            let delta = e.call(POINT_SUBTRACT, &args![to, difference, from]).u32();
            let step = e.call(POINT_LENGTH, &args![delta]).f64();
            length = (step + length as f64) as f32;
            index += 1;
        }
    });
    length
}

// Translated from 006e7970 (decompiled, FalloutNV.exe 1.4.0.525)
/// The current node `index` (element of the array at +0x20).
pub fn fn_006e7970(e: &mut Engine, this: Ptr<PathingSolution>, index: u32) -> Ptr<PathingNode> {
    fn_006e8f80(e, this.at(PathingSolution::CurrentPathingNodes), index)
}

// Translated from 006e7990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Distance in 3D between current node `index` and the next one, 0 for the
/// last node (or an index beyond it).
pub fn fn_006e7990(e: &mut Engine, this: Ptr<PathingSolution>, index: i32) -> f32 {
    let last = current_node_count(e, this).wrapping_sub(1) as i32;
    if index >= last {
        return 0.0;
    }
    e.with_stack(0x30, |e, frame| {
        let next_position = frame.addr();
        let difference = frame.addr() + 0x0c;
        let position = frame.addr() + 0x18;
        let next = fn_006e7970(e, this, index.wrapping_add(1) as u32);
        let to = node_position(e, next.addr(), next_position);
        let node = fn_006e7970(e, this, index as u32);
        let from = node_position(e, node.addr(), position);
        let delta = e.call(POINT_SUBTRACT, &args![from, difference, to]).u32();
        e.call(POINT_LENGTH, &args![delta]).f32()
    })
}

// Translated from 006e7a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the point at path parameter `parameter` (node index plus the
/// fraction towards the next node) of the current nodes to `out`: the last
/// node's position when the parameter is at or beyond it, otherwise the
/// curve between the two nodes at the fraction.
// Translation incomplete: 006e7a10 - C++ exception unwinding frame (FS:[0]) not translated.
pub fn fn_006e7a10(e: &mut Engine, this: Ptr<PathingSolution>, parameter: f32, out: Ptr) {
    let index = float_to_int(e, parameter as f64);
    let fraction = (parameter as f64 - (index as u32) as f64) as f32;
    let last = current_node_count(e, this).wrapping_sub(1) as i32;
    if index >= last {
        e.with_stack(0x0c, |e, position| {
            let node = fn_006e7970(e, this, last as u32);
            let point = node_position(e, node.addr(), position.addr());
            copy_words(e, point, out.addr(), 3);
        });
        return;
    }
    e.with_stack(0x44 + 2 * SEGMENT_NODE_SIZE + 0x0c, |e, frame| {
        let segment = Ptr::<CurveSegment>::new(frame.addr());
        let start_block = frame.addr() + 0x44;
        let end_block = start_block + SEGMENT_NODE_SIZE;
        let result_block = end_block + SEGMENT_NODE_SIZE;
        fn_006e7b60(e, segment);
        fn_006e7c20(e, segment, fraction);
        let next = fn_006e7970(e, this, index.wrapping_add(1) as u32);
        let end = fn_006e7e10(e, next, Ptr::new(end_block));
        let node = fn_006e7970(e, this, index as u32);
        let start = fn_006e7e10(e, node, Ptr::new(start_block));
        let point = fn_006e7cc0(e, segment, Ptr::new(result_block), start, end);
        copy_words(e, point.addr(), out.addr(), 3);
        e.call(SEGMENT_DESTRUCT, &args![segment]);
    });
}

// Translated from 006e7b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of [`CurveSegment`]: the no-op base constructor, then the
/// start and end blocks set to the zero point and zero tangent. The
/// parameter and weights are left as they were.
// Translation incomplete: 006e7b60 - C++ exception unwinding frame (FS:[0]) not translated.
pub fn fn_006e7b60(e: &mut Engine, this: Ptr<CurveSegment>) -> Ptr<CurveSegment> {
    e.call(TRIVIAL_CONSTRUCT, &args![this]);
    fn_006e7bd0(e, Ptr::new(this.addr() + SEGMENT_START));
    fn_006e7bd0(e, Ptr::new(this.addr() + SEGMENT_END));
    this
}

// Translated from 006e7bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets a 24-byte position-and-tangent block to the zero point twice.
pub fn fn_006e7bd0(e: &mut Engine, this: Ptr) -> Ptr {
    copy_words(e, ZERO_POINT, this.addr(), 3);
    copy_words(e, ZERO_POINT, this.addr() + 0x0c, 3);
    this
}

// Translated from 006e7c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the curve parameter (forwards to [`fn_006e7c40`]).
pub fn fn_006e7c20(e: &mut Engine, this: Ptr<CurveSegment>, parameter: f32) {
    fn_006e7c40(e, this, parameter)
}

// Translated from 006e7c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the parameter `t` and its four cubic weights: `t^2` and `t^3`
/// are rounded to `float`, the end position weight is `-2 t^3 + 3 t^2`, the
/// start position weight `1` minus that, the end tangent weight
/// `t^3 - t^2` and the start tangent weight `t^3 - 2 t^2 + t`.
pub fn fn_006e7c40(e: &mut Engine, this: Ptr<CurveSegment>, t: f32) {
    // Every value goes through the x87 stack (`FLD`/`FMUL`/`FSTP`, the x87 NaN
    // choice); the scratch floats `t`, `t^2` and `t^3` are memory operands.
    let t = x87_quiet(t);
    e.set(this, CurveSegment::fParameter, t);
    let t_bits = t.to_bits();
    let square = x87_store32(x87_mul(x87_load32(t_bits), x87_m32(t_bits)));
    let cube = x87_store32(x87_mul(x87_load32(square), x87_m32(t_bits)));
    let minus_two: f64 = e.global(MINUS_TWO);
    let three: f64 = e.global(THREE);
    let one: f64 = e.global(ONE);
    let end_position = x87_add(
        x87_mul(x87_load32(cube), minus_two),
        x87_mul(x87_load32(square), three),
    );
    e.set(
        this,
        CurveSegment::fWeightEndPosition,
        f32::from_bits(x87_store32(end_position)),
    );
    // `FLD` the stored weight back, `FCHS`, `FADD` the double 1.0.
    let stored = x87_load32(e.get(this, CurveSegment::fWeightEndPosition).to_bits());
    e.set(
        this,
        CurveSegment::fWeightStartPosition,
        f32::from_bits(x87_store32(x87_add(-stored, one))),
    );
    let end_tangent = x87_sub(x87_load32(cube), x87_m32(square));
    e.set(
        this,
        CurveSegment::fWeightEndTangent,
        f32::from_bits(x87_store32(end_tangent)),
    );
    let stored = x87_load32(e.get(this, CurveSegment::fWeightEndTangent).to_bits());
    let start_tangent = x87_add(x87_sub(stored, x87_m32(square)), x87_m32(t_bits));
    e.set(
        this,
        CurveSegment::fWeightStartTangent,
        f32::from_bits(x87_store32(start_tangent)),
    );
}

// Translated from 006e7cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the start and end blocks of the curve from `start` and `end`
/// (24 bytes each) and blends them at the current parameter into `out`
/// (three floats); returns `out`.
pub fn fn_006e7cc0(e: &mut Engine, this: Ptr<CurveSegment>, out: Ptr, start: Ptr, end: Ptr) -> Ptr {
    fn_006e7d10(e, Ptr::new(this.addr() + SEGMENT_START), start);
    fn_006e7d10(e, Ptr::new(this.addr() + SEGMENT_END), end);
    fn_006e7d60(e, this, out, start, end);
    out
}

// Translated from 006e7d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a 24-byte position-and-tangent block (two points) from `other`.
pub fn fn_006e7d10(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    copy_words(e, other.addr(), this.addr(), 3);
    copy_words(e, other.addr() + 0x0c, this.addr() + 0x0c, 3);
    this
}

// Translated from 006e7d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Blends two position-and-tangent blocks with the curve's weights into the
/// point `out`: x and y use `fn_006e9000` with both tangents, z with zero
/// tangents. Returns `out`.
pub fn fn_006e7d60(e: &mut Engine, this: Ptr<CurveSegment>, out: Ptr, start: Ptr, end: Ptr) -> Ptr {
    e.with_stack(0x10, |e, frame| {
        e.call(TRIVIAL_CONSTRUCT, &args![frame]);
        let x = fn_006e9000(
            e,
            this,
            start,
            end,
            Ptr::new(start.addr() + 0x0c),
            Ptr::new(end.addr() + 0x0c),
        );
        let y = fn_006e9000(
            e,
            this,
            Ptr::new(start.addr() + 4),
            Ptr::new(end.addr() + 4),
            Ptr::new(start.addr() + 0x10),
            Ptr::new(end.addr() + 0x10),
        );
        // Two zeroed floats stand in for the z tangents.
        let zero_start_tangent = Ptr::new(frame.addr() + 4);
        let zero_end_tangent = Ptr::new(frame.addr() + 8);
        e.mem.set_f32(zero_start_tangent.addr(), 0.0);
        e.mem.set_f32(zero_end_tangent.addr(), 0.0);
        let z = fn_006e9000(
            e,
            this,
            Ptr::new(start.addr() + 8),
            Ptr::new(end.addr() + 8),
            zero_start_tangent,
            zero_end_tangent,
        );
        e.mem.set_f32(out.addr(), x);
        e.mem.set_f32(out.addr() + 4, y);
        e.mem.set_f32(out.addr() + 8, z);
    });
    out
}

// Translated from 006e7e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the position-and-tangent block of a current node into `out`: the
/// node's position (through `005c3420` on the node's `Location`) and its
/// `Tangent`. Returns `out`.
pub fn fn_006e7e10(e: &mut Engine, this: Ptr<PathingNode>, out: Ptr) -> Ptr {
    e.with_stack(0x18 + 0x0c, |e, frame| {
        let block = frame.addr();
        let position = frame.addr() + 0x18;
        fn_006e7bd0(e, Ptr::new(block));
        let point = e
            .call(LOCATION_POINT, &args![this.addr() + 4, position])
            .u32();
        copy_words(e, point, block, 3);
        copy_words(e, this.addr() + 0x2c, block + 0x0c, 3);
        fn_006e7d10(e, out, Ptr::new(block));
    });
    out
}

// Translated from 006e7e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks along the current-node curves from path parameter `position`
/// for `distance` units (forward when `forward`, backward otherwise) and
/// reports where it ends: `out_parameter` (a `float*`, may be null) gets
/// the path parameter reached, `out_point` (three floats, may be null) the
/// point. Returns true when the distance was used up inside the path, false
/// when the walk ran off an end of the path (the end node's parameter and
/// position are reported then).
// Translation incomplete: 006e7e70 - C++ exception unwinding frame (FS:[0]) not translated.
pub fn fn_006e7e70(
    e: &mut Engine,
    this: Ptr<PathingSolution>,
    position: f32,
    distance: f32,
    forward: bool,
    out_parameter: Ptr,
    out_point: Ptr,
) -> bool {
    if current_node_count(e, this) == 0 {
        return false;
    }
    let nodes = this.at(PathingSolution::CurrentPathingNodes);
    e.with_stack(0x44 + 2 * SEGMENT_NODE_SIZE + 0x0c, |e, frame| {
        let segment = Ptr::<CurveSegment>::new(frame.addr());
        let start_block = Ptr::<()>::new(frame.addr() + 0x44);
        let end_block = Ptr::<()>::new(start_block.addr() + SEGMENT_NODE_SIZE);
        let point_buffer = frame.addr() + 0x44 + 2 * SEGMENT_NODE_SIZE;
        if forward {
            let last = current_node_count(e, this).wrapping_sub(1) as i32;
            if position as f64 >= last as f64 {
                if !out_parameter.is_null() {
                    e.mem.set_f32(out_parameter.addr(), last as f32);
                }
                if !out_point.is_null() {
                    let node = e.call(LAST_CURRENT_NODE, &args![this]).u32();
                    let point = node_position(e, node, point_buffer);
                    copy_words(e, point, out_point.addr(), 3);
                }
                return false;
            }
            let mut index = float_to_int(e, position as f64);
            let mut parameter = position;
            let mut remaining = distance;
            fn_006e7b60(e, segment);
            let next = fn_006e8f80(e, nodes, index.wrapping_add(1) as u32);
            let end = fn_006e7e10(e, next, end_block);
            let node = fn_006e8f80(e, nodes, index as u32);
            let start = fn_006e7e10(e, node, start_block);
            fn_006e8450(e, segment, start, end);
            let mut found = fn_006e9830(
                e,
                segment,
                (parameter as f64 - index as f64) as f32,
                remaining,
                forward,
                out_parameter,
                out_point,
                1.0,
            );
            while !found {
                let length =
                    fn_006e9af0(e, segment, (parameter as f64 - index as f64) as f32, 1.0, 2);
                remaining = (remaining as f64 - length as f64) as f32;
                index = index.wrapping_add(1);
                parameter = index as f32;
                if !(index < last.wrapping_sub(1)) {
                    if !out_parameter.is_null() {
                        e.mem.set_f32(out_parameter.addr(), last as f32);
                    }
                    if !out_point.is_null() {
                        let node = e.call(LAST_CURRENT_NODE, &args![this]).u32();
                        let point = node_position(e, node, point_buffer);
                        copy_words(e, point, out_point.addr(), 3);
                    }
                    e.call(SEGMENT_DESTRUCT, &args![segment]);
                    return false;
                }
                let next = fn_006e8f80(e, nodes, index.wrapping_add(1) as u32);
                let end = fn_006e7e10(e, next, end_block);
                let node = fn_006e8f80(e, nodes, index as u32);
                let start = fn_006e7e10(e, node, start_block);
                fn_006e8450(e, segment, start, end);
                found = fn_006e9830(
                    e,
                    segment,
                    0.0,
                    remaining,
                    forward,
                    out_parameter,
                    out_point,
                    1.0,
                );
            }
            if !out_parameter.is_null() {
                let reached = e.mem.f32(out_parameter.addr());
                e.mem
                    .set_f32(out_parameter.addr(), (index as f64 + reached as f64) as f32);
            }
            e.call(SEGMENT_DESTRUCT, &args![segment]);
            return true;
        }

        // Backward.
        if (position as f64) <= 0.0 {
            if !out_parameter.is_null() {
                e.mem.set_f32(out_parameter.addr(), 0.0);
            }
            if !out_point.is_null() {
                let node = fn_006e8480(e, this);
                let point = node_position(e, node.addr(), point_buffer);
                copy_words(e, point, out_point.addr(), 3);
            }
            return false;
        }
        let mut index = float_to_int(e, position as f64);
        if index < 0 {
            index = 0;
        }
        let size = e.call(ARRAY_SIZE, &args![nodes]).u32();
        if index >= size.wrapping_sub(1) as i32 {
            index = index.wrapping_sub(1);
        }
        let mut parameter = position;
        let mut remaining = distance;
        fn_006e7b60(e, segment);
        let next = fn_006e8f80(e, nodes, index.wrapping_add(1) as u32);
        let end = fn_006e7e10(e, next, end_block);
        let node = fn_006e8f80(e, nodes, index as u32);
        let start = fn_006e7e10(e, node, start_block);
        fn_006e8450(e, segment, start, end);
        let mut found = fn_006e9830(
            e,
            segment,
            (parameter as f64 - index as f64) as f32,
            remaining,
            forward,
            out_parameter,
            out_point,
            1.0,
        );
        while !found {
            let length = fn_006e9af0(e, segment, 0.0, (parameter as f64 - index as f64) as f32, 2);
            remaining = (remaining as f64 - length as f64) as f32;
            index = index.wrapping_sub(1);
            parameter = index.wrapping_add(1) as f32;
            if index < 0 {
                if !out_parameter.is_null() {
                    e.mem.set_f32(out_parameter.addr(), 0.0);
                }
                if !out_point.is_null() {
                    let node = fn_006e8480(e, this);
                    let point = node_position(e, node.addr(), point_buffer);
                    copy_words(e, point, out_point.addr(), 3);
                }
                e.call(SEGMENT_DESTRUCT, &args![segment]);
                return false;
            }
            let next = fn_006e8f80(e, nodes, index.wrapping_add(1) as u32);
            let end = fn_006e7e10(e, next, end_block);
            let node = fn_006e8f80(e, nodes, index as u32);
            let start = fn_006e7e10(e, node, start_block);
            fn_006e8450(e, segment, start, end);
            found = fn_006e9830(
                e,
                segment,
                1.0,
                remaining,
                forward,
                out_parameter,
                out_point,
                1.0,
            );
        }
        if !out_parameter.is_null() {
            let reached = e.mem.f32(out_parameter.addr());
            e.mem
                .set_f32(out_parameter.addr(), (index as f64 + reached as f64) as f32);
        }
        e.call(SEGMENT_DESTRUCT, &args![segment]);
        true
    })
}

// Translated from 006e8450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the curve's start and end blocks (24 bytes each).
pub fn fn_006e8450(e: &mut Engine, this: Ptr<CurveSegment>, start: Ptr, end: Ptr) {
    fn_006e7d10(e, Ptr::new(this.addr() + SEGMENT_START), start);
    fn_006e7d10(e, Ptr::new(this.addr() + SEGMENT_END), end);
}

// Translated from 006e8480 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first current node, or null when there are none.
pub fn fn_006e8480(e: &mut Engine, this: Ptr<PathingSolution>) -> Ptr<PathingNode> {
    let nodes = this.at(PathingSolution::CurrentPathingNodes);
    if e.call(ARRAY_IS_EMPTY, &args![nodes]).bool() {
        Ptr::NULL
    } else {
        fn_006e8fc0(e, nodes)
    }
}

// Translated from 006e84b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Length of the current-node path up to path parameter `parameter`: the
/// whole path length (`fn_006e78d0`) when the parameter is at or beyond the
/// last node, otherwise the sum of the lengths of the complete segments
/// before it and of the partial segment, each measured by `fn_006e9af0`
/// with two steps; 0 without nodes.
// Translation incomplete: 006e84b0 - C++ exception unwinding frame (FS:[0]) not translated.
pub fn fn_006e84b0(e: &mut Engine, this: Ptr<PathingSolution>, parameter: f32) -> f32 {
    if current_node_count(e, this) == 0 {
        return 0.0;
    }
    let last = current_node_count(e, this).wrapping_sub(1);
    if parameter as f64 >= last as f64 {
        return fn_006e78d0(e, this);
    }
    let whole = float_to_int(e, parameter as f64);
    let mut length: f32 = 0.0;
    e.with_stack(0x44 + 2 * SEGMENT_NODE_SIZE, |e, frame| {
        let segment = Ptr::<CurveSegment>::new(frame.addr());
        let start_block = Ptr::<()>::new(frame.addr() + 0x44);
        let end_block = Ptr::<()>::new(start_block.addr() + SEGMENT_NODE_SIZE);
        fn_006e7b60(e, segment);
        let mut index = 0i32;
        while index < whole {
            let next = fn_006e7970(e, this, index.wrapping_add(1) as u32);
            let end = fn_006e7e10(e, next, end_block);
            let node = fn_006e7970(e, this, index as u32);
            let start = fn_006e7e10(e, node, start_block);
            fn_006e8450(e, segment, start, end);
            let part = fn_006e9af0(e, segment, 0.0, 1.0, 2);
            length = (part as f64 + length as f64) as f32;
            index += 1;
        }
        let next = fn_006e7970(e, this, whole.wrapping_add(1) as u32);
        let end = fn_006e7e10(e, next, end_block);
        let node = fn_006e7970(e, this, whole as u32);
        let start = fn_006e7e10(e, node, start_block);
        fn_006e8450(e, segment, start, end);
        let fraction = (parameter as f64 - whole as f64) as f32;
        let part = fn_006e9af0(e, segment, 0.0, fraction, 2);
        length = (part as f64 + length as f64) as f32;
        e.call(SEGMENT_DESTRUCT, &args![segment]);
    });
    length
}

// Translated from 006e8680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the segment of the current-node path that contains the fraction
/// `fraction` (clamped to 0..1) of the total length `total`: writes the
/// segment's index to `out_index` and returns that index plus how far into
/// the segment the point lies (0..1). A path shorter than the fraction
/// gives the second-to-last segment and 1.
pub fn fn_006e8680(
    e: &mut Engine,
    this: Ptr<PathingSolution>,
    fraction: f32,
    total: f32,
    out_index: Ptr,
) -> f32 {
    let clamped = e.with_stack(4, |e, slot| {
        e.mem.set_f32(slot.addr(), fraction);
        e.call(CLAMP, &args![slot, 0.0f32, 1.0f32]);
        e.mem.f32(slot.addr())
    });
    let target = (clamped as f64 * total as f64) as f32;
    let mut walked: f32 = 0.0;
    let mut found = false;
    e.mem.set_u32(out_index.addr(), 0);
    let mut inside: f32 = 0.0;
    let mut index = 0u32;
    loop {
        let last = current_node_count(e, this).wrapping_sub(1);
        if index >= last {
            break;
        }
        let length = fn_006e7990(e, this, index as i32);
        if target as f64 <= walked as f64 + length as f64 {
            e.mem.set_u32(out_index.addr(), index);
            inside = if length != 0.0 {
                ((target as f64 - walked as f64) / length as f64) as f32
            } else {
                0.0
            };
            found = true;
            break;
        }
        walked = (walked as f64 + length as f64) as f32;
        index += 1;
    }
    if !found {
        let count = current_node_count(e, this);
        e.mem.set_u32(out_index.addr(), count.wrapping_sub(2));
        inside = 1.0;
    }
    let index = e.mem.u32(out_index.addr());
    (index as f64 + inside as f64) as f32
}

// Translated from 006e8790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes to `out` the point `distance` units further along the current-node
/// path from path parameter `parameter`: the zero point without nodes, the
/// last node's position when the parameter is at or beyond it or the
/// distance runs past the end of the path, otherwise the point found by
/// `fn_006e9830` on the segment where the distance runs out (the segment's
/// end position when that search fails). Returns `out`.
// Translation incomplete: 006e8790 - C++ exception unwinding frame (FS:[0]) not translated.
pub fn fn_006e8790(
    e: &mut Engine,
    this: Ptr<PathingSolution>,
    out: Ptr,
    parameter: f32,
    distance: f32,
) -> Ptr {
    if current_node_count(e, this) == 0 {
        copy_words(e, ZERO_POINT, out.addr(), 3);
        return out;
    }
    let last = current_node_count(e, this).wrapping_sub(1);
    if parameter as f64 >= last as f64 {
        let node = e.call(LAST_CURRENT_NODE, &args![this]).u32();
        node_position(e, node, out.addr());
        return out;
    }
    let mut remaining = distance;
    let mut index = float_to_int(e, parameter as f64);
    let mut fraction = (parameter as f64 - index as f64) as f32;
    e.with_stack(0x44 + 3 * SEGMENT_NODE_SIZE + 0x14, |e, frame| {
        let segment = Ptr::<CurveSegment>::new(frame.addr());
        let end_block = Ptr::<()>::new(frame.addr() + 0x44);
        let start_block = Ptr::<()>::new(end_block.addr() + SEGMENT_NODE_SIZE);
        let scratch_block = Ptr::<()>::new(start_block.addr() + SEGMENT_NODE_SIZE);
        let found_parameter = scratch_block.addr() + SEGMENT_NODE_SIZE;
        let found_point = found_parameter + 4;
        let next = fn_006e7970(e, this, index.wrapping_add(1) as u32);
        fn_006e7e10(e, next, end_block);
        let node = fn_006e7970(e, this, index as u32);
        fn_006e7e10(e, node, start_block);
        fn_006e7b60(e, segment);
        fn_006e8450(e, segment, start_block, end_block);
        let mut segment_length = fn_006e9af0(e, segment, fraction, 1.0, 2);
        loop {
            let limit = current_node_count(e, this).wrapping_sub(2) as i32;
            if index >= limit {
                break;
            }
            if !(remaining as f64 > segment_length as f64) {
                break;
            }
            remaining = (remaining as f64 - segment_length as f64) as f32;
            index = index.wrapping_add(1);
            fn_006e7d10(e, start_block, end_block);
            let next = fn_006e7970(e, this, index.wrapping_add(1) as u32);
            let block = fn_006e7e10(e, next, scratch_block);
            fn_006e7d10(e, end_block, block);
            fn_006e8450(e, segment, start_block, end_block);
            fraction = 0.0;
            segment_length = fn_006e9af0(e, segment, 0.0, 1.0, 2);
        }
        if (remaining as f64) < segment_length as f64 {
            e.call(TRIVIAL_CONSTRUCT, &args![found_point]);
            let tolerance = e.global::<f32>(TEN);
            let ok = fn_006e9830(
                e,
                segment,
                fraction,
                remaining,
                true,
                Ptr::new(found_parameter),
                Ptr::new(found_point),
                tolerance,
            );
            if !ok {
                copy_words(e, end_block.addr(), found_point, 3);
            }
            copy_words(e, found_point, out.addr(), 3);
            e.call(SEGMENT_DESTRUCT, &args![segment]);
        } else {
            let node = e.call(LAST_CURRENT_NODE, &args![this]).u32();
            node_position(e, node, out.addr());
            e.call(SEGMENT_DESTRUCT, &args![segment]);
        }
    });
    out
}

// Translated from 006e8ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the solution into `other`: the two loaded virtual node indices,
/// `bIncompletePath`, and the virtual and current node arrays (not the
/// door ids).
pub fn fn_006e8ab0(e: &mut Engine, this: Ptr<PathingSolution>, other: Ptr<PathingSolution>) {
    let first = e.get(this, PathingSolution::iFirstLoadedVirtualNodeIndex);
    e.set(other, PathingSolution::iFirstLoadedVirtualNodeIndex, first);
    let last = e.get(this, PathingSolution::iLastLoadedVirtualNodeIndex);
    e.set(other, PathingSolution::iLastLoadedVirtualNodeIndex, last);
    let incomplete = e.get(this, PathingSolution::bIncompletePath);
    e.set(other, PathingSolution::bIncompletePath, incomplete);
    fn_006e9410(
        e,
        this.at(PathingSolution::VirtualPathingNodes),
        other.at(PathingSolution::VirtualPathingNodes),
    );
    fn_006e9470(
        e,
        this.at(PathingSolution::CurrentPathingNodes),
        other.at(PathingSolution::CurrentPathingNodes),
    );
}

// Translated from 006e8b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingSolution::SaveGame` (Xbox PDB): writes `bIncompletePath` (1
/// byte), the two loaded virtual node indices (4 bytes each), the virtual
/// nodes (count, then each node), the current nodes (count, then each), a
/// local array of `NavMeshInfo` entries (count, then each; the local array
/// is only constructed here, so its count is whatever the constructor
/// leaves) and the door ids (count, then each as a form id).
// Translation incomplete: 006e8b10 - C++ exception unwinding frame (FS:[0]) not translated.
pub fn pathing_solution_save_game(e: &mut Engine, this: Ptr<PathingSolution>, buffer: Ptr) {
    let incomplete = this.addr() + 0x40;
    e.call(SAVE_BYTES, &args![buffer, incomplete, 1u32, 0u32]);
    let first = this.addr() + 0x18;
    e.call(SAVE_BYTES, &args![buffer, first, 4u32, 0u32]);
    let last = this.addr() + 0x1c;
    e.call(SAVE_BYTES, &args![buffer, last, 4u32, 0u32]);

    let virtual_nodes = this.at(PathingSolution::VirtualPathingNodes);
    // `&this->VirtualPathingNodes` is tested against null before the size
    // is read; it never is.
    let virtual_count = if virtual_nodes.addr() != 0 {
        e.call(ARRAY_SIZE, &args![virtual_nodes]).u32()
    } else {
        0
    };
    e.call(SAVE_SIZED_VALUE, &args![buffer, virtual_count]);
    for index in 0..virtual_count {
        let node = fn_006e8f60(e, virtual_nodes, index);
        e.call(VIRTUAL_NODE_SAVE, &args![node, buffer]);
    }

    let current_nodes = this.at(PathingSolution::CurrentPathingNodes);
    let current_count = if current_nodes.addr() != 0 {
        e.call(ARRAY_SIZE, &args![current_nodes]).u32()
    } else {
        0
    };
    e.call(SAVE_SIZED_VALUE, &args![buffer, current_count]);
    for index in 0..current_count {
        let node = fn_006e8fa0(e, current_nodes, index);
        e.call(CURRENT_NODE_SAVE, &args![node, buffer]);
    }

    // The info array is 0x18 bytes: 006e3850 builds a simple array and a second object at +0x10
    // (two vtables, 006e3960 and 004968b0); EBP-0x24 in the exe.
    e.with_stack(0x18, |e, infos| {
        e.call(INFO_ARRAY_CONSTRUCT, &args![infos]);
        let info_count = if infos.addr() != 0 {
            e.call(ARRAY_SIZE, &args![infos]).u32()
        } else {
            0
        };
        e.call(SAVE_SIZED_VALUE, &args![buffer, info_count]);
        for index in 0..info_count {
            let info = e.call(INFO_ARRAY_AT, &args![infos, index]).u32();
            e.call(INFO_SAVE, &args![info, buffer]);
        }

        let door_ids = this.addr() + DOOR_IDS;
        let door_count = if door_ids != 0 {
            e.call(ARRAY_SIZE, &args![door_ids]).u32()
        } else {
            0
        };
        e.call(SAVE_SIZED_VALUE, &args![buffer, door_count]);
        for index in 0..door_count {
            let slot = e.call(POINTER_ARRAY_AT, &args![door_ids, index]).u32();
            let id = e.mem.u32(slot);
            e.call(SAVE_FORM_ID, &args![buffer, id, 0u32]);
        }
        e.call(INFO_ARRAY_DESTRUCT, &args![infos]);
    });
}

// Translated from 006e8d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathingSolution::LoadGame` (Xbox PDB): reads what
/// [`pathing_solution_save_game`] wrote, sizing the arrays first. The door
/// ids are read only when the buffer's version byte (its first virtual
/// function) is at least 0x12.
// Translation incomplete: 006e8d50 - C++ exception unwinding frame (FS:[0]) not translated.
pub fn pathing_solution_load_game(e: &mut Engine, this: Ptr<PathingSolution>, buffer: Ptr) {
    let incomplete = this.addr() + 0x40;
    e.call(LOAD_BYTES, &args![buffer, incomplete, 1u32]);
    let first = this.addr() + 0x18;
    e.call(LOAD_BYTES, &args![buffer, first, 4u32]);
    let last = this.addr() + 0x1c;
    e.call(LOAD_BYTES, &args![buffer, last, 4u32]);

    let virtual_nodes = this.at(PathingSolution::VirtualPathingNodes);
    let virtual_count = e.call(LOAD_SIZED_VALUE, &args![buffer]).u32();
    fn_006e90a0(e, virtual_nodes, virtual_count, true);
    for index in 0..virtual_count {
        let node = fn_006e8f60(e, virtual_nodes, index);
        e.call(VIRTUAL_NODE_LOAD, &args![node, buffer]);
    }

    let current_nodes = this.at(PathingSolution::CurrentPathingNodes);
    let current_count = e.call(LOAD_SIZED_VALUE, &args![buffer]).u32();
    fn_006e9280(e, current_nodes, current_count, true);
    for index in 0..current_count {
        let node = fn_006e8fa0(e, current_nodes, index);
        e.call(CURRENT_NODE_LOAD, &args![node, buffer]);
    }

    // The info array is 0x18 bytes: 006e3850 builds a simple array and a second object at +0x10
    // (two vtables, 006e3960 and 004968b0); EBP-0x24 in the exe.
    e.with_stack(0x18, |e, infos| {
        e.call(INFO_ARRAY_CONSTRUCT, &args![infos]);
        let info_count = e.call(LOAD_SIZED_VALUE, &args![buffer]).u32();
        e.call(INFO_ARRAY_SET_SIZE, &args![infos, info_count, 1u32]);
        for index in 0..info_count {
            let info = e.call(INFO_ARRAY_AT, &args![infos, index]).u32();
            e.call(INFO_LOAD, &args![info, buffer]);
        }

        let version = e.vcall(buffer.addr(), LOAD_BUFFER_VERSION_SLOT, &[]).u8();
        if version as u32 >= DOOR_IDS_VERSION {
            let door_ids = this.addr() + DOOR_IDS;
            let door_count = e.call(LOAD_SIZED_VALUE, &args![buffer]).u32();
            e.call(POINTER_ARRAY_SET_SIZE, &args![door_ids, door_count, 1u32]);
            for index in 0..door_count {
                let slot = e.call(POINTER_ARRAY_AT, &args![door_ids, index]).u32();
                e.call(LOAD_FORM_ID, &args![buffer, slot]);
            }
        }
        e.call(INFO_ARRAY_DESTRUCT, &args![infos]);
    });
}

// ---------------------------------------------------------------------------
// BSSimpleArray<VirtualPathingNode,1024> and BSSimpleArray<PathingNode,1024>
// ---------------------------------------------------------------------------

// Translated from 006e8f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Address of element `index` of a `BSSimpleArray<VirtualPathingNode,1024>`
/// (0x30 bytes each).
pub fn fn_006e8f40(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    index: u32,
) -> Ptr<VirtualPathingNode> {
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    Ptr::new(index.wrapping_mul(VIRTUAL_NODE_SIZE).wrapping_add(buffer))
}

// Translated from 006e8f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same element accessor as [`fn_006e8f40`].
pub fn fn_006e8f60(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    index: u32,
) -> Ptr<VirtualPathingNode> {
    fn_006e8f40(e, this, index)
}

// Translated from 006e8f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Address of element `index` of a `BSSimpleArray<PathingNode,1024>`
/// (0x3C bytes each).
pub fn fn_006e8f80(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32) -> Ptr<PathingNode> {
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    Ptr::new(index.wrapping_mul(CURRENT_NODE_SIZE).wrapping_add(buffer))
}

// Translated from 006e8fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same element accessor as [`fn_006e8f80`].
pub fn fn_006e8fa0(e: &mut Engine, this: Ptr<BSSimpleArray>, index: u32) -> Ptr<PathingNode> {
    fn_006e8f80(e, this, index)
}

// Translated from 006e8fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first element of a `BSSimpleArray<PathingNode,1024>`.
pub fn fn_006e8fc0(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<PathingNode> {
    fn_006e8f80(e, this, 0)
}

// Translated from 006e8fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The last element of a `BSSimpleArray<PathingNode,1024>` (`iSize - 1`).
pub fn fn_006e8fe0(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<PathingNode> {
    let size = e.get(this, BSSimpleArray::iSize);
    fn_006e8f80(e, this, size.wrapping_sub(1))
}

// Translated from 006e9000 (decompiled, FalloutNV.exe 1.4.0.525)
/// The curve's weighted sum of four floats read through pointers: start
/// position (`start`) times the start-position weight, `start_tangent`
/// times the start-tangent weight, `end` times the end-position weight and
/// `end_tangent` times the end-tangent weight, rounded to `float`.
pub fn fn_006e9000(
    e: &mut Engine,
    this: Ptr<CurveSegment>,
    start: Ptr,
    end: Ptr,
    start_tangent: Ptr,
    end_tangent: Ptr,
) -> f32 {
    // Each weight is loaded (`FLD`, quiet) and multiplied by the value in
    // memory (`FMUL dword ptr`); the products are summed with `FADDP`
    // (x87 NaN choice, #199).
    let start_weight = f64::from(x87_quiet(e.get(this, CurveSegment::fWeightStartPosition)));
    let start_tangent_weight = f64::from(x87_quiet(e.get(this, CurveSegment::fWeightStartTangent)));
    let end_weight = f64::from(x87_quiet(e.get(this, CurveSegment::fWeightEndPosition)));
    let end_tangent_weight = f64::from(x87_quiet(e.get(this, CurveSegment::fWeightEndTangent)));
    let start_value = x87_m32(e.mem.u32(start.addr()));
    let start_tangent_value = x87_m32(e.mem.u32(start_tangent.addr()));
    let end_value = x87_m32(e.mem.u32(end.addr()));
    let end_tangent_value = x87_m32(e.mem.u32(end_tangent.addr()));
    let sum = x87_add(
        x87_mul(start_weight, start_value),
        x87_mul(start_tangent_weight, start_tangent_value),
    );
    let sum = x87_add(sum, x87_mul(end_weight, end_value));
    x87_add(sum, x87_mul(end_tangent_weight, end_tangent_value)) as f32
}

// Translated from 006e9050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `BSSimpleArray<VirtualPathingNode,1024>`: sets the vtable
/// and builds an empty array (`fn_006e9670` with no capacity and no size).
pub fn fn_006e9050(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), VIRTUAL_ARRAY_VTABLE);
    fn_006e9670(e, this, 0, 0);
    this
}

// Translated from 006e9080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `BSSimpleArray<VirtualPathingNode,1024>`: sets the vtable
/// and empties the array, releasing the buffer.
pub fn fn_006e9080(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), VIRTUAL_ARRAY_VTABLE);
    e.call(VIRTUAL_ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 006e90a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SetSize(newSize, shrink)` of `BSSimpleArray<VirtualPathingNode,1024>`:
/// 0 empties it; growing allocates (virtual slot +4) or reallocates when
/// the reserve is too small, then constructs the new elements; shrinking
/// destroys the tail and, when `shrink` and the new size is at most a
/// quarter of the reserve, reallocates down to the new size.
pub fn fn_006e90a0(e: &mut Engine, this: Ptr<BSSimpleArray>, new_size: u32, shrink: bool) {
    if new_size == 0 {
        e.call(VIRTUAL_ARRAY_CLEAR, &args![this, shrink]);
        return;
    }
    let reserved = e.get(this, BSSimpleArray::iReservedSize);
    if new_size > reserved {
        if reserved == 0 {
            let buffer = e.vcall(this.addr(), 4, &args![new_size]).u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
            e.set(this, BSSimpleArray::iReservedSize, new_size);
        } else {
            let size = e.get(this, BSSimpleArray::iSize);
            e.call(VIRTUAL_ARRAY_REALLOCATE, &args![this, new_size, size]);
            e.set(this, BSSimpleArray::iReservedSize, new_size);
        }
        let size = e.get(this, BSSimpleArray::iSize);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let tail = size.wrapping_mul(VIRTUAL_NODE_SIZE).wrapping_add(buffer);
        e.call(
            VIRTUAL_ARRAY_CONSTRUCT,
            &args![this, tail, new_size.wrapping_sub(size)],
        );
        e.set(this, BSSimpleArray::iSize, new_size);
        return;
    }
    let size = e.get(this, BSSimpleArray::iSize);
    if new_size < size {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let tail = new_size
            .wrapping_mul(VIRTUAL_NODE_SIZE)
            .wrapping_add(buffer);
        e.call(VIRTUAL_ARRAY_DESTROY, &args![this, tail, size - new_size]);
        e.set(this, BSSimpleArray::iSize, new_size);
        if shrink && new_size <= reserved >> 2 {
            e.call(VIRTUAL_ARRAY_REALLOCATE, &args![this, new_size, new_size]);
            e.set(this, BSSimpleArray::iReservedSize, new_size);
        }
    } else {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let tail = size.wrapping_mul(VIRTUAL_NODE_SIZE).wrapping_add(buffer);
        e.call(VIRTUAL_ARRAY_CONSTRUCT, &args![this, tail, new_size - size]);
        e.set(this, BSSimpleArray::iSize, new_size);
    }
}

// Translated from 006e91e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<VirtualPathingNode,1024>::_Allocate` (Xbox PDB):
/// `operator new` of `count * 0x30` bytes.
pub fn bs_simple_array_virtual_pathing_node_1024_allocate(
    e: &mut Engine,
    _this: Ptr<BSSimpleArray>,
    count: u32,
) -> u32 {
    e.call(OPERATOR_NEW, &args![count.wrapping_mul(VIRTUAL_NODE_SIZE)])
        .u32()
}

// Translated from 006e9210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<VirtualPathingNode,1024>::_Reallocate` (Xbox PDB): the
/// reallocation primitive `(from, count * 0x30)` (`0042f5d0`); returns what
/// it returns.
pub fn bs_simple_array_virtual_pathing_node_1024_reallocate(
    e: &mut Engine,
    _this: Ptr<BSSimpleArray>,
    from: u32,
    count: u32,
) -> u32 {
    e.call(
        REALLOCATE_BYTES,
        &args![from, count.wrapping_mul(VIRTUAL_NODE_SIZE)],
    )
    .u32()
}

// Translated from 006e9230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `BSSimpleArray<PathingNode,1024>`: sets the vtable and
/// builds an empty array (`fn_006e97a0` with no capacity and no size).
pub fn fn_006e9230(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), CURRENT_ARRAY_VTABLE);
    fn_006e97a0(e, this, 0, 0);
    this
}

// Translated from 006e9260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `BSSimpleArray<PathingNode,1024>`: sets the vtable and
/// empties the array, releasing the buffer.
pub fn fn_006e9260(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), CURRENT_ARRAY_VTABLE);
    e.call(CURRENT_ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 006e9280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SetSize(newSize, shrink)` of `BSSimpleArray<PathingNode,1024>`: the same
/// logic as [`fn_006e90a0`] with 0x3C-byte elements (reallocation
/// `006a21c0`, destruction `006cff20`, construction `006e9700`).
pub fn fn_006e9280(e: &mut Engine, this: Ptr<BSSimpleArray>, new_size: u32, shrink: bool) {
    if new_size == 0 {
        e.call(CURRENT_ARRAY_CLEAR, &args![this, shrink]);
        return;
    }
    let reserved = e.get(this, BSSimpleArray::iReservedSize);
    if new_size > reserved {
        if reserved == 0 {
            let buffer = e.vcall(this.addr(), 4, &args![new_size]).u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
            e.set(this, BSSimpleArray::iReservedSize, new_size);
        } else {
            let size = e.get(this, BSSimpleArray::iSize);
            e.call(CURRENT_ARRAY_REALLOCATE, &args![this, new_size, size]);
            e.set(this, BSSimpleArray::iReservedSize, new_size);
        }
        let size = e.get(this, BSSimpleArray::iSize);
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let tail = size.wrapping_mul(CURRENT_NODE_SIZE).wrapping_add(buffer);
        e.call(
            CURRENT_ARRAY_CONSTRUCT,
            &args![this, tail, new_size.wrapping_sub(size)],
        );
        e.set(this, BSSimpleArray::iSize, new_size);
        return;
    }
    let size = e.get(this, BSSimpleArray::iSize);
    if new_size < size {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let tail = new_size
            .wrapping_mul(CURRENT_NODE_SIZE)
            .wrapping_add(buffer);
        e.call(CURRENT_ARRAY_DESTROY, &args![this, tail, size - new_size]);
        e.set(this, BSSimpleArray::iSize, new_size);
        if shrink && new_size <= reserved >> 2 {
            e.call(CURRENT_ARRAY_REALLOCATE, &args![this, new_size, new_size]);
            e.set(this, BSSimpleArray::iReservedSize, new_size);
        }
    } else {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let tail = size.wrapping_mul(CURRENT_NODE_SIZE).wrapping_add(buffer);
        e.call(CURRENT_ARRAY_CONSTRUCT, &args![this, tail, new_size - size]);
        e.set(this, BSSimpleArray::iSize, new_size);
    }
}

// Translated from 006e93c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathingNode,1024>::_Allocate` (Xbox PDB): `operator new`
/// of `count * 0x3C` bytes.
pub fn bs_simple_array_pathing_node_1024_allocate(
    e: &mut Engine,
    _this: Ptr<BSSimpleArray>,
    count: u32,
) -> u32 {
    e.call(OPERATOR_NEW, &args![count.wrapping_mul(CURRENT_NODE_SIZE)])
        .u32()
}

// Translated from 006e93f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathingNode,1024>::_Reallocate` (Xbox PDB): the
/// reallocation primitive `(from, count * 0x3C)` (`0042f5d0`); returns what
/// it returns.
pub fn bs_simple_array_pathing_node_1024_reallocate(
    e: &mut Engine,
    _this: Ptr<BSSimpleArray>,
    from: u32,
    count: u32,
) -> u32 {
    e.call(
        REALLOCATE_BYTES,
        &args![from, count.wrapping_mul(CURRENT_NODE_SIZE)],
    )
    .u32()
}

// Translated from 006e9410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a `BSSimpleArray<VirtualPathingNode,1024>` into `other`: sizes
/// `other` to this array's size (shrinking allowed) and copy-constructs
/// each element ([`fn_006e9530`]).
pub fn fn_006e9410(e: &mut Engine, this: Ptr<BSSimpleArray>, other: Ptr<BSSimpleArray>) {
    let size = e.get(this, BSSimpleArray::iSize);
    fn_006e90a0(e, other, size, true);
    let mut index = 0u32;
    while index < e.get(this, BSSimpleArray::iSize) {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let from = index.wrapping_mul(VIRTUAL_NODE_SIZE).wrapping_add(buffer);
        let to = fn_006e8f40(e, other, index);
        fn_006e9530(e, to, Ptr::new(from));
        index += 1;
    }
}

// Translated from 006e9470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a `BSSimpleArray<PathingNode,1024>` into `other`: sizes `other`
/// to this array's size (shrinking allowed) and copy-constructs each
/// element ([`fn_006e9570`]).
pub fn fn_006e9470(e: &mut Engine, this: Ptr<BSSimpleArray>, other: Ptr<BSSimpleArray>) {
    let size = e.get(this, BSSimpleArray::iSize);
    fn_006e9280(e, other, size, true);
    let mut index = 0u32;
    while index < e.get(this, BSSimpleArray::iSize) {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        let from = index.wrapping_mul(CURRENT_NODE_SIZE).wrapping_add(buffer);
        let to = fn_006e8f80(e, other, index);
        fn_006e9570(e, to, Ptr::new(from));
        index += 1;
    }
}

// Translated from 006e94d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<VirtualPathingNode,1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): destroys the array and, when bit 0 of `flags` is set, frees
/// the object (`operator delete`).
pub fn bs_simple_array_virtual_pathing_node_1024_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    fn_006e9080(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006e9500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<PathingNode,1024>::_scalar_deleting_destructor_` (Xbox
/// PDB): destroys the array and, when bit 0 of `flags` is set, frees the
/// object (`operator delete`).
pub fn bs_simple_array_pathing_node_1024_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    fn_006e9260(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006e9530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy-assigns a `VirtualPathingNode` from `other`: flags, location
/// (`PathingLocation::operator=`) and door reference. Returns `this`.
pub fn fn_006e9530(
    e: &mut Engine,
    this: Ptr<VirtualPathingNode>,
    other: Ptr<VirtualPathingNode>,
) -> Ptr<VirtualPathingNode> {
    let flags = e.get(other, VirtualPathingNode::iFlags);
    e.set(this, VirtualPathingNode::iFlags, flags);
    e.call(LOCATION_ASSIGN, &args![this.addr() + 4, other.addr() + 4]);
    let door = e.get(other, VirtualPathingNode::pDoorRef);
    e.set(this, VirtualPathingNode::pDoorRef, door);
    this
}

// Translated from 006e9570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy-assigns a `PathingNode` from `other`: flags, location
/// (`PathingLocation::operator=`), the 12-byte tangent and the door
/// reference. Returns `this`.
pub fn fn_006e9570(
    e: &mut Engine,
    this: Ptr<PathingNode>,
    other: Ptr<PathingNode>,
) -> Ptr<PathingNode> {
    let flags = e.get(other, PathingNode::iFlags);
    e.set(this, PathingNode::iFlags, flags);
    e.call(LOCATION_ASSIGN, &args![this.addr() + 4, other.addr() + 4]);
    copy_words(e, other.addr() + 0x2c, this.addr() + 0x2c, 3);
    let door = e.get(other, PathingNode::pDoorRef);
    e.set(this, PathingNode::pDoorRef, door);
    this
}

// Translated from 006e9670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor with sizes for `BSSimpleArray<VirtualPathingNode,1024>`:
/// an empty array, then `capacity` (raised to `size` when smaller)
/// elements allocated through the virtual `_Allocate` (slot +4) and `size`
/// elements constructed.
pub fn fn_006e9670(e: &mut Engine, this: Ptr<BSSimpleArray>, capacity: u32, size: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let capacity = capacity.max(size);
    if capacity > 0 {
        let buffer = e.vcall(this.addr(), 4, &args![capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    }
    if size > 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(VIRTUAL_ARRAY_CONSTRUCT, &args![this, buffer, size]);
        e.set(this, BSSimpleArray::iSize, size);
    }
}

// Translated from 006e97a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor with sizes for `BSSimpleArray<PathingNode,1024>`: the same
/// as [`fn_006e9670`] with the `PathingNode` construction loop.
pub fn fn_006e97a0(e: &mut Engine, this: Ptr<BSSimpleArray>, capacity: u32, size: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let capacity = capacity.max(size);
    if capacity > 0 {
        let buffer = e.vcall(this.addr(), 4, &args![capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    }
    if size > 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(CURRENT_ARRAY_CONSTRUCT, &args![this, buffer, size]);
        e.set(this, BSSimpleArray::iSize, size);
    }
}

// Translated from 006e9830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks on the curve for the point whose distance from the point at
/// parameter `start` is about `distance` (within `tolerance`), going
/// forward from `start` when `forward` and backward otherwise.
///
/// A `distance` within `tolerance` returns `start` itself. Otherwise the
/// search is a bisection of the parameter range (`start`..1 forward,
/// 0..`start` backward) that keeps comparing the squared distance with
/// `(distance - tolerance)^2` and `(distance + tolerance)^2`; it fails
/// (returns false) when the interval collapses against 0.99 (forward) or
/// 0.01 (backward). On success `out_parameter` (a `float*`) gets the
/// parameter and `out_point` (three floats) the point; on failure
/// `out_parameter` gets 1 and the end node's position (forward) or 0 and
/// the start node's position (backward) and false is returned.
#[allow(clippy::too_many_arguments)]
pub fn fn_006e9830(
    e: &mut Engine,
    this: Ptr<CurveSegment>,
    start: f32,
    distance: f32,
    forward: bool,
    out_parameter: Ptr,
    out_point: Ptr,
    tolerance: f32,
) -> bool {
    e.with_stack(0x60, |e, frame| {
        // Locals of the game's frame (offsets from its `EBP`).
        let scratch = frame.addr(); // ebp-0x40 .. ebp-0x34: the point at `start` (short path)
        let base = frame.addr() + 0x10; // ebp-0x10: the point at `start`
        let current = frame.addr() + 0x20; // ebp-0x20: the point being tested
        let difference = frame.addr() + 0x30; // ebp-0x5c: base - current
        let mut start = start;

        if tolerance as f64 >= distance as f64 {
            let point = segment_point(e, this, scratch, start);
            copy_words(e, point, out_point.addr(), 3);
            e.mem.set_f32(out_parameter.addr(), start);
            return true;
        }

        let outer_squared =
            ((distance as f64 + tolerance as f64) * (distance as f64 + tolerance as f64)) as f32;
        let inner_squared =
            ((distance as f64 - tolerance as f64) * (distance as f64 - tolerance as f64)) as f32;
        // Clamp the `start` argument slot to 0..1.
        start = e.with_stack(4, |e, slot| {
            e.mem.set_f32(slot.addr(), start);
            e.call(CLAMP, &args![slot, 0.0f32, 1.0f32]);
            e.mem.f32(slot.addr())
        });
        let (mut low, mut high, mut middle): (f32, f32, f32) = if forward {
            (start, 1.0, 1.0)
        } else {
            (0.0, start, 0.0)
        };
        segment_point(e, this, base, start);
        let bounds = if forward {
            this.addr() + SEGMENT_END
        } else {
            this.addr() + SEGMENT_START
        };
        copy_words(e, bounds, current, 3);
        let mut found = false;
        let upper_limit: f64 = e.global(UPPER_LIMIT);
        let lower_limit: f64 = e.global(LOWER_LIMIT);
        let half: f64 = e.global(HALF);
        while !found {
            let scratch_point = frame.addr() + 0x40;
            let point = segment_point(e, this, scratch_point, middle);
            copy_words(e, point, current, 3);
            let delta = e
                .call(POINT_SUBTRACT, &args![base, difference, current])
                .u32();
            let squared = e.call(POINT_LENGTH_SQUARED, &args![delta]).f64() as f32;
            let near = (squared as f64) < inner_squared as f64;
            let far = (outer_squared as f64) < squared as f64;
            // Which way the interval moves: 1 raises `low`, 2 lowers `high`,
            // 0 stops (the point is acceptable).
            let action = if forward {
                if near {
                    1
                } else if far {
                    2
                } else {
                    0
                }
            } else if far {
                1
            } else if near {
                2
            } else {
                0
            };
            match action {
                1 => {
                    if !(middle as f64 >= upper_limit) {
                        low = middle;
                    } else {
                        break;
                    }
                }
                2 => {
                    if !(middle as f64 <= lower_limit) {
                        high = middle;
                    } else {
                        break;
                    }
                }
                _ => {
                    found = true;
                    continue;
                }
            }
            middle = ((low as f64 + high as f64) * half) as f32;
        }
        if !found {
            if forward {
                e.mem.set_f32(out_parameter.addr(), 1.0);
                copy_words(e, this.addr() + SEGMENT_END, out_point.addr(), 3);
            } else {
                e.mem.set_f32(out_parameter.addr(), 0.0);
                copy_words(e, this.addr() + SEGMENT_START, out_point.addr(), 3);
            }
            return false;
        }
        e.mem.set_f32(out_parameter.addr(), middle);
        copy_words(e, current, out_point.addr(), 3);
        true
    })
}

/// The point on the curve at parameter `t` (`006e9ab0`) into `out`;
/// returns `out`.
fn segment_point(e: &mut Engine, segment: Ptr<CurveSegment>, out: u32, t: f32) -> u32 {
    e.call(SEGMENT_POINT, &args![segment, out, t]).u32()
}

// Translated from 006e9af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Length of the curve between parameters `from` and `to` measured as the
/// sum of `steps` straight pieces between the points at equally spaced
/// parameters (clamped to 0..1).
pub fn fn_006e9af0(e: &mut Engine, this: Ptr<CurveSegment>, from: f32, to: f32, steps: u32) -> f32 {
    e.with_stack(0x30, |e, frame| {
        let previous = frame.addr();
        let current = frame.addr() + 0x0c;
        let difference = frame.addr() + 0x18;
        let mut length: f32 = 0.0;
        segment_point(e, this, previous, from);
        let mut index = 0u32;
        while index < steps {
            let parameter = ((to as f64 - from as f64) * (index as u64 + 1) as f64 / steps as f64
                + from as f64) as f32;
            let parameter = e.with_stack(4, |e, slot| {
                e.mem.set_f32(slot.addr(), parameter);
                e.call(CLAMP, &args![slot, 0.0f32, 1.0f32]);
                e.mem.f32(slot.addr())
            });
            segment_point(e, this, current, parameter);
            let delta = e
                .call(POINT_SUBTRACT, &args![current, difference, previous])
                .u32();
            let piece = e.call(POINT_LENGTH, &args![delta]).f64();
            length = (piece + length as f64) as f32;
            copy_words(e, current, previous, 3);
            index += 1;
        }
        length
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006e76f0,
            pathing_solution_scalar_deleting_destructor(
                Ptr<PathingSolution>,
                u32,
            ) -> Ptr<PathingSolution>
        ),
        entry!(0x006e7720, fn_006e7720(Ptr<PathingSolution>)),
        entry!(0x006e77b0, fn_006e77b0(Ptr<PathingSolution>)),
        entry!(0x006e7800, fn_006e7800(Ptr<PathingSolution>) -> f32),
        entry!(
            0x006e78b0,
            fn_006e78b0(Ptr<PathingSolution>, u32) -> Ptr<VirtualPathingNode>
        ),
        entry!(0x006e78d0, fn_006e78d0(Ptr<PathingSolution>) -> f32),
        entry!(
            0x006e7970,
            fn_006e7970(Ptr<PathingSolution>, u32) -> Ptr<PathingNode>
        ),
        entry!(0x006e7990, fn_006e7990(Ptr<PathingSolution>, i32) -> f32),
        entry!(0x006e7a10, fn_006e7a10(Ptr<PathingSolution>, f32, Ptr)),
        entry!(
            0x006e7b60,
            fn_006e7b60(Ptr<CurveSegment>) -> Ptr<CurveSegment>
        ),
        entry!(0x006e7bd0, fn_006e7bd0(Ptr) -> Ptr),
        entry!(0x006e7c20, fn_006e7c20(Ptr<CurveSegment>, f32)),
        entry!(0x006e7c40, fn_006e7c40(Ptr<CurveSegment>, f32)),
        entry!(
            0x006e7cc0,
            fn_006e7cc0(Ptr<CurveSegment>, Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(0x006e7d10, fn_006e7d10(Ptr, Ptr) -> Ptr),
        entry!(
            0x006e7d60,
            fn_006e7d60(Ptr<CurveSegment>, Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(0x006e7e10, fn_006e7e10(Ptr<PathingNode>, Ptr) -> Ptr),
        entry!(
            0x006e7e70,
            fn_006e7e70(Ptr<PathingSolution>, f32, f32, bool, Ptr, Ptr) -> bool
        ),
        entry!(0x006e8450, fn_006e8450(Ptr<CurveSegment>, Ptr, Ptr)),
        entry!(
            0x006e8480,
            fn_006e8480(Ptr<PathingSolution>) -> Ptr<PathingNode>
        ),
        entry!(0x006e84b0, fn_006e84b0(Ptr<PathingSolution>, f32) -> f32),
        entry!(
            0x006e8680,
            fn_006e8680(Ptr<PathingSolution>, f32, f32, Ptr) -> f32
        ),
        entry!(
            0x006e8790,
            fn_006e8790(Ptr<PathingSolution>, Ptr, f32, f32) -> Ptr
        ),
        entry!(
            0x006e8ab0,
            fn_006e8ab0(Ptr<PathingSolution>, Ptr<PathingSolution>)
        ),
        entry!(
            0x006e8b10,
            pathing_solution_save_game(Ptr<PathingSolution>, Ptr)
        ),
        entry!(
            0x006e8d50,
            pathing_solution_load_game(Ptr<PathingSolution>, Ptr)
        ),
        entry!(
            0x006e8f40,
            fn_006e8f40(Ptr<BSSimpleArray>, u32) -> Ptr<VirtualPathingNode>
        ),
        entry!(
            0x006e8f60,
            fn_006e8f60(Ptr<BSSimpleArray>, u32) -> Ptr<VirtualPathingNode>
        ),
        entry!(
            0x006e8f80,
            fn_006e8f80(Ptr<BSSimpleArray>, u32) -> Ptr<PathingNode>
        ),
        entry!(
            0x006e8fa0,
            fn_006e8fa0(Ptr<BSSimpleArray>, u32) -> Ptr<PathingNode>
        ),
        entry!(
            0x006e8fc0,
            fn_006e8fc0(Ptr<BSSimpleArray>) -> Ptr<PathingNode>
        ),
        entry!(
            0x006e8fe0,
            fn_006e8fe0(Ptr<BSSimpleArray>) -> Ptr<PathingNode>
        ),
        entry!(
            0x006e9000,
            fn_006e9000(Ptr<CurveSegment>, Ptr, Ptr, Ptr, Ptr) -> f32
        ),
        entry!(
            0x006e9050,
            fn_006e9050(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x006e9080, fn_006e9080(Ptr<BSSimpleArray>)),
        entry!(0x006e90a0, fn_006e90a0(Ptr<BSSimpleArray>, u32, bool)),
        entry!(
            0x006e91e0,
            bs_simple_array_virtual_pathing_node_1024_allocate(Ptr<BSSimpleArray>, u32) -> u32
        ),
        entry!(
            0x006e9210,
            bs_simple_array_virtual_pathing_node_1024_reallocate(
                Ptr<BSSimpleArray>,
                u32,
                u32,
            ) -> u32
        ),
        entry!(
            0x006e9230,
            fn_006e9230(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x006e9260, fn_006e9260(Ptr<BSSimpleArray>)),
        entry!(0x006e9280, fn_006e9280(Ptr<BSSimpleArray>, u32, bool)),
        entry!(
            0x006e93c0,
            bs_simple_array_pathing_node_1024_allocate(Ptr<BSSimpleArray>, u32) -> u32
        ),
        entry!(
            0x006e93f0,
            bs_simple_array_pathing_node_1024_reallocate(Ptr<BSSimpleArray>, u32, u32) -> u32
        ),
        entry!(
            0x006e9410,
            fn_006e9410(Ptr<BSSimpleArray>, Ptr<BSSimpleArray>)
        ),
        entry!(
            0x006e9470,
            fn_006e9470(Ptr<BSSimpleArray>, Ptr<BSSimpleArray>)
        ),
        entry!(
            0x006e94d0,
            bs_simple_array_virtual_pathing_node_1024_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<
                BSSimpleArray,
            >
        ),
        entry!(
            0x006e9500,
            bs_simple_array_pathing_node_1024_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x006e9530,
            fn_006e9530(
                Ptr<VirtualPathingNode>,
                Ptr<VirtualPathingNode>,
            ) -> Ptr<VirtualPathingNode>
        ),
        entry!(
            0x006e9570,
            fn_006e9570(Ptr<PathingNode>, Ptr<PathingNode>) -> Ptr<PathingNode>
        ),
        entry!(0x006e9670, fn_006e9670(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x006e97a0, fn_006e97a0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(
            0x006e9830,
            fn_006e9830(Ptr<CurveSegment>, f32, f32, bool, Ptr, Ptr, f32) -> bool
        ),
        entry!(
            0x006e9af0,
            fn_006e9af0(Ptr<CurveSegment>, f32, f32, u32) -> f32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Calls a recording double saw, as (address, argument words).
    type CallRecord = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    const BUFFER: u32 = 0x0090_0000;
    /// The `_Allocate` virtual functions the array vtables point at.
    const VIRTUAL_ALLOCATE: u32 = 0x006e_91e0;
    const CURRENT_ALLOCATE: u32 = 0x006e_93c0;

    /// An engine with the constants the code reads and a stand-in for every
    /// leaf callee outside this file, each with the body read from the exe.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for (address, value) in [
            (MINUS_TWO, -2.0f64),
            (THREE, 3.0),
            (ONE, 1.0),
            (HALF, 0.5),
            (UPPER_LIMIT, 0.99f32 as f64),
            (LOWER_LIMIT, 0.01f32 as f64),
        ] {
            e.map(address, 8);
            e.set_global(address, value);
        }
        e.map(TEN, 4);
        e.set_global(TEN, 10.0f32);
        e.map(ZERO_POINT, 12);
        // `iSize` getter and the solution's size getters.
        e.register(ARRAY_SIZE, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(CURRENT_NODE_COUNT, |e, a| e.mem.u32(a[0] + 0x28).into_ret());
        e.register(VIRTUAL_NODE_COUNT, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(ARRAY_IS_EMPTY, |e, a| (e.mem.u32(a[0] + 8) == 0).into_ret());
        // `this + 4` and the location's point copy.
        e.register(NODE_LOCATION, |_, a| (a[0] + 4).into_ret());
        e.register(LOCATION_POINT, |e, a| {
            copy_words(e, a[0] + 4, a[1], 3);
            a[1].into_ret()
        });
        // Point subtraction and the lengths.
        e.register(POINT_SUBTRACT, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            a[1].into_ret()
        });
        e.register(POINT_LENGTH, |e, a| {
            let sum = squares(e, a[0], 3);
            (sum as f64).sqrt().into_ret()
        });
        e.register(POINT_LENGTH_2D, |e, a| {
            let sum = squares(e, a[0], 2);
            (sum as f64).sqrt().into_ret()
        });
        e.register(POINT_LENGTH_SQUARED, |e, a| squares(e, a[0], 3).into_ret());
        e.register(NODE_FLAG_4, |e, a| (e.mem.u32(a[0]) & 4 == 4).into_ret());
        e.register(TRIVIAL_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(SEGMENT_DESTRUCT, |_, _| Ret::default());
        e.register(CLAMP, |e, a| {
            let value = e.mem.f32(a[0]);
            let (min, max) = (f32::from_bits(a[1]), f32::from_bits(a[2]));
            if value > max {
                e.mem.set_f32(a[0], max);
            } else if value < min {
                e.mem.set_f32(a[0], min);
            }
            Ret::default()
        });
        e.register(LAST_CURRENT_NODE, |e, a| {
            let size = e.mem.u32(a[0] + 0x28);
            if size == 0 {
                0u32.into_ret()
            } else {
                (e.mem.u32(a[0] + 0x24) + (size - 1) * CURRENT_NODE_SIZE).into_ret()
            }
        });
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            (value.trunc() as i32).into_ret()
        });
        // `006e9ab0`: sets the parameter, then blends start and end.
        e.register(SEGMENT_POINT, |e, a| {
            let segment = Ptr::<CurveSegment>::new(a[0]);
            fn_006e7c20(e, segment, f32::from_bits(a[2]));
            fn_006e7d60(
                e,
                segment,
                Ptr::new(a[1]),
                Ptr::new(a[0] + SEGMENT_START),
                Ptr::new(a[0] + SEGMENT_END),
            );
            a[1].into_ret()
        });
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e
    }

    /// Sum of the squares of `count` floats, rounded to `float` as the game
    /// stores it.
    fn squares(e: &Engine, address: u32, count: u32) -> f32 {
        let mut sum = 0.0f64;
        for i in 0..count {
            let v = e.mem.f32(address + 4 * i) as f64;
            sum += v * v;
        }
        sum as f32
    }

    /// Registers do-nothing stand-ins for the given addresses.
    fn stub(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn solution(e: &mut Engine) -> Ptr<PathingSolution> {
        e.new_object::<PathingSolution>()
    }

    fn set_array(e: &mut Engine, array: u32, buffer: u32, size: u32, reserved: u32) {
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 8, size);
        e.mem.set_u32(array + 0xc, reserved);
    }

    /// Fills the current nodes with positions and tangents.
    fn put_current_nodes(
        e: &mut Engine,
        solution: Ptr<PathingSolution>,
        positions: &[[f32; 3]],
        tangents: &[[f32; 3]],
    ) -> u32 {
        let count = positions.len() as u32;
        let buffer = e.mem.alloc(count * CURRENT_NODE_SIZE);
        for (i, position) in positions.iter().enumerate() {
            let node = buffer + i as u32 * CURRENT_NODE_SIZE;
            put_point(e, node + 8, *position);
            if let Some(tangent) = tangents.get(i) {
                put_point(e, node + 0x2c, *tangent);
            }
        }
        set_array(e, solution.addr() + 0x20, buffer, count, count);
        buffer
    }

    fn put_virtual_nodes(
        e: &mut Engine,
        solution: Ptr<PathingSolution>,
        positions: &[[f32; 3]],
        flags: &[u32],
    ) -> u32 {
        let count = positions.len() as u32;
        let buffer = e.mem.alloc(count * VIRTUAL_NODE_SIZE);
        for (i, position) in positions.iter().enumerate() {
            let node = buffer + i as u32 * VIRTUAL_NODE_SIZE;
            e.mem.set_u32(node, flags[i]);
            put_point(e, node + 8, *position);
        }
        set_array(e, solution.addr() + 8, buffer, count, count);
        buffer
    }

    fn point(e: &Engine, address: u32) -> [f32; 3] {
        [
            e.mem.f32(address),
            e.mem.f32(address + 4),
            e.mem.f32(address + 8),
        ]
    }

    fn put_point(e: &mut Engine, address: u32, value: [f32; 3]) {
        for (k, v) in value.iter().enumerate() {
            e.mem.set_f32(address + 4 * k as u32, *v);
        }
    }

    /// Four nodes of a square path with zero tangents: each segment is a
    /// straight line of length 100.
    fn square_path(e: &mut Engine) -> Ptr<PathingSolution> {
        let sol = solution(e);
        put_current_nodes(
            e,
            sol,
            &[
                [0.0, 0.0, 0.0],
                [100.0, 0.0, 0.0],
                [100.0, 100.0, 0.0],
                [0.0, 100.0, 0.0],
            ],
            &[],
        );
        sol
    }

    /// A curve along x from 0 to 100 with zero tangents.
    fn straight_segment(e: &mut Engine) -> Ptr<CurveSegment> {
        let curve = e.new_object::<CurveSegment>();
        put_point(e, curve.addr() + SEGMENT_END, [100.0, 0.0, 0.0]);
        curve
    }

    fn calls(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.clone().unwrap()
    }

    fn addresses(e: &Engine) -> Vec<u32> {
        calls(e).iter().map(|c| c.0).collect()
    }

    #[test]
    fn table_has_every_function_once() {
        let table = funcs();
        assert_eq!(table.len(), 53);
        let mut seen: Vec<u32> = table.iter().map(|f| f.0).collect();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(seen.len(), 53);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_bit_0() {
        let mut e = engine();
        stub(
            &mut e,
            &[
                VIRTUAL_ARRAY_CLEAR,
                CURRENT_ARRAY_CLEAR,
                DOOR_IDS_CLEAR,
                DOOR_IDS_DESTRUCT,
                REF_OBJECT_DESTRUCT,
                OPERATOR_DELETE_SIZED,
            ],
        );
        let sol = solution(&mut e);
        e.call_log = Some(vec![]);
        let result = pathing_solution_scalar_deleting_destructor(&mut e, sol, 1);
        assert_eq!(result, sol);
        assert_eq!(
            calls(&e).last().unwrap(),
            &(OPERATOR_DELETE_SIZED, vec![sol.addr(), 0x44])
        );
        e.call_log = Some(vec![]);
        pathing_solution_scalar_deleting_destructor(&mut e, sol, 0);
        assert!(!addresses(&e).contains(&OPERATOR_DELETE_SIZED));
        assert!(addresses(&e).contains(&REF_OBJECT_DESTRUCT));
    }

    #[test]
    fn destructor_resets_then_destroys_members_in_order() {
        let mut e = engine();
        stub(
            &mut e,
            &[
                VIRTUAL_ARRAY_CLEAR,
                CURRENT_ARRAY_CLEAR,
                DOOR_IDS_CLEAR,
                DOOR_IDS_DESTRUCT,
                REF_OBJECT_DESTRUCT,
            ],
        );
        let sol = solution(&mut e);
        e.call_log = Some(vec![]);
        fn_006e7720(&mut e, sol);
        assert_eq!(e.mem.u32(sol.addr()), SOLUTION_VTABLE);
        assert_eq!(
            calls(&e),
            vec![
                (VIRTUAL_ARRAY_CLEAR, vec![sol.addr() + 8, 1]),
                (CURRENT_ARRAY_CLEAR, vec![sol.addr() + 0x20, 1]),
                (DOOR_IDS_CLEAR, vec![sol.addr() + 0x30, 1]),
                (DOOR_IDS_DESTRUCT, vec![sol.addr() + 0x30]),
                (CURRENT_ARRAY_CLEAR, vec![sol.addr() + 0x20, 1]),
                (VIRTUAL_ARRAY_CLEAR, vec![sol.addr() + 8, 1]),
                (REF_OBJECT_DESTRUCT, vec![sol.addr()]),
            ]
        );
        // The array destructors set their own vtables.
        assert_eq!(e.mem.u32(sol.addr() + 8), VIRTUAL_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(sol.addr() + 0x20), CURRENT_ARRAY_VTABLE);
    }

    #[test]
    fn reset_clears_arrays_and_indices() {
        let mut e = engine();
        stub(
            &mut e,
            &[VIRTUAL_ARRAY_CLEAR, CURRENT_ARRAY_CLEAR, DOOR_IDS_CLEAR],
        );
        let sol = solution(&mut e);
        e.set(sol, PathingSolution::iFirstLoadedVirtualNodeIndex, 4);
        e.set(sol, PathingSolution::iLastLoadedVirtualNodeIndex, 9);
        e.set(sol, PathingSolution::bIncompletePath, true);
        e.call_log = Some(vec![]);
        fn_006e77b0(&mut e, sol);
        assert_eq!(
            addresses(&e),
            vec![VIRTUAL_ARRAY_CLEAR, CURRENT_ARRAY_CLEAR, DOOR_IDS_CLEAR]
        );
        assert_eq!(
            e.get(sol, PathingSolution::iFirstLoadedVirtualNodeIndex),
            -1
        );
        assert_eq!(e.get(sol, PathingSolution::iLastLoadedVirtualNodeIndex), -1);
        assert!(!e.get(sol, PathingSolution::bIncompletePath));
    }

    #[test]
    fn virtual_path_length_skips_pairs_after_flagged_nodes() {
        let mut e = engine();
        let sol = solution(&mut e);
        let nodes = put_virtual_nodes(
            &mut e,
            sol,
            &[
                [0.0, 0.0, 0.0],
                [3.0, 4.0, 50.0],
                [3.0, 10.0, 0.0],
                [9.0, 12.0, 0.0],
            ],
            &[0, 0, 4, 0],
        );
        // (0,0)-(3,4) = 5 and (3,4)-(3,10) = 6; the pair after the flagged
        // node 2 is skipped.
        assert_eq!(fn_006e7800(&mut e, sol), 11.0);
        // Without the flag the last pair counts: (3,10)-(9,12) = sqrt(40).
        e.mem.set_u32(nodes + 2 * VIRTUAL_NODE_SIZE, 0);
        let expected = (40.0f64.sqrt() + 11.0) as f32;
        assert_eq!(fn_006e7800(&mut e, sol), expected);
    }

    #[test]
    fn virtual_node_accessor_indexes_the_first_array() {
        let mut e = engine();
        let sol = solution(&mut e);
        set_array(&mut e, sol.addr() + 8, BUFFER, 5, 5);
        assert_eq!(fn_006e78b0(&mut e, sol, 3).addr(), BUFFER + 3 * 0x30);
    }

    #[test]
    fn current_path_length_sums_3d_distances() {
        let mut e = engine();
        let sol = solution(&mut e);
        put_current_nodes(
            &mut e,
            sol,
            &[[0.0, 0.0, 0.0], [3.0, 4.0, 12.0], [3.0, 4.0, 13.0]],
            &[],
        );
        assert_eq!(fn_006e78d0(&mut e, sol), 14.0);
        let empty = solution(&mut e);
        assert_eq!(fn_006e78d0(&mut e, empty), 0.0);
    }

    #[test]
    fn current_node_accessor_indexes_the_second_array() {
        let mut e = engine();
        let sol = solution(&mut e);
        set_array(&mut e, sol.addr() + 0x20, BUFFER, 5, 5);
        assert_eq!(fn_006e7970(&mut e, sol, 3).addr(), BUFFER + 3 * 0x3c);
    }

    #[test]
    fn segment_length_between_nodes() {
        let mut e = engine();
        let sol = square_path(&mut e);
        assert_eq!(fn_006e7990(&mut e, sol, 1), 100.0);
        // The last node, and anything past it, has no next node.
        assert_eq!(fn_006e7990(&mut e, sol, 3), 0.0);
        assert_eq!(fn_006e7990(&mut e, sol, 7), 0.0);
    }

    #[test]
    fn point_at_parameter_is_on_the_curve_or_the_last_node() {
        let mut e = engine();
        let sol = square_path(&mut e);
        let out = e.mem.alloc(12);
        // Half way along the first segment (zero tangents: a smooth step).
        e.call_log = Some(vec![]);
        fn_006e7a10(&mut e, sol, 0.5, Ptr::new(out));
        assert_eq!(point(&e, out), [50.0, 0.0, 0.0]);
        assert!(addresses(&e).contains(&SEGMENT_DESTRUCT));
        // A quarter of the way down the second segment.
        fn_006e7a10(&mut e, sol, 1.25, Ptr::new(out));
        assert_eq!(point(&e, out), [100.0, 15.625, 0.0]);
        // At and beyond the last node: its position.
        fn_006e7a10(&mut e, sol, 3.0, Ptr::new(out));
        assert_eq!(point(&e, out), [0.0, 100.0, 0.0]);
        fn_006e7a10(&mut e, sol, 9.5, Ptr::new(out));
        assert_eq!(point(&e, out), [0.0, 100.0, 0.0]);
    }

    #[test]
    fn segment_constructor_sets_the_end_blocks_to_the_zero_point() {
        let mut e = engine();
        put_point(&mut e, ZERO_POINT, [1.0, 2.0, 3.0]);
        let curve = e.new_object::<CurveSegment>();
        e.mem.set_f32(curve.addr(), 7.0);
        e.call_log = Some(vec![]);
        assert_eq!(fn_006e7b60(&mut e, curve), curve);
        assert_eq!(calls(&e), vec![(TRIVIAL_CONSTRUCT, vec![curve.addr()])]);
        for block in [SEGMENT_START, SEGMENT_END] {
            assert_eq!(point(&e, curve.addr() + block), [1.0, 2.0, 3.0]);
            assert_eq!(point(&e, curve.addr() + block + 12), [1.0, 2.0, 3.0]);
        }
        // The parameter is not touched.
        assert_eq!(e.mem.f32(curve.addr()), 7.0);
    }

    #[test]
    fn block_initializer_copies_the_zero_point_twice() {
        let mut e = engine();
        put_point(&mut e, ZERO_POINT, [1.0, 2.0, 3.0]);
        let block = e.mem.alloc(24);
        assert_eq!(fn_006e7bd0(&mut e, Ptr::new(block)).addr(), block);
        assert_eq!(point(&e, block), [1.0, 2.0, 3.0]);
        assert_eq!(point(&e, block + 12), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn parameter_setter_forwards_to_the_weights() {
        let mut e = engine();
        let curve = e.new_object::<CurveSegment>();
        fn_006e7c20(&mut e, curve, 0.5);
        assert_eq!(e.get(curve, CurveSegment::fParameter), 0.5);
        assert_eq!(e.get(curve, CurveSegment::fWeightEndPosition), 0.5);
    }

    #[test]
    fn weights_follow_the_cubic_blend() {
        let mut e = engine();
        let curve = e.new_object::<CurveSegment>();
        fn_006e7c40(&mut e, curve, 0.25);
        assert_eq!(e.get(curve, CurveSegment::fParameter), 0.25);
        // t^2 = 0.0625, t^3 = 0.015625.
        assert_eq!(e.get(curve, CurveSegment::fWeightEndPosition), 0.15625);
        assert_eq!(e.get(curve, CurveSegment::fWeightStartPosition), 0.84375);
        assert_eq!(e.get(curve, CurveSegment::fWeightEndTangent), -0.046875);
        assert_eq!(e.get(curve, CurveSegment::fWeightStartTangent), 0.140625);
        fn_006e7c40(&mut e, curve, 0.0);
        assert_eq!(e.get(curve, CurveSegment::fWeightStartPosition), 1.0);
        assert_eq!(e.get(curve, CurveSegment::fWeightEndPosition), 0.0);
        fn_006e7c40(&mut e, curve, 1.0);
        assert_eq!(e.get(curve, CurveSegment::fWeightStartPosition), 0.0);
        assert_eq!(e.get(curve, CurveSegment::fWeightEndPosition), 1.0);
    }

    fn two_blocks(e: &mut Engine) -> (Ptr, Ptr) {
        let start = e.mem.alloc(24);
        let end = e.mem.alloc(24);
        put_point(e, start, [0.0, 0.0, 0.0]);
        put_point(e, start + 12, [8.0, 16.0, 24.0]);
        put_point(e, end, [10.0, 20.0, 30.0]);
        put_point(e, end + 12, [0.0, 0.0, 0.0]);
        (Ptr::new(start), Ptr::new(end))
    }

    #[test]
    fn blend_loads_the_blocks_and_returns_out() {
        let mut e = engine();
        let curve = e.new_object::<CurveSegment>();
        fn_006e7c40(&mut e, curve, 0.5);
        let (start, end) = two_blocks(&mut e);
        let out = e.mem.alloc(12);
        assert_eq!(
            fn_006e7cc0(&mut e, curve, Ptr::new(out), start, end).addr(),
            out
        );
        assert_eq!(point(&e, curve.addr() + SEGMENT_START), [0.0, 0.0, 0.0]);
        assert_eq!(
            point(&e, curve.addr() + SEGMENT_START + 12),
            [8.0, 16.0, 24.0]
        );
        assert_eq!(point(&e, curve.addr() + SEGMENT_END), [10.0, 20.0, 30.0]);
        assert_eq!(point(&e, out), [6.0, 12.0, 15.0]);
    }

    #[test]
    fn block_copy_copies_24_bytes() {
        let mut e = engine();
        let (start, _) = two_blocks(&mut e);
        let copy = e.mem.alloc(24);
        assert_eq!(fn_006e7d10(&mut e, Ptr::new(copy), start).addr(), copy);
        assert_eq!(point(&e, copy), [0.0, 0.0, 0.0]);
        assert_eq!(point(&e, copy + 12), [8.0, 16.0, 24.0]);
    }

    #[test]
    fn blend_uses_both_tangents_in_x_and_y_but_none_in_z() {
        let mut e = engine();
        let curve = e.new_object::<CurveSegment>();
        // t = 0.5: weights 0.5, 0.125, 0.5, -0.125.
        fn_006e7c40(&mut e, curve, 0.5);
        let (start, end) = two_blocks(&mut e);
        put_point(&mut e, end.addr() + 12, [16.0, 8.0, 1000.0]);
        let out = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        assert_eq!(
            fn_006e7d60(&mut e, curve, Ptr::new(out), start, end).addr(),
            out
        );
        // x = 0.5*0 + 0.125*8 + 0.5*10 - 0.125*16, y = 0.125*16 + 0.5*20 - 0.125*8
        // z = 0.5*0 + 0.5*30 (tangent z is not used).
        assert_eq!(point(&e, out), [4.0, 11.0, 15.0]);
        assert_eq!(addresses(&e), vec![TRIVIAL_CONSTRUCT]);
    }

    #[test]
    fn weighted_sum_uses_the_four_weights() {
        let mut e = engine();
        let curve = e.new_object::<CurveSegment>();
        e.set(curve, CurveSegment::fWeightStartPosition, 0.5);
        e.set(curve, CurveSegment::fWeightStartTangent, 0.25);
        e.set(curve, CurveSegment::fWeightEndPosition, 0.125);
        e.set(curve, CurveSegment::fWeightEndTangent, 0.0625);
        let values = e.mem.alloc(16);
        put_point(&mut e, values, [2.0, 4.0, 8.0]);
        e.mem.set_f32(values + 12, 16.0);
        let at = |k: u32| Ptr::new(values + 4 * k);
        // start 2, end 4, start tangent 8, end tangent 16:
        // 0.5*2 + 0.25*8 + 0.125*4 + 0.0625*16.
        assert_eq!(fn_006e9000(&mut e, curve, at(0), at(1), at(2), at(3)), 4.5);
    }

    #[test]
    fn weighted_sum_keeps_the_x87_nan_when_two_nans_meet() {
        // #199: the weights hold -qNaN and +qNaN; FADDP keeps the positive one.
        let mut e = engine();
        let curve = e.new_object::<CurveSegment>();
        e.set(
            curve,
            CurveSegment::fWeightStartPosition,
            f32::from_bits(0xffff_ffff),
        );
        e.set(curve, CurveSegment::fWeightStartTangent, 0.25);
        e.set(curve, CurveSegment::fWeightEndPosition, 0.125);
        e.set(
            curve,
            CurveSegment::fWeightEndTangent,
            f32::from_bits(0x7fff_ffff),
        );
        let values = e.mem.alloc(16);
        put_point(&mut e, values, [1.0, 1.0, 1.0]);
        e.mem.set_f32(values + 12, 1.0);
        let at = |k: u32| Ptr::new(values + 4 * k);
        let result = fn_006e9000(&mut e, curve, at(0), at(1), at(2), at(3));
        assert_eq!(result.to_bits(), 0x7fff_ffff);
    }

    #[test]
    fn node_block_has_position_and_tangent() {
        let mut e = engine();
        let sol = solution(&mut e);
        let nodes = put_current_nodes(&mut e, sol, &[[1.0, 2.0, 3.0]], &[[4.0, 5.0, 6.0]]);
        let out = e.mem.alloc(24);
        assert_eq!(
            fn_006e7e10(&mut e, Ptr::new(nodes), Ptr::new(out)).addr(),
            out
        );
        assert_eq!(point(&e, out), [1.0, 2.0, 3.0]);
        assert_eq!(point(&e, out + 12), [4.0, 5.0, 6.0]);
    }

    #[test]
    fn forward_walk_stops_inside_a_segment() {
        let mut e = engine();
        let sol = square_path(&mut e);
        let out_parameter = e.mem.alloc(4);
        let out_point = e.mem.alloc(12);
        // 50 units from the start, tolerance 1: half way along segment 0.
        let found = fn_006e7e70(
            &mut e,
            sol,
            0.0,
            50.0,
            true,
            Ptr::new(out_parameter),
            Ptr::new(out_point),
        );
        assert!(found);
        assert_eq!(e.mem.f32(out_parameter), 0.5);
        assert_eq!(point(&e, out_point), [50.0, 0.0, 0.0]);
        // 150 units: the first segment is used up, 50 units into segment 1.
        let found = fn_006e7e70(
            &mut e,
            sol,
            0.0,
            150.0,
            true,
            Ptr::new(out_parameter),
            Ptr::new(out_point),
        );
        assert!(found);
        assert_eq!(e.mem.f32(out_parameter), 1.5);
        assert_eq!(point(&e, out_point), [100.0, 50.0, 0.0]);
    }

    #[test]
    fn forward_walk_reports_the_end_of_the_path() {
        let mut e = engine();
        let sol = square_path(&mut e);
        let out_parameter = e.mem.alloc(4);
        let out_point = e.mem.alloc(12);
        // At the last node already.
        assert!(!fn_006e7e70(
            &mut e,
            sol,
            3.0,
            10.0,
            true,
            Ptr::new(out_parameter),
            Ptr::new(out_point)
        ));
        assert_eq!(e.mem.f32(out_parameter), 3.0);
        assert_eq!(point(&e, out_point), [0.0, 100.0, 0.0]);
        // At the last node the out pointers may be null (the search routine
        // that writes through them is not reached).
        assert!(!fn_006e7e70(
            &mut e,
            sol,
            3.0,
            1000.0,
            true,
            Ptr::NULL,
            Ptr::NULL
        ));
        // Running off the end reports the last node.
        assert!(!fn_006e7e70(
            &mut e,
            sol,
            0.0,
            1000.0,
            true,
            Ptr::new(out_parameter),
            Ptr::new(out_point)
        ));
        assert_eq!(e.mem.f32(out_parameter), 3.0);
        assert_eq!(point(&e, out_point), [0.0, 100.0, 0.0]);
    }

    #[test]
    fn walk_without_nodes_fails() {
        let mut e = engine();
        let sol = solution(&mut e);
        assert!(!fn_006e7e70(
            &mut e,
            sol,
            0.0,
            1.0,
            true,
            Ptr::NULL,
            Ptr::NULL
        ));
    }

    #[test]
    fn backward_walk_moves_towards_the_start() {
        let mut e = engine();
        let sol = square_path(&mut e);
        let out_parameter = e.mem.alloc(4);
        let out_point = e.mem.alloc(12);
        // From 1.5 (halfway down segment 1) 50 units back: node 1.
        let found = fn_006e7e70(
            &mut e,
            sol,
            1.5,
            50.0,
            false,
            Ptr::new(out_parameter),
            Ptr::new(out_point),
        );
        assert!(found);
        assert_eq!(e.mem.f32(out_parameter), 1.0);
        assert_eq!(point(&e, out_point), [100.0, 0.0, 0.0]);
        // Another 100 units back from 1.5 passes node 1 into segment 0.
        let found = fn_006e7e70(
            &mut e,
            sol,
            1.5,
            100.0,
            false,
            Ptr::new(out_parameter),
            Ptr::new(out_point),
        );
        assert!(found);
        assert_eq!(e.mem.f32(out_parameter), 0.5);
        assert_eq!(point(&e, out_point), [50.0, 0.0, 0.0]);
    }

    #[test]
    fn backward_walk_reports_the_start_of_the_path() {
        let mut e = engine();
        let sol = square_path(&mut e);
        let out_parameter = e.mem.alloc(4);
        let out_point = e.mem.alloc(12);
        // Already at the start.
        assert!(!fn_006e7e70(
            &mut e,
            sol,
            0.0,
            10.0,
            false,
            Ptr::new(out_parameter),
            Ptr::new(out_point)
        ));
        assert_eq!(e.mem.f32(out_parameter), 0.0);
        assert_eq!(point(&e, out_point), [0.0, 0.0, 0.0]);
        // Walking back past the start.
        e.mem.set_f32(out_parameter, 9.0);
        assert!(!fn_006e7e70(
            &mut e,
            sol,
            1.5,
            1000.0,
            false,
            Ptr::new(out_parameter),
            Ptr::new(out_point)
        ));
        assert_eq!(e.mem.f32(out_parameter), 0.0);
        assert_eq!(point(&e, out_point), [0.0, 0.0, 0.0]);
        assert!(!fn_006e7e70(
            &mut e,
            sol,
            0.0,
            10.0,
            false,
            Ptr::NULL,
            Ptr::NULL
        ));
    }

    #[test]
    fn end_setter_loads_start_and_end_blocks() {
        let mut e = engine();
        let curve = e.new_object::<CurveSegment>();
        let (start, end) = two_blocks(&mut e);
        fn_006e8450(&mut e, curve, start, end);
        assert_eq!(
            point(&e, curve.addr() + SEGMENT_START + 12),
            [8.0, 16.0, 24.0]
        );
        assert_eq!(point(&e, curve.addr() + SEGMENT_END), [10.0, 20.0, 30.0]);
    }

    #[test]
    fn first_node_or_null() {
        let mut e = engine();
        let sol = square_path(&mut e);
        let buffer = e.mem.u32(sol.addr() + 0x24);
        assert_eq!(fn_006e8480(&mut e, sol).addr(), buffer);
        let empty = solution(&mut e);
        assert!(fn_006e8480(&mut e, empty).is_null());
    }

    #[test]
    fn length_up_to_a_parameter() {
        let mut e = engine();
        let sol = square_path(&mut e);
        // No nodes: zero.
        let empty = solution(&mut e);
        assert_eq!(fn_006e84b0(&mut e, empty, 1.0), 0.0);
        // At or beyond the last node: the whole path.
        assert_eq!(fn_006e84b0(&mut e, sol, 3.0), 300.0);
        assert_eq!(fn_006e84b0(&mut e, sol, 8.0), 300.0);
        // Half way along segment 1: 100 + 50.
        assert_eq!(fn_006e84b0(&mut e, sol, 1.5), 150.0);
        // Inside the first segment.
        assert_eq!(fn_006e84b0(&mut e, sol, 0.5), 50.0);
    }

    #[test]
    fn segment_containing_a_fraction_of_the_length() {
        let mut e = engine();
        let sol = square_path(&mut e);
        let index = e.mem.alloc(4);
        // Half of 300 is 150: 50 into segment 1.
        assert_eq!(fn_006e8680(&mut e, sol, 0.5, 300.0, Ptr::new(index)), 1.5);
        assert_eq!(e.mem.u32(index), 1);
        // The fraction is clamped to 1: the end of the last segment.
        assert_eq!(fn_006e8680(&mut e, sol, 7.0, 300.0, Ptr::new(index)), 3.0);
        assert_eq!(e.mem.u32(index), 2);
        // A total longer than the path never ends inside a segment.
        assert_eq!(fn_006e8680(&mut e, sol, 1.0, 400.0, Ptr::new(index)), 3.0);
        assert_eq!(e.mem.u32(index), 2);
        // Zero length: the first segment, nothing walked.
        assert_eq!(fn_006e8680(&mut e, sol, -1.0, 300.0, Ptr::new(index)), 0.0);
        assert_eq!(e.mem.u32(index), 0);
    }

    #[test]
    fn point_further_along_the_path() {
        let mut e = engine();
        let sol = square_path(&mut e);
        let out = e.mem.alloc(12);
        // No nodes: the zero point.
        let empty = solution(&mut e);
        put_point(&mut e, ZERO_POINT, [0.0, 0.0, 0.0]);
        put_point(&mut e, out, [9.0, 9.0, 9.0]);
        assert_eq!(
            fn_006e8790(&mut e, empty, Ptr::new(out), 0.0, 5.0).addr(),
            out
        );
        assert_eq!(point(&e, out), [0.0, 0.0, 0.0]);
        // Parameter at the last node: its position.
        fn_006e8790(&mut e, sol, Ptr::new(out), 3.0, 5.0);
        assert_eq!(point(&e, out), [0.0, 100.0, 0.0]);
        // 150 units from the start: 50 into segment 1.
        assert_eq!(
            fn_006e8790(&mut e, sol, Ptr::new(out), 0.0, 150.0).addr(),
            out
        );
        assert_eq!(point(&e, out), [100.0, 50.0, 0.0]);
        // 50 units from the start, inside the first segment.
        fn_006e8790(&mut e, sol, Ptr::new(out), 0.0, 50.0);
        assert_eq!(point(&e, out), [50.0, 0.0, 0.0]);
        // Further than the path: its last node.
        fn_006e8790(&mut e, sol, Ptr::new(out), 0.0, 1000.0);
        assert_eq!(point(&e, out), [0.0, 100.0, 0.0]);
    }

    #[test]
    fn point_search_failure_uses_the_segment_end() {
        let mut e = engine();
        // Two nodes joined by a strongly curved segment: its measured length
        // (two straight pieces through the middle, about 100) is longer than
        // the distance asked for (30), so the search runs, but the end is only
        // 10 units from the start. `fn_006e9830` fails and the segment's end
        // position is used.
        let sol = solution(&mut e);
        put_current_nodes(
            &mut e,
            sol,
            &[[0.0, 0.0, 0.0], [10.0, 0.0, 0.0]],
            &[[0.0, 400.0, 0.0]],
        );
        let out = e.mem.alloc(12);
        fn_006e8790(&mut e, sol, Ptr::new(out), 0.0, 30.0);
        assert_eq!(point(&e, out), [10.0, 0.0, 0.0]);
    }

    #[test]
    fn copy_of_a_solution() {
        let mut e = engine();
        stub(&mut e, &[VIRTUAL_ARRAY_CONSTRUCT, CURRENT_ARRAY_CONSTRUCT]);
        e.register(LOCATION_ASSIGN, |e, a| {
            copy_words(e, a[1], a[0], 10);
            a[0].into_ret()
        });
        let source = solution(&mut e);
        put_current_nodes(&mut e, source, &[[1.0, 2.0, 3.0]], &[[4.0, 5.0, 6.0]]);
        put_virtual_nodes(&mut e, source, &[[7.0, 8.0, 9.0]], &[5]);
        e.set(source, PathingSolution::iFirstLoadedVirtualNodeIndex, 2);
        e.set(source, PathingSolution::iLastLoadedVirtualNodeIndex, 6);
        e.set(source, PathingSolution::bIncompletePath, true);
        let copy = solution(&mut e);
        e.mem.set_u32(copy.addr() + 8, VIRTUAL_ARRAY_VTABLE);
        e.mem.set_u32(copy.addr() + 0x20, CURRENT_ARRAY_VTABLE);
        e.put_vtable(
            VIRTUAL_ARRAY_VTABLE,
            &[0x006e_94d0, 0x006e_91e0, 0x006d_b090, 0x006e_9210],
        );
        e.put_vtable(
            CURRENT_ARRAY_VTABLE,
            &[0x006e_9500, 0x006e_93c0, 0x006d_b090, 0x006e_93f0],
        );
        fn_006e8ab0(&mut e, source, copy);
        assert_eq!(
            e.get(copy, PathingSolution::iFirstLoadedVirtualNodeIndex),
            2
        );
        assert_eq!(e.get(copy, PathingSolution::iLastLoadedVirtualNodeIndex), 6);
        assert!(e.get(copy, PathingSolution::bIncompletePath));
        let virtual_nodes = e.mem.u32(copy.addr() + 0xc);
        assert_eq!(e.mem.u32(copy.addr() + 0x10), 1);
        assert_eq!(e.mem.u32(virtual_nodes), 5);
        assert_eq!(point(&e, virtual_nodes + 8), [7.0, 8.0, 9.0]);
        let current_nodes = e.mem.u32(copy.addr() + 0x24);
        assert_eq!(e.mem.u32(copy.addr() + 0x28), 1);
        assert_eq!(point(&e, current_nodes + 8), [1.0, 2.0, 3.0]);
        assert_eq!(point(&e, current_nodes + 0x2c), [4.0, 5.0, 6.0]);
    }

    /// Doubles for the save buffer calls: each records its words in a log.
    fn recording(e: &mut Engine, addresses: &[u32]) -> CallRecord {
        let log = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        for address in addresses {
            let log = log.clone();
            let address = *address;
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        log
    }

    #[test]
    fn save_writes_the_flag_indices_and_every_array() {
        let mut e = engine();
        let log = recording(
            &mut e,
            &[
                SAVE_BYTES,
                SAVE_SIZED_VALUE,
                SAVE_FORM_ID,
                VIRTUAL_NODE_SAVE,
                CURRENT_NODE_SAVE,
                INFO_ARRAY_CONSTRUCT,
                INFO_ARRAY_DESTRUCT,
            ],
        );
        e.register(POINTER_ARRAY_AT, |e, a| {
            (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret()
        });
        let sol = solution(&mut e);
        let virtual_nodes = put_virtual_nodes(&mut e, sol, &[[0.0; 3], [0.0; 3]], &[0, 0]);
        let current_nodes = put_current_nodes(&mut e, sol, &[[0.0; 3]], &[]);
        let doors = e.mem.alloc(8);
        e.mem.set_u32(doors, 0x0001_0aaa);
        e.mem.set_u32(doors + 4, 0x0001_0bbb);
        set_array(&mut e, sol.addr() + 0x30, doors, 2, 2);
        let buffer = e.mem.alloc(16);
        pathing_solution_save_game(&mut e, sol, Ptr::new(buffer));
        let log = log.borrow();
        let b = buffer;
        let s = sol.addr();
        let infos = log
            .iter()
            .find(|c| c.0 == INFO_ARRAY_CONSTRUCT)
            .map(|c| c.1[0])
            .unwrap();
        assert_eq!(
            *log,
            vec![
                (SAVE_BYTES, vec![b, s + 0x40, 1, 0]),
                (SAVE_BYTES, vec![b, s + 0x18, 4, 0]),
                (SAVE_BYTES, vec![b, s + 0x1c, 4, 0]),
                (SAVE_SIZED_VALUE, vec![b, 2]),
                (VIRTUAL_NODE_SAVE, vec![virtual_nodes, b]),
                (VIRTUAL_NODE_SAVE, vec![virtual_nodes + 0x30, b]),
                (SAVE_SIZED_VALUE, vec![b, 1]),
                (CURRENT_NODE_SAVE, vec![current_nodes, b]),
                (INFO_ARRAY_CONSTRUCT, vec![infos]),
                (SAVE_SIZED_VALUE, vec![b, 0]),
                (SAVE_SIZED_VALUE, vec![b, 2]),
                (SAVE_FORM_ID, vec![b, 0x0001_0aaa, 0]),
                (SAVE_FORM_ID, vec![b, 0x0001_0bbb, 0]),
                (INFO_ARRAY_DESTRUCT, vec![infos]),
            ]
        );
    }

    /// A load buffer whose version slot (+0) returns `version`, and a
    /// `LOAD_SIZED_VALUE` that returns the given counts in turn.
    fn load_doubles(e: &mut Engine, version: u32, counts: Vec<u32>) -> (u32, CallRecord) {
        stub(e, &[VIRTUAL_ARRAY_CLEAR, CURRENT_ARRAY_CLEAR]);
        let log = recording(
            e,
            &[
                LOAD_BYTES,
                LOAD_FORM_ID,
                VIRTUAL_NODE_LOAD,
                CURRENT_NODE_LOAD,
                INFO_ARRAY_CONSTRUCT,
                INFO_ARRAY_DESTRUCT,
                INFO_ARRAY_SET_SIZE,
                INFO_LOAD,
                POINTER_ARRAY_SET_SIZE,
                VIRTUAL_ARRAY_CONSTRUCT,
                CURRENT_ARRAY_CONSTRUCT,
            ],
        );
        let mut counts = counts.into_iter();
        e.register_double(LOAD_SIZED_VALUE, move |_, _| {
            counts.next().unwrap_or(0).into_ret()
        });
        e.register(POINTER_ARRAY_AT, |e, a| {
            (e.mem.u32(a[0] + 4) + 4 * a[1]).into_ret()
        });
        e.put_vtable(0x0050_0000, &[0x0050_0100]);
        e.register_double(0x0050_0100, move |_, _| version.into_ret());
        e.put_vtable(
            VIRTUAL_ARRAY_VTABLE,
            &[0x006e_94d0, 0x006e_91e0, 0x006d_b090, 0x006e_9210],
        );
        e.put_vtable(
            CURRENT_ARRAY_VTABLE,
            &[0x006e_9500, 0x006e_93c0, 0x006d_b090, 0x006e_93f0],
        );
        let buffer = e.mem.alloc(16);
        e.mem.set_u32(buffer, 0x0050_0000);
        (buffer, log)
    }

    #[test]
    fn load_sizes_the_arrays_and_reads_door_ids_from_version_0x12() {
        let mut e = engine();
        let (buffer, log) = load_doubles(&mut e, 0x12, vec![2, 1, 0, 3]);
        let sol = solution(&mut e);
        e.mem.set_u32(sol.addr() + 8, VIRTUAL_ARRAY_VTABLE);
        e.mem.set_u32(sol.addr() + 0x20, CURRENT_ARRAY_VTABLE);
        let doors = e.mem.alloc(12);
        set_array(&mut e, sol.addr() + 0x30, doors, 3, 3);
        pathing_solution_load_game(&mut e, sol, Ptr::new(buffer));
        let s = sol.addr();
        // The arrays were sized by the counts.
        assert_eq!(e.mem.u32(s + 0x10), 2);
        assert_eq!(e.mem.u32(s + 0x28), 1);
        let virtual_nodes = e.mem.u32(s + 0xc);
        let current_nodes = e.mem.u32(s + 0x24);
        let log = log.borrow();
        let infos = log
            .iter()
            .find(|c| c.0 == INFO_ARRAY_CONSTRUCT)
            .map(|c| c.1[0])
            .unwrap();
        assert_eq!(
            *log,
            vec![
                (LOAD_BYTES, vec![buffer, s + 0x40, 1]),
                (LOAD_BYTES, vec![buffer, s + 0x18, 4]),
                (LOAD_BYTES, vec![buffer, s + 0x1c, 4]),
                (VIRTUAL_ARRAY_CONSTRUCT, vec![s + 8, virtual_nodes, 2]),
                (VIRTUAL_NODE_LOAD, vec![virtual_nodes, buffer]),
                (VIRTUAL_NODE_LOAD, vec![virtual_nodes + 0x30, buffer]),
                (CURRENT_ARRAY_CONSTRUCT, vec![s + 0x20, current_nodes, 1]),
                (CURRENT_NODE_LOAD, vec![current_nodes, buffer]),
                (INFO_ARRAY_CONSTRUCT, vec![infos]),
                (INFO_ARRAY_SET_SIZE, vec![infos, 0, 1]),
                (POINTER_ARRAY_SET_SIZE, vec![s + 0x30, 3, 1]),
                (LOAD_FORM_ID, vec![buffer, doors]),
                (LOAD_FORM_ID, vec![buffer, doors + 4]),
                (LOAD_FORM_ID, vec![buffer, doors + 8]),
                (INFO_ARRAY_DESTRUCT, vec![infos]),
            ]
        );
    }

    #[test]
    fn load_skips_door_ids_before_version_0x12() {
        let mut e = engine();
        let (buffer, log) = load_doubles(&mut e, 0x11, vec![0, 0, 0, 5]);
        let sol = solution(&mut e);
        e.mem.set_u32(sol.addr() + 8, VIRTUAL_ARRAY_VTABLE);
        e.mem.set_u32(sol.addr() + 0x20, CURRENT_ARRAY_VTABLE);
        pathing_solution_load_game(&mut e, sol, Ptr::new(buffer));
        let log = log.borrow();
        assert!(!log.iter().any(|c| c.0 == POINTER_ARRAY_SET_SIZE));
        assert!(!log.iter().any(|c| c.0 == LOAD_FORM_ID));
        assert_eq!(log.last().unwrap().0, INFO_ARRAY_DESTRUCT);
    }

    #[test]
    fn element_accessors_scale_the_index() {
        let mut e = engine();
        let array = e.mem.alloc(16);
        set_array(&mut e, array, BUFFER, 5, 8);
        let array = Ptr::<BSSimpleArray>::new(array);
        assert_eq!(fn_006e8f40(&mut e, array, 3).addr(), BUFFER + 3 * 0x30);
        assert_eq!(fn_006e8f60(&mut e, array, 4).addr(), BUFFER + 4 * 0x30);
        assert_eq!(fn_006e8f80(&mut e, array, 3).addr(), BUFFER + 3 * 0x3c);
        assert_eq!(fn_006e8fa0(&mut e, array, 4).addr(), BUFFER + 4 * 0x3c);
        assert_eq!(fn_006e8fc0(&mut e, array).addr(), BUFFER);
        // The last element is `iSize - 1`.
        assert_eq!(fn_006e8fe0(&mut e, array).addr(), BUFFER + 4 * 0x3c);
    }

    #[test]
    fn virtual_array_constructor_sets_the_vtable_and_an_empty_array() {
        let mut e = engine();
        let array = e.mem.alloc(16);
        set_array(&mut e, array, 5, 6, 7);
        let result = fn_006e9050(&mut e, Ptr::new(array));
        assert_eq!(result.addr(), array);
        assert_eq!(e.mem.u32(array), VIRTUAL_ARRAY_VTABLE);
        assert_eq!(
            [
                e.mem.u32(array + 4),
                e.mem.u32(array + 8),
                e.mem.u32(array + 0xc)
            ],
            [0, 0, 0]
        );
    }

    #[test]
    fn virtual_array_destructor_clears_with_release() {
        let mut e = engine();
        stub(&mut e, &[VIRTUAL_ARRAY_CLEAR]);
        let array = e.mem.alloc(16);
        e.call_log = Some(vec![]);
        fn_006e9080(&mut e, Ptr::new(array));
        assert_eq!(e.mem.u32(array), VIRTUAL_ARRAY_VTABLE);
        assert_eq!(calls(&e), vec![(VIRTUAL_ARRAY_CLEAR, vec![array, 1])]);
    }

    fn virtual_array(e: &mut Engine) -> u32 {
        let array = e.mem.alloc(16);
        e.mem.set_u32(array, VIRTUAL_ARRAY_VTABLE);
        e.put_vtable(
            VIRTUAL_ARRAY_VTABLE,
            &[0x006e_94d0, 0x006e_91e0, 0x006d_b090, 0x006e_9210],
        );
        stub(
            e,
            &[
                VIRTUAL_ARRAY_CLEAR,
                VIRTUAL_ARRAY_REALLOCATE,
                VIRTUAL_ARRAY_DESTROY,
                VIRTUAL_ARRAY_CONSTRUCT,
            ],
        );
        array
    }

    #[test]
    fn virtual_array_set_size_zero_clears() {
        let mut e = engine();
        let array = virtual_array(&mut e);
        e.call_log = Some(vec![]);
        fn_006e90a0(&mut e, Ptr::new(array), 0, true);
        fn_006e90a0(&mut e, Ptr::new(array), 0, false);
        assert_eq!(
            calls(&e),
            vec![
                (VIRTUAL_ARRAY_CLEAR, vec![array, 1]),
                (VIRTUAL_ARRAY_CLEAR, vec![array, 0]),
            ]
        );
    }

    #[test]
    fn virtual_array_set_size_grows_by_allocating_then_reallocating() {
        let mut e = engine();
        let array = virtual_array(&mut e);
        e.call_log = Some(vec![]);
        // Empty array: the buffer comes from the virtual `_Allocate`.
        fn_006e90a0(&mut e, Ptr::new(array), 3, false);
        let buffer = e.mem.u32(array + 4);
        assert_ne!(buffer, 0);
        assert_eq!(e.mem.u32(array + 8), 3);
        assert_eq!(e.mem.u32(array + 0xc), 3);
        assert_eq!(
            calls(&e),
            vec![
                (VIRTUAL_ALLOCATE, vec![array, 3]),
                (OPERATOR_NEW, vec![3 * 0x30]),
                (VIRTUAL_ARRAY_CONSTRUCT, vec![array, buffer, 3]),
            ]
        );
        // Beyond the reserve: reallocated to the new size, the new tail built.
        e.call_log = Some(vec![]);
        fn_006e90a0(&mut e, Ptr::new(array), 5, false);
        assert_eq!(e.mem.u32(array + 0xc), 5);
        assert_eq!(e.mem.u32(array + 8), 5);
        assert_eq!(
            calls(&e),
            vec![
                (VIRTUAL_ARRAY_REALLOCATE, vec![array, 5, 3]),
                (VIRTUAL_ARRAY_CONSTRUCT, vec![array, buffer + 3 * 0x30, 2]),
            ]
        );
    }

    #[test]
    fn virtual_array_set_size_shrinks_and_optionally_gives_memory_back() {
        let mut e = engine();
        let array = virtual_array(&mut e);
        set_array(&mut e, array, BUFFER, 8, 8);
        e.call_log = Some(vec![]);
        // 8 -> 6 destroys the two tail elements; 6 is not <= 8/4.
        fn_006e90a0(&mut e, Ptr::new(array), 6, true);
        assert_eq!(e.mem.u32(array + 8), 6);
        assert_eq!(e.mem.u32(array + 0xc), 8);
        assert_eq!(
            calls(&e),
            vec![(VIRTUAL_ARRAY_DESTROY, vec![array, BUFFER + 6 * 0x30, 2])]
        );
        // 6 -> 2 with shrinking allowed: 2 <= 8/4, so the buffer is reduced.
        e.call_log = Some(vec![]);
        fn_006e90a0(&mut e, Ptr::new(array), 2, true);
        assert_eq!(e.mem.u32(array + 0xc), 2);
        assert_eq!(
            calls(&e),
            vec![
                (VIRTUAL_ARRAY_DESTROY, vec![array, BUFFER + 2 * 0x30, 4]),
                (VIRTUAL_ARRAY_REALLOCATE, vec![array, 2, 2]),
            ]
        );
        // Without the flag the reserve stays.
        set_array(&mut e, array, BUFFER, 8, 8);
        e.call_log = Some(vec![]);
        fn_006e90a0(&mut e, Ptr::new(array), 2, false);
        assert_eq!(e.mem.u32(array + 0xc), 8);
        assert_eq!(addresses(&e), vec![VIRTUAL_ARRAY_DESTROY]);
    }

    #[test]
    fn virtual_array_set_size_grows_within_the_reserve() {
        let mut e = engine();
        let array = virtual_array(&mut e);
        set_array(&mut e, array, BUFFER, 2, 8);
        e.call_log = Some(vec![]);
        fn_006e90a0(&mut e, Ptr::new(array), 5, true);
        assert_eq!(e.mem.u32(array + 8), 5);
        assert_eq!(e.mem.u32(array + 0xc), 8);
        assert_eq!(
            calls(&e),
            vec![(VIRTUAL_ARRAY_CONSTRUCT, vec![array, BUFFER + 2 * 0x30, 3])]
        );
        // The same size again constructs nothing new (zero elements).
        fn_006e90a0(&mut e, Ptr::new(array), 5, true);
        assert_eq!(e.mem.u32(array + 8), 5);
    }

    #[test]
    fn virtual_array_allocate_and_reallocate_scale_by_the_element_size() {
        let mut e = engine();
        e.register(REALLOCATE_BYTES, |_, a| (a[0] + a[1]).into_ret());
        let array = Ptr::<BSSimpleArray>::new(BUFFER);
        e.call_log = Some(vec![]);
        let block = bs_simple_array_virtual_pathing_node_1024_allocate(&mut e, array, 4);
        assert_ne!(block, 0);
        assert_eq!(calls(&e), vec![(OPERATOR_NEW, vec![4 * 0x30])]);
        e.call_log = Some(vec![]);
        let result = bs_simple_array_virtual_pathing_node_1024_reallocate(&mut e, array, 0x1000, 3);
        assert_eq!(result, 0x1000 + 3 * 0x30);
        assert_eq!(calls(&e), vec![(REALLOCATE_BYTES, vec![0x1000, 3 * 0x30])]);
    }

    #[test]
    fn current_array_constructor_and_destructor() {
        let mut e = engine();
        stub(&mut e, &[CURRENT_ARRAY_CLEAR]);
        let array = e.mem.alloc(16);
        set_array(&mut e, array, 5, 6, 7);
        assert_eq!(fn_006e9230(&mut e, Ptr::new(array)).addr(), array);
        assert_eq!(e.mem.u32(array), CURRENT_ARRAY_VTABLE);
        assert_eq!(
            [
                e.mem.u32(array + 4),
                e.mem.u32(array + 8),
                e.mem.u32(array + 0xc)
            ],
            [0, 0, 0]
        );
        e.mem.set_u32(array, 0);
        e.call_log = Some(vec![]);
        fn_006e9260(&mut e, Ptr::new(array));
        assert_eq!(e.mem.u32(array), CURRENT_ARRAY_VTABLE);
        assert_eq!(calls(&e), vec![(CURRENT_ARRAY_CLEAR, vec![array, 1])]);
    }

    fn current_array(e: &mut Engine) -> u32 {
        let array = e.mem.alloc(16);
        e.mem.set_u32(array, CURRENT_ARRAY_VTABLE);
        e.put_vtable(
            CURRENT_ARRAY_VTABLE,
            &[0x006e_9500, 0x006e_93c0, 0x006d_b090, 0x006e_93f0],
        );
        stub(
            e,
            &[
                CURRENT_ARRAY_CLEAR,
                CURRENT_ARRAY_REALLOCATE,
                CURRENT_ARRAY_DESTROY,
                CURRENT_ARRAY_CONSTRUCT,
            ],
        );
        array
    }

    #[test]
    fn current_array_set_size_follows_the_same_rules() {
        let mut e = engine();
        let array = current_array(&mut e);
        e.call_log = Some(vec![]);
        fn_006e9280(&mut e, Ptr::new(array), 0, true);
        assert_eq!(calls(&e), vec![(CURRENT_ARRAY_CLEAR, vec![array, 1])]);
        // Grow from empty through `_Allocate`.
        e.call_log = Some(vec![]);
        fn_006e9280(&mut e, Ptr::new(array), 4, true);
        let buffer = e.mem.u32(array + 4);
        assert_eq!(
            calls(&e),
            vec![
                (CURRENT_ALLOCATE, vec![array, 4]),
                (OPERATOR_NEW, vec![4 * 0x3c]),
                (CURRENT_ARRAY_CONSTRUCT, vec![array, buffer, 4]),
            ]
        );
        // Grow past the reserve.
        e.call_log = Some(vec![]);
        fn_006e9280(&mut e, Ptr::new(array), 6, true);
        assert_eq!(
            calls(&e),
            vec![
                (CURRENT_ARRAY_REALLOCATE, vec![array, 6, 4]),
                (CURRENT_ARRAY_CONSTRUCT, vec![array, buffer + 4 * 0x3c, 2]),
            ]
        );
        // Shrink below a quarter of the reserve: the tail is destroyed and
        // the buffer reduced.
        e.call_log = Some(vec![]);
        fn_006e9280(&mut e, Ptr::new(array), 1, true);
        assert_eq!(
            calls(&e),
            vec![
                (CURRENT_ARRAY_DESTROY, vec![array, buffer + 0x3c, 5]),
                (CURRENT_ARRAY_REALLOCATE, vec![array, 1, 1]),
            ]
        );
        assert_eq!(e.mem.u32(array + 0xc), 1);
        // Grow within the reserve.
        set_array(&mut e, array, buffer, 1, 6);
        e.call_log = Some(vec![]);
        fn_006e9280(&mut e, Ptr::new(array), 3, false);
        assert_eq!(
            calls(&e),
            vec![(CURRENT_ARRAY_CONSTRUCT, vec![array, buffer + 0x3c, 2])]
        );
        assert_eq!(e.mem.u32(array + 8), 3);
        // Shrink without releasing.
        e.call_log = Some(vec![]);
        fn_006e9280(&mut e, Ptr::new(array), 2, false);
        assert_eq!(e.mem.u32(array + 0xc), 6);
        assert_eq!(addresses(&e), vec![CURRENT_ARRAY_DESTROY]);
    }

    #[test]
    fn current_array_allocate_and_reallocate_scale_by_the_element_size() {
        let mut e = engine();
        e.register(REALLOCATE_BYTES, |_, a| (a[0] + a[1]).into_ret());
        let array = Ptr::<BSSimpleArray>::new(BUFFER);
        e.call_log = Some(vec![]);
        bs_simple_array_pathing_node_1024_allocate(&mut e, array, 4);
        assert_eq!(calls(&e), vec![(OPERATOR_NEW, vec![4 * 0x3c])]);
        let result = bs_simple_array_pathing_node_1024_reallocate(&mut e, array, 0x1000, 3);
        assert_eq!(result, 0x1000 + 3 * 0x3c);
    }

    #[test]
    fn array_copies_size_the_target_and_copy_each_element() {
        let mut e = engine();
        e.register(LOCATION_ASSIGN, |e, a| {
            copy_words(e, a[1], a[0], 10);
            a[0].into_ret()
        });
        let source = e.mem.alloc(16);
        let nodes = e.mem.alloc(2 * 0x30);
        for i in 0..2u32 {
            e.mem.set_u32(nodes + i * 0x30, 10 + i);
            e.mem.set_u32(nodes + i * 0x30 + 0x2c, 0x1000 + i);
            e.mem.set_f32(nodes + i * 0x30 + 8, i as f32 + 1.0);
        }
        set_array(&mut e, source, nodes, 2, 2);
        let target = virtual_array(&mut e);
        e.call_log = Some(vec![]);
        fn_006e9410(&mut e, Ptr::new(source), Ptr::new(target));
        assert_eq!(e.mem.u32(target + 8), 2);
        let copy = e.mem.u32(target + 4);
        assert_ne!(copy, nodes);
        assert_eq!(e.mem.u32(copy + 0x30), 11);
        assert_eq!(e.mem.u32(copy + 0x30 + 0x2c), 0x1001);
        assert_eq!(e.mem.f32(copy + 0x30 + 8), 2.0);
        let assigned: Vec<_> = calls(&e)
            .into_iter()
            .filter(|c| c.0 == LOCATION_ASSIGN)
            .collect();
        assert_eq!(
            assigned,
            vec![
                (LOCATION_ASSIGN, vec![copy + 4, nodes + 4]),
                (LOCATION_ASSIGN, vec![copy + 0x30 + 4, nodes + 0x30 + 4]),
            ]
        );

        // The current-node copy does the same with 0x3C-byte elements.
        let source = e.mem.alloc(16);
        let nodes = e.mem.alloc(0x3c);
        e.mem.set_u32(nodes, 77);
        put_point(&mut e, nodes + 0x2c, [1.0, 2.0, 3.0]);
        e.mem.set_u32(nodes + 0x38, 0x2222);
        set_array(&mut e, source, nodes, 1, 1);
        let target = current_array(&mut e);
        fn_006e9470(&mut e, Ptr::new(source), Ptr::new(target));
        let copy = e.mem.u32(target + 4);
        assert_eq!(e.mem.u32(target + 8), 1);
        assert_eq!(e.mem.u32(copy), 77);
        assert_eq!(point(&e, copy + 0x2c), [1.0, 2.0, 3.0]);
        assert_eq!(e.mem.u32(copy + 0x38), 0x2222);
    }

    #[test]
    fn scalar_deleting_destructors_of_the_arrays() {
        let mut e = engine();
        stub(&mut e, &[VIRTUAL_ARRAY_CLEAR, CURRENT_ARRAY_CLEAR]);
        let array = Ptr::<BSSimpleArray>::new(e.mem.alloc(16));
        e.call_log = Some(vec![]);
        let result =
            bs_simple_array_virtual_pathing_node_1024_scalar_deleting_destructor(&mut e, array, 1);
        assert_eq!(result, array);
        assert_eq!(
            calls(&e),
            vec![
                (VIRTUAL_ARRAY_CLEAR, vec![array.addr(), 1]),
                (OPERATOR_DELETE, vec![array.addr()]),
            ]
        );
        e.call_log = Some(vec![]);
        bs_simple_array_pathing_node_1024_scalar_deleting_destructor(&mut e, array, 0);
        assert_eq!(
            calls(&e),
            vec![(CURRENT_ARRAY_CLEAR, vec![array.addr(), 1])]
        );
        e.call_log = Some(vec![]);
        bs_simple_array_pathing_node_1024_scalar_deleting_destructor(&mut e, array, 3);
        assert_eq!(addresses(&e), vec![CURRENT_ARRAY_CLEAR, OPERATOR_DELETE]);
    }

    #[test]
    fn node_copy_assignment() {
        let mut e = engine();
        e.register(LOCATION_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], 0xabcd);
            a[0].into_ret()
        });
        let from = e.mem.alloc(0x3c);
        let to = e.mem.alloc(0x3c);
        e.mem.set_u32(from, 6);
        e.mem.set_u32(from + 0x2c, 0x77);
        e.call_log = Some(vec![]);
        let result = fn_006e9530(&mut e, Ptr::new(to), Ptr::new(from));
        assert_eq!(result.addr(), to);
        assert_eq!(e.mem.u32(to), 6);
        assert_eq!(e.mem.u32(to + 0x2c), 0x77);
        assert_eq!(calls(&e), vec![(LOCATION_ASSIGN, vec![to + 4, from + 4])]);

        put_point(&mut e, from + 0x2c, [1.0, 2.0, 3.0]);
        e.mem.set_u32(from + 0x38, 0x55);
        let result = fn_006e9570(&mut e, Ptr::new(to), Ptr::new(from));
        assert_eq!(result.addr(), to);
        assert_eq!(e.mem.u32(to), 6);
        assert_eq!(point(&e, to + 0x2c), [1.0, 2.0, 3.0]);
        assert_eq!(e.mem.u32(to + 0x38), 0x55);
    }

    #[test]
    fn sized_array_constructors() {
        let mut e = engine();
        let array = virtual_array(&mut e);
        e.call_log = Some(vec![]);
        // Capacity 4, size 2: four elements allocated, two constructed.
        fn_006e9670(&mut e, Ptr::new(array), 4, 2);
        let buffer = e.mem.u32(array + 4);
        assert_eq!(e.mem.u32(array + 8), 2);
        assert_eq!(e.mem.u32(array + 0xc), 4);
        assert_eq!(
            calls(&e),
            vec![
                (VIRTUAL_ALLOCATE, vec![array, 4]),
                (OPERATOR_NEW, vec![4 * 0x30]),
                (VIRTUAL_ARRAY_CONSTRUCT, vec![array, buffer, 2]),
            ]
        );
        // Capacity below size is raised to the size.
        fn_006e9670(&mut e, Ptr::new(array), 1, 3);
        assert_eq!(e.mem.u32(array + 0xc), 3);
        assert_eq!(e.mem.u32(array + 8), 3);
        // Nothing requested: nothing allocated.
        e.call_log = Some(vec![]);
        fn_006e9670(&mut e, Ptr::new(array), 0, 0);
        assert!(calls(&e).is_empty());
        assert_eq!(e.mem.u32(array + 4), 0);

        let array = current_array(&mut e);
        e.call_log = Some(vec![]);
        fn_006e97a0(&mut e, Ptr::new(array), 2, 5);
        let buffer = e.mem.u32(array + 4);
        assert_eq!(e.mem.u32(array + 0xc), 5);
        assert_eq!(
            calls(&e),
            vec![
                (CURRENT_ALLOCATE, vec![array, 5]),
                (OPERATOR_NEW, vec![5 * 0x3c]),
                (CURRENT_ARRAY_CONSTRUCT, vec![array, buffer, 5]),
            ]
        );
        e.call_log = Some(vec![]);
        fn_006e97a0(&mut e, Ptr::new(array), 0, 0);
        assert!(calls(&e).is_empty());
    }

    #[test]
    fn search_returns_the_start_when_the_distance_is_within_tolerance() {
        let mut e = engine();
        let curve = straight_segment(&mut e);
        let out_parameter = e.mem.alloc(4);
        let out_point = e.mem.alloc(12);
        let found = fn_006e9830(
            &mut e,
            curve,
            0.25,
            5.0,
            true,
            Ptr::new(out_parameter),
            Ptr::new(out_point),
            10.0,
        );
        assert!(found);
        assert_eq!(e.mem.f32(out_parameter), 0.25);
        // 100 * (3 * 0.0625 - 2 * 0.015625)
        assert_eq!(point(&e, out_point), [15.625, 0.0, 0.0]);
    }

    #[test]
    fn search_bisects_forward_and_backward() {
        let mut e = engine();
        let curve = straight_segment(&mut e);
        let out_parameter = e.mem.alloc(4);
        let out_point = e.mem.alloc(12);
        // Forward from 0: the point 50 units away (within 10) is at 0.5.
        assert!(fn_006e9830(
            &mut e,
            curve,
            0.0,
            50.0,
            true,
            Ptr::new(out_parameter),
            Ptr::new(out_point),
            10.0
        ));
        assert_eq!(e.mem.f32(out_parameter), 0.5);
        assert_eq!(point(&e, out_point), [50.0, 0.0, 0.0]);
        // Backward from 1.
        assert!(fn_006e9830(
            &mut e,
            curve,
            1.0,
            50.0,
            false,
            Ptr::new(out_parameter),
            Ptr::new(out_point),
            10.0
        ));
        assert_eq!(e.mem.f32(out_parameter), 0.5);
        assert_eq!(point(&e, out_point), [50.0, 0.0, 0.0]);
        // A start beyond 1 is clamped.
        assert!(fn_006e9830(
            &mut e,
            curve,
            4.0,
            50.0,
            false,
            Ptr::new(out_parameter),
            Ptr::new(out_point),
            10.0
        ));
        assert_eq!(e.mem.f32(out_parameter), 0.5);
        // A search that needs several halvings: 80 units forward from 0.
        assert!(fn_006e9830(
            &mut e,
            curve,
            0.0,
            80.0,
            true,
            Ptr::new(out_parameter),
            Ptr::new(out_point),
            10.0
        ));
        let t = e.mem.f32(out_parameter);
        let reached = point(&e, out_point)[0];
        assert!((70.0..=90.0).contains(&reached), "{t} {reached}");
    }

    #[test]
    fn search_failure_reports_the_far_end() {
        let mut e = engine();
        let curve = straight_segment(&mut e);
        let out_parameter = e.mem.alloc(4);
        let out_point = e.mem.alloc(12);
        // 500 units cannot be reached along 100: the forward search ends at
        // the parameter limit and reports the end node.
        assert!(!fn_006e9830(
            &mut e,
            curve,
            0.0,
            500.0,
            true,
            Ptr::new(out_parameter),
            Ptr::new(out_point),
            10.0
        ));
        assert_eq!(e.mem.f32(out_parameter), 1.0);
        assert_eq!(point(&e, out_point), [100.0, 0.0, 0.0]);
        // Backward the failure reports the start node.
        put_point(&mut e, curve.addr() + SEGMENT_START, [-1.0, 2.0, 3.0]);
        assert!(!fn_006e9830(
            &mut e,
            curve,
            1.0,
            500.0,
            false,
            Ptr::new(out_parameter),
            Ptr::new(out_point),
            10.0
        ));
        assert_eq!(e.mem.f32(out_parameter), 0.0);
        assert_eq!(point(&e, out_point), [-1.0, 2.0, 3.0]);
    }

    #[test]
    fn curve_length_is_the_sum_of_the_pieces() {
        let mut e = engine();
        let curve = straight_segment(&mut e);
        assert_eq!(fn_006e9af0(&mut e, curve, 0.0, 1.0, 2), 100.0);
        assert_eq!(fn_006e9af0(&mut e, curve, 0.0, 1.0, 1), 100.0);
        // From 0 to 0.5 in two pieces: 0.25 -> 15.625, 0.5 -> 50.
        assert_eq!(fn_006e9af0(&mut e, curve, 0.0, 0.5, 2), 50.0);
        // No steps: no length.
        assert_eq!(fn_006e9af0(&mut e, curve, 0.0, 1.0, 0), 0.0);
        // Parameters are clamped to 0..1: 0 to 4 in one step is 0 to 1.
        assert_eq!(fn_006e9af0(&mut e, curve, 0.0, 4.0, 1), 100.0);
    }
}
