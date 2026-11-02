//! `BSMain/bsdebrisnode.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSDebrisNode` is a `BSRangeNode` (0xB0 bytes on PC) that adds no members:
//! its methods forward to the `BSRangeNode` ones, and `GetViewerStrings`
//! adds the class name. The C++ exception-unwinding frames of `CreateClone`
//! and of the factory `00c4ae60` are not translated; those two carry the
//! `Translation incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;

/// Address of the `BSDebrisNode` vtable.
pub(crate) const BS_DEBRIS_NODE_VTABLE: u32 = 0x010c_1fc4;
/// Address of `BSDebrisNode`'s run-time type information (`GetRTTI`'s
/// result); its first word is the class name the viewer strings use.
pub(crate) const BS_DEBRIS_NODE_RTTI: u32 = 0x0120_2e84;
/// Size of a `BSDebrisNode` on PC (the Xbox build is 0xE0).
pub(crate) const BS_DEBRIS_NODE_SIZE: u32 = 0xb0;

/// The allocator `00aa13e0` (cdecl, size): returns the block.
const ALLOCATE: u32 = 0x00aa_13e0;
/// `BSRangeNode::BSRangeNode` (Xbox PDB), thiscall, no arguments.
const BS_RANGE_NODE_CONSTRUCT: u32 = 0x00c4_50d0;
/// `BSRangeNode::CopyMembers(this, target, cloneProcess)` (Xbox PDB).
const BS_RANGE_NODE_COPY_MEMBERS: u32 = 0x00c4_51b0;
/// `BSRangeNode::GetViewerStrings(this, strings)` (Xbox PDB).
const BS_RANGE_NODE_GET_VIEWER_STRINGS: u32 = 0x00c4_54c0;
/// `BSRangeNode::LoadBinary(this, stream)` (Xbox PDB).
const BS_RANGE_NODE_LOAD_BINARY: u32 = 0x00c4_5220;
/// A one-argument `BSRangeNode` method the engine map has no name for (the
/// one `00c4adc0` forwards to).
const BS_RANGE_NODE_METHOD_00C452A0: u32 = 0x00c4_52a0;
/// A one-argument `BSRangeNode` method returning a byte, unnamed in the
/// engine map (the one `00c4ade0` forwards to).
const BS_RANGE_NODE_METHOD_00C452C0: u32 = 0x00c4_52c0;
/// `BSRangeNode::SaveBinary(this, stream)` (Xbox PDB).
const BS_RANGE_NODE_SAVE_BINARY: u32 = 0x00c4_52f0;
/// `BSRangeNode::IsEqual(this, other)` (Xbox PDB): result in `AL`.
const BS_RANGE_NODE_IS_EQUAL: u32 = 0x00c4_5380;
/// `NiGetViewerString(name)` (Xbox PDB), cdecl: returns the new string.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
/// Appends the string whose address is passed to the viewer-string array
/// (`this` = the array; its body grows the `NiTArray` with `SetAtGrow`).
const VIEWER_STRINGS_ADD: u32 = 0x0096_a610;

layout! {
    /// `BSDebrisNode` (Xbox PDB), 0xB0 bytes on PC: a `BSRangeNode` with no
    /// members of its own.
    pub struct BSDebrisNode: 0xb0 {}
}

type Node = Ptr<BSDebrisNode>;

