//! `NiMain/nisortadjustnode.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiSortAdjustNode::ms_RTTI` (the class's `NiRTTI` record).
const NI_SORT_ADJUST_NODE_RTTI: u32 = 0x011f_5ed8;

// Translated from 00a95310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSortAdjustNode::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_sort_adjust_node_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SORT_ADJUST_NODE_RTTI)
}

/// `00a5dce0`: the jump target of `00a95290`.
const JUMP_TARGET_00A95290: u32 = 0x00a5_dce0;

// Translated from 00a95290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSortAdjustNode::ProcessClone` (Xbox PDB): a 5-byte jump to `00a5dce0` (engine map: `NiNode::ProcessClone`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?ProcessClone@NiSortAdjustNode@@UAAXAAVNiCloningProcess@@@Z`.
pub fn ni_sort_adjust_node_process_clone(e: &mut Engine, this: Ptr, process: Ptr) {
    e.call(JUMP_TARGET_00A95290, &args![this, process]);
}

/// `NiNode`'s functions (`ninode.cpp`, `__thiscall`): `IsEqual` (`00a5e600`), the destructor
/// body `00a5e930`, `OnVisible` (`00a5dbe0`), the constructor body `00a5ecb0` (one word: the
/// child count), the member copy `00a5dc80` (clone, cloning process), `LinkObject`
/// (`00a5e9d0`), `LoadBinary` (`00a5dd40`) and `SaveBinary` (`00a5e880`).
const NODE_IS_EQUAL: u32 = 0x00a5_e600;
const NODE_DESTRUCTOR: u32 = 0x00a5_e930;
const NODE_ON_VISIBLE: u32 = 0x00a5_dbe0;
const NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
const NODE_COPY_MEMBERS: u32 = 0x00a5_dc80;
const NODE_LINK_OBJECT: u32 = 0x00a5_e9d0;
const NODE_LOAD_BINARY: u32 = 0x00a5_dd40;
const NODE_SAVE_BINARY: u32 = 0x00a5_e880;
/// `NiStream::GetObjectFromLinkID` (`00a63430`) and the link-id read `00a645b0`
/// (`__thiscall` on the stream), the stream's file version (+0xd8), input (+0x24c, read
/// function +8) and output (+0x250, write function +0xc), each position at +4; the version
/// from which the node no longer stores a link (20.0.0.4).
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;
const VERSION_20_0_0_4: u32 = 0x1400_0004;
/// The allocation `00aa13e0` and the sized delete `00aa1460` (cdecl).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiSortAdjustNode`'s vtable (`010a0384`), size and sorting mode (`m_eSortingMode`, +0xac;
/// Xbox PDB name).
const NI_SORT_ADJUST_NODE_VTABLE: u32 = 0x010a_0384;
const NI_SORT_ADJUST_NODE_SIZE: u32 = 0xb0;
const SORTING_MODE: u32 = 0xac;
/// The culler's visible array (+8: its objects at +0, their count at +4) and the flag bit
/// (0x40) `OnVisible` sets or clears in each new object's flags word (+0x30).
const CULLER_VISIBLE: u32 = 0x8;
const OBJECT_FLAGS: u32 = 0x30;
const SORT_FLAG: u32 = 0x40;

// Translated from 00a952a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function here; translated from the disassembled bytes), cdecl, no
/// words: a new `NiSortAdjustNode` (0xb0 bytes from `00aa13e0`; `NiNode`'s constructor body
/// `00a5ecb0` with 0, the vtable `010a0384`, sorting mode 0), or 0 when the allocation fails.
/// `fn_00a89cc0` (`nimainsdm.rs`) registers it as the loader for the class name
/// `"NiSortAdjustNode"` (`0109f030`).
pub fn fn_00a952a0(e: &mut Engine) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![NI_SORT_ADJUST_NODE_SIZE]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    e.call(NODE_CONSTRUCT, &args![block, 0u32]);
    e.mem.set_u32(block, NI_SORT_ADJUST_NODE_VTABLE);
    e.mem.set_u32(block + SORTING_MODE, 0);
    Ptr::new(block)
}

// Translated from 00a952e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSortAdjustNode::IsEqual` (Xbox PDB): `NiNode::IsEqual` (`00a5e600`) and the same sorting
/// mode.
pub fn ni_sort_adjust_node_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(NODE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    e.mem.u32(this.addr() + SORTING_MODE) == e.mem.u32(other.addr() + SORTING_MODE)
}

