//! `BSMain/bscompoundfrustum.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `BSCompoundFrustum` is a small program of frustum tests. Its operations
//! (`FunctionOp`: `op`, `jp`, `jf`) sit in an array of 12-byte entries; the
//! plane sets they test (`NiFrustumPlanes`, 0x64 bytes each) sit in a second
//! array. `iFreeOp` and `iFreePlane` count the used entries of the two (the
//! arrays' own `iSize` is their constructed size, which the grow helpers
//! keep). After `PrethreadOpList` every test operation carries the index of
//! the next operation to run when it passes (`jp`) and when it fails (`jf`),
//! and `Process` and its two siblings walk the list from `iFirstOp` until
//! they reach an end-of-list pass or fail.
//!
//! Operation codes (`BSCompoundFrustum::BSCFOperator`, Xbox PDB): see the
//! `OP_*` constants. A test operation (`OP_PORTAL`, `OP_OCCLUSION`) is
//! followed by one more entry whose `op` word holds the index of the plane
//! set it tests; the list walkers skip it.
//!
//! The array templates this unit instantiates (`BSSimpleArray` of
//! `NiFrustumPlanes` and of `FunctionOp`) are translated here as well. They
//! reach the allocator through the array's own vtable (slot +4 allocates
//! `count` elements, +8 frees, +0xC reallocates `(block, count)`), exactly as
//! the game does, so the tests put the two vtables in memory.
//!
//! `Clear` (`00c4a9c0`) is shared by both arrays and is called by address
//! ([`ARRAY_CLEAR`]); the unit is complete.
//!
//! C++ exception unwinding frames (constructor, destructor,
//! `_ConstructItems`) are not translated.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, NiPoint3};
use crate::units::platform::MEMORY_MANAGER;

// ---------------------------------------------------------------------------
// Constants

/// `BSCompoundFrustum::BSCFOperator` (Xbox PDB): `BSCF_OP_EOL`, the end of
/// the program as written (`PrethreadOpList` appends one).
pub(crate) const OP_END_OF_LIST: i32 = 1;
/// `BSCF_OP_EOLPASS`: the walk ends and the test passes.
pub(crate) const OP_END_PASS: i32 = 2;
/// `BSCF_OP_EOLFAIL`: the walk ends and the test fails.
pub(crate) const OP_END_FAIL: i32 = 3;
/// `BSCF_OP_GROUPINTERSECTION`: opens a group (written by `00c47a00`).
pub(crate) const OP_GROUP_INTERSECTION: i32 = 4;
/// `BSCF_OP_GROUPUNION`: opens a group (written by `StartGroupUnion`).
pub(crate) const OP_GROUP_UNION: i32 = 5;
/// `BSCF_OP_GROUPEND`: closes the innermost group.
pub(crate) const OP_GROUP_END: i32 = 6;
/// `BSCF_OP_PORTAL`: a test operation, followed by its plane set index.
pub(crate) const OP_PORTAL: i32 = 7;
/// `BSCF_OP_OCCLUSION`: a test operation, followed by its plane set index.
pub(crate) const OP_OCCLUSION: i32 = 8;

/// Size of a `NiFrustumPlanes` (six `NiPlane`s and the active-plane mask).
pub(crate) const FRUSTUM_PLANES_SIZE: u32 = 0x64;
/// Size of a `FunctionOp`.
pub(crate) const FUNCTION_OP_SIZE: u32 = 0xc;
/// Mask with the six planes of a `NiFrustumPlanes` active.
const ALL_PLANES_ACTIVE: u32 = 0x3f;

/// Vtable of the `BSSimpleArray<NiFrustumPlanes, 1024>` member: slot 0
/// `00c4a960`, +4 `00c4a860`, +8 `00c4a890`, +0xC `00c4a8b0`.
pub(crate) const PLANES_ARRAY_VTABLE: u32 = 0x010c_1e88;
/// Vtable of the `BSSimpleArray<FunctionOp, 1024>` member: slot 0
/// `00c4a990`, +4 `00c4a900`, +8 `00c4a890`, +0xC `00c4a930`.
pub(crate) const OPS_ARRAY_VTABLE: u32 = 0x010c_1e9c;
/// Slots of those vtables: allocate `(count)`, free `(block)` and reallocate
/// `(block, count)`.
const ARRAY_SLOT_ALLOCATE: u32 = 0x4;
const ARRAY_SLOT_FREE: u32 = 0x8;
const ARRAY_SLOT_REALLOCATE: u32 = 0xc;

/// `BSSimpleArray<...>::Clear(bool free)`: empties the array and, with the
/// flag, frees its buffer (shared by both arrays).
pub(crate) const ARRAY_CLEAR: u32 = 0x00c4_a9c0;
/// `MemoryManager::Deallocate(block)` (Xbox PDB), a method of the singleton
/// at [`MEMORY_MANAGER`].
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// `NiPlane::NiPlane()` (Xbox PDB).
const NI_PLANE_CONSTRUCT: u32 = 0x00a6_9940;
/// `NiPlane::NiPlane_ov4(v0, v1, v2)` (Xbox PDB): the plane through three
/// points; returns `this`.
const NI_PLANE_FROM_POINTS: u32 = 0x00a6_99d0;
/// `NiFrustumPlanes::NiFrustumPlanes()` (Xbox PDB).
const NI_FRUSTUM_PLANES_CONSTRUCT: u32 = 0x0045_c620;
/// `NiFrustumPlanes::Set(a, b)` (Xbox PDB).
const NI_FRUSTUM_PLANES_SET: u32 = 0x00a7_4e10;
/// `NiBound::WhichSide(plane)` (Xbox PDB), `this` the bound.
const NI_BOUND_WHICH_SIDE: u32 = 0x004b_6100;
/// `NiPoint3::Dot(other)` (Xbox PDB), result in `ST0`.
const NI_POINT3_DOT: u32 = 0x004b_6190;
/// The signed distance test of a point against a plane (`this` the plane,
/// then the point): 0 on the plane, 1 in front, 2 behind.
const PLANE_SIDE_OF_POINT: u32 = 0x0049_da80;
/// `fabs` (`00ec6cde`), double in, double out.
const FABS: u32 = 0x00ec_6cde;
/// `memset`.
const MEMSET: u32 = 0x00ec_61c0;
/// `memmove`.
const MEMMOVE: u32 = 0x00ec_7230;
/// `MemoryManager::Allocate(size)` and the reallocation routine `(block,
/// size)`, methods of the singleton at [`MEMORY_MANAGER`].
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_REALLOCATE: u32 = 0x00aa_4150;
/// `MemoryManager::GetThreadScrapHeap()`, `ScrapHeap::Allocate(size,
/// alignment)` and `ScrapHeap::Deallocate(block)` (Xbox PDB).
const GET_THREAD_SCRAP_HEAP: u32 = 0x00aa_42e0;
const SCRAP_HEAP_ALLOCATE: u32 = 0x00aa_54a0;
const SCRAP_HEAP_DEALLOCATE: u32 = 0x00aa_5610;
/// The alignment the scrap arrays pass to `ScrapHeap::Allocate`.
const SCRAP_ALIGNMENT: u32 = 0x010a_2720;
/// `BSOcclusionPlane` methods (Xbox PDB names where the map has them):
/// `GetFrustumPlanes(x, y, z, out)`, `GetVertices(out)`,
/// `GetJoinedPlaneType()`, `WithinFrustum(planes, flag, x, y, z)`,
/// `WithinFrustumFullTest(planes)`, and the two unnamed siblings
/// `00c344c0(planes, position)` and `00c347d0(planes)`.
const OCCLUSION_GET_FRUSTUM_PLANES: u32 = 0x00c3_3870;
const OCCLUSION_GET_VERTICES: u32 = 0x00c3_5c10;
const OCCLUSION_GET_JOINED_PLANE_TYPE: u32 = 0x00c3_5980;
const OCCLUSION_WITHIN_FRUSTUM: u32 = 0x00c3_3cf0;
const OCCLUSION_WITHIN_FRUSTUM_FULL_TEST: u32 = 0x00c3_4350;
const OCCLUSION_TEST_AGAINST_VIEW: u32 = 0x00c3_44c0;
const OCCLUSION_TEST_PLANES: u32 = 0x00c3_47d0;
/// `00b5aa00`: four flag bytes of an occlusion plane (+0xE6 to +0xE9) packed
/// into a 4-bit value, a bit set for each byte that is zero.
const OCCLUSION_FLAG_BITS: u32 = 0x00b5_aa00;
/// The move routine the `FunctionOp` array shares with another template
/// (`006dc6e0(this, dst, src, count)`, 12-byte elements).
const MOVE_FUNCTION_OPS: u32 = 0x006d_c6e0;
/// A debug-message routine (`005b5e40(text)`), called with the address of
/// the text.
const DEBUG_MESSAGE: u32 = 0x005b_5e40;
/// `"BSCompoundFrustum : prethreading an effectively empty op list"`.
const EMPTY_OP_LIST_TEXT: u32 = 0x010c_1e44;
/// The import slot of `DebugBreak`.
const DEBUG_BREAK: u32 = 0x00fd_f0c8;
/// The camera position the occlusion code reads (three `float`s).
const VIEW_POSITION_GLOBAL: u32 = 0x011f_426c;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `5.0` (`double`): how far from the view position the portal's plane has
/// to be for its planes to stay active.
const DISTANCE_TOLERANCE: u32 = 0x0102_0998;

// ---------------------------------------------------------------------------
// Layouts

layout! {
    /// `NiFrustumPlanes` (Xbox PDB), 0x64 bytes: six `NiPlane`s, then the
    /// mask of the active planes.
    pub struct NiFrustumPlanes: 0x64 {
        /// `m_uiActivePlanes` (Xbox PDB): bit `i` set when plane `i` is used.
        0x60 m_uiActivePlanes: u32,
    }

    /// `BSCompoundFrustum::FunctionOp` (Xbox PDB), 0xC bytes.
    pub struct FunctionOp: 0x0c {
        /// `op` (Xbox PDB): one of the `OP_*` codes (or, in the entry after
        /// a test operation, the plane set index).
        0x00 op: i32,
        /// `jp` (Xbox PDB): the operation to run when the test passes.
        0x04 jp: i32,
        /// `jf` (Xbox PDB): the operation to run when the test fails.
        0x08 jf: i32,
    }

    /// `BSCompoundFrustum` (Xbox PDB), 0xA4 bytes. It has no vtable.
    pub struct BSCompoundFrustum: 0xa4 {
        /// `bPrethreaded` (Xbox PDB).
        0x00 bPrethreaded: bool,
        /// `kPlanes` (Xbox PDB): `BSSimpleArray<NiFrustumPlanes, 1024>`.
        0x04 kPlanes: Inline<BSSimpleArray>,
        /// `kFunctionOperators` (Xbox PDB):
        /// `BSSimpleArray<FunctionOp, 1024>`.
        0x14 kFunctionOperators: Inline<BSSimpleArray>,
        /// `iFreePlane` (Xbox PDB): plane sets in use.
        0x24 iFreePlane: i32,
        /// `iFreeOp` (Xbox PDB): operations in use.
        0x28 iFreeOp: i32,
        /// `iFirstOp` (Xbox PDB): where the walk starts.
        0x2C iFirstOp: i32,
        /// `kViewFrustum` (Xbox PDB): `NiFrustumPlanes`.
        0x30 kViewFrustum: Inline<NiFrustumPlanes>,
        /// `kViewPosition` (Xbox PDB): `NiPoint3`.
        0x94 kViewPosition: Inline<NiPoint3>,
        /// `bSkipViewFrustum` (Xbox PDB).
        0xA0 bSkipViewFrustum: bool,
    }
}

type Frustum = Ptr<BSCompoundFrustum>;
type Array = Ptr<BSSimpleArray>;

// ---------------------------------------------------------------------------
// Helpers

fn planes_array(this: Frustum) -> Array {
    this.at(BSCompoundFrustum::kPlanes)
}

fn ops_array(this: Frustum) -> Array {
    this.at(BSCompoundFrustum::kFunctionOperators)
}

fn view_position(this: Frustum) -> Ptr<NiPoint3> {
    this.at(BSCompoundFrustum::kViewPosition)
}

fn view_frustum(this: Frustum) -> Ptr<NiFrustumPlanes> {
    this.at(BSCompoundFrustum::kViewFrustum)
}

/// The three `float` words of the view position, as the stack arguments the
/// occlusion plane methods take them in.
fn view_position_words(e: &Engine, this: Frustum) -> [u32; 3] {
    let at = view_position(this).addr();
    [e.mem.u32(at), e.mem.u32(at + 4), e.mem.u32(at + 8)]
}

/// Address of operation `index` (12-byte entries from the array's buffer).
fn op_address(e: &Engine, this: Frustum, index: i32) -> u32 {
    e.get(ops_array(this), BSSimpleArray::pBuffer)
        .wrapping_add((index as u32).wrapping_mul(FUNCTION_OP_SIZE))
}

fn op_code(e: &Engine, this: Frustum, index: i32) -> i32 {
    e.mem.i32(op_address(e, this, index))
}

fn set_op_code(e: &mut Engine, this: Frustum, index: i32, value: i32) {
    let at = op_address(e, this, index);
    e.mem.set_i32(at, value);
}

fn op_jump_pass(e: &Engine, this: Frustum, index: i32) -> i32 {
    e.mem.i32(op_address(e, this, index) + 4)
}

fn op_jump_fail(e: &Engine, this: Frustum, index: i32) -> i32 {
    e.mem.i32(op_address(e, this, index) + 8)
}

fn set_op_jump_pass(e: &mut Engine, this: Frustum, index: i32, value: i32) {
    let at = op_address(e, this, index) + 4;
    e.mem.set_i32(at, value);
}

fn set_op_jump_fail(e: &mut Engine, this: Frustum, index: i32, value: i32) {
    let at = op_address(e, this, index) + 8;
    e.mem.set_i32(at, value);
}

/// Appends an operation word at `iFreeOp` and advances `iFreeOp`.
fn append_op(e: &mut Engine, this: Frustum, value: i32) {
    let at = e.get(this, BSCompoundFrustum::iFreeOp);
    set_op_code(e, this, at, value);
    e.set(this, BSCompoundFrustum::iFreeOp, at.wrapping_add(1));
}

/// Address of plane set `index` (0x64-byte entries from the array's buffer).
fn plane_address(e: &Engine, this: Frustum, index: i32) -> u32 {
    e.get(planes_array(this), BSSimpleArray::pBuffer)
        .wrapping_add((index as u32).wrapping_mul(FRUSTUM_PLANES_SIZE))
}

/// Grows the operation array when `count` more entries would not leave it
/// room: `if (reserved <= iFreeOp + count) SetSize(reserved + grow, true)`.
fn ensure_op_room(e: &mut Engine, this: Frustum, count: i32, grow: i32) {
    let reserved = e.get(ops_array(this), BSSimpleArray::iReservedSize) as i32;
    let free = e.get(this, BSCompoundFrustum::iFreeOp);
    if reserved <= free.wrapping_add(count) {
        bs_simple_array_function_op_set_size(
            e,
            ops_array(this),
            reserved.wrapping_add(grow) as u32,
            true,
        );
    }
}

/// The same for the plane set array.
fn ensure_plane_room(e: &mut Engine, this: Frustum, count: i32, grow: i32) {
    let reserved = e.get(planes_array(this), BSSimpleArray::iReservedSize) as i32;
    let free = e.get(this, BSCompoundFrustum::iFreePlane);
    if reserved <= free.wrapping_add(count) {
        bs_simple_array_ni_frustum_planes_set_size(
            e,
            planes_array(this),
            reserved.wrapping_add(grow) as u32,
            true,
        );
    }
}

/// `rep movsd` of one `NiFrustumPlanes` (25 words).
fn copy_frustum_planes(e: &mut Engine, from: u32, to: u32) {
    let bytes = e.mem.bytes(from, FRUSTUM_PLANES_SIZE);
    e.mem.write(to, &bytes);
}

/// `MemoryManager::GetThreadScrapHeap()` of the singleton.
fn thread_scrap_heap(e: &mut Engine) -> u32 {
    e.call(GET_THREAD_SCRAP_HEAP, &args![MEMORY_MANAGER]).u32()
}

/// `ScrapHeap::Allocate(size, alignment)` with the scrap arrays' alignment.
fn scrap_allocate(e: &mut Engine, heap: u32, size: u32) -> u32 {
    let alignment: u32 = e.global(SCRAP_ALIGNMENT);
    e.call(SCRAP_HEAP_ALLOCATE, &args![heap, size, alignment])
        .u32()
}

/// Reads word `index` of a scrap array of 32-bit entries.
fn scrap_word(e: &Engine, array: u32, index: i32) -> u32 {
    e.mem
        .u32(array.wrapping_add((index as u32).wrapping_mul(4)))
}

fn set_scrap_word(e: &mut Engine, array: u32, index: i32, value: u32) {
    e.mem
        .set_u32(array.wrapping_add((index as u32).wrapping_mul(4)), value);
}

/// The walk the three test functions share: from `iFirstOp`, runs `test`
/// on every test operation's plane set (address) and follows `jp` when it
/// passes and `jf` otherwise (any other operation counts as a fail), until
/// an end-of-list pass or fail; returns whether it ended in a pass.
fn walk_ops(
    e: &mut Engine,
    this: Frustum,
    test: &mut dyn FnMut(&mut Engine, i32, u32) -> bool,
) -> bool {
    let mut index = e.get(this, BSCompoundFrustum::iFirstOp);
    loop {
        let op = op_code(e, this, index);
        if op == OP_END_PASS || op == OP_END_FAIL {
            break;
        }
        let mut passed = false;
        if op == OP_PORTAL || op == OP_OCCLUSION {
            let plane_set = op_code(e, this, index.wrapping_add(1));
            let planes = plane_address(e, this, plane_set);
            passed = test(e, op, planes);
        }
        index = if passed {
            op_jump_pass(e, this, index)
        } else {
            op_jump_fail(e, this, index)
        };
    }
    op_code(e, this, index) == OP_END_PASS
}

// ---------------------------------------------------------------------------
// BSCompoundFrustum

