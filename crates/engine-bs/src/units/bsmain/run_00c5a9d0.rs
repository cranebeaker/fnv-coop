//! `BSMain/run_00c5a9d0` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsmain::bsportalgraph::BSPortalGraph;

/// `NiRefObject`'s vtable (stored first, as the base constructor does).
const NI_REF_OBJECT_VTABLE: u32 = 0x0101_dce4;
/// `NiRefObject::ms_uiObjects` (Xbox PDB), the live-object count.
const NI_REF_OBJECT_COUNT: u32 = 0x011f_4400;
/// `BSPortalGraph`'s vtable (RTTI `.?AVBSPortalGraph@@`).
const PORTAL_GRAPH_VTABLE: u32 = 0x010c_2d44;
/// `NiTObjectArray<NiPointer<NiAVObject>>`'s vtable (RTTI).
const OBJECT_ARRAY_VTABLE: u32 = 0x0101_fbd4;
/// `BSSimpleArray<NiPointer<NiNode>,1024>`'s vtable (RTTI).
const SIMPLE_ARRAY_VTABLE: u32 = 0x0108_40e4;
/// `InterlockedIncrement` (import slot).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `004b0260` (thiscall, 2 words): the `NiTArray` base constructor (initial
/// size, grow by).
const ARRAY_CONSTRUCT: u32 = 0x004b_0260;
/// `00822860` (thiscall, 2 words): sets up the simple array.
const SIMPLE_ARRAY_INIT: u32 = 0x0082_2860;
/// `0066b0d0` (thiscall, pointer): assigns an `NiPointer`.
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;

/// Clears an `NiTPointerList`'s three words in the order the exe stores
/// them (+8, +0, +4).
fn clear_list(e: &mut Engine, list: u32) {
    e.mem.set_u32(list + 8, 0);
    e.mem.set_u32(list, 0);
    e.mem.set_u32(list + 4, 0);
}

/// An `NiPointer` constructed from null: stores 0 and, if the stored word is
/// non-zero (never, here), increments the object's count at +4.
fn null_ni_pointer(e: &mut Engine, slot: u32) {
    e.mem.set_u32(slot, 0);
    let object = e.mem.u32(slot);
    if object != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
    }
}

// Translated from 00c5a9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSPortalGraph` constructor (vtable `010c2d44`): the `NiRefObject`
/// base (count 0, `ms_uiObjects` incremented), every list empty, both
/// `NiPointer`s null, the always-render array (`004b0260(0, 1)`), the
/// unbound-node array (`00822860(0, 0)`), then both `NiPointer`s assigned
/// null again (`0066b0d0`). Returns `this`.
// Translation incomplete: 00c5a9d0 - SEH exception unwinding is unsupported.
pub fn fn_00c5a9d0(e: &mut Engine, this: Ptr<BSPortalGraph>) -> Ptr<BSPortalGraph> {
    let t = this.addr();
    e.mem.set_u32(t, NI_REF_OBJECT_VTABLE);
    e.mem.set_u32(t + 4, 0);
    e.call(INTERLOCKED_INCREMENT, &args![NI_REF_OBJECT_COUNT]);
    e.mem.set_u32(t, PORTAL_GRAPH_VTABLE);
    clear_list(e, t + BSPortalGraph::lOccluderList.off);
    clear_list(e, t + BSPortalGraph::lPortalList.off);
    clear_list(e, t + BSPortalGraph::lMultiBoundRoomList.off);
    null_ni_pointer(e, t + BSPortalGraph::spMultiBoundRoomRoot.off);
    clear_list(e, t + BSPortalGraph::lMultiBoundRoomAccumList.off);
    let array = t + BSPortalGraph::m_kAlwaysRenderChildren.off;
    e.call(ARRAY_CONSTRUCT, &args![array, 0u32, 1u32]);
    e.mem.set_u32(array, OBJECT_ARRAY_VTABLE);
    null_ni_pointer(e, t + BSPortalGraph::spPortalNodeRoot.off);
    clear_list(e, t + BSPortalGraph::kAttachAlwaysRenderQueue.off);
    clear_list(e, t + BSPortalGraph::kDetachAlwaysRenderQueue.off);
    let unbound = t + BSPortalGraph::kUnboundNodes.off;
    e.mem.set_u32(unbound, SIMPLE_ARRAY_VTABLE);
    e.call(SIMPLE_ARRAY_INIT, &args![unbound, 0u32, 0u32]);
    e.call(
        NI_POINTER_ASSIGN,
        &args![t + BSPortalGraph::spMultiBoundRoomRoot.off, 0u32],
    );
    e.call(
        NI_POINTER_ASSIGN,
        &args![t + BSPortalGraph::spPortalNodeRoot.off, 0u32],
    );
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00c5_a9d0,
        fn_00c5a9d0(Ptr<BSPortalGraph>) -> Ptr<BSPortalGraph>
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c5a9d0_constructs_an_empty_portal_graph() {
        let mut e = Engine::new();
        for addr in [
            INTERLOCKED_INCREMENT,
            ARRAY_CONSTRUCT,
            SIMPLE_ARRAY_INIT,
            NI_POINTER_ASSIGN,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x78);
        for i in 0..0x1e {
            e.mem.set_u32(this + 4 * i, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c5_a9d0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), PORTAL_GRAPH_VTABLE);
        assert_eq!(e.mem.u32(this + 0x3c), OBJECT_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x68), SIMPLE_ARRAY_VTABLE);
        for off in (0x04..0x3c).step_by(4).chain((0x4c..0x68).step_by(4)) {
            assert_eq!(e.mem.u32(this + off), 0, "+{off:#x}");
        }
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().skip(1).cloned().collect();
        assert_eq!(
            calls,
            vec![
                (INTERLOCKED_INCREMENT, vec![NI_REF_OBJECT_COUNT]),
                (ARRAY_CONSTRUCT, vec![this + 0x3c, 0, 1]),
                (SIMPLE_ARRAY_INIT, vec![this + 0x68, 0, 0]),
                (NI_POINTER_ASSIGN, vec![this + 0x2c, 0]),
                (NI_POINTER_ASSIGN, vec![this + 0x4c, 0]),
            ]
        );
    }
}