// Translated from 00a95320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSortAdjustNode::_scalar_deleting_destructor_` (Xbox PDB): the vtable `010a0384`,
/// `NiNode`'s destructor body (`00a5e930`), then the sized delete of 0xb0 bytes (`00aa1460`)
/// when bit 0 of `flags` is set. Answers `this`.
pub fn ni_sort_adjust_node_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), NI_SORT_ADJUST_NODE_VTABLE);
    e.call(NODE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, NI_SORT_ADJUST_NODE_SIZE]);
    }
    this
}

// Translated from 00a95350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSortAdjustNode::OnVisible` (Xbox PDB; `__thiscall`, the culler): `NiNode::OnVisible`
/// (`00a5dbe0`); every object it added to the culler's visible array gets flag bit 0x40 set
/// when the sorting mode is 1 and cleared otherwise.
pub fn ni_sort_adjust_node_on_visible(e: &mut Engine, this: Ptr, culler: Ptr) {
    let off = e.mem.u32(this.addr() + SORTING_MODE) != 1;
    let before = e.mem.u32(e.mem.u32(culler.addr() + CULLER_VISIBLE) + 4);
    e.call(NODE_ON_VISIBLE, &args![this, culler]);
    let visible = e.mem.u32(culler.addr() + CULLER_VISIBLE);
    let after = e.mem.u32(visible + 4);
    let mut index = before;
    while index < after {
        let object = e
            .mem
            .u32(e.mem.u32(visible).wrapping_add(index.wrapping_mul(4)));
        let flags = e.mem.u32(object + OBJECT_FLAGS);
        let flags = if off {
            flags & !SORT_FLAG
        } else {
            flags | SORT_FLAG
        };
        e.mem.set_u32(object + OBJECT_FLAGS, flags);
        index += 1;
    }
}

// Translated from 00a953b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSortAdjustNode::CreateClone` (Xbox PDB; `__thiscall`, the cloning process): a new node
/// (0xb0 bytes; `NiNode`'s constructor body with 0, the vtable, sorting mode 0) or null when
/// the allocation fails, `NiNode`'s member copy (`00a5dc80`) into it, then the sorting mode
/// copied (also into the null clone). Answers the clone.
pub fn ni_sort_adjust_node_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let block = e.call(NI_ALLOCATE, &args![NI_SORT_ADJUST_NODE_SIZE]).u32();
    let clone = if block != 0 {
        e.call(NODE_CONSTRUCT, &args![block, 0u32]);
        e.mem.set_u32(block, NI_SORT_ADJUST_NODE_VTABLE);
        e.mem.set_u32(block + SORTING_MODE, 0);
        block
    } else {
        0
    };
    e.call(NODE_COPY_MEMBERS, &args![this, clone, cloning]);
    let mode = e.mem.u32(this.addr() + SORTING_MODE);
    e.mem.set_u32(clone.wrapping_add(SORTING_MODE), mode);
    Ptr::new(clone)
}

// Translated from 00a95410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSortAdjustNode::LinkObject` (Xbox PDB): `NiNode::LinkObject` (`00a5e9d0`), then before
/// stream version 20.0.0.4 one more linked object is taken (`GetObjectFromLinkID`, the result
/// unused).
pub fn ni_sort_adjust_node_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NODE_LINK_OBJECT, &args![this, stream]);
    if e.mem.u32(stream.addr() + STREAM_VERSION) < VERSION_20_0_0_4 {
        e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]);
    }
}

// Translated from 00a95440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSortAdjustNode::LoadBinary` (Xbox PDB): `NiNode::LoadBinary` (`00a5dd40`), the sorting
/// mode (4 bytes through a stack word; the component size in the stream argument's own word),
/// set to 0 when it is 2 or more (signed); before 20.0.0.4 a link id is read (`00a645b0`).
pub fn ni_sort_adjust_node_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let s = stream.addr();
    e.call(NODE_LOAD_BINARY, &args![this, stream]);
    let mode = e.with_stack(8, |e, frame| {
        let (value, sizes) = (frame.addr(), frame.addr() + 4);
        e.mem.set_u32(sizes, 4);
        let input = e.mem.u32(s + STREAM_INPUT);
        let read = e.mem.u32(input + 8);
        let count = e.call(read, &args![input, value, 4u32, sizes, 1u32]).u32();
        let position = e.mem.u32(input + 4).wrapping_add(count);
        e.mem.set_u32(input + 4, position);
        e.mem.u32(value)
    });
    e.mem.set_u32(this.addr() + SORTING_MODE, mode);
    if (mode as i32) >= 2 {
        e.mem.set_u32(this.addr() + SORTING_MODE, 0);
    }
    if e.mem.u32(s + STREAM_VERSION) < VERSION_20_0_0_4 {
        e.call(STREAM_READ_LINK_ID, &args![stream]);
    }
}