// Translated from 00c47660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCompoundFrustum::BSCompoundFrustum` (Xbox PDB): constructs the two
/// arrays, the six planes of the view frustum (all active), and starts empty
/// with room for 4 plane sets and 10 operations. The view position is taken
/// from the global camera position. Returns `this`. The exception frame is
/// not translated.
pub fn bs_compound_frustum_bs_compound_frustum(e: &mut Engine, this: Frustum) -> Frustum {
    fn_00c49fe0(e, planes_array(this));
    fn_00c4a1b0(e, ops_array(this));
    let mut plane = view_frustum(this).addr();
    for _ in 0..6 {
        e.call(NI_PLANE_CONSTRUCT, &args![plane]);
        plane = plane.wrapping_add(0x10);
    }
    e.set(
        view_frustum(this),
        NiFrustumPlanes::m_uiActivePlanes,
        ALL_PLANES_ACTIVE,
    );
    e.set(this, BSCompoundFrustum::iFreePlane, 0);
    e.set(this, BSCompoundFrustum::iFreeOp, 0);
    let position = view_position(this).addr();
    for word in 0..3 {
        let value: u32 = e.global(VIEW_POSITION_GLOBAL + word * 4);
        e.mem.set_u32(position + word * 4, value);
    }
    bs_simple_array_ni_frustum_planes_set_size(e, planes_array(this), 4, true);
    bs_simple_array_function_op_set_size(e, ops_array(this), 10, true);
    e.set(this, BSCompoundFrustum::bPrethreaded, false);
    e.set(this, BSCompoundFrustum::iFirstOp, 0);
    e.set(this, BSCompoundFrustum::bSkipViewFrustum, false);
    this
}

// Translated from 00c47770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCompoundFrustum::~BSCompoundFrustum` (Xbox PDB): clears and frees both
/// arrays (`Clear(true)`), then runs the two array destructors. The
/// exception frame is not translated.
pub fn bs_compound_frustum_destructor(e: &mut Engine, this: Frustum) {
    e.call(ARRAY_CLEAR, &args![planes_array(this), true]);
    e.call(ARRAY_CLEAR, &args![ops_array(this), true]);
    fn_00c4a8e0(e, ops_array(this));
    bs_simple_array_ni_frustum_planes_destructor(e, planes_array(this));
}

// Translated from 00c477f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCompoundFrustum::CopyMembers` (Xbox PDB): copies the counters, the
/// view position and frustum, the flag byte, then sizes `target`'s arrays to
/// this one's capacities and copies every operation and plane set (the
/// whole capacity, not only the used part).
pub fn bs_compound_frustum_copy_members(e: &mut Engine, this: Frustum, target: Frustum) {
    let free_op = e.get(this, BSCompoundFrustum::iFreeOp);
    e.set(target, BSCompoundFrustum::iFreeOp, free_op);
    let free_plane = e.get(this, BSCompoundFrustum::iFreePlane);
    e.set(target, BSCompoundFrustum::iFreePlane, free_plane);
    let first_op = e.get(this, BSCompoundFrustum::iFirstOp);
    e.set(target, BSCompoundFrustum::iFirstOp, first_op);
    for word in 0..3 {
        let value = e.mem.u32(view_position(this).addr() + word * 4);
        e.mem
            .set_u32(view_position(target).addr() + word * 4, value);
    }
    copy_frustum_planes(e, view_frustum(this).addr(), view_frustum(target).addr());
    let prethreaded = e.get(this, BSCompoundFrustum::bPrethreaded);
    e.set(target, BSCompoundFrustum::bPrethreaded, prethreaded);

    let op_capacity = e.get(ops_array(this), BSSimpleArray::iReservedSize);
    bs_simple_array_function_op_set_size(e, ops_array(target), op_capacity, true);
    let mut index = 0u32;
    while index < e.get(ops_array(this), BSSimpleArray::iReservedSize) {
        let from = op_address(e, this, index as i32);
        let to = op_address(e, target, index as i32);
        for word in 0..3 {
            let value = e.mem.u32(from + word * 4);
            e.mem.set_u32(to + word * 4, value);
        }
        index += 1;
    }

    let plane_capacity = e.get(planes_array(this), BSSimpleArray::iReservedSize);
    bs_simple_array_ni_frustum_planes_set_size(e, planes_array(target), plane_capacity, true);
    let mut index = 0u32;
    while index < e.get(planes_array(this), BSSimpleArray::iReservedSize) {
        let from = plane_address(e, this, index as i32);
        let to = plane_address(e, target, index as i32);
        copy_frustum_planes(e, from, to);
        index += 1;
    }
}

// Translated from 00c47980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCompoundFrustum::SetCamera` (Xbox PDB): with a camera, takes its
/// frustum (`NiFrustumPlanes::Set(camera + 0xDC, camera + 0x68)`) and its
/// position (+0x8C); then empties the operation and plane lists and clears
/// the flags.
pub fn bs_compound_frustum_set_camera(e: &mut Engine, this: Frustum, camera: Ptr) {
    if !camera.is_null() {
        let camera = camera.addr();
        e.call(
            NI_FRUSTUM_PLANES_SET,
            &args![view_frustum(this), camera + 0xdc, camera + 0x68],
        );
        for word in 0..3 {
            let value = e.mem.u32(camera + 0x8c + word * 4);
            e.mem.set_u32(view_position(this).addr() + word * 4, value);
        }
    }
    e.set(this, BSCompoundFrustum::iFreePlane, 0);
    e.set(this, BSCompoundFrustum::iFreeOp, 0);
    e.set(this, BSCompoundFrustum::bPrethreaded, false);
    e.set(this, BSCompoundFrustum::iFirstOp, 0);
    e.set(this, BSCompoundFrustum::bSkipViewFrustum, false);
}

// Translated from 00c47a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a group-intersection operation (`OP_GROUP_INTERSECTION`) to the
/// operation list; unlike `StartGroupUnion` it leaves `bPrethreaded` alone.
pub fn fn_00c47a00(e: &mut Engine, this: Frustum) {
    ensure_op_room(e, this, 1, 10);
    append_op(e, this, OP_GROUP_INTERSECTION);
}

// Translated from 00c47a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCompoundFrustum::StartGroupUnion` (Xbox PDB): clears `bPrethreaded` and
/// appends an `OP_GROUP_UNION` operation.
pub fn bs_compound_frustum_start_group_union(e: &mut Engine, this: Frustum) {
    ensure_op_room(e, this, 1, 10);
    e.set(this, BSCompoundFrustum::bPrethreaded, false);
    append_op(e, this, OP_GROUP_UNION);
}

// Translated from 00c47ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCompoundFrustum::EndGroup` (Xbox PDB): clears `bPrethreaded` and
/// appends an `OP_GROUP_END` operation.
pub fn bs_compound_frustum_end_group(e: &mut Engine, this: Frustum) {
    ensure_op_room(e, this, 1, 10);
    e.set(this, BSCompoundFrustum::bPrethreaded, false);
    append_op(e, this, OP_GROUP_END);
}

// Translated from 00c47b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends an `OP_PORTAL` test for an occlusion `plane` (the portal): the
/// operation, the index of a new plane set filled by
/// `BSOcclusionPlane::GetFrustumPlanes`, and, when the plane through the
/// portal's first three vertices lies more than 5.0 (a global) from the view
/// position, all six planes of the new set active (otherwise none). The
/// second stack word is never read.
pub fn fn_00c47b50(e: &mut Engine, this: Frustum, plane: Ptr, _unused_1: u32) {
    ensure_op_room(e, this, 2, 10);
    ensure_plane_room(e, this, 1, 4);
    e.with_stack(0x30 + 0x10, |e, scratch| {
        let vertices = scratch.addr();
        let portal_plane = vertices + 0x30;
        e.call(OCCLUSION_GET_VERTICES, &args![plane, vertices]);
        e.call(
            NI_PLANE_FROM_POINTS,
            &args![portal_plane, vertices, vertices + 12, vertices + 24],
        );
        append_op(e, this, OP_PORTAL);
        let free_plane = e.get(this, BSCompoundFrustum::iFreePlane);
        append_op(e, this, free_plane);
        let position = view_position_words(e, this);
        let planes = plane_address(e, this, free_plane);
        e.call(
            OCCLUSION_GET_FRUSTUM_PLANES,
            &args![plane, position[0], position[1], position[2], planes],
        );
        let dot = e
            .call(NI_POINT3_DOT, &args![portal_plane, view_position(this)])
            .f64();
        let constant = e.mem.f32(portal_plane + 12);
        // `FSUB` in extended precision, stored as a `float`.
        let distance = (dot - constant as f64) as f32;
        let magnitude = e.call(FABS, &args![distance as f64]).f64() as f32;
        let tolerance: f64 = e.global(DISTANCE_TOLERANCE);
        let mask = if magnitude as f64 > tolerance {
            ALL_PLANES_ACTIVE
        } else {
            0
        };
        e.mem.set_u32(planes + 0x60, mask);
        e.set(this, BSCompoundFrustum::iFreePlane, free_plane + 1);
    });
}

// Translated from 00c47d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCompoundFrustum::AddOcclusionPlane` (Xbox PDB): appends the tests for
/// an occlusion `plane`. `GetJoinedPlaneType` and the plane's joined plane
/// (a pointer in the slots at +0xEC, +0xF0, +0xF4, +0xF8 that the type and
/// the slots' contents select) decide the form:
///
/// - the joined plane's +0x40 is not 1, or the view position is on a
///   different side of the planes through the two planes' vertices, or there
///   is no joined plane: one `OP_OCCLUSION` test with a new plane set;
/// - otherwise, for type 0: the same single test; for type 1: an
///   intersection group with a test of the plane, a union of the joined
///   plane's and the plane's second tests, and the joined plane's test
///   (four new plane sets and 12 operations in all), the planes shared by
///   neighbouring tests made inactive.
pub fn bs_compound_frustum_add_occlusion_plane(e: &mut Engine, this: Frustum, plane: Ptr) {
    let plane_start = plane.addr();
    let joined_type = e.call(OCCLUSION_GET_JOINED_PLANE_TYPE, &args![plane]).i32();
    let joined: u32 = if joined_type == 1 {
        let slot = if e.mem.u32(plane_start + 0xf0) == 0 {
            3
        } else {
            1
        };
        e.mem.u32(plane_start + 0xec + slot * 4)
    } else {
        let slot = if e.mem.u32(plane_start + 0xec) != 0 {
            0
        } else {
            2
        };
        e.mem.u32(plane_start + 0xec + slot * 4)
    };

    let mut same_side = false;
    if joined != 0 {
        let position = view_position_words(e, this);
        let position_address = view_position(this).addr();
        same_side = e.with_stack(FRUSTUM_PLANES_SIZE + 0x30 + 0x10 + 0x10, |e, scratch| {
            let frustum = scratch.addr();
            let vertices = frustum + FRUSTUM_PLANES_SIZE;
            let first_plane = vertices + 0x30;
            let second_plane = first_plane + 0x10;
            e.call(NI_FRUSTUM_PLANES_CONSTRUCT, &args![frustum]);
            e.call(
                OCCLUSION_GET_FRUSTUM_PLANES,
                &args![plane, position[0], position[1], position[2], frustum],
            );
            e.call(OCCLUSION_GET_VERTICES, &args![plane, vertices]);
            e.call(
                NI_PLANE_FROM_POINTS,
                &args![first_plane, vertices, vertices + 12, vertices + 24],
            );
            let first_side = e
                .call(PLANE_SIDE_OF_POINT, &args![first_plane, position_address])
                .i32();
            e.call(
                OCCLUSION_GET_FRUSTUM_PLANES,
                &args![joined, position[0], position[1], position[2], frustum],
            );
            e.call(OCCLUSION_GET_VERTICES, &args![joined, vertices]);
            let made = e
                .call(
                    NI_PLANE_FROM_POINTS,
                    &args![second_plane, vertices, vertices + 12, vertices + 24],
                )
                .u32();
            // The plane the call returns is copied over the first one.
            let copy = e.mem.bytes(made, 0x10);
            e.mem.write(first_plane, &copy);
            let second_side = e
                .call(PLANE_SIDE_OF_POINT, &args![first_plane, position_address])
                .i32();
            first_side == second_side
        });
    }

    let joined_accepts = joined == 0 || e.mem.i32(joined + 0x40) == 1;
    if joined_accepts && same_side {
        if joined_type == 0 {
            add_single_occlusion_test(e, this, plane);
        } else if joined_type == 1 {
            ensure_op_room(e, this, 0xc, 0xc);
            ensure_plane_room(e, this, 4, 4);
            let first = e.get(this, BSCompoundFrustum::iFreePlane);
            for (op, plane_set) in [
                (OP_GROUP_INTERSECTION, None),
                (OP_OCCLUSION, Some(first)),
                (OP_OCCLUSION, Some(first + 1)),
                (OP_GROUP_UNION, None),
                (OP_OCCLUSION, Some(first + 2)),
                (OP_OCCLUSION, Some(first + 3)),
                (OP_GROUP_END, None),
                (OP_GROUP_END, None),
            ] {
                append_op(e, this, op);
                if let Some(plane_set) = plane_set {
                    append_op(e, this, plane_set);
                }
            }
            let position = view_position_words(e, this);
            let own_sets = [first, first + 2];
            for set in own_sets {
                let planes = plane_address(e, this, set);
                e.call(
                    OCCLUSION_GET_FRUSTUM_PLANES,
                    &args![plane, position[0], position[1], position[2], planes],
                );
            }
            // Which of the six planes of set 2 goes inactive depends on
            // which of the joined plane's slots (+0xF0) was used.
            let slot_first = e.mem.u32(plane_start + 0xf0) == 0;
            let shift_own: u32 = if slot_first { 3 } else { 2 };
            let planes = plane_address(e, this, first + 2);
            let mask = e.mem.u32(planes + 0x60);
            e.mem.set_u32(planes + 0x60, !(1u32 << shift_own) & mask);
            let slot: u32 = if slot_first { 3 } else { 1 };
            let joined_plane = e.mem.u32(plane_start + 0xec + slot * 4);
            let joined_sets = [first + 1, first + 3];
            for set in joined_sets {
                let planes = plane_address(e, this, set);
                e.call(
                    OCCLUSION_GET_FRUSTUM_PLANES,
                    &args![joined_plane, position[0], position[1], position[2], planes],
                );
            }
            let shift_joined = if slot_first {
                u32::from(shift_own == 4) + 4
            } else {
                u32::from(shift_own == 2) + 2
            };
            let planes = plane_address(e, this, first + 3);
            let mask = e.mem.u32(planes + 0x60);
            e.mem.set_u32(planes + 0x60, !(1u32 << shift_joined) & mask);
            e.set(this, BSCompoundFrustum::iFreePlane, first + 4);
        }
    } else {
        add_single_occlusion_test(e, this, plane);
    }
}

/// The one-test form of `AddOcclusionPlane`: room for 2 operations and a
/// plane set, `OP_OCCLUSION` and the set's index, the set filled from
/// `plane`.
fn add_single_occlusion_test(e: &mut Engine, this: Frustum, plane: Ptr) {
    ensure_op_room(e, this, 2, 10);
    ensure_plane_room(e, this, 1, 4);
    append_op(e, this, OP_OCCLUSION);
    let free_plane = e.get(this, BSCompoundFrustum::iFreePlane);
    append_op(e, this, free_plane);
    let position = view_position_words(e, this);
    let planes = plane_address(e, this, free_plane);
    e.call(
        OCCLUSION_GET_FRUSTUM_PLANES,
        &args![plane, position[0], position[1], position[2], planes],
    );
    e.set(this, BSCompoundFrustum::iFreePlane, free_plane + 1);
}

// Translated from 00c48800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCompoundFrustum::AddCompoundFrustum` (Xbox PDB): appends all of
/// `other`'s operations (the `op` word only) and plane sets to this one,
/// making room first, and moves the plane set indexes of the appended test
/// operations up by this frustum's plane count. `bPrethreaded` is cleared.
pub fn bs_compound_frustum_add_compound_frustum(e: &mut Engine, this: Frustum, other: Frustum) {
    let other_ops = e.get(other, BSCompoundFrustum::iFreeOp);
    let other_planes = e.get(other, BSCompoundFrustum::iFreePlane);
    ensure_op_room(
        e,
        this,
        other_ops.wrapping_add(1),
        other_ops.wrapping_add(1).max(10),
    );
    ensure_plane_room(
        e,
        this,
        other_planes.wrapping_add(1),
        other_planes.wrapping_add(1).max(4),
    );
    e.set(this, BSCompoundFrustum::bPrethreaded, false);

    let free_op = e.get(this, BSCompoundFrustum::iFreeOp);
    let free_plane = e.get(this, BSCompoundFrustum::iFreePlane);
    for index in 0..other_ops {
        let value = op_code(e, other, index);
        set_op_code(e, this, free_op.wrapping_add(index), value);
    }
    for index in 0..other_planes {
        let from = plane_address(e, other, index);
        let to = plane_address(e, this, free_plane.wrapping_add(index));
        copy_frustum_planes(e, from, to);
    }
    if free_plane != 0 {
        let mut index = free_op;
        while index < free_op.wrapping_add(other_ops) {
            let op = op_code(e, this, index);
            if (OP_PORTAL..=OP_OCCLUSION).contains(&op) {
                let shifted = op_code(e, this, index + 1).wrapping_add(free_plane);
                set_op_code(e, this, index + 1, shifted);
                index += 1;
            }
            index += 1;
        }
    }
    e.set(
        this,
        BSCompoundFrustum::iFreeOp,
        free_op.wrapping_add(other_ops),
    );
    e.set(
        this,
        BSCompoundFrustum::iFreePlane,
        free_plane.wrapping_add(other_planes),
    );
}