/// Allocates and constructs a `BSDebrisNode` (null when the allocation
/// failed), as `CreateClone` and the factory do.
fn new_debris_node(e: &mut Engine) -> Node {
    let block = e.call(ALLOCATE, &args![BS_DEBRIS_NODE_SIZE]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(BS_RANGE_NODE_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, BS_DEBRIS_NODE_VTABLE);
    Ptr::new(block)
}

// Translated from 00c4aca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDebrisNode::CreateClone` (Xbox PDB): allocates 0xB0 bytes, builds a
/// `BSRangeNode` in them with the `BSDebrisNode` vtable (null if the
/// allocation failed) and copies this node's members into it
/// (`00c4ad40`, which also runs on a null result, as in the game).
// Translation incomplete: 00c4aca0 - C++ exception unwinding frame is not translated.
pub fn bs_debris_node_create_clone(e: &mut Engine, this: Node, clone_process: u32) -> Node {
    let clone = new_debris_node(e);
    fn_00c4ad40(e, this, clone, clone_process);
    clone
}

// Translated from 00c4ad40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CopyMembers` that `CreateClone` calls: forwards to
/// `BSRangeNode::CopyMembers`.
pub fn fn_00c4ad40(e: &mut Engine, this: Node, target: Node, clone_process: u32) {
    e.call(
        BS_RANGE_NODE_COPY_MEMBERS,
        &args![this, target, clone_process],
    );
}

// Translated from 00c4ad60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDebrisNode::GetViewerStrings` (Xbox PDB): the `BSRangeNode` strings,
/// then `NiGetViewerString` of the class name (the first word of the
/// run-time type information) appended to `strings`.
pub fn bs_debris_node_get_viewer_strings(e: &mut Engine, this: Node, strings: Ptr) {
    e.call(BS_RANGE_NODE_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(BS_DEBRIS_NODE_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(VIEWER_STRINGS_ADD, &args![strings, slot]);
    });
}

// Translated from 00c4ada0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSDebrisNode` `LoadBinary`: forwards to `BSRangeNode::LoadBinary`.
pub fn fn_00c4ada0(e: &mut Engine, this: Node, stream: Ptr) {
    e.call(BS_RANGE_NODE_LOAD_BINARY, &args![this, stream]);
}

// Translated from 00c4adc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards its one argument to the `BSRangeNode` method `00c452a0`.
pub fn fn_00c4adc0(e: &mut Engine, this: Node, stream: Ptr) {
    e.call(BS_RANGE_NODE_METHOD_00C452A0, &args![this, stream]);
}

// Translated from 00c4ade0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards its one argument to the `BSRangeNode` method `00c452c0` and
/// returns whether its byte result is non-zero.
pub fn fn_00c4ade0(e: &mut Engine, this: Node, stream: Ptr) -> bool {
    e.call(BS_RANGE_NODE_METHOD_00C452C0, &args![this, stream])
        .u8()
        != 0
}

// Translated from 00c4ae10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSDebrisNode` `SaveBinary`: forwards to `BSRangeNode::SaveBinary`.
pub fn fn_00c4ae10(e: &mut Engine, this: Node, stream: Ptr) {
    e.call(BS_RANGE_NODE_SAVE_BINARY, &args![this, stream]);
}

// Translated from 00c4ae30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSDebrisNode` `IsEqual`: the byte `BSRangeNode::IsEqual` returns.
pub fn fn_00c4ae30(e: &mut Engine, this: Node, other: Ptr) -> u8 {
    e.call(BS_RANGE_NODE_IS_EQUAL, &args![this, other]).u8()
}

// Translated from 00c4ae60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSDebrisNode` factory (no arguments): allocates and constructs a
/// node as `CreateClone` does and returns it (null if the allocation
/// failed).
// Translation incomplete: 00c4ae60 - C++ exception unwinding frame is not translated.
pub fn fn_00c4ae60(e: &mut Engine) -> Node {
    new_debris_node(e)
}

// Translated from 00c4aef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSDebrisNode::GetRTTI` (Xbox PDB): the address of the class's run-time
/// type information.
pub fn bs_debris_node_get_rtti(_e: &mut Engine, _this: Node) -> u32 {
    BS_DEBRIS_NODE_RTTI
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c4aca0,
            bs_debris_node_create_clone(Ptr<BSDebrisNode>, u32) -> Ptr<BSDebrisNode>
        ),
        entry!(
            0x00c4ad40,
            fn_00c4ad40(Ptr<BSDebrisNode>, Ptr<BSDebrisNode>, u32)
        ),
        entry!(
            0x00c4ad60,
            bs_debris_node_get_viewer_strings(Ptr<BSDebrisNode>, Ptr)
        ),
        entry!(0x00c4ada0, fn_00c4ada0(Ptr<BSDebrisNode>, Ptr)),
        entry!(0x00c4adc0, fn_00c4adc0(Ptr<BSDebrisNode>, Ptr)),
        entry!(0x00c4ade0, fn_00c4ade0(Ptr<BSDebrisNode>, Ptr) -> bool),
        entry!(0x00c4ae10, fn_00c4ae10(Ptr<BSDebrisNode>, Ptr)),
        entry!(0x00c4ae30, fn_00c4ae30(Ptr<BSDebrisNode>, Ptr) -> u8),
        entry!(0x00c4ae60, fn_00c4ae60() -> Ptr<BSDebrisNode>),
        entry!(
            0x00c4aef0,
            bs_debris_node_get_rtti(Ptr<BSDebrisNode>) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_engine() -> Engine {
        let mut e = Engine::new();
        for addr in [
            BS_RANGE_NODE_CONSTRUCT,
            BS_RANGE_NODE_COPY_MEMBERS,
            BS_RANGE_NODE_GET_VIEWER_STRINGS,
            BS_RANGE_NODE_LOAD_BINARY,
            BS_RANGE_NODE_METHOD_00C452A0,
            BS_RANGE_NODE_SAVE_BINARY,
            VIEWER_STRINGS_ADD,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn create_clone_builds_a_debris_node_and_copies_members() {
        let mut e = test_engine();
        let this: Node = e.new_object();
        e.call_log = Some(vec![]);
        let clone = e
            .call(0x00c4_aca0, &args![this, 0x55u32])
            .ptr::<BSDebrisNode>();
        let log = e.call_log.take().unwrap();
        assert!(!clone.is_null());
        assert_eq!(e.mem.u32(clone.addr()), BS_DEBRIS_NODE_VTABLE);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![0xb0]]);
        assert_eq!(
            calls_to(&log, BS_RANGE_NODE_CONSTRUCT),
            vec![vec![clone.addr()]]
        );
        assert_eq!(
            calls_to(&log, BS_RANGE_NODE_COPY_MEMBERS),
            vec![vec![this.addr(), clone.addr(), 0x55]]
        );
    }

    #[test]
    fn create_clone_copies_into_null_when_allocation_fails() {
        let mut e = test_engine();
        e.register(ALLOCATE, |_, _| Ret::default());
        let this: Node = e.new_object();
        e.call_log = Some(vec![]);
        let clone = e
            .call(0x00c4_aca0, &args![this, 7u32])
            .ptr::<BSDebrisNode>();
        let log = e.call_log.take().unwrap();
        assert!(clone.is_null());
        assert!(calls_to(&log, BS_RANGE_NODE_CONSTRUCT).is_empty());
        assert_eq!(
            calls_to(&log, BS_RANGE_NODE_COPY_MEMBERS),
            vec![vec![this.addr(), 0, 7]]
        );
    }

    #[test]
    fn copy_members_forwards() {
        let mut e = test_engine();
        e.call_log = Some(vec![]);
        e.call(0x00c4_ad40, &args![0x100u32, 0x200u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, BS_RANGE_NODE_COPY_MEMBERS),
            vec![vec![0x100, 0x200, 3]]
        );
    }

    #[test]
    fn get_viewer_strings_adds_the_class_name() {
        let mut e = test_engine();
        e.map(BS_DEBRIS_NODE_RTTI & !0xfff, 0x1000);
        e.set_global(BS_DEBRIS_NODE_RTTI, 0x0123_4567u32);
        e.register(NI_GET_VIEWER_STRING, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.register(VIEWER_STRINGS_ADD, |e, a| {
            // Record the string the slot holds.
            let text = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], text);
            Ret::default()
        });
        let strings = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x00c4_ad60, &args![0x300u32, strings]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, BS_RANGE_NODE_GET_VIEWER_STRINGS),
            vec![vec![0x300, strings]]
        );
        assert_eq!(
            calls_to(&log, NI_GET_VIEWER_STRING),
            vec![vec![0x0123_4567]]
        );
        assert_eq!(e.mem.u32(strings), 0x0123_4568);
    }

    #[test]
    fn stream_methods_forward() {
        let mut e = test_engine();
        e.call_log = Some(vec![]);
        e.call(0x00c4_ada0, &args![1u32, 2u32]);
        e.call(0x00c4_adc0, &args![3u32, 4u32]);
        e.call(0x00c4_ae10, &args![5u32, 6u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, BS_RANGE_NODE_LOAD_BINARY), vec![vec![1, 2]]);
        assert_eq!(
            calls_to(&log, BS_RANGE_NODE_METHOD_00C452A0),
            vec![vec![3, 4]]
        );
        assert_eq!(calls_to(&log, BS_RANGE_NODE_SAVE_BINARY), vec![vec![5, 6]]);
    }

    #[test]
    fn byte_method_is_normalised_to_bool() {
        let mut e = test_engine();
        e.register(BS_RANGE_NODE_METHOD_00C452C0, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        assert!(e.call(0x00c4_ade0, &args![0u32, 0x7702u32]).bool());
        assert_eq!(e.call(0x00c4_ade0, &args![0u32, 0x02u32]).u8(), 1);
        assert!(!e.call(0x00c4_ade0, &args![0u32, 0x100u32]).bool());
    }

    #[test]
    fn is_equal_returns_the_base_byte() {
        let mut e = test_engine();
        e.register(BS_RANGE_NODE_IS_EQUAL, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        assert_eq!(e.call(0x00c4_ae30, &args![0u32, 0x1234u32]).u8(), 0x34);
        assert_eq!(e.call(0x00c4_ae30, &args![0u32, 0u32]).u8(), 0);
    }

    #[test]
    fn factory_builds_a_node_or_returns_null() {
        let mut e = test_engine();
        let node = e.call(0x00c4_ae60, &args![]).ptr::<BSDebrisNode>();
        assert_eq!(e.mem.u32(node.addr()), BS_DEBRIS_NODE_VTABLE);
        e.register(ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let node = e.call(0x00c4_ae60, &args![]).ptr::<BSDebrisNode>();
        let log = e.call_log.take().unwrap();
        assert!(node.is_null());
        assert!(calls_to(&log, BS_RANGE_NODE_CONSTRUCT).is_empty());
    }

    #[test]
    fn get_rtti_is_the_class_block() {
        let mut e = test_engine();
        assert_eq!(e.call(0x00c4_aef0, &args![0u32]).u32(), 0x0120_2e84);
    }
}