// Translated from 00a954b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiSortAdjustNode::SaveBinary` (Xbox PDB): `NiNode::SaveBinary` (`00a5e880`), then the
/// sorting mode (4 bytes).
pub fn ni_sort_adjust_node_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NODE_SAVE_BINARY, &args![this, stream]);
    let output = e.mem.u32(stream.addr() + STREAM_OUTPUT);
    let write = e.mem.u32(output + 0xc);
    let count = e.with_stack(4, |e, sizes| {
        e.mem.set_u32(sizes.addr(), 4);
        e.call(
            write,
            &args![output, this.addr() + SORTING_MODE, 4u32, sizes, 1u32],
        )
        .u32()
    });
    let position = e.mem.u32(output + 4).wrapping_add(count);
    e.mem.set_u32(output + 4, position);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a95310, ni_sort_adjust_node_get_rtti() -> Ptr),
        entry!(0x00a9_5290, ni_sort_adjust_node_process_clone(Ptr, Ptr)),
        entry!(0x00a9_52a0, fn_00a952a0() -> Ptr),
        entry!(0x00a9_52e0, ni_sort_adjust_node_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a9_5320,
            ni_sort_adjust_node_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a9_5350, ni_sort_adjust_node_on_visible(Ptr, Ptr)),
        entry!(0x00a9_53b0, ni_sort_adjust_node_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a9_5410, ni_sort_adjust_node_link_object(Ptr, Ptr)),
        entry!(0x00a9_5440, ni_sort_adjust_node_load_binary(Ptr, Ptr)),
        entry!(0x00a9_54b0, ni_sort_adjust_node_save_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_sort_adjust_node_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a95310, &args![]).u32(), NI_SORT_ADJUST_NODE_RTTI);
    }

    #[test]
    fn thunk_00a95290_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_5290;
        const TARGET: u32 = 0x00a5_dce0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }
}