// Translated from 00c48a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Opens a gap of `count` operations at the start of the operation list (or,
/// for a negative `count`, removes the first `-count`): the `op` words move
/// up (growing the array if needed) and `iFreeOp` follows. With `clear`, the
/// entries 1 to `count - 1` of the gap are set to 0 (entry 0 is left for the
/// caller).
pub fn fn_00c48a10(e: &mut Engine, this: Frustum, count: i32, clear: bool) {
    if count > 0 {
        ensure_op_room(e, this, count, count.max(10));
        let free_op = e.get(this, BSCompoundFrustum::iFreeOp);
        let mut index = free_op.wrapping_add(count).wrapping_sub(1);
        while index >= count {
            let value = op_code(e, this, index - count);
            set_op_code(e, this, index, value);
            index -= 1;
        }
        if clear {
            while index > 0 {
                set_op_code(e, this, index, 0);
                index -= 1;
            }
        }
        e.set(
            this,
            BSCompoundFrustum::iFreeOp,
            free_op.wrapping_add(count),
        );
    } else if count < 0 {
        let removed = count.wrapping_neg();
        let free_op = e.get(this, BSCompoundFrustum::iFreeOp);
        if free_op > removed {
            let mut index = 0;
            while index < free_op.wrapping_sub(removed) {
                let value = op_code(e, this, index + removed);
                set_op_code(e, this, index, value);
                index += 1;
            }
            e.set(
                this,
                BSCompoundFrustum::iFreeOp,
                free_op.wrapping_sub(removed),
            );
        } else {
            e.set(this, BSCompoundFrustum::iFreeOp, 0);
        }
    }
}

/// Walks the operation list from entry 1, counting group nesting from 1
/// (`OP_GROUP_INTERSECTION` and `OP_GROUP_UNION` open, `OP_GROUP_END`
/// closes); true when the walk ends with the list used up and the nesting
/// back at 0, which means the first group spans the whole list.
fn first_group_spans_list(e: &Engine, this: Frustum) -> bool {
    let mut depth = 1;
    let mut index = 1;
    let free_op = e.get(this, BSCompoundFrustum::iFreeOp);
    while depth > 0 && index < free_op {
        let op = op_code(e, this, index);
        if op >= OP_GROUP_INTERSECTION {
            if op <= OP_GROUP_UNION {
                depth += 1;
            } else if op == OP_GROUP_END {
                depth -= 1;
            }
        }
        index += 1;
    }
    index == free_op && depth == 0
}

/// `00c48b90` and `00c48ca0`: wraps the whole operation list in a group
/// opened by `group_op`, unless entry 0 already is that group and it spans
/// the list; in that case drops the last entry instead.
fn wrap_in_group(e: &mut Engine, this: Frustum, group_op: i32) {
    let mut wrap = true;
    if op_code(e, this, 0) == group_op && first_group_spans_list(e, this) {
        wrap = false;
    }
    if wrap {
        ensure_op_room(e, this, 1, 10);
        fn_00c48a10(e, this, 1, false);
        set_op_code(e, this, 0, group_op);
    } else {
        let free_op = e.get(this, BSCompoundFrustum::iFreeOp);
        e.set(this, BSCompoundFrustum::iFreeOp, free_op.wrapping_sub(1));
    }
}

// Translated from 00c48b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Wraps the whole operation list in a group union: unless entry 0 already
/// opens an `OP_GROUP_UNION` that spans the list to its end, makes room,
/// shifts everything up by one (`00c48a10`) and writes `OP_GROUP_UNION` into
/// entry 0; when it does, drops the last entry instead.
pub fn fn_00c48b90(e: &mut Engine, this: Frustum) {
    wrap_in_group(e, this, OP_GROUP_UNION);
}

// Translated from 00c48ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `OP_GROUP_INTERSECTION` twin of `00c48b90`.
pub fn fn_00c48ca0(e: &mut Engine, this: Frustum) {
    wrap_in_group(e, this, OP_GROUP_INTERSECTION);
}

// Translated from 00c48db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCompoundFrustum::BuildForPortal` (Xbox PDB): builds this frustum from
/// `source` for the occlusion plane `portal`. Operations are copied one by
/// one; for every test operation (`OP_PORTAL` or `OP_OCCLUSION`) the plane
/// set is copied too, its operation entry records `iFreePlane`, and, when
/// `portal` passes `WithinFrustumFullTest` on the set and `00b5aa00(portal)`
/// is 0xF, the copied set has no active planes. Ends with the operation
/// count of `source` and `RemoveEmptyGroups`.
///
/// The copied plane set lands at the same index it had in `source` (the
/// code subtracts two locals that are always zero).
pub fn bs_compound_frustum_build_for_portal(
    e: &mut Engine,
    this: Frustum,
    source: Frustum,
    portal: Ptr,
) {
    let op_count = e.get(source, BSCompoundFrustum::iFreeOp);
    bs_simple_array_function_op_set_size(e, ops_array(this), op_count as u32, true);
    let source_planes = e.get(source, BSCompoundFrustum::iFreePlane);
    bs_simple_array_ni_frustum_planes_set_size(e, planes_array(this), source_planes as u32, true);
    let mut index = 0;
    while index < op_count {
        let op = op_code(e, source, index);
        if (OP_PORTAL..=OP_OCCLUSION).contains(&op) {
            let source_set = op_code(e, source, index + 1);
            let from = plane_address(e, source, source_set);
            let inside = e
                .call(OCCLUSION_WITHIN_FRUSTUM_FULL_TEST, &args![portal, from])
                .bool();
            set_op_code(e, this, index, op);
            let free_op = e.get(this, BSCompoundFrustum::iFreeOp);
            e.set(this, BSCompoundFrustum::iFreeOp, free_op.wrapping_add(1));
            let free_plane = e.get(this, BSCompoundFrustum::iFreePlane);
            set_op_code(e, this, index + 1, free_plane);
            let free_op = e.get(this, BSCompoundFrustum::iFreeOp);
            e.set(this, BSCompoundFrustum::iFreeOp, free_op.wrapping_add(1));
            let from = plane_address(e, source, source_set);
            let to = plane_address(e, this, source_set);
            copy_frustum_planes(e, from, to);
            e.set(
                this,
                BSCompoundFrustum::iFreePlane,
                free_plane.wrapping_add(1),
            );
            if inside && e.call(OCCLUSION_FLAG_BITS, &args![portal]).u32() == 0xf {
                let to = plane_address(e, this, source_set);
                e.mem.set_u32(to + 0x60, 0);
            }
            index += 1;
        } else {
            set_op_code(e, this, index, op);
            let free_op = e.get(this, BSCompoundFrustum::iFreeOp);
            e.set(this, BSCompoundFrustum::iFreeOp, free_op.wrapping_add(1));
        }
        index += 1;
    }
    e.set(this, BSCompoundFrustum::iFreeOp, op_count);
    bs_compound_frustum_remove_empty_groups(e, this);
}

// Translated from 00c48fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Counts the test operations (`OP_PORTAL` and `OP_OCCLUSION`) in the
/// operation list, skipping the plane set index entry after each.
pub fn fn_00c48fe0(e: &mut Engine, this: Frustum) -> i32 {
    let mut count = 0;
    let mut index = 0;
    while index < e.get(this, BSCompoundFrustum::iFreeOp) {
        let op = op_code(e, this, index);
        if op == OP_PORTAL || op == OP_OCCLUSION {
            count += 1;
            index += 1;
        }
        index += 1;
    }
    count
}

// Translated from 00c49050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCompoundFrustum::GetActivePlaneState` (Xbox PDB): returns a block of
/// the thread's scrap heap (one word per plane set slot of the array's
/// capacity) holding the view frustum's active-plane mask first, then each
/// used plane set's mask. Words past the used sets are left as the heap
/// gave them. `SetActivePlaneState` gives the block back.
pub fn bs_compound_frustum_get_active_plane_state(e: &mut Engine, this: Frustum) -> u32 {
    let heap = thread_scrap_heap(e);
    let capacity = e.get(planes_array(this), BSSimpleArray::iReservedSize);
    let state = scrap_allocate(e, heap, capacity << 2);
    let view_mask = e.get(view_frustum(this), NiFrustumPlanes::m_uiActivePlanes);
    e.mem.set_u32(state, view_mask);
    let mut index = 1;
    while index < e.get(this, BSCompoundFrustum::iFreePlane).wrapping_add(1)
        && index < e.get(planes_array(this), BSSimpleArray::iReservedSize) as i32
    {
        let mask = e.mem.u32(plane_address(e, this, index - 1) + 0x60);
        set_scrap_word(e, state, index, mask);
        index += 1;
    }
    state
}

// Translated from 00c49100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCompoundFrustum::SetActivePlaneState` (Xbox PDB): writes the masks in
/// `state` (from `GetActivePlaneState`) back and gives the block back to the
/// thread's scrap heap.
pub fn bs_compound_frustum_set_active_plane_state(e: &mut Engine, this: Frustum, state: u32) {
    let view_mask = e.mem.u32(state);
    e.set(
        view_frustum(this),
        NiFrustumPlanes::m_uiActivePlanes,
        view_mask,
    );
    let mut index = 1;
    while index < e.get(this, BSCompoundFrustum::iFreePlane).wrapping_add(1)
        && index < e.get(planes_array(this), BSSimpleArray::iReservedSize) as i32
    {
        let mask = scrap_word(e, state, index);
        let at = plane_address(e, this, index - 1) + 0x60;
        e.mem.set_u32(at, mask);
        index += 1;
    }
    let heap = thread_scrap_heap(e);
    e.call(SCRAP_HEAP_DEALLOCATE, &args![heap, state]);
}

// Translated from 00c491a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCompoundFrustum::Process` (Xbox PDB): tests the bound of `object`
/// (the `NiBound` its pointer at +0x20 points to; false when there is none
/// or its radius is 0) against the view frustum (unless `bSkipViewFrustum`)
/// and then walks the operation list with the bound: `OP_PORTAL` tests with
/// `00c4a6e0`, `OP_OCCLUSION` with `00c4a790`. True when the walk ends in an
/// end-of-list pass.
///
/// (The code falls back to a default bound at `011f4288` for a null
/// pointer, which cannot happen after the check above; left out.)
pub fn bs_compound_frustum_process(e: &mut Engine, this: Frustum, object: Ptr) -> bool {
    let bound_pointer = e.mem.u32(object.addr() + 0x20);
    let has_bound =
        bound_pointer != 0 && e.mem.f32(bound_pointer + 0xc) as f64 != e.global::<f64>(ZERO);
    if !has_bound {
        return false;
    }
    let bound_bytes = e.mem.bytes(bound_pointer, 16);
    e.with_stack(16, |e, bound| {
        e.mem.write(bound.addr(), &bound_bytes);
        let mut result = true;
        if !e.get(this, BSCompoundFrustum::bSkipViewFrustum) {
            result = fn_00c4a6e0(e, this, bound, view_frustum(this).cast());
        }
        if result && e.get(this, BSCompoundFrustum::iFreeOp) > 0 {
            result = walk_ops(e, this, &mut |e, op, planes| {
                if op == OP_PORTAL {
                    fn_00c4a6e0(e, this, bound, Ptr::new(planes))
                } else {
                    fn_00c4a790(e, this, bound, Ptr::new(planes))
                }
            });
        }
        result
    })
}

// Translated from 00c493a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `Process`, for a `target` object whose pointer at +0xC is an object
/// with two virtual tests taking a plane set (slot +0x9C for the view
/// frustum and for `OP_PORTAL`; slot +0xA0, inverted, for `OP_OCCLUSION`).
/// True when the walk ends in an end-of-list pass.
pub fn fn_00c493a0(e: &mut Engine, this: Frustum, target: Ptr) -> bool {
    let tester = e.mem.u32(target.addr() + 0xc);
    let mut result = true;
    if !e.get(this, BSCompoundFrustum::bSkipViewFrustum) {
        result = e.vcall(tester, 0x9c, &args![view_frustum(this)]).bool();
    }
    if result && e.get(this, BSCompoundFrustum::iFreeOp) > 0 {
        result = walk_ops(e, this, &mut |e, op, planes| {
            let tester = e.mem.u32(target.addr() + 0xc);
            if op == OP_PORTAL {
                e.vcall(tester, 0x9c, &args![planes]).bool()
            } else {
                !e.vcall(tester, 0xa0, &args![planes]).bool()
            }
        });
    }
    result
}

// Translated from 00c49560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `Process`, testing an occlusion `plane` instead of a bound: against
/// the view frustum with `00c344c0(plane; view frustum, view position)`,
/// `OP_PORTAL` with `WithinFrustum(planes, 0, camera position globals)` and
/// `OP_OCCLUSION` with the inverse of `00c347d0`. True when the walk ends in
/// an end-of-list pass.
pub fn fn_00c49560(e: &mut Engine, this: Frustum, plane: Ptr) -> bool {
    let mut result = true;
    if !e.get(this, BSCompoundFrustum::bSkipViewFrustum) {
        result = e
            .call(
                OCCLUSION_TEST_AGAINST_VIEW,
                &args![plane, view_frustum(this), view_position(this)],
            )
            .bool();
    }
    if result && e.get(this, BSCompoundFrustum::iFreeOp) > 0 {
        result = walk_ops(e, this, &mut |e, op, planes| {
            if op == OP_PORTAL {
                let x: u32 = e.global(VIEW_POSITION_GLOBAL);
                let y: u32 = e.global(VIEW_POSITION_GLOBAL + 4);
                let z: u32 = e.global(VIEW_POSITION_GLOBAL + 8);
                e.call(
                    OCCLUSION_WITHIN_FRUSTUM,
                    &args![plane, planes, 0u32, x, y, z],
                )
                .bool()
            } else {
                !e.call(OCCLUSION_TEST_PLANES, &args![plane, planes]).bool()
            }
        });
    }
    result
}

// Translated from 00c49700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCompoundFrustum::RemoveEmptyGroups` (Xbox PDB): compacts the operation
/// list, dropping every group-open operation (`OP_GROUP_INTERSECTION` or
/// `OP_GROUP_UNION`) that is immediately followed by `OP_GROUP_END`
/// (repeatedly, so nested empty groups go too). Each kept test operation
/// takes its plane set index entry along.
pub fn bs_compound_frustum_remove_empty_groups(e: &mut Engine, this: Frustum) {
    let mut removed = 0;
    let original_count = e.get(this, BSCompoundFrustum::iFreeOp);
    let mut index = 0;
    while index < e.get(this, BSCompoundFrustum::iFreeOp) {
        while index < e.get(this, BSCompoundFrustum::iFreeOp)
            && e.get(this, BSCompoundFrustum::iFreeOp) > 0
            && (op_code(e, this, index + removed) == OP_GROUP_INTERSECTION
                || op_code(e, this, index + removed) == OP_GROUP_UNION)
            && op_code(e, this, index + 1 + removed) == OP_GROUP_END
        {
            removed += 2;
            let free_op = e.get(this, BSCompoundFrustum::iFreeOp);
            e.set(this, BSCompoundFrustum::iFreeOp, free_op - 2);
        }
        if index + removed < original_count {
            let value = op_code(e, this, index + removed);
            set_op_code(e, this, index, value);
            let op = op_code(e, this, index);
            if op == OP_PORTAL || op == OP_OCCLUSION {
                let value = op_code(e, this, index + 1 + removed);
                set_op_code(e, this, index + 1, value);
                index += 1;
            }
        }
        index += 1;
    }
}

