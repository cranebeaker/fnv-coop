//! The walk itself: an object handed to the process, and what a visible object does (a node culls
//! its children, a geometry hands itself to the process to be drawn).

use super::layout::{
    BSCullingProcess, BSMultiBound, BSMultiBoundNode, BSMultiBoundShape, NiAVObject,
    NiCullingProcess, NiGeometry, NiNode,
};
use super::{radius_not_zero, FAMILY, FLAG_APP_CULLED};
use engine_core::ptr::Ptr;
use engine_core::{args, typed_call, Engine};

/// The process's `Process`, virtual slot 0x44: `thiscall(process, object)`.
const SLOT_PROCESS: u32 = 0x44;
/// The process's `AppendVirtual`, virtual slot 0x4c: `thiscall(process, geometry)`.
const SLOT_APPEND_VIRTUAL: u32 = 0x4c;
/// A multi-bound shape's `WithinFrustum`, virtual slot 0x9c: `thiscall(shape, planes)`, a bool.
const SLOT_WITHIN_FRUSTUM: u32 = 0x9c;

/// `kCullMode` values the multi-bound node reads: every object visible, the planes only, the
/// shapes asked again whatever they said last.
const MODE_ALL_VISIBLE: u32 = 1;
const MODE_PLANES_ONLY: u32 = 3;
const MODE_ASK_SHAPES: u32 = 4;

/// `BSMultiBoundShape::kCullResult` (Xbox PDB `BSMBCullResult`): not known yet, inside, outside.
const RESULT_UNKNOWN: u32 = 0;
const RESULT_INSIDE: u32 = 1;
const RESULT_OUTSIDE: u32 = 2;

/// Child `index` of a node (null for an empty slot).
#[inline(always)]
fn child(e: &Engine, node: Ptr<NiNode>, index: u32) -> Ptr<NiAVObject> {
    let base = e.get(node, NiNode::m_pBase);
    Ptr::new(e.mem.u32(base.index(index, 4).addr()))
}

// Typed from 00a59e00
/// `NiAVObject::Cull` (Xbox PDB, `thiscall(object, process)`): unless the application culled the
/// object, the process's `Process` (slot 0x44) gets it.
pub fn cull(e: &mut Engine, this: Ptr<NiAVObject>, process: Ptr<NiCullingProcess>) {
    if e.get(this, NiAVObject::m_uFlags) & FLAG_APP_CULLED == 0 {
        e.vcall(process.addr(), SLOT_PROCESS, &args![this]);
    }
}

// Typed from 00a5dbe0
/// `NiNode::OnVisible` (Xbox PDB, `thiscall(node, process)`): only with a world bound whose radius
/// isn't zero (a NaN radius goes on), each child (the count read once, empty slots skipped) is
/// culled with the process.
pub fn node_on_visible(e: &mut Engine, this: Ptr<NiNode>, process: Ptr<NiCullingProcess>) {
    let object = this.cast::<NiAVObject>();
    let bound = e.get(object, NiAVObject::m_kWorldBound);
    if bound.is_null() || !radius_not_zero(e, bound) {
        return;
    }
    let count = u32::from(e.get(this, NiNode::m_usSize));
    for index in 0..count {
        let child = child(e, this, index);
        if !child.is_null() {
            typed_call!(FAMILY, 0x00a5_9e00, cull(e, child, process));
        }
    }
}

// Typed from 00a7fd90
/// `NiGeometry::OnVisible` (Xbox PDB, `thiscall(geometry, process)`): the process's `AppendVirtual`
/// (slot 0x4c) gets the geometry.
pub fn geometry_on_visible(e: &mut Engine, this: Ptr<NiGeometry>, process: Ptr<NiCullingProcess>) {
    e.vcall(process.addr(), SLOT_APPEND_VIRTUAL, &args![this]);
}

// Typed from 00c46f60
/// `BSMultiBoundNode::OnVisible` (Xbox PDB, `thiscall(node, process)`): nothing without non-null
/// children (`m_usESize`). Unless the process's mode is 1 or 3, a multi-bound's shape decides: its
/// last verdict, or in mode 4 a new one (`WithinFrustum`, slot 0x9c, on the process's planes:
/// inside or outside). A shape with no verdict yet is asked, and keeps the answer. Inside is
/// visible, outside or anything else not. When visible, the process's mode becomes the node's
/// (unless it was 3), each child (the count read at each step) is culled, and the mode is put back.
pub fn multibound_node_on_visible(
    e: &mut Engine,
    this: Ptr<BSMultiBoundNode>,
    process: Ptr<BSCullingProcess>,
) {
    let node = this.at(BSMultiBoundNode::base);
    if e.get(node, NiNode::m_usESize) == 0 {
        return;
    }
    let mode = e.get(process, BSCullingProcess::kCullMode);
    let mut visible = true;
    if mode != MODE_PLANES_ONLY
        && mode != MODE_ALL_VISIBLE
        && !e.get(this, BSMultiBoundNode::spMultiBound).is_null()
    {
        let shape = |e: &Engine| {
            let bound = e.get(this, BSMultiBoundNode::spMultiBound);
            e.get(bound, BSMultiBound::spShape)
        };
        let planes = process.addr().wrapping_add(0x2c);
        let mut result = e.get(shape(e), BSMultiBoundShape::kCullResult);
        if e.get(process, BSCullingProcess::kCullMode) == MODE_ASK_SHAPES {
            let inside = e
                .vcall(shape(e).addr(), SLOT_WITHIN_FRUSTUM, &args![planes])
                .u8();
            result = if inside != 0 {
                RESULT_INSIDE
            } else {
                RESULT_OUTSIDE
            };
        }
        match result {
            RESULT_UNKNOWN => {
                let inside = e
                    .vcall(shape(e).addr(), SLOT_WITHIN_FRUSTUM, &args![planes])
                    .u8();
                visible = inside != 0;
                let verdict = if inside != 0 {
                    RESULT_INSIDE
                } else {
                    RESULT_OUTSIDE
                };
                e.set(shape(e), BSMultiBoundShape::kCullResult, verdict);
            }
            RESULT_INSIDE => visible = true,
            _ => visible = false,
        }
    }
    if !visible {
        return;
    }
    if mode != MODE_PLANES_ONLY {
        let own = e.get(this, BSMultiBoundNode::CullingMode);
        e.set(process, BSCullingProcess::kCullMode, own);
    }
    let mut index = 0;
    while index < u32::from(e.get(node, NiNode::m_usSize)) {
        let child = child(e, node, index);
        if !child.is_null() {
            typed_call!(
                FAMILY,
                0x00a5_9e00,
                cull(e, child, process.cast::<NiCullingProcess>())
            );
        }
        index += 1;
    }
    e.set(process, BSCullingProcess::kCullMode, mode);
}