#[cfg(test)]
mod n019_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn create_compare_clone_and_destroy() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |e, a| word(e.mem.alloc(a[0])));
        for callee in [
            NODE_CONSTRUCT,
            NODE_COPY_MEMBERS,
            NODE_DESTRUCTOR,
            SIZED_DELETE,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(NODE_IS_EQUAL, |_, _| word(1));
        e.call_log = Some(vec![]);
        let node = e.call(0x00a9_52a0, &args![]).u32();
        assert_eq!(e.mem.u32(node), NI_SORT_ADJUST_NODE_VTABLE);
        assert_eq!(calls_to(&e, NODE_CONSTRUCT), vec![vec![node, 0]]);
        e.mem.set_u32(node + SORTING_MODE, 1);
        let clone = e.call(0x00a9_53b0, &args![node, 0x66u32]).u32();
        assert_eq!(e.mem.u32(clone + SORTING_MODE), 1);
        assert_eq!(
            calls_to(&e, NODE_COPY_MEMBERS),
            vec![vec![node, clone, 0x66]]
        );
        assert!(e.call(0x00a9_52e0, &args![node, clone]).bool());
        e.mem.set_u32(clone + SORTING_MODE, 0);
        assert!(!e.call(0x00a9_52e0, &args![node, clone]).bool());
        e.register(NODE_IS_EQUAL, |_, _| word(0));
        e.mem.set_u32(clone + SORTING_MODE, 1);
        assert!(!e.call(0x00a9_52e0, &args![node, clone]).bool());
        e.mem.set_u32(node, 0);
        assert_eq!(e.call(0x00a9_5320, &args![node, 1u32]).u32(), node);
        assert_eq!(e.mem.u32(node), NI_SORT_ADJUST_NODE_VTABLE);
        assert_eq!(
            calls_to(&e, SIZED_DELETE),
            vec![vec![node, NI_SORT_ADJUST_NODE_SIZE]]
        );
        e.call(0x00a9_5320, &args![node, 0u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE).len(), 1);
        e.register(NI_ALLOCATE, |_, _| word(0));
        assert_eq!(e.call(0x00a9_52a0, &args![]).u32(), 0);
    }

    #[test]
    fn on_visible_marks_the_new_objects() {
        let mut e = Engine::new();
        let objects: Vec<u32> = (0..3)
            .map(|_| {
                let object = e.mem.alloc(0x40);
                e.mem.set_u32(object + OBJECT_FLAGS, 0x41);
                object
            })
            .collect();
        let visible = e.mem.alloc(8);
        let array = e.mem.alloc(12);
        for (i, o) in objects.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *o);
        }
        e.mem.set_u32(visible, array);
        e.mem.set_u32(visible + 4, 1);
        let culler = e.mem.alloc(0x10);
        e.mem.set_u32(culler + CULLER_VISIBLE, visible);
        // The node's own `OnVisible` adds the other two.
        e.register_double(NODE_ON_VISIBLE, move |e, _| {
            e.mem.set_u32(visible + 4, 3);
            Ret::default()
        });
        let node = e.mem.alloc(NI_SORT_ADJUST_NODE_SIZE);
        e.call(0x00a9_5350, &args![node, culler]);
        let flags: Vec<u32> = objects
            .iter()
            .map(|o| e.mem.u32(o + OBJECT_FLAGS))
            .collect();
        assert_eq!(flags, vec![0x41, 0x01, 0x01]);
        e.mem.set_u32(visible + 4, 1);
        e.mem.set_u32(node + SORTING_MODE, 1);
        for o in &objects {
            e.mem.set_u32(o + OBJECT_FLAGS, 0);
        }
        e.call(0x00a9_5350, &args![node, culler]);
        let flags: Vec<u32> = objects
            .iter()
            .map(|o| e.mem.u32(o + OBJECT_FLAGS))
            .collect();
        assert_eq!(flags, vec![0, 0x40, 0x40]);
    }

    #[test]
    fn link_load_and_save() {
        const READ_DOUBLE: u32 = 0x0071_1000;
        let mut e = Engine::new();
        for callee in [
            NODE_LINK_OBJECT,
            NODE_LOAD_BINARY,
            NODE_SAVE_BINARY,
            STREAM_GET_OBJECT_FROM_LINK_ID,
            STREAM_READ_LINK_ID,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let stream = e.mem.alloc(0x260);
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 8, READ_DOUBLE);
        e.mem.set_u32(file + 0xc, READ_DOUBLE);
        e.mem.set_u32(stream + STREAM_INPUT, file);
        e.mem.set_u32(stream + STREAM_OUTPUT, file);
        // Reads give 3, then 1; writes are recorded through the same double.
        let mut values = vec![1u32, 3].into_iter();
        e.register_double(READ_DOUBLE, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3])), (4, 4));
            if let Some(v) = values.next_back() {
                e.mem.set_u32(a[1], v);
            }
            word(4)
        });
        let node = e.mem.alloc(NI_SORT_ADJUST_NODE_SIZE);
        e.mem.set_u32(stream + STREAM_VERSION, 0x1400_0003);
        e.call_log = Some(vec![]);
        e.call(0x00a9_5410, &args![node, stream]);
        assert_eq!(
            calls_to(&e, STREAM_GET_OBJECT_FROM_LINK_ID),
            vec![vec![stream]]
        );
        e.call(0x00a9_5440, &args![node, stream]);
        assert_eq!(e.mem.u32(node + SORTING_MODE), 0);
        assert_eq!(calls_to(&e, STREAM_READ_LINK_ID), vec![vec![stream]]);
        e.mem.set_u32(stream + STREAM_VERSION, VERSION_20_0_0_4);
        e.call(0x00a9_5440, &args![node, stream]);
        assert_eq!(e.mem.u32(node + SORTING_MODE), 1);
        e.call(0x00a9_5410, &args![node, stream]);
        assert_eq!(calls_to(&e, STREAM_READ_LINK_ID).len(), 1);
        assert_eq!(calls_to(&e, STREAM_GET_OBJECT_FROM_LINK_ID).len(), 1);
        e.call(0x00a9_54b0, &args![node, stream]);
        let writes = calls_to(&e, READ_DOUBLE);
        assert_eq!(writes[2][1], node + SORTING_MODE);
        assert_eq!(e.mem.u32(file + 4), 12);
    }
}