// Translated from 00c49850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCompoundFrustum::PrethreadOpList` (Xbox PDB): fills in the `jp` and
/// `jf` jump targets of the operation list so the walkers can follow them.
/// Does nothing but set `bPrethreaded` when it already is or the list is
/// empty.
///
/// Three operation words are appended past `iFreeOp` (`OP_END_OF_LIST`,
/// pass, fail; `iFreeOp` is not advanced, so they are the targets the
/// jumps use). Three scrap-heap word arrays of `iFreeOp / 2` entries track
/// the groups: the kind of each open group, the end of each closed group,
/// and a running group id. Pass one finds where every group ends and stops
/// (with `iFreeOp` set to 0 and `bPrethreaded` still clear) if an
/// `OP_END_OF_LIST` shows up inside a group; pass two writes the targets,
/// remembering the first test operation as `iFirstOp`; with no test
/// operation at all it reports through the debug message routine and
/// stores `iFirstOp = iFreeOp + 1`; pass three shortens every test
/// operation's jumps over operations that are not tests or ends. Those two
/// early exits leave the scrap arrays allocated, as the game does.
pub fn bs_compound_frustum_prethread_op_list(e: &mut Engine, this: Frustum) {
    if e.get(this, BSCompoundFrustum::bPrethreaded) || e.get(this, BSCompoundFrustum::iFreeOp) <= 0
    {
        e.set(this, BSCompoundFrustum::bPrethreaded, true);
        return;
    }
    let end_pass_index = e.get(this, BSCompoundFrustum::iFreeOp).wrapping_add(1);
    let end_fail_index = e.get(this, BSCompoundFrustum::iFreeOp).wrapping_add(2);
    ensure_op_room(e, this, 3, 10);
    let free_op = e.get(this, BSCompoundFrustum::iFreeOp);
    set_op_code(e, this, free_op, OP_END_OF_LIST);
    set_op_code(e, this, free_op + 1, OP_END_PASS);
    set_op_code(e, this, free_op + 2, OP_END_FAIL);

    let slots = free_op / 2;
    let heap = thread_scrap_heap(e);
    let kinds = scrap_allocate(e, heap, (slots as u32) << 2);
    e.mem.set_u32(kinds, 0);
    let mut depth: i32 = 0;
    let mut next_group = 1u32;
    let group_ends = scrap_allocate(e, heap, (slots as u32) << 2);
    let group_ids = scrap_allocate(e, heap, (slots as u32) << 2);

    // Pass one: where every group ends.
    let mut index = 0;
    while index < e.get(this, BSCompoundFrustum::iFreeOp).wrapping_add(1) {
        match op_code(e, this, index) {
            OP_END_OF_LIST => {
                e.mem.set_u32(group_ends, index as u32);
                if depth != 0 {
                    e.set(this, BSCompoundFrustum::iFreeOp, 0);
                    return;
                }
            }
            OP_GROUP_INTERSECTION | OP_GROUP_UNION => {
                depth += 1;
                set_scrap_word(e, kinds, depth, next_group);
                next_group += 1;
            }
            OP_GROUP_END => {
                let group = scrap_word(e, kinds, depth) as i32;
                set_scrap_word(e, group_ends, group, index as u32);
                depth -= 1;
            }
            OP_PORTAL | OP_OCCLUSION => index += 1,
            _ => {}
        }
        index += 1;
    }

    // Pass two: the jump targets.
    e.call(MEMSET, &args![kinds, 0u32, (slots as u32) << 2]);
    e.mem.set_u32(kinds, 0);
    e.mem.set_u32(group_ids, 0);
    depth = 1;
    next_group = 1;
    let mut found_test = false;
    let mut index = 0;
    while index < e.get(this, BSCompoundFrustum::iFreeOp).wrapping_add(1) {
        match op_code(e, this, index) {
            OP_END_OF_LIST => {
                set_op_jump_pass(e, this, index, end_pass_index);
                set_op_jump_fail(e, this, index, end_fail_index);
            }
            OP_GROUP_INTERSECTION | OP_GROUP_UNION => {
                let op = op_code(e, this, index);
                set_scrap_word(e, kinds, depth, op as u32);
                set_scrap_word(e, group_ids, depth, next_group);
                next_group += 1;
                set_op_jump_pass(e, this, index, index + 1);
                set_op_jump_fail(e, this, index, index + 1);
                depth += 1;
            }
            OP_GROUP_END => {
                depth -= 1;
                let kind = scrap_word(e, kinds, depth - 1) as i32;
                let group = scrap_word(e, group_ids, depth - 1) as i32;
                let group_end = scrap_word(e, group_ends, group) as i32;
                if kind == 0 {
                    set_op_jump_pass(e, this, index, index + 1);
                    set_op_jump_fail(e, this, index, index + 1);
                } else if kind == OP_GROUP_INTERSECTION {
                    set_op_jump_pass(e, this, index, index + 1);
                    set_op_jump_fail(e, this, index, group_end);
                } else if kind == OP_GROUP_UNION {
                    set_op_jump_pass(e, this, index, group_end);
                    set_op_jump_fail(e, this, index, index + 1);
                }
            }
            OP_PORTAL | OP_OCCLUSION => {
                if !found_test {
                    e.set(this, BSCompoundFrustum::iFirstOp, index);
                    found_test = true;
                }
                let kind = scrap_word(e, kinds, depth - 1) as i32;
                let group = scrap_word(e, group_ids, depth - 1) as i32;
                let group_end = scrap_word(e, group_ends, group) as i32;
                let free_op = e.get(this, BSCompoundFrustum::iFreeOp);
                if kind == 0 {
                    set_op_jump_pass(e, this, index, free_op);
                    set_op_jump_fail(e, this, index, free_op);
                } else if kind == OP_GROUP_INTERSECTION {
                    set_op_jump_pass(e, this, index, index + 2);
                    set_op_jump_fail(e, this, index, group_end);
                } else if kind == OP_GROUP_UNION {
                    set_op_jump_pass(e, this, index, group_end);
                    set_op_jump_fail(e, this, index, index + 2);
                }
                index += 1;
            }
            _ => {}
        }
        index += 1;
    }

    if !found_test {
        e.call(DEBUG_MESSAGE, &args![EMPTY_OP_LIST_TEXT]);
        e.set(this, BSCompoundFrustum::iFirstOp, end_pass_index);
        e.set(this, BSCompoundFrustum::bPrethreaded, true);
        return;
    }

    // Pass three: shorten the test operations' jumps.
    let mut index = e.get(this, BSCompoundFrustum::iFirstOp);
    while index < e.get(this, BSCompoundFrustum::iFreeOp) {
        let op = op_code(e, this, index);
        if (OP_PORTAL..=OP_OCCLUSION).contains(&op) {
            loop {
                let target = op_jump_pass(e, this, index);
                let target_op = op_code(e, this, target);
                if matches!(
                    target_op,
                    OP_PORTAL | OP_OCCLUSION | OP_END_PASS | OP_END_FAIL
                ) {
                    break;
                }
                let next = op_jump_pass(e, this, target);
                set_op_jump_pass(e, this, index, next);
            }
            loop {
                let target = op_jump_fail(e, this, index);
                let target_op = op_code(e, this, target);
                if matches!(
                    target_op,
                    OP_PORTAL | OP_OCCLUSION | OP_END_PASS | OP_END_FAIL
                ) {
                    break;
                }
                let next = op_jump_fail(e, this, target);
                set_op_jump_fail(e, this, index, next);
            }
            index += 1;
        }
        index += 1;
    }
    e.call(SCRAP_HEAP_DEALLOCATE, &args![heap, kinds]);
    e.call(SCRAP_HEAP_DEALLOCATE, &args![heap, group_ends]);
    e.call(SCRAP_HEAP_DEALLOCATE, &args![heap, group_ids]);
    e.set(this, BSCompoundFrustum::bPrethreaded, true);
}

// Translated from 00c49f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Checks that the groups of the operation list nest properly (never more
/// closes than opens, and balanced at the end), calling `DebugBreak` for
/// each violation.
pub fn fn_00c49f10(e: &mut Engine, this: Frustum) {
    let mut depth = 0;
    let mut index = 0;
    while index < e.get(this, BSCompoundFrustum::iFreeOp) {
        let op = op_code(e, this, index);
        if op == OP_GROUP_INTERSECTION || op == OP_GROUP_UNION {
            depth += 1;
        } else if op == OP_GROUP_END {
            depth -= 1;
        } else if op == OP_PORTAL || op == OP_OCCLUSION {
            index += 1;
        }
        if depth < 0 {
            e.call(DEBUG_BREAK, &args![]);
        }
        index += 1;
    }
    if depth != 0 {
        e.call(DEBUG_BREAK, &args![]);
    }
}

// ---------------------------------------------------------------------------
// BSSimpleArray<NiFrustumPlanes, 1024> and BSSimpleArray<FunctionOp, 1024>

// Translated from 00c49fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor body of `BSSimpleArray<NiFrustumPlanes, 1024>`: installs the
/// vtable and empties the array. Returns `this`.
pub fn fn_00c49fe0(e: &mut Engine, this: Array) -> Array {
    e.mem.set_u32(this.addr(), PLANES_ARRAY_VTABLE);
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    this
}

// Translated from 00c4a070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiFrustumPlanes, 1024>::SetSize` (Xbox PDB): sets the
/// number of constructed elements. Zero clears (`shrink` frees the buffer);
/// more than the capacity allocates or reallocates to exactly `count` and
/// constructs the new elements; fewer than now shrinks (and, with `shrink`,
/// reallocates when the count is at most a quarter of the capacity); in
/// between constructs the new elements.
pub fn bs_simple_array_ni_frustum_planes_set_size(
    e: &mut Engine,
    this: Array,
    count: u32,
    shrink: bool,
) {
    if count == 0 {
        e.call(ARRAY_CLEAR, &args![this, shrink]);
        return;
    }
    let reserved = e.get(this, BSSimpleArray::iReservedSize);
    let size = e.get(this, BSSimpleArray::iSize);
    if count > reserved {
        if reserved == 0 {
            let buffer = e
                .vcall(this.addr(), ARRAY_SLOT_ALLOCATE, &args![count])
                .u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
            e.set(this, BSSimpleArray::iReservedSize, count);
        } else {
            bs_simple_array_ni_frustum_planes_reallocate_buffer(e, this, count, size);
            e.set(this, BSSimpleArray::iReservedSize, count);
        }
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        bs_simple_array_ni_frustum_planes_construct_items(
            e,
            this,
            buffer.wrapping_add(size.wrapping_mul(FRUSTUM_PLANES_SIZE)),
            count.wrapping_sub(size),
        );
        e.set(this, BSSimpleArray::iSize, count);
    } else if count < size {
        // The destruction loop over the dropped elements is empty.
        e.set(this, BSSimpleArray::iSize, count);
        if shrink && count <= reserved >> 2 {
            bs_simple_array_ni_frustum_planes_reallocate_buffer(e, this, count, count);
            e.set(this, BSSimpleArray::iReservedSize, count);
        }
    } else {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        bs_simple_array_ni_frustum_planes_construct_items(
            e,
            this,
            buffer.wrapping_add(size.wrapping_mul(FRUSTUM_PLANES_SIZE)),
            count.wrapping_sub(size),
        );
        e.set(this, BSSimpleArray::iSize, count);
    }
}

// Translated from 00c4a1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `BSSimpleArray<FunctionOp, 1024>`: installs the vtable and
/// initialises the array empty with `00c4a5a0(0, 0)`. Returns `this`.
pub fn fn_00c4a1b0(e: &mut Engine, this: Array) -> Array {
    e.mem.set_u32(this.addr(), OPS_ARRAY_VTABLE);
    fn_00c4a5a0(e, this, 0, 0);
    this
}

// Translated from 00c4a1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BSCompoundFrustum::FunctionOp, 1024>::SetSize` (Xbox PDB):
/// the same as the plane array's, with 12-byte elements that need no
/// construction.
pub fn bs_simple_array_function_op_set_size(e: &mut Engine, this: Array, count: u32, shrink: bool) {
    if count == 0 {
        e.call(ARRAY_CLEAR, &args![this, shrink]);
        return;
    }
    let reserved = e.get(this, BSSimpleArray::iReservedSize);
    let size = e.get(this, BSSimpleArray::iSize);
    if count > reserved {
        if reserved == 0 {
            let buffer = e
                .vcall(this.addr(), ARRAY_SLOT_ALLOCATE, &args![count])
                .u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
            e.set(this, BSSimpleArray::iReservedSize, count);
        } else {
            fn_00c4a4e0(e, this, count, size);
            e.set(this, BSSimpleArray::iReservedSize, count);
        }
        e.set(this, BSSimpleArray::iSize, count);
    } else if count < size {
        e.set(this, BSSimpleArray::iSize, count);
        if shrink && count <= reserved >> 2 {
            fn_00c4a4e0(e, this, count, count);
            e.set(this, BSSimpleArray::iReservedSize, count);
        }
    } else {
        e.set(this, BSSimpleArray::iSize, count);
    }
}

// Translated from 00c4a360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiFrustumPlanes, 1024>::_ConstructItems` (Xbox PDB):
/// constructs `count` elements at `first` (skipping an address that wraps to
/// 0): six `NiPlane`s each, and all six planes active. `this` is not used.
/// The exception frame is not translated.
pub fn bs_simple_array_ni_frustum_planes_construct_items(
    e: &mut Engine,
    _unused_0: Array,
    first: u32,
    count: u32,
) {
    for index in 0..count {
        let item = index.wrapping_mul(FRUSTUM_PLANES_SIZE).wrapping_add(first);
        if item != 0 {
            let mut plane = item;
            for _ in 0..6 {
                e.call(NI_PLANE_CONSTRUCT, &args![plane]);
                plane = plane.wrapping_add(0x10);
            }
            e.mem.set_u32(item + 0x60, ALL_PLANES_ACTIVE);
        }
    }
}

// Translated from 00c4a420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiFrustumPlanes, 1024>::_ReallocateBuffer` (Xbox PDB):
/// gives the array a buffer for `new_count` elements. With no buffer it
/// allocates one (and records the capacity); when `old_count` equals the
/// capacity it reallocates in place through vtable slot +0xC; otherwise it
/// allocates a new block, moves `old_count` elements into it (`_MoveItems`)
/// and frees the old block.
pub fn bs_simple_array_ni_frustum_planes_reallocate_buffer(
    e: &mut Engine,
    this: Array,
    new_count: u32,
    old_count: u32,
) {
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    if buffer == 0 {
        let block = e
            .vcall(this.addr(), ARRAY_SLOT_ALLOCATE, &args![new_count])
            .u32();
        e.set(this, BSSimpleArray::pBuffer, block);
        e.set(this, BSSimpleArray::iReservedSize, new_count);
    } else if old_count == e.get(this, BSSimpleArray::iReservedSize) {
        let block = e
            .vcall(
                this.addr(),
                ARRAY_SLOT_REALLOCATE,
                &args![buffer, new_count],
            )
            .u32();
        e.set(this, BSSimpleArray::pBuffer, block);
    } else {
        let block = e
            .vcall(this.addr(), ARRAY_SLOT_ALLOCATE, &args![new_count])
            .u32();
        let old = e.get(this, BSSimpleArray::pBuffer);
        bs_simple_array_ni_frustum_planes_move_items(e, this, block, old, old_count);
        e.vcall(this.addr(), ARRAY_SLOT_FREE, &args![old]);
        e.set(this, BSSimpleArray::pBuffer, 0);
        e.set(this, BSSimpleArray::pBuffer, block);
    }
}

// Translated from 00c4a4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_ReallocateBuffer` of `BSSimpleArray<FunctionOp, 1024>` (no map name):
/// the plane array's, moving with `006dc6e0(this, new, old, count)`.
pub fn fn_00c4a4e0(e: &mut Engine, this: Array, new_count: u32, old_count: u32) {
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    if buffer == 0 {
        let block = e
            .vcall(this.addr(), ARRAY_SLOT_ALLOCATE, &args![new_count])
            .u32();
        e.set(this, BSSimpleArray::pBuffer, block);
        e.set(this, BSSimpleArray::iReservedSize, new_count);
    } else if old_count == e.get(this, BSSimpleArray::iReservedSize) {
        let block = e
            .vcall(
                this.addr(),
                ARRAY_SLOT_REALLOCATE,
                &args![buffer, new_count],
            )
            .u32();
        e.set(this, BSSimpleArray::pBuffer, block);
    } else {
        let block = e
            .vcall(this.addr(), ARRAY_SLOT_ALLOCATE, &args![new_count])
            .u32();
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.call(MOVE_FUNCTION_OPS, &args![this, block, old, old_count]);
        e.vcall(this.addr(), ARRAY_SLOT_FREE, &args![old]);
        e.set(this, BSSimpleArray::pBuffer, 0);
        e.set(this, BSSimpleArray::pBuffer, block);
    }
}

// Translated from 00c4a5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialiser of `BSSimpleArray<FunctionOp, 1024>` `(capacity, size)` (no
/// map name): empties the array, allocates room for the larger of the two
/// (through vtable slot +4) when it is not 0, and sets the used size to
/// `size`.
pub fn fn_00c4a5a0(e: &mut Engine, this: Array, capacity: u32, size: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let capacity = capacity.max(size);
    if capacity != 0 {
        let buffer = e
            .vcall(this.addr(), ARRAY_SLOT_ALLOCATE, &args![capacity])
            .u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    }
    if size != 0 {
        e.set(this, BSSimpleArray::iSize, size);
    }
}

// Translated from 00c4a640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiFrustumPlanes, 1024>::_MoveItems` (Xbox PDB): moves
/// `count` elements from `source` to `target` with `memmove`, front to back
/// when the target is below the source and back to front when above.
pub fn bs_simple_array_ni_frustum_planes_move_items(
    e: &mut Engine,
    _unused_0: Array,
    target: u32,
    source: u32,
    count: u32,
) {
    if count == 0 {
        return;
    }
    if target < source {
        for index in 0..count {
            let offset = index.wrapping_mul(FRUSTUM_PLANES_SIZE);
            e.call(
                MEMMOVE,
                &args![
                    offset.wrapping_add(target),
                    offset.wrapping_add(source),
                    FRUSTUM_PLANES_SIZE
                ],
            );
        }
    } else if target > source {
        let mut index = count.wrapping_sub(1) as i32;
        while index >= 0 {
            let offset = (index as u32).wrapping_mul(FRUSTUM_PLANES_SIZE);
            e.call(
                MEMMOVE,
                &args![
                    offset.wrapping_add(target),
                    offset.wrapping_add(source),
                    FRUSTUM_PLANES_SIZE
                ],
            );
            index -= 1;
        }
    }
}

// Translated from 00c4a6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests the `NiBound` `bound` against the active planes of a plane set
/// (`this` is not used): true when no active plane has the bound wholly
/// behind it (`NiBound::WhichSide` = 2). Planes that have it wholly in front
/// (= 1) are made inactive on the way. A set with no active plane passes.
pub fn fn_00c4a6e0(e: &mut Engine, _unused_0: Frustum, bound: Ptr, planes: Ptr) -> bool {
    let mask_at = planes.addr() + 0x60;
    if e.mem.u32(mask_at) == 0 {
        return true;
    }
    let mut plane = 0;
    while plane < 6 {
        if (1u32 << plane) & e.mem.u32(mask_at) != 0 {
            let side = e
                .call(
                    NI_BOUND_WHICH_SIDE,
                    &args![bound, planes.addr() + (plane << 4)],
                )
                .i32();
            if side == 2 {
                break;
            }
            if side == 1 {
                let mask = e.mem.u32(mask_at);
                e.mem.set_u32(mask_at, !(1u32 << plane) & mask);
            }
        }
        plane += 1;
    }
    plane == 6
}

// Translated from 00c4a790 (decompiled, FalloutNV.exe 1.4.0.525)
/// The inverse test of `00c4a6e0`: false for a set with no active plane;
/// otherwise walks the active planes, making inactive each one the bound is
/// wholly in front of (= 1), and stops at the first where it is not. True
/// when the walk stops early.
pub fn fn_00c4a790(e: &mut Engine, _unused_0: Frustum, bound: Ptr, planes: Ptr) -> bool {
    let mask_at = planes.addr() + 0x60;
    if e.mem.u32(mask_at) == 0 {
        return false;
    }
    let mut plane = 0;
    while plane < 6 {
        if (1u32 << plane) & e.mem.u32(mask_at) != 0 {
            let side = e
                .call(
                    NI_BOUND_WHICH_SIDE,
                    &args![bound, planes.addr() + (plane << 4)],
                )
                .i32();
            if side != 1 {
                break;
            }
            let mask = e.mem.u32(mask_at);
            e.mem.set_u32(mask_at, !(1u32 << plane) & mask);
        }
        plane += 1;
    }
    plane != 6
}

// Translated from 00c4a840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiFrustumPlanes, 1024>::~BSSimpleArray` (Xbox PDB):
/// installs the vtable and clears the array, freeing the buffer.
pub fn bs_simple_array_ni_frustum_planes_destructor(e: &mut Engine, this: Array) {
    e.mem.set_u32(this.addr(), PLANES_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, true]);
}

// Translated from 00c4a860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiFrustumPlanes, 1024>::_Allocate` (Xbox PDB): a block of
/// `count` plane sets from the memory manager.
pub fn bs_simple_array_ni_frustum_planes_allocate(
    e: &mut Engine,
    _unused_0: Array,
    count: u32,
) -> u32 {
    e.call(
        MEMORY_ALLOCATE,
        &args![MEMORY_MANAGER, count.wrapping_mul(FRUSTUM_PLANES_SIZE)],
    )
    .u32()
}

// Translated from 00c4a8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiFrustumPlanes, 1024>::_Reallocate` (Xbox PDB): resizes a
/// block to `count` plane sets through the memory manager and returns the
/// new block.
pub fn bs_simple_array_ni_frustum_planes_reallocate(
    e: &mut Engine,
    _unused_0: Array,
    block: u32,
    count: u32,
) -> u32 {
    e.call(
        MEMORY_REALLOCATE,
        &args![
            MEMORY_MANAGER,
            block,
            count.wrapping_mul(FRUSTUM_PLANES_SIZE)
        ],
    )
    .u32()
}

// Translated from 00c4a8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BSCompoundFrustum::FunctionOp, 1024>::~BSSimpleArray`
/// (Xbox PDB): installs the vtable and clears the array, freeing the buffer.
pub fn fn_00c4a8e0(e: &mut Engine, this: Array) {
    e.mem.set_u32(this.addr(), OPS_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, true]);
}

// Translated from 00c4a900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BSCompoundFrustum::FunctionOp, 1024>::_Allocate` (Xbox
/// PDB): a block of `count` operations from the memory manager.
pub fn bs_simple_array_function_op_allocate(e: &mut Engine, _unused_0: Array, count: u32) -> u32 {
    e.call(
        MEMORY_ALLOCATE,
        &args![MEMORY_MANAGER, count.wrapping_mul(FUNCTION_OP_SIZE)],
    )
    .u32()
}

// Translated from 00c4a930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BSCompoundFrustum::FunctionOp, 1024>::_Reallocate` (Xbox
/// PDB): resizes a block to `count` operations through the memory manager
/// and returns the new block.
pub fn bs_simple_array_function_op_reallocate(
    e: &mut Engine,
    _unused_0: Array,
    block: u32,
    count: u32,
) -> u32 {
    e.call(
        MEMORY_REALLOCATE,
        &args![MEMORY_MANAGER, block, count.wrapping_mul(FUNCTION_OP_SIZE)],
    )
    .u32()
}

// Translated from 00c4a960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiFrustumPlanes, 1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor and, when bit 0 of `flags` is set, gives
/// the object itself back to the memory manager. Returns `this`.
pub fn bs_simple_array_ni_frustum_planes_scalar_deleting_destructor(
    e: &mut Engine,
    this: Array,
    flags: u32,
) -> u32 {
    bs_simple_array_ni_frustum_planes_destructor(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this.addr()
}

// Translated from 00c4a990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BSCompoundFrustum::FunctionOp, 1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs the destructor and, when bit 0 of `flags` is set, gives
/// the object itself back to the memory manager. Returns `this`.
pub fn bs_simple_array_function_op_scalar_deleting_destructor(
    e: &mut Engine,
    this: Array,
    flags: u32,
) -> u32 {
    fn_00c4a8e0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this.addr()
}

// Translated from 00c4a9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<...>::Clear` (Xbox PDB, shared by both arrays): with a
/// buffer present, `free` gives the buffer back through the array's free
/// slot (+8) and zeroes the buffer and reserved size; the used size is
/// zeroed either way. The game's empty loop over the elements (no element
/// destructor) is left out.
pub fn bs_simple_array_clear(e: &mut Engine, this: Array, free: bool) {
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    if buffer == 0 {
        return;
    }
    if free {
        e.vcall(this.addr(), ARRAY_SLOT_FREE, &args![buffer]);
        e.set(this, BSSimpleArray::pBuffer, 0);
        e.set(this, BSSimpleArray::iReservedSize, 0);
    }
    e.set(this, BSSimpleArray::iSize, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c47660,
            bs_compound_frustum_bs_compound_frustum(
                Ptr<BSCompoundFrustum>,
            ) -> Ptr<BSCompoundFrustum>
        ),
        entry!(
            0x00c47770,
            bs_compound_frustum_destructor(Ptr<BSCompoundFrustum>)
        ),
        entry!(
            0x00c477f0,
            bs_compound_frustum_copy_members(Ptr<BSCompoundFrustum>, Ptr<BSCompoundFrustum>)
        ),
        entry!(
            0x00c47980,
            bs_compound_frustum_set_camera(Ptr<BSCompoundFrustum>, Ptr)
        ),
        entry!(0x00c47a00, fn_00c47a00(Ptr<BSCompoundFrustum>)),
        entry!(
            0x00c47a70,
            bs_compound_frustum_start_group_union(Ptr<BSCompoundFrustum>)
        ),
        entry!(
            0x00c47ae0,
            bs_compound_frustum_end_group(Ptr<BSCompoundFrustum>)
        ),
        entry!(0x00c47b50, fn_00c47b50(Ptr<BSCompoundFrustum>, Ptr, u32)),
        entry!(
            0x00c47d20,
            bs_compound_frustum_add_occlusion_plane(Ptr<BSCompoundFrustum>, Ptr)
        ),
        entry!(
            0x00c48800,
            bs_compound_frustum_add_compound_frustum(
                Ptr<BSCompoundFrustum>,
                Ptr<BSCompoundFrustum>,
            )
        ),
        entry!(0x00c48a10, fn_00c48a10(Ptr<BSCompoundFrustum>, i32, bool)),
        entry!(0x00c48b90, fn_00c48b90(Ptr<BSCompoundFrustum>)),
        entry!(0x00c48ca0, fn_00c48ca0(Ptr<BSCompoundFrustum>)),
        entry!(
            0x00c48db0,
            bs_compound_frustum_build_for_portal(
                Ptr<BSCompoundFrustum>,
                Ptr<BSCompoundFrustum>,
                Ptr,
            )
        ),
        entry!(0x00c48fe0, fn_00c48fe0(Ptr<BSCompoundFrustum>) -> i32),
        entry!(
            0x00c49050,
            bs_compound_frustum_get_active_plane_state(Ptr<BSCompoundFrustum>) -> u32
        ),
        entry!(
            0x00c49100,
            bs_compound_frustum_set_active_plane_state(Ptr<BSCompoundFrustum>, u32)
        ),
        entry!(
            0x00c491a0,
            bs_compound_frustum_process(Ptr<BSCompoundFrustum>, Ptr) -> bool
        ),
        entry!(0x00c493a0, fn_00c493a0(Ptr<BSCompoundFrustum>, Ptr) -> bool),
        entry!(0x00c49560, fn_00c49560(Ptr<BSCompoundFrustum>, Ptr) -> bool),
        entry!(
            0x00c49700,
            bs_compound_frustum_remove_empty_groups(Ptr<BSCompoundFrustum>)
        ),
        entry!(
            0x00c49850,
            bs_compound_frustum_prethread_op_list(Ptr<BSCompoundFrustum>)
        ),
        entry!(0x00c49f10, fn_00c49f10(Ptr<BSCompoundFrustum>)),
        entry!(
            0x00c49fe0,
            fn_00c49fe0(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x00c4a070,
            bs_simple_array_ni_frustum_planes_set_size(Ptr<BSSimpleArray>, u32, bool)
        ),
        entry!(
            0x00c4a1b0,
            fn_00c4a1b0(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x00c4a1e0,
            bs_simple_array_function_op_set_size(Ptr<BSSimpleArray>, u32, bool)
        ),
        entry!(
            0x00c4a360,
            bs_simple_array_ni_frustum_planes_construct_items(Ptr<BSSimpleArray>, u32, u32)
        ),
        entry!(
            0x00c4a420,
            bs_simple_array_ni_frustum_planes_reallocate_buffer(Ptr<BSSimpleArray>, u32, u32)
        ),
        entry!(0x00c4a4e0, fn_00c4a4e0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x00c4a5a0, fn_00c4a5a0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(
            0x00c4a640,
            bs_simple_array_ni_frustum_planes_move_items(Ptr<BSSimpleArray>, u32, u32, u32)
        ),
        entry!(
            0x00c4a6e0,
            fn_00c4a6e0(Ptr<BSCompoundFrustum>, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00c4a790,
            fn_00c4a790(Ptr<BSCompoundFrustum>, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00c4a840,
            bs_simple_array_ni_frustum_planes_destructor(Ptr<BSSimpleArray>)
        ),
        entry!(
            0x00c4a860,
            bs_simple_array_ni_frustum_planes_allocate(Ptr<BSSimpleArray>, u32) -> u32
        ),
        entry!(
            0x00c4a8b0,
            bs_simple_array_ni_frustum_planes_reallocate(Ptr<BSSimpleArray>, u32, u32) -> u32
        ),
        entry!(0x00c4a8e0, fn_00c4a8e0(Ptr<BSSimpleArray>)),
        entry!(
            0x00c4a900,
            bs_simple_array_function_op_allocate(Ptr<BSSimpleArray>, u32) -> u32
        ),
        entry!(
            0x00c4a930,
            bs_simple_array_function_op_reallocate(Ptr<BSSimpleArray>, u32, u32) -> u32
        ),
        entry!(
            0x00c4a960,
            bs_simple_array_ni_frustum_planes_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            ) -> u32
        ),
        entry!(
            0x00c4a990,
            bs_simple_array_function_op_scalar_deleting_destructor(Ptr<BSSimpleArray>, u32) -> u32
        ),
        entry!(0x00c4a9c0, bs_simple_array_clear(Ptr<BSSimpleArray>, bool)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fake scrap heap handle the doubles hand out.
    const FAKE_SCRAP_HEAP: u32 = 0x00ab_0000;

    fn noop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// A test engine: the exe data the code reads is mapped (the two array
    /// vtables, the alignment, the tolerance), and every callee outside this
    /// file has a double. The occlusion plane doubles read a fake plane
    /// object (see [`occlusion_plane`]).
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x0102_0000,
            0x010a_2000,
            0x010c_1000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(SCRAP_ALIGNMENT, 4u32);
        e.set_global(DISTANCE_TOLERANCE, 5.0f64);
        e.put_vtable(
            PLANES_ARRAY_VTABLE,
            &[0x00c4_a960, 0x00c4_a860, 0x00c4_a890, 0x00c4_a8b0],
        );
        e.put_vtable(
            OPS_ARRAY_VTABLE,
            &[0x00c4_a990, 0x00c4_a900, 0x00c4_a890, 0x00c4_a930],
        );
        // The array's free slot (`00c4a890`): gives the block back.
        e.register(0x00c4_a890, |e, a| {
            e.mem.free(a[1]);
            Ret::default()
        });
        // Reallocation `(manager, block, size)`: a new block with the old
        // contents.
        e.register(MEMORY_REALLOCATE, |e, a| {
            let new = e.mem.alloc(a[2]);
            let kept = e.mem.block_size(a[1]).unwrap_or(0).min(a[2]);
            let bytes = e.mem.bytes(a[1], kept);
            e.mem.write(new, &bytes);
            e.mem.free(a[1]);
            Ret {
                eax: new,
                ..Ret::default()
            }
        });
        // `Clear(free)`: the real one's effect.
        e.register(ARRAY_CLEAR, |e, a| {
            let array = Ptr::<BSSimpleArray>::new(a[0]);
            let buffer = e.get(array, BSSimpleArray::pBuffer);
            if buffer != 0 {
                if a[1] as u8 != 0 {
                    e.mem.free(buffer);
                    e.set(array, BSSimpleArray::pBuffer, 0);
                    e.set(array, BSSimpleArray::iReservedSize, 0);
                }
                e.set(array, BSSimpleArray::iSize, 0);
            }
            Ret::default()
        });
        e.register(NI_PLANE_CONSTRUCT, noop);
        e.register(NI_FRUSTUM_PLANES_CONSTRUCT, noop);
        e.register(NI_FRUSTUM_PLANES_SET, noop);
        e.register(DEBUG_MESSAGE, noop);
        e.register(DEBUG_BREAK, noop);
        // `NiPlane(v0, v1, v2)`: normal = v0, constant = v1.x; returns this.
        e.register(NI_PLANE_FROM_POINTS, |e, a| {
            for word in 0..3 {
                let value = e.mem.u32(a[1] + word * 4);
                e.mem.set_u32(a[0] + word * 4, value);
            }
            let constant = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 12, constant);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        // `NiBound::WhichSide(plane)`: the plane's normal x as an integer.
        e.register(NI_BOUND_WHICH_SIDE, |e, a| Ret {
            eax: e.mem.f32(a[1]) as i32 as u32,
            ..Ret::default()
        });
        // The signed side of a point: the plane's normal x as an integer.
        e.register(PLANE_SIDE_OF_POINT, |e, a| Ret {
            eax: e.mem.f32(a[0]) as i32 as u32,
            ..Ret::default()
        });
        // `NiPoint3::Dot`: the normal x of the plane.
        e.register(NI_POINT3_DOT, |e, a| Ret {
            st0: e.mem.f32(a[0]) as f64,
            ..Ret::default()
        });
        e.register(FABS, |_, a| Ret {
            st0: f64::from_bits(a[0] as u64 | (a[1] as u64) << 32).abs(),
            ..Ret::default()
        });
        // Occlusion planes: `GetFrustumPlanes` stamps the plane's address in
        // the set's first word and activates all six planes.
        e.register(OCCLUSION_GET_FRUSTUM_PLANES, |e, a| {
            e.mem.set_u32(a[4], a[0]);
            e.mem.set_u32(a[4] + 0x60, 0x3f);
            Ret::default()
        });
        // `GetVertices`: v0.x and v1.x come from the fake plane object.
        e.register(OCCLUSION_GET_VERTICES, |e, a| {
            let first = e.mem.u32(a[0] + 0x104);
            let second = e.mem.u32(a[0] + 0x108);
            e.mem.set_u32(a[1], first);
            e.mem.set_u32(a[1] + 12, second);
            Ret::default()
        });
        e.register(OCCLUSION_GET_JOINED_PLANE_TYPE, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x100),
            ..Ret::default()
        });
        // The scrap heap: a handle, blocks with slack for the originals'
        // out-of-range group indexes, and the free.
        e.register(GET_THREAD_SCRAP_HEAP, |_, _| Ret {
            eax: FAKE_SCRAP_HEAP,
            ..Ret::default()
        });
        e.register(SCRAP_HEAP_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1] + 64),
            ..Ret::default()
        });
        e.register(SCRAP_HEAP_DEALLOCATE, |e, a| {
            e.mem.free(a[1]);
            Ret::default()
        });
        // `006dc6e0(this, dst, src, count)`: 12-byte elements.
        e.register(MOVE_FUNCTION_OPS, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3] * 12);
            e.mem.write(a[1], &bytes);
            Ret::default()
        });
        e
    }

    /// A double that returns 1.
    fn returns_true(_: &mut Engine, _: &[u32]) -> Ret {
        Ret {
            eax: 1,
            ..Ret::default()
        }
    }

    /// The calls of the log made to `addr`, as their argument words.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .expect("call log")
            .iter()
            .filter(|(at, _)| *at == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A frustum built by the real constructor.
    fn new_frustum(e: &mut Engine) -> Frustum {
        let frustum = e.new_object::<BSCompoundFrustum>();
        e.call(0x00c4_7660, &args![frustum]);
        frustum
    }

    /// Writes `ops` (op, jp, jf) into the operation list and sets `iFreeOp`.
    fn load_ops(e: &mut Engine, frustum: Frustum, ops: &[[i32; 3]]) {
        let count = ops.len() as u32;
        if count + 4 > e.get(ops_array(frustum), BSSimpleArray::iReservedSize) {
            bs_simple_array_function_op_set_size(e, ops_array(frustum), count + 4, true);
        }
        for (index, op) in ops.iter().enumerate() {
            let at = op_address(e, frustum, index as i32);
            for (word, value) in op.iter().enumerate() {
                e.mem.set_i32(at + word as u32 * 4, *value);
            }
        }
        e.set(frustum, BSCompoundFrustum::iFreeOp, ops.len() as i32);
    }

    /// The operation words `[op, jp, jf]` at `index`.
    fn op_words(e: &Engine, frustum: Frustum, index: i32) -> [i32; 3] {
        [
            op_code(e, frustum, index),
            op_jump_pass(e, frustum, index),
            op_jump_fail(e, frustum, index),
        ]
    }

    /// The `op` words of the first `count` operations.
    fn op_codes(e: &Engine, frustum: Frustum, count: i32) -> Vec<i32> {
        (0..count).map(|index| op_code(e, frustum, index)).collect()
    }

    /// Makes room for `count` plane sets and sets `iFreePlane`.
    fn load_planes(e: &mut Engine, frustum: Frustum, count: i32) {
        if count as u32 + 4 > e.get(planes_array(frustum), BSSimpleArray::iReservedSize) {
            bs_simple_array_ni_frustum_planes_set_size(
                e,
                planes_array(frustum),
                count as u32 + 4,
                true,
            );
        }
        e.set(frustum, BSCompoundFrustum::iFreePlane, count);
    }

    fn plane_mask(e: &Engine, frustum: Frustum, index: i32) -> u32 {
        e.mem.u32(plane_address(e, frustum, index) + 0x60)
    }

    fn set_plane_mask(e: &mut Engine, frustum: Frustum, index: i32, mask: u32) {
        let at = plane_address(e, frustum, index) + 0x60;
        e.mem.set_u32(at, mask);
    }

    /// A fake occlusion plane object: joined plane type at +0x100, the
    /// vertex values `a` and `b` the doubles use at +0x104 and +0x108.
    fn occlusion_plane(e: &mut Engine, kind: u32, a: f32, b: f32) -> u32 {
        let plane = e.mem.alloc(0x110);
        e.mem.set_u32(plane + 0x100, kind);
        e.mem.set_f32(plane + 0x104, a);
        e.mem.set_f32(plane + 0x108, b);
        plane
    }

    // --- the class ---------------------------------------------------------

    #[test]
    fn constructor_builds_empty_frustum_with_room() {
        let mut e = engine();
        e.set_global(VIEW_POSITION_GLOBAL, 1.5f32);
        e.set_global(VIEW_POSITION_GLOBAL + 4, 2.5f32);
        e.set_global(VIEW_POSITION_GLOBAL + 8, -3.5f32);
        e.call_log = Some(vec![]);
        let object = e.new_object::<BSCompoundFrustum>();
        let result = e
            .call(0x00c4_7660, &args![object])
            .ptr::<BSCompoundFrustum>();
        assert_eq!(result, object);
        assert_eq!(e.mem.u32(planes_array(object).addr()), PLANES_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(ops_array(object).addr()), OPS_ARRAY_VTABLE);
        assert_eq!(e.get(planes_array(object), BSSimpleArray::iReservedSize), 4);
        assert_eq!(e.get(planes_array(object), BSSimpleArray::iSize), 4);
        assert_eq!(e.get(ops_array(object), BSSimpleArray::iReservedSize), 10);
        assert_eq!(e.get(ops_array(object), BSSimpleArray::iSize), 10);
        assert_eq!(
            e.get(view_frustum(object), NiFrustumPlanes::m_uiActivePlanes),
            0x3f
        );
        for set in 0..4 {
            assert_eq!(plane_mask(&e, object, set), 0x3f);
        }
        assert_eq!(e.get(object, BSCompoundFrustum::iFreePlane), 0);
        assert_eq!(e.get(object, BSCompoundFrustum::iFreeOp), 0);
        assert_eq!(e.get(object, BSCompoundFrustum::iFirstOp), 0);
        assert!(!e.get(object, BSCompoundFrustum::bPrethreaded));
        assert!(!e.get(object, BSCompoundFrustum::bSkipViewFrustum));
        assert_eq!(e.mem.f32(view_position(object).addr()), 1.5);
        assert_eq!(e.mem.f32(view_position(object).addr() + 4), 2.5);
        assert_eq!(e.mem.f32(view_position(object).addr() + 8), -3.5);
        // Six planes of the view frustum, six in each of the four sets.
        assert_eq!(calls(&e, NI_PLANE_CONSTRUCT).len(), 6 + 4 * 6);
    }

    #[test]
    fn destructor_clears_both_arrays_then_runs_their_destructors() {
        let mut e = engine();
        let object = new_frustum(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c4_7770, &args![object]);
        let planes = planes_array(object).addr();
        let ops = ops_array(object).addr();
        assert_eq!(
            calls(&e, ARRAY_CLEAR),
            vec![vec![planes, 1], vec![ops, 1], vec![ops, 1], vec![planes, 1]]
        );
        assert_eq!(e.get(planes_array(object), BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(ops_array(object), BSSimpleArray::pBuffer), 0);
    }

    #[test]
    fn copy_members_copies_counters_and_every_capacity_slot() {
        let mut e = engine();
        let source = new_frustum(&mut e);
        let target = new_frustum(&mut e);
        // Capacity 12 operations and 6 plane sets, so the target has to grow.
        load_ops(&mut e, source, &[[5, 1, 2], [8, 3, 4], [0, 5, 6]]);
        bs_simple_array_function_op_set_size(&mut e, ops_array(source), 12, true);
        load_planes(&mut e, source, 2);
        bs_simple_array_ni_frustum_planes_set_size(&mut e, planes_array(source), 6, true);
        let marker = plane_address(&e, source, 5);
        e.mem.set_u32(marker, 0xfeed);
        e.set(source, BSCompoundFrustum::iFirstOp, 1);
        e.set(source, BSCompoundFrustum::bPrethreaded, true);
        e.mem.set_f32(view_position(source).addr(), 9.0);
        e.set(
            view_frustum(source),
            NiFrustumPlanes::m_uiActivePlanes,
            0x15,
        );
        e.call(0x00c4_77f0, &args![source, target]);
        assert_eq!(e.get(target, BSCompoundFrustum::iFreeOp), 3);
        assert_eq!(e.get(target, BSCompoundFrustum::iFreePlane), 2);
        assert_eq!(e.get(target, BSCompoundFrustum::iFirstOp), 1);
        assert!(e.get(target, BSCompoundFrustum::bPrethreaded));
        assert_eq!(e.mem.f32(view_position(target).addr()), 9.0);
        assert_eq!(
            e.get(view_frustum(target), NiFrustumPlanes::m_uiActivePlanes),
            0x15
        );
        assert_eq!(e.get(ops_array(target), BSSimpleArray::iReservedSize), 12);
        assert_eq!(op_words(&e, target, 1), [8, 3, 4]);
        assert_eq!(op_words(&e, target, 2), [0, 5, 6]);
        assert_eq!(e.get(planes_array(target), BSSimpleArray::iReservedSize), 6);
        assert_eq!(e.mem.u32(plane_address(&e, target, 5)), 0xfeed);
    }

    #[test]
    fn set_camera_takes_the_camera_and_empties_the_lists() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        load_ops(&mut e, frustum, &[[8, 0, 0], [0, 0, 0]]);
        e.set(frustum, BSCompoundFrustum::iFreePlane, 1);
        e.set(frustum, BSCompoundFrustum::iFirstOp, 1);
        e.set(frustum, BSCompoundFrustum::bPrethreaded, true);
        e.set(frustum, BSCompoundFrustum::bSkipViewFrustum, true);
        let camera = e.mem.alloc(0x100);
        e.mem.set_f32(camera + 0x8c, 4.0);
        e.mem.set_f32(camera + 0x90, 5.0);
        e.mem.set_f32(camera + 0x94, 6.0);
        e.call_log = Some(vec![]);
        e.call(0x00c4_7980, &args![frustum, camera]);
        assert_eq!(
            calls(&e, NI_FRUSTUM_PLANES_SET),
            vec![vec![frustum.addr() + 0x30, camera + 0xdc, camera + 0x68]]
        );
        assert_eq!(e.mem.f32(view_position(frustum).addr() + 4), 5.0);
        assert_eq!(e.mem.f32(view_position(frustum).addr() + 8), 6.0);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 0);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreePlane), 0);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFirstOp), 0);
        assert!(!e.get(frustum, BSCompoundFrustum::bPrethreaded));
        assert!(!e.get(frustum, BSCompoundFrustum::bSkipViewFrustum));

        // Without a camera only the lists and flags are reset.
        e.set(frustum, BSCompoundFrustum::iFreeOp, 3);
        e.call_log = Some(vec![]);
        e.call(0x00c4_7980, &args![frustum, 0u32]);
        assert!(calls(&e, NI_FRUSTUM_PLANES_SET).is_empty());
        assert_eq!(e.mem.f32(view_position(frustum).addr() + 4), 5.0);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 0);
    }

    #[test]
    fn group_intersection_appends_op_four_and_keeps_the_flag() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        e.set(frustum, BSCompoundFrustum::bPrethreaded, true);
        e.call(0x00c4_7a00, &args![frustum]);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 1);
        assert_eq!(op_code(&e, frustum, 0), OP_GROUP_INTERSECTION);
        assert!(e.get(frustum, BSCompoundFrustum::bPrethreaded));

        // With the array full (9 used of 10) it grows by 10 first.
        e.set(frustum, BSCompoundFrustum::iFreeOp, 9);
        e.call(0x00c4_7a00, &args![frustum]);
        assert_eq!(e.get(ops_array(frustum), BSSimpleArray::iReservedSize), 20);
        assert_eq!(op_code(&e, frustum, 9), OP_GROUP_INTERSECTION);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 10);
    }

    #[test]
    fn start_group_union_appends_op_five_and_clears_the_flag() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        e.set(frustum, BSCompoundFrustum::bPrethreaded, true);
        e.call(0x00c4_7a70, &args![frustum]);
        assert_eq!(op_code(&e, frustum, 0), OP_GROUP_UNION);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 1);
        assert!(!e.get(frustum, BSCompoundFrustum::bPrethreaded));
    }

    #[test]
    fn end_group_appends_op_six_and_clears_the_flag() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        e.set(frustum, BSCompoundFrustum::bPrethreaded, true);
        e.call(0x00c4_7ae0, &args![frustum]);
        assert_eq!(op_code(&e, frustum, 0), OP_GROUP_END);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 1);
        assert!(!e.get(frustum, BSCompoundFrustum::bPrethreaded));
    }

    #[test]
    fn portal_op_keeps_planes_active_only_when_far_from_the_view() {
        for (a, b, expected_mask) in [(20.0f32, 10.0f32, 0x3fu32), (12.0, 10.0, 0)] {
            let mut e = engine();
            let frustum = new_frustum(&mut e);
            let plane = occlusion_plane(&mut e, 0, a, b);
            e.call_log = Some(vec![]);
            e.call(0x00c4_7b50, &args![frustum, plane, 0xdeadu32]);
            assert_eq!(op_code(&e, frustum, 0), OP_PORTAL);
            assert_eq!(op_code(&e, frustum, 1), 0);
            assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 2);
            assert_eq!(e.get(frustum, BSCompoundFrustum::iFreePlane), 1);
            assert_eq!(e.mem.u32(plane_address(&e, frustum, 0)), plane);
            assert_eq!(plane_mask(&e, frustum, 0), expected_mask);
            let position = view_position_words(&e, frustum);
            assert_eq!(
                calls(&e, OCCLUSION_GET_FRUSTUM_PLANES),
                vec![vec![
                    plane,
                    position[0],
                    position[1],
                    position[2],
                    plane_address(&e, frustum, 0)
                ]]
            );
        }
    }

    #[test]
    fn occlusion_plane_without_a_joined_plane_adds_one_test() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        let plane = occlusion_plane(&mut e, 0, 1.0, 0.0);
        e.call_log = Some(vec![]);
        e.call(0x00c4_7d20, &args![frustum, plane]);
        assert_eq!(op_code(&e, frustum, 0), OP_OCCLUSION);
        assert_eq!(op_code(&e, frustum, 1), 0);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 2);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreePlane), 1);
        assert_eq!(e.mem.u32(plane_address(&e, frustum, 0)), plane);
        assert!(calls(&e, PLANE_SIDE_OF_POINT).is_empty());
    }

    #[test]
    fn occlusion_plane_of_type_zero_checks_the_sides_and_adds_one_test() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        // Type 0 takes the slot at +0xEC, or +0xF4 when that is empty.
        let plane = occlusion_plane(&mut e, 0, 1.0, 0.0);
        let joined = occlusion_plane(&mut e, 0, 1.0, 0.0);
        e.mem.set_u32(joined + 0x40, 1);
        e.mem.set_u32(plane + 0xf4, joined);
        e.call_log = Some(vec![]);
        e.call(0x00c4_7d20, &args![frustum, plane]);
        assert_eq!(calls(&e, PLANE_SIDE_OF_POINT).len(), 2);
        assert_eq!(calls(&e, NI_FRUSTUM_PLANES_CONSTRUCT).len(), 1);
        assert_eq!(op_code(&e, frustum, 0), OP_OCCLUSION);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 2);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreePlane), 1);
        assert_eq!(e.mem.u32(plane_address(&e, frustum, 0)), plane);
    }

    #[test]
    fn occlusion_plane_of_type_one_builds_the_group_of_four_sets() {
        // (+0xF0 holds the joined plane, or is empty so the one at +0xF8
        // is used; the planes made inactive differ).
        for (joined_in_f0, shifts) in [(false, (3u32, 4u32)), (true, (2, 3))] {
            let mut e = engine();
            let frustum = new_frustum(&mut e);
            let plane = occlusion_plane(&mut e, 1, 1.0, 0.0);
            let joined = occlusion_plane(&mut e, 0, 1.0, 0.0);
            e.mem.set_u32(joined + 0x40, 1);
            e.mem
                .set_u32(plane + if joined_in_f0 { 0xf0 } else { 0xf8 }, joined);
            e.call(0x00c4_7d20, &args![frustum, plane]);
            assert_eq!(
                op_codes(&e, frustum, 12),
                vec![4, 8, 0, 8, 1, 5, 8, 2, 8, 3, 6, 6]
            );
            assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 12);
            assert_eq!(e.get(frustum, BSCompoundFrustum::iFreePlane), 4);
            assert_eq!(e.mem.u32(plane_address(&e, frustum, 0)), plane);
            assert_eq!(e.mem.u32(plane_address(&e, frustum, 1)), joined);
            assert_eq!(e.mem.u32(plane_address(&e, frustum, 2)), plane);
            assert_eq!(e.mem.u32(plane_address(&e, frustum, 3)), joined);
            assert_eq!(plane_mask(&e, frustum, 0), 0x3f);
            assert_eq!(plane_mask(&e, frustum, 1), 0x3f);
            assert_eq!(plane_mask(&e, frustum, 2), 0x3f & !(1 << shifts.0));
            assert_eq!(plane_mask(&e, frustum, 3), 0x3f & !(1 << shifts.1));
        }
    }

    #[test]
    fn occlusion_plane_on_different_sides_falls_back_to_one_test() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        let plane = occlusion_plane(&mut e, 1, 1.0, 0.0);
        let joined = occlusion_plane(&mut e, 0, 2.0, 0.0);
        e.mem.set_u32(joined + 0x40, 1);
        e.mem.set_u32(plane + 0xf8, joined);
        e.call(0x00c4_7d20, &args![frustum, plane]);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 2);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreePlane), 1);
        assert_eq!(op_code(&e, frustum, 0), OP_OCCLUSION);
        assert_eq!(e.mem.u32(plane_address(&e, frustum, 0)), plane);
    }

    #[test]
    fn add_compound_frustum_appends_ops_and_planes_and_shifts_set_indexes() {
        let mut e = engine();
        let this = new_frustum(&mut e);
        let other = new_frustum(&mut e);
        load_ops(&mut e, this, &[[4, 0, 0], [8, 0, 0], [0, 0, 0], [6, 0, 0]]);
        load_planes(&mut e, this, 2);
        // A second frustum with 12 operations forces a grow.
        let other_ops: Vec<[i32; 3]> = [[8, 0, 0], [1, 0, 0], [7, 0, 0], [0, 0, 0]]
            .into_iter()
            .chain((0..8).map(|_| [6, 0, 0]))
            .collect();
        load_ops(&mut e, other, &other_ops);
        load_planes(&mut e, other, 2);
        let marker = plane_address(&e, other, 1);
        e.mem.set_u32(marker, 0xabc);
        e.set(this, BSCompoundFrustum::bPrethreaded, true);
        e.call(0x00c4_8800, &args![this, other]);
        assert!(!e.get(this, BSCompoundFrustum::bPrethreaded));
        assert_eq!(e.get(this, BSCompoundFrustum::iFreeOp), 16);
        assert_eq!(e.get(this, BSCompoundFrustum::iFreePlane), 4);
        assert_eq!(
            e.get(ops_array(this), BSSimpleArray::iReservedSize),
            10 + 13
        );
        // The plane set indexes after the appended test operations moved
        // up by this frustum's plane count (2).
        assert_eq!(op_codes(&e, this, 8), vec![4, 8, 0, 6, 8, 3, 7, 2]);
        assert_eq!(e.mem.u32(plane_address(&e, this, 3)), 0xabc);
    }

    #[test]
    fn insert_gap_shifts_ops_up_and_clears_all_but_the_first_entry() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        load_ops(
            &mut e,
            frustum,
            &[[5, 0, 0], [8, 0, 0], [0, 0, 0], [6, 0, 0]],
        );
        e.call(0x00c4_8a10, &args![frustum, 2i32, true]);
        assert_eq!(op_codes(&e, frustum, 6), vec![5, 0, 5, 8, 0, 6]);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 6);

        // Without the clear the gap keeps what was shifted over it.
        load_ops(
            &mut e,
            frustum,
            &[[5, 0, 0], [8, 0, 0], [0, 0, 0], [6, 0, 0]],
        );
        e.call(0x00c4_8a10, &args![frustum, 1i32, false]);
        assert_eq!(op_codes(&e, frustum, 5), vec![5, 5, 8, 0, 6]);

        // A gap past the capacity grows the array (by the count, over 10).
        load_ops(&mut e, frustum, &[[5, 0, 0]]);
        e.call(0x00c4_8a10, &args![frustum, 14i32, false]);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 15);
        assert_eq!(
            e.get(ops_array(frustum), BSSimpleArray::iReservedSize),
            10 + 14
        );
        assert_eq!(op_code(&e, frustum, 14), 5);
    }

    #[test]
    fn negative_gap_removes_leading_ops() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        load_ops(
            &mut e,
            frustum,
            &[
                [5, 0, 0],
                [8, 0, 0],
                [0, 0, 0],
                [6, 0, 0],
                [7, 0, 0],
                [1, 0, 0],
            ],
        );
        e.call(0x00c4_8a10, &args![frustum, -2i32, false]);
        assert_eq!(op_codes(&e, frustum, 4), vec![0, 6, 7, 1]);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 4);
        // Zero changes nothing; removing at least all of them empties the list.
        e.call(0x00c4_8a10, &args![frustum, 0i32, false]);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 4);
        e.call(0x00c4_8a10, &args![frustum, -9i32, false]);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 0);
    }

    #[test]
    fn wrap_in_union_group_unless_one_already_spans_the_list() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        load_ops(&mut e, frustum, &[[8, 0, 0], [0, 0, 0]]);
        e.call(0x00c4_8b90, &args![frustum]);
        assert_eq!(op_codes(&e, frustum, 3), vec![5, 8, 0]);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 3);

        // Already one union group over everything: only the last entry goes.
        load_ops(
            &mut e,
            frustum,
            &[[5, 0, 0], [8, 0, 0], [0, 0, 0], [6, 0, 0]],
        );
        e.call(0x00c4_8b90, &args![frustum]);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 3);
        assert_eq!(op_code(&e, frustum, 0), 5);

        // A union group that closes before the end is wrapped again.
        load_ops(
            &mut e,
            frustum,
            &[[5, 0, 0], [6, 0, 0], [8, 0, 0], [0, 0, 0]],
        );
        e.call(0x00c4_8b90, &args![frustum]);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 5);
        assert_eq!(op_code(&e, frustum, 1), 5);
    }

    #[test]
    fn wrap_in_intersection_group_unless_one_already_spans_the_list() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        load_ops(&mut e, frustum, &[[8, 0, 0], [0, 0, 0]]);
        e.call(0x00c4_8ca0, &args![frustum]);
        assert_eq!(op_codes(&e, frustum, 3), vec![4, 8, 0]);

        load_ops(
            &mut e,
            frustum,
            &[[4, 0, 0], [5, 0, 0], [6, 0, 0], [6, 0, 0]],
        );
        e.call(0x00c4_8ca0, &args![frustum]);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 3);

        // A union group is not an intersection group: wrapped.
        load_ops(
            &mut e,
            frustum,
            &[[5, 0, 0], [8, 0, 0], [0, 0, 0], [6, 0, 0]],
        );
        e.call(0x00c4_8ca0, &args![frustum]);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 5);
        assert_eq!(op_code(&e, frustum, 0), 4);
    }

    #[test]
    fn build_for_portal_copies_ops_and_sets_and_deactivates_covered_sets() {
        for (inside, expected_mask) in [(true, 0u32), (false, 0x3f)] {
            let mut e = engine();
            let this = new_frustum(&mut e);
            let source = new_frustum(&mut e);
            load_ops(
                &mut e,
                source,
                &[
                    [5, 0, 0],
                    [8, 0, 0],
                    [1, 0, 0],
                    [4, 0, 0],
                    [6, 0, 0],
                    [6, 0, 0],
                ],
            );
            load_planes(&mut e, source, 2);
            let at = plane_address(&e, source, 1);
            e.mem.set_u32(at, 0x5151);
            e.mem.set_u32(at + 0x60, 0x3f);
            if inside {
                e.register(OCCLUSION_WITHIN_FRUSTUM_FULL_TEST, returns_true);
            } else {
                e.register(OCCLUSION_WITHIN_FRUSTUM_FULL_TEST, noop);
            }
            e.register(OCCLUSION_FLAG_BITS, |_, _| Ret {
                eax: 0xf,
                ..Ret::default()
            });
            let portal = e.mem.alloc(0x110);
            e.call_log = Some(vec![]);
            e.call(0x00c4_8db0, &args![this, source, portal]);
            // The empty group (4, 6) is gone after `RemoveEmptyGroups`.
            assert_eq!(e.get(this, BSCompoundFrustum::iFreeOp), 4);
            assert_eq!(op_codes(&e, this, 4), vec![5, 8, 0, 6]);
            assert_eq!(e.get(this, BSCompoundFrustum::iFreePlane), 1);
            assert_eq!(e.mem.u32(plane_address(&e, this, 1)), 0x5151);
            assert_eq!(plane_mask(&e, this, 1), expected_mask);
            assert_eq!(
                calls(&e, OCCLUSION_WITHIN_FRUSTUM_FULL_TEST),
                vec![vec![portal, plane_address(&e, source, 1)]]
            );
        }
    }

    #[test]
    fn counts_test_operations() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        // The entry after a test is a plane set index and is skipped even
        // when it looks like a test operation.
        load_ops(
            &mut e,
            frustum,
            &[
                [5, 0, 0],
                [8, 0, 0],
                [7, 0, 0],
                [7, 0, 0],
                [8, 0, 0],
                [6, 0, 0],
            ],
        );
        assert_eq!(e.call(0x00c4_8fe0, &args![frustum]).i32(), 2);
    }

    #[test]
    fn active_plane_state_round_trips_through_a_scrap_heap_block() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        load_planes(&mut e, frustum, 2);
        set_plane_mask(&mut e, frustum, 0, 0x11);
        set_plane_mask(&mut e, frustum, 1, 0x22);
        e.set(
            view_frustum(frustum),
            NiFrustumPlanes::m_uiActivePlanes,
            0x33,
        );
        e.call_log = Some(vec![]);
        let state = e.call(0x00c4_9050, &args![frustum]).u32();
        // Capacity 6 sets (4 + 2): 24 bytes, the game's alignment 4.
        assert_eq!(
            calls(&e, SCRAP_HEAP_ALLOCATE),
            vec![vec![FAKE_SCRAP_HEAP, 24, 4]]
        );
        assert_eq!(calls(&e, GET_THREAD_SCRAP_HEAP), vec![vec![MEMORY_MANAGER]]);
        assert_eq!(e.mem.u32(state), 0x33);
        assert_eq!(e.mem.u32(state + 4), 0x11);
        assert_eq!(e.mem.u32(state + 8), 0x22);

        e.mem.set_u32(state, 0x44);
        e.mem.set_u32(state + 4, 0x55);
        e.mem.set_u32(state + 8, 0x66);
        e.call_log = Some(vec![]);
        e.call(0x00c4_9100, &args![frustum, state]);
        assert_eq!(
            e.get(view_frustum(frustum), NiFrustumPlanes::m_uiActivePlanes),
            0x44
        );
        assert_eq!(plane_mask(&e, frustum, 0), 0x55);
        assert_eq!(plane_mask(&e, frustum, 1), 0x66);
        assert_eq!(
            calls(&e, SCRAP_HEAP_DEALLOCATE),
            vec![vec![FAKE_SCRAP_HEAP, state]]
        );
    }

    /// An object with a bound (center, radius) as `Process` reads it.
    fn bound_object(e: &mut Engine, radius: f32) -> u32 {
        let bound = e.mem.alloc(16);
        e.mem.set_f32(bound + 12, radius);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0x20, bound);
        object
    }

    /// The `WhichSide` double returns the integer part of a plane's normal
    /// x; this sets plane `plane` of the set at `planes`.
    fn set_plane_side(e: &mut Engine, planes: u32, plane: u32, side: f32) {
        e.mem.set_f32(planes + plane * 16, side);
    }

    /// A program of one test `op` of plane set 0 (pass at 2, fail at 3),
    /// threaded by hand.
    fn single_test_program(e: &mut Engine, frustum: Frustum, op: i32) {
        load_ops(
            e,
            frustum,
            &[
                [op, 2, 3],
                [0, 0, 0],
                [OP_END_PASS, 0, 0],
                [OP_END_FAIL, 0, 0],
            ],
        );
        e.set(frustum, BSCompoundFrustum::iFirstOp, 0);
        load_planes(e, frustum, 1);
    }

    #[test]
    fn process_needs_a_bound_with_a_radius() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        let no_bound = e.mem.alloc(0x40);
        assert!(!e.call(0x00c4_91a0, &args![frustum, no_bound]).bool());
        let flat = bound_object(&mut e, 0.0);
        assert!(!e.call(0x00c4_91a0, &args![frustum, flat]).bool());
        // A real bound with no operations passes when the frustum accepts it.
        let object = bound_object(&mut e, 2.0);
        assert!(e.call(0x00c4_91a0, &args![frustum, object]).bool());
    }

    #[test]
    fn process_tests_the_view_frustum_then_walks_the_program() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        single_test_program(&mut e, frustum, OP_PORTAL);
        let object = bound_object(&mut e, 2.0);
        let view = view_frustum(frustum).addr();
        let set = plane_address(&e, frustum, 0);

        // The bound is in front of the view planes (side 0) and the set's
        // active planes are all in front (side 1): passes.
        set_plane_mask(&mut e, frustum, 0, 0x3);
        set_plane_side(&mut e, set, 0, 1.0);
        set_plane_side(&mut e, set, 1, 1.0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c4_91a0, &args![frustum, object]).bool());
        assert_eq!(plane_mask(&e, frustum, 0), 0, "front planes go inactive");
        assert_eq!(calls(&e, NI_BOUND_WHICH_SIDE).len(), 6 + 2);

        // Behind one active plane of the set: the program fails.
        set_plane_mask(&mut e, frustum, 0, 0x3);
        set_plane_side(&mut e, set, 1, 2.0);
        assert!(!e.call(0x00c4_91a0, &args![frustum, object]).bool());

        // Behind a view frustum plane: false without walking.
        set_plane_side(&mut e, set, 1, 1.0);
        set_plane_mask(&mut e, frustum, 0, 0x3);
        set_plane_side(&mut e, view, 2, 2.0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c4_91a0, &args![frustum, object]).bool());
        assert_eq!(calls(&e, NI_BOUND_WHICH_SIDE).len(), 3);

        // With the skip flag the view frustum is not looked at.
        e.set(frustum, BSCompoundFrustum::bSkipViewFrustum, true);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c4_91a0, &args![frustum, object]).bool());
        assert_eq!(calls(&e, NI_BOUND_WHICH_SIDE).len(), 2);
    }

    #[test]
    fn process_occlusion_op_uses_the_inverse_bound_test() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        single_test_program(&mut e, frustum, OP_OCCLUSION);
        let object = bound_object(&mut e, 2.0);
        let set = plane_address(&e, frustum, 0);
        set_plane_mask(&mut e, frustum, 0, 0x1);
        // Wholly in front of the only plane: the inverse test is false, so
        // the program fails.
        set_plane_side(&mut e, set, 0, 1.0);
        assert!(!e.call(0x00c4_91a0, &args![frustum, object]).bool());
        // Not wholly in front: true, the program passes.
        set_plane_mask(&mut e, frustum, 0, 0x1);
        set_plane_side(&mut e, set, 0, 0.0);
        assert!(e.call(0x00c4_91a0, &args![frustum, object]).bool());
    }

    /// The object `00c493a0` takes: its +0xC points at a tester whose slot
    /// +0x9C passes when the set has any active plane and slot +0xA0 when
    /// bit 0 is active.
    fn tester_object(e: &mut Engine) -> u32 {
        let vtable = 0x0070_0000;
        e.map(vtable, 0x1000);
        e.mem.set_u32(vtable + 0x9c, 0x0070_0100);
        e.mem.set_u32(vtable + 0xa0, 0x0070_0104);
        e.register(0x0070_0100, |e, a| Ret {
            eax: (e.mem.u32(a[1] + 0x60) != 0) as u32,
            ..Ret::default()
        });
        e.register(0x0070_0104, |e, a| Ret {
            eax: e.mem.u32(a[1] + 0x60) & 1,
            ..Ret::default()
        });
        let tester = e.mem.alloc(8);
        e.mem.set_u32(tester, vtable);
        let target = e.mem.alloc(0x20);
        e.mem.set_u32(target + 0xc, tester);
        target
    }

    #[test]
    fn virtual_test_walks_the_program_with_the_targets_tests() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        let target = tester_object(&mut e);
        single_test_program(&mut e, frustum, OP_PORTAL);
        set_plane_mask(&mut e, frustum, 0, 0x2);
        // The view frustum has planes active (0x3f): slot 0x9c passes; the
        // set has planes: the portal passes.
        assert!(e.call(0x00c4_93a0, &args![frustum, target]).bool());
        set_plane_mask(&mut e, frustum, 0, 0);
        assert!(!e.call(0x00c4_93a0, &args![frustum, target]).bool());

        // An occlusion op inverts slot 0xa0: bit 0 active means fail.
        single_test_program(&mut e, frustum, OP_OCCLUSION);
        set_plane_mask(&mut e, frustum, 0, 0x1);
        assert!(!e.call(0x00c4_93a0, &args![frustum, target]).bool());
        set_plane_mask(&mut e, frustum, 0, 0x2);
        assert!(e.call(0x00c4_93a0, &args![frustum, target]).bool());

        // A view frustum with nothing active fails before the program,
        // unless skipped.
        e.set(view_frustum(frustum), NiFrustumPlanes::m_uiActivePlanes, 0);
        assert!(!e.call(0x00c4_93a0, &args![frustum, target]).bool());
        e.set(frustum, BSCompoundFrustum::bSkipViewFrustum, true);
        assert!(e.call(0x00c4_93a0, &args![frustum, target]).bool());
    }

    #[test]
    fn occlusion_plane_walk_uses_the_plane_tests() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        e.register(OCCLUSION_TEST_AGAINST_VIEW, returns_true);
        e.register(OCCLUSION_WITHIN_FRUSTUM, |e, a| Ret {
            eax: (e.mem.u32(a[1] + 0x60) != 0) as u32,
            ..Ret::default()
        });
        e.register(OCCLUSION_TEST_PLANES, |e, a| Ret {
            eax: e.mem.u32(a[1] + 0x60) & 1,
            ..Ret::default()
        });
        e.set_global(VIEW_POSITION_GLOBAL, 1.0f32);
        e.set_global(VIEW_POSITION_GLOBAL + 4, 2.0f32);
        e.set_global(VIEW_POSITION_GLOBAL + 8, 3.0f32);
        let plane = e.mem.alloc(0x110);
        single_test_program(&mut e, frustum, OP_PORTAL);
        set_plane_mask(&mut e, frustum, 0, 0x2);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c4_9560, &args![frustum, plane]).bool());
        assert_eq!(
            calls(&e, OCCLUSION_TEST_AGAINST_VIEW),
            vec![vec![plane, frustum.addr() + 0x30, frustum.addr() + 0x94]]
        );
        assert_eq!(
            calls(&e, OCCLUSION_WITHIN_FRUSTUM),
            vec![vec![
                plane,
                plane_address(&e, frustum, 0),
                0,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits()
            ]]
        );
        set_plane_mask(&mut e, frustum, 0, 0);
        assert!(!e.call(0x00c4_9560, &args![frustum, plane]).bool());

        // The occlusion op inverts the plane test.
        single_test_program(&mut e, frustum, OP_OCCLUSION);
        set_plane_mask(&mut e, frustum, 0, 0x1);
        assert!(!e.call(0x00c4_9560, &args![frustum, plane]).bool());
        set_plane_mask(&mut e, frustum, 0, 0x2);
        assert!(e.call(0x00c4_9560, &args![frustum, plane]).bool());

        // The skip flag bypasses the view test.
        e.set(frustum, BSCompoundFrustum::bSkipViewFrustum, true);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c4_9560, &args![frustum, plane]).bool());
        assert!(calls(&e, OCCLUSION_TEST_AGAINST_VIEW).is_empty());
    }

    #[test]
    fn remove_empty_groups_drops_nested_and_repeated_empty_pairs() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        load_ops(
            &mut e,
            frustum,
            &[
                [4, 0, 0],
                [6, 0, 0],
                [5, 0, 0],
                [8, 0, 0],
                [0, 0, 0],
                [6, 0, 0],
                [5, 0, 0],
                [6, 0, 0],
            ],
        );
        e.call(0x00c4_9700, &args![frustum]);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 4);
        assert_eq!(op_codes(&e, frustum, 4), vec![5, 8, 0, 6]);
    }

    #[test]
    fn prethread_single_test_jumps_to_the_end_ops() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        load_ops(&mut e, frustum, &[[8, 0, 0], [0, 0, 0]]);
        e.call_log = Some(vec![]);
        e.call(0x00c4_9850, &args![frustum]);
        assert!(e.get(frustum, BSCompoundFrustum::bPrethreaded));
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFirstOp), 0);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 2);
        // The end-of-list, pass and fail operations sit past the list.
        assert_eq!(op_code(&e, frustum, 2), OP_END_OF_LIST);
        assert_eq!(op_code(&e, frustum, 3), OP_END_PASS);
        assert_eq!(op_code(&e, frustum, 4), OP_END_FAIL);
        assert_eq!(op_words(&e, frustum, 0), [8, 3, 4]);
        // Three scrap arrays, all given back.
        assert_eq!(calls(&e, SCRAP_HEAP_ALLOCATE).len(), 3);
        assert_eq!(calls(&e, SCRAP_HEAP_DEALLOCATE).len(), 3);
    }

    #[test]
    fn prethread_union_group_links_pass_and_fail_paths() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        load_ops(
            &mut e,
            frustum,
            &[
                [5, 0, 0],
                [8, 0, 0],
                [0, 0, 0],
                [8, 0, 0],
                [1, 0, 0],
                [6, 0, 0],
            ],
        );
        e.call(0x00c4_9850, &args![frustum]);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFirstOp), 1);
        // First test: pass leaves the union (to the end-pass op at 7), fail
        // tries the second; the second passes to 7 and fails to 8.
        assert_eq!(op_words(&e, frustum, 1), [8, 7, 3]);
        assert_eq!(op_words(&e, frustum, 3), [8, 7, 8]);
        assert_eq!(op_code(&e, frustum, 7), OP_END_PASS);
        assert_eq!(op_code(&e, frustum, 8), OP_END_FAIL);
    }

    #[test]
    fn prethread_stops_when_the_list_ends_inside_a_group() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        load_ops(&mut e, frustum, &[[5, 0, 0], [1, 0, 0], [6, 0, 0]]);
        e.call_log = Some(vec![]);
        e.call(0x00c4_9850, &args![frustum]);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFreeOp), 0);
        assert!(!e.get(frustum, BSCompoundFrustum::bPrethreaded));
        // The scrap arrays are not given back on this path.
        assert!(calls(&e, SCRAP_HEAP_DEALLOCATE).is_empty());
    }

    #[test]
    fn prethread_without_tests_reports_and_points_at_the_end() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        load_ops(&mut e, frustum, &[[4, 0, 0], [6, 0, 0]]);
        e.call_log = Some(vec![]);
        e.call(0x00c4_9850, &args![frustum]);
        assert_eq!(calls(&e, DEBUG_MESSAGE), vec![vec![EMPTY_OP_LIST_TEXT]]);
        assert_eq!(e.get(frustum, BSCompoundFrustum::iFirstOp), 3);
        assert!(e.get(frustum, BSCompoundFrustum::bPrethreaded));
    }

    #[test]
    fn prethread_does_nothing_twice_or_for_an_empty_list() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00c4_9850, &args![frustum]);
        assert!(e.get(frustum, BSCompoundFrustum::bPrethreaded));
        assert!(calls(&e, SCRAP_HEAP_ALLOCATE).is_empty());

        load_ops(&mut e, frustum, &[[8, 0, 0], [0, 0, 0]]);
        e.call(0x00c4_9850, &args![frustum]);
        assert!(calls(&e, SCRAP_HEAP_ALLOCATE).is_empty());
        assert_eq!(op_words(&e, frustum, 0), [8, 0, 0]);
    }

    #[test]
    fn debug_check_breaks_on_unbalanced_groups() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        e.call_log = Some(vec![]);
        load_ops(&mut e, frustum, &[[4, 0, 0], [6, 0, 0], [6, 0, 0]]);
        // The last close takes the depth below zero: one break there, one
        // for the unbalanced end (depth -1).
        e.call(0x00c4_9f10, &args![frustum]);
        assert_eq!(calls(&e, DEBUG_BREAK).len(), 2);

        e.call_log = Some(vec![]);
        load_ops(
            &mut e,
            frustum,
            &[[5, 0, 0], [8, 0, 0], [0, 0, 0], [6, 0, 0]],
        );
        e.call(0x00c4_9f10, &args![frustum]);
        assert!(calls(&e, DEBUG_BREAK).is_empty());

        load_ops(&mut e, frustum, &[[4, 0, 0]]);
        e.call(0x00c4_9f10, &args![frustum]);
        assert_eq!(calls(&e, DEBUG_BREAK).len(), 1);
    }

    // --- the array templates ----------------------------------------------

    #[test]
    fn plane_array_constructor_installs_the_vtable_and_empties() {
        let mut e = engine();
        let array = e.new_object::<BSSimpleArray>();
        e.set(array, BSSimpleArray::iSize, 9);
        let result = e.call(0x00c4_9fe0, &args![array]).ptr::<BSSimpleArray>();
        assert_eq!(result, array);
        assert_eq!(e.mem.u32(array.addr()), PLANES_ARRAY_VTABLE);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 0);
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 0);
    }

    #[test]
    fn plane_array_set_size_allocates_grows_and_shrinks() {
        let mut e = engine();
        let array = e.new_object::<BSSimpleArray>();
        e.call(0x00c4_9fe0, &args![array]);
        e.call_log = Some(vec![]);
        // From empty: one allocation of 3 sets, all constructed.
        e.call(0x00c4_a070, &args![array, 3u32, true]);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 3);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 3);
        let buffer = e.get(array, BSSimpleArray::pBuffer);
        assert_eq!(calls(&e, NI_PLANE_CONSTRUCT).len(), 18);
        assert_eq!(e.mem.u32(buffer + 0x60 + 2 * 0x64), 0x3f);

        // Growing to 5 reallocates in place (size equals capacity) and
        // constructs the two new sets only.
        e.mem.set_u32(buffer, 0x777);
        e.call_log = Some(vec![]);
        e.call(0x00c4_a070, &args![array, 5u32, true]);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 5);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 5);
        let buffer = e.get(array, BSSimpleArray::pBuffer);
        assert_eq!(e.mem.u32(buffer), 0x777);
        assert_eq!(calls(&e, NI_PLANE_CONSTRUCT).len(), 12);

        // Shrinking without the flag keeps the capacity.
        e.call(0x00c4_a070, &args![array, 4u32, false]);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 4);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 5);
        // Growing within the capacity constructs only the new sets.
        e.call_log = Some(vec![]);
        e.call(0x00c4_a070, &args![array, 5u32, false]);
        assert_eq!(calls(&e, NI_PLANE_CONSTRUCT).len(), 6);

        // Shrinking to a quarter of the capacity or less, with the flag,
        // moves the elements into a block of the new size.
        e.call(0x00c4_a070, &args![array, 1u32, true]);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 1);
        let moved = e.get(array, BSSimpleArray::pBuffer);
        assert_ne!(moved, buffer);
        assert_eq!(e.mem.u32(moved), 0x777);

        // Zero clears.
        e.call_log = Some(vec![]);
        e.call(0x00c4_a070, &args![array, 0u32, true]);
        assert_eq!(calls(&e, ARRAY_CLEAR), vec![vec![array.addr(), 1]]);
    }

    #[test]
    fn op_array_constructor_installs_its_vtable_and_initialises() {
        let mut e = engine();
        let array = e.new_object::<BSSimpleArray>();
        let result = e.call(0x00c4_a1b0, &args![array]).ptr::<BSSimpleArray>();
        assert_eq!(result, array);
        assert_eq!(e.mem.u32(array.addr()), OPS_ARRAY_VTABLE);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 0);
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0);
    }

    #[test]
    fn op_array_set_size_allocates_grows_and_shrinks() {
        let mut e = engine();
        let array = e.new_object::<BSSimpleArray>();
        e.call(0x00c4_a1b0, &args![array]);
        e.call(0x00c4_a1e0, &args![array, 4u32, true]);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 4);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 4);
        let buffer = e.get(array, BSSimpleArray::pBuffer);
        e.mem.set_u32(buffer + 12, 0x42);
        // Grow: size equals capacity, so the block is reallocated in place.
        e.call(0x00c4_a1e0, &args![array, 8u32, true]);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 8);
        let buffer = e.get(array, BSSimpleArray::pBuffer);
        assert_eq!(e.mem.u32(buffer + 12), 0x42);
        // Shrink below a quarter of the capacity: a smaller block with the
        // elements moved by `006dc6e0`.
        e.call_log = Some(vec![]);
        e.call(0x00c4_a1e0, &args![array, 2u32, true]);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 2);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 2);
        assert_eq!(calls(&e, MOVE_FUNCTION_OPS).len(), 1);
        // Shrinking between a quarter and the size keeps the block.
        let before = e.get(array, BSSimpleArray::pBuffer);
        e.call(0x00c4_a1e0, &args![array, 1u32, false]);
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), before);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 1);
        // Growing within the capacity only moves the size.
        e.call(0x00c4_a1e0, &args![array, 2u32, false]);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 2);
        e.call_log = Some(vec![]);
        e.call(0x00c4_a1e0, &args![array, 0u32, true]);
        assert_eq!(calls(&e, ARRAY_CLEAR), vec![vec![array.addr(), 1]]);
    }

    #[test]
    fn construct_items_builds_six_planes_each_and_skips_a_null_item() {
        let mut e = engine();
        let array = e.new_object::<BSSimpleArray>();
        e.call_log = Some(vec![]);
        e.call(0x00c4_a360, &args![array, 0u32, 1u32]);
        assert!(calls(&e, NI_PLANE_CONSTRUCT).is_empty());
        let items = e.mem.alloc(2 * 0x64);
        e.call(0x00c4_a360, &args![array, items, 2u32]);
        let constructed = calls(&e, NI_PLANE_CONSTRUCT);
        assert_eq!(constructed.len(), 12);
        assert_eq!(constructed[0], vec![items]);
        assert_eq!(constructed[7], vec![items + 0x64 + 0x10]);
        assert_eq!(e.mem.u32(items + 0x60), 0x3f);
        assert_eq!(e.mem.u32(items + 0x64 + 0x60), 0x3f);
    }

    #[test]
    fn plane_reallocate_buffer_covers_its_three_cases() {
        let mut e = engine();
        let array = e.new_object::<BSSimpleArray>();
        e.call(0x00c4_9fe0, &args![array]);
        // No buffer: allocated through slot +4, capacity recorded.
        e.call(0x00c4_a420, &args![array, 2u32, 0u32]);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 2);
        let first = e.get(array, BSSimpleArray::pBuffer);
        assert_ne!(first, 0);
        e.mem.set_u32(first, 0x1111);
        e.mem.set_u32(first + 0x64, 0x2222);
        // The element count equals the capacity: reallocated through +0xC.
        e.call(0x00c4_a420, &args![array, 4u32, 2u32]);
        let second = e.get(array, BSSimpleArray::pBuffer);
        assert_eq!(e.mem.u32(second + 0x64), 0x2222);
        // Otherwise a new block, the elements moved, the old block freed.
        e.call(0x00c4_a420, &args![array, 6u32, 1u32]);
        let third = e.get(array, BSSimpleArray::pBuffer);
        assert_ne!(third, second);
        assert_eq!(e.mem.u32(third), 0x1111);
        assert_eq!(e.mem.block_size(second), None);
    }

    #[test]
    fn op_reallocate_buffer_covers_its_three_cases() {
        let mut e = engine();
        let array = e.new_object::<BSSimpleArray>();
        e.call(0x00c4_a1b0, &args![array]);
        e.call(0x00c4_a4e0, &args![array, 2u32, 0u32]);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 2);
        let first = e.get(array, BSSimpleArray::pBuffer);
        e.mem.set_u32(first + 12, 0x2222);
        e.call(0x00c4_a4e0, &args![array, 4u32, 2u32]);
        let second = e.get(array, BSSimpleArray::pBuffer);
        assert_eq!(e.mem.u32(second + 12), 0x2222);
        e.call_log = Some(vec![]);
        e.call(0x00c4_a4e0, &args![array, 6u32, 1u32]);
        let third = e.get(array, BSSimpleArray::pBuffer);
        assert_ne!(third, second);
        assert_eq!(
            calls(&e, MOVE_FUNCTION_OPS),
            vec![vec![array.addr(), third, second, 1]]
        );
        assert_eq!(e.mem.block_size(second), None);
    }

    #[test]
    fn op_array_initialiser_allocates_the_larger_of_capacity_and_size() {
        let mut e = engine();
        let array = e.new_object::<BSSimpleArray>();
        e.call(0x00c4_a1b0, &args![array]);
        e.call(0x00c4_a5a0, &args![array, 0u32, 0u32]);
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0);
        e.call(0x00c4_a5a0, &args![array, 3u32, 0u32]);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 3);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 0);
        e.call(0x00c4_a5a0, &args![array, 2u32, 5u32]);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 5);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 5);
        assert_ne!(e.get(array, BSSimpleArray::pBuffer), 0);
    }

    #[test]
    fn move_items_goes_forward_below_and_backward_above() {
        let mut e = engine();
        let array = e.new_object::<BSSimpleArray>();
        let base = e.mem.alloc(5 * 0x64);
        for index in 0..5u32 {
            e.mem.set_u32(base + index * 0x64, index + 1);
        }
        // Target below the source: ascending copies.
        e.call_log = Some(vec![]);
        e.call(0x00c4_a640, &args![array, base, base + 0x64, 3u32]);
        let order: Vec<u32> = calls(&e, MEMMOVE).iter().map(|c| c[0]).collect();
        assert_eq!(order, vec![base, base + 0x64, base + 0xc8]);
        assert_eq!(calls(&e, MEMMOVE)[0], vec![base, base + 0x64, 0x64]);
        let values: Vec<u32> = (0..5).map(|index| e.mem.u32(base + index * 0x64)).collect();
        assert_eq!(values, vec![2, 3, 4, 4, 5]);
        // Target above the source: descending copies.
        e.mem.set_u32(base, 1);
        e.mem.set_u32(base + 0x64, 2);
        e.mem.set_u32(base + 0xc8, 3);
        e.call_log = Some(vec![]);
        e.call(0x00c4_a640, &args![array, base + 0x64, base, 3u32]);
        let order: Vec<u32> = calls(&e, MEMMOVE).iter().map(|c| c[0]).collect();
        assert_eq!(
            order,
            vec![base + 0x64 + 0xc8, base + 0x64 + 0x64, base + 0x64]
        );
        let values: Vec<u32> = (0..4).map(|index| e.mem.u32(base + index * 0x64)).collect();
        assert_eq!(values, vec![1, 1, 2, 3]);
        // Nothing for a count of 0 or equal addresses.
        e.call_log = Some(vec![]);
        e.call(0x00c4_a640, &args![array, base, base, 3u32]);
        e.call(0x00c4_a640, &args![array, base, base + 8, 0u32]);
        assert!(calls(&e, MEMMOVE).is_empty());
    }

    /// A plane set with the given mask and `sides` as the planes' normal x.
    fn plane_set(e: &mut Engine, mask: u32, sides: &[f32]) -> u32 {
        let set = e.mem.alloc(0x64);
        e.mem.set_u32(set + 0x60, mask);
        for (index, side) in sides.iter().enumerate() {
            e.mem.set_f32(set + index as u32 * 16, *side);
        }
        set
    }

    #[test]
    fn bound_test_passes_unless_behind_an_active_plane() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        let bound = e.mem.alloc(16);
        // No active planes: passes without asking.
        let none = plane_set(&mut e, 0, &[]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c4_a6e0, &args![frustum, bound, none]).bool());
        assert!(calls(&e, NI_BOUND_WHICH_SIDE).is_empty());
        // In front of plane 0 (made inactive), straddling plane 1: passes.
        let set = plane_set(&mut e, 0b11, &[1.0, 0.0]);
        assert!(e.call(0x00c4_a6e0, &args![frustum, bound, set]).bool());
        assert_eq!(e.mem.u32(set + 0x60), 0b10);
        assert_eq!(
            calls(&e, NI_BOUND_WHICH_SIDE),
            vec![vec![bound, set], vec![bound, set + 16]]
        );
        // Behind plane 1: fails.
        let set = plane_set(&mut e, 0b11, &[1.0, 2.0]);
        assert!(!e.call(0x00c4_a6e0, &args![frustum, bound, set]).bool());
    }

    #[test]
    fn inverse_bound_test_stops_at_the_first_plane_not_wholly_in_front() {
        let mut e = engine();
        let frustum = new_frustum(&mut e);
        let bound = e.mem.alloc(16);
        let none = plane_set(&mut e, 0, &[]);
        assert!(!e.call(0x00c4_a790, &args![frustum, bound, none]).bool());
        // In front of both planes: both made inactive, the walk completes.
        let set = plane_set(&mut e, 0b11, &[1.0, 1.0]);
        assert!(!e.call(0x00c4_a790, &args![frustum, bound, set]).bool());
        assert_eq!(e.mem.u32(set + 0x60), 0);
        // Not in front of the second plane: stops early.
        let set = plane_set(&mut e, 0b11, &[1.0, 0.0]);
        assert!(e.call(0x00c4_a790, &args![frustum, bound, set]).bool());
        assert_eq!(e.mem.u32(set + 0x60), 0b10);
    }

    #[test]
    fn plane_array_destructor_installs_the_vtable_and_clears() {
        let mut e = engine();
        let array = e.new_object::<BSSimpleArray>();
        e.call_log = Some(vec![]);
        e.call(0x00c4_a840, &args![array]);
        assert_eq!(e.mem.u32(array.addr()), PLANES_ARRAY_VTABLE);
        assert_eq!(calls(&e, ARRAY_CLEAR), vec![vec![array.addr(), 1]]);
    }

    #[test]
    fn op_array_destructor_installs_the_vtable_and_clears() {
        let mut e = engine();
        let array = e.new_object::<BSSimpleArray>();
        e.call_log = Some(vec![]);
        e.call(0x00c4_a8e0, &args![array]);
        assert_eq!(e.mem.u32(array.addr()), OPS_ARRAY_VTABLE);
        assert_eq!(calls(&e, ARRAY_CLEAR), vec![vec![array.addr(), 1]]);
    }

    #[test]
    fn plane_array_allocate_asks_the_memory_manager_for_sets() {
        let mut e = engine();
        let array = e.new_object::<BSSimpleArray>();
        e.call_log = Some(vec![]);
        let block = e.call(0x00c4_a860, &args![array, 2u32]).u32();
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 2 * 0x64]]
        );
        assert!(e.mem.block_size(block).unwrap() >= 0xc8);
    }

    #[test]
    fn plane_array_reallocate_forwards_to_the_memory_manager() {
        let mut e = engine();
        let array = e.new_object::<BSSimpleArray>();
        let block = e.mem.alloc(0x64);
        e.call_log = Some(vec![]);
        let result = e.call(0x00c4_a8b0, &args![array, block, 3u32]).u32();
        assert_eq!(
            calls(&e, MEMORY_REALLOCATE),
            vec![vec![MEMORY_MANAGER, block, 3 * 0x64]]
        );
        assert_ne!(result, 0);
    }

    #[test]
    fn op_array_allocate_asks_the_memory_manager_for_ops() {
        let mut e = engine();
        let array = e.new_object::<BSSimpleArray>();
        e.call_log = Some(vec![]);
        let block = e.call(0x00c4_a900, &args![array, 5u32]).u32();
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 60]]);
        assert!(e.mem.block_size(block).unwrap() >= 60);
    }

    #[test]
    fn op_array_reallocate_forwards_to_the_memory_manager() {
        let mut e = engine();
        let array = e.new_object::<BSSimpleArray>();
        let block = e.mem.alloc(24);
        e.call_log = Some(vec![]);
        let result = e.call(0x00c4_a930, &args![array, block, 4u32]).u32();
        assert_eq!(
            calls(&e, MEMORY_REALLOCATE),
            vec![vec![MEMORY_MANAGER, block, 48]]
        );
        assert_ne!(result, 0);
    }

    /// The real `Clear`, over the engine's double.
    fn real_clear(e: &mut Engine, a: &[u32]) -> Ret {
        bs_simple_array_clear(e, Ptr::new(a[0]), a[1] as u8 != 0);
        Ret::default()
    }

    #[test]
    fn clear_without_a_buffer_changes_nothing() {
        let mut e = engine();
        e.register(ARRAY_CLEAR, real_clear);
        let array = e.new_object::<BSSimpleArray>();
        e.set(array, BSSimpleArray::iSize, 3);
        e.call(ARRAY_CLEAR, &args![array, true]);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 3);
    }

    #[test]
    fn clear_keeps_the_buffer_unless_asked_to_free() {
        let mut e = engine();
        e.register(ARRAY_CLEAR, real_clear);
        let array = e.new_object::<BSSimpleArray>();
        let block = e.mem.alloc(24);
        e.set(array, BSSimpleArray::pBuffer, block);
        e.set(array, BSSimpleArray::iSize, 2);
        e.set(array, BSSimpleArray::iReservedSize, 4);
        e.call(ARRAY_CLEAR, &args![array, false]);
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), block);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 0);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 4);
    }

    #[test]
    fn clear_with_free_gives_the_buffer_back() {
        let mut e = engine();
        e.register(ARRAY_CLEAR, real_clear);
        let array = e.new_object::<BSSimpleArray>();
        e.mem.set_u32(array.addr(), OPS_ARRAY_VTABLE);
        let block = e.mem.alloc(24);
        e.set(array, BSSimpleArray::pBuffer, block);
        e.set(array, BSSimpleArray::iSize, 2);
        e.set(array, BSSimpleArray::iReservedSize, 4);
        e.call(ARRAY_CLEAR, &args![array, true]);
        assert_eq!(e.get(array, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(array, BSSimpleArray::iSize), 0);
        assert_eq!(e.get(array, BSSimpleArray::iReservedSize), 0);
        assert_eq!(e.mem.block_size(block), None);
    }

    #[test]
    fn scalar_deleting_destructors_free_the_object_only_with_bit_zero() {
        for (addr, vtable) in [
            (0x00c4_a960u32, PLANES_ARRAY_VTABLE),
            (0x00c4_a990, OPS_ARRAY_VTABLE),
        ] {
            let mut e = engine();
            e.register(MEMORY_DEALLOCATE, noop);
            let array = e.new_object::<BSSimpleArray>();
            e.call_log = Some(vec![]);
            let result = e.call(addr, &args![array, 0u32]).u32();
            assert_eq!(result, array.addr());
            assert_eq!(e.mem.u32(array.addr()), vtable);
            assert_eq!(calls(&e, ARRAY_CLEAR), vec![vec![array.addr(), 1]]);
            assert!(calls(&e, MEMORY_DEALLOCATE).is_empty());
            let result = e.call(addr, &args![array, 3u32]).u32();
            assert_eq!(result, array.addr());
            assert_eq!(
                calls(&e, MEMORY_DEALLOCATE),
                vec![vec![MEMORY_MANAGER, array.addr()]]
            );
        }
    }
}
